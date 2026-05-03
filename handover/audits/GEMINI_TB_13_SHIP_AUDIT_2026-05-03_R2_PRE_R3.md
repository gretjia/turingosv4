# Gemini TB-13 Ship Audit — CompleteSet + MarketSeedTx (Class 3)
**Round**: R2
**Date**: 2026-05-03
**Test baseline**: cargo test --workspace = 783 PASS / 0 FAILED / 150 ignored
**Recursive self-audit**: PASS (5 clauses + 12 SG + 11 G + 0/7 halts)
**Real-LLM regression smoke**: PASS (7/7 chaintape replay indicators GREEN)
**Elapsed**: 56.8s
**Prompt size**: 758,549 chars
**Audit mode**: Class 3 dual (Codex impl-paranoid + Gemini architectural strategic; per architect Part A §4.8)

---

**TO**: TURINGOS v4 Architect
**FROM**: Gemini TB-13 Independent Auditor
**SUBJECT**: Class 3 Architectural Strategic Review — CompleteSet + MarketSeedTx
**DATE**: 2026-05-03

This document constitutes my independent, adversarial audit of the TB-13 implementation. My review is based on the provided charter, architect's ruling, recursive self-audit, smoke evidence, and source code excerpts. My mandate is to be skeptical and apply a conservative merge rule (VETO > CHALLENGE > PASS).

---

### **Architect's Mandated Questions (Q1-Q9)**

**1. Does CompleteSetMint create or destroy money?**
**Verdict**: **PASS**.
**Finding**: No. The `CompleteSetMintTx` accept arm in `src/state/sequencer.rs:1805-1839` performs a balanced transfer. It debits `balances_t[owner]` and credits `conditional_collateral_t[event_id]` by the same `amount`. The `assert_total_ctf_conserved` invariant, extended in TB-13 to include `conditional_collateral_t` as a coin holding (`src/economy/monetary_invariant.rs:186-189`), correctly verifies that this is a migration of funds, not a mint or burn. The integration test `sg_13_1_mint_one_coin_yields_one_yes_plus_one_no_total_coin_conserved` (`tests/tb_13_complete_set.rs:88`) explicitly confirms this.

**2. Can Redeem fire without a system-emitted resolution?**
**Verdict**: **PASS**.
**Finding**: No. The `CompleteSetRedeemTx` accept arm in `src/state/sequencer.rs:1840-1923` gates execution on the `task_markets_t` state. The `match` statement on `market_state` (`src/state/sequencer.rs:1852`) explicitly returns `Err(TransitionError::RedeemBeforeResolution)` for `TaskMarketState::Open` and `TaskMarketState::Expired`. Redemption is only possible for `Finalized` or `Bankrupt` states. This is confirmed by the integration test `sg_13_5_redeem_unavailable_before_outcome_resolution` (`tests/tb_13_complete_set.rs:170`).

**3. Can Redeem with `outcome=Yes` and a TaskBankruptcy-style resolution_ref bypass the outcome check?**
**Verdict**: **PASS**.
**Finding**: No. The `CompleteSetRedeemTx` accept arm in `src/state/sequencer.rs:1852-1866` contains a nested `match` that strictly enforces consistency between the on-chain `market_state` and the transaction's `redeem.outcome`. A `TaskMarketState::Bankrupt` (NO outcome) paired with an `outcome=Yes` redeem attempt will fall through to the `(_, _)` arm and be rejected with `Err(TransitionError::InvalidResolutionRef)`. The integration test `sg_13_6_redeem_after_yes_outcome_pays_yes_not_no` (`tests/tb_13_complete_set.rs:199`) includes symmetric checks for this mismatch.

**4. Does the 6-holding `total_supply_micro` sum hold across all TB-13 typed_tx?**
**Verdict**: **PASS**.
**Finding**: Yes. The `total_supply_micro` function in `src/economy/monetary_invariant.rs:158-192` was extended to include `conditional_collateral_t` as the 6th coin holding (alongside `balances_t`, `escrows_t`, `stakes_t`, `challenge_cases_t`, and the now-removed `claims_t` which was an intent registry). All three new typed-tx variants (`CompleteSetMintTx`, `CompleteSetRedeemTx`, `MarketSeedTx`) are balanced transfers between these holdings (`balances_t` ↔ `conditional_collateral_t`). The `assert_total_ctf_conserved` invariant is called after each transition in the integration tests (e.g., `tests/tb_13_complete_set.rs:125`), confirming the sum holds.

**5. Does `assert_complete_set_balanced` (MIN-semantics: `min(Σ_yes, Σ_no) == collateral`) hold after every transition?**
**Verdict**: **PASS**.
**Finding**: Yes. The `assert_complete_set_balanced` function in `src/economy/monetary_invariant.rs:278-323` correctly implements the MIN-semantics invariant. This form is robust to post-resolution redemptions where the losing side's shares become stranded (and thus `Σ_losing > collateral`). The integration tests (`sg_13_1...`, `halt_total_supply_micro_unchanged_across_mint_redeem`, `halt_complete_set_balanced_post_seed`) confirm this invariant holds after mint, seed, and redeem transitions.

**6. Can MarketSeedTx create liquidity without provider balance?**
**Verdict**: **PASS**.
**Finding**: No. The `MarketSeedTx` accept arm in `src/state/sequencer.rs:1924-1961` has two gates. First, it rejects zero or negative collateral with `InsufficientCollateral` (`src/state/sequencer.rs:1933`). Second, it verifies `provider_bal >= seed.collateral_amount` and rejects with `InsufficientBalanceForMint` if the check fails (`src/state/sequencer.rs:1941`). This is confirmed by tests `sg_13_3_market_seed_fails_if_provider_lacks_balance` and `sg_13_4_market_seed_cannot_create_liquidity_without_collateral`.

**7. Are conditional shares anywhere counted as Coin?**
**Verdict**: **PASS**.
**Finding**: No. Per architect CR-13.3 + SG-13.2, shares are claims, not Coin. The implementation in `src/economy/monetary_invariant.rs:189-192` explicitly omits `conditional_share_balances_t` from the `total_supply_micro` sum. The integration test `sg_13_2_yes_no_shares_not_in_total_coin_supply` (`tests/tb_13_complete_set.rs:133`) is designed to fail if this were violated.

**8. Could a malformed `ShareAmount` underflow at redeem?**
**Verdict**: **PASS**.
**Finding**: No. The `ShareAmount` struct in `src/state/typed_tx.rs:1003` uses `units: u128`, which is non-negative by definition. The `CompleteSetRedeemTx` accept arm in `src/state/sequencer.rs:1879` performs a pre-subtraction check: `if owned_units < redeem.share_amount.units { return Err(TransitionError::RedeemMoreThanOwned); }`. This gate prevents any subtraction that could lead to an underflow. This is confirmed by `halt_redeem_more_than_owned_rejected` (`tests/tb_13_complete_set.rs:327`).

**9. Forward-fence: does any new TB-13 module file import legacy `prediction_market`?**
**Verdict**: **PASS**.
**Finding**: No. The ship-gate test `tests/tb_13_legacy_cpmm_forward_fence.rs` provides a robust, automated check for this. It scans all in-scope files for TB-13 authoring markers and then greps those spans for forbidden tokens, including `prediction_market::` and `BinaryMarket`. The self-audit report confirms this fence is active and has already caught and fixed violations during development. This satisfies the architect's halting trigger.

---

### **Architectural Strategic Questions (Q10-Q13)**

**10. Does CompleteSet schema extend cleanly to TB-14 PriceIndex?**
**Verdict**: **PASS**.
**Finding**: Yes. The `ConditionalShareBalances` schema in `src/state/q_state.rs:556` is a `BTreeMap<AgentId, BTreeMap<EventId, ShareSidePair>>`. Aggregating total YES or NO shares for a given `EventId` (to derive long/short interest for a price index) is a straightforward iteration over the outer map's values and summing the inner map's `ShareSidePair` units. The schema is well-structured for this purpose.

**11. Does the `EventId == TaskId` 1:1 simplification hold up under TB-14+ multi-event-per-task scenarios?**
**Verdict**: **PASS**.
**Finding**: Yes. The schema `pub struct EventId(pub TaskId)` (`src/state/typed_tx.rs:989`) is a newtype wrapper. This is a sound architectural choice. If future requirements demand multiple events per task, `EventId` can be evolved into a struct like `struct EventId { task: TaskId, sub_event: u16 }` without breaking the schemas that use it as a key (e.g., `ConditionalCollateralIndex`), provided it continues to implement `Ord`, `Hash`, etc. The simplification is effective for TB-13 and does not create a future roadblock.

**12. Is the `ResolutionRef` model robust to multi-resolver scenarios in TB-15+?**
**Verdict**: **PASS**.
**Finding**: Yes. The `ResolutionRef` struct (`src/state/typed_tx.rs:1019`) points to a `resolution_tx_id`. The sequencer logic (`src/state/sequencer.rs:1852`) currently hardcodes the accepted resolution transaction types (`TaskBankruptcyTx`, `FinalizeRewardTx`). To support a new resolver (e.g., a ChallengeCourt emitting a `CourtRulingTx`), the `match` statement can be extended. The model of referencing an on-chain, system-emitted fact is fundamentally robust and extensible.

**13. Is the MIN-semantics `assert_complete_set_balanced` invariant the right form, particularly for adversarial patterns?**
**Verdict**: **CHALLENGE**.
**Finding**: The MIN-semantics invariant itself is correct for the transitions it models, as demonstrated by tracing a partial redeem sequence. However, the state machine appears to permit an undesirable transition: minting new shares for an event that has already been resolved.
- **Trace**: An agent can call `CompleteSetMintTx` for an `event_id` whose corresponding `task_markets_t` entry is already in state `Finalized` or `Bankrupt`.
- **Code**: The `CompleteSetMintTx` accept arm (`src/state/sequencer.rs:1805-1839`) does not check the `task_markets_t` state for the given `event_id`.
- **Impact**: This allows an agent to lock collateral and mint a complete set of shares for a known outcome. If the outcome was YES, they can immediately redeem the YES shares for a risk-free refund of their collateral. If the outcome was NO, their collateral is locked against worthless YES shares and stranded NO shares. While this doesn't break conservation or create free money, it introduces an unnecessary and potentially confusing state where liquidity can be added to a closed market. This could be exploited for griefing or creating noise in on-chain data that TB-14+ will consume.
- **Recommendation**: The `CompleteSetMintTx` and `MarketSeedTx` dispatch arms should be gated to reject transactions targeting an `event_id` that is no longer in the `Open` state.

---

### **Final Verdict**

-   **VERDICT**: **CHALLENGE**
-   **Conviction**: **High**
-   **Recommendation**: **FIX-THEN-PROCEED**

The core economic invariants (Q1-Q9) are soundly implemented and rigorously tested. The architecture is robust and forward-compatible for its intended purpose (Q10-Q12). However, the lack of a state check in the `CompleteSetMintTx` and `MarketSeedTx` dispatch arms (Q13) represents a minor but tangible architectural smell that should be rectified before shipping. It does not violate a halting trigger but fails the principle of least surprise and could complicate future layers that build on this substrate. The fix is localized and low-risk.

**Action Item**: Add a state check in the `CompleteSetMintTx` and `MarketSeedTx` dispatch arms in `src/state/sequencer.rs` to reject transactions if `task_markets_t[event_id.0].state != TaskMarketState::Open`. After this fix is implemented and verified by a new integration test, the ship can proceed.