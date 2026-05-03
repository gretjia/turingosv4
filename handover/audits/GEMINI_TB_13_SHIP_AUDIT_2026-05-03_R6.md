# Gemini TB-13 Ship Audit — CompleteSet + MarketSeedTx (Class 3)
**Round**: R6
**Date**: 2026-05-03
**Test baseline**: cargo test --workspace = 794 PASS / 0 FAILED / 150 ignored
**Recursive self-audit**: PASS (5 clauses + 12 SG + 11 G + 0/7 halts)
**Round-5 closures**: RQ3 + Q9/RQ6 + RQ5 (R3 challenges).
**Round-6 closures**: Q9/RQ6 Layer 2 + RQ3 direct map equality (R4 challenges).
**Round-7 closures**: PARTIAL-MARKER + DASHBOARD-FLOOR (R5 challenges).
**Elapsed**: 36.5s
**Prompt size**: 917,032 chars
**Audit mode**: Class 3 dual (Codex impl-paranoid + Gemini architectural strategic; per architect Part A §4.8)

---

An audit of the TuringOS v4 TB-13 implementation follows.

---

### **Executive Summary**

The `8efffa8` commit correctly and robustly addresses both challenges raised by Codex in the previous round. The fixes are not mere patches but thoughtful architectural adjustments that demonstrate a clear understanding of the attack vectors.

The `PARTIAL-MARKER` fix for the forward-fence significantly hardens the scanner by adding a type-use heuristic, closing a subtle but real loophole. The acknowledged residual gap is a reasonable tradeoff against the risk of false positives and represents a much smaller attack surface than the one just closed.

The `DASHBOARD-FLOOR` fix is an elegant two-tier solution that correctly separates the concerns of critical import-banning (Layer 1) from concept-banning (Layer 2). It restores full Layer 1 protection to `audit_dashboard.rs` while correctly deferring Layer 2 scanning until that file actually contributes TB-13 code, at which point it will be automatically discovered.

My previous PASS verdicts on the core economic and architectural questions (Q1-Q13) stand, and are in fact reinforced by the increased robustness of the forward-fence (Q9). The implementation is sound, the invariants are correctly enforced, and the latest fixes are airtight. There are no remaining blockers to shipment.

---

### **Round-7-specific risks to scrutinize**

#### **PARTIAL-MARKER fix**

**Finding**: PASS. The rewritten `tb_13_scan_lines()` at `tests/tb_13_legacy_cpmm_forward_fence.rs` is a robust and well-reasoned closure for the attack vector raised by Codex.

-   **(a) Closure of the attack**: The new logic—returning marker-spans UNION non-comment lines containing TB-13 type names—is the correct closure. An attacker can no longer hide forbidden tokens (e.g., `f64`) in an unmarked span of a marker-bearing file if that span also uses a TB-13 type. The new test `tb_13_scan_lines_partial_marker_catches_stealth_type_use` correctly verifies this.
-   **(b) Acceptability of residual gap**: The prompt correctly identifies the residual gap: a line containing a forbidden token (like `f64`) that is in an unmarked span AND does not use a TB-13 type name would not be scanned by Layer 2 in a marker-bearing file. This is an acceptable tradeoff. The alternative—scanning all non-comment lines in any file with a marker—would re-introduce the false-positive problem on unrelated code or test fixtures. The current fix narrows the attack surface to a highly contrived scenario that is easily caught by manual code review, which the team's code comment correctly identifies as the fallback guard. The risk reduction is substantial.

#### **DASHBOARD-FLOOR two-tier scope**

**Finding**: PASS. The two-tier scope split is an excellent architectural solution.

-   **(a) Layer 1 enforcement**: By restoring `src/bin/audit_dashboard.rs` to `FENCE_SCOPE_FLOOR` and using the broader `effective_fence_scope()` for the Layer 1 check, the fix correctly ensures that hard-banned legacy imports (`use crate::prediction_market::*`) are once again forbidden in that file. This restores the primary security guarantee that was lost in the previous round.
-   **(b) Layer 2 tradeoff**: The decision to narrow the Layer 2 scope to discovered-only files (`effective_layer_2_scope()`) is the correct tradeoff. It correctly prevents the false positive on the dashboard's negative-list test fixture. The risk of a contributor sneaking `f64` into `audit_dashboard.rs` is mitigated by the fact that as soon as that file uses any TB-13 type or gains a TB-13 authoring marker (as planned for TB-14), it will be automatically "discovered" and pulled into the Layer 2 scope. The new test `audit_dashboard_in_layer_1_scope_but_not_layer_2_scope` correctly asserts this behavior.

#### **Round-7 unit-test coverage**

**Finding**: PASS. The two new tests are sufficient and targeted.
-   `tb_13_scan_lines_partial_marker_catches_stealth_type_use` directly and correctly exercises the exact scenario described in the PARTIAL-MARKER challenge.
-   `audit_dashboard_in_layer_1_scope_but_not_layer_2_scope` directly and correctly asserts the shape of the new two-tier scope.

The tests cover the new logic completely. No other adversarial patterns are immediately apparent that would not be caught by the existing, broader fence tests.

---

### **Architect Part A §4 + charter §3 Atom 6 mandated audit questions (Q1-Q9)**

My PASS verdicts from round-5 on these questions are unchanged. The `8efffa8` commit is orthogonal to the core economic logic and only serves to strengthen the Q9 forward-fence guarantees.

1.  **Does CompleteSetMint create or destroy money?**
    **Finding**: **PASS**. No. The logic remains a pure balance↔collateral migration. `assert_total_ctf_conserved` correctly includes `conditional_collateral_t` in the 6-holding sum.

2.  **Can Redeem fire without a system-emitted resolution?**
    **Finding**: **PASS**. No. The gate at `src/state/sequencer.rs` correctly checks `task_markets_t` for a resolved state (`Finalized` or `Bankrupt`), rejecting with `RedeemBeforeResolution` otherwise.

3.  **Can Redeem with `outcome=Yes` against a Bankrupt event bypass the outcome check?**
    **Finding**: **PASS**. No. The `match` arm at `src/state/sequencer.rs` directly compares `(market_state, redeem.outcome)` and correctly rejects mismatches with `InvalidResolutionRef`.

4.  **Does the 6-holding `total_supply_micro` sum hold across all TB-13 typed_tx?**
    **Finding**: **PASS**. Yes. All three new transaction types represent balanced transfers between coin holdings.

5.  **Does `assert_complete_set_balanced` (MIN-semantics) hold after every transition?**
    **Finding**: **PASS**. Yes. The MIN-semantics form is correct, and the assertion is called live from all three TB-13 dispatch arms.

6.  **Can MarketSeedTx create liquidity without provider balance?**
    **Finding**: **PASS**. No. The check at `src/state/sequencer.rs` correctly rejects with `InsufficientBalanceForMint`.

7.  **Are conditional shares anywhere counted as Coin?**
    **Finding**: **PASS**. No. `total_supply_micro` correctly excludes `conditional_share_balances_t`.

8.  **Could a malformed `ShareAmount` underflow at redeem?**
    **Finding**: **PASS**. No. The `u128` type and `RedeemMoreThanOwned` check prevent this.

9.  **Forward-fence: does any new TB-13 module file import legacy `prediction_market`?**
    **Finding**: **PASS**. No. The fence is now significantly stronger than when I last passed it. The two-tier scope and the type-use heuristic in `tb_13_scan_lines()` at `tests/tb_13_legacy_cpmm_forward_fence.rs` make the bypass surface extremely small and the guarantees robust.

---

### **Architectural Strategic Questions (Class 3 review)**

My PASS verdicts from round-5 on these questions are also unchanged. The fixes in `8efffa8` do not alter the strategic implications of the TB-13 design.

10. **Does CompleteSet schema extend cleanly to TB-14 PriceIndex?**
    **Finding**: **PASS**. Yes. The `conditional_share_balances_t` schema is sufficient for deriving long/short interest aggregates.

11. **Does the `EventId == TaskId` 1:1 simplification hold up under TB-14+?**
    **Finding**: **PASS**. Yes. The `EventId(pub TaskId)` newtype is the correct pattern for providing type safety now while allowing future evolution.

12. **With ResolutionRef now removed (round-5 RQ5), is the redeem-mechanism-coupling concern from your round-3 Q12 CHALLENGE addressed?**
    **Finding**: **PASS (Addressed)**. Yes. As noted in my R5 audit, the removal of `ResolutionRef` and the direct check against `task_markets_t.state` fully resolved my previous architectural challenge, making the design more extensible.

13. **Is the MIN-semantics `assert_complete_set_balanced` invariant the right form?**
    **Finding**: **PASS**. Yes. The MIN form is correct for handling post-resolution states, and adversarial minting is blocked by the `EventNotOpen` gate.

---

### **Final Verdict**

-   **VERDICT**: **PASS**
-   **Conviction**: **High**
-   **Recommendation**: **PROCEED to SHIP**

**Rationale**: The implementation has successfully passed multiple rounds of intense adversarial scrutiny from two independent auditors. The final set of fixes in commit `8efffa8` are not only correct but demonstrate a mature and thoughtful approach to resolving subtle security and architectural issues. The forward-fence is now robust, the core economic logic is sound, and there are no outstanding VETO or CHALLENGE-level issues.