#!/usr/bin/env bash
# MiniF2F v4 Batch Runner — PPUT-Optimized Auto-Research Inner Loop
#
# Sole metric: PPUT (Progress Per Unit Time)
#   PPUT = 100% / time_to_omega if GP exists, 0 otherwise
#   Problems with PPUT=0 are logged but not prioritized
#
# Usage: ./run_batch.sh [oneshot|n1|n3] [test|valid|all]
#
# PPUT strategy: sort problems by expected difficulty (easy first).
#   mathd → algebra/numtheory → amc → induction → aime → imo
#   This maximizes early PPUT yield and identifies solvable problems fast.
#
# Resume: if RESUME_FROM=<results.jsonl>, skips already-completed problems.
#
# Prerequisites:
#   export DEEPSEEK_API_KEY=...
#   export LLM_PROXY_URL=http://localhost:8080  (default)
#   export ACTIVE_MODEL=deepseek-chat           (default — TuringOS IS the CoT, project memory chat_over_reasoner)
#
# Auto-loads v3 .env if DEEPSEEK_API_KEY not set

set -uo pipefail
# Note: no set -e — batch runner must survive individual problem failures

# Auto-load v3 .env for API keys if not already set
if [ -z "${DEEPSEEK_API_KEY:-}" ] && [ -f "$HOME/projects/turingosv3/.env" ]; then
    source "$HOME/projects/turingosv3/.env"
fi
export LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
export ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"

CONDITION="${1:-oneshot}"
SPLIT="${2:-test}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"
LOG_DIR="$SCRIPT_DIR/logs"
TIMESTAMP=$(date +%Y%m%dT%H%M%S)
RESULTS_FILE="$LOG_DIR/pput_${CONDITION}_${SPLIT}_${TIMESTAMP}.jsonl"
STDERR_LOG="$LOG_DIR/stderr_${CONDITION}_${SPLIT}_${TIMESTAMP}.log"

# Resume support: copy previous results and skip those problems
RESUME_FROM="${RESUME_FROM:-}"
SKIP_SET=""
if [ -n "$RESUME_FROM" ] && [ -f "$RESUME_FROM" ]; then
    cp "$RESUME_FROM" "$RESULTS_FILE"
    SKIP_SET=$(python3 -c "
import json, sys
with open('$RESUME_FROM') as f:
    for line in f:
        try:
            d = json.loads(line)
            print(d['problem'].split('/')[-1])
        except: pass
" 2>/dev/null || true)
    SKIP_COUNT=$(echo "$SKIP_SET" | grep -c . || echo 0)
    echo "Resuming from $RESUME_FROM ($SKIP_COUNT problems already done)"
fi

mkdir -p "$LOG_DIR"

echo "=== MiniF2F v4 — PPUT Batch Runner ==="
echo "Condition: $CONDITION | Split: $SPLIT | Strategy: easy-first"
echo "Results: $RESULTS_FILE"
echo ""

# Problem directories
case "$SPLIT" in
    test)  PROBLEM_DIRS=("$MINIF2F_DIR/MiniF2F/Test") ;;
    valid) PROBLEM_DIRS=("$MINIF2F_DIR/MiniF2F/Valid") ;;
    all)   PROBLEM_DIRS=("$MINIF2F_DIR/MiniF2F/Test" "$MINIF2F_DIR/MiniF2F/Valid") ;;
    *)     echo "Unknown split: $SPLIT"; exit 1 ;;
esac

# Build
echo "Building evaluator (release)..."
(cd "$PROJECT_ROOT" && CARGO_TARGET_DIR="$PROJECT_ROOT/target" cargo build --release --manifest-path "$PROJECT_ROOT/experiments/minif2f_v4/Cargo.toml" 2>&1 | tail -1)
EVALUATOR="$PROJECT_ROOT/target/release/evaluator"

# C-012 preflight: oracle health check before burning API budget.
# Catches missing Mathlib, toolchain mismatch, or sandbox regressions up front.
echo "Oracle preflight..."
LEAN_BIN="${LEAN_BINARY:-$HOME/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean}"
PREFLIGHT_LEAN_PATH=$(find "$MINIF2F_DIR/.lake/packages" \
    \( -path "*/.lake/build/lib/lean" -o -path "*/lib/lean" \) \
    -type d 2>/dev/null | tr '\n' ':')
if [ -z "$PREFLIGHT_LEAN_PATH" ]; then
    echo "PREFLIGHT FAIL: no Mathlib packages under $MINIF2F_DIR/.lake/packages. Run 'lake update && lake exe cache get'."; exit 2
fi
PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:ℝ) + 1 = 2 := by norm_num\n' \
    | LEAN_PATH="$PREFLIGHT_LEAN_PATH" timeout 180 "$LEAN_BIN" --stdin 2>&1)
PREFLIGHT_CODE=$?
if [ "$PREFLIGHT_CODE" -ne 0 ] || echo "$PREFLIGHT_OUT" | grep -q "error:"; then
    echo "PREFLIGHT FAIL (exit=$PREFLIGHT_CODE): $PREFLIGHT_OUT" | head -c 500
    echo ""
    echo "Oracle cannot verify a trivially-true theorem — ABORTING batch."; exit 2
fi
echo "Oracle preflight OK."

# Collect all problems and sort by difficulty (easy first)
ALL_PROBLEMS=()
for DIR in "${PROBLEM_DIRS[@]}"; do
    for PROBLEM in "$DIR"/*.lean; do
        ALL_PROBLEMS+=("$PROBLEM")
    done
done

# Sort: mathd first, then algebra/numtheory, then amc, then hard stuff last
SORTED_PROBLEMS=$(printf '%s\n' "${ALL_PROBLEMS[@]}" | python3 -c "
import sys

priority = {
    'mathd_algebra': 0,
    'mathd_numbertheory': 1,
    'algebra': 2,
    'numbertheory': 3,
    'amc': 4,
    'induction': 5,
    'aime': 6,
    'imo': 7,
}

def get_priority(path):
    name = path.strip().split('/')[-1].lower()
    for prefix, p in sorted(priority.items(), key=lambda x: -len(x[0])):
        if name.startswith(prefix):
            return (p, name)
    return (99, name)

lines = [l.strip() for l in sys.stdin if l.strip()]
lines.sort(key=get_priority)
for l in lines:
    print(l)
")

BATCH_START=$(date +%s)
TOTAL=0
SOLVED=0
SKIPPED=0
PPUT_SUM="0"

while IFS= read -r PROBLEM; do
    BASENAME=$(basename "$PROBLEM")
    TOTAL=$((TOTAL + 1))

    # Skip if already completed (resume mode)
    if [ -n "$SKIP_SET" ] && echo "$SKIP_SET" | grep -qF "$BASENAME"; then
        SKIPPED=$((SKIPPED + 1))
        continue
    fi

    echo -n "[$TOTAL/$((TOTAL + SKIPPED))] $BASENAME ... "

    # C-017: stderr archived, never discarded (Art. II.1 broadcast errors)
    echo "=== $BASENAME @ $(date -Is) ===" >> "$STDERR_LOG"
    OUTPUT=$(timeout 2400 env CONDITION="$CONDITION" MINIF2F_DIR="$MINIF2F_DIR" \
        RUST_LOG=info "$EVALUATOR" "$PROBLEM" 2>>"$STDERR_LOG") || true
    EXIT_CODE=$?

    PPUT_JSON=$(echo "$OUTPUT" | grep "^PPUT_RESULT:" | sed 's/^PPUT_RESULT://' | head -1)

    if [ -n "$PPUT_JSON" ]; then
        echo "$PPUT_JSON" >> "$RESULTS_FILE"

        HAS_GP=$(echo "$PPUT_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin)['has_golden_path'])" 2>/dev/null || echo "False")
        PPUT_VAL=$(echo "$PPUT_JSON" | python3 -c "import sys,json; print(f\"{json.load(sys.stdin)['pput']:.2f}\")" 2>/dev/null || echo "0")

        if [ "$HAS_GP" = "True" ]; then
            SOLVED=$((SOLVED + 1))
            PPUT_SUM=$(python3 -c "print($PPUT_SUM + $PPUT_VAL)")
            echo "SOLVED (PPUT=${PPUT_VAL}%/s)"
        else
            echo "PPUT=0"
        fi
    else
        # C-012: measurement failure ≠ verified failure. Do NOT pollute jsonl.
        # Resume will re-attempt this problem next run.
        echo "MEASUREMENT_ERROR (exit=$EXIT_CODE, see $(basename "$STDERR_LOG"))"
    fi
done <<< "$SORTED_PROBLEMS"

BATCH_END=$(date +%s)
WALL_TIME=$((BATCH_END - BATCH_START))
WALL_TIME=$((WALL_TIME > 0 ? WALL_TIME : 1))

echo ""
echo "╔══════════════════════════════════════╗"
echo "║     PPUT BATCH SUMMARY               ║"
echo "╠══════════════════════════════════════╣"
echo "║ Total problems:  $TOTAL (skipped: $SKIPPED)"
echo "║ GP found (solved): $SOLVED"
echo "║ PPUT=0 (no GP):   $((TOTAL - SOLVED - SKIPPED))"
echo "║ Σ PPUT:           ${PPUT_SUM}%/s"
if [ "$SOLVED" -gt 0 ]; then
    AVG_PPUT=$(python3 -c "print(f'{$PPUT_SUM / $SOLVED:.2f}')")
    echo "║ Avg PPUT (solved): ${AVG_PPUT}%/s"
fi
echo "║ Wall time:        ${WALL_TIME}s"
echo "║ Aggregate PPUT:   $(python3 -c "print(f'{$PPUT_SUM / $WALL_TIME:.4f}')" 2>/dev/null || echo "N/A")%/s²"
echo "╚══════════════════════════════════════╝"
echo ""
echo "Results: $RESULTS_FILE"

# Export for history tracker
python3 "$SCRIPT_DIR/history.py" --export 2>/dev/null || true
