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

## 📚 文档导航 (Documentation)

第一次使用 Rayload？先看快速上手，再按使用场景打开对应指南。

| 文档 | 内容简介 |
| --- | --- |
| 🚀 [快速上手](QUICK_START.md) | 选择正确的安装包、授予权限并完成第一个下载。 |
| 🌐 [局域网互联指南](LAN_GUIDE.md) | 双向配对、扫码与局域网发现、跨端任务推送与实时统计。 |
| 🔌 [插件开发指南](PLUGIN_DEVELOPMENT_GUIDE.md) | JSON 解析规则、网络白名单、请求头改写与 `{{password}}` 变量。 |
| 💻 [CLI 命令行手册](HEADLESS_CLI.md) | NAS/服务器无头部署、YAML 配置详解与 `ray` 命令全集。 |
| ❓ [常见问题](FAQ.md) | 下载慢、配对失败、权限被拒与链接解析失败。 |
| 📱 [Android 说明](ANDROID.md) | 构建变体、签名差异与 Android 平台特性。 |
| 🗺️ [路线图](../ROADMAP.md) | 计划中的功能与工程方向。 |

## 项目愿景

> 🚀 致力于打造 Android 平台最纯粹、极速、可定制的开源下载器。无广告，无遥测，由极客打造，为极客服务。

坚持隐私优先，用一个引擎处理所有协议，提供原生移动端体验，并通过主题与插件
持续扩展。未来计划见[路线图](../ROADMAP.md)。

## v1.0.4 更新亮点

- 双向局域网配对：双方确认设备与指纹并互相保存，任意一方都能推送任务、
  查看实时统计。
- 任务列表汇总已配对设备的远程任务，支持暂停、继续与删除，并显示离线状态。
- 数据统计页新增“本机”与“全部设备”两个 Tab，展示各设备实时速度与活动任务数。
- JSON 插件权限沙箱：风险分级授权、域名白名单与请求头强制剥离。
- `ray` CLI 支持 YAML 配置、守护进程、systemd 与终端任务控制。
- 拆分为轻量版、完整版与 F-Droid 版；F-Droid 版使用独立包名，并彻底移除
  GitHub 自更新。

## 平台要求

| 项目     | 要求                        |
| -------- | --------------------------- |
| 最低系统 | Android 7.0 / API 24        |
| 目标系统 | Android 17 / API 37         |
| 生产架构 | `arm64-v8a`、`x86_64`（完整版） |
| 应用包名 | `com.shiwushuifeng.rayload` |

发布附件：

| 附件 | 用途 |
| --- | --- |
| `Rayload-v1.0.4-lite-arm64.apk` | ARM64 轻量包（核心下载功能） |
| `Rayload-v1.0.4-full-arm64.apk` | 含局域网与插件引擎的 ARM64 完整包 |
| `Rayload-v1.0.4-full-x86_64.apk` | x86_64 完整包（x86 设备与模拟器） |
| `Rayload-v1.0.4-fdroid-arm64.apk` | F-Droid ARM64 包（由商店管理更新） |
| `rayload-cli-v1.0.4-linux-x86_64.tar.gz` | Linux 无界面 CLI（含 YAML 示例） |
| `SHA256SUMS-v1.0.4.txt` | 全部发行产物的 SHA-256 校验值 |

Rayload 不支持桌面平台。标准 Google Chrome for Android 不能加载桌面扩展；
需要浏览器联动时，请使用 Firefox Android 或支持扩展侧载的 Chromium 构建。

## 安装

从 [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases)
下载最新 ARM64 APK：

### 快速开始

1. 从 Releases 下载最新 APK，并安装到 ARM64 设备。
2. 授予“所有文件访问权限”并允许通知，以便后台下载正常运行。
3. 点击 **+**，或从任意浏览器分享链接，开始第一个下载任务。

长时间下载建议同时授予电池优化豁免。

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
docs/screenshots/            README 截图
ROADMAP.md                   产品与工程路线图
```

## 常见问题

### 通知栏没有暂停/继续按钮

请允许 Rayload 发送通知、关闭电池优化，并重新打开一次应用。部分定制
Android 系统还会在前台服务重启前隐藏通知操作按钮。

### 后台下载会中断

请将 Rayload 排除在电池优化和深度睡眠限制之外。在限制较激进的部分系统上，
保持应用打开或充电时使用不息屏模式会更稳定。

### 浏览器无法连接 RPC

Chrome for Android 不支持桌面扩展。请使用 Firefox Android 或支持扩展侧载的
Chromium 构建，并填写**设置 → 高级**中的地址与 RPC Secret。

### 切换安装包时提示“应用未安装”

Release 与 Debug APK 使用不同的签名证书。安装另一个版本前，请先卸载当前版本。

更多问题与排查步骤见 [完整常见问题](FAQ.md)。

## 致谢与许可

Rayload 以 [GPL-3.0](../LICENSE) 分发，是 **AnInsomniacy** 创建的
[Rayburst](https://github.com/AnInsomniacy/rayburst)（MIT 许可）的独立非官方
Android 移植版。内置的 Aria2 Next（`aria2-next`）引擎按 GPL-2.0-or-later
许可分发。三方组件的版权与许可证声明（包括保留的 Rayburst MIT 全文）见
[NOTICE](../NOTICE)。Rayload 由独立维护者发布，不是 Rayburst 官方版本，也不代表
原作者背书。

---

<div align="center">

**⭐ 如果 Rayload 对你有帮助，请点一个 Star，让更多 Android 用户发现它。**

👉 [查看我们的完整开发路线图与未来目标 (Roadmap)](../ROADMAP.md)

</div>

> ❤️ 喜欢这个项目？欢迎 [赞助与支持 (Sponsor)](../SPONSOR.md)
