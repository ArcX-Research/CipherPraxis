#!/usr/bin/env bash
# Builds the Cipher Praxis web app to ./dist (static files; serve from any HTTP server with an
# SPA fallback to index.html).
#
#   scripts/build.sh            release build (optimised, small)
#   scripts/build.sh dev        fast debug build for local iteration
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROFILE="${1:-release}"
cd "$ROOT"

if ! rustup target list --installed 2>/dev/null | grep -q '^wasm32-unknown-unknown$'; then
  echo "error: the wasm32-unknown-unknown target is not installed. Run: rustup target add wasm32-unknown-unknown" >&2
  exit 1
fi
LOCK_VER="$(sed -n '/^name = "wasm-bindgen"$/{n;s/^version = "\(.*\)"/\1/p;}' Cargo.lock | head -1)"
if ! command -v wasm-bindgen >/dev/null 2>&1; then
  echo "error: wasm-bindgen CLI not found. Install the pinned version: cargo install wasm-bindgen-cli --version ${LOCK_VER:-0.2.127}" >&2
  exit 1
fi
CLI_VER="$(wasm-bindgen --version | awk '{print $2}')"
if [ -n "$LOCK_VER" ] && [ "$CLI_VER" != "$LOCK_VER" ]; then
  echo "error: wasm-bindgen CLI $CLI_VER does not match the crate version $LOCK_VER in Cargo.lock." >&2
  echo "       Run: cargo install wasm-bindgen-cli --version $LOCK_VER   (or pin Cargo.toml to $CLI_VER)" >&2
  exit 1
fi

case "$PROFILE" in
  release)
    OUT_DIR="release"
    ;;
  dev)
    OUT_DIR="debug"
    ;;
  *)
    echo "usage: $0 [release|dev]" >&2
    exit 2
    ;;
esac

echo "▸ cargo build (wasm32, $PROFILE)"
if [ "$PROFILE" = "release" ]; then
  cargo build -p praxis-web --target wasm32-unknown-unknown --release
else
  cargo build -p praxis-web --target wasm32-unknown-unknown
fi

WASM="$ROOT/target/wasm32-unknown-unknown/$OUT_DIR/praxis-web.wasm"
rm -rf "$ROOT/dist"
mkdir -p "$ROOT/dist/pkg"
echo "▸ wasm-bindgen"
wasm-bindgen --target web --no-typescript --out-dir "$ROOT/dist/pkg" --out-name praxis "$WASM"
if [ "$PROFILE" = release ] && command -v wasm-opt >/dev/null 2>&1; then
  echo "▸ wasm-opt -Os"
  wasm-opt -Os -o "$ROOT/dist/pkg/praxis_bg.wasm" "$ROOT/dist/pkg/praxis_bg.wasm"
fi
cp -R "$ROOT/static/." "$ROOT/dist/"
# Cache-bust the module and stylesheet with a content hash.
if command -v shasum >/dev/null 2>&1; then
  HASH="$(shasum -a 256 "$ROOT/dist/pkg/praxis_bg.wasm" | cut -c1-10)"
else
  HASH="$(sha256sum "$ROOT/dist/pkg/praxis_bg.wasm" | cut -c1-10)"
fi
sed -i.bak -e "s#/pkg/praxis.js#/pkg/praxis.js?v=$HASH#g" -e "s#/styles.css#/styles.css?v=$HASH#g" "$ROOT/dist/index.html"
rm -f "$ROOT/dist/index.html.bak"
# SPA fallback for static hosts that serve 404.html.
cp "$ROOT/dist/index.html" "$ROOT/dist/404.html"
echo "▸ dist/ ready ($PROFILE)"
ls -la "$ROOT/dist/pkg" | awk '$1 ~ /^-/ {printf "   %-22s %8.1f KB\n", $9, $5/1024}'
