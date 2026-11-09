use crate::error::AppError;
use crate::services::external_input::{
    route_external_inputs, ExternalDownloadInput, ExternalRequestHeader,
};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebviewDownloadRequest {
    pub url: String,
    pub referer: Option<String>,
    pub user_agent: Option<String>,
    pub content_disposition: Option<String>,
    pub content_length: Option<i64>,
}

#[tauri::command]
pub fn webview_download_requested(
    app: tauri::AppHandle,
    request: WebviewDownloadRequest,
) -> Result<(), AppError> {
    let input = webview_external_input(request)?;
    route_external_inputs(&app, vec![input], "webview", false);
    Ok(())
}

fn webview_external_input(
    request: WebviewDownloadRequest,
) -> Result<ExternalDownloadInput, AppError> {
    let url = Url::parse(&request.url)
        .map_err(|error| AppError::InvalidInput(format!("Invalid download URL: {error}")))?;
    if !matches!(url.scheme(), "http" | "https") {
        return Err(AppError::InvalidInput(
            "WebView downloads must use HTTP or HTTPS".into(),
        ));
    }

    let referer = request.referer.filter(|value| {
        Url::parse(value).is_ok_and(|url| matches!(url.scheme(), "http" | "https"))
    });
    let filename = request
        .content_disposition
        .as_deref()
        .and_then(content_disposition_filename);
    let size = request
        .content_length
        .and_then(|value| u64::try_from(value).ok())
        .filter(|value| *value > 0);

    Ok(ExternalDownloadInput {
        request_id: None,
        filename_source: None,
        url: url.to_string(),
        final_url: Some(url.to_string()),
        referer,
        cookie: None,
        filename,
        user_agent: request.user_agent.filter(|value| !value.trim().is_empty()),
        request_headers: Vec::<ExternalRequestHeader>::new(),
        source: Some("webview".into()),
        size,
    })
}

fn content_disposition_filename(value: &str) -> Option<String> {
    let encoded = value.split(';').find_map(|part| {
        let (name, value) = part.trim().split_once('=')?;
        name.trim()
            .eq_ignore_ascii_case("filename*")
            .then_some(value.trim())
    });
    if let Some(value) = encoded {
        let value = value.split_once("''").map_or(value, |(_, value)| value);
        if let Ok(decoded) = urlencoding::decode(value) {
            return safe_filename(&decoded);
        }
    }

    value.split(';').find_map(|part| {
        let (name, value) = part.trim().split_once('=')?;
        if !name.trim().eq_ignore_ascii_case("filename") {
            return None;
        }
        safe_filename(value.trim().trim_matches('"'))
    })
}

fn safe_filename(value: &str) -> Option<String> {
    let name = value
        .rsplit(|character| character == '/' || character == '\\')
        .next()
        .map(str::trim)
        .filter(|name| !name.is_empty() && *name != "." && *name != "..")?;
    Some(name.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(url: &str) -> WebviewDownloadRequest {
        WebviewDownloadRequest {
            url: url.into(),
            referer: Some("https://example.com/downloads".into()),
            user_agent: Some("Rayburst WebView".into()),
            content_disposition: Some("attachment; filename*=UTF-8''Rayburst%20Setup.zip".into()),
            content_length: Some(4096),
        }
    }

    #[test]
    fn webview_download_becomes_an_internal_handoff() {
        let input = webview_external_input(request("https://cdn.example.com/setup")).unwrap();

        assert_eq!(input.url, "https://cdn.example.com/setup");
        assert_eq!(input.final_url.as_deref(), Some(input.url.as_str()));
        assert_eq!(input.filename.as_deref(), Some("Rayburst Setup.zip"));
        assert_eq!(input.size, Some(4096));
        assert_eq!(input.source.as_deref(), Some("webview"));
    }

    #[test]
    fn quoted_and_path_bearing_content_disposition_is_made_safe() {
        let mut request = request("https://example.com/file");
        request.content_disposition = Some("attachment; filename=\"../unsafe/report.pdf\"".into());

        assert_eq!(
            webview_external_input(request).unwrap().filename.as_deref(),
            Some("report.pdf")
        );
    }

    #[test]
    fn unsupported_navigation_schemes_are_not_downloaded() {
        let error = webview_external_input(request("moz-extension://extension/download"))
            .expect_err("extension resource must not reach Aria2");

        assert!(error.to_string().contains("HTTP or HTTPS"));
    }

    #[test]
    fn missing_or_negative_size_and_non_web_referer_are_discarded() {
        let mut request = request("https://example.com/file");
        request.content_length = Some(-1);
        request.referer = Some("tauri://localhost/".into());

        let input = webview_external_input(request).unwrap();
        assert_eq!(input.size, None);
        assert_eq!(input.referer, None);
    }
}
