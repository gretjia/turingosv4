# OBL-005 Final Closure Blocker Audit — 2026-05-27

Active obligations: OBL-001 open, OBL-004 in_progress, OBL-005 in_progress.

## Objective

Determine whether the current mainline state can close OBL-005: no
constitutional drift, no zombie retained module, no unconstitutional retained
module, and all retained production code groups lit by real ChainTape/CAS or
explicitly excluded.

## Prompt-to-artifact checklist

| Requirement | Evidence inspected | Result |
|---|---|---|
| Sync main and check open PR overlap | `git fetch origin main --prune`; `gh pr list --state open --json number,headRefName,title,files` returned no rows; `HEAD == origin/main == 2ce179b8416125b8fdcbde77a2b91e28ffe036da` before this audit branch | PASS |
| Rebuild production liveness inventory from code/tests/evidence | `cargo test --test constitution_production_module_liveness -- --nocapture` | PASS: 11 passed, 0 failed |
| Verify all exported module groups are classified | `tests/constitution_production_module_liveness.rs::every_exported_module_has_exactly_one_liveness_group`; `tests/fixtures/liveness/production_module_liveness.toml` | PASS for scanned exported Rust module/bin inventory |
| Verify real-world and broad-family true-suite evidence reconciliation | PR #201 merged `tests/constitution_true_suite_evidence_reconciliation.rs`; prior verification passed and clean-context audit returned `NO-VIOLATION` | PASS as candidate |
| Verify reconciliation is final closure, not candidate-only | `tests/fixtures/liveness/true_suite_evidence_reconciliation.toml`; `tests/constitution_true_suite_evidence_reconciliation.rs` | BLOCKED: `reconciliation_status = "FULL_SYSTEM_RECONCILIATION_CANDIDATE"`, `final_closure_claimed = false`, and the test asserts realworld/broad manifests remain `OPEN_REAL_WORLD_COVERAGE_PENDING` |
| Verify OBL-004 no-unconstitutional-code repair ledger is closed | `OBLIGATIONS.md`; merged PR query over constitution-repair branches; local audit file search | BLOCKED: PR-A (#139) and predicate registry (#140) are merged, but PR-B/PR-C/PR-D/PR-E entries remain unreconciled placeholders and `handover/audits/CONSTITUTION_REPAIR_R1R2R3_SYNTHESIS_2026-05-24.md` is absent |
| Verify retained scripts are first-class no-zombie accounted | `scripts/`; `tests/constitution_script_liveness_inventory.rs`; `tests/fixtures/liveness/script_liveness_inventory.toml` | GAP FOUND AND GATED: prior Rust module/bin inventory did not cover `scripts/`; this PR adds a script inventory gate that classifies every retained script exactly once |
| Verify OBL-001 does not block overall done/complete claim | `OBLIGATIONS.md` | BLOCKED: OBL-001 remains `open` and has no evidence path |

## Findings

1. OBL-005 is close to closure but is still candidate-only by executable
   contract. The reconciliation gate proves all 10 real-world domains and all
   11 broad AGI families have `FULL_SYSTEM_LIT` evidence, but it deliberately
   refuses to claim final closure.

2. The remaining OBL-005 blocker is not another benchmark runner. The next
   required action is obligation/repair reconciliation:
   - Reconcile OBL-004 PR-B/PR-C/PR-D/PR-E against actual merged PRs or mark
     each as blocked with concrete proof.
   - If any OBL-004 item is still a real code defect, implement it before
     OBL-005 final closure.
   - After OBL-004 is reconciled, run a final OBL-005 closure witness and only
     then flip `final_closure_claimed` / `final_closure_status`.

3. `src/sdk/prompt.rs::build_agent_prompt` is not currently a silent zombie:
   it is covered as `agent_prompt_model_boundary` in
   `tests/fixtures/liveness/production_module_liveness.toml` and appears in
   active G3/G5/REAL-12 tests. This conflicts with the older OBL-004 PR-E text
   that expected deletion, so OBL-004 needs reconciliation rather than an
   unreviewed deletion.

4. The audit found one mechanically closeable no-zombie gap: retained
   `scripts/` files were not covered by the Rust module/bin liveness inventory.
   This PR adds `tests/constitution_script_liveness_inventory.rs` and
   `tests/fixtures/liveness/script_liveness_inventory.toml` so every script is
   classified exactly once. True-suite production scripts bind to realworld or
   broad contracts; historical/dev-only scripts cannot count toward final
   closure.

## Verdict

`OBL005-FINAL-CLOSURE-BLOCKED OBL-004`

Do not mark OBL-005 satisfied yet. The next PR should reconcile or complete
OBL-004; after that, a final closure PR may update the candidate-only
reconciliation manifest and OBL ledger if the closure witness passes.
