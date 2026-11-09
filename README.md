# Rayload - Android Downloader

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[**English**](README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

Rayload is an independent, unofficial Android port of Rayburst and focuses on
Android only. It is built with Tauri, Rust, Vue 3, and Aria2 Next, with a
mobile-first interface, system sharing, background downloads, notifications, a
public Downloads integration, and a local Aria2 JSON-RPC service.

## Platform requirements

| Item           | Requirement                 |
| -------------- | --------------------------- |
| Minimum system | Android 7.0 / API 24        |
| Target system  | Android 17 / API 37         |
| Production ABI | `arm64-v8a`                 |
| Application ID | `com.shiwushuifeng.rayload` |

Rayload does not target desktop platforms. Standard Google Chrome for Android
cannot load desktop extensions. Use Firefox for Android or a Chromium build
with extension sideloading support when browser integration is required.

## Installation

Download the latest ARM64 APK from
[GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases):

```text
Rayload-v1.0.1-android17-arm64-release.apk
```

On first launch, choose your language and complete the Android setup steps:

1. Grant all-files access so Rayload can write to the public Downloads folder.
2. Allow notifications so the foreground download service can report status.
3. Optionally allow battery optimization exemption for reliable background
   transfers.

Default download directory:

```text
/storage/emulated/0/Download/Rayload
```

Release APKs use the independent Rayload signing identity. Debug APKs use the
Android debug certificate. Uninstall the other variant before switching between
them.

## Building for Android

### Dependencies

| Tool        | Version or target                   |
| ----------- | ----------------------------------- |
| Node.js     | 22 or newer                         |
| pnpm        | `11.5.2`, matching `packageManager` |
| Rust        | stable with `aarch64-linux-android` |
| Java        | JDK 21                              |
| Android SDK | Platform 37 and Build Tools 37.0.0  |
| Android NDK | `29.0.14206865`                     |
| Git         | current stable                      |
| GitHub CLI  | optional, only for releases         |

Install the Rust Android target:

```bash
rustup target add aarch64-linux-android
```

Configure the Android toolchain:

```bash
export JAVA_HOME=/path/to/jdk-21
export ANDROID_HOME=/path/to/android-sdk
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export NDK_HOME="$ANDROID_HOME/ndk/29.0.14206865"
```

### Build the APK

```bash
pnpm install
pnpm android:prepare
pnpm tauri android build --apk --target aarch64
```

Build a debug APK:

```bash
pnpm tauri android build --apk --target aarch64 --debug
```

The customized Android Studio project lives in `src-tauri/gen/android` and must
be preserved. Do not run `tauri android init` over this directory because it
would replace Rayload's share target, foreground service, DocumentsUI bridge,
and Kotlin entry point.

Release signing is read from the local `signing/keystore.properties` file.
Keystores and signing passwords are never committed.

### Quality checks

```bash
pnpm lint
pnpm format:check
pnpm check:repo
pnpm test
npx vue-tsc --noEmit
cargo test --manifest-path src-tauri/Cargo.toml --workspace --all-targets
```

## Browser RPC configuration

Rayload exposes an authenticated Aria2 JSON-RPC endpoint on the Android device:

```text
Address: http://127.0.0.1:6800/jsonrpc
Secret:  Settings → Advanced → RPC Secret
```

Install an extension compatible with Aria2 JSON-RPC in Firefox for Android or a
Chromium build that supports extension sideloading, then enter the address and
paste the RPC secret.

## Project structure

```text
src/                         Vue frontend
src-tauri/src/               Rust backend and native services
src-tauri/gen/android/       Customized Android Studio project
src-tauri/locales/           Native JSON translations
src/shared/locales/          Frontend JSON translations
scripts/android/             Android Kotlin source templates
```

## License and attribution

Rayload is based on [Rayburst](https://github.com/AnInsomniacy/rayburst), created
by **AnInsomniacy**, and follows the original project's
[MIT License](LICENSE). Rayload is independently maintained and is not an
official Rayburst release or endorsed by the original author.
