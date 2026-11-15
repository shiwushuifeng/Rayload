#!/usr/bin/env bash
set -euo pipefail

# Prints release/current/v<base>/<channel> for the version in Cargo.toml.
ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' "$ROOT/src-tauri/Cargo.toml" | head -n 1)
BASE=${VERSION%%-*}
CHANNEL=stable
if [ "$VERSION" != "$BASE" ]; then
  CHANNEL=${VERSION#*-}
fi
printf '%s\n' "$ROOT/release/current/v${BASE}/${CHANNEL}"
