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
| A | 1 | PROCEED | 14 | 5/14 | 0.1634..0.6124 | 2.316171796136764e-05 | 6.485281029182938e-05 | 0 | 14465 | 72325/308920ms | 0 | none_observed | -1500000..295000 | 5 | 45 | 16155.785714285714 | 45 | 0 |
| B | 1 | PROCEED | 14 | 6/14 | 0.2138..0.6741 | 2.3282003723854363e-05 | 5.4324675355660187e-05 | 0 | 13310 | 79863/222199ms | 14 | none_observed | -1500000..393000 | 5 | 40 | 9936.92857142857 | 40 | 0 |
