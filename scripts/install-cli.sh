#!/bin/sh
# Install the Rayload headless CLI into ~/.local/bin without root.
set -eu

repo="shiwushuifeng/Rayload"
bin_dir="${RAYLOAD_BIN_DIR:-$HOME/.local/bin}"

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

curl -fsSL "$base/$asset" -o "$tmp/$asset"
curl -fsSL "$base/SHA256SUMS-v$version.txt" -o "$tmp/SHA256SUMS.txt"
(cd "$tmp" && grep "  $asset\$" SHA256SUMS.txt | sha256sum -c -)

mkdir -p "$bin_dir"
tar -xzf "$tmp/$asset" -C "$tmp"
install -m 0755 "$tmp/rayload-cli-v$version-linux-x86_64/ray" \
  "$tmp/rayload-cli-v$version-linux-x86_64/rayload" \
  "$tmp/rayload-cli-v$version-linux-x86_64/rayload-cli" \
  "$bin_dir/"

echo "Installed Rayload CLI $version to $bin_dir"
case ":$PATH:" in
  *":$bin_dir:"*) ;;
  *) echo "Add it to PATH: export PATH=\"$bin_dir:\$PATH\"" ;;
esac
