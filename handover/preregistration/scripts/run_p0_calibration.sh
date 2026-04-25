#!/usr/bin/env bash
# PPUT-CCL B7-extra — p_0 calibration runner (audit-fixed 2026-04-25).
#
# PREREG § 5.5 protocol:
#   - control:    evaluator on adaptation-144 × seeds [31415, 2718]
#   - treatment:  same + SIMULATE_ROLLBACK_AT_TX_50=1
#   - 288 + 288 = 576 runs total.
#   - regression_p = 1 iff control SOLVED && treatment UNSOLVED, same (problem, seed)
#   - p_0 = sum_p max_seed(regression_p) / 144
#
# Constitutional anchor: see experiments/minif2f_v4/src/rollback_sim.rs header.
#
# Audit-fix 2026-04-25 (dual VETO):
#   - set -e (Codex B1 + Gemini Q6.a) — any subprocess failure aborts batch
#   - cargo build exit checked (Codex B1)
#   - timeout / crash emits a valid UNSOLVED jsonl row instead of dropping
#     to MEASUREMENT_ERROR (Gemini Q7.b + Codex B2) — strict-completeness
#     of compute_p0.py requires every (problem, seed) pair present
#   - runner invokes compute_p0.py at end with exit-code propagation
#     (Codex B3) — p_0 > 0.10 ceiling triggers ABORT
#   - MODEL_SNAPSHOT + GIT_SHA stamped in env for drift detection
#     (Codex Q7) — feeds into evaluator's existing model_snapshot field
#   - canary timestamps logged at batch start + end
#
# Usage:
#   bash handover/preregistration/scripts/run_p0_calibration.sh [--smoke|--smoke-hard]
#     --smoke        1 mathd_algebra problem × 4 runs (~5 min, ~$0.05) — infra check
#     --smoke-hard   1 aime problem × 2 runs (control + treatment, seed=31415,
#                                             ~20 min, ~$0.20) — toggle-fire check
#     (no flag)      full 576-run batch (~$3-5, ~8h — needs explicit user GO)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# Auto-load v3 .env for API keys if not already set.
if [ -z "${DEEPSEEK_API_KEY:-}" ] && [ -f "$HOME/projects/turingosv3/.env" ]; then
    # shellcheck disable=SC1090
    source "$HOME/projects/turingosv3/.env"
fi
export LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
export ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"

MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"
LOG_DIR="$PROJECT_ROOT/experiments/minif2f_v4/logs"
TIMESTAMP=$(date +%Y%m%dT%H%M%S)
SPLITS_JSON="$PROJECT_ROOT/handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json"

MODE_ARG="${1:-}"
SMOKE=0
SMOKE_HARD=0
case "$MODE_ARG" in
    --smoke)        SMOKE=1 ;;
    --smoke-hard)   SMOKE_HARD=1 ;;
    "")             ;;
    *)              echo "Unknown arg: $MODE_ARG"; exit 1 ;;
esac

# PREREG § 5.5: condition fixed at n3 (3-agent swarm — needs >=50 tx capacity).
# Boltzmann seeds frozen at PREREG values. Audit-fix: no seed override path.
CONDITION="n3"
SEEDS=(31415 2718)
MODES=("control" "treatment")

# Drift-detection provenance (Codex Q7). MODEL_SNAPSHOT seeds the evaluator's
# existing model_snapshot jsonl field; GIT_SHA stamps the build commit.
GIT_SHA=$(cd "$PROJECT_ROOT" && git rev-parse HEAD)
GIT_DIRTY=""
if ! (cd "$PROJECT_ROOT" && git diff --quiet HEAD); then
    GIT_DIRTY="-dirty"
fi
export MODEL_SNAPSHOT="${MODEL_SNAPSHOT:-${ACTIVE_MODEL}@${GIT_SHA:0:12}${GIT_DIRTY}}"
export BUILD_SHA="${BUILD_SHA:-${GIT_SHA}${GIT_DIRTY}}"

mkdir -p "$LOG_DIR"
if [ "$SMOKE" -eq 1 ]; then
    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
elif [ "$SMOKE_HARD" -eq 1 ]; then
    OUT_PREFIX="$LOG_DIR/p0_smoke_hard_${TIMESTAMP}"
else
    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
fi

# Resolve adaptation-144 problem list from frozen splits.
ADAPTATION_IDS=$(python3 -c "
import json
d = json.load(open('$SPLITS_JSON'))
for pid in d['splits']['adaptation']['problem_ids']:
    print(pid)
")

if [ "$SMOKE" -eq 1 ]; then
    SMOKE_ID=$(echo "$ADAPTATION_IDS" | grep "^mathd_algebra" | head -1)
    [ -z "$SMOKE_ID" ] && SMOKE_ID=$(echo "$ADAPTATION_IDS" | head -1)
    ADAPTATION_IDS="$SMOKE_ID"
    echo "[smoke] using single problem: $SMOKE_ID"
elif [ "$SMOKE_HARD" -eq 1 ]; then
    HARD_ID=$(echo "$ADAPTATION_IDS" | grep "^aime_" | head -1)
    [ -z "$HARD_ID" ] && HARD_ID=$(echo "$ADAPTATION_IDS" | tail -1)
    ADAPTATION_IDS="$HARD_ID"
    SEEDS=(31415)  # smoke-hard uses single seed to bound cost
    echo "[smoke-hard] using single problem: $HARD_ID (seed 31415 only)"
fi

# Audit-fix Codex B1: build must succeed; failure aborts.
echo "[$(date -Is)] Building evaluator (release)..."
( cd "$PROJECT_ROOT" && cargo build --release -p minif2f_v4 ) 2>&1 | tail -3
EVALUATOR="$PROJECT_ROOT/target/release/evaluator"
if [ ! -x "$EVALUATOR" ]; then
    echo "BUILD FAIL: $EVALUATOR not produced. ABORT."
    exit 2
fi

# C-012 oracle preflight (memory feedback_oracle_preflight.md).
echo "[$(date -Is)] Oracle preflight..."
LEAN_BIN="${LEAN_BINARY:-$HOME/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean}"
PREFLIGHT_LEAN_PATH=$(find "$MINIF2F_DIR/.lake/packages" \
    \( -path "*/.lake/build/lib/lean" -o -path "*/lib/lean" \) \
    -type d 2>/dev/null | tr '\n' ':')
if [ -z "$PREFLIGHT_LEAN_PATH" ]; then
    echo "PREFLIGHT FAIL: no Mathlib packages under $MINIF2F_DIR/.lake/packages."
    exit 2
fi
PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:\xe2\x84\x9d) + 1 = 2 := by norm_num\n' \
    | LEAN_PATH="$PREFLIGHT_LEAN_PATH" timeout 180 "$LEAN_BIN" --stdin 2>&1) || true
if echo "$PREFLIGHT_OUT" | grep -q "error:"; then
    echo "PREFLIGHT FAIL — Oracle cannot verify trivial theorem. ABORTING."
    echo "$PREFLIGHT_OUT" | head -c 500
    exit 2
fi
echo "Oracle preflight OK."

# Run loop. Each (mode, seed, problem) combination = 1 run.
TOTAL_PROBLEMS=$(echo "$ADAPTATION_IDS" | wc -l)
TOTAL_RUNS=$((TOTAL_PROBLEMS * ${#SEEDS[@]} * ${#MODES[@]}))
CANARY_START=$(date -Is)
echo ""
echo "=== p_0 calibration ==="
echo "Mode count:    ${#MODES[@]} (control + treatment)"
echo "Seed count:    ${#SEEDS[@]} (${SEEDS[*]})"
echo "Problem count: $TOTAL_PROBLEMS"
echo "Total runs:    $TOTAL_RUNS"
echo "MODEL_SNAPSHOT: $MODEL_SNAPSHOT"
echo "BUILD_SHA:     $BUILD_SHA"
echo "Canary start:  $CANARY_START"
echo ""

# Audit-fix Gemini Q7.b: emit a valid UNSOLVED jsonl row on timeout/crash so
# strict-completeness compute_p0 join sees every pair. The synthesized row
# carries the same calibration tags + a `synthetic_timeout_or_crash: true`
# disambiguator so downstream tooling can distinguish a timeout from a real
# UNSOLVED.
emit_synthetic_unsolved() {
    # args: out_file mode seed pid reason exit_code
    python3 - <<EOF >> "$1"
import json, time
print(json.dumps({
    "schema_version": "v2.0",
    "run_id": "synthetic_${2}_${4}_$(date +%s)",
    "problem_id": "$4",
    "solved": False,
    "verified": False,
    "progress": 0,
    "split": "adaptation",
    "calibration_mode": "$2",
    "calibration_seed": $3,
    "calibration_problem_id": "$4",
    "synthetic_timeout_or_crash": True,
    "synthetic_reason": "$5",
    "synthetic_exit_code": $6,
    "model_snapshot": "$MODEL_SNAPSHOT",
    "build_sha": "$BUILD_SHA",
    "boltzmann_seed": $3,
    "tx_count": 0,
    "total_run_token_count": 0,
    "total_wall_time_ms": 0,
    "pput_runtime": 0.0,
    "pput_verified": 0.0,
    "pput_m_verified": 0.0,
    "failed_branch_count": 0,
    "rollback_count": 0,
    "far": 0.0, "err": 0.0, "iac": 0.0, "cpr": 0.0,
    "git_sha": "$BUILD_SHA",
    "binary_sha256": "",
    "mode": "full",
    "problem": "${MINIF2F_DIR}/MiniF2F/Test/${4}.lean",
    "condition": "$CONDITION",
    "model": "$ACTIVE_MODEL",
    "has_golden_path": False,
    "time_secs": 0.0,
    "pput": 0.0,
    "gp_token_count": 0,
    "gp_node_count": 0,
}))
EOF
}

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
                echo "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID — PROBLEM_NOT_FOUND"
                emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "problem_file_missing" 99
                continue
            fi
            echo -n "[$RUN_IDX/$TOTAL_RUNS] $MODE seed=$SEED $PID ... "
            echo "=== $MODE seed=$SEED $PID @ $(date -Is) ===" >> "$STDERR_LOG"
            # Note: `set -e` is bypassed for this single command via `|| EXIT=$?`
            # so timeout/crash flows into the synthetic-UNSOLVED branch instead
            # of aborting the entire batch.
            EXIT=0
            OUTPUT=$(timeout 2400 env \
                CONDITION="$CONDITION" \
                MINIF2F_DIR="$MINIF2F_DIR" \
                BOLTZMANN_SEED="$SEED" \
                SIMULATE_ROLLBACK_AT_TX_50="$ROLLBACK_FLAG" \
                MODEL_SNAPSHOT="$MODEL_SNAPSHOT" \
                BUILD_SHA="$BUILD_SHA" \
                SPLIT="adaptation" \
                RUST_LOG=info \
                "$EVALUATOR" "$PROBLEM" 2>>"$STDERR_LOG") || EXIT=$?
            PPUT_JSON=$(echo "$OUTPUT" | grep "^PPUT_RESULT:" | sed 's/^PPUT_RESULT://' | head -1 || true)
            if [ -n "$PPUT_JSON" ] && [ "$EXIT" -eq 0 ]; then
                ENRICHED=$(printf '%s' "$PPUT_JSON" | MODE_ENV="$MODE" SEED_ENV="$SEED" PID_ENV="$PID" python3 -c "
import json, os, sys
row = json.loads(sys.stdin.read())
row['calibration_mode'] = os.environ['MODE_ENV']
row['calibration_seed'] = int(os.environ['SEED_ENV'])
row['calibration_problem_id'] = os.environ['PID_ENV']
print(json.dumps(row))
")
                echo "$ENRICHED" >> "$OUT_FILE"
                TX=$(echo "$ENRICHED" | python3 -c "import sys,json; print(json.load(sys.stdin).get('tx_count', 0))")
                SOLVED_FLAG=$(echo "$ENRICHED" | python3 -c "import sys,json; r=json.load(sys.stdin); print(int(r.get('progress', 1 if r.get('has_golden_path') else 0)))")
                if [ "$SOLVED_FLAG" = "1" ]; then
                    echo "SOLVED (tx=$TX)"
                else
                    echo "UNSOLVED (tx=$TX)"
                fi
            else
                # Audit-fix Gemini Q7.b: emit valid UNSOLVED row instead of drop.
                if [ "$EXIT" -eq 124 ]; then
                    echo "TIMEOUT (exit=124) — emitting synthetic UNSOLVED row"
                    emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "timeout_2400s" 124
                else
                    echo "CRASH (exit=$EXIT) — emitting synthetic UNSOLVED row"
                    emit_synthetic_unsolved "$OUT_FILE" "$MODE" "$SEED" "$PID" "evaluator_crash" "$EXIT"
                fi
            fi
        done <<< "$ADAPTATION_IDS"
    done
done

CANARY_END=$(date -Is)
BATCH_END=$(date +%s)
WALL_TIME=$((BATCH_END - BATCH_START))

echo ""
echo "╔═══════════════════════════════════════════╗"
echo "║   p_0 CALIBRATION SUMMARY"
echo "╠═══════════════════════════════════════════╣"
echo "║ Wall time:        ${WALL_TIME}s"
echo "║ Canary start:     $CANARY_START"
echo "║ Canary end:       $CANARY_END"
echo "║ MODEL_SNAPSHOT:   $MODEL_SNAPSHOT"
echo "║ BUILD_SHA:        $BUILD_SHA"
echo "║ Control jsonl:    ${OUT_PREFIX}_control.jsonl"
echo "║ Treatment jsonl:  ${OUT_PREFIX}_treatment.jsonl"
echo "╚═══════════════════════════════════════════╝"
echo ""

# Audit-fix Codex B3: invoke compute_p0 + propagate exit code (p_0 > 0.10
# = ceiling abort). For smoke modes we skip the estimator (sample size too
# small to be meaningful) and just print the diagnostic snippet.
if [ "$SMOKE" -eq 1 ] || [ "$SMOKE_HARD" -eq 1 ]; then
    echo "[smoke mode] skipping compute_p0 estimator (sample size <144)."
    echo "Diagnostic: head 1 row from each jsonl"
    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
    exit 0
fi

# Full batch: estimator MUST run, exit code MUST propagate.
echo "[$(date -Is)] Running p_0 estimator (strict-complete mode)..."
P0_JSON="${OUT_PREFIX}_p0_result.json"
set +e
python3 "$SCRIPT_DIR/compute_p0.py" \
    --control "${OUT_PREFIX}_control.jsonl" \
    --treatment "${OUT_PREFIX}_treatment.jsonl" \
    --out-json "$P0_JSON"
P0_EXIT=$?
set -e

if [ "$P0_EXIT" -eq 0 ]; then
    echo ""
    echo "✓ p_0 PASSED ceiling. Result: $P0_JSON"
    echo "  Next: ArchitectAI updates genesis_payload.toml [pput_accounting_0]"
    echo "        + Trust Root manifest entry for the calibration jsonl."
elif [ "$P0_EXIT" -eq 2 ]; then
    echo ""
    echo "✗ p_0 EXCEEDS ceiling (>0.10) — PREREG § 5.5 ABORT."
    echo "  Calibration result NOT frozen into genesis_payload.toml."
    echo "  Action: redesign rollback simulation (per PREREG § 5.5), redo."
    exit 2
else
    echo ""
    echo "✗ compute_p0.py errored (exit=$P0_EXIT) — see stderr above."
    echo "  Calibration result NOT frozen. Investigate before retry."
    exit "$P0_EXIT"
fi
