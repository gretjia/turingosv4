# OBL005 Fresh OSWorld Source Evidence Clean-Context Audit

Date: 2026-06-04
Reviewer: Claude Code headless, clean context, no session persistence
Risk class: Class 2
Verdict: NO-VIOLATION

## Scope

- OBLIGATIONS.md
- handover/ai-direct/LATEST.md
- tests/fixtures/liveness/true_suite_evidence_reconciliation.toml
- handover/evidence/true_suite/obl005_fresh_osworld_20260604T171857Z/

## Verdict JSON

```json
{
  "task_id": "OBL005_FRESH_OSWORLD_SOURCE_EVIDENCE",
  "workspace": "/home/zephryj/projects/turingosv4-main",
  "verdict": "NO-VIOLATION",
  "findings": [
    "Check 1 PASS: git diff main --name-only shows only OBLIGATIONS.md, handover/ai-direct/LATEST.md, and tests/fixtures/liveness/true_suite_evidence_reconciliation.toml. No src/ files. No AGENTS section 6 restricted surfaces appear in the diff.",
    "Check 2 PASS: OBL-005 remains in_progress. No final closure claimed. Fixture carries reconciliation_status=OBL005_REAUDIT_IN_PROGRESS and final_closure_claimed=false.",
    "Check 3 PASS: Fixture correctly removes source_receipt_final_closure_false and source_tree_fingerprint_missing from both OSWorld rows, while retaining domain_receipt_final_closure_false, benchmark_capability_not_solved, and fresh_final_closure_witness_missing.",
    "Check 4 PASS: Evidence directory contains the cited runner, full-system, domain, replay, restore, and package receipts. Replay/restore indicators are green, source_tree.commit is 1254212e10afe939b466d8404889106383d9bdb8, and benchmark result remains sandbox_action_mismatch.",
    "Check 5 PASS: ChainTape + CAS + replay verifier remain authoritative; no dashboard-only or stdout-only second source of truth detected.",
    "Check 6 PASS: No raw provider responses, API keys, or secrets found in the evidence directory.",
    "Additional PASS: Unrelated failed generate directory handover/evidence/true_suite/obl005_fresh_generate_20260604T160500Z/ is untracked and not part of this diff."
  ],
  "evidence_checked": [
    "git diff main --name-only",
    "git status --short",
    "handover/evidence/true_suite/obl005_fresh_osworld_20260604T171857Z/runner_execution_results.jsonl",
    "handover/evidence/true_suite/obl005_fresh_osworld_20260604T171857Z/osworld/full_system_participation.json",
    "handover/evidence/true_suite/obl005_fresh_osworld_20260604T171857Z/osworld/osworld_computer_use_manifest.json",
    "handover/evidence/true_suite/obl005_fresh_osworld_20260604T171857Z/osworld/replay_report.json",
    "handover/evidence/true_suite/obl005_fresh_osworld_20260604T171857Z/osworld/restore_replay_report.json",
    "handover/evidence/true_suite/obl005_fresh_osworld_20260604T171857Z/evidence_package_manifest.json",
    "tests/fixtures/liveness/true_suite_evidence_reconciliation.toml",
    "OBLIGATIONS.md",
    "handover/ai-direct/LATEST.md"
  ]
}
```
