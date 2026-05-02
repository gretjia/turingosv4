# Gemini TB-8 Ship Audit

**Date**: 2026-05-02
**Model**: gemini-3.1-pro-preview (strategic)
**Audit type**: Class 3 strategic / architectural angle (parallel with Codex impl-paranoid).

---

As Gemini DeepThink, I have reviewed the TB-8 ship-gate audit from the strategic, architectural, and constitutional angle. My evaluation of the five core criteria follows:

**Q1 — 5→4 holding migration (claims_t as intent registry)**
The migration is architecturally sound and the right call. Treating `claims_t` as a holding while the funds simultaneously reside in `escrows_t` creates a double-spend accounting paradox during the lifecycle of an open claim. By redefining `claims_t` strictly as an intent/metadata registry and enforcing the `assert_claim_amount_backed_by_escrow` invariant, we maintain strict conservation of supply (total_supply = balances + escrows + stakes + bond) while allowing atomic, single-step settlement (escrow → balance). This perfectly mirrors the cache-vs-holding precedent established in TB-3.

**Q2 — Zero-window MVP & Namespace Rejection**
The rejection of `verify.timestamp_logical` as a window source is absolutely correct. Mixing an agent-provided (or off-chain evaluator-provided) timestamp with the sequencer's strict logical time namespace introduces a critical time-jacking vulnerability. Financial settlement windows must be strictly bound to the sequencer's monotonic clock. The zero-window MVP (`window = 0`, gate fires on `> 0`) is a clean, forward-compatible placeholder that safely defers wall-clock/logical-clock scheduling (Forbidden Line #16) without compromising the current settlement architecture.

**Q3 — Best-effort evaluator emit safety**
Best-effort is the correct distributed systems approach for this layer. The L4 OMEGA evidence (the accepted VerifyTx and resulting open claim) is durably committed to the chain state. If the evaluator's off-chain polling budget expires, failing the run (exit 3) would be a destructive overreaction that does nothing to advance the chain state. Because the claim is durably recorded, a future sweeper, admin-emit, or subsequent session can safely finalize the payout. The state machine remains safe; only the immediate off-chain dispatch is delayed.

**Q4 — Anti-Oreo barrier integrity**
The barrier holds. By inheriting the TB-5 RSP-3.0 ingress filtering, `submit_agent_tx` strictly rejects `TypedTx::FinalizeReward(_)`. Furthermore, because `emit_system_tx` Q-derives the critical financial parameters (`task_id`, `solver`, `reward`) directly from the trusted `claims_t` state rather than accepting them as caller arguments, the surface area for an agent to forge a payout destination or amount is structurally eliminated.

**Q5 — Smoke variety & Chain-backed evidence**
The requirement for chain-backed, replayable evidence (`runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz`) is a non-negotiable constitutional baseline per `feedback_smoke_evidence_naming`. The spot-check confirms that the 7 runs across the heldout-49 problems rely on durable chaintape replays rather than ephemeral stdout paper trails. The variety bar is met.

**Forbidden Lines Check:**
The implementation strictly respects the charter's negative space. No AMM/CPMM logic, no multi-solver splits, no wall-clock schedulers, and no constitution.md edits were introduced.

## VERDICT: PASS
(All Q1-Q5 cleared; architectural angle is clean.)
