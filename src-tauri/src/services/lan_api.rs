//! Wires the LAN control API to the task service and the auth layer.

use std::path::PathBuf;
use std::sync::Arc;

use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_store::StoreExt;

use crate::api_auth::{AuthService, PairingRequestView, PendingPairData, HEARTBEAT_TTL};
use crate::api_server::{
    self, Backend, BackendFuture, DeviceStatus, HandoffRequest, HandoffResult, HandoffTask,
    HistoryDay, HistoryExtension, HistoryStats, TaskAction,
};
use crate::database::{Database, HistoryRecord};
use crate::lan_client;
use crate::services::tasks::TaskService;

/// Loads the persistent server identity and the admin credentials.
///
/// Credentials come from `preferences.lanApiUsername` / `lanApiPassword`
/// (written by the LAN settings UI) with environment fallbacks for headless
/// runs. Empty credentials lock every protected endpoint.
pub struct LanAuthState(pub Arc<AuthService>);

pub fn init_auth(app: &tauri::AppHandle) -> Arc<AuthService> {
    let username = read_pref(app, "lanApiUsername")
        .or_else(|| std::env::var("RAYLOAD_LAN_USERNAME").ok())
        .unwrap_or_default();
    let password = read_pref(app, "lanApiPassword")
        .or_else(|| std::env::var("RAYLOAD_LAN_PASSWORD").ok())
        .unwrap_or_default();
    if username.is_empty() || password.is_empty() {
        log::warn!(
            "lan_api: admin credentials are not configured; protected endpoints reject requests"
        );
    }

    let identity = app
        .path()
        .app_local_data_dir()
        .ok()
        .map(|directory| directory.join("lan").join("identity.key"));
    let mut service = match identity {
        Some(path) => match AuthService::load_or_create(&path, &username, &password, HEARTBEAT_TTL)
        {
            Ok(service) => service,
            Err(error) => {
                log::warn!(
                    target: "lan_api",
                    event = "lan_identity_load_failed",
                    error:% = error;
                    "lan_identity_load_failed"
                );
                AuthService::new(&username, &password, HEARTBEAT_TTL)
            }
        },
        None => AuthService::new(&username, &password, HEARTBEAT_TTL),
    };
    service.set_device_name(
        &read_pref(app, "lanDeviceName").unwrap_or_else(crate::api_server::device_name),
    );
    Arc::new(service)
}

/// Starts the 6767 listener once per process. A bind failure is logged and
/// never blocks startup.
pub fn spawn(
    app: &AppHandle,
    tasks: Arc<TaskService>,
    auth: Arc<AuthService>,
    database: Arc<Database>,
) {
    let store_path = app
        .path()
        .app_local_data_dir()
        .map(|directory| directory.join("lan").join("peers.json"))
        .unwrap_or_default();
    let backend: Arc<dyn Backend> = Arc::new(TaskBackend {
        tasks,
        database,
        auth: Arc::clone(&auth),
        app: app.clone(),
        store_path,
    });
    tauri::async_runtime::spawn(async move {
        let listeners = match api_server::bind_listeners(api_server::LISTEN_ADDR.port()).await {
            Ok(listeners) => listeners,
            Err(error) => {
                log::warn!(
                    target: "lan_api",
                    event = "lan_api_bind_failed",
                    address:% = api_server::LISTEN_ADDR,
                    error:% = error;
                    "lan_api_bind_failed"
                );
                return;
            }
        };
        for listener in listeners {
            let backend = Arc::clone(&backend);
            let auth = Arc::clone(&auth);
            tauri::async_runtime::spawn(async move {
                if let Err(error) = api_server::serve(listener, backend, auth).await {
                    log::warn!(
                        target: "lan_api",
                        event = "lan_api_serve_failed",
                        error:% = error;
                        "lan_api_serve_failed"
                    );
                }
            });
        }
    });
}

struct TaskBackend {
    tasks: Arc<TaskService>,
    database: Arc<Database>,
    auth: Arc<AuthService>,
    app: AppHandle,
    store_path: PathBuf,
}

impl Backend for TaskBackend {
    fn status(&self) -> BackendFuture<DeviceStatus> {
        let tasks = Arc::clone(&self.tasks);
        Box::pin(async move {
            let stat = tasks
                .get_global_stat()
                .await
                .map_err(|error| error.to_string())?;
            Ok(DeviceStatus {
                download_speed: stat.download_speed.parse().unwrap_or(0),
                active_tasks: stat.num_active.parse().unwrap_or(0),
            })
        })
    }

    fn history_stats(&self) -> BackendFuture<HistoryStats> {
        let database = Arc::clone(&self.database);
        Box::pin(async move {
            let records = database
                .get_records(Some("complete"))
                .await
                .map_err(|error| error.to_string())?;
            Ok(aggregate_history(records))
        })
    }

    fn handoff(&self, request: HandoffRequest) -> BackendFuture<HandoffResult> {
        let tasks = Arc::clone(&self.tasks);
        Box::pin(async move {
            if let Some(name) = request.filename.as_deref() {
                crate::services::downloads::validate_filename(name)
                    .map_err(|error| error.to_string())?;
            }
            let options = match request.filename {
                Some(name) => serde_json::json!({ "out": name }),
                None => serde_json::json!({}),
            };
            let gid = tasks
                .add_uri(vec![request.url], options)
                .await
                .map_err(|error| error.to_string())?;
            Ok(HandoffResult { gid })
        })
    }

    fn task(&self, gid: String) -> BackendFuture<HandoffTask> {
        let tasks = Arc::clone(&self.tasks);
        Box::pin(async move {
            let task = tasks
                .tell_status(&gid)
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
        let tasks = Arc::clone(&self.tasks);
        Box::pin(async move {
            let result = match action {
                TaskAction::Pause => tasks.pause(&gid).await,
                TaskAction::Unpause => tasks.unpause(&gid).await,
                TaskAction::Remove => match tasks.force_remove(&gid).await {
                    // Completed downloads have no active entry to cancel, so
                    // fall back to dropping the stored result from the list.
                    Ok(value) => Ok(value),
                    Err(error) => tasks.remove_download_result(&gid).await.map_err(|_| error),
                },
            };
            result.map_err(|error| error.to_string())?;
            Ok(())
        })
    }

    fn requires_pair_approval(&self) -> bool {
        true
    }

    fn notify_pair_request(&self, request: &PairingRequestView) {
        if let Err(error) = self.app.emit("lan:pair-request", request) {
            log::warn!(target: "lan_api", "pair request event failed: {error}");
        }
    }

    fn complete_pair(&self, data: PendingPairData) -> BackendFuture<String> {
        let auth = Arc::clone(&self.auth);
        let app = self.app.clone();
        let store_path = self.store_path.clone();
        Box::pin(async move {
            let device_id = lan_client::complete_pair(&store_path, &data, &auth).await?;
            if let Err(error) = app.emit("lan:paired", &data.name) {
                log::warn!(target: "lan_api", "paired event failed: {error}");
            }
            Ok(device_id)
        })
    }

    fn forget_device(&self, device_id: &str) -> BackendFuture<()> {
        let app = self.app.clone();
        let store_path = self.store_path.clone();
        let device_id = device_id.to_string();
        Box::pin(async move {
            lan_client::remove_device(&store_path, &device_id)?;
            if let Err(error) = app.emit("lan:unpaired", &device_id) {
                log::warn!(target: "lan_api", "unpaired event failed: {error}");
            }
            Ok(())
        })
    }
}

fn completed_date(value: &str) -> Option<NaiveDate> {
    if let Ok(parsed) = DateTime::parse_from_rfc3339(value) {
        return Some(parsed.with_timezone(&Local).date_naive());
    }
    let naive = NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S").ok()?;
    Some(
        Utc.from_utc_datetime(&naive)
            .with_timezone(&Local)
            .date_naive(),
    )
}

fn file_extension(name: &str) -> String {
    name.rsplit_once('.')
        .map(|(_, extension)| extension.trim().to_ascii_lowercase())
        .unwrap_or_default()
}

/// Builds compact aggregates so paired devices never need the full history.
fn aggregate_history(records: Vec<HistoryRecord>) -> HistoryStats {
    let today = Local::now().date_naive();
    let start = today - Duration::days(6);
    let mut days = (0..7)
        .map(|offset| HistoryDay {
            date: (start + Duration::days(offset))
                .format("%Y-%m-%d")
                .to_string(),
            bytes: 0,
        })
        .collect::<Vec<_>>();
    let mut extensions = std::collections::BTreeMap::<String, (u64, u64)>::new();
    let mut total_bytes = 0u64;
    let mut total_tasks = 0u64;

    for record in records {
        let bytes = record.total_length.unwrap_or_default().max(0) as u64;
        total_bytes = total_bytes.saturating_add(bytes);
        total_tasks = total_tasks.saturating_add(1);
        let entry = extensions.entry(file_extension(&record.name)).or_default();
        entry.0 = entry.0.saturating_add(bytes);
        entry.1 = entry.1.saturating_add(1);

        let stamp = record
            .completed_at
            .as_deref()
            .or(record.created_at.as_deref());
        let Some(date) = stamp.and_then(completed_date) else {
            continue;
        };
        let offset = (date - start).num_days();
        if (0..7).contains(&offset) {
            days[offset as usize].bytes = days[offset as usize].bytes.saturating_add(bytes);
        }
    }

    HistoryStats {
        total_bytes,
        total_tasks,
        days,
        extensions: extensions
            .into_iter()
            .map(|(extension, (bytes, tasks))| HistoryExtension {
                extension,
                bytes,
                tasks,
            })
            .collect(),
    }
}

fn read_pref(app: &tauri::AppHandle, key: &str) -> Option<String> {
    let store = app.store("config.json").ok()?;
    let preferences = store.get("preferences")?;
    Some(preferences.get(key)?.as_str()?.to_string()).filter(|value| !value.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(name: &str, bytes: i64, completed_at: String) -> HistoryRecord {
        HistoryRecord {
            id: None,
            gid: name.to_string(),
            name: name.to_string(),
            uri: None,
            dir: None,
            total_length: Some(bytes),
            status: "complete".into(),
            task_type: None,
            added_at: None,
            created_at: None,
            completed_at: Some(completed_at),
            meta: None,
        }
    }

    #[test]
    fn history_aggregates_extensions_and_recent_days() {
        let now = Local::now().to_rfc3339();
        let old = (Local::now() - Duration::days(30)).to_rfc3339();
        let stats = aggregate_history(vec![
            record("movie.MP4", 100, now.clone()),
            record("notes", 50, now),
            record("old.bin", 25, old),
        ]);

        assert_eq!(stats.total_bytes, 175);
        assert_eq!(stats.total_tasks, 3);
        assert!(stats.extensions.contains(&HistoryExtension {
            extension: "mp4".into(),
            bytes: 100,
            tasks: 1,
        }));
        assert!(stats.extensions.contains(&HistoryExtension {
            extension: String::new(),
            bytes: 50,
            tasks: 1,
        }));
        assert_eq!(stats.days.last().map(|day| day.bytes), Some(150));
    }
}
