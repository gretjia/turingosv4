# REAL-12 Task-Market Action Probe Report

run_tag: `real12_task_market_probe_trader_boundary_gated_20260515T184026Z`
runtime_repo: `/home/zephryj/projects/turingosv4-real12-action-probes/handover/evidence/real12_task_market_probe_trader_boundary_gated_20260515T184026Z/runtime_repo`
CAS path: `/home/zephryj/projects/turingosv4-real12-action-probes/handover/evidence/real12_task_market_probe_trader_boundary_gated_20260515T184026Z/cas`
audit_tape verdict: `PROCEED`

## Constitutional Sentinels

```text
No forced trade
No price-as-truth
No scripted buys
live_real6b_enabled=false
attempt_prediction_fixture_count=0
TURINGOS_REAL12_TASK_MARKET_AFFORDANCE=1
TURINGOS_REAL12_TRADER_OBJECTIVE=0
```

## Metrics

| Metric | Value |
| --- | ---: |
| MarketOpportunityTrace count | 4 |
| market_seed | 5 |
| cpmm_pool | 5 |
| event_resolve | 2 |
| bid_task_attempted | 0 |
| invest_attempted | 0 |
| invest_submitted | 0 |
| buy_with_coin_router | 0 |
| live_non_scripted_router_tx_count | 0 |
| no_trade_no_perceived_edge | 12 |
| no_trade_zero_amount | 0 |
| no_trade_no_pool | 0 |
| no_trade_amount_exceeds_balance | 0 |

## Interpretation Boundary

`NOT ACHIEVED`

This probe tests whether the advertised task-level market action affordance
causes live agents to emit `bid_task` or `invest`. It does not force trades,
does not enable live REAL-6B, and does not allow price to affect Lean predicates.

Scripted actions cannot satisfy E2. A live non-scripted router tx requires
ChainTape/CAS evidence, PromptCapsule/trace provenance, and audit_tape PROCEED.
