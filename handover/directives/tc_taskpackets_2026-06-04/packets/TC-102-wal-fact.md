# TC-102 WAL Recovery Fact

Status: ready
Owner lane: gateway
Risk class: Class 1 additive helper
FC nodes: FC1 side effect record, FC3 replay
Dependencies: TC-101
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/tc_tape_canonical.rs`
- `tests/tc_tape_canonical_repairs.rs`

Forbidden paths:

- `src/wal.rs`
- typed-tx schema
- sequencer
- CAS schema

Task:

Define WAL as a recovery/outbox fact only. Do not resurrect old WAL authority or
make a sidecar a replay input.

Test first:

`wal_fact_is_recovery_log_not_authority`.

Assertions:

- WAL fact requires `TapeAnchor`.
- WAL fact has `kind == "wal_recovery"`.
- derived replay result is unchanged if WAL fact is absent.
- WAL can explain recovery but cannot decide accepted state.

Ship gate:

```bash
cargo test --test tc_tape_canonical_repairs --no-fail-fast
git diff --name-only origin/main...HEAD | grep 'src/wal.rs'
```

Expected: cargo test exits 0; grep has no output.

Audit: Reliability Auditor.
Verdict: `NO-VIOLATION` or `SECOND-SOURCE-DRIFT <view>`.
