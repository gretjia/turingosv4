# Gemini TB-13 Ship Audit — CompleteSet + MarketSeedTx (Class 3)
**Round**: R4
**Date**: 2026-05-03
**Test baseline**: cargo test --workspace = 791 PASS / 0 FAILED / 150 ignored
**Recursive self-audit**: PASS (5 clauses + 12 SG + 11 G + 0/7 halts)
**Round-5 closures**: RQ3 (chaintape replay smoke) + Q9/RQ6 (type-use fence) + RQ5 (ResolutionRef wrapper drop)
**Elapsed**: 45.0s
**Prompt size**: 876,637 chars
**Audit mode**: Class 3 dual (Codex impl-paranoid + Gemini architectural strategic; per architect Part A §4.8)

---

An audit of the TuringOS v4 TB-13 implementation follows.

**Role**: Skeptical adversarial reviewer, architectural strategic angle.
**Merge Rule**: VETO > CHALLENGE > PASS.

### Executive Summary

This audit re-evaluates TB-13 against its charter after the closure of three residual challenges from the prior audit round. The fixes are not merely patches but structural improvements that significantly harden the implementation.

1.  The new chaintape smoke test (RQ3) provides cryptographic proof of replay-determinism for the new non-empty economic state maps, directly addressing a key evidence gap.
2.  The forward-fence (RQ6) has been hardened with type-use discovery, making it robust against contributors who might accidentally import legacy APIs without following documentation conventions.
3.  The removal of the `ResolutionRef` wrapper (RQ5) simplifies the `CompleteSetRedeemTx` wire format and, critically, resolves the architectural concern I raised in round-3 regarding future extensibility. The new design, which couples redemption to the canonical `TaskMarketState` enum rather than a specific transaction type, is cleaner and more forward-compatible.

All architect-mandated questions (Q1-Q9) continue to pass, with some gates now having stronger evidence behind them. The architectural questions (Q10-Q13) also resolve cleanly, with the new design demonstrating superior extensibility. The implementation is sound, the invariants are correctly enforced, and the evidence is now robust.

---

### Round-5-Specific Risks Scrutiny

The three fixes introduced in this round have been scrutinized for new risks.

-   **RQ3 smoke determinism**: **PASS**. The new test at `tests/tb_13_chaintape_smoke.rs` correctly proves non-empty replay determinism. The state-root equality argument holds. The test uses `timestamp_logical` for transaction ordering, which is a deterministic counter, not a non-deterministic wall-clock time. The `QState` struct and its sub-fields do not contain any other sources of non-determinism (e.g., random salts) that would invalidate the state-root comparison. The use of `complete_set_mint_accept_state_root` to pre-compute the parent for the subsequent redeem transaction is a sound method for creating a deterministic, multi-step chaintape for testing purposes.

-   **RQ5 wire-format break**: **PASS**. The removal of `ResolutionRef` from `CompleteSetRedeemTx` at `src/state/typed_tx.rs:1178` constitutes a wire-format break. As TB-13 has not shipped and has no production data, this is acceptable and desirable. The accompanying skip-token at `handover/alignment/OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md` provides a correct and sufficient justification, noting that the removed fields were either unused (`resolution_tx_id`) or redundant (`claimed_outcome`). No persisted artifacts encoding the old shape were found in the codebase.

-   **RQ6 false-positives**: **PASS**. The forward-fence at `tests/tb_13_legacy_cpmm_forward_fence.rs` was hardened by adding `discover_by_type_use`. This function now scans for usage of `TB_13_TYPE_NAMES` in non-comment lines, which is a significant improvement over relying solely on authoring markers. The new unit test `discover_by_type_use_catches_unmarked_imports_and_skips_doc_xref` correctly verifies that this mechanism catches unmarked uses while ignoring benign references in documentation. The risk of false positives from common-sounding names like `EventId` is low, as their usage in non-TB-13 code would be a type error, and their declaration is scoped to the new modules.

-   **R-022 skip-token**: **PASS**. The justification in `handover/alignment/OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md` is sound. The `ResolutionRef` struct was correctly identified as carrying dead weight. Its role is fully and more cleanly absorbed by the `outcome` field on `CompleteSetRedeemTx` itself, which is validated directly against the canonical `TaskMarketState`. The TRACE_MATRIX backlink is correctly retired as the symbol it pointed to is gone.

---

### Architect Part A §4 + charter §3 Atom 6 mandated audit questions (Q1-Q9)

1.  **Does CompleteSetMint create or destroy money?**
    **Finding**: **PASS**. No. The logic at `src/state/sequencer.rs:1808` remains a pure balance↔collateral migration. `balances_t` is debited and `conditional_collateral_t` is credited by the same amount. The 6-holding sum defined in `src/economy/monetary_invariant.rs:158` correctly includes `conditional_collateral_t`, ensuring `assert_total_ctf_conserved` passes.

2.  **Can Redeem fire without a system-emitted resolution?**
    **Finding**: **PASS**. No. The gate at `src/state/sequencer.rs:1882` checks `task_markets_t` for a resolved state (`Finalized` or `Bankrupt`). An `Open` or `Expired` state correctly results in a `RedeemBeforeResolution` rejection.

3.  **Can Redeem with `outcome=Yes` against a Bankrupt event bypass the outcome check?**
    **Finding**: **PASS**. No. The `ResolutionRef` wrapper has been removed, simplifying the check. The `match` arm at `src/state/sequencer.rs:1889` now directly compares `(market_state, redeem.outcome)`. A `(TaskMarketState::Bankrupt, OutcomeSide::Yes)` tuple correctly falls through to the `InvalidResolutionRef` rejection path. The logic is sound and more direct than before.

4.  **Does the 6-holding `total_supply_micro` sum hold across all TB-13 typed_tx?**
    **Finding**: **PASS**. Yes. All three new transaction types represent balanced transfers between coin holdings (balances ↔ collateral). The 6-holding sum in `src/economy/monetary_invariant.rs:158` is correct, and the logic is verified by integration tests like `halt_total_supply_micro_unchanged_across_mint_redeem`.

5.  **Does `assert_complete_set_balanced` (MIN-semantics) hold after every transition?**
    **Finding**: **PASS**. Yes. The MIN-semantics implementation at `src/economy/monetary_invariant.rs:289` is the correct form of the invariant, robustly handling post-resolution states where losing-side shares are stranded. The invariant is now called live from all three TB-13 dispatch arms in `src/state/sequencer.rs`, ensuring it is not just a test-time check.

6.  **Can MarketSeedTx create liquidity without provider balance?**
    **Finding**: **PASS**. No. The check at `src/state/sequencer.rs:1959` correctly rejects with `InsufficientBalanceForMint` if the provider's balance is insufficient.

7.  **Are conditional shares anywhere counted as Coin?**
    **Finding**: **PASS**. No. `total_supply_micro` at `src/economy/monetary_invariant.rs:158` correctly excludes `conditional_share_balances_t` from the sum, per architect CR-13.3.

8.  **Could a malformed `ShareAmount` underflow at redeem?**
    **Finding**: **PASS**. No. `ShareAmount` at `src/state/typed_tx.rs:1105` is a `u128` newtype, preventing negative values. The `RedeemMoreThanOwned` check at `src/state/sequencer.rs:1907` prevents subtraction that would underflow.

9.  **Forward-fence: does any new TB-13 module file import legacy `prediction_market`?**
    **Finding**: **PASS**. No. The fence is now stronger. The addition of `discover_by_type_use` at `tests/tb_13_legacy_cpmm_forward_fence.rs:141` provides a robust secondary check that does not rely on developer discipline with authoring markers, significantly reducing the bypass surface.

---

### Architectural Strategic Questions (Class 3 review)

10. **Does CompleteSet schema extend cleanly to TB-14 PriceIndex?**
    **Finding**: **PASS**. Yes. The `conditional_share_balances_t` schema at `src/state/q_state.rs:511` stores share data in a way that allows for aggregation of total YES/NO depth per `EventId`. While iterating over all agents to sum up shares might be inefficient for markets with a very large number of participants, it is a correct and sufficient data structure for deriving a price index as a read-only view.

11. **Does the `EventId == TaskId` 1:1 simplification hold up under TB-14+?**
    **Finding**: **PASS**. Yes. The implementation of `EventId` as a newtype wrapper around `TaskId` (`src/state/typed_tx.rs:1088`) is the correct pattern. It provides type safety within TB-13 while making it straightforward to change the inner representation of `EventId` in the future (e.g., to an enum or a struct for multi-event scenarios) without breaking function signatures that consume it.

12. **With ResolutionRef now removed, is the redeem-mechanism-coupling concern from your round-3 Q12 CHALLENGE addressed?**
    **Finding**: **PASS (Addressed)**. Yes, the concern is fully and elegantly addressed. My previous challenge noted that coupling redemption to specific transaction types via `ResolutionRef` was brittle. By removing `ResolutionRef` and having the `CompleteSetRedeemTx` dispatch arm at `src/state/sequencer.rs:1882` check the canonical `task_markets_t.state` enum directly, the design is now much more extensible. If a future TB introduces a new resolution mechanism (e.g., a `ChallengeCourtRulingTx`), it only needs to mutate `task_markets_t.state` to a resolved value. The `CompleteSetRedeemTx` logic will not need to be changed at all. This is a significant architectural improvement.

13. **Is the MIN-semantics `assert_complete_set_balanced` invariant the right form?**
    **Finding**: **PASS**. Yes. The MIN-semantics form is correct, as it properly accounts for the post-resolution state where losing-side shares are stranded but the winning side remains fully collateralized. Adversarial sequences like "redeem-and-remint" are blocked by the `EventNotOpen` gate added in a previous round (`src/state/sequencer.rs:1823`), which prevents minting into a resolved event. The invariant appears robust.

---

### Final Verdict

-   **VERDICT**: **PASS**
-   **Conviction**: **High**
-   **Recommendation**: **PROCEED to SHIP**

**Rationale**: The TB-13 implementation is now in an excellent state. The round-5 fixes have not only closed the specific challenges raised but have resulted in a simpler, more robust, and more extensible design. The addition of cryptographic proof for non-empty replay determinism provides strong assurance. My primary architectural concern from the previous round has been fully resolved by the removal of `ResolutionRef`. There are no remaining VETO or CHALLENGE-level issues.