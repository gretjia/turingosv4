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
export TURINGOS_REAL12_TASK_MARKET_AFFORDANCE=1
export TURINGOS_REAL12_TRADER_OBJECTIVE="${TURINGOS_REAL12_TRADER_OBJECTIVE:-1}"
export TURINGOS_REAL5_ROLE_ASSIGNMENT="${TURINGOS_REAL5_ROLE_ASSIGNMENT:-BullTrader,BearTrader,Solver,Verifier,Challenger}"
export TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION=0
export TURINGOS_REAL11_NO_SCRIPTED_BUYS=1
unset TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS

bash "$PROJECT_ROOT/scripts/run_real12_task_market_probe.sh" "$RUN_TAG"

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

## Sentinels

\`\`\`text
TURINGOS_REAL13_EV_DECISION_TRACE=1
TURINGOS_MARKET_REVIEW_MODE=sequential
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
| market_review_summary_cas_count | $market_review_summary_cas_count |
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
echo "market_review_summary_cas_count=$market_review_summary_cas_count"
echo "live_non_scripted_router_tx_count=$live_non_scripted_router_tx_count"
echo "e2_verdict=$e2_verdict"
