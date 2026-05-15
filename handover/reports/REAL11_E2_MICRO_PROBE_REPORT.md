# REAL-11 E2 Micro-Probe Report

run_tag: `real11_e2_micro_probe_20260515T172707Z_r2_max10`
runtime_repo: `/home/zephryj/projects/turingosv4/handover/evidence/real11_e2_micro_probe_20260515T172707Z_r2_max10/runtime_repo`
CAS path: `/home/zephryj/projects/turingosv4/handover/evidence/real11_e2_micro_probe_20260515T172707Z_r2_max10/cas`
audit_tape verdict: `PROCEED`

## Required Sentinels

```text
live_real6b_enabled=false
attempt_prediction_fixture_count=0
No forced trade
No price-as-truth
No scripted buys in Atom 5
```

## Metrics

| Metric | Value |
| --- | ---: |
| Trader turn count | 0 |
| Trader turn count source | dashboard |
| MarketOpportunityTrace count | 0 |
| market_seed | 6 |
| cpmm_pool | 6 |
| buy_with_coin_router | 0 |
| live_non_scripted_router_tx_count | 0 |
| scripted_fixture_tx_count | 0 |
| agent_economic_action_tx_count | 0 |

NoTradeReason distribution: `not_rendered`

MarketOpportunityTrace summary: `not_rendered`

## E2 Verdict

`NOT ACHIEVED`

E2 achieved only if live_non_scripted_router_tx_count >= 1 and every qualifying
tx has ChainTape/CAS anchor + PromptCapsule/trace provenance + audit_tape
PROCEED + no forced/scripted flag.

Decision branch: `B/C diagnostic: no live non-scripted router tx observed`

## Forbidden Claims

```text
No E3 claim.
No E4 claim.
No live REAL-6B approval.
No market-caused solve improvement claim.
No model ranking.
```
