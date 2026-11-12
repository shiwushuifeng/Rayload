# Contributing to Rayload

Thanks for your interest in Rayload. Rayload is an independent Android port
based on Rayburst. Contributions are welcome, provided they keep the app
private by default, Android-first, and consistent with the existing design.

## Project scope

- Rayload targets Android 7.0 (API 24) and newer, with `arm64-v8a` as the only
  production ABI.
- The frontend is Vue 3, Pinia, Naive UI, and TypeScript. The backend is Rust
  with Tauri 2 and a native Android bridge. Aria2 Next is the bundled engine.
- Do not reintroduce desktop-only settings or integrations.
- User-visible product copy must use the name **Rayload**.
- Do not run `tauri android init` over `src-tauri/gen/android`. The committed
  Android project contains the share target, foreground service, DocumentsUI
  bridge, and Kotlin entry point.

## Development setup

Prerequisites:

- Node.js 22 or newer
- pnpm `11.5.2`, matching `packageManager`
- Rust stable with the `aarch64-linux-android` target
- JDK 21
- Android SDK Platform 37, Build Tools 37.0.0
- Android NDK `29.0.14206865`

```bash
pnpm install
pnpm android:prepare
pnpm dev
```

Build an APK with:

```bash
pnpm tauri android build --apk --target aarch64
```

Debug signing uses the Android debug certificate. Release signing is read from
the local `signing/keystore.properties` file, which is never committed.

## Before opening a pull request

Run the checks that cover your change:

```bash
pnpm lint
pnpm format:check
pnpm check:repo
pnpm test
npx vue-tsc --noEmit
cargo test --manifest-path src-tauri/Cargo.toml --workspace --all-targets
```

For Android-native IPC, sharing, DocumentsUI, foreground services, or WebView
behavior, describe the device test you performed. Browser-based UI tests are
not a substitute for Android device verification.

## Coding guidelines

- Keep TypeScript in strict mode. Do not introduce `any`; justify every
  non-trivial `as` cast.
- Return structured Rust errors through `AppError`.
- Use semantic Material Design 3 CSS tokens instead of fixed colors.
- Extract reusable composables and utilities instead of duplicating logic.
- Prefer mature libraries when they reduce maintenance cost.
- Preserve Rayload brand colors, logo geometry, safe areas, and 44 px minimum
  touch targets.
- Write comments and documentation in English.
- Keep changes focused. Do not fix unrelated bugs in the same pull request.

## Adding an app configuration key

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
- Do not edit locale files one by one. Use the batch scripts used for the
  affected surface and preserve nested namespaces, placeholders, and existing
  translations.
- Product names remain untranslated.
- Run `pnpm check:repo` after locale changes.
- Keep the language selector in `README.md` and `docs/README.*.md` valid.

## Reporting bugs and requesting features

Use the GitHub issue templates. Include the Rayload version, Android version,
device model, reproduction steps, and relevant logs. Remove RPC secrets,
private URLs, file paths, and other personal data before posting.

## Licensing

Rayload is distributed under GPL-3.0. By submitting a contribution, you agree
that it may be distributed under the same license. Preserve the MIT attribution
for Rayburst and the GPL-2.0-or-later attribution for aria2-next in `NOTICE`.
Do not add dependencies or assets with licenses incompatible with GPL-3.0.

Never commit keystores, signing passwords, local SDK paths, build output, or
generated APK files.
