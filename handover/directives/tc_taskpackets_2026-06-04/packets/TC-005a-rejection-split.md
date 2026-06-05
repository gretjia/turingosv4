# TC-005a Rejection Split

Status: ready
Owner lane: substrate
Risk class: Class 2 unless sequencer changes are required
FC nodes: FC1 predicate rejection, L4/L4.E split
Dependencies: TC-004b
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/bottom_white/ledger/rejection_evidence.rs`
- `tests/tc_l4_l4e_split.rs`

Forbidden paths:

- `src/state/sequencer.rs` unless reclassified
- typed transaction schema
- signing payloads

Task:

Prove rejected predicate paths advance only L4.E and never mutate accepted L4
or accepted world state.

Tests first:

- `rejected_predicate_advances_only_l4e`
- `accepted_world_unchanged_after_rejection`

Required assertions:

- L4 count/head unchanged after rejection.
- L4.E count/head changes after rejection.
- accepted state root before and after rejection is identical.

Ship gate:

```bash
cargo test --test tc_l4_l4e_split --no-fail-fast
```

Expected: command exits 0.

Audit: Data-integrity Auditor.
Verdict: `NO-VIOLATION` or `SECOND-SOURCE-DRIFT <view>`.
