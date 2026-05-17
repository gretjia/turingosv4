# REAL-16 Market Performance Report

claim_boundary: `E4 candidate pending audit`
claim_note: candidate-only; not E4 achieved

verdict: `Proceed`
e4_candidate: `true`

## Source Boundary

ChainTape/CAS/exact-join verifier-derived metrics only; dashboard text and evaluator stdout are not source of truth

## Arms

| arm | tasks | solved | exact_join | verified_pput_micro | wasted | latency_ms_total | role_diversity_bps | market_tx_count |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| A | 10 | 0 | 0 | 0 | 173 | 0 | 10000 | 21 |
| B | 10 | 0 | 9 | 0 | 180 | 0 | 10000 | 29 |
| C | 10 | 0 | 3 | 0 | 166 | 0 | 10000 | 23 |
| D | 10 | 0 | 16 | 0 | 149 | 0 | 10000 | 36 |

## Improved Metrics

- `wasted_attempts`
- `failed_branch_count`
- `ev_to_action_conversion`

## Failure Reasons

none

## Forbidden Claim Boundary

This report supports candidate-only language. It does not ship a mechanism or authorize achieved-status wording.
