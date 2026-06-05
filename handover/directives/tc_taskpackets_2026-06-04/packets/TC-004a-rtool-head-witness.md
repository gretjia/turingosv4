# TC-004a rtool HEAD_t Witness

Status: ready
Owner lane: substrate
Risk class: Class 2 unless restricted surface is needed
FC nodes: FC1 `Q_t -> rtool -> input`
Dependencies: TC-003c
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/state/head_t_witness.rs`
- `src/state/mod.rs`
- `tests/tc_qstate_triple.rs`

Forbidden paths:

- all restricted surfaces
- any global latest pointer as canonical input

Task:

Expose a small read-only witness that reconstructs `Q_t = <q_t, HEAD_t, tape_t>`
from refs and files only.

Test first:

`rtool_reconstructs_q_head_t_tape_t_from_refs_only`.

Assertions:

- accepted L4 head read from `refs/chaintape/l4`.
- rejected L4.E head read from `refs/chaintape/l4e`.
- CAS root read from locked ref or explicit absent state.
- no `LATEST.md`, dashboard, or stdout log is read.

Stop condition:

If implementation requires `src/state/sequencer.rs`, `src/state/typed_tx.rs`,
or signing payload edits, stop for reclassification.

Ship gate:

```bash
cargo test --test tc_qstate_triple --no-fail-fast
git diff --name-only origin/main...HEAD | grep -E 'src/(kernel|bus|state/sequencer|state/typed_tx|sdk/tools/wallet|bottom_white/cas/schema)\.rs'
```

Expected: cargo test exits 0; restricted grep has no output.

Audit: Constitution Auditor.
Verdict: `NO-VIOLATION` or `SECOND-SOURCE-DRIFT <view>`.
