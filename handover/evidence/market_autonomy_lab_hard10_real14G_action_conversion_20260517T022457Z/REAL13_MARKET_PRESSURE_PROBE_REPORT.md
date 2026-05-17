# REAL-13 Market Pressure Probe Report

run_tag: `market_autonomy_lab_hard10_real14G_action_conversion_20260517T022457Z`
runtime_repo: `/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/evidence/market_autonomy_lab_hard10_real14G_action_conversion_20260517T022457Z/runtime_repo`
CAS path: `/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/evidence/market_autonomy_lab_hard10_real14G_action_conversion_20260517T022457Z/cas`
audit_tape verdict: `PROCEED`
config_hash: `35b03c8ed3d49979390e11235657baa057b7b93f7608d99994037bc8d5043d42`
problem_set_hash: `138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc`
model_assignment_hash: `fe61fc358c92d4ba02112595ebd13574d6a082ce4f234df758fe31b7d02c0482`
budget_config_hash: `cfc243b393295fa6fc1868553308cea9b49c8c1b36504b7ae34145b8e15c717d`
prompt_template_hash: `cf860511f71d56b53b2df000150f2cf42fb47caaa3d26a20ca9f15a443f3c4f2`

## Sentinels

```text
TURINGOS_REAL13_EV_DECISION_TRACE=1
TURINGOS_MARKET_REVIEW_MODE=sequential
TURINGOS_REAL13_TRADER_EV_SCAFFOLD=1
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
| ev_decision_trace_total_cas | 38 |
| ev_decision_trace_bull_count_cas | 19 |
| ev_decision_trace_bear_count_cas | 19 |
| ev_decision_trace_buy_yes_count_cas | 8 |
| ev_decision_trace_buy_no_count_cas | 0 |
| ev_decision_trace_abstain_count_cas | 30 |
| ev_public_basis_available_count | 38 |
| ev_public_basis_missing_count | 0 |
| ev_public_basis_delivery_rate_bps | 10000 |
| market_review_summary_cas_count | 38 |
| policy_trader_trace_total_cas | 38 |
| policy_positive_ev_count | 25 |
| policy_positive_ev_llm_abstained_count | 17 |
| policy_insufficient_public_basis_count | 0 |
| policy_counts_for_e2 | false |
| config_hash | 35b03c8ed3d49979390e11235657baa057b7b93f7608d99994037bc8d5043d42 |
| problem_set_hash | 138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc |
| model_assignment_hash | fe61fc358c92d4ba02112595ebd13574d6a082ce4f234df758fe31b7d02c0482 |
| budget_config_hash | cfc243b393295fa6fc1868553308cea9b49c8c1b36504b7ae34145b8e15c717d |
| prompt_template_hash | cf860511f71d56b53b2df000150f2cf42fb47caaa3d26a20ca9f15a443f3c4f2 |
| live_non_scripted_router_tx_count | 8 |

## Interpretation

`E2 candidate pending audit`

EVDecisionTrace and MarketReviewSummary counts are derived from Generic CAS
schema IDs through `audit_dashboard --run-report`. They are not stdout
claims. A live non-scripted router tx remains only an E2 candidate until a
clean-context audit confirms PromptCapsule provenance, ChainTape tx evidence,
no forced trade, and no price-as-truth.
