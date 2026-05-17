#!/usr/bin/env bash
# REAL-16 -- Market Performance / E4 Candidate Benchmark.
#
# Arms:
#   A: baseline market-visible
#   B: EV scaffold
#   C: EV + BCAST
#   D: EV + BCAST + PnL + role-specialized action-conversion view
#
# This is Constitutional Research Mode evidence only. It does not claim E4
# achieved, market emergence proven, or market mechanism shipped.

set -uo pipefail

usage() {
    cat <<'USAGE'
usage: scripts/run_real16_market_performance_benchmark.sh \
  [--problems handover/preregistration/sample_E1v2_hard10_S20260423.txt] \
  [--models <model_manifest.env>] \
  [--budgets <budget_manifest.env>] \
  [--arms A,B,C,D] \
  --out handover/evidence/market_autonomy_lab_real16_<UTC>

The default hard10 problem set is:
  handover/preregistration/sample_E1v2_hard10_S20260423.txt
  sha256=138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc

Forbidden mechanisms guarded here:
  forced trade
  price-as-truth
  ghost liquidity
  PolicyTrader action counted as E4
  live REAL-6B
  scripted attempt prediction fixtures
  scripted task-outcome buys
USAGE
}

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EVIDENCE_ROOT="$PROJECT_ROOT/handover/evidence"
DEFAULT_PROBLEMS="$PROJECT_ROOT/handover/preregistration/sample_E1v2_hard10_S20260423.txt"
DEFAULT_PROBLEMS_SHA="138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc"

PROBLEMS="$DEFAULT_PROBLEMS"
MODELS=""
BUDGETS=""
ARMS="A,B,C,D"
OUT=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --problems) PROBLEMS="${2:-}"; shift 2 ;;
        --models) MODELS="${2:-}"; shift 2 ;;
        --budgets) BUDGETS="${2:-}"; shift 2 ;;
        --arms) ARMS="${2:-}"; shift 2 ;;
        --out) OUT="${2:-}"; shift 2 ;;
        -h|--help) usage; exit 0 ;;
        *) echo "ERROR: unknown arg: $1" >&2; usage >&2; exit 2 ;;
    esac
done

[[ -n "$OUT" ]] || { echo "ERROR: --out required" >&2; exit 2; }
[[ -f "$PROBLEMS" ]] || { echo "ERROR: problems file missing: $PROBLEMS" >&2; exit 2; }

case "$OUT" in
    "$PROJECT_ROOT"/*) OUT_ABS="$OUT" ;;
    /*) OUT_ABS="$OUT" ;;
    *) OUT_ABS="$PROJECT_ROOT/$OUT" ;;
esac
mkdir -p "$OUT_ABS/arm_config_manifests"

PROBLEMS_ABS="$(cd "$(dirname "$PROBLEMS")" && pwd)/$(basename "$PROBLEMS")"
PROBLEM_SET_HASH="$(sha256sum "$PROBLEMS_ABS" | awk '{print $1}')"
if [[ "$(basename "$PROBLEMS_ABS")" == "sample_E1v2_hard10_S20260423.txt" ]] \
    && [[ "$PROBLEM_SET_HASH" != "$DEFAULT_PROBLEMS_SHA" ]]; then
    echo "ERROR: sample_E1v2_hard10_S20260423.txt hash mismatch: $PROBLEM_SET_HASH" >&2
    exit 2
fi

if [[ "${TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION:-0}" == "1" ]]; then
    echo "ERROR: TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION is forbidden for REAL-16 without separate ratification" >&2
    exit 2
fi
if [[ "${TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE:-0}" == "1" ]]; then
    echo "ERROR: TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE is forbidden for REAL-16 E4 evidence" >&2
    exit 2
fi
if [[ -n "${TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS:-}" ]]; then
    echo "ERROR: TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS is forbidden for REAL-16 E4 evidence" >&2
    exit 2
fi

if [[ -z "$MODELS" ]]; then
    MODELS="$OUT_ABS/model_assignment.pinned.env"
    {
        printf 'ACTIVE_MODEL=%s\n' "${ACTIVE_MODEL:-deepseek-chat}"
        printf 'AGENT_MODELS=%s\n' "${AGENT_MODELS:-}"
        printf 'PHASE_D_HETERO_OK=%s\n' "${PHASE_D_HETERO_OK:-1}"
        printf 'TURINGOS_REAL5_ROLE_ASSIGNMENT=%s\n' "${TURINGOS_REAL5_ROLE_ASSIGNMENT:-BullTrader,BearTrader,Solver,Verifier,Challenger}"
        printf 'TURINGOS_G_PHASE_N_AGENTS=%s\n' "${TURINGOS_G_PHASE_N_AGENTS:-5}"
        printf 'LLM_PROXY_URL=%s\n' "${LLM_PROXY_URL:-http://localhost:8080}"
    } > "$MODELS"
fi
if [[ -z "$BUDGETS" ]]; then
    BUDGETS="$OUT_ABS/budgets.pinned.env"
    {
        printf 'MAX_TRANSACTIONS=%s\n' "${MAX_TRANSACTIONS:-30}"
        printf 'PER_PROBLEM_TIMEOUT_S=%s\n' "${PER_PROBLEM_TIMEOUT_S:-900}"
        printf 'TURINGOS_REAL6A_POLL_BUDGET_MS=%s\n' "${TURINGOS_REAL6A_POLL_BUDGET_MS:-30000}"
        printf 'TURINGOS_REAL13_CANDIDATE_AMOUNT_MICRO=%s\n' "${TURINGOS_REAL13_CANDIDATE_AMOUNT_MICRO:-1000}"
        printf 'TURINGOS_REAL13_POLICY_TRADER_THRESHOLD_BPS=%s\n' "${TURINGOS_REAL13_POLICY_TRADER_THRESHOLD_BPS:-0}"
    } > "$BUDGETS"
fi

cp "$PROBLEMS_ABS" "$OUT_ABS/problems.pinned.txt"
cp "$MODELS" "$OUT_ABS/model_assignment.pinned.env"
cp "$BUDGETS" "$OUT_ABS/budgets.pinned.env"

MODEL_ASSIGNMENT_HASH="$(sha256sum "$OUT_ABS/model_assignment.pinned.env" | awk '{print $1}')"
BUDGET_HASH="$(sha256sum "$OUT_ABS/budgets.pinned.env" | awk '{print $1}')"
PROMPT_TEMPLATE_HASH="$(sha256sum "$PROJECT_ROOT/experiments/minif2f_v4/src/bin/evaluator.rs" | awk '{print $1}')"
RUNTIME_CONFIG_HASH="$(
    {
        sha256sum "$PROJECT_ROOT/scripts/run_real16_market_performance_benchmark.sh"
        sha256sum "$PROJECT_ROOT/scripts/run_g_phase_batch.sh"
        sha256sum "$PROJECT_ROOT/src/runtime/market_performance_e4.rs"
        sha256sum "$PROJECT_ROOT/src/bin/real16_market_performance_verifier.rs"
    } | sha256sum | awk '{print $1}'
)"

cat > "$OUT_ABS/arm_config_manifests/arm_diff_allowlist.txt" <<'EOF'
ARM
ARM_CONDITION
RUN_TAG
RUN_DIR
TURINGOS_REAL13_EV_DECISION_TRACE
TURINGOS_REAL13_TRADER_EV_SCAFFOLD
TURINGOS_REAL11_MARKET_OPPORTUNITY_TRACE
TURINGOS_REAL_BCAST_LIBRARIAN
TURINGOS_REAL11_TRADER_PNL_VIEW
TURINGOS_REAL14G_ACTION_CONVERSION_VIEW
EOF

read_manifest_value() {
    local file="$1"
    local key="$2"
    awk -F= -v k="$key" '
        /^[[:space:]]*#/ { next }
        /^[[:space:]]*$/ { next }
        {
            gsub(/^[[:space:]]+|[[:space:]]+$/, "", $1)
            if ($1 == k) {
                v = substr($0, index($0, "=") + 1)
                gsub(/^[[:space:]]+|[[:space:]]+$/, "", v)
                print v
                exit
            }
        }
    ' "$file"
}

ACTIVE_MODEL_PIN="$(read_manifest_value "$OUT_ABS/model_assignment.pinned.env" ACTIVE_MODEL)"
AGENT_MODELS_PIN="$(read_manifest_value "$OUT_ABS/model_assignment.pinned.env" AGENT_MODELS)"
PHASE_D_HETERO_OK_PIN="$(read_manifest_value "$OUT_ABS/model_assignment.pinned.env" PHASE_D_HETERO_OK)"
ROLE_ASSIGNMENT_PIN="$(read_manifest_value "$OUT_ABS/model_assignment.pinned.env" TURINGOS_REAL5_ROLE_ASSIGNMENT)"
N_AGENTS_PIN="$(read_manifest_value "$OUT_ABS/model_assignment.pinned.env" TURINGOS_G_PHASE_N_AGENTS)"
LLM_PROXY_URL_PIN="$(read_manifest_value "$OUT_ABS/model_assignment.pinned.env" LLM_PROXY_URL)"
MAX_TX_PIN="$(read_manifest_value "$OUT_ABS/budgets.pinned.env" MAX_TRANSACTIONS)"
TIMEOUT_PIN="$(read_manifest_value "$OUT_ABS/budgets.pinned.env" PER_PROBLEM_TIMEOUT_S)"
POLL_PIN="$(read_manifest_value "$OUT_ABS/budgets.pinned.env" TURINGOS_REAL6A_POLL_BUDGET_MS)"
CANDIDATE_AMOUNT_PIN="$(read_manifest_value "$OUT_ABS/budgets.pinned.env" TURINGOS_REAL13_CANDIDATE_AMOUNT_MICRO)"
POLICY_THRESHOLD_PIN="$(read_manifest_value "$OUT_ABS/budgets.pinned.env" TURINGOS_REAL13_POLICY_TRADER_THRESHOLD_BPS)"

arm_condition() {
    case "$1" in
        A) echo "baseline market-visible" ;;
        B) echo "EV scaffold" ;;
        C) echo "EV + BCAST" ;;
        D) echo "EV + BCAST + PnL + role-specialized action-conversion view" ;;
        *) echo "unknown" ;;
    esac
}

run_tag_prefix() {
    if [[ "$OUT_ABS" == "$EVIDENCE_ROOT"/* ]]; then
        echo "${OUT_ABS#$EVIDENCE_ROOT/}"
    else
        basename "$OUT_ABS"
    fi
}

ARM_JSON_DIR="$OUT_ABS/arm_metrics"
mkdir -p "$ARM_JSON_DIR"
ARM_FAILURES=0
IFS=',' read -r -a ARM_LIST <<< "$ARMS"
for raw_arm in "${ARM_LIST[@]}"; do
    arm="$(printf '%s' "$raw_arm" | xargs)"
    [[ -n "$arm" ]] || continue
    case "$arm" in A|B|C|D) ;; *) echo "ERROR: unsupported arm: $arm" >&2; exit 2 ;; esac

    run_tag="$(run_tag_prefix)/arm_${arm}"
    run_dir="$EVIDENCE_ROOT/$run_tag"
    condition="$(arm_condition "$arm")"
    echo "[real16] running arm $arm: $condition"

    export ACTIVE_MODEL="$ACTIVE_MODEL_PIN"
    export PHASE_D_HETERO_OK="${PHASE_D_HETERO_OK_PIN:-1}"
    export TURINGOS_G_PHASE_N_AGENTS="${N_AGENTS_PIN:-5}"
    export LLM_PROXY_URL="${LLM_PROXY_URL_PIN:-http://localhost:8080}"
    export TURINGOS_REAL5_ROLE_ASSIGNMENT="${ROLE_ASSIGNMENT_PIN:-BullTrader,BearTrader,Solver,Verifier,Challenger}"
    export TURINGOS_REAL5_ROLE_VIEWS=1
    export TURINGOS_G_PHASE_DIRTY_OK=1
    export TURINGOS_G_PHASE_LOW_DISK_OK=1
    export MAX_TRANSACTIONS="${MAX_TX_PIN:-30}"
    export PER_PROBLEM_TIMEOUT_S="${TIMEOUT_PIN:-900}"
    export TURINGOS_REAL6A_POLL_BUDGET_MS="${POLL_PIN:-30000}"
    export TURINGOS_REAL13_CANDIDATE_AMOUNT_MICRO="${CANDIDATE_AMOUNT_PIN:-1000}"
    export TURINGOS_REAL13_POLICY_TRADER_THRESHOLD_BPS="${POLICY_THRESHOLD_PIN:-0}"
    export TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION=0
    export TURINGOS_REAL11_NO_SCRIPTED_BUYS=1
    export TURINGOS_REAL12_TASK_MARKET_AFFORDANCE=1
    export TURINGOS_REAL12_TRADER_OBJECTIVE=1
    export TURINGOS_REAL6_TASK_OUTCOME_MARKET=1
    export TURINGOS_TB_N3_AUTO_MARKET=1
    export TURINGOS_MARKET_REVIEW_MODE=sequential
    export TURINGOS_REAL13_DISPLAY_COIN=1
    export TURINGOS_REAL13_SIGNAL_PURIFICATION=1
    export TURINGOS_REAL13_POLICY_TRADER=1
    export TURINGOS_REAL11_MARKET_OPPORTUNITY_TRACE=1
    if [[ -n "${AGENT_MODELS_PIN:-}" ]]; then
        export AGENT_MODELS="$AGENT_MODELS_PIN"
    else
        unset AGENT_MODELS
    fi
    unset TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE
    unset TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS
    unset TURINGOS_FORCE_BOLTZMANN_SEED_WORKTXS
    unset TURINGOS_REAL7_SCRIPTED_VERIFY_CHALLENGE
    unset TURINGOS_REAL_BCAST_LIBRARIAN
    unset TURINGOS_REAL11_TRADER_PNL_VIEW
    unset TURINGOS_REAL14G_ACTION_CONVERSION_VIEW
    export TURINGOS_REAL13_EV_DECISION_TRACE=0
    export TURINGOS_REAL13_TRADER_EV_SCAFFOLD=0

    case "$arm" in
        A)
            ;;
        B)
            export TURINGOS_REAL13_EV_DECISION_TRACE=1
            export TURINGOS_REAL13_TRADER_EV_SCAFFOLD=1
            ;;
        C)
            export TURINGOS_REAL13_EV_DECISION_TRACE=1
            export TURINGOS_REAL13_TRADER_EV_SCAFFOLD=1
            export TURINGOS_REAL_BCAST_LIBRARIAN=1
            ;;
        D)
            export TURINGOS_REAL13_EV_DECISION_TRACE=1
            export TURINGOS_REAL13_TRADER_EV_SCAFFOLD=1
            export TURINGOS_REAL_BCAST_LIBRARIAN=1
            export TURINGOS_REAL11_TRADER_PNL_VIEW=1
            export TURINGOS_REAL14G_ACTION_CONVERSION_VIEW=1
            ;;
    esac

    {
        printf 'ARM=%s\n' "$arm"
        printf 'ARM_CONDITION=%s\n' "$condition"
        printf 'RUN_TAG=%s\n' "$run_tag"
        printf 'RUN_DIR=%s\n' "$run_dir"
        printf 'TURINGOS_REAL13_EV_DECISION_TRACE=%s\n' "$TURINGOS_REAL13_EV_DECISION_TRACE"
        printf 'TURINGOS_REAL13_TRADER_EV_SCAFFOLD=%s\n' "$TURINGOS_REAL13_TRADER_EV_SCAFFOLD"
        printf 'TURINGOS_REAL11_MARKET_OPPORTUNITY_TRACE=%s\n' "$TURINGOS_REAL11_MARKET_OPPORTUNITY_TRACE"
        printf 'TURINGOS_REAL_BCAST_LIBRARIAN=%s\n' "${TURINGOS_REAL_BCAST_LIBRARIAN:-0}"
        printf 'TURINGOS_REAL11_TRADER_PNL_VIEW=%s\n' "${TURINGOS_REAL11_TRADER_PNL_VIEW:-0}"
        printf 'TURINGOS_REAL14G_ACTION_CONVERSION_VIEW=%s\n' "${TURINGOS_REAL14G_ACTION_CONVERSION_VIEW:-0}"
    } > "$OUT_ABS/arm_config_manifests/arm_${arm}_toggles.env"

    bash "$PROJECT_ROOT/scripts/run_g_phase_batch.sh" "$run_tag" "$OUT_ABS/problems.pinned.txt"
    exit_code=$?
    if [[ "$exit_code" -ne 0 ]]; then
        ARM_FAILURES=$((ARM_FAILURES + 1))
    fi

    e2_json="$run_dir/REAL16_ARM_${arm}_E2_VERIFIER.json"
    e2_md="$run_dir/REAL16_ARM_${arm}_E2_VERIFIER.md"
    cargo run --quiet --bin real14_e2_candidate_verifier -- \
        --repo "$run_dir/runtime_repo" \
        --cas "$run_dir/cas" \
        --json-out "$e2_json" \
        --md-out "$e2_md"
    e2_exit=$?
    if [[ "$e2_exit" -ne 0 ]]; then
        ARM_FAILURES=$((ARM_FAILURES + 1))
    fi

    cargo run --quiet --bin real16_market_performance_verifier -- \
        --derive-arm-json \
        --arm-id "$arm" \
        --evidence-dir "$run_dir" \
        --e2-json "$e2_json" \
        --problem-set-hash "$PROBLEM_SET_HASH" \
        --model-assignment-hash "$MODEL_ASSIGNMENT_HASH" \
        --budget-hash "$BUDGET_HASH" \
        --prompt-template-hash "$PROMPT_TEMPLATE_HASH" \
        --runtime-config-hash "$RUNTIME_CONFIG_HASH" \
        --market-pressure-enabled "$([[ "$arm" == "A" ]] && echo false || echo true)" \
        --json-out "$ARM_JSON_DIR/arm_${arm}.json"
    arm_derive_exit=$?
    if [[ "$arm_derive_exit" -ne 0 ]]; then
        ARM_FAILURES=$((ARM_FAILURES + 1))
    fi
    arm_e2_verifier_verdict="$(jq -r '.e2_verifier_verdict // "missing"' "$ARM_JSON_DIR/arm_${arm}.json" 2>/dev/null || echo missing)"
    if [[ "$arm_e2_verifier_verdict" != "PROCEED" ]]; then
        echo "REAL-16 arm $arm checkpoint: e2_verifier_verdict=$arm_e2_verifier_verdict" >&2
        ARM_FAILURES=$((ARM_FAILURES + 1))
    fi
done

jq -s '{arms: .}' "$ARM_JSON_DIR"/arm_*.json > "$OUT_ABS/REAL16_VERIFIER_INPUT.json"

cargo run --quiet --bin real16_market_performance_verifier -- \
    --input-json "$OUT_ABS/REAL16_VERIFIER_INPUT.json" \
    --json-out "$OUT_ABS/REAL16_MARKET_PERFORMANCE_REPORT.json" \
    --md-out "$OUT_ABS/REAL16_MARKET_PERFORMANCE_REPORT.md"
verifier_exit=$?
if [[ "$verifier_exit" -ne 0 ]]; then
    ARM_FAILURES=$((ARM_FAILURES + 1))
fi

cat > "$OUT_ABS/REAL16_FORBIDDEN_CLAIMS.txt" <<'EOF'
No E4 achieved.
No market emergence proven.
No market mechanism shipped.
No forced trade.
No price-as-truth.
No ghost liquidity.
No PolicyTrader action counted as E4.
EOF

if [[ "$ARM_FAILURES" -ne 0 ]]; then
    echo "REAL-16 benchmark completed with $ARM_FAILURES failure/checkpoint(s); see $OUT_ABS" >&2
    exit 1
fi

echo "REAL-16 benchmark PASS: $OUT_ABS"
