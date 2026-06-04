# OBL-005 Report JSON Liveness Boundary — Clean-Context Audit

Date: 2026-06-04
Reviewer: AGY clean-context witness
Risk class: Class 2 evidence-boundary hardening
Verdict domain: `NO-VIOLATION | VIOLATION-FOUND | RECONSTRUCTION-FAILURE | SECOND-SOURCE-DRIFT`

## Verdict

`NO-VIOLATION`

## Scope

The witness inspected the branch diff and the four touched liveness accounting
files:

- `tests/constitution_production_module_liveness.rs`
- `tests/constitution_realworld_liveness_coverage.rs`
- `tests/fixtures/liveness/production_module_liveness.toml`
- `tests/fixtures/liveness/realworld_liveness_coverage.toml`

The intended boundary is that report-like JSON files such as
`candidate_report`, `performance_report`, and `REAL*_REPORT.json` cannot
satisfy production/current liveness evidence or final real-world evidence
claims. Historical report files may remain archived, but current closing
manifests must rely on machine-checkable ChainTape/CAS/replay/manifest
receipts.

## Findings

- The diff introduces verification constraints that reject report-like JSON as
  liveness and final evidence.
- `tests/constitution_production_module_liveness.rs` now identifies
  `candidate_report`, `performance_report`, and `REAL*_REPORT.json` style
  paths for rejection in current true-suite receipt checks.
- `tests/constitution_realworld_liveness_coverage.rs` now rejects the same
  report-like JSON shapes through the final evidence path hygiene check.
- The fixture references to `REAL16_MARKET_PERFORMANCE_REPORT.json` were
  removed from production liveness and real-world final evidence manifests.
- No production source files or restricted Class 4 surfaces listed in
  `AGENTS.md` were touched.

## Evidence Checked

- Diff: `/tmp/obl005_report_json.diff`
- Focused witness test command:
  `timeout 120 cargo test --test constitution_production_module_liveness --test constitution_realworld_liveness_coverage -- --nocapture`
  returned exit code 0.

## Witness Log

The witness run log was captured at:

`/tmp/agy_obl005_report_json_audit.log`
