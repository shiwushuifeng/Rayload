#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../.."
export RAYLOAD_FDROID=1
exec pnpm tauri android build --apk --target aarch64 --features fdroid "$@"
