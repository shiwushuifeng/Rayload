//! Tauri commands for the LAN devices page.

use std::path::PathBuf;
use std::time::Duration;

use futures_util::{stream, StreamExt};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_store::StoreExt;

use crate::api_auth::PairingRequestView;
use crate::error::AppError;
use crate::lan_client::{self, PairResult, PeerHistoryStats, PeerStatus, PeerView, RemoteTaskView};
use crate::lan_discovery::DiscoveredPeer;
use crate::services::lan_api::LanAuthState;

fn store_path(app: &AppHandle) -> Result<PathBuf, AppError> {
    Ok(app
        .path()
        .app_local_data_dir()
        .map_err(|error| AppError::Io(error.to_string()))?
        .join("lan")
        .join("peers.json"))
}

#[tauri::command]
pub async fn lan_pair(
    app: AppHandle,
    state: State<'_, LanAuthState>,
    address: String,
    username: String,
    password: String,
) -> Result<PairResult, AppError> {
    let ticket = state.0.create_ticket();
    let result = lan_client::pair(
        &store_path(&app)?,
        &address,
        &username,
        &password,
        &state.0.device_name(),
        &ticket,
        &state.0.server_public_key(),
    )
    .await;
    if result.is_err() {
        // A rejected request must not leave a ticket the peer can redeem later.
        state.0.discard_ticket(&ticket);
    }
    result.map_err(AppError::Lan)
}

/// Pairs a discovered peer with this device's own admin credentials.
#[tauri::command]
pub async fn lan_pair_discovered(
    app: AppHandle,
    state: State<'_, LanAuthState>,
    address: String,
) -> Result<PairResult, AppError> {
    let (username, password) = state
        .0
        .credentials()
        .ok_or_else(|| AppError::Lan("LAN credentials are not configured".into()))?;
    let ticket = state.0.create_ticket();
    let result = lan_client::pair(
        &store_path(&app)?,
        &address,
        &username,
        &password,
        &state.0.device_name(),
        &ticket,
        &state.0.server_public_key(),
    )
    .await;
    if result.is_err() {
        state.0.discard_ticket(&ticket);
    }
    result.map_err(AppError::Lan)
}

/// Incoming pairing requests waiting for this device's approval.
#[tauri::command]
pub fn lan_pair_requests(state: State<'_, LanAuthState>) -> Vec<PairingRequestView> {
    state.0.pending_requests()
}

#[tauri::command]
pub fn lan_pair_approve(state: State<'_, LanAuthState>, id: String) -> Result<(), AppError> {
    state
        .0
        .resolve_pair(&id, true)
        .map_err(|error| AppError::Lan(error.as_str().into()))
}

#[tauri::command]
pub fn lan_pair_reject(state: State<'_, LanAuthState>, id: String) -> Result<(), AppError> {
    state
        .0
        .resolve_pair(&id, false)
        .map_err(|error| AppError::Lan(error.as_str().into()))
}

#[tauri::command]
pub fn lan_confirm_device(app: AppHandle, id: String) -> Result<(), AppError> {
    lan_client::confirm(&store_path(&app)?, &id).map_err(AppError::Lan)
}

#[tauri::command]
pub fn lan_forget_device(
    app: AppHandle,
    state: State<'_, LanAuthState>,
    id: String,
) -> Result<(), AppError> {
    lan_client::forget(&store_path(&app)?, &id, &state.0).map_err(AppError::Lan)
}

#[tauri::command]
pub fn lan_rename_device(app: AppHandle, id: String, name: String) -> Result<(), AppError> {
    lan_client::rename(&store_path(&app)?, &id, &name).map_err(AppError::Lan)
}

#[tauri::command]
pub fn lan_set_device_note(app: AppHandle, id: String, note: String) -> Result<(), AppError> {
    lan_client::set_note(&store_path(&app)?, &id, &note).map_err(AppError::Lan)
}

#[tauri::command]
pub async fn lan_devices(app: AppHandle) -> Result<Vec<PeerView>, AppError> {
    lan_client::devices(&store_path(&app)?)
        .await
        .map_err(AppError::Lan)
}

#[tauri::command]
pub async fn lan_device_statuses(app: AppHandle) -> Result<Vec<PeerStatus>, AppError> {
    lan_client::device_statuses(&store_path(&app)?)
        .await
        .map_err(AppError::Lan)
}

#[tauri::command]
pub async fn lan_device_history_stats(app: AppHandle) -> Result<Vec<PeerHistoryStats>, AppError> {
    lan_client::device_history_stats(&store_path(&app)?)
        .await
        .map_err(AppError::Lan)
}

#[tauri::command]
pub async fn lan_handoff(
    app: AppHandle,
    device_id: String,
    url: String,
    filename: Option<String>,
) -> Result<RemoteTaskView, AppError> {
    lan_client::handoff(&store_path(&app)?, &device_id, &url, filename)
        .await
        .map_err(AppError::Lan)
}

#[tauri::command]
pub async fn lan_remote_tasks(app: AppHandle) -> Result<Vec<RemoteTaskView>, AppError> {
    lan_client::remote_tasks(&store_path(&app)?)
        .await
        .map_err(AppError::Lan)
}

#[tauri::command]
pub async fn lan_remote_action(
    app: AppHandle,
    device_id: String,
    gid: String,
    action: String,
) -> Result<(), AppError> {
    lan_client::remote_action(&store_path(&app)?, &device_id, &gid, &action)
        .await
        .map_err(AppError::Lan)
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LanCredentials {
    pub username: String,
    pub configured: bool,
    pub name: String,
}

#[tauri::command]
pub fn lan_credentials(state: State<'_, LanAuthState>) -> LanCredentials {
    let username = state.0.username();
    LanCredentials {
        configured: !username.is_empty() && state.0.is_configured(),
        username,
        name: state.0.device_name(),
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPairingInfo {
    pub addresses: Vec<String>,
    pub username: String,
    pub password: String,
    pub name: String,
    pub arch: String,
    pub fingerprint: String,
}

/// Connection details for the pairing QR. Only callers inside the app can ask.
#[tauri::command]
pub fn lan_pairing_info(state: State<'_, LanAuthState>) -> Result<LanPairingInfo, AppError> {
    let (username, password) = state
        .0
        .credentials()
        .ok_or_else(|| AppError::Lan("LAN credentials are not configured".into()))?;
    let addresses = crate::lan_discovery::local_addresses()
        .into_iter()
        .map(|address| match address {
            std::net::IpAddr::V4(address) => {
                format!("http://{address}:{}", crate::lan_client::API_PORT)
            }
            std::net::IpAddr::V6(address) => {
                format!("http://[{address}]:{}", crate::lan_client::API_PORT)
            }
        })
        .collect::<Vec<_>>();
    if addresses.is_empty() {
        return Err(AppError::Lan("No LAN address detected".into()));
    }
    Ok(LanPairingInfo {
        addresses,
        username,
        password,
        name: state.0.device_name(),
        arch: std::env::consts::ARCH.to_string(),
        fingerprint: state.0.fingerprint(),
    })
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LanScanResult {
    pub address: String,
    pub name: String,
    pub arch: String,
    pub fingerprint: String,
}

/// Probes every host on each local IPv4 /24 for the public LAN ping endpoint.
/// Only runs when the user taps Scan network, never from status polling.
#[tauri::command]
pub async fn lan_scan(state: State<'_, LanAuthState>) -> Result<Vec<LanScanResult>, AppError> {
    scan_hosts(state.0.fingerprint()).await
}

async fn scan_hosts(own_fingerprint: String) -> Result<Vec<LanScanResult>, AppError> {
    // shortcut: IPv6 has no cheap /64 host scan; discovered or manual peers cover it.
    let prefixes = crate::lan_discovery::local_addresses()
        .into_iter()
        .filter_map(|address| match address {
            std::net::IpAddr::V4(address) => Some(address.octets()),
            std::net::IpAddr::V6(_) => None,
        })
        .map(|[a, b, c, _]| (a, b, c))
        .collect::<std::collections::HashSet<_>>();
    if prefixes.is_empty() {
        return Err(AppError::Lan("No LAN address detected".into()));
    }
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(450))
        .no_proxy()
        .build()
        .map_err(|error| AppError::Lan(error.to_string()))?;
    let hosts = prefixes
        .into_iter()
        .flat_map(|(a, b, c)| (1u8..=254).map(move |host| (a, b, c, host)))
        .collect::<Vec<_>>();
    let results = stream::iter(hosts)
        .map(|(a, b, c, d)| {
            let client = client.clone();
            async move {
                let address = format!("http://{a}.{b}.{c}.{d}:{}", crate::lan_client::API_PORT);
                let response = client
                    .get(format!("{address}/api/v1/ping"))
                    .send()
                    .await
                    .ok()?;
                if !response.status().is_success() {
                    return None;
                }
                let value: serde_json::Value = response.json().await.ok()?;
                Some(LanScanResult {
                    address,
                    name: value["name"].as_str().unwrap_or("Rayload").to_string(),
                    arch: value["arch"].as_str().unwrap_or_default().to_string(),
                    fingerprint: value["fingerprint"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                })
            }
        })
        .buffer_unordered(64)
        .filter_map(|result| async move { result })
        .collect::<Vec<_>>()
        .await;
    let mut seen = std::collections::HashSet::new();
    Ok(results
        .into_iter()
        .filter(|result| result.fingerprint.is_empty() || result.fingerprint != own_fingerprint)
        .filter(|result| {
            seen.insert(if result.fingerprint.is_empty() {
                result.address.clone()
            } else {
                result.fingerprint.clone()
            })
        })
        .collect())
}

/// Multicast peers that accept this device's admin credentials.
#[tauri::command]
pub async fn lan_discovered(
    state: State<'_, LanAuthState>,
) -> Result<Vec<DiscoveredPeer>, AppError> {
    let Some((username, password)) = state.0.credentials() else {
        return Ok(Vec::new());
    };
    let peers = crate::lan_discovery::current();
    let results = stream::iter(peers)
        .map(|peer| {
            let username = username.clone();
            let password = password.clone();
            async move {
                let mut candidates = peer.addresses.clone();
                if !candidates.contains(&peer.address) {
                    candidates.insert(0, peer.address.clone());
                }
                for address in candidates {
                    if lan_client::accepts_credentials(&address, &username, &password).await {
                        return Some(DiscoveredPeer { address, ..peer });
                    }
                }
                None
            }
        })
        .buffer_unordered(8)
        .filter_map(|result| async move { result })
        .collect::<Vec<_>>()
        .await;
    Ok(results)
}

/// Stores the host's admin credentials and applies them without a restart.
#[tauri::command]
pub fn lan_set_credentials(
    app: AppHandle,
    state: State<'_, LanAuthState>,
    username: String,
    password: String,
) -> Result<(), AppError> {
    state.0.set_credentials(&username, &password);
    save_prefs(
        &app,
        [("lanApiUsername", username), ("lanApiPassword", password)],
    )
}

/// Renames this device for LAN discovery, pairing and the device list.
#[tauri::command]
pub fn lan_set_device_name(
    app: AppHandle,
    state: State<'_, LanAuthState>,
    name: String,
) -> Result<(), AppError> {
    let name: String = name.trim().chars().take(64).collect();
    state.0.set_device_name(&name);
    save_prefs(&app, [("lanDeviceName", name)])
}

fn save_prefs<const N: usize>(
    app: &AppHandle,
    values: [(&str, String); N],
) -> Result<(), AppError> {
    let store = app
        .store("config.json")
        .map_err(|error| AppError::Store(error.to_string()))?;
    let mut preferences = store
        .get("preferences")
        .unwrap_or_else(|| serde_json::json!({}));
    if let Some(object) = preferences.as_object_mut() {
        for (key, value) in values {
            object.insert(key.into(), value.into());
        }
    }
    store.set("preferences", preferences);
    store
        .save()
        .map_err(|error| AppError::Store(error.to_string()))
}
