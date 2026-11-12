# Project Layout

Rayload keeps tool-required source paths at the repository root while grouping
documentation, release assets, branding sources, and local-only material.

## Top-level categories

| Path              | Purpose                                                    |
| ----------------- | ---------------------------------------------------------- |
| `src/`            | Vue frontend and frontend tests                            |
| `src-tauri/`      | Rust backend, native services, and Android Studio project  |
| `scripts/`        | Build, localization, release, and repository maintenance   |
| `public/`         | Static frontend assets                                     |
| `e2e/`            | End-to-end launcher scripts                                |
| `docs/`           | README translations, release notes, handoff, project docs  |
| `docs/assets/`    | Documentation images, including the sponsor QR code        |
| `assets/branding/`| Original logo, icon, and preview source artwork            |
| `release/current/`| Current APK and checksum material                          |
| `release/archive/`| Historical Rayload and Rayburst build artifacts            |
| `signing/`        | Local release signing configuration                        |
| `local/`          | Local-only unrelated material, never committed             |
| `dist/`           | Generated frontend build output                            |
| `node_modules/`   | Installed JavaScript dependencies                          |
| `.toolchains/`    | Local toolchain cache                                      |
| `src-tauri/target/`| Rust and Android build cache                              |

## Root files

- `README.md` is the default bilingual English/Simplified Chinese entry point.
- `AGENTS.md` contains repository working rules.
- `LICENSE` contains the GPL-3.0 license text and Rayload copyright notice.
- `NOTICE` contains third-party attributions, including the Rayburst MIT
  notice and aria2-next's GPL-2.0-or-later notice.
- `CONTRIBUTING.md` contains contribution, verification, and licensing rules.
- `SPONSOR.md` is the standalone sponsorship page linked from the README.
- `ROADMAP.md` tracks product milestones and engineering goals.
- `package.json`, `pnpm-lock.yaml`, and workspace/config files are required by
  the frontend build and remain at the root.

## Documentation

- `docs/README.<locale>.md` contains each translated README.
- `docs/CHANGELOG.md`, `docs/RELEASE_NOTES.md`, and `docs/ANDROID.md` contain
  project history, current release information, and Android instructions.
- `docs/HANDOFF.md` is local-only and Git-excluded.

## Release assets

- `release/current/` contains only the current distributable APK and checksums.
- `release/archive/` contains older APKs and Rayburst-derived reference builds.
- `release/` is Git-ignored and must not be committed.

## Local-only material

- `local/` and `docs/HANDOFF.md` are excluded through `.git/info/exclude`.
- Never commit keystores, signing passwords, local SDK paths, build output, or
  generated APK files.
