# REAL-10 Decision Gate Report - CLEAN REAL-8X Evidence

Scope: Class 0 analysis report only. No source or test files were edited.
Touched invariants: FC1 market/action evidence and predicate interpretation; FC2
pinned benchmark evidence; FC3 report/materialized-view claim boundary.

Contamination boundary: this report uses only the CLEAN rerun evidence under
`handover/evidence/real8x_market_ab_clean_20260515T141331Z/` and does not use
the invalid prior directory `handover/evidence/real8x_market_ab_20260515T134453Z/`.

Primary sources:
`handover/evidence/real8x_market_ab_clean_20260515T141331Z/REAL8_MARKET_AB_BENCHMARK_REPORT.md`;
`handover/evidence/real8x_market_ab_clean_20260515T141331Z/real8_arm_summary.tsv`;
`handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_config_manifests/REAL8X_CONFIG_AUDIT.json`;
`handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_A/aggregate_verdict.json`;
`handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_B/aggregate_verdict.json`;
`handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_C/aggregate_verdict.json`;
`handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_D/aggregate_verdict.json`;
`handover/alignment/EMERGENCE_METRICS_E1_E2_E3_E4.md`;
`handover/directives/2026-05-15_REAL10_CONTROLLED_MARKET_EVIDENCE_EXPANSION_EXECUTION_PLAN.md`.

Environmental caveat: clean rerun stdout reported dirty-tree non-evidence
changes and free disk around 19G, below the 20G recommendation. This caveat does
not invalidate the listed chain-backed arm results, but it should be carried
forward for larger reruns.

## Evidence Table

| Arm | Condition | exit/audit/tasks | market_tx_count | buy_with_coin_router | solve_rate | Wilson 95% CI | failed_branch/wasted | verification_latency_ms_mean |
| --- | --- | ---: | ---: | ---: | ---: | --- | ---: | ---: |
| A | market disabled | 0 / PROCEED / 15 | 0 | 0 | 5/15 | 0.1518..0.5829 | 50 / 50 | 12918.0667 |
| B | market visible, no TaskOutcomeMarket | 0 / PROCEED / 15 | 10 | 0 | 5/15 | 0.1518..0.5829 | 50 / 50 | 7747.6 |
| C | TaskOutcomeMarket enabled | 0 / PROCEED / 15 | 42 | 0 | 6/15 | 0.1982..0.6425 | 45 / 45 | 7702.8667 |
| D | TaskOutcomeMarket + scripted AttemptPrediction fixture | 0 / PROCEED / 15 | 38 | 0 | 4/15 | 0.1090..0.5195 | 55 / 55 | 10386.3333 |

Config audit: `disallowed_config_drift=[]` in
`handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_config_manifests/REAL8X_CONFIG_AUDIT.json`.

Aggregate verdicts show all arms have `buy_with_coin_router=0`; market-visible
arms increased market machinery tx through market seed/pool/event activity, not
through live agent router buys.

## E1/E2/E3/E4 Verdicts

E1 - Market Visibility: SATISFIED for market-visible arms B/C/D. The arms have
market-visible configuration and chain-backed market activity, with B=10, C=42,
and D=38 market tx counted in the benchmark report.

E2 - Spontaneous Market Action: NOT ACHIEVED. The E2 metric requires at least one
live, non-scripted, agent-generated `BuyWithCoinRouterTx` or short-equivalent,
ChainTape/CAS visible, with no forced or scripted action. All arms report
`buy_with_coin_router=0`. Arm D includes a scripted AttemptPrediction fixture,
which is explicitly non-qualifying for E2.

E3 - Persistent Role Differentiation: NOT ESTABLISHED. The benchmark reports
`role_diversity_index=5`, but the E3 rule requires persistent, distinct
ChainTape/CAS-derived action distributions across roles and tasks/batches. A
role diversity index alone is insufficient.

E4 - Causal Performance Signal: NOT ESTABLISHED. Market activity increased in
B/C/D, but solve-rate Wilson intervals overlap: A/B `0.1518..0.5829`, C
`0.1982..0.6425`, D `0.1090..0.5195`. C is descriptively higher at 6/15 and has
lower failed_branch/wasted counts plus low latency; B has the lowest comparable
latency among non-scripted market-visible arms; D is lower solve and higher
waste. These are descriptive signals only, not statistical support for a causal
performance gain.

## Decision Gate

Decision: lawful market machinery is active in the CLEAN REAL-8X market-visible
arms, but there is no spontaneous market action and no causal performance gain.

Allowed conclusion: REAL-8X CLEAN supports a narrow statement that pinned,
audited market-visible conditions produced more market activity than market
disabled A, while preserving all arm exits as 0 and audits as PROCEED.

Not allowed conclusion: REAL-8X CLEAN does not support autonomous market
emergence, live non-scripted router/short behavior, persistent role
differentiation, or market-caused solve-rate/PPUT improvement.

## Next Recommendation

Run a larger and cleaner follow-up only if the goal is to investigate the
descriptive latency/waste signal. The next benchmark should preserve pinned
problem/model/budget/config controls, avoid scripted fixtures for E2 claims, keep
`buy_with_coin_router` or a short-equivalent as the E2 trigger, and carry forward
the disk/dirty-tree preflight caveat.

If the priority is E2 rather than performance, prepare a separate Class 4 live
REAL-6B packet with explicit ratification, timing/settlement semantics, abort
path, replay invariants, and no price-as-truth proof path. Do not treat D's
scripted fixture as a substitute.

## Forbidden Claims

- Autonomous market emergence.
- Spontaneous market action achieved.
- Live REAL-6B approval or live real-LLM AttemptPrediction approval.
- Causal performance improvement, model ranking, or market-caused solve-rate gain.
- Persistent role differentiation established from `role_diversity_index=5`.
- Price-as-truth, proof-by-price, ghost liquidity, forced trade, real-money or
  public-chain readiness.
- Off-tape WAL as truth, private CoT recording, raw-log broadcast, or dashboard
  counters as primary evidence.
