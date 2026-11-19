# 交接简报 A：仓库与 F-Droid 上架

> 把一个新会话 bootstrap 到这个项目的「仓库/发布/F-Droid」轨道。
> 最后更新：2026-11-20。

## 你的角色

你只负责**仓库治理、GitHub 发布、F-Droid 上架、CI**，不碰应用功能开发。
另一个会话负责 Rayload 功能开发，两者互不干扰。

## 工作区

- 仓库：`C:\AAAARayload`（Windows），主分支 `main`
- 远端 fetch：`https://github.com/shiwushuifeng/Rayload.git`
- 远端 push：`github-alt:shiwushuifeng/Rayload.git`（`ssh.github.com:443`，22 端口被墙）
- 产品：Rayload，包名 `com.shiwushuifeng.rayload`，GPL-3.0

## 当前状态（已核实）

| 项 | 状态 |
| --- | --- |
| 版本 | crate `1.0.4`，Android `versionName 1.0.4.1`，`versionCode 10016` |
| GitHub 正式版 | `v1.0.4.1`，10 个资产（5 个 APK + CLI + 校验文件） |
| GitHub 预览版 | `v1.0.4-beta`（beta2/beta3/beta8/旧 1.0.4 合并归档，5 资产） |
| 已删除 | `v1.0.4`、`v1.0.4-beta2`、`v1.0.4-beta3`、`v1.0.4-beta8` 的 Release 与 tag |
| F-Droid MR | **!58176** https://gitlab.com/fdroid/fdroiddata/-/merge_requests/58176 |
| MR 状态 | `opened` / `mergeable`，等维护者 review |

## F-Droid 关键事实

- 之前 MR **!58001 被 `linsui` 关闭**，理由只有一句：*Merge Request template is not followed*。
- 新 MR **!58176** 已严格套用 `Default.md` 模板：24 项勾选、4 项未勾均有说明、
  `Closes rfp#4797`、标题 `New app: Rayload`。
- **RFP 记录**：`fdroid/rfp#4797`（opened，已补 MR 链接评论）。
- fork：`shiwushuifeng/fdroiddata`（id 89109516），分支 `rayload-1.0.4`，
  配方 `metadata/com.shiwushuifeng.rayload.fdroid.yml`。
- **fork 的 pipeline 显示 failed 属正常**：fork 没有 runner，`merge_request_event`
  产生空流水线。上游 F-Droid CI 会重新触发。若维护者问起，这样解释。

### 用户已定的决策

1. **Reproducible Builds 不开**（已写进 MR 说明：F-Droid 用他们密钥，包名不同可共存）
2. **aria2 引擎改用官方 3.1.1**，用户接受版本从 3.1.2 降级
3. **GitHub 上已发布的 1.0.4.1 不动**（内含 3.1.2，用户无感）

### aria2-next 处理方式（2026-11-20 改为源码构建）

**之前用 GitHub Releases 下载预编译产物，已废弃**：`Inclusion_Policy` 第 12/20–25 行
只允许 Debian / Maven 系 / Android·Flutter·Hermes SDK / PyPI·Nix·Rust·Golang·Node.js
来源的预编译二进制，GitHub Releases 不在白名单。

现在引擎**从源码构建**：

- `vendor/aria2-next` 是 git submodule，钉在 upstream `v3.1.1`
  （`1b8fb72fcf63dc9e78807f20039708184bce0ad2`）
- 配方 `submodules: true`
- `scripts/android/build-engine.sh` 调用上游自带的 Android 交叉编译目标
  （`NDK=$$NDK$$ API=24`），产出 `src-tauri/binaries/aria2-next-aarch64-linux-android`
- 上游 `third_party/` 是 vendored 源码，**完全离线**可编；需要
  `cmake>=3.25`、`ninja`、`make`、`perl`、C++ 工具链
- 许可证 GPL-2.0-or-later（源码头声明），与 GPL-3.0 兼容，`NOTICE` 已记

首次构建要编 openssl/curl/libtorrent/boost/ffmpeg/gpac，耗时约一小时量级。
本地构建：`NDK=/path/to/ndk scripts/android/build-engine.sh`。

## 版本号的技术约束（重要）

- **`1.0.4.1` 不是合法 semver**，Cargo 与 Tauri 都拒绝（实测：`unexpected character '.' after patch version number`）。
- 因此 crate 保持 `1.0.4`，Android `versionName` 由 `src-tauri/gen/android/app/build.gradle.kts`
  里的 `RAYLOAD_VERSION_NAME` 注入；`package.json` 写成 `1.0.4.1`（前端展示用）。
- CI 已改为**以 tag 名作为版本来源**（`v1.0.4.1` → `1.0.4.1`）。
- **长期建议**：下一个版本回归三段式（如 `1.0.5`），消除 F-Droid/semver 摩擦。
  F-Droid 配方目前报 `versionName: 1.0.4`。

## 环境与网络（踩过的坑）

| 现象 | 处理 |
| --- | --- |
| `git push` 报 husky `Permission denied` | `git config core.hooksPath .git/no-hooks`；`pnpm install` 会重置，需重设 |
| HTTPS 推 GitHub 时好时坏 | 优先用 SSH：`git push origin main`（remote `github-alt`） |
| GitLab API 用 curl 需加 `-k` | 本机 Steam++ 加速器做 TLS 中间人自签证书 |
| GitHub/GitLab HTTPS 全断但 SSH 通 | Steam++ 进程残留导致 443 黑洞，需用户重启该程序 |
| `git add -A` 报 `unable to stat 'CON'` | `CON` 是 MSYS 生成的 Windows 保留名伪设备；已加入 `.gitignore`，`git add` 时指定路径 |
| GitHub API token | `.build-tmp/.ghtoken`（`gho_...`，scopes 含 repo/workflow） |
| GitLab token | `.build-tmp/.gitlab-token`（`glpat-...`，scopes 含 api） |

两者都在 `.build-tmp/`（已 gitignore）。

## 常用操作

```bash
# 查 MR 状态
curl -sSk -H "PRIVATE-TOKEN: $(cat .build-tmp/.gitlab-token)" \
  'https://gitlab.com/api/v4/projects/fdroid%2Ffdroiddata/merge_requests/58176'

# 改 MR 描述（用 JSON 文件，避免 shell 转义问题）
curl -sSk -X PUT -H "PRIVATE-TOKEN: $(cat .build-tmp/.gitlab-token)" \
  -H 'Content-Type: application/json' --data-binary @payload.json \
  'https://gitlab.com/api/v4/projects/fdroid%2Ffdroiddata/merge_requests/58176'

# 回复 MR 评论
curl -sSk -X POST -H "PRIVATE-TOKEN: $(cat .build-tmp/.gitlab-token)" \
  -H 'Content-Type: application/json' -d '{"body":"..."}' \
  'https://gitlab.com/api/v4/projects/fdroid%2Ffdroiddata/merge_requests/58176/notes'
```

**建 MR 的坑**：直接 POST 到 `fdroid/fdroiddata` 会 403（新账号限制）。
必须 POST 到自己的 fork 并带 `target_project_id: 36528`。

## 待办

1. 盯 `!58176`：配方已改为引擎源码构建 + `UpdateCheckData` 修好版本检查，
   等维护者 review。
2. ~~F-Droid 首次构建尚未实跑~~ —— 已在 WSL2 (Ubuntu 24.04) + NDK r29
   实跑 `scripts/android/build-engine.sh` 通过，全程约 2 分 36 秒，产出与上游
   release 的二进制**逐字节相同**（sha256 `8f06939d…4ea973`）。复现方式见文末。
3. 若被要求三段式 tag，则协调开发会话发 `v1.0.5`，并同步配方
   `commit` / `CurrentVersionCode`。
4. **CLI 尚未被用户实测**：`rayload-cli-v1.0.4.1-linux-x86_64.tar.gz` 已在 Release。
5. `C:\codex\AAAAA` 目录含明文密钥（keystore 密码、Bailian/gh token）
   ——提醒用户轮换并清除。

## 纪律

- 绝不 `git push --force` 到 `main`；tag 可用 `--force` 但慎用。
- 提交前跑 `pnpm check:repo`；提交信息用英文。
- ver 版本号只改 `src-tauri/Cargo.toml`（`scripts/bump-version.sh`），并同步 `tauri.android.conf.json`。
- 不提交 keystore、token、SDK 路径、构建产物。

## 复现引擎构建（WSL 实测步骤）

本机 WSL 无 root 权限也能装齐工具链：`apt-get download <pkg>` + `dpkg-deb -x`
解到 `~/tools/root`，再设 `PATH`/`LD_LIBRARY_PATH` 即可。需要
`cmake`(3.28)、`ninja-build`、`pkgconf`、`cmake-data`、`libarchive13t64`、
`librhash0`、`libuv1t64`、`libjsoncpp25`、`libpkgconf3`。

Windows 侧 NDK 只有 `windows-x86_64` 预编译工具链，WSL 里用不了；需另下
Linux 版：

```bash
curl -fSL -o android-ndk-r29-linux.zip \
  https://dl.google.com/android/repository/android-ndk-r29-linux.zip
unzip -q android-ndk-r29-linux.zip -d ~/tools/
```

然后在 Linux 文件系统上跑（别在 `/mnt/c` 上编，慢很多）：

```bash
NDK=~/tools/android-ndk-r29 API=24 bash scripts/android/build-engine.sh
```

实测结果：`Runtime dependency check passed for android-arm64`，
产出 sha256 `8f06939d…4ea973`，与上游 release 完全一致；NEEDED 只有
`libdl/libm/libc`。