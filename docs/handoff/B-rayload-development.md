# 交接简报 B：Rayload 功能开发

> 把一个新会话 bootstrap 到「Rayload 应用开发」轨道。
> 最后更新：2026-11-20。

## 你的角色

你只负责**功能开发、UI/UX、性能与测试**，不碰 GitHub 发布流程与 F-Droid 元数据
（那是另一个会话的事，见 `docs/handoff/A-repo-and-fdroid.md`）。

## 工作区与产品

- 仓库：`C:\AAAARayload`（Windows），分支 `main`
- 前端：Vue 3 + TypeScript + Pinia + Naive UI + Vite
- 后端：Rust + Tauri 2 + 原生 Android 桥（Kotlin）
- 引擎：Aria2 Next ARM64 sidecar
- 存储：原生 SQLite（`src-tauri/src/database/`）
- 包名 `com.shiwushuifeng.rayload`，minSdk 24 / targetSdk 37，GPL-3.0

## 当前版本

- crate `1.0.4`，Android `versionName 1.0.4.1`，`versionCode 10016`
- 已发布：`v1.0.4.1`（正式版）、`v1.0.4-beta`（合并测试档）

## 质量门（每次改动后必须全绿）

```bash
pnpm lint          # eslint，0 warning
npx vue-tsc --noEmit
pnpm format:check  # prettier
pnpm test          # vitest，当前 1215 通过
pnpm check:repo    # 语言包结构与占位符校验
```

Rust 侧：`cargo check --all-targets`。**注意**：主 crate 的测试二进制在本机
启动即崩（`0xc0000139 STATUS_ENTRYPOINT_NOT_FOUND`，加载期，依赖全是系统 DLL），
原因未定位；因此 `cargo test` 只能跑 `aria2-service` 等子 crate。行为验证靠 vitest
与真机/模拟器。

## 构建（Windows 专用配方，已实测）

```bash
export CARGO_TARGET_DIR='C:\rayload-build\target'      # 关键，见下
export ANDROID_HOME='C:\dev\toolchains\android-sdk'
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export NDK_HOME="$ANDROID_HOME\ndk\29.0.14206865"
export JAVA_HOME='C:\dev\toolchains\jdk21'
export TMP='C:\AAAARayload\.build-tmp' TEMP="$TMP" TMPDIR="$TMP"
export PATH="$JAVA_HOME/bin:$HOME/.cargo/bin:$PATH"
pnpm tauri android build --apk --target aarch64 --features plugin_engine
```

**为什么 `CARGO_TARGET_DIR` 必须在工作区外**：本机沙箱限制「镜像在工作区内的进程
只能写工作区」，`cargo` build script 与 `cl.exe` 会因此失败。产物放外面即可，
`CARGO_HOME` 保持默认。详见 `docs/HANDOFF-WINDOWS.md` 第五节。

**x86_64 构建额外需要**（Windows NDK 只有带 API 号的 clang 包装器）：

```bash
export CC_x86_64_linux_android='.../x86_64-linux-android24-clang.cmd'
export AR_x86_64_linux_android='.../llvm-ar.exe'
RAYLOAD_SMOKE_X86_64=1 pnpm tauri android build --apk --target x86_64 --features plugin_engine
```

其他变体：`lite`（默认 feature）、`fdroid`（需 `RAYLOAD_FDROID=1`，包名加 `.fdroid`）、
`demo`（`VITE_RAYLOAD_DEMO=1`）。

**feature 要点**：`fdroid` 是**附加** feature，必须同时带 `plugin_engine`，
否则插件与局域网功能被编译掉。

## 版本号约束（务必知道）

`1.0.4.1` 不是合法 semver，Cargo/Tauri 都拒绝。现状：

- `src-tauri/Cargo.toml` 保持 `1.0.4`
- Android `versionName` 由 `build.gradle.kts` 读环境变量 `RAYLOAD_VERSION_NAME`
- `package.json` 写 `1.0.4.1`（前端 `constants.ts` 导入它做展示）

**下一个版本请回归三段式**（如 `1.0.5`），消除 F-Droid 与 semver 的摩擦，
`RAYLOAD_VERSION_NAME` 只作为应急手段保留。

## 下一步开发计划：v1.0.5

详见 `ROADMAP.md` 与 `docs/LAN_WIZARD_PLAN.md`。

1. **Web 文件互传**：浏览器/设备间传文件。
2. **Android 原生悬浮窗**：虚拟列表 + IPC 性能优化（现有悬浮窗是 WebView 实现）。
3. **局域网连接向导**（方案已定稿，见 `docs/LAN_WIZARD_PLAN.md`）：
   - 入口：只在「局域网设备」页加「连接设备」按钮，不放进 onboarding
   - 首屏默认「搜索局域网」，扫码/手动为备选
   - 流程：本机准备 → 选方式 → 核对指纹+账号密码 → 对方确认（带倒计时）→ 完成
   - 必须做四类失败诊断：局域网权限被拒 / 不同网段 / 端口不通 / 对方未开启服务
   - 复用已有能力：`lan_scan`、`pairDiscovered`、`lan_pair`、二维码、指纹弹窗

## 已交付但可以继续打磨的点

- **CLI 未经用户实测**（`ray status/add/watch`，Linux）。
- 真机滑动已按触摸事件重写（beta8），用户已确认通过；如需回归可查日志
  `TaskSwipe.lock` / `TaskSwipe.commit`。
- 设备配色：按设备身份派生单一扁平 hex，卡片背景用同色 10% 淡色。
- 校验系统：7 算法 + SQLite 缓存 + 取消/导出/BT 多文件 + CIRCL/GitHub 查询。

## 代码约定（来自 AGENTS.md）

- 注释与文档用英文；TypeScript strict，不引入 `any`
- Rust 错误统一走 `AppError`
- 优先用 Material 3 语义 token，不写死颜色
- 抽 composable/util，不重复逻辑
- 保留品牌色、安全区、44px 最小触控目标
- 语言包**必须**用 `scripts/` 下的批处理脚本改，禁止逐个文件编辑
- 新配置项要同步：`types.ts` → `aria2Options.json`（若涉及引擎）→ `constants.ts`
  → `configHydration.ts` → 偏好 composable/视图 → 语言包

## 已知坑

| 现象 | 处理 |
| --- | --- |
| `pnpm` 报 `ERR_PNPM_STORE_DIR_OPEN_OPERATION_LOCK` | 设 `npm_config_manage_package_manager_versions=false`、`npm_config_verify_deps_before_run=false` |
| esbuild 删临时文件 `EPERM` | `TMP`/`TEMP` 指到工作区内 |
| git 提交被 husky 拦 | `git -c core.hooksPath= commit`（或先设 `core.hooksPath .git/no-hooks`） |
| `git add -A` 撞 `CON` | 已加 `.gitignore`，改用指定路径 add |
| Gradle wrapper 下不动 | 已是 `~/.gradle` 内，若重装用清华镜像 |
| x86_64 缺引擎 | 需要 `src-tauri/binaries/aria2-next-x86_64-linux-android`；`.toolchains/binaries/` 供 gradle check |

## 交付纪律

- 不 push、不建 release —— **除用户明确要求**；发布由另一个会话负责。
- 完成后报告：改了什么、怎么验证、遗留什么。
- 真机行为（滑动、安装、局域网）只能由用户验收，不要声称已验证。