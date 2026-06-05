# TC-007d Clean Halt Gate

Status: ready
Owner lane: gateway
Risk class: Class 2
FC nodes: FC2 halt, FC3 replay
Dependencies: TC-007c
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/external_call.rs`
- `tests/tc_external_call_records.rs`

Forbidden paths: typed-tx schema, sequencer.

Task:

Expose a fail-closed clean-halt predicate for external-call lifecycle balance.

Test first:

`pending_external_call_blocks_clean_claim`.

Assertion:

`Intent count == Result + Failure + Abandoned` and `pending_count == 0` must be
true before clean completion language is allowed.

Ship gate:

```bash
cargo test --test tc_external_call_records --no-fail-fast
```

Expected: command exits 0.

Audit: Reliability Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE <artifact>`.
