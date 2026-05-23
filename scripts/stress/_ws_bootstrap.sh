#!/usr/bin/env bash
# Bootstrap a test workspace with: turingos init + assets/ copy from repo root.
# Usage: _ws_bootstrap.sh <abs_ws_path>
set -euo pipefail
WS="$1"
REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BIN="$REPO/target/debug/turingos"
"$BIN" init --project "$WS" --template proof --force >/dev/null 2>&1 || true
# Copy assets so spec_turn_handler can find grill_meta_v1.md
if [[ -d "$REPO/assets" && ! -d "$WS/assets" ]]; then
  cp -r "$REPO/assets" "$WS/assets"
fi
echo "$WS"
