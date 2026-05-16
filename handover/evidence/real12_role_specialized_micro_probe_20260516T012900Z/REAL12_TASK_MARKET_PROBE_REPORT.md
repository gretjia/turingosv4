# REAL-12 Task-Market Action Probe Report

run_tag: `real12_role_specialized_micro_probe_20260516T012900Z`
runtime_repo: `/home/zephryj/projects/turingosv4-real12-action-probes/handover/evidence/real12_role_specialized_micro_probe_20260516T012900Z/runtime_repo`
CAS path: `/home/zephryj/projects/turingosv4-real12-action-probes/handover/evidence/real12_role_specialized_micro_probe_20260516T012900Z/cas`
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
| MarketOpportunityTrace count | 2 |
| market_seed | 6 |
| cpmm_pool | 6 |
| event_resolve | 2 |
| bid_task_attempted | 0 |
| invest_attempted | 0 |
| invest_submitted | 0 |
| buy_with_coin_router | 0 |
| buy_yes_router_count | 0 |
| buy_no_router_count | 0 |
| live_non_scripted_router_tx_count | 0 |
| economic_judgment_total | 2 |
| bull_judgment_count | 1 |
| bear_judgment_count | 1 |
| abstain_structured_reason_count | 2 |
| no_trade_no_perceived_edge | 5 |
| no_trade_zero_amount | 0 |
| no_trade_no_pool | 0 |
| no_trade_amount_exceeds_balance | 0 |

abstain_reason_distribution:

```json
{
  "no_perceived_edge": 5,
  "zero_amount": 0,
  "no_pool": 0,
  "amount_exceeds_balance": 0,
  "prompt_budget_exceeded": 0,
  "router_rejected": 0
}
```

## Interpretation Boundary

`E2 NOT ACHIEVED`

This probe tests whether the advertised task-level market action affordance
causes live agents to emit `bid_task` or `invest`. It does not force trades,
does not enable live REAL-6B, and does not allow price to affect Lean predicates.

Scripted actions cannot satisfy E2. A live non-scripted router tx requires
ChainTape/CAS evidence, PromptCapsule/trace provenance, and audit_tape PROCEED.
