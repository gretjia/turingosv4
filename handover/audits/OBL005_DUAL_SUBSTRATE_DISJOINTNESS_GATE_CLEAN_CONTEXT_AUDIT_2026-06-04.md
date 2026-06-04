# OBL-005 Dual Substrate Disjointness Gate Clean-Context Audit

Date: 2026-06-04

Workspace: `/home/zephryj/projects/turingosv4-main`

Branch: `codex/dual-substrate-disjointness-gate`

Risk class: Class 2 test-gate hardening only.

Touched FC / invariant: FC1a-substrate_seam / Art. 0.4 tape substrate
authority boundary.

Changed files under audit:

- `tests/dual_substrate_disjointness.rs`

## Scope

Audit whether the new gate legitimately proves the retained TDMA
`GitTapeLedger` and runtime ChainTape `Git2LedgerWriter` remain separate
substrate authorities:

- disjoint ref namespaces
- disjoint default repo directories
- disjoint git object pools

The audit also checked that this change does not rewrite evidence, does not
use dashboards or stdout as proof, does not touch Class 4 surfaces, and does
not claim final OBL-005 closure.

## Deterministic Evidence

Fresh local verification before witness:

```text
cargo test --test dual_substrate_disjointness -- --nocapture
# 3 passed; 0 failed

rustfmt --edition 2021 --check tests/dual_substrate_disjointness.rs
# exit 0

cargo test --test constitution_matrix_drift -- --nocapture
# 3 passed; 0 failed

cargo test --test constitution_production_module_liveness -- --nocapture
# 19 passed; 0 failed

bash scripts/run_constitution_gates.sh
# [k-1-5] total=164 failed=0

cargo test --workspace --no-fail-fast
# exit 0
```

## Witness Results

AGY clean-context read-only audit:

```json
{
  "agent": "agy",
  "task_id": "OBL005_DUAL_SUBSTRATE_DISJOINTNESS_GATE_AUDIT",
  "verdict": "NO-VIOLATION",
  "status": "complete"
}
```

Claude clean-context witness:

```json
{
  "agent": "claude",
  "task_id": "OBL005_DUAL_SUBSTRATE_DISJOINTNESS_GATE_CLAUDE_WITNESS_R2",
  "verdict": "NO-VIOLATION",
  "status": "complete"
}
```

Claude R1 timed out without output and was discarded as an infrastructure
failure, not counted as a witness verdict. Claude R2 completed successfully
with `NO-VIOLATION`.

## Verdict

NO-VIOLATION.

The new test is a legitimate OBL-005 support gate. It strengthens substrate
accounting for retained TDMA and runtime ChainTape git-backed ledgers, but it
does not close OBL-005. OBL-005 remains `in_progress` pending full current-tree
ChainTape/CAS final-closure evidence for every retained module/script group.
