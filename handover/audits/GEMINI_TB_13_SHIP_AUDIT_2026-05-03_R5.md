# Gemini TB-13 Ship Audit — CompleteSet + MarketSeedTx (Class 3)
**Round**: R5
**Date**: 2026-05-03
**Test baseline**: cargo test --workspace = 792 PASS / 0 FAILED / 150 ignored
**Recursive self-audit**: PASS (5 clauses + 12 SG + 11 G + 0/7 halts)
**Round-5 closures**: RQ3 + Q9/RQ6 + RQ5 (R3 challenges).
**Round-6 closures**: Q9/RQ6 Layer 2 + RQ3 direct map equality (R4 challenges).
**Elapsed**: 46.2s
**Prompt size**: 900,191 chars
**Audit mode**: Class 3 dual (Codex impl-paranoid + Gemini architectural strategic; per architect Part A §4.8)

---

An audit of the TuringOS v4 TB-13 implementation follows.

---

### **Executive Summary**

This round-5 audit re-evaluates TB-13 after the `d3473bb` commit, which addresses two substantive challenges raised by Codex in the previous round. My review finds that both fixes are not only correct but represent significant structural improvements that harden the implementation and its evidentiary basis.

1.  The new `tb_13_scan_lines()` helper for the forward-fence (Q9/RQ6) correctly closes a loophole where an unmarked file using TB-13 types could have bypassed Layer 2 scans. The logic is sound, providing a robust fallback that does not rely on developer discipline with authoring markers.
2.  The new `manual_replay_from_disk()` test helper (RQ3) provides direct, byte-for-byte proof of map equality for the new economic state fields. This replaces a weaker, inferential proof based on state-root matching and provides airtight evidence of replay determinism for non-empty TB-13 state.

All architect-mandated implementation questions (Q1-Q9) continue to pass, with some gates now backed by stronger evidence. My primary architectural concern from round-3 (Q12, redeem-mechanism coupling) was fully resolved by the round-5 `ResolutionRef` removal, which has resulted in a cleaner and more extensible design. The project is in a robust state for shipment.

---

### **Round-6-Specific Risks to Scrutinize**

#### **Q9/RQ6 (R4) Layer 2 fix**

**Finding**: PASS. The new `tb_13_scan_lines()` helper at `tests/tb_13_legacy_cpmm_forward_fence.rs:165..183` is robust.

-   **Partially-marked file classification**: The conditional `source.lines().any(is_tb_13_authoring_marker)` is the correct test under the project's trace-matrix discipline. If a file contains *any* TB-13 authoring marker, it is classified as a "marker-file," and only its explicitly marked TB-13 spans are scanned by the Layer 2 forbidden-token list. Unmarked code within that same file is, by definition, not TB-13 code and is correctly excluded from the Layer 2 scan. The most critical security boundary—the Layer 1 `HARD_BANNED_LEGACY_IMPORTS` check—still runs on the entire file, preventing any legacy `use` statements regardless of marker placement. This logic is sound.
-   **Removal of `src/bin/audit_dashboard.rs` from `FENCE_SCOPE_FLOOR`**: This is safe and correct. The file was removed because the new Layer 2 logic (scanning all non-comment lines of unmarked files) would have caused a false positive on string literals inside the dashboard's own test fixtures. The file currently has no TB-13 markers or type uses. If it is extended in the future to include TB-13 contributions, it will be automatically re-added to the fence's scope by either `discover_by_marker()` (if a marker is added) or `discover_by_type_use()` (if it uses a TB-13 type). The auto-discovery mechanism correctly justifies its removal from the static floor list.

#### **RQ3 (R4) direct map-equality fix**

**Finding**: PASS. The evidence provided by `manual_replay_from_disk()` in `tests/tb_13_chaintape_smoke.rs` is airtight.

-   **(a) Full sub-field reconstruction**: The test asserts full equality on `economic_state_t` (`replayed_q.economic_state_t == live_q.economic_state_t`). As `EconomicState` at `src/state/q_state.rs:158` is a struct containing all 13 economic sub-fields, a direct `==` comparison constitutes a byte-for-byte equality check on all of them, not just the TB-13 ones. This is a complete and direct proof.
-   **(b) Non-determinism**: The state path correctly uses `BTreeMap` for all map types (`src/state/q_state.rs`), which has a deterministic iteration order sorted by key. Other types in the state path (`MicroCoin`, `Hash`, etc.) are also deterministic. There are no sources of non-determinism (like `HashMap` iteration) that would invalidate the byte-equal comparison.
-   **(c) Faithful mirror of `verify_chaintape`**: The manual replay helper faithfully reconstructs the exact environment that `verify_chaintape` uses: it opens the `Git2LedgerWriter`, loads `initial_q_state.json`, decodes `pinned_pubkeys.json`, opens the `CasStore`, and calls the public `replay_full_transition` API at `src/bottom_white/ledger/transition_ledger.rs:430`. This is not a mock; it is a direct invocation of the same replay engine against the same on-disk artifacts. The evidence is therefore direct and conclusive.

#### **R-022 skip-token continuity**

**Finding**: PASS. The round-6 commit `d3473bb` introduces no new public symbol removals. The only relevant removal was `ResolutionRef` in the prior round-5 commit, which is correctly justified by the skip-token at `handover/alignment/OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md`.

---

### **Architect Part A §4 + charter §3 Atom 6 mandated audit questions (Q1-Q9)**

1.  **Does CompleteSetMint create or destroy money?**
    **Finding**: PASS. No. The logic at `src/state/sequencer.rs:1808` remains a pure balance↔collateral migration. The 6-holding sum in `src/economy/monetary_invariant.rs:158` correctly includes `conditional_collateral_t`, ensuring `assert_total_ctf_conserved` passes.

2.  **Can Redeem fire without a system-emitted resolution?**
    **Finding**: PASS. No. The gate at `src/state/sequencer.rs:1882` checks `task_markets_t` for a resolved state (`Finalized` or `Bankrupt`), correctly rejecting with `RedeemBeforeResolution` otherwise.

3.  **Can Redeem with `outcome=Yes` against a Bankrupt event bypass the outcome check?**
    **Finding**: PASS. No. The round-5 removal of `ResolutionRef` simplified and hardened this gate. The `match` arm at `src/state/sequencer.rs:1889` now directly compares `(market_state, redeem.outcome)`. A `(TaskMarketState::Bankrupt, OutcomeSide::Yes)` tuple correctly falls through to the `InvalidResolutionRef` rejection path.

4.  **Does the 6-holding `total_supply_micro` sum hold across all TB-13 typed_tx?**
    **Finding**: PASS. Yes. All three new transaction types represent balanced transfers between coin holdings (balances ↔ collateral). The 6-holding sum in `src/economy/monetary_invariant.rs:158` is correct.

5.  **Does `assert_complete_set_balanced` (MIN-semantics) hold after every transition?**
    **Finding**: PASS. Yes. The MIN-semantics implementation at `src/economy/monetary_invariant.rs:289` is the correct form. Per the round-3 remediation, this assertion is now called live from all three TB-13 dispatch arms in `src/state/sequencer.rs`, ensuring it is not just a test-time check.

6.  **Can MarketSeedTx create liquidity without provider balance?**
    **Finding**: PASS. No. The check at `src/state/sequencer.rs:1959` correctly rejects with `InsufficientBalanceForMint`.

7.  **Are conditional shares anywhere counted as Coin?**
    **Finding**: PASS. No. `total_supply_micro` at `src/economy/monetary_invariant.rs:158` correctly excludes `conditional_share_balances_t` from the sum, per architect CR-13.3.

8.  **Could a malformed `ShareAmount` underflow at redeem?**
    **Finding**: PASS. No. `ShareAmount` at `src/state/typed_tx.rs:1105` is a `u128` newtype. The `RedeemMoreThanOwned` check at `src/state/sequencer.rs:1907` prevents subtraction that would underflow.

9.  **Forward-fence: does any new TB-13 module file import legacy `prediction_market`?**
    **Finding**: PASS. No. The fence at `tests/tb_13_legacy_cpmm_forward_fence.rs` is now stronger. The addition of `discover_by_type_use` provides a robust secondary check that does not rely on developer discipline with authoring markers, significantly reducing the bypass surface.

---

### **Architectural Strategic Questions (Class 3 review)**

10. **Does CompleteSet schema extend cleanly to TB-14 PriceIndex?**
    **Finding**: PASS. Yes. The `conditional_share_balances_t` schema at `src/state/q_state.rs:511` allows for aggregation of total YES/NO depth per `EventId`. While iterating over all agents might be inefficient for markets with a very large number of participants, it is a correct and sufficient data structure for deriving a price index as a read-only view.

11. **Does the `EventId == TaskId` 1:1 simplification hold up under TB-14+?**
    **Finding**: PASS. Yes. The implementation of `EventId` as a newtype wrapper around `TaskId` (`src/state/typed_tx.rs:1088`) is the correct architectural pattern. It provides type safety within TB-13 while making it straightforward to change the inner representation of `EventId` in the future without breaking function signatures.

12. **With ResolutionRef now removed (round-5 RQ5), is the redeem-mechanism-coupling concern from your round-3 Q12 CHALLENGE addressed?**
    **Finding**: PASS (Fully Addressed). Yes. My previous challenge noted that coupling redemption to specific transaction types via `ResolutionRef` was brittle. By removing `ResolutionRef` and having the `CompleteSetRedeemTx` dispatch arm at `src/state/sequencer.rs:1882` check the canonical `task_markets_t.state` enum directly, the design is now much more extensible and robust. If a future TB introduces a new resolution mechanism, it only needs to mutate `task_markets_t.state` to a resolved value; the `CompleteSetRedeemTx` logic will not need to be changed. This is a significant architectural improvement that makes future evolution easier.

13. **Is the MIN-semantics `assert_complete_set_balanced` invariant the right form?**
    **Finding**: PASS. Yes. The MIN-semantics form is correct, as it properly accounts for the post-resolution state where losing-side shares are stranded. Adversarial sequences like "redeem-and-remint" are blocked by the `EventNotOpen` gate added in a previous round (`src/state/sequencer.rs:1823`), which prevents minting into a resolved event. The invariant appears robust.

---

### **Final Verdict**

-   **VERDICT**: **PASS**
-   **Conviction**: **High**
-   **Recommendation**: **PROCEED to SHIP**

**Rationale**: The TB-13 implementation is in an excellent state. The round-6 fixes have not only closed the specific challenges raised by Codex but have resulted in a simpler, more robust, and more extensible design. The addition of direct, cryptographic proof for non-empty replay determinism provides strong assurance. My primary architectural concern from the previous round has been fully resolved by the removal of `ResolutionRef`. There are no remaining VETO or CHALLENGE-level issues.