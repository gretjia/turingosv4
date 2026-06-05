# TC-ORCH-002 Worker Prompt

Status: ready
Owner lane: audit
Risk class: Class 0
FC nodes: orchestration only
Dependencies: TC-ORCH-000
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `handover/directives/tc_taskpackets_2026-06-04/templates/LOW_REASONING_WORKER_PROMPT.md`

Forbidden paths: all implementation files.

Task:

Maintain the low-reasoning worker prompt used for atom dispatch.

Required content:

- One packet only.
- Allowed write paths only.
- No `OBLIGATIONS.md` edits by workers.
- No restricted surfaces.
- Stop on Class-4 need.
- Lean is feature-layer only, not kernel.
- Write named failing test first.
- Return `STATUS`, `ATOM`, `CHANGED_FILES`, `TESTS_RUN`, `SHIP_GATE`, `NOTES`.

Ship gate:

```bash
grep -n "Lean is not the TuringOS kernel" handover/directives/tc_taskpackets_2026-06-04/templates/LOW_REASONING_WORKER_PROMPT.md
grep -n "Do not edit files outside" handover/directives/tc_taskpackets_2026-06-04/templates/LOW_REASONING_WORKER_PROMPT.md
```

Expected: both commands print one matching line.

Audit: Karpathy Simple Code Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND <clause> <file>:<line>`.
