# OBL-005 Product Workload Liveness Rebind Clean-Context Audit

Date: 2026-06-04
Reviewer: AGY headless clean-context witness
Risk class: Class 2
Scope: `tests/constitution_production_module_liveness.rs`, `tests/fixtures/liveness/production_module_liveness.toml`, `OBLIGATIONS.md`

## Task

Audit whether OBL-005 product workload liveness rows and the production `real_world_suite`
registry stop citing historical REAL/stage reports, transcripts, and logs as current
production liveness evidence. Retained product workload evidence must come from current
`handover/evidence/true_suite/*` receipts/artifacts and bind at least one replay-green
`full_system_participation.json` receipt.

## Evidence Reviewed

- Working tree diff at `/tmp/obl005_product_workload_rebind.diff`
- `tests/constitution_production_module_liveness.rs`
- `tests/fixtures/liveness/production_module_liveness.toml`
- `OBLIGATIONS.md`

## Verification Reported To Witness

- RED pre-fix targeted gate failed on `agent_prompt_model_boundary` citing `handover/evidence/real_bcast_1_hard10_B_20260516T100140Z/REAL12_TASK_MARKET_PROBE_REPORT.md`.
- `rustfmt --edition 2021 --check tests/constitution_production_module_liveness.rs`: pass.
- `git diff --check`: pass.
- Historical-pattern scan against `tests/fixtures/liveness/production_module_liveness.toml`: no matches.
- `cargo test --test constitution_production_module_liveness -- --nocapture`: 19 passed.
- Adjacent OBL-005 gates: `constitution_realworld_liveness_coverage`, `constitution_true_suite_evidence_reconciliation`, `constitution_script_liveness_inventory`, `constitution_obl005_final_closure_witness`, `constitution_matrix_drift`: pass.
- `cargo check --workspace`: pass.
- `bash scripts/run_constitution_gates.sh`: `[k-1-5] total=164 failed=0`.
- `cargo test --workspace --no-fail-fast`: pass.

## Witness Findings

The witness found no violation. It confirmed:

- The diff adds executable enforcement through `product_workload_groups_use_current_true_suite_receipts`.
- Product workload rows remain `allowed_as_fc_authority = false`.
- Historical evidence files are not rewritten.
- OBL-005 remains reopened / re-audit in progress and does not claim final closure.
- No `src/`, runtime, sequencer, typed transaction schema, money, CAS integrity, or trust-root surface is modified.

## Verdict

NO-VIOLATION
