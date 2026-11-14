//! Implementations for the ray/rayload subcommands.

use std::fs;
use std::io::{self, IsTerminal, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use aria2_service::{Aria2Config, Aria2Service};
use comfy_table::{Cell, Color, ContentArrangement, Table};
use dialoguer::{Confirm, Input, Password};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use super::api::{DeviceInfo, LocalApi};
use super::config::{self, ConfigFile, LanConfig, Overrides, Settings};
use super::style::{format_speed, render_tree, Node, Styles};
use crate::aria2::rpc::RpcClient;
use crate::aria2::types::Aria2Task;
use crate::headless::{self, LanServeOptions};

pub fn settings(overrides: &Overrides) -> Result<Settings, String> {
    let settings = config::load(overrides)?;
    if settings.rpc_port == 0 || settings.lan_port == 0 {
        return Err("ports must be greater than zero".into());
    }
    fs::create_dir_all(&settings.data_dir)
        .map_err(|error| format!("{}: {error}", settings.data_dir.display()))?;
    fs::create_dir_all(&settings.download_dir)
        .map_err(|error| format!("{}: {error}", settings.download_dir.display()))?;
    Ok(settings)
}

pub fn init(overrides: &Overrides, force: bool) -> Result<(), String> {
    ensure_interactive("ray init")?;
    let path = overrides
        .config
        .clone()
        .unwrap_or_else(config::default_config_path);
    let existing = config::read_file(&path).unwrap_or_default();
    if path.exists() && !force {
        let overwrite = Confirm::new()
            .with_prompt(format!("{} already exists. Overwrite?", path.display()))
            .default(false)
            .interact()
            .map_err(|error| error.to_string())?;
        if !overwrite {
            println!("Configuration left unchanged.");
            return Ok(());
        }
    }

    let defaults = if path.exists() {
        existing
    } else {
        ConfigFile::default()
    };
    let download_dir: String = Input::new()
        .with_prompt("Download directory")
        .default(defaults.download.dir.display().to_string())
        .interact_text()
        .map_err(|error| error.to_string())?;
    let device_name: String = Input::new()
        .with_prompt("Device name")
        .default(defaults.device.name.clone())
        .interact_text()
        .map_err(|error| error.to_string())?;
    let lan_enabled = Confirm::new()
        .with_prompt("Enable the LAN API?")
        .default(defaults.lan.enabled)
        .interact()
        .map_err(|error| error.to_string())?;
    let (lan_port, username, password, require_manual_confirm) = if lan_enabled {
        let port: u16 = Input::new()
            .with_prompt("LAN API port")
            .default(defaults.lan.port)
            .interact_text()
            .map_err(|error| error.to_string())?;
        let username: String = Input::new()
            .with_prompt("LAN admin username")
            .default(defaults.lan.username.clone())
            .interact_text()
            .map_err(|error| error.to_string())?;
        if username.trim().is_empty() {
            return Err("LAN admin username cannot be empty".into());
        }
        let password = Password::new()
            .with_prompt("LAN admin password")
            .with_confirmation("Repeat password", "Passwords do not match")
            .interact()
            .map_err(|error| error.to_string())?;
        if password.is_empty() {
            return Err("LAN admin password cannot be empty".into());
        }
        let manual = Confirm::new()
            .with_prompt("Require manual approval for new pairings?")
            .default(defaults.lan.require_manual_confirm)
            .interact()
            .map_err(|error| error.to_string())?;
        (port, username, password, manual)
    } else {
        (defaults.lan.port, String::new(), String::new(), false)
    };
    let max_concurrent = ask_aria2_number(
        "Max concurrent downloads",
        "max-concurrent-downloads",
        &defaults,
        5,
    )?;
    let max_per_server = ask_aria2_number(
        "Max connections per server",
        "max-connection-per-server",
        &defaults,
        16,
    )?;
    let split = ask_aria2_number("Split parts per download", "split", &defaults, 16)?;

    let mut file = ConfigFile {
        device: config::DeviceConfig {
            name: device_name.trim().to_string(),
        },
        lan: LanConfig {
            enabled: lan_enabled,
            port: lan_port,
            username: username.trim().to_string(),
            password,
            require_manual_confirm,
        },
        download: config::DownloadConfig {
            dir: config::expand_tilde(Path::new(&download_dir)),
            engine_bin: defaults.download.engine_bin.clone(),
            rpc_port: defaults.download.rpc_port,
            aria2_options: defaults.download.aria2_options.clone(),
        },
    };
    file.download.aria2_options.insert(
        "max-concurrent-downloads".into(),
        max_concurrent.to_string(),
    );
    file.download.aria2_options.insert(
        "max-connection-per-server".into(),
        max_per_server.to_string(),
    );
    file.download
        .aria2_options
        .insert("split".into(), split.to_string());
    config::save_file(&path, &file)?;
    println!("Wrote {}", path.display());
    println!("Start it with: ray start");
    Ok(())
}

fn ask_aria2_number(
    prompt: &str,
    key: &str,
    defaults: &ConfigFile,
    fallback: u32,
) -> Result<u32, String> {
    let default = defaults
        .download
        .aria2_options
        .get(key)
        .and_then(|value| value.parse().ok())
        .unwrap_or(fallback);
    Input::new()
        .with_prompt(prompt)
        .default(default)
        .interact_text()
        .map_err(|error| error.to_string())
}

pub fn start(overrides: &Overrides, daemon: bool) -> Result<(), String> {
    let settings = settings(overrides)?;
    if daemon {
        return spawn_daemon(&settings, overrides);
    }
    run_foreground(settings)
}

fn run_foreground(settings: Settings) -> Result<(), String> {
    if crate::security::is_sensitive_path(&settings.download_dir) {
        return Err(format!(
            "refusing to use {} as the download directory",
            settings.download_dir.display()
        ));
    }
    if crate::security::running_as_root() {
        eprintln!("warning: running as root; prefer a dedicated user or container");
    }
    let service = Aria2Service::new(Aria2Config {
        binary: settings.engine_bin.clone(),
        data_dir: settings.data_dir.clone(),
        download_dir: settings.download_dir.clone(),
        rpc_port: settings.rpc_port,
        extra_options: settings.aria2_options.clone(),
    });
    let mut process = service.start().map_err(|error| error.to_string())?;
    process
        .wait_until_ready()
        .map_err(|error| error.to_string())?;
    write_pid(&settings, process.pid())?;
    let _guard = PidGuard {
        path: settings.pid_path(),
    };

    println!("aria2 rpc: http://127.0.0.1:{}/jsonrpc", settings.rpc_port);
    println!("downloads: {}", settings.download_dir.display());
    println!("pid: {}", process.pid());

    if settings.lan_enabled && !settings.lan_ready() {
        eprintln!("warning: LAN API stays off until lan.username and lan.password are configured");
    } else if settings.lan_ready() {
        let lan_settings = settings.clone();
        std::thread::Builder::new()
            .name("rayload-lan-api".into())
            .spawn(move || {
                let runtime = match tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                {
                    Ok(runtime) => runtime,
                    Err(error) => {
                        eprintln!("ray: LAN runtime failed: {error}");
                        return;
                    }
                };
                if let Err(error) = runtime.block_on(headless::serve_lan(
                    &lan_settings.data_dir,
                    &lan_settings.download_dir,
                    LanServeOptions {
                        rpc_port: lan_settings.rpc_port,
                        api_port: lan_settings.lan_port,
                        device_name: lan_settings.device_name.clone(),
                        username: lan_settings.lan_username.clone(),
                        password: lan_settings.lan_password.clone(),
                        require_manual_confirm: lan_settings.require_manual_confirm,
                        interactive: true,
                    },
                )) {
                    eprintln!("ray: LAN API failed: {error}");
                }
            })
            .map_err(|error| error.to_string())?;
        let host = crate::lan_discovery::local_ipv4()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| "127.0.0.1".into());
        println!("lan api: http://{host}:{}", settings.lan_port);
    }

    let status = process.wait().map_err(|error| error.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("aria2 exited with {status}"))
    }
}

fn spawn_daemon(settings: &Settings, overrides: &Overrides) -> Result<(), String> {
    #[cfg(not(unix))]
    {
        let _ = (settings, overrides);
        return Err("daemon mode is only supported on Unix systems".into());
    }
    #[cfg(unix)]
    {
        let log = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(settings.log_path())
            .map_err(|error| format!("{}: {error}", settings.log_path().display()))?;
        let mut command = Command::new(std::env::current_exe().map_err(|error| error.to_string())?);
        command
            .arg("--config")
            .arg(&settings.config_path)
            .arg("start")
            .stdin(Stdio::null())
            .stdout(Stdio::from(
                log.try_clone().map_err(|error| error.to_string())?,
            ))
            .stderr(Stdio::from(log));
        if let Some(value) = &overrides.download_dir {
            command.env("RAYLOAD_DOWNLOAD_DIR", value);
        }
        if let Some(value) = &overrides.engine_bin {
            command.env("RAYLOAD_ARIA2_BIN", value);
        }
        if let Some(value) = &overrides.device_name {
            command.env("RAYLOAD_DEVICE_NAME", value);
        }
        if let Some(value) = overrides.lan_port {
            command.env("RAYLOAD_LAN_PORT", value.to_string());
        }
        if let Some(value) = overrides.rpc_port {
            command.env("RAYLOAD_RPC_PORT", value.to_string());
        }
        if let Some(value) = &overrides.lan_username {
            command.env("RAYLOAD_LAN_USERNAME", value);
        }
        if let Some(value) = &overrides.lan_password {
            command.env("RAYLOAD_LAN_PASSWORD", value);
        }
        if let Some(value) = overrides.require_manual_confirm {
            command.env("RAYLOAD_REQUIRE_MANUAL_CONFIRM", value.to_string());
        }
        if overrides.no_lan {
            command.arg("--no-lan");
        }
        unsafe {
            use std::os::unix::process::CommandExt;
            command.pre_exec(|| {
                libc::setsid();
                Ok(())
            });
        }
        let mut child = command.spawn().map_err(|error| error.to_string())?;
        write_pid(settings, child.id())?;
        std::thread::sleep(Duration::from_millis(300));
        if let Ok(Some(status)) = child.try_wait() {
            return Err(format!(
                "background process exited immediately ({status}); see {}",
                settings.log_path().display()
            ));
        }
        println!("Rayload started in the background (pid {})", child.id());
        println!("log: {}", settings.log_path().display());
        Ok(())
    }
}

pub fn add(
    overrides: &Overrides,
    url: &str,
    out: Option<&str>,
    json_output: bool,
) -> Result<(), String> {
    let settings = settings(overrides)?;
    if url.trim().is_empty() {
        return Err("URL cannot be empty".into());
    }
    let mut options = serde_json::Map::new();
    if let Some(name) = out.filter(|name| !name.trim().is_empty()) {
        options.insert("out".into(), json!(name));
    }
    let gid: String = rpc_call(
        &settings,
        "aria2.addUri",
        vec![json!([url]), json!(options)],
    )?;
    if json_output {
        println!("{}", json!({ "gid": gid }));
    } else {
        println!("Added {gid}");
    }
    Ok(())
}

pub fn list(overrides: &Overrides, json_output: bool) -> Result<(), String> {
    let settings = settings(overrides)?;
    let tasks = fetch_tasks(&settings)?;
    if json_output {
        let items: Vec<Value> = tasks.iter().map(task_json).collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({ "tasks": items }))
                .map_err(|error| error.to_string())?
        );
        return Ok(());
    }
    if tasks.is_empty() {
        println!("No downloads.");
        return Ok(());
    }
    let mut table = Table::new();
    table
        .load_preset(comfy_table::presets::UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["GID", "Name", "Progress", "Speed", "Status"]);
    for task in &tasks {
        let (done, total) = task_progress(task);
        let percent = if total == 0 {
            0
        } else {
            done.saturating_mul(100) / total
        };
        let speed = task.download_speed.parse::<u64>().unwrap_or(0);
        table.add_row(vec![
            Cell::new(task.gid.as_str()),
            Cell::new(task_name(task)),
            Cell::new(format!("{percent:>3}% {}", progress_bar(percent))),
            Cell::new(format_speed(speed)),
            state_cell(&task.status),
        ]);
    }
    println!("{table}");
    Ok(())
}

pub fn pause(overrides: &Overrides, gid: &str) -> Result<(), String> {
    let settings = settings(overrides)?;
    let _: String = rpc_call(&settings, "aria2.pause", vec![json!(gid)])?;
    println!("Paused {gid}");
    Ok(())
}

pub fn resume(overrides: &Overrides, gid: &str) -> Result<(), String> {
    let settings = settings(overrides)?;
    let _: String = rpc_call(&settings, "aria2.unpause", vec![json!(gid)])?;
    println!("Resumed {gid}");
    Ok(())
}

pub fn remove(overrides: &Overrides, gid: &str) -> Result<(), String> {
    let settings = settings(overrides)?;
    let removed = rpc_call::<String>(&settings, "aria2.remove", vec![json!(gid)]).is_ok();
    let mut cleared = false;
    for _ in 0..5 {
        if rpc_call::<String>(&settings, "aria2.removeDownloadResult", vec![json!(gid)]).is_ok() {
            cleared = true;
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    if !removed && !cleared {
        return Err(format!("could not remove {gid}"));
    }
    println!("Removed {gid} (downloaded files are kept)");
    Ok(())
}

pub fn log(overrides: &Overrides, lines: usize, follow: bool) -> Result<(), String> {
    let settings = settings(overrides)?;
    let path = settings.log_path();
    if !path.exists() {
        return Err(format!(
            "{} does not exist; start the daemon with ray start -d",
            path.display()
        ));
    }
    let mut position = print_log_tail(&path, lines)?;
    if !follow {
        return Ok(());
    }
    loop {
        std::thread::sleep(Duration::from_millis(250));
        let metadata = match fs::metadata(&path) {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };
        if metadata.len() < position {
            position = 0;
        }
        let mut file = fs::File::open(&path).map_err(|error| error.to_string())?;
        file.seek(SeekFrom::Start(position))
            .map_err(|error| error.to_string())?;
        let mut chunk = String::new();
        use std::io::Read;
        file.read_to_string(&mut chunk)
            .map_err(|error| error.to_string())?;
        if !chunk.is_empty() {
            print!("{chunk}");
            io::stdout().flush().map_err(|error| error.to_string())?;
            position += chunk.len() as u64;
        }
    }
}

pub fn plugin_list(
    overrides: &Overrides,
    directory: Option<&Path>,
    json_output: bool,
) -> Result<(), String> {
    let settings = settings(overrides)?;
    let directories: Vec<PathBuf> = match directory {
        Some(path) => vec![path.to_path_buf()],
        None => {
            let mut directories = Vec::new();
            if let Ok(cwd) = std::env::current_dir() {
                directories.push(cwd.join("plugins"));
            }
            directories.push(settings.data_dir.join("plugins"));
            directories
        }
    };
    let mut plugins = Vec::new();
    for directory in &directories {
        let Ok(entries) = fs::read_dir(directory) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some("json") {
                continue;
            }
            let Ok(content) = fs::read_to_string(&path) else {
                continue;
            };
            let Ok(manifest) = serde_json::from_str::<Value>(&content) else {
                continue;
            };
            plugins.push(json!({
                "file": path.display().to_string(),
                "name": manifest.get("name").and_then(Value::as_str).unwrap_or("unnamed"),
                "version": manifest.get("version").and_then(Value::as_str).unwrap_or(""),
                "type": manifest.get("type").and_then(Value::as_str).unwrap_or("rule"),
                "description": manifest.get("description").and_then(Value::as_str).unwrap_or(""),
            }));
        }
    }
    plugins.sort_by_key(|plugin| plugin["name"].as_str().unwrap_or_default().to_lowercase());
    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({ "plugins": plugins }))
                .map_err(|error| error.to_string())?
        );
        return Ok(());
    }
    if plugins.is_empty() {
        println!(
            "No JSON plugins in {}",
            directories
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        return Ok(());
    }
    let mut table = Table::new();
    table
        .load_preset(comfy_table::presets::UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Name", "Version", "Type", "File"]);
    for plugin in plugins {
        table.add_row(vec![
            Cell::new(plugin["name"].as_str().unwrap_or_default()),
            Cell::new(plugin["version"].as_str().unwrap_or_default()),
            Cell::new(plugin["type"].as_str().unwrap_or_default()),
            Cell::new(plugin["file"].as_str().unwrap_or_default()),
        ]);
    }
    println!("{table}");
    Ok(())
}

pub fn stop(_overrides: &Overrides) -> Result<(), String> {
    let settings = settings(_overrides)?;
    #[cfg(unix)]
    {
        let Some(pid) = read_pid(&settings.pid_path()) else {
            println!("Rayload is not running (no pid file).");
            return Ok(());
        };
        if !process_alive(pid) {
            let _ = fs::remove_file(settings.pid_path());
            println!("Rayload is not running (stale pid {pid}).");
            return Ok(());
        }
        unsafe {
            libc::kill(pid as i32, libc::SIGTERM);
        }
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        while process_alive(pid) && std::time::Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(100));
        }
        if process_alive(pid) {
            unsafe {
                libc::kill(pid as i32, libc::SIGKILL);
            }
        }
        let _ = fs::remove_file(settings.pid_path());
        println!("Stopped Rayload (pid {pid}).");
        Ok(())
    }
    #[cfg(not(unix))]
    Err("stop is only supported on Unix systems".into())
}

pub fn security_check(overrides: &Overrides, json_output: bool) -> Result<(), String> {
    let settings = settings(overrides)?;
    let checks = crate::security::audit(&crate::security::SecuritySettings {
        lan_port: settings.lan_port,
        rpc_port: settings.rpc_port,
        lan_username: &settings.lan_username,
        lan_password: &settings.lan_password,
        download_dir: &settings.download_dir,
        is_root: crate::security::running_as_root(),
    });
    let failures = checks
        .iter()
        .filter(|check| check.severity == crate::security::Severity::Fail)
        .count();
    if json_output {
        let items: Vec<Value> = checks
            .iter()
            .map(|check| {
                json!({
                    "name": check.name,
                    "severity": match check.severity {
                        crate::security::Severity::Pass => "pass",
                        crate::security::Severity::Warn => "warn",
                        crate::security::Severity::Fail => "fail",
                    },
                    "detail": check.detail,
                    "advice": check.advice,
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({ "checks": items, "failures": failures }))
                .map_err(|error| error.to_string())?
        );
        if failures > 0 {
            return Err(format!("{failures} high-risk checks failed"));
        }
        return Ok(());
    }
    let styles = Styles::detect();
    println!();
    println!("{}", styles.bold("Rayload CLI 安全体检"));
    println!("{}", styles.dim("===================="));
    for check in &checks {
        let color = match check.severity {
            crate::security::Severity::Pass => styles.green("PASS"),
            crate::security::Severity::Warn => styles.yellow("WARN"),
            crate::security::Severity::Fail => styles.red("FAIL"),
        };
        println!("[{color}] {}", check.name);
        println!("      {}", check.detail);
        println!("      {}", styles.dim(format!("建议：{}", check.advice)));
    }
    println!();
    if failures == 0 {
        println!("{}", styles.green("结论：未发现高危问题。"));
        Ok(())
    } else {
        Err(format!(
            "{failures} high-risk checks failed; fix them before enabling the LAN API"
        ))
    }
}

pub fn status(overrides: &Overrides, json_output: bool) -> Result<(), String> {
    let settings = settings(overrides)?;
    let styles = Styles::detect();
    let pid = read_pid(&settings.pid_path()).filter(|pid| process_alive(*pid));
    let api = LocalApi::new(&settings).ok();
    let engine = api.as_ref().and_then(|api| api.status().ok());
    let devices = api
        .as_ref()
        .and_then(|api| api.devices().ok())
        .unwrap_or_else(|| devices_from_file(&settings));
    let online = devices.iter().filter(|device| device.online).count();
    let lan_reachable = api.is_some() && engine.is_some();

    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "device": settings.device_name,
                "running": pid.is_some(),
                "pid": pid,
                "rpcPort": settings.rpc_port,
                "lanEnabled": settings.lan_enabled,
                "lanPort": settings.lan_port,
                "lanReachable": lan_reachable,
                "speed": engine.map(|status| status.download_speed),
                "activeTasks": engine.map(|status| status.active_tasks),
                "pairedDevices": devices.len(),
                "onlineDevices": online,
            }))
            .map_err(|error| error.to_string())?
        );
        return Ok(());
    }

    let engine_line = match pid {
        Some(pid) => styles.green(&format!("running (pid {pid})")),
        None => styles.red("stopped"),
    };
    let lan_line = if !settings.lan_enabled {
        styles.dim("disabled")
    } else if lan_reachable {
        styles.green(&format!("http://0.0.0.0:{}", settings.lan_port))
    } else if settings.lan_ready() {
        styles.red("not reachable")
    } else {
        styles.yellow("waiting for credentials")
    };
    let mut root = Node::new(styles.bold("Rayload"))
        .child(Node::new(format!(
            "{}  {}",
            styles.dim("Device"),
            settings.device_name
        )))
        .child(Node::new(format!(
            "{}  {engine_line}",
            styles.dim("Engine")
        )))
        .child(Node::new(format!(
            "{}  {}",
            styles.dim("RPC"),
            format!("127.0.0.1:{}", settings.rpc_port)
        )))
        .child(Node::new(format!("{}  {lan_line}", styles.dim("LAN"))));
    if let Some(engine) = engine {
        root = root
            .child(Node::new(format!(
                "{}  {}",
                styles.dim("Speed"),
                styles.cyan(&format_speed(engine.download_speed))
            )))
            .child(Node::new(format!(
                "{}  {}",
                styles.dim("Active"),
                engine.active_tasks
            )));
    }
    root = root.child(Node::new(format!(
        "{}  {} paired ({} online)",
        styles.dim("Devices"),
        devices.len(),
        online
    )));
    println!("{}", render_tree(&root));
    Ok(())
}

pub fn auth(overrides: &Overrides) -> Result<(), String> {
    ensure_interactive("ray auth")?;
    let path = overrides
        .config
        .clone()
        .unwrap_or_else(config::default_config_path);
    let mut file = config::read_file(&path)?;
    let username: String = Input::new()
        .with_prompt("LAN admin username")
        .default(file.lan.username.clone())
        .interact_text()
        .map_err(|error| error.to_string())?;
    if username.trim().is_empty() {
        return Err("LAN admin username cannot be empty".into());
    }
    let password = Password::new()
        .with_prompt("New LAN admin password")
        .with_confirmation("Repeat password", "Passwords do not match")
        .interact()
        .map_err(|error| error.to_string())?;
    if password.is_empty() {
        return Err("LAN admin password cannot be empty".into());
    }
    file.lan.username = username.trim().to_string();
    file.lan.password = password;
    config::save_file(&path, &file)?;
    println!("Updated LAN credentials in {}", path.display());
    println!("Restart Rayload for the new credentials to take effect.");
    Ok(())
}

pub fn config_show(overrides: &Overrides) -> Result<(), String> {
    let settings = settings(overrides)?;
    println!("{}", settings.config_path.display());
    let yaml =
        serde_yaml::to_string(&settings.as_config_file()).map_err(|error| error.to_string())?;
    print!("{yaml}");
    Ok(())
}

pub fn config_path(overrides: &Overrides) -> Result<(), String> {
    let path = overrides
        .config
        .clone()
        .unwrap_or_else(config::default_config_path);
    println!("{}", path.display());
    Ok(())
}

pub fn config_get(overrides: &Overrides, key: &str) -> Result<(), String> {
    let path = overrides
        .config
        .clone()
        .unwrap_or_else(config::default_config_path);
    println!("{}", config::get_value(&path, key)?);
    Ok(())
}

pub fn config_set(overrides: &Overrides, key: &str, value: &str) -> Result<(), String> {
    let path = overrides
        .config
        .clone()
        .unwrap_or_else(config::default_config_path);
    config::set_value(&path, key, value)?;
    println!("Set {key} in {}", path.display());
    Ok(())
}

pub fn device_list(overrides: &Overrides, json_output: bool) -> Result<(), String> {
    let settings = settings(overrides)?;
    let styles = Styles::detect();
    let api = LocalApi::new(&settings).ok();
    let devices = api
        .as_ref()
        .and_then(|api| api.devices().ok())
        .unwrap_or_else(|| devices_from_file(&settings));
    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({ "devices": devices }))
                .map_err(|error| error.to_string())?
        );
        return Ok(());
    }
    if devices.is_empty() {
        println!("No paired devices.");
        return Ok(());
    }
    let mut root = Node::new(styles.bold("Paired devices"));
    for device in devices {
        let status = if device.online {
            styles.green("● online")
        } else {
            styles.red("○ offline")
        };
        let label = format!(
            "{}  {}  {}",
            styles.cyan(&device.name),
            status,
            styles.dim(&format!("id {}", device.id))
        );
        root = root.child(Node::new(label).child(Node::new(
            styles.dim(format!("last seen {}", relative_time(device.last_seen))),
        )));
    }
    println!("{}", render_tree(&root));
    Ok(())
}

pub fn device_pending(overrides: &Overrides, json_output: bool) -> Result<(), String> {
    let settings = settings(overrides)?;
    let styles = Styles::detect();
    let api = LocalApi::new(&settings).map_err(|error| {
        format!("{error}; pending requests are only visible while the LAN API is running")
    })?;
    let pending = api.pending()?;
    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({ "pending": pending }))
                .map_err(|error| error.to_string())?
        );
        return Ok(());
    }
    if pending.is_empty() {
        println!("No pending pairing requests.");
        return Ok(());
    }
    let mut root = Node::new(styles.bold("Pending pairing requests"));
    for request in pending {
        root = root.child(
            Node::new(format!(
                "{}  {}",
                styles.cyan(&request.name),
                styles.dim(format!("({})", request.arch))
            ))
            .child(Node::new(format!("id  {}", request.id)))
            .child(Node::new(format!("fingerprint  {}", request.fingerprint))),
        );
    }
    println!("{}", render_tree(&root));
    println!("Approve with: ray -d allow <id>");
    Ok(())
}

pub fn device_allow(overrides: &Overrides, id: &str) -> Result<(), String> {
    let settings = settings(overrides)?;
    let api = LocalApi::new(&settings)?;
    api.decide(id, true)?;
    println!("Approved pairing request {id}.");
    Ok(())
}

pub fn device_reject(overrides: &Overrides, id: &str) -> Result<(), String> {
    let settings = settings(overrides)?;
    let api = LocalApi::new(&settings)?;
    api.decide(id, false)?;
    println!("Rejected pairing request {id}.");
    Ok(())
}

pub fn device_revoke(overrides: &Overrides, id: &str) -> Result<(), String> {
    let settings = settings(overrides)?;
    match LocalApi::new(&settings).and_then(|api| api.revoke(id)) {
        Ok(()) => {
            println!("Revoked {id}.");
            Ok(())
        }
        Err(error) => {
            if read_pid(&settings.pid_path()).is_some_and(process_alive) {
                Err(format!(
                    "LAN API unavailable ({error}); keep the daemon running with LAN enabled to revoke"
                ))
            } else {
                remove_device_from_file(&settings, id)?;
                println!("Revoked {id} (daemon is stopped).");
                Ok(())
            }
        }
    }
}

pub fn install(overrides: &Overrides) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        install_systemd(overrides)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = overrides;
        Err("systemd installation is only supported on Linux".into())
    }
}

#[cfg(target_os = "linux")]
fn install_systemd(overrides: &Overrides) -> Result<(), String> {
    if !crate::security::running_as_root() {
        return Err("ray install must run as root (try sudo ray install)".into());
    }
    let user = std::env::var("SUDO_USER")
        .ok()
        .filter(|user| !user.is_empty())
        .or_else(|| std::env::var("USER").ok())
        .unwrap_or_else(|| "root".into());
    let home = if user == "root" {
        PathBuf::from("/root")
    } else {
        user_home(&user).ok_or_else(|| format!("cannot resolve home directory for {user}"))?
    };
    let config_path = overrides
        .config
        .clone()
        .unwrap_or_else(|| home.join(".config/rayload/config.yml"));
    if !config_path.exists() {
        return Err(format!(
            "{} does not exist; run ray init as {user} first",
            config_path.display()
        ));
    }
    let exe = std::env::current_exe().map_err(|error| error.to_string())?;
    let target = PathBuf::from("/usr/local/bin/ray");
    if exe != target {
        fs::copy(&exe, &target).map_err(|error| format!("{}: {error}", target.display()))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&target, fs::Permissions::from_mode(0o755))
                .map_err(|error| error.to_string())?;
        }
    }
    let link = PathBuf::from("/usr/local/bin/rayload");
    if !link.exists() {
        std::os::unix::fs::symlink(&target, &link)
            .map_err(|error| format!("{}: {error}", link.display()))?;
    }
    let group = user_group(&user).unwrap_or_else(|| user.clone());
    let unit = format!(
        "[Unit]\nDescription=Rayload headless downloader\nAfter=network-online.target\nWants=network-online.target\n\n[Service]\nType=simple\nUser={user}\nGroup={group}\nEnvironment=HOME={}\nExecStart=\"{}\" --config \"{}\" start\nRestart=on-failure\nRestartSec=3\n\n[Install]\nWantedBy=multi-user.target\n",
        home.display(),
        target.display(),
        config_path.display()
    );
    let unit_path = PathBuf::from("/etc/systemd/system/rayload.service");
    fs::write(&unit_path, unit).map_err(|error| format!("{}: {error}", unit_path.display()))?;
    run_systemctl(&["daemon-reload"])?;
    run_systemctl(&["enable", "--now", "rayload.service"])?;
    println!("Installed {}", unit_path.display());
    println!("Status: systemctl status rayload");
    println!("Logs:   journalctl -u rayload -f");
    Ok(())
}

#[cfg(target_os = "linux")]
fn run_systemctl(args: &[&str]) -> Result<(), String> {
    let status = Command::new("systemctl")
        .args(args)
        .status()
        .map_err(|error| format!("systemctl: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("systemctl {} failed ({status})", args.join(" ")))
    }
}

#[cfg(target_os = "linux")]
fn user_group(user: &str) -> Option<String> {
    let output = Command::new("id").args(["-gn", user]).output().ok()?;
    let group = String::from_utf8(output.stdout).ok()?.trim().to_string();
    (!group.is_empty()).then_some(group)
}

#[cfg(target_os = "linux")]
fn user_home(user: &str) -> Option<PathBuf> {
    let output = Command::new("getent")
        .args(["passwd", user])
        .output()
        .ok()?;
    let text = String::from_utf8(output.stdout).ok()?;
    let home = text.trim().split(':').nth(5)?;
    (!home.is_empty()).then(|| PathBuf::from(home))
}

fn rpc_call<T: DeserializeOwned>(
    settings: &Settings,
    method: &str,
    params: Vec<Value>,
) -> Result<T, String> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| error.to_string())?;
    runtime
        .block_on(async {
            RpcClient::new(settings.rpc_port, String::new())
                .call::<T>(method, params)
                .await
        })
        .map_err(|error| error.to_string())
}

fn fetch_tasks(settings: &Settings) -> Result<Vec<Aria2Task>, String> {
    let mut tasks: Vec<Aria2Task> = rpc_call(settings, "aria2.tellActive", vec![])?;
    tasks.extend(rpc_call::<Vec<Aria2Task>>(
        settings,
        "aria2.tellWaiting",
        vec![json!(0), json!(1000)],
    )?);
    tasks.extend(rpc_call::<Vec<Aria2Task>>(
        settings,
        "aria2.tellStopped",
        vec![json!(0), json!(200)],
    )?);
    Ok(tasks)
}

fn task_name(task: &Aria2Task) -> String {
    task.files
        .first()
        .map(|file| {
            file.path
                .rsplit(['/', '\\'])
                .next()
                .unwrap_or_default()
                .to_string()
        })
        .filter(|name| !name.is_empty())
        .or_else(|| {
            task.bittorrent
                .as_ref()?
                .info
                .as_ref()
                .map(|info| info.name.clone())
        })
        .unwrap_or_else(|| task.gid.clone())
}

fn task_progress(task: &Aria2Task) -> (u64, u64) {
    (
        task.completed_length.parse().unwrap_or(0),
        task.total_length.parse().unwrap_or(0),
    )
}

fn task_json(task: &Aria2Task) -> Value {
    let (done, total) = task_progress(task);
    json!({
        "gid": task.gid,
        "name": task_name(task),
        "status": task.status,
        "totalLength": total,
        "completedLength": done,
        "progress": if total == 0 { 0 } else { done.saturating_mul(100) / total },
        "speed": task.download_speed.parse::<u64>().unwrap_or(0),
    })
}

fn progress_bar(percent: u64) -> String {
    let filled = (percent.min(100) * 12 / 100) as usize;
    format!("{}{}", "█".repeat(filled), "░".repeat(12 - filled))
}

fn state_cell(status: &str) -> Cell {
    let color = match status {
        "active" => Color::Green,
        "waiting" => Color::Cyan,
        "paused" => Color::Yellow,
        "complete" => Color::Green,
        "error" => Color::Red,
        _ => Color::DarkGrey,
    };
    Cell::new(status).fg(color)
}

fn print_log_tail(path: &Path, lines: usize) -> Result<u64, String> {
    let content = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let all: Vec<&str> = content.lines().collect();
    let start = all.len().saturating_sub(lines);
    for line in &all[start..] {
        println!("{line}");
    }
    Ok(content.len() as u64)
}

fn devices_from_file(settings: &Settings) -> Vec<DeviceInfo> {
    let path = settings.data_dir.join("lan").join("devices.json");
    let Ok(content) = fs::read_to_string(path) else {
        return Vec::new();
    };
    let Ok(document) = serde_json::from_str::<Value>(&content) else {
        return Vec::new();
    };
    let now = unix_now();
    let Some(devices) = document.get("devices").and_then(Value::as_object) else {
        return Vec::new();
    };
    let mut result: Vec<DeviceInfo> = devices
        .iter()
        .map(|(id, device)| {
            let last_seen = device.get("last_seen").and_then(Value::as_u64).unwrap_or(0);
            DeviceInfo {
                id: id.clone(),
                name: device
                    .get("name")
                    .and_then(Value::as_str)
                    .unwrap_or("device")
                    .to_string(),
                last_seen,
                online: now.saturating_sub(last_seen) <= 120,
            }
        })
        .collect();
    result.sort_by(|left, right| right.last_seen.cmp(&left.last_seen));
    result
}

fn remove_device_from_file(settings: &Settings, id: &str) -> Result<(), String> {
    let path = settings.data_dir.join("lan").join("devices.json");
    let content =
        fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut document =
        serde_json::from_str::<Value>(&content).map_err(|error| error.to_string())?;
    let removed = document
        .get_mut("devices")
        .and_then(Value::as_object_mut)
        .and_then(|devices| devices.remove(id))
        .is_some();
    if !removed {
        return Err(format!("device {id} is not paired"));
    }
    config::write_secure(
        &path,
        &format!(
            "{}\n",
            serde_json::to_string_pretty(&document).unwrap_or_default()
        ),
    )?;
    Ok(())
}

fn write_pid(settings: &Settings, pid: u32) -> Result<(), String> {
    config::write_secure(&settings.pid_path(), &format!("{pid}\n"))?;
    Ok(())
}

fn read_pid(path: &Path) -> Option<u32> {
    fs::read_to_string(path).ok()?.trim().parse::<u32>().ok()
}

fn process_alive(pid: u32) -> bool {
    #[cfg(unix)]
    {
        if unsafe { libc::kill(pid as i32, 0) } != 0 {
            return false;
        }
        // A stale PID must not make us signal an unrelated process.
        match fs::read(format!("/proc/{pid}/cmdline")) {
            Ok(command) => String::from_utf8_lossy(&command).contains("ray"),
            Err(_) => true,
        }
    }
    #[cfg(not(unix))]
    {
        let _ = pid;
        false
    }
}

fn relative_time(last_seen: u64) -> String {
    if last_seen == 0 {
        return "unknown".into();
    }
    let seconds = unix_now().saturating_sub(last_seen);
    if seconds < 60 {
        format!("{seconds}s ago")
    } else if seconds < 3600 {
        format!("{}m ago", seconds / 60)
    } else if seconds < 86_400 {
        format!("{}h ago", seconds / 3600)
    } else {
        format!("{}d ago", seconds / 86_400)
    }
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn ensure_interactive(command: &str) -> Result<(), String> {
    if io::stdin().is_terminal() && io::stdout().is_terminal() {
        Ok(())
    } else {
        Err(format!("{command} needs an interactive terminal"))
    }
}

struct PidGuard {
    path: PathBuf,
}

impl Drop for PidGuard {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
