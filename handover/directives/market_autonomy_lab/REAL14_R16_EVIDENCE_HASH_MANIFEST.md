# REAL-14 R16 Evidence Hash Manifest

claim_boundary: `E2 candidate pending audit`

R16 evidence is frozen for REAL-14 audit. Do not retroactively rewrite
ChainTape/CAS/evaluator evidence; add annotations or new verifier outputs
outside the evidence dir.

Evidence dir:

```text
handover/evidence/market_autonomy_lab_hard10_bcast_exact_join_R16_20260516T202248Z
```

Status:

```text
active_label: E2 candidate pending audit
forbidden_overclaim_scan_status: PASS
```

Superseded / non-canonical comparison evidence:

```text
handover/evidence/market_autonomy_lab_hard10_action_handoff_R14_20260516T191707Z
status: superseded_by_R15_R16
reason: R14 clean-context audit CHALLENGE; duplicate/exact-join/report-wording
        issues closed by R15/R16.
```

Machine-readable manifest:

```text
handover/directives/market_autonomy_lab/REAL14_R16_EVIDENCE_HASH_MANIFEST.json
```

Required-file coverage:

```text
run_log.txt: present, hashed
aggregate_verdict.json: present, hashed
audit_dashboard_run_report.txt: present, hashed
cas/.turingos_cas_index.jsonl: present, hashed
runtime_repo/*.json/jsonl: present, hashed
per-problem evaluator stdout/stderr: present, hashed for P000..P009
```

Independent verifier output added outside the immutable evidence dir:

```text
handover/directives/market_autonomy_lab/REAL14_R16_VERIFIER_REPORT.json
handover/directives/market_autonomy_lab/REAL14_R16_VERIFIER_REPORT.md
```
