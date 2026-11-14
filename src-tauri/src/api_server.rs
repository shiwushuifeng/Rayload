//! LAN control API (v1) on port 6767.
//!
//! This layer knows HTTP only: device state and task control arrive through a
//! [`Backend`], so it has no Tauri, GUI or Aria2-transport dependency. The
//! listener is separate from Aria2's 6800 RPC port.
//!
//! `ping` is public for reachability checks; everything else sits behind
//! [`AuthService`] (admin credentials or a paired device's signature).

use std::future::Future;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::pin::Pin;
use std::sync::Arc;

use axum::{
    body::{to_bytes, Body},
    extract::{ConnectInfo, Path as UrlPath, State},
    http::{HeaderMap, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use socket2::{Domain, Protocol, Socket, Type};

use crate::api_auth::{
    fingerprint_of, AuthService, PairingRequestView, PendingPairData, ARCH_HEADER, DEVICE_HEADER,
    IDENTITY_HEADER, NAME_HEADER, PUBLIC_KEY_HEADER, TICKET_HEADER,
};

pub const LISTEN_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 6767);
const MAX_BODY_BYTES: usize = 1024 * 1024;

/// Binds both address families on the same port. One family failing is not an
/// error as long as the other listener starts.
pub async fn bind_listeners(port: u16) -> std::io::Result<Vec<tokio::net::TcpListener>> {
    let mut listeners = Vec::new();
    let mut last_error = None;
    for address in [
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port),
        SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), port),
    ] {
        match bind_family(address) {
            Ok(listener) => listeners.push(listener),
            Err(error) => last_error = Some(error),
        }
    }
    if listeners.is_empty() {
        return Err(last_error.unwrap_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, "no LAN listener")
        }));
    }
    Ok(listeners)
}

/// Android can leave a dual-stack IPv6 wildcard unreachable over IPv4, so bind
/// an explicit IPv4 socket and a v6-only IPv6 socket instead.
fn bind_family(address: SocketAddr) -> std::io::Result<tokio::net::TcpListener> {
    let domain = if address.is_ipv4() {
        Domain::IPV4
    } else {
        Domain::IPV6
    };
    let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_reuse_address(true)?;
    if address.is_ipv6() {
        socket.set_only_v6(true)?;
    }
    socket.bind(&address.into())?;
    socket.listen(1024)?;
    socket.set_nonblocking(true)?;
    tokio::net::TcpListener::from_std(socket.into())
}

/// Live counters the LAN dashboard polls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceStatus {
    pub download_speed: u64,
    pub active_tasks: u32,
}

/// Historical aggregates shared with paired devices for the all-devices tab.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryStats {
    pub total_bytes: u64,
    pub total_tasks: u64,
    pub days: Vec<HistoryDay>,
    pub extensions: Vec<HistoryExtension>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryDay {
    pub date: String,
    pub bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryExtension {
    pub extension: String,
    pub bytes: u64,
    pub tasks: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandoffRequest {
    pub url: String,
    #[serde(default)]
    pub filename: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HandoffResult {
    pub gid: String,
}

/// Task snapshot returned to the device that pushed it.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HandoffTask {
    pub gid: String,
    pub status: String,
    pub total_length: u64,
    pub completed_length: u64,
    pub download_speed: u64,
    pub filename: String,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskAction {
    Pause,
    Unpause,
    Remove,
}

pub type BackendFuture<T> = Pin<Box<dyn Future<Output = Result<T, String>> + Send>>;

/// Everything the LAN API needs from the application, injected so the HTTP
/// layer stays free of engine and GUI details.
pub trait Backend: Send + Sync + 'static {
    fn status(&self) -> BackendFuture<DeviceStatus>;
    fn history_stats(&self) -> BackendFuture<HistoryStats> {
        Box::pin(async { Ok(HistoryStats::default()) })
    }
    fn handoff(&self, request: HandoffRequest) -> BackendFuture<HandoffResult>;
    fn task(&self, gid: String) -> BackendFuture<HandoffTask>;
    fn task_action(&self, gid: String, action: TaskAction) -> BackendFuture<()>;
    /// Whether a human must approve incoming pairing requests.
    fn requires_pair_approval(&self) -> bool {
        false
    }
    /// Presents a pairing request to the local user.
    fn notify_pair_request(&self, _request: &PairingRequestView) {}
    /// Finishes an approved request: trust the peer and pair back if a ticket
    /// was supplied. Returns this host's device id for the reverse direction.
    fn complete_pair(&self, _data: PendingPairData) -> BackendFuture<String> {
        Box::pin(async { Ok(String::new()) })
    }
    /// Drops the stored record of a device that unpaired itself.
    fn forget_device(&self, _device_id: &str) -> BackendFuture<()> {
        Box::pin(async { Ok(()) })
    }
}

#[derive(Clone)]
struct AppState {
    backend: Arc<dyn Backend>,
    auth: Arc<AuthService>,
}

pub fn router(backend: Arc<dyn Backend>, auth: Arc<AuthService>) -> Router {
    let state = AppState { backend, auth };
    let protected = Router::new()
        .route("/api/v1/status", get(device_status))
        .route("/api/v1/history-stats", get(device_history_stats))
        .route("/api/v1/pair", post(pair))
        .route("/api/v1/pair/revoke", post(revoke))
        .route("/api/v1/pair/pending", get(pair_pending))
        .route("/api/v1/pair/pending/{id}", post(resolve_pair_request))
        .route("/api/v1/devices", get(devices))
        .route("/api/v1/devices/{id}/revoke", post(revoke_device))
        .route("/api/v1/handoff", post(handoff))
        .route("/api/v1/tasks/{gid}", get(task_status))
        .route("/api/v1/tasks/{gid}/{action}", post(task_action))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth));
    Router::new()
        .route("/api/v1/ping", get(ping))
        .route("/api/v1/pair/redeem", post(redeem))
        .merge(protected)
        .with_state(state)
}

pub async fn serve(
    listener: tokio::net::TcpListener,
    backend: Arc<dyn Backend>,
    auth: Arc<AuthService>,
) -> std::io::Result<()> {
    let address = listener.local_addr()?;
    log::info!(target: "lan_api", "lan_api_listening on {address}");
    axum::serve(
        listener,
        router(backend, auth).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
}

async fn ping(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "name": state.auth.device_name(),
        "arch": std::env::consts::ARCH,
        "fingerprint": state.auth.fingerprint(),
        // Stable identity of this host, so the peer list keeps one id across re-pairs.
        "deviceId": state.auth.fingerprint(),
    }))
}

async fn devices(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "devices": state.auth.devices() }))
}

async fn revoke_device(
    State(state): State<AppState>,
    UrlPath(device_id): UrlPath<String>,
) -> Json<serde_json::Value> {
    state.auth.revoke(&device_id);
    let _ = state.backend.forget_device(&device_id).await;
    Json(serde_json::json!({ "status": "revoked", "deviceId": device_id }))
}

async fn pair_pending(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "pending": state.auth.pending_requests() }))
}

#[derive(Debug, Deserialize)]
struct PairDecision {
    approved: bool,
}

async fn resolve_pair_request(
    State(state): State<AppState>,
    UrlPath(id): UrlPath<String>,
    Json(request): Json<PairDecision>,
) -> Response {
    match state.auth.resolve_pair(&id, request.approved) {
        Ok(()) => Json(serde_json::json!({
            "status": if request.approved { "approved" } else { "rejected" },
            "id": id,
        }))
        .into_response(),
        Err(error) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": error.as_str() })),
        )
            .into_response(),
    }
}

pub fn device_name() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .ok()
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| format!("Rayload {}", std::env::consts::OS))
}

async fn require_auth(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let (parts, body) = request.into_parts();
    let bytes = match to_bytes(body, MAX_BODY_BYTES).await {
        Ok(bytes) => bytes,
        Err(_) => {
            return (
                StatusCode::PAYLOAD_TOO_LARGE,
                Json(serde_json::json!({ "error": "request body too large" })),
            )
                .into_response()
        }
    };
    if let Err(error) =
        state
            .auth
            .authorize(&parts.method, parts.uri.path(), &parts.headers, &bytes)
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": error.as_str() })),
        )
            .into_response();
    }
    next.run(Request::from_parts(parts, Body::from(bytes)))
        .await
}

fn backend_error(error: String) -> Response {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({ "error": error })),
    )
        .into_response()
}

async fn device_status(State(state): State<AppState>) -> Response {
    match state.backend.status().await {
        Ok(value) => (StatusCode::OK, Json(value)).into_response(),
        Err(error) => backend_error(error),
    }
}

async fn device_history_stats(State(state): State<AppState>) -> Response {
    match state.backend.history_stats().await {
        Ok(value) => (StatusCode::OK, Json(value)).into_response(),
        Err(error) => backend_error(error),
    }
}

/// Exchanges public keys after the admin credentials were accepted.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemRequest {
    pub ticket: String,
    pub device_id: String,
    pub name: String,
    pub public_key: String,
}

async fn redeem(State(state): State<AppState>, Json(request): Json<RedeemRequest>) -> Response {
    let Ok(public_key) = STANDARD.decode(&request.public_key) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "public key is not valid base64" })),
        )
            .into_response();
    };
    match state.auth.redeem_ticket(
        &request.ticket,
        &request.device_id,
        &request.name,
        &public_key,
    ) {
        Ok(()) => Json(serde_json::json!({
            "status": "ok",
            "name": state.auth.device_name(),
            "fingerprint": state.auth.fingerprint(),
        }))
        .into_response(),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": error.as_str() })),
        )
            .into_response(),
    }
}

async fn pair(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> Response {
    let device = headers
        .get(DEVICE_HEADER)
        .and_then(|value| value.to_str().ok());
    let public_key = headers
        .get(PUBLIC_KEY_HEADER)
        .and_then(|value| value.to_str().ok());
    let Some(device) = device else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "device id header is missing" })),
        )
            .into_response();
    };
    let Some(public_key) = public_key else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "public key header is missing" })),
        )
            .into_response();
    };
    let Ok(public_key) = STANDARD.decode(public_key) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "public key is not valid base64" })),
        )
            .into_response();
    };
    let name = headers
        .get(NAME_HEADER)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("device");
    let arch = headers
        .get(ARCH_HEADER)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    let ticket = headers
        .get(TICKET_HEADER)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    // The identity key is what the device publishes on its ping endpoint, so
    // the prompt shows the same fingerprint the user saw while scanning.
    let fingerprint = headers
        .get(IDENTITY_HEADER)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| STANDARD.decode(value).ok())
        .filter(|bytes| bytes.len() == 32)
        .map(|bytes| fingerprint_of(&bytes))
        .unwrap_or_else(|| fingerprint_of(&public_key));
    let address = peer.ip().to_string();
    let (data, receiver) = match state.auth.begin_pair(
        device,
        name,
        &public_key,
        fingerprint,
        arch,
        &address,
        ticket,
    ) {
        Ok(value) => value,
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": error.as_str() })),
            )
                .into_response()
        }
    };

    if state.backend.requires_pair_approval() {
        state.backend.notify_pair_request(&PairingRequestView {
            id: data.id.clone(),
            name: data.name.clone(),
            fingerprint: data.fingerprint.clone(),
            arch: data.arch.clone(),
        });
        match tokio::time::timeout(std::time::Duration::from_secs(90), receiver).await {
            Ok(Ok(true)) => {}
            _ => {
                let _ = state.auth.resolve_pair(&data.id, false);
                return (
                    StatusCode::FORBIDDEN,
                    Json(serde_json::json!({ "error": "pairing request was not approved" })),
                )
                    .into_response();
            }
        }
    } else {
        // Headless hosts have no UI, so the credentials in the request are the
        // approval. Register the key before finishing the shared flow.
        if let Err(error) = state
            .auth
            .pair_named(&data.device_id, &data.name, &data.public_key)
        {
            let _ = state.auth.resolve_pair(&data.id, false);
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": error.as_str() })),
            )
                .into_response();
        }
        let _ = state.auth.resolve_pair(&data.id, true);
    }

    let device_id = match state.backend.complete_pair(data).await {
        Ok(device_id) => device_id,
        Err(error) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({ "error": error })),
            )
                .into_response()
        }
    };
    Json(serde_json::json!({
        "status": "paired",
        "serverPublicKey": state.auth.server_public_key(),
        "fingerprint": state.auth.fingerprint(),
        // Id the initiator must revoke on this host when it unpairs. Empty for
        // one-way hosts that never registered the reverse direction.
        "deviceId": device_id,
    }))
    .into_response()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RevokeRequest {
    device_id: String,
}

/// Unpairs the calling device. A device may only remove itself, so a paired
/// peer cannot kick other devices off this host.
async fn revoke(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<RevokeRequest>,
) -> Response {
    let caller = headers
        .get(DEVICE_HEADER)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    if caller.is_empty() || caller != request.device_id {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({ "error": "a device can only unpair itself" })),
        )
            .into_response();
    }
    state.auth.revoke(&request.device_id);
    if let Err(error) = state.backend.forget_device(&request.device_id).await {
        log::warn!(target: "lan_api", "unpair cleanup failed: {error}");
    }
    Json(serde_json::json!({ "status": "unpaired" })).into_response()
}

async fn handoff(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    Json(request): Json<HandoffRequest>,
) -> Response {
    if request.url.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "url is required" })),
        )
            .into_response();
    }
    // Audit trail for the headless service: who pushed which URL.
    log::info!(
        target: "lan_api",
        "handoff request from {}: {}",
        peer.ip(),
        request.url
    );
    match state.backend.handoff(request).await {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(error) => backend_error(error),
    }
}

async fn task_status(State(state): State<AppState>, UrlPath(gid): UrlPath<String>) -> Response {
    match state.backend.task(gid).await {
        Ok(task) => (StatusCode::OK, Json(task)).into_response(),
        Err(error) => backend_error(error),
    }
}

async fn task_action(
    State(state): State<AppState>,
    UrlPath((gid, action)): UrlPath<(String, String)>,
) -> Response {
    let Ok(action) = serde_json::from_value::<TaskAction>(serde_json::json!(action)) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "unknown task action" })),
        )
            .into_response();
    };
    match state.backend.task_action(gid, action).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({ "status": "ok" }))).into_response(),
        Err(error) => backend_error(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_auth::{sign_request, HEARTBEAT_TTL};
    use ed25519_dalek::SigningKey;
    use rand_core::OsRng;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct Stub {
        status: Result<DeviceStatus, &'static str>,
    }

    impl Backend for Stub {
        fn status(&self) -> BackendFuture<DeviceStatus> {
            let status = self.status;
            Box::pin(async move { status.map_err(str::to_string) })
        }

        fn handoff(&self, request: HandoffRequest) -> BackendFuture<HandoffResult> {
            Box::pin(async move {
                Ok(HandoffResult {
                    gid: format!("gid-{}", request.filename.unwrap_or_default()),
                })
            })
        }

        fn task(&self, gid: String) -> BackendFuture<HandoffTask> {
            Box::pin(async move {
                Ok(HandoffTask {
                    gid,
                    status: "active".into(),
                    total_length: 100,
                    completed_length: 40,
                    download_speed: 20,
                    filename: "file.bin".into(),
                })
            })
        }

        fn task_action(&self, _gid: String, _action: TaskAction) -> BackendFuture<()> {
            Box::pin(async move { Ok(()) })
        }
    }

    fn auth() -> Arc<AuthService> {
        Arc::new(AuthService::new("admin", "secret", HEARTBEAT_TTL))
    }

    fn basic() -> String {
        format!("Basic {}", STANDARD.encode("admin:secret"))
    }

    async fn spawn_server(
        status: Result<DeviceStatus, &'static str>,
        auth: Arc<AuthService>,
    ) -> String {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let backend: Arc<dyn Backend> = Arc::new(Stub { status });
        tokio::spawn(async move {
            serve(listener, backend, auth).await.unwrap();
        });
        format!("http://{address}")
    }

    fn client() -> reqwest::Client {
        reqwest::Client::new()
    }

    #[tokio::test]
    async fn ping_stays_public() {
        let base = spawn_server(Err("unused"), auth()).await;

        let body: serde_json::Value = reqwest::get(format!("{base}/api/v1/ping"))
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        assert_eq!(body["status"], "ok");
    }

    #[tokio::test]
    async fn status_requires_credentials() {
        let base = spawn_server(
            Ok(DeviceStatus {
                download_speed: 0,
                active_tasks: 0,
            }),
            auth(),
        )
        .await;

        let response = reqwest::get(format!("{base}/api/v1/status")).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn status_accepts_the_admin_password() {
        let base = spawn_server(
            Ok(DeviceStatus {
                download_speed: 2500,
                active_tasks: 3,
            }),
            auth(),
        )
        .await;

        let response = client()
            .get(format!("{base}/api/v1/status"))
            .header("authorization", basic())
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value = response.json().await.unwrap();
        assert_eq!(
            body,
            serde_json::json!({ "downloadSpeed": 2500, "activeTasks": 3 })
        );
    }

    #[tokio::test]
    async fn admin_can_list_and_revoke_devices() {
        let auth = auth();
        let key = SigningKey::generate(&mut OsRng);
        auth.pair_named("phone-1", "Pixel", key.verifying_key().as_bytes())
            .unwrap();
        let base = spawn_server(Err("unused"), Arc::clone(&auth)).await;

        let unauthorized = reqwest::get(format!("{base}/api/v1/devices"))
            .await
            .unwrap();
        assert_eq!(unauthorized.status(), StatusCode::UNAUTHORIZED);

        let body: serde_json::Value = client()
            .get(format!("{base}/api/v1/devices"))
            .header("authorization", basic())
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        assert_eq!(body["devices"][0]["id"], "phone-1");
        assert_eq!(body["devices"][0]["name"], "Pixel");

        let response = client()
            .post(format!("{base}/api/v1/devices/phone-1/revoke"))
            .header("authorization", basic())
            .json(&serde_json::json!({}))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(auth.devices().is_empty());
    }

    #[tokio::test]
    async fn admin_can_approve_a_pending_pairing_request() {
        let auth = auth();
        let key = SigningKey::generate(&mut OsRng);
        let (data, receiver) = auth
            .begin_pair(
                "phone-2",
                "Pixel 9",
                key.verifying_key().as_bytes(),
                "fingerprint".into(),
                "aarch64",
                "127.0.0.1",
                "",
            )
            .unwrap();
        let base = spawn_server(Err("unused"), Arc::clone(&auth)).await;

        let pending: serde_json::Value = client()
            .get(format!("{base}/api/v1/pair/pending"))
            .header("authorization", basic())
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        assert_eq!(pending["pending"][0]["id"], data.id);

        let response = client()
            .post(format!("{base}/api/v1/pair/pending/{}", data.id))
            .header("authorization", basic())
            .json(&serde_json::json!({ "approved": true }))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(receiver.await.unwrap(), true);
    }

    #[tokio::test]
    async fn status_reports_engine_failure_as_503() {
        let base = spawn_server(Err("engine offline"), auth()).await;

        let response = client()
            .get(format!("{base}/api/v1/status"))
            .header("authorization", basic())
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn paired_device_signs_requests_instead_of_sending_a_password() {
        let auth = auth();
        let base = spawn_server(
            Ok(DeviceStatus {
                download_speed: 2500,
                active_tasks: 3,
            }),
            Arc::clone(&auth),
        )
        .await;
        let client_key = SigningKey::generate(&mut OsRng);
        let http = client();

        let pairing = http
            .post(format!("{base}/api/v1/pair"))
            .header("authorization", basic())
            .header(DEVICE_HEADER, "nas-1")
            .header(NAME_HEADER, "Living room NAS")
            .header(
                PUBLIC_KEY_HEADER,
                STANDARD.encode(client_key.verifying_key().to_bytes()),
            )
            .send()
            .await
            .unwrap();
        assert_eq!(pairing.status(), StatusCode::OK);
        let pairing: serde_json::Value = pairing.json().await.unwrap();
        assert_eq!(
            pairing["serverPublicKey"].as_str().unwrap(),
            auth.server_public_key()
        );

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let signature = sign_request(&client_key, "GET", "/api/v1/status", timestamp, b"");
        let response = http
            .get(format!("{base}/api/v1/status"))
            .header(DEVICE_HEADER, "nas-1")
            .header("x-rayload-timestamp", timestamp.to_string())
            .header("x-rayload-signature", signature)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn ping_reports_a_stable_device_id() {
        let auth = auth();
        let base = spawn_server(Err("unused"), Arc::clone(&auth)).await;

        let body: serde_json::Value = reqwest::get(format!("{base}/api/v1/ping"))
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        assert_eq!(body["deviceId"].as_str(), Some(auth.fingerprint().as_str()));
    }

    #[tokio::test]
    async fn a_device_can_only_revoke_itself() {
        let auth = auth();
        let base = spawn_server(Err("unused"), Arc::clone(&auth)).await;
        let key = SigningKey::generate(&mut OsRng);
        auth.pair_named("tablet", "Tablet", key.verifying_key().as_bytes())
            .unwrap();
        let http = client();

        let signed_post = |device: &'static str, body: &'static [u8]| {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            let signature = sign_request(&key, "POST", "/api/v1/pair/revoke", timestamp, body);
            let request = http
                .post(format!("{base}/api/v1/pair/revoke"))
                .header(DEVICE_HEADER, device)
                .header("content-type", "application/json")
                .header("x-rayload-timestamp", timestamp.to_string())
                .header("x-rayload-signature", signature);
            async move { request.body(body.to_vec()).send().await.unwrap() }
        };

        assert_eq!(
            signed_post("tablet", br#"{"deviceId":"other"}"#)
                .await
                .status(),
            StatusCode::FORBIDDEN
        );
        assert_eq!(
            signed_post("tablet", br#"{"deviceId":"tablet"}"#)
                .await
                .status(),
            StatusCode::OK
        );

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let signature = sign_request(&key, "GET", "/api/v1/status", timestamp, b"");
        let response = http
            .get(format!("{base}/api/v1/status"))
            .header(DEVICE_HEADER, "tablet")
            .header("x-rayload-timestamp", timestamp.to_string())
            .header("x-rayload-signature", signature)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn handoff_creates_a_task_for_authenticated_callers() {
        let base = spawn_server(
            Ok(DeviceStatus {
                download_speed: 0,
                active_tasks: 0,
            }),
            auth(),
        )
        .await;

        let response = client()
            .post(format!("{base}/api/v1/handoff"))
            .header("authorization", basic())
            .json(&serde_json::json!({ "url": "https://example.com/a.bin", "filename": "a.bin" }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value = response.json().await.unwrap();
        assert_eq!(body["gid"], "gid-a.bin");
    }

    #[tokio::test]
    async fn task_status_and_actions_round_trip() {
        let base = spawn_server(
            Ok(DeviceStatus {
                download_speed: 0,
                active_tasks: 0,
            }),
            auth(),
        )
        .await;

        let status = client()
            .get(format!("{base}/api/v1/tasks/abc"))
            .header("authorization", basic())
            .send()
            .await
            .unwrap();
        assert_eq!(status.status(), StatusCode::OK);
        let body: serde_json::Value = status.json().await.unwrap();
        assert_eq!(body["completedLength"], 40);

        let paused = client()
            .post(format!("{base}/api/v1/tasks/abc/pause"))
            .header("authorization", basic())
            .send()
            .await
            .unwrap();
        assert_eq!(paused.status(), StatusCode::OK);

        let invalid = client()
            .post(format!("{base}/api/v1/tasks/abc/explode"))
            .header("authorization", basic())
            .send()
            .await
            .unwrap();
        assert_eq!(invalid.status(), StatusCode::BAD_REQUEST);
    }
}
