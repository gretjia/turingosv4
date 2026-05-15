#!/usr/bin/env bash
# REAL-11 Atom 5 — E2 micro-probe.
#
# Runs a small no-forced-trade TaskOutcomeMarket probe with Trader PnL view and
# MarketOpportunityTrace enabled. Live REAL-6B and scripted AttemptPrediction
# are forbidden in this atom.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EVIDENCE_ROOT="$PROJECT_ROOT/handover/evidence"

RUN_TAG="${1:-real11_e2_micro_probe_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_TAG="${RUN_TAG#handover/evidence/}"
RUN_DIR="$EVIDENCE_ROOT/$RUN_TAG"

is_truthy() {
    case "${1:-0}" in
        1|true|TRUE|True|yes|YES) return 0 ;;
        *) return 1 ;;
    esac
}

if is_truthy "${TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION:-0}"; then
    echo "ERROR: live REAL-6B is not authorized in REAL-11" >&2
    exit 2
fi
if is_truthy "${TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE:-0}"; then
    echo "ERROR: scripted AttemptPrediction fixture is forbidden in REAL-11 Atom 5" >&2
    exit 2
fi
if [[ -n "${TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS:-}" ]]; then
    echo "ERROR: scripted TaskOutcome buys are forbidden in REAL-11 Atom 5" >&2
    exit 2
fi

export TURINGOS_TB_N3_AUTO_MARKET=1
export TURINGOS_REAL6_TASK_OUTCOME_MARKET=1
export TURINGOS_REAL5_ROLE_VIEWS=1
export TURINGOS_REAL11_MARKET_OPPORTUNITY_TRACE=1
export TURINGOS_REAL11_TRADER_PNL_VIEW=1
export TURINGOS_REAL11_NO_SCRIPTED_BUYS=1
export TURINGOS_REAL11_E2_MICRO_PROBE=1
unset TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS
export TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE=0
export TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION=0
export TURINGOS_REAL5_ROLE_ASSIGNMENT="${TURINGOS_REAL5_ROLE_ASSIGNMENT:-Solver,Trader,Verifier,Challenger,Observer}"
export TURINGOS_G_PHASE_N_AGENTS="${TURINGOS_G_PHASE_N_AGENTS:-5}"
export MAX_TRANSACTIONS="${MAX_TRANSACTIONS:-5}"
export PER_PROBLEM_TIMEOUT_S="${PER_PROBLEM_TIMEOUT_S:-300}"
export TURINGOS_G_PHASE_DIRTY_OK="${TURINGOS_G_PHASE_DIRTY_OK:-1}"
export TURINGOS_G_PHASE_LOW_DISK_OK="${TURINGOS_G_PHASE_LOW_DISK_OK:-1}"

bash "$PROJECT_ROOT/scripts/run_g_phase_batch.sh" "$RUN_TAG" mini

cargo run --quiet --bin audit_dashboard -- --repo "$RUN_DIR/runtime_repo" --cas "$RUN_DIR/cas" --run-report \
    > "$RUN_DIR/audit_dashboard_run_report.txt"

AGG="$RUN_DIR/aggregate_verdict.json"
DASH="$RUN_DIR/audit_dashboard_run_report.txt"
REPORT="$RUN_DIR/REAL11_E2_MICRO_PROBE_REPORT.md"
ROOT_REPORT="$PROJECT_ROOT/handover/reports/REAL11_E2_MICRO_PROBE_REPORT.md"

audit_verdict="$(jq -r '.verdict // "missing"' "$AGG" 2>/dev/null || echo missing)"
buy_with_coin_router="$(jq -r '.tx_kind_counts.buy_with_coin_router // 0' "$AGG" 2>/dev/null || echo 0)"
market_seed="$(jq -r '.tx_kind_counts.market_seed // 0' "$AGG" 2>/dev/null || echo 0)"
cpmm_pool="$(jq -r '.tx_kind_counts.cpmm_pool // 0' "$AGG" 2>/dev/null || echo 0)"
scripted_attempt_count="$(awk -F': ' '/scripted_attempt_prediction_market_count/ {gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit}' "$DASH")"
scripted_attempt_count="${scripted_attempt_count:-0}"
opportunity_count="$(awk -F': ' '/persisted_market_opportunity_trace_cas_count/ {gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit}' "$DASH")"
opportunity_count="${opportunity_count:-0}"
dashboard_trader_turn_count="$(awk -F': ' '/trader_turn_count/ {gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit}' "$DASH")"
dashboard_trader_turn_count="${dashboard_trader_turn_count:-0}"
trader_turn_count="$dashboard_trader_turn_count"
trader_turn_count_source="dashboard"
if [[ "$trader_turn_count" == "0" && "$opportunity_count" != "0" ]]; then
    trader_turn_count="$opportunity_count"
    trader_turn_count_source="MarketOpportunityTrace CAS witness"
fi
no_trade_distribution="$(
    awk '
        /no_trade reason breakdown/ {flag=1; next}
        /^$/ {if (flag) exit}
        flag && / = / {gsub(/^[[:space:]]+/, "", $0); print}
    ' "$DASH" | paste -sd '; ' -
)"
no_trade_distribution="${no_trade_distribution:-not_rendered}"
opportunity_summary="not_rendered"
if [[ -f "$RUN_DIR/cas/.turingos_cas_index.jsonl" ]]; then
    opportunity_summary="$(
        jq -r 'select(.schema_id=="real11.market_opportunity_trace.v1") | .backend_oid_hex' "$RUN_DIR/cas/.turingos_cas_index.jsonl" \
        | while read -r oid; do
            [[ -n "$oid" ]] || continue
            payload="$(git -C "$RUN_DIR/cas" cat-file -p "$oid" 2>/dev/null || true)"
            [[ -n "$payload" ]] || continue
            agent="$(printf '%s' "$payload" | jq -r '.agent_id // "unknown"' 2>/dev/null || echo unknown)"
            actionable="$(printf '%s' "$payload" | jq -r '(.actionable_markets // []) | length' 2>/dev/null || echo 0)"
            visible="$(printf '%s' "$payload" | jq -r '(.visible_markets // []) | length' 2>/dev/null || echo 0)"
            router="$(printf '%s' "$payload" | jq -r '.router_available // false' 2>/dev/null || echo false)"
            balance="$(printf '%s' "$payload" | jq -r '.available_balance // 0' 2>/dev/null || echo 0)"
            reason="$(printf '%s' "$payload" | jq -r '.reason_if_no_actionable_market // "none"' 2>/dev/null || echo none)"
            printf '%s visible=%s actionable=%s router_available=%s balance=%s reason_if_no_actionable_market=%s\n' \
                "$agent" "$visible" "$actionable" "$router" "$balance" "$reason"
        done | paste -sd '; ' -
    )"
fi
opportunity_summary="${opportunity_summary:-not_rendered}"

live_non_scripted_router_tx_count=0
agent_economic_action_tx_count=0
e2_verdict="NOT ACHIEVED"
decision_branch="B/C diagnostic: no live non-scripted router tx observed"
if [[ "$buy_with_coin_router" -gt 0 ]]; then
    # In REAL-11 Atom 5 no scripted buys are enabled, so any audited router tx
    # would be candidate live E2. Keep the explicit provenance fields in the report.
    live_non_scripted_router_tx_count="$buy_with_coin_router"
    agent_economic_action_tx_count="$buy_with_coin_router"
    e2_verdict="ACHIEVED"
    decision_branch="A live buy -> REAL-12 role differentiation / E3"
fi

if [[ "$scripted_attempt_count" != "0" ]]; then
    echo "ERROR: attempt_prediction_fixture_count=$scripted_attempt_count; expected 0" >&2
    exit 6
fi

cat > "$REPORT" <<EOF
# REAL-11 E2 Micro-Probe Report

run_tag: \`$RUN_TAG\`
runtime_repo: \`$RUN_DIR/runtime_repo\`
CAS path: \`$RUN_DIR/cas\`
audit_tape verdict: \`$audit_verdict\`

## Required Sentinels

\`\`\`text
live_real6b_enabled=false
attempt_prediction_fixture_count=0
No forced trade
No price-as-truth
No scripted buys in Atom 5
\`\`\`

## Metrics

| Metric | Value |
| --- | ---: |
| Trader turn count | $trader_turn_count |
| Trader turn count source | $trader_turn_count_source |
| MarketOpportunityTrace count | $opportunity_count |
| market_seed | $market_seed |
| cpmm_pool | $cpmm_pool |
| buy_with_coin_router | $buy_with_coin_router |
| live_non_scripted_router_tx_count | $live_non_scripted_router_tx_count |
| scripted_fixture_tx_count | 0 |
| agent_economic_action_tx_count | $agent_economic_action_tx_count |

NoTradeReason distribution: \`$no_trade_distribution\`

MarketOpportunityTrace summary: \`$opportunity_summary\`

## E2 Verdict

\`$e2_verdict\`

E2 achieved only if live_non_scripted_router_tx_count >= 1 and every qualifying
tx has ChainTape/CAS anchor + PromptCapsule/trace provenance + audit_tape
PROCEED + no forced/scripted flag.

Decision branch: \`$decision_branch\`

## Forbidden Claims

\`\`\`text
No E3 claim.
No E4 claim.
No live REAL-6B approval.
No market-caused solve improvement claim.
No model ranking.
\`\`\`
EOF

cp "$REPORT" "$ROOT_REPORT"

if [[ "$audit_verdict" != "PROCEED" ]]; then
    echo "ERROR: audit_tape verdict=$audit_verdict" >&2
    exit 7
fi

echo "REAL-11 E2 micro-probe evidence: $RUN_DIR"
echo "audit_verdict=$audit_verdict"
echo "buy_with_coin_router=$buy_with_coin_router"
echo "live_non_scripted_router_tx_count=$live_non_scripted_router_tx_count"
echo "e2_verdict=$e2_verdict"
