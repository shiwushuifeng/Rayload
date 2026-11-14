#!/usr/bin/env bash
set -euo pipefail

# Builds the x86_64 release APK. x86_64 always ships the full feature set.

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
ANDROID_HOME=${ANDROID_HOME:-"$ROOT/.toolchains/android-sdk"}
ANDROID_SDK_ROOT=$ANDROID_HOME
JAVA_HOME=${JAVA_HOME:-"$ROOT/.toolchains/jdk21"}
NDK_HOME=${NDK_HOME:-$(find "$ANDROID_HOME/ndk" -mindepth 1 -maxdepth 1 -type d | sort -V | tail -n 1)}
export ANDROID_HOME ANDROID_SDK_ROOT JAVA_HOME NDK_HOME

VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' "$ROOT/src-tauri/Cargo.toml" | head -n 1)
APK="$ROOT/release/current/Rayload-v${VERSION}-x86_64-full.apk"

cd "$ROOT"
RAYLOAD_SMOKE_X86_64=1 pnpm tauri android build --apk --target x86_64 --features plugin_engine "$@"

mkdir -p "$ROOT/release/current"
cp "$ROOT/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk" "$APK"
echo "x86_64 full APK: $APK"
