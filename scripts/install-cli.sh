#!/bin/sh
# Install the Rayload headless CLI, optionally as a system-wide service.
set -eu

repo="shiwushuifeng/Rayload"

if [ "$(uname -s)" != "Linux" ]; then
  echo "rayload CLI currently supports Linux only." >&2
  exit 1
fi
case "$(uname -m)" in
  x86_64 | amd64) ;;
  *)
    echo "rayload CLI currently supports x86_64 only." >&2
    exit 1
    ;;
esac

tty=""
if ( : </dev/tty ) 2>/dev/null; then
  tty=1
fi

scope="${RAYLOAD_INSTALL_SCOPE:-}"
if [ -z "$scope" ] && [ -n "$tty" ]; then
  printf 'Install location [user/global] (user): ' >&2
  read -r scope </dev/tty || scope=""
fi
scope="${scope:-user}"
case "$scope" in
  user | global) ;;
  *)
    echo "Install location must be user or global." >&2
    exit 1
    ;;
esac
if [ "$scope" = global ]; then
  bin_dir="${RAYLOAD_BIN_DIR:-/usr/local/bin}"
else
  bin_dir="${RAYLOAD_BIN_DIR:-$HOME/.local/bin}"
fi

version="${RAYLOAD_VERSION:-}"
if [ -z "$version" ]; then
  release_json="$(curl -fsSL "https://api.github.com/repos/$repo/releases/latest")" || {
    echo "Could not query the latest Rayload release." >&2
    exit 1
  }
  version="$(printf '%s' "$release_json" | sed -n 's/.*"tag_name": *"v\?\([^"]*\)".*/\1/p')"
fi
version="${version#v}"
if [ -z "$version" ]; then
  echo "Could not determine the Rayload CLI version." >&2
  exit 1
fi

asset="rayload-cli-v$version-linux-x86_64.tar.gz"
base="${RAYLOAD_CLI_BASE:-https://github.com/$repo/releases/download/v$version}"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

curl_opts="-fsSL"
if [ -t 2 ]; then
  curl_opts="-fL --progress-bar"
  echo "Downloading Rayload CLI $version..."
fi
curl $curl_opts "$base/$asset" -o "$tmp/$asset"
curl -fsSL "$base/SHA256SUMS-v$version.txt" -o "$tmp/SHA256SUMS.txt"
(cd "$tmp" && grep "  $asset\$" SHA256SUMS.txt | sha256sum -c -)

tar -xzf "$tmp/$asset" -C "$tmp"
if [ "$scope" = global ] && [ -z "${RAYLOAD_BIN_DIR:-}" ]; then
  sudo mkdir -p "$bin_dir"
  sudo install -m 0755 "$tmp/rayload-cli-v$version-linux-x86_64/ray" \
    "$tmp/rayload-cli-v$version-linux-x86_64/rayload" \
    "$tmp/rayload-cli-v$version-linux-x86_64/rayload-cli" \
    "$bin_dir/"
else
  mkdir -p "$bin_dir"
  install -m 0755 "$tmp/rayload-cli-v$version-linux-x86_64/ray" \
    "$tmp/rayload-cli-v$version-linux-x86_64/rayload" \
    "$tmp/rayload-cli-v$version-linux-x86_64/rayload-cli" \
    "$bin_dir/"
fi

echo "Installed Rayload CLI $version to $bin_dir"
if [ "${RAYLOAD_SKIP_INIT:-0}" != 1 ] && [ ! -f "${RAYLOAD_CONFIG:-$HOME/.config/rayload/config.yml}" ]; then
  if [ -n "$tty" ]; then
    echo "First-time setup:" >&2
    "$bin_dir/ray" init </dev/tty
  else
    echo "Config not found. Run: $bin_dir/ray init" >&2
  fi
fi

enable="${RAYLOAD_ENABLE_SERVICE:-}"
if [ -z "$enable" ] && [ -n "$tty" ]; then
  printf 'Enable start on boot? [Y/n]: ' >&2
  read -r answer </dev/tty || answer=""
  case "$answer" in
    n | N | no | No | NO) enable=0 ;;
    *) enable=1 ;;
  esac
fi
if [ "${enable:-0}" = 1 ]; then
  if [ "$scope" = global ]; then
    sudo "$bin_dir/ray" install
  else
    "$bin_dir/ray" install --user
  fi
fi

case ":$PATH:" in
  *":$bin_dir:"*) ;;
  *) echo "Add it to PATH: export PATH=\"$bin_dir:\$PATH\"" ;;
esac
