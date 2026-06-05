# TC-007a Durable External-Call Outbox

Status: ready
Owner lane: gateway
Risk class: Class 2
FC nodes: FC1 external call, FC3 replay
Dependencies: TC-101
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/external_call.rs`
- `tests/tc_external_call_records.rs`

Forbidden paths: typed-tx schema, sequencer, CAS schema.

Task:

Persist external-call records to append-only JSONL at
`<runtime_repo>/external_calls.jsonl`.

Test first:

`outbox_reopens_and_preserves_pending_records`.

Assertions:

- intent is durable before terminal.
- reopening preserves pending intent.
- duplicate intent id fails closed.
- malformed JSONL fails closed with explicit error.

Ship gate:

```bash
cargo test --test tc_external_call_records --no-fail-fast
```

Expected: command exits 0.

Audit: Reliability Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE <artifact>`.
