# TB-8 Atom 0.5 — Charter Ratification (auto-applied per user directive)

**Date**: 2026-05-02
**Charter**: `handover/tracer_bullets/TB-8_charter_2026-05-02.md`
**Class**: 0 (charter ratification doc; no code; no money)
**Iteration cap**: 24h (cleared in <1h via auto-apply)
**Authority**: user directive 2026-05-02 — "i already told you to meet my minimum requirements, you just fix it and move on" + "finish until TB-8 is finished" → auto-execute on charter §7 defaults; no menu surfaced (per `feedback_no_fake_menus`).
**Precedent**: TB-7R Atom 0.5 ratification pattern (`handover/audits/CHARTER_RATIFICATION_TB_7R_*` shape).

---

## §0 Why this doc exists

Atom 0.5 is the codified-recommendation step for any TB charter that records `open_questions` in §7. The user has authorized auto-execute, so the §7 defaults proposed in the charter are ratified verbatim below — no architect ping required for these specific 5 questions. Each ratification carries a one-line *Why* (the reason the default is the smallest-correct choice), a *How to apply* (where the choice lands in code), and a *Reversibility* note (cost of changing the answer post-Atom-1).

Atom-0.5's *only* purpose is to remove ambiguity *before* Atom 1 begins. If any choice below proves wrong during implementation, the surgical patch goes back through STEP_B (for sequencer.rs touches) or directly through Edit (for q_state.rs / typed_tx.rs touches) rather than an Atom-0.5 v2.

---

## §1 The 5 ratifications

### Q1 — `ClaimEntry` schema extension (Atom 1)

**Ratified**: 6-field expansion verbatim per charter §3 Atom 1:

```rust
pub struct ClaimEntry {
    pub amount: MicroCoin,                         // existing
    pub claimant: AgentId,                         // existing
    pub task_id: TaskId,                           // NEW — escrow lookup back-ref
    pub escrow_lock_tx_id: TxId,                   // NEW — which escrow row to debit
    pub work_tx_id: TxId,                          // NEW — accepted WorkTx
    pub verify_tx_id: TxId,                        // NEW — OMEGA-verdict VerifyTx
    pub status: ClaimStatus,                       // NEW — Open | Finalized
    pub challenge_window_close_logical_t: u64,     // NEW — when finalize becomes legal
}

pub enum ClaimStatus {
    Open,
    Finalized,
    // Slashed reserved — TB-15+ slash territory per directive 2026-05-02 ruling 13.
}
```

**Why**: Each new field is read by the Atom-3 dispatch arm during finalize and cannot be re-derived at finalize time without it (e.g., `escrow_lock_tx_id` is the row to debit; `task_id` keys the escrow's task_market entry; `challenge_window_close_logical_t` is the gate). A "compact + lookup_refs" packed shape would force per-finalize re-traversal of stakes_t / escrows_t / L4 to reconstruct the same set — slower, harder to audit, and prone to ambiguity when multiple WorkTxs share a task. The 6-field expansion is the smallest correct set.

**How to apply**: extend `src/state/q_state.rs:233` `ClaimEntry` + add `pub enum ClaimStatus` next to it. All fields `#[serde(default)]` so historical (TB-3..TB-7R) serialized rows that never wrote a `ClaimEntry` deserialize cleanly. The Atom-1 writer is inserted at the Verify dispatch arm (`src/state/sequencer.rs:500`) when `verify.verdict == VerifyVerdict::Confirm` (= "OMEGA" per current enum; no `Verdict::Omega` variant exists, see §2 below).

**Reversibility**: removing a field later is a Q_t schema break (need ledger replay to validate). Adding a 7th field later is additive-safe via `#[serde(default)]`. Default to "smallest correct" wins.

---

### Q2 — Idempotency error variant naming (Atom 3)

**Ratified**: add new variant `ClaimAlreadyFinalized` to `TransitionError`. Do NOT broaden `ClaimAlreadySlashed` (it does not exist as a variant — the existing variant is `AlreadySlashed`, which has different semantics: "claim was slashed, cannot reward"). Do NOT introduce a parameterized `ClaimAlreadyResolved(ClaimStatus)` (CO1.1.4-pre1 § 7.2 mandate: `TransitionError` keeps primitive-payloads-only — no nested `ClaimStatus` payload).

**Why**: `AlreadySlashed` semantically means "this claim was already settled by the adversarial path" (slash). `ClaimAlreadyFinalized` semantically means "this claim was already settled by the cooperative path" (reward). These two are distinct ledger transitions and distinct rejection classes for L4.E provenance. Conflating them under one variant would lose the reward/slash discriminator that Phase 4 Information Loom needs.

**How to apply**: insert `ClaimAlreadyFinalized` next to `AlreadySlashed` in `src/state/typed_tx.rs:1012` `TransitionError` enum + add Display arm + L4.E rejection_class mapping (`PolicyViolation` per charter § 4.5).

**Reversibility**: variant addition is additive-safe (existing match arms are exhaustive but new variant just adds a case). Removal would be a wire break. Adding it now costs nothing.

---

### Q3 — Zero-window MVP vs minimum-1-block window (Atom 4)

**Ratified**: zero-window MVP. `claims_t[claim_id].challenge_window_close_logical_t = 0` at claim-creation (the literal value 0 is the structural "window-closed-immediately" marker). Solo-run / single-verifier path collapses the window to zero.

**Why** (corrected during code-walk; see §2.4 below): An earlier draft of this ratification proposed `claim.challenge_window_close_logical_t = verify.timestamp_logical`. That choice mixes namespaces — `verify.timestamp_logical` is **agent-controlled** (set by the agent in `make_real_verifytx_signed_by`), while `fr.timestamp_logical` (the field the dispatch-arm gate would compare against) is **sequencer-controlled** (set by `emit_system_tx` from `next_logical_t.load + 1`). The two namespaces are incommensurable; an agent can set its `verify.timestamp_logical` arbitrarily large and starve every finalize. The corrected ratification: **window=0 is the explicit MVP marker**. Atom-3 dispatch gates on `claim.window > 0 AND fr.timestamp_logical ≤ claim.window` — a no-op when window=0. The structural ordering guarantee (claim must exist for finalize to dispatch; claim only exists post-Verify-Confirm) replaces the time-based gate for TB-8 MVP.

**How to apply**: `claims_t[claim_id].challenge_window_close_logical_t = 0` in the Atom-1 writer. Atom-3 dispatch arm: `if claim.window > 0 && fr.timestamp_logical <= claim.window → ChallengeWindowStillOpen`. The dispatch-side gate is forward-compat with a future TB that introduces real window timing (set window to a non-zero sequencer-namespace logical_t at claim-creation; gate fires until current logical_t passes it).

**Reversibility**: changing zero→N-block requires only the writer change (set `window = q.q_t.current_round + N` once `current_round` is wired to advance per-accept; or set window = sequencer-side stamped logical_t). No dispatch-side change.

---

### Q4 — Conservation invariant: `debug_assert` vs `assert` (Atom 3)

**Ratified**: `debug_assert!` in the dispatch arm hot path + dedicated `cargo test --release` unit test that re-runs the conservation check explicitly.

**Why**: `assert!` in the dispatch arm panics in release mode if invariant violates. For a runtime that processes thousands of TXs per minute, this is a cheap ($1 cycle) but non-zero overhead per TX. `debug_assert!` is free in release. The dedicated `cargo test --release` unit test exercises the conservation check on the same path — so a release-mode invariant violation would trip the test (caught at CI), not in production. Cost-benefit favors `debug_assert!` + release-mode test.

**Counter-argument considered + rejected**: "release-mode panic on invariant violation is the strongest signal we can give." True, but the production path *cannot* produce an invariant violation absent a code bug in dispatch_transition itself, which would be caught at the test layer (since the test re-runs the same dispatch path). The release-mode panic guard is defense-in-depth at runtime overhead cost — not the cheapest correct choice.

**How to apply**: in `dispatch_transition` `TypedTx::FinalizeReward` arm, after the atomic mutation block, call `monetary_invariant::total_supply_micro(&q_next.economic_state_t)` and `debug_assert_eq!` against the pre-mutation snapshot. Add `tests/tb_8_finalize_conservation.rs` with `#[cfg(test)]` test that runs the same arm under `cargo test --release`.

**Reversibility**: switching `debug_assert!` → `assert!` is a one-character edit. Cheap to reverse.

---

### Q5 — `reward_factor` (Atom 1)

**Ratified**: `claim.amount = task_market_entry.total_escrow` for single-solver MVP. No platform-fee field, no reserve, no discount factor.

**Why**: `feedback_launch_priority` defines TB-8 as *minimal* payout. Adding a `reward_factor` or `platform_fee_basis_points` field at this stage introduces a settlement-rule decision that the architect explicitly deferred to RSP-4-full territory (charter §4 forbidden #11: "Generalized settlement-rule-hash parsing is RSP-4-full territory"). The single-solver case has exactly one correct answer (solver gets the whole escrow; otherwise the escrow becomes unattributable dust). Reserving a placeholder field now would invite Chesterton-fence behavior at TB-9..TB-15 where some atom would either (a) ratify a default value or (b) defer that ratification too — net cost > net benefit.

**How to apply**: in the Atom-1 writer at the Verify dispatch arm, `claim.amount = task_markets_t[task_id].total_escrow` at OMEGA-Confirm acceptance time. No field added to `ClaimEntry` for fee/factor.

**Reversibility**: adding a `platform_fee_basis_points` field to `ClaimEntry` later is additive-safe (default = 0). Adding it now and trying to remove it later is a wire break. Don't reserve placeholder fields.

---

## §2 Architectural clarifications discovered during code-walk (NOT in §7 §1-§5)

### §2.1 OMEGA verdict mapping

`VerifyVerdict` enum at `src/state/typed_tx.rs:178` has only `Confirm = 0` and `Doubt = 1`. There is NO `Verdict::Omega` variant — "OMEGA" is a *runtime concept* meaning "Lean oracle accepted the proof and the verifier confirms via VerifyTx{verdict: Confirm}". Per TB-7 commit map + `make_real_verifytx_signed_by` (`src/runtime/adapter.rs:227`): `verdict_confirms = true → VerifyVerdict::Confirm`.

**Ratification**: TB-8 Atom 1 writer fires when `verify.verdict == VerifyVerdict::Confirm` AND `target_work_tx` is in `stakes_t`. In single-verifier MVP, this is the OMEGA-Confirm path. Multi-verifier with quorum (RSP-2 territory) would require `task_markets_t[task_id].verifier_quorum` Confirms before claim-creation — explicitly out of TB-8 scope. The `verifier_quorum` field is read but not enforced in TB-8 (single-solver MVP — quorum=1 default suffices).

### §2.2 ClaimId construction

`ClaimId` (`src/state/typed_tx.rs:52`) wraps `TxId`. The Atom-1 writer needs a deterministic claim_id derivation. Per single-solver MVP, the natural choice is `ClaimId(TxId(format!("claim-{}", verify.tx_id.0)))` — deterministic, replay-safe, collision-free since each accepted Confirm VerifyTx has a unique tx_id.

**Ratification**: `claim_id = ClaimId(TxId(format!("claim-{}", verify.tx_id.0)))` at writer site. Single-solver MVP — multi-solver (TB-15+) would key claim by `(task_id, solver)` tuple to support per-solver split. Deferred per charter §4 forbidden #6.

### §2.3 settlement_rule_hash opacity preserved

Per charter §4 forbidden #11: `settlement_rule_hash` stays opaque for TB-8. The Atom-1 writer does NOT inspect it. The Atom-3 dispatch arm does NOT inspect it. TB-8 picks the literal trivial settlement (`amount = total_escrow`) without consulting the hash. This preserves the RSP-4-full upgrade path: a future TB introducing a `SettlementEngine` will inspect `settlement_rule_hash` and override the trivial default — additive change to the dispatch arm, no schema migration needed.

### §2.4 Window-namespace correction (discovered during Atom-3 implementation)

The original Q3 ratification ("`window = verify.timestamp_logical`") was **revised during Atom-3 implementation** when the test `finalize_reward_happy_path_debits_escrow_credits_solver_conserves_ctf` failed with `ChallengeWindowStillOpen` — agent-side `verify.timestamp_logical = 7` exceeded sequencer-emit-side `fr.timestamp_logical ≈ 5`, blocking finalize. The fix: `window = 0` (literal MVP marker), gate fires only when `window > 0`. This correction is incorporated into the §1 Q3 ratification above. The architectural insight worth recording: **agent-controlled timestamps are NEVER directly comparable to sequencer-controlled timestamps**; any future windowing scheme must use sequencer-side logical_t exclusively (e.g., `q.q_t.current_round` advanced per-accept, or `next_logical_t` exposed into dispatch_transition via a future API). For TB-8 MVP, the simplest correct version is "no time-based gate; structural ordering suffices".

---

## §3 Acceptance — Atom 0.5 closes here

```text
✅ Q1 — ClaimEntry 6-field expansion ratified
✅ Q2 — ClaimAlreadyFinalized variant ratified
✅ Q3 — Zero-window MVP ratified
✅ Q4 — debug_assert + release-mode test ratified
✅ Q5 — claim.amount = total_escrow no-fee ratified
✅ §2.1 OMEGA = VerifyVerdict::Confirm clarification recorded
✅ §2.2 claim_id = ClaimId(TxId(format!("claim-{}", verify.tx_id.0))) clarification recorded
✅ §2.3 settlement_rule_hash opacity preserved
```

Atom 1 (claims_t writer + ClaimEntry schema extension) begins immediately on user authorization "finish until TB-8 is finished" — no separate ratification ping required.

---

## §4 Cross-references

- Charter: `handover/tracer_bullets/TB-8_charter_2026-05-02.md` §7
- TB-7R Atom 0.5 precedent: `handover/audits/CHARTER_RATIFICATION_TB_7R_*` (analogous shape)
- Memory rules in effect: `feedback_no_fake_menus`, `feedback_kolmogorov_compression`, `feedback_dual_audit` Class 3, `feedback_iteration_cap_24h`, `feedback_step_b_protocol`
- Architect directive 2026-05-02 ruling 12 (TB-8 = Minimal Payout) + ruling 13 (NodeMarket post-Lean-MVP)
