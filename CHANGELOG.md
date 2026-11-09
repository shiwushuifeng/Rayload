# Changelog

All notable changes to the independent Rayload Android port are documented in
this file.

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
