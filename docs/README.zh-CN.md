# Rayload - Android 下载器

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [**简体中文 · Chinese Simplified**](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

Rayload 是 Rayburst 的独立非官方 Android 移植版，仅专注于 Android 平台。
项目使用 Tauri、Rust、Vue 3 和 Aria2 Next 构建，提供移动端界面、系统分享、
后台下载、通知、公共下载目录集成以及本地 Aria2 JSON-RPC 服务。

## 平台要求

| 项目     | 要求                        |
| -------- | --------------------------- |
| 最低系统 | Android 7.0 / API 24        |
| 目标系统 | Android 17 / API 37         |
| 生产架构 | `arm64-v8a`                 |
| 应用包名 | `com.shiwushuifeng.rayload` |

Rayload 不支持桌面平台。标准 Google Chrome for Android 不能加载桌面扩展；
需要浏览器联动时，请使用 Firefox Android 或支持扩展侧载的 Chromium 构建。

## 安装

从 [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases)
下载最新 ARM64 APK：

```text
Rayload-v1.0.2-android17-arm64-release.apk
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

## Android 编译指南

### 依赖

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

### 构建 APK

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

### 质量检查

```bash
pnpm lint
pnpm format:check
pnpm check:repo
pnpm test
npx vue-tsc --noEmit
cargo test --manifest-path src-tauri/Cargo.toml --workspace --all-targets
```

## 浏览器 RPC 配置

Rayload 在 Android 本机提供带鉴权的 Aria2 JSON-RPC 服务：

```text
地址：http://127.0.0.1:6800/jsonrpc
密钥：设置 → 高级 → RPC Secret
```

在 Firefox Android 或支持扩展侧载的 Chromium 浏览器中，安装兼容 Aria2
JSON-RPC 的扩展，填写上述地址，并粘贴设置页中的 RPC Secret。

## 项目结构

```text
src/                         Vue 前端
src-tauri/src/               Rust 后端与原生服务
src-tauri/gen/android/       定制 Android Studio 工程
src-tauri/locales/           原生 JSON 翻译
src/shared/locales/          前端 JSON 翻译
scripts/android/             Android Kotlin 源模板
```

## 致谢与许可

Rayload 基于 **AnInsomniacy** 创建的
[Rayburst](https://github.com/AnInsomniacy/rayburst)，并遵循原项目的
[MIT License](LICENSE)。Rayload 由独立维护者发布，不是 Rayburst 官方版本，
也不代表原作者背书。
