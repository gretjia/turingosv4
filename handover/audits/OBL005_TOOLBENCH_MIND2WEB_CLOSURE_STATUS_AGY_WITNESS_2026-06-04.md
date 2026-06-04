# OBL-005 ToolBench/Mind2Web Closure Status Witness

Date: 2026-06-04
Reviewer: AGY clean-context witness
Branch: `codex/obl005-domain-manifest-closure-status`
Risk class: Class 2

Scope: constitutional/evidence witness for the OBL-005 slice that converts
ToolBench and Mind2Web domain closure status from missing to explicit
non-closing (`final_closure_possible=false`) while preserving OBL-005 as
`in_progress`.

```json
{
  "verdict": "NO-VIOLATION",
  "findings": [
    "The diff contains no touches to Class 4 restricted surfaces; changes are scoped strictly to Class 2 evidence-accounting and runner configuration files.",
    "ToolBench and Mind2Web domain manifests in the fresh evidence directories explicitly define closure_scope='domain_adapter_smoke_only', full_system_participation_required=true, and final_closure_possible=false.",
    "Reconciliation maps for toolbench_api_tool_use and mind2web_open_web are correctly updated to point to the fresh runs (obl005_fresh_toolbench_20260604T224409Z and obl005_fresh_mind2web_20260604T224409Z), converting their blockers from domain_receipt_final_closure_missing to domain_receipt_final_closure_false.",
    "The obligations ledger (OBLIGATIONS.md) has been updated to reflect the new evidence runs while explicitly keeping OBL-005 in_progress without claiming final closure.",
    "No historical evidence directories have been modified or rewritten; the new evidence directories were added as fresh runs.",
    "No secrets, raw prompts, or raw provider responses are persisted in the fresh evidence files; prompt and response bodies are securely stored via prompt_sha256 and provider_response_sha256 fields.",
    "All compiler and constitution gates checks pass cleanly with [k-1-5] total=165 failed=0."
  ],
  "checked": [
    "git diff --stat origin/main",
    "git diff origin/main -- OBLIGATIONS.md src/bin/ tests/",
    "handover/evidence/true_suite/obl005_fresh_toolbench_20260604T224409Z/toolbench/toolbench_api_tool_use_manifest.json",
    "handover/evidence/true_suite/obl005_fresh_toolbench_20260604T224409Z/toolbench/full_system_participation.json",
    "handover/evidence/true_suite/obl005_fresh_mind2web_20260604T224409Z/mind2web/mind2web_browser_action_manifest.json",
    "handover/evidence/true_suite/obl005_fresh_mind2web_20260604T224409Z/mind2web/full_system_participation.json",
    "tests/fixtures/liveness/true_suite_evidence_reconciliation.toml",
    "OBLIGATIONS.md",
    "cargo test --test constitution_true_suite_toolbench_runner -- --nocapture",
    "cargo test --test constitution_true_suite_mind2web_runner -- --nocapture",
    "cargo test --test constitution_true_suite_evidence_reconciliation -- --nocapture",
    "cargo test --test constitution_matrix_drift -- --nocapture",
    "bash scripts/run_constitution_gates.sh"
  ]
}
```
