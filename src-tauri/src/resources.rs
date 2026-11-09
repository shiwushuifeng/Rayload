//! Resolves bundled application resources to paths Rust can open.

use crate::error::AppError;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[cfg(any(target_os = "android", test))]
use std::path::{Component, Path};

#[cfg(target_os = "android")]
const RESOURCE_CACHE_DIR: &str = "bundled-resources";

/// Resolves a bundled resource to a physical path.
///
/// Desktop platforms already bundle resources into the application directory.
/// Android exposes them through the virtual `asset://` URL, so they are
/// materialized into a versioned application cache directory first.
pub(crate) fn resolve_resource(app: &AppHandle, resource_path: &str) -> Result<PathBuf, AppError> {
    #[cfg(target_os = "android")]
    {
        let version = app.package_info().version.to_string();
        let cache_root = app
            .path()
            .app_cache_dir()
            .map_err(|error| AppError::Io(format!("Failed to get app cache dir: {error}")))?
            .join(RESOURCE_CACHE_DIR)
            .join(version);
        let target = resource_cache_path(&cache_root, resource_path)?;
        if target.is_file() {
            return Ok(target);
        }

        let bytes = read_android_resource(app, resource_path)?;
        write_resource(&target, &bytes)?;
        log::info!(
            "resources: materialized {} at {}",
            resource_path,
            target.display()
        );
        Ok(target)
    }

    #[cfg(not(target_os = "android"))]
    {
        app.path()
            .resolve(resource_path, tauri::path::BaseDirectory::Resource)
            .map_err(|error| {
                AppError::Io(format!(
                    "Failed to resolve bundled resource {resource_path}: {error}"
                ))
            })
    }
}

#[cfg(any(target_os = "android", test))]
fn resource_cache_path(cache_root: &Path, resource_path: &str) -> Result<PathBuf, AppError> {
    let relative = Path::new(resource_path);
    if relative.is_absolute() {
        return Err(AppError::InvalidInput(
            "Bundled resource path must be relative".into(),
        ));
    }

    let mut safe = PathBuf::new();
    for component in relative.components() {
        match component {
            Component::Normal(value) => safe.push(value),
            _ => {
                return Err(AppError::InvalidInput(format!(
                    "Unsafe bundled resource path: {resource_path}"
                )));
            }
        }
    }
    if safe.as_os_str().is_empty() {
        return Err(AppError::InvalidInput(
            "Bundled resource path is empty".into(),
        ));
    }
    Ok(cache_root.join(safe))
}

#[cfg(any(target_os = "android", test))]
fn write_resource(target: &Path, bytes: &[u8]) -> Result<(), AppError> {
    if bytes.is_empty() {
        return Err(AppError::Io(format!(
            "Bundled resource is empty: {}",
            target.display()
        )));
    }
    let directory = target
        .parent()
        .ok_or_else(|| AppError::Io("Invalid bundled resource cache path".into()))?;
    std::fs::create_dir_all(directory)?;
    let temporary = target.with_extension("rayburst.tmp");
    std::fs::write(&temporary, bytes)?;
    std::fs::rename(&temporary, target)?;
    Ok(())
}

#[cfg(target_os = "android")]
fn read_android_resource(app: &AppHandle, resource_path: &str) -> Result<Vec<u8>, AppError> {
    let resolver = app.asset_resolver();
    // Tauri falls back to index.html when an asset key is absent. Verify the
    // exact key before trusting the returned bytes.
    let present = resolver
        .iter()
        .any(|(candidate, _)| candidate.as_ref() == resource_path);
    if present {
        return resolver
            .get(resource_path.to_string())
            .map(|asset| asset.bytes().to_vec())
            .ok_or_else(|| {
                AppError::Io(format!(
                    "Failed to read bundled resource asset: {resource_path}"
                ))
            });
    }

    read_android_asset_manager(resource_path)
}

#[cfg(target_os = "android")]
fn read_android_asset_manager(resource_path: &str) -> Result<Vec<u8>, AppError> {
    use jni::objects::{JObject, JValue};

    let context = ndk_context::android_context();
    // SAFETY: Tauri initializes ndk-context with the process-wide JavaVM and
    // application context before resources are requested.
    let vm = unsafe { jni::JavaVM::from_raw(context.vm().cast()) }
        .map_err(|error| AppError::Io(format!("Failed to access Android JavaVM: {error}")))?;
    let mut env = vm
        .attach_current_thread()
        .map_err(|error| AppError::Io(format!("Failed to attach Android JNI thread: {error}")))?;
    // SAFETY: The context local reference belongs to the Tauri Android runtime.
    let application = unsafe { JObject::from_raw(context.context() as jni::sys::jobject) };
    let asset_manager = env
        .call_method(
            &application,
            "getAssets",
            "()Landroid/content/res/AssetManager;",
            &[],
        )
        .and_then(|value| value.l())
        .map_err(|error| AppError::Io(format!("Failed to access Android assets: {error}")))?;
    let path = env
        .new_string(resource_path)
        .map_err(|error| AppError::Io(format!("Failed to encode Android asset path: {error}")))?;
    let stream = env
        .call_method(
            &asset_manager,
            "open",
            "(Ljava/lang/String;)Ljava/io/InputStream;",
            &[JValue::Object(&path)],
        )
        .and_then(|value| value.l())
        .map_err(|error| {
            AppError::Io(format!(
                "Failed to open bundled Android resource {resource_path}: {error}"
            ))
        })?;

    const CHUNK_SIZE: i32 = 16 * 1024;
    let buffer = env.new_byte_array(CHUNK_SIZE).map_err(|error| {
        AppError::Io(format!("Failed to allocate Android asset buffer: {error}"))
    })?;
    let mut bytes = Vec::new();
    loop {
        let read = env
            .call_method(
                &stream,
                "read",
                "([BII)I",
                &[
                    JValue::Object(buffer.as_ref()),
                    JValue::Int(0),
                    JValue::Int(CHUNK_SIZE),
                ],
            )
            .and_then(|value| value.i())
            .map_err(|error| {
                AppError::Io(format!(
                    "Failed to read bundled Android resource {resource_path}: {error}"
                ))
            })?;
        if read < 0 {
            break;
        }
        if read == 0 {
            continue;
        }
        let mut chunk = vec![0_i8; read as usize];
        env.get_byte_array_region(&buffer, 0, &mut chunk)
            .map_err(|error| {
                AppError::Io(format!(
                    "Failed to copy bundled Android resource {resource_path}: {error}"
                ))
            })?;
        bytes.extend(chunk.into_iter().map(|byte| byte as u8));
    }
    let _ = env.call_method(&stream, "close", "()V", &[]);
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn android_cache_target_is_a_real_sandbox_path() {
        let root = Path::new("/data/user/0/dev.aninsomniacy.rayburst/cache");
        let target = resource_cache_path(root, "data/bt-peer-blocklist.txt").unwrap();

        assert_eq!(target, root.join("data").join("bt-peer-blocklist.txt"));
        assert!(target.is_absolute());
        assert!(!target.to_string_lossy().starts_with("asset://"));
    }

    #[test]
    fn android_cache_rejects_paths_outside_the_cache_root() {
        let root = Path::new("/data/user/0/dev.aninsomniacy.rayburst/cache");

        assert!(resource_cache_path(root, "../outside.txt").is_err());
        assert!(resource_cache_path(root, "/absolute.txt").is_err());
    }

    #[test]
    fn materialized_resource_has_exact_bytes() {
        let directory = tempfile::tempdir().unwrap();
        let target = resource_cache_path(directory.path(), "data/server.met").unwrap();

        write_resource(&target, b"server-data").unwrap();

        assert_eq!(std::fs::read(target).unwrap(), b"server-data");
    }
}
