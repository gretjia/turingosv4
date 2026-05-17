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
| A | 10 | 0 | 1 | 0 | 162 | 0 | 10000 | 21 |
| B | 10 | 0 | 7 | 0 | 158 | 0 | 10000 | 27 |
| C | 10 | 0 | 7 | 0 | 144 | 0 | 10000 | 27 |
| D | 10 | 0 | 0 | 0 | 179 | 0 | 10000 | 20 |

## Improved Metrics

- `wasted_attempts`
- `failed_branch_count`
- `ev_to_action_conversion`

## Failure Reasons

- `e2_verifier_not_proceed`
- `e2_verifier_failure_reasons_present`

## Forbidden Claim Boundary

This report supports candidate-only language. It does not ship a mechanism or authorize achieved-status wording.
