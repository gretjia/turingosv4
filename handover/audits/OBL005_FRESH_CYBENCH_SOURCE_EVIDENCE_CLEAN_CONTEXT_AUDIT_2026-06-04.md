# OBL-005 Fresh Cybench Source Evidence Clean-Context Audit

Date: 2026-06-04

Reviewer: Claude headless, clean context

Workspace: `/home/zephryj/projects/turingosv4-main`

Risk class: Class 2 evidence reconciliation

FC trace: FC1 WorkTx/predicate/wtool evidence; FC2 replay/restore verification; FC3 full-system participation accounting.

Verdict: `NO-VIOLATION`

## Scope

Review the branch that rebinds the Cybench true-suite rows to fresh current-source evidence:

- `tests/fixtures/liveness/true_suite_evidence_reconciliation.toml`
- `OBLIGATIONS.md`
- `handover/ai-direct/LATEST.md`
- `handover/evidence/true_suite/obl005_fresh_cybench_20260604T164533Z/`

The branch must reduce source blockers for `cybench_security_sandbox_fresh` and `cybench_security_sandbox` while preserving domain/benchmark/fresh-witness blockers. It must not claim final OBL-005 closure.

## Finding Summary

No constitutional or evidence-boundary violation found.

The diff only rebinds the two Cybench rows to `obl005_fresh_cybench_20260604T164533Z` and removes `source_receipt_final_closure_false` plus `source_tree_fingerprint_missing` from those rows. The receipt records `source_tree.commit=0f38026e4d03177ad4b6641086e9f1e98f751e8b`, `verdict.final_closure_possible=true`, `FULL_SYSTEM_LIT`, `missing=[]`, and green replay/restore indicators. The remaining blockers are receipt-derived:

- `domain_receipt_final_closure_false`
- `benchmark_capability_not_solved`
- `fresh_final_closure_witness_missing`

The reviewer found no evidence rewrite, no dashboard/stdout-only proof, no restricted-surface mutation, no second source of truth, and no OBL-005 final-closure overclaim.

## Notes

The reviewer recorded one non-blocking P3 note: the unrelated untracked failed evidence directory `handover/evidence/true_suite/obl005_fresh_generate_20260604T160500Z/` exists locally, but is not referenced by this diff. It must not be staged for this PR.

Claude's environment denied recursive `rg` over the evidence directory and targeted `cargo test` execution. The witness therefore verified the receipt fields against the reconciliation gate logic and relied on the orchestrator's reported local GREEN evidence for:

- real Cybench runner exit 0
- focused reconciliation/Cybench/final-closure/matrix tests
- `bash scripts/run_constitution_gates.sh`
- `cargo test --workspace --no-fail-fast`
- `git diff --check`
- local secret scan

## Machine Verdict

```json
{
  "agent": "claude",
  "workspace": "/home/zephryj/projects/turingosv4-main",
  "ok": true,
  "verdict": "NO-VIOLATION",
  "summary": "Cybench source receipt rebinding is receipt-derived, keeps non-source blockers, and makes no final-closure claim."
}
```
