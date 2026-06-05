# TC-002 Boot Trust-Root Manifest

Status: ready
Owner lane: substrate
Risk class: Class 2 verifier, Class 3 caution
FC nodes: FC2 boot, FC1 predicate root, FC3 trust-root review
Dependencies: TC-000
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/boot_trust_root_manifest.rs`
- `src/bin/turingos/cmd_boot.rs`
- `src/bin/turingos.rs`
- `tests/constitution_tc_boot_trust_root_manifest.rs`
- `scripts/constitution_gates.manifest.toml`
- `tests/fixtures/liveness/production_module_liveness.toml`
- `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`

Forbidden paths:

- `genesis_payload.toml`
- typed transaction schema
- sequencer admission
- canonical signing payloads
- CAS schema
- constitution or flowchart authority

Task:

Verify trust-root manifest bytes without mutating trust-root authority.

Test-first requirements:

- constitution hash verification passes for current file.
- predicate manifest root verification passes.
- ref contract verification passes.
- one SHA mismatch fixture fails closed.

Ship gate:

```bash
cargo test --test constitution_tc_boot_trust_root_manifest --no-fail-fast
cargo test --test constitution_matrix_drift --no-fail-fast
bash scripts/run_constitution_gates.sh
```

Expected: all commands exit 0.

Audit: Constitution Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE <artifact>`.
