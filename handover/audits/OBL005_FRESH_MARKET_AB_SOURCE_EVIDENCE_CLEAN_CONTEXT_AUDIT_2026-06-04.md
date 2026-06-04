# OBL-005 Fresh Market A/B Source Evidence Clean-Context Audit

Date: 2026-06-04

Reviewer: Claude Code headless, clean context, no session persistence

Task ID: OBL005_FRESH_MARKET_AB_G0_BOUNDARY_AUDIT_2026_06_04

Workspace: `/home/zephryj/projects/turingosv4-main`

Risk class: Class 2

Verdict: NO-VIOLATION

## Scope

The witness was asked to inspect the current workspace diff, the fresh market
A/B evidence run, and the G0 boundary tests without seeing the implementation
transcript. The audit focused on whether this PR honestly proves only core
market price discovery while keeping c4/c5 priced-DAG branching and c10/c11
reward-claim settlement closure as constrained/stage-2.

Touched FC nodes/invariants:

- FC1 market/WorkTx/ChainTape/CAS evidence path
- FC2 replay/source receipt verification
- FC3 derived liveness/matrix only

## Evidence Checked

- `src/bin/g0_market_activation_current_kernel.rs`
- `tests/constitution_g0_market_activation_boundary.rs`
- `tests/constitution_real16_market_performance.rs`
- `scripts/constitution_gates.manifest.toml`
- `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
- `tests/fixtures/liveness/true_suite_evidence_reconciliation.toml`
- `OBLIGATIONS.md`
- `handover/ai-direct/LATEST.md`
- `handover/evidence/true_suite/obl005_fresh_market_ab_20260604T175932Z/market_ab/g0/g0_market_activation_manifest.json`
- `handover/evidence/true_suite/obl005_fresh_market_ab_20260604T175932Z/market_ab/g0/replay_report.json`
- `handover/evidence/true_suite/obl005_fresh_market_ab_20260604T175932Z/market_ab/market_ab_run_manifest.json`
- `handover/evidence/true_suite/obl005_fresh_market_ab_20260604T175932Z/market_ab/full_system_participation.json`
- `handover/evidence/true_suite/obl005_fresh_market_ab_20260604T175932Z/runner_execution_results.jsonl`

## Witness Findings

- G0 no-overclaim: PASS. The source and manifest use
  `g0_core_market_price_discovery_conditions_1_2_3_6_7_8_9`, carry
  c4/c5 constraint notes, and carry c10/c11 stage-2 notes. The boundary test
  mechanically rejects c1-11 closure language.
- G0 market-side evidence: PASS. The manifest records `worktx_count=1`,
  `challengetx_count=1`, `buy_yes_count=1`, `buy_no_count=1`, price movement,
  and replay-green evidence.
- Trust-Root boundary: PASS. `tests/constitution_real16_market_performance.rs`
  remains Trust-Root pinned and restored to its genesis hash; the new G0 tests
  live in `tests/constitution_g0_market_activation_boundary.rs`, which is not
  in `genesis_payload.toml` and is wired through the constitution gate manifest
  and execution matrix.
- Reconciliation boundary: PASS. The market A/B row removes only the
  source-receipt/source-tree/no-short blockers justified by this fresh evidence
  and retains `domain_receipt_final_closure_missing` plus
  `fresh_final_closure_witness_missing`.
- Closure and leakage boundary: PASS. OBL-005 remains `in_progress`; no
  historical evidence rewrite or final-closure overclaim was found, and no
  credentials or provider transcript payloads were found in the inspected
  evidence paths.

## Commands Run By Witness

- `grep -c constitution_g0_market_activation_boundary genesis_payload.toml`
  returned 0.
- `grep constitution_real16_market_performance genesis_payload.toml` confirmed
  the Trust-Root hash `6e499d13...`.
- `grep -c buy_no_count src/bin/g0_market_activation_current_kernel.rs`
  confirmed the NO-side receipt field is present.
- `grep -c g0_market_activation_boundary|G0 market activation handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
  confirmed the matrix row is present.

## Orchestrator Verification Observed

- Focused market/reconciliation/flowchart package: exit 0.
- Trust-Root intact boot unit: exit 0.
- `rustfmt --edition 2021 --check` on touched Rust files: exit 0.
- `bash -n scripts/run_true_suite_market_ab_current_kernel.sh scripts/run_true_suite_broad_agi_batch.sh && git diff --check`: exit 0.
- `cargo test -p turingosv4 --test constitution_matrix_drift -- --nocapture`: exit 0.
- `bash scripts/run_constitution_gates.sh`: exit 0, `[k-1-5] total=165 failed=0`.
- `cargo test --workspace --no-fail-fast`: exit 0.

## Verdict

NO-VIOLATION
