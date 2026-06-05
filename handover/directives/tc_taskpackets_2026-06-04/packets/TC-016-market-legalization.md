# TC-016 Market Legalization

Status: ready
Owner lane: lean-search
Risk class: Class 2 signal-boundary enforcement
FC nodes: FC1 scheduler/search spine, market signal boundary
Dependencies: TC-013B, TC-014B
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/g0_completeness.rs`
- `tests/tc_g0_completeness.rs`

Forbidden paths:

- market/economy authority surfaces
- verifier authority
- rank authority outside G0
- kernel, bus, sequencer, typed-tx schema

Task:

Legalize market/price input only as odd-lane priority metadata. Market price
must not influence G0 rank, even-lane coverage, verifier acceptance, predicate
truth, or final proof authority.

Tests first:

- `market_price_is_odd_lane_metadata_only`
- `market_price_cannot_change_g0_rank_digest_or_even_schedule`
- `market_price_cannot_create_verifier_acceptance`

Rules:

- Market data may reorder odd heuristic candidates.
- Market data may be included in odd-lane trace fields.
- Market data cannot be read by rank computation.
- Market data cannot mark an even candidate as attempted or covered.

Ship gate:

```bash
cargo test --test tc_g0_completeness --no-fail-fast
```

Expected: command exits 0.

Audit: Constitution Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND market-authority <file>:<line>`.
