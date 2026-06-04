# OBL-005 Market NO/Short Closure Gate Clean-Context Audit

Date: 2026-06-04

Risk class: Class 2, gate/evidence hardening only.

Scope:
- `OBLIGATIONS.md`
- `src/bin/market_external_agent_current_kernel.rs`
- `tests/constitution_true_suite_market_external_agent_runner.rs`
- `tests/constitution_true_suite_evidence_reconciliation.rs`

Intent:
- Future OBL-005 final closure must not cite YES-only market/economy receipts.
- The market external-agent runner must exercise both YES and NO provider decisions through the current-kernel path.
- Each market case stays on a single-WorkTx-node path (`work_tx_count_for_task == 1`); market side activity is router-side YES/NO action, not same-task reward fan-out.
- OBL-005 remains `in_progress`; this audit does not claim final closure.

Verification evidence reported to witnesses:
- `rustfmt --edition 2021 --check src/bin/market_external_agent_current_kernel.rs tests/constitution_true_suite_market_external_agent_runner.rs tests/constitution_true_suite_evidence_reconciliation.rs` exit 0.
- `git diff --check` exit 0.
- `cargo test --test constitution_true_suite_market_external_agent_runner -- --nocapture` exit 0.
- `cargo test --test constitution_true_suite_evidence_reconciliation --test constitution_obl005_final_closure_witness --test constitution_matrix_drift --test constitution_realworld_liveness_coverage --test constitution_broad_agi_true_suite_manifest -- --nocapture` exit 0.
- `bash scripts/run_constitution_gates.sh` exit 0, `[k-1-5] total=164 failed=0`.
- `cargo test --workspace --no-fail-fast` exit 0.

AGY witness:

```json
{
  "task_id": "OBL005_MARKET_NO_SHORT_CLOSURE_GATE_POST_AUDIT_2026_06_04",
  "workspace": "/home/zephryj/projects/turingosv4-main",
  "verdict": "NO-VIOLATION",
  "findings": [],
  "checked_files": [
    "OBLIGATIONS.md",
    "src/bin/market_external_agent_current_kernel.rs",
    "tests/constitution_true_suite_market_external_agent_runner.rs",
    "tests/constitution_true_suite_evidence_reconciliation.rs",
    "constitution.md"
  ],
  "constitutional_boundary": "The constitution is silent on WorkTx/escrow reward single-claim semantics; the single-solver/finalize constraint is a kernel-level payout safety check, while WorkTx admission permits multiple WorkTx under one task in existing tests.",
  "notes": "Uncommitted diff successfully audited. No restricted Class 4 surfaces were touched. OBL-005 remains in_progress."
}
```

Claude witness:

```json
{
  "task_id": "OBL005_MARKET_NO_SHORT_CLOSURE_GATE_CLAUDE_AUDIT_2026_06_04",
  "workspace": "/home/zephryj/projects/turingosv4-main",
  "verdict": "NO-VIOLATION",
  "findings": [],
  "constitutional_boundary": "constitution.md does not define single-WorkTx-per-task or task-escrow single-reward semantics; the work_tx_count_for_task=1 assertion is a test-contract pin on the current-kernel path, not a constitution-derived invariant, and is correctly scoped to the market external-agent bin rather than any restricted surface.",
  "notes": "All four diff files are additive gate hardening only (Class 2). OBL-005 explicitly remains in_progress in OBLIGATIONS.md; no closure claim is made. YES/NO CPMM pool economics are symmetric and internally consistent. work_tx_count_for_task is computed directly from post-drain economic_state_t.stakes_t, not a shadow counter. The reconciliation detector is structural JSON recursion, not text grep; the non-empty market binding guard prevents vacuous pass. No restricted surfaces are touched."
}
```

Constitutional boundary conclusion:

The quoted rule must not be stated as constitution text. `constitution.md` does not explicitly specify WorkTx/task-escrow single-reward semantics. The accurate statement is: current kernel payout/finalize is a single-solver safety model, while WorkTx admission still permits multiple WorkTx under one task. Therefore G0 evidence should use single WorkTx node plus multi-agent YES/NO router-side market activity, and defer same-task multi-reward settlement to M2/M3 settlement redesign or a per-task node model.
