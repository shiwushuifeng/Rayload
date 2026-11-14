#!/usr/bin/env bash
set -euo pipefail

# Headless emulator smoke test for the lite x86_64 build. Release x86_64
# artifacts are built separately by scripts/android/build-x86_64.sh.

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
ANDROID_HOME=${ANDROID_HOME:-"$ROOT/.toolchains/android-sdk"}
ANDROID_SDK_ROOT=$ANDROID_HOME
JAVA_HOME=${JAVA_HOME:-"$ROOT/.toolchains/jdk21"}
NDK_HOME=${NDK_HOME:-$(find "$ANDROID_HOME/ndk" -mindepth 1 -maxdepth 1 -type d | sort -V | tail -n 1)}
ADB="$ANDROID_HOME/platform-tools/adb"
EMULATOR="$ANDROID_HOME/emulator/emulator"
AVD=${RAYLOAD_AVD:-Rayload_Test}
SERIAL=${RAYLOAD_SERIAL:-emulator-5554}
PACKAGE=com.shiwushuifeng.rayload
FRESH=0
BUILD=0
APK=""

usage() {
  cat <<EOF
Usage: $0 [--build] [--fresh] [--apk PATH]

  --build      Build a fresh x86_64 emulator APK before installing it.
  --fresh      Clear app data so the first-run wizard is exercised.
  --apk PATH   Install a specific APK instead of the default smoke APK.
EOF
}

while [ $# -gt 0 ]; do
  case "$1" in
    --build) BUILD=1 ;;
    --fresh) FRESH=1 ;;
    --apk)
      shift
      APK=${1:?--apk requires a path}
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
  shift
done

if [ ! -r /dev/kvm ] || [ ! -w /dev/kvm ]; then
  echo "KVM is not accessible (/dev/kvm). Add this user to the kvm group and retry." >&2
  exit 2
fi

if [ ! -x "$ADB" ] || [ ! -x "$EMULATOR" ]; then
  echo "Android SDK tools are missing under $ANDROID_HOME." >&2
  exit 2
fi

export ANDROID_HOME ANDROID_SDK_ROOT JAVA_HOME NDK_HOME
export PATH="$JAVA_HOME/bin:$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH"

if ! "$ADB" devices | awk '{print $1}' | grep -qx "$SERIAL"; then
  echo "Starting $AVD in headless mode..."
  setsid -f "$EMULATOR" -avd "$AVD" \
    -no-window -no-audio -no-boot-anim \
    -gpu swiftshader_indirect \
    -no-snapshot -no-metrics \
    > /tmp/rayload-emulator.log 2>&1
fi

"$ADB" -s "$SERIAL" wait-for-device
for _ in $(seq 1 120); do
  BOOTED=$("$ADB" -s "$SERIAL" shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')
  [ "$BOOTED" = "1" ] && break
  sleep 2
done
if [ "${BOOTED:-}" != "1" ]; then
  echo "Emulator did not finish booting. See /tmp/rayload-emulator.log." >&2
  exit 1
fi

VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' "$ROOT/src-tauri/Cargo.toml" | head -n 1)
DEFAULT_APK="$ROOT/release/current/Rayload-v${VERSION}-x86_64-smoke.apk"

if [ "$BUILD" = "1" ]; then
  ln -sfn ../../.toolchains/binaries/aria2-next-android-x86_64 \
    "$ROOT/src-tauri/binaries/aria2-next-x86_64-linux-android"
  echo "Building x86_64 emulator APK..."
  (
    cd "$ROOT"
    RAYLOAD_SMOKE_X86_64=1 pnpm tauri android build --apk --target x86_64
  )
  mkdir -p "$ROOT/release/current"
  cp "$ROOT/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk" "$DEFAULT_APK"
  APK=$DEFAULT_APK
fi

APK=${APK:-$DEFAULT_APK}
if [ ! -f "$APK" ]; then
  echo "APK not found: $APK (run with --build first)." >&2
  exit 2
fi

echo "Installing $APK..."
"$ADB" -s "$SERIAL" install -r "$APK" >/dev/null

if [ "$FRESH" = "1" ]; then
  "$ADB" -s "$SERIAL" shell pm clear "$PACKAGE" >/dev/null
fi

"$ADB" -s "$SERIAL" logcat -c
"$ADB" -s "$SERIAL" shell am force-stop "$PACKAGE"
"$ADB" -s "$SERIAL" shell monkey -p "$PACKAGE" -c android.intent.category.LAUNCHER 1 >/dev/null 2>&1
sleep 8

PID=$("$ADB" -s "$SERIAL" shell pidof "$PACKAGE" | tr -d '\r')
if [ -z "$PID" ]; then
  echo "App process is not running after launch." >&2
  "$ADB" -s "$SERIAL" logcat -d -v brief | tail -n 120 >&2
  exit 1
fi

if ! "$ADB" -s "$SERIAL" shell ps -A | grep -q 'libaria2.so'; then
  echo "Aria2 engine process did not start." >&2
  "$ADB" -s "$SERIAL" logcat -d -v brief | tail -n 120 >&2
  exit 1
fi

CRASH=$("$ADB" -s "$SERIAL" logcat -d -v brief | grep -E "FATAL EXCEPTION|ANR in $PACKAGE" || true)
if [ -n "$CRASH" ]; then
  echo "Crash or ANR detected:" >&2
  printf '%s\n' "$CRASH" >&2
  exit 1
fi

"$ADB" -s "$SERIAL" forward tcp:9222 "localabstract:webview_devtools_remote_$PID" >/dev/null
sleep 1
TITLE=$(curl -fsS http://127.0.0.1:9222/json | python3 -c 'import json,sys; pages=json.load(sys.stdin); print(pages[0]["title"] if pages else "")')
if [ "$TITLE" != "Rayload" ]; then
  echo "WebView page is not ready (title: ${TITLE:-missing})." >&2
  exit 1
fi

"$ADB" -s "$SERIAL" exec-out screencap -p > "$ROOT/release/current/emulator-smoke.png"

echo "Smoke test passed."
echo "  serial:     $SERIAL"
echo "  app pid:    $PID"
echo "  webview:    $TITLE"
echo "  screenshot: $ROOT/release/current/emulator-smoke.png"
