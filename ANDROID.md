# Rayload for Android

Rayload 1.0.0 is an independently branded Android port of Rayburst. It targets
API level 37 (Android 17) and supports API level 24 (Android 7) and later.

## Install

1. Install `artifacts/Rayload-1.0.0-android17-arm64-release.apk`.
2. On first launch, allow **All files access** so Rayload can write to the
   public `Download/Rayload` directory.
3. Allow notifications and battery optimization exemption when prompted.

The package contains the ARM64 Android build of aria2-next and does not require
a separate engine installation.

The debug package uses the Android debug certificate. Uninstall the other
package before switching between debug and release builds because their signing
identities differ.

## Browser extension

Use any Android browser extension that supports Aria2 JSON-RPC. Configure it
with `http://127.0.0.1:6800/jsonrpc` and copy the RPC secret from Rayload's
Advanced settings.

Standard Chrome for Android does not load desktop extensions. Use Firefox
Android or a Chromium fork that supports extension sideloading; the exact
install UI depends on the browser build.

## Build

```bash
pnpm install
pnpm android:prepare
pnpm tauri android build --apk --target aarch64
pnpm tauri android build --apk --target aarch64 --debug
```

`scripts/configure-android.mjs` re-applies the API 37 manifest, deep links,
foreground service, release signing, and aria2 native-library packaging after
the generated project changes. Do not run `tauri android init` over the
committed Android project because it replaces the Rayload-specific native
integration. The ignored local release key is stored under `signing/`; retain
it to install future builds over this APK.

## Platform differences

- In-app desktop updates are not supported on Android; install newer APKs.
- The floating window, tray, desktop drag-out, and autostart-at-login features
  are desktop-only.
- Android uses a single WebView and opens the add-task form inside the main page.
- Magnet, ED2K, Thunder, and Rayload links are registered through Android
  intent filters.
- Deleted downloads are moved to the hidden `.trash` directory inside the
  configured download folder.
