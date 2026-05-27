# OBL-005 Script Inventory Clean-Context Audit — 2026-05-27

Reviewer: clean-context Codex
Session: `019e6856-ed82-7e23-8380-30457c277448`
Risk class: Class 2 derived liveness/accounting gate

## Scope

Review the current workspace diff on branch
`codex/obl005-closure-blocker-audit`:

- `OBLIGATIONS.md`
- `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
- `scripts/constitution_gates.manifest.toml`
- `handover/audits/OBL005_FINAL_CLOSURE_BLOCKER_AUDIT_2026-05-27.md`
- `tests/constitution_script_liveness_inventory.rs`
- `tests/fixtures/liveness/script_liveness_inventory.toml`

Touched invariants:

- FC1/FC2/FC3 evidence accounting only.
- No-zombie liveness inventory for retained `scripts/` files.
- Derived inventories must not become canonical truth.
- OBL-005 remains candidate-only while OBL-004 and OBL-001 are unresolved.

## Witness Checks

- `git status --short --branch`
- `git diff --name-only -- .`
- `git ls-files --others --exclude-standard`
- `git diff -- OBLIGATIONS.md`
- `git diff -- handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
- `git diff -- scripts/constitution_gates.manifest.toml`
- `nl -ba tests/constitution_script_liveness_inventory.rs`
- `nl -ba tests/fixtures/liveness/script_liveness_inventory.toml`
- `nl -ba handover/audits/OBL005_FINAL_CLOSURE_BLOCKER_AUDIT_2026-05-27.md`
- `find scripts -type f | sort`
- `find scripts -type l -ls`
- `git diff --check`

## Evidence Notes

- The diff and untracked set matched the expected Class 2 accounting files.
- The script inventory test recursively expands declared `scripts/` paths,
  rejects missing or non-`scripts/` paths, rejects duplicate claims, and
  compares the claimed set to `find scripts -type f`.
- Historical/dev/local-probe groups are required to set
  `counts_for_obl005_script_closure = false`.
- Production entrypoint groups must bind to existing realworld task IDs or
  broad family IDs.
- The matrix, obligation ledger, and blocker audit preserve candidate-only
  language and state that OBL-005 is still blocked by OBL-004 reconciliation
  and OBL-001.
- No restricted Class 4 surface, historical evidence rewrite, secret leak,
  or derived-view authority inversion was found.

## Verdict

`NO-VIOLATION`
