# Rayload v1.0.4

## 中文

Rayload `v1.0.4` 新增局域网设备互联与多设备数据视图，并拆分出
轻量版、完整版与 F-Droid 版三个构建变体。

### 局域网设备

- 新增双向配对：接收端弹出包含发起方设备名与公钥指纹的确认框，双方确认后
  互相保存；任意一方都能推送任务和查看统计。
- 新增“局域网设备”页面：可设置本机名称、配对账号密码，支持扫码添加、
  扫描局域网与手动输入地址三种配对方式。
- 已配对设备支持在线状态检测、备注、手动移除与凭据保存。
- 新建任务弹窗新增“目标设备”下拉框。选择其他设备后，链接通过局域网
  直接推送到该设备，本机不创建任务；选择“本机”时行为不变。
- 目标为远端设备时隐藏代理、Cookie 等高级选项，并提示在目标设备上配置。

### 多设备任务与统计

- 任务列表同时展示本机与已配对设备的远程任务，远程任务支持暂停、继续
  与删除，设备离线时状态降级显示。
- 数据统计页新增“本机”与“全部设备”两个 Tab。全部设备视图汇总各设备的
  实时下载速度与活动任务数。

### 插件与权限

- 规则插件运行在宿主中介的 JSON 沙箱内：不执行任意代码，所有网络请求都经过
  权限与域名白名单校验；脚本插件仍需显式确认高风险提示。
- 插件清单新增 `permissions` 权限作用域：`network`、`cookies`、
  `modify_headers`、`read_clipboard`；`network` 必须配合 `allowedDomains`
  使用，`cookies` 必须同时声明 `network`。
- 导入插件时按风险分级展示权限：常规权限绿色，Cookie 与剪贴板权限红色
  高亮，必须勾选“我已知晓风险”才能导入；规则插件与脚本插件显示不同的
  第三方风险提示。
- Rust 宿主在执行时再次校验声明与授权交集，逐请求校验域名，强制剥离
  `Cookie`、`Host`、`Content-Length` 等头部；未授予 `cookies` 时的任何
  Cookie 头都会被移除。
- 新增 `plugin.schema.json` 与中英双语 `PLUGIN_DEVELOPMENT_GUIDE.md`，
  包含完整 Schema、POST 提取码示例、`{{password}}` 变量注入与脚本协议说明。
- 内置示例插件改为下载 Rayload 自身安装包，自动匹配当前 ABI 的最新 Release。

### 无界面 CLI

- `ray` / `rayload` 重建为终端下载控制台：YAML 配置、交互式 `ray init`、
  后台守护进程、systemd 安装、树状状态、任务增删暂停、日志跟踪、设备审批
  与 `--json` 自动化输出。
- 新增 `ray add/list/pause/resume/rm/log/plugin list` 与
  `ray -d pending/allow/reject`。
- 简写命令统一为 `ray -s`、`ray -a`、`ray -c`、`ray -d`；`ray -d list` 与
  `ray -s` 使用 ANSI 彩色树状图，在线绿色、离线红色。
- `ray start -d` 写入 `<data-dir>/rayload.log`，`ray log -f` 可实时跟踪；
  `ray install` 注册 systemd 开机自启服务。

### 合规与引导

- 首次启动向导新增《用户使用协议》与《隐私政策》强制勾选，未同意时无法
  完成向导。
- 关于页新增中英双语免责声明；README 顶部新增声明区块。
- 设置重组：下载、BitTorrent、ED2K、网络合并为“下载器”设置，文件分类
  独立成页；导航栏自定义与悬浮球开关迁入向导并附带说明。

### 修复

- 修复局域网设置页与局域网设备页无法滑动的问题。
- 修复多端口设备在局域网列表中被拆分为多条记录的问题，改为按设备指纹聚合。
- 修复文件详情二级页面文件名过长时显示不全的问题。
- 修复全部设备统计缺少历史总量与折线图的问题。
- 修复无界面 CLI 主机无法删除已完成远程任务的问题。
- 修复空密码保存会清空管理账号、取消配对未清理对端状态、轮询失败静默无提示
  的问题。

### 安全加固

- 配对改为一次性凭证交换反向公钥，不再传输管理密码；取消或拒绝会立即作废凭证。
- 局域网任务推送限制在下载根目录内，并记录来源 IP、URL 与任务元数据审计日志。
- 插件请求按声明与授权交集执行，逐请求校验域名，强制剥离 `Cookie`、`Host`、
  `Content-Length` 等请求头；未授权 Cookie 永远不会注入。

### 构建与更新

- 新增 `lite`、`full`、`fdroid` 三个构建变体；局域网功能仅随 `full` 提供。
- 新增 x86_64 发行包，固定使用 `full` 特性集，供 x86 设备与模拟器安装。
- 发行产物统一命名为 `Rayload-v1.0.4-lite-arm64.apk`、
  `Rayload-v1.0.4-full-arm64.apk`、`Rayload-v1.0.4-full-x86_64.apk`、
  `Rayload-v1.0.4-fdroid-arm64.apk` 与
  `rayload-cli-v1.0.4-linux-x86_64.tar.gz`。
- F-Droid 版使用独立包名 `com.shiwushuifeng.rayload.fdroid` 与独立启动
  快捷方式，可与 GitHub 版并存安装。
- F-Droid 版彻底移除 GitHub 自更新：不加载更新插件，更新检查命令直接
  返回“由 F-Droid 客户端提供更新”，设置页仅显示说明文案。

### 构建信息

- 版本：`1.0.4`
- 目标 API：37（Android 17）
- 最低 API：24（Android 7）
- 生产 ABI：`arm64-v8a`、`x86_64`（完整版）
- 发行版：Lite、Full、F-Droid、Linux CLI

## English

Rayload `v1.0.4` adds LAN device pairing and multi-device views, and splits the
Android build into lite, full and F-Droid variants.

### LAN devices

- Mutual pairing: the receiver shows the initiator name and public-key
  fingerprint, both sides confirm, and either device can push tasks or read
  statistics afterwards.
- A new LAN devices page configures the local name and host credentials, with QR
  scanning, subnet discovery and manual IP/password pairing.
- Trusted devices show online status, support notes and can be removed cleanly.
- The new-task dialog gains a target-device selector. Choosing another device
  hands the link off over the LAN without creating a local task; “This device”
  keeps the existing behavior.
- Proxy, Cookie and other advanced options are hidden for remote targets, with
  guidance to configure them on the target device.

### Remote tasks and statistics

- The task list shows remote tasks from paired devices next to local tasks,
  with pause, resume and remove actions and an offline fallback state.
- Statistics gains “This device” and “All devices” tabs, the latter showing
  live speed and active task counts for every paired device.

### Plugins and permissions

- Rule plugins run in a host-mediated JSON sandbox with no arbitrary code
  execution; every network request is permission-checked and domain-limited.
  Script plugins remain available behind an explicit high-risk warning.
- Plugin manifests declare `permissions` scopes: `network`, `cookies`,
  `modify_headers` and `read_clipboard`. `network` requires `allowedDomains`;
  `cookies` requires `network` as well.
- The import dialog grades permissions by risk: normal scopes in green,
  Cookie and clipboard access highlighted in red. High-risk and script
  imports require an explicit "I understand the risk" checkbox.
- The Rust host intersects declared and granted permissions at runtime,
  validates every request host and strips `Cookie`, `Host`,
  `Content-Length`, `Connection`, `Transfer-Encoding` and
  `Proxy-Authorization` headers unconditionally.
- New `plugin.schema.json` and bilingual `PLUGIN_DEVELOPMENT_GUIDE.md` with the
  full Schema, a POST extraction-code example, `{{password}}` variable
  injection and the script plugin protocol.
- The bundled sample plugin now downloads Rayload itself, resolving the latest
  release asset for the current ABI.

### Compliance and onboarding

- The first-run wizard now requires accepting the User Agreement and Privacy
  Policy before it can be completed.
- The About page and README carry a bilingual disclaimer.
- Settings are reorganized: Downloads, BitTorrent, ED2K and Network share one
  Downloader page, file categories have their own page, and navigation
  customization plus the floating-orb switch moved into the wizard with
  per-item descriptions.

### Fixed

- LAN settings and the LAN devices page now scroll correctly.
- Devices reachable on multiple ports are aggregated by fingerprint instead
  of appearing as duplicate entries.
- Long file names wrap correctly on the file-detail screen.
- All-devices statistics now include historical totals and trend charts.
- Headless CLI hosts now remove completed remote tasks instead of failing with
  "active download not found".
- Saving an empty password no longer clears the host account, cancelling a
  pairing cleans up both sides, and polling failures now surface an error.

### Security hardening

- Pairing now uses a one-time ticket and exchanges reverse public keys instead
  of transmitting the admin password; cancel and reject invalidate the ticket.
- LAN handoff is confined to the download root and records source IP, URL and
  task metadata in the audit log.
- Plugin requests obey the declared/granted permission intersection and
  `allowedDomains`; unsafe headers are stripped and ungranted cookies are never
  injected.

### Headless CLI

- Rebuilt `ray` / `rayload` as a terminal download console with YAML config,
  interactive `init`, daemon mode, systemd installation, tree status, task
  control, log tailing, plugin listing, device approval and `--json` output.
- Shorthand commands include `ray -s`, `ray -a`, `ray -c` and `ray -d`.
  `ray -s` and `ray -d list` print ANSI tree output (green online, red offline).
- `ray start -d` writes `<data-dir>/rayload.log`; `ray log -f` follows it, and
  `ray install` registers the systemd service for boot startup.

### Builds and updates

- Separate `lite`, `full` and `fdroid` variants; LAN features ship only in
  `full`.
- Added an x86_64 release artifact that always uses the `full` feature set for
  x86 devices and emulators.
- Artifacts use the final names `Rayload-v1.0.4-lite-arm64.apk`,
  `Rayload-v1.0.4-full-arm64.apk`, `Rayload-v1.0.4-full-x86_64.apk`,
  `Rayload-v1.0.4-fdroid-arm64.apk` and
  `rayload-cli-v1.0.4-linux-x86_64.tar.gz`.
- The F-Droid variant uses the `com.shiwushuifeng.rayload.fdroid` application ID
  and its own launcher shortcuts, so it can be installed alongside the GitHub
  build.
- F-Droid builds ship no GitHub self-update path: the updater plugin is not
  loaded and update commands report that the F-Droid client owns upgrades.

### Build information

- Version: `1.0.4`
- Target API: 37 (Android 17)
- Minimum API: 24 (Android 7)
- Production ABIs: `arm64-v8a`, `x86_64` (full edition)
- Editions: Lite, Full, F-Droid and Linux CLI

---

# Rayload v1.0.3

## 中文

Rayload `v1.0.3` 是 Android 正式稳定版。本版本完成核心下载与任务管理、
系统交互与防错机制、数据看板与 UI 动效三轮开发，并补齐真机回归中发现的
细节问题。

### 下载与任务管理

- 新建任务支持一键粘贴、清空，多行链接自动拆分为多个独立任务。
- 高级防盗链新增 `Cookie`、`User-Agent`、`Referer`，安全拼装为 Aria2
  `header` 数组，特殊字符不会破坏解析。
- 长按任务卡片进入多选，支持全选、批量暂停/继续/彻底删除（二级确认）。
- 任务卡片支持左滑删除、右滑暂停，并调用 Android 原生马达提供轻微触觉反馈。
- 批量粘贴链接时预检重复任务，过滤列表内重复与已存在任务，并提示
  “已跳过 X 个重复任务”。
- 完善磁力链接与种子文件解析流程，文件选择列表与授权信息展示更稳定。
- 新增存储空间预警，空间不足时在建任务前给出明确提示。
- 文件校验支持 `MD5`、`SHA1`、`SHA256` 异步计算，界面显示
  “正在后台计算...”并提供复制按钮。

### 系统交互与防错

- 首次启动向导提供可跳过的多页引导、个性化与引擎选项、自动归档开关、
  全部分类图标和语言修改后的重启提醒，并修复底部安全区遮挡。
- 权限中心集中管理存储、通知、电池优化、未知应用安装与剪贴板权限，
  全部走 Android 原生权限申请。
- 通知设置迁入权限中心，新增下载开始/完成/失败提醒、通知中隐藏文件名、
  后台保活常驻通知；关闭保活后切换为原生极简静默通道，下载仍继续。
- 桌面长按图标提供“新建下载”静态快捷方式。
- App 内通过 GitHub Release 检查更新，支持仅检查正式版；关于页提供
  “立即检查”，网络或接口异常统一为中文提示。
- 自动归档支持自定义文件夹名称，例如将默认 `Video` 改为“影视”。
- 设置支持配置导出/导入 JSON，以及通过系统分享面板导出诊断日志。
- 局域网互联面板展示 Wi-Fi、热点或以太网私网 IPv4、RPC 6800、密钥与
  直连二维码，并排除虚拟网卡地址。

### 数据看板

- 主页顶部展示总下载大小、平均速率与当前会话下载量；平均速率只累计有
  下载活动的时间，按“本次会话已下载字节 ÷ 活动秒数”每 2 秒刷新。
- 数据统计页提供实时网速、总下载、总任务、当前会话下载四张核心卡片。
- 近 7 天下载趋势基于本地历史实时聚合；实时网速趋势每 2 秒采样、滚动
  保留 3 分钟，并显示平均值参考线与放大的坐标轴文字。
- 分类统计包含视频、音频、图片、文档、安装包、压缩包与其他。
- “全部”页标题旁小箭头可进入数据统计；统计页默认不占用底部导航栏。
- 当前会话数据通过 Aria2 RPC 获取，历史统计骨架已预留本地持久化。

### 界面与动效

- 下载速度、进度百分比使用平滑数字滚动动画。
- 多选模式顶部标题栏颜色与文字平滑渐变；左滑删除卡片带阻尼感弹性消失。
- 列表项、底部导航与按钮统一 Material 水波纹反馈。
- 任务列表、回收站、局域网互联补充极简 SVG 空状态插画与优化文案。
- 全站可点击热区不小于 44×44px，并修复暗色模式下文字与图标对比度问题。
- 设置底栏移除“…”菜单及“重启引擎”入口，只保留保存/放弃。
- 关于页版本号与平台合并展示；悬浮速度球更换初始位置，在统计页和设置页
  自动隐藏，并在外观设置中提供显示开关。
- 新建任务长表单保持紧凑，仅展开“高级选项”后滚动；认证信息输入框完整换行。

### 构建信息

- 版本：`1.0.3`
- 目标 API：37（Android 17）
- 最低 API：24（Android 7）
- 生产架构：`arm64-v8a`
- 应用包名：`com.shiwushuifeng.rayload`

| 附件 | 文件大小 | SHA-256 |
| --- | --- | --- |
| `Rayload-v1.0.3-android17-arm64-release.apk` | 22,457,507 B | `d1604489b16602940461613fdcbf1d460b02d3cc55d2a201316a94b83878cab9` |
| `Rayload-v1.0.3-android17-arm64-debug.apk` | 120,378,194 B | `07b1faab074faafa4db998501e12b22d6213377518cca4156339b41d4b5fc115` |
| `Rayload-v1.0.3-x86_64-emulator.apk` | 22,385,829 B | `77daf51bd45a3570f31acf81f648c05dbe75a7df019bc9541249fc165c70dc31` |

- 校验文件：`SHA256SUMS-v1.0.3.txt`
- 验证：前端 1189 项测试、TypeScript、ESLint、Prettier、仓库完整性检查、
  Rust 通知/配置测试及 Android 模拟器冒烟测试全部通过。

Release 与 Debug APK 使用不同签名；切换安装前请先卸载另一版本。

## English

Rayload `v1.0.3` is the stable Android release that completes the download
engine and task-management work, system integration and guardrails, the
statistics dashboard, and the UI motion pass, together with fixes from device
regression testing.

### Downloads and tasks

- The new-task dialog adds one-tap paste and clear, and splits multi-line input
  into independent tasks.
- Advanced anti-leech fields `Cookie`, `User-Agent` and `Referer` are safely
  mapped to the Aria2 `header` array, including special characters.
- Long-press multi-select supports select-all plus batch pause, resume and
  permanent delete with a second confirmation.
- Task cards support swipe-to-delete and swipe-to-pause with native Android
  haptic feedback.
- Batch input filters duplicates before creating tasks and reports
  “Skipped X duplicate tasks”.
- Magnet and torrent parsing now handles file selection lists and
  authentication fields reliably.
- Disk-space checks warn before a download is likely to fail.
- File verification computes `MD5`, `SHA1` and `SHA256` asynchronously, shows a
  “calculating in background” notice, and offers copy actions.

### System integration and guardrails

- A skippable multi-page onboarding flow covers personalization, engine choice,
  auto-archive, all category icons and a restart reminder after language
  changes, with safe-area padding.
- The permission center manages storage, notifications, battery optimization,
  unknown-app installation and clipboard access through native Android prompts.
- Notification settings moved into the permission center and now include
  start/complete/error alerts, filename privacy and a background keep-alive
  notification with a native minimal silent channel.
- A static “New download” launcher shortcut is available on long-press.
- GitHub release checks support stable-only mode, with an About-page action and
  friendly Chinese network error messages.
- Auto-archive folder names are customizable.
- Configuration can be exported or imported as JSON, and diagnostic logs can be
  shared through the system share sheet.
- The LAN panel shows private Wi-Fi, hotspot or Ethernet IPv4, RPC port 6800,
  the secret and a direct-connect QR code while excluding virtual interfaces.

### Statistics dashboard

- The home overview shows total downloaded size, average speed and current
  session volume; average speed counts only active download time and refreshes
  every 2 seconds.
- The statistics page presents live speed, total download, total tasks and
  session download cards.
- The 7-day trend aggregates local history; the live speed trend samples every
  2 seconds over a rolling 3-minute window with an average reference line and
  larger axis labels.
- Category statistics cover video, audio, images, documents, packages, archives
  and other files.
- An arrow beside the “All” title opens statistics, which stays out of the
  bottom navigation by default.

### UI and motion

- Animated number rolling for speed and progress values.
- Smooth title-bar transition in multi-select mode and damped swipe-out cards.
- Material ripple feedback across lists, navigation and buttons.
- Minimal SVG empty states for tasks, trash and LAN devices.
- Minimum 44×44px touch targets and dark-mode contrast fixes.
- The settings footer no longer contains the overflow menu or engine restart.
- The About page combines version and platform badges; the floating speedometer
  has a new initial position, hides on statistics and settings pages, and can
  be disabled in Appearance settings.
- Long add-task forms stay compact and scroll only after Advanced options are
  expanded.

### Build information

- Version: `1.0.3`
- Target API: 37 (Android 17)
- Minimum API: 24 (Android 7)
- Production ABI: `arm64-v8a`
- Application ID: `com.shiwushuifeng.rayload`
- Artifacts: ARM64 release APK, ARM64 debug APK, x86_64 emulator APK and
  `SHA256SUMS-v1.0.3.txt`.
- Verification: 1189 frontend tests, TypeScript, ESLint, Prettier, repository
  integrity checks, Rust notification/config tests and Android emulator smoke
  tests passed.

Release and debug APKs use different signing identities; uninstall the other
variant before switching.

---

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
