#!/usr/bin/env bash
# PPUT-CCL B7-extra — p_0 calibration runner.
#
# PREREG § 5.5 protocol:
#   - control:    evaluator on adaptation-144 × seeds [31415, 2718]
#   - treatment:  same + SIMULATE_ROLLBACK_AT_TX_50=1
#   - 288 + 288 = 576 runs total.
#   - regression_p = 1 iff control SOLVED && treatment UNSOLVED, same (problem, seed)
#   - p_0 = sum_p max_seed(regression_p) / 144
#
# Constitutional anchor (TRACE_MATRIX_v1 § 2):
#   treatment runs route through the existing FC1-E18 (∏p=0 → Q_t)
#   semantics — see experiments/minif2f_v4/src/rollback_sim.rs header.
#
# Usage:
#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke]
#
#   --smoke  run 1 problem × 2 seeds × 2 modes = 4 runs (~5 min, ~$0.05)
#            for pre-batch verification per feedback_smoke_before_batch.md
#   (no flag) full 576-run batch (~8h, ~$3-5 — needs explicit user GO)
#
# Prerequisites (same as run_batch.sh):
#   export DEEPSEEK_API_KEY=...
#   export LLM_PROXY_URL=http://localhost:8080  (default)
#   export ACTIVE_MODEL=deepseek-chat           (default)

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# Auto-load v3 .env for API keys if not already set
if [ -z "${DEEPSEEK_API_KEY:-}" ] && [ -f "$HOME/projects/turingosv3/.env" ]; then
    source "$HOME/projects/turingosv3/.env"
fi
export LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
export ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"

MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"
LOG_DIR="$PROJECT_ROOT/experiments/minif2f_v4/logs"
TIMESTAMP=$(date +%Y%m%dT%H%M%S)
SPLITS_JSON="$PROJECT_ROOT/handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json"

SMOKE=0
if [ "${1:-}" = "--smoke" ]; then
    SMOKE=1
fi

# PREREG § 5.5: condition fixed at n3 (3-agent swarm — needs >=50 tx capacity).
# Boltzmann seeds frozen at PREREG values.
CONDITION="n3"
SEEDS=(31415 2718)
MODES=("control" "treatment")

mkdir -p "$LOG_DIR"

if [ "$SMOKE" -eq 1 ]; then
    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
else
    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
fi

# Resolve adaptation-144 problem list from frozen splits.
# Each problem ID maps to <MINIF2F_DIR>/MiniF2F/Test/<id>.lean.
ADAPTATION_IDS=$(python3 -c "
import json
d = json.load(open('$SPLITS_JSON'))
for pid in d['splits']['adaptation']['problem_ids']:
    print(pid)
")

if [ "$SMOKE" -eq 1 ]; then
    # Smoke: pick one short mathd_algebra problem (typically solved in <50 tx).
    SMOKE_ID=$(echo "$ADAPTATION_IDS" | grep "^mathd_algebra" | head -1)
    if [ -z "$SMOKE_ID" ]; then
        SMOKE_ID=$(echo "$ADAPTATION_IDS" | head -1)
    fi
    ADAPTATION_IDS="$SMOKE_ID"
    echo "[smoke] using single problem: $SMOKE_ID"
fi

# Build evaluator (release).
echo "Building evaluator (release)..."
(cd "$PROJECT_ROOT" && cargo build --release -p minif2f_v4 2>&1 | tail -1)
EVALUATOR="$PROJECT_ROOT/target/release/evaluator"

# C-012 oracle preflight (memory feedback_oracle_preflight.md).
echo "Oracle preflight..."
LEAN_BIN="${LEAN_BINARY:-$HOME/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean}"
PREFLIGHT_LEAN_PATH=$(find "$MINIF2F_DIR/.lake/packages" \
    \( -path "*/.lake/build/lib/lean" -o -path "*/lib/lean" \) \
    -type d 2>/dev/null | tr '\n' ':')
if [ -z "$PREFLIGHT_LEAN_PATH" ]; then
    echo "PREFLIGHT FAIL: no Mathlib packages under $MINIF2F_DIR/.lake/packages."
    exit 2
fi
PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:\xe2\x84\x9d) + 1 = 2 := by norm_num\n' \
    | LEAN_PATH="$PREFLIGHT_LEAN_PATH" timeout 180 "$LEAN_BIN" --stdin 2>&1)
PREFLIGHT_CODE=$?
if [ "$PREFLIGHT_CODE" -ne 0 ] || echo "$PREFLIGHT_OUT" | grep -q "error:"; then
    echo "PREFLIGHT FAIL — Oracle cannot verify trivial theorem. ABORTING."
    echo "$PREFLIGHT_OUT" | head -c 500
    exit 2
fi
echo "Oracle preflight OK."

# Run loop. Each (mode, seed, problem) combination = 1 run.
TOTAL_PROBLEMS=$(echo "$ADAPTATION_IDS" | wc -l)
TOTAL_RUNS=$((TOTAL_PROBLEMS * ${#SEEDS[@]} * ${#MODES[@]}))
echo ""
echo "=== p_0 calibration ==="
echo "Mode count:    ${#MODES[@]} (control + treatment)"
echo "Seed count:    ${#SEEDS[@]} (${SEEDS[*]})"
echo "Problem count: $TOTAL_PROBLEMS"
echo "Total runs:    $TOTAL_RUNS"
echo ""

BATCH_START=$(date +%s)
RUN_IDX=0
for MODE in "${MODES[@]}"; do
    OUT_FILE="${OUT_PREFIX}_${MODE}.jsonl"
    STDERR_LOG="${OUT_PREFIX}_${MODE}.stderr.log"
    : > "$OUT_FILE"
    : > "$STDERR_LOG"
    case "$MODE" in
        control)   ROLLBACK_FLAG="" ;;
        treatment) ROLLBACK_FLAG="1" ;;
    esac
    for SEED in "${SEEDS[@]}"; do
        while IFS= read -r PID; do
            [ -z "$PID" ] && continue
            RUN_IDX=$((RUN_IDX + 1))
            PROBLEM="$MINIF2F_DIR/MiniF2F/Test/${PID}.lean"
            if [ ! -f "$PROBLEM" ]; then
                echo "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID — PROBLEM_NOT_FOUND, skip"
                continue
            fi
            echo -n "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID ... "
            echo "=== $MODE seed=$SEED $PID @ $(date -Is) ===" >> "$STDERR_LOG"
            OUTPUT=$(timeout 2400 env \
                CONDITION="$CONDITION" \
                MINIF2F_DIR="$MINIF2F_DIR" \
                BOLTZMANN_SEED="$SEED" \
                SIMULATE_ROLLBACK_AT_TX_50="$ROLLBACK_FLAG" \
                RUST_LOG=info \
                "$EVALUATOR" "$PROBLEM" 2>>"$STDERR_LOG") || true
            PPUT_JSON=$(echo "$OUTPUT" | grep "^PPUT_RESULT:" | sed 's/^PPUT_RESULT://' | head -1)
            if [ -n "$PPUT_JSON" ]; then
                # Stamp mode + seed + problem_id for downstream pairing analysis
                # (PREREG § 5.5 estimator does control-vs-treatment join on
                # (problem, seed)).
                ENRICHED=$(echo "$PPUT_JSON" | python3 -c "
import json, sys
row = json.loads(sys.stdin.read())
row['calibration_mode'] = '$MODE'
row['calibration_seed'] = $SEED
row['calibration_problem_id'] = '$PID'
print(json.dumps(row))
")
                echo "$ENRICHED" >> "$OUT_FILE"
                HAS_GP=$(echo "$ENRICHED" | python3 -c "import sys,json; print(json.load(sys.stdin).get('has_golden_path', False))")
                TX=$(echo "$ENRICHED" | python3 -c "import sys,json; print(json.load(sys.stdin).get('tx_count', 0))")
                if [ "$HAS_GP" = "True" ]; then
                    echo "SOLVED (tx=$TX)"
                else
                    echo "UNSOLVED (tx=$TX)"
                fi
            else
                echo "MEASUREMENT_ERROR"
            fi
        done <<< "$ADAPTATION_IDS"
    done
done

BATCH_END=$(date +%s)
WALL_TIME=$((BATCH_END - BATCH_START))

echo ""
echo "╔═══════════════════════════════════════════╗"
echo "║   p_0 CALIBRATION SUMMARY"
echo "╠═══════════════════════════════════════════╣"
echo "║ Wall time:      ${WALL_TIME}s"
echo "║ Control jsonl:  ${OUT_PREFIX}_control.jsonl"
echo "║ Treatment jsonl: ${OUT_PREFIX}_treatment.jsonl"
echo "╚═══════════════════════════════════════════╝"
echo ""
if [ "$SMOKE" -eq 1 ]; then
    echo "Smoke complete. Verify (1) treatment row tx_count == 50 if it would have"
    echo "exceeded 50, (2) both rows parse via RunRecord::V2, (3) calibration_mode +"
    echo "calibration_seed + calibration_problem_id are present. Then re-run without"
    echo "--smoke for the full 576-run batch."
else
    echo "Compute p_0:"
    echo "  python3 $SCRIPT_DIR/compute_p0.py \\"
    echo "    --control ${OUT_PREFIX}_control.jsonl \\"
    echo "    --treatment ${OUT_PREFIX}_treatment.jsonl"
    echo ""
    echo "(compute_p0.py is the next deliverable — to be written before the full batch lands.)"
fi
