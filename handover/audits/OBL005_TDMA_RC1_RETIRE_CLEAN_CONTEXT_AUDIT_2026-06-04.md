# OBL-005 TDMA RC1 Retire Clean-Context Audit

Date: 2026-06-04

Scope: Review the dirty diff for retiring historical TDMA RC1 standalone bins
from production source and liveness accounting while preserving the current TDMA
production path.

Risk class: Class 2

Touched invariants:
- OBL-005 current-tree no-zombie liveness accounting
- FC1 rtool/input evidence boundary for TDMA evidence
- FC2 production entrypoint remains `turingos tdma run`
- FC3 replay/CAS evidence remains derived from current true-suite receipts

Witness command:

```bash
codex exec -C /home/zephryj/projects/turingosv4-main -s read-only --ephemeral <clean-context-audit-prompt>
```

Witness summary:

- Inspected dirty diff only.
- Changed paths do not touch AGENTS.md restricted surfaces.
- Deleted `tdma_rc1_*` bins are removed from production liveness accounting.
- Old `tdma_zero_gain_demo` evidence paths are no longer counted.
- New regression test blocks retired modules, source paths, and evidence rows
  from re-entering production accounting.
- Current TDMA substrate remains present through `turingos tdma run`,
  `src/tdma_runner.rs`, `src/bin/tdma_proof_current_kernel.rs`, true-suite TDMA
  tests, and full-system ChainTape/CAS bridge evidence rows.
- `OBLIGATIONS.md` and current liveness manifests keep OBL-005 reopened /
  re-audit in progress, with no current final-closure claim.
- `git diff --check` passed.

Verdict:

```text
NO-VIOLATION
```
