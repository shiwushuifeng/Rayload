#!/usr/bin/env bash
# Builds the pinned Aria2 Next engine for Android arm64 from vendor/aria2-next.
#
# Upstream provides an Android target that cross-compiles the vendored
# dependency stack with the NDK, so the engine can be compiled from source
# instead of downloading a prebuilt binary.
#
# Usage: NDK=/path/to/android-ndk [API=24] scripts/android/build-engine.sh
set -euo pipefail

: "${NDK:?NDK must point to an Android NDK}"
api=${API:-24}

cd "$(dirname "$0")/../.."
root=$(pwd)
source_dir=$root/vendor/aria2-next
output=$root/src-tauri/binaries/aria2-next-aarch64-linux-android

if [ ! -f "$source_dir/packaging/scripts/build-release" ]; then
  echo "vendor/aria2-next is missing; run 'git submodule update --init vendor/aria2-next'" >&2
  exit 1
fi

case $(uname -s) in
  Linux) ndk_host=linux-x86_64 ;;
  Darwin) ndk_host=darwin-x86_64 ;;
  *)
    echo "unsupported host for the Android engine build: $(uname -s)" >&2
    exit 1
    ;;
esac

toolchain=$NDK/toolchains/llvm/prebuilt/$ndk_host/bin
export PATH="$toolchain:$PATH"
export STRIP="$toolchain/llvm-strip"
export READELF="$toolchain/llvm-readelf"

# Drop the committed copy first so a failed cross-compile can never leave the
# previous engine in place and silently ship it.
rm -f "$output"

(
  cd "$source_dir"
  NDK="$NDK" API="$api" BUILD_DIR=build/android \
    ./packaging/scripts/build-release android
)

install -m 0755 "$source_dir/build/android/aria2-next" "$output"
echo "engine written to ${output#"$root"/}"