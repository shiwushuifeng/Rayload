//! YAML configuration loading with CLI > environment > file > default priority.

use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub const DEFAULT_LAN_PORT: u16 = 6767;
pub const DEFAULT_RPC_PORT: u16 = 6800;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ConfigFile {
    pub device: DeviceConfig,
    pub lan: LanConfig,
    pub download: DownloadConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DeviceConfig {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LanConfig {
    pub enabled: bool,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub require_manual_confirm: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DownloadConfig {
    pub dir: PathBuf,
    pub engine_bin: PathBuf,
    pub rpc_port: u16,
    /// Raw aria2 options; managed options such as `dir` still win.
    #[serde(
        default,
        deserialize_with = "deserialize_options",
        serialize_with = "serialize_options"
    )]
    pub aria2_options: BTreeMap<String, String>,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            device: DeviceConfig::default(),
            lan: LanConfig::default(),
            download: DownloadConfig::default(),
        }
    }
}

impl Default for DeviceConfig {
    fn default() -> Self {
        Self {
            name: crate::api_server::device_name(),
        }
    }
}

impl Default for LanConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            port: DEFAULT_LAN_PORT,
            username: String::new(),
            password: String::new(),
            require_manual_confirm: false,
        }
    }
}

impl Default for DownloadConfig {
    fn default() -> Self {
        let defaults = [
            ("max-concurrent-downloads", "5"),
            ("max-connection-per-server", "16"),
            ("split", "16"),
        ];
        Self {
            dir: std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("downloads"),
            engine_bin: PathBuf::from("aria2-next"),
            rpc_port: DEFAULT_RPC_PORT,
            aria2_options: defaults
                .into_iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Overrides {
    pub config: Option<PathBuf>,
    pub data_dir: Option<PathBuf>,
    pub download_dir: Option<PathBuf>,
    pub engine_bin: Option<PathBuf>,
    pub device_name: Option<String>,
    pub lan_port: Option<u16>,
    pub rpc_port: Option<u16>,
    pub lan_username: Option<String>,
    pub lan_password: Option<String>,
    pub require_manual_confirm: Option<bool>,
    pub no_lan: bool,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub config_path: PathBuf,
    pub data_dir: PathBuf,
    pub device_name: String,
    pub lan_enabled: bool,
    pub lan_port: u16,
    pub lan_username: String,
    pub lan_password: String,
    pub require_manual_confirm: bool,
    pub download_dir: PathBuf,
    pub engine_bin: PathBuf,
    pub rpc_port: u16,
    pub aria2_options: BTreeMap<String, String>,
}

impl Settings {
    pub fn lan_ready(&self) -> bool {
        self.lan_enabled && !self.lan_username.trim().is_empty() && !self.lan_password.is_empty()
    }

    pub fn as_config_file(&self) -> ConfigFile {
        ConfigFile {
            device: DeviceConfig {
                name: self.device_name.clone(),
            },
            lan: LanConfig {
                enabled: self.lan_enabled,
                port: self.lan_port,
                username: self.lan_username.clone(),
                password: self.lan_password.clone(),
                require_manual_confirm: self.require_manual_confirm,
            },
            download: DownloadConfig {
                dir: self.download_dir.clone(),
                engine_bin: self.engine_bin.clone(),
                rpc_port: self.rpc_port,
                aria2_options: self.aria2_options.clone(),
            },
        }
    }

    pub fn pid_path(&self) -> PathBuf {
        self.data_dir.join("rayload.pid")
    }

    pub fn log_path(&self) -> PathBuf {
        self.data_dir.join("rayload.log")
    }
}

pub fn default_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from(".").join(".config"))
        .join("rayload")
        .join("config.yml")
}

pub fn default_data_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from(".").join(".local/share"))
        .join("rayload-cli")
}

pub fn load(overrides: &Overrides) -> Result<Settings, String> {
    let config_path = overrides.config.clone().unwrap_or_else(default_config_path);
    let file = read_file(&config_path)?;
    let download_dir = overrides
        .download_dir
        .clone()
        .map(|path| expand_tilde(&path))
        .unwrap_or_else(|| file.download.dir.clone());
    let settings = Settings {
        config_path,
        data_dir: overrides.data_dir.clone().unwrap_or_else(default_data_dir),
        device_name: overrides
            .device_name
            .clone()
            .unwrap_or_else(|| file.device.name.clone()),
        lan_enabled: !overrides.no_lan && file.lan.enabled,
        lan_port: overrides.lan_port.unwrap_or(file.lan.port),
        lan_username: overrides
            .lan_username
            .clone()
            .unwrap_or_else(|| file.lan.username.clone()),
        lan_password: overrides
            .lan_password
            .clone()
            .unwrap_or_else(|| file.lan.password.clone()),
        require_manual_confirm: overrides
            .require_manual_confirm
            .unwrap_or(file.lan.require_manual_confirm),
        download_dir,
        engine_bin: overrides
            .engine_bin
            .clone()
            .unwrap_or_else(|| file.download.engine_bin.clone()),
        rpc_port: overrides.rpc_port.unwrap_or(file.download.rpc_port),
        aria2_options: file.download.aria2_options,
    };
    Ok(settings)
}

pub fn read_file(path: &Path) -> Result<ConfigFile, String> {
    match fs::read_to_string(path) {
        Ok(content) => {
            serde_yaml::from_str(&content).map_err(|error| format!("{}: {error}", path.display()))
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(ConfigFile::default()),
        Err(error) => Err(format!("{}: {error}", path.display())),
    }
}

pub fn save_file(path: &Path, file: &ConfigFile) -> Result<(), String> {
    let yaml = serde_yaml::to_string(file).map_err(|error| error.to_string())?;
    write_secure(path, &format!("# Rayload CLI configuration\n{yaml}"))
}

pub fn write_secure(path: &Path, content: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(path)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        file.write_all(content.as_bytes())
            .map_err(|error| format!("{}: {error}", path.display()))?;
    }
    #[cfg(not(unix))]
    fs::write(path, content).map_err(|error| format!("{}: {error}", path.display()))?;
    Ok(())
}

/// Changes one dotted YAML key while preserving unrelated fields.
pub fn set_value(path: &Path, dotted: &str, raw: &str) -> Result<(), String> {
    let mut document = if path.exists() {
        let content = fs::read_to_string(path).map_err(|error| error.to_string())?;
        serde_yaml::from_str::<serde_yaml::Value>(&content).map_err(|error| error.to_string())?
    } else {
        serde_yaml::to_value(ConfigFile::default()).map_err(|error| error.to_string())?
    };
    let keys: Vec<&str> = dotted.split('.').filter(|key| !key.is_empty()).collect();
    if keys.is_empty() {
        return Err("empty configuration key".into());
    }
    let mut cursor = &mut document;
    for key in &keys[..keys.len() - 1] {
        let mapping = cursor
            .as_mapping_mut()
            .ok_or_else(|| format!("{key} is not a mapping"))?;
        cursor = mapping
            .entry(serde_yaml::Value::String((*key).to_string()))
            .or_insert_with(|| serde_yaml::Value::Mapping(Default::default()));
    }
    let last = keys[keys.len() - 1];
    let mapping = cursor
        .as_mapping_mut()
        .ok_or_else(|| format!("{last} is not a mapping"))?;
    mapping.insert(
        serde_yaml::Value::String(last.to_string()),
        parse_scalar(raw),
    );
    let yaml = serde_yaml::to_string(&document).map_err(|error| error.to_string())?;
    write_secure(path, &format!("# Rayload CLI configuration\n{yaml}"))
}

pub fn get_value(path: &Path, dotted: &str) -> Result<String, String> {
    let file = read_file(path)?;
    let document = serde_yaml::to_value(file).map_err(|error| error.to_string())?;
    let mut cursor = &document;
    for key in dotted.split('.').filter(|key| !key.is_empty()) {
        cursor = cursor
            .get(key)
            .ok_or_else(|| format!("{dotted} is not set"))?;
    }
    if cursor.is_string() {
        Ok(cursor.as_str().unwrap_or_default().to_string())
    } else {
        serde_yaml::to_string(cursor)
            .map(|value| value.trim_end().to_string())
            .map_err(|error| error.to_string())
    }
}

fn parse_scalar(raw: &str) -> serde_yaml::Value {
    if raw.eq_ignore_ascii_case("true") {
        return serde_yaml::Value::Bool(true);
    }
    if raw.eq_ignore_ascii_case("false") {
        return serde_yaml::Value::Bool(false);
    }
    if let Ok(number) = raw.parse::<i64>() {
        return serde_yaml::Value::Number(number.into());
    }
    serde_yaml::Value::String(raw.to_string())
}

fn deserialize_options<'de, D>(deserializer: D) -> Result<BTreeMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = BTreeMap::<String, serde_yaml::Value>::deserialize(deserializer)?;
    raw.into_iter()
        .map(|(key, value)| {
            let value = match value {
                serde_yaml::Value::String(value) => value,
                serde_yaml::Value::Number(value) => value.to_string(),
                serde_yaml::Value::Bool(value) => value.to_string(),
                _ => {
                    return Err(serde::de::Error::custom(format!(
                        "aria2_options.{key} must be a scalar"
                    )))
                }
            };
            Ok((key, value))
        })
        .collect()
}

fn serialize_options<S>(
    options: &BTreeMap<String, String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let raw: BTreeMap<String, serde_yaml::Value> = options
        .iter()
        .map(|(key, value)| (key.clone(), parse_scalar(value)))
        .collect();
    raw.serialize(serializer)
}

pub fn expand_tilde(path: &Path) -> PathBuf {
    let text = path.to_string_lossy();
    if let Some(rest) = text.strip_prefix('~') {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest.trim_start_matches('/'));
        }
    }
    path.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_values_and_cli_overrides_follow_the_documented_priority() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("config.yml");
        save_file(
            &path,
            &ConfigFile {
                lan: LanConfig {
                    port: 7000,
                    username: "file-user".into(),
                    password: "file-pass".into(),
                    ..LanConfig::default()
                },
                download: DownloadConfig {
                    dir: PathBuf::from("/srv/file"),
                    ..DownloadConfig::default()
                },
                ..ConfigFile::default()
            },
        )
        .expect("save");

        let settings = load(&Overrides {
            config: Some(path),
            download_dir: Some(PathBuf::from("/srv/cli")),
            lan_port: Some(8000),
            ..Overrides::default()
        })
        .expect("load");

        assert_eq!(settings.download_dir, PathBuf::from("/srv/cli"));
        assert_eq!(settings.lan_port, 8000);
        assert_eq!(settings.lan_username, "file-user");
        assert_eq!(settings.lan_password, "file-pass");
    }

    #[test]
    fn dotted_updates_round_trip_through_yaml() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("config.yml");
        set_value(
            &path,
            "download.aria2_options.max-concurrent-downloads",
            "12",
        )
        .expect("set number");
        set_value(&path, "lan.require_manual_confirm", "true").expect("set bool");
        let file = read_file(&path).expect("read");
        assert_eq!(
            file.download.aria2_options.get("max-concurrent-downloads"),
            Some(&"12".to_string())
        );
        assert!(file.lan.require_manual_confirm);
        assert_eq!(get_value(&path, "lan.port").expect("get"), "6767");
    }

    #[test]
    fn numeric_aria2_options_from_the_sample_parse_as_strings() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("config.yml");
        fs::write(
            &path,
            "download:\n  aria2_options:\n    max-concurrent-downloads: 5\n    split: 16\n",
        )
        .expect("write");
        let file = read_file(&path).expect("read");
        assert_eq!(
            file.download.aria2_options.get("max-concurrent-downloads"),
            Some(&"5".to_string())
        );
    }
}
