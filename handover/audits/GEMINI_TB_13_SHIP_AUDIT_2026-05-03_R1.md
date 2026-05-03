# Gemini TB-13 Ship Audit — CompleteSet + MarketSeedTx (Class 3)
**Round**: R1
**Date**: 2026-05-03
**Test baseline**: cargo test --workspace = 783 PASS / 0 FAILED / 150 ignored
**Recursive self-audit**: PASS (5 clauses + 12 SG + 11 G + 0/7 halts)
**Real-LLM regression smoke**: PASS (7/7 chaintape replay indicators GREEN)
**Elapsed**: 54.4s
**Prompt size**: 752,718 chars
**Audit mode**: Class 3 dual (Codex impl-paranoid + Gemini architectural strategic; per architect Part A §4.8)

---

An independent, skeptical adversarial review of Gemini TB-13 follows.

---

**AUDIT START: Gemini TB-13 Ship Audit**
**Role**: Skeptical Adversarial Reviewer
**Mode**: Class 3 Architectural Strategic Review
**Conservative Merge Rule**: VETO > CHALLENGE > PASS

### §1 Mandated Audit Questions (Q1-Q9)

These questions address the core safety, conservation, and gating invariants mandated by the architect. Any failure here constitutes a high-severity finding.

**1. Does CompleteSetMint create or destroy money? (Must be balance↔collateral migration only.)**

*   **Verification**: The `CompleteSetMintTx` dispatch arm in `src/state/sequencer.rs:1904-1949` performs two primary economic mutations:
    1.  `q_next.economic_state_t.balances_t.0.insert(...)` debits the owner's balance by `mint.amount`.
    2.  `*collateral_entry = ...` credits `q_next.economic_state_t.conditional_collateral_t` by the same `mint.amount`.
*   The conservation invariant function `assert_total_ctf_conserved` in `src/economy/monetary_invariant.rs:159-179` is extended to include `conditional_collateral_t` in its sum of holdings.
*   **Finding**: The transaction is a pure migration of value from one Coin-holding ledger (`balances_t`) to another (`conditional_collateral_t`). The total supply of Coin is bit-for-bit conserved. The subsequent minting of `conditional_share_balances_t` does not affect Coin supply, as shares are correctly excluded from the `total_supply_micro` sum (see Q7).
*   **Verdict**: **PASS**.

**2. Can Redeem fire without a system-emitted resolution? (Must be sequencer-rejected with `RedeemBeforeResolution`.)**

*   **Verification**: The `CompleteSetRedeemTx` dispatch arm in `src/state/sequencer.rs:1964-1983` performs a state lookup on `task_markets_t` for the given `event_id`.
*   The `match` statement explicitly handles `TaskMarketState::Open` and `TaskMarketState::Expired` by returning `Err(TransitionError::RedeemBeforeResolution)`.
*   **Finding**: The gate is correctly implemented. Redemption is only possible if the market state is `Finalized` or `Bankrupt`.
*   **Verdict**: **PASS**.

**3. Can Redeem with `outcome=Yes` and a TaskBankruptcy-style resolution_ref bypass the outcome check? (Must be rejected with `InvalidResolutionRef`.)**

*   **Verification**: The `CompleteSetRedeemTx` dispatch arm in `src/state/sequencer.rs:1964-1983` contains a `match` on the tuple `(market_state, redeem.outcome)`.
*   The only successful paths are `(TaskMarketState::Finalized, OutcomeSide::Yes)` and `(TaskMarketState::Bankrupt, OutcomeSide::No)`.
*   Any other combination on a resolved market, such as `(TaskMarketState::Bankrupt, OutcomeSide::Yes)`, falls through to the wildcard arm which returns `Err(TransitionError::InvalidResolutionRef)`.
*   **Finding**: The outcome-to-state match is strictly enforced. An attacker cannot redeem winning shares with a losing resolution reference.
*   **Verdict**: **PASS**.

**4. Does the 6-holding `total_supply_micro` sum hold across all TB-13 typed_tx?**

*   **Verification**:
    *   `CompleteSetMintTx` (`src/state/sequencer.rs:1904-1949`): `balances_t` debit is equal to `conditional_collateral_t` credit. Both are in the 6-holding sum. Net change is zero.
    *   `CompleteSetRedeemTx` (`src/state/sequencer.rs:1951-2041`): `conditional_collateral_t` debit is equal to `balances_t` credit. Both are in the 6-holding sum. Net change is zero.
    *   `MarketSeedTx` (`src/state/sequencer.rs:2043-2090`): `balances_t` debit is equal to `conditional_collateral_t` credit. Both are in the 6-holding sum. Net change is zero.
*   The integration test `halt_total_supply_micro_unchanged_across_mint_redeem` in `tests/tb_13_complete_set.rs:260` explicitly asserts this by calling `assert_total_ctf_conserved` pre- and post- a mint-then-redeem sequence.
*   **Finding**: All three new transaction types perform balanced transfers between ledgers that are correctly included in the 6-holding total supply calculation. The invariant holds.
*   **Verdict**: **PASS**.

**5. Does `assert_complete_set_balanced` (MIN-semantics: `min(Σ_yes, Σ_no) == collateral`) hold after every transition?**

*   **Verification**:
    *   `CompleteSetMintTx` / `MarketSeedTx`: Credit `collateral` by `N`, and credit `Σ_yes` and `Σ_no` by `N`. Post-state: `collateral = Σ_yes = Σ_no`, so `min(Σ_yes, Σ_no) == collateral`.
    *   `CompleteSetRedeemTx`: Assume `outcome=Yes`. Debit `collateral` by `M` and debit `Σ_yes` by `M`. `Σ_no` is unchanged. Post-state: `collateral' = collateral - M`, `Σ_yes' = Σ_yes - M`, `Σ_no' = Σ_no`. Since `collateral = Σ_yes` pre-redeem, `collateral' = Σ_yes'`. `Σ_no'` is now greater than `collateral'`. Thus, `min(Σ_yes', Σ_no') = Σ_yes' = collateral'`. The MIN-semantic correctly handles the asymmetry of the losing side's stranded shares.
*   The recursive self-audit correctly identifies the necessity of the MIN-semantic, and integration tests (`halt_total_supply_micro_unchanged_across_mint_redeem`, `halt_complete_set_balanced_post_seed`) confirm this invariant holds post-transition.
*   **Finding**: The logic in all three dispatch arms correctly maintains the `assert_complete_set_balanced` invariant under its MIN-semantic definition.
*   **Verdict**: **PASS**.

**6. Can MarketSeedTx create liquidity without provider balance? (Must be rejected with `InsufficientBalanceForMint` or `InsufficientCollateral`.)**

*   **Verification**: The `MarketSeedTx` dispatch arm in `src/state/sequencer.rs:2051-2059` has two gates:
    1.  `if seed.collateral_amount.micro_units() == 0 { return Err(TransitionError::InsufficientCollateral); }` rejects zero-collateral seeds.
    2.  `if provider_bal.micro_units() < seed.collateral_amount.micro_units() { return Err(TransitionError::InsufficientBalanceForMint); }` rejects seeds where the provider's balance is insufficient.
*   **Finding**: Both required gates are present and correctly implemented.
*   **Verdict**: **PASS**.

**7. Are conditional shares anywhere counted as Coin? (Must be excluded — CR-13.3 + SG-13.2.)**

*   **Verification**: The `total_supply_micro` function in `src/economy/monetary_invariant.rs:159-179` sums the values from `balances_t`, `escrows_t`, `stakes_t`, `challenge_cases_t`, and `conditional_collateral_t`.
*   The doc-comment at `src/economy/monetary_invariant.rs:173` explicitly states: `conditional_share_balances_t is INTENTIONALLY OMITTED per CR-13.3 + SG-13.2`. The code follows this comment.
*   **Finding**: The implementation correctly treats conditional shares as claims, not Coin, by excluding `conditional_share_balances_t` from the total supply calculation.
*   **Verdict**: **PASS**.

**8. Could a malformed `ShareAmount` underflow at redeem? (`u128` type guarantee + `RedeemMoreThanOwned` gate.)**

*   **Verification**:
    1.  The `ShareAmount` struct in `src/state/typed_tx.rs:1034` defines its `units` field as `u128`. This provides a type-level guarantee against negative share balances.
    2.  The `CompleteSetRedeemTx` dispatch arm in `src/state/sequencer.rs:1990-1996` checks `if owned_units < redeem.share_amount.units { return Err(TransitionError::RedeemMoreThanOwned); }` *before* performing the subtraction at `src/state/sequencer.rs:2014`.
*   **Finding**: The combination of the `u128` type and the pre-subtraction ownership check makes underflow impossible.
*   **Verdict**: **PASS**.

**9. Forward-fence: does any new TB-13 module file import legacy `prediction_market`?**

*   **Verification**: The recursive self-audit cites `tests/tb_13_legacy_cpmm_forward_fence.rs` as the ship gate for this. The test's described logic—identifying TB-13 code spans via authoring markers and running a forbidden-token grep—is a robust mechanism for enforcing this fence. A manual review of the provided source code for `typed_tx.rs`, `q_state.rs`, `sequencer.rs`, and `monetary_invariant.rs` confirms the absence of any imports or references to `prediction_market`, `BinaryMarket`, or `f64`.
*   **Finding**: The forward-fence is correctly implemented and verified by an automated ship gate.
*   **Verdict**: **PASS**.

---

### §2 Architectural Strategic Questions (Q10-Q13)

These questions assess the design's robustness and extensibility. Findings here are less likely to be VETO-level unless they represent a fundamental, unrecoverable design flaw.

**10. Does CompleteSet schema extend cleanly to TB-14 PriceIndex (long/short interest derivable from `conditional_share_balances_t` aggregates)?**

*   **Scrutiny**: The schema for share balances is `ConditionalShareBalances(BTreeMap<AgentId, BTreeMap<EventId, ShareSidePair>>)` in `src/state/q_state.rs:563`. To derive total YES/NO depth for a given `EventId`, one must iterate over all `AgentId` entries and sum the `pair.yes.units` and `pair.no.units` for that `EventId`.
*   **Finding**: While requiring a full table scan over agents, this is a standard aggregation pattern for a derived view. The data is present and structured to allow for this derivation. The schema is clean and sufficient for TB-14's needs.
*   **Verdict**: **PASS**.

**11. Does the `EventId == TaskId` 1:1 simplification hold up under TB-14+ multi-event-per-task scenarios?**

*   **Scrutiny**: The schema in `src/state/typed_tx.rs:1021` is `pub struct EventId(pub TaskId);`. This is a newtype wrapper.
*   **Finding**: This is a strong, forward-compatible design choice. If a future TB requires a more complex event identifier (e.g., `(TaskId, NodeId)`), the `EventId` struct definition can be changed to `pub struct EventId(pub (TaskId, NodeId));` or similar, without needing to change the function signatures in the sequencer or the keys in the BTreeMaps that use `EventId`. The abstraction isolates the system from this future change.
*   **Verdict**: **PASS**.

**12. Is the `ResolutionRef` model robust to multi-resolver scenarios in TB-15+?**

*   **Scrutiny**: The `ResolutionRef` in `src/state/typed_tx.rs:1044` points to a single `resolution_tx_id`. The sequencer logic at `src/state/sequencer.rs:1964` checks if this `tx_id` corresponds to a `TaskBankruptcyTx` or a `FinalizeRewardTx`.
*   **Finding**: The current model assumes a single, canonical resolution transaction for an event. It is not robust to scenarios with multiple valid-but-competing resolutions or partial resolutions. However, this is an explicit simplification for TB-13. The model is additive; extending the `match` in the sequencer to handle new resolution transaction types in TB-15 is straightforward. The risk is that the *concept* of a single `resolution_tx_id` may be insufficient, but this is a TB-15 design problem, not a TB-13 implementation flaw.
*   **Verdict**: **CHALLENGE**. This is not a blocker for TB-13, but the architectural roadmap for TB-15 must explicitly address how `ResolutionRef` will evolve to handle more complex resolution scenarios. The current schema correctly implements the narrow TB-13 mandate.

**13. Is the MIN-semantics `assert_complete_set_balanced` invariant the right form (vs. strict equality), particularly for adversarial patterns?**

*   **Scrutiny**: I traced the following sequence:
    1.  **Mint**: Alice mints 100 Coin. State: `collateral=100`, `Σ_yes=100`, `Σ_no=100`. `min(100, 100) == 100`. Invariant holds.
    2.  **Resolve**: Event resolves YES.
    3.  **Partial Redeem**: Alice redeems 30 YES shares. State: `collateral=70`, `Σ_yes=70`, `Σ_no=100`. `min(70, 100) == 70`. Invariant holds. Strict equality (`Σ_yes == Σ_no == collateral`) would have failed here.
    4.  **Re-Mint (Adversarial)**: Alice mints another 20 Coin *after* partial redemption. State: `collateral=70+20=90`, `Σ_yes=70+20=90`, `Σ_no=100+20=120`. `min(90, 120) == 90`. The invariant holds.
*   **Finding**: The MIN-semantic is not a weaker form of the invariant; it is the *correct* form. It correctly asserts that the amount of collateral available is always equal to the total number of outstanding shares on the *winning* (or potentially winning) side. The losing side's shares become unbacked claims, and the invariant correctly ignores their surplus. This is robust.
*   **Verdict**: **PASS**.

---

### §3 Final Verdict

The TB-13 implementation is a disciplined and correct execution of the architect's narrow mandate. It successfully lays the foundational substrate for conditional shares without overstepping into forbidden territory (trading, pricing, AMMs). All mandated safety and conservation invariants (Q1-Q9) are robustly implemented and verified. The architectural choices for future extensibility (Q10-Q13) are sound, with a minor, non-blocking challenge noted for future roadmap planning. The code is clean, the logic follows the specification, and the self-audit appears accurate.

- **VERDICT**: **PASS**
- **Conviction**: **high**
- **Recommendation**: **PROCEED to SHIP**