# TC-000 Path B Decision

Status: ready
Owner lane: substrate
Risk class: Class 0
FC nodes: FC1 `Q_t`, FC2 boot, FC3 replay
Dependencies: TC-Q001
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `handover/directives/TC_000_PATH_B_DECISION.md`
- `tests/tc_operationalization_docs.rs`

Forbidden paths: source implementation and restricted surfaces.

Task:

Keep the Path B decision explicit and scoped.

Required facts:

- `Decision: Path B`
- Path A rejected.
- Path C rejected.
- Ref topology includes `refs/chaintape/l4`, `refs/chaintape/l4e`,
  `refs/chaintape/cas`, `refs/tdma/verified_head`, `refs/tdma/ledger_tail`.
- No strong claim words.

Ship gate:

```bash
cargo test --test tc_operationalization_docs --no-fail-fast
grep -n "Decision: Path B" handover/directives/TC_000_PATH_B_DECISION.md
```

Expected: tests pass and grep prints one matching line.

Audit: Constitution Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND <clause> <file>:<line>`.
