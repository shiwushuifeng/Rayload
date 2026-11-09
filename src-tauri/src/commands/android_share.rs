use crate::error::AppError;
#[cfg(any(target_os = "android", test))]
use std::collections::HashSet;
#[cfg(target_os = "android")]
use tauri::Manager;

#[cfg(target_os = "android")]
const SHARE_QUEUE_FILE: &str = "rayload-share-pending.txt";

#[tauri::command]
pub fn take_android_shared_links(app: tauri::AppHandle) -> Result<Vec<String>, AppError> {
    #[cfg(not(target_os = "android"))]
    {
        let _ = app;
        Ok(Vec::new())
    }

    #[cfg(target_os = "android")]
    {
        let path = app
            .path()
            .app_data_dir()
            .map_err(|error| AppError::Io(format!("Failed to get app data dir: {error}")))?
            .join("files")
            .join(SHARE_QUEUE_FILE);
        if !path.is_file() {
            return Ok(Vec::new());
        }

        let text = std::fs::read_to_string(&path)?;
        std::fs::remove_file(&path)?;
        let links = extract_http_links(&text);
        log::info!("android_share: dequeued links={}", links.len());
        Ok(links)
    }
}

#[cfg(any(target_os = "android", test))]
fn extract_http_links(text: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    text.split_whitespace()
        .filter_map(|candidate| {
            let candidate = candidate.trim_matches(|character: char| {
                matches!(
                    character,
                    '"' | '\''
                        | '('
                        | ')'
                        | '['
                        | ']'
                        | '{'
                        | '}'
                        | '<'
                        | '>'
                        | ','
                        | '.'
                        | ';'
                        | ':'
                        | '!'
                        | '?'
                        | '。'
                        | '，'
                        | '；'
                        | '！'
                        | '？'
                )
            });
            if !candidate.starts_with("http://") && !candidate.starts_with("https://") {
                return None;
            }
            let url = url::Url::parse(candidate).ok()?;
            if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
                return None;
            }
            let value = url.to_string();
            seen.insert(value.clone()).then_some(value)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_unique_web_links_from_shared_text() {
        let links = extract_http_links(
            "Look at https://example.com/file.zip, also http://example.org/a.\n\
             https://example.com/file.zip",
        );

        assert_eq!(
            links,
            vec!["https://example.com/file.zip", "http://example.org/a"]
        );
    }

    #[test]
    fn ignores_non_http_share_text() {
        assert!(extract_http_links("magnet:?xt=urn:btih:123 file:///tmp/a").is_empty());
    }
}
