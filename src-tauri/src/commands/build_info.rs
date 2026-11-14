//! Compile-time variant of this build.

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildInfo {
    pub variant: &'static str,
    pub lan: bool,
    pub plugin_engine: bool,
    pub fdroid: bool,
}

#[tauri::command]
pub fn build_info() -> BuildInfo {
    BuildInfo {
        variant: if cfg!(feature = "fdroid") {
            "fdroid"
        } else if cfg!(feature = "plugin_engine") {
            "full"
        } else {
            "lite"
        },
        lan: cfg!(feature = "plugin_engine"),
        plugin_engine: cfg!(feature = "plugin_engine"),
        fdroid: cfg!(feature = "fdroid"),
    }
}
