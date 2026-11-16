# Rayload beta7 — Handoff & TODO

Local working document for the `v1.0.4-beta7` cycle. Keep it in sync with the
code; it is the single source of truth for the next session.

## Where we are

- Repository: `/home/qi/桌面/rayload`, branch `main`.
- Last committed state: `16cf2db save: beta6 gesture, settings, engine startup and packaging fixes`.
- Version in `src-tauri/Cargo.toml`: `1.0.4-beta6`.
- Delivered beta6 artifact:
  `/home/qi/桌面/rayload/release/current/v1.0.4/beta6/v1.0.4-beta6-full-arm64.apk`
  (arm64-v8a only, 23,045,035 B, sha256
  `d15accadefc1896f58980feaed5f5ad0b07f94a896f388bf1fa181b47087e870`).
- Test plugins (beta6): `test_plugins/test_safe.json`,
  `test_plugins/test_script.sh`.

### Build facts that must not be rediscovered

- Always build with `--features plugin_engine`; without it `build_info.lan`
  and `build_info.pluginEngine` are false, which hides LAN and plugin entries.
- `useLegacyPackaging` must stay `true` in
  `src-tauri/gen/android/app/build.gradle.kts`. Android 10+ SELinux refuses to
  execute a binary extracted to app data (`avc: denied { execute_no_trans }`),
  and Aria2 is an exec'd sidecar from `nativeLibraryDir`. Setting it to `false`
  makes the engine fail at startup.
- Version source of truth is `src-tauri/Cargo.toml`; bump with
  `./scripts/bump-version.sh <version>`.
- Locale files must be edited through the batch script (`scripts/update_locale`
  / `pnpm` locale tasks), never one by one.
- Reporting: `printf '%s\n' "..." | ./notify_dingtalk.sh` after a finished
  package build.
- For beta7 the user explicitly said **do not start the AVD or run device
  tests**; compile and hand over the APK. Do not push, do not create a release.

### Build command

```bash
export ANDROID_HOME="$HOME/.cache/rayburst-android/android-sdk"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export NDK_HOME="$ANDROID_HOME/ndk/29.0.14206865"
export JAVA_HOME="/usr/lib/jvm/java-21-openjdk-amd64"
pnpm tauri android build --apk --target aarch64 --features plugin_engine
```

Copy the result to
`release/current/v1.0.4/beta7/v1.0.4-beta7-full-arm64.apk`, verify with
`aapt2 dump badging` (single ABI, `versionName='1.0.4-beta7'`) and report size
plus SHA-256 against beta6.

## Test-build history (beta4 → beta6)

### beta4 — UI/UX restructure

- Bottom bar defaults to All / Keep-awake / Settings / About; category dropdown
  replaced by a refresh button.
- Task cards: right-to-left soft gradient, 3px per-device accent rail, compact
  mode with details-only button, swipe actions, wide-screen max-width.
- Task detail: two tabs (file details / checksums), card-based flow, all six
  footer actions kept.
- Settings: vertical groups (account, downloads, appearance, advanced) with
  hint subtitles.
- Plugins: UTF-8 `.sh` import with `# @name:` metadata, `.zip` with
  `plugin.json` + `main.sh`, filename fallback to kill hex garbage.
- Statistics: live-speed chart enlarged to match the seven-day trend chart.

### beta5 — data and layout fixes

- Local + remote task merge fixed so polling cannot drop local tasks.
- Plugin management re-mounted under Advanced & extensions, plugin toggle back
  in navigation customization.
- Settings card flow unified, legacy footer form removed.
- LAN push blocked with a toast when the peer is offline; per-device colour
  picker in the device list.
- Onboarding: Skip jumps to the final consent page, merged agreement page with
  mandatory checkbox, compact/full task-card choice, continuous progress bar.

### beta6 — gestures, engine, packaging

- Swipe gestures hardened (pointer capture, 48px threshold); left = delete,
  right = pause/resume or send to this device.
- Navigation customization gained a plugin switch; enabling LAN devices renders
  its icon immediately.
- Android default directory is `/storage/emulated/0/Download/Rayload`; skipping
  the permission page still requests all-files access.
- Compact detail button is a bare small icon; detail footer is a single
  icon row with safe-area padding; settings rows show a `›` chevron.
- Rust release profile: `strip`, `lto`, `codegen-units = 1`, `opt-level = "z"`;
  Vite uses esbuild minify plus `treeshake: 'smallest'`.
- `jniLibs { useLegacyPackaging = false }` was attempted and reverted: it
  prevents the Aria2 sidecar from executing.
- AVD (x86_64 API 33) smoke test passed: engine process up, onboarding,
  settings, plugin page, swipe delete/pause/resume, no FATAL/ANR.

## beta7 TODO

The user's brief, plus three follow-ups that arrived while beta7 was starting.
Nothing in this list has been implemented yet.

### Module 1 — UI and responsive layout

1. Settings page:
   - Mobile (< 768px): replace the vertical long list with horizontal tabs at
     the top (Account, Downloads, Appearance, Advanced) and show the card flow
     underneath. Files: `src/views/PreferenceView.vue`,
     `src/components/layout/navigation.ts` (`preferenceGroups`),
     `src/styles/preferences.css`.
   - Tablet: keep the two-column layout; cap and center the right pane width.
2. Wide screens: cap and center task-list cards, header statistics tiles and
   charts. `src/components/task/TaskList.vue` already sets `max-width: 1120px`;
   check `HomeStatsBar.vue`, `SpeedTrendCard.vue`, `HistoryTrendCard.vue`.
3. Bottom bar overflow: it wraps into two rows when many icons are enabled.
   Keep one row only — max five items, extras behind "More", or a horizontally
   scrollable row. File: `src/components/layout/AppSidebar.vue`
   (`.sidebar.compact` uses `grid-template-columns: repeat(auto-fit, ...)`).

### Module 2 — interaction and logic

4. Swipe gestures still reported as broken on device. Requirements: left =
   delete, right = pause/resume (local) or send to this device (remote).
   Files: `src/components/task/TaskSwipeRow.vue`,
   `src/components/task/TaskList.vue` (`handleSwipePrimary`).
   Note: AVD smoke test passed, so suspect a real-device-only cause (long-press
   timer vs. list scroll, pointer capture lost when the list re-renders on
   polling, or `touch-action` on a parent).
5. Device colour desync: a colour chosen in settings shows up differently on
   the list and cards. Unify through one source.
   `src/shared/utils/task.ts` (`getTaskAccentColor`), `deviceColors.ts`,
   `LanDevicesView.vue`, card components.
6. ✅ Plain HTTP/HTTPS tasks must hide the "Nodes" and "Tracker" panels in the
   detail page instead of rendering empty states. `TaskDetail.vue`.
   Done in the Windows migration kickoff: both cards now render only when the
   task is not a plain URI download (`!isURI`).
7. Compact remote cards squeeze the filename; reduce the right-side icon set
   and give the name the space. `TaskCompactItem.vue`.

### Module 3 — advanced checksum system

8. Algorithm switches under Settings → Advanced → Checksums: MD5, SHA-1,
   SHA-256, SHA-512, BLAKE3, SM3 (national standard), CRC32. The detail page
   computes only enabled algorithms.
   Existing: `src-tauri/src/commands/*` `FileDigests` command,
   `src/components/task/detail/TaskChecksumPanel.vue`.
   Likely crates: `blake3`, `sm3`, `crc32fast` (check `src-tauri/Cargo.toml`).
9. SQLite cache of computed hashes keyed by mtime + size; unchanged files
   return instantly. Storage lives in `src-tauri/src/database/`.
10. "Auto-verify after download" master switch plus category filter (for
    example only installers and archives).
11. Allow/deny rules (wildcard or regex) to skip verification per URL suffix.
    Default: online lookup only for GitHub sources.
12. Online multi-source lookup for uncached, non-blacklisted files: GitHub API
    first, then CIRCL Hash Lookup and similar public APIs.
13. Checksum UI:
    - "Expected checksum" input with automatic comparison: green tick on match,
      red cross on mismatch.
    - "Cancel calculation" button while running.
    - "Export checksum file" button (`.sha256` in the download directory or via
      system share).
    - Per-file verification for multi-file BT tasks.

### Module 4 — detail and feature completion

14. Onboarding engine page: delete the "default download path" box.
    `src/components/onboarding/AndroidPermissionWelcome.vue`.
15. Statistics quick entries: replace "LAN" and "Download history" with
    "Clear download cache" and "Storage space analysis".
    `src/views/StatisticsView.vue` (check the quick-action cards).
16. Completion follow-up: completed `.apk` gets a one-tap install button in the
    detail page; audio/video files get a preview button.
    Reuse the existing Android bridge (`open_path_normalized`, FileProvider) and
    media helpers in `src-tauri/src/services/media/`.

### Module 5 — test plugins and delivery

17. Generate `test_safe.json` and `test_script.sh` in the repository root (the
    beta6 copies live under `test_plugins/`), print their absolute paths.
18. Build `v1.0.4-beta7-full-arm64.apk`, report paths and send the DingTalk
    report. No AVD, no push, no release.

### Extra requests from the user (must ship in beta7)

A. Completed local tasks also need a right-to-left sharing interaction.
   Interpretation to confirm: right swipe on a completed local task triggers
   share (right swipe currently does nothing for completed tasks), while left
   swipe stays delete.

B. ✅ Tapping a finished file currently offers "Open file" and "Show in folder".
   Remove the "Open file" action and keep only "Show in folder".
   File: `src/composables/useNotificationToast.ts` (completion toast buttons).
   Done in the Windows migration kickoff: the toast, its dependencies and the
   unused `openFileFromNotification` helper were removed together.

C. The "Transfer" card in the task detail page still overflows its box.
   File: `src/components/task/TaskDetail.vue` (`.detail-card`, transfer rows)
   and `src/components/task/detail/TaskDetailActivity.vue`.

## Suggested order

1. Module 2 fixes (gestures, colour, HTTP panels, compact remote cards).
2. Module 1 layout work (settings tabs, width caps, bottom bar).
3. Module 3 checksum system (largest, touches Rust, SQLite, UI, i18n).
4. Module 4 and the extras (A, B, C).
5. Locales, tests, version bump, arm64 build, DingTalk report.
