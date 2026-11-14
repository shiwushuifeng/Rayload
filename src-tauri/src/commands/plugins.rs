//! Local plugin manifest management and the privileged plugin host.
//!
//! JSON rule plugins are executed by Rust so domain, Cookie and header policy
//! can be enforced after the user grants permissions. Script plugins run in a
//! separate shell process and receive only the capabilities the user approved.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::error::AppError;
use crate::plugin_policy::{
    PluginCookieStore, PluginKind, PluginManifest, PluginPermission, PluginPolicy, PolicyError,
};

const MAX_SCRIPT_BYTES: u64 = 1024 * 1024;
const MAX_HTTP_RESPONSE_BYTES: usize = 8 * 1024 * 1024;

/// The bundled sample downloads Rayload itself. It asks the GitHub release
/// API for the newest published package matching the current ABI, so the test
/// keeps working after every release without editing the plugin.
fn sample_plugin() -> String {
    let version = env!("CARGO_PKG_VERSION");
    #[cfg(target_arch = "x86_64")]
    let asset = "x86_64-emulator.apk";
    #[cfg(not(target_arch = "x86_64"))]
    let asset = "arm64";
    format!(
        r#"{{
  "name": "Rayload self-download test",
  "version": "{version}",
  "description": "Downloads the latest Rayload Android package to test the download engine.",
  "type": "rule",
  "runtime": "download",
  "permissions": ["network"],
  "allowedDomains": ["api.github.com", "github.com"],
  "grantedPermissions": ["network"],
  "repository": "shiwushuifeng/Rayload",
  "asset": "{asset}"
}}
"#
    )
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginEntry {
    pub file: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub kind: String,
    pub runtime: String,
    pub permissions: Vec<PluginPermission>,
    pub granted_permissions: Vec<PluginPermission>,
    pub size: u64,
}

fn plugins_dir(app: &AppHandle) -> Result<PathBuf, AppError> {
    let directory = app
        .path()
        .app_local_data_dir()
        .map_err(|error| AppError::Io(error.to_string()))?
        .join("plugins");
    std::fs::create_dir_all(&directory).map_err(|error| AppError::Io(error.to_string()))?;
    Ok(directory)
}

fn plugin_cookie_store(app: &AppHandle) -> PluginCookieStore {
    let Ok(directory) = app.path().app_local_data_dir() else {
        return PluginCookieStore::default();
    };
    PluginCookieStore::load(&directory.join("plugin-cookies.json"))
}

fn safe_manifest_name(filename: &str) -> Result<&str, AppError> {
    let name = filename.trim();
    if name.is_empty()
        || name.len() > 128
        || !name.ends_with(".json")
        || name.contains(['/', '\\'])
        || name.starts_with('.')
    {
        return Err(AppError::InvalidInput("Invalid plugin file name".into()));
    }
    Ok(name)
}

fn safe_script_name(filename: &str) -> Result<&str, AppError> {
    let name = filename.trim();
    if name.is_empty()
        || name.len() > 128
        || !name.ends_with(".sh")
        || name.contains(['/', '\\'])
        || name.starts_with('.')
    {
        return Err(AppError::InvalidInput("Invalid plugin script name".into()));
    }
    Ok(name)
}

fn policy_message(error: &PolicyError) -> String {
    error.to_string()
}

fn manifest_policy(manifest: &PluginManifest) -> Result<PluginPolicy, AppError> {
    manifest
        .policy()
        .map_err(|error| AppError::InvalidInput(policy_message(&error)))
}

fn read_manifest(path: &Path) -> Result<(String, PluginManifest), AppError> {
    let content = std::fs::read_to_string(path).map_err(|error| AppError::Io(error.to_string()))?;
    let manifest = PluginManifest::parse(&content)
        .map_err(|error| AppError::InvalidInput(policy_message(&error)))?;
    Ok((content, manifest))
}

fn entry(path: &Path) -> Option<PluginEntry> {
    let (_, manifest) = read_manifest(path).ok()?;
    let directory = path.parent()?;
    let mut size = std::fs::metadata(path).ok()?.len();
    if let Some(script) = manifest.entry.as_deref() {
        if safe_script_name(script).is_ok() {
            if let Ok(metadata) = std::fs::metadata(directory.join(script)) {
                size += metadata.len();
            }
        }
    }
    Some(PluginEntry {
        file: path.file_name()?.to_string_lossy().to_string(),
        name: manifest.name,
        version: manifest.version,
        description: manifest.description,
        kind: match manifest.kind {
            PluginKind::Rule => "rule",
            PluginKind::Script => "script",
        }
        .into(),
        runtime: manifest.runtime,
        permissions: manifest.permissions,
        granted_permissions: manifest.granted_permissions,
        size,
    })
}

#[tauri::command]
pub fn plugin_list(app: AppHandle) -> Result<Vec<PluginEntry>, AppError> {
    let directory = plugins_dir(&app)?;
    let mut plugins = std::fs::read_dir(directory)
        .map_err(|error| AppError::Io(error.to_string()))?
        .flatten()
        .filter(|item| item.path().extension().is_some_and(|ext| ext == "json"))
        .filter_map(|item| entry(&item.path()))
        .collect::<Vec<_>>();
    plugins.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(plugins)
}

fn requested_grants(
    manifest: &PluginManifest,
    granted_permissions: &[String],
) -> Result<Vec<PluginPermission>, AppError> {
    let declared: BTreeSet<_> = manifest.permissions.iter().copied().collect();
    let mut granted = Vec::new();
    for raw in granted_permissions {
        let permission = match raw.as_str() {
            "network" => PluginPermission::Network,
            "cookies" => PluginPermission::Cookies,
            "modify_headers" => PluginPermission::ModifyHeaders,
            "read_clipboard" => PluginPermission::ReadClipboard,
            _ => {
                return Err(AppError::InvalidInput(format!(
                    "Unknown plugin permission: {raw}"
                )))
            }
        };
        if !declared.contains(&permission) {
            return Err(AppError::InvalidInput(format!(
                "Plugin did not declare permission: {raw}"
            )));
        }
        if !granted.contains(&permission) {
            granted.push(permission);
        }
    }
    Ok(granted)
}

#[tauri::command]
pub fn plugin_save(
    app: AppHandle,
    filename: String,
    content: String,
    granted_permissions: Vec<String>,
) -> Result<PluginEntry, AppError> {
    let filename = safe_manifest_name(&filename)?;
    let manifest = PluginManifest::parse(&content)
        .map_err(|error| AppError::InvalidInput(policy_message(&error)))?;
    manifest_policy(&manifest)?;
    let granted = requested_grants(&manifest, &granted_permissions)?;
    let mut value: Value = serde_json::from_str(&content)
        .map_err(|_| AppError::InvalidInput("Plugin manifest is not valid JSON".into()))?;
    let object = value
        .as_object_mut()
        .ok_or_else(|| AppError::InvalidInput("Plugin manifest must be an object".into()))?;
    object.insert(
        "grantedPermissions".into(),
        Value::Array(
            granted
                .into_iter()
                .map(|permission| Value::String(permission.as_str().into()))
                .collect(),
        ),
    );
    let normalized = serde_json::to_string_pretty(&value)
        .map_err(|_| AppError::InvalidInput("Plugin manifest could not be serialized".into()))?;
    let path = plugins_dir(&app)?.join(filename);
    std::fs::write(&path, normalized).map_err(|error| AppError::Io(error.to_string()))?;
    entry(&path).ok_or_else(|| AppError::InvalidInput("Plugin manifest could not be read".into()))
}

/// Imports a standalone shell plugin. Standalone scripts cannot declare
/// privileged JSON capabilities; use a JSON manifest when network or cookies
/// are required.
#[tauri::command]
pub fn plugin_save_script(
    app: AppHandle,
    filename: String,
    content: String,
) -> Result<PluginEntry, AppError> {
    let filename = safe_script_name(&filename)?;
    if content.len() as u64 > MAX_SCRIPT_BYTES {
        return Err(AppError::InvalidInput("Plugin script is too large".into()));
    }
    if content.trim().is_empty() {
        return Err(AppError::InvalidInput("Plugin script is empty".into()));
    }
    let directory = plugins_dir(&app)?;
    std::fs::write(directory.join(filename), content)
        .map_err(|error| AppError::Io(error.to_string()))?;
    let stem = filename.trim_end_matches(".sh");
    let manifest_path = directory.join(format!("{stem}.json"));
    let manifest = serde_json::json!({
        "name": stem,
        "version": "",
        "description": "Imported shell plugin",
        "type": "script",
        "runtime": "script",
        "entry": filename,
        "permissions": [],
        "allowedDomains": [],
        "grantedPermissions": [],
    });
    std::fs::write(
        &manifest_path,
        serde_json::to_string_pretty(&manifest).map_err(|_| {
            AppError::InvalidInput("Plugin manifest could not be serialized".into())
        })?,
    )
    .map_err(|error| AppError::Io(error.to_string()))?;
    entry(&manifest_path)
        .ok_or_else(|| AppError::InvalidInput("Plugin manifest could not be read".into()))
}

#[tauri::command]
pub fn plugin_remove(app: AppHandle, file: String) -> Result<(), AppError> {
    let directory = plugins_dir(&app)?;
    let path = directory.join(safe_manifest_name(&file)?);
    if let Ok((_, manifest)) = read_manifest(&path) {
        if let Some(script) = manifest
            .entry
            .as_deref()
            .filter(|value| safe_script_name(value).is_ok())
        {
            let _ = std::fs::remove_file(directory.join(script));
        }
    }
    std::fs::remove_file(path).map_err(|error| AppError::Io(error.to_string()))
}

#[tauri::command]
pub fn plugin_sample_manifest() -> String {
    sample_plugin()
}

fn load_plugin(app: &AppHandle, file: &str) -> Result<(PathBuf, String, PluginManifest), AppError> {
    let path = plugins_dir(app)?.join(safe_manifest_name(file)?);
    let (content, manifest) = read_manifest(&path)?;
    Ok((path, content, manifest))
}

#[tauri::command]
pub fn plugin_read_clipboard(app: AppHandle, file: String) -> Result<String, AppError> {
    let (_, _, manifest) = load_plugin(&app, &file)?;
    let policy = manifest_policy(&manifest)?;
    if !policy.can_read_clipboard() {
        return Err(AppError::InvalidInput(
            "Plugin does not have clipboard permission".into(),
        ));
    }
    app.clipboard()
        .read_text()
        .map_err(|error| AppError::InvalidInput(error.to_string()))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RuleSpec {
    #[serde(rename = "match")]
    pattern: String,
    request: RuleRequest,
    #[serde(default)]
    extract: RuleExtraction,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RuleRequest {
    #[serde(default = "default_method")]
    method: String,
    url: String,
    #[serde(default)]
    headers: BTreeMap<String, String>,
    #[serde(default)]
    body: Option<String>,
}

fn default_method() -> String {
    "GET".into()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RuleExtraction {
    #[serde(default)]
    json: Option<String>,
    #[serde(default)]
    regex: Option<String>,
    #[serde(default = "default_group")]
    group: usize,
}

impl Default for RuleExtraction {
    fn default() -> Self {
        Self {
            json: None,
            regex: None,
            group: 1,
        }
    }
}

fn default_group() -> usize {
    1
}

fn render_template(
    template: &str,
    captures: &[String],
    variables: &BTreeMap<String, String>,
) -> String {
    let mut rendered = template.to_string();
    for (index, value) in captures.iter().enumerate().skip(1) {
        rendered = rendered.replace(&format!("{{{index}}}"), value);
    }
    for (key, value) in variables {
        rendered = rendered.replace(&format!("{{{{{key}}}}}"), value);
    }
    rendered
}

fn json_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    path.split('.')
        .filter(|segment| !segment.is_empty())
        .try_fold(value, |current, segment| {
            if let Ok(index) = segment.parse::<usize>() {
                current.get(index)
            } else {
                current.get(segment)
            }
        })
}

fn extract_download_url(body: &str, extraction: &RuleExtraction) -> Result<String, AppError> {
    if let Some(path) = extraction.json.as_deref() {
        let value: Value = serde_json::from_str(body)
            .map_err(|_| AppError::InvalidInput("Plugin response is not valid JSON".into()))?;
        return json_path(&value, path)
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(str::to_string)
            .ok_or_else(|| {
                AppError::InvalidInput(format!("Plugin JSON path `{path}` has no URL"))
            });
    }
    if let Some(pattern) = extraction.regex.as_deref() {
        let regex = regex::Regex::new(pattern)
            .map_err(|error| AppError::InvalidInput(format!("Invalid plugin regex: {error}")))?;
        let captures = regex
            .captures(body)
            .ok_or_else(|| AppError::InvalidInput("Plugin regex did not match".into()))?;
        return captures
            .get(extraction.group)
            .map(|capture| capture.as_str().to_string())
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| AppError::InvalidInput("Plugin regex capture is empty".into()));
    }
    Err(AppError::InvalidInput(
        "Plugin rule needs a json or regex extraction".into(),
    ))
}

fn http_client(timeout: Duration) -> Result<reqwest::Client, AppError> {
    reqwest::Client::builder()
        .timeout(timeout)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|error| AppError::Updater(format!("Plugin HTTP client failed: {error}")))
}

/// Looks up the newest published release of a repository and returns the
/// download URL of the first `.apk` asset containing `asset_hint`.
async fn latest_release_asset(repository: &str, asset_hint: &str) -> Result<String, AppError> {
    let client = http_client(Duration::from_secs(15))?;
    let response = client
        .get(format!(
            "https://api.github.com/repos/{repository}/releases/latest"
        ))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", concat!("Rayload/", env!("CARGO_PKG_VERSION")))
        .send()
        .await
        .map_err(|error| AppError::Updater(format!("Plugin release lookup failed: {error}")))?;
    if !response.status().is_success() {
        return Err(AppError::Updater(format!(
            "Plugin release lookup returned HTTP {}",
            response.status().as_u16()
        )));
    }
    let body: Value = response.json().await.map_err(|error| {
        AppError::Updater(format!("Plugin release lookup was invalid: {error}"))
    })?;
    let assets = body["assets"]
        .as_array()
        .ok_or_else(|| AppError::Updater("Plugin release has no assets".into()))?;
    let urls = assets
        .iter()
        .filter_map(|asset| asset["browser_download_url"].as_str())
        .filter(|url| url.ends_with(".apk"))
        .collect::<Vec<_>>();
    let preferred = urls
        .iter()
        .find(|url| url.contains(asset_hint))
        .or_else(|| urls.first())
        .ok_or_else(|| AppError::NotFound("Rayload release has no APK asset".into()))?;
    Ok((*preferred).to_string())
}

/// Resolves a share URL with a rule plugin and returns the extracted download
/// URL. Every request passes through the permission policy.
#[tauri::command]
pub async fn plugin_resolve(
    app: AppHandle,
    file: String,
    url: String,
    variables: Option<BTreeMap<String, String>>,
) -> Result<Value, AppError> {
    let (_, content, manifest) = load_plugin(&app, &file)?;
    let policy = manifest_policy(&manifest)?;
    let value: Value = serde_json::from_str(&content)
        .map_err(|_| AppError::InvalidInput("Plugin manifest is not valid JSON".into()))?;
    let rules: Vec<RuleSpec> = serde_json::from_value(
        value
            .get("rules")
            .cloned()
            .ok_or_else(|| AppError::InvalidInput("Rule plugin has no rules".into()))?,
    )
    .map_err(|error| AppError::InvalidInput(format!("Invalid plugin rules: {error}")))?;
    let variables = variables.unwrap_or_default();
    let selected = rules.iter().find_map(|rule| {
        regex::Regex::new(&rule.pattern)
            .ok()
            .and_then(|regex| regex.captures(&url).map(|captures| (rule, captures)))
    });
    let Some((rule, captures)) = selected else {
        return Err(AppError::NotFound("No plugin rule matched this URL".into()));
    };
    let capture_values = (0..captures.len())
        .map(|index| {
            captures
                .get(index)
                .map_or_else(String::new, |value| value.as_str().to_string())
        })
        .collect::<Vec<_>>();
    let request_url = render_template(&rule.request.url, &capture_values, &variables);
    let headers = rule
        .request
        .headers
        .iter()
        .map(|(name, value)| {
            (
                name.clone(),
                render_template(value, &capture_values, &variables),
            )
        })
        .collect::<Vec<_>>();
    let prepared = policy
        .prepare_request(&request_url, headers, &plugin_cookie_store(&app))
        .map_err(|error| AppError::InvalidInput(policy_message(&error)))?;
    let mut request = http_client(Duration::from_secs(20))?.request(
        rule.request
            .method
            .parse()
            .map_err(|_| AppError::InvalidInput("Invalid plugin HTTP method".into()))?,
        prepared.url,
    );
    for (name, value) in prepared.headers {
        request = request.header(name, value);
    }
    if let Some(body) = rule.request.body.as_deref() {
        request = request.body(render_template(body, &capture_values, &variables));
    }
    let response = request
        .send()
        .await
        .map_err(|error| AppError::Updater(format!("Plugin request failed: {error}")))?;
    let status = response.status();
    let bytes = response
        .bytes()
        .await
        .map_err(|error| AppError::Updater(format!("Plugin response failed: {error}")))?;
    if bytes.len() > MAX_HTTP_RESPONSE_BYTES {
        return Err(AppError::InvalidInput(
            "Plugin response is too large".into(),
        ));
    }
    let body = String::from_utf8_lossy(&bytes);
    let download_url = extract_download_url(&body, &rule.extract)?;
    Ok(serde_json::json!({
        "status": status.as_u16(),
        "downloadUrl": download_url,
    }))
}

/// Runs a plugin through the real child-process host. Rule plugins are tested
/// with an HTTP request when they declare a `testUrl`; script plugins must
/// follow the JSON line protocol on stdin/stdout.
#[tauri::command]
pub async fn plugin_test(app: AppHandle, file: String) -> Result<Value, AppError> {
    let (path, content, manifest) = load_plugin(&app, &file)?;
    let policy = manifest_policy(&manifest)?;
    let value: Value = serde_json::from_str(&content)
        .map_err(|_| AppError::InvalidInput("Plugin manifest is not valid JSON".into()))?;

    if manifest.runtime == "download" {
        let url = match value["url"].as_str().filter(|url| {
            url.starts_with("https://github.com/shiwushuifeng/Rayload/releases/download/")
        }) {
            Some(url) => url.to_string(),
            None => {
                let repository = value["repository"]
                    .as_str()
                    .filter(|repository| {
                        repository
                            .split_once('/')
                            .is_some_and(|_| !repository.contains(".."))
                    })
                    .ok_or_else(|| {
                        AppError::InvalidInput("Plugin download URL is not allowed".into())
                    })?;
                let asset = value["asset"].as_str().unwrap_or(".apk");
                latest_release_asset(repository, asset).await?
            }
        };
        let filename = url
            .rsplit('/')
            .next()
            .filter(|name| !name.is_empty())
            .ok_or_else(|| AppError::InvalidInput("Plugin download URL has no filename".into()))?;
        return Ok(serde_json::json!({
            "method": "download",
            "downloadUrl": url,
            "filename": filename,
        }));
    }

    if manifest.runtime == "http" {
        let test_url = value["testUrl"]
            .as_str()
            .ok_or_else(|| AppError::InvalidInput("Rule plugin needs a testUrl".into()))?;
        let headers = value["headers"]
            .as_object()
            .map(|headers| {
                headers
                    .iter()
                    .filter_map(|(name, value)| {
                        value
                            .as_str()
                            .map(|value| (name.clone(), value.to_string()))
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let prepared = policy
            .prepare_request(test_url, headers, &plugin_cookie_store(&app))
            .map_err(|error| AppError::InvalidInput(policy_message(&error)))?;
        let mut request = http_client(Duration::from_secs(15))?.get(prepared.url.clone());
        for (name, value) in &prepared.headers {
            request = request.header(name, value);
        }
        let response = request
            .send()
            .await
            .map_err(|error| AppError::Updater(format!("Plugin request failed: {error}")))?;
        let status = response.status();
        let bytes = response
            .bytes()
            .await
            .map_err(|error| AppError::Updater(format!("Plugin response failed: {error}")))?;
        return Ok(serde_json::json!({
            "method": "http",
            "status": status.as_u16(),
            "url": prepared.url.to_string(),
            "permissions": policy.granted(),
            "bytes": bytes.len(),
        }));
    }

    if manifest.runtime == "script" {
        #[cfg(feature = "plugin_engine")]
        {
            let entry = manifest
                .entry
                .as_deref()
                .ok_or_else(|| AppError::InvalidInput("Script plugin has no entry".into()))?;
            let entry = safe_script_name(entry)
                .map_err(|error| AppError::InvalidInput(error.to_string()))?;
            let script = path.parent().unwrap_or(Path::new(".")).join(entry);
            if !script.exists() {
                return Err(AppError::NotFound("Plugin script is missing".into()));
            }
            let mut command = tokio::process::Command::new(shell_path());
            command.arg(script);
            for (name, env_value) in policy.script_environment(&plugin_cookie_store(&app)) {
                command.env(name, env_value);
            }
            let mut host = crate::plugin_host::PluginHost::spawn(command)
                .map_err(|error| AppError::Io(error.to_string()))?;
            host.handshake()
                .await
                .map_err(|error| AppError::InvalidInput(error.to_string()))?;
            return host
                .request(&serde_json::json!({
                    "id": 1,
                    "method": "echo",
                    "plugin": manifest.name,
                }))
                .await
                .map_err(|error| AppError::InvalidInput(error.to_string()));
        }
        #[cfg(not(feature = "plugin_engine"))]
        return Err(AppError::InvalidInput(
            "Plugins require the full build".into(),
        ));
    }

    if manifest.runtime != "echo" {
        return Err(AppError::InvalidInput(
            "This plugin does not support the test runtime".into(),
        ));
    }
    let name = manifest.name;

    #[cfg(feature = "plugin_engine")]
    {
        let mut command = tokio::process::Command::new(shell_path());
        command.args([
            "-c",
            "while IFS= read -r line; do printf '%s\\n' \"$line\"; done",
        ]);
        for (name, env_value) in policy.script_environment(&plugin_cookie_store(&app)) {
            command.env(name, env_value);
        }
        let mut host = crate::plugin_host::PluginHost::spawn(command)
            .map_err(|error| AppError::Io(error.to_string()))?;
        host.handshake()
            .await
            .map_err(|error| AppError::InvalidInput(error.to_string()))?;
        host.request(&serde_json::json!({
            "id": 1,
            "method": "echo",
            "plugin": name,
        }))
        .await
        .map_err(|error| AppError::InvalidInput(error.to_string()))
    }
    #[cfg(not(feature = "plugin_engine"))]
    {
        let _ = name;
        Err(AppError::InvalidInput(
            "Plugins require the full build".into(),
        ))
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
fn shell_path() -> &'static str {
    #[cfg(target_os = "android")]
    {
        "/system/bin/sh"
    }
    #[cfg(target_os = "linux")]
    {
        "/bin/sh"
    }
}

#[cfg(not(any(target_os = "android", target_os = "linux")))]
fn shell_path() -> &'static str {
    "sh"
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn template_renders_captures_and_variables() {
        let captures = vec!["full".into(), "abc".into()];
        let variables = BTreeMap::from([("pwd".to_string(), "1234".to_string())]);
        assert_eq!(
            render_template("https://x.test/{1}?pwd={{pwd}}", &captures, &variables),
            "https://x.test/abc?pwd=1234"
        );
    }

    #[test]
    fn json_path_supports_arrays() {
        let value = json!({ "data": { "list": [{ "url": "https://cdn.test/a" }] } });
        assert_eq!(
            json_path(&value, "data.list.0.url").and_then(Value::as_str),
            Some("https://cdn.test/a")
        );
    }

    #[test]
    fn regex_extraction_uses_the_requested_group() {
        let extraction = RuleExtraction {
            regex: Some("download=(https://[^&]+)".into()),
            group: 1,
            json: None,
        };
        assert_eq!(
            extract_download_url("x download=https://cdn.test/a&y", &extraction).unwrap(),
            "https://cdn.test/a"
        );
    }
}
