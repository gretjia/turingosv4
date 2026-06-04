# OBL-005 Fresh Generate Source Evidence Clean-Context Audit

Date: 2026-06-04

Auditor: Claude headless clean-context witness

Task ID: OBL005-fresh-generate-source-evidence-audit-2026-06-04

Workspace: `/home/zephryj/projects/turingosv4-main`

Branch: `codex/obl005-generate-no-remote-fonts`

Base: `origin/main`

Risk class: Class 2

Touched FC nodes/invariants:
- FC1 output -> predicates -> wtool
- FC2 replay/restore
- FC3 full-system augment / derived evidence

## Scope

The witness was asked to inspect the diff from `origin/main...HEAD`, the prompt
hardening in `src/bin/turingos/cmd_generate.rs`, the new evidence under
`handover/evidence/true_suite/obl005_fresh_generate_20260604T171500Z/`, and the
fixture/docs updates binding `generate_artifact_chain_fresh` to that run.

The witness was also asked to check that OBL-005 remains in progress, no final
closure is overclaimed, the failed local attempt
`obl005_fresh_generate_20260604T160500Z` is not committed as GREEN evidence, and
no derived view usurps ChainTape/CAS ground truth.

## Witness Notes

The witness reported that focused `cargo test` invocations were blocked by its
tool permission context. It therefore did not independently rerun those cargo
commands, and instead inspected the workspace, git diff, committed evidence
JSON/TOML, and gate source logic. This audit is paired with orchestrator-run
verification evidence recorded separately in this PR.

## Findings

- Diff scope is Class 2: the branch edits `src/bin/turingos/cmd_generate.rs`,
  `tests/fixtures/liveness/true_suite_evidence_reconciliation.toml`,
  `OBLIGATIONS.md`, `handover/ai-direct/LATEST.md`, and adds the fresh
  `171500Z` evidence directory.
- No restricted Class 4 surface was edited: no sequencer, typed transaction,
  CAS schema, wallet, kernel, bus, trust-root, or constitution/flowchart file
  was changed.
- Prompt hardening is real and fail-capable: the old remote-font allowances are
  removed, and the new unit test asserts that the blackbox prompt forbids remote
  fonts/CDNs/external runtime URLs.
- The committed evidence is the successful
  `obl005_fresh_generate_20260604T171500Z` run. The failed
  `obl005_fresh_generate_20260604T160500Z` attempt is untracked and not part of
  the commit.
- Replay/restore evidence reconstructs cleanly: `replay_report.json` and
  `restore_replay_report.json` carry matching roots and all replay indicators
  true; `full_system_participation.json` reports `FULL_SYSTEM_LIT`,
  `missing: []`, FC1/FC2/FC3 present, and source commit
  `7dd5b3d80b0313520b01c9c0fc56bd7117ff8b63`.
- Blocker reduction is evidence-derived: removing
  `source_receipt_final_closure_false` and
  `source_tree_fingerprint_missing` for the generate row is required by the new
  closing source receipt and 40-hex source commit. The row correctly retains
  `domain_receipt_final_closure_missing` and
  `fresh_final_closure_witness_missing`.
- No closure overclaim was found: the fixture still keeps
  `final_closure_claimed = false`, and OBL-005 remains `in_progress`.
- No source-of-truth drift was found: the reconciliation TOML remains a derived
  view validated against ChainTape/CAS-derived receipts.

## Verdict

NO-VIOLATION
