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
| C | 10 | 0 | 17 | 0 | 154 | 0 | 10000 | 37 |

## Improved Metrics

none

## Failure Reasons

- `fewer_than_two_ab_arms`

## Forbidden Claim Boundary

This report supports candidate-only language. It does not ship a mechanism or authorize achieved-status wording.
