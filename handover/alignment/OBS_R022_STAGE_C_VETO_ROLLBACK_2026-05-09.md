# OBS R-022 — Stage C Polymarket VETO rollback (TRACE_MATRIX backlinks retired in bulk)

**Date**: 2026-05-09 session #28.
**Triggered by**: pre-commit hook R-022 (TRACE_MATRIX pub-symbol-block) on the 11-commit revert chain that rolls back Stage C P-M2..P-M9 work.
**Authority**: Architect verbatim VETO 2026-05-09: "我是要 VETO + 全 rollback".
**Companion directives**:
- `handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM2_PM4_PM6_BATCH_§8_VETO.md`
- `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md`

## Removed backlinks (bulk; ~40 entries)

All removed TRACE_MATRIX backlinks reference Stage C session #27 work that is being rolled back per architect VETO + Codex G2 audit aggregate VETO. The removed symbols span:

| Atom | Files | Symbols removed |
|------|-------|-----------------|
| P-M2 CompleteSetMergeTx | `src/state/typed_tx.rs`, `src/state/sequencer.rs` | `CompleteSetMergeTx` struct + signing payload + sequencer admission arm + state-root mutator |
| P-M4 CpmmPool | `src/state/q_state.rs` | `CpmmPool` struct + `CpmmPoolsIndex` + `LpShareAmount` + `PoolStatus` + `PoolEventKind` discriminator + EconomicState `cpmm_pools_t` sub-field |
| P-M5 CpmmSwap | `src/state/typed_tx.rs`, `src/state/sequencer.rs` | `CpmmSwapTx` struct + signing payload + `Side` enum + sequencer admission arm + state-root mutator |
| P-M6 Mint-and-Swap Router | `src/state/typed_tx.rs`, `src/state/sequencer.rs` | `BuyWithCoinRouterTx` struct + signing payload + sequencer admission arm + state-root mutator |
| P-M7 PriceIndex extensions | `src/state/price_index.rs` | low-liquidity-warning helper + CPMM price quote fn |
| P-M8 audit_tape views | `src/runtime/audit_views.rs` (deleted), `src/runtime/mod.rs` | per-event view helpers + 4 audit_tape subcommand projections |

Each backlink originally cited architect manual §7.3 / §7.5 / §7.6 / §7.7 / §7.8 / §7.9 verbatim. The architect-spec layer is **unchanged** by this rollback — only the implementation symbols are retired pending rebuild under stricter per-atom §8 cadence.

## Why removal is correct (per atom)

### P-M6 — load-bearing constitution defects

Two findings in Codex G2 audit:
1. `assert_complete_set_balanced` in `src/economy/monetary_invariant.rs` accepted `min(sum_yes, sum_no) == collateral` instead of strict `sum_yes == collateral && sum_no == collateral`. Violates CLAUDE.md §13 economy law ("1 Coin = 1 YES + 1 NO" + "no ghost liquidity") and architect §6.1 CTF invariant. CPMM pool reserves can diverge YES/NO during swaps; `min()` admits ghost-liquidity edge cases.
2. `tests/constitution_router_buy_with_coin.rs::router_atomic_rollback_on_failure` triggered insufficient-balance failure that `src/state/sequencer.rs:2469` rejected BEFORE `q_next` mutation began (`:2514`). Test name was verbatim-correct per architect §7.7 but body never exercised the 9-step composite atomic-rollback path. No tape evidence of atomicity → FC1 "tape-first" implicit invariant violated.

### P-M2 + P-M4 — verbatim spec drift

- P-M2 `src/state/typed_tx.rs:1417` added `timestamp_logical` field; architect §7.3 verbatim specifies 6 fields only.
- P-M4 `src/state/q_state.rs:694` used `event_id_kind` where architect §7.5 verbatim specifies `event_id`.

Non-load-bearing per Codex but break the verbatim-binding contract precedent set by Stage A2/A3.

### P-M3, P-M5, P-M7, P-M8, P-M9 — cascade

These atoms were correctly implemented per their respective architect §7.x verbatim specs (Codex audit only flagged P-M2/P-M4/P-M6). However, they depend on the runtime that's being rolled back:
- P-M3 MarketSeed → mints into the (now-removed) `CompleteSetMintTx` flow
- P-M5 CpmmSwap → operates on the (now-removed) CpmmPool
- P-M7 PriceIndex → reads (now-removed) CPMM pool reserves for price
- P-M8 audit_tape views → projects (now-removed) pool / position state
- P-M9 controlled smoke → end-to-end uses entire rolled-back stack

Per packet §6 batch §8 cascade rule (user-accepted risk 2026-05-09 verbatim), VETO on any atom in the batch cascades to all batch atoms; non-batch atoms (P-M3/P-M5/P-M7/P-M8/P-M9) cascade because they runtime-depend on the batch atoms.

## Why no immediate replacement TRACE_MATRIX backlinks needed

The architect-spec layer (manual §7.3 / §7.5 / §7.6 / §7.7 / §7.8 / §7.9) is unchanged. Once the per-atom rebuild lands per the remediation directive Phase F, each rebuilt symbol will re-introduce its TRACE_MATRIX backlink with the same architect manual reference. The current rollback removes the implementation; it does not retire the architect spec.

The remediation directive §1.B mandates **three new constitution gates** (Phase E) that run BEFORE Phase F rebuild begins:
- `tests/constitution_architect_verbatim_struct_binding.rs` — verbatim spec binding gate (catches schema drift mechanically)
- `tests/constitution_class4_atomic_rollback_witness.rs` — atomic rollback test pattern enforcement (catches vacuous tests)
- `tests/constitution_economy_strict_equality.rs` — strict-equality lint (catches `min()` weakening)

These gates ensure the rebuilt TRACE_MATRIX backlinks anchor symbols that pass the new mechanism checks.

## Behavioral preservation evidence

This rollback is mechanical (`git revert` of 11 commits in reverse-chronological order). Behavioral preservation = **return to pre-Stage-C state**:
- Pre-Stage-C: HEAD `b468140` (manifest baseline) — gates 175 / workspace ~1308
- Post-rollback: equivalent semantically; HEAD lineage one composite-revert past pre-rollback state
- All evidence preserved in git history (revert is additive; per `feedback_no_retroactive_evidence_rewrite` strict reading)

Verification (will be run before this OBS doc closes):
- `cargo check --workspace` — clean
- `cargo test --workspace --no-fail-fast` — count matches pre-Stage-C ~1308 baseline
- `bash scripts/run_constitution_gates.sh` — 175 / 0 / 1
- `cargo test --lib verify_trust_root_passes_on_intact_repo` — PASS

## Wire-format note

The 11-commit revert removes:
- `EconomicState.cpmm_pools_t` sub-field (P-M4 added; revert removes)
- `TypedTx::CompleteSetMerge` / `TypedTx::CpmmSwap` / `TypedTx::BuyWithCoinRouter` enum variants (P-M2/P-M5/P-M6 added; revert removes)

Pre-Stage-C chain snapshots (b468140 era) deserialize cleanly post-rollback (the schema returns to its pre-Stage-C shape). Session #27 chain snapshots (if any persisted in `handover/evidence/stage_c_pm9_controlled_smoke_20260509T042633Z/`) are wire-incompatible with post-rollback code — but per `feedback_no_retroactive_evidence_rewrite` they remain in git history under their original commits and are not migrated forward.

## Cross-references

- VETO directive: `handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM2_PM4_PM6_BATCH_§8_VETO.md`
- Remediation directive: `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md`
- Codex G2 audit transcript: agent ID `a1e5cd6edeb8377bc` (session #28 audit dispatch)
- Architect manual: `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md` §7.3 / §7.5 / §7.6 / §7.7 / §7.8 / §7.9 (verbatim spec; unchanged by rollback)
- Pre-Stage-C baseline HEAD: `b468140` (manifest baseline cited in plan `cozy-waddling-raven.md` §Context)
- M2 kill decision (preserved through rollback; separate authority): `handover/decisions/2026-05-09_M2_KILL_AND_SUBSTRATE_STABLE_DECLARATION.md`
