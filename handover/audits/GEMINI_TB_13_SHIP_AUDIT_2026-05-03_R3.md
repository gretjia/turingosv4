# Gemini TB-13 Ship Audit — CompleteSet + MarketSeedTx (Class 3)
**Round**: R3
**Date**: 2026-05-03
**Test baseline**: cargo test --workspace = 783 PASS / 0 FAILED / 150 ignored
**Recursive self-audit**: PASS (5 clauses + 12 SG + 11 G + 0/7 halts)
**Real-LLM regression smoke**: PASS (7/7 chaintape replay indicators GREEN)
**Elapsed**: 57.1s
**Prompt size**: 782,898 chars
**Audit mode**: Class 3 dual (Codex impl-paranoid + Gemini architectural strategic; per architect Part A §4.8)

---

Here is my independent ship audit for Gemini TB-13.

---

### **AUDIT START: Gemini TB-13 Ship Audit**

**Role**: Skeptical adversarial reviewer. Independent of Codex.
**Merge Rule**: VETO > CHALLENGE > PASS.
**Scope**: TB-13 CompleteSet + MarketSeedTx (Class 3 architectural strategic review).

### Executive Summary

The TB-13 implementation correctly establishes the foundational substrate for conditional shares ("1 locked Coin = 1 YES_E + 1 NO_E") without introducing the forbidden trading, pricing, or AMM logic. The core monetary invariants—conservation of total supply across mint/redeem/seed operations and the exclusion of shares from the coin supply—are correctly implemented and tested. The critical resolution-gating logic for `CompleteSetRedeemTx` appears robust, correctly rejecting redemptions before a system-emitted resolution and enforcing outcome consistency.

However, the audit identifies one non-blocking but significant architectural risk concerning the forward-compatibility of the `ResolutionRef` model (Q12), which warrants a **CHALLENGE**. The current model tightly couples redemption to specific transaction types (`TaskBankruptcyTx`, `FinalizeRewardTx`), which may prove brittle in TB-15+ multi-resolver scenarios. While not a halting trigger for TB-13, this design choice introduces future refactoring debt.

All architect-mandated implementation-paranoid questions (Q1-Q9) are assessed as PASS in the final remediated version of the code, which correctly handles negative-amount attacks and enforces event-state checks on mint/seed operations.

---

### **Architect's Mandated Questions (Q1-Q9)**

**1. Does CompleteSetMint create or destroy money?**
**Finding**: PASS. The operation is a balance↔collateral migration. The `CompleteSetMintTx` dispatch arm in `src/state/sequencer.rs:1808` correctly debits `balances_t` and credits `conditional_collateral_t` by the same `amount`. The extended `assert_total_ctf_conserved` invariant in `src/economy/monetary_invariant.rs:205` correctly includes `conditional_collateral_t` in the 6-holding sum, ensuring the total supply is bit-for-bit conserved.
**Adversarial Note**: An initial vulnerability existed where a negative `amount` (`MicroCoin` is `i64`) would credit the user's balance while creating negative collateral and a huge `u128` share balance. The recursive self-audit's remediation log (`RECURSIVE_AUDIT...md` §12.3) confirms this was fixed by changing the check from `== 0` to `<= 0` at `src/state/sequencer.rs:1816`. The remediated implementation is correct.

**2. Can Redeem fire without a system-emitted resolution?**
**Finding**: PASS. No. The `CompleteSetRedeemTx` dispatch arm at `src/state/sequencer.rs:1882` performs a state lookup on `task_markets_t`. If the state is `Open` or `Expired`, it is correctly rejected with `RedeemBeforeResolution`. This is confirmed by integration test `sg_13_5_redeem_unavailable_before_outcome_resolution` in `tests/tb_13_complete_set.rs`.

**3. Can Redeem with `outcome=Yes` and a TaskBankruptcy-style resolution_ref bypass the outcome check?**
**Finding**: PASS. No. The dispatch arm at `src/state/sequencer.rs:1889-1899` contains an explicit `match` on `(market_state, redeem.outcome)`. A `(TaskMarketState::Bankrupt, OutcomeSide::Yes)` pair correctly falls through to the `InvalidResolutionRef` rejection path. This is confirmed by integration test `sg_13_6_redeem_after_yes_outcome_pays_yes_not_no`.

**4. Does the 6-holding `total_supply_micro` sum hold across all TB-13 typed_tx?**
**Finding**: PASS. Yes. The `total_supply_micro` function at `src/economy/monetary_invariant.rs:158` was correctly extended to include the sum of `conditional_collateral_t`. All three new transaction types (`CompleteSetMintTx`, `CompleteSetRedeemTx`, `MarketSeedTx`) represent balanced transfers between holdings (balances ↔ collateral), preserving the sum. This is verified by `halt_total_supply_micro_unchanged_across_mint_redeem` in `tests/tb_13_complete_set.rs`.

**5. Does `assert_complete_set_balanced` (MIN-semantics: `min(Σ_yes, Σ_no) == collateral`) hold after every transition?**
**Finding**: PASS. Yes. The MIN-semantics implementation at `src/economy/monetary_invariant.rs:289` is the correct and robust form of the invariant, correctly handling the post-resolution state where losing-side shares are stranded but the winning side remains fully collateralized. The recursive self-audit (`RECURSIVE_AUDIT...md` §12.3) notes that this assertion is now called from every relevant dispatch arm in `src/state/sequencer.rs` post-remediation, ensuring live enforcement.

**6. Can MarketSeedTx create liquidity without provider balance?**
**Finding**: PASS. No. The `MarketSeedTx` dispatch arm at `src/state/sequencer.rs:1959` correctly gates on `balances_t[provider] >= collateral_amount`, rejecting with `InsufficientBalanceForMint`. This is confirmed by test `sg_13_3_market_seed_fails_if_provider_lacks_balance`. The negative-amount attack vector is also closed by the `<= 0` check at `src/state/sequencer.rs:1953` per the VETO remediation.

**7. Are conditional shares anywhere counted as Coin?**
**Finding**: PASS. No. The `total_supply_micro` function at `src/economy/monetary_invariant.rs:158` correctly omits `conditional_share_balances_t` from its sum, per architect CR-13.3. The test `sg_13_2_yes_no_shares_not_in_total_coin_supply` provides explicit verification.

**8. Could a malformed `ShareAmount` underflow at redeem?**
**Finding**: PASS. No. Per `src/state/typed_tx.rs:1105`, `ShareAmount` is a `u128` newtype, preventing negative values at the type level. The `CompleteSetRedeemTx` dispatch arm at `src/state/sequencer.rs:1907` performs a `owned_units < redeem.share_amount.units` check before debiting, rejecting with `RedeemMoreThanOwned` and preventing underflow.

**9. Forward-fence: does any new TB-13 module file import legacy `prediction_market`?**
**Finding**: PASS. No. The ship-gate test `tests/tb_13_legacy_cpmm_forward_fence.rs` provides robust protection. The recursive self-audit's remediation log (`RECURSIVE_AUDIT...md` §12.3, Q9 fix) confirms the fence was hardened to include an unconditional whole-file scan for `HARD_BANNED_LEGACY_IMPORTS`, closing a potential loophole. This is a strong guarantee.

---

### **Architectural Strategic Questions (Q10-Q13)**

**10. Does CompleteSet schema extend cleanly to TB-14 PriceIndex?**
**Finding**: PASS. Yes. The `conditional_share_balances_t` schema at `src/state/q_state.rs:511` is a `BTreeMap<AgentId, BTreeMap<EventId, ShareSidePair>>`. Aggregating total YES/NO depth for a given `EventId` requires iterating over the outer `AgentId` map. While potentially inefficient for markets with many small holders, it is a clean and correct derivation path. The schema is sufficient for TB-14's needs.

**11. Does the `EventId == TaskId` 1:1 simplification hold up?**
**Finding**: PASS. Yes. The use of a newtype `pub struct EventId(pub TaskId)` at `src/state/typed_tx.rs:1088` is a sound architectural choice. It provides type safety for TB-13 while allowing `EventId` to be redefined in the future (e.g., as a struct or enum for multi-event scenarios) without breaking the signatures of functions that consume it. This is a good example of simplifying for the present without foreclosing the future.

**12. Is the `ResolutionRef` model robust to multi-resolver scenarios in TB-15+?**
**Finding**: CHALLENGE. The current `ResolutionRef` model at `src/state/typed_tx.rs:1120` and its validation logic in `src/state/sequencer.rs:1882` are tightly coupled to specific transaction types (`TaskBankruptcyTx`, `FinalizeRewardTx`). If TB-15+ introduces new resolution mechanisms (e.g., `ChallengeCourtRulingTx`, `OracleUpdateTx`), the `CompleteSetRedeemTx` dispatch arm will require modification to recognize these new transaction types. A more abstract and robust long-term design might involve a canonical `ResolutionsIndex` in `QState` that maps `EventId` to a resolved `OutcomeSide`, decoupling the redemption logic from the specific mechanism of resolution. The current implementation is not a bug but represents an architectural choice that incurs future refactoring debt.

**13. Is the MIN-semantics `assert_complete_set_balanced` invariant the right form?**
**Finding**: PASS. Yes. The MIN-semantics invariant is correct and more robust than a strict equality check on both sides. It correctly models the post-resolution state where losing-side shares become unbacked claims. My primary adversarial concern was minting/seeding into an already-resolved event to grief the chain state. The recursive self-audit's remediation log (`RECURSIVE_AUDIT...md` §12.3, Gemini Q13 fix) confirms that an `EventNotOpen` gate was added to the `CompleteSetMintTx` and `MarketSeedTx` dispatch arms at `src/state/sequencer.rs:1823` and `src/state/sequencer.rs:1959` respectively. This closes the attack vector and makes the overall design robust.

---

### **Final Verdict**

-   **VERDICT**: **CHALLENGE**
-   **Conviction**: **High**
-   **Recommendation**: **FIX-THEN-PROCEED**

**Rationale**: The implementation of TB-13 is sound and passes all critical safety checks (Q1-Q9) in its remediated form. However, the `ResolutionRef` model (Q12) introduces a point of future architectural friction. While not a VETO-level flaw for TB-13, it is a significant enough design choice to warrant a CHALLENGE. The architect should acknowledge this future refactoring cost. The recommended "fix" is to add an OBS (observation) to the project backlog to track the redesign of the resolution mechanism for TB-15, rather than blocking the TB-13 ship. With this acknowledgment, the project can proceed.