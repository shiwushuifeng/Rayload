use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tauri::AppHandle;

#[cfg(test)]
const SERVER_MET_FILE: &str = "server.met";
#[cfg(test)]
const NODES_DAT_FILE: &str = "nodes.dat";
const BUNDLED_SERVER_MET_PATH: &str = "data/ed2k-bootstrap/server.met";
const BUNDLED_NODES_DAT_PATH: &str = "data/ed2k-bootstrap/nodes.dat";
const MAX_BOOTSTRAP_FILE_SIZE: u64 = 16 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ed2kBootstrapStatus {
    pub server_met_size: Option<u64>,
    pub nodes_dat_size: Option<u64>,
    pub server_met_modified: Option<u64>,
    pub nodes_dat_modified: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ed2kBootstrapSyncResult {
    pub status: Ed2kBootstrapStatus,
}

pub(crate) fn ed2k_bootstrap_paths(app: &AppHandle) -> Result<(PathBuf, PathBuf), AppError> {
    Ok((
        crate::resources::resolve_resource(app, BUNDLED_SERVER_MET_PATH)?,
        crate::resources::resolve_resource(app, BUNDLED_NODES_DAT_PATH)?,
    ))
}

pub(crate) fn remove_ed2k_bootstrap_cache(app: &AppHandle) -> Result<(), AppError> {
    let (server, _) = ed2k_bootstrap_paths(app)?;
    let Some(directory) = server.parent() else {
        return Ok(());
    };
    if directory.exists() {
        std::fs::remove_dir_all(directory)?;
    }
    Ok(())
}

fn modified_millis(path: &Path) -> Option<u64> {
    let modified = std::fs::metadata(path).ok()?.modified().ok()?;
    modified
        .duration_since(std::time::UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_millis() as u64)
}

pub(crate) fn ed2k_bootstrap_status_from_paths(
    server_met_path: &Path,
    nodes_dat_path: &Path,
) -> Ed2kBootstrapStatus {
    Ed2kBootstrapStatus {
        server_met_size: std::fs::metadata(server_met_path).ok().map(|m| m.len()),
        nodes_dat_size: std::fs::metadata(nodes_dat_path).ok().map(|m| m.len()),
        server_met_modified: modified_millis(server_met_path),
        nodes_dat_modified: modified_millis(nodes_dat_path),
    }
}

pub(crate) fn inject_ed2k_bootstrap_options(
    options: &mut serde_json::Value,
    server_met_path: &Path,
    nodes_dat_path: &Path,
) {
    let obj = if let Some(obj) = options.as_object_mut() {
        obj
    } else {
        *options = serde_json::Value::Object(serde_json::Map::new());
        options
            .as_object_mut()
            .expect("options was normalized to object")
    };
    obj.entry("ed2k-server-list").or_insert_with(|| {
        serde_json::Value::String(crate::engine::path_to_safe_string(server_met_path))
    });
    obj.entry("ed2k-node-list").or_insert_with(|| {
        serde_json::Value::String(crate::engine::path_to_safe_string(nodes_dat_path))
    });
}

pub(crate) fn inject_managed_ed2k_bootstrap_options(
    app: &AppHandle,
    options: &mut serde_json::Value,
) -> Result<(), AppError> {
    ensure_ed2k_bootstrap_cache(app)?;
    let (server_met_path, nodes_dat_path) = ed2k_bootstrap_paths(app)?;
    inject_ed2k_bootstrap_options(options, &server_met_path, &nodes_dat_path);
    Ok(())
}

pub(crate) fn ensure_ed2k_bootstrap_cache(app: &AppHandle) -> Result<(String, String), AppError> {
    let (server_met_path, nodes_dat_path) = ed2k_bootstrap_paths(app)?;

    Ok((
        crate::engine::path_to_safe_string(&server_met_path),
        crate::engine::path_to_safe_string(&nodes_dat_path),
    ))
}

#[tauri::command]
pub async fn get_ed2k_bootstrap_status(app: AppHandle) -> Result<Ed2kBootstrapStatus, AppError> {
    ensure_ed2k_bootstrap_cache(&app)?;
    let (server_met_path, nodes_dat_path) = ed2k_bootstrap_paths(&app)?;
    Ok(ed2k_bootstrap_status_from_paths(
        &server_met_path,
        &nodes_dat_path,
    ))
}

#[tauri::command]
pub async fn sync_ed2k_bootstrap_files(
    app: AppHandle,
    server_met_url: String,
    nodes_dat_url: String,
    proxy: Option<String>,
) -> Result<Ed2kBootstrapSyncResult, AppError> {
    let (server_met_path, nodes_dat_path) = ed2k_bootstrap_paths(&app)?;
    let dir = server_met_path
        .parent()
        .ok_or_else(|| AppError::Io("Invalid ED2K bootstrap cache path".into()))?;
    std::fs::create_dir_all(dir)?;

    let server_met = download_bootstrap_file(&app, &server_met_url, &proxy).await?;
    let nodes_dat = download_bootstrap_file(&app, &nodes_dat_url, &proxy).await?;
    write_cache_file(&server_met_path, &server_met)?;
    write_cache_file(&nodes_dat_path, &nodes_dat)?;

    Ok(Ed2kBootstrapSyncResult {
        status: ed2k_bootstrap_status_from_paths(&server_met_path, &nodes_dat_path),
    })
}

async fn download_bootstrap_file(
    app: &AppHandle,
    url: &str,
    proxy: &Option<String>,
) -> Result<Vec<u8>, AppError> {
    validate_bootstrap_url(url)?;
    let request = crate::services::engine_fetch::FetchRequest::small(
        url,
        proxy.clone(),
        MAX_BOOTSTRAP_FILE_SIZE,
    );
    let bytes = crate::services::engine_fetch::fetch(app, &request)
        .await
        .map_err(|e| AppError::Io(format!("Failed to fetch ED2K bootstrap file: {e}")))?;
    if bytes.is_empty() {
        return Err(AppError::Io("Invalid ED2K bootstrap file size: 0".into()));
    }
    Ok(bytes)
}

fn write_cache_file(target: &Path, bytes: &[u8]) -> Result<(), AppError> {
    let tmp = target.with_extension("tmp");
    std::fs::write(&tmp, bytes)?;
    if target.exists() {
        std::fs::remove_file(target)?;
    }
    std::fs::rename(&tmp, target)?;
    Ok(())
}

fn validate_bootstrap_url(value: &str) -> Result<(), AppError> {
    let url = url::Url::parse(value)
        .map_err(|e| AppError::Io(format!("Invalid ED2K bootstrap URL: {e}")))?;
    if matches!(url.scheme(), "http" | "https") {
        Ok(())
    } else {
        Err(AppError::Io(
            "ED2K bootstrap URL must use HTTP or HTTPS".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn status_reads_existing_cache_files() {
        let dir = tempdir().expect("temp dir");
        let server = dir.path().join(SERVER_MET_FILE);
        let nodes = dir.path().join(NODES_DAT_FILE);
        std::fs::write(&server, b"server").expect("write server");
        std::fs::write(&nodes, b"nodes").expect("write nodes");

        let status = ed2k_bootstrap_status_from_paths(&server, &nodes);

        assert_eq!(status.server_met_size, Some(6));
        assert_eq!(status.nodes_dat_size, Some(5));
        assert!(status.server_met_modified.is_some());
        assert!(status.nodes_dat_modified.is_some());
    }

    #[test]
    fn validates_only_http_bootstrap_urls() {
        assert!(validate_bootstrap_url("https://upd.emule-security.org/server.met").is_ok());
        assert!(validate_bootstrap_url("http://upd.emule-security.org/nodes.dat").is_ok());
        assert!(validate_bootstrap_url("file:///tmp/server.met").is_err());
        assert!(validate_bootstrap_url("not-a-url").is_err());
    }

    #[test]
    fn injects_ed2k_bootstrap_options_without_overriding_explicit_values() {
        let mut options = serde_json::json!({
            "ed2k-server-list": "/explicit/server.met"
        });
        inject_ed2k_bootstrap_options(
            &mut options,
            Path::new("/cache/server.met"),
            Path::new("/cache/nodes.dat"),
        );

        assert_eq!(options["ed2k-server-list"], "/explicit/server.met");
        assert_eq!(options["ed2k-node-list"], "/cache/nodes.dat");
    }
}
