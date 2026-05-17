#!/usr/bin/env bash
# TISR Phase 6.3 — end-to-end demo runner (real SiliconFlow LLM).
#
# Args:
#   $1 — workspace dir (will be wiped + recreated)
#   $2 — answers JSON file path
#
# Per the architect mandate (2026-05-17), this exercises the FULL flow:
#   init -> llm config -> spec (real Meta LLM) -> generate (real Blackbox LLM)
#   -> welcome -> structural verification of the generated artifact.
#
# Used by phase63_three_round_demo.sh for the 3-round non-randomness check.

set -euo pipefail
WORKSPACE="${1:?workspace required}"
ANSWERS="${2:?answers file required}"

cd "$(dirname "$0")/.."
ROOT="$(pwd)"

# Source main-repo .env for SILICONFLOW_API_KEY.
if [ -f /home/zephryj/projects/turingosv4/.env ]; then
    set -a
    # shellcheck disable=SC1091
    . /home/zephryj/projects/turingosv4/.env
    set +a
fi
: "${SILICONFLOW_API_KEY:?SILICONFLOW_API_KEY not set; populate /home/zephryj/projects/turingosv4/.env}"

BIN="$ROOT/target/debug/turingos"
[ -x "$BIN" ] || { echo "[FAIL] turingos binary missing; run: cargo build --bin turingos"; exit 2; }

echo "[1/6] init workspace at $WORKSPACE"
rm -rf "$WORKSPACE"
"$BIN" init --project "$WORKSPACE" --template proof >/dev/null
[ -f "$WORKSPACE/genesis_payload.toml" ] || { echo "[FAIL] init did not create genesis_payload.toml"; exit 1; }

echo "[2/6] llm config (SiliconFlow defaults)"
"$BIN" llm config --workspace "$WORKSPACE" >/dev/null

echo "[3/6] spec (real Meta LLM call — DeepSeek-V3.2)"
"$BIN" spec --workspace "$WORKSPACE" --answers-file "$ANSWERS" 2>&1
[ -f "$WORKSPACE/spec.md" ] || { echo "[FAIL] spec.md not created"; exit 1; }
[ -f "$WORKSPACE/spec_transcript.jsonl" ] || { echo "[FAIL] transcript not created"; exit 1; }
SIDECAR="$WORKSPACE/cas/.turingos_cas_index.jsonl"
[ -f "$SIDECAR" ] || { echo "[FAIL] CAS sidecar missing"; exit 1; }
grep -q "turingos-spec-capsule-v1" "$SIDECAR" || { echo "[FAIL] CAS sidecar missing spec-capsule schema"; exit 1; }

echo "[4/6] generate (real Blackbox LLM call — Qwen3-Coder-30B)"
"$BIN" generate --workspace "$WORKSPACE" --emit-transcript 2>&1
[ -d "$WORKSPACE/artifacts" ] || { echo "[FAIL] artifacts/ not created"; exit 1; }
ARTIFACT_COUNT=$(find "$WORKSPACE/artifacts" -type f | wc -l)
[ "$ARTIFACT_COUNT" -gt 0 ] || { echo "[FAIL] no artifact files emitted"; exit 1; }
echo "       artifacts emitted: $ARTIFACT_COUNT"

echo "[5/6] welcome (CAS-aware status)"
"$BIN" welcome --workspace "$WORKSPACE"

echo "[6/7] structural game-artifact verification"
python3 "$ROOT/scripts/phase63_verify_artifact.py" "$WORKSPACE/artifacts"

echo "[7/7] functional gameplay verification (jsdom)"
# Ensure jsdom is available in a fixed location (one-time install).
JSDOM_DIR="/tmp/p63-test-deps"
if [ ! -d "$JSDOM_DIR/node_modules/jsdom" ]; then
    echo "       installing jsdom (one-time)..."
    mkdir -p "$JSDOM_DIR"
    (cd "$JSDOM_DIR" && npm init -y >/dev/null 2>&1 && npm install jsdom >/dev/null 2>&1)
fi
HTML_FILE=$(find "$WORKSPACE/artifacts" -name "*.html" -o -name "*.htm" | head -1)
if [ -z "$HTML_FILE" ]; then
    echo "[FAIL] no HTML file to functional-test"
    exit 1
fi
node "$ROOT/scripts/phase63_functional_play.js" "$HTML_FILE"

echo "[OK] full E2E demo passed for $WORKSPACE"
