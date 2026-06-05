# TC-ORCH-003 Reviewer Prompts

Status: ready
Owner lane: audit
Risk class: Class 0
FC nodes: FC3 audit feedback
Dependencies: TC-ORCH-000
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `handover/directives/tc_taskpackets_2026-06-04/templates/SPEC_REVIEWER_PROMPT.md`
- `handover/directives/tc_taskpackets_2026-06-04/templates/CONSTITUTION_AUDITOR_PROMPT.md`
- `handover/directives/tc_taskpackets_2026-06-04/templates/OBLIGATION_WITNESS_PROMPT.md`

Forbidden paths: all implementation files.

Task:

Maintain review prompts and verdict domains.

Required reviewer boundaries:

- Spec reviewer checks packet compliance only.
- Constitution auditor checks constitutional clauses and reconstruction only.
- Obligation witness checks `OBLIGATIONS.md` only.
- No reviewer emits style, performance, coverage preference, or architecture
  taste as a blocking verdict.

Ship gate:

```bash
grep -RIn "Do not review style" handover/directives/tc_taskpackets_2026-06-04/templates
grep -RIn "OBL-ALL-CLOSED" handover/directives/tc_taskpackets_2026-06-04/templates
```

Expected: both commands print matching lines.

Audit: Constitution Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND <clause> <file>:<line>`.
