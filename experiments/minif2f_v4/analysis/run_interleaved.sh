#!/usr/bin/env bash
# Interleaved multi-condition runner — v3.1
# Per-problem rotation of condition order to neutralize API drift (C-033).
# Abort gate per condition independently (Art. V.2, C-012).
#
# Usage: run_interleaved.sh [sample_file]
#   Defaults to sample_N50_S74677.txt in this dir.

set -uo pipefail

if [ -z "${DEEPSEEK_API_KEY:-}" ] && [ -f "$HOME/projects/turingosv3/.env" ]; then
    source "$HOME/projects/turingosv3/.env"
fi
export LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
export ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-reasoner}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXP_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PROJECT_ROOT="$(cd "$EXP_DIR/../.." && pwd)"
MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"

SAMPLE="${1:-$SCRIPT_DIR/sample_N50_S74677.txt}"
[ ! -f "$SAMPLE" ] && { echo "Sample file not found: $SAMPLE"; exit 1; }

# Gate is orchestrator-side (local dual audit via Codex+Gemini agents before invoking).
# No shell-side gate: synchronous coupling to remote routine would add 4h lag.
# Independence per C-010 is maintained by the cross-vendor local agents.

TIMESTAMP=$(date +%Y%m%dT%H%M%S)
LOG_DIR="$EXP_DIR/logs"
mkdir -p "$LOG_DIR"
OUT_ONESHOT="$LOG_DIR/v31_oneshot_${TIMESTAMP}.jsonl"
OUT_N1="$LOG_DIR/v31_n1_${TIMESTAMP}.jsonl"
OUT_N3="$LOG_DIR/v31_n3_${TIMESTAMP}.jsonl"
STDERR_LOG="$LOG_DIR/v31_${TIMESTAMP}.err"

echo "v3.1 interleaved runner"
echo "Sample: $SAMPLE"
echo "Outputs: $OUT_ONESHOT | $OUT_N1 | $OUT_N3"
echo "Stderr: $STDERR_LOG"
echo ""

# Build
(cd "$PROJECT_ROOT" && cargo build --release -p minif2f_v4 2>&1 | tail -1)
EVALUATOR="$PROJECT_ROOT/target/release/evaluator"

# Preflight (C-012)
LEAN_BIN="${LEAN_BINARY:-$HOME/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean}"
PFL=$(find "$MINIF2F_DIR/.lake/packages" \( -path "*/.lake/build/lib/lean" -o -path "*/lib/lean" \) -type d 2>/dev/null | tr '\n' ':')
[ -z "$PFL" ] && { echo "PREFLIGHT FAIL: no Mathlib"; exit 2; }
OUT=$(printf 'import Mathlib\nexample : (1:ℝ) + 1 = 2 := by norm_num\n' | LEAN_PATH="$PFL" timeout 180 "$LEAN_BIN" --stdin 2>&1)
if [ $? -ne 0 ] || echo "$OUT" | grep -q "error:"; then echo "PREFLIGHT FAIL: $OUT" | head -c 400; exit 2; fi
echo "Preflight OK."

# Read sample (skip comments)
mapfile -t PROBLEMS < <(grep -v '^#' "$SAMPLE" | grep -v '^$')
N=${#PROBLEMS[@]}
echo "Loaded $N problems"

# Per-condition abort gate: first 10 problems (20% of 50); if >= 3 timeout → halt that condition
declare -A SOLVED TIMEOUT ABORTED
for c in oneshot n1 n3; do SOLVED[$c]=0; TIMEOUT[$c]=0; ABORTED[$c]=0; done
ABORT_AFTER=10   # first 20% of 50
ABORT_THRESH=3   # 30% of 10 = 3

# Rotation: 3 conditions × 3 permutations
# idx 0: oneshot, n1, n3
# idx 1: n1, n3, oneshot
# idx 2: n3, oneshot, n1
ROT_0="oneshot n1 n3"
ROT_1="n1 n3 oneshot"
ROT_2="n3 oneshot n1"

run_one() {
    local CONDITION="$1"; local PROBLEM_FILE="$2"; local NAME="$3"
    local OUT_FILE
    case "$CONDITION" in
        oneshot) OUT_FILE="$OUT_ONESHOT" ;;
        n1) OUT_FILE="$OUT_N1" ;;
        n3) OUT_FILE="$OUT_N3" ;;
    esac
    # Abort check
    if [ "${ABORTED[$CONDITION]}" = 1 ]; then
        echo "  [$CONDITION] SKIPPED (aborted)" ; return
    fi
    echo -n "  [$CONDITION] ... "
    echo "=== $NAME @ $(date -Is) condition=$CONDITION ===" >> "$STDERR_LOG"
    local OUTPUT
    OUTPUT=$(timeout 900 env CONDITION="$CONDITION" MINIF2F_DIR="$MINIF2F_DIR" \
        EXPERIMENT_DIR="$EXP_DIR" RUST_LOG=info \
        "$EVALUATOR" "$PROBLEM_FILE" 2>>"$STDERR_LOG") || true
    local PPUT_JSON
    PPUT_JSON=$(echo "$OUTPUT" | grep "^PPUT_RESULT:" | sed 's/^PPUT_RESULT://' | head -1)
    if [ -n "$PPUT_JSON" ]; then
        echo "$PPUT_JSON" >> "$OUT_FILE"
        local HAS_GP
        HAS_GP=$(echo "$PPUT_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin)['has_golden_path'])" 2>/dev/null || echo False)
        if [ "$HAS_GP" = "True" ]; then
            SOLVED[$CONDITION]=$((${SOLVED[$CONDITION]}+1))
            local PV; PV=$(echo "$PPUT_JSON" | python3 -c "import sys,json; print(f\"{json.load(sys.stdin)['pput']:.2f}\")")
            local TV; TV=$(echo "$PPUT_JSON" | python3 -c "import sys,json; print(f\"{json.load(sys.stdin)['time_secs']:.0f}\")")
            echo "SOLVED (${TV}s PPUT=${PV})"
        else
            echo "FAIL"
        fi
    else
        TIMEOUT[$CONDITION]=$((${TIMEOUT[$CONDITION]}+1))
        echo "MEASUREMENT_ERROR/TIMEOUT"
    fi
}

check_abort_gate() {
    local CONDITION="$1"; local PROBLEMS_SEEN="$2"
    if [ "$PROBLEMS_SEEN" -eq "$ABORT_AFTER" ] && [ "${TIMEOUT[$CONDITION]}" -ge "$ABORT_THRESH" ] && [ "${ABORTED[$CONDITION]}" = 0 ]; then
        echo "  >>> ABORT GATE TRIGGERED for $CONDITION: ${TIMEOUT[$CONDITION]}/$PROBLEMS_SEEN timeouts >= 30% threshold"
        ABORTED[$CONDITION]=1
        # Mark abort in the jsonl trailer
        case "$CONDITION" in
            oneshot) echo '{"ABORTED_FUTILITY":true,"after_problem":'"$PROBLEMS_SEEN"',"timeouts":'"${TIMEOUT[$CONDITION]}"'}' >> "$OUT_ONESHOT" ;;
            n1) echo '{"ABORTED_FUTILITY":true,"after_problem":'"$PROBLEMS_SEEN"',"timeouts":'"${TIMEOUT[$CONDITION]}"'}' >> "$OUT_N1" ;;
            n3) echo '{"ABORTED_FUTILITY":true,"after_problem":'"$PROBLEMS_SEEN"',"timeouts":'"${TIMEOUT[$CONDITION]}"'}' >> "$OUT_N3" ;;
        esac
    fi
}

for i in "${!PROBLEMS[@]}"; do
    NAME="${PROBLEMS[$i]}"
    PROBLEM_FILE="$MINIF2F_DIR/MiniF2F/Test/${NAME}.lean"
    [ ! -f "$PROBLEM_FILE" ] && { echo "[$NAME] MISSING"; continue; }

    ROT_IDX=$((i % 3))
    case "$ROT_IDX" in
        0) ORDER="$ROT_0" ;;
        1) ORDER="$ROT_1" ;;
        2) ORDER="$ROT_2" ;;
    esac

    echo "[$((i+1))/$N] $NAME  (rot=$ROT_IDX: $ORDER)"
    for C in $ORDER; do
        run_one "$C" "$PROBLEM_FILE" "$NAME"
    done

    # Abort gate checks after ABORT_AFTER problems
    if [ $((i+1)) -eq "$ABORT_AFTER" ]; then
        for C in oneshot n1 n3; do
            check_abort_gate "$C" $((i+1))
        done
    fi
done

echo ""
echo "=== SUMMARY ==="
for C in oneshot n1 n3; do
    echo "  $C: solves=${SOLVED[$C]} timeouts=${TIMEOUT[$C]} aborted=${ABORTED[$C]}"
done
echo ""
echo "Run frozen_analysis:"
echo "  python3 $SCRIPT_DIR/frozen_analysis.py \\"
echo "    --sample $SAMPLE \\"
echo "    --oneshot $OUT_ONESHOT --n1 $OUT_N1 --n3 $OUT_N3"
