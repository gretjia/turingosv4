# TC-ORCH-001 Gate Policy

Status: ready
Owner lane: audit
Risk class: Class 0
FC nodes: FC1 predicates, FC2 boot checks, FC3 audit feedback
Dependencies: TC-ORCH-000
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `handover/directives/tc_taskpackets_2026-06-04/INDEX.md`
- `handover/directives/tc_taskpackets_2026-06-04/packets/TC-ORCH-001-gate-policy.md`

Forbidden paths: all `src/`, `tests/`, `scripts/`, restricted surfaces.

Task:

Lock the mechanical gates that every worker and reviewer must use. The gate
policy is documentation only; it must not weaken existing scripts.

Required policy:

- Every code atom runs `git diff --check`.
- Every code atom runs its targeted test.
- Every wave runs `cargo test --test constitution_matrix_drift --no-fail-fast`.
- Every wave runs `bash scripts/run_constitution_gates.sh`.
- Any new `tests/tc_*.rs` must be explicitly listed in the atom ship gate or
  renamed/registered as `constitution_tc_*`.
- Restricted-surface grep printing any path is a stop event.
- Strong-claim grep printing any load-bearing claim is a stop event.
- Secret/verifier-output grep printing any real secret or unshielded verifier body is a
  stop event.

Ship gate:

```bash
grep -n "Restricted-surface grep printing any path is a stop event" handover/directives/tc_taskpackets_2026-06-04/packets/TC-ORCH-001-gate-policy.md
grep -n "Any new .*tc_.*rs" handover/directives/tc_taskpackets_2026-06-04/packets/TC-ORCH-001-gate-policy.md
```

Expected: both commands print one matching line.

Audit: Constitution Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND <clause> <file>:<line>`.
