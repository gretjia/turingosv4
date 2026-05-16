# REAL-BCAST-1 Class-4 Ratification

Date: 2026-05-16

## User Authorization

The user authorized the Class-4 change after the clean-context audit VETO:

```text
我授权你可以进行改动。同时，之前发现真题测试的题太简单了，
导致不足以测试出现有架构的完整性。一个是增加真题测试的量，
还有一个是适当增加难度。包括扩展你的 time out 时间和 max TX
```

## Ratified Scope

This ratifies the REAL-BCAST-1 Trust Root rehash needed for evaluator-level
Librarian prompt injection.

Ratified:

```text
- Trust Root rehash for modified pinned files:
  - experiments/minif2f_v4/src/bin/evaluator.rs
  - src/bin/audit_dashboard.rs
  - src/runtime/mod.rs
- Trust Root inclusion of the new load-bearing broadcast module:
  - src/runtime/librarian_broadcast.rs
- REAL-BCAST A/B rerun after Trust Root verify passes.
- Expanded real-problem testing with larger task count, higher difficulty,
  longer timeout, and higher max_tx.
```

Not ratified:

```text
- new TypedTx;
- sequencer admission change;
- canonical signing payload change;
- CAS ObjectType schema edit;
- live REAL-6B;
- full async ship path;
- forced trade;
- price-as-truth;
- raw prompt/completion/CoT/Lean stderr broadcast;
- E2/E3/E4 overclaim.
```

## Required Gates

```text
1. Trust Root verify passes.
2. REAL-BCAST targeted tests pass.
3. Constitution gates pass.
4. REAL-BCAST A/B evidence is regenerated after rehash.
5. Expanded real-problem evidence is marked separately from small smoke.
6. Clean-context audit reviews post-rehash evidence before ship.
```
