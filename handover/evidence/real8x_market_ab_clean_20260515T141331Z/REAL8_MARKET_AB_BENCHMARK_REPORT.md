# REAL-8 Formal Market A/B Benchmark

This report is descriptive benchmark evidence only. It does not claim causality.
Negative result is valid and documented.

## Pinned Inputs

| Pin | SHA-256 |
| --- | --- |
| same problem set | `0c484c4e6cfc949f608ad5ee568f86edb56b32d387cf1f8a375e4f044f82f437` |
| same model assignment | `62d1e5862881ff8124ffa0159df78c62f91dde52cedbdd5fb966774440051526` |
| same budgets | `70d88fcf2cf0e0b8826145b9176237be58820e9006faaa3fe9435f418859a42e` |
| same seed/config except arm toggles | `52b6e553430c25bb2902db6fb208973535941deb57b3f7a14103b2ad90abd176` |
| tasks per arm | `15` |

Forbidden claim boundary:

```text
no forced trades
no price-as-truth
no ghost liquidity
no f64 economy
no off-tape WAL as truth
no private CoT recording
no raw-log broadcast
```

## Arms

| Arm | Condition |
| --- | --- |
| A | market disabled |
| B | market visible, no TaskOutcomeMarket |
| C | TaskOutcomeMarket enabled |
| D | TaskOutcomeMarket + scripted AttemptPrediction fixture |

## Metrics

| Arm | exit | audit | tasks | solve_rate | wilson_ci_95 | verified_pput_mean | mean_pput_solved | false_accept_rate_mean | cost_per_verified_proof_tokens | cost_time_tokens_ms | market_tx_count | no_trade_reason_distribution | pnl_dispersion_micro | role_diversity_index | failed_branch_count | verification_latency_ms_mean | wasted_attempts | audit_failure_rate |
| --- | ---: | --- | ---: | --- | --- | ---: | ---: | ---: | --- | --- | ---: | --- | --- | ---: | ---: | ---: | ---: | ---: |
| A | 0 | PROCEED | 15 | 5/15 | 0.1518..0.5829 | 2.702478436032902e-05 | 8.107435308098706e-05 | 0 | 15911 | 79555/296702ms | 0 | none_observed | -1500000..295000 | 5 | 50 | 12918.066666666668 | 50 | 0 |
| B | 0 | PROCEED | 15 | 5/15 | 0.1518..0.5829 | 2.2452001484638334e-05 | 6.7356004453915e-05 | 0 | 17330 | 86651/220596ms | 10 | none_observed | -1500000..295000 | 5 | 50 | 7747.6 | 50 | 0 |
| C | 0 | PROCEED | 15 | 6/15 | 0.1982..0.6425 | 2.6566475655613922e-05 | 6.64161891390348e-05 | 0 | 13994 | 83966/208032ms | 42 | invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5 | -2100000..194000 | 5 | 45 | 7702.866666666667 | 45 | 0 |
| D | 0 | PROCEED | 15 | 4/15 | 0.1090..0.5195 | 1.6642574812388545e-05 | 6.240965554645705e-05 | 0 | 22977 | 91909/282403ms | 38 | invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5;invest_no_trade_no_perceived_edge=5 | -1900000..0 | 5 | 55 | 10386.333333333334 | 55 | 0 |

## Gate Verdicts

```text
SG-8.1 Same problem set across arms: PASS (single pinned problem manifest hash above).
SG-8.2 Same model assignment: PASS (single pinned model manifest hash above).
SG-8.3 Same budgets: PASS (single pinned budget manifest hash above).
SG-8.4 Same seed/config except arm toggles: PASS iff arm shared-config hashes match and arm toggles are allowlisted.
SG-8.5 All runs chain-backed: PASS iff every arm exit=0 and audit=PROCEED.
SG-8.6 No overclaim of causality: PASS (this report is descriptive evidence only).
SG-8.7 Negative result is valid and documented: PASS (undefined/no-effect metrics are retained, not rewritten).
```

## Claim Boundary

REAL-8 does not claim that a market arm caused higher solve rate, higher PPUT,
role differentiation, or spontaneous trading. It reports chain-backed
observations under pinned A/B/C/D conditions. A negative result is a valid
scientific result and must remain in the handover evidence.
