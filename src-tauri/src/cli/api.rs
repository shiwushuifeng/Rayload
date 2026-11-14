//! Local HTTP client for the running LAN API.

use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::config::Settings;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub last_seen: u64,
    pub online: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PendingInfo {
    pub id: String,
    pub name: String,
    pub fingerprint: String,
    pub arch: String,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineStatus {
    pub download_speed: u64,
    pub active_tasks: u32,
}

pub struct LocalApi {
    base: String,
    username: String,
    password: String,
    client: reqwest::Client,
}

impl LocalApi {
    pub fn new(settings: &Settings) -> Result<Self, String> {
        if settings.lan_username.trim().is_empty() || settings.lan_password.is_empty() {
            return Err("LAN admin credentials are not configured".into());
        }
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .map_err(|error| error.to_string())?;
        Ok(Self {
            base: format!("http://127.0.0.1:{}", settings.lan_port),
            username: settings.lan_username.clone(),
            password: settings.lan_password.clone(),
            client,
        })
    }

    pub fn status(&self) -> Result<EngineStatus, String> {
        self.get("/api/v1/status")
    }

    pub fn devices(&self) -> Result<Vec<DeviceInfo>, String> {
        #[derive(Deserialize)]
        struct Response {
            devices: Vec<DeviceInfo>,
        }
        self.get::<Response>("/api/v1/devices")
            .map(|response| response.devices)
    }

    pub fn pending(&self) -> Result<Vec<PendingInfo>, String> {
        #[derive(Deserialize)]
        struct Response {
            pending: Vec<PendingInfo>,
        }
        self.get::<Response>("/api/v1/pair/pending")
            .map(|response| response.pending)
    }

    pub fn revoke(&self, device_id: &str) -> Result<(), String> {
        let path = format!("/api/v1/devices/{}/revoke", urlencoding::encode(device_id));
        self.post::<serde_json::Value>(&path, serde_json::json!({}))
            .map(|_| ())
    }

    pub fn decide(&self, id: &str, approved: bool) -> Result<(), String> {
        let path = format!("/api/v1/pair/pending/{}", urlencoding::encode(id));
        self.post::<serde_json::Value>(&path, serde_json::json!({ "approved": approved }))
            .map(|_| ())
    }

    fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, String> {
        runtime()?.block_on(async {
            let response = self
                .client
                .get(format!("{}{path}", self.base))
                .basic_auth(&self.username, Some(&self.password))
                .send()
                .await
                .map_err(|error| error.to_string())?;
            response_body(response).await
        })
    }

    fn post<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> Result<T, String> {
        runtime()?.block_on(async {
            let response = self
                .client
                .post(format!("{}{path}", self.base))
                .basic_auth(&self.username, Some(&self.password))
                .json(&body)
                .send()
                .await
                .map_err(|error| error.to_string())?;
            response_body(response).await
        })
    }
}

fn runtime() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| error.to_string())
}

async fn response_body<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T, String> {
    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    if !status.is_success() {
        return Err(format!("HTTP {status}: {body}"));
    }
    serde_json::from_str(&body).map_err(|error| format!("invalid API response: {error}"))
}
