# TC-013B Market Invariance

Status: ready
Owner lane: lean-search
Risk class: Class 2 scheduler invariant
FC nodes: FC1 search spine, market signal boundary
Dependencies: TC-013A
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/g0_completeness.rs`
- `tests/tc_g0_completeness.rs`

Forbidden paths: market/economy authority, verifier authority.

Task:

Prove price, market, and autonomous routing can change odd-lane trace only.

Tests first:

- `market_toggle_changes_only_odd_trace`
- `odd_shuffle_keeps_even_trace_byte_identical`

Assertions:

- even candidate order is byte-identical across odd queue variants.
- price cannot alter rank.
- price cannot suppress even candidate.
- no global priority heap mixes lanes.

Ship gate:

```bash
cargo test --test tc_g0_completeness --no-fail-fast
```

Expected: command exits 0.

Audit: Formal-methods Auditor.
Verdict: `NO-VIOLATION` or `SECOND-SOURCE-DRIFT rank-input`.
