# TB-13 Atom 6 round-6 — non-empty TB-13 chaintape replay smoke

**Date**: 2026-05-03
**Source**: `tests/tb_13_chaintape_smoke.rs::rq3_non_empty_tb13_chaintape_replays_with_state_root_match`
**Trigger**: Codex round-3 RQ3 finding (the existing real-LLM smoke proves the EconomicState 13-sub-field schema round-trips with EMPTY TB-13 maps; non-empty replay determinism was not directly evidenced) + Codex round-4 RQ3 follow-up (the round-5 closure overclaimed state-root equality as cryptographic proof of map equality — fixed in round-6 by adding a direct map-equality assertion via manual `replay_full_transition` re-replay).

## Headline

- L4 entries: 2 (mint + redeem)
- L4.E entries: 0
- All 7 ReplayReport indicators GREEN: true
- Live `state_root_t` (post-drain): `6986f6d4045a7e7c9785177ddbbbbf80f7a8215a4b0409f6a88bb3f645a8d5eb`
- Replay `final_state_root_hex`: `6986f6d4045a7e7c9785177ddbbbbf80f7a8215a4b0409f6a88bb3f645a8d5eb`
- Pre-shutdown `conditional_collateral_t` size: 2
- Pre-shutdown `conditional_share_balances_t` owner count: 1

## What this evidence proves (RQ3 closure — round-6)

1. Two real signed TB-13 typed-tx (CompleteSetMint + CompleteSetRedeem) flow through the full production path: `submit_agent_tx` → driver → `Git2LedgerWriter` persist → on-disk L4 chain.
2. Pre-shutdown live state has non-empty TB-13 maps (sanity).
3. `verify_chaintape` reconstructs a `QState` from the persisted runtime_repo + cas + initial_q_state.json + agent_pubkeys.json + pinned_pubkeys.json whose `final_state_root_hex` matches the live `state_root_t`. Codex round-4 follow-up clarification: the state-root mutator hashes `domain || prev_root || canonical_tx`, NOT the full QState — so state-root equality on its own proves deterministic tx-chain replay (same initial state + same canonical-encoded txs + same pure dispatcher → same root); it does NOT directly assert byte-equal QState reconstruction.
4. **Round-6 direct map-equality check**: the smoke also runs `replay_full_transition` manually against the persisted artifacts and asserts `replayed_q.economic_state_t.conditional_collateral_t == live_q.economic_state_t.conditional_collateral_t` AND `... .conditional_share_balances_t == ...` AND full `economic_state_t` equality. This is the direct map-equality evidence that closes RQ3 without relying on dispatch-determinism implication.
5. Submit-time + replay-time agent signature verification is exercised end-to-end for both `CompleteSetMint` and `CompleteSetRedeem` (Gate 4 covers both).
6. Two-tx state-root chain (initial → mint → redeem) replays deterministically.

## What is NOT in scope here

- **`MarketSeedTx`**: not exercised in this smoke. Coverage lives in `tests/tb_13_complete_set.rs::sg_13_3` / `sg_13_4` + canonical encode round-trip in `typed_tx.rs` U3. Adding seed to this smoke would not add chaintape-replay evidence beyond what mint already proves (seed mutates the same maps).
- **Resolution mid-test flip**: `task-REDEEM` is pre-seeded as `Finalized` in `initial_q` rather than flipped via a system-emitted `FinalizeReward` / `TaskBankruptcy` mid-test. The state-flip mechanism itself is exercised by TB-8 / TB-11 integration tests; here we focus on the TB-13 mint+redeem chaintape replay determinism.
- **Per-tactic decomposition**: per `feedback_chaintape_externalized_proposal`, ChainTape records what the system externalized via `submit_typed_tx`, not private CoT. 1 LLM call → 1 compound payload = 1 Attempt Node remains in effect.
