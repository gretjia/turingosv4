# OBL005 Source-Tree Receipt Identity Clean-Context Audit

Date: 2026-06-04
Task id: OBL005_SOURCE_TREE_RECEIPT_IDENTITY
Risk: Class 2
Touched FC: FC1/FC2/FC3 evidence reconciliation and true-suite runner evidence

## Scope

Review the current diff for the source-tree receipt identity atom. The witness was asked to verify:

- `full_system_participation_current_kernel` records the current TuringOS source tree commit, not the runtime ChainTape head.
- `replay.head_commit_oid_hex` remains the runtime ChainTape head and is not reused as source proof.
- Dirty source-tree state is recorded explicitly.
- True-suite runners pass `--source-root "$PROJECT_ROOT"`.
- Historical evidence is not rewritten.
- OBL-005 remains `in_progress`.
- Web/CLI/kernel boundaries are not changed by this diff.

## Orchestrator Evidence

- `cargo test --test constitution_true_suite_evidence_reconciliation -- --nocapture` passed 11/11.
- `bash scripts/run_constitution_gates.sh` passed with `[k-1-5] total=164 failed=0`.
- `cargo test --workspace --no-fail-fast` passed, exit 0.
- `rustfmt --edition 2021 --check` on touched Rust files passed.
- `bash -n scripts/run_true_suite_*.sh && git diff --check` passed.

## Witness Routing Note

Two Claude CLI attempts were terminated as infrastructure failures after long silent runs without a verdict. The audit was retried with AGY as the clean-context witness, using the same diff and invariant list. This follows the platform-agnostic clean-context audit doctrine.

## Witness Verdict

```json
{
  "agent": "agy",
  "task_id": "OBL005_SOURCE_TREE_RECEIPT_IDENTITY",
  "workspace": "/home/zephryj/projects/turingosv4-main",
  "verdict": "NO-VIOLATION",
  "findings": [
    "All invariants verified successfully.",
    "full_system_participation_current_kernel correctly queries the current TuringOS source tree repository via git2 to resolve the source_tree.commit.",
    "The replay head_commit_oid_hex remains untouched and mapped from the replay report.",
    "Dirty worktree status is recorded explicitly via status mapping to either clean or dirty_allowed_recorded.",
    "True-suite runner scripts updated to pass --source-root $PROJECT_ROOT.",
    "No changes to existing historical evidence records.",
    "OBL-005 remains in progress with no closure claim.",
    "Web/CLI/kernel boundaries are not impacted by this diff."
  ],
  "summary": "The diff implements source tree fingerprinting for full-system participation receipts. It queries git2 to identify the commit and status of the current TuringOS source tree, preserving the separate runtime ChainTape head identification. All invocation scripts pass the --source-root argument. The verification checks pass successfully.",
  "files_reviewed": [
    "scripts/run_true_suite_boot_cli_current_kernel.sh",
    "scripts/run_true_suite_cybench_current_kernel.sh",
    "scripts/run_true_suite_fc3_governance_reinit_current_kernel.sh",
    "scripts/run_true_suite_gaia_general_assistant_current_kernel.sh",
    "scripts/run_true_suite_generate_artifact_current_kernel.sh",
    "scripts/run_true_suite_gpqa_science_reasoning_current_kernel.sh",
    "scripts/run_true_suite_market_ab_current_kernel.sh",
    "scripts/run_true_suite_market_external_agent.sh",
    "scripts/run_true_suite_math_competition_current_kernel.sh",
    "scripts/run_true_suite_mind2web_current_kernel.sh",
    "scripts/run_true_suite_osworld_current_kernel.sh",
    "scripts/run_true_suite_replay_cas_tamper_current_kernel.sh",
    "scripts/run_true_suite_swebench_current_kernel.sh",
    "scripts/run_true_suite_tdma_current_kernel.sh",
    "scripts/run_true_suite_toolbench_current_kernel.sh",
    "scripts/run_true_suite_webarena_current_kernel.sh",
    "src/bin/full_system_participation_current_kernel.rs",
    "tests/constitution_true_suite_boot_cli_runner.rs",
    "tests/constitution_true_suite_evidence_reconciliation.rs",
    "tests/support/full_system.rs"
  ]
}
```
