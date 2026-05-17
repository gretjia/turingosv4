#!/usr/bin/env bash
# TB-G G1.2-6 / G1.2-7 — chain-continuous batch runner (Option B+ canonical).
#
# Drives `batch_evaluator` (G1.2-3) which spawns the existing `evaluator`
# subprocess once per problem, sharing ONE `runtime_repo/` + ONE `cas/` +
# ONE genesis_report across the batch. Subprocess resume is mandatory for
# every task at task_index > 0 per architect Option B+ ruling §1.
#
# Authority:
#   - architect 2026-05-11 Option B+ orchestration ruling
#     `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md`
#     (§1 canonical orchestration + §3.1 ResumePreflight + §3.2
#     ChainTapeLease + §3.3 BatchContinuationManifest + §3.4
#     persistence evidence + §3.5 halt-and-record)
#   - charter §1 G1.2-6 / G1.2-7 ship-gate rows
#   - parent G1.1 §8 packet ship-condition (b) sets the precedent
#     evidence shape (handover/evidence/g_phase_g1_1_smoke_*/)
#
# Per-task evidence shape (sibling of G1.1 smoke + Stage B3):
#   <RUN_DIR>/P<NN>_<problem>/
#     ├── evaluator.stdout    (PPUT_RESULT JSON line + diagnostics)
#     └── evaluator.stderr    (full subprocess stderr)
#
# Run-level artifacts:
#   <RUN_DIR>/runtime_repo/         (shared chain across all tasks)
#   <RUN_DIR>/cas/                  (shared CAS across all tasks)
#   <RUN_DIR>/BatchContinuationManifest.json   (G1.2-4; written by batch_evaluator)
#   <RUN_DIR>/PROBLEMS.txt           (canonical problem list for replay)
#   <RUN_DIR>/G_PHASE_BATCH_MANIFEST.json   (this script; pinning batch-id + git HEAD + model + proxy)
#   <RUN_DIR>/aggregate_verdict.json   (audit_tape over shared runtime_repo + cas)
#   <RUN_DIR>/audit_tape.stderr
#   <RUN_DIR>/run_log.txt              (per-task summary line)
#
# Usage:
#   bash scripts/run_g_phase_batch.sh <run_tag> <problem_set>
#
# Args:
#   run_tag       — output dir suffix; final dir is handover/evidence/<run_tag>/.
#                   Recommended:
#                     g_phase_g1_2_mini_<TS>  (G1.2-6 3-task)
#                     g_phase_g1_2_full_<TS>  (G1.2-7 9-task)
#   problem_set   — one of:
#                     mini   — first 3 of the TB-N3 Phase 2 problem set
#                     full   — full 9-problem TB-N3 Phase 2 set
#                     <path> — explicit problems file (one basename per line)
#
# Env (override-able):
#   LLM_PROXY_URL          — default http://localhost:8080 (matches G1.1 smoke)
#   ACTIVE_MODEL           — default deepseek-chat
#   MINIF2F_DIR            — default /home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4
#   PER_PROBLEM_TIMEOUT_S  — default 900 (15 min)
#   TURINGOS_TB_N3_AUTO_MARKET — default 1 (auto-emit node-survive markets)

set -uo pipefail

clear_stale_cas_chain_lock() {
    local cas_dir="$1"
    local lock_path="${2:-$cas_dir/.turingos_cas_chain.lock}"
    local log_path="${3:-$cas_dir/../cas_stale_lock_cleanup.log}"
    local lock_body=""
    local lock_pid=""
    local ts=""

    [[ -e "$lock_path" ]] || return 0
    ts="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    lock_body="$(cat "$lock_path" 2>/dev/null || true)"
    if [[ "$lock_body" =~ (^|[[:space:]])pid=([0-9]+)($|[[:space:]]) ]]; then
        lock_pid="${BASH_REMATCH[2]}"
        if kill -0 "$lock_pid" 2>/dev/null; then
            {
                echo "$ts preserve_live_lock path=$lock_path pid=$lock_pid"
            } >> "$log_path"
            return 0
        fi
        {
            echo "$ts remove_stale_lock path=$lock_path pid=$lock_pid reason=pid_not_alive"
        } >> "$log_path"
        rm -f -- "$lock_path"
        return 0
    fi

    {
        echo "$ts remove_stale_lock path=$lock_path reason=missing_or_invalid_pid"
    } >> "$log_path"
    rm -f -- "$lock_path"
}

# ── Argument parsing ────────────────────────────────────────────────────────

if [[ $# -lt 2 ]]; then
    echo "ERROR: usage: $0 <run_tag> <problem_set:mini|full|<path>>" >&2
    exit 2
fi

RUN_TAG="$1"
PROBLEM_SET="$2"

# ── Paths and constants ─────────────────────────────────────────────────────

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
RUN_DIR="$PROJECT_ROOT/handover/evidence/$RUN_TAG"
MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"
ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
PER_PROBLEM_TIMEOUT_S="${PER_PROBLEM_TIMEOUT_S:-900}"
G_PHASE_N_AGENTS="${TURINGOS_G_PHASE_N_AGENTS:-10}"
G_PHASE_CONDITION="${TURINGOS_G_PHASE_CONDITION:-n${G_PHASE_N_AGENTS}}"
AGENT_MODELS_RAW="${AGENT_MODELS:-}"
PHASE_D_HETERO_OK_VALUE="${PHASE_D_HETERO_OK:-0}"
AGENT_MODELS_HASH="$(printf '%s' "$AGENT_MODELS_RAW" | sha256sum | awk '{print $1}')"
REAL5_ROLE_ASSIGNMENT_RAW="${TURINGOS_REAL5_ROLE_ASSIGNMENT:-}"
REAL5_ROLE_ASSIGNMENT_HASH="$(printf '%s' "$REAL5_ROLE_ASSIGNMENT_RAW" | sha256sum | awk '{print $1}')"
REAL5_ROLE_VIEWS_VALUE="${TURINGOS_REAL5_ROLE_VIEWS:-0}"
if [[ -z "${TURINGOS_G4_REQUIRED_MODEL_FAMILIES:-}" ]]; then
    if [[ "$RUN_TAG" == g_phase_g4_2_* ]]; then
        TURINGOS_G4_REQUIRED_MODEL_FAMILIES=3
    else
        TURINGOS_G4_REQUIRED_MODEL_FAMILIES=0
    fi
fi

model_family_for() {
    local model_lc
    model_lc="$(printf '%s' "$1" | tr '[:upper:]' '[:lower:]')"
    case "$model_lc" in
        *claude*|*anthropic*) echo "claude" ;;
        *gpt*|*openai*|o1*|o3*|o4*) echo "openai" ;;
        *qwen*) echo "qwen" ;;
        *deepseek*) echo "deepseek" ;;
        *gemini*) echo "gemini" ;;
        *llama*|*local*) echo "local" ;;
        *) echo "unknown" ;;
    esac
}

if [[ -n "$AGENT_MODELS_RAW" ]]; then
    IFS=',' read -r -a G4_MODELS <<< "$AGENT_MODELS_RAW"
else
    G4_MODELS=("$ACTIVE_MODEL")
fi
declare -A G4_FAMILIES=()
ASSIGNMENT_SUMMARY=""
for model_entry in "${G4_MODELS[@]}"; do
    model_entry="$(printf '%s' "$model_entry" | xargs)"
    [[ -n "$model_entry" ]] || continue
    family="$(model_family_for "$model_entry")"
    G4_FAMILIES["$family"]=1
    if [[ -z "$ASSIGNMENT_SUMMARY" ]]; then
        ASSIGNMENT_SUMMARY="${model_entry}:${family}"
    else
        ASSIGNMENT_SUMMARY="${ASSIGNMENT_SUMMARY};${model_entry}:${family}"
    fi
done
MODEL_FAMILY_COUNT_OBSERVED="${#G4_FAMILIES[@]}"
if [[ "$TURINGOS_G4_REQUIRED_MODEL_FAMILIES" -gt 0 \
      && "$MODEL_FAMILY_COUNT_OBSERVED" -lt "$TURINGOS_G4_REQUIRED_MODEL_FAMILIES" \
      && "${TURINGOS_G4_SINGLE_MODEL_DIAGNOSTIC:-0}" != "1" ]]; then
    echo "ERROR: G4.2 multi-model evidence requires at least ${TURINGOS_G4_REQUIRED_MODEL_FAMILIES} model families; observed ${MODEL_FAMILY_COUNT_OBSERVED}. Set TURINGOS_G4_SINGLE_MODEL_DIAGNOSTIC=1 only for non-ship diagnostics." >&2
    exit 7
fi

# ── Source .env (DeepSeek key) ──────────────────────────────────────────────

if [[ -f "$PROJECT_ROOT/.env" ]]; then
    set -a
    # shellcheck disable=SC1091
    . "$PROJECT_ROOT/.env"
    set +a
fi
: "${DEEPSEEK_API_KEY:?DEEPSEEK_API_KEY not set; populate .env first}"

# ── Problem set resolution ──────────────────────────────────────────────────

# Canonical TB-N3 Phase 2 9-problem set (matches
# handover/evidence/tb_n3_phase2_2026-05-11T06-58-07Z/tbc0_problems.txt).
TBN3_PHASE2_PROBLEMS=(
    mathd_algebra_107
    mathd_algebra_125
    mathd_algebra_141
    mathd_algebra_113
    mathd_algebra_114
    mathd_numbertheory_1124
    numbertheory_2pownm1prime_nprime
    aime_1983_p1
    aime_1984_p1
)

PROBLEMS_FILE="$RUN_DIR/PROBLEMS.txt"
mkdir -p "$RUN_DIR"

case "$PROBLEM_SET" in
    mini)
        printf '%s\n' "${TBN3_PHASE2_PROBLEMS[@]:0:3}" > "$PROBLEMS_FILE"
        ;;
    full)
        printf '%s\n' "${TBN3_PHASE2_PROBLEMS[@]}" > "$PROBLEMS_FILE"
        ;;
    *)
        if [[ -f "$PROBLEM_SET" ]]; then
            cp "$PROBLEM_SET" "$PROBLEMS_FILE"
        else
            echo "ERROR: PROBLEM_SET '$PROBLEM_SET' is not 'mini', 'full', nor an existing file" >&2
            exit 2
        fi
        ;;
esac

PROBLEM_COUNT=$(grep -cvE '^(#|$)' "$PROBLEMS_FILE")
[[ "$PROBLEM_COUNT" -ge 1 ]] || { echo "ERROR: PROBLEMS.txt is empty" >&2; exit 2; }

# ── Pre-loop validation ─────────────────────────────────────────────────────

echo "════════════════════════════════════════════════════════════════════════"
echo "TB-G G1.2 chain-continuous batch (Option B+ canonical)"
echo "════════════════════════════════════════════════════════════════════════"
echo "RUN_TAG          = $RUN_TAG"
echo "RUN_DIR          = $RUN_DIR"
echo "PROBLEM_SET      = $PROBLEM_SET  ($PROBLEM_COUNT tasks)"
echo "ACTIVE_MODEL     = $ACTIVE_MODEL"
echo "G_PHASE_N_AGENTS = $G_PHASE_N_AGENTS"
echo "G_PHASE_CONDITION= $G_PHASE_CONDITION"
echo "AGENT_MODELS     = ${AGENT_MODELS_RAW:-<empty>}"
echo "PHASE_D_HETERO_OK= $PHASE_D_HETERO_OK_VALUE"
echo "MODEL_FAMILIES   = observed=$MODEL_FAMILY_COUNT_OBSERVED required=$TURINGOS_G4_REQUIRED_MODEL_FAMILIES"
echo "REAL5_ROLES      = ${REAL5_ROLE_ASSIGNMENT_RAW:-<empty>}"
echo "REAL5_ROLE_VIEWS = $REAL5_ROLE_VIEWS_VALUE"
echo "LLM_PROXY_URL    = $LLM_PROXY_URL"
echo "MINIF2F_DIR      = $MINIF2F_DIR"
echo "PER_PROB_TIMEOUT = ${PER_PROBLEM_TIMEOUT_S}s"
echo

# Tree status — non-evidence drift is a /runner-preflight Stage-1 failure
# per CR-18B.9. Backstop here in case the caller did not invoke the gate.
if [[ -n "$(cd "$PROJECT_ROOT" && git status --porcelain | grep -vE '^\?\? handover/evidence/' | head -1)" ]]; then
    echo "WARNING: working tree has non-evidence changes; /runner-preflight Stage 1 normally rejects this." >&2
    if [[ "${TURINGOS_G_PHASE_DIRTY_OK:-0}" != "1" ]]; then
        echo "  Set TURINGOS_G_PHASE_DIRTY_OK=1 to override (smoke testing only)." >&2
        exit 3
    fi
fi

# Disk free check — G1.2-7 9-task batches write multi-GB to runtime_repo +
# CAS. 20G free is the architect-stated minimum.
DISK_FREE_G=$(df -BG --output=avail "$PROJECT_ROOT" | tail -1 | tr -dc '0-9')
if [[ "${DISK_FREE_G:-0}" -lt 20 ]]; then
    echo "WARNING: only ${DISK_FREE_G}G free on $(df -h --output=target "$PROJECT_ROOT" | tail -1); architect minimum is 20G." >&2
    if [[ "${TURINGOS_G_PHASE_LOW_DISK_OK:-0}" != "1" ]]; then
        echo "  Set TURINGOS_G_PHASE_LOW_DISK_OK=1 to override (smoke only)." >&2
        exit 4
    fi
fi

# LLM proxy health probe.
if ! curl -sS --max-time 5 "$LLM_PROXY_URL/health" | grep -q '"status": "ok"'; then
    echo "ERROR: LLM proxy $LLM_PROXY_URL/health not OK" >&2
    exit 5
fi

GIT_HEAD="$(cd "$PROJECT_ROOT" && git rev-parse HEAD 2>/dev/null || echo unknown)"
RUN_TIMESTAMP_UTC="$(date -u +%Y-%m-%dT%H-%M-%SZ)"
BATCH_ID="${RUN_TAG}_${GIT_HEAD:0:7}"

# ── Build release binaries ──────────────────────────────────────────────────

echo "[build] cargo build --release --manifest-path experiments/minif2f_v4/Cargo.toml --bin evaluator --bin batch_evaluator; cargo build --release -p turingosv4 --bin audit_tape --bin tb_g_persistence_report"
BUILD_LOG="$RUN_DIR/cargo_build_release.log"
if ! (cd "$PROJECT_ROOT" && {
    CARGO_TARGET_DIR="$PROJECT_ROOT/target" cargo build --release --manifest-path "$PROJECT_ROOT/experiments/minif2f_v4/Cargo.toml" --bin evaluator --bin batch_evaluator
    cargo build --release -p turingosv4 --bin audit_tape --bin tb_g_persistence_report
} > "$BUILD_LOG" 2>&1); then
    tail -80 "$BUILD_LOG" >&2 || true
    echo "ERROR: release binary build failed; refusing to continue with stale release binaries" >&2
    exit 6
fi
tail -5 "$BUILD_LOG"
EVALUATOR="$PROJECT_ROOT/target/release/evaluator"
BATCH_EVALUATOR="$PROJECT_ROOT/target/release/batch_evaluator"
AUDIT_TAPE="$PROJECT_ROOT/target/release/audit_tape"
PERSISTENCE_REPORT="$PROJECT_ROOT/target/release/tb_g_persistence_report"
for b in "$EVALUATOR" "$BATCH_EVALUATOR" "$AUDIT_TAPE" "$PERSISTENCE_REPORT"; do
    [[ -x "$b" ]] || { echo "ERROR: binary not built: $b" >&2; exit 6; }
done

# ── Manifest header ─────────────────────────────────────────────────────────

cat > "$RUN_DIR/G_PHASE_BATCH_MANIFEST.json" <<EOF
{
  "tb_id": "TB-G",
  "atom": "G1.2-6/7",
  "phase": "G1.2 chain-continuous batch (Option B+ canonical)",
  "ruling": "handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md",
  "run_tag": "$RUN_TAG",
  "batch_id": "$BATCH_ID",
  "problem_count": $PROBLEM_COUNT,
  "problem_set": "$PROBLEM_SET",
  "n_agents": $G_PHASE_N_AGENTS,
  "condition": "$G_PHASE_CONDITION",
  "shared_runtime_repo": "$RUN_DIR/runtime_repo",
  "shared_cas": "$RUN_DIR/cas",
  "active_model": "$ACTIVE_MODEL",
  "agent_models_env_hash": "$AGENT_MODELS_HASH",
  "agent_models_source": "AGENT_MODELS",
  "phase_d_hetero_ok": "$PHASE_D_HETERO_OK_VALUE",
  "assignment_summary": "$ASSIGNMENT_SUMMARY",
  "model_family_count_required": $TURINGOS_G4_REQUIRED_MODEL_FAMILIES,
  "model_family_count_observed": $MODEL_FAMILY_COUNT_OBSERVED,
  "real5_role_assignment": "$REAL5_ROLE_ASSIGNMENT_RAW",
  "real5_role_assignment_hash": "$REAL5_ROLE_ASSIGNMENT_HASH",
  "real5_role_views_enabled": "$REAL5_ROLE_VIEWS_VALUE",
  "llm_proxy_url": "$LLM_PROXY_URL",
  "minif2f_dir": "$MINIF2F_DIR",
  "per_problem_timeout_s": $PER_PROBLEM_TIMEOUT_S,
  "git_head": "$GIT_HEAD",
  "run_timestamp_utc": "$RUN_TIMESTAMP_UTC",
  "resume_semantic": "task_0 = fresh genesis; task_k>0 = TURINGOS_CHAINTAPE_RESUME=1 via batch_evaluator (Option B+ §1)",
  "halt_on": "any subprocess exits non-zero (architect §3.5 halt-and-record)"
}
EOF

# ── Run batch_evaluator ─────────────────────────────────────────────────────

echo
echo "[batch_evaluator] launching $PROBLEM_COUNT-task batch ($BATCH_ID)"
echo "[batch_evaluator] runtime_repo=$RUN_DIR/runtime_repo cas=$RUN_DIR/cas"
START_TS=$(date +%s)

export MINIF2F_DIR
export ACTIVE_MODEL
export AGENT_MODELS="$AGENT_MODELS_RAW"
export PHASE_D_HETERO_OK="$PHASE_D_HETERO_OK_VALUE"
export TURINGOS_G4_REQUIRED_MODEL_FAMILIES
export TURINGOS_REAL5_ROLE_ASSIGNMENT="$REAL5_ROLE_ASSIGNMENT_RAW"
export TURINGOS_REAL5_ROLE_VIEWS="$REAL5_ROLE_VIEWS_VALUE"
export LLM_PROXY_URL
export TURINGOS_TB_N3_AUTO_MARKET="${TURINGOS_TB_N3_AUTO_MARKET:-1}"

# batch_evaluator owns the per-subprocess env. It sets
# TURINGOS_CHAINTAPE_PATH / TURINGOS_CAS_PATH / TURINGOS_RUN_ID /
# TURINGOS_CHAINTAPE_RESUME (the last only for task_k > 0). It does NOT
# set ACTIVE_MODEL / LLM_PROXY_URL / MINIF2F_DIR — those inherit from
# this script's exports above.

"$BATCH_EVALUATOR" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --batch-id "$BATCH_ID" \
    --problems-file "$PROBLEMS_FILE" \
    --model "$ACTIVE_MODEL" \
    --n-agents "$G_PHASE_N_AGENTS" \
    --condition "$G_PHASE_CONDITION" \
    --out-dir "$RUN_DIR" \
    --evaluator-bin "$EVALUATOR" \
    --minif2f-dir "$MINIF2F_DIR/MiniF2F/Test" \
    --llm-proxy-url "$LLM_PROXY_URL" \
    --per-task-timeout-s "$PER_PROBLEM_TIMEOUT_S" \
    2>&1 | tee "$RUN_DIR/batch_evaluator.log"
BATCH_EXIT=${PIPESTATUS[0]}

ELAPSED=$(($(date +%s) - START_TS))
echo "[batch_evaluator] exit_code=$BATCH_EXIT elapsed=${ELAPSED}s"

CAS_CHAIN_LOCK_PATH="$RUN_DIR/cas/.turingos_cas_chain.lock"
CAS_STALE_LOCK_CLEANUP_LOG="$RUN_DIR/cas_stale_lock_cleanup.log"
# Reads pid= from the exact CAS lock path and preserves live locks with kill -0.
clear_stale_cas_chain_lock "$RUN_DIR/cas" "$CAS_CHAIN_LOCK_PATH" "$CAS_STALE_LOCK_CLEANUP_LOG"

# ── Aggregate audit_tape over the shared chain ──────────────────────────────

echo
echo "[audit_tape] running over shared runtime_repo + cas"
( cd "$PROJECT_ROOT" && "$AUDIT_TAPE" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas-dir "$RUN_DIR/cas" \
    --agent-pubkeys "$RUN_DIR/runtime_repo/agent_pubkeys.json" \
    --pinned-pubkeys "$RUN_DIR/runtime_repo/pinned_pubkeys.json" \
    --genesis "$PROJECT_ROOT/genesis_payload.toml" \
    --constitution "$PROJECT_ROOT/constitution.md" \
    --alignment-dir "$PROJECT_ROOT/handover/alignment" \
    --out "$RUN_DIR/aggregate_verdict.json" \
    ) 2> "$RUN_DIR/audit_tape.stderr"
AUDIT_EXIT=$?

VERDICT="$(grep -E '"verdict"' "$RUN_DIR/aggregate_verdict.json" 2>/dev/null | head -1 | sed -E 's/.*"verdict": *"([^"]+)".*/\1/')"
echo "[audit_tape] exit=$AUDIT_EXIT verdict=$VERDICT"

# ── Persistence binding report (Codex Q6 closure) ───────────────────────────

echo
echo "[persistence_report] running tb_g_persistence_report"
"$PERSISTENCE_REPORT" --run-dir "$RUN_DIR" \
    > "$RUN_DIR/persistence_report.stdout" \
    2> "$RUN_DIR/persistence_report.stderr"
PERSISTENCE_EXIT=$?
PERSISTENCE_PASSING="$(grep -oE 'is_passing=(true|false)' "$RUN_DIR/persistence_report.stdout" 2>/dev/null | head -1 | sed -E 's/.*=(true|false)/\1/')"
PERSISTENCE_N_WITNESSED="$(grep -oE 'n_witnessed=[0-9]+' "$RUN_DIR/persistence_report.stdout" 2>/dev/null | head -1 | sed -E 's/.*=([0-9]+)/\1/')"
echo "[persistence_report] exit=$PERSISTENCE_EXIT is_passing=$PERSISTENCE_PASSING n_witnessed=$PERSISTENCE_N_WITNESSED"

# ── Run log (canonical; final after every artifact settles) ─────────────────

{
    echo "batch_id            $BATCH_ID"
    echo "run_tag             $RUN_TAG"
    echo "problem_count       $PROBLEM_COUNT"
    echo "git_head            $GIT_HEAD"
    echo "elapsed_s           $ELAPSED"
    echo "batch_exit          $BATCH_EXIT"
    echo "audit_exit          $AUDIT_EXIT"
    echo "audit_verdict       $VERDICT"
    echo "persistence_exit    $PERSISTENCE_EXIT"
    echo "persistence_passing $PERSISTENCE_PASSING"
    echo "persistence_n_witnessed $PERSISTENCE_N_WITNESSED"
} > "$RUN_DIR/run_log.txt"

echo
echo "════════════════════════════════════════════════════════════════════════"
echo "Done. Evidence dir: $RUN_DIR"
echo "  - BatchContinuationManifest.json   (G1.2-4 fact-identity)"
echo "  - aggregate_verdict.json verdict=$VERDICT"
echo "  - PERSISTENCE_BINDING_REPORT.json  is_passing=$PERSISTENCE_PASSING (n_witnessed=$PERSISTENCE_N_WITNESSED)"
echo "  - run_log.txt"
echo "════════════════════════════════════════════════════════════════════════"

# Exit non-zero on any failure mode (batch crash OR audit non-PROCEED OR persistence Reset).
if [[ "$BATCH_EXIT" -ne 0 ]]; then
    exit "$BATCH_EXIT"
fi
if [[ "$VERDICT" != "PROCEED" ]]; then
    echo "WARNING: aggregate audit_tape verdict=$VERDICT (not PROCEED)" >&2
    exit 1
fi
if [[ "$PERSISTENCE_PASSING" != "true" ]]; then
    echo "WARNING: persistence binding has Reset verdict(s) (kill-criterion #1)" >&2
    exit 1
fi
exit 0
