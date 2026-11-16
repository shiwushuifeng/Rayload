# Migration Notes — Linux → Windows

Snapshot taken 2026-11-16 on Ubuntu. Source tree: `/home/qi/桌面/rayload`
(branch `main`, commit `40594eb`, working tree clean). App version
`1.0.4-beta6` from `src-tauri/Cargo.toml`. This file is the entry point for a
new agent picking the project up on Windows; deep detail lives in `AGENTS.md`,
`docs/PROJECT_LAYOUT.md`, `docs/BETA7_TODO.md` and `docs/HANDOFF.md`.

## Stack

| Layer      | Stack                                                              |
| ---------- | ------------------------------------------------------------------ |
| Frontend   | Vue 3 Composition API, Pinia, Naive UI, TypeScript (strict)        |
| Backend    | Rust, Tauri 2, native Android bridge (Kotlin)                      |
| Engine     | Aria2 Next ARM64 sidecar (`src-tauri/binaries/`)                   |
| Storage    | SQLite owned by Rust in `src-tauri/src/database/`                  |
| Packaging  | pnpm (pinned), Gradle, NDK 29, JDK 21, target API 37, min API 24   |
| Platform   | Android first; desktop Windows config in `src-tauri/tauri.windows.conf.json` |

## Layout conventions

- Source paths required by toolchains stay at the repo root (`src/`,
  `src-tauri/`, `package.json`, `index.html`).
- `src-tauri/gen/android/` is a committed, customized Android Studio project.
  Never run `tauri android init` over it — it would destroy the share target,
  foreground service, DocumentsUI bridge and Kotlin entry point.
- Kotlin sources live in `scripts/android/*.kt` and are copied into
  `src-tauri/gen/android/app/src/main/java/com/shiwushuifeng/rayload/`.
- `release/` holds APK/CLI artifacts and is Git-ignored; `local/` and
  `docs/HANDOFF.md` are local-only.
- Locales: English is the schema and fallback. `src/shared/locales/` (frontend)
  and `src-tauri/locales/` (native) are edited only through the batch scripts in
  `scripts/`, never file by file.

## Common commands

```bash
pnpm install
pnpm dev                 # vite dev server
pnpm lint                # eslint src/ --max-warnings=0
pnpm format:check        # prettier
pnpm test                # vitest run
pnpm check:repo          # repository integrity checks
npx vue-tsc --noEmit
cargo test --manifest-path src-tauri/Cargo.toml --workspace --all-targets
./scripts/bump-version.sh <version>
```

Android APK build (Linux paths must be replaced on Windows, see below):

```bash
export ANDROID_HOME="$HOME/.cache/rayburst-android/android-sdk"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export NDK_HOME="$ANDROID_HOME/ndk/29.0.14206865"
export JAVA_HOME="/usr/lib/jvm/java-21-openjdk-amd64"
pnpm tauri android build --apk --target aarch64 --features plugin_engine
```

## Decisions already made (do not rediscover)

- Always build with `--features plugin_engine`; without it LAN and plugin
  entries are hidden because `build_info.lan` / `build_info.pluginEngine` are false.
- `useLegacyPackaging` stays `true` in
  `src-tauri/gen/android/app/build.gradle.kts`; Aria2 is exec'd from
  `nativeLibraryDir` and Android 10+ SELinux blocks execution from app data.
- Version source of truth is `src-tauri/Cargo.toml`; bump only via the script.
- Primary ABI `arm64-v8a`; `x86_64` only for full/F-Droid builds.
- Product identity is Rayload (`com.shiwushuifeng.rayload`); compatibility
  identifiers (crate name `rayburst`, deep-link schemes, native-messaging IDs)
  stay unchanged.
- Phase completion and finished package builds are reported with
  `printf '%s\n' "..." | ./notify_dingtalk.sh`.

## Open items

- `docs/BETA7_TODO.md` is the live task list for the beta7 cycle: swipe
  gestures on real devices, device colour desync, HTTP/HTTPS detail panels,
  compact remote cards, settings width caps, bottom-bar overflow, and the
  advanced checksum system (extra algorithms, SQLite hash cache, auto-verify
  rules, online lookup, export/cancel UI), plus onboarding and statistics
  quick-entry changes.
- `ROADMAP.md`: v1.0.5 web transfer + native floating window, v1.0.7
  Rayload-Video, v1.0.8 theme store, v1.0.9 design-system pass, v1.1.0 plugin
  marketplace, v1.1.1 AI assistant. CI automation and ABI splits are still open.
- Unfinished by design from this migration: no device/AVD tests were run, and
  the beta7 APK was not rebuilt.

## Windows differences

- Project path: unpack to `C:\dev\rayload` (ASCII, no spaces); avoid `桌面` and
  other non-ASCII directories, Gradle/NDK handle them poorly.
- Reinstall per-machine state: `pnpm install`, `cargo fetch`; `node_modules/`,
  `src-tauri/target/`, `.toolchains/`, `dist/` and Gradle `build/` output were
  not packaged because they are machine- and OS-specific.
- Toolchain env vars: `ANDROID_HOME`, `ANDROID_SDK_ROOT`, `NDK_HOME`,
  `JAVA_HOME` must point at Windows installs; use `gradlew.bat` and
  `.\scripts\...` instead of `./scripts/...`.
- `notify_dingtalk.sh` in the repo root is a symlink to `/home/qi/notify_dingtalk.sh`
  and is not portable: keep a real copy with `curl` available, or skip reporting.
- Bash-based helpers (`scripts/bump-version.sh`, `scripts/install-cli.sh`,
  `scripts/update-android-*.py`, `scripts/android/build-*.sh`,
  `emulator-smoke.sh`) need Git Bash or WSL; the `.mjs` helpers run natively
  with Node.
- `src-tauri/gen/android/local.properties` and any recorded `sdk.dir` contain
  Linux paths; regenerate them locally and never commit them.
- Shell scripts may have been checked out with LF; keep `core.autocrlf=input`
  so `.sh` files stay LF, or run them under WSL.
- `~/.codex` becomes `%USERPROFILE%\.codex`; `config.toml` has
  `[projects."/home/qi/桌面/rayload"]` trust entries keyed by absolute path —
  re-trust the new Windows path on first launch.
- MCP servers and any absolute binary paths in the restored Codex config must be
  reinstalled for Windows (Node/Python/uv executables differ).
- Signing keystores (`signing/*.jks`, `keystore.properties`) and the
  DingTalk webhook were deliberately excluded from the migration package; copy
  them out-of-band if release signing is required on the new machine.
