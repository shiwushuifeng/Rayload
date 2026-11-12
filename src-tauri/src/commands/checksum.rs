//! File digests and optional release-asset digest lookup for GitHub downloads.
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Digest, Sha256, Sha512};
use std::io::Read;
use std::path::Path;
use std::time::Duration;

/// Common digests for one local file, read from disk only once.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDigests {
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
    pub sha512: String,
    pub size: u64,
}

/// A digest published by GitHub for a release asset.
#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseAssetDigest {
    pub algorithm: String,
    pub digest: String,
}

#[tauri::command]
pub async fn file_digests(path: String) -> Result<FileDigests, AppError> {
    tauri::async_runtime::spawn_blocking(move || file_digests_sync(Path::new(&path)))
        .await
        .map_err(|error| AppError::Io(format!("Digest worker failed: {error}")))?
}

fn file_digests_sync(path: &Path) -> Result<FileDigests, AppError> {
    if !path.is_file() {
        return Err(AppError::NotFound(format!(
            "File is unavailable for checksum calculation: {}",
            path.display()
        )));
    }

    let mut file = std::fs::File::open(path)?;
    let mut md5 = md5::Md5::new();
    let mut sha1 = Sha1::new();
    let mut sha256 = Sha256::new();
    let mut sha512 = Sha512::new();
    let mut buffer = vec![0_u8; 1024 * 1024];
    let mut size = 0_u64;

    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        let chunk = &buffer[..read];
        md5.update(chunk);
        sha1.update(chunk);
        sha256.update(chunk);
        sha512.update(chunk);
        size += read as u64;
    }

    Ok(FileDigests {
        md5: format!("{:x}", md5.finalize()),
        sha1: format!("{:x}", sha1.finalize()),
        sha256: format!("{:x}", sha256.finalize()),
        sha512: format!("{:x}", sha512.finalize()),
        size,
    })
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
        let dir = std::env::temp_dir().join(format!("rayload-digests-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).expect("create temp dir");
        let file = dir.join("hello.txt");
        std::fs::write(&file, b"hello").expect("write file");

        let digests = file_digests_sync(&file).expect("hash file");
        assert_eq!(digests.size, 5);
        assert_eq!(digests.md5, "5d41402abc4b2a76b9719d911017c592");
        assert_eq!(digests.sha1, "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
        assert_eq!(
            digests.sha256,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );

        std::fs::remove_dir_all(&dir).expect("clean temp dir");
    }
}
