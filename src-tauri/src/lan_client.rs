//! Client side of the LAN API: pairing, signed requests, remote task tracking.
//!
//! Peer records hold the private key used to sign requests, so the file is
//! written owner-only. All crypto stays in Rust; the frontend only passes
//! addresses and credentials through Tauri commands.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use ed25519_dalek::{Signer, SigningKey};
use rand_core::OsRng;
use reqwest::{
    header::{HeaderMap, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};

use crate::api_auth::{signing_payload, AuthService, PendingPairData};
use crate::api_server::HistoryStats;

pub const API_PORT: u16 = 6767;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerView {
    pub id: String,
    pub name: String,
    pub note: String,
    pub arch: String,
    pub address: String,
    pub fingerprint: String,
    pub online: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PairResult {
    pub id: String,
    pub name: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerStatus {
    pub id: String,
    pub name: String,
    pub online: bool,
    pub download_speed: u64,
    pub active_tasks: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerHistoryStats {
    pub id: String,
    pub online: bool,
    #[serde(flatten)]
    pub stats: HistoryStats,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteTaskView {
    pub device_id: String,
    pub device_name: String,
    pub gid: String,
    pub url: String,
    pub filename: String,
    pub status: String,
    pub total_length: u64,
    pub completed_length: u64,
    pub download_speed: u64,
    pub online: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
struct Peer {
    id: String,
    /// Device id this app uses when signing requests to that peer. Older
    /// records fall back to `id`, which is what the remote registered.
    #[serde(default)]
    my_id: String,
    name: String,
    #[serde(default)]
    note: String,
    #[serde(default)]
    arch: String,
    address: String,
    private_key: String,
    fingerprint: String,
    paired_at: u64,
    #[serde(default)]
    pending: bool,
    /// Id the peer registered here when it paired back. Unpairing revokes it
    /// so the peer can no longer call this host. Empty on one-way records.
    #[serde(default)]
    inbound_id: String,
    /// One-time ticket handed to the peer; discarded when the user cancels.
    #[serde(default)]
    ticket: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct RemoteTask {
    device_id: String,
    gid: String,
    url: String,
    filename: String,
    created_at: u64,
}

#[derive(Default, Deserialize, Serialize)]
struct Store {
    #[serde(default)]
    peers: Vec<Peer>,
    #[serde(default)]
    tasks: Vec<RemoteTask>,
}

/// Groups stored peers that describe the same physical device. Pairing the
/// same host through another address or port leaves a second record behind, so
/// every read path collapses them by the host identity fingerprint.
fn peer_groups(store: &Store) -> Vec<Vec<&Peer>> {
    let mut groups: Vec<Vec<&Peer>> = Vec::new();
    let mut positions: HashMap<&str, usize> = HashMap::new();
    for peer in store.peers.iter().filter(|peer| !peer.pending) {
        if peer.fingerprint.is_empty() {
            groups.push(vec![peer]);
            continue;
        }
        match positions.get(peer.fingerprint.as_str()) {
            Some(index) => groups[*index].push(peer),
            None => {
                positions.insert(peer.fingerprint.as_str(), groups.len());
                groups.push(vec![peer]);
            }
        }
    }
    groups
}

fn file_lock() -> &'static Mutex<()> {
    static LOCK: Mutex<()> = Mutex::new(());
    &LOCK
}

fn load(path: &Path) -> Store {
    let _guard = file_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let Ok(bytes) = std::fs::read(path) else {
        return Store::default();
    };
    serde_json::from_slice(&bytes).unwrap_or_default()
}

fn save(path: &Path, store: &Store) -> Result<(), String> {
    let _guard = file_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = path.parent().ok_or("invalid LAN store path")?;
    std::fs::create_dir_all(directory).map_err(|error| error.to_string())?;
    let bytes = serde_json::to_vec_pretty(store).map_err(|error| error.to_string())?;
    let temporary = path.with_extension("json.tmp");
    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(&temporary)
            .map_err(|error| error.to_string())?;
        file.write_all(&bytes).map_err(|error| error.to_string())?;
    }
    #[cfg(not(unix))]
    std::fs::write(&temporary, &bytes).map_err(|error| error.to_string())?;
    std::fs::rename(temporary, path).map_err(|error| error.to_string())
}

fn http_client(direct: bool) -> Result<Client, String> {
    let builder = Client::builder().timeout(Duration::from_secs(10));
    let builder = if direct { builder.no_proxy() } else { builder };
    builder.build().map_err(|error| error.to_string())
}

/// Devices are reached directly first (LAN, VPN overlays, public addresses),
/// then through the system proxy. This keeps local traffic off the proxy while
/// still letting a remote device work when the proxy is the only route.
async fn send_with_fallback<F>(
    address: &str,
    timeout: Duration,
    build: F,
) -> Result<reqwest::Response, String>
where
    F: Fn(Client) -> reqwest::RequestBuilder,
{
    match build(http_client(true)?).timeout(timeout).send().await {
        Ok(response) => Ok(response),
        Err(direct_error) => build(http_client(false)?)
            .timeout(timeout)
            .send()
            .await
            .map_err(|proxy_error| {
                format!("{address}: direct failed ({direct_error}); proxy failed ({proxy_error})")
            }),
    }
}

fn base_url(address: &str) -> String {
    let address = address.trim().trim_end_matches('/');
    let with_scheme = if address.starts_with("http://") || address.starts_with("https://") {
        address.to_string()
    } else if address.starts_with('[') {
        format!("http://{address}")
    } else if address.matches(':').count() > 1 {
        format!("http://[{address}]")
    } else {
        format!("http://{address}")
    };
    let Ok(mut url) = url::Url::parse(&with_scheme) else {
        return with_scheme;
    };
    if url.port().is_none() {
        let _ = url.set_port(Some(API_PORT));
    }
    url.to_string().trim_end_matches('/').to_string()
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn signing_key(peer: &Peer) -> Result<SigningKey, String> {
    let bytes = STANDARD
        .decode(&peer.private_key)
        .map_err(|_| "stored private key is not valid base64".to_string())?;
    let bytes: [u8; 32] = bytes
        .try_into()
        .map_err(|_| "stored private key has the wrong length".to_string())?;
    Ok(SigningKey::from_bytes(&bytes))
}

fn signed_headers(peer: &Peer, method: &str, path: &str, body: &[u8]) -> Result<HeaderMap, String> {
    let timestamp = unix_now() as i64;
    let payload = signing_payload(method, path, timestamp, body);
    let signature = signing_key(peer)?.sign(payload.as_bytes());
    let mut headers = HeaderMap::new();
    let device = if peer.my_id.is_empty() {
        &peer.id
    } else {
        &peer.my_id
    };
    headers.insert("x-rayload-device", device.parse().map_err(|_| "device id")?);
    headers.insert(
        "x-rayload-timestamp",
        timestamp.to_string().parse().map_err(|_| "timestamp")?,
    );
    headers.insert(
        "x-rayload-signature",
        STANDARD
            .encode(signature.to_bytes())
            .parse()
            .map_err(|_| "signature")?,
    );
    Ok(headers)
}

/// Pairs with a remote Rayload device. The returned fingerprint must be shown
/// to the user before the device is trusted.
pub async fn pair(
    path: &Path,
    address: &str,
    username: &str,
    password: &str,
    self_name: &str,
    ticket: &str,
    identity: &str,
) -> Result<PairResult, String> {
    let base = base_url(address);
    let ping: serde_json::Value = send_with_fallback(&base, Duration::from_secs(4), |http| {
        http.get(format!("{base}/api/v1/ping"))
    })
    .await
    .map_err(|error| format!("device did not answer: {error}"))?
    .json()
    .await
    .map_err(|error| format!("device returned an invalid ping: {error}"))?;
    let name = ping["name"].as_str().unwrap_or(address).to_string();

    let key = SigningKey::generate(&mut OsRng);
    let my_id = uuid::Uuid::new_v4().to_string();
    let public_key = STANDARD.encode(key.verifying_key().to_bytes());
    // The receiver shows an approval prompt, so allow the full decision window.
    let response = send_with_fallback(&base, Duration::from_secs(120), |http| {
        http.post(format!("{base}/api/v1/pair"))
            .basic_auth(username, Some(password))
            .header("x-rayload-device", &my_id)
            .header("x-rayload-name", self_name)
            .header("x-rayload-public-key", &public_key)
            .header("x-rayload-identity", identity)
            .header("x-rayload-arch", std::env::consts::ARCH)
            .header("x-rayload-ticket", ticket)
    })
    .await?;
    let status = response.status();
    let body = response.text().await.map_err(|error| error.to_string())?;
    if !status.is_success() {
        return Err(error_message(&body, status.as_u16()));
    }
    let value: serde_json::Value =
        serde_json::from_str(&body).map_err(|error| error.to_string())?;
    let fingerprint = value["fingerprint"]
        .as_str()
        .ok_or("device did not return a fingerprint")?
        .to_string();
    // Newer hosts publish a stable id on ping; older ones omit it, so keep the
    // id this device registered under.
    let remote_id = ping["deviceId"].as_str().unwrap_or(&my_id).to_string();
    // Id the peer signs its own requests with; only newer hosts report it.
    let inbound_id = value["deviceId"].as_str().unwrap_or_default().to_string();
    let arch = ping["arch"].as_str().unwrap_or_default().to_string();

    let mut store = load(path);
    store.peers.retain(|peer| peer.address != base);
    if let Some(index) = store
        .peers
        .iter()
        .position(|peer| peer.fingerprint == fingerprint)
    {
        let previous = store.peers[index].id.clone();
        {
            let record = &mut store.peers[index];
            record.id = remote_id.clone();
            record.name = name.clone();
            record.arch = arch.clone();
            record.address = base.clone();
            record.private_key = STANDARD.encode(key.to_bytes());
            record.my_id = my_id.clone();
            record.pending = true;
            record.paired_at = unix_now();
            // Older hosts omit `deviceId`; keep whatever we already knew.
            if !inbound_id.is_empty() {
                record.inbound_id = inbound_id.clone();
            }
            record.ticket = ticket.to_string();
        }
        let mut dropped = Vec::new();
        let mut kept = Vec::with_capacity(store.peers.len());
        for peer in std::mem::take(&mut store.peers) {
            if peer.id == remote_id {
                kept.push(peer);
            } else if peer.fingerprint == fingerprint {
                dropped.push(peer.id);
            } else {
                kept.push(peer);
            }
        }
        store.peers = kept;
        for task in store.tasks.iter_mut() {
            if task.device_id == previous || dropped.contains(&task.device_id) {
                task.device_id = remote_id.clone();
            }
        }
        save(path, &store)?;
        return Ok(PairResult {
            id: remote_id,
            name,
            fingerprint,
        });
    }
    store.peers.push(Peer {
        id: remote_id.clone(),
        my_id,
        name: name.clone(),
        note: String::new(),
        arch,
        address: base,
        private_key: STANDARD.encode(key.to_bytes()),
        fingerprint: fingerprint.clone(),
        paired_at: unix_now(),
        pending: true,
        inbound_id,
        ticket: ticket.to_string(),
    });
    save(path, &store)?;
    Ok(PairResult {
        id: remote_id,
        name,
        fingerprint,
    })
}

/// Finishes an approved incoming pairing. The initiator's key is registered
/// here, then the one-time ticket is redeemed on the initiator so it trusts
/// this device in return. No password is sent in the reverse direction.
pub async fn complete_pair(
    path: &Path,
    data: &PendingPairData,
    auth: &AuthService,
) -> Result<String, String> {
    auth.pair_named(&data.device_id, &data.name, &data.public_key)
        .map_err(|error| error.as_str().to_string())?;
    if data.ticket.is_empty() {
        return Ok(String::new());
    }

    let key = SigningKey::generate(&mut OsRng);
    let my_id = uuid::Uuid::new_v4().to_string();
    let base = base_url(&data.address);
    let body = serde_json::to_vec(&serde_json::json!({
        "ticket": data.ticket,
        "deviceId": my_id,
        "name": auth.device_name(),
        "publicKey": STANDARD.encode(key.verifying_key().to_bytes()),
    }))
    .map_err(|error| error.to_string())?;
    let response = send_with_fallback(&base, Duration::from_secs(10), |http| {
        http.post(format!("{base}/api/v1/pair/redeem"))
            .header(CONTENT_TYPE, "application/json")
            .body(body.clone())
    })
    .await;
    let response = match response {
        Ok(response) => response,
        Err(error) => {
            auth.revoke(&data.device_id);
            return Err(error);
        }
    };
    let status = response.status();
    let text = response.text().await.map_err(|error| error.to_string())?;
    if !status.is_success() {
        auth.revoke(&data.device_id);
        return Err(error_message(&text, status.as_u16()));
    }
    let value: serde_json::Value =
        serde_json::from_str(&text).map_err(|error| error.to_string())?;
    if value["fingerprint"].as_str() != Some(data.fingerprint.as_str()) {
        auth.revoke(&data.device_id);
        return Err("paired device reported a different fingerprint".into());
    }

    let mut store = load(path);
    let record = Peer {
        id: data.device_id.clone(),
        my_id: my_id.clone(),
        name: data.name.clone(),
        note: String::new(),
        arch: data.arch.clone(),
        address: base,
        private_key: STANDARD.encode(key.to_bytes()),
        fingerprint: data.fingerprint.clone(),
        paired_at: unix_now(),
        pending: false,
        inbound_id: data.device_id.clone(),
        ticket: String::new(),
    };
    let existing = store.peers.iter().position(|peer| {
        peer.id == data.device_id
            || (!data.fingerprint.is_empty() && peer.fingerprint == data.fingerprint)
    });
    if let Some(index) = existing {
        let previous = store.peers[index].id.clone();
        let note = std::mem::take(&mut store.peers[index].note);
        store.peers[index] = record;
        store.peers[index].note = note;
        // Keep the tasks this side already pushed under the fresh id.
        for task in store.tasks.iter_mut() {
            if task.device_id == previous {
                task.device_id = data.device_id.clone();
            }
        }
    } else {
        store.peers.push(record);
    }
    save(path, &store)?;
    // The initiator revokes this id when it removes us.
    Ok(my_id)
}

pub fn confirm(path: &Path, id: &str) -> Result<(), String> {
    let mut store = load(path);
    let Some(peer) = store.peers.iter_mut().find(|peer| peer.id == id) else {
        return Err("device not found".into());
    };
    peer.pending = false;
    save(path, &store)
}

/// Drops a device that asked to unpair. Missing records are fine, so a retry
/// stays harmless.
pub fn remove_device(path: &Path, id: &str) -> Result<(), String> {
    let mut store = load(path);
    // The caller knows itself by the id it signs with, which is this side's
    // record id or the id it registered here.
    let removed: Vec<String> = store
        .peers
        .iter()
        .filter(|peer| peer.id == id || peer.my_id == id || peer.inbound_id == id)
        .map(|peer| peer.id.clone())
        .collect();
    store.peers.retain(|peer| !removed.contains(&peer.id));
    store
        .tasks
        .retain(|task| !removed.contains(&task.device_id));
    save(path, &store)
}

/// Removes a device locally and revokes its inbound trust. Returns the record
/// to tell the remote about, if any.
pub fn forget(path: &Path, id: &str, auth: &AuthService) -> Result<(), String> {
    let removed = take_peer_records(path, id)?;
    for peer in &removed {
        if !peer.inbound_id.is_empty() {
            auth.revoke(&peer.inbound_id);
        }
        auth.discard_ticket(&peer.ticket);
    }
    if let Some(peer) = removed.into_iter().next() {
        // The local card is already gone; an offline peer must not block the
        // UI, so the remote cleanup runs in the background.
        tauri::async_runtime::spawn(async move {
            if let Err(error) = revoke_remote(&peer).await {
                log::debug!(target: "lan_api", "remote unpair failed: {error}");
            }
        });
    }
    Ok(())
}

/// Drops every stored address record of one device, plus its tracked tasks.
fn take_peer_records(path: &Path, id: &str) -> Result<Vec<Peer>, String> {
    let mut store = load(path);
    let Some(fingerprint) = store
        .peers
        .iter()
        .find(|peer| peer.id == id)
        .map(|peer| peer.fingerprint.clone())
    else {
        return Err("device not found".into());
    };
    let removed: Vec<Peer> = store
        .peers
        .iter()
        .filter(|peer| {
            peer.id == id || (!fingerprint.is_empty() && peer.fingerprint == fingerprint)
        })
        .cloned()
        .collect();
    store
        .peers
        .retain(|peer| !removed.iter().any(|dropped| dropped.id == peer.id));
    store
        .tasks
        .retain(|task| !removed.iter().any(|peer| peer.id == task.device_id));
    save(path, &store)?;
    Ok(removed)
}

/// Asks the peer to drop this device from its own device list. Older hosts
/// have no such endpoint and answer 404, which is ignored.
async fn revoke_remote(peer: &Peer) -> Result<(), String> {
    let base = base_url(&peer.address);
    let device = if peer.my_id.is_empty() {
        &peer.id
    } else {
        &peer.my_id
    };
    let body = serde_json::to_vec(&serde_json::json!({ "deviceId": device }))
        .map_err(|error| error.to_string())?;
    let headers = signed_headers(peer, "POST", "/api/v1/pair/revoke", &body)?;
    let response = send_with_fallback(&base, Duration::from_secs(2), |http| {
        http.post(format!("{base}/api/v1/pair/revoke"))
            .headers(headers.clone())
            .header(CONTENT_TYPE, "application/json")
            .body(body.clone())
    })
    .await?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("unpair returned {}", response.status()))
    }
}

/// Applies a local change to every stored address of one physical device.
fn update_matching(path: &Path, id: &str, update: impl Fn(&mut Peer)) -> Result<(), String> {
    let mut store = load(path);
    let Some(peer) = store.peers.iter().find(|peer| peer.id == id) else {
        return Err("device not found".into());
    };
    let fingerprint = peer.fingerprint.clone();
    let mut found = false;
    for peer in store.peers.iter_mut() {
        if peer.id == id || (!fingerprint.is_empty() && peer.fingerprint == fingerprint) {
            update(peer);
            found = true;
        }
    }
    if !found {
        return Err("device not found".into());
    }
    save(path, &store)
}

/// Renames a paired device locally; the remote host is not modified.
pub fn rename(path: &Path, id: &str, name: &str) -> Result<(), String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("device name must not be empty".into());
    }
    let name: String = name.chars().take(64).collect();
    update_matching(path, id, |peer| peer.name = name.clone())
}

/// Sets a local note for a paired device; the remote host is not modified.
pub fn set_note(path: &Path, id: &str, note: &str) -> Result<(), String> {
    let note: String = note.trim().chars().take(120).collect();
    update_matching(path, id, |peer| peer.note = note.clone())
}

pub async fn devices(path: &Path) -> Result<Vec<PeerView>, String> {
    let store = load(path);
    let mut views = Vec::new();
    for group in peer_groups(&store) {
        let mut picked = group[0];
        let mut online = false;
        for peer in group.iter().copied() {
            online = send_with_fallback(&peer.address, Duration::from_millis(1500), |http| {
                http.get(format!("{}/api/v1/ping", peer.address))
            })
            .await
            .is_ok_and(|response| response.status().is_success());
            if online {
                picked = peer;
                break;
            }
        }
        views.push(PeerView {
            id: picked.id.clone(),
            name: picked.name.clone(),
            note: picked.note.clone(),
            arch: picked.arch.clone(),
            address: picked.address.clone(),
            fingerprint: picked.fingerprint.clone(),
            online,
        });
    }
    Ok(views)
}

/// Returns true when the remote device accepts the local admin credentials.
pub async fn accepts_credentials(address: &str, username: &str, password: &str) -> bool {
    let base = base_url(address);
    send_with_fallback(address, Duration::from_millis(1500), |http| {
        http.get(format!("{base}/api/v1/status"))
            .basic_auth(username, Some(password))
    })
    .await
    .is_ok_and(|response| response.status().is_success())
}

/// Reads the live counters of every paired device for the all-devices dashboard.
pub async fn device_statuses(path: &Path) -> Result<Vec<PeerStatus>, String> {
    let store = load(path);
    let mut views = Vec::new();
    for group in peer_groups(&store) {
        let answered = first_answering::<serde_json::Value>(
            &group,
            "/api/v1/status",
            Duration::from_millis(2500),
        )
        .await;
        views.push(match answered {
            Some((peer, value)) => PeerStatus {
                id: peer.id.clone(),
                name: peer.name.clone(),
                online: true,
                download_speed: value["downloadSpeed"].as_u64().unwrap_or(0),
                active_tasks: value["activeTasks"].as_u64().unwrap_or(0) as u32,
            },
            None => {
                let peer = group[0];
                PeerStatus {
                    id: peer.id.clone(),
                    name: peer.name.clone(),
                    online: false,
                    download_speed: 0,
                    active_tasks: 0,
                }
            }
        });
    }
    Ok(views)
}

/// Queries every duplicate record of one device until one answers a signed GET.
async fn first_answering<'a, T: serde::de::DeserializeOwned>(
    group: &[&'a Peer],
    route: &str,
    timeout: Duration,
) -> Option<(&'a Peer, T)> {
    for peer in group.iter().copied() {
        let Ok(headers) = signed_headers(peer, "GET", route, b"") else {
            continue;
        };
        let Ok(response) = send_with_fallback(&peer.address, timeout, |http| {
            http.get(format!("{}{route}", peer.address))
                .headers(headers.clone())
        })
        .await
        else {
            continue;
        };
        if !response.status().is_success() {
            continue;
        }
        let Ok(value) = response.json::<T>().await else {
            continue;
        };
        return Some((peer, value));
    }
    None
}

/// Reads compact historical aggregates from every paired device.
pub async fn device_history_stats(path: &Path) -> Result<Vec<PeerHistoryStats>, String> {
    let store = load(path);
    let mut views = Vec::new();
    for group in peer_groups(&store) {
        let answered = first_answering::<HistoryStats>(
            &group,
            "/api/v1/history-stats",
            Duration::from_millis(2500),
        )
        .await;
        views.push(match answered {
            Some((peer, stats)) => PeerHistoryStats {
                id: peer.id.clone(),
                online: true,
                stats,
            },
            None => {
                let peer = group[0];
                PeerHistoryStats {
                    id: peer.id.clone(),
                    online: false,
                    stats: HistoryStats::default(),
                }
            }
        });
    }
    Ok(views)
}

pub async fn handoff(
    path: &Path,
    device_id: &str,
    url: &str,
    filename: Option<String>,
) -> Result<RemoteTaskView, String> {
    let store = load(path);
    let peer = store
        .peers
        .iter()
        .find(|peer| peer.id == device_id && !peer.pending)
        .cloned()
        .ok_or("device is not paired")?;
    let body = serde_json::to_vec(&serde_json::json!({
        "url": url,
        "filename": filename,
    }))
    .map_err(|error| error.to_string())?;
    let headers = signed_headers(&peer, "POST", "/api/v1/handoff", &body)?;
    let response = send_with_fallback(&peer.address, Duration::from_secs(10), |http| {
        http.post(format!("{}/api/v1/handoff", peer.address))
            .headers(headers.clone())
            .header(CONTENT_TYPE, "application/json")
            .body(body.clone())
    })
    .await?;
    let status = response.status();
    let text = response.text().await.map_err(|error| error.to_string())?;
    if !status.is_success() {
        return Err(error_message(&text, status.as_u16()));
    }
    let value: serde_json::Value =
        serde_json::from_str(&text).map_err(|error| error.to_string())?;
    let gid = value["gid"]
        .as_str()
        .ok_or("device did not return a gid")?
        .to_string();
    let filename = filename.unwrap_or_default();

    let mut store = load(path);
    store.tasks.push(RemoteTask {
        device_id: peer.id.clone(),
        gid: gid.clone(),
        url: url.to_string(),
        filename: filename.clone(),
        created_at: unix_now(),
    });
    save(path, &store)?;

    Ok(RemoteTaskView {
        device_id: peer.id,
        device_name: peer.name,
        gid,
        url: url.to_string(),
        filename,
        status: "waiting".into(),
        total_length: 0,
        completed_length: 0,
        download_speed: 0,
        online: true,
        error: None,
    })
}

pub async fn remote_tasks(path: &Path) -> Result<Vec<RemoteTaskView>, String> {
    let store = load(path);
    let mut views = Vec::new();
    for task in &store.tasks {
        let Some(peer) = store.peers.iter().find(|peer| peer.id == task.device_id) else {
            continue;
        };
        let route = format!("/api/v1/tasks/{}", task.gid);
        let offline = |error: String| RemoteTaskView {
            device_id: task.device_id.clone(),
            device_name: peer.name.clone(),
            gid: task.gid.clone(),
            url: task.url.clone(),
            filename: task.filename.clone(),
            status: "error".into(),
            total_length: 0,
            completed_length: 0,
            download_speed: 0,
            online: false,
            error: Some(error),
        };
        let Ok(headers) = signed_headers(peer, "GET", &route, b"") else {
            views.push(offline("stored key is invalid".into()));
            continue;
        };
        match send_with_fallback(&peer.address, Duration::from_secs(10), |http| {
            http.get(format!("{}{route}", peer.address))
                .headers(headers.clone())
        })
        .await
        {
            Ok(response) if response.status().is_success() => {
                let Ok(value) = response.json::<serde_json::Value>().await else {
                    views.push(offline("invalid task response".into()));
                    continue;
                };
                views.push(RemoteTaskView {
                    device_id: task.device_id.clone(),
                    device_name: peer.name.clone(),
                    gid: task.gid.clone(),
                    url: task.url.clone(),
                    filename: value["filename"]
                        .as_str()
                        .filter(|name| !name.is_empty())
                        .unwrap_or(&task.filename)
                        .to_string(),
                    status: value["status"].as_str().unwrap_or("unknown").to_string(),
                    total_length: value["totalLength"].as_u64().unwrap_or(0),
                    completed_length: value["completedLength"].as_u64().unwrap_or(0),
                    download_speed: value["downloadSpeed"].as_u64().unwrap_or(0),
                    online: true,
                    error: None,
                });
            }
            Ok(response) => views.push(offline(format!(
                "device returned HTTP {}",
                response.status()
            ))),
            Err(error) => views.push(offline(error.to_string())),
        }
    }
    Ok(views)
}

pub async fn remote_action(
    path: &Path,
    device_id: &str,
    gid: &str,
    action: &str,
) -> Result<(), String> {
    if !matches!(action, "pause" | "unpause" | "remove") {
        return Err("unknown task action".into());
    }
    let store = load(path);
    let peer = store
        .peers
        .iter()
        .find(|peer| peer.id == device_id)
        .cloned()
        .ok_or("device is not paired")?;
    if action == "remove" {
        let mut store = load(path);
        store
            .tasks
            .retain(|task| !(task.device_id == device_id && task.gid == gid));
        save(path, &store)?;
    }
    let route = format!("/api/v1/tasks/{gid}/{action}");
    let headers = signed_headers(&peer, "POST", &route, b"")?;
    let response = send_with_fallback(&peer.address, Duration::from_secs(10), |http| {
        http.post(format!("{}{route}", peer.address))
            .headers(headers.clone())
    })
    .await;
    // Removing an offline peer must not leave a card the user cannot dismiss.
    let Ok(response) = response else {
        return if action == "remove" {
            Ok(())
        } else {
            Err("device is offline".into())
        };
    };
    if !response.status().is_success() {
        let status = response.status().as_u16();
        let text = response.text().await.unwrap_or_default();
        if action != "remove" {
            return Err(error_message(&text, status));
        }
    }
    Ok(())
}

fn error_message(body: &str, status: u16) -> String {
    serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|value| value["error"].as_str().map(str::to_string))
        .unwrap_or_else(|| format!("device returned HTTP {status}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::Arc;

    #[test]
    fn base_url_adds_scheme_and_port_once() {
        assert_eq!(base_url("192.168.1.5"), "http://192.168.1.5:6767");
        assert_eq!(base_url("192.168.1.5:7000"), "http://192.168.1.5:7000");
        assert_eq!(base_url("http://nas.local/"), "http://nas.local:6767");
        assert_eq!(base_url("http://nas.local:6767"), "http://nas.local:6767");
        assert_eq!(base_url("[fe80::1]"), "http://[fe80::1]:6767");
        assert_eq!(base_url("[fe80::1]:7000"), "http://[fe80::1]:7000");
        assert_eq!(base_url("fe80::1"), "http://[fe80::1]:6767");
        assert_eq!(
            base_url("http://[2001:db8::5]"),
            "http://[2001:db8::5]:6767"
        );
    }

    #[test]
    fn pending_devices_are_persisted_and_can_be_confirmed() {
        let directory = std::env::temp_dir().join(format!("rayload-peers-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join("peers.json");
        let mut store = Store::default();
        store.peers.push(Peer {
            id: "p1".into(),
            my_id: String::new(),
            name: "NAS".into(),
            note: String::new(),
            arch: "x86_64".into(),
            address: "http://127.0.0.1:6767".into(),
            private_key: STANDARD.encode([7u8; 32]),
            fingerprint: "aa:bb".into(),
            paired_at: 1,
            pending: true,
            inbound_id: String::new(),
            ticket: String::new(),
        });
        save(&path, &store).unwrap();

        assert!(load(&path).peers[0].pending);
        confirm(&path, "p1").unwrap();
        assert!(!load(&path).peers[0].pending);
        rename(&path, "p1", "Living room NAS").unwrap();
        assert_eq!(load(&path).peers[0].name, "Living room NAS");
        take_peer_records(&path, "p1").unwrap();
        assert!(load(&path).peers.is_empty());
    }

    #[test]
    fn unpairing_by_the_callers_own_id_drops_the_card() {
        let directory =
            std::env::temp_dir().join(format!("rayload-peers-unpair-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join("peers.json");
        let mut store = Store::default();
        let mut record = peer("remote-fingerprint", "http://192.168.1.5:6767", "aa:bb");
        record.my_id = "local-id".into();
        record.inbound_id = "remote-signing-id".into();
        store.peers.push(record);
        store.tasks.push(RemoteTask {
            device_id: "remote-fingerprint".into(),
            gid: "g1".into(),
            url: "http://example.com/a".into(),
            filename: "a".into(),
            created_at: 1,
        });
        save(&path, &store).unwrap();

        remove_device(&path, "remote-signing-id").unwrap();

        let cleared = load(&path);
        assert!(cleared.peers.is_empty());
        assert!(cleared.tasks.is_empty());
    }

    fn peer(id: &str, address: &str, fingerprint: &str) -> Peer {
        Peer {
            id: id.into(),
            my_id: String::new(),
            name: "Tablet".into(),
            note: String::new(),
            arch: "aarch64".into(),
            address: address.into(),
            private_key: STANDARD.encode([9u8; 32]),
            fingerprint: fingerprint.into(),
            paired_at: 1,
            pending: false,
            inbound_id: String::new(),
            ticket: String::new(),
        }
    }

    #[test]
    fn signed_headers_identify_this_side_of_the_pair() {
        let mut record = peer("remote-id", "http://127.0.0.1:6767", "aa:bb");
        let headers = signed_headers(&record, "GET", "/api/v1/status", b"").unwrap();
        assert_eq!(headers["x-rayload-device"], "remote-id");

        record.my_id = "local-id".into();
        let headers = signed_headers(&record, "GET", "/api/v1/status", b"").unwrap();
        assert_eq!(headers["x-rayload-device"], "local-id");
    }

    #[test]
    fn one_device_reached_over_several_addresses_is_one_group() {
        let mut store = Store::default();
        store
            .peers
            .push(peer("a", "http://192.168.1.5:6767", "aa:bb"));
        store
            .peers
            .push(peer("b", "http://[fc00::5]:6767", "aa:bb"));
        store
            .peers
            .push(peer("c", "http://192.168.1.6:6767", "cc:dd"));
        store.peers.push(peer("d", "http://192.168.1.7:6767", ""));

        let groups = peer_groups(&store);
        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0].len(), 2);
        assert_eq!(groups[1].len(), 1);
        assert_eq!(groups[2].len(), 1);
    }

    #[test]
    fn forget_rename_and_note_touch_every_address_of_a_device() {
        let directory =
            std::env::temp_dir().join(format!("rayload-peers-dup-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join("peers.json");
        let mut store = Store::default();
        store
            .peers
            .push(peer("a", "http://192.168.1.5:6767", "aa:bb"));
        store
            .peers
            .push(peer("b", "http://[fc00::5]:6767", "aa:bb"));
        store.tasks.push(RemoteTask {
            device_id: "b".into(),
            gid: "g1".into(),
            url: "http://example.com/a".into(),
            filename: "a".into(),
            created_at: 1,
        });
        save(&path, &store).unwrap();

        rename(&path, "a", "Desk tablet").unwrap();
        let renamed = load(&path);
        assert!(renamed.peers.iter().all(|peer| peer.name == "Desk tablet"));

        set_note(&path, "a", "Bedroom").unwrap();
        let noted = load(&path);
        assert!(noted.peers.iter().all(|peer| peer.note == "Bedroom"));

        let removed = take_peer_records(&path, "a").unwrap();
        assert_eq!(removed.len(), 2);
        let cleared = load(&path);
        assert!(cleared.peers.is_empty());
        assert!(cleared.tasks.is_empty());
    }

    /// Two real HTTP hosts: pair both ways, push a task, unpair, then pair again.
    struct TestHost {
        auth: Arc<AuthService>,
        store: PathBuf,
        received: Arc<Mutex<Vec<String>>>,
        /// Address the peer can be reached at; the test uses ephemeral ports,
        /// while a real host derives `ip:6767` from the request itself.
        reverse_address: Arc<Mutex<Option<String>>>,
    }

    impl crate::api_server::Backend for TestHost {
        fn status(&self) -> crate::api_server::BackendFuture<crate::api_server::DeviceStatus> {
            Box::pin(async {
                Ok(crate::api_server::DeviceStatus {
                    download_speed: 7,
                    active_tasks: 1,
                })
            })
        }

        fn handoff(
            &self,
            request: crate::api_server::HandoffRequest,
        ) -> crate::api_server::BackendFuture<crate::api_server::HandoffResult> {
            self.received.lock().unwrap().push(request.url);
            Box::pin(async { Ok(crate::api_server::HandoffResult { gid: "g1".into() }) })
        }

        fn task(
            &self,
            gid: String,
        ) -> crate::api_server::BackendFuture<crate::api_server::HandoffTask> {
            Box::pin(async move {
                Ok(crate::api_server::HandoffTask {
                    gid,
                    status: "active".into(),
                    total_length: 10,
                    completed_length: 5,
                    download_speed: 1,
                    filename: "a.bin".into(),
                })
            })
        }

        fn task_action(
            &self,
            _gid: String,
            _action: crate::api_server::TaskAction,
        ) -> crate::api_server::BackendFuture<()> {
            Box::pin(async { Ok(()) })
        }

        fn requires_pair_approval(&self) -> bool {
            true
        }

        // Stands in for the user tapping "Allow".
        fn notify_pair_request(&self, request: &crate::api_auth::PairingRequestView) {
            let auth = Arc::clone(&self.auth);
            let id = request.id.clone();
            tokio::spawn(async move {
                let _ = auth.resolve_pair(&id, true);
            });
        }

        fn complete_pair(
            &self,
            data: crate::api_auth::PendingPairData,
        ) -> crate::api_server::BackendFuture<String> {
            let auth = Arc::clone(&self.auth);
            let store = self.store.clone();
            let reverse = self.reverse_address.lock().unwrap().clone();
            Box::pin(async move {
                let mut data = data;
                if let Some(address) = reverse {
                    data.address = address;
                }
                complete_pair(&store, &data, &auth).await
            })
        }

        fn forget_device(&self, device_id: &str) -> crate::api_server::BackendFuture<()> {
            let store = self.store.clone();
            let id = device_id.to_string();
            Box::pin(async move { remove_device(&store, &id) })
        }
    }

    async fn spawn_host(store: PathBuf, name: &str) -> (String, Arc<AuthService>, Arc<TestHost>) {
        let auth = Arc::new(AuthService::new(
            "admin",
            "secret",
            crate::api_auth::HEARTBEAT_TTL,
        ));
        auth.set_device_name(name);
        let host = Arc::new(TestHost {
            auth: Arc::clone(&auth),
            store,
            received: Arc::new(Mutex::new(Vec::new())),
            reverse_address: Arc::new(Mutex::new(None)),
        });
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let backend: Arc<dyn crate::api_server::Backend> = host.clone();
        tokio::spawn(async move {
            let _ = crate::api_server::serve(listener, backend, auth).await;
        });
        (format!("127.0.0.1:{port}"), host.auth.clone(), host)
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn pairing_handoff_unpair_and_repair_round_trip() {
        let directory =
            std::env::temp_dir().join(format!("rayload-pair-roundtrip-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let store_a = directory.join("a.json");
        let store_b = directory.join("b.json");
        let (address_b, auth_b, host_b) = spawn_host(store_b.clone(), "Tablet B").await;
        let (address_a, auth_a, _host_a) = spawn_host(store_a.clone(), "Desk A").await;
        *host_b.reverse_address.lock().unwrap() = Some(address_a.clone());

        // Pair A -> B, with A's user trusting the fingerprint B published.
        let ticket = auth_a.create_ticket();
        let paired = pair(
            &store_a,
            &address_b,
            "admin",
            "secret",
            "Desk A",
            &ticket,
            &auth_a.server_public_key(),
        )
        .await
        .unwrap();
        assert_eq!(paired.fingerprint, auth_b.fingerprint());
        confirm(&store_a, &paired.id).unwrap();

        assert_eq!(devices(&store_a).await.unwrap().len(), 1);
        assert_eq!(devices(&store_b).await.unwrap().len(), 1);

        // A pushes a task to B.
        let task = handoff(&store_a, &paired.id, "http://example.com/a.bin", None)
            .await
            .unwrap();
        assert_eq!(task.device_id, paired.id);
        assert_eq!(
            host_b.received.lock().unwrap().as_slice(),
            ["http://example.com/a.bin"]
        );

        // Cancelling/removing drops both cards and revokes both keys.
        let signed = load(&store_a).peers[0].clone();
        forget(&store_a, &paired.id, &auth_a).unwrap();
        assert!(devices(&store_a).await.unwrap().is_empty());
        let headers = signed_headers(&signed, "GET", "/api/v1/status", b"").unwrap();
        let mut revoked = false;
        for _ in 0..80 {
            let status = reqwest::Client::new()
                .get(format!("http://{address_b}/api/v1/status"))
                .headers(headers.clone())
                .send()
                .await
                .unwrap()
                .status();
            if status == reqwest::StatusCode::UNAUTHORIZED {
                revoked = true;
                break;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        assert!(revoked, "B kept trusting A after the unpair");
        assert!(devices(&store_b).await.unwrap().is_empty());

        // Pairing again after the unpair still works.
        let ticket = auth_a.create_ticket();
        let again = pair(
            &store_a,
            &address_b,
            "admin",
            "secret",
            "Desk A",
            &ticket,
            &auth_a.server_public_key(),
        )
        .await
        .unwrap();
        confirm(&store_a, &again.id).unwrap();
        assert_eq!(devices(&store_a).await.unwrap().len(), 1);
        assert_eq!(devices(&store_b).await.unwrap().len(), 1);
        handoff(&store_a, &again.id, "http://example.com/b.bin", None)
            .await
            .unwrap();
        let _ = (address_a, host_b);
    }
}
