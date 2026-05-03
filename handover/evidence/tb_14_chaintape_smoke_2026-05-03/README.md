# TB-14 Atom 6 — ChainTape smoke (post-wire-swap regression)

**Date**: 2026-05-03
**Source**: `tests/tb_14_chaintape_smoke.rs::tb_14_atom_6_post_wire_swap_chaintape_replay_preserves_price_index_determinism`
**Trigger**: TB-14 Atom 6 production wire-swap (excise legacy CPMM scaffolding; reroute bus snapshot price-signal surface through `compute_price_index` + `compute_mask_set` integer-rational derived views).

## Headline

- L4 entries: 2 (mint + redeem)
- L4.E entries: 0
- All 7 ReplayReport indicators GREEN: true
- Live `state_root_t`: `3aa95383939c8f9246c080b54141e03581614d155b6d3599f5b0f8a4f3be1610`
- Replay `final_state_root_hex`: `3aa95383939c8f9246c080b54141e03581614d155b6d3599f5b0f8a4f3be1610`
- `live.economic_state_t == replayed.economic_state_t`: byte-equal
- `compute_price_index(live)` == `compute_price_index(replayed)`: byte-equal
- `compute_price_index` idempotent across 5 invocations: ✓
- Empty `node_positions_t` → empty PriceIndex BTreeMap: ✓

## What this evidence proves (Atom 6 specific)

1. The Atom 6 production wire-swap (excised `prediction_market.rs`, `kernel.markets`, `BoltzmannParams`, legacy f64 `boltzmann_select_parent`; rewired `bus.snapshot` to derive `price_index` + `mask_set` from `Sequencer::q_snapshot`'s `EconomicState`) does NOT regress chain-replay determinism.
2. `verify_chaintape` reconstructs a `QState` from persisted artifacts whose `final_state_root_hex` matches live `state_root_t` (Art.0.2 Tape Canonical preserved across the wire-swap).
3. The TB-14 derived view (`compute_price_index(econ)`) is replay-deterministic by composition: pure function over a byte-equal-replayed `EconomicState` yields byte-equal `BTreeMap<TxId, NodeMarketEntry>` (FR-14.x / FC3-N42 chaintape integration evidence).
4. `compute_price_index` is idempotent across N calls (Art.0.2 pure-function determinism at the derived-view layer).
5. Empty `node_positions_t` → empty PriceIndex (FR-14.3 / halt-trigger #5 extended at the chaintape integration layer).

## What is NOT in scope here

- **Non-empty PriceIndex via WorkTx**: this smoke uses CompleteSet flow only (TB-13 substrate). A WorkTx-creates-NodePosition flow (TB-12 substrate that produces non-empty PriceIndex) is covered by the in-memory unit tests at `tests/tb_14_price_index.rs` + halt-triggers + `src/state/price_index.rs` inline tests. Per `feedback_chaintape_externalized_proposal`, the chaintape smoke records what the system externalizes via `submit_typed_tx` end-to-end; the per-position aggregation is pure-function-tested elsewhere.
- **`mask_set` via Tape children**: `compute_mask_set` requires a Tape; this smoke does not exercise mask computation (covered by `tests/tb_14_mask_set.rs` + halt-triggers #3 / #6).
- **Boltzmann v2 selector**: covered by inline tests in `src/sdk/actor.rs::tests::v2_*`. Production wire-up at `experiments/minif2f_v4/src/bin/evaluator.rs:~1559` is exercised by the `--smoke` / `--half` evaluator runs.
