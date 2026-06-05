# TC-005b L4.E Reconstruction

Status: ready
Owner lane: substrate
Risk class: Class 2
FC nodes: FC3 replay, L4.E evidence
Dependencies: TC-005a
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/audit_assertions.rs`
- `tests/tc_l4_l4e_split.rs`

Forbidden paths: restricted surfaces.

Task:

Add a reconstruction assertion showing L4.E JSONL and git attestation agree.

Test first:

`accepted_and_rejected_refs_reconstruct_independently`.

Failure fixtures:

- missing JSONL record for L4.E ref
- orphan L4.E ref
- body hash mismatch

Rule:

L4.E diagnostics remain shielded. Public views never expose raw diagnostics.

Ship gate:

```bash
cargo test --test tc_l4_l4e_split --no-fail-fast
```

Expected: command exits 0.

Audit: Data-integrity Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE <artifact>`.
