# OBS R-022 — TB-14 legacy `PriceIndex` removed (TRACE_MATRIX backlink retired)

**Date**: 2026-05-03 (TB-14 Atom 2).
**Triggered by**: pre-commit hook R-022 (TRACE_MATRIX pub-symbol-block).
**Removed symbol**: `pub struct PriceIndex(pub BTreeMap<TxId, MicroCoin>)` in `src/state/q_state.rs:699` + `EconomicState.price_index_t: PriceIndex` field at `src/state/q_state.rs:167`.
**Removed backlink**: `/// TRACE_MATRIX WP § 2 — tx → posted price (last accepted price index).` (the doc-comment immediately preceding the deleted `pub struct PriceIndex(...)` declaration).

## Why removal is correct

The legacy `PriceIndex(BTreeMap<TxId, MicroCoin>)` was a TB-3 stub: a never-populated single-MicroCoin-per-tx map declared as a placeholder for a future "price index" concept. No production code path ever wrote to `economic_state_t.price_index_t`; no test other than serde round-trip ever read it.

TB-14 (architect 2026-05-03 ruling §5.1 + §5.2; charter `handover/tracer_bullets/TB-14_charter_2026-05-03.md` §1) replaces this stub with `compute_price_index(econ: &EconomicState) -> BTreeMap<TxId, NodeMarketEntry>` in `src/state/price_index.rs` — a **pure deterministic derived view** over `node_positions_t` (TB-12 substrate) + `conditional_share_balances_t` (TB-13 substrate). Per architect §5.1 ("price is signal, not truth") and charter §7 auto-resolution A ("no second source-of-truth"), the price view is **NEVER stored on canonical state**: storing it would create a sync-bug surface (cache-truth divergence) and a Goodhart vector. The pure-fn shape is replay-deterministic per Art.0.2 (no env / clock / RNG input).

## Why the TRACE_MATRIX entry does not need a replacement

The legacy `PriceIndex` backlink (`/// TRACE_MATRIX WP § 2 — tx → posted price`) anchored a stub type that had no flowchart role: WP § 2 lists `price_index_t` as a sub-field of `EconomicState`, but neither the constitutional flowchart nor any TB shipped a sequencer dispatch arm that wrote to it. The role it carried is now expressed by `compute_price_index` whose backlink is `TRACE_MATRIX FC3-N42` (architect §5.1 + charter §3 Atom 2) — registered in this same atom's commit at `src/state/price_index.rs` and witnessed by `tests/fc_alignment_conformance.rs::fc3_n42_compute_price_index_pure_fn_witness`.

Net effect: the canonical state has 12 sub-fields (was 13 pre-TB-14); the price-derivation role moves from a never-populated stub field to a pure derived view; flowchart anchoring strengthens (FC3-N42 typed and witnessed) rather than weakens.

## Behavioral preservation evidence

Pre-rehash:
- `cargo test --workspace = 808 passed / 7 failed (3 trust-root pre-rehash + 4 halt-trigger stubs) / 150 ignored`

Post-rehash (G2 single-rehash discipline; CP-A gate):
- `cargo test --workspace = 811 passed / 4 failed (only halt-trigger stubs #1/#2/#3/#6 — Atoms 3/5 fill) / 150 ignored`
- Δ vs HEAD (`0370d66`): +17 passed (= 15 net new tests + 2 halt-trigger transitions #4 + #5 from `unimplemented!()` → PASS); -2 failed.
- All 17 `PriceIndex` / `price_index_t` references identified by pre-Atom-2 reference scan are accounted for in the commit diff (G4 enforcement). No silent test-file compile failures (the previous `/opusplan` attempt missed `tests/economic_state_reconstruct.rs:129` `PriceIndex::default()`, causing 131 tests to vanish; this commit explicitly updates that line plus the other 16).

Conservation invariant (`monetary_invariant.rs`) unchanged: `price_index_t` was already in the explicit "NOT counted" list (it never held a Coin; the TB-14 derived view stays not-a-holding by construction).

## Wire-format note

This is a wire-format break for `EconomicState` serde JSON: pre-TB-14 chain snapshots include the empty `"price_index_t": {}` field; post-TB-14 snapshots do not. Per `feedback_no_retroactive_evidence_rewrite`: legacy `economic_state_t.price_index_t` was always serialized as the empty map (no production writer). Going-forward chain snapshots use the 12-sub-field shape; historical snapshots are not migrated.

## Cross-references

- Architect operative spec: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §5 (TB-14 PriceIndex v0 + Boltzmann Masking).
- TB-14 charter: `handover/tracer_bullets/TB-14_charter_2026-05-03.md` §1 + §2 + §3 Atom 2 + §7 auto-resolution A.
- Plan v2 (refined post-/opusplan-failure): `~/.claude/plans/sparkling-hugging-donut.md` §G3 (architectural fence fix replacing band-aid) + §G4 (17-reference enumeration).
- Replacement TRACE_MATRIX backlink: `src/state/price_index.rs::compute_price_index` (`TRACE_MATRIX FC3-N42`); `src/state/price_index.rs::RationalPrice::dominates_by` (`TRACE_MATRIX FC3-N42 helper`).
- FC3-N42 witness: `tests/fc_alignment_conformance.rs::fc3_n42_compute_price_index_pure_fn_witness`.
- Precedent (TB-13 ResolutionRef removal): `OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md`.
