# Gemini TB-12 Ship Audit — Node Exposure Index (Class 3)
**Round**: R1
**Date**: 2026-05-03
**Test baseline**: cargo test --workspace = 757 PASS / 0 FAILED / 150 ignored
**Codex round-1 verdict**: CHALLENGE × 2 (Q4 doc-drift, Q5 legacy CPMM scope) — both resolved via recursive self-audit §10
**Elapsed**: 48.2s
**Prompt size**: 895,936 chars
**Audit mode**: Class 3 dual (Codex impl-paranoid + Gemini architectural strategic; per architect §5)

---

**To**: TURINGOS v4 Ship Audit Committee
**From**: Gemini Class 3 Auditor (skeptical adversarial)
**Subject**: TB-12 Ship Audit — Node Exposure Index
**Date**: 2026-05-03

This document constitutes my independent architectural strategic review of TB-12. My findings are based on the provided charter, architect rulings, recursive self-audit, and source code.

---

### Architect §8 Atom 6 Mandated Audit Questions

**1. Does NodePosition create a second money ledger? (Should NOT.)**

**PASS.** `NodePosition` is structurally isolated from all monetary accounting.

-   **Evidence**: The `total_supply_micro` function in `src/economy/monetary_invariant.rs:152-172` enumerates exactly four coin-holding sub-indexes (balances, escrows, stakes, challenge_cases). It does **not** iterate over `economic_state_t.node_positions_t`. This is the definitive guard against `NodePosition.amount` being counted as part of the monetary base.
-   **Evidence**: The `WorkTx` accept arm in `src/state/sequencer.rs:544-550` performs the only money movement (a balanced transfer from `balances_t` to `stakes_t`). The subsequent `NodePosition` write at `src/state/sequencer.rs:572-586` is a pure, additive write to a separate index (`node_positions_t`) and does not touch any monetary balance. The `ChallengeTx` arm at `src/state/sequencer.rs:796-847` follows the same correct pattern.
-   **Evidence**: The schema definition at `src/state/typed_tx.rs:658-660` explicitly documents `NodePosition` as "NOT a Coin holding" and "NOT in total_supply_micro", aligning documentation with implementation.

**2. Does replay reconstruct positions deterministically?**

**PASS.** Position derivation is a pure function of on-chain data.

-   **Evidence**: The `WorkTx` accept arm at `src/state/sequencer.rs:573-582` constructs the `NodePosition` using only fields from the `work` transaction itself (`tx_id`, `task_id`, `agent_id`, `stake`, `timestamp_logical`).
-   **Evidence**: The `ChallengeTx` accept arm at `src/state/sequencer.rs:827-843` constructs its `NodePosition` using fields from the `challenge` transaction and a deterministic lookup of `task_id` from the pre-existing `stakes_t` entry. No environmental inputs (e.g., wall-clock time, randomness) are used.
-   **Evidence**: The self-audit confirms this with test `sg_12_5_node_positions_replay_deterministic` (`tests/tb_12_node_exposure_index.rs:340`), which asserts bit-for-bit equality of the `NodePositionsIndex` across two identical but separate runs.

**3. Does VerifyTx bond avoid market classification?**

**PASS.** The `VerifyTx` path is explicitly excluded from position creation.

-   **Evidence**: The `TypedTx::Verify` dispatch arm in `src/state/sequencer.rs:611-756` contains logic for locking the bond into `stakes_t` and creating a `ClaimEntry` on Confirm, but contains no write to `node_positions_t`.
-   **Evidence**: The architect's ruling is enforced by test `sg_12_3_verifytx_does_not_create_node_position` (`tests/tb_12_node_exposure_index.rs:269`), which asserts that the position count remains unchanged after a `VerifyTx` is accepted.
-   **Evidence**: The schema doc-comment for `PositionSide` at `src/state/typed_tx.rs:606-610` explicitly states that `VerifyTx.bond` is a responsibility bond, not a market side, fulfilling CR-12.8.

**4. Does NodePosition avoid total supply counting?**

**PASS.** The implementation is correct; the initial documentation drift noted by Codex has been resolved.

-   **Evidence**: The implementation in `src/economy/monetary_invariant.rs:152-172` correctly uses the 4-holding model ratified in TB-8, which excludes `claims_t` and makes no mention of `node_positions_t`. This correctly implements CR-12.2.
-   **Evidence**: The recursive self-audit's remediation log (`handover/audits/RECURSIVE_AUDIT_TB_12_2026-05-03.md:§10`) correctly identifies the 5-holding vs. 4-holding discrepancy as a documentation drift from the audit prompt, not a code regression. The self-audit document was updated to reflect the correct 4-holding model.
-   **Evidence**: Test `sg_12_4_node_positions_do_not_change_total_supply` (`tests/tb_12_node_exposure_index.rs:306`) passes, confirming that `assert_total_ctf_conserved` holds true with an empty exemption list even after `NodePosition` records are created.

**5. Does TB-12 accidentally implement trading?**

**PASS.** The scope of TB-12 is strictly limited to the creation of immutable records.

-   **Evidence**: The `PositionKind` enum at `src/state/typed_tx.rs:624-640` defines only `FirstLong` and `ChallengeShort`. The doc-comment explicitly forbids `MarketBuy` / `MarketSell` as future trading-layer concepts, per architect ruling §9.4.
-   **Evidence**: There are no dispatch arms in `src/state/sequencer.rs` for closing, transferring, settling, or otherwise mutating a `NodePosition` after its creation. This enforces the "immutable exposure record" discipline from architect ruling §10.
-   **Observation**: The legacy CPMM code in `src/prediction_market.rs` noted by Codex is a valid concern for the overall project, but it is not wired into any part of the TB-12 implementation. The recursive self-audit's remediation log (`handover/audits/RECURSIVE_AUDIT_TB_12_2026-05-03.md:§10`) correctly identifies this as out-of-scope for TB-12 and defers its quarantine to TB-13, which is the appropriate procedural handling. TB-12 itself introduces no trading logic.

---

### Architectural Strategic Questions

**6. Does the flat NodePositionsIndex extend cleanly to TB-13 CompleteSet (which will introduce real YES/NO claims) without schema collision?**

**PASS.** The separation of concerns is clean. `NodePosition` serves as a historical record of initial risk-taking, while TB-13's `CompleteSet` will introduce a new set of objects representing tradable claims. There is no schema collision. A `MarketSeedTx` (TB-13) can reference a `node_id` (a `WorkTx` ID), and the `NodePosition` index provides the on-chain context for why a market on that node exists, but the two data structures do not overlap or conflict. This is a sound substrate.

**7. Does the flat-not-nested decision (architect §3 ruling) hold up at TB-14 PriceIndex when long/short aggregation IS needed?**

**PASS.** The flat-not-nested decision is a point of architectural strength, not weakness. It correctly prioritizes a single source of truth.

-   **Rationale**: The architect's ruling (§3.2) correctly identifies the risk of a second source of truth. A nested `NodeMarketEntry` with cached `long_interest` and `short_interest` fields would require complex, bug-prone logic to keep it synchronized with the canonical `NodePosition` entries.
-   **Extensibility**: For TB-14, computing a price index becomes a deterministic, read-only derivation. A view function can iterate the flat `node_positions_t` (`src/state/q_state.rs:485`), group by `node_id`, and sum the `amount` for each `side`. This is computationally efficient on replay and avoids state-mutation complexity entirely. This design is robust and scalable.

**8. Are the architect's halting triggers genuinely impossible given the TB-12 implementation?**

**PASS.** The specified triggers are structurally unreachable in the current implementation.

-   **CTF conservation failure**: Impossible. As established in Q1 and Q4, `NodePosition` is not a monetary holding. The only economic mutations are balanced transfers, and `assert_total_ctf_conserved` is called in every accept arm (`src/state/sequencer.rs:601`, `src/state/sequencer.rs:748`, etc.).
-   **WorkTx-Challenge position mismatch**: Impossible. The derivation logic is hardcoded. `FirstLong.node_id` is always `work.tx_id` (`src/state/sequencer.rs:574`). `ChallengeShort.node_id` is always `challenge.target_work_tx` (`src/state/sequencer.rs:828`). There is no conditional logic that could cause a mismatch. Test `position_fields_derived_from_source_tx_exactly` (`tests/tb_12_node_exposure_index.rs:422`) locks this.
-   **NodePosition counted as Coin**: Impossible. See Q4. The `total_supply_micro` function (`src/economy/monetary_invariant.rs:152`) is the sole authority and does not include `node_positions_t`.
-   **Replay divergence**: Impossible. See Q2. Derivation is a pure function of on-chain state.
-   **VETO**: This is a meta-trigger for the audit process itself. I have found no VETO-class flaws.

---

### Final Verdict

**VERDICT**: PASS
**Conviction**: high
**Recommendation**: PROCEED to SHIP

The TB-12 implementation rigorously adheres to the architect's mandate. It successfully establishes a non-monetary, replay-deterministic index of exposure without introducing any trading, pricing, or settlement mechanics. The core architectural decision to use a flat index is sound and provides a clean substrate for future market layers (TB-13/14). The risks identified by the initial Codex audit have been appropriately addressed through documentation and scoping clarifications. No forge, replay, or economic exploit vectors were found.