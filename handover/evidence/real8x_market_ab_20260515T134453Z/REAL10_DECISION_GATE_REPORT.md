# REAL-10 Decision Gate Report

Risk class: Class 0 report over completed Class 4 evidence. Touched FC nodes:
FC1 market/action evidence, FC2 pinned replay/evidence packaging, FC3 derived
report boundary. This report is a materialized interpretation view, not a new
source of truth.

Sources:
`handover/evidence/real8x_market_ab_20260515T134453Z/REAL8_MARKET_AB_BENCHMARK_REPORT.md`,
`handover/evidence/real8x_market_ab_20260515T134453Z/real8_arm_summary.tsv`,
`handover/evidence/real8x_market_ab_20260515T134453Z/arm_config_manifests/REAL8X_CONFIG_AUDIT.json`,
`handover/evidence/real8x_market_ab_20260515T134453Z/arm_{A,B,C,D}/aggregate_verdict.json`,
`handover/alignment/EMERGENCE_METRICS_E1_E2_E3_E4.md`,
`handover/directives/2026-05-15_REAL10_CONTROLLED_MARKET_EVIDENCE_EXPANSION_EXECUTION_PLAN.md`.

## Evidence Table

| Arm | Condition | Audit | Tasks | Solve Rate | Wilson 95% CI | Market Tx Count | BuyWithCoinRouter | Failed/Wasted Attempts | Verification Latency Mean |
| --- | --- | --- | ---: | --- | --- | ---: | ---: | ---: | ---: |
| A | market disabled | PROCEED | 15 | 5/15 | 0.1518..0.5829 | 0 | 0 | 50 | 15078.73ms |
| B | market visible, no TaskOutcomeMarket | PROCEED | 15 | 7/15 | 0.2481..0.6988 | 14 | 0 | 40 | 10244.07ms |
| C | TaskOutcomeMarket enabled | PROCEED | 15 | 5/15 | 0.1518..0.5829 | 40 | 0 | 50 | 7792.93ms |
| D | TaskOutcomeMarket + scripted AttemptPrediction fixture | PROCEED | 15 | 5/15 | 0.1518..0.5829 | 40 | 0 | 50 | 7588.33ms |

Config audit: `disallowed_config_drift=[]`. All arm runs exited 0, all
aggregate verdicts are `PROCEED`, and persistence reports show `is_passing=true`
with `n_witnessed=5`.

Pinned-input evidence: same problem set hash
`0c484c4e6cfc949f608ad5ee568f86edb56b32d387cf1f8a375e4f044f82f437`, same
model assignment hash `62d1e5862881ff8124ffa0159df78c62f91dde52cedbdd5fb966774440051526`,
same budget hash `70d88fcf2cf0e0b8826145b9176237be58820e9006faaa3fe9435f418859a42e`,
and same seed/config except allowlisted arm toggles.

Environmental caveat: run stdout reported a working tree with non-evidence
changes and free disk around 19G, below the 20G recommendation. This is a caveat
rather than a hidden exclusion: audit_tape and persistence proceeded, and the
runner completed PASS/exit 0 for all arms.

## E1/E2/E3/E4 Verdicts

E1 Market Visibility: satisfied for market-enabled arms as controlled substrate
evidence. B/C/D expose market machinery, and C/D record NoTradeReason activity
as repeated `invest_no_trade_no_perceived_edge=5` rows. A is the disabled
control.

E2 Spontaneous Market Action: not achieved. The aggregate data shows
`buy_with_coin_router=0` in A/B/C/D. D is a scripted fixture condition only, and
scripted AttemptPrediction cannot satisfy E2 under
`handover/alignment/EMERGENCE_METRICS_E1_E2_E3_E4.md`.

E3 Persistent Role Differentiation: not established. `role_diversity_index=5`
appears in every arm, but role_diversity_index alone is not enough for E3. The
required evidence would be persistent, distinct ChainTape/CAS-derived action
distributions across at least two consecutive tasks or batches.

E4 Causal Performance Signal: not established. B has the highest solve rate
(7/15), while A/C/D are 5/15; the Wilson intervals overlap
(B 0.2481..0.6988 overlaps A/C/D 0.1518..0.5829). C/D increase market activity
to 40 market tx each but do not improve solve rate. Waste and latency signals
are descriptive only: B has fewer failed/wasted attempts (40 vs A/C/D at 50),
and latency is lower in B/C/D than A, with C/D lowest descriptively.

## Decision Gate

Gate outcome: market activity is up, but no performance gain is proven.

REAL-8X supports the narrow conclusion that lawful market machinery is active
under pinned conditions: market_tx_count rises from A=0 to B=14 and C/D=40, with
all arms audit-clean and no disallowed config drift. It does not support a claim
that markets improved solving.

Market activity remains non-spontaneous for the E2 standard. No live
non-scripted router or short-equivalent transaction exists in the aggregate
data, and D's extra behavior is scripted fixture evidence only.

There is no evidence that market context distracted Solvers: B improved solve
rate descriptively and reduced failed/wasted attempts, while C/D did not improve
solve rate. The latency means appear lower in market-enabled arms, especially
C/D, but this is descriptive and not causal E4 evidence.

## Next Recommendation

Proceed to a separate Class 4 decision only if the architect wants live E2
evidence: either a live REAL-6B packet for non-scripted AttemptPrediction/router
behavior, or stronger Trader utility/PnL visibility designed to make voluntary
market action rational. Keep the next run pinned and predeclare E2/E3/E4 gates.

If the goal is performance evidence instead of emergence evidence, run a larger
REAL-8Y-style benchmark after audit-clean packaging, preserving the same arm
toggle discipline and Wilson/comparable statistical support.

## Forbidden Claims

- Do not claim spontaneous market emergence or E2.
- Do not claim E3 from `role_diversity_index` alone.
- Do not claim E4 causality or market-caused performance gain.
- Do not claim live REAL-6B approval.
- Do not claim price-as-truth, forced trade, ghost liquidity, model ranking, or
  real-world readiness.
- Do not treat dashboards, stdout, or this report as stronger truth than
  ChainTape/CAS and aggregate verdict evidence.
