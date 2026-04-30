# Gemini TB-2 Phase-1c Diff Dual External Audit
**Date**: 2026-04-30
**Target**: experiment branch `experiment/tb2-sequencer-runtime-closure` HEAD `138d5ac32fb2caca1397150d2a844446772fa2bb`
**Base**: `f9ace5e` (preflight v3 + charter v3 + Phase-0 audits)
**Prompt size**: 457745 chars
**API latency**: 41.0s
**Mandate**: STEP_B Phase-1c diff audit; strategic / architectural / constitutional (Q1-Q8). Independent of Codex Phase-1c (parallel, implementer-paranoid).

---

**Section A: Overall verdict**

**PASS** (Conviction: 5/5)

The `experiment/tb2-sequencer-runtime-closure` branch is a surgically precise implementation of the preflight v3 specification. It honors all architectural constraints, introduces zero scope creep, and successfully establishes the L4 / L4.E split at the runtime boundary. The diff is cleared for merge to `main` pending Codex's verdict.

---

**Section B: Per-Q1-Q8 disposition**

**Q1. Minimality vs scope**
**PASS**. The diff is strictly confined to the minimum-sufficient version defined in §3.1–§3.7. The ~544 LOC added to `src/state/sequencer.rs` consist entirely of the explicitly authorized helpers (`WORKTX_ACCEPT_DOMAIN_V1`, `worktx_canonical_hash`, `worktx_accept_state_root`, `SYSTEM_AGENT_ID_STR`, `rejection_class_for`, `public_summary_for`), the `SubmissionEnvelope` struct, the `try_apply_one` test driver, and the filled bodies for `dispatch_transition` and `apply_one`. The ~13 LOC in `typed_tx.rs` are exactly the two new error variants and their `Display` arms. The ~726 LOC in `tests/tb_2_runtime_boundary.rs` perfectly match the 13-test integration battery. There are no unauthorized abstractions or indirections.

**Q2. `dispatch_transition` purity**
**PASS**. A verbatim inspection of the `TypedTx::Work(work)` arm (`src/state/sequencer.rs:158-235`) confirms it is a 100% pure validation pipeline. It performs parent-root matching, iterates over predicate bundles, checks stake via `.micro_units() > 0`, checks escrow presence via `q.economic_state_t`, and asserts monetary invariants. It acquires zero locks, performs zero I/O, and makes zero writer/CAS calls. It returns `Ok((q_next, SignalBundle::default()))` or `Err(TransitionError)` exactly as mandated, preserving replay determinism.

**Q3. `apply_one` rejection-path discipline**
**PASS**. The `Err(transition_err)` match arm in `apply_one` (`src/state/sequencer.rs:580-652`) perfectly honors the K1 and Inv 7 contracts:
- It reads `rejection_logical_t = self.next_logical_t.load(...)` but does NOT advance it via `store` or `fetch_add`.
- It writes to `cas` and `rejection_writer`, but makes zero writes to `q` or `ledger_writer`.
- It keys the L4.E append using `submit_id` destructured directly from the `SubmissionEnvelope` (`writer_w.append_rejected(submit_id, ...)` at line 640).
- It returns `Err(ApplyError::Transition(transition_err))` at line 651 without calling `dispatch_transition` again.

**Q4. `assert_total_ctf_conserved` call shape**
**PASS**. The runtime call site inside the `WorkTx` arm (`src/state/sequencer.rs:228-232`) passes exactly `&[]` as the third argument:
`assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])`. No non-empty exempt lists are used in production.

**Q5. New `TransitionError` variants — minimal set?**
**PASS**. The diff adds exactly two new variants to `src/state/typed_tx.rs:786-790`: `EscrowMissing` and `MonetaryInvariantViolation`. The corresponding `Display` arms are added at lines 825-826. The obsolete names `StaleParentRoot` and `PostInitMint` do not appear anywhere in the diff; the code correctly reuses the existing `TransitionError::StaleParent` variant (mapped to `PolicyViolation` in `rejection_class_for`).

**Q6. `EscrowVault` non-use red line (§8 line 8)**
**PASS**. `EscrowVault` is completely absent from the diff. The escrow lookup bridge in `src/state/sequencer.rs:194-200` reads exclusively from `q.economic_state_t.escrows_t.0` and `q.economic_state_t.task_markets_t.0`. The single-truth-source contract for the runtime spine is preserved.

**Q7. P0-B option (a) deletion-target comment (§8 line 9)**
**PASS**. The bridge line in `src/state/sequencer.rs:189-191` carries the exact required inline comment:
`// Step 5: escrow presence gate (RSP-1 P3:5; P0-B option (a) — bridge`
`// at lookup site). TB-3 introduces formal task_open_tx /`
`// escrow_lock_tx / yes_stake_tx variants and DELETES this bridge.`

**Q8. Replay invariant — does I13 actually prove P1:8 / Art IV Boot?**
**PASS**. Test I13 (`tests/tb_2_runtime_boundary.rs:658-726`) rigorously proves the invariant. It submits one accepted and one rejected `WorkTx`, captures the live sequencer's post-submission roots, and then reconstructs the state using `replay_full_transition`. Crucially, it reconstructs the state by reading `entries` directly from `h.ledger_writer` (the canonical L4) and passes `h.initial_q` as the genesis base. It does not pass the `rejection_writer` to the replay function (which wouldn't compile anyway), proving that the rejected submission in L4.E had zero influence on the canonical reconstruction.

---

**Section C: New CONSTITUTIONAL DEBT introduced**

None. The interim state-root mutator (`WORKTX_ACCEPT_DOMAIN_V1`) and the `TaskId` → `TxId` bridge are technical debt, but they are explicitly authorized by the preflight v3 charter, properly domain-separated, and explicitly marked for deletion in TB-5 and TB-3 respectively. There is no *unapproved* constitutional debt.

---

**Section D: TROJAN / scope-creep findings**

None. The diff is exceptionally clean and strictly adheres to the restricted-file protocol.

---

**Section E: Recommendation**

**Merge cleared.** The branch `experiment/tb2-sequencer-runtime-closure` successfully implements the TB-2 runtime boundary closure. Execute `git merge experiment/tb2-sequencer-runtime-closure --no-ff` on `main` once Codex confirms.