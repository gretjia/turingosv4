# Gemini TB-2 Phase-0 Round-1 Dual External Audit
**Date**: 2026-04-30
**Target**: STEP_B preflight `handover/ai-direct/TB-2_SEQUENCER_RUNTIME_CLOSURE_2026-04-30.md`
**HEAD**: 3f06d512983a2bcb5ebc103b203cdaa7412b6362
**Prompt size**: 476880 chars
**API latency**: 42.7s
**Mandate**: STEP_B Phase-0 necessity audit; strategic / architectural / constitutional (Q1-Q8). Independent of Codex r1 (parallel, implementer-paranoid).

---

**Section A: Overall Verdict**

**Verdict**: **CHALLENGE**
**Conviction**: 5/5

The architectural framing of the STEP_B preflight is exceptionally sound. The decision to keep `dispatch_transition` pure (rejecting naive A) and the strict enforcement of the `exempt_tx_kinds` red line demonstrate a deep understanding of the TuringOS constitutional invariants. However, I am issuing a **CHALLENGE** based on Q7: the proposed 12-test acceptance battery has critical blind spots regarding `StaleParent` rejections, `submit_id` consumption on queue saturation, and the fundamental P1:8 replay invariant. These must be added to the preflight before Phase 1 begins.

---

**Section B: Per-Question Disposition**

**Q1. STEP_B applicability** — **PASS**
Relying on the C-031 "institution" catch-all is valid here. `sequencer.rs` is the absolute core of the state transition machine; it dictates what enters the canonical L4 tape and what is banished to L4.E. Changing its queue payload and error-handling paths is a fundamental architectural shift that warrants STEP_B rigor. However, relying on the catch-all is a temporary crutch; `sequencer.rs` should be formally added to `CLAUDE.md`'s restricted list (see P1).

**Q2. A-corrected vs naive A** — **PASS**
The preflight's rejection of naive A is architecturally mandatory. `dispatch_transition` represents $\prod p(output | Q_t)$. It must remain a pure function. If ledger I/O were placed inside it, replaying the ledger (which is required by Art IV Boot and P1:8) would re-trigger those side effects, destroying determinism and creating a catastrophic chicken-and-egg loop. The FC2 (state mutation) / FC3 (ledger persistence) separation is absolute.

**Q3. AcceptedLedger as TB-1 primitive** — **PASS**
The "two-L4 spines" concern is real and fatal if violated. `transition_ledger` already possesses the `LedgerEntry` schema, `LedgerWriter` trait, and `Git2LedgerWriter` backend (from CO1.7). `AcceptedLedger` was temporary RSP-0 scaffolding to prove the hash-chain math. Wiring `AcceptedLedger` into the production sequencer would create a parallel accepted ledger, violating the ChainTape single-spine contract. The preflight correctly banishes it from the production path.

**Q4. RSP-1 admission via stake>0 + seeded EconomicState** — **PASS**
This is a valid minimum-sufficient slice. The goal of TB-2 is to close the runtime boundary (P1) and prove that the sequencer *can* reject based on economic state (P3). By seeding the state in the test fixture and checking `WorkTx.stake` at runtime, we prove the *enforcement mechanism* works. The *lifecycle* of how that state gets there (formal `task_open_tx` / `escrow_lock_tx` variants) is a separate concern properly deferred to TB-3. It is not "fake" because the sequencer is genuinely reading the state and enforcing the rule.

**Q5. SubmissionEnvelope first atom** — **PASS**
This is necessary plumbing, not premature churn. The L4.E contract (P1:6) requires rejections to be keyed by `submit_id`. If `apply_one` only receives `TypedTx`, it has no way to know the `submit_id` to write to L4.E. A tuple `(u64, TypedTx)` is functionally identical but less extensible. Embedding `submit_id` inside `TypedTx` would violate the separation of concerns (submit_id is a sequencer queue concept, not a cryptographic payload concept). `SubmissionEnvelope` is the correct abstraction and needs no further fields today.

**Q6. `exempt_tx_kinds` red line** — **PASS**
The fence is strictly correct for the runtime spine. Genesis mint happens at `on_init`, which executes outside the `Sequencer::apply_one` runtime loop. The runtime loop processes `TypedTx` submissions. None of the current `TypedTx` variants are allowed to mint (`FinalizeReward` is a transfer from escrow to claims). Therefore, passing `&[]` at runtime is strictly correct and mechanically enforces Inv 4 (no post-init mint).

**Q7. Acceptance battery** — **CHALLENGE**
The 12/12 battery is strong but incomplete. It misses four critical proofs:
1. `StaleParent` rejection is a distinct and vital rejection class that must be explicitly tested.
2. The queue-full backpressure interaction with `submit_id` allocation (if `try_send` fails, the `submit_id` is consumed via `fetch_add`, which is acceptable but must be asserted to prevent future "bug fixes" that try to reuse IDs).
3. The `raw_diagnostic_cid` serde-shield must be re-confirmed at the runtime path to ensure `apply_one`'s usage doesn't accidentally bypass it.
4. **Most critically**: A replay test proving P1:8. We must prove that rebuilding `state.db` from the canonical L4 transitions completely ignores L4.E records.

**Q8. Phase-ordering** — **PASS**
Co-discharging P1 and P3 here is honest and architecturally sound. The dependency graph notes that `P2 Agent Runtime` depends on `RSP-1`. The sequencer runtime closure (P1) *needs* predicates to evaluate. The economic predicates (RSP-1) are the most fundamental gates. Discharging them together at the runtime boundary proves the enforcement mechanism works. Doing P1 without P3 would mean testing against dummy predicates, which proves nothing about the system's actual security.

---

**Section C: P0 List (Must-fix before Phase-1)**

1. **File**: `handover/ai-direct/TB-2_SEQUENCER_RUNTIME_CLOSURE_2026-04-30.md`
   **Concrete Remediation**: Expand the §5 Acceptance Battery from 12 tests to 16 tests. Add the following:
   *   `13. runtime_stale_parent_worktx_appends_l4e`: Submit a WorkTx with a `parent_state_root` that does not match `q.state_root_t`. Expect 1 L4.E row with `rejection_class = StaleParentRoot`.
   *   `14. submit_queue_full_consumes_submit_id`: Saturate the queue, call `submit()` to get `QueueFull`, then free a slot and `submit()` again. Assert the successful `submit_id` is `+2` from the last successful one (proving failed submits safely burn their ID).
   *   `15. runtime_l4e_public_view_honors_serde_shield`: Retrieve the L4.E record generated by a runtime rejection and assert `raw_diagnostic_cid` is structurally absent from its `public_view()`.
   *   `16. runtime_replay_ignores_l4e`: Submit 1 accepted WorkTx and 1 rejected WorkTx. Reconstruct the state from L4 only. Assert the reconstructed state matches the sequencer's state, proving L4.E side-effects did not bleed into the canonical state reconstruction.
   **Estimated Effort**: 30 minutes to update the preflight markdown.

---

**Section D: P1 List (Should-fix; can proceed-with-OBS)**

1. **File**: `CLAUDE.md`
   **Concrete Remediation**: Add `src/state/sequencer.rs` to the explicit restricted-file list under the "Code Standard" section. Relying on the C-031 "institution" catch-all is valid for this audit, but explicit listing prevents future LLM agents from bypassing STEP_B due to context-window truncation of case law.
   **Estimated Effort**: 2 minutes (can be done in a separate hygiene commit).

---

**Section E: Recommendation**

**Revise preflight.** 
The architectural design is excellent and correctly navigates the TuringOS constitutional constraints. However, the acceptance battery must be expanded to cover the four missing cases identified in Q7/Section C. 

Once the preflight is updated to require a 16/16 deterministic PASS (incorporating the new tests), you are cleared to proceed to STEP_B Phase 1 (branch creation and implementation).