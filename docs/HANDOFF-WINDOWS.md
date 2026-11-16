# Rayload — Windows 开发交接（beta7 起点）

> 本文件由 2026-11-16 的迁移会话生成，随工程放在 `C:\AAAARayload`。
> 目标：在 Windows 上继续 Rayload 开发，从 `v1.0.4-beta7` 开始。

## 一、当前状态（从 43 条 Codex 开发对话 + 仓库文档归纳）

### 已完成并交付

| 版本 | 内容 |
| --- | --- |
| v1.0.0 | 更名 Rayload，独立包名 `com.shiwushuifeng.rayload`，首次启动权限引导、DocumentsUI、FileProvider、前台服务、27 种语言 |
| v1.0.1 | 35° 双箭头品牌 Logo、启动语言选择、27 种 README、修复系统分享与 SAF 目录选择 |
| v1.0.2 | 视觉重构：长列表、两行卡片、挂机模式、悬浮球贴边、移除无效设置 |
| v1.0.3 | 多选批量操作、哈希校验、数据统计看板、权限中心、新手引导完善 |
| v1.0.4 | 双向局域网配对与远程任务、JSON 插件权限沙箱、`ray` CLI（提前交付 v1.0.6）、Lite/Full/F-Droid 版本、x86_64 支持 |
| beta4 | 底栏精简为 All/Keep awake/Settings/About + 导航自定义页；任务卡渐变与 3px 设备色条、紧凑模式、滑动操作；详情页双 Tab；设置纵向分组；插件 `.sh`/`.zip` 导入；统计图表放大 |
| beta5 | 本地+远程任务合并修复、插件管理归入高级、LAN 离线 toast、每设备配色、引导页与协议页合并 |
| beta6 | 滑动加固（pointer capture、48px 阈值）、导航自定义插件开关、Android 默认目录 `/storage/emulated/0/Download/Rayload`、Rust release 配置（strip/lto/codegen-units=1/opt-level=z）、AVD x86_64 API33 冒烟通过；产物 `v1.0.4-beta6-full-arm64.apk`（23,045,035 B） |

### 未完成：beta7（用户明确要求全部做完再打包，**尚未开始**）

完整清单见 `docs/BETA7_TODO.md`，这里只列主线：

1. **模块 2（先做，用户排的第一优先级）**
   - 真机滑动仍然失效：左滑删除、右滑暂停/继续（本地）或推回本机（远程）。AVD 通过 → 疑真机特有（长按计时器与列表滚动冲突、轮询重渲染丢失 pointer capture、父级 `touch-action`）。
   - 设备配色不同步：设置里选的颜色与列表/卡片显示不一致，需统一到单一来源。
   - 纯 HTTP/HTTPS 任务详情页应隐藏「节点」「Tracker」面板，而不是渲染空状态。
   - 紧凑远程卡片挤压文件名，减少右侧图标占位。
2. **模块 1（布局）**
   - 设置页移动端（<768px）改为顶部横向 Tab + 卡片流；平板保持两列并限制右栏宽度。
   - 宽屏下任务卡、统计磁贴、趋势图统一限宽居中。
   - 底栏图标过多会换行，改为单行（最多 5 个 + More 或横向滚动）。
3. **模块 3（高级校验系统，最大块）**
   - 算法开关：MD5/SHA-1/SHA-256/SHA-512/BLAKE3/SM3/CRC32。
   - 按 mtime+size 的 SQLite 哈希缓存。
   - 下载后自动校验总开关 + 分类过滤。
   - 允许/拒绝规则（通配或正则），默认只对 GitHub 源做在线查询。
   - 在线多源查询（GitHub API → CIRCL Hash Lookup）。
   - 校验 UI：期望值比对（对勾/叉）、取消计算、导出 `.sha256`、BT 多文件逐个校验。
4. **模块 4**
   - 引导页引擎页删除「默认下载路径」输入框。
   - 统计快捷入口换成「清理下载缓存」「存储空间分析」。
   - 完成后 `.apk` 一键安装、音视频预览按钮。
5. **模块 5（交付）**
   - 生成根目录 `test_safe.json`、`test_script.sh` 并打印绝对路径。
   - 构建 `v1.0.4-beta7-full-arm64.apk`，报告路径，发钉钉。
   - 不跑 AVD、不 push、不建 release（这是 beta7 的原始约定，可再确认）。
6. **附加需求 A/B/C**
   - A：已完成的本地任务右滑分享（左滑仍是删除）。
   - B：完成提示里去掉「打开文件」，只留「在文件夹中显示」。
   - C：详情页「传输」卡片仍溢出。

### 规划（ROADMAP）

- v1.0.5：Web 文件互传 + Android 原生悬浮窗（虚拟列表、IPC 性能优化）。
- v1.0.7：Rayload-Video（内置 ffmpeg 解析网页视频）。
- v1.0.8：主题商店（自定义皮肤与配色）。
- v1.0.9：视觉大统一（全局设计系统规范落地）。
- v1.1.0：插件生态（插件市场、规则分享、签名校验；沙箱已随 v1.0.4 交付）。
- v1.1.1：AI 智能助手（自然语言搜索资源并安全推送到引擎）。
- 工程化：GitHub Actions 打 Tag 自动编译签名发布、WebView 启动性能、IPC 序列化瓶颈、ABI Splits、内存泄漏监控。

## 二、这台机器上的环境

| 项 | 路径 / 版本 |
| --- | --- |
| 工程 | `C:\AAAARayload`（`main` @ `40594eb`，remote `https://github.com/shiwushuifeng/Rayload.git`） |
| 前端依赖 | 已 `pnpm install`（pnpm 11.5.2 / Node 26.13） |
| Rust | stable 1.100.0；已加 `aarch64-linux-android`、`x86_64-linux-android`、`armv7-linux-androideabi`、`i686-linux-android` |
| Android SDK | `C:\dev\toolchains\android-sdk`（platform-tools 37.0.1、platforms;android-37.0、build-tools;37.0.0、ndk;29.0.14206865） |
| JDK | 21 正在装；本机已有 Java 26（Gradle 9.6.1 需要 21） |
| `local.properties` | 已指向 `C:\dev\toolchains\android-sdk` |

构建前建议设置：

```powershell
$env:ANDROID_HOME       = 'C:\dev\toolchains\android-sdk'
$env:ANDROID_SDK_ROOT   = $env:ANDROID_HOME
$env:NDK_HOME           = "$env:ANDROID_HOME\ndk\29.0.14206865"
$env:JAVA_HOME          = 'C:\dev\toolchains\jdk21'
```

## 三、必须守住的工程纪律

- 构建必须带 `--features plugin_engine`，否则 LAN 与插件入口被隐藏。
- `jniLibs { useLegacyPackaging = true }` 不能改回 false：Aria2 sidecar 从 `nativeLibraryDir` 执行，Android 10+ SELinux 会拒绝。
- 版本号唯一来源 `src-tauri/Cargo.toml`，只通过 `scripts/bump-version.sh` 修改，并同步 `src-tauri/tauri.android.conf.json`。
- 语言包只能走 `scripts/` 里的批处理脚本，禁止逐个文件改。
- 绝不跑 `tauri android init` 覆盖已提交的 `src-tauri/gen/android`。
- 不提交 keystore / 签名密码 / 本地 SDK 路径 / 构建产物。
- 保持 MIT 归属与 Rayload 产品身份；兼容标识（crate 名 rayburst、深链 scheme、native-messaging ID）不动。

## 四、建议的第一步

从模块 2 开始（用户排序即如此），先做能独立验证的三件事：

1. 纯 HTTP/HTTPS 任务隐藏「节点」「Tracker」（`TaskDetail.vue`）。
2. 紧凑远程卡片给文件名让位（`TaskCompactItem.vue`）。
3. 完成提示去掉「打开文件」（`src/composables/useNotificationToast.ts`）。

每步跑 `pnpm lint`、`npx vue-tsc --noEmit`，最后统一跑 `pnpm test` 与 `pnpm check:repo`。
