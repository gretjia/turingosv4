# TC-001 Veto-AI Scope Lock

Status: ready
Owner lane: audit
Risk class: Class 0
FC nodes: FC3 Veto-AI
Dependencies: TC-000
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `handover/directives/TC_001_VETO_AI_SCOPE_LOCK.md`
- `tests/tc_operationalization_docs.rs`

Forbidden paths: source implementation and restricted surfaces.

Task:

Keep Veto-AI scoped to constitutional PASS/VETO only.

Required facts:

- Veto-AI output domain is `{PASS,VETO}`.
- Veto-AI does not review style, performance, coverage, or architecture taste.
- Other audit roles are separate from Veto-AI.

Ship gate:

```bash
cargo test --test tc_operationalization_docs --no-fail-fast
grep -n 'Veto-AI output domain: `{PASS,VETO}`' handover/directives/TC_001_VETO_AI_SCOPE_LOCK.md
```

Expected: tests pass and grep prints one matching line.

Audit: Constitution Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND <clause> <file>:<line>`.
