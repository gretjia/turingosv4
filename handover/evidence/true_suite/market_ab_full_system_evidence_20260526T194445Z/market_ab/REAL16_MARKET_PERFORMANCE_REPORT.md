# REAL-16 Market Performance Report

claim_boundary: `clean-negative; no E4 candidate`
claim_note: candidate-only; not E4 achieved

verdict: `Veto`
e4_candidate: `false`

## Source Boundary

ChainTape/CAS/exact-join verifier-derived metrics only; dashboard text and evaluator stdout are not source of truth

## Arms

| arm | tasks | solved | exact_join | verified_pput_micro | wasted | latency_ms_total | role_diversity_bps | market_tx_count |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| A | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 6 |
| D | 1 | 0 | 1 | 0 | 1 | 0 | 0 | 6 |

## Improved Metrics

- `ev_to_action_conversion`

## Failure Reasons

- `e2_verifier_not_proceed`
- `e2_verifier_failure_reasons_present`

## Forbidden Claim Boundary

This report supports candidate-only language. It does not ship a mechanism or authorize achieved-status wording.
