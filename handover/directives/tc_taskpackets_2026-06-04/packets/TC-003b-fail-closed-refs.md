# TC-003b Fail-Closed TDMA Ref Movement

Status: ready
Owner lane: substrate
Risk class: Class 2
FC nodes: FC1 `wtool -> Q_{t+1}`, FC3 replay
Dependencies: TC-003a
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/git_tape_ledger.rs`
- `tests/tc_git_tape_ledger_hardening.rs`

Forbidden paths: restricted surfaces.

Task:

Make new authoritative TDMA ref movements fail closed. Do not swallow ref
update errors.

Test first:

Add or maintain test:
`tdma_authority_ref_update_returns_error_or_detected_recovery`.

Failure fixtures:

- invalid OID
- broken bare repo path
- ref name collision or malformed symbolic ref

Implementation rules:

- No `let _ = repo.reference(...)` on authority refs.
- No best-effort authority updates.
- Error returns must include enough context to diagnose the ref.

Ship gate:

```bash
cargo test --test tc_git_tape_ledger_hardening --no-fail-fast
git -C <test-created-bare-repo> fsck --full
```

Expected: cargo test exits 0; fsck exits 0 for valid fixture repos.

Audit: Reliability Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE <artifact>`.
