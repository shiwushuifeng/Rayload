<div align="center">

<img src="docs/screenshots/logo.png" width="112" alt="Rayload logo" />

# Rayload

> 🚀 致力于打造 Android 平台最纯粹、极速、可定制的开源下载器。无广告，无遥测，由极客打造，为极客服务。
>
> 🚀 Building the purest, fastest and most customizable open-source downloader
> for Android. No ads, no telemetry — built by geeks, for geeks.

**A fast, private Android downloader for HTTP, BitTorrent, Magnet and ED2K.**

Built on Aria2 Next with a mobile-first interface, background downloads,
share-to-download, a floating speed dashboard and a keep-awake mode.

[<img src="https://img.shields.io/github/v/release/shiwushuifeng/Rayload?style=flat-square&label=release" alt="Latest release" />](https://github.com/shiwushuifeng/Rayload/releases/latest)
<img src="https://img.shields.io/badge/Android-7.0%2B-3DDC84?style=flat-square&logo=android&logoColor=white" alt="Android 7.0+" />
<img src="https://img.shields.io/badge/ABI-arm64--v8a-4C8BF5?style=flat-square" alt="arm64-v8a" />
<img src="https://img.shields.io/badge/License-GPLv3-2EA44F?style=flat-square" alt="GPL-3.0 License" />
<img src="https://img.shields.io/github/stars/shiwushuifeng/Rayload?style=flat-square" alt="GitHub stars" />

[**Download APK**](https://github.com/shiwushuifeng/Rayload/releases/latest) ·
[Why Rayload](#why-rayload) · [Screenshots](#screenshots) · [Roadmap](ROADMAP.md) ·
[Build](#build-from-source) · [简体中文](#简体中文)

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

## Vision

Rayload aims to be the most focused, fastest and most customizable open-source
downloader on Android. We are building a downloader that stays private by
default, runs every protocol through one reliable engine, feels native on a
phone, and grows through themes and plugins without losing its simplicity. See
the [roadmap](ROADMAP.md) for what comes next.

## v1.0.3 Highlights

- Finished core download and task management: multi-line links, anti-leech
  headers, long-press multi-select, swipe actions, duplicate filtering, storage
  warnings and `MD5`/`SHA1`/`SHA256` verification.
- Added the skippable onboarding flow, centralized permissions and notification
  controls, launcher shortcut, GitHub update checks, JSON config transfer and
  LAN connection panel.
- Added the home overview and statistics dashboard with live-speed sampling,
  7-day trends, activity-based average speed and category totals.
- Polished motion and accessibility: animated numbers, ripple feedback, damped
  swipe-out cards, SVG empty states, 44px touch targets and dark-mode contrast.

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

- Light, dark and system appearance; keep-awake mode adds a quick light/dark
  toggle.
- Choose the app logo, launcher icon, card density and navigation entries.
- 27 interface languages with English as the schema fallback.

## Installation

Download the latest ARM64 APK from
[GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest):

| Item           | Requirement                 |
| -------------- | --------------------------- |
| Minimum system | Android 7.0 / API 24        |
| Target system  | Android 17 / API 37         |
| Production ABI | `arm64-v8a`                 |
| Application ID | `com.shiwushuifeng.rayload` |

Release artifacts:

| Artifact | Purpose |
| --- | --- |
| `Rayload-v1.0.3-android17-arm64-release.apk` | Production ARM64 release |
| `Rayload-v1.0.3-android17-arm64-debug.apk` | ARM64 debug/testing build |
| `Rayload-v1.0.3-x86_64-emulator.apk` | x86_64 emulator build |
| `SHA256SUMS-v1.0.3.txt` | SHA-256 checksums for all APKs |

### Quick start

1. Download the latest APK from Releases and install it on an ARM64 device.
2. Grant all-files access and allow notifications so downloads can run in the
   background.
3. Tap **+** or share a link from any browser to start your first download.

For reliable long downloads, also allow battery-optimization exemption. The
default download folder is `/storage/emulated/0/Download/Rayload`.

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

### Architecture

```text
Vue 3 + Pinia + Naive UI
          │ Tauri IPC
Rust backend + native Android bridge
          │ JSON-RPC / process control
      Aria2 Next (arm64-v8a)
```

```text
src/                         Vue frontend
src-tauri/src/               Rust backend and native services
src-tauri/gen/android/       Customized Android Studio project
src-tauri/locales/           Native JSON translations
src/shared/locales/          Frontend JSON translations
scripts/android/             Android Kotlin source templates
docs/screenshots/            README screenshots
ROADMAP.md                   Product and engineering roadmap
```

## Troubleshooting and FAQ

### The notification has no pause or resume button

Allow notifications for Rayload, disable battery optimization, and reopen the
app once. Some customized Android ROMs also hide notification actions until the
foreground service is restarted.

### Downloads stop in the background

Exclude Rayload from battery and deep-sleep restrictions. Keeping the app open,
or using keep-awake mode while charging, helps on aggressive vendor ROMs.

### My browser cannot connect to the RPC endpoint

Chrome for Android cannot load desktop extensions. Use Firefox for Android or a
Chromium build that supports extension sideloading, then enter the address and
RPC secret from **Settings → Advanced**.

### "App not installed" appears when switching builds

Release and debug APKs use different signing identities. Uninstall the existing
version before installing the other variant.

## Contributing and support

- Found a bug or have a feature request? Open an issue with the app version,
  device model and reproduction steps.
- Pull requests are welcome for Android behavior, translations and documentation.
- If Rayload saves you time, a star helps other Android users find it.

## License and attribution

Rayload is licensed under the **GNU General Public License v3.0 only
(GPL-3.0)**. It is an independent, unofficial Android port based on
[Rayburst](https://github.com/AnInsomniacy/rayburst), created by
**AnInsomniacy** and licensed under the MIT License. The bundled Aria2 Next
(`aria2-next`) engine is licensed under **GPL-2.0-or-later**.

| Component | License |
| --------- | ------- |
| Rayload | GPL-3.0-only |
| Rayburst | MIT |
| aria2-next | GPL-2.0-or-later |

Copyright and license notices for all third-party components, including the
preserved Rayburst MIT text, are documented in [NOTICE](NOTICE). Rayload is
independently maintained and is not an official Rayburst release or endorsed by
the original author.

## Support

❤️ 喜欢这个项目？欢迎 [赞助与支持 (Sponsor)](./SPONSOR.md)

---

## 简体中文

<div align="center">

**极速、私密的 Android 下载器：HTTP、BitTorrent、Magnet、ED2K 一个应用全搞定。**

[**下载 APK**](https://github.com/shiwushuifeng/Rayload/releases/latest) ·
[功能](#功能亮点) · [截图](#截图) · [路线图](ROADMAP.md) · [编译](#从源码编译)

</div>

Rayload 是 Rayburst 的独立非官方 Android 移植版，仅专注于 Android 平台。
项目使用 Tauri、Rust、Vue 3 和 Aria2 Next 构建，提供移动端界面、系统分享、
后台下载、通知、公共下载目录集成以及本地 Aria2 JSON-RPC 服务。

### 项目愿景

打造 Android 平台上最纯粹、极速、可定制的开源下载器。坚持隐私优先，用一个
引擎处理所有协议，提供原生移动端体验，并通过主题与插件持续扩展。未来计划见
[路线图](ROADMAP.md)。

### v1.0.3 更新亮点

- 完成核心下载与任务管理：多行链接拆分、防盗链请求头、长按多选、滑动操作、
  重复任务过滤、存储预警与 `MD5`/`SHA1`/`SHA256` 校验。
- 新增可跳过的首次向导、集中式权限与通知管理、桌面快捷方式、GitHub 更新检查、
  JSON 配置导入导出和局域网互联面板。
- 新增主页概览与数据统计页：实时网速采样、7 天趋势、按活动时间计算的平均速率
  与分类统计。
- 打磨动效与可访问性：数字滚动、水波纹、弹性滑出、SVG 空状态、44px 热区与
  暗色模式对比度。

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

| 项目     | 要求                        |
| -------- | --------------------------- |
| 最低系统 | Android 7.0 / API 24        |
| 目标系统 | Android 17 / API 37         |
| 生产架构 | `arm64-v8a`                 |
| 应用包名 | `com.shiwushuifeng.rayload` |

发布附件：

| 附件 | 用途 |
| --- | --- |
| `Rayload-v1.0.3-android17-arm64-release.apk` | ARM64 正式发布包 |
| `Rayload-v1.0.3-android17-arm64-debug.apk` | ARM64 调试/测试包 |
| `Rayload-v1.0.3-x86_64-emulator.apk` | x86_64 模拟器包 |
| `SHA256SUMS-v1.0.3.txt` | 全部 APK 的 SHA-256 校验值 |

#### 快速开始

1. 从 Releases 下载最新 APK，并安装到 ARM64 设备。
2. 授予“所有文件访问权限”并允许通知，以便后台下载正常运行。
3. 点击 **+**，或从任意浏览器分享链接，开始第一个下载任务。

长时间下载建议同时授予电池优化豁免。默认下载目录为
`/storage/emulated/0/Download/Rayload`。

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

### 项目结构

```text
Vue 3 + Pinia + Naive UI
          │ Tauri IPC
Rust 后端 + Android 原生桥接
          │ JSON-RPC / 进程控制
      Aria2 Next (arm64-v8a)
```

```text
src/                         Vue 前端
src-tauri/src/               Rust 后端与原生服务
src-tauri/gen/android/       定制 Android Studio 工程
src-tauri/locales/           原生 JSON 翻译
src/shared/locales/          前端 JSON 翻译
scripts/android/             Android Kotlin 源模板
docs/screenshots/            README 截图
ROADMAP.md                   产品与工程路线图
```

### 常见问题

#### 通知栏没有暂停/继续按钮

请允许 Rayload 发送通知、关闭电池优化，并重新打开一次应用。部分定制
Android 系统还会在前台服务重启前隐藏通知操作按钮。

#### 后台下载会中断

请将 Rayload 排除在电池优化和深度睡眠限制之外。在限制较激进的部分系统上，
保持应用打开或充电时使用不息屏模式会更稳定。

#### 浏览器无法连接 RPC

Chrome for Android 不支持桌面扩展。请使用 Firefox Android 或支持扩展侧载的
Chromium 构建，并填写**设置 → 高级**中的地址与 RPC Secret。

#### 切换安装包时提示“应用未安装”

Release 与 Debug APK 使用不同的签名证书。安装另一个版本前，请先卸载当前版本。

### 参与与支持

- 遇到问题请提交 Issue，并附上版本号、设备型号与复现步骤。
- 欢迎提交 Android 行为、翻译与文档相关的 Pull Request。
- 如果 Rayload 对你有帮助，点一个 Star 能让更多 Android 用户发现它。

### 致谢与许可

Rayload 以 **GNU General Public License v3.0（GPL-3.0）** 分发，是
**AnInsomniacy** 创建的 [Rayburst](https://github.com/AnInsomniacy/rayburst)
（MIT 许可）的独立非官方 Android 移植版。内置的 Aria2 Next（`aria2-next`）
引擎按 **GPL-2.0-or-later** 许可分发。

| 组件 | 许可证 |
| ---- | ------ |
| Rayload | GPL-3.0-only |
| Rayburst | MIT |
| aria2-next | GPL-2.0-or-later |

三方组件的版权与许可证声明（包括保留的 Rayburst MIT 全文）见
[NOTICE](./NOTICE)。Rayload 由独立维护者发布，不是 Rayburst 官方版本，也不代表
原作者背书。

### 支持

❤️ 喜欢这个项目？欢迎 [赞助与支持 (Sponsor)](./SPONSOR.md)

---

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=shiwushuifeng/Rayload&type=Date)](https://star-history.com/#shiwushuifeng/Rayload&Date)

<div align="center">

**⭐ If Rayload saves you time, give it a star — it helps more Android users find it.**

**⭐ 如果 Rayload 对你有帮助，请点一个 Star，让更多 Android 用户发现它。**

👉 [View the full roadmap](./ROADMAP.md) · [查看我们的完整开发路线图与未来目标 (Roadmap)](./ROADMAP.md)

</div>
