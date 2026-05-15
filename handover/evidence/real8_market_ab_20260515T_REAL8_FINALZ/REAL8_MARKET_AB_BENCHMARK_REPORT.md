# REAL-8 Formal Market A/B Benchmark

This report is descriptive benchmark evidence only. It does not claim causality.
Negative result is valid and documented.

## Pinned Inputs

| Pin | SHA-256 |
| --- | --- |
| same problem set | `a7bbb29cec0726769e5bb39a602c54713e7007bb717731f31298437d4b2367e8` |
| same model assignment | `62d1e5862881ff8124ffa0159df78c62f91dde52cedbdd5fb966774440051526` |
| same budgets | `70d88fcf2cf0e0b8826145b9176237be58820e9006faaa3fe9435f418859a42e` |

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
| A | 0 | PROCEED | 3 | 2/3 | 6.408417874077507e-05 | 0 | 3765 | 0 | none_observed | -300000..0 | 5 | 0 |
| B | 0 | PROCEED | 3 | 2/3 | 5.823441602130985e-05 | 0 | 4268 | 4 | none_observed | -300000..0 | 5 | 0 |
| C | 0 | PROCEED | 3 | 2/3 | 5.879789962533711e-05 | 0 | 4745 | 10 | invest_no_trade_no_perceived_edge=5 | -500000..0 | 5 | 0 |
| D | 0 | PROCEED | 3 | 2/3 | 4.5265617164744224e-05 | 0 | 4204 | 10 | invest_no_trade_no_perceived_edge=5 | -500000..0 | 5 | 0 |

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
