# REAL-11 Decision Gate Report

Date: 2026-05-15

## Scope

REAL-11 answers the post-REAL-10 bottleneck:

```text
market_tx_count increased, but buy_with_coin_router = 0.
```

REAL-11 does not authorize live REAL-6B. It tests whether the live agent
economic action path is blocked by substrate wiring, lack of opportunity,
missing PnL/risk visibility, or agent abstention.

## Canonical Evidence

REAL-10 canonical clean evidence:

```text
handover/evidence/real8x_market_ab_clean_20260515T141331Z/
```

REAL-10 contaminated/remediation-only evidence:

```text
handover/evidence/real8x_market_ab_20260515T134453Z/
```

REAL-11 router positive-control canonical evidence:

```text
handover/evidence/real11_router_positive_control_20260515T172419Z_r2b/
```

REAL-11 patched E2 micro-probe canonical evidence:

```text
handover/evidence/real11_e2_micro_probe_20260515T172707Z_r2_max10/
```

REAL-11 actionable-opportunity supplemental diagnostic:

```text
handover/evidence/real11_e2_micro_probe_20260515T165855Z/
```

The supplemental diagnostic is conclusion-bearing only for the narrow fact that
one Trader turn saw actionable markets and still produced no live buy. It is not
used as the final patched no-scripted-contamination runner proof.

## Atom Results

| Atom | Result | Evidence |
| --- | --- | --- |
| Atom 0 REAL-10 ratification + evidence hygiene | PASS | `handover/directives/2026-05-15_REAL10_NARROW_RATIFICATION_REAL11.md` |
| Atom 1 market tx decomposition | PASS | `src/runtime/market_tx_category.rs`, `handover/reports/REAL11_MARKET_TX_CATEGORY_REPORT.md` |
| Atom 2 router positive-control fixture | PASS | `handover/evidence/real11_router_positive_control_20260515T172419Z_r2b/` |
| Atom 3 MarketOpportunityTrace | PASS | `src/runtime/market_opportunity_trace.rs`, supplemental diagnostic E2 micro-probe report |
| Atom 4 PnL / incentive visibility | PASS | `src/runtime/real6_conviction_budget.rs`, `handover/reports/REAL11_PNL_INCENTIVE_VISIBILITY_REPORT.md` |
| Atom 5 live E2 micro-probe without REAL-6B | PASS as diagnostic, E2 NOT ACHIEVED | `handover/evidence/real11_e2_micro_probe_20260515T172707Z_r2_max10/` |

## Data

Router positive control:

```text
aggregate_verdict = PROCEED
runtime_repo/CAS/dashboard evidence = present
runtime market_seed = 6
runtime cpmm_pool = 6
runtime buy_with_coin_router = 6
scripted_task_outcome_buys = Agent_1:Agent_2:1000
scripted BuyYesWithCoinRouterTx enters L4 = PASS
scripted BuyNo / short-equivalent path enters L4 or explicit L4.E = PASS
insufficient balance routes L4.E / pre-submit classification = PASS
missing pool routes NoPool / L4.E = PASS
CTF conserved = PASS
no ghost liquidity = PASS
no f64 money path = PASS
scripted positive control is not E2 = PASS
```

Patched E2 micro-probe:

```text
audit_tape verdict = PROCEED
live_real6b_enabled = false
attempt_prediction_fixture_count = 0
scripted TaskOutcome buys forbidden by runner = PASS
market_seed = 6
cpmm_pool = 6
buy_with_coin_router = 0
live_non_scripted_router_tx_count = 0
scripted_fixture_tx_count = 0
agent_economic_action_tx_count = 0
MarketOpportunityTrace count = 0
Trader turn count = 0
E2 = NOT ACHIEVED
```

Supplemental actionable-opportunity diagnostic:

```text
audit_tape verdict = PROCEED
live_real6b_enabled = false
attempt_prediction_fixture_count = 0
scripted_fixture_tx_count = 0
market_seed = 5
cpmm_pool = 5
buy_with_coin_router = 0
live_non_scripted_router_tx_count = 0
MarketOpportunityTrace count = 1
Trader turn count source = MarketOpportunityTrace CAS witness
NoTradeReason distribution = no_perceived_edge = 5
```

Observed actionable opportunity in supplemental diagnostic:

```text
Agent_1 visible=3
Agent_1 actionable=3
router_available=true
balance=1000000
reason_if_no_actionable_market=none
```

## Decision Branch

REAL-11 takes a B/C diagnostic branch:

```text
No live non-scripted router tx was observed.
The router substrate is not the immediate blocker.
Patched no-scripted-contamination runs did not schedule an actionable Trader turn.
A supplemental diagnostic shows that when an actionable Trader turn exists,
the agent can still abstain with NoPerceivedEdge rather than buy.
```

Reason:

```text
Router positive control passes through both unit harness and runtime_repo/CAS/audit-dashboard evidence.
Patched E2 micro-probe fail-closes against live REAL-6B, scripted AttemptPrediction,
and scripted TaskOutcome buys.
The latest patched probe remains E2 negative with buy_with_coin_router = 0.
Supplemental diagnostic evidence shows actionable_markets > 0, router_available=true,
and NoTradeReason is no_perceived_edge, not no_pool.
PnL/risk/balance is visible in the scoped view.
```

Recommended next step:

```text
Live Trader activation / objective-routing redesign:
  ensure Trader turns are scheduled in the clean patched micro-probe path;
  make role objective stronger;
  make PnL prompt more explicit;
  consider risk-adjusted return prompt;
  still no forced trade.
```

Do not jump directly to live REAL-6B from this evidence. REAL-6B becomes
necessary when no actionable market window exists after Trader scheduling and
objective-routing are confirmed; current evidence first points to live Trader
activation and abstention behavior.

## Claim Boundary

Ratified for REAL-11:

```text
scripted router path works;
scripted router path is explicitly not E2;
market tx count is decomposed into structural / agent / scripted / resolution categories;
MarketOpportunityTrace can be CAS-anchored for Trader turns;
Trader can see actionable market + router + balance in supplemental diagnostic;
PnL/risk visibility is present in scoped view;
patched E2 micro-probe produced no live non-scripted router tx and no live REAL-6B.
```

Not ratified:

```text
E2 spontaneous market action;
E3 persistent role differentiation;
E4 causal performance signal;
live REAL-6B approval;
market-caused solve improvement;
model ranking;
autonomous secondary market.
```

## Top-Level Whitebox Mapping

Quantification:

```text
StructuralMarketTx / AgentEconomicActionTx / ScriptedFixtureTx / ResolutionTx split;
MarketOpportunityTrace actionable counts;
PnL/risk/balance summary;
NoTradeReason distribution.
```

Broadcast:

```text
Trader PromptCapsule / role-scoped view receives market, PnL, risk, balance signals;
dashboard report materializes ChainTape/CAS counts.
```

Shielding:

```text
No raw prompt/completion/CoT stored;
MarketOpportunityTrace stores CIDs and typed summary only;
reports remain materialized views, not source of truth.
```

## Notes

`audit_dashboard` §K is a G7 structural-smoke view. Its `no_forced_live_investment`
and `no_ghost_liquidity` booleans depend on REAL-7 structural guard records and
router-count minimum-tier conditions; they are not used as REAL-11 Atom 5
sentinels. REAL-11's enforced sentinels are the runner-level `live_real6b=false`,
`attempt_prediction_fixture_count=0`, scripted TaskOutcome buy fail-closed checks,
aggregate `audit_tape=PROCEED`, and explicit no-forced/no-price-as-truth claim
gates. In short: those §K booleans are not used as REAL-11 Atom 5 sentinels.
