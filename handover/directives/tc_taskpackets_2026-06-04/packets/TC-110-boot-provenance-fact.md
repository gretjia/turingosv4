# TC-110 Boot Provenance Fact

Status: ready
Owner lane: substrate
Risk class: Class 2 verifier
FC nodes: FC2 boot, FC3 trust-root review
Dependencies: TC-002, TC-101
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/boot_trust_root_manifest.rs`
- `src/runtime/tc_tape_canonical.rs`
- `tests/tc_tape_canonical_repairs.rs`
- `tests/constitution_tc_boot_trust_root_manifest.rs`

Forbidden paths: trust-root mutation, constitution, typed-tx schema.

Task:

Bind boot provenance facts to manifest hashes, locked refs, and predicate root.
This is verifier evidence only.

Test first:

`boot_provenance_binds_manifest_hashes_refs_and_predicate_root`.

Assertions:

- constitution hash included.
- genesis payload hash included.
- predicate root included.
- locked ref contract included.
- no trust-root file mutation occurs.

Ship gate:

```bash
cargo test --test tc_tape_canonical_repairs --test constitution_tc_boot_trust_root_manifest --no-fail-fast
```

Expected: command exits 0.

Audit: Constitution Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE <artifact>`.
