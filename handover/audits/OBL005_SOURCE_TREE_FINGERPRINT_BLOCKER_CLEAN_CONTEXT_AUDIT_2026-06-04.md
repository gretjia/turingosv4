# OBL-005 Source-Tree Fingerprint Blocker Clean-Context Audit

Date: 2026-06-04

Workspace: `/home/zephryj/projects/turingosv4-main`

Task id: `OBL005_SOURCE_TREE_FINGERPRINT_BLOCKER`

Risk: Class 2, evidence reconciliation / closure accounting only.

Touched FC nodes: FC1 / FC2 / FC3 evidence reconciliation. No restricted
surface under `AGENTS.md` section 6 was touched.

Auditor: Claude headless, clean context, `--no-session-persistence`,
`--permission-mode plan`.

Verdict:

```json
{
  "verdict": "NO-VIOLATION",
  "findings": [],
  "summary": "Source-tree fingerprint blocker is bidirectionally re-derived from immutable full_system_participation.json content, so the TOML blocker list cannot become a second source of truth. The detector's pointer set excludes /replay/head_commit_oid_hex and a dedicated test proves runtime ChainTape HEAD is not counted as source proof. The final-closure guard blocks any future final_closure_claimed=true over closing receipts lacking a source-tree fingerprint. Manifest stays final_closure_claimed=false / OBL005_REAUDIT_IN_PROGRESS and OBL-005 remains in_progress with no closure claim. RED test missing_source_tree_fingerprint_must_be_explicit_blocker is fail-capable. Change is fail-closed hardening, touches no historical evidence and no section-6 restricted surface; blocker count 21 matches 10 coverage_task + 11 broad_family bindings.",
  "files_reviewed": [
    "tests/constitution_true_suite_evidence_reconciliation.rs",
    "tests/fixtures/liveness/true_suite_evidence_reconciliation.toml",
    "OBLIGATIONS.md"
  ]
}
```

Command:

```bash
printf '%s\n' "$prompt" | timeout 900 claude --print --output-format json --no-session-persistence --permission-mode plan
```

Relevant deterministic evidence supplied to the auditor:

```text
RED: missing_source_tree_fingerprint_must_be_explicit_blocker failed before implementation.
GREEN: cargo test --test constitution_true_suite_evidence_reconciliation -- --nocapture => 10/10
cargo test --test constitution_matrix_drift -- --nocapture => 3/3
cargo test --test constitution_realworld_liveness_coverage --test constitution_broad_agi_true_suite_manifest --test constitution_production_module_liveness -- --nocapture => 27/27
rustfmt --edition 2021 --check tests/constitution_true_suite_evidence_reconciliation.rs && git diff --check => pass
bash scripts/run_constitution_gates.sh => [k-1-5] total=164 failed=0
cargo test --workspace --no-fail-fast => exit 0
```
