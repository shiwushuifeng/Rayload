//! Mobile substitutes for desktop tray and window-activation APIs.

use std::sync::Mutex;

use tauri::{AppHandle, Manager};

use crate::services::float_window::Mode as FloatMode;

#[derive(Default)]
pub struct MainWindowState(pub Mutex<()>);

#[derive(Default)]
pub struct TrayMenuState;

impl TrayMenuState {
    pub fn set_labels(&self, _labels: &serde_json::Value) {}

    pub fn set_float_mode(&self, _mode: FloatMode) {}
}

pub fn setup_tray(_app: &AppHandle) -> Result<TrayMenuState, Box<dyn std::error::Error>> {
    Ok(TrayMenuState)
}

pub fn request_main_window(app: &AppHandle, source: &'static str, visible: bool) {
    let Some(window) = app.get_webview_window("main") else {
        log::warn!("window:request-failed source={source} reason=no-main-window");
        return;
    };
    if visible {
        if let Err(error) = window.show() {
            log::warn!("window:show-failed source={source} error={error}");
        }
        if let Err(error) = window.set_focus() {
            log::warn!("window:focus-failed source={source} error={error}");
        }
    }
}

pub fn set_float_mode(_app: &AppHandle, _mode: FloatMode) {}

pub fn set_title(_app: &AppHandle, _title: &str) -> tauri::Result<()> {
    Ok(())
}

pub fn set_visible(_app: &AppHandle, _visible: bool) -> tauri::Result<()> {
    Ok(())
}

pub fn set_activity(_app: &AppHandle, _active: bool, _tooltip: &str) -> tauri::Result<()> {
    Ok(())
}

pub fn icon_size() -> u32 {
    0
}

pub fn dispatch_menu_action(_app: &AppHandle, _id: &str) {}
