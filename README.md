# Rayload - Android Downloader / Android 下载器

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[**English + 简体中文**](README.md) · [简体中文 · Chinese Simplified](docs/README.zh-CN.md) · [繁體中文 · Chinese Traditional](docs/README.zh-TW.md) · [日本語 · Japanese](docs/README.ja.md) · [한국어 · Korean](docs/README.ko.md) · [Français · French](docs/README.fr.md) · [Deutsch · German](docs/README.de.md) · [Español · Spanish](docs/README.es.md) · [Português · Portuguese (Brazil)](docs/README.pt-BR.md) · [Русский · Russian](docs/README.ru.md) · [Türkçe · Turkish](docs/README.tr.md) · [العربية · Arabic](docs/README.ar.md) · [Български · Bulgarian](docs/README.bg.md) · [Català · Catalan](docs/README.ca.md) · [Ελληνικά · Greek](docs/README.el.md) · [فارسی · Persian](docs/README.fa.md) · [Magyar · Hungarian](docs/README.hu.md) · [हिन्दी · Hindi](docs/README.hi.md) · [Bahasa Indonesia · Indonesian](docs/README.id.md) · [Italiano · Italian](docs/README.it.md) · [Norsk Bokmål · Norwegian](docs/README.nb.md) · [Nederlands · Dutch](docs/README.nl.md) · [Polski · Polish](docs/README.pl.md) · [Română · Romanian](docs/README.ro.md) · [ไทย · Thai](docs/README.th.md) · [Українська · Ukrainian](docs/README.uk.md) · [Tiếng Việt · Vietnamese](docs/README.vi.md)

</details>
<!-- language-selector:end -->

## English

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

---

## 简体中文

Rayload 是 Rayburst 的独立非官方 Android 移植版，仅专注于 Android 平台。
项目使用 Tauri、Rust、Vue 3 和 Aria2 Next 构建，提供移动端界面、系统分享、
后台下载、通知、公共下载目录集成以及本地 Aria2 JSON-RPC 服务。

### 平台要求

| 项目     | 要求                        |
| -------- | --------------------------- |
| 最低系统 | Android 7.0 / API 24        |
| 目标系统 | Android 17 / API 37         |
| 生产架构 | `arm64-v8a`                 |
| 应用包名 | `com.shiwushuifeng.rayload` |

Rayload 不支持桌面平台。标准 Google Chrome for Android 不能加载桌面扩展；
需要浏览器联动时，请使用 Firefox Android 或支持扩展侧载的 Chromium 构建。

### 安装

从 [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases)
下载最新 ARM64 APK：

```text
Rayload-v1.0.1-android17-arm64-release.apk
```

首次启动时可以选择界面语言，随后完成 Android 初始化：

1. 授予“所有文件访问权限”，以便写入公共下载目录。
2. 允许通知，以便前台下载服务显示状态。
3. 按需授予电池优化豁免，避免后台下载被系统中断。

默认下载目录：

```text
/storage/emulated/0/Download/Rayload
```

Release APK 使用 Rayload 独立签名。Debug APK 使用 Android 调试证书，两者
签名不同；切换安装前需要先卸载另一版本。

### Android 编译指南

#### 依赖

| 工具        | 版本或目标                             |
| ----------- | -------------------------------------- |
| Node.js     | 22 或更高版本                          |
| pnpm        | `11.5.2`，与 `packageManager` 保持一致 |
| Rust        | stable，并安装 `aarch64-linux-android` |
| Java        | JDK 21                                 |
| Android SDK | Platform 37、Build Tools 37.0.0        |
| Android NDK | `29.0.14206865`                        |
| Git         | 当前稳定版                             |
| GitHub CLI  | 可选，仅用于发布                       |

安装 Rust Android 目标：

```bash
rustup target add aarch64-linux-android
```

配置 Android 工具链：

```bash
export JAVA_HOME=/path/to/jdk-21
export ANDROID_HOME=/path/to/android-sdk
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export NDK_HOME="$ANDROID_HOME/ndk/29.0.14206865"
```

#### 构建 APK

```bash
pnpm install
pnpm android:prepare
pnpm tauri android build --apk --target aarch64
```

构建 Debug APK：

```bash
pnpm tauri android build --apk --target aarch64 --debug
```

定制的 Android Studio 工程位于 `src-tauri/gen/android`，必须保留。不要在该
目录上重新执行 `tauri android init`；它会覆盖 Rayload 的分享接收、前台服务、
DocumentsUI 桥接和 Kotlin 入口。

Release 签名从本地 `signing/keystore.properties` 读取。密钥库和签名密码不会
提交到 Git。

#### 质量检查

```bash
pnpm lint
pnpm format:check
pnpm check:repo
pnpm test
npx vue-tsc --noEmit
cargo test --manifest-path src-tauri/Cargo.toml --workspace --all-targets
```

### 浏览器 RPC 配置

Rayload 在 Android 本机提供带鉴权的 Aria2 JSON-RPC 服务：

```text
地址：http://127.0.0.1:6800/jsonrpc
密钥：设置 → 高级 → RPC Secret
```

在 Firefox Android 或支持扩展侧载的 Chromium 浏览器中，安装兼容 Aria2
JSON-RPC 的扩展，填写上述地址，并粘贴设置页中的 RPC Secret。

### 项目结构

```text
src/                         Vue 前端
src-tauri/src/               Rust 后端与原生服务
src-tauri/gen/android/       定制 Android Studio 工程
src-tauri/locales/           原生 JSON 翻译
src/shared/locales/          前端 JSON 翻译
scripts/android/             Android Kotlin 源模板
```

### 致谢与许可

Rayload 基于 **AnInsomniacy** 创建的
[Rayburst](https://github.com/AnInsomniacy/rayburst)，并遵循原项目的
[MIT License](LICENSE)。Rayload 由独立维护者发布，不是 Rayburst 官方版本，
也不代表原作者背书。
