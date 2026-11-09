# Rayload for Android — v1.0.0

> [!IMPORTANT]
> Rayload is an independent, unofficial Android port based on Rayburst by
> **AnInsomniacy**. It follows the original project's MIT license.

Rayload brings the Aria2 Next download engine to Android with a mobile-first
interface, system sharing, public Downloads integration, background operation,
and local browser-extension connectivity.

## ✨ Highlights

- **Independent Android identity** — package name
  `com.shiwushuifeng.rayload`, version `1.0.0`, and a dedicated Rayload release
  signing certificate.
- **Mobile-first interface** — bottom navigation, safe-area handling, larger
  touch targets, corrected add-task tabs, and Android-specific preference
  filtering.
- **Guided first launch** — sequential storage, notification, and battery
  optimization setup.
- **Aria2 Next on Android** — bundled ARM64 engine with authenticated JSON-RPC
  on port `6800`.
- **System Share Target** — send HTTP/HTTPS links from Firefox and other apps
  directly into Rayload.
- **Native file actions** — DocumentsUI folder access, FileProvider sharing and
  opening, and native directory selection for new tasks.
- **Background operation** — foreground data-sync service, completion
  notifications, partial wake lock, and optional exit after all downloads
  complete.
- **Safer deletion** — removed download files and metadata move into the hidden
  `.trash` directory instead of being permanently deleted.

## 📦 Packages

| Package | File | Size |
| ------- | ---- | ---- |
| Release | `Rayload-1.0.0-android17-arm64-release.apk` | 22,068,182 bytes (21.05 MiB) |
| Debug | `Rayload-1.0.0-android17-arm64-debug.apk` | 119,715,478 bytes (114.17 MiB) |

| Property | Value |
| -------- | ----- |
| Architecture | `arm64-v8a` |
| Minimum Android | Android 7.0 / API 24 |
| Target Android | Android 17 / API 37 |
| Release version code | `10000` |
| Release signer | `CN=Rayload, O=shiwushuifeng, C=CN` |

Verify the packages:

```bash
sha256sum Rayload-1.0.0-android17-arm64-release.apk
sha256sum Rayload-1.0.0-android17-arm64-debug.apk
```

Expected output:

```text
94282a76aaccf7c514bdfa7a23fca61bd0645bb4c91c0466123351f3df4f6197  Rayload-1.0.0-android17-arm64-release.apk
6bd1ad8aeed2359d58bf37e6762f0aa5e3dae497d4c327cd625dc6ba7cab3940  Rayload-1.0.0-android17-arm64-debug.apk
```

The Release package is signed with the Rayload release identity. The Debug
package uses the Android debug certificate. Uninstall the other package before
switching between them.

## 🚀 First launch

1. Install `Rayload-1.0.0-android17-arm64-release.apk`.
2. Follow the welcome flow to grant **All files access**.
3. Allow notifications for the foreground download service.
4. Allow Rayload to be excluded from battery optimization.

Downloads are stored in `/storage/emulated/0/Download/Rayload`.

## 🌐 Browser extension configuration

Use the local Aria2 JSON-RPC endpoint:

```text
Address: http://127.0.0.1:6800/jsonrpc
Secret:  Copy it from Settings → Advanced → RPC secret
```

The RPC port is fixed at `6800` on Android. Standard Chrome for Android does not
load desktop extensions; use Firefox Android or a Chromium build with extension
sideloading support.

## ✅ Verification

- Strict TypeScript type checking, ESLint, Prettier, locale parity, and
  repository integrity checks passed.
- 1,174 frontend tests passed.
- 339 Rust tests passed; 5 integration tests remain intentionally ignored.
- 8 Native Messaging tests passed.
- Both APKs were built successfully and verified for package name, version,
  target SDK, ARM64 ABI, and signatures.
- Release R8/resource shrinking and Rust `strip` are enabled.

Manual device acceptance is still recommended for the native directory picker,
file sharing, DocumentsUI navigation, `.trash` deletion, wake lock, and
exit-after-download behavior.

## 📄 License and attribution

Rayload is an unofficial port based on
[Rayburst](https://github.com/AnInsomniacy/rayburst). Original author:
**AnInsomniacy**. The port follows the original project's MIT license.
