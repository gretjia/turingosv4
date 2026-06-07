# A03 KEEP-SRC-BOOT Clean-Context Audit

Date: 2026-06-06

Workspace:
`/home/zephryj/projects/turingosv4-a03-keep-src-boot`

Reviewer:
Fresh Codex CLI clean-context witness, read-only sandbox, ephemeral session.

Task:
A03 KEEP-SRC-BOOT landing after exact user Section-8 ratification
`APPROVE-A03-SECTION8-KEEP-SRC-BOOT`.

Risk:
Class 3 floor with Class 4 boundary awareness around boot / Trust Root
authority.

FC nodes / invariants:

- FC2 boot Trust Root verification
- FC3-N34 readonly Trust Root guard
- `src/boot.rs::verify_trust_root` remains the single authority
- existing public `turingos boot --verify-manifest` CLI hook is used
- no `src/runtime/boot_trust_root_manifest.rs` wrapper
- no trust-root rehash
- no env bypass
- no source authority move

Implementation evidence provided to witness:

- `cargo fmt --check`: exit 0
- `git diff --check`: exit 0
- `cargo test --test constitution_tc_boot_trust_root_manifest --no-fail-fast -- --test-threads=1`: 8 passed, 0 failed
- `bash scripts/run_constitution_gates.sh`: exit 0, `[k-1-5] total=168 failed=0`
- `cargo test --workspace --no-fail-fast`: exit 0
- source-only bypass grep over `src/main.rs`, `src/boot.rs`, and
  `src/bin/turingos/cmd_boot.rs`: no matches
- `test ! -e src/runtime/boot_trust_root_manifest.rs`: exit 0

Witness read-only checks:

- inspected `git status` / `git diff`, including the untracked A03 test
- inspected `tests/constitution_tc_boot_trust_root_manifest.rs`
- inspected `src/bin/turingos.rs`, `src/bin/turingos/cmd_boot.rs`,
  `src/main.rs`, and `src/boot.rs`
- confirmed the gate manifest row has a matching execution-matrix row
- confirmed OBL Level=must entries are satisfied and no `Status: open`
  remains
- independently reran `git diff --check` and `cargo fmt --check` with exit 0

Read-only caveat:

The witness attempted to rerun the focused A03 cargo test inside a read-only
sandbox. Cargo failed before entering the test body because it could not open
`target/debug/.cargo-lock` on a read-only filesystem. The witness did not count
that attempted rerun as a pass; it relied on the implementation environment's
fresh successful focused test, constitution gates, and workspace test evidence.

Verdict:

```json
{"task_id":"A03_KEEP_SRC_BOOT_AUDIT","verdict":"NO-VIOLATION","detail":"Inspected current status/diff including untracked test. Diff is limited to OBL/LATEST/matrix/directive/gate manifest plus tests/constitution_tc_boot_trust_root_manifest.rs; no restricted source diff. src/bin/turingos.rs dispatches boot to cmd_boot::run; cmd_boot.rs and src/main.rs call turingosv4::boot::verify_trust_root; src/runtime/boot_trust_root_manifest.rs is absent; no env/catch bypass surfaced in boot sources. Gate manifest row has matching matrix row. Test file contains meaningful fail-closed checks for intact repo, file hash mismatch, constitution hash mismatch, child manifest mismatch, missing trust_root, missing constitution_root, bypass env non-effect, and no second authority. OBL Level=must entries are satisfied with no Status: open. git diff --check and cargo fmt --check exit 0. Focused cargo test could not be rerun in this read-only sandbox because Cargo failed opening target/debug/.cargo-lock.","checked_files":["OBLIGATIONS.md","handover/ai-direct/LATEST.md","handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md","handover/directives/2026-06-05_A03_BOOT_TRUST_ROOT_MANIFEST_PREFLIGHT_AND_SECTION8_REQUEST.md","scripts/constitution_gates.manifest.toml","tests/constitution_tc_boot_trust_root_manifest.rs","src/bin/turingos.rs","src/bin/turingos/cmd_boot.rs","src/main.rs","src/boot.rs","build.rs","genesis_payload.toml"]}
```

