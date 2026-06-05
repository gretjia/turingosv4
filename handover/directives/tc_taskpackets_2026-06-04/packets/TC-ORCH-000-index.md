# TC-ORCH-000 Orchestrator Index

Status: ready
Owner lane: audit
Risk class: Class 0
FC nodes: FC1/FC2/FC3 planning only
Dependencies: none
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `handover/directives/tc_taskpackets_2026-06-04/INDEX.md`
- `handover/directives/tc_taskpackets_2026-06-04/packets/*.md`
- `handover/directives/tc_taskpackets_2026-06-04/templates/*.md`

Forbidden paths: all `src/`, `tests/`, `scripts/`, restricted surfaces.

Task:

Create and maintain the packet directory, status ledger, dependency table,
verdict domains, universal forbidden path list, and final completion definition.

Worker steps:

1. Read `AGENTS.md`, OBL-014, and the full TC operationalization plan.
2. Confirm `INDEX.md` lists every atom from TC-Q000 through TC-021B.
3. Confirm every index row has packet path, lane, status, dependency, and audit role.
4. Do not change implementation files.

Ship gate:

```bash
test -f handover/directives/tc_taskpackets_2026-06-04/INDEX.md
grep -RInE 'T[B]D|TO[D]O|Simila[r] to|implement late[r]' handover/directives/tc_taskpackets_2026-06-04
```

Expected: first command exits 0; second command has no output.

Audit: Karpathy Architect Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND <clause> <file>:<line>`.
