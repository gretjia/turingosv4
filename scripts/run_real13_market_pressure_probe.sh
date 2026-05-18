#!/usr/bin/env bash
# REAL-13H — live integrated market-pressure probe.
#
# This wraps the audited REAL-12 task-market probe and adds REAL-13A/B/C/D
# sentinels: EVDecisionTrace sidecars, sequential MarketReviewWindow evidence,
# DisplayCoin/EV cognitive bridge flags, and no live REAL-6B/scripted buys.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EVIDENCE_ROOT="$PROJECT_ROOT/handover/evidence"

RUN_TAG="${1:-real13_market_pressure_probe_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_TAG="${RUN_TAG#handover/evidence/}"
RUN_DIR="$EVIDENCE_ROOT/$RUN_TAG"

is_truthy() {
    case "${1:-0}" in
        1|true|TRUE|True|yes|YES) return 0 ;;
        *) return 1 ;;
    esac
}

if is_truthy "${TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION:-0}"; then
    echo "ERROR: live REAL-6B is not authorized in REAL-13 market-pressure probe" >&2
    exit 2
fi
if [[ "${TURINGOS_MARKET_REVIEW_MODE:-sequential}" == "full_async_experimental" ]] \
    && ! is_truthy "${TURINGOS_UNSAFE_RESEARCH:-0}"; then
    echo "ERROR: full async market review requires TURINGOS_UNSAFE_RESEARCH=1" >&2
    exit 2
fi
if is_truthy "${TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE:-0}"; then
    echo "ERROR: scripted AttemptPrediction fixture is forbidden in REAL-13 probe" >&2
    exit 2
fi
if [[ -n "${TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS:-}" ]]; then
    echo "ERROR: scripted TaskOutcome buys are forbidden in REAL-13 probe" >&2
    exit 2
fi

export TURINGOS_REAL13_EV_DECISION_TRACE=1
export TURINGOS_MARKET_REVIEW_MODE="${TURINGOS_MARKET_REVIEW_MODE:-sequential}"
export TURINGOS_REAL13_DISPLAY_COIN=1
export TURINGOS_REAL13_SIGNAL_PURIFICATION=1
export TURINGOS_REAL13_TRADER_EV_SCAFFOLD="${TURINGOS_REAL13_TRADER_EV_SCAFFOLD:-1}"
export TURINGOS_REAL12_TASK_MARKET_AFFORDANCE=1
export TURINGOS_REAL12_TRADER_OBJECTIVE="${TURINGOS_REAL12_TRADER_OBJECTIVE:-1}"
export TURINGOS_REAL5_ROLE_ASSIGNMENT="${TURINGOS_REAL5_ROLE_ASSIGNMENT:-BullTrader,BearTrader,Solver,Verifier,Challenger}"
export TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION=0
export TURINGOS_REAL11_NO_SCRIPTED_BUYS=1
unset TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS

bash "$PROJECT_ROOT/scripts/run_real12_task_market_probe.sh" "$RUN_TAG"

CONFIG_JSON="$RUN_DIR/REAL14F_RUNTIME_CONFIG.json"
CONFIG_SHA="$RUN_DIR/REAL14F_RUNTIME_CONFIG.sha256"
problem_set_hash="$(sha256sum "$RUN_DIR/PROBLEMS.txt" | awk '{print $1}')"
model_assignment_hash="$(
    {
        printf 'ACTIVE_MODEL=%s\n' "${ACTIVE_MODEL:-}"
        printf 'AGENT_MODELS=%s\n' "${AGENT_MODELS:-}"
        printf 'TURINGOS_REAL5_ROLE_ASSIGNMENT=%s\n' "$TURINGOS_REAL5_ROLE_ASSIGNMENT"
        printf 'TURINGOS_G_PHASE_N_AGENTS=%s\n' "${TURINGOS_G_PHASE_N_AGENTS:-}"
    } | sha256sum | awk '{print $1}'
)"
budget_config_hash="$(
    {
        printf 'BUDGET_REGIME=%s\n' "${BUDGET_REGIME:-}"
        printf 'MAX_TRANSACTIONS=%s\n' "${MAX_TRANSACTIONS:-}"
        printf 'PER_PROBLEM_TIMEOUT_S=%s\n' "${PER_PROBLEM_TIMEOUT_S:-}"
        printf 'TURINGOS_REAL13_CANDIDATE_AMOUNT_MICRO=%s\n' "${TURINGOS_REAL13_CANDIDATE_AMOUNT_MICRO:-}"
        printf 'TURINGOS_REAL13_POLICY_TRADER_THRESHOLD_BPS=%s\n' "${TURINGOS_REAL13_POLICY_TRADER_THRESHOLD_BPS:-}"
        printf 'TURINGOS_REAL6_TASK_OUTCOME_SEED_MICRO=%s\n' "${TURINGOS_REAL6_TASK_OUTCOME_SEED_MICRO:-}"
        printf 'TURINGOS_DEFAULT_POOL_SEED_MICRO=%s\n' "${TURINGOS_DEFAULT_POOL_SEED_MICRO:-}"
    } | sha256sum | awk '{print $1}'
)"
prompt_template_hash="$(sha256sum "$PROJECT_ROOT/experiments/minif2f_v4/src/bin/evaluator.rs" | awk '{print $1}')"
runtime_source_hash="$(
    {
        sha256sum "$PROJECT_ROOT/scripts/run_real13_market_pressure_probe.sh"
        sha256sum "$PROJECT_ROOT/scripts/run_real12_task_market_probe.sh"
        sha256sum "$PROJECT_ROOT/scripts/run_g_phase_batch.sh"
        sha256sum "$PROJECT_ROOT/src/bin/audit_dashboard.rs"
        sha256sum "$PROJECT_ROOT/src/runtime/ev_decision_trace.rs"
    } | sha256sum | awk '{print $1}'
)"
jq -n \
    --arg schema_version "real14f.runtime_config.v1" \
    --arg run_tag "$RUN_TAG" \
    --arg problem_set "${REAL12_PROBLEM_SET:-mini}" \
    --arg problem_set_hash "$problem_set_hash" \
    --arg active_model "${ACTIVE_MODEL:-}" \
    --arg agent_models "${AGENT_MODELS:-}" \
    --arg model_assignment_hash "$model_assignment_hash" \
    --arg budget_regime "${BUDGET_REGIME:-}" \
    --arg max_transactions "${MAX_TRANSACTIONS:-}" \
    --arg per_problem_timeout_s "${PER_PROBLEM_TIMEOUT_S:-}" \
    --arg candidate_amount_micro "${TURINGOS_REAL13_CANDIDATE_AMOUNT_MICRO:-}" \
    --arg policy_threshold_bps "${TURINGOS_REAL13_POLICY_TRADER_THRESHOLD_BPS:-}" \
    --arg budget_config_hash "$budget_config_hash" \
    --arg prompt_template_hash "$prompt_template_hash" \
    --arg runtime_source_hash "$runtime_source_hash" \
    --arg market_review_mode "$TURINGOS_MARKET_REVIEW_MODE" \
    --arg ev_scaffold "$TURINGOS_REAL13_TRADER_EV_SCAFFOLD" \
    --arg bcast_librarian "${TURINGOS_REAL_BCAST_LIBRARIAN:-0}" \
    --arg role_assignment "$TURINGOS_REAL5_ROLE_ASSIGNMENT" \
    --arg task_market_seed_micro "${TURINGOS_REAL6_TASK_OUTCOME_SEED_MICRO:-}" \
    --arg default_pool_seed_micro "${TURINGOS_DEFAULT_POOL_SEED_MICRO:-}" \
    '{
      schema_version: $schema_version,
      run_tag: $run_tag,
      problem_set: $problem_set,
      problem_set_hash: $problem_set_hash,
      active_model: $active_model,
      agent_models: $agent_models,
      model_assignment_hash: $model_assignment_hash,
      budget_regime: $budget_regime,
      max_transactions: $max_transactions,
      per_problem_timeout_s: $per_problem_timeout_s,
      candidate_amount_micro: $candidate_amount_micro,
      policy_threshold_bps: $policy_threshold_bps,
      budget_config_hash: $budget_config_hash,
      prompt_template_hash: $prompt_template_hash,
      runtime_source_hash: $runtime_source_hash,
      market_review_mode: $market_review_mode,
      ev_scaffold: $ev_scaffold,
      bcast_librarian: $bcast_librarian,
      role_assignment: $role_assignment,
      task_market_seed_micro: $task_market_seed_micro,
      default_pool_seed_micro: $default_pool_seed_micro
    }' > "$CONFIG_JSON"
config_hash="$(sha256sum "$CONFIG_JSON" | awk '{print $1}')"
printf '%s  %s\n' "$config_hash" "$(basename "$CONFIG_JSON")" > "$CONFIG_SHA"

DASH="$RUN_DIR/audit_dashboard_run_report.txt"
REPORT="$RUN_DIR/REAL13_MARKET_PRESSURE_PROBE_REPORT.md"
ROOT_REPORT="$PROJECT_ROOT/handover/reports/REAL13_MARKET_PRESSURE_PROBE_REPORT.md"

dash_metric() {
    local key="$1"
    awk -F': ' -v key="$key" '$1 ~ key"$" {
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2);
        print $2;
        found=1;
        exit
    } END { if (!found) print "" }' "$DASH"
}

audit_verdict="$(jq -r '.verdict // "missing"' "$RUN_DIR/aggregate_verdict.json" 2>/dev/null || echo missing)"
agent_economic_action_tx_count="$(dash_metric agent_economic_action_tx_count)"
agent_economic_action_tx_count="${agent_economic_action_tx_count:-0}"
live_non_scripted_router_tx_count="$agent_economic_action_tx_count"
ev_decision_trace_total_cas="$(dash_metric ev_decision_trace_total_cas)"
ev_decision_trace_total_cas="${ev_decision_trace_total_cas:-0}"
ev_decision_trace_bull_count_cas="$(dash_metric ev_decision_trace_bull_count_cas)"
ev_decision_trace_bull_count_cas="${ev_decision_trace_bull_count_cas:-0}"
ev_decision_trace_bear_count_cas="$(dash_metric ev_decision_trace_bear_count_cas)"
ev_decision_trace_bear_count_cas="${ev_decision_trace_bear_count_cas:-0}"
ev_decision_trace_buy_yes_count_cas="$(dash_metric ev_decision_trace_buy_yes_count_cas)"
ev_decision_trace_buy_yes_count_cas="${ev_decision_trace_buy_yes_count_cas:-0}"
ev_decision_trace_buy_no_count_cas="$(dash_metric ev_decision_trace_buy_no_count_cas)"
ev_decision_trace_buy_no_count_cas="${ev_decision_trace_buy_no_count_cas:-0}"
ev_decision_trace_abstain_count_cas="$(dash_metric ev_decision_trace_abstain_count_cas)"
ev_decision_trace_abstain_count_cas="${ev_decision_trace_abstain_count_cas:-0}"
market_review_summary_cas_count="$(dash_metric market_review_summary_cas_count)"
market_review_summary_cas_count="${market_review_summary_cas_count:-0}"
ev_public_basis_available_count="$(dash_metric ev_public_basis_available_count)"
ev_public_basis_available_count="${ev_public_basis_available_count:-0}"
ev_public_basis_missing_count="$(dash_metric ev_public_basis_missing_count)"
ev_public_basis_missing_count="${ev_public_basis_missing_count:-0}"
ev_public_basis_delivery_rate_bps="$(dash_metric ev_public_basis_delivery_rate_bps)"
ev_public_basis_delivery_rate_bps="${ev_public_basis_delivery_rate_bps:-0}"
policy_trader_trace_total_cas="$(dash_metric policy_trader_trace_total_cas)"
policy_trader_trace_total_cas="${policy_trader_trace_total_cas:-0}"
policy_positive_ev_count="$(dash_metric policy_positive_ev_count)"
policy_positive_ev_count="${policy_positive_ev_count:-0}"
policy_positive_ev_llm_abstained_count="$(dash_metric policy_positive_ev_llm_abstained_count)"
policy_positive_ev_llm_abstained_count="${policy_positive_ev_llm_abstained_count:-0}"
policy_insufficient_public_basis_count="$(dash_metric policy_insufficient_public_basis_count)"
policy_insufficient_public_basis_count="${policy_insufficient_public_basis_count:-0}"
policy_counts_for_e2="$(dash_metric policy_counts_for_e2)"
policy_counts_for_e2="${policy_counts_for_e2:-false}"

e2_verdict="E2 NOT ACHIEVED"
if [[ "$live_non_scripted_router_tx_count" -gt 0 ]]; then
    e2_verdict="E2 candidate pending audit"
fi

cat > "$REPORT" <<EOF
# REAL-13 Market Pressure Probe Report

run_tag: \`$RUN_TAG\`
runtime_repo: \`$RUN_DIR/runtime_repo\`
CAS path: \`$RUN_DIR/cas\`
audit_tape verdict: \`$audit_verdict\`
config_hash: \`$config_hash\`
problem_set_hash: \`$problem_set_hash\`
model_assignment_hash: \`$model_assignment_hash\`
budget_config_hash: \`$budget_config_hash\`
prompt_template_hash: \`$prompt_template_hash\`

## Sentinels

\`\`\`text
TURINGOS_REAL13_EV_DECISION_TRACE=1
TURINGOS_MARKET_REVIEW_MODE=sequential
TURINGOS_REAL13_TRADER_EV_SCAFFOLD=$TURINGOS_REAL13_TRADER_EV_SCAFFOLD
TURINGOS_REAL5_ROLE_ASSIGNMENT=$TURINGOS_REAL5_ROLE_ASSIGNMENT
TURINGOS_REAL12_TASK_MARKET_AFFORDANCE=1
TURINGOS_REAL12_TRADER_OBJECTIVE=$TURINGOS_REAL12_TRADER_OBJECTIVE
TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION=0
TURINGOS_REAL11_NO_SCRIPTED_BUYS=1
No forced trade
No price-as-truth
No ghost liquidity
No f64/f32 money path
\`\`\`

## CAS-Derived Metrics

| Metric | Value |
| --- | ---: |
| ev_decision_trace_total_cas | $ev_decision_trace_total_cas |
| ev_decision_trace_bull_count_cas | $ev_decision_trace_bull_count_cas |
| ev_decision_trace_bear_count_cas | $ev_decision_trace_bear_count_cas |
| ev_decision_trace_buy_yes_count_cas | $ev_decision_trace_buy_yes_count_cas |
| ev_decision_trace_buy_no_count_cas | $ev_decision_trace_buy_no_count_cas |
| ev_decision_trace_abstain_count_cas | $ev_decision_trace_abstain_count_cas |
| ev_public_basis_available_count | $ev_public_basis_available_count |
| ev_public_basis_missing_count | $ev_public_basis_missing_count |
| ev_public_basis_delivery_rate_bps | $ev_public_basis_delivery_rate_bps |
| market_review_summary_cas_count | $market_review_summary_cas_count |
| policy_trader_trace_total_cas | $policy_trader_trace_total_cas |
| policy_positive_ev_count | $policy_positive_ev_count |
| policy_positive_ev_llm_abstained_count | $policy_positive_ev_llm_abstained_count |
| policy_insufficient_public_basis_count | $policy_insufficient_public_basis_count |
| policy_counts_for_e2 | $policy_counts_for_e2 |
| config_hash | $config_hash |
| problem_set_hash | $problem_set_hash |
| model_assignment_hash | $model_assignment_hash |
| budget_config_hash | $budget_config_hash |
| prompt_template_hash | $prompt_template_hash |
| live_non_scripted_router_tx_count | $live_non_scripted_router_tx_count |

## Interpretation

\`$e2_verdict\`

EVDecisionTrace and MarketReviewSummary counts are derived from Generic CAS
schema IDs through \`audit_dashboard --run-report\`. They are not stdout
claims. A live non-scripted router tx remains only an E2 candidate until a
clean-context audit confirms PromptCapsule provenance, ChainTape tx evidence,
no forced trade, and no price-as-truth.
EOF

cp "$REPORT" "$ROOT_REPORT"

if [[ "$audit_verdict" != "PROCEED" ]]; then
    echo "ERROR: audit_tape verdict=$audit_verdict" >&2
    exit 7
fi
if [[ "$ev_decision_trace_total_cas" -le 0 ]]; then
    echo "ERROR: EVDecisionTrace CAS count is zero" >&2
    exit 8
fi
if [[ "$market_review_summary_cas_count" -le 0 ]]; then
    echo "ERROR: MarketReviewSummary CAS count is zero" >&2
    exit 9
fi

echo "REAL-13 market-pressure probe evidence: $RUN_DIR"
echo "audit_verdict=$audit_verdict"
echo "ev_decision_trace_total_cas=$ev_decision_trace_total_cas"
echo "ev_public_basis_delivery_rate_bps=$ev_public_basis_delivery_rate_bps"
echo "policy_trader_trace_total_cas=$policy_trader_trace_total_cas"
echo "policy_positive_ev_count=$policy_positive_ev_count"
echo "policy_insufficient_public_basis_count=$policy_insufficient_public_basis_count"
echo "policy_counts_for_e2=$policy_counts_for_e2"
echo "market_review_summary_cas_count=$market_review_summary_cas_count"
echo "live_non_scripted_router_tx_count=$live_non_scripted_router_tx_count"
echo "e2_verdict=$e2_verdict"
