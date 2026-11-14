//! Plugin permission model and request sandbox.
//!
//! Plugin authors declare capabilities in the manifest. The user grants a
//! subset during import, and every request is checked again at runtime. Headers
//! that would bypass the declared capabilities are removed unconditionally.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};
use url::Url;

/// Capabilities a plugin may request from the host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginPermission {
    Network,
    Cookies,
    ModifyHeaders,
    ReadClipboard,
}

impl PluginPermission {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Network => "network",
            Self::Cookies => "cookies",
            Self::ModifyHeaders => "modify_headers",
            Self::ReadClipboard => "read_clipboard",
        }
    }
}

/// The two supported plugin shapes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginKind {
    #[default]
    Rule,
    Script,
}

/// A parsed plugin manifest. Unknown fields are preserved by callers that edit
/// the original JSON document, so this view only covers security-relevant data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginManifest {
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "type", default)]
    pub kind: PluginKind,
    #[serde(default)]
    pub runtime: String,
    #[serde(default)]
    pub entry: Option<String>,
    #[serde(default)]
    pub permissions: Vec<PluginPermission>,
    #[serde(default)]
    pub allowed_domains: Vec<String>,
    #[serde(default)]
    pub granted_permissions: Vec<PluginPermission>,
}

impl PluginManifest {
    pub fn parse(content: &str) -> Result<Self, PolicyError> {
        let manifest: Self =
            serde_json::from_str(content).map_err(|_| PolicyError::InvalidManifest)?;
        if manifest.name.trim().is_empty() {
            return Err(PolicyError::InvalidManifest);
        }
        manifest.policy()?;
        Ok(manifest)
    }

    pub fn policy(&self) -> Result<PluginPolicy, PolicyError> {
        PluginPolicy::new(
            self.permissions.clone(),
            self.granted_permissions.clone(),
            self.allowed_domains.clone(),
        )
    }
}

/// Errors raised while validating or enforcing plugin capabilities.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PolicyError {
    #[error("plugin manifest is invalid")]
    InvalidManifest,
    #[error("network permission requires at least one allowed domain")]
    NetworkRequiresDomains,
    #[error("cookies permission also requires network permission")]
    CookiesRequiresNetwork,
    #[error("allowed domain `{0}` is invalid")]
    InvalidDomain(String),
    #[error("network permission was not granted to this plugin")]
    NetworkNotGranted,
    #[error("plugin may not access `{0}`")]
    DomainDenied(String),
    #[error("invalid request URL: {0}")]
    InvalidUrl(String),
    #[error("plugin script entry `{0}` is invalid")]
    InvalidEntry(String),
}

/// Runtime policy built from declared and user-granted permissions.
#[derive(Debug, Clone)]
pub struct PluginPolicy {
    declared: BTreeSet<PluginPermission>,
    granted: BTreeSet<PluginPermission>,
    domains: Vec<String>,
}

impl PluginPolicy {
    pub fn new(
        declared: Vec<PluginPermission>,
        granted: Vec<PluginPermission>,
        allowed_domains: Vec<String>,
    ) -> Result<Self, PolicyError> {
        let declared: BTreeSet<_> = declared.into_iter().collect();
        let mut domains = Vec::new();
        for raw in allowed_domains {
            let domain = normalize_domain(&raw)?;
            if !domains.contains(&domain) {
                domains.push(domain);
            }
        }
        if declared.contains(&PluginPermission::Network) && domains.is_empty() {
            return Err(PolicyError::NetworkRequiresDomains);
        }
        if declared.contains(&PluginPermission::Cookies)
            && !declared.contains(&PluginPermission::Network)
        {
            return Err(PolicyError::CookiesRequiresNetwork);
        }
        // A manifest can never grant itself more than it declared. The user
        // grant is stored separately and is intersected here again at runtime.
        let granted: BTreeSet<_> = granted
            .into_iter()
            .filter(|permission| declared.contains(permission))
            .collect();
        Ok(Self {
            declared,
            granted,
            domains,
        })
    }

    pub fn declared(&self) -> &BTreeSet<PluginPermission> {
        &self.declared
    }

    pub fn granted(&self) -> &BTreeSet<PluginPermission> {
        &self.granted
    }

    pub fn is_granted(&self, permission: PluginPermission) -> bool {
        self.granted.contains(&permission)
    }

    pub fn can_read_clipboard(&self) -> bool {
        self.is_granted(PluginPermission::ReadClipboard)
    }

    pub fn allowed_domains(&self) -> &[String] {
        &self.domains
    }

    pub fn authorize_url(&self, raw_url: &str) -> Result<Url, PolicyError> {
        if !self.is_granted(PluginPermission::Network) {
            return Err(PolicyError::NetworkNotGranted);
        }
        let url =
            Url::parse(raw_url).map_err(|error| PolicyError::InvalidUrl(error.to_string()))?;
        if !matches!(url.scheme(), "http" | "https") {
            return Err(PolicyError::InvalidUrl(
                "only http and https are supported".into(),
            ));
        }
        let host = url
            .host_str()
            .ok_or_else(|| PolicyError::InvalidUrl("request URL has no host".into()))?
            .to_ascii_lowercase();
        if !self
            .domains
            .iter()
            .any(|pattern| domain_matches(&host, pattern))
        {
            return Err(PolicyError::DomainDenied(host));
        }
        Ok(url)
    }

    /// Applies header stripping, user-agent/referer gating and cookie injection.
    pub fn prepare_request(
        &self,
        raw_url: &str,
        headers: Vec<(String, String)>,
        cookies: &PluginCookieStore,
    ) -> Result<PreparedRequest, PolicyError> {
        let url = self.authorize_url(raw_url)?;
        let host = url.host_str().unwrap_or_default().to_ascii_lowercase();
        let allow_headers = self.is_granted(PluginPermission::ModifyHeaders);
        let mut sanitized = Vec::with_capacity(headers.len() + 1);
        for (name, value) in headers {
            let lowercase = name.trim().to_ascii_lowercase();
            if lowercase.is_empty() || value.contains(['\r', '\n']) {
                continue;
            }
            // A plugin can never supply cookies itself, and transport-level
            // headers are owned by the host.
            if matches!(
                lowercase.as_str(),
                "cookie"
                    | "host"
                    | "content-length"
                    | "connection"
                    | "transfer-encoding"
                    | "proxy-authorization"
            ) {
                continue;
            }
            if matches!(lowercase.as_str(), "user-agent" | "referer") && !allow_headers {
                continue;
            }
            sanitized.push((name, value));
        }
        if self.is_granted(PluginPermission::Cookies) {
            if let Some(cookie) = cookies.cookies_for(&host) {
                sanitized.push(("Cookie".into(), cookie.to_string()));
            }
        }
        Ok(PreparedRequest {
            url,
            headers: sanitized,
        })
    }

    /// Environment passed to a script plugin. Secrets are limited to the
    /// domains the plugin declared and the user approved.
    pub fn script_environment(&self, cookies: &PluginCookieStore) -> Vec<(String, String)> {
        let permissions = self
            .granted
            .iter()
            .map(|permission| permission.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let mut environment = vec![
            ("RAYLOAD_PLUGIN_PERMISSIONS".into(), permissions),
            (
                "RAYLOAD_PLUGIN_ALLOWED_DOMAINS".into(),
                self.domains.join(","),
            ),
        ];
        if self.is_granted(PluginPermission::Cookies) {
            let scoped = self
                .domains
                .iter()
                .filter_map(|domain| {
                    let host = domain.trim_start_matches("*.");
                    cookies
                        .cookies_for(host)
                        .map(|value| (host.to_string(), value.to_string()))
                })
                .collect::<BTreeMap<_, _>>();
            if let Ok(json) = serde_json::to_string(&scoped) {
                environment.push(("RAYLOAD_PLUGIN_COOKIES".into(), json));
            }
        }
        environment
    }
}

/// A sanitized request ready for the host HTTP client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedRequest {
    pub url: Url,
    pub headers: Vec<(String, String)>,
}

/// Cookies available to the plugin host. The file format is a flat host to
/// cookie-header map, for example `{"pan.example.com":"sid=..."}`.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PluginCookieStore {
    cookies: BTreeMap<String, String>,
}

impl PluginCookieStore {
    pub fn from_json(content: &str) -> Self {
        serde_json::from_str(content).unwrap_or_default()
    }

    pub fn load(path: &Path) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .map(|content| Self::from_json(&content))
            .unwrap_or_default()
    }

    pub fn insert(&mut self, host: &str, cookie: &str) {
        let host = host.trim().trim_start_matches('.').to_ascii_lowercase();
        if !host.is_empty() && !cookie.contains(['\r', '\n']) {
            self.cookies.insert(host, cookie.to_string());
        }
    }

    pub fn cookies_for(&self, host: &str) -> Option<&str> {
        let host = host.trim_start_matches('.').to_ascii_lowercase();
        if let Some(cookie) = self.cookies.get(&host) {
            return Some(cookie);
        }
        let mut labels = host.split('.').collect::<Vec<_>>();
        while labels.len() > 1 {
            labels.remove(0);
            let parent = labels.join(".");
            if let Some(cookie) = self.cookies.get(&parent) {
                return Some(cookie);
            }
        }
        None
    }
}

fn normalize_domain(raw: &str) -> Result<String, PolicyError> {
    let trimmed = raw.trim().trim_end_matches('.').to_ascii_lowercase();
    if trimmed.is_empty() || trimmed.contains(['/', '?', '#', '@', ':']) {
        return Err(PolicyError::InvalidDomain(raw.to_string()));
    }
    let (wildcard, host) = match trimmed.strip_prefix("*.") {
        Some(host) => (true, host),
        None => (false, trimmed.as_str()),
    };
    if host.is_empty() || host.starts_with('.') || host.contains('*') {
        return Err(PolicyError::InvalidDomain(raw.to_string()));
    }
    if host.parse::<std::net::IpAddr>().is_ok() {
        return Err(PolicyError::InvalidDomain(raw.to_string()));
    }
    let parsed = Url::parse(&format!("https://{host}"))
        .map_err(|_| PolicyError::InvalidDomain(raw.to_string()))?;
    let normalized = parsed
        .host_str()
        .ok_or_else(|| PolicyError::InvalidDomain(raw.to_string()))?;
    Ok(if wildcard {
        format!("*.{normalized}")
    } else {
        normalized.to_string()
    })
}

fn domain_matches(host: &str, pattern: &str) -> bool {
    if let Some(suffix) = pattern.strip_prefix("*.") {
        host.ends_with(&format!(".{suffix}")) && host != suffix
    } else {
        host == pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy(declared: Vec<PluginPermission>, granted: Vec<PluginPermission>) -> PluginPolicy {
        PluginPolicy::new(
            declared,
            granted,
            vec!["*.example.com".into(), "rayload.example".into()],
        )
        .unwrap()
    }

    fn cookies() -> PluginCookieStore {
        let mut store = PluginCookieStore::default();
        store.insert("pan.example.com", "sid=secret");
        store.insert("example.com", "root=1");
        store
    }

    #[test]
    fn network_requires_an_allowed_domain() {
        let error = PluginPolicy::new(vec![PluginPermission::Network], vec![], vec![]).unwrap_err();
        assert_eq!(error, PolicyError::NetworkRequiresDomains);
    }

    #[test]
    fn cookies_require_network_declaration() {
        let error = PluginPolicy::new(
            vec![PluginPermission::Cookies],
            vec![],
            vec!["example.com".into()],
        )
        .unwrap_err();
        assert_eq!(error, PolicyError::CookiesRequiresNetwork);
    }

    #[test]
    fn user_grant_cannot_exceed_declared_permissions() {
        let policy = policy(
            vec![PluginPermission::Network],
            vec![PluginPermission::Network, PluginPermission::Cookies],
        );
        assert!(policy.is_granted(PluginPermission::Network));
        assert!(!policy.is_granted(PluginPermission::Cookies));
    }

    #[test]
    fn wildcard_domains_match_subdomains_but_not_the_root() {
        let policy = policy(
            vec![PluginPermission::Network],
            vec![PluginPermission::Network],
        );
        assert!(policy.authorize_url("https://pan.example.com/file").is_ok());
        assert!(policy.authorize_url("https://example.com/file").is_err());
        assert!(policy
            .authorize_url("https://evil-example.com/file")
            .is_err());
    }

    #[test]
    fn cookie_headers_are_stripped_without_the_cookie_permission() {
        let policy = policy(
            vec![PluginPermission::Network],
            vec![PluginPermission::Network],
        );
        let request = policy
            .prepare_request(
                "https://pan.example.com/file",
                vec![("Cookie".into(), "sid=attacker".into())],
                &cookies(),
            )
            .unwrap();
        assert!(!request
            .headers
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case("cookie")));
    }

    #[test]
    fn granted_cookie_permission_injects_only_scoped_cookies() {
        let policy = policy(
            vec![PluginPermission::Network, PluginPermission::Cookies],
            vec![PluginPermission::Network, PluginPermission::Cookies],
        );
        let request = policy
            .prepare_request(
                "https://pan.example.com/file",
                vec![("Cookie".into(), "sid=attacker".into())],
                &cookies(),
            )
            .unwrap();
        assert_eq!(
            request.headers,
            vec![("Cookie".into(), "sid=secret".into())]
        );
    }

    #[test]
    fn custom_headers_require_explicit_permission() {
        let denied = policy(
            vec![PluginPermission::Network],
            vec![PluginPermission::Network],
        );
        let request = denied
            .prepare_request(
                "https://pan.example.com/file",
                vec![
                    ("User-Agent".into(), "custom".into()),
                    ("Referer".into(), "https://pan.example.com".into()),
                ],
                &cookies(),
            )
            .unwrap();
        assert!(request.headers.is_empty());

        let allowed = policy(
            vec![PluginPermission::Network, PluginPermission::ModifyHeaders],
            vec![PluginPermission::Network, PluginPermission::ModifyHeaders],
        );
        let request = allowed
            .prepare_request(
                "https://pan.example.com/file",
                vec![("User-Agent".into(), "custom".into())],
                &cookies(),
            )
            .unwrap();
        assert_eq!(
            request.headers,
            vec![("User-Agent".into(), "custom".into())]
        );
    }

    #[test]
    fn script_environment_only_contains_granted_capabilities() {
        let policy = policy(
            vec![
                PluginPermission::Network,
                PluginPermission::Cookies,
                PluginPermission::ReadClipboard,
            ],
            vec![PluginPermission::Network, PluginPermission::Cookies],
        );
        let environment = policy.script_environment(&cookies());
        let map: BTreeMap<_, _> = environment.into_iter().collect();
        assert_eq!(
            map.get("RAYLOAD_PLUGIN_PERMISSIONS").map(String::as_str),
            Some("network,cookies")
        );
        assert!(map
            .get("RAYLOAD_PLUGIN_COOKIES")
            .is_some_and(|value| value.contains("root=1")));
        assert!(!map
            .get("RAYLOAD_PLUGIN_COOKIES")
            .is_some_and(|value| value.contains("sid=secret")));
        assert!(!map.contains_key("RAYLOAD_PLUGIN_CLIPBOARD"));
    }

    #[test]
    fn cookie_store_matches_parent_domains() {
        let store = cookies();
        assert_eq!(store.cookies_for("pan.example.com"), Some("sid=secret"));
        assert_eq!(store.cookies_for("example.com"), Some("root=1"));
        assert_eq!(store.cookies_for("other.test"), None);
    }
}
