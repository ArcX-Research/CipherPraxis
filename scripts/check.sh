#!/usr/bin/env bash
# Formatting, lints, unit tests, content validation and a wasm type-check.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
echo "▸ cargo fmt --check"; cargo fmt --all -- --check
echo "▸ cargo test (core, native)"; cargo test -p praxis-core --features authoring
echo "▸ cargo test (web unit tests, native)"; cargo test -p praxis-web --lib 2>/dev/null || cargo test -p praxis-web
echo "▸ content check"; cargo run -q -p praxis-core --features authoring --bin praxis-check -- content
if command -v cargo-clippy >/dev/null 2>&1; then
  echo "▸ clippy (wasm32)"; cargo clippy -p praxis-web --target wasm32-unknown-unknown -- -D warnings
fi
echo "▸ cargo check (wasm32)"; cargo check -p praxis-web --target wasm32-unknown-unknown
if [ "${REPRO:-0}" = "1" ]; then
  echo "▸ reproducibility: two clean release builds must hash identically"
  scripts/build.sh release >/dev/null && H1="$(shasum -a 256 dist/pkg/praxis_bg.wasm | cut -d' ' -f1)"
  cargo clean -p praxis-web --release --target wasm32-unknown-unknown >/dev/null 2>&1 || true
  scripts/build.sh release >/dev/null && H2="$(shasum -a 256 dist/pkg/praxis_bg.wasm | cut -d' ' -f1)"
  if [ "$H1" != "$H2" ]; then echo "reproducibility FAILED: $H1 != $H2" >&2; exit 1; fi
  echo "  identical: $H1"
fi
echo "✓ all checks passed"
