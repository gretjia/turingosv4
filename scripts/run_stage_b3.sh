#!/usr/bin/env bash
# Stage B3 / TB-18B M2 — chain-backed multi-condition benchmark runner.
#
# TB-18B charter §1 verbatim: M2 = 100 problems × n=3 × 3 seeds × 2 models
#   = 1800 invocations.
#
# Per-invocation output (matches mini-M1 R6 evidence shape):
#   <RUN_DIR>/<model_dir>/seed<S>/rep<R>/<idx>_<problem>/
#     ├── runtime_repo/         (chain output via TURINGOS_CHAINTAPE_PATH)
#     ├── cas/                  (CAS objects)
#     ├── evaluator.stdout      (PPUT_RESULT JSON line + diagnostics)
#     ├── evaluator.stderr      (full evaluator stderr)
#     ├── chain_invariant.json  (FR-18B.4 / FC1 hard invariant verdict)
#     ├── chain_invariant.stderr
#     ├── runtime_repo.dotgit.tar.gz   (FR-18B.2 EvidencePackagingPolicy)
#     └── cas.dotgit.tar.gz            (FR-18B.2 EvidencePackagingPolicy)
#
# Run-level artifacts:
#   <RUN_DIR>/BenchmarkManifest.json   (FR-18B.1 / CR-18B.5 — pinned
#                                       BEFORE batch per
#                                       feedback_benchmark_manifest_required)
#   <RUN_DIR>/PROBLEMS.txt              (canonical problem list for replay)
#   <RUN_DIR>/run_log.txt               (per-invocation status line)
#   <RUN_DIR>/SUMMARY.json              (post-loop tally; full
#                                       AggregateReport.json is a
#                                       separate aggregate-runner invocation
#                                       per CLAUDE.md §17 / FR-18B.5)
#
# RESUMABILITY: an invocation whose chain_invariant.json already exists
# is SKIPPED. Restart-after-crash is safe; partial runs accumulate. To
# force a full re-run, delete the target run dir first.
#
# Usage:
#   bash scripts/run_stage_b3.sh <run_tag> [problems_file] [n_attempts] [seeds_csv]
#
# Args (all positional, only run_tag mandatory):
#   run_tag       — output dir suffix; final dir is
#                   handover/evidence/<run_tag>/. Recommended:
#                   stage_b3_smoke_<TS>  (smoke validation)
#                   stage_b3_r7_m2_<TS>  (full M2 charter shape)
#   problems_file — newline-separated problem basenames (without .lean).
#                   Default: derive lex-first 100 from
#                   $MINIF2F_DIR/MiniF2F/Test/.
#                   Smoke recommended: a 2-4 line file.
#   n_attempts    — replicates per (problem, seed, model). Default 3
#                   (TB-18B charter §1).
#   seeds_csv     — comma-separated BOLTZMANN_SEED list. Default
#                   "1,2,3" (TB-18B charter §1).
#
# Hard-coded for charter strict alignment (NOT configurable):
#   MODELS = ("deepseek-chat" "deepseek-ai/DeepSeek-V3")
#     - primary = deepseek-chat (api.deepseek.com)
#     - alternative = deepseek-ai/DeepSeek-V3 via SiliconFlow per user
#       2026-05-08 session #25 AskUserQuestion verbatim
#
# Authority:
#   - TB-18B charter §1 (M2 scope) + §3 CR-18B.5 (BenchmarkManifest pin)
#     + §3 CR-18B.6 (EvidencePackagingPolicy 100+×n>1 mandatory)
#     + §3 CR-18B.9 (/runner-preflight 7-stage gate before launch)
#   - architect 2026-05-07 ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL §3.B3
#     verbatim
#   - parent §3.2 Class-3 + §6 LLM real-problem testing authorization
#   - session #25 AskUserQuestion: M2 = charter shape (1800 runs);
#     alt-model = deepseek-ai/DeepSeek-V3 via SiliconFlow

set -uo pipefail

# ── Argument parsing ────────────────────────────────────────────────────────

if [[ $# -lt 1 ]]; then
    echo "ERROR: run_tag required (e.g., stage_b3_smoke_$(date -u +%Y%m%dT%H%M%SZ))" >&2
    echo "Usage: $0 <run_tag> [problems_file] [n_attempts] [seeds_csv]" >&2
    exit 2
fi

RUN_TAG="$1"
PROBLEMS_FILE="${2:-}"
N_ATTEMPTS="${3:-3}"
SEEDS_CSV="${4:-1,2,3}"

# ── Paths and constants ─────────────────────────────────────────────────────

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
EXPERIMENT_DIR="$PROJECT_ROOT/experiments/minif2f_v4"
MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"
RUN_DIR="$PROJECT_ROOT/handover/evidence/$RUN_TAG"

# Charter-pinned model list (NOT configurable; aligns with TB-18B §1
# + PREREG_PPUT_CCL_2026-04-26.md §1.8 canonical).
#
# Primary: deepseek-v4-flash (api.deepseek.com — PREREG §1.8 canonical
# thinking-off backend; rules-engine R-019 compliant; supersedes earlier
# Wave 3 50p / mini-M1 R6 alias `deepseek-chat` per memory
# `project_deepseek_drift_2026-04-24` — aliases may silently drift to
# different backends mid-experiment; pin to canonical for ship-grade
# BenchmarkManifest discipline per CR-18B.5).
#
# Alternative: Qwen/Qwen2.5-72B-Instruct via SiliconFlow.
#   - User AskUserQuestion 2026-05-08 session #25: deepseek-ai/DeepSeek-V3
#   - Session #26 smoke (commit 1210ea3) showed V3 wall-time at ~790s/cell
#     misdiagnosed as model speed; turned out to be FORCED_PROVIDER misroute
#     at proxy port :8080 (commit 1f7879a fix to :18080).
#   - User AskUserQuestion 2026-05-08 session #26 verbatim: "换 alt-model 为
#     Qwen2.5-72B (Recommended)" — multi-condition signal preserved (Qwen
#     different vendor + different architecture from DeepSeek-v4-flash),
#     smoke v3 confirmed Qwen actually FASTER than chat (97-173s/cell vs
#     131-255s) at SiliconFlow, expected=9 (real LLM activity, no MAX_TX).
MODELS=("deepseek-v4-flash" "Qwen/Qwen2.5-72B-Instruct")

# Per-problem evaluator wall-clock cap (matches Wave 3 50p PHASE_3_RUN_MANIFEST).
PER_PROBLEM_TIMEOUT_S="${PER_PROBLEM_TIMEOUT_S:-1800}"
MAX_TX_PER_PROBLEM="${MAX_TX_PER_PROBLEM:-200}"

# Lean toolchain (preflight per C-012 + reference_oracle_preflight).
LEAN_BIN="${LEAN_BINARY:-$HOME/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean}"

# LLM proxy. MUST be a multi-provider auto-routing instance (no
# --provider FORCED flag) so requests for `Qwen/*` and `deepseek-ai/*`
# route to api.siliconflow.cn while `deepseek-chat` routes to
# api.deepseek.com per `src/drivers/llm_proxy.py::detect_provider`
# slash-prefix rule.
#
# Default port 18080 = multi-provider instance (verified by
# session #26 curl probe with Qwen2.5-7B-Instruct → 200 OK +
# real `content` not deepseek 400 invalid-model error).
#
# Port 8080 is FORCED `--provider deepseek` and CANNOT be used here
# — it 400s every Qwen/SF request and silently produces a degenerate
# `tool_dist.llm_err=200` cell (chain stays correct but no real LLM
# activity from alt-model). This was the root cause of the
# session #26 first-smoke 5x slowdown finding (commit 1210ea3 +
# 9f9aee7 evidence).
export LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:18080}"

# ── Source .env (DeepSeek + SiliconFlow keys) ───────────────────────────────

if [[ -f "$PROJECT_ROOT/.env" ]]; then
    set -a
    # shellcheck disable=SC1091
    . "$PROJECT_ROOT/.env"
    set +a
elif [[ -z "${DEEPSEEK_API_KEY:-}" && -f "$HOME/projects/turingosv3/.env" ]]; then
    set -a
    # shellcheck disable=SC1091
    . "$HOME/projects/turingosv3/.env"
    set +a
fi

: "${DEEPSEEK_API_KEY:?DEEPSEEK_API_KEY not set; populate .env first (primary model)}"
: "${SILICONFLOW_API_KEY:?SILICONFLOW_API_KEY not set; populate .env first (alt-model via SF)}"

# ── Pre-loop validation ─────────────────────────────────────────────────────

echo "════════════════════════════════════════════════════════════════════════"
echo "Stage B3 / TB-18B M2 chain-backed batch runner"
echo "════════════════════════════════════════════════════════════════════════"
echo "RUN_TAG          = $RUN_TAG"
echo "RUN_DIR          = $RUN_DIR"
echo "MODELS           = ${MODELS[*]}"
echo "N_ATTEMPTS (n)   = $N_ATTEMPTS"
echo "SEEDS            = $SEEDS_CSV"
echo "PER_PROB_TIMEOUT = ${PER_PROBLEM_TIMEOUT_S}s"
echo "MAX_TX/PROBLEM   = $MAX_TX_PER_PROBLEM"
echo "MINIF2F_DIR      = $MINIF2F_DIR"
echo "LLM_PROXY_URL    = $LLM_PROXY_URL"
echo

# Tree must be clean per CR-18B.9 / runner-preflight Stage 1 (caller
# should have invoked /runner-preflight already; this is a backstop).
if [[ -n "$(cd "$PROJECT_ROOT" && git status --porcelain | grep -vE '^\?\? handover/evidence/' | head -1)" ]]; then
    echo "WARNING: working tree has non-evidence changes. /runner-preflight Stage 1 normally rejects this." >&2
    echo "  Set TURINGOS_STAGE_B3_DIRTY_OK=1 to override (smoke testing only)." >&2
    if [[ "${TURINGOS_STAGE_B3_DIRTY_OK:-0}" != "1" ]]; then
        exit 3
    fi
fi

GIT_HEAD="$(cd "$PROJECT_ROOT" && git rev-parse HEAD 2>/dev/null || echo unknown)"
GIT_HEAD_SHORT="$(cd "$PROJECT_ROOT" && git rev-parse --short HEAD 2>/dev/null || echo unknown)"
RUN_TIMESTAMP_UTC="$(date -u +%Y-%m-%dT%H-%M-%SZ)"

# ── Build release binaries ──────────────────────────────────────────────────

echo "[build] cargo build --release -p minif2f_v4 -p turingosv4 --bin evaluator --bin tb_18r_compute_invariant"
(cd "$PROJECT_ROOT" && cargo build --release -p minif2f_v4 -p turingosv4 \
    --bin evaluator --bin tb_18r_compute_invariant 2>&1 | tail -5)
EVALUATOR="$PROJECT_ROOT/target/release/evaluator"
COMPUTE_INVARIANT="$PROJECT_ROOT/target/release/tb_18r_compute_invariant"
[[ -x "$EVALUATOR" ]] || { echo "ERROR: evaluator binary not built: $EVALUATOR" >&2; exit 4; }
[[ -x "$COMPUTE_INVARIANT" ]] || { echo "ERROR: tb_18r_compute_invariant not built: $COMPUTE_INVARIANT" >&2; exit 4; }

# ── Lean / Mathlib preflight (C-012) ────────────────────────────────────────

echo "[preflight] Lean + Mathlib trivial probe"
PFL=$(find "$MINIF2F_DIR/.lake/packages" \( -path "*/.lake/build/lib/lean" -o -path "*/lib/lean" \) -type d 2>/dev/null | tr '\n' ':')
if [[ -z "$PFL" ]]; then
    echo "PREFLIGHT FAIL: no Mathlib lake build under $MINIF2F_DIR/.lake/packages" >&2
    exit 5
fi
PROBE_OUT=$(printf 'import Mathlib\nexample : (1:ℝ) + 1 = 2 := by norm_num\n' \
    | LEAN_PATH="$PFL" timeout 180 "$LEAN_BIN" --stdin 2>&1) || PROBE_RC=$?
if [[ "${PROBE_RC:-0}" -ne 0 ]] || echo "$PROBE_OUT" | grep -q "error:"; then
    echo "PREFLIGHT FAIL: $(echo "$PROBE_OUT" | head -c 400)" >&2
    exit 5
fi
echo "[preflight] OK"

# ── Resolve problem list ────────────────────────────────────────────────────

mkdir -p "$RUN_DIR"

if [[ -n "$PROBLEMS_FILE" ]]; then
    if [[ ! -f "$PROBLEMS_FILE" ]]; then
        echo "ERROR: problems_file does not exist: $PROBLEMS_FILE" >&2
        exit 6
    fi
    cp "$PROBLEMS_FILE" "$RUN_DIR/PROBLEMS.txt"
else
    # Derive: lex-first 100 from MiniF2F/Test/. Reproducible (sort -u).
    echo "[problems] no list provided; deriving lex-first 100 from $MINIF2F_DIR/MiniF2F/Test/"
    find "$MINIF2F_DIR/MiniF2F/Test" -maxdepth 1 -name "*.lean" -type f -printf "%f\n" \
        | sed 's/\.lean$//' | sort -u | head -100 > "$RUN_DIR/PROBLEMS.txt"
fi

mapfile -t PROBLEMS < "$RUN_DIR/PROBLEMS.txt"
PROBLEM_COUNT=${#PROBLEMS[@]}
if [[ "$PROBLEM_COUNT" -lt 1 ]]; then
    echo "ERROR: PROBLEMS.txt is empty" >&2
    exit 6
fi

IFS=',' read -ra SEEDS <<< "$SEEDS_CSV"
SEED_COUNT=${#SEEDS[@]}
EXPECTED_TOTAL=$(( PROBLEM_COUNT * SEED_COUNT * N_ATTEMPTS * ${#MODELS[@]} ))

echo "[plan] problems=$PROBLEM_COUNT × seeds=$SEED_COUNT × n_attempts=$N_ATTEMPTS × models=${#MODELS[@]} = $EXPECTED_TOTAL invocations"
echo

# ── Emit BenchmarkManifest.json (FR-18B.1 / CR-18B.5) ───────────────────────

MANIFEST="$RUN_DIR/BenchmarkManifest.json"
PROBLEMS_JSON_ARRAY=$(printf '%s\n' "${PROBLEMS[@]}" | python3 -c 'import sys,json; print(json.dumps([l.strip() for l in sys.stdin if l.strip()]))')
SEEDS_JSON_ARRAY=$(printf '%s\n' "${SEEDS[@]}" | python3 -c 'import sys,json; print(json.dumps([int(l.strip()) for l in sys.stdin if l.strip()]))')
MODELS_JSON_ARRAY=$(printf '%s\n' "${MODELS[@]}" | python3 -c 'import sys,json; print(json.dumps([l.strip() for l in sys.stdin if l.strip()]))')

# Lean version probe.
LEAN_VERSION=$("$LEAN_BIN" --version 2>/dev/null | head -1 || echo unknown)
# Mathlib commit (best-effort: read from .lake/packages/mathlib/.git or lake-manifest).
MATHLIB_COMMIT="unknown"
if [[ -f "$MINIF2F_DIR/lake-manifest.json" ]]; then
    MATHLIB_COMMIT=$(python3 -c "
import json
with open('$MINIF2F_DIR/lake-manifest.json') as f:
    m = json.load(f)
for pkg in m.get('packages', []):
    if pkg.get('name') == 'mathlib':
        print(pkg.get('rev', 'unknown'))
        break
else:
    print('unknown')
" 2>/dev/null || echo unknown)
fi

# Evaluator binary sha256 (FR-18B.1 binary identity).
BINARY_SHA256=$(sha256sum "$EVALUATOR" | awk '{print $1}')

cat > "$MANIFEST" <<JSON
{
  "schema_version": "stage_b3_v1",
  "run_tag": "$RUN_TAG",
  "run_timestamp_utc": "$RUN_TIMESTAMP_UTC",
  "charter": "handover/tracer_bullets/TB-18B_charter_2026-05-07.md",
  "charter_section": "§1 (M2 scope) + §3 CR-18B.5 (manifest pin) + §3 CR-18B.9 (preflight gate)",
  "architect_directive": "handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_zh.md §3.B3",
  "git_head": "$GIT_HEAD",
  "git_head_short": "$GIT_HEAD_SHORT",
  "evaluator_binary_sha256": "$BINARY_SHA256",
  "lean_version": "$(echo "$LEAN_VERSION" | sed 's/"/\\"/g')",
  "mathlib_commit": "$MATHLIB_COMMIT",
  "minif2f_dir": "$MINIF2F_DIR",
  "models": $MODELS_JSON_ARRAY,
  "seeds": $SEEDS_JSON_ARRAY,
  "n_attempts": $N_ATTEMPTS,
  "problem_count": $PROBLEM_COUNT,
  "expected_total_invocations": $EXPECTED_TOTAL,
  "max_tx_per_problem": $MAX_TX_PER_PROBLEM,
  "per_problem_timeout_s": $PER_PROBLEM_TIMEOUT_S,
  "llm_proxy_url": "$LLM_PROXY_URL",
  "problems": $PROBLEMS_JSON_ARRAY,
  "sampling_strategy": "exhaustive (no sampling; all problem×seed×model×rep tuples enumerated)",
  "evidence_packaging_policy": {
    "per_run_runtime_repo_tarball": true,
    "per_run_cas_tarball": true,
    "tarball_format": "tar.gz",
    "feedback_anchor": "feedback_evidence_packaging_policy_required"
  },
  "fc1_invariant_formula": "evaluator_reported_completed_llm_calls == l4_work_attempt_count + l4e_work_attempt_count + capsule_anchored_attempt_count",
  "fc1_invariant_formula_lhs_decomposition": "tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err (per CLAUDE.md §6 OBS_TB18R_INV1_NONLLM_TX clarification 2026-05-07)"
}
JSON

echo "[manifest] wrote $MANIFEST"
echo

# ── Helpers ─────────────────────────────────────────────────────────────────

slugify_model() {
    # "deepseek-chat" → "deepseek-chat"
    # "deepseek-ai/DeepSeek-V3" → "deepseek-ai_DeepSeek-V3"
    echo "$1" | tr '/' '_'
}

derive_halt_class() {
    # Read PPUT_RESULT JSON from stdin; emit halt_class string.
    python3 -c '
import sys, json
try:
    j = json.loads(sys.stdin.read().strip())
except Exception:
    print("ErrorHalt"); sys.exit(0)
if j.get("solved"):
    print("OmegaAccepted")
elif j.get("hit_max_tx"):
    print("MaxTxExhausted")
else:
    # No clean signal in PPUT for WallClockCap vs ComputeCap vs DegradedLLM
    # vs ErrorHalt; default to ErrorHalt (audit-visible) — operator can
    # reclassify in OBS_TB18B_* if needed.
    print("ErrorHalt")
'
}

derive_expected_completed() {
    # Read PPUT_RESULT JSON from stdin; emit
    # tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err
    # per CLAUDE.md §6 OBS_TB18R_INV1_NONLLM_TX 2026-05-07 clarification.
    python3 -c '
import sys, json
try:
    j = json.loads(sys.stdin.read().strip())
except Exception:
    print(0); sys.exit(0)
td = j.get("tool_dist", {}) or {}
n = int(td.get("step", 0)) + int(td.get("parse_fail", 0)) + int(td.get("llm_err", 0))
print(n)
'
}

# ── Main loop ───────────────────────────────────────────────────────────────

RUN_LOG="$RUN_DIR/run_log.txt"
echo "Stage B3 batch run log — $RUN_TIMESTAMP_UTC" > "$RUN_LOG"

INVOCATION_INDEX=0
COMPLETED=0
SKIPPED=0
FAILED=0

for MODEL in "${MODELS[@]}"; do
    MODEL_SLUG=$(slugify_model "$MODEL")
    for SEED in "${SEEDS[@]}"; do
        for REP in $(seq 1 "$N_ATTEMPTS"); do
            for I in "${!PROBLEMS[@]}"; do
                INVOCATION_INDEX=$((INVOCATION_INDEX + 1))
                PROBLEM="${PROBLEMS[$I]}"
                # 1-indexed, zero-padded width 3 for sort-stable file order.
                PROBLEM_IDX=$(printf "P%03d" $((I + 1)))
                CELL_DIR="$RUN_DIR/$MODEL_SLUG/seed${SEED}/rep${REP}/${PROBLEM_IDX}_${PROBLEM}"
                CHAIN_INVAR_OUT="$CELL_DIR/chain_invariant.json"

                LABEL="[$INVOCATION_INDEX/$EXPECTED_TOTAL] model=$MODEL_SLUG seed=$SEED rep=$REP $PROBLEM_IDX $PROBLEM"

                # Resumability: skip if chain_invariant.json exists AND is
                # non-empty AND no DEGENERATE_RUN.flag is set. Empty
                # chain_invariant.json (compute_invariant failure) and
                # DEGENERATE_RUN.flag (vacuous expected=0+ErrorHalt) both
                # indicate prior failure — clear them and retry.
                if [[ -f "$CHAIN_INVAR_OUT" && -s "$CHAIN_INVAR_OUT" && ! -f "$CELL_DIR/DEGENERATE_RUN.flag" ]]; then
                    echo "$LABEL SKIP (existing valid chain_invariant.json)" | tee -a "$RUN_LOG"
                    SKIPPED=$((SKIPPED + 1))
                    continue
                fi
                # Clear any stale failed-cell artifacts so this iteration
                # is a clean retry.
                rm -f "$CHAIN_INVAR_OUT" "$CELL_DIR/DEGENERATE_RUN.flag" 2>/dev/null || true
                find "$CELL_DIR" -mindepth 1 -maxdepth 1 -name "cas*" -type d -exec rm -rf {} + 2>/dev/null || true
                find "$CELL_DIR" -mindepth 1 -maxdepth 1 -name "runtime_repo" -type d -exec rm -rf {} + 2>/dev/null || true
                find "$CELL_DIR" -maxdepth 1 -name "*.tar.gz" -delete 2>/dev/null || true

                PROBLEM_PATH="$MINIF2F_DIR/MiniF2F/Test/${PROBLEM}.lean"
                if [[ ! -f "$PROBLEM_PATH" ]]; then
                    echo "$LABEL MISSING_PROBLEM (skip)" | tee -a "$RUN_LOG"
                    FAILED=$((FAILED + 1))
                    continue
                fi

                mkdir -p "$CELL_DIR/runtime_repo" "$CELL_DIR/cas"

                START_S=$(date +%s)

                # Per-run env. CONDITION="n1" matches mini-M1 R6 pattern
                # (single-agent swarm; the only ChainTape-wired condition
                # used at scale per TB-7R verdict B3 — `oneshot` is NOT
                # ChainTape-wired and fails-closed).
                #
                # TB-18B charter §1 "n=3" = 3 INDEPENDENT REPLICATES per
                # (problem, seed, model) tuple (REP loop above), each
                # invocation = single CONDITION=n1 evaluator run. NOT
                # CONDITION=n3 (which would be 3-agent swarm with 3 LLM
                # calls in one chain) — interpretation aligned with mini-M1
                # R6 + LATEST.md cardinality (M1 50p×n=3×3seeds×1model=450
                # invocations, NOT 150).
                #
                # TURINGOS_CAS_PATH is set EXPLICITLY (not auto-derived)
                # because src/runtime/mod.rs::RuntimeChaintapeConfig::from_env
                # would otherwise produce `cas_<chain_basename>` (e.g.,
                # `cas_runtime_repo`) breaking the canonical evidence shape
                # of `<cell>/{runtime_repo,cas}/` per mini-M1 R6 + replay
                # tooling expectation. Setting explicitly aligns to mini-M1.
                set +e
                timeout "$PER_PROBLEM_TIMEOUT_S" env \
                    CONDITION="n1" \
                    MINIF2F_DIR="$MINIF2F_DIR" \
                    EXPERIMENT_DIR="$EXPERIMENT_DIR" \
                    RUST_LOG="${RUST_LOG:-info}" \
                    ACTIVE_MODEL="$MODEL" \
                    BOLTZMANN_SEED="$SEED" \
                    TURINGOS_CHAINTAPE_PATH="$CELL_DIR/runtime_repo" \
                    TURINGOS_CAS_PATH="$CELL_DIR/cas" \
                    MAX_TX_PER_PROBLEM="$MAX_TX_PER_PROBLEM" \
                    "$EVALUATOR" "$PROBLEM_PATH" \
                    > "$CELL_DIR/evaluator.stdout" \
                    2> "$CELL_DIR/evaluator.stderr"
                EVAL_RC=$?
                set -e

                ELAPSED_S=$(( $(date +%s) - START_S ))

                # Extract PPUT_RESULT from stdout.
                PPUT_LINE=$(grep "^PPUT_RESULT:" "$CELL_DIR/evaluator.stdout" 2>/dev/null | sed 's/^PPUT_RESULT://' | head -1 || echo "")

                if [[ -z "$PPUT_LINE" ]]; then
                    # No PPUT_RESULT — evaluator crashed / timed out before emitting.
                    if [[ "$EVAL_RC" -eq 124 ]]; then
                        HALT_CLASS="WallClockCap"
                    else
                        HALT_CLASS="ErrorHalt"
                    fi
                    EXPECTED=0
                else
                    HALT_CLASS=$(echo "$PPUT_LINE" | derive_halt_class)
                    EXPECTED=$(echo "$PPUT_LINE" | derive_expected_completed)
                fi

                # Emit chain_invariant.json via tb_18r_compute_invariant.
                set +e
                "$COMPUTE_INVARIANT" \
                    --runtime-repo "$CELL_DIR/runtime_repo" \
                    --cas "$CELL_DIR/cas" \
                    --expected-completed "$EXPECTED" \
                    --halt-class "$HALT_CLASS" \
                    > "$CHAIN_INVAR_OUT" \
                    2> "$CELL_DIR/chain_invariant.stderr"
                INVAR_RC=$?
                set -e

                if [[ "$INVAR_RC" -ne 0 ]]; then
                    echo "$LABEL FAIL (compute_invariant rc=$INVAR_RC after ${ELAPSED_S}s)" | tee -a "$RUN_LOG"
                    FAILED=$((FAILED + 1))
                    # Do NOT pack tarballs on compute_invariant failure;
                    # leaves cell in a state where rerun (after deletion)
                    # can retry without partial artifacts.
                    continue
                fi

                # Sanity: smoke-grade check that the cell actually exercised
                # the chain (avoid vacuous Ok where 0==0+0+0). A real n1 run
                # against a valid problem produces ≥1 attempt; if EXPECTED=0
                # AND HALT_CLASS=ErrorHalt, the evaluator likely fail-closed
                # before any LLM call (e.g., misconfigured CONDITION). Mark
                # the cell DEGENERATE so smoke catches it, but don't delete
                # the chain_invariant.json (audit-visible).
                if [[ "$EXPECTED" -eq 0 && "$HALT_CLASS" == "ErrorHalt" ]]; then
                    echo "$LABEL DEGENERATE expected=0 halt=ErrorHalt — evaluator likely fail-closed before LLM; check evaluator.stderr" | tee -a "$RUN_LOG"
                    touch "$CELL_DIR/DEGENERATE_RUN.flag"
                    FAILED=$((FAILED + 1))
                    continue
                fi

                # Pack EvidencePackagingPolicy tarballs (FR-18B.2 / CR-18B.6).
                tar -czf "$CELL_DIR/runtime_repo.dotgit.tar.gz" \
                    -C "$CELL_DIR" runtime_repo 2>>"$CELL_DIR/chain_invariant.stderr" || true
                tar -czf "$CELL_DIR/cas.dotgit.tar.gz" \
                    -C "$CELL_DIR" cas 2>>"$CELL_DIR/chain_invariant.stderr" || true

                VERDICT=$(python3 -c "import json; print(json.load(open('$CHAIN_INVAR_OUT'))['invariant_verdict'])" 2>/dev/null || echo "?")
                DELTA=$(python3 -c "import json; print(json.load(open('$CHAIN_INVAR_OUT'))['delta'])" 2>/dev/null || echo "?")

                echo "$LABEL OK verdict=$VERDICT delta=$DELTA halt=$HALT_CLASS expected=$EXPECTED elapsed=${ELAPSED_S}s" | tee -a "$RUN_LOG"
                COMPLETED=$((COMPLETED + 1))
            done
        done
    done
done

# ── Post-loop summary (NOT full AggregateReport.json — that runs separately) ─

SUMMARY="$RUN_DIR/SUMMARY.json"
cat > "$SUMMARY" <<JSON
{
  "schema_version": "stage_b3_summary_v1",
  "run_tag": "$RUN_TAG",
  "completed_at_utc": "$(date -u +%Y-%m-%dT%H-%M-%SZ)",
  "expected_total_invocations": $EXPECTED_TOTAL,
  "completed": $COMPLETED,
  "skipped_resumed": $SKIPPED,
  "failed": $FAILED,
  "manifest": "BenchmarkManifest.json",
  "next_step": "Run the aggregate-runner binary (consumes wilson_ci.rs + diversity.rs per CLAUDE.md §17 / FR-18B.5) to emit AggregateReport.json."
}
JSON

echo
echo "════════════════════════════════════════════════════════════════════════"
echo "Stage B3 batch complete"
echo "════════════════════════════════════════════════════════════════════════"
echo "expected: $EXPECTED_TOTAL"
echo "completed: $COMPLETED"
echo "skipped (already done): $SKIPPED"
echo "failed: $FAILED"
echo "summary:  $SUMMARY"
echo "manifest: $MANIFEST"
echo "log:      $RUN_LOG"
echo

if [[ "$FAILED" -gt 0 ]]; then
    echo "Some invocations failed. Review $RUN_LOG and per-cell evaluator.stderr."
    exit 1
fi
exit 0
