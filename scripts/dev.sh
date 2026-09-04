#!/usr/bin/env bash
# One-command local preview: builds the WebAssembly app, serves it, watches for changes.
#   ./scripts/dev.sh                 → http://127.0.0.1:8787/
#   PORT=9000 ./scripts/dev.sh       → custom port
#   ./scripts/dev.sh --profile release
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec python3 "$ROOT/scripts/serve.py" --watch "$@"
