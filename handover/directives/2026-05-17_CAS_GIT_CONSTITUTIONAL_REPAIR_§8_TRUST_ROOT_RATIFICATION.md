# CAS Git Constitutional Repair §8 Trust Root Ratification

Date: 2026-05-17

## User Authorization

The user explicitly authorized the Class 4 Trust Root rehash for this repair
branch:

```text
我授权对 codex/cas-git-constitutional-repair 执行 Class 4 Trust Root rehash，仅限 CAS Git repair 已改动的 pinned files，用于完成 final 真题测试；不得合并 main。
```

## Ratified Scope

This ratifies only the Trust Root rehash required to complete CAS Git repair
evidence on branch `codex/cas-git-constitutional-repair`.

Allowed exact files:

```text
- Cargo.lock
- Cargo.toml
- src/runtime/evidence_capsule.rs
- src/bottom_white/cas/mod.rs
- src/bottom_white/cas/store.rs
- src/bottom_white/cas/git_chain.rs
- src/bottom_white/ledger/transition_ledger.rs
- src/state/sequencer.rs
- genesis_payload.toml trust-root hash update
```

## Forbidden Scope

Not ratified:

```text
- src/state/typed_tx.rs
- src/bottom_white/cas/schema.rs
- src/kernel.rs
- src/bus.rs
- src/sdk/tools/wallet.rs
- canonical signing payload changes
- sequencer admission policy changes
- constitution or flowchart text changes
- merge to main before user review
- historical evidence rewrite
```

## Required Gates

```text
1. Targeted CAS Git repair tests pass.
2. Trust Root verification passes after rehash.
3. Constitution gates are rerun after hydration/remediation.
4. CAS repair mini and TB-18R R9 real-problem evidence are rerun or explicitly
   refreshed with postprocess evidence.
5. Final report states baseline vs final deltas, residual risks, and non-claims.
6. Clean-context Codex audit returns PROCEED before branch submission.
```
