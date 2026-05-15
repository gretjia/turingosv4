# REAL-11 E2 Micro-Probe Report

run_tag: `real11_e2_micro_probe_20260515T165204Z`
runtime_repo: `/home/zephryj/projects/turingosv4/handover/evidence/real11_e2_micro_probe_20260515T165204Z/runtime_repo`
CAS path: `/home/zephryj/projects/turingosv4/handover/evidence/real11_e2_micro_probe_20260515T165204Z/cas`
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
| MarketOpportunityTrace count | 1 |
| market_seed | 5 |
| cpmm_pool | 5 |
| buy_with_coin_router | 0 |
| live_non_scripted_router_tx_count | 0 |
| scripted_fixture_tx_count | 0 |
| agent_economic_action_tx_count | 0 |

NoTradeReason distribution: `## §F MarketDecisionTrace summary;## §F.A NoTradeReason exhaustive breakdown;`

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
