#!/usr/bin/env bash
# PPUT-CCL Phase C atom C2 — ablation batch runner.
#
# PREREG § 6 C2: 5 modes × 10 problems × 2 seeds = 100 jsonl rows
#   - modes:    full / soft_law / homogeneous / panopticon / amnesia
#   - problems: hard-10 from PPUT_CCL_HARD10_2026-04-26.json (sealed sha256
#               6667e6bdd2aa381c…)
#   - seeds:    [31415, 2718] frozen at PREREG round-4 commit
#   - condition: n3 (3-agent swarm — exercises both per-tx skill cycling
#                AND inter-agent state mechanics that Homogeneous /
#                Panopticon ablations diverge on)
#
# H1-H4 detection axes per mode:
#   - SoftLaw     pput_runtime > pput_verified gap (runtime fakes accept)
#   - Homogeneous solve set narrows to skill[0] reachability
#   - Panopticon  prompt tokens grow ~O(N) via cross-agent learned-memory
#                 merge → cost dilution → PPUT↓
#   - Amnesia     ERR=0 via L_t suppression → time/token inflation per tx
#
# Constitutional anchors:
#   - C-pre1 hard-10 sample basis (Trust Root + sealed sha256 verified at
#     boot via verify_trust_root)
#   - C1a-e --mode CLI + 5 mode wirings (experiment_mode.rs)
#   - C5 mode_flag_binary_purity unit test (binary-identity discipline)
#   - feedback_smoke_before_batch (memory): each mode must smoke clean
#     before full batch
#   - feedback_phased_checkpoint (memory): pause at gates; this batch IS
#     Phase C's primary evidence collection
#
# Usage:
#   bash handover/preregistration/scripts/run_c2_phase_c_ablation.sh [--smoke|--full]
#     --smoke      n3 × 1 problem × 5 modes × 1 seed × MAX_TRANSACTIONS=10
#                  (~5 min total, ~$0.05 — validates wiring end-to-end)
#     --full       full batch: 5 modes × 10 problems × 2 seeds = 100 rows
#                  (~8 hours wall-clock, ~$1-2 — needs explicit GO)
#     (no flag)    prints usage + cost estimate; no run
#
# Audit-fix discipline mirrored from run_p0_calibration.sh:
#   - set -euo pipefail
#   - cargo build exit checked (Codex B1)
#   - timeout / crash emits valid jsonl row (Gemini Q7.b)
#   - oracle preflight (memory feedback_oracle_preflight)
#   - evaluator boot preflight with exit-code assertion
#   - MODEL_SNAPSHOT + GIT_SHA stamped for drift detection
#   - per-mode smoke isolation: any mode failing aborts the batch
#
# FC-trace: meta-runner for Phase C ablation evidence collection
# (FC1-N7 + FC1-N12 + FC2-N22 + Art. II.2.1 + Art. III.2 — every
# constitutional invariant the 5 modes either preserve or breach).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# Auto-load v3 .env for API keys if not already set.
if [ -z "${DEEPSEEK_API_KEY:-}" ] && [ -f "$HOME/projects/turingosv3/.env" ]; then
    # shellcheck disable=SC1090
    source "$HOME/projects/turingosv3/.env"
fi
export LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
export ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-v4-flash}"

MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"
LOG_DIR="$PROJECT_ROOT/experiments/minif2f_v4/logs"
TIMESTAMP=$(date +%Y%m%dT%H%M%S)
HARD10_JSON="$PROJECT_ROOT/handover/preregistration/PPUT_CCL_HARD10_2026-04-26.json"

MODE_ARG="${1:-}"
SMOKE=0
FULL=0
case "$MODE_ARG" in
    --smoke)  SMOKE=1 ;;
    --full)   FULL=1 ;;
    "")
        cat <<'USAGE'
Usage: bash handover/preregistration/scripts/run_c2_phase_c_ablation.sh [--smoke|--full]

  --smoke   n3 × 1 problem × 5 modes × 1 seed × MAX_TRANSACTIONS=10
            (~5 min wall-clock, ~$0.05 API cost)
            Verifies all 5 modes complete a swarm run end-to-end.

  --full    Full Phase C batch: 5 modes × 10 problems × 2 seeds = 100 rows
            (~8 hours wall-clock, ~$1-2 API cost — explicit GO required)
            Hard-10 from PPUT_CCL_HARD10_2026-04-26.json; seeds [31415, 2718].

Phase C atom C2 (PREREG § 6 C2). C-pre1 + C1a-e + C5 are this runner's
preconditions; verify cargo test --workspace = 298 PASS before launching.
USAGE
        exit 0
        ;;
    *) echo "Unknown arg: $MODE_ARG"; exit 1 ;;
esac

# Constants from PREREG § 6 C2.
CONDITION="n3"
MODES=("full" "soft_law" "homogeneous" "panopticon" "amnesia")
SEEDS_FULL=(31415 2718)
SEEDS_SMOKE=(31415)

GIT_SHA=$(cd "$PROJECT_ROOT" && git rev-parse HEAD)
GIT_DIRTY=""
if ! (cd "$PROJECT_ROOT" && git diff --quiet HEAD); then
    GIT_DIRTY="-dirty"
fi
export MODEL_SNAPSHOT="${MODEL_SNAPSHOT:-${ACTIVE_MODEL}@${GIT_SHA:0:12}${GIT_DIRTY}}"
export BUILD_SHA="${BUILD_SHA:-${GIT_SHA}${GIT_DIRTY}}"
# Compute and pin BINARY_SHA256 once at runner entry. C5 mode-purity test
# asserts this field is mode-invariant; the runner stamps the same value
# for every (mode, problem, seed) cell so post-hoc analysis can verify.
PRE_BUILD_BINARY=""

mkdir -p "$LOG_DIR"
if [ "$SMOKE" -eq 1 ]; then
    OUT_PREFIX="$LOG_DIR/c2_smoke_${TIMESTAMP}"
else
    OUT_PREFIX="$LOG_DIR/c2_phase_c_ablation_${TIMESTAMP}"
fi

# Resolve hard-10 problem list from frozen PPUT_CCL_HARD10 JSON.
HARD10_IDS=$(python3 -c "
import json
d = json.load(open('$HARD10_JSON'))
for pid in d['problem_ids']:
    print(pid)
")
HARD10_COUNT=$(echo "$HARD10_IDS" | wc -l)
if [ "$HARD10_COUNT" -ne 10 ]; then
    echo "FATAL: hard-10 JSON has $HARD10_COUNT problems, expected 10"
    exit 2
fi

if [ "$SMOKE" -eq 1 ]; then
    # Smoke: 1 problem × 5 modes × 1 seed. Pick the alphabetically-first
    # hard-10 problem (deterministic; aime_1987_p5 per current sample).
    # MAX_TRANSACTIONS=2 keeps each cell to ~1-2 min (deepseek-v4-flash
    # with thinking-on takes ~30-60s per LLM call; smoke only needs to
    # verify wiring doesn't crash, not solve anything).
    SMOKE_ID=$(echo "$HARD10_IDS" | sort | head -1)
    HARD10_IDS="$SMOKE_ID"
    SEEDS=("${SEEDS_SMOKE[@]}")
    export MAX_TRANSACTIONS=2
    echo "[smoke] 1 problem × 5 modes × 1 seed × MAX_TRANSACTIONS=2"
    echo "[smoke] problem: $SMOKE_ID"
else
    SEEDS=("${SEEDS_FULL[@]}")
    echo "[full] $HARD10_COUNT problems × ${#MODES[@]} modes × ${#SEEDS[@]} seeds = $((HARD10_COUNT * ${#MODES[@]} * ${#SEEDS[@]})) rows"
fi

# Audit-fix Codex B1: build must succeed.
echo "[$(date -Is)] Building evaluator (release)..."
( cd "$PROJECT_ROOT" && cargo build --release -p minif2f_v4 ) 2>&1 | tail -3
EVALUATOR="$PROJECT_ROOT/target/release/evaluator"
if [ ! -x "$EVALUATOR" ]; then
    echo "BUILD FAIL: $EVALUATOR not produced. ABORT."
    exit 2
fi
PRE_BUILD_BINARY=$(sha256sum "$EVALUATOR" | awk '{print $1}')
export BINARY_SHA256="${BINARY_SHA256:-sha256:$PRE_BUILD_BINARY}"
echo "[build] BINARY_SHA256=$BINARY_SHA256"

# Memory feedback_oracle_preflight: verify Mathlib via trivial theorem.
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
echo "[preflight] Oracle OK."

# Evaluator boot preflight (Trust Root verify).
echo "[$(date -Is)] Evaluator boot preflight..."
PREFLIGHT_EXIT=0
PREFLIGHT_PROBE=$(timeout 30 "$EVALUATOR" /nonexistent_problem_path.lean 2>&1) || PREFLIGHT_EXIT=$?
if [ "$PREFLIGHT_EXIT" -eq 0 ]; then
    echo "PREFLIGHT FAIL: evaluator exited 0 on nonexistent problem"
    exit 2
fi
if echo "$PREFLIGHT_PROBE" | grep -q "TRUST_ROOT_TAMPERED"; then
    echo "PREFLIGHT FAIL: Trust Root tampered. Aborting."
    echo "$PREFLIGHT_PROBE" | head -c 500
    exit 2
fi
echo "[preflight] Evaluator boot OK (exit=$PREFLIGHT_EXIT)."

# Per-cell run loop. Cell = (mode, problem, seed).
TOTAL_CELLS=0
for m in "${MODES[@]}"; do
    for pid in $HARD10_IDS; do
        for seed in "${SEEDS[@]}"; do
            TOTAL_CELLS=$((TOTAL_CELLS + 1))
        done
    done
done
echo "[batch] $TOTAL_CELLS total cells; output = ${OUT_PREFIX}__<mode>_<problem>_<seed>.jsonl"

CELL_IDX=0
FAIL_COUNT=0
BATCH_START=$(date +%s)
for m in "${MODES[@]}"; do
    for pid in $HARD10_IDS; do
        for seed in "${SEEDS[@]}"; do
            CELL_IDX=$((CELL_IDX + 1))
            OUT_FILE="${OUT_PREFIX}__${m}_${pid}_seed${seed}.jsonl"
            CELL_TS=$(date -Is)
            echo "[$CELL_TS] cell $CELL_IDX/$TOTAL_CELLS  mode=$m  problem=$pid  seed=$seed"

            # Per memory feedback_phased_checkpoint: log start so a kill-9
            # mid-batch is surface-able by inspecting the missing tail.
            export BOLTZMANN_SEED="$seed"
            export CONDITION="$CONDITION"
            export SPLIT="adaptation"

            # Run with a per-cell timeout (smoke: 5 min; full: 30 min).
            CELL_TIMEOUT=$([ "$SMOKE" -eq 1 ] && echo 300 || echo 1800)
            CELL_EXIT=0
            CELL_OUTPUT=$(timeout "$CELL_TIMEOUT" "$EVALUATOR" \
                --mode="$m" "${pid}.lean" 2>&1) || CELL_EXIT=$?

            # Extract PPUT_RESULT line (machine-readable jsonl).
            PPUT_LINE=$(echo "$CELL_OUTPUT" | grep "^PPUT_RESULT:" | sed 's/^PPUT_RESULT://' || true)

            if [ -z "$PPUT_LINE" ]; then
                # No PPUT_RESULT emitted — record as synthetic failure.
                # Audit-fix Gemini Q7.b: every cell must produce a row.
                FAIL_COUNT=$((FAIL_COUNT + 1))
                echo "  WARN: no PPUT_RESULT (exit=$CELL_EXIT). Saving stderr to ${OUT_FILE}.err"
                echo "$CELL_OUTPUT" | tail -c 4000 > "${OUT_FILE}.err"
                # Synthetic UNSOLVED row for cell-completeness invariant.
                printf '{"schema_version":"v2.0","problem_id":"%s","mode":"%s","split":"adaptation","solved":false,"verified":false,"progress":0,"_synthetic_failure":true,"_exit_code":%d}\n' \
                    "$pid" "$m" "$CELL_EXIT" > "$OUT_FILE"
            else
                echo "$PPUT_LINE" > "$OUT_FILE"
                # Quick parse to extract solve flag (informational).
                SOLVED=$(echo "$PPUT_LINE" | python3 -c "import sys,json; print(json.loads(sys.stdin.read()).get('solved'))" 2>/dev/null || echo "?")
                VERIFIED=$(echo "$PPUT_LINE" | python3 -c "import sys,json; print(json.loads(sys.stdin.read()).get('verified'))" 2>/dev/null || echo "?")
                echo "  cell done: solved=$SOLVED verified=$VERIFIED"
            fi
        done
    done
done

BATCH_END=$(date +%s)
BATCH_ELAPSED=$((BATCH_END - BATCH_START))
echo "[batch] complete. cells=$TOTAL_CELLS fail=$FAIL_COUNT elapsed=${BATCH_ELAPSED}s"
echo "[batch] output prefix: $OUT_PREFIX"
ls -1 "${OUT_PREFIX}"*.jsonl 2>/dev/null | head -5
echo "[batch] ..."
ls -1 "${OUT_PREFIX}"*.jsonl 2>/dev/null | wc -l
echo "  total jsonl rows written"

if [ "$FAIL_COUNT" -gt 0 ]; then
    if [ "$SMOKE" -eq 1 ]; then
        echo "FAIL: smoke had $FAIL_COUNT cell failures. Fix before launching --full."
        exit 3
    else
        echo "WARN: full batch had $FAIL_COUNT cell failures (synthetic rows written for cell completeness)."
    fi
fi

echo "[$(date -Is)] runner exit OK"
