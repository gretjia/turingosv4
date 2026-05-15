# REAL-6B Trust Root Scope Note

Date: 2026-05-15

Harness run:

- `dev_1778822123466_1203906`

Related audit:

- `handover/audits/CODEX_REAL6B_IMPLEMENTATION_REVIEW_R1.md`

## Why This Note Exists

The R1 clean-context Codex review returned `CHALLENGE` with a P1 finding:

```text
Trust Root rehash is not narrow in the recorded REAL-6B diff.
```

The finding is accepted. Because the branch contains earlier uncommitted
REAL-5 / REAL-6A work, a `git diff HEAD -- genesis_payload.toml` artifact shows
all Trust Root changes since `HEAD`, including prior reviewed worktree
normalization. That branch-level diff is not a narrow semantic proof for
REAL-6B.

## REAL-6B Semantic Trust Root Change

The REAL-6B semantic Trust Root change is limited to:

```text
src/runtime/mod.rs
```

Reason:

```text
exported pure runtime helper module `real6_attempt_prediction`
```

Current hash:

```text
2a6ade1437683f1c45de94b01d52e97c3c347da6752f244397e8d05350f0605a
```

This helper is:

```text
design + scripted fixture only;
no live real-LLM ship;
no sequencer admission change;
no TypedTx schema/discriminant change;
no canonical signing payload change;
no wallet/kernel/bus change.
```

## Prior Broader Rehashes

Broader Trust Root normalization from the dirty shared branch is documented
under REAL-6A:

```text
handover/evidence/real6_task_outcome/TRUST_ROOT_REHASH_REAL6A_WORKSPACE_NORMALIZATION.md
handover/evidence/real6_task_outcome/REAL6A_RESTRICTED_SURFACE_AUDIT_NOTE.md
```

Those files must not be interpreted as REAL-6B semantic changes.

## Verification

REAL-6B Trust Root evidence:

```text
command_0004: Trust Root RED after adding src/runtime/mod.rs export.
command_0009: Trust Root PASS after rehashing src/runtime/mod.rs.
```

Post-R1 remediation additionally updates the `genesis_payload.toml` comment for
`src/runtime/mod.rs` so the recorded rationale names REAL-6B rather than only
the predecessor REAL-5 module export.
