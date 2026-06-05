# TC-003c Reopen Sequence Identity

Status: ready
Owner lane: substrate
Risk class: Class 2
FC nodes: FC1 tape, FC3 replay
Dependencies: TC-003b
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/git_tape_ledger.rs`
- `tests/tc_git_tape_ledger_hardening.rs`

Forbidden paths: restricted surfaces.

Task:

Ensure reopening a TDMA git tape derives the next sequence from tape, not RAM.

Test first:

Maintain or extend:
`reopen_append_continues_monotonic_tape_ids`.

Required cases:

- append `tn-1`, close, reopen, append `tn-2`.
- mixed scopes do not reset global sequence.
- malformed commit produces explicit error, not silent sequence skip.

Ship gate:

```bash
cargo test --test tc_git_tape_ledger_hardening --no-fail-fast
```

Expected: command exits 0.

Audit: Reliability Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE <artifact>`.
