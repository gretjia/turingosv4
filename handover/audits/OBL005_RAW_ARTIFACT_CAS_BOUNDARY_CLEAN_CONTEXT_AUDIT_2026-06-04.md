# OBL-005 Raw Artifact CAS Boundary Clean-Context Audit

- Date: 2026-06-04
- Task id: `obl005-raw-artifact-cas-boundary-fast-audit`
- Workspace: `/home/zephryj/projects/turingosv4-main`
- Auditor: clean-context AGY headless witness
- Risk class: Class 2 evidence/liveness gate hardening
- Verdict domain: `NO-VIOLATION | VIOLATION-FOUND | RECONSTRUCTION-FAILURE | SECOND-SOURCE-DRIFT`
- Verdict: `NO-VIOLATION`

## Scope

Audit the current diff against OBL-005 evidence-boundary requirements:

- Raw provider output, human-readable files, browser traces, DOM logs,
  screenshots, and browser HTML must not satisfy current real-world liveness
  or final evidence unless they are bound back to CAS/replay receipts.
- Broad AGI family manifests that cite raw observation artifacts must declare
  observation or trace CAS CIDs plus CAS-bound/quarantine guards.
- True-suite reconciliation must prove that any raw observation final artifact
  template is backed by a full-system receipt with CAS path, CAS payload
  retrievability, and observation/trace CIDs.
- Historical `handover/evidence/` receipts must remain immutable.
- No Class 4 restricted surface from `AGENTS.md` section 6 may be touched.

## Witness Output

```json
{
  "agent": "agy",
  "task_id": "obl005-raw-artifact-cas-boundary-fast-audit",
  "workspace": "/home/zephryj/projects/turingosv4-main",
  "verdict": "NO-VIOLATION",
  "findings": [],
  "summary": "The diff successfully strengthens the validation gates in the true_suite, realworld_liveness, and true_suite_evidence_reconciliation tests. It ensures that raw provider outputs, browser traces, screenshots, and DOM logs cannot satisfy OBL-005 liveness unless they are CAS-bound (validated via observation/trace CIDs, quarantine guards, and full-system replay receipts)."
}
```

## Owner Verification Before Witness

- `rustfmt --edition 2021 --check tests/constitution_realworld_liveness_coverage.rs tests/constitution_broad_agi_true_suite_manifest.rs tests/constitution_true_suite_evidence_reconciliation.rs`
- `cargo test --test constitution_realworld_liveness_coverage -- --nocapture`
- `cargo test --test constitution_broad_agi_true_suite_manifest -- --nocapture`
- `cargo test --test constitution_true_suite_evidence_reconciliation -- --nocapture`
- `cargo test --test constitution_production_module_liveness -- --nocapture`
- `cargo test --test constitution_obl005_final_closure_witness -- --nocapture`
- `cargo test --test constitution_matrix_drift -- --nocapture`
- `bash scripts/run_constitution_gates.sh` returned `[k-1-5] total=164 failed=0`
- `cargo test --workspace --no-fail-fast`
- `git diff --check`

## Orchestrator Notes

The first read-only AGY scanner timed out before producing a validated witness
object, and the first Claude command failed before invoking Claude because of a
shell-pipe syntax error. A later slow Claude/AGY audit pair was abandoned after
the validated fast AGY witness above and deterministic gates were available.
Those failed or abandoned runs are not used as ship evidence.
