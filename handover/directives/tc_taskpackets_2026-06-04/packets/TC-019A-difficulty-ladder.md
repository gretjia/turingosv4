# TC-019A Difficulty Ladder Manifest

Status: ready
Owner lane: audit
Risk class: Class 1 manifest, Class 2 if runner integration changes
FC nodes: FC1 workload corpus, FC3 audit evidence
Dependencies: TC-011C, TC-014B
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `handover/directives/tc_ladder_2026-06-04/L0_L5_MANIFEST.yaml`
- `tests/tc_difficulty_ladder.rs`

Forbidden paths: benchmark result rewriting, hidden theorem leakage.

Task:

Freeze L0-L5 task bank and gate L5 entry on L0-L4 receipts.

Tests first:

- `ladder_manifest_has_l0_through_l5`
- `l5_entry_requires_l0_l4_green_receipts`
- `hard_claim_c_cannot_start_without_ladder_gate`

Ship gate:

```bash
cargo test --test tc_difficulty_ladder --no-fail-fast
```

Expected: command exits 0.

Audit: Formal-methods Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND ladder-gate <file>:<line>`.
