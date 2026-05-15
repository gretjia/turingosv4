# REAL-8 Formal Market A/B Benchmark

This report is descriptive benchmark evidence only. It does not claim causality.
Negative result is valid and documented.

## Pinned Inputs

| Pin | SHA-256 |
| --- | --- |
| same problem set | `a7bbb29cec0726769e5bb39a602c54713e7007bb717731f31298437d4b2367e8` |
| same model assignment | `62d1e5862881ff8124ffa0159df78c62f91dde52cedbdd5fb966774440051526` |
| same budgets | `fbb435c4615a70cc70a687b3683d305d8f6f6c505c40f280fc6c9f01bb7e3f5b` |

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

| Arm | exit | audit | tasks | solve_rate | verified_pput_mean | false_accept_rate_mean | cost_per_verified_proof_tokens | market_tx_count | no_trade_reason_distribution | pnl_dispersion_micro | role_diversity_index | audit_failure_rate |
| --- | ---: | --- | ---: | --- | ---: | ---: | --- | ---: | --- | --- | ---: | ---: |
| A | 0 | PROCEED | 3 | 2/3 | 3.693726217093287e-05 | 0 | 3545 | 0 | none_observed | -300000..0 | 5 | 0 |
| B | 0 | PROCEED | 3 | 1/3 | 3.0717105259318254e-05 | 0 | 14072 | 2 | none_observed | -300000..0 | 5 | 0 |
| C | 1 | PROCEED | 1 | 0/1 | 0 | 0 | undefined_no_verified_proof | 6 | invest_no_trade_no_perceived_edge=5 | -300000..0 | 5 | 0 |
| D | 1 | PROCEED | 1 | 0/1 | 0 | 0 | undefined_no_verified_proof | 6 | invest_no_trade_no_perceived_edge=5 | -300000..0 | 5 | 0 |

## Gate Verdicts

```text
SG-8.1 Same problem set across arms: PASS (single pinned problem manifest hash above).
SG-8.2 Same model assignment: PASS (single pinned model manifest hash above).
SG-8.3 Same budgets: PASS (single pinned budget manifest hash above).
SG-8.4 All runs chain-backed: PASS iff every arm exit=0 and audit=PROCEED.
SG-8.5 No overclaim of causality: PASS (this report is descriptive evidence only).
SG-8.6 Negative result is valid and documented: PASS (undefined/no-effect metrics are retained, not rewritten).
```

## Claim Boundary

REAL-8 does not claim that a market arm caused higher solve rate, higher PPUT,
role differentiation, or spontaneous trading. It reports chain-backed
observations under pinned A/B/C/D conditions. A negative result is a valid
scientific result and must remain in the handover evidence.
