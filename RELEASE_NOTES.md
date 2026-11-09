# Rayload v1.0.1-beta1

## 中文

Rayload `v1.0.1` 启用全新的 35 度倾斜双向箭头品牌图标：黑底、淡紫细线框，
配合顶部渐变实心箭头和更轻盈的几何层次。本次更新同时加入首次启动语言选择，
并为全部 27 种受支持语言提供 README 语言入口。

### 更新内容

- 修复首次启动语言下拉框无法展开的问题。
- 修复空任务页面 Logo 不显示的问题。
- 修复 Android 任务详情面板过窄和长路径挤压的问题。
- 修复文件分享误报“文件不存在”的问题，统一通过 FileProvider URI 分享。
- 修复权限引导在点击“稍后”后仍于冷启动重复出现的问题。
- 修复系统 SAF“使用此文件夹”回调未送达前端的问题。
- 全量更新 Android、Windows、macOS、iOS 和 Tauri 图标资源。
- 侧边栏、悬浮窗、关于页、空状态和加载状态统一使用新 Logo。
- 首次启动时可直接选择界面语言，并在选择后立即应用。
- Release 介绍提供中英双语内容。
- GitHub README 顶部可通过语言选择器切换全部 27 种语言版本。

### 安装包

```text
Rayload-v1.0.1-android17-arm64-release.apk
```

- 目标：Android 17 / API 37
- 最低：Android 7.0 / API 24
- 架构：`arm64-v8a`
- SHA-256：`ca8624b5b9aed3f9b5cbf5d7ea648fa348db0cda1ea5f58b32941ad15b87168d`

Release APK 使用 Rayload 独立签名。升级前请确认设备上安装的是同一签名版本。

## English

Rayload `v1.0.1` introduces a new slanted double-arrow identity: a black
background, fine lavender outlines, a compact gradient-filled accent, and a
lighter geometric composition. This release also adds language selection to
first launch and provides README entry points for all 27 supported languages.

### What's new

- Fixed the first-launch language picker not opening.
- Fixed the missing empty-state logo.
- Expanded the Android task-detail drawer and improved long-path wrapping.
- Fixed false missing-file errors when sharing through the native share sheet.
- Persisted the permission-guide dismissal flag across cold starts.
- Moved SAF directory selection to the Android Activity Result API.
- Regenerated all Android, Windows, macOS, iOS, and Tauri icon assets.
- Applied the new logo across the sidebar, floating windows, About page, empty
  states, and loading presentation.
- Added language selection to first launch with immediate application.
- Added bilingual Chinese and English release notes.
- Added a GitHub language picker covering all 27 supported README editions.

### Package

```text
Rayload-v1.0.1-android17-arm64-release.apk
```

- Target: Android 17 / API 37
- Minimum: Android 7.0 / API 24
- ABI: `arm64-v8a`
- SHA-256: `ca8624b5b9aed3f9b5cbf5d7ea648fa348db0cda1ea5f58b32941ad15b87168d`

The Release APK uses the independent Rayload signing identity. Verify that the
installed build uses the same signing certificate before upgrading.

Rayload is an independent, unofficial port based on Rayburst by AnInsomniacy and
follows the original MIT license.
