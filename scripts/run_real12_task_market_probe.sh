#!/usr/bin/env bash
# REAL-12 — task-market action affordance probe.
#
# Runs real MiniF2F tasks with TaskOutcomeMarket visible and the REAL-12
# task-market affordance enabled. The run is still no-forced-trade and keeps
# live REAL-6B disabled; it measures whether the model emits `bid_task`,
# `invest`, or neither under ordinary role-scoped prompting.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EVIDENCE_ROOT="$PROJECT_ROOT/handover/evidence"

RUN_TAG="${1:-real12_task_market_probe_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_TAG="${RUN_TAG#handover/evidence/}"
RUN_DIR="$EVIDENCE_ROOT/$RUN_TAG"
ECONOMIC_JUDGMENT_SCHEMA_ID="real12.economic_judgment.v1"
ROLE_TURN_TRACE_SCHEMA_ID="real5.role_turn_trace.v1"

is_truthy() {
    case "${1:-0}" in
        1|true|TRUE|True|yes|YES) return 0 ;;
        *) return 1 ;;
    esac
}

if is_truthy "${TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION:-0}"; then
    echo "ERROR: live REAL-6B is not authorized in REAL-12 task-market probe" >&2
    exit 2
fi
if is_truthy "${TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE:-0}"; then
    echo "ERROR: scripted AttemptPrediction fixture is forbidden in REAL-12 task-market probe" >&2
    exit 2
fi
if [[ -n "${TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS:-}" ]]; then
    echo "ERROR: scripted TaskOutcome buys are forbidden in REAL-12 task-market probe" >&2
    exit 2
fi

export TURINGOS_TB_N3_AUTO_MARKET=1
export TURINGOS_REAL6_TASK_OUTCOME_MARKET=1
export TURINGOS_REAL5_ROLE_VIEWS=1
export TURINGOS_REAL11_MARKET_OPPORTUNITY_TRACE=1
export TURINGOS_REAL11_TRADER_PNL_VIEW=1
export TURINGOS_REAL11_NO_SCRIPTED_BUYS=1
export TURINGOS_REAL12_TASK_MARKET_AFFORDANCE=1
export TURINGOS_REAL12_TRADER_OBJECTIVE="${TURINGOS_REAL12_TRADER_OBJECTIVE:-0}"
export TURINGOS_REAL12_TASK_MARKET_PROBE=1
unset TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS
export TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE=0
export TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION=0
export TURINGOS_REAL5_ROLE_ASSIGNMENT="${TURINGOS_REAL5_ROLE_ASSIGNMENT:-Solver,BullTrader,BearTrader,Verifier,Challenger}"
export TURINGOS_G_PHASE_N_AGENTS="${TURINGOS_G_PHASE_N_AGENTS:-5}"
export MAX_TRANSACTIONS="${MAX_TRANSACTIONS:-10}"
export PER_PROBLEM_TIMEOUT_S="${PER_PROBLEM_TIMEOUT_S:-300}"
export TURINGOS_G_PHASE_DIRTY_OK="${TURINGOS_G_PHASE_DIRTY_OK:-1}"
export TURINGOS_G_PHASE_LOW_DISK_OK="${TURINGOS_G_PHASE_LOW_DISK_OK:-1}"

bash "$PROJECT_ROOT/scripts/run_g_phase_batch.sh" "$RUN_TAG" "${REAL12_PROBLEM_SET:-mini}"

cargo run --quiet --bin audit_dashboard -- --repo "$RUN_DIR/runtime_repo" --cas "$RUN_DIR/cas" --run-report \
    > "$RUN_DIR/audit_dashboard_run_report.txt"

AGG="$RUN_DIR/aggregate_verdict.json"
DASH="$RUN_DIR/audit_dashboard_run_report.txt"
REPORT="$RUN_DIR/REAL12_TASK_MARKET_PROBE_REPORT.md"
ROOT_REPORT="$PROJECT_ROOT/handover/reports/REAL12_TASK_MARKET_PROBE_REPORT.md"

audit_verdict="$(jq -r '.verdict // "missing"' "$AGG" 2>/dev/null || echo missing)"
buy_with_coin_router="$(jq -r '.tx_kind_counts.buy_with_coin_router // 0' "$AGG" 2>/dev/null || echo 0)"
buy_yes_router_count="$(awk -F': ' '/router_buy_yes/ {gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit}' "$DASH")"
buy_yes_router_count="${buy_yes_router_count:-0}"
buy_no_router_count="$(awk -F': ' '/router_buy_no/ {gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit}' "$DASH")"
buy_no_router_count="${buy_no_router_count:-0}"
market_seed="$(jq -r '.tx_kind_counts.market_seed // 0' "$AGG" 2>/dev/null || echo 0)"
cpmm_pool="$(jq -r '.tx_kind_counts.cpmm_pool // 0' "$AGG" 2>/dev/null || echo 0)"
event_resolve="$(jq -r '.tx_kind_counts.event_resolve // 0' "$AGG" 2>/dev/null || echo 0)"
opportunity_count="$(awk -F': ' '/persisted_market_opportunity_trace_cas_count/ {gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit}' "$DASH")"
opportunity_count="${opportunity_count:-0}"

dash_metric() {
    local key="$1"
    awk -F': ' -v key="$key" '$1 ~ key"$" {
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2);
        print $2;
        found=1;
        exit
    } END { if (!found) print "" }' "$DASH"
}

agent_economic_action_tx_count="$(dash_metric agent_economic_action_tx_count)"
agent_economic_action_tx_count="${agent_economic_action_tx_count:-0}"
economic_judgment_total="$(dash_metric economic_judgment_total_cas)"
economic_judgment_total="${economic_judgment_total:-0}"
bull_judgment_count="$(dash_metric bull_judgment_count_cas)"
bull_judgment_count="${bull_judgment_count:-0}"
bear_judgment_count="$(dash_metric bear_judgment_count_cas)"
bear_judgment_count="${bear_judgment_count:-0}"
abstain_structured_reason_count="$(dash_metric abstain_structured_reason_count_cas)"
abstain_structured_reason_count="${abstain_structured_reason_count:-0}"
economic_judgment_coverage_ok="$(dash_metric economic_judgment_coverage_ok)"
economic_judgment_coverage_ok="${economic_judgment_coverage_ok:-false}"
economic_judgment_required_trader_turns="$(dash_metric economic_judgment_required_trader_turns_cas)"
economic_judgment_required_trader_turns="${economic_judgment_required_trader_turns:-0}"
economic_judgment_linked_trader_turns="$(dash_metric economic_judgment_linked_trader_turns_cas)"
economic_judgment_linked_trader_turns="${economic_judgment_linked_trader_turns:-0}"

tool_dist_jsonl="$RUN_DIR/real12_tool_dist.jsonl"
: > "$tool_dist_jsonl"
find "$RUN_DIR" -mindepth 2 -maxdepth 2 -name evaluator.stdout -print0 \
    | while IFS= read -r -d '' stdout_file; do
        grep '^PPUT_RESULT:' "$stdout_file" \
            | sed 's/^PPUT_RESULT://' \
            | jq -c '{problem_id, solved, tool_dist: (.tool_dist // {})}' \
            >> "$tool_dist_jsonl" || true
    done

sum_tool() {
    local key="$1"
    if [[ ! -s "$tool_dist_jsonl" ]]; then
        echo 0
        return
    fi
    jq -s --arg key "$key" 'map(.tool_dist[$key] // 0) | add // 0' "$tool_dist_jsonl"
}

bid_task_attempted="$(sum_tool bid_task_attempted)"
invest_attempted="$(sum_tool invest_attempted)"
invest_submitted="$(sum_tool invest_submitted)"
no_perceived_edge="$(sum_tool invest_no_trade_no_perceived_edge)"
zero_amount="$(sum_tool invest_no_trade_zero_amount)"
no_pool="$(sum_tool invest_no_trade_no_pool)"
amount_exceeds_balance="$(sum_tool invest_no_trade_amount_exceeds_balance)"
abstain_reason_distribution="$(awk -F': ' '/economic_judgment_reason_/ {
    key=$1;
    sub(/^.*economic_judgment_reason_/, "", key);
    gsub(/^[[:space:]-]+|[[:space:]]+$/, "", key);
    value=$2;
    gsub(/^[[:space:]]+|[[:space:]]+$/, "", value);
    print key "=" value;
}' "$DASH" | jq -Rn '
    reduce inputs as $line ({}; ($line | split("=")) as $parts
      | if ($parts | length) == 2 then .[$parts[0]] = ($parts[1] | tonumber) else . end)
' 2>/dev/null || echo '{}')"

live_non_scripted_router_tx_count="$agent_economic_action_tx_count"
e2_verdict="NOT ACHIEVED"
if [[ "$live_non_scripted_router_tx_count" -gt 0 ]]; then
    e2_verdict="E2 candidate achieved; CANDIDATE_ACHIEVED_REQUIRES_AUDIT"
fi
if [[ "$live_non_scripted_router_tx_count" -eq 0 ]]; then
    e2_verdict="E2 NOT ACHIEVED"
fi

cat > "$REPORT" <<EOF
# REAL-12 Task-Market Action Probe Report

run_tag: \`$RUN_TAG\`
runtime_repo: \`$RUN_DIR/runtime_repo\`
CAS path: \`$RUN_DIR/cas\`
audit_tape verdict: \`$audit_verdict\`

## Constitutional Sentinels

\`\`\`text
No forced trade
No price-as-truth
No scripted buys
scripted_positive_control_is_not_e2=true
live_real6b_enabled=false
attempt_prediction_fixture_count=0
TURINGOS_REAL12_TASK_MARKET_AFFORDANCE=1
TURINGOS_REAL12_TRADER_OBJECTIVE=$TURINGOS_REAL12_TRADER_OBJECTIVE
No ghost liquidity
No f64/f32 money path
\`\`\`

## Metrics

| Metric | Value |
| --- | ---: |
| MarketOpportunityTrace count | $opportunity_count |
| market_seed | $market_seed |
| cpmm_pool | $cpmm_pool |
| event_resolve | $event_resolve |
| bid_task_attempted | $bid_task_attempted |
| invest_attempted | $invest_attempted |
| invest_submitted | $invest_submitted |
| buy_with_coin_router | $buy_with_coin_router |
| buy_yes_router_count | $buy_yes_router_count |
| buy_no_router_count | $buy_no_router_count |
| agent_economic_action_tx_count | $agent_economic_action_tx_count |
| live_non_scripted_router_tx_count | $live_non_scripted_router_tx_count |
| economic_judgment_total | $economic_judgment_total |
| bull_judgment_count | $bull_judgment_count |
| bear_judgment_count | $bear_judgment_count |
| abstain_structured_reason_count | $abstain_structured_reason_count |
| economic_judgment_coverage_ok | $economic_judgment_coverage_ok |
| economic_judgment_required_trader_turns | $economic_judgment_required_trader_turns |
| economic_judgment_linked_trader_turns | $economic_judgment_linked_trader_turns |
| no_trade_no_perceived_edge | $no_perceived_edge |
| no_trade_zero_amount | $zero_amount |
| no_trade_no_pool | $no_pool |
| no_trade_amount_exceeds_balance | $amount_exceeds_balance |

abstain_reason_distribution:

\`\`\`json
$abstain_reason_distribution
\`\`\`

## Interpretation Boundary

\`$e2_verdict\`

This probe tests whether the advertised task-level market action affordance
causes live agents to emit \`bid_task\` or \`invest\`. It does not force trades,
does not enable live REAL-6B, and does not allow price to affect Lean predicates.

Scripted actions cannot satisfy E2. A live non-scripted router tx requires
ChainTape/CAS evidence, PromptCapsule/trace provenance, and audit_tape PROCEED.
This report derives EconomicJudgment counts from CAS schema
\`$ECONOMIC_JUDGMENT_SCHEMA_ID\` and Bull/Bear turn coverage from
\`$ROLE_TURN_TRACE_SCHEMA_ID\`; stdout tool_dist is diagnostic only.
EOF

cp "$REPORT" "$ROOT_REPORT"

if [[ "$audit_verdict" != "PROCEED" ]]; then
    echo "ERROR: audit_tape verdict=$audit_verdict" >&2
    exit 7
fi
if [[ "$economic_judgment_coverage_ok" != "true" ]]; then
    echo "ERROR: EconomicJudgment coverage failed: $economic_judgment_coverage_ok" >&2
    exit 8
fi

echo "REAL-12 task-market probe evidence: $RUN_DIR"
echo "audit_verdict=$audit_verdict"
echo "bid_task_attempted=$bid_task_attempted"
echo "invest_attempted=$invest_attempted"
echo "buy_with_coin_router=$buy_with_coin_router"
echo "buy_yes_router_count=$buy_yes_router_count"
echo "buy_no_router_count=$buy_no_router_count"
echo "economic_judgment_total=$economic_judgment_total"
echo "bull_judgment_count=$bull_judgment_count"
echo "bear_judgment_count=$bear_judgment_count"
echo "e2_verdict=$e2_verdict"
