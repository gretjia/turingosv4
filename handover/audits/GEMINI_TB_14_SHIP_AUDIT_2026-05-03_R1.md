# Gemini TB-14 Atom 6 Ship Audit — production wire-swap (Class 3)
**Round**: R1
**Date**: 2026-05-03
**HEAD**: 38412bf (Atom 6 main 44cd480 + auditor F1 follow-up 38412bf)
**Test baseline**: cargo test --workspace = 821 PASS / 0 FAILED / 150 ignored
**Halt-triggers**: 6/6 GREEN (architect §5.7)
**ChainTape smoke**: chain-backed PASS (handover/evidence/tb_14_chaintape_smoke_2026-05-03/)
**Internal auditor**: CHALLENGE→F1 addressed by 38412bf; F2-F5 ACCEPTED
**Elapsed**: 69.0s
**Prompt size**: 837,647 chars
**Audit mode**: Class 3 dual (Codex impl-paranoid + Gemini architectural strategic; per charter §4)

---

An independent, skeptical architectural strategic review of TB-14 Atom 6 at HEAD `38412bf` has been completed. The findings are as follows.

---

### **CR-14.x Conformance & Halt-Trigger Soundness (Q1-Q9)**

**1. CR-14.1: Price Signal Influence on Predicate Gates**

**Finding**: The price signal does not influence predicate gates. The structural fence implemented in `tests/tb_14_halt_triggers.rs::price_does_not_affect_predicate_result` is sound. It scans the body of `src/state/sequencer.rs` and confirms the absence of any reference to TB-14 price or mask types (`compute_price_index`, `NodeMarketEntry`, etc.). Manual inspection of `src/state/sequencer.rs`, particularly the `dispatch_transition` function (`sequencer.rs:516-1360`), confirms that predicate evaluation and state transition logic depend only on the `QState` and the incoming `TypedTx`, with no access to the derived price view.

**Verdict**: PASS.

**2. CR-14.2: Price Signal Affect on L4 / L4.E Classification**

**Finding**: The price signal does not affect L4 / L4.E classification. The structural fence in `tests/tb_14_halt_triggers.rs::price_does_not_change_l4_decision` is sound. It scans the `use` block of `src/state/sequencer.rs` and confirms that no TB-14 price or mask types are imported. This prevents both direct and indirect influence on the classification logic, which is determined by the `Result` of `dispatch_transition` within `apply_one` (`sequencer.rs:1363-1498`). The wire-swap correctly isolates the new imports to `src/bus.rs` for snapshot creation, preserving the sequencer's price-blindness.

**Verdict**: PASS.

**3. CR-14.3 / SG-14.3: Masked Parents Still in `tape.nodes()`**

**Finding**: Masking is a read-view operation and does not mutate the `ChainTape`. The function signature for `compute_mask_set` (`src/state/price_index.rs:375`) correctly takes an immutable reference `&Tape`. The implementation only calls read methods (`tape.children`) and returns a new `BTreeSet<TxId>`. The test `tests/tb_14_halt_triggers.rs::parent_not_deleted_from_chaintape` provides a direct witness: it confirms a parent is masked and then asserts that `tape.nodes().contains_key("parent")` remains true.

**Verdict**: PASS.

**4. CR-14.4 / SG-14.8: Low-Liquidity Children Cannot Mask Parent**

**Finding**: The implementation correctly prevents low-liquidity children from masking a parent. The code at `src/state/price_index.rs:403-407` explicitly checks `child_entry.liquidity_depth < policy.min_liquidity` and uses `continue` to skip the child if the condition is met. This directly implements the architect's requirement. The ship-gate test `tests/tb_14_mask_set.rs::sg_14_8_low_liquidity_child_cannot_mask_parent` provides a valid witness.

**Verdict**: PASS.

**5. CR-14.5 / SG-14.7 / halt-trigger #6: Open Challenges Block Masking**

**Finding**: The implementation correctly prevents children with open challenges from masking a parent. The logic is two-part and sound:
1.  `src/state/price_index.rs:377-382`: A set of all `target_work_tx` IDs for challenges with `status == ChallengeStatus::Open` is built.
2.  `src/state/price_index.rs:410-412`: Inside the child-iteration loop, a check `open_challenge_targets.contains(&child_tx_id)` correctly skips any child with an open challenge.
The halt-trigger test `unresolved_challenge_blocks_masking` and ship-gate test `sg_14_7_unresolved_challenge_blocks_masking` both provide valid witnesses.

**Verdict**: PASS.

**6. CR-14.6 / Goodhart shield: `NodeMarketEntry` Exposure**

**Finding**: The `NodeMarketEntry` struct (`src/state/price_index.rs:97-109`) and its derivation in `compute_price_index` (`price_index.rs:125-224`) do not expose private predicate content. The struct's 10 fields are purely economic summaries derived from `node_positions_t` and `conditional_share_balances_t`. No field contains or references the `Tape` node's payload or predicate results. The dashboard rendering in `src/bin/audit_dashboard.rs::render_section_14` (`~1500-1570`) respects this boundary, displaying only the economic fields. The Goodhart shield holds.

**Verdict**: PASS.

**7. G-14.11 / charter §5.6: No `f64` in TB-14 Module Surface**

**Finding**: The TB-14 code surface is free of `f64`.
- `src/state/price_index.rs`: Verified by halt-trigger #4 and manual inspection.
- `src/sdk/snapshot.rs`, `src/sdk/actor.rs`, `src/bus.rs`: All legacy `f64` fields and types related to the old CPMM have been excised. The F1 follow-up commit `38412bf` correctly removed the final `f64` from `bus.rs`.
- `src/bin/audit_dashboard.rs`: The §14 render block correctly renders prices as integer-rational strings, with a test (`sg_14_6_dashboard_renders_price_as_integer_rational_never_decimal`) enforcing the "no decimal" rule.

**Position on `evaluator.rs::prompt_balance: f64`**: The scope decision is sound. The `f64` is used at `experiments/minif2f_v4/src/bin/evaluator.rs:1559-1564` purely for rendering a value into the LLM prompt string. This path is outside the core library, does not affect canonical state, and does not participate in any price computation or state transition logic. It respects the spirit of the rule, which is to protect the system's deterministic core from floating-point arithmetic.

**Verdict**: PASS.

**8. Art.0.2 replay determinism: `tb_14_chaintape_smoke.rs` Claim**

**Finding**: The argument by composition is sound. A pure function (`compute_price_index`) operating on byte-identical inputs (`EconomicState` from a live vs. replayed chain) must produce byte-identical outputs.
- `compute_price_index` (`src/state/price_index.rs:125-224`) is verified to be a pure function, using deterministic iteration over `BTreeMap`s and only integer arithmetic.
- `tests/tb_14_chaintape_smoke.rs` provides a direct, end-to-end witness. It verifies that the live and replayed `EconomicState` are byte-equal, then calls `compute_price_index` on both and asserts the resulting `BTreeMap`s are also byte-equal.

**Verdict**: PASS.

**9. Charter §5.6 forbidden list: `git diff a9fbdf3..38412bf`**

**Finding**: The diff shows a systematic *removal* of legacy market, trading, and settlement concepts. `src/prediction_market.rs` is deleted, and all its consumers in `kernel.rs`, `bus.rs`, `sdk/actor.rs`, etc., are either removed or rewired to the new read-only signal/masking mechanism. Zero new instances of market trading, price-based settlement, parent deletion, AMM, DPMM, or price-as-oracle language are introduced.

**Verdict**: PASS.

---

### **Architectural Strategic Questions (Q10-Q13)**

**10. STEP_B Phase 1 deviation (working on main)**

**Finding**: The justification for bypassing worktree isolation is reasonable for this specific change, which was a self-contained wire-swap with a clear specification (the charter). The most critical phase of the STEP_B protocol—the dual audit and merge gate—was preserved. While this deviation should not become a default for more complex or exploratory changes, it does not represent a review-quality or code-quality concern for this atom.

**Verdict**: PASS. This is a process-discipline observation, not a blocker.

**11. Bus.snapshot() empty fallback architectural soundness**

**Finding**: The semantics of returning an empty `price_index` and `mask_set` when the sequencer is not present are sound. This represents a "no signal" state, which is a correct reflection of the available information. Key consumers—`evaluator.rs` (`boltzmann_select_parent_v2`) and `audit_dashboard.rs`—are confirmed to handle this empty state gracefully, treating it as "no candidates" or "no data to display," respectively. The risk of an operator misinterpreting a misconfigured run is an operational concern, not an architectural flaw in the contract. The system degrades gracefully rather than crashing.

**Verdict**: PASS.

**12. F1 follow-up commit atom-cohesion**

**Finding**: The use of a follow-up commit (`38412bf`) to address an audit finding (F1) from the internal auditor is acceptable hygiene. It makes the audit-and-fix cycle explicit in the commit history, which enhances transparency. While amending the original commit is also an option for trivial fixes, a follow-up commit does not compromise the quality of the final code at HEAD and is a valid workflow, especially for a Class 3 change under dual audit.

**Verdict**: PASS.

**13. Dashboard §14 synthesis approach soundness**

**Finding**: The approach is sound. By synthesizing a minimal `EconomicState` from ledger-derived `ExposureRecordRow`s and then calling the canonical `compute_price_index` function (`audit_dashboard.rs:1455-1493`), the dashboard correctly avoids implementing a second, parallel source of truth for price calculation logic. This is architecturally robust. The residual risk—that the dashboard's logic for building the `exposures` vector could drift from the sequencer's canonical state transitions—is a standard maintenance and testing concern for any derived-view tool, not a flaw in this design.

**Verdict**: PASS.

---

### **Final Verdict**

All mandated checks pass with high confidence. The code correctly implements the architect's specification, adheres to all forbidden rules and halt-triggers, and excises the legacy CPMM system cleanly. The strategic and process-related points are noted but do not constitute blockers. The internal auditor's `CHALLENGE` was correctly addressed.

-   **VERDICT**: PASS
-   **Conviction**: high
-   **Recommendation**: PROCEED to SHIP