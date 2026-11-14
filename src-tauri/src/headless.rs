//! LAN API backend for the headless CLI.
//!
//! The Tauri app injects its task service; the CLI injects the same aria2
//! JSON-RPC client directly so pairing, handoff, status and task actions work
//! without a GUI.

use std::io::{IsTerminal, Write};
use std::path::Path;
use std::sync::Arc;

use serde_json::{json, Value};

use crate::api_auth::{AuthService, PairingRequestView, PendingPairData, HEARTBEAT_TTL};
use crate::api_server::{
    self, Backend, BackendFuture, DeviceStatus, HandoffRequest, HandoffResult, HandoffTask,
    TaskAction,
};
use crate::aria2::rpc::RpcClient;
use crate::aria2::types::Aria2Task;

struct RpcBackend {
    rpc: Arc<RpcClient>,
    download_root: Arc<std::path::PathBuf>,
}

/// RPC backend plus the pairing decision policy for a headless host.
struct CliBackend {
    rpc: RpcBackend,
    auth: Arc<AuthService>,
    require_manual_confirm: bool,
    interactive: bool,
}

impl Backend for CliBackend {
    fn status(&self) -> BackendFuture<DeviceStatus> {
        self.rpc.status()
    }

    fn handoff(&self, request: HandoffRequest) -> BackendFuture<HandoffResult> {
        self.rpc.handoff(request)
    }

    fn task(&self, gid: String) -> BackendFuture<HandoffTask> {
        self.rpc.task(gid)
    }

    fn task_action(&self, gid: String, action: TaskAction) -> BackendFuture<()> {
        self.rpc.task_action(gid, action)
    }

    fn requires_pair_approval(&self) -> bool {
        self.require_manual_confirm
    }

    fn notify_pair_request(&self, request: &PairingRequestView) {
        if !self.interactive {
            return;
        }
        let auth = Arc::clone(&self.auth);
        let request = request.clone();
        std::thread::spawn(move || {
            eprint!(
                "\nPairing request from {} ({}, {}). Allow? [y/N] ",
                request.name, request.arch, request.fingerprint
            );
            let _ = std::io::stdout().flush();
            let mut answer = String::new();
            let approved = std::io::stdin().read_line(&mut answer).is_ok()
                && matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes");
            let _ = auth.resolve_pair(&request.id, approved);
        });
    }

    fn complete_pair(&self, data: PendingPairData) -> BackendFuture<String> {
        let auth = Arc::clone(&self.auth);
        Box::pin(async move {
            auth.pair_named(&data.device_id, &data.name, &data.public_key)
                .map_err(|error| error.as_str().to_string())?;
            // The headless host has no peer database for a reverse handoff.
            Ok(String::new())
        })
    }
}

pub struct LanServeOptions {
    pub rpc_port: u16,
    pub api_port: u16,
    pub device_name: String,
    pub username: String,
    pub password: String,
    pub require_manual_confirm: bool,
    pub interactive: bool,
}

fn number(value: &Value, key: &str) -> u64 {
    value[key]
        .as_str()
        .and_then(|raw| raw.parse().ok())
        .or_else(|| value[key].as_u64())
        .unwrap_or(0)
}

impl Backend for RpcBackend {
    fn status(&self) -> BackendFuture<DeviceStatus> {
        let rpc = Arc::clone(&self.rpc);
        Box::pin(async move {
            let value = rpc
                .call::<Value>("aria2.getGlobalStat", Vec::new())
                .await
                .map_err(|error| error.to_string())?;
            Ok(DeviceStatus {
                download_speed: number(&value, "downloadSpeed"),
                active_tasks: number(&value, "numActive") as u32,
            })
        })
    }

    fn handoff(&self, request: HandoffRequest) -> BackendFuture<HandoffResult> {
        let rpc = Arc::clone(&self.rpc);
        let download_root = Arc::clone(&self.download_root);
        Box::pin(async move {
            let options = match request.filename.as_deref() {
                Some(name) => {
                    crate::services::downloads::validate_filename(name)
                        .map_err(|error| error.to_string())?;
                    // Handoff targets must resolve inside the configured root,
                    // which blocks `../` traversal even if validation weakens.
                    let path = crate::security::resolve_download_path(&download_root, name)
                        .map_err(|error| format!("unsafe download path: {error}"))?;
                    let file_name = path
                        .file_name()
                        .map(|value| value.to_string_lossy().to_string())
                        .unwrap_or_default();
                    json!({ "out": file_name })
                }
                None => json!({}),
            };
            let gid = rpc
                .call::<String>("aria2.addUri", vec![json!([request.url]), options])
                .await
                .map_err(|error| error.to_string())?;
            Ok(HandoffResult { gid })
        })
    }

    fn task(&self, gid: String) -> BackendFuture<HandoffTask> {
        let rpc = Arc::clone(&self.rpc);
        Box::pin(async move {
            let task = rpc
                .call::<Aria2Task>("aria2.tellStatus", vec![json!(gid)])
                .await
                .map_err(|error| error.to_string())?;
            Ok(HandoffTask {
                gid: task.gid,
                status: task.status,
                total_length: task.total_length.parse().unwrap_or(0),
                completed_length: task.completed_length.parse().unwrap_or(0),
                download_speed: task.download_speed.parse().unwrap_or(0),
                filename: task
                    .files
                    .first()
                    .map(|file| {
                        file.path
                            .rsplit(['/', '\\'])
                            .next()
                            .unwrap_or_default()
                            .to_string()
                    })
                    .unwrap_or_default(),
            })
        })
    }

    fn task_action(&self, gid: String, action: TaskAction) -> BackendFuture<()> {
        let rpc = Arc::clone(&self.rpc);
        Box::pin(async move {
            let method = match action {
                TaskAction::Pause => "aria2.pause",
                TaskAction::Unpause => "aria2.unpause",
                TaskAction::Remove => "aria2.remove",
            };
            let result = match rpc.call::<String>(method, vec![json!(gid.clone())]).await {
                Ok(value) => Ok(value),
                // Completed downloads have no active entry; drop the stored
                // result instead, matching the Tauri backend.
                Err(error) if matches!(action, TaskAction::Remove) => rpc
                    .call::<String>("aria2.removeDownloadResult", vec![json!(gid)])
                    .await
                    .map_err(|_| error.to_string()),
                Err(error) => Err(error.to_string()),
            };
            result.map(|_| ())
        })
    }
}

/// Serves the LAN API until the process exits.
pub async fn serve_lan(
    data_dir: &Path,
    download_root: &Path,
    options: LanServeOptions,
) -> Result<(), String> {
    let identity = data_dir.join("lan").join("identity.key");
    let auth = AuthService::load_or_create(
        &identity,
        &options.username,
        &options.password,
        HEARTBEAT_TTL,
    )
    .map_err(|error| error.to_string())?;
    auth.set_device_name(&options.device_name);
    let auth = Arc::new(auth);
    let backend: Arc<dyn Backend> = Arc::new(CliBackend {
        rpc: RpcBackend {
            rpc: Arc::new(RpcClient::new(options.rpc_port, String::new())),
            download_root: Arc::new(download_root.to_path_buf()),
        },
        auth: Arc::clone(&auth),
        require_manual_confirm: options.require_manual_confirm,
        interactive: options.interactive && std::io::stdin().is_terminal(),
    });
    let listeners = api_server::bind_listeners(options.api_port)
        .await
        .map_err(|error| format!("LAN API port {}: {error}", options.api_port))?;
    crate::lan_discovery::spawn(options.api_port);
    let mut tasks = Vec::new();
    for listener in listeners {
        let backend = Arc::clone(&backend);
        let auth = Arc::clone(&auth);
        tasks.push(tokio::spawn(api_server::serve(listener, backend, auth)));
    }
    for task in tasks {
        task.await
            .map_err(|error| error.to_string())?
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}
