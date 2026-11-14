# Quick Start / 快速上手

Install Rayload, grant the Android permissions it needs, and start your first
download. / 安装 Rayload、授予必要权限，然后开始第一个下载任务。

## 1. Choose a package / 选择安装包

| Package | Use it for |
| --- | --- |
| `Rayload-*-arm64-full.apk` | Most ARM64 phones and tablets; includes LAN features |
| `Rayload-*-arm64-lite.apk` | ARM64 devices that prefer a smaller package |
| `Rayload-*-arm64-fdroid.apk` | F-Droid flavor with a separate package name and no in-app GitHub updater |
| `Rayload-*-x86_64-full.apk` | x86_64 tablets, emulators and test devices |

Release and debug APKs use different signing identities. Uninstall one before
installing the other. / Release 与 Debug 签名不同，切换前请先卸载旧版本。

## 2. Install and grant permissions / 安装与授权

1. Install the APK matching your device architecture.
2. Allow **All files access** so downloads can be written to the public folder.
3. Allow **notifications** for progress and pause/resume actions.
4. Allow **battery optimization exemption** for long background downloads.
5. On Android 17, allow **local network access** when you plan to use LAN pairing.

The default download folder is `/storage/emulated/0/Download/Rayload`.

## 3. Start downloading / 开始下载

- Tap **+** and paste an HTTP, FTP, BitTorrent, Magnet, Metalink or ED2K link.
- Or share a link from a browser and choose Rayload.
- Use the notification or the task card to pause, resume or delete a task.
- Deleted downloads move to the hidden `.trash` folder inside the download root.

## 4. Browser RPC (optional) / 浏览器 RPC（可选）

```text
Address: http://127.0.0.1:6800/jsonrpc
Secret:  Settings → Advanced → RPC Secret
```

Install an Aria2 JSON-RPC extension in Firefox for Android, or a Chromium build
that supports extension sideloading. Standard Chrome for Android cannot load
desktop extensions.

## Next steps / 下一步

- [LAN networking guide](LAN_GUIDE.md) — pair two devices and push tasks.
- [Plugin development guide](PLUGIN_DEVELOPMENT_GUIDE.md) — add cloud-drive parsing rules.
- [Headless CLI manual](HEADLESS_CLI.md) — run Rayload on a NAS or server.
- [FAQ](FAQ.md) — permissions, speed, pairing and parsing problems.

---

## 简体中文

### 1. 选择安装包

| 安装包 | 适用场景 |
| --- | --- |
| `Rayload-*-arm64-full.apk` | 绝大多数 ARM64 手机与平板，包含局域网功能 |
| `Rayload-*-arm64-lite.apk` | 希望安装包更小的 ARM64 设备 |
| `Rayload-*-arm64-fdroid.apk` | F-Droid 版本，独立包名，移除应用内 GitHub 更新 |
| `Rayload-*-x86_64-full.apk` | x86_64 平板、模拟器与测试设备 |

Release 与 Debug 包签名不同，切换安装前请先卸载旧版本。

### 2. 安装与授权

1. 安装与设备架构匹配的 APK。
2. 授予 **所有文件访问权限**，以便写入公共下载目录。
3. 允许 **通知**，用于显示进度与暂停/继续按钮。
4. 允许 **电池优化豁免**，保证长时间后台下载。
5. Android 17 使用局域网配对时，允许 **局域网访问权限**。

默认下载目录为 `/storage/emulated/0/Download/Rayload`。

### 3. 开始下载

- 点击 **+**，粘贴 HTTP、FTP、BitTorrent、磁力链、Metalink 或 ED2K 链接。
- 也可以从浏览器分享链接并选择 Rayload。
- 在通知栏或任务卡片上暂停、继续或删除任务。
- 删除的任务会移动到下载目录内的隐藏 `.trash` 文件夹。

### 4. 浏览器 RPC（可选）

```text
地址：http://127.0.0.1:6800/jsonrpc
密钥：设置 → 高级 → RPC Secret
```

在 Firefox Android 或支持扩展侧载的 Chromium 中安装兼容 Aria2 JSON-RPC
的扩展。标准 Chrome Android 不支持桌面扩展。

### 下一步

- [局域网互联指南](LAN_GUIDE.md)：双设备配对与跨端推任务。
- [插件开发指南](PLUGIN_DEVELOPMENT_GUIDE.md)：编写网盘解析规则。
- [CLI 命令行手册](HEADLESS_CLI.md)：在 NAS 或服务器上无头运行。
- [常见问题](FAQ.md)：权限、速度、配对与解析问题。
