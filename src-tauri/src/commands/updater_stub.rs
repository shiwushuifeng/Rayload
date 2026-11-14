//! F-Droid builds ship no self-update code: F-Droid owns upgrades.
//! Only the command names remain so the shared invoke list stays identical.

use tauri::AppHandle;

use crate::error::AppError;

fn unsupported<T>() -> Result<T, AppError> {
    Err(AppError::Updater(
        "updates are managed by the F-Droid client".into(),
    ))
}

#[tauri::command]
pub async fn check_for_update(
    _app: AppHandle,
    _channel: String,
    _proxy: Option<String>,
) -> Result<Option<serde_json::Value>, AppError> {
    unsupported()
}

#[tauri::command]
pub async fn check_github_update(
    _current_version: String,
    _stable_only: bool,
    _proxy: Option<String>,
) -> Result<serde_json::Value, AppError> {
    unsupported()
}

#[tauri::command]
pub async fn download_update(_app: AppHandle, _version: String) -> Result<(), AppError> {
    unsupported()
}

#[tauri::command]
pub async fn apply_update(_app: AppHandle) -> Result<(), AppError> {
    unsupported()
}

#[tauri::command]
pub fn cancel_update(_app: AppHandle) -> Result<(), AppError> {
    Ok(())
}
