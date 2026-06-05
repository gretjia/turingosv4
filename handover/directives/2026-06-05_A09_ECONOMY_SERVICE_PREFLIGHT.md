# A09 Economy Service Preflight

Date: 2026-06-05

Parent plan:
`handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`

Atom: A09. Economy Service v0

Document role: Class 0 preflight. This document does not authorize money,
wallet, sequencer admission, typed transaction, CAS schema, or settlement
authority changes by itself.

## Decision

A09 is architecturally required, but it is not ready for production
implementation from the current repo state. The repo has older production
economic state and useful economy witnesses, but it does not yet have the
planned A09 `EconomyEvent` tape-payload projection surface.

A09 must wait for:

- A05 TapeEvent/projection authority over ChainTape/L4
- A08 PredicateReceipt shape before settlement can depend on predicate pass
- A04 ChainTape physical authority decisions for tape-head identity

Safe work now:

- docs-only preflight
- allowed-path correction
- economy witness inventory
- atom split and acceptance-command correction

Blocked until predecessors exist:

- claiming wallet/market/price/settlement replay from A05 tape events
- using `EconomicState` snapshots as the A09 projection source of truth
- using `src/economy/ledger.rs` or `EscrowVault` as a second economy ledger
- changing wallet authority, sequencer admission, typed tx schema, or CAS schema
- paying out without a future A08 `PredicateReceipt::Pass`

## Current-State Facts

Parent-plan A09 allowed paths as originally written:

```text
src/economy/events.rs
src/economy/projections.rs
src/economy/conservation.rs
src/economy/settlement.rs
src/economy/price_broadcast.rs
tests/economy_tape_replay.rs
tests/economy_conservation.rs
tests/economy_predicate_price_blindness.rs
tests/economy_no_parallel_ledger.rs
```

Corrected implementation path inventory:

```text
src/economy/mod.rs
src/economy/events.rs
src/economy/projections.rs
src/economy/conservation.rs
src/economy/settlement.rs
src/economy/price_broadcast.rs
src/economy/money.rs
src/economy/monetary_invariant.rs
src/state/q_state.rs
src/state/price_index.rs
src/sdk/tools/wallet.rs
tests/economy_tape_replay.rs
tests/economy_conservation.rs
tests/economy_predicate_price_blindness.rs
tests/economy_no_parallel_ledger.rs
tests/constitution_economy_gate.rs
tests/tb_14_price_index.rs
tests/constitution_router_price_quote.rs
```

Write-scope guidance:

```text
src/economy/mod.rs
  needed only to export new A09 modules
src/economy/money.rs
  read/reuse MicroCoin; do not redesign money type inside A09
src/economy/monetary_invariant.rs
  Class 3 if edited; prefer reuse first
src/state/q_state.rs
  read-only source inventory; do not make QState snapshots the projection truth
src/state/price_index.rs
  read-only derived-view precedent unless a narrow adapter is necessary
src/sdk/tools/wallet.rs
  read-only wallet witness; do not add owned wallet state
src/state/sequencer.rs
src/state/typed_tx.rs
src/bottom_white/cas/schema.rs
  out of A09 write scope without explicit higher-risk ratification
```

Existence check:

```text
MISSING src/economy/events.rs
MISSING src/economy/projections.rs
MISSING src/economy/conservation.rs
MISSING src/economy/settlement.rs
MISSING src/economy/price_broadcast.rs
MISSING tests/economy_tape_replay.rs
MISSING tests/economy_conservation.rs
MISSING tests/economy_predicate_price_blindness.rs
MISSING tests/economy_no_parallel_ledger.rs
EXISTS src/economy/mod.rs
EXISTS src/economy/money.rs
EXISTS src/economy/ledger.rs
EXISTS src/economy/escrow_vault.rs
EXISTS src/economy/monetary_invariant.rs
EXISTS src/state/q_state.rs
EXISTS src/state/price_index.rs
EXISTS src/sdk/tools/wallet.rs
EXISTS tests/constitution_economy_gate.rs
EXISTS tests/tb_14_price_index.rs
EXISTS tests/constitution_router_price_quote.rs
```

Dirty-path check for relevant inventory:

```text
pre-existing dirty paths include:
  handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md
  src/economy/monetary_invariant.rs
  src/runtime/mod.rs
  src/state/sequencer.rs
  src/state/typed_tx.rs

Implementation must read and preserve those edits. Do not overwrite them as
part of A09 scaffolding.
```

Existing economy witnesses:

```text
src/economy/mod.rs:8
  economy module root currently exports escrow_vault, ledger,
  monetary_invariant, and money only
src/economy/money.rs:27
  MicroCoin is the integer money type
src/economy/money.rs:72
  arithmetic is checked, not float-based
src/sdk/tools/wallet.rs:1
  WalletTool is a read-only projection over EconomicState.balances_t
src/sdk/tools/wallet.rs:28
  WalletTool holds zero owned ledger state
src/sdk/tools/wallet.rs:42
  balance reads route to EconomicState.balances_t
src/state/q_state.rs:167
  EconomicState is current chain-resident production economic state
src/state/q_state.rs:181
  price_index_t was removed from canonical EconomicState
src/state/q_state.rs:192
  node_positions_t is canonical exposure state, not Coin holding
src/state/q_state.rs:472
  task_markets_t.total_escrow is a derived cached index, not a holding
src/state/q_state.rs:901
  legacy PriceIndex canonical field removed; compute_price_index is derived
src/state/price_index.rs:1
  PriceIndex is a derived view; price is signal, not truth
src/state/price_index.rs:146
  compute_price_index is pure over EconomicState
src/economy/monetary_invariant.rs:53
  DerivedCacheMismatch already treats drifted derived aggregates as bugs
src/economy/monetary_invariant.rs:162
  total_supply_micro is the production holding-list source of truth
src/economy/monetary_invariant.rs:307
  task market total_escrow cache must equal escrow-lock sum
tests/constitution_economy_gate.rs:1
  economy gate covers read-is-free, no post-init mint, wallet read-only,
  no f64 money path, and conservation surfaces
tests/tb_14_price_index.rs:1
  price-index witnesses exist for the older derived view
tests/constitution_router_price_quote.rs:172
  price signal must not decide predicate truth
```

Older surfaces that must not become A09 authority:

```text
src/economy/escrow_vault.rs:14
  in-memory vault; no I/O and no L4 emission
src/economy/ledger.rs:28
  old accepted-only wrapper explicitly defers real Git2LedgerWriter backend
src/economy/ledger.rs:165
  AcceptedLedger is an in-memory Vec wrapper, not the OS L2 writer
```

## Risk Classification

Risk floor: Class 3, because A09 concerns money, market/economic state,
escrow, settlement, and wallet/price derivation.

No new Section 8 appears required if A09 stays within new L6 projection
modules/tests, uses A05/A08 outputs as inputs, and does not touch restricted
authority.

Promote to Class 4 if:

- `src/state/sequencer.rs` changes
- `src/state/typed_tx.rs` changes
- sequencer admission changes
- typed tx wire schema or discriminants change
- canonical signing payload changes
- wallet authority changes
- CAS `ObjectType` or schema authority changes
- trust-root / constitution / flowchart authority changes

## Recommended Contract

A09 should expose economy projection data, not a second ledger:

```text
EconomyEvent {
  event_id: EventId,
  tape_head_oid: GitOid,
  logical_t: u64,
  tx_id: TxId,
  event_kind: EconomyEventKind,
  payload_cid: Cid,
  predicate_receipt_cid: Option<Cid>,
}

EconomyProjection {
  projection_id: "economy.v0",
  projection_version: u32,
  derived_from_tape_head: GitOid,
  last_applied_logical_t: u64,
  wallet_balances: BTreeMap<AgentId, MicroCoin>,
  escrows: BTreeMap<TxId, EscrowProjection>,
  market_books: BTreeMap<TaskId, MarketBookProjection>,
  price_index: BTreeMap<TxId, NodeMarketEntry>,
  settlements: BTreeMap<TxId, SettlementProjection>,
  conservation_root: Hash,
}
```

Required invariant:

```text
projection = derive_from_tape(prefix)
wallet/market/price/settlement are projection fields, not write authority
price may inform routing but cannot change predicate verdict
settlement requires PredicateReceipt::Pass
money math is MicroCoin/integer-rational only
dropping the projection and replaying tape yields byte-equivalent state
```

Use `ObjectType::Generic + schema_id` for new economy receipts until a
dedicated CAS enum variant receives explicit schema-risk ratification.

## Atomized A09 Tasks

### A09.0 Preflight Lock

Description:
Record missing files, corrected paths, existing economy witnesses, predecessor
dependencies, risk boundaries, and acceptance commands.

Acceptance:

```bash
for f in \
  handover/directives/2026-06-05_A09_ECONOMY_SERVICE_PREFLIGHT.md \
  handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md
do
  git diff --no-index --check /dev/null "$f" || true
done
```

Expected:

```text
no whitespace errors
```

### A09.1 Economy Event And Projection Shape

Description:
After A05 and A08 exist, add the economy event envelope and a replay-only
projection shell. This atom must not edit sequencer admission or typed tx schema.

Primary paths:

```text
src/economy/mod.rs
src/economy/events.rs
src/economy/projections.rs
tests/economy_tape_replay.rs
```

Guidance:

```text
Input = A05 TapeEvent prefix + CAS payloads + optional A08 PredicateReceipt CIDs.
Output = EconomyProjection derived from a specific tape head.
Do not read dashboards, reports, stdout, or old latest pointers.
Do not persist EconomyProjection as canonical state.
```

Acceptance:

```bash
cargo test --test economy_tape_replay --no-fail-fast -- --test-threads=1
git diff --check
```

Expected:

```text
replay reconstructs exact wallet/market/price/settlement projection from tape.
```

### A09.2 Conservation Projection

Description:
Derive conservation checks from the same A09 projection without duplicating the
production holding-list truth.

Primary paths:

```text
src/economy/conservation.rs
tests/economy_conservation.rs
tests/constitution_economy_gate.rs
```

Guidance:

```text
Reuse MicroCoin.
Reuse or call the production conservation reducer where possible.
Do not create a second independent holding-list table.
Do not count task_markets_t.total_escrow as money.
```

Acceptance:

```bash
cargo test --test economy_conservation --no-fail-fast -- --test-threads=1
cargo test --test constitution_economy_gate --no-fail-fast -- --test-threads=1
grep -RInE 'f32|f64' src/economy tests/economy_* && exit 1 || true
```

Expected:

```text
sum(wallet_balances) + escrow + open_positions == minted_total.
no float money math exists in A09 money/conservation paths.
```

### A09.3 Settlement And Predicate Price Blindness

Description:
Derive settlement from tape only after A08 predicate receipts exist. Price
signals may be present in projection output, but they cannot decide predicate
truth or payout eligibility.

Primary paths:

```text
src/economy/settlement.rs
tests/economy_predicate_price_blindness.rs
tests/constitution_router_price_quote.rs
```

Guidance:

```text
No payout without PredicateReceipt::Pass.
Price can route future work but cannot alter predicate verdict or settlement
eligibility.
Settlement consumes escrow projection; it does not mint or write a second
wallet ledger.
```

Acceptance:

```bash
cargo test --test economy_predicate_price_blindness --no-fail-fast -- --test-threads=1
cargo test --test constitution_router_price_quote --no-fail-fast -- --test-threads=1
```

Expected:

```text
predicate fail + high price => no payout.
predicate pass + low price => payout follows predicate/escrow rules.
```

### A09.4 Price Broadcast And No Parallel Ledger

Description:
Expose price/market broadcast as a projection over a tape prefix, not as a
standalone market ledger.

Primary paths:

```text
src/economy/price_broadcast.rs
src/economy/projections.rs
tests/economy_no_parallel_ledger.rs
tests/tb_14_price_index.rs
```

Guidance:

```text
price broadcast includes derived_from_tape_head.
no MarketTape/market_tape_shared/root-level market ledger module.
broadcast is read-only; it does not mutate wallet, escrow, or predicate state.
```

Acceptance:

```bash
cargo test --test economy_no_parallel_ledger --no-fail-fast -- --test-threads=1
cargo test --test tb_14_price_index --no-fail-fast -- --test-threads=1
grep -RIn 'market_tape_shared' src tests && exit 1 || true
```

Expected:

```text
no parallel market ledger exists.
price broadcast references a tape prefix and remains reconstructable.
```

## Final Pre-Implementation Gate

A09 implementation may start only when all are true:

- A05 TapeEvent/projection authority exists and targets ChainTape-L4.
- A08 PredicateReceipt shape exists or A09 explicitly uses a stub that cannot
  settle payout.
- the first code change is a failing `economy_tape_replay` or
  `economy_conservation` test.
- `cargo test --test economy_tape_replay --no-fail-fast -- --test-threads=1`
  fails for the missing A09 projection before implementation.
- `cargo test --test economy_conservation --no-fail-fast -- --test-threads=1`
  fails for the missing conservation proof before implementation.
- no wallet, market, price, settlement, or reputation state is introduced as a
  second ledger.

## Full A09 Acceptance

After A05 and A08 exist and A09 implementation is complete:

```bash
cargo test --test economy_tape_replay --no-fail-fast -- --test-threads=1
cargo test --test economy_conservation --no-fail-fast -- --test-threads=1
cargo test --test economy_predicate_price_blindness --no-fail-fast -- --test-threads=1
cargo test --test economy_no_parallel_ledger --no-fail-fast -- --test-threads=1
cargo test --test constitution_economy_gate --no-fail-fast -- --test-threads=1
cargo test --test tb_14_price_index --no-fail-fast -- --test-threads=1
cargo test --test constitution_router_price_quote --no-fail-fast -- --test-threads=1
bash scripts/run_constitution_gates.sh
git diff --check
grep -RInE 'f32|f64' src/economy tests/economy_* && exit 1 || true
grep -RIn 'market_tape_shared' src tests && exit 1 || true
```

Expected:

```text
PREDICATES-GREEN
EconomyProjection is derivable from ChainTape/L4 + CAS only.
No wallet, market, price, settlement, or reputation state becomes a second
source of truth.
No payout can occur without PredicateReceipt::Pass.
No float money math exists in economy money/conservation/settlement paths.
No parallel market ledger exists.
```

## Hard Blockers

```text
A09-IMPLEMENTABLE-AFTER-A05-A08
```

Hard blockers:

- A05 generic tape event/projection authority is missing.
- A08 PredicateReceipt authority shape is missing.
- All parent-plan A09 source/test files are missing.
- Existing `EconomicState` is production chain-resident state, not the planned
  A09 event projection.
- Existing `src/economy/ledger.rs` and `EscrowVault` are not A09 authority.
- Dirty restricted/authority-adjacent paths must be preserved until their
  owning work is understood.

Clean-context audit input for a future implementation PR:

```text
Task brief: A09 Economy Service v0.
Risk class: Class 3, promote to Class 4 if restricted surfaces are touched.
FC nodes: Art. 0.2, FC2-N21, FC2-N28, FC1-N11/N12 predicate boundary.
Evidence: A05/A08 predecessor evidence, A09 tests, constitution gates,
static grep output.
Verdict domain: NO-VIOLATION | VIOLATION-FOUND | RECONSTRUCTION-FAILURE |
SECOND-SOURCE-DRIFT
```
