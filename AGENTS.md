# AGENTS.md — Rayload

This file provides working instructions for AI coding agents. Rayload is an
independent Android port based on Rayburst by AnInsomniacy. Preserve the
upstream MIT attribution and avoid presenting Rayload as an official Rayburst
release.

## Product boundaries

- Product name: **Rayload**
- Android package: `com.shiwushuifeng.rayload`
- Target API: 37 (Android 17)
- Minimum API: 24 (Android 7)
- Supported production ABI: `arm64-v8a`
- Work on the current branch. Do not spawn agents unless the user explicitly
  requests them.

Keep compatibility identifiers such as the Rust crate name, deep-link schemes,
native-messaging IDs, and Android namespace unless a change explicitly requires
an identity migration. User-visible product copy must use Rayload.

## Architecture

| Layer           | Stack                                              |
| --------------- | -------------------------------------------------- |
| Frontend        | Vue 3 Composition API, Pinia, Naive UI, TypeScript |
| Backend         | Rust, Tauri 2, native Android bridge               |
| Engine          | Aria2 Next ARM64 sidecar                           |
| Storage         | Native SQLite owner in `src-tauri/src/database/`   |
| Tests           | Vitest and Cargo tests                             |
| Package manager | pnpm, pinned by `packageManager`                   |

Important paths:

```text
src/                         Vue frontend
src-tauri/src/               Rust backend and native services
src-tauri/gen/android/       Customized Android Studio project
src-tauri/locales/           Native JSON translations
src/shared/locales/          Frontend JSON translations
scripts/android/             Android Kotlin source templates
```

Do not run `tauri android init` over the committed Android project. It would
replace Rayload's share target, foreground service, DocumentsUI bridge, and
Kotlin entry point.

## Quality rules

- All implementation comments and documentation are in English.
- TypeScript strict mode is required. Do not introduce `any`; justify every
  non-trivial `as` cast.
- Return structured Rust errors through `AppError`.
- Use semantic Material Design 3 CSS tokens instead of fixed colors.
- Extract reusable composables and utilities instead of duplicating logic.
- Use mature libraries where they reduce maintenance cost.
- Preserve the Rayload brand colors, logo geometry, safe areas, and 44px minimum
  touch targets.

## Configuration

When adding an app configuration key:

1. Add the typed field to `src/shared/types.ts`.
2. Add engine mappings to `src/shared/aria2Options.json` when applicable.
3. Add the default value to `src/shared/constants.ts`.
4. Validate or hydrate complex values in
   `src/shared/utils/configHydration.ts`.
5. Bind the field through the relevant preference composable and view.
6. Update all supported locale files with the batch script.

Do not add migrations for other products or historical Rayburst storage.

## Localization

- English is the schema and fallback locale.
- Do not edit locale files one by one; use a batch script.
- Preserve nested namespaces, placeholders, and existing translations.
- Product names remain untranslated.
- Run `pnpm check:repo` after locale changes.

## Verification

Run checks proportional to the changed surface:

```bash
pnpm lint
pnpm format:check
pnpm check:repo
pnpm test
npx vue-tsc --noEmit
cargo test --manifest-path src-tauri/Cargo.toml --workspace --all-targets
```

Use `pnpm tauri android build --apk --target aarch64` when validating an APK.
Do not use browser-based UI tests for Android-native IPC, sharing, DocumentsUI,
foreground services, or WebView behavior. Final device acceptance belongs to
the user.

## Releases and repository hygiene

- `src-tauri/Cargo.toml` is the application version source of truth.
- Use `./scripts/bump-version.sh <version>` for version changes.
- Never commit keystores, signing passwords, local SDK paths, build output, or
  generated APK files.
- Keep the README Android-first and free of upstream badges, sponsorship links,
  promotional animations, and unrelated desktop release material.
- Preserve the MIT license and a concise attribution to the original Rayburst
  author in README and in-app notices.
- Local verification never publishes to a store or starts release automation.
