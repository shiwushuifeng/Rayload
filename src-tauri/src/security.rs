//! Headless security audit and download-path containment.
//!
//! The audit mirrors what `rayload-cli security-check` prints. Path helpers are
//! shared so a handoff request can never resolve outside the configured
//! download root.

use std::path::{Component, Path, PathBuf};

/// How serious a finding is. `Fail` means the environment is unsafe as-is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Pass,
    Warn,
    Fail,
}

/// One line of the security report.
#[derive(Debug, Clone)]
pub struct Check {
    pub name: &'static str,
    pub severity: Severity,
    pub detail: String,
    pub advice: &'static str,
}

/// Inputs for [`audit`]. Paths are already resolved by the caller.
#[derive(Debug, Clone)]
pub struct SecuritySettings<'a> {
    pub lan_port: u16,
    pub rpc_port: u16,
    pub lan_username: &'a str,
    pub lan_password: &'a str,
    pub download_dir: &'a Path,
    pub is_root: bool,
}

/// Directories a download must never be able to write into.
#[cfg(unix)]
const SENSITIVE_DIRS: &[&str] = &[
    "/etc", "/root", "/bin", "/sbin", "/usr", "/boot", "/dev", "/proc", "/sys", "/lib", "/lib64",
    "/var/lib", "/System", "/Library", "/private",
];

#[cfg(windows)]
const SENSITIVE_DIRS: &[&str] = &[
    "C:\\Windows",
    "C:\\Program Files",
    "C:\\Program Files (x86)",
    "C:\\ProgramData",
];

/// Runs every check in a stable order so the report is easy to scan.
pub fn audit(settings: &SecuritySettings<'_>) -> Vec<Check> {
    vec![
        lan_listener_check(settings.lan_port),
        rpc_listener_check(settings.rpc_port),
        auth_check(settings),
        download_dir_check(settings.download_dir),
        root_check(settings.is_root),
    ]
}

fn lan_listener_check(port: u16) -> Check {
    match listener_state(port) {
        ListenerState::Wildcard => Check {
            name: "局域网微服务端口",
            severity: Severity::Warn,
            detail: format!("{port} 端口正在监听 0.0.0.0/::，同一网络内的设备都能发起连接。"),
            advice: "仅在可信内网使用；用防火墙限制来源网段，公网暴露时放在反向代理与 TLS 之后。",
        },
        ListenerState::NotListening => Check {
            name: "局域网微服务端口",
            severity: Severity::Warn,
            detail: format!(
                "{port} 端口当前未运行；启动后会监听 0.0.0.0/::，同一网络内的设备都能发起连接。"
            ),
            advice: "仅在可信内网使用；用防火墙限制来源网段，公网暴露时放在反向代理与 TLS 之后。",
        },
        ListenerState::LoopbackOnly => Check {
            name: "局域网微服务端口",
            severity: Severity::Pass,
            detail: format!("{port} 端口仅监听回环地址。"),
            advice: "保持当前配置。",
        },
        ListenerState::Unknown => Check {
            name: "局域网微服务端口",
            severity: Severity::Warn,
            detail: format!("无法读取 {port} 端口状态；该构建默认监听 0.0.0.0/::。"),
            advice: "仅在可信内网使用，并用防火墙限制来源网段。",
        },
    }
}

fn rpc_listener_check(port: u16) -> Check {
    match listener_state(port) {
        ListenerState::Wildcard => Check {
            name: "Aria2 RPC 端口",
            severity: Severity::Fail,
            detail: format!("{port} 端口监听 0.0.0.0/::，而 RPC 没有鉴权，任何人都能添加任务。"),
            advice: "设置 rpc-listen-all=false，或为 RPC 配置 rpc-secret 后再暴露。",
        },
        ListenerState::NotListening => Check {
            name: "Aria2 RPC 端口",
            severity: Severity::Pass,
            detail: format!("{port} 端口当前未运行；启动后仅监听 127.0.0.1。"),
            advice: "保持当前配置。",
        },
        ListenerState::LoopbackOnly => Check {
            name: "Aria2 RPC 端口",
            severity: Severity::Pass,
            detail: format!("{port} 端口仅监听回环地址（127.0.0.1）。"),
            advice: "保持当前配置。",
        },
        ListenerState::Unknown => Check {
            name: "Aria2 RPC 端口",
            severity: Severity::Pass,
            detail: format!("无法读取 {port} 端口状态；该构建默认仅监听 127.0.0.1。"),
            advice: "保持当前配置。",
        },
    }
}

fn auth_check(settings: &SecuritySettings<'_>) -> Check {
    let configured =
        !settings.lan_username.trim().is_empty() && !settings.lan_password.trim().is_empty();
    if configured {
        Check {
            name: "局域网管理鉴权",
            severity: Severity::Pass,
            detail: "已配置管理账号与密码。".into(),
            advice: "定期更换密码，不要与其他服务共用。",
        }
    } else {
        Check {
            name: "局域网管理鉴权",
            severity: Severity::Fail,
            detail: "未配置 6767 管理账号密码，任何人都能在同一网络内连接。".into(),
            advice: "设置 RAYLOAD_LAN_USERNAME 与 RAYLOAD_LAN_PASSWORD 后再启动局域网服务。",
        }
    }
}

fn download_dir_check(dir: &Path) -> Check {
    if is_sensitive_path(dir) {
        Check {
            name: "下载目录",
            severity: Severity::Fail,
            detail: format!(
                "下载目录 {} 位于系统敏感路径，恶意任务可能覆盖系统文件。",
                dir.display()
            ),
            advice: "把下载目录改到专用目录，例如 /srv/rayload/downloads 或容器内的 /downloads。",
        }
    } else {
        Check {
            name: "下载目录",
            severity: Severity::Pass,
            detail: format!("下载目录 {} 不在系统目录内。", dir.display()),
            advice: "保持当前配置。",
        }
    }
}

fn root_check(is_root: bool) -> Check {
    if is_root {
        Check {
            name: "运行权限",
            severity: Severity::Warn,
            detail: "当前进程以 root 运行，被利用时影响范围是整个系统。".into(),
            advice: "改用普通用户运行，或用 Docker 隔离（镜像内已使用非特权 rayload 用户）。",
        }
    } else {
        Check {
            name: "运行权限",
            severity: Severity::Pass,
            detail: "当前进程不是 root。".into(),
            advice: "保持当前配置。",
        }
    }
}

/// True when `path` is a filesystem root or lives inside a system directory.
pub fn is_sensitive_path(path: &Path) -> bool {
    let normalized = normalize(path);
    if normalized.parent().is_none() {
        return true;
    }
    SENSITIVE_DIRS.iter().any(|entry| {
        let sensitive = normalize(Path::new(entry));
        normalized == sensitive || normalized.starts_with(&sensitive)
    })
}

/// Resolves a client-supplied file name inside `root`.
///
/// Only a single normal path segment is accepted, so `../`, absolute paths and
/// nested directories are rejected even if an upstream validator is bypassed.
pub fn resolve_download_path(root: &Path, filename: &str) -> Result<PathBuf, String> {
    let name = filename.trim();
    if name.is_empty() {
        return Err("download file name is empty".into());
    }
    let candidate = Path::new(name);
    let mut segments = 0;
    for component in candidate.components() {
        match component {
            Component::Normal(_) => segments += 1,
            _ => return Err("download path must stay inside the download root".into()),
        }
    }
    if segments != 1 {
        return Err("download path must be a single file name".into());
    }
    let resolved = normalize(root).join(candidate);
    if !resolved.starts_with(normalize(root)) {
        return Err("download path must stay inside the download root".into());
    }
    Ok(resolved)
}

/// Lexical normalization without touching the filesystem.
fn normalize(path: &Path) -> PathBuf {
    let mut output = PathBuf::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                output.pop();
            }
            Component::CurDir => {}
            other => output.push(other.as_os_str()),
        }
    }
    output
}

/// Observed state of a TCP port for the report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ListenerState {
    Wildcard,
    LoopbackOnly,
    NotListening,
    #[cfg_attr(target_os = "linux", allow(dead_code))]
    Unknown,
}

/// Reads listener sockets from `/proc` on Linux; other platforms report
/// `Unknown` so the report falls back to the build defaults.
#[cfg(target_os = "linux")]
fn listener_state(port: u16) -> ListenerState {
    let mut listening = false;
    let mut wildcard = false;
    for table in ["/proc/net/tcp", "/proc/net/tcp6"] {
        let Ok(content) = std::fs::read_to_string(table) else {
            continue;
        };
        for line in content.lines().skip(1) {
            let columns = line.split_whitespace().collect::<Vec<_>>();
            let Some(local) = columns.get(1) else {
                continue;
            };
            let Some(state) = columns.get(3) else {
                continue;
            };
            if *state != "0A" {
                continue;
            }
            let Some((address, raw_port)) = local.split_once(':') else {
                continue;
            };
            if u16::from_str_radix(raw_port, 16).ok() != Some(port) {
                continue;
            }
            listening = true;
            if address.chars().all(|ch| ch == '0') {
                wildcard = true;
            }
        }
    }
    match (listening, wildcard) {
        (true, true) => ListenerState::Wildcard,
        (true, false) => ListenerState::LoopbackOnly,
        (false, _) => ListenerState::NotListening,
    }
}

#[cfg(not(target_os = "linux"))]
fn listener_state(_port: u16) -> ListenerState {
    ListenerState::Unknown
}

/// `true` on Unix when the effective user id is 0.
#[cfg(unix)]
pub fn running_as_root() -> bool {
    // SAFETY: geteuid has no preconditions and cannot fail.
    unsafe { libc::geteuid() == 0 }
}

#[cfg(not(unix))]
pub fn running_as_root() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_and_system_paths_are_sensitive() {
        assert!(is_sensitive_path(Path::new("/")));
        assert!(is_sensitive_path(Path::new("/etc")));
        assert!(is_sensitive_path(Path::new("/etc/rayload/downloads")));
        assert!(is_sensitive_path(Path::new("/usr/local/share/rayload")));
        assert!(!is_sensitive_path(Path::new("/home/alice/downloads")));
        assert!(!is_sensitive_path(Path::new("/srv/rayload")));
    }

    #[test]
    fn traversal_and_absolute_names_are_rejected() {
        let root = Path::new("/srv/rayload/downloads");
        assert!(resolve_download_path(root, "../etc/passwd").is_err());
        assert!(resolve_download_path(root, "/etc/passwd").is_err());
        assert!(resolve_download_path(root, "sub/file.bin").is_err());
        assert!(resolve_download_path(root, "..").is_err());
        assert!(resolve_download_path(root, "").is_err());
        assert_eq!(
            resolve_download_path(root, "file.bin").unwrap(),
            PathBuf::from("/srv/rayload/downloads/file.bin")
        );
    }

    #[test]
    fn audit_requires_credentials_and_marks_root() {
        let settings = SecuritySettings {
            lan_port: 6767,
            rpc_port: 6800,
            lan_username: "",
            lan_password: "",
            download_dir: Path::new("/etc"),
            is_root: true,
        };
        let checks = audit(&settings);
        assert_eq!(checks.len(), 5);
        assert_eq!(checks[2].severity, Severity::Fail);
        assert_eq!(checks[3].severity, Severity::Fail);
        assert_eq!(checks[4].severity, Severity::Warn);
    }
}
