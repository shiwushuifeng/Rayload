# 下载与安装指南

面向普通用户的选包说明。每个包都来自同一个代码库，区别只在功能集与目标设备。

## 快速选择

| 你的设备 | 推荐下载 | 说明 |
| --- | --- | --- |
| **绝大多数安卓手机 / 平板** | `Rayload-1.0.4.1-full-arm64.apk` | 完整功能：局域网互联、插件引擎、远程任务 |
| 只想要纯下载、不要插件 | `Rayload-1.0.4.1-lite-arm64.apk` | 去掉插件引擎与局域网，体积更小、权限更少 |
| 在 **F-Droid** 环境 / 想与另一个版本共存 | `Rayload-1.0.4.1-fdroid-arm64.apk` | 包名独立（`.fdroid`），可与主包同时安装 |
| 安卓模拟器 / x86_64 平板 | `Rayload-1.0.4.1-full-x86_64.apk` | 仅 x86_64 架构，功能同完整版 |
| 想看演示效果、截图录屏 | `Rayload-1.0.4.1-demo-arm64.apk` | 内置演示数据，不影响真实任务 |

> **不确定就选第一个**：`full-arm64` 覆盖 99% 的现代安卓设备。

## 怎么判断自己的架构

- 2017 年之后的主流手机几乎都是 **arm64-v8a**。
- 模拟器（BlueStacks、MuMu、Android Studio AVD）通常是 **x86_64**。
- 拿不准时：装 `full-arm64`，若提示"应用未安装/不兼容"，再换 `full-x86_64`。

## 安装步骤

1. 下载对应 APK。
2. 若系统提示"未知来源"，在弹窗中允许浏览器/文件管理器安装应用。
3. 直接覆盖安装即可：自 `v1.0.4-beta8` 起签名一致，**无需卸载**。
4. 首次启动会走权限引导（存储、通知、局域网），局域网权限是设备互联的前提。

## 命令行（Linux）

```bash
# 下载并安装（脚本会放置 ray / rayload / rayload-cli 三个入口）
curl -fsSL https://raw.githubusercontent.com/shiwushuifeng/Rayload/main/scripts/install-cli.sh | sh
```

或手动下载 `rayload-cli-v1.0.4.1-linux-x86_64.tar.gz`，解包后把其中的
`ray`、`rayload`、`rayload-cli` 放进 `PATH`。

```bash
ray status              # 查看守护进程与任务概览
ray add https://example.com/file.zip
ray watch               # 跟随进度直到完成
ray --json status       # 机器可读输出，便于脚本处理
```

## 校验下载完整性

每个 Release 都附带 `SHA256SUMS-1.0.4.1.txt`。Linux / macOS：

```bash
sha256sum -c SHA256SUMS-1.0.4.1.txt
```

Windows（PowerShell）：

```powershell
Get-FileHash .\Rayload-1.0.4.1-full-arm64.apk -Algorithm SHA256
```

## 版本命名

- **`1.0.4.1`** — 当前正式版（本页）。
- **`v1.0.4-beta`** — 测试版归档，全部 beta 合并为一个预览 Release，仅供回溯，请勿用于日常使用。
- 更早的 `1.0.0`–`1.0.3` 为历史稳定版。

## 系统要求

| 项 | 要求 |
| --- | --- |
| Android | 7.0（API 24）及以上 |
| 目标版本 | Android 17（API 37） |
| 架构 | arm64-v8a（主力）/ x86_64（模拟器） |
| 存储 | 约 25 MB 应用本体，另需下载目标文件的空间 |
| 网络 | 下载需要联网；局域网互联需同一 Wi-Fi |

---

# Download guide (English)

| Device | File | Notes |
| --- | --- | --- |
| Most Android phones and tablets | `Rayload-1.0.4.1-full-arm64.apk` | LAN pairing, plugin engine, remote tasks |
| Plain downloading only | `Rayload-1.0.4.1-lite-arm64.apk` | No plugins or LAN, smaller and fewer permissions |
| F-Droid setups or side-by-side | `Rayload-1.0.4.1-fdroid-arm64.apk` | Separate package id, installs next to the main build |
| Emulator / x86_64 tablet | `Rayload-1.0.4.1-full-x86_64.apk` | x86_64 only, full feature set |
| Demos and screenshots | `Rayload-1.0.4.1-demo-arm64.apk` | Bundled sample data |

Pick `full-arm64` if unsure. Install over `v1.0.4-beta8` or later without
uninstalling: the signing key has not changed. The checksum manifest is
`SHA256SUMS-1.0.4.1.txt`.

Linux CLI:

```bash
curl -fsSL https://raw.githubusercontent.com/shiwushuifeng/Rayload/main/scripts/install-cli.sh | sh
```