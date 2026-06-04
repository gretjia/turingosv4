# OBL-005 Fresh ToolBench Source Evidence Clean-Context Audit

Date: 2026-06-04
Reviewer: Claude Code headless (`claude --print --output-format json --no-session-persistence --model sonnet --effort high`)
Risk class: Class 2 evidence/fixture reconciliation
Verdict: `NO-VIOLATION`

## Scope

Branch under audit: `codex/obl005-fresh-toolbench-source-evidence`

Evidence run:
`handover/evidence/true_suite/obl005_fresh_toolbench_20260604T194611Z/`

Primary checked paths:

- `tests/fixtures/liveness/true_suite_evidence_reconciliation.toml`
- `OBLIGATIONS.md`
- `handover/ai-direct/LATEST.md`
- `handover/evidence/true_suite/obl005_fresh_toolbench_20260604T194611Z/evidence_package_manifest.json`
- `handover/evidence/true_suite/obl005_fresh_toolbench_20260604T194611Z/runner_execution_results.jsonl`
- `handover/evidence/true_suite/obl005_fresh_toolbench_20260604T194611Z/toolbench/full_system_participation.json`
- `handover/evidence/true_suite/obl005_fresh_toolbench_20260604T194611Z/toolbench/toolbench_api_tool_use_manifest.json`
- `handover/evidence/true_suite/obl005_fresh_toolbench_20260604T194611Z/toolbench/toolbench_api_tool_use_run_manifest.json`

## Witness Result

```json
{
  "agent": "claude",
  "ok": true,
  "status": "complete",
  "verdict": "NO-VIOLATION",
  "findings": [],
  "checked_paths": [
    "tests/fixtures/liveness/true_suite_evidence_reconciliation.toml",
    "OBLIGATIONS.md",
    "handover/ai-direct/LATEST.md",
    "handover/evidence/true_suite/obl005_fresh_toolbench_20260604T194611Z/evidence_package_manifest.json",
    "handover/evidence/true_suite/obl005_fresh_toolbench_20260604T194611Z/runner_execution_results.jsonl",
    "handover/evidence/true_suite/obl005_fresh_toolbench_20260604T194611Z/toolbench/full_system_participation.json",
    "handover/evidence/true_suite/obl005_fresh_toolbench_20260604T194611Z/toolbench/toolbench_api_tool_use_manifest.json",
    "handover/evidence/true_suite/obl005_fresh_toolbench_20260604T194611Z/toolbench/toolbench_api_tool_use_run_manifest.json"
  ],
  "commands": [
    "git diff main -- tests/fixtures/liveness/true_suite_evidence_reconciliation.toml",
    "git diff main -- OBLIGATIONS.md",
    "git diff main -- handover/ai-direct/LATEST.md",
    "git diff main --name-only",
    "grep -r hf_|sk- ... evidence dir",
    "grep final_closure_claimed|rewrites_historical fixture"
  ],
  "summary": "Fixture change is narrowly scoped: only toolbench_api_tool_use broad_family row updated, evidence_run pointer advanced to obl005_fresh_toolbench_20260604T194611Z, exactly source_receipt_final_closure_false and source_tree_fingerprint_missing removed, three domain blockers retained as required. Evidence records honest failure (exact_match=false, tool_selection_mismatch) with FULL_SYSTEM_LIT and all FC1/FC2/FC3 indicators green. No raw secrets in evidence. final_closure_claimed=false and rewrites_historical_evidence=false in fixture header. No restricted Class 4 surfaces touched. No historical evidence rewritten. Constitutional invariants satisfied."
}
```

## Orchestrator Evidence Given to Witness

```text
scripts/run_true_suite_broad_agi_batch.sh --execute-installed \
  --run-id obl005_fresh_toolbench_20260604T194611Z \
  --runners toolbench_api_tool_use_fresh
# exit 0

cargo test -p turingosv4 \
  --test constitution_true_suite_evidence_reconciliation \
  --test constitution_obl005_final_closure_witness \
  --test constitution_realworld_liveness_coverage \
  --test constitution_matrix_drift -- --nocapture
# exit 0

cargo test -p turingosv4 \
  --test constitution_true_suite_toolbench_runner -- --nocapture
# exit 0

git diff --check
# exit 0

secret/raw-provider-payload scans over the final ToolBench evidence and edited
docs/fixture
# no disallowed hits

bash scripts/run_constitution_gates.sh
# exit 0; [k-1-5] total=165 failed=0

cargo test --workspace --no-fail-fast
# exit 0
```
