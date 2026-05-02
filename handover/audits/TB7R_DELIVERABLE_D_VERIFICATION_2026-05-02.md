# TB-7R Deliverable D — On-chain TaskOpenTx + EscrowLockTx Verification

**Date**: 2026-05-02
**Auditor**: claude (TB-7R Deliverable D)
**Verdict**: **NO CODE CHANGE REQUIRED** — current bootstrap already emits TaskOpenTx + EscrowLockTx as on-chain transitions, never via direct memory insert.

---

## Verdict criteria recap

Per architect verdict 2026-05-01 §6.2 + Deliverable D:

> "如果这次 run 需要 task / escrow，不要只在内存里塞:
>   `q.economic_state_t.task_markets_t.insert(...)`
>   `q.economic_state_t.escrows_t.insert(...)`
> 而应该生成 accepted:
>   `TaskOpenTx`
>   `EscrowLockTx`"

The verdict's binding rule: the run's task and escrow must enter `Q_t`
**only via predicate-passed L4 transitions** — never via direct
`task_markets_t.insert` / `escrows_t.insert` memory writes.

---

## Verification (1) — no direct memory inserts exist

Repo-wide grep for the forbidden patterns:

```text
$ grep -rn "task_markets_t.insert\|escrows_t.insert" src/ experiments/minif2f_v4/
(zero matches)
```

Neither `task_markets_t` nor `escrows_t` is ever mutated by direct
`insert` outside the Sequencer's transition apply path. The only way
those maps grow is when an accepted `TaskOpenTx` / `EscrowLockTx`
threads through `Sequencer::apply_one`.

---

## Verification (2) — preseed bootstrap emits real transactions

`experiments/minif2f_v4/src/bin/evaluator.rs:843-898` (TB-7.7 D3 site)
shows the preseed pattern:

```rust
// Line 843-848: build a TypedTx::TaskOpen via the adapter
let task_open_real = turingosv4::runtime::adapter::make_synthetic_task_open(
    &real_task_id,
    "tb7-7-sponsor",
    turingosv4::state::q_state::Hash::ZERO,
    "tb7-7-d3-seed",
);

// Line 849: submit through the authoritative path — NOT a memory insert
if let Err(e) = bus.submit_typed_tx(task_open_real).await {
    error!("[chaintape/d3] preseed TaskOpen submit failed: {e}");
}

// Line 854-880: poll q_snapshot until state_root_t advances
//   (proves the TaskOpen reached L4 accepted, not L4.E)

// Line 887-893: build TypedTx::EscrowLock with the post-TaskOpen state_root
let escrow_lock = turingosv4::runtime::adapter::make_synthetic_escrow_lock(
    &real_task_id,
    "tb7-7-sponsor",
    escrow_micro,
    parent_for_escrow,
    "tb7-7-d3-escrow",
);

// Line 894: submit through the authoritative path
if let Err(e) = bus.submit_typed_tx(escrow_lock).await {
    error!("[chaintape/d3] preseed EscrowLock submit failed: {e}");
}
```

Both transactions go through `bus.submit_typed_tx` →
`Sequencer::apply_one` → `transition_ledger.append` (L4 accepted) or
`rejection_writer.append_rejected` (L4.E). The Sequencer is the only
mutator of `task_markets_t` / `escrows_t`.

---

## Verification (3) — agent-balance preseed is allowed (different category)

`experiments/minif2f_v4/src/bin/evaluator.rs:701-714` seeds agent
balances via `genesis_with_balances`:

```rust
let mut pairs: Vec<(AgentId, MicroCoin)> = vec![
    (AgentId("tb7-7-sponsor".into()),
     MicroCoin::from_micro_units(10_000_000)),
];
for i in 0..10 {
    pairs.push((
        AgentId(format!("Agent_{i}")),
        MicroCoin::from_micro_units(1_000_000),
    ));
}
let initial_q = turingosv4::runtime::adapter::genesis_with_balances(&pairs);
```

This is **NOT** a `task_markets_t` or `escrows_t` write — it sets the
**genesis economic state** before the Sequencer is constructed. Per
the Constitution `on_init` is the only legitimate mint event; this is
the Constitutional `on_init` analog for the run. The verdict §6.2
forbids only post-genesis memory inserts of task / escrow records, NOT
genesis-time balance allocation.

The TB-7R Deliverable C `genesis_report.json` records these balances
under the `initial_balances` field so post-hoc audits can verify
the bootstrap's economic priors.

---

## Verification (4) — D7 evidence shows the on-chain pattern in practice

`handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/dashboard.txt`
§5 Proposal flow:

```
side  | t   | tx_kind         | agent              | reject
------+-----+-----------------+--------------------+--------
L4    |   1 | TaskOpen        | tb7-7-sponsor      | -
L4    |   2 | EscrowLock      | tb7-7-sponsor      | -
L4    |   3 | Work            | Agent_0            | -
```

Both `TaskOpen` and `EscrowLock` appear at distinct `logical_t` slots
in the L4 accepted transition ledger — proving they were applied as
predicate-passed transitions, not memory shortcuts. Replay verifies
the ledger root chain (per `replay_report.json`
`ledger_root_verified: true`).

---

## Aggregate verdict

```text
Verdict §6.2 / Deliverable D criteria:        SATISFIED
Code-search for direct task_markets_t insert: NONE FOUND
Code-search for direct escrows_t insert:      NONE FOUND
Existing on-chain TaskOpenTx pattern:         CONFIRMED at evaluator.rs:843-849
Existing on-chain EscrowLockTx pattern:       CONFIRMED at evaluator.rs:887-894
Live evidence of the pattern:                 CONFIRMED at D7 dashboard §5
Agent-balance preseed (genesis on_init):      LEGITIMATE — not a §6.2 violation

Net code change required for Deliverable D:   ZERO
```

Deliverable D ship-gate is met without code changes. The original
verdict §B1 caveat — "redo D3 memory-only pre-seed for new runs" —
applies to a hypothetical scenario where D3 had only used memory
inserts. The actual D3 commit (`054254f` + the e9cb023-merged D7 patches)
already routes both TaskOpen and EscrowLock through the authoritative
`submit_typed_tx` path. New TB-7R runs inherit this correct behavior
unchanged.

## Cross-references

- TB-7R charter: `handover/tracer_bullets/TB-7R_charter_2026-05-01.md` §3 Deliverable D
- TB-7R authorization: `handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md` B1 + §6.2
- L4 / L4.E ledger separation: `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`
- D7 evidence dashboard (live witness): `handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/dashboard.txt`
- Genesis report module: `src/runtime/genesis_report.rs` (TB-7R Deliverable C)
- Adapter constructors: `src/runtime/adapter.rs:51-87`
