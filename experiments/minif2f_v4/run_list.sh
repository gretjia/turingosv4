#!/usr/bin/env bash
# List-driven runner — reads problem basenames from a file, runs one per line.
# Use: ./run_list.sh <condition> <list-file> [tag]

set -uo pipefail

if [ -z "${DEEPSEEK_API_KEY:-}" ] && [ -f "$HOME/projects/turingosv3/.env" ]; then
    source "$HOME/projects/turingosv3/.env"
fi
export LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
export ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
# C-012 provenance: stamp per-row commit SHA so post-hoc audits can match
# each jsonl row to the binary that produced it. Overridable; falls back
# to `git rev-parse --short HEAD` evaluated at runner start.
export BUILD_SHA="${BUILD_SHA:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && git rev-parse --short HEAD 2>/dev/null || echo unknown)}"

CONDITION="${1:-oneshot}"
LIST_FILE="${2:?usage: $0 <condition> <list-file> [tag]}"
TAG="${3:-list}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"
LOG_DIR="$SCRIPT_DIR/logs"
TIMESTAMP=$(date +%Y%m%dT%H%M%S)
RESULTS_FILE="$LOG_DIR/${TAG}_${CONDITION}_${TIMESTAMP}.jsonl"
STDERR_LOG="$LOG_DIR/${TAG}_${CONDITION}_${TIMESTAMP}.err"
mkdir -p "$LOG_DIR"

mapfile -t PROBLEMS < "$LIST_FILE"

echo "List runner | Condition=$CONDITION | Tag=$TAG | N=${#PROBLEMS[@]}"
echo "Results: $RESULTS_FILE"
echo "Stderr:  $STDERR_LOG"

(cd "$PROJECT_ROOT" && cargo build --release -p minif2f_v4 2>&1 | tail -1)
EVALUATOR="$PROJECT_ROOT/target/release/evaluator"

# Preflight (C-012)
LEAN_BIN="${LEAN_BINARY:-$HOME/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean}"
PFL=$(find "$MINIF2F_DIR/.lake/packages" \( -path "*/.lake/build/lib/lean" -o -path "*/lib/lean" \) -type d 2>/dev/null | tr '\n' ':')
if [ -z "$PFL" ]; then echo "PREFLIGHT FAIL: no Mathlib"; exit 2; fi
OUT=$(printf 'import Mathlib\nexample : (1:ℝ) + 1 = 2 := by norm_num\n' | LEAN_PATH="$PFL" timeout 180 "$LEAN_BIN" --stdin 2>&1)
if [ $? -ne 0 ] || echo "$OUT" | grep -q "error:"; then echo "PREFLIGHT FAIL: $OUT" | head -c 400; exit 2; fi
echo "Preflight OK."

SOLVED=0
for NAME in "${PROBLEMS[@]}"; do
    [ -z "$NAME" ] && continue
    PROBLEM="$MINIF2F_DIR/MiniF2F/Test/${NAME}.lean"
    if [ ! -f "$PROBLEM" ]; then echo "[$NAME] missing, skip"; continue; fi

    echo -n "[$NAME] ... "
    echo "=== $NAME @ $(date -Is) ===" >> "$STDERR_LOG"
    OUTPUT=$(timeout 900 env CONDITION="$CONDITION" MINIF2F_DIR="$MINIF2F_DIR" \
        EXPERIMENT_DIR="$SCRIPT_DIR" RUST_LOG=info \
        "$EVALUATOR" "$PROBLEM" 2>>"$STDERR_LOG") || true
    PPUT_JSON=$(echo "$OUTPUT" | grep "^PPUT_RESULT:" | sed 's/^PPUT_RESULT://' | head -1)
    if [ -n "$PPUT_JSON" ]; then
        echo "$PPUT_JSON" >> "$RESULTS_FILE"
        HAS_GP=$(echo "$PPUT_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin)['has_golden_path'])" 2>/dev/null || echo False)
        if [ "$HAS_GP" = "True" ]; then
            SOLVED=$((SOLVED + 1))
            PV=$(echo "$PPUT_JSON" | python3 -c "import sys,json; print(f\"{json.load(sys.stdin)['pput']:.2f}\")")
            TV=$(echo "$PPUT_JSON" | python3 -c "import sys,json; print(f\"{json.load(sys.stdin)['time_secs']:.0f}\")")
            echo "SOLVED (${TV}s PPUT=${PV})"
        else
            TV=$(echo "$PPUT_JSON" | python3 -c "import sys,json; print(f\"{json.load(sys.stdin)['time_secs']:.0f}\")")
            echo "FAIL (${TV}s)"
        fi
    else
        echo "MEASUREMENT_ERROR/TIMEOUT"
    fi
done

echo ""
echo "Summary: $SOLVED / ${#PROBLEMS[@]} solved"
