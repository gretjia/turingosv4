# TC-109 Halt Fact

Status: ready
Owner lane: reliability
Risk class: Class 2
FC nodes: FC2 halt, FC3 replay
Dependencies: TC-101, TC-007d
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/tc_tape_canonical.rs`
- `src/runtime/external_call.rs`
- `tests/tc_tape_canonical_repairs.rs`
- `tests/tc_external_call_records.rs`

Forbidden paths: typed-tx schema, sequencer, trust-root authority.

Task:

Clean halt fact must require balanced side-effect lifecycle.

Test first:

`halt_fact_blocks_clean_claim_with_pending_side_effect`.

Assertions:

- halt fact cannot be clean when any external call is pending.
- halt fact includes source head and side-effect summary hash.
- terminal summary cannot contain completion language if pending count > 0.

Ship gate:

```bash
cargo test --test tc_tape_canonical_repairs --test tc_external_call_records --no-fail-fast
```

Expected: command exits 0.

Audit: Reliability Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE <artifact>`.
