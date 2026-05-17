# REAL-13 Market Pressure Probe Report

run_tag: `market_autonomy_lab_hard10_exact_join_R15_20260516T195846Z`
runtime_repo: `/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/evidence/market_autonomy_lab_hard10_exact_join_R15_20260516T195846Z/runtime_repo`
CAS path: `/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/evidence/market_autonomy_lab_hard10_exact_join_R15_20260516T195846Z/cas`
audit_tape verdict: `PROCEED`

## Sentinels

```text
TURINGOS_REAL13_EV_DECISION_TRACE=1
TURINGOS_MARKET_REVIEW_MODE=sequential
TURINGOS_REAL5_ROLE_ASSIGNMENT=BullTrader,BearTrader,Solver,Verifier,Challenger
TURINGOS_REAL12_TASK_MARKET_AFFORDANCE=1
TURINGOS_REAL12_TRADER_OBJECTIVE=1
TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION=0
TURINGOS_REAL11_NO_SCRIPTED_BUYS=1
No forced trade
No price-as-truth
No ghost liquidity
No f64/f32 money path
```

## CAS-Derived Metrics

| Metric | Value |
| --- | ---: |
| ev_decision_trace_total_cas | 106 |
| ev_decision_trace_bull_count_cas | 53 |
| ev_decision_trace_bear_count_cas | 53 |
| ev_decision_trace_buy_yes_count_cas | 13 |
| ev_decision_trace_buy_no_count_cas | 0 |
| ev_decision_trace_abstain_count_cas | 93 |
| market_review_summary_cas_count | 106 |
| live_non_scripted_router_tx_count | 13 |

## Interpretation

`E2 candidate pending audit`

EVDecisionTrace and MarketReviewSummary counts are derived from Generic CAS
schema IDs through `audit_dashboard --run-report`. They are not stdout
claims. A live non-scripted router tx remains only an E2 candidate until a
clean-context audit confirms PromptCapsule provenance, ChainTape tx evidence,
no forced trade, and no price-as-truth.
