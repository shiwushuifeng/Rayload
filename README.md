<div align="center">

<img src="docs/screenshots/logo.png" width="112" alt="Rayload logo" />

# Rayload

**A fast, private Android downloader for HTTP, BitTorrent, Magnet and ED2K.**

Built on Aria2 Next with a mobile-first interface, background downloads,
share-to-download, a floating speed dashboard and a keep-awake mode.

[<img src="https://img.shields.io/github/v/release/shiwushuifeng/Rayload?style=flat-square&label=release" alt="Latest release" />](https://github.com/shiwushuifeng/Rayload/releases/latest)
<img src="https://img.shields.io/badge/Android-7.0%2B-3DDC84?style=flat-square&logo=android&logoColor=white" alt="Android 7.0+" />
<img src="https://img.shields.io/badge/ABI-arm64--v8a-4C8BF5?style=flat-square" alt="arm64-v8a" />
<img src="https://img.shields.io/badge/License-MIT-2EA44F?style=flat-square" alt="MIT License" />
<img src="https://img.shields.io/github/stars/shiwushuifeng/Rayload?style=flat-square" alt="GitHub stars" />

[**Download APK**](https://github.com/shiwushuifeng/Rayload/releases/latest) ·
[Features](#features) · [Screenshots](#screenshots) · [Build](#build-from-source) · [简体中文](#简体中文)

</div>

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[**English + 简体中文**](README.md) · [简体中文 · Chinese Simplified](docs/README.zh-CN.md) · [繁體中文 · Chinese Traditional](docs/README.zh-TW.md) · [日本語 · Japanese](docs/README.ja.md) · [한국어 · Korean](docs/README.ko.md) · [Français · French](docs/README.fr.md) · [Deutsch · German](docs/README.de.md) · [Español · Spanish](docs/README.es.md) · [Português · Portuguese (Brazil)](docs/README.pt-BR.md) · [Русский · Russian](docs/README.ru.md) · [Türkçe · Turkish](docs/README.tr.md) · [العربية · Arabic](docs/README.ar.md) · [Български · Bulgarian](docs/README.bg.md) · [Català · Catalan](docs/README.ca.md) · [Ελληνικά · Greek](docs/README.el.md) · [فارسی · Persian](docs/README.fa.md) · [Magyar · Hungarian](docs/README.hu.md) · [हिन्दी · Hindi](docs/README.hi.md) · [Bahasa Indonesia · Indonesian](docs/README.id.md) · [Italiano · Italian](docs/README.it.md) · [Norsk Bokmål · Norwegian](docs/README.nb.md) · [Nederlands · Dutch](docs/README.nl.md) · [Polski · Polish](docs/README.pl.md) · [Română · Romanian](docs/README.ro.md) · [ไทย · Thai](docs/README.th.md) · [Українська · Ukrainian](docs/README.uk.md) · [Tiếng Việt · Vietnamese](docs/README.vi.md)

</details>
<!-- language-selector:end -->

## Screenshots

<table>
  <tr>
    <td align="center"><img src="docs/screenshots/task-list.webp" width="270" alt="Rayload task list" /><br /><sub>Seamless task list with progress and quick actions</sub></td>
    <td align="center"><img src="docs/screenshots/speedometer.webp" width="270" alt="Rayload speedometer panel" /><br /><sub>Floating live dashboard, limits and bulk controls</sub></td>
  </tr>
  <tr>
    <td align="center"><img src="docs/screenshots/keep-awake.webp" width="270" alt="Rayload keep-awake mode" /><br /><sub>Keep-awake dashboard with live speed and task count</sub></td>
    <td align="center"><img src="docs/screenshots/settings.webp" width="270" alt="Rayload settings" /><br /><sub>Android-first settings and 27 interface languages</sub></td>
  </tr>
</table>

## Why Rayload

- **One downloader for every link.** HTTP/HTTPS, FTP, BitTorrent, magnet links,
  Metalink and ED2K run through the same Aria2 Next engine.
- **Designed for phones, not ported from desktop.** Share links from any
  browser, download in the background, and control everything from a floating
  speedometer or the foreground notification.
- **Private by design.** No account, no telemetry and no cloud relay. The local
  Aria2 JSON-RPC endpoint is authenticated and bound to the device.
- **Made to fit your workflow.** Infinite task list, full/compact cards, speed
  limits, scheduled limits, file selection, task trash, themes and a
  customizable bottom navigation.

## Features

### Downloads

- Multi-protocol engine: HTTP, HTTPS, FTP, BitTorrent, magnet, Metalink and ED2K.
- Pause, resume, retry and remove individual tasks; bulk pause and resume.
- Per-task speed limits plus scheduled global limits.
- BitTorrent file selection with select-all, independent scrolling and
  configurable download policy.
- Tracker management, peer blocklists, DHT, PEX, LPD and encrypted transport.
- SHA-256 verification and a recoverable file trash for downloaded files.

### Mobile experience

- Seamless infinite-scroll task list with full and compact display modes.
- Floating speedometer with live speeds, connection quality, task counts, bulk
  controls, limits and engine restart.
- Edge-docked speed chip that can be dragged back into the full capsule.
- Keep-awake dashboard with theme-aware logo, breathing glow, live speed and
  active task count; respects reduced-motion settings.
- Share-to-download from browsers and other apps, with files saved to the public
  `Download/Rayload` folder by default.

### Android integration

- Foreground download service with a localized persistent notification and
  working pause/resume actions.
- Native wake lock while downloading, optional screen-on keep-awake mode, and
  optional shutdown when all downloads finish.
- DocumentsUI folder picker, FileProvider sharing, all-files access guidance and
  battery-optimization guidance.
- Static gold launcher icon plus in-app theme palettes: gold, violet, blue and
  teal.

### Customization

- Light, dark and system appearance with an in-app quick toggle.
- Choose the app logo, launcher icon, card density and navigation entries.
- 27 interface languages with English as the schema fallback.

## Installation

Download the latest ARM64 APK from
[GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest):

```text
Rayload-v1.0.2-android17-arm64-release.apk
```

| Item           | Requirement                 |
| -------------- | --------------------------- |
| Minimum system | Android 7.0 / API 24        |
| Target system  | Android 17 / API 37         |
| Production ABI | `arm64-v8a`                 |
| Application ID | `com.shiwushuifeng.rayload` |

On first launch, complete the Android setup steps:

1. Grant all-files access so Rayload can write to the public Downloads folder.
2. Allow notifications so the foreground download service can report status.
3. Optionally allow battery-optimization exemption for reliable background
   transfers.

Release APKs use the independent Rayload signing identity. Debug APKs use the
Android debug certificate; uninstall the other variant before switching between
them. Rayload does not target desktop platforms. Standard Chrome for Android
cannot load desktop extensions; use Firefox for Android or a Chromium build with
extension sideloading for browser integration.

## Browser RPC configuration

Rayload exposes an authenticated Aria2 JSON-RPC endpoint on the device:

```text
Address: http://127.0.0.1:6800/jsonrpc
Secret:  Settings → Advanced → RPC Secret
```

Install an extension compatible with Aria2 JSON-RPC in Firefox for Android or a
Chromium build that supports extension sideloading, then enter the address and
paste the RPC secret.

## Build from source

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

```bash
rustup target add aarch64-linux-android

export JAVA_HOME=/path/to/jdk-21
export ANDROID_HOME=/path/to/android-sdk
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export NDK_HOME="$ANDROID_HOME/ndk/29.0.14206865"

pnpm install
pnpm android:prepare
pnpm tauri android build --apk --target aarch64
```

Build a debug APK with `--debug`. Release signing is read from the local
`signing/keystore.properties` file; keystores and passwords are never committed.

The customized Android Studio project lives in `src-tauri/gen/android` and must
be preserved. Do not run `tauri android init` over it; that would replace
Rayload's share target, foreground service, DocumentsUI bridge and Kotlin entry
point.

### Quality checks

```bash
pnpm lint
pnpm format:check
pnpm check:repo
pnpm test
npx vue-tsc --noEmit
cargo test --manifest-path src-tauri/Cargo.toml --workspace --all-targets
```

## Project structure

```text
src/                         Vue frontend
src-tauri/src/               Rust backend and native services
src-tauri/gen/android/       Customized Android Studio project
src-tauri/locales/           Native JSON translations
src/shared/locales/          Frontend JSON translations
scripts/android/             Android Kotlin source templates
docs/screenshots/            README screenshots
```

## Contributing and support

- Found a bug or have a feature request? Open an issue with the app version,
  device model and reproduction steps.
- Pull requests are welcome for Android behavior, translations and documentation.
- If Rayload saves you time, a star helps other Android users find it.

## License and attribution

Rayload is an independent, unofficial Android port based on
[Rayburst](https://github.com/AnInsomniacy/rayburst), created by
**AnInsomniacy**, and follows the original project's [MIT License](LICENSE).
Rayload is independently maintained and is not an official Rayburst release or
endorsed by the original author.

---

## 简体中文

<div align="center">

**极速、私密的 Android 下载器：HTTP、BitTorrent、Magnet、ED2K 一个应用全搞定。**

[**下载 APK**](https://github.com/shiwushuifeng/Rayload/releases/latest) ·
[功能](#功能亮点) · [截图](#截图) · [编译](#从源码编译)

</div>

Rayload 是 Rayburst 的独立非官方 Android 移植版，仅专注于 Android 平台。
项目使用 Tauri、Rust、Vue 3 和 Aria2 Next 构建，提供移动端界面、系统分享、
后台下载、通知、公共下载目录集成以及本地 Aria2 JSON-RPC 服务。

### 截图

<table>
  <tr>
    <td align="center"><img src="docs/screenshots/task-list.webp" width="270" alt="任务列表" /><br /><sub>无缝任务列表与快捷操作</sub></td>
    <td align="center"><img src="docs/screenshots/speedometer.webp" width="270" alt="速度球面板" /><br /><sub>悬浮实时状态、限速与批量控制</sub></td>
  </tr>
  <tr>
    <td align="center"><img src="docs/screenshots/keep-awake.webp" width="270" alt="不息屏挂机" /><br /><sub>挂机页：实时速度与任务数</sub></td>
    <td align="center"><img src="docs/screenshots/settings.webp" width="270" alt="设置" /><br /><sub>Android 优先的设置与 27 种语言</sub></td>
  </tr>
</table>

### 功能亮点

- **一个引擎下载所有链接**：HTTP/HTTPS、FTP、BitTorrent、磁力链、Metalink、ED2K。
- **为手机而生**：浏览器分享即下载、后台前台服务、通知栏暂停/继续、悬浮速度球与不息屏挂机。
- **隐私优先**：无账号、无遥测、无云端中转；本地 Aria2 JSON-RPC 带鉴权并仅绑定本机。
- **顺手好用**：无限滚动任务列表、完整/紧凑卡片、限速与定时限速、种子文件全选、回收站、主题与底部导航自定义。

### 安装

从 [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)
下载最新 ARM64 APK：

```text
Rayload-v1.0.2-android17-arm64-release.apk
```

| 项目     | 要求                        |
| -------- | --------------------------- |
| 最低系统 | Android 7.0 / API 24        |
| 目标系统 | Android 17 / API 37         |
| 生产架构 | `arm64-v8a`                 |
| 应用包名 | `com.shiwushuifeng.rayload` |

首次启动时可以选择界面语言，随后完成 Android 初始化：

1. 授予“所有文件访问权限”，以便写入公共下载目录。
2. 允许通知，以便前台下载服务显示状态并支持暂停/继续。
3. 按需授予电池优化豁免，避免后台下载被系统中断。

默认下载目录：`/storage/emulated/0/Download/Rayload`。

Release APK 使用 Rayload 独立签名，Debug APK 使用 Android 调试证书；两者签名
不同，切换安装前需要先卸载另一版本。

### 浏览器 RPC 配置

```text
地址：http://127.0.0.1:6800/jsonrpc
密钥：设置 → 高级 → RPC Secret
```

在 Firefox Android 或支持扩展侧载的 Chromium 浏览器中，安装兼容 Aria2
JSON-RPC 的扩展，填写上述地址，并粘贴设置页中的 RPC Secret。

### 从源码编译

```bash
rustup target add aarch64-linux-android

export JAVA_HOME=/path/to/jdk-21
export ANDROID_HOME=/path/to/android-sdk
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export NDK_HOME="$ANDROID_HOME/ndk/29.0.14206865"

pnpm install
pnpm android:prepare
pnpm tauri android build --apk --target aarch64
```

调试包追加 `--debug`。Release 签名从本地 `signing/keystore.properties` 读取，
密钥库和密码不会提交到 Git。定制的 Android Studio 工程位于
`src-tauri/gen/android`，不要重新执行 `tauri android init`。

### 质量检查

```bash
pnpm lint
pnpm format:check
pnpm check:repo
pnpm test
npx vue-tsc --noEmit
cargo test --manifest-path src-tauri/Cargo.toml --workspace --all-targets
```

### 参与与支持

- 遇到问题请提交 Issue，并附上版本号、设备型号与复现步骤。
- 欢迎提交 Android 行为、翻译与文档相关的 Pull Request。
- 如果 Rayload 对你有帮助，点一个 Star 能让更多 Android 用户发现它。

### 致谢与许可

Rayload 基于 **AnInsomniacy** 创建的
[Rayburst](https://github.com/AnInsomniacy/rayburst)，并遵循原项目的
[MIT License](LICENSE)。Rayload 由独立维护者发布，不是 Rayburst 官方版本，
也不代表原作者背书。
