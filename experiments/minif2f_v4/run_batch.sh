#!/usr/bin/env bash
# MiniF2F v4 Batch Runner — PPUT-Optimized Auto-Research Inner Loop
#
# Sole metric: PPUT (Progress Per Unit Time)
#   PPUT = 100% / time_to_omega if GP exists, 0 otherwise
#   Problems with PPUT=0 are logged but not prioritized
#
# Usage: ./run_batch.sh [oneshot|n1|n3] [test|valid|all]
#
# Prerequisites:
#   export DEEPSEEK_API_KEY=...
#   export LLM_PROXY_URL=http://localhost:8080
#   export ACTIVE_MODEL=deepseek-reasoner

set -euo pipefail

CONDITION="${1:-oneshot}"
SPLIT="${2:-test}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"
LOG_DIR="$SCRIPT_DIR/logs"
TIMESTAMP=$(date +%Y%m%dT%H%M%S)
RESULTS_FILE="$LOG_DIR/pput_${CONDITION}_${SPLIT}_${TIMESTAMP}.jsonl"

mkdir -p "$LOG_DIR"

echo "=== MiniF2F v4 — PPUT Batch Runner ==="
echo "Condition: $CONDITION | Split: $SPLIT"
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
(cd "$PROJECT_ROOT" && cargo build --release -p minif2f_v4 2>&1 | tail -1)
EVALUATOR="$PROJECT_ROOT/target/release/evaluator"

BATCH_START=$(date +%s)
TOTAL=0
SOLVED=0
PPUT_SUM="0"

for DIR in "${PROBLEM_DIRS[@]}"; do
    for PROBLEM in "$DIR"/*.lean; do
        BASENAME=$(basename "$PROBLEM")
        TOTAL=$((TOTAL + 1))

        echo -n "[$TOTAL] $BASENAME ... "

        # Run evaluator, extract PPUT_RESULT line
        OUTPUT=$(CONDITION="$CONDITION" MINIF2F_DIR="$MINIF2F_DIR" \
            RUST_LOG=info "$EVALUATOR" "$PROBLEM" 2>/dev/null || true)

        PPUT_JSON=$(echo "$OUTPUT" | grep "^PPUT_RESULT:" | sed 's/^PPUT_RESULT://' | head -1)

        if [ -n "$PPUT_JSON" ]; then
            echo "$PPUT_JSON" >> "$RESULTS_FILE"

            HAS_GP=$(echo "$PPUT_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin)['has_golden_path'])" 2>/dev/null || echo "False")
            PPUT_VAL=$(echo "$PPUT_JSON" | python3 -c "import sys,json; print(f\"{json.load(sys.stdin)['pput']:.2f}\")" 2>/dev/null || echo "0")

            if [ "$HAS_GP" = "True" ]; then
                SOLVED=$((SOLVED + 1))
                PPUT_SUM=$(echo "$PPUT_SUM + $PPUT_VAL" | bc)
                echo "SOLVED (PPUT=${PPUT_VAL}%/s)"
            else
                echo "PPUT=0"
            fi
        else
            echo '{"problem":"'"$BASENAME"'","has_golden_path":false,"pput":0,"time_secs":0,"tx_count":0}' >> "$RESULTS_FILE"
            echo "ERROR (no output)"
        fi
    done
done

BATCH_END=$(date +%s)
WALL_TIME=$((BATCH_END - BATCH_START))

echo ""
echo "╔══════════════════════════════════════╗"
echo "║     PPUT BATCH SUMMARY               ║"
echo "╠══════════════════════════════════════╣"
echo "║ Total problems:  $TOTAL"
echo "║ GP found (solved): $SOLVED"
echo "║ PPUT=0 (no GP):   $((TOTAL - SOLVED))"
echo "║ Σ PPUT:           ${PPUT_SUM}%/s"
if [ "$SOLVED" -gt 0 ]; then
    AVG_PPUT=$(echo "scale=2; $PPUT_SUM / $SOLVED" | bc)
    echo "║ Avg PPUT (solved): ${AVG_PPUT}%/s"
fi
echo "║ Wall time:        ${WALL_TIME}s"
echo "║ Aggregate PPUT:   $(echo "scale=4; $PPUT_SUM / $WALL_TIME" | bc 2>/dev/null || echo "N/A")%/s²"
echo "╚══════════════════════════════════════╝"
echo ""
echo "Results: $RESULTS_FILE"

# Export for history tracker
python3 "$SCRIPT_DIR/history.py" --export 2>/dev/null || true
