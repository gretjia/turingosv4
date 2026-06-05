# TC-007b Crash Terminal Mapping

Status: ready
Owner lane: gateway
Risk class: Class 2
FC nodes: FC1 external call, FC2 recovery, FC3 replay
Dependencies: TC-007a
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/external_call.rs`
- `tests/tc_external_call_records.rs`

Forbidden paths: provider network in tests, typed-tx schema.

Task:

Map crash recovery states to deterministic terminal records.

Test first:

`crash_states_map_to_deterministic_terminals`.

Locked semantics:

- durable intent before send and no send marker -> `Abandoned { may_have_spent: false }`
- send marker and no terminal -> `Abandoned { may_have_spent: true }`
- HTTP timeout or transport error -> retryable `Failure`
- parse fail after response -> non-retryable `Failure`
- parsed success -> `Result`

Ship gate:

```bash
cargo test --test tc_external_call_records --no-fail-fast
```

Expected: command exits 0.

Audit: Reliability Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE <artifact>`.
