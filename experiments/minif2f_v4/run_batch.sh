#!/usr/bin/env bash
# MiniF2F v4 Batch Runner — Auto-Research Inner Loop
# Usage: ./run_batch.sh [oneshot|n1|n3] [test|valid|all]
#
# Prerequisites:
#   export DEEPSEEK_API_KEY=...
#   export LLM_PROXY_URL=http://localhost:8080  (or direct API URL)
#   export ACTIVE_MODEL=deepseek-reasoner
#
# The script runs the evaluator on all problems in the specified split,
# logs results to experiments/minif2f_v4/logs/, and produces a summary.

set -euo pipefail

CONDITION="${1:-oneshot}"
SPLIT="${2:-test}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"
LOG_DIR="$SCRIPT_DIR/logs"
RESULTS_FILE="$LOG_DIR/batch_${CONDITION}_${SPLIT}_$(date +%Y%m%dT%H%M%S).jsonl"

mkdir -p "$LOG_DIR"

echo "=== MiniF2F v4 Batch Runner ==="
echo "Condition: $CONDITION"
echo "Split: $SPLIT"
echo "Results: $RESULTS_FILE"
echo ""

# Determine problem directory
case "$SPLIT" in
    test)  PROBLEM_DIRS=("$MINIF2F_DIR/MiniF2F/Test") ;;
    valid) PROBLEM_DIRS=("$MINIF2F_DIR/MiniF2F/Valid") ;;
    all)   PROBLEM_DIRS=("$MINIF2F_DIR/MiniF2F/Test" "$MINIF2F_DIR/MiniF2F/Valid") ;;
    *)     echo "Unknown split: $SPLIT (use test, valid, or all)"; exit 1 ;;
esac

# Build evaluator
echo "Building evaluator..."
(cd "$PROJECT_ROOT" && cargo build --release -p minif2f_v4 2>&1 | tail -1)
EVALUATOR="$PROJECT_ROOT/target/release/evaluator"

TOTAL=0
SOLVED=0
ERRORS=0

for DIR in "${PROBLEM_DIRS[@]}"; do
    for PROBLEM in "$DIR"/*.lean; do
        BASENAME=$(basename "$PROBLEM")
        TOTAL=$((TOTAL + 1))

        echo -n "[$TOTAL] $BASENAME ... "

        # Run evaluator, capture output
        RESULT=$(CONDITION="$CONDITION" MINIF2F_DIR="$MINIF2F_DIR" \
            RUST_LOG=info "$EVALUATOR" "$PROBLEM" 2>&1 || true)

        if echo "$RESULT" | grep -q "OmegaAccepted"; then
            STATUS="solved"
            SOLVED=$((SOLVED + 1))
            echo "SOLVED"
        elif echo "$RESULT" | grep -q "OmegaError"; then
            STATUS="error"
            ERRORS=$((ERRORS + 1))
            echo "ERROR"
        else
            STATUS="unsolved"
            echo "unsolved"
        fi

        # Log result
        echo "{\"problem\":\"$BASENAME\",\"status\":\"$STATUS\",\"condition\":\"$CONDITION\"}" >> "$RESULTS_FILE"
    done
done

echo ""
echo "=== BATCH RESULTS ==="
echo "Total: $TOTAL"
echo "Solved: $SOLVED"
echo "Errors: $ERRORS"
echo "Unsolved: $((TOTAL - SOLVED - ERRORS))"
echo "Solve rate: $(echo "scale=1; $SOLVED * 100 / $TOTAL" | bc)%"
echo "Results saved to: $RESULTS_FILE"
