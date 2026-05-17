# REAL-12 Task-Market Action Probe Report

run_tag: `market_autonomy_lab_hard10_real14H_frozen_real14G_replication_20260517T025400Z`
runtime_repo: `/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/evidence/market_autonomy_lab_hard10_real14H_frozen_real14G_replication_20260517T025400Z/runtime_repo`
CAS path: `/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/evidence/market_autonomy_lab_hard10_real14H_frozen_real14G_replication_20260517T025400Z/cas`
audit_tape verdict: `PROCEED`

## Constitutional Sentinels

```text
No forced trade
No price-as-truth
No scripted buys
scripted_positive_control_is_not_e2=true
live_real6b_enabled=false
attempt_prediction_fixture_count=0
TURINGOS_REAL12_TASK_MARKET_AFFORDANCE=1
TURINGOS_REAL12_TRADER_OBJECTIVE=1
No ghost liquidity
No f64/f32 money path
```

## Metrics

| Metric | Value |
| --- | ---: |
| MarketOpportunityTrace count | 40 |
| market_seed | 10 |
| cpmm_pool | 10 |
| event_resolve | 10 |
| bid_task_attempted | 0 |
| invest_attempted | 0 |
| invest_submitted | 13 |
| buy_with_coin_router | 13 |
| buy_yes_router_count | 11 |
| buy_no_router_count | 2 |
| agent_economic_action_tx_count | 13 |
| live_non_scripted_router_tx_count | 13 |
| economic_judgment_total | 40 |
| bull_judgment_count | 20 |
| bear_judgment_count | 20 |
| abstain_structured_reason_count | 27 |
| economic_judgment_coverage_ok | true |
| economic_judgment_required_trader_turns | 40 |
| economic_judgment_linked_trader_turns | 40 |
| no_trade_no_perceived_edge | 87 |
| no_trade_zero_amount | 0 |
| no_trade_no_pool | 0 |
| no_trade_amount_exceeds_balance | 0 |

economic_judgment_reason_distribution:

```json
{
  "NoPerceivedEdge": 40
}
```

## Interpretation Boundary

`E2 candidate pending audit`

This probe tests whether the advertised task-level market action affordance
causes live agents to emit `bid_task` or `invest`. It does not force trades,
does not enable live REAL-6B, and does not allow price to affect Lean predicates.

Scripted actions cannot satisfy E2. A live non-scripted router tx requires
ChainTape/CAS evidence, PromptCapsule/trace provenance, and audit_tape PROCEED.
This report derives EconomicJudgment counts from CAS schema
`real12.economic_judgment.v1` and Bull/Bear turn coverage from
`real5.role_turn_trace.v1`; stdout tool_dist is diagnostic only.
