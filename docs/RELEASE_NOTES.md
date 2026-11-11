# Rayload v1.0.2

## 中文

Rayload `v1.0.2` 是 Android 正式版本，集中完成任务列表、悬浮速度球、
不息屏挂机、通知控制与设置体验的收尾修复。

### 任务列表与卡片

- 移除底部分页器，任务列表改为无缝无限滚动。
- 重排完整模式卡片：信息行与操作按钮行严格分离，不再重叠。
- 完整模式显示进度条；紧凑模式隐藏进度条，仅保留单行高度与百分比。
- 空状态增加“点击右下角 + 号新建任务，或从浏览器分享链接至 Rayload”引导。

### 悬浮速度球

- 单击弹出居中实时状态面板，悬浮球在面板打开时隐藏。
- 面板提供全部暂停、全部恢复、实时速度、任务数、网络状态与连接质量。
- 限速设置默认折叠；下载完成后关闭软件、重启引擎、应用按钮移出折叠区，常显。
- 支持贴边停靠为速度胶囊，修复恢复位置时超出屏幕被裁切的问题。
- 保留拖拽、吸附动画与主题色适配。

### 不息屏挂机

- 新增全屏挂机界面：主题色 Logo、呼吸光效、实时总下载速度与任务数。
- 挂机入口加入底部导航自定义，默认开启；外观跟随主题色与深浅模式。
- 挂机页右上角提供深浅外观切换；开启“减弱动态效果”时停用呼吸动画。
- 进入时申请原生屏幕常亮，退出时恢复默认。

### Android 原生与设置

- 常驻前台通知中文化，并补齐可用的暂停/继续控制按钮。
- 桌面图标回退为静态米金色图标，移除残缺的自适应图标配置。
- “下载完成后关闭软件”接入原生进程退出；“下载时保持唤醒”接入唤醒锁。
- 删除 Android 无法使用的“下载完成后运行程序”设置及其后端残留。
- 底部导航默认仅显示全部、挂机、设置、回收站与关于。
- 修复文本弹窗/Toast 遮挡顶部信息栏、设置页滚动条外露与输入框内透明带。
- BT 监听端口随机生成后立即回显并给出提示。

### 构建信息

- 版本：`1.0.2`
- 目标 API：37（Android 17）
- 最低 API：24（Android 7）
- 架构：`arm64-v8a`
- 附件：Release APK、Debug APK、`SHA256SUMS`
- Release SHA-256：`fc766310e10fbf2b6a996a15eede367cdb11404c1f9cc9b94c55fac8eb188db3`
- 验证：前端 1174 项测试通过；TypeScript、ESLint、Prettier 与仓库完整性检查通过。

## English

Rayload `v1.0.2` is the Android release that finishes the task list, floating
speedometer, keep-awake dashboard, notification controls, and settings polish.

### Task list and cards

- Removed the bottom pager; the task list now uses seamless infinite scroll.
- Reworked full-mode cards so metadata and action buttons occupy separate rows.
- Full mode shows a progress bar; compact mode keeps a single row with percent.
- Added clearer empty-state guidance for creating tasks or sharing links.

### Floating speedometer

- A centered live-status panel opens on tap while the capsule hides.
- Includes bulk pause/resume, live speeds, task counts, network status, and
  connection quality.
- Speed limits start collapsed; shutdown-when-complete, engine restart, and
  apply actions stay outside the collapsible section.
- Edge docking and position clamping prevent the expanded capsule from being
  clipped off-screen.

### Keep-awake mode

- Full-screen dashboard with a themed logo, breathing glow, live download speed,
  and active task count.
- Added to the customizable bottom navigation (enabled by default); follows the
  selected theme color and light/dark appearance.
- Built-in light/dark toggle; breathing animation respects reduced motion.
- Uses native screen-on while active and restores the default on exit.

### Android and settings

- Localized the persistent foreground notification and added working
  pause/resume actions.
- Restored a static gold launcher icon and removed the broken adaptive icon XML.
- Wired shutdown-when-complete to native process exit and keep-awake to a
  partial wake lock.
- Removed the unsupported desktop-only post-download program option.
- Bottom navigation now defaults to All, Keep Awake, Settings, Trash, and About.
- Fixed toast/dialog placement under the header, hidden scrollbars, and input
  fill gaps; randomized BT ports now report the generated value.

### Build

- Version: `1.0.2`
- Target API: 37 (Android 17)
- Minimum API: 24 (Android 7)
- ABI: `arm64-v8a`
- Assets: Release APK, Debug APK, `SHA256SUMS`
- Release SHA-256: `fc766310e10fbf2b6a996a15eede367cdb11404c1f9cc9b94c55fac8eb188db3`
- Verification: 1174 frontend tests passed; TypeScript, ESLint, Prettier, and
  repository integrity checks passed.

---

# Rayload v1.0.1-beta2

## 中文

Rayload `v1.0.1-beta2` 汇总了 beta1 的语言选择、品牌图标和 Android 原生修复，
并继续重构移动端任务体验、精简任务卡片、补上可恢复的文件回收站。

### 更新内容

#### Beta 2 移动端改进

- 移除任务卡片中已废弃的“打开文件位置”按钮。
- 移除任务卡片左侧的竖条拖拽把手，手机端改为长按任务卡片排序。
- 将“全部”“进行中”“失败”“已完成”等任务分类文字居中。
- 新增侧边栏“回收站”入口，可查看、恢复或彻底删除移入 `.trash` 的文件。
- 重构手机任务页顶部：只保留标题、排序和“更多操作”，全局操作收纳到底部动作面板。
- 将“新建任务”改为右下角悬浮按钮，减少拥挤和误触。
- 修复长任务名在卡片中竖排，以及手机页面标题被工具栏挤没的问题。

#### Beta 1 更新

- 修复首次启动语言下拉框无法展开的问题。
- 修复空任务页面 Logo 不显示的问题。
- 修复 Android 任务详情面板过窄和长路径挤压的问题。
- 修复文件分享误报“文件不存在”的问题，统一通过 FileProvider URI 分享。
- 修复权限引导在点击“稍后”后仍于冷启动重复出现的问题。
- 修复系统 SAF“使用此文件夹”回调未送达前端的问题。
- 全量更新 Android、Windows、macOS、iOS 和 Tauri 图标资源。
- 侧边栏、悬浮窗、关于页、空状态和加载状态统一使用全新的 35 度倾斜双向箭头 Logo。
- 首次启动时可直接选择界面语言，并在选择后立即应用。
- Release 介绍提供中英双语内容。
- GitHub README 顶部可通过语言选择器切换全部 27 种语言版本。

### 安装包

```text
Rayload-v1.0.2-android17-arm64-release.apk
```

- 目标：Android 17 / API 37
- 最低：Android 7.0 / API 24
- 架构：`arm64-v8a`
- SHA-256：`d23e60ad6039ea2bee5a5bf38899da5be85432ca3085af5948d0066c093f428c`

Release APK 使用 Rayload 独立签名。升级前请确认设备上安装的是同一签名版本。

## English

Rayload `v1.0.1-beta2` includes all beta 1 language, branding, and Android-native
fixes while continuing to rebuild the mobile task experience, simplify task
cards, and add a recoverable file Trash.

### What's new

#### Beta 2 mobile improvements

- Removed the obsolete open-location button from task cards.
- Removed the visible drag rail and enabled long-press reordering on phones.
- Centered task-scope navigation labels.
- Added a sidebar Trash panel for restoring or permanently deleting `.trash`
  entries.
- Rebuilt the mobile task header around the title, sort, and a bottom action
  sheet, and moved New Task to a floating action button.
- Fixed clipped mobile headings and vertically collapsed long task names.

#### Included from beta 1

- Fixed the first-launch language picker not opening.
- Fixed the missing empty-state logo.
- Expanded the Android task-detail drawer and fixed narrow path wrapping.
- Fixed false missing-file errors when sharing through the native share sheet.
- Persisted the permission-guide dismissal flag across cold starts.
- Moved SAF directory selection to the Android Activity Result API.
- Regenerated all Android, Windows, macOS, iOS, and Tauri icon assets.
- Applied the new 35-degree slanted double-arrow logo across the sidebar,
  floating window, About page, empty states, and loading presentation.
- Added language selection to first launch with immediate application.
- Added bilingual Chinese and English release notes.
- Added a GitHub language picker covering all 27 supported README editions.

### Package

```text
Rayload-v1.0.2-android17-arm64-release.apk
```

- Target: Android 17 / API 37
- Minimum: Android 7.0 / API 24
- ABI: `arm64-v8a`
- SHA-256: `d23e60ad6039ea2bee5a5bf38899da5be85432ca3085af5948d0066c093f428c`

The Release APK uses the independent Rayload signing identity. Verify that the
installed build uses the same signing certificate before upgrading.

Rayload is an independent, unofficial port based on Rayburst by AnInsomniacy and
follows the original MIT license.
