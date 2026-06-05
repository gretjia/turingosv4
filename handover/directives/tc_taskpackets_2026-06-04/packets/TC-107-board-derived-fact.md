# TC-107 Board Derived Fact

Status: ready
Owner lane: audit
Risk class: Class 1 derived-view test
FC nodes: FC1 derived views, FC3 second-source drift
Dependencies: TC-101
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/tc_tape_canonical.rs`
- `tests/tc_tape_canonical_repairs.rs`

Forbidden paths: board/dashboard authority paths as canonical input.

Task:

Prove board/dashboard facts are reconstructable views, not truth.

Test first:

`board_fact_is_reconstructable_view_not_truth`.

Assertions:

- board fact kind is `board_derived`.
- constructor rejects `canonical_input = "dashboard"` and
  `canonical_input = "board"`.
- fact must cite ChainTape/CAS source head.

Ship gate:

```bash
cargo test --test tc_tape_canonical_repairs --no-fail-fast
```

Expected: command exits 0.

Audit: Second-source drift Auditor.
Verdict: `NO-VIOLATION` or `SECOND-SOURCE-DRIFT <view>`.
