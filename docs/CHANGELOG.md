# Changelog

All notable changes to the independent Rayload Android port are documented in
this file.

## Unreleased

### Added

- Added a standalone `SPONSOR.md` page, a minimal README sponsor link, and a
  Star History chart.
- Added GPL-3.0 licensing, a `NOTICE` file with Rayburst and aria2-next
  attributions, issue templates, and a contribution guide.
- Added a tag-driven GitHub Actions workflow that builds the ARM64 APK and
  publishes it to GitHub Releases.

### Changed

- Converted the sponsor QR image to `docs/assets/sponsor.webp` and removed the
  original JPEG.
- Normalized license and sponsor links across all translated README editions.
- Documented the v0.9.0-beta through v1.0.1 milestones in `ROADMAP.md`.

## v1.0.4 - 2026-11-14

### Added

- Mutual LAN device pairing: both sides confirm the peer and fingerprint, store
  each other, and can push tasks or read statistics in either direction.
- QR-code pairing, subnet scanning and manual IP/password pairing, plus device
  names, per-device notes and an explicit incoming pairing-request dialog.
- Saved host credentials, a dedicated device-management page and online/offline
  state for every paired peer.
- Download handoff to any paired online device from the new-task dialog.
- Remote task list showing paired-device downloads alongside local tasks, with
  pause, resume and remove actions.
- This-device and all-devices tabs on the statistics page, including per-peer
  live speed and active-task counts.
- Separate `lite`, `full` and `fdroid` build variants.
- An `x86_64` release APK that always ships the `full` feature set.
- Declarative JSON plugin sandbox with scoped permissions (`network`, `cookies`,
  `modify_headers`, `read_clipboard`), risk-graded import review, per-request
  domain allow-list checks, runtime header stripping, a public
  `plugin.schema.json` and a bilingual development guide.
- Templated plugin variables (`{{password}}` and other caller-supplied values)
  with scoped cookie injection and no credential logging.
- First-run User Agreement and Privacy Policy consent, About and README
  disclaimers, and differentiated third-party plugin import warnings.
- Historical all-device statistics, sparklines, merged Downloader settings and
  in-wizard navigation customization with per-item descriptions.
- Rebuilt `ray`/`rayload` CLI: YAML configuration, interactive `ray init`,
  foreground and daemon startup, systemd installation, tree status, task
  control (`add`, `list`, `pause`, `resume`, `rm`), log tailing, plugin listing,
  device approval and `--json` automation output.
- `rayload-cli security-check`, a startup credential fail-safe, download-root
  containment for handoff requests, source-IP/URL handoff audit logging and a
  non-root Docker image.

### Changed

- The F-Droid variant uses the `com.shiwushuifeng.rayload.fdroid` application ID
  and its own launcher shortcuts.
- Version bumped to `1.0.4`.
- Release artifacts renamed to `Rayload-v1.0.4-{lite,full}-arm64.apk`,
  `Rayload-v1.0.4-full-x86_64.apk`, `Rayload-v1.0.4-fdroid-arm64.apk` and
  `rayload-cli-v1.0.4-linux-x86_64.tar.gz`.
- README restructured into a documentation hub with bilingual navigation;
  `QUICK_START.md`, `LAN_GUIDE.md` and `FAQ.md` were added and the plugin guide
  moved under `docs/`.

### Fixed

- F-Droid builds no longer execute GitHub self-update checks or ship the updater
  plugin; updates are left to the F-Droid client.
- The headless CLI host now removes completed remote tasks instead of
  returning an "active download not found" error.
- LAN host credentials and the device name survive restarts again, and saving
  the host form with an empty password no longer clears the account.
- Unpairing revokes the peer on both sides, and cancelling a pairing request
  invalidates its one-time ticket, so a rejected device can no longer push
  tasks.
- Sending a download to another device hides the advanced options instead of
  dropping them silently, and pairing or polling failures now report an error.
- Completed remote tasks can be removed by the CLI host without an
  "active download not found" error.

### Security

- Pairing uses a one-time ticket and exchanges reverse public keys instead of
  transmitting the admin password; cancelling or rejecting invalidates the ticket.
- The receiver always shows the initiator name and key fingerprint before trust.
- Plugin requests are checked against the declared/granted permission
  intersection and `allowedDomains`; `Cookie`, `Host`, `Content-Length`,
  `Connection`, `Transfer-Encoding` and `Proxy-Authorization` are stripped.
- LAN handoff rejects paths outside the download root and records source IP,
  URL and task metadata in the audit log.

## v1.0.3 - 2026-11-12

### Added

- One-tap paste/clear, multi-line task splitting and advanced anti-leech
  `Cookie`, `User-Agent` and `Referer` handling in the new-task dialog.
- Long-press multi-select with select-all and batch pause, resume and permanent
  delete, plus swipe gestures and native haptic feedback.
- Duplicate-link filtering, storage-space warnings and asynchronous
  `MD5`/`SHA1`/`SHA256` file verification.
- Skippable onboarding, centralized permissions, notification controls,
  launcher shortcut, GitHub update checks, JSON config transfer and LAN panel.
- Statistics dashboard with home overview cards, 7-day trends, live-speed
  sampling, category totals and Aria2 session data.
- Animated numbers, multi-select title transitions, damped swipe-out cards,
  Material ripples and SVG empty states.
- ARM64 release/debug and x86_64 emulator build artifacts with SHA256SUMS.

### Changed

- Auto-archive folder names are customizable.
- The settings footer no longer contains the overflow menu or engine restart.
- The floating speedometer moved to a lower-left initial position, hides on
  statistics/settings pages, and can be disabled in Appearance settings.
- The About page combines version/platform badges and adds update checks.
- Notification settings moved from General into the permission center.

### Fixed

- Duplicate-task dialog layering, onboarding safe areas, auto-archive icons and
  detail-page error-banner overlap.
- Friendly update-check and notification errors, statistics chart labels,
  live-trend sampling and activity-based average speed.
- LAN IP detection excluding virtual interfaces, settings tab-strip jumping,
  dark-mode contrast and undersized touch targets.

See `RELEASE_NOTES.md` for the complete bilingual release notes and artifact
checksums.

## v1.0.2 - 2026-11-11

### Changed

- Replaced the paged task list with seamless infinite scrolling and split task
  card metadata from the action row.
- Rebuilt the floating speedometer around a centered live-status panel with
  bulk pause/resume, collapsible limits, engine restart, and edge docking.
- Added the keep-awake dashboard with theme-aware colors, light/dark switching,
  reduced-motion support, and native screen-on.
- Added keep-awake to the customizable bottom navigation and localized the
  foreground notification with pause/resume actions.
- Simplified the torrent upload dialog, added select-all, and made the file
  list scroll independently.
- Removed the unsupported post-download program setting and wired Android
  shutdown and wake-lock behavior to native APIs.
- Restored static gold launcher icons and reduced the release APK size with
  Rust stripping and frontend minification.

### Fixed

- Prevented task card metadata from overlapping the action bar.
- Fixed dialog and toast placement above the page header, exposed settings
  scrollbars, and input fill gaps on mobile.
- Clamped the expanded speedometer capsule inside the viewport after restoring
  its saved position.
- Reported the generated BT listen port after randomization.

## v1.0.1-beta2 - 2026-11-09

### Changed

- Removed the obsolete open-location button from task cards.
- Replaced the visible task drag rail with long-press reordering on touch
  screens and centered task-scope navigation labels.
- Added an Android Trash panel with restore and permanent-delete actions.
- Rebuilt the Android task toolbar around the page title, sort, and a bottom
  action sheet; task creation now uses a right-side floating action button.

### Fixed

- Kept long task names readable in cards and preserved mobile page headings
  when the task toolbar wraps.

## v1.0.1-beta1 - 2026-11-09

### Fixed

- Replaced the first-launch language dropdown with a WebView-safe native
  selector.
- Restored the empty-state logo artwork.
- Expanded the Android task-detail drawer and prevented task names and path text
  from collapsing into unreadable vertical columns.
- Kept mobile task-page headings visible when the action toolbar wraps.
- Routed Android file sharing through readable task-file candidates and
  FileProvider content URIs without a premature Rust path check.
- Persisted permission-guide completion in localStorage so dismissed battery
  guidance does not reappear after a cold start.
- Switched SAF folder selection to the Activity Result API so DocumentsUI
  selections return reliably to the WebView bridge.

## v1.0.1 - 2026-11-09

### Changed

- Replaced the application icon and in-app brand artwork with the final 35-degree
  counterclockwise double-arrow logo.
- Regenerated Android, Windows, macOS, iOS, and Tauri icon assets from the
  1024×1024 source artwork.
- Updated the application version to `1.0.1`.
- Updated release notes in Simplified Chinese and English.
- Added selectable README editions for all 27 supported interface languages.

### Added

- Added interface language selection to the first-launch setup flow.

## v1.0.0 - 2026-11-09

### Added

- Rebranded the Android application as Rayload with package name
  `com.shiwushuifeng.rayload`, version `1.0.0`, and a dedicated signing
  identity.
- Rebuilt the About page with Rayload attribution, developer links, and MIT
  attribution to AnInsomniacy.
- Added a first-launch permission flow for storage, notifications, and battery
  optimization exemption.
- Added native directory selection and typed Android WebView bridge access.
- Added DocumentsUI folder navigation, FileProvider sharing, and file opening.
- Added foreground download service, completion notifications, haptics, share
  target support, and partial wake-lock handling.
- Added localized Rayload Android guidance across all 27 supported locales.

### Changed

- Moved add-task type tabs below the modal header with icon-over-label controls.
- Locked the Android JSON-RPC port to `6800`.
- Changed “shut down after downloads” to exit Rayload and stop its engine.
- Changed deletion to move files and metadata into a hidden `.trash` folder.
- Removed the desktop-only post-download executable option on Android.
- Enabled R8, resource shrinking, Rust symbol stripping, LTO, and size-focused
  release optimization.

### Fixed

- Fixed intermittent false “file deleted” errors from sharing downloaded files.
- Fixed the public-download folder actions and native file-path handling.
- Removed desktop-only integrations from the Android runtime and preferences.
- Corrected mobile modal masking, tab overlap, stacking, and touch layouts.

### Verified

- Frontend: 90 test files and 1,174 tests passed.
- Rust: 339 tests passed with 5 intentionally ignored.
- Native Messaging: 8 tests passed.
- Debug and Release APKs were built and verified for API 24/37, ARM64, package
  identity, version, signatures, and release shrinking.
