# TB-13 Atom 6 round-5 — non-empty TB-13 chaintape replay smoke

**Date**: 2026-05-03
**Source**: `tests/tb_13_chaintape_smoke.rs::rq3_non_empty_tb13_chaintape_replays_with_state_root_match`
**Trigger**: Codex round-3 RQ3 finding — the existing real-LLM smoke at `handover/evidence/tb_13_real_llm_smoke_2026-05-03/` proves EconomicState's 13-sub-field schema round-trips with EMPTY TB-13 maps; non-empty `conditional_collateral_t` / `conditional_share_balances_t` round-trip via `verify_chaintape` was not directly evidenced.

## Headline

- L4 entries: 2 (mint + redeem)
- L4.E entries: 0
- All 7 ReplayReport indicators GREEN: true
- Live `state_root_t` (post-drain): `25fe0968049d243cd6c3189f1dc83fe13f2eeb0f65b9d524f2063cec4af15dc5`
- Replay `final_state_root_hex`: `25fe0968049d243cd6c3189f1dc83fe13f2eeb0f65b9d524f2063cec4af15dc5`
- Pre-shutdown `conditional_collateral_t` size: 2
- Pre-shutdown `conditional_share_balances_t` owner count: 1

## What this evidence proves (RQ3 closure)

1. Two real signed TB-13 typed-tx (CompleteSetMint + CompleteSetRedeem) flow through the full production path: `submit_agent_tx` → driver → `Git2LedgerWriter` persist → on-disk L4 chain.
2. Pre-shutdown live state has non-empty TB-13 maps (sanity).
3. `verify_chaintape` reconstructs a `QState` from the persisted runtime_repo + cas + initial_q_state.json + agent_pubkeys.json + pinned_pubkeys.json whose `final_state_root_hex` matches the live `state_root_t` byte-for-byte. Because `state_root_t` is the SHA-256 chain-fold over the entire `QState` (including the TB-13 sub-fields), state-root equality is **cryptographic proof** that replay reconstructed the non-empty `conditional_collateral_t` and `conditional_share_balances_t` bit-equal to the live runtime state.
4. Submit-time + replay-time agent signature verification is exercised end-to-end for both `CompleteSetMint` and `CompleteSetRedeem` (Gate 4 covers both).
5. Two-tx state-root chain (initial → mint → redeem) replays deterministically.

## What is NOT in scope here

- **`MarketSeedTx`**: not exercised in this smoke. Coverage lives in `tests/tb_13_complete_set.rs::sg_13_3` / `sg_13_4` + canonical encode round-trip in `typed_tx.rs` U3. Adding seed to this smoke would not add chaintape-replay evidence beyond what mint already proves (seed mutates the same maps).
- **Resolution mid-test flip**: `task-REDEEM` is pre-seeded as `Finalized` in `initial_q` rather than flipped via a system-emitted `FinalizeReward` / `TaskBankruptcy` mid-test. The state-flip mechanism itself is exercised by TB-8 / TB-11 integration tests; here we focus on the TB-13 mint+redeem chaintape replay determinism.
- **Per-tactic decomposition**: per `feedback_chaintape_externalized_proposal`, ChainTape records what the system externalized via `submit_typed_tx`, not private CoT. 1 LLM call → 1 compound payload = 1 Attempt Node remains in effect.
