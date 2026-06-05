# TC-012A G0 Manifest Freeze

Status: ready
Owner lane: lean-search
Risk class: Class 2 bounded completeness
FC nodes: FC1 search spine
Dependencies: TC-011A
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/g0_completeness.rs`
- `tests/tc_g0_completeness.rs`

Forbidden paths: kernel, bus, sequencer, typed-tx schema.

Task:

Freeze G0 manifest validation for bounded completeness.

Tests first:

- `g0_manifest_rejects_duplicates_empty_and_hidden_automation`
- `g0_manifest_hash_changes_on_any_atom_change`

Blocked atoms:

`sorry`, `admit`, `native_decide`, `decide`, `omega`, `aesop`, `simp_all`,
`raw:*`.

Ship gate:

```bash
cargo test --test tc_g0_completeness --no-fail-fast
```

Expected: command exits 0.

Audit: Formal-methods Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND hidden-automation <file>:<line>`.
