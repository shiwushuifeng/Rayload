//! Two-layer authentication for the LAN API.
//!
//! Layer one is HTTP Basic with the configured admin credentials. A paired
//! device then signs each request with its ed25519 key and skips the password.
//! Any verified request refreshes the device heartbeat; a device idle for
//! [`HEARTBEAT_TTL`] is revoked.
//!
//! shortcut: paired devices live in memory, so an app restart requires pairing
//! again; persist the device map when the LAN page ships.

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use axum::http::{HeaderMap, Method};
use base64::{engine::general_purpose::STANDARD, Engine as _};
#[cfg(test)]
use ed25519_dalek::Signer;
use ed25519_dalek::{Signature, SigningKey, Verifier, VerifyingKey};
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::sync::oneshot;

pub const HEARTBEAT_TTL: Duration = Duration::from_secs(72 * 60 * 60);
/// Signed requests older or newer than this are refused (replay window).
pub const CLOCK_SKEW: Duration = Duration::from_secs(300);
/// A pairing request must be approved within this window.
pub const PAIR_TTL: Duration = Duration::from_secs(120);
/// A reverse-pairing ticket can be redeemed once within this window.
pub const TICKET_TTL: Duration = Duration::from_secs(300);
/// A device counts as online while its heartbeat is this fresh.
pub const DEVICE_ONLINE_WINDOW: Duration = Duration::from_secs(120);

pub const DEVICE_HEADER: &str = "x-rayload-device";
pub const PUBLIC_KEY_HEADER: &str = "x-rayload-public-key";
pub const IDENTITY_HEADER: &str = "x-rayload-identity";
pub const NAME_HEADER: &str = "x-rayload-name";
pub const ARCH_HEADER: &str = "x-rayload-arch";
pub const TICKET_HEADER: &str = "x-rayload-ticket";
pub const TIMESTAMP_HEADER: &str = "x-rayload-timestamp";
pub const SIGNATURE_HEADER: &str = "x-rayload-signature";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthError {
    NotConfigured,
    MissingCredentials,
    InvalidCredentials,
    UnknownDevice,
    Expired,
    BadTimestamp,
    BadSignature,
    BadDeviceId,
    BadPublicKey,
    BadTicket,
}

impl AuthError {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotConfigured => "LAN API credentials are not configured",
            Self::MissingCredentials => "authentication required",
            Self::InvalidCredentials => "invalid username or password",
            Self::UnknownDevice => "device is not paired",
            Self::Expired => "device authorization expired",
            Self::BadTimestamp => "request timestamp is outside the allowed window",
            Self::BadSignature => "request signature is invalid",
            Self::BadDeviceId => "device id is invalid",
            Self::BadPublicKey => "public key must be 32 bytes",
            Self::BadTicket => "pairing ticket is invalid or expired",
        }
    }
}

struct Device {
    name: String,
    key: VerifyingKey,
    last_seen: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceView {
    pub id: String,
    pub name: String,
    pub last_seen: u64,
    pub online: bool,
}

#[derive(Default, Deserialize, Serialize)]
struct DeviceFile {
    #[serde(default)]
    devices: HashMap<String, DeviceRecord>,
}

#[derive(Deserialize, Serialize)]
struct DeviceRecord {
    #[serde(default)]
    name: String,
    public_key: String,
    last_seen: u64,
}

/// A pairing request waiting for the local user to approve it.
struct PendingPair {
    name: String,
    fingerprint: String,
    arch: String,
    created: Instant,
    responder: Option<oneshot::Sender<bool>>,
}

/// What the UI shows for an incoming pairing request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PairingRequestView {
    pub id: String,
    pub name: String,
    pub fingerprint: String,
    pub arch: String,
}

/// Everything the backend needs to finish a pairing request.
#[derive(Debug, Clone)]
pub struct PendingPairData {
    pub id: String,
    pub device_id: String,
    pub name: String,
    pub public_key: Vec<u8>,
    /// Fingerprint of the peer's stable identity key, shown to the user.
    pub fingerprint: String,
    pub arch: String,
    pub address: String,
    pub ticket: String,
}

/// The exact bytes a client signs. Kept public so clients implement it once.
pub fn signing_payload(method: &str, path: &str, timestamp: i64, body: &[u8]) -> String {
    let digest = Sha256::digest(body);
    let mut hash = String::with_capacity(64);
    for byte in digest {
        hash.push_str(&format!("{byte:02x}"));
    }
    format!("{method}\n{path}\n{timestamp}\n{hash}")
}

pub struct AuthService {
    credentials: RwLock<Option<(String, String)>>,
    devices: Mutex<HashMap<String, Device>>,
    devices_path: Option<PathBuf>,
    last_persist: Mutex<Instant>,
    server_key: SigningKey,
    device_name: RwLock<String>,
    ttl: Duration,
    pending: Mutex<HashMap<String, PendingPair>>,
    tickets: Mutex<HashMap<String, Instant>>,
}

impl AuthService {
    pub fn new(username: &str, password: &str, ttl: Duration) -> Self {
        Self {
            credentials: RwLock::new(configured(username, password)),
            devices: Mutex::new(HashMap::new()),
            devices_path: None,
            last_persist: Mutex::new(Instant::now()),
            server_key: SigningKey::generate(&mut OsRng),
            device_name: RwLock::new(String::new()),
            ttl,
            pending: Mutex::new(HashMap::new()),
            tickets: Mutex::new(HashMap::new()),
        }
    }

    /// Loads the server identity so its fingerprint stays stable across
    /// restarts; creates it with owner-only permissions on first run.
    pub fn load_or_create(
        path: &Path,
        username: &str,
        password: &str,
        ttl: Duration,
    ) -> io::Result<Self> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let server_key = match fs::read(path) {
            Ok(bytes) => SigningKey::from_bytes(&bytes.as_slice().try_into().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "invalid LAN identity file")
            })?),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                let key = SigningKey::generate(&mut OsRng);
                #[cfg(unix)]
                {
                    use std::io::Write;
                    use std::os::unix::fs::OpenOptionsExt;
                    let mut file = fs::OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .mode(0o600)
                        .open(path)?;
                    file.write_all(&key.to_bytes())?;
                }
                #[cfg(not(unix))]
                fs::write(path, key.to_bytes())?;
                key
            }
            Err(error) => return Err(error),
        };
        let devices_path = path.with_file_name("devices.json");
        let devices = load_devices(&devices_path);
        Ok(Self {
            credentials: RwLock::new(configured(username, password)),
            devices: Mutex::new(devices),
            devices_path: Some(devices_path),
            last_persist: Mutex::new(Instant::now()),
            server_key,
            device_name: RwLock::new(String::new()),
            ttl,
            pending: Mutex::new(HashMap::new()),
            tickets: Mutex::new(HashMap::new()),
        })
    }

    pub fn set_credentials(&self, username: &str, password: &str) {
        *self
            .credentials
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = configured(username, password);
    }

    /// Name this device advertises on the LAN and sends when pairing.
    pub fn set_device_name(&self, name: &str) {
        *self
            .device_name
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner) =
            name.trim().chars().take(64).collect();
    }

    pub fn device_name(&self) -> String {
        let name = self
            .device_name
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if name.is_empty() {
            format!("Rayload {}", std::env::consts::OS)
        } else {
            name.clone()
        }
    }

    pub fn is_configured(&self) -> bool {
        self.credentials
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .is_some()
    }

    pub fn username(&self) -> String {
        self.credentials
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .as_ref()
            .map(|(username, _)| username.clone())
            .unwrap_or_default()
    }

    /// Credentials for the pairing QR code. The password never leaves the
    /// device except inside a QR the user explicitly opens.
    pub fn credentials(&self) -> Option<(String, String)> {
        self.credentials
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
    }

    pub fn server_public_key(&self) -> String {
        STANDARD.encode(self.server_key.verifying_key().to_bytes())
    }

    /// Short human-readable identity used for out-of-band fingerprint checks.
    pub fn fingerprint(&self) -> String {
        fingerprint_of(&self.server_key.verifying_key().to_bytes())
    }

    /// Issues a one-time ticket the peer may redeem to register its key here.
    pub fn create_ticket(&self) -> String {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        let ticket = STANDARD.encode(bytes);
        let now = Instant::now();
        let mut tickets = self
            .tickets
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        tickets.retain(|_, created| now.duration_since(*created) <= TICKET_TTL);
        tickets.insert(ticket.clone(), now);
        ticket
    }

    /// Consumes a ticket and trusts the peer key it carries.
    pub fn redeem_ticket(
        &self,
        ticket: &str,
        device_id: &str,
        name: &str,
        public_key: &[u8],
    ) -> Result<(), AuthError> {
        let now = Instant::now();
        let valid = self
            .tickets
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(ticket)
            .is_some_and(|created| now.duration_since(created) <= TICKET_TTL);
        if !valid {
            return Err(AuthError::BadTicket);
        }
        self.pair_named(device_id, name, public_key).map(|_| ())
    }

    /// Drops a ticket the local user cancelled, so the peer cannot redeem it
    /// after the pairing was rejected.
    pub fn discard_ticket(&self, ticket: &str) {
        if ticket.is_empty() {
            return;
        }
        self.tickets
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(ticket);
    }

    /// Registers a pairing request and hands back a signal for its decision.
    /// The peer is not trusted until `redeem_ticket` (reverse) or
    /// `pair_named` (one-way fallback) runs.
    pub fn begin_pair(
        &self,
        device_id: &str,
        name: &str,
        public_key: &[u8],
        fingerprint: String,
        arch: &str,
        address: &str,
        ticket: &str,
    ) -> Result<(PendingPairData, oneshot::Receiver<bool>), AuthError> {
        if device_id.is_empty()
            || device_id.len() > 128
            || !device_id.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.')
            })
        {
            return Err(AuthError::BadDeviceId);
        }
        VerifyingKey::from_bytes(&public_key.try_into().map_err(|_| AuthError::BadPublicKey)?)
            .map_err(|_| AuthError::BadPublicKey)?;

        let id = uuid::Uuid::new_v4().to_string();
        let data = PendingPairData {
            id: id.clone(),
            device_id: device_id.to_string(),
            name: name.chars().take(64).collect(),
            public_key: public_key.to_vec(),
            fingerprint,
            arch: arch.chars().take(32).collect(),
            address: address.to_string(),
            ticket: ticket.to_string(),
        };
        let (sender, receiver) = oneshot::channel();
        let now = Instant::now();
        let mut pending = self
            .pending
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        pending.retain(|_, request| now.duration_since(request.created) <= PAIR_TTL);
        pending.insert(
            id,
            PendingPair {
                name: data.name.clone(),
                fingerprint: data.fingerprint.clone(),
                arch: data.arch.clone(),
                created: now,
                responder: Some(sender),
            },
        );
        Ok((data, receiver))
    }

    /// Pairing requests still waiting for the local user.
    pub fn pending_requests(&self) -> Vec<PairingRequestView> {
        let now = Instant::now();
        let mut pending = self
            .pending
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        pending.retain(|_, request| now.duration_since(request.created) <= PAIR_TTL);
        pending
            .iter()
            .map(|(id, request)| PairingRequestView {
                id: id.clone(),
                name: request.name.clone(),
                fingerprint: request.fingerprint.clone(),
                arch: request.arch.clone(),
            })
            .collect()
    }

    /// Resolves a pending request; the waiting handler completes the pairing.
    pub fn resolve_pair(&self, id: &str, approved: bool) -> Result<(), AuthError> {
        let mut pending = self
            .pending
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(mut request) = pending.remove(id) else {
            return Err(AuthError::BadTicket);
        };
        if let Some(responder) = request.responder.take() {
            let _ = responder.send(approved);
        }
        Ok(())
    }

    /// Registers or rotates a device key and returns the server public key.
    pub fn pair(&self, device_id: &str, public_key: &[u8]) -> Result<String, AuthError> {
        self.pair_named(device_id, "device", public_key)
    }

    /// Registers a device with a display name for the host's device list.
    pub fn pair_named(
        &self,
        device_id: &str,
        name: &str,
        public_key: &[u8],
    ) -> Result<String, AuthError> {
        if device_id.is_empty()
            || device_id.len() > 128
            || !device_id.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.')
            })
        {
            return Err(AuthError::BadDeviceId);
        }
        let key =
            VerifyingKey::from_bytes(&public_key.try_into().map_err(|_| AuthError::BadPublicKey)?)
                .map_err(|_| AuthError::BadPublicKey)?;

        let now = unix_now();
        let mut devices = self
            .devices
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        devices.retain(|_, device| now.saturating_sub(device.last_seen) <= self.ttl.as_secs());
        devices.insert(
            device_id.to_string(),
            Device {
                name: name.chars().take(64).collect(),
                key,
                last_seen: now,
            },
        );
        self.persist(&devices);
        Ok(self.server_public_key())
    }

    /// Drops a device this host no longer trusts.
    pub fn revoke(&self, device_id: &str) {
        let mut devices = self
            .devices
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if devices.remove(device_id).is_some() {
            self.persist(&devices);
        }
    }

    /// Paired devices, newest heartbeat first, with an online flag for the CLI.
    pub fn devices(&self) -> Vec<DeviceView> {
        let now = unix_now();
        let devices = self
            .devices
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut views: Vec<DeviceView> = devices
            .iter()
            .filter(|(_, device)| now.saturating_sub(device.last_seen) <= self.ttl.as_secs())
            .map(|(id, device)| DeviceView {
                id: id.clone(),
                name: device.name.clone(),
                last_seen: device.last_seen,
                online: now.saturating_sub(device.last_seen) <= DEVICE_ONLINE_WINDOW.as_secs(),
            })
            .collect();
        views.sort_by(|left, right| {
            right
                .online
                .cmp(&left.online)
                .then(right.last_seen.cmp(&left.last_seen))
                .then(left.name.cmp(&right.name))
        });
        views
    }

    /// Accepts either a valid device signature or the admin credentials.
    pub fn authorize(
        &self,
        method: &Method,
        path: &str,
        headers: &HeaderMap,
        body: &[u8],
    ) -> Result<(), AuthError> {
        if headers.contains_key(SIGNATURE_HEADER) {
            return self.authorize_signature(method, path, headers, body);
        }
        self.authorize_basic(headers)
    }

    fn persist(&self, devices: &HashMap<String, Device>) {
        let Some(path) = &self.devices_path else {
            return;
        };
        let file = DeviceFile {
            devices: devices
                .iter()
                .map(|(id, device)| {
                    (
                        id.clone(),
                        DeviceRecord {
                            name: device.name.clone(),
                            public_key: STANDARD.encode(device.key.to_bytes()),
                            last_seen: device.last_seen,
                        },
                    )
                })
                .collect(),
        };
        if let Err(error) =
            write_atomic(path, &serde_json::to_vec_pretty(&file).unwrap_or_default())
        {
            log::warn!(target: "lan_api", event = "lan_devices_persist_failed", error:% = error; "lan_devices_persist_failed");
        }
    }

    fn authorize_basic(&self, headers: &HeaderMap) -> Result<(), AuthError> {
        let credentials = self
            .credentials
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some((username, password)) = credentials.as_ref() else {
            return Err(AuthError::NotConfigured);
        };
        let header = headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(AuthError::MissingCredentials)?;
        let encoded = header
            .strip_prefix("Basic ")
            .ok_or(AuthError::MissingCredentials)?;
        let decoded = STANDARD
            .decode(encoded.trim())
            .map_err(|_| AuthError::InvalidCredentials)?;
        let text = String::from_utf8(decoded).map_err(|_| AuthError::InvalidCredentials)?;
        let (user, pass) = text.split_once(':').ok_or(AuthError::InvalidCredentials)?;
        if constant_time_eq(user.as_bytes(), username.as_bytes())
            && constant_time_eq(pass.as_bytes(), password.as_bytes())
        {
            Ok(())
        } else {
            Err(AuthError::InvalidCredentials)
        }
    }

    fn authorize_signature(
        &self,
        method: &Method,
        path: &str,
        headers: &HeaderMap,
        body: &[u8],
    ) -> Result<(), AuthError> {
        let device_id = header_str(headers, DEVICE_HEADER).ok_or(AuthError::BadDeviceId)?;
        let timestamp: i64 = header_str(headers, TIMESTAMP_HEADER)
            .and_then(|value| value.parse().ok())
            .ok_or(AuthError::BadTimestamp)?;
        let signature = STANDARD
            .decode(header_str(headers, SIGNATURE_HEADER).ok_or(AuthError::BadSignature)?)
            .map_err(|_| AuthError::BadSignature)?;
        let signature = Signature::from_slice(&signature).map_err(|_| AuthError::BadSignature)?;

        let now = unix_now();
        if (now as i64 - timestamp).abs() > CLOCK_SKEW.as_secs() as i64 {
            return Err(AuthError::BadTimestamp);
        }
        let payload = signing_payload(method.as_str(), path, timestamp, body);

        let mut devices = self
            .devices
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let device = devices.get(device_id).ok_or(AuthError::UnknownDevice)?;
        if now.saturating_sub(device.last_seen) > self.ttl.as_secs() {
            devices.remove(device_id);
            self.persist(&devices);
            return Err(AuthError::Expired);
        }
        device
            .key
            .verify(payload.as_bytes(), &signature)
            .map_err(|_| AuthError::BadSignature)?;
        if let Some(device) = devices.get_mut(device_id) {
            device.last_seen = now;
        }
        // Heartbeats arrive every few seconds; persisting each one would
        // rewrite the file constantly.
        let mut last_persist = self
            .last_persist
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if last_persist.elapsed() >= Duration::from_secs(60) {
            *last_persist = Instant::now();
            self.persist(&devices);
        }
        Ok(())
    }
}

/// Signs a request from the device side; used by tests.
#[cfg(test)]
pub fn sign_request(
    key: &SigningKey,
    method: &str,
    path: &str,
    timestamp: i64,
    body: &[u8],
) -> String {
    let payload = signing_payload(method, path, timestamp, body);
    STANDARD.encode(key.sign(payload.as_bytes()).to_bytes())
}

fn header_str<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers.get(name).and_then(|value| value.to_str().ok())
}

fn configured(username: &str, password: &str) -> Option<(String, String)> {
    (!username.is_empty() && !password.is_empty())
        .then(|| (username.to_string(), password.to_string()))
}

fn load_devices(path: &Path) -> HashMap<String, Device> {
    let Ok(bytes) = fs::read(path) else {
        return HashMap::new();
    };
    let Ok(file) = serde_json::from_slice::<DeviceFile>(&bytes) else {
        log::warn!(target: "lan_api", event = "lan_devices_corrupt"; "lan_devices_corrupt");
        return HashMap::new();
    };
    file.devices
        .into_iter()
        .filter_map(|(id, record)| {
            let key = STANDARD.decode(record.public_key).ok()?;
            let key = VerifyingKey::from_bytes(&key.try_into().ok()?).ok()?;
            Some((
                id,
                Device {
                    name: record.name,
                    key,
                    last_seen: record.last_seen,
                },
            ))
        })
        .collect()
}

fn write_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let temporary = path.with_extension("json.tmp");
    fs::write(&temporary, bytes)?;
    fs::rename(temporary, path)
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

pub(crate) fn fingerprint_of(public_key: &[u8]) -> String {
    let digest = Sha256::digest(public_key);
    digest[..16]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<_>>()
        .join(":")
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand_core::OsRng;

    fn service(ttl: Duration) -> AuthService {
        AuthService::new("admin", "secret", ttl)
    }

    fn basic_headers(user: &str, pass: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::AUTHORIZATION,
            format!("Basic {}", STANDARD.encode(format!("{user}:{pass}")))
                .parse()
                .unwrap(),
        );
        headers
    }

    fn signed_headers(
        device: &str,
        key: &SigningKey,
        method: &str,
        path: &str,
        timestamp: i64,
    ) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(DEVICE_HEADER, device.parse().unwrap());
        headers.insert(TIMESTAMP_HEADER, timestamp.to_string().parse().unwrap());
        headers.insert(
            SIGNATURE_HEADER,
            sign_request(key, method, path, timestamp, b"")
                .parse()
                .unwrap(),
        );
        headers
    }

    fn pair(auth: &AuthService, device: &str, key: &SigningKey) {
        auth.pair(device, key.verifying_key().as_bytes()).unwrap();
    }

    #[test]
    fn basic_auth_accepts_the_configured_password() {
        let auth = service(HEARTBEAT_TTL);

        assert_eq!(
            auth.authorize(
                &Method::GET,
                "/api/v1/status",
                &basic_headers("admin", "secret"),
                b""
            ),
            Ok(())
        );
    }

    #[test]
    fn basic_auth_rejects_a_wrong_password() {
        let auth = service(HEARTBEAT_TTL);

        assert_eq!(
            auth.authorize(
                &Method::GET,
                "/api/v1/status",
                &basic_headers("admin", "wrong"),
                b""
            ),
            Err(AuthError::InvalidCredentials)
        );
    }

    #[test]
    fn unconfigured_service_rejects_everything() {
        let auth = AuthService::new("", "", HEARTBEAT_TTL);

        assert_eq!(
            auth.authorize(&Method::GET, "/api/v1/status", &HeaderMap::new(), b""),
            Err(AuthError::NotConfigured)
        );
    }

    #[test]
    fn paired_device_signature_is_accepted() {
        let auth = service(HEARTBEAT_TTL);
        let key = SigningKey::generate(&mut OsRng);
        pair(&auth, "nas-1", &key);
        let now = unix_now() as i64;

        assert_eq!(
            auth.authorize(
                &Method::GET,
                "/api/v1/status",
                &signed_headers("nas-1", &key, "GET", "/api/v1/status", now),
                b""
            ),
            Ok(())
        );
    }

    #[test]
    fn tampered_signature_is_rejected() {
        let auth = service(HEARTBEAT_TTL);
        let key = SigningKey::generate(&mut OsRng);
        pair(&auth, "nas-1", &key);
        let now = unix_now() as i64;
        let mut headers = signed_headers("nas-1", &key, "GET", "/api/v1/status", now);

        assert_eq!(
            auth.authorize(&Method::POST, "/api/v1/status", &headers, b""),
            Err(AuthError::BadSignature)
        );
        headers.insert(SIGNATURE_HEADER, "AAAA".parse().unwrap());
        assert_eq!(
            auth.authorize(&Method::GET, "/api/v1/status", &headers, b""),
            Err(AuthError::BadSignature)
        );
    }

    #[test]
    fn replay_outside_the_window_is_rejected() {
        let auth = service(HEARTBEAT_TTL);
        let key = SigningKey::generate(&mut OsRng);
        pair(&auth, "nas-1", &key);
        let stale = unix_now() as i64 - CLOCK_SKEW.as_secs() as i64 - 5;

        assert_eq!(
            auth.authorize(
                &Method::GET,
                "/api/v1/status",
                &signed_headers("nas-1", &key, "GET", "/api/v1/status", stale),
                b""
            ),
            Err(AuthError::BadTimestamp)
        );
    }

    #[test]
    fn unknown_device_is_rejected() {
        let auth = service(HEARTBEAT_TTL);
        let key = SigningKey::generate(&mut OsRng);
        let now = unix_now() as i64;

        assert_eq!(
            auth.authorize(
                &Method::GET,
                "/api/v1/status",
                &signed_headers("ghost", &key, "GET", "/api/v1/status", now),
                b""
            ),
            Err(AuthError::UnknownDevice)
        );
    }

    #[test]
    fn device_is_revoked_without_a_heartbeat() {
        let auth = service(Duration::from_secs(1));
        let key = SigningKey::generate(&mut OsRng);
        pair(&auth, "nas-1", &key);
        let now = unix_now() as i64;
        assert!(auth
            .authorize(
                &Method::GET,
                "/api/v1/status",
                &signed_headers("nas-1", &key, "GET", "/api/v1/status", now),
                b""
            )
            .is_ok());

        // Strictly beyond the 1 s TTL measured in whole seconds.
        std::thread::sleep(Duration::from_millis(2200));

        let later = unix_now() as i64;
        assert_eq!(
            auth.authorize(
                &Method::GET,
                "/api/v1/status",
                &signed_headers("nas-1", &key, "GET", "/api/v1/status", later),
                b""
            ),
            Err(AuthError::Expired)
        );
    }

    #[test]
    fn pairing_rejects_invalid_device_ids_and_keys() {
        let auth = service(HEARTBEAT_TTL);
        let key = SigningKey::generate(&mut OsRng);

        assert_eq!(
            auth.pair("", key.verifying_key().as_bytes()),
            Err(AuthError::BadDeviceId)
        );
        assert_eq!(
            auth.pair("bad id!", key.verifying_key().as_bytes()),
            Err(AuthError::BadDeviceId)
        );
        assert_eq!(auth.pair("nas-1", b"short"), Err(AuthError::BadPublicKey));
    }

    #[test]
    fn paired_devices_persist_across_restarts() {
        let directory =
            std::env::temp_dir().join(format!("rayload-devices-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join("identity.key");
        let key = SigningKey::generate(&mut OsRng);

        let first = AuthService::load_or_create(&path, "admin", "secret", HEARTBEAT_TTL).unwrap();
        first
            .pair_named("nas-1", "Living room NAS", key.verifying_key().as_bytes())
            .unwrap();
        drop(first);

        let second = AuthService::load_or_create(&path, "admin", "secret", HEARTBEAT_TTL).unwrap();
        let now = unix_now() as i64;
        assert_eq!(
            second.authorize(
                &Method::GET,
                "/api/v1/status",
                &signed_headers("nas-1", &key, "GET", "/api/v1/status", now),
                b""
            ),
            Ok(())
        );
    }

    #[test]
    fn the_server_identity_persists_across_restarts() {
        let directory = std::env::temp_dir().join(format!("rayload-lan-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join("identity.key");

        let first = AuthService::load_or_create(&path, "admin", "secret", HEARTBEAT_TTL).unwrap();
        let second = AuthService::load_or_create(&path, "admin", "secret", HEARTBEAT_TTL).unwrap();

        assert_eq!(first.fingerprint(), second.fingerprint());
    }

    #[test]
    fn a_pairing_ticket_can_only_be_redeemed_once() {
        let auth = service(HEARTBEAT_TTL);
        let key = SigningKey::generate(&mut OsRng);
        let ticket = auth.create_ticket();

        assert_eq!(
            auth.redeem_ticket(&ticket, "tablet", "Tablet", key.verifying_key().as_bytes()),
            Ok(())
        );
        assert_eq!(
            auth.redeem_ticket(&ticket, "tablet", "Tablet", key.verifying_key().as_bytes()),
            Err(AuthError::BadTicket)
        );
    }

    #[test]
    fn a_discarded_ticket_cannot_be_redeemed() {
        let auth = service(HEARTBEAT_TTL);
        let key = SigningKey::generate(&mut OsRng);
        let ticket = auth.create_ticket();

        auth.discard_ticket(&ticket);

        assert_eq!(
            auth.redeem_ticket(&ticket, "tablet", "Tablet", key.verifying_key().as_bytes()),
            Err(AuthError::BadTicket)
        );
    }

    #[test]
    fn an_incoming_pair_waits_for_the_local_decision() {
        let auth = service(HEARTBEAT_TTL);
        let key = SigningKey::generate(&mut OsRng);
        let (data, receiver) = auth
            .begin_pair(
                "tablet",
                "Tablet",
                key.verifying_key().as_bytes(),
                "aa:bb".into(),
                "aarch64",
                "192.168.1.5",
                "ticket",
            )
            .unwrap();

        let pending = auth.pending_requests();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].fingerprint, "aa:bb");
        auth.resolve_pair(&data.id, true).unwrap();
        assert!(receiver.blocking_recv().unwrap());
        assert!(auth.pending_requests().is_empty());
    }
}
