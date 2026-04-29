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
HALF=0
FULL=0
case "$MODE_ARG" in
    --smoke)  SMOKE=1 ;;
    --half)   HALF=1 ;;
    --full)   FULL=1 ;;
    "")
        cat <<'USAGE'
Usage: bash handover/preregistration/scripts/run_c2_phase_c_ablation.sh [--smoke|--half|--full]

  --smoke   n3 × 1 problem × 5 modes × 1 seed × MAX_TRANSACTIONS=2
            (~90s wall-clock at thinking-off; ~$0.05 API cost)
            Pipeline-liveness check: verifies all 5 modes complete a swarm
            run end-to-end. Catches harness breakage; does NOT measure
            scientific signal.

  --half    n3 × 3 problems × 5 modes × 1 seed × MAX_TRANSACTIONS=20
            (~10-15 min wall-clock at thinking-off; ~$0.20-0.40 API cost)
            Half-real regression: catches architectural changes that break
            solve rate / PPUT structure on 3 hard-10 problems. Use at
            atom-bundle phase ends as a meaningful regression check
            between cheap --smoke and the full Phase C batch.

  --full    Full Phase C batch: 5 modes × 10 problems × 2 seeds = 100 rows
            Hard-10 from PPUT_CCL_HARD10_2026-04-26.json; seeds [31415, 2718].

Concurrency:
  CONCURRENCY env var sets the parallel-cell pool size (default 1 = serial).
  Recommended on 4-core hardware (this machine):
    CONCURRENCY=1  serial — ~25-50 hr wall-clock, 1 DeepSeek key fine
    CONCURRENCY=2  ~12-25 hr; 1 key safe (~0.6 RPS aggregate LLM)
    CONCURRENCY=4  ~6-13 hr; needs >=2 DeepSeek keys for rate-limit margin
                   (proxy round-robins across DEEPSEEK_API_KEY +
                   DEEPSEEK_API_KEY_SECONDARY) and saturates 4 cores
  Examples:
    CONCURRENCY=2 LLM_PROXY_URL=http://localhost:18080 \\
      bash handover/preregistration/scripts/run_c2_phase_c_ablation.sh --full
    CONCURRENCY=4 LLM_PROXY_URL=http://localhost:18080 \\
      bash handover/preregistration/scripts/run_c2_phase_c_ablation.sh --full

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
elif [ "$HALF" -eq 1 ]; then
    OUT_PREFIX="$LOG_DIR/c2_half_${TIMESTAMP}"
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
elif [ "$HALF" -eq 1 ]; then
    # Half: 3 problems × 5 modes × 1 seed × MAX_TRANSACTIONS=20.
    # Picks the alphabetically-first 3 hard-10 problems. MAX_TX=20 lets
    # each cell exercise real solve attempts (vs MAX_TX=2 in --smoke
    # which only verifies plumbing). Catches architectural regressions
    # that change solve rate / PPUT structure without committing to the
    # full 100-cell ~12hr Phase C batch.
    HARD10_IDS=$(echo "$HARD10_IDS" | sort | head -3)
    SEEDS=("${SEEDS_SMOKE[@]}")
    export MAX_TRANSACTIONS=20
    echo "[half] 3 problems × 5 modes × 1 seed × MAX_TRANSACTIONS=20"
    echo "[half] problems:"
    echo "$HARD10_IDS" | sed 's/^/[half]   /'
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

# Per-cell run loop. Cell = (mode, problem, seed). Cells run independently;
# CONCURRENCY env var enables parallel-pool execution (default 1 = serial,
# matches pre-parallel behavior).
#
# Concurrency notes (4-core test machine, 2026-04-26 measurement):
#   - Each cell = 1 evaluator process + 1 Lean child + N LLM calls
#   - Lean verify dominates (~88% of cell wall-clock at thinking-off)
#   - K=1: serial (matches Path A serial overnight estimate)
#   - K=2: safe with 1 DeepSeek key (~0.6 RPS aggregate, well under typical
#          150 RPM single-key limit)
#   - K=4: saturates 4 CPU cores; needs ≥2 DeepSeek keys for rate-limit margin
#          (round-robin across keys handled by llm_proxy.py)
#   - K>4: oversubscribes CPU (Lean processes thrash); not recommended on
#          this hardware
CONCURRENCY="${CONCURRENCY:-1}"
if ! [[ "$CONCURRENCY" =~ ^[1-9][0-9]*$ ]]; then
    echo "FATAL: CONCURRENCY=$CONCURRENCY must be a positive integer"
    exit 2
fi

TOTAL_CELLS=0
for m in "${MODES[@]}"; do
    for pid in $HARD10_IDS; do
        for seed in "${SEEDS[@]}"; do
            TOTAL_CELLS=$((TOTAL_CELLS + 1))
        done
    done
done
echo "[batch] $TOTAL_CELLS total cells; concurrency=$CONCURRENCY"
echo "[batch] output = ${OUT_PREFIX}__<mode>_<problem>_<seed>.jsonl"

# Cell timeout: smoke 5 min, full 30 min (inherited from serial design).
CELL_TIMEOUT=$([ "$SMOKE" -eq 1 ] && echo 300 || echo 1800)

# Single-cell runner — backgroundable; each invocation produces exactly
# one jsonl row (real or synthetic-failure stub) for cell-completeness.
# All env vars used inside are exported in the parent shell + the
# function reads them on each invocation, so subshells inherit cleanly.
run_cell() {
    local cell_idx=$1
    local m=$2
    local pid=$3
    local seed=$4
    local out_file="${OUT_PREFIX}__${m}_${pid}_seed${seed}.jsonl"
    local cell_ts cell_exit cell_output pput_line solved verified
    cell_ts=$(date -Is)
    echo "[$cell_ts] cell $cell_idx/$TOTAL_CELLS START  mode=$m  problem=$pid  seed=$seed"

    cell_exit=0
    cell_output=$(BOLTZMANN_SEED="$seed" CONDITION="$CONDITION" SPLIT="adaptation" \
        timeout "$CELL_TIMEOUT" "$EVALUATOR" --mode="$m" "${pid}.lean" 2>&1) || cell_exit=$?

    pput_line=$(echo "$cell_output" | grep "^PPUT_RESULT:" | sed 's/^PPUT_RESULT://' || true)

    if [ -z "$pput_line" ]; then
        # Audit-fix Gemini Q7.b: every cell must produce a row.
        echo "[$(date -Is)] cell $cell_idx FAIL  mode=$m  problem=$pid  seed=$seed  exit=$cell_exit"
        echo "$cell_output" | tail -c 4000 > "${out_file}.err"
        printf '{"schema_version":"v2.0","problem_id":"%s","mode":"%s","split":"adaptation","solved":false,"verified":false,"progress":0,"_synthetic_failure":true,"_exit_code":%d}\n' \
            "$pid" "$m" "$cell_exit" > "$out_file"
        return 1
    fi
    echo "$pput_line" > "$out_file"
    solved=$(echo "$pput_line" | python3 -c "import sys,json; print(json.loads(sys.stdin.read()).get('solved'))" 2>/dev/null || echo "?")
    verified=$(echo "$pput_line" | python3 -c "import sys,json; print(json.loads(sys.stdin.read()).get('verified'))" 2>/dev/null || echo "?")
    echo "[$(date -Is)] cell $cell_idx DONE  mode=$m  problem=$pid  seed=$seed  solved=$solved  verified=$verified"
    return 0
}

# Pool dispatcher: run cells with up to CONCURRENCY concurrent children.
# `wait -n` (bash 4.3+) waits for the next finished job — true pool, no
# K-batch-of-K stragglers. FAIL_COUNT incremented atomically by reading
# child exit status via wait.
declare -A CHILD_EXITS=()
declare -a CHILD_PIDS=()
CELL_IDX=0
FAIL_COUNT=0
BATCH_START=$(date +%s)

for m in "${MODES[@]}"; do
    for pid in $HARD10_IDS; do
        for seed in "${SEEDS[@]}"; do
            CELL_IDX=$((CELL_IDX + 1))
            # Drain one slot if at capacity. -n waits for ANY child.
            while [ "${#CHILD_PIDS[@]}" -ge "$CONCURRENCY" ]; do
                # Reap completed children (POSIX-portable scan; bash wait -n
                # caveats around lost exit codes when multiple finish
                # simultaneously avoided by per-pid wait).
                NEW_PIDS=()
                for p in "${CHILD_PIDS[@]}"; do
                    if kill -0 "$p" 2>/dev/null; then
                        NEW_PIDS+=("$p")
                    else
                        # `set -e` would otherwise abort on first non-zero
                        # cell exit (run_cell returns 1 on synthetic-failure).
                        # `|| rc=$?` puts wait in a logical chain so set -e
                        # is suppressed and we can branch on rc explicitly.
                        rc=0
                        wait "$p" || rc=$?
                        if [ "$rc" -ne 0 ]; then
                            FAIL_COUNT=$((FAIL_COUNT + 1))
                        fi
                    fi
                done
                CHILD_PIDS=("${NEW_PIDS[@]}")
                if [ "${#CHILD_PIDS[@]}" -ge "$CONCURRENCY" ]; then
                    sleep 1
                fi
            done
            # Spawn new cell.
            run_cell "$CELL_IDX" "$m" "$pid" "$seed" &
            CHILD_PIDS+=("$!")
        done
    done
done

# Drain remaining children. Same set-e suppression as in the pool loop.
for p in "${CHILD_PIDS[@]}"; do
    rc=0
    wait "$p" || rc=$?
    if [ "$rc" -ne 0 ]; then
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
done

BATCH_END=$(date +%s)
BATCH_ELAPSED=$((BATCH_END - BATCH_START))
echo "[batch] complete. cells=$TOTAL_CELLS fail=$FAIL_COUNT elapsed=${BATCH_ELAPSED}s concurrency=$CONCURRENCY"
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
