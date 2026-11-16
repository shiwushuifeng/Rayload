//! File digests with a persistent cache, checksum export and public hash
//! lookups for GitHub release assets and CIRCL Hash Lookup.
use crate::database::{CachedDigest, DatabaseState};
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Digest, Sha256, Sha512};
use std::collections::{BTreeMap, HashSet};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Every algorithm the app can compute, in display order.
pub const ALL_ALGORITHMS: [&str; 7] = ["md5", "sha1", "sha256", "sha512", "blake3", "sm3", "crc32"];

/// One supported digest algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumAlgorithm {
    Md5,
    Sha1,
    Sha256,
    Sha512,
    Blake3,
    Sm3,
    Crc32,
}

impl ChecksumAlgorithm {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Md5 => "md5",
            Self::Sha1 => "sha1",
            Self::Sha256 => "sha256",
            Self::Sha512 => "sha512",
            Self::Blake3 => "blake3",
            Self::Sm3 => "sm3",
            Self::Crc32 => "crc32",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "md5" => Some(Self::Md5),
            "sha1" | "sha-1" => Some(Self::Sha1),
            "sha256" | "sha-256" => Some(Self::Sha256),
            "sha512" | "sha-512" => Some(Self::Sha512),
            "blake3" => Some(Self::Blake3),
            "sm3" => Some(Self::Sm3),
            "crc32" => Some(Self::Crc32),
            _ => None,
        }
    }

    /// File suffix used by the exported checksum file.
    fn export_suffix(self) -> &'static str {
        match self {
            Self::Md5 => "md5",
            Self::Sha1 => "sha1",
            Self::Sha256 => "sha256",
            Self::Sha512 => "sha512",
            Self::Blake3 => "blake3",
            Self::Sm3 => "sm3",
            Self::Crc32 => "crc32",
        }
    }
}

/// Resolves the requested algorithm list, falling back to the full set.
fn resolve_algorithms(requested: Option<&[String]>) -> Vec<ChecksumAlgorithm> {
    let mut algorithms: Vec<ChecksumAlgorithm> = Vec::new();
    for value in requested.into_iter().flatten() {
        if let Some(algorithm) = ChecksumAlgorithm::parse(value) {
            if !algorithms.contains(&algorithm) {
                algorithms.push(algorithm);
            }
        }
    }
    if algorithms.is_empty() {
        algorithms = ALL_ALGORITHMS
            .iter()
            .filter_map(|value| ChecksumAlgorithm::parse(value))
            .collect();
    }
    algorithms
}

enum Hasher {
    Md5(md5::Md5),
    Sha1(Sha1),
    Sha256(Sha256),
    Sha512(Sha512),
    Blake3(Box<blake3::Hasher>),
    Sm3(sm3::Sm3),
    Crc32(crc32fast::Hasher),
}

impl Hasher {
    fn new(algorithm: ChecksumAlgorithm) -> Self {
        match algorithm {
            ChecksumAlgorithm::Md5 => Self::Md5(md5::Md5::new()),
            ChecksumAlgorithm::Sha1 => Self::Sha1(Sha1::new()),
            ChecksumAlgorithm::Sha256 => Self::Sha256(Sha256::new()),
            ChecksumAlgorithm::Sha512 => Self::Sha512(Sha512::new()),
            ChecksumAlgorithm::Blake3 => Self::Blake3(Box::new(blake3::Hasher::new())),
            ChecksumAlgorithm::Sm3 => Self::Sm3(sm3::Sm3::new()),
            ChecksumAlgorithm::Crc32 => Self::Crc32(crc32fast::Hasher::new()),
        }
    }

    fn update(&mut self, chunk: &[u8]) {
        match self {
            Self::Md5(hasher) => hasher.update(chunk),
            Self::Sha1(hasher) => hasher.update(chunk),
            Self::Sha256(hasher) => hasher.update(chunk),
            Self::Sha512(hasher) => hasher.update(chunk),
            Self::Blake3(hasher) => {
                hasher.update(chunk);
            }
            Self::Sm3(hasher) => hasher.update(chunk),
            Self::Crc32(hasher) => hasher.update(chunk),
        }
    }

    fn finish(self) -> String {
        match self {
            Self::Md5(hasher) => format!("{:x}", hasher.finalize()),
            Self::Sha1(hasher) => format!("{:x}", hasher.finalize()),
            Self::Sha256(hasher) => format!("{:x}", hasher.finalize()),
            Self::Sha512(hasher) => format!("{:x}", hasher.finalize()),
            Self::Blake3(hasher) => hasher.finalize().to_hex().to_string(),
            Self::Sm3(hasher) => format!("{:x}", hasher.finalize()),
            Self::Crc32(hasher) => format!("{:08x}", hasher.finalize()),
        }
    }
}

/// Digests computed for one file, plus what came from the cache.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDigests {
    pub path: String,
    pub size: u64,
    pub mtime_secs: i64,
    /// Algorithm name to lowercase hex digest.
    pub digests: BTreeMap<String, String>,
    /// Algorithms answered by the cache without reading the file again.
    pub cached: Vec<String>,
    pub elapsed_ms: u64,
}

/// Cancellation flags for running digest jobs, keyed by the caller's request id.
static CANCELLED: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

fn cancellation_requested(request_id: Option<&str>) -> bool {
    let Some(id) = request_id else {
        return false;
    };
    CANCELLED
        .lock()
        .map(|flags| flags.contains(id))
        .unwrap_or(false)
}

fn clear_cancellation(request_id: Option<&str>) {
    let Some(id) = request_id else { return };
    if let Ok(mut flags) = CANCELLED.lock() {
        flags.remove(id);
    }
}

/// Requests cancellation of a running digest job.
#[tauri::command]
pub fn cancel_file_digests(request_id: String) {
    if let Ok(mut flags) = CANCELLED.lock() {
        flags.insert(request_id);
    }
}

fn file_version(path: &Path) -> Result<(u64, i64), AppError> {
    let metadata = std::fs::metadata(path)?;
    if !metadata.is_file() {
        return Err(AppError::NotFound(format!(
            "File is unavailable for checksum calculation: {}",
            path.display()
        )));
    }
    let mtime_secs = metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0);
    Ok((metadata.len(), mtime_secs))
}

/// Computes the requested digests in a single read of the file.
pub(crate) fn compute_digests(
    path: &Path,
    algorithms: &[ChecksumAlgorithm],
    request_id: Option<&str>,
) -> Result<BTreeMap<String, String>, AppError> {
    if algorithms.is_empty() {
        return Ok(BTreeMap::new());
    }
    let mut file = std::fs::File::open(path)?;
    let mut hashers: Vec<(ChecksumAlgorithm, Hasher)> = algorithms
        .iter()
        .map(|algorithm| (*algorithm, Hasher::new(*algorithm)))
        .collect();
    let mut buffer = vec![0_u8; 1024 * 1024];

    loop {
        if cancellation_requested(request_id) {
            return Err(AppError::InvalidInput("Checksum calculation cancelled".into()));
        }
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        let chunk = &buffer[..read];
        for (_, hasher) in hashers.iter_mut() {
            hasher.update(chunk);
        }
    }

    let mut digests = BTreeMap::new();
    for (algorithm, hasher) in hashers {
        digests.insert(algorithm.as_str().to_string(), hasher.finish());
    }
    Ok(digests)
}

fn unix_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0)
}

/// The classic digest set the LAN protocol advertises to paired devices.
pub(crate) struct ClassicDigests {
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
    pub sha512: String,
    pub size: u64,
}

/// Computes the four classic digests for a file shared over LAN. Local
/// preferences about which algorithms to show do not change the wire format.
pub(crate) fn file_digests_sync(path: &Path) -> Result<ClassicDigests, AppError> {
    let (size, _) = file_version(path)?;
    let algorithms = [
        ChecksumAlgorithm::Md5,
        ChecksumAlgorithm::Sha1,
        ChecksumAlgorithm::Sha256,
        ChecksumAlgorithm::Sha512,
    ];
    let digests = compute_digests(path, &algorithms, None)?;
    Ok(ClassicDigests {
        md5: digests["md5"].clone(),
        sha1: digests["sha1"].clone(),
        sha256: digests["sha256"].clone(),
        sha512: digests["sha512"].clone(),
        size,
    })
}

/// Computes the requested digests, reusing cached values for an unchanged file.
#[tauri::command]
pub async fn file_digests(
    state: tauri::State<'_, DatabaseState>,
    path: String,
    algorithms: Option<Vec<String>>,
    request_id: Option<String>,
) -> Result<FileDigests, AppError> {
    let requested = resolve_algorithms(algorithms.as_deref());
    let file_path = PathBuf::from(&path);
    let started = unix_millis();
    let (size, mtime_secs) = file_version(&file_path)?;

    let mut cached: Vec<CachedDigest> = Vec::new();
    if state.0.is_ready().await {
        cached = state
            .0
            .read_cached_digests(&path, size, mtime_secs)
            .await
            .unwrap_or_default();
    }
    let mut digests: BTreeMap<String, String> = BTreeMap::new();
    let mut cached_algorithms: Vec<String> = Vec::new();
    for entry in &cached {
        if requested
            .iter()
            .any(|algorithm| algorithm.as_str() == entry.algorithm)
        {
            digests.insert(entry.algorithm.clone(), entry.digest.clone());
            cached_algorithms.push(entry.algorithm.clone());
        }
    }

    let missing: Vec<ChecksumAlgorithm> = requested
        .into_iter()
        .filter(|algorithm| !digests.contains_key(algorithm.as_str()))
        .collect();

    if !missing.is_empty() {
        let compute_path = file_path.clone();
        let request = request_id.clone();
        let computed = tauri::async_runtime::spawn_blocking(move || {
            compute_digests(&compute_path, &missing, request.as_deref())
        })
        .await
        .map_err(|error| AppError::Io(format!("Digest worker failed: {error}")))?;
        clear_cancellation(request_id.as_deref());
        let computed = computed?;
        if state.0.is_ready().await {
            let entries: Vec<(String, String)> = computed
                .iter()
                .map(|(algorithm, digest)| (algorithm.clone(), digest.clone()))
                .collect();
            let _ = state
                .0
                .write_cached_digests(&path, size, mtime_secs, &entries)
                .await;
        }
        digests.extend(computed);
    }
    clear_cancellation(request_id.as_deref());

    Ok(FileDigests {
        path,
        size,
        mtime_secs,
        digests,
        cached: cached_algorithms,
        elapsed_ms: unix_millis().saturating_sub(started) as u64,
    })
}

/// Drops every cached digest and reports how many rows were removed.
#[tauri::command]
pub async fn clear_checksum_cache(state: tauri::State<'_, DatabaseState>) -> Result<usize, AppError> {
    state.0.clear_cached_digests().await
}

/// Writes a `digest  filename` checksum file next to the downloaded file.
#[tauri::command]
pub async fn export_checksum_file(
    path: String,
    algorithm: String,
    digest: String,
) -> Result<String, AppError> {
    let parsed = ChecksumAlgorithm::parse(&algorithm)
        .ok_or_else(|| AppError::InvalidInput(format!("Unsupported algorithm: {algorithm}")))?;
    let digest = digest.trim().to_ascii_lowercase();
    if digest.is_empty() || !digest.chars().all(|character| character.is_ascii_hexdigit()) {
        return Err(AppError::InvalidInput("Digest must be hexadecimal".into()));
    }
    let source = PathBuf::from(&path);
    let file_name = source
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .ok_or_else(|| AppError::InvalidInput("Path has no file name".into()))?;
    let target = source.with_extension(format!(
        "{}.{}",
        source
            .extension()
            .map(|extension| extension.to_string_lossy().into_owned())
            .unwrap_or_default(),
        parsed.export_suffix()
    ));
    let target = if target.as_os_str().is_empty() {
        source.with_extension(parsed.export_suffix())
    } else {
        target
    };
    let contents = format!("{digest}  {file_name}\n");
    let write_target = target.clone();
    tauri::async_runtime::spawn_blocking(move || std::fs::write(&write_target, contents))
        .await
        .map_err(|error| AppError::Io(format!("Checksum export worker failed: {error}")))?
        .map_err(|error| AppError::Io(error.to_string()))?;
    Ok(target.display().to_string())
}

/// One public database that knows a hash.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KnownDigestEntry {
    /// Database that answered, for example `circl`.
    pub source: String,
    pub file_name: Option<String>,
    pub sha1: Option<String>,
    pub sha256: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CirclLookup {
    #[serde(default)]
    file_name: Option<String>,
    #[serde(default, rename = "SHA-1")]
    sha1: Option<String>,
    #[serde(default, rename = "SHA-256")]
    sha256: Option<String>,
}

async fn circl_lookup(algorithm: &str, digest: &str) -> Result<Option<KnownDigestEntry>, AppError> {
    let url = format!("https://hashlookup.circl.lu/lookup/{algorithm}/{digest}");
    let response = reqwest::Client::builder()
        .timeout(Duration::from_secs(12))
        .build()?
        .get(url)
        .header(reqwest::header::USER_AGENT, "Rayload")
        .send()
        .await?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    if !response.status().is_success() {
        return Err(AppError::Io(format!(
            "CIRCL lookup failed with HTTP {}",
            response.status()
        )));
    }
    let body: CirclLookup = response.json().await?;
    Ok(Some(KnownDigestEntry {
        source: "circl".into(),
        file_name: body.file_name,
        sha1: body.sha1,
        sha256: body.sha256,
    }))
}

/// Asks public hash databases about a file. GitHub release metadata is handled
/// separately by [`github_asset_digest`] because it needs the release URL.
#[tauri::command]
pub async fn online_checksum_lookup(
    sha1: Option<String>,
    sha256: Option<String>,
) -> Result<Vec<KnownDigestEntry>, AppError> {
    let mut entries = Vec::new();
    if let Some(digest) = sha1.filter(|value| !value.trim().is_empty()) {
        if let Some(entry) = circl_lookup("sha1", digest.trim()).await? {
            entries.push(entry);
        }
    }
    if let Some(digest) = sha256.filter(|value| !value.trim().is_empty()) {
        if let Some(entry) = circl_lookup("sha256", digest.trim()).await? {
            if !entries.iter().any(|existing| {
                existing.sha256.is_some() && existing.sha256 == entry.sha256
            }) {
                entries.push(entry);
            }
        }
    }
    Ok(entries)
}

/// A digest published by GitHub for a release asset.
#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseAssetDigest {
    pub algorithm: String,
    pub digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GitHubReleaseAsset {
    owner: String,
    repository: String,
    tag: Option<String>,
    asset: String,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    assets: Vec<GitHubAsset>,
}

#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    digest: Option<String>,
}

fn parse_github_release_asset(value: &str) -> Option<GitHubReleaseAsset> {
    let parsed = url::Url::parse(value).ok()?;
    if parsed.scheme() != "https" {
        return None;
    }
    let host = parsed.host_str()?.to_ascii_lowercase();
    if host != "github.com" && host != "www.github.com" {
        return None;
    }
    let segments: Vec<String> = parsed
        .path_segments()?
        .map(|segment| urlencoding::decode(segment).map(|value| value.into_owned()))
        .collect::<Result<_, _>>()
        .ok()?;
    if segments.len() < 6 || segments[2] != "releases" {
        return None;
    }

    let owner = segments[0].clone();
    let repository = segments[1].clone();
    if segments[3] == "download" {
        let tag = segments[4].clone();
        let asset = segments[5..].join("/");
        return (!owner.is_empty()
            && !repository.is_empty()
            && !tag.is_empty()
            && !asset.is_empty())
        .then_some(GitHubReleaseAsset {
            owner,
            repository,
            tag: Some(tag),
            asset,
        });
    }
    if segments[3] == "latest" && segments[4] == "download" {
        let asset = segments[5..].join("/");
        return (!owner.is_empty() && !repository.is_empty() && !asset.is_empty()).then_some(
            GitHubReleaseAsset {
                owner,
                repository,
                tag: None,
                asset,
            },
        );
    }
    None
}

fn parse_release_digest(value: &str) -> Option<ReleaseAssetDigest> {
    let (algorithm, digest) = value.split_once(':')?;
    let algorithm = algorithm.trim().to_ascii_lowercase();
    let digest = digest.trim().to_ascii_lowercase();
    if !matches!(algorithm.as_str(), "md5" | "sha1" | "sha256" | "sha512")
        || digest.is_empty()
        || !digest
            .chars()
            .all(|character| character.is_ascii_hexdigit())
    {
        return None;
    }
    Some(ReleaseAssetDigest { algorithm, digest })
}

/// Reads the `digest` GitHub publishes for a release asset, when the source
/// URL names one. Other URLs and releases without a digest return `None`.
#[tauri::command]
pub async fn github_asset_digest(url: String) -> Result<Option<ReleaseAssetDigest>, AppError> {
    let Some(reference) = parse_github_release_asset(&url) else {
        return Ok(None);
    };
    let api_url = match reference.tag.as_deref() {
        Some(tag) => format!(
            "https://api.github.com/repos/{}/{}/releases/tags/{}",
            reference.owner,
            reference.repository,
            urlencoding::encode(tag)
        ),
        None => format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            reference.owner, reference.repository
        ),
    };

    let response = reqwest::Client::builder()
        .timeout(Duration::from_secs(12))
        .build()?
        .get(api_url)
        .header(reqwest::header::USER_AGENT, "Rayload")
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    if !response.status().is_success() {
        return Err(AppError::Io(format!(
            "GitHub release lookup failed with HTTP {}",
            response.status()
        )));
    }
    let release: GitHubRelease = response.json().await?;
    Ok(release
        .assets
        .into_iter()
        .find(|asset| asset.name == reference.asset)
        .and_then(|asset| asset.digest.as_deref().and_then(parse_release_digest)))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Writes a temporary file and returns its path.
    fn temp_file(name: &str, contents: &[u8]) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("rayload-digests-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).expect("create temp dir");
        let file = dir.join(name);
        std::fs::write(&file, contents).expect("write file");
        file
    }

    #[test]
    fn parses_tagged_github_release_asset() {
        assert_eq!(
            parse_github_release_asset(
                "https://github.com/owner/repo/releases/download/v1.2.3/tool%20arm64.apk"
            ),
            Some(GitHubReleaseAsset {
                owner: "owner".into(),
                repository: "repo".into(),
                tag: Some("v1.2.3".into()),
                asset: "tool arm64.apk".into(),
            })
        );
    }

    #[test]
    fn parses_latest_github_release_asset() {
        assert_eq!(
            parse_github_release_asset(
                "https://github.com/owner/repo/releases/latest/download/tool.apk"
            ),
            Some(GitHubReleaseAsset {
                owner: "owner".into(),
                repository: "repo".into(),
                tag: None,
                asset: "tool.apk".into(),
            })
        );
    }

    #[test]
    fn ignores_non_release_urls() {
        assert_eq!(
            parse_github_release_asset("https://github.com/owner/repo"),
            None
        );
        assert_eq!(
            parse_github_release_asset("https://example.com/owner/repo/releases/download/v1/a"),
            None
        );
    }

    #[test]
    fn parses_release_digest_values() {
        assert_eq!(
            parse_release_digest("sha256:ABCDEF0123"),
            Some(ReleaseAssetDigest {
                algorithm: "sha256".into(),
                digest: "abcdef0123".into(),
            })
        );
        assert_eq!(parse_release_digest("sha256"), None);
        assert_eq!(parse_release_digest("sha256:not-hex"), None);
    }

    #[test]
    fn computes_common_digests_in_one_pass() {
        let file = temp_file("hello.txt", b"hello");
        let algorithms = resolve_algorithms(None);
        let digests = compute_digests(&file, &algorithms, None).expect("hash file");

        assert_eq!(digests["md5"], "5d41402abc4b2a76b9719d911017c592");
        assert_eq!(digests["sha1"], "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
        assert_eq!(
            digests["sha256"],
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
        assert_eq!(
            digests["blake3"],
            "ea8f163db38682925e4491c5e58d4bb3506ef8c14eb78a86e908c5624a67200f"
        );
        assert_eq!(
            digests["sm3"],
            "becbbfaae6548b8bf0cfcad5a27183cd1be6093b1cceccc303d9c61d0a645268"
        );
        assert_eq!(digests["crc32"], "3610a686");
        assert_eq!(
            digests["sha512"],
            "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca7\
             2323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"
        );
    }

    #[test]
    fn computes_only_the_requested_algorithms() {
        let file = temp_file("partial.txt", b"hello");
        let requested = vec!["sha256".to_string(), "sha-256".to_string()];
        let algorithms = resolve_algorithms(Some(&requested));
        let digests = compute_digests(&file, &algorithms, None).expect("hash file");
        assert_eq!(algorithms.len(), 1);
        assert_eq!(digests.len(), 1);
        assert!(digests.contains_key("sha256"));
    }

    #[test]
    fn cancellation_stops_a_running_job() {
        let file = temp_file("cancel.txt", b"hello");
        let algorithms = resolve_algorithms(None);
        cancel_file_digests("job-test".into());
        let result = compute_digests(&file, &algorithms, Some("job-test"));
        clear_cancellation(Some("job-test"));
        assert!(result.is_err());
        // The next run on the same id works again after clearing the flag.
        assert!(compute_digests(&file, &algorithms, Some("job-test")).is_ok());
    }
}
