# TC-004b wtool Accepted Write Triple

Status: ready
Owner lane: substrate
Risk class: Class 2 unless restricted surface is needed
FC nodes: FC1 `predicates -> wtool -> Q_{t+1}`
Dependencies: TC-004a
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/state/head_t_witness.rs`
- `tests/tc_qstate_triple.rs`

Forbidden paths:

- `src/state/sequencer.rs` unless explicitly reclassified
- typed transaction schema
- signing payloads
- restricted surfaces

Task:

Prove accepted writes advance the canonical accepted head witness and do not
use alias refs as authority.

Test first:

`wtool_accepted_commit_advances_l4_and_updates_head_witness`.

Assertions:

- accepted write advances `refs/chaintape/l4`.
- witness reads canonical C2 head.
- alias refs may be repairable, but cannot outrank canonical C2.

Ship gate:

```bash
cargo test --test tc_qstate_triple --no-fail-fast
cargo test --test constitution_matrix_drift --no-fail-fast
```

Expected: both commands exit 0.

Audit: Constitution Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE <artifact>`.
