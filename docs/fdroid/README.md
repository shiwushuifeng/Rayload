# F-Droid submission / F-Droid 提交说明

`com.shiwushuifeng.rayload.fdroid.yml` is the recipe submitted to fdroiddata
and the canonical copy kept here. The build sets `RAYLOAD_FDROID=1`, so the
resulting applicationId is `com.shiwushuifeng.rayload.fdroid`; fdroiddata
requires the metadata filename to match that applicationId, which is why the
file is already named this way.

`com.shiwushuifeng.rayload.fdroid.yml` 是提交给 fdroiddata 的配方，也是本仓库
保留的权威副本。构建时设置 `RAYLOAD_FDROID=1`，因此实际 applicationId 为
`com.shiwushuifeng.rayload.fdroid`。fdroiddata 要求元数据文件名与
applicationId 一致，所以文件已按此命名，提交时无需重命名。

## Current submission / 当前提交

| Item | Value |
| --- | --- |
| Merge request | [!58176](https://gitlab.com/fdroid/fdroiddata/-/merge_requests/58176) |
| RFP | [fdroid/rfp#4797](https://gitlab.com/fdroid/rfp/-/issues/4797) |
| Fork / branch | `shiwushuifeng/fdroiddata`, `rayload-1.0.4` |

## Engine is built from source / 引擎从源码构建

The inclusion policy only allows prebuilt binaries from authorized sources, so
the Aria2 Next sidecar is compiled during the build instead of downloaded:

- `vendor/aria2-next` is a git submodule pinned to upstream `v3.1.1`.
- The recipe sets `submodules: true`, so fdroiddata checks it out.
- `scripts/android/build-engine.sh` drives the Android cross-compile target
  upstream already provides (`NDK=$$NDK$$ API=24`) and installs the result as
  `src-tauri/binaries/aria2-next-aarch64-linux-android`.
- The submodule vendors its whole dependency stack in `third_party/`, so it
  builds without network access. `cmake`, `ninja-build`, `make`, `perl` and a
  C++ toolchain are required on the build host.

本地同样可以构建：`NDK=/path/to/ndk scripts/android/build-engine.sh`。

## Version handling / 版本处理

- `versionName` and `versionCode` come from `src-tauri/tauri.android.conf.json`,
  which is committed.
- `UpdateCheckData` points at that file. Without it, fdroiddata parses
  `build.gradle.kts`, which reads both values from the generated
  `tauri.properties` that is never committed, so `fdroid checkupdates` would
  find no version at any tag.
- Keep `CurrentVersionCode` in sync with `tauri.android.versionCode` on every
  release. Four-part versions such as `1.0.4.1` are not valid semver, so the
  crate stays at `1.0.4` and the installer version is injected separately;
  prefer three-part release versions going forward.

每次发版都要让 `CurrentVersionCode` 与 `tauri.android.versionCode` 保持一致。
四段式版本号（如 `1.0.4.1`）不是合法 semver，crate 保持 `1.0.4`，安装包版本号
另行注入；后续版本建议回归三段式。

## Updating the submission / 更新提交

1. Edit `docs/fdroid/com.shiwushuifeng.rayload.fdroid.yml` here, then copy it
   into the fdroiddata branch `rayload-1.0.4` at
   `metadata/com.shiwushuifeng.rayload.fdroid.yml`.
2. `commit` must be the full 40-character hash of a commit that contains the
   engine submodule; a tag name is not accepted.
3. Optionally verify locally with `fdroid lint` and
   `fdroid build com.shiwushuifeng.rayload.fdroid:<versionCode>`.

## Notes / 注意事项

- `fastlane/metadata/android/en-US/` in this repository provides the store
  listing, icon, feature graphic and screenshots.
- The F-Droid APK is signed by F-Droid, not by the GitHub release key. Users
  switching from a GitHub build must uninstall it first if the package ID matches.
- `MANAGE_EXTERNAL_STORAGE` is requested only when the user picks a download
  directory, and `REQUEST_INSTALL_PACKAGES` only hands a downloaded `.apk` to the
  system installer; both are explained to reviewers in the merge request.