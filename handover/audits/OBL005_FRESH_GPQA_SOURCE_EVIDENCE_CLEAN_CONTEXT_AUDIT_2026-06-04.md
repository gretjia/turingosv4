# OBL-005 Fresh GPQA Source Evidence Clean-Context Audit

Date: 2026-06-04

Reviewer: Claude Code headless, clean context, no session persistence

Task ID: OBL005_FRESH_GPQA_SOURCE_EVIDENCE_AUDIT_2026_06_04

Workspace: `/home/zephryj/projects/turingosv4-main`

Risk class: Class 2

Verdict: NO-VIOLATION

## Scope

The witness was asked to inspect the current diff, the fresh GPQA evidence
root, and the reconciliation/ledger updates without seeing the implementation
transcript. The audit focused on whether the source blockers were removed only
where justified by current-source ChainTape/CAS receipts and whether OBL-005
remains open.

Touched FC nodes/invariants:

- FC1 WorkTx / ChainTape / CAS evidence path
- FC2 replay and source receipt verification
- FC3 full-system participation / derived liveness only

## Evidence Checked

- `tests/fixtures/liveness/true_suite_evidence_reconciliation.toml`
- `OBLIGATIONS.md`
- `handover/ai-direct/LATEST.md`
- `handover/evidence/true_suite/obl005_fresh_gpqa_20260604T183931Z/gpqa/full_system_participation.json`
- `handover/evidence/true_suite/obl005_fresh_gpqa_20260604T183931Z/gpqa/gpqa_science_reasoning_manifest.json`
- `handover/evidence/true_suite/obl005_fresh_gpqa_20260604T183931Z/gpqa/replay_report.json`
- `handover/evidence/true_suite/obl005_fresh_gpqa_20260604T183931Z/gpqa/restore_replay_report.json`
- `handover/evidence/true_suite/obl005_fresh_gpqa_20260604T183931Z/evidence_package_manifest.json`
- `handover/evidence/true_suite/obl005_fresh_gpqa_20260604T183931Z/runner_execution_results.jsonl`
- `handover/evidence/true_suite/obl005_fresh_gpqa_20260604T183931Z/broad_batch/broad_agi_batch_manifest.json`

## Witness Findings

- Reconciliation target: PASS. The `gpqa_science_reasoning` broad-family row
  now points to `obl005_fresh_gpqa_20260604T183931Z`.
- Source blockers: PASS. The full-system receipt records
  `/verdict/final_closure_possible=true` and a 40-hex
  `source_tree.commit=7b12e9f1fc6469682af6d5f4e8a2cba18ba0c0d2`, so removing
  `source_receipt_final_closure_false` and `source_tree_fingerprint_missing`
  is receipt-derived.
- Retained blockers: PASS. The GPQA domain manifest remains
  `closure_scope=domain_adapter_smoke_only` with
  `final_closure_possible=false`, and OBL-005 still lacks a fresh final closure
  witness. `domain_receipt_final_closure_false` and
  `fresh_final_closure_witness_missing` are correctly retained.
- Replay and closure boundary: PASS. `replay_report.json` and
  `restore_replay_report.json` show all canonical replay indicators true with
  `replay_failure=null`; `broad_agi_batch_manifest.json` keeps
  `closure_status=OPEN_REAL_WORLD_COVERAGE_PENDING` and
  `full_system_closure_candidate=false`.
- Leakage boundary: PASS. The inspected evidence contains prompt/provider
  response hashes only, not raw provider payloads, raw prompts, CoT, API keys,
  or secrets.
- Count consistency: PASS. The witness independently counted
  `source_receipt_final_closure_false=8`, `source_tree_fingerprint_missing=8`,
  `domain_receipt_final_closure_false=13`,
  `fresh_final_closure_witness_missing=21`,
  `benchmark_capability_not_solved=10`,
  `domain_receipt_final_closure_missing=5`, and
  `market_no_or_short_side_missing=2`.

## Orchestrator Verification Observed

- `scripts/run_true_suite_broad_agi_batch.sh --execute-installed --run-id obl005_fresh_gpqa_20260604T183931Z --runners gpqa_science_reasoning_fresh`: exit 0.
- `cargo test -p turingosv4 --test constitution_true_suite_evidence_reconciliation --test constitution_obl005_final_closure_witness --test constitution_realworld_liveness_coverage --test constitution_matrix_drift -- --nocapture`: exit 0.
- `git diff --check`: exit 0.
- Refined secret scan: `SECRET_SCAN_REFINED=PASS`.
- Refined raw payload scan: `RAW_PAYLOAD_SCAN_REFINED=PASS`.
- AGY blocker-selection advisory: `VALID`.

## Verdict

NO-VIOLATION
