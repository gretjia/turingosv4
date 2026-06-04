# OBL-005 Fresh Mind2Web Source Evidence Clean-Context Audit

Date: 2026-06-04
Reviewer: clean-context Codex exec, read-only
Verdict: NO-VIOLATION

## Scope

This audit covers the Mind2Web source-evidence update for
`obl005_fresh_mind2web_20260604T210300Z`, including:

- `tests/fixtures/liveness/true_suite_evidence_reconciliation.toml`
- `tests/fixtures/liveness/production_module_liveness.toml`
- `OBLIGATIONS.md`
- `handover/ai-direct/LATEST.md`
- `handover/evidence/true_suite/obl005_fresh_mind2web_20260604T210300Z/`

The auditor used read-only `git`, `rg`, `jq`, `nl`, `find`, and `tar -tzf`
inspection. The auditor did not rerun cargo or constitution gates in the
read-only witness context.

## Findings

- Mind2Web reconciliation points to `obl005_fresh_mind2web_20260604T210300Z`.
  Current blockers remain `domain_receipt_final_closure_missing`,
  `benchmark_capability_not_solved`, and `fresh_final_closure_witness_missing`.
  The manifest still records `final_closure_claimed = false`.
- OBL-005 remains open. `OBLIGATIONS.md` is still `in_progress`, and
  `handover/ai-direct/LATEST.md` explicitly says this does not claim final
  closure.
- Evidence is ChainTape/CAS/replay/package based. The full-system receipt
  records source commit, replay indicators green, FC1/FC2/FC3 rows, market
  participation, and `FULL_SYSTEM_LIT` with `missing=[]`.
- Replay and restore replay reports are green. The package manifest contains
  runtime/CAS dotgit and worktree archives.
- No capability or closure overclaim was found. Mind2Web records
  `exact_match=false` / `browser_action_mismatch`, and domain FC trace remains
  `domain_adapter_smoke_only` with `final_closure_possible=false`.
- Raw prompt/provider response and secrets were not found in persisted evidence
  by read-only `rg` and tar-listing checks. Persisted files carry hashes and
  explicit `raw_provider_response_persisted=false`; config stores env var names
  and placeholders, not actual keys.

## Boundary

This verdict covers only the Mind2Web source-evidence claim and the referenced
paths above. It does not assert OBL-005 final closure and does not cover old
untracked failed/intermediate evidence directories.
