# Gemini TB-7R Ship Audit (audit-point-2; round 1)

**Date**: 2026-05-02
**Range**: `9e74195..4470036` (5 commits)
**HEAD**: 4470036876d06d794036a1818d0331692046a482
**Workspace test count**: 712 / 0 / 150 (cargo test --workspace canonical)
**Audit class**: Class 3 (auth-crypto-money) — full dual; Codex-impl + Gemini-arch.
**Auditor**: Gemini DeepThink (gemini-3.1-pro-preview; tier label = strategic)
**API latency**: 44.1s
**Prompt size**: 581740 chars
**Mandate**: TB-7R ship-gate strategic / architectural / constitutional audit (Q1-Q8). Independent of Codex (parallel; implementation-paranoid).

> If `tier label = degraded` above, this audit is DEGRADED-MODE per memory `feedback_dual_audit`. The merged dual verdict MUST display `degraded — Gemini at degraded tier` per TB-5/TB-6/TB-7 supplement precedent.

---

**Section A: Overall Ship Verdict**

**Verdict: PASS**  
**Conviction: 4 / 5**

TB-7R successfully executes the "Constitution-Aligned Frame B Repair" exactly as authorized by the architect. It structurally enforces the `Predicate Pass → L4 / Predicate Fail → L4.E` boundary, decouples oracle truth (`VerificationResult` CAS) from economic admission (`VerifyTx`), and implements the fail-closed ChainTape gate. The codebase is now strictly aligned with the authorized three-node taxonomy. The two outstanding constitutional tensions (OBS-1 and OBS-2) are pre-existing architectural debts exposed by TB-7R's new rigor, not regressions introduced by it. They are correctly bounded as post-TB-7R follow-ups. TB-7R is cleared for ship.

---

**Section B: Per-Q1-Q8 Disposition**

**Q1. Four-clause acceptance — clause 1 (externalized proposals)**  
**Verdict: PASS (Pragmatic) / CHALLENGE (Constitutional)**  
Under the strict three-node taxonomy (`DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md`), Clause 1 is trivially satisfied because "externalized" is defined as routing through `submit_typed_tx`. However, constitutionally (Art. 0.2 Tape Canonical Axiom + WP §5 L4), if the LLM emits a `step` tool call and the system evaluates it via Lean, that *is* an externalized proposal. Bypassing the chain for `PartialOk` (shadow-only) or `Reject` (in-memory counter) is a constitutional gap. The architect's deferral (OBS-1) is a defensible *sequencing* choice to unblock Frame B, but the constitutional reading demands that all evaluated LLM outputs eventually route to L4/L4.E. 

**Q2. Four-clause acceptance — clause 2 (predicate evidence)**  
**Verdict: PASS**  
`chain_oracle_verified=true` is a robust, cryptographically sound predicate. `src/runtime/chain_derived_run_facts.rs:321-336` correctly walks `accepted_worktx_vr_cid`, fetches the `VerificationResult` from CAS, and checks `vr.verified`. Because `VerificationResult` is generated directly from the Lean exit code (`src/runtime/verification_result.rs:65`), this completely severs the prior defect where a verifier's economic declaration (`VerifyTx::Confirm`) could manufacture acceptance.

**Q3. Four-clause acceptance — clause 3 (failed proposal shielding)**  
**Verdict: PASS**  
The structural shielding holds. Failed proposals route to `RejectionEvidenceWriter` (`src/bottom_white/ledger/rejection_evidence.rs`). The raw diagnostic is stored in CAS, and only the `raw_diagnostic_cid` and `public_summary` are written to the L4.E JSONL index. The dashboard (`src/bin/audit_dashboard.rs:388`) only reads and prints the `rejection_class` (e.g., `PredicateFailed`), ensuring that raw diagnostics do not leak into the public materialized view.

**Q4. Four-clause acceptance — clause 4 (dashboard regeneratable)**  
**Verdict: PASS**  
`src/bin/audit_dashboard.rs` is strictly a read-only materialized view. It instantiates `Git2LedgerWriter::open`, `CasStore::open`, and `RejectionEvidenceWriter::open_jsonl` purely for reading. It caches nothing locally; it only outputs to stdout or a user-specified `--out` file. Deleting the dashboard binary or its output loses zero authoritative state.

**Q5. Hard-guardrail compliance**  
**Verdict: PASS**  
I verified the 13 forbidden lines against the diff. 
- *No per-tactic decomposition*: `evaluator.rs:1743-1860` still processes the `complete` tool as a single compound block.
- *No retroactive ledger rewrite*: Historical evidence directories only received `README.md` annotations.
- *No fabricated genesis*: `genesis_report.rs` is only called during live evaluator bootstrap (`evaluator.rs:969-1016`).
- *No new TypedTx*: `VerificationResult` is correctly implemented as a CAS object (`src/runtime/verification_result.rs:114`), not a ledger transition.

**Q6. parent_tx as conditional invariant**  
**Verdict: PASS**  
The case-split in `src/runtime/chain_derived_run_facts.rs:200-240` (`compute_parent_tx_state`) is mathematically exhaustive for the defined states. It groups attempts by `(agent_id, branch_id)`. If max attempts < 2, it yields `SingletonGoldenPathValid` or `NoMultiAttemptObserved`. If max attempts >= 2, it checks every non-root index for `parent_tx.is_none()`. There is no edge case where a multi-attempt branch with missing edges could accidentally render as `SingletonGoldenPathValid`.

**Q7. Constitutional question (OBS-2 Prompt Pollution)**  
**Verdict: CHALLENGE (Deferral Stands)**  
Reading (b) is correct: the architect likely missed this in the D7 resolution, and it is a strict violation of Art. III.4 ("失败候选不能污染其他 Agent 上下文"). In multi-agent runs, `acc` is shared, meaning Agent A's raw Lean error leaks into Agent B's prompt. However, **TB-7R should ship with OBS-2 deferred**. TB-7R's charter strictly forbade touching prompt generation or `bus.rs` internals unless required for the L4/L4.E split. Fixing this requires redesigning `AccountingState` isolation, which is a feature/performance trade-off (PPUT-CCL B2) that requires its own dedicated TB.

**Q8. Production claim defensibility**  
**Verdict: PASS**  
The claim in charter §8 is defensible at HEAD. The only slight overstatement is "every externalized LLM proposal" due to the `step` tool bypass (OBS-1), but because TB-7R formally adopted the three-node taxonomy defining "externalized" as `submit_typed_tx`, the claim is technically true under its own definitions. No rewording is strictly necessary, provided OBS-1 remains an active follow-up.

---

**Section C: NEW Constitutional Debt Introduced by TB-7R**

**None.** TB-7R introduces zero *new* constitutional debt. It actually pays down massive debt by enforcing the Tape Canonical Axiom (Art. 0.2) via payload CAS storage and separating the L4/L4.E ledgers. The debts discussed in OBS-1 and OBS-2 are pre-existing architectural flaws that TB-7R's strictness has simply made highly visible.

---

**Section D: OBS Bounding Review**

*   **OBS-1 (Coverage Denominator)**: **STANDS**. The strict three-node taxonomy provides a mathematically sound boundary for TB-7R. Routing `PartialOk` and `Reject` to the chain requires refactoring the evaluator's tool dispatch and Sequencer acceptance classes. Deferring this to TB-8+ is the correct pragmatic choice. Severity if wrong: Low for TB-7R, but High for the ultimate TuringOS v4 audit, as unchained LLM outputs violate the Tape Canonical Axiom.
*   **OBS-2 (Prompt Pollution)**: **STANDS**. TB-7R did not introduce the shared `acc` state. Fixing it requires a prompt-builder refactor that risks degrading PPUT (agent self-correction). Deferring it to a dedicated architectural review is correct. Severity if wrong: Medium. It pollutes context but does not corrupt the authoritative L4 ledger.

---

**Section E: Production Claim Review**

The production claim (charter §8) is accurate and defensible. 
*   *Overstates*: "every externalized LLM proposal lands in either L4... or L4.E". (Only true under the strict taxonomy; `step` tool outputs are practically externalized but unchained).
*   *Understates*: Nothing. It perfectly captures the boundaries of what was achieved.
*   *Rewording*: None required for ship, but future TBs should retire the "strict taxonomy" crutch once OBS-1 is resolved.

---

**Section F: Recommendation**

**SHIP-CLEAR WITH OBS-TIGHTENING.**

Merge the TB-7R commit range. The implementation is exceptionally rigorous, the conformance tests prove the DAG logic without fabricating smoke evidence, and the fail-closed ChainTape gate (`chaintape_mode_gate.rs`) successfully prevents silent degradation. 

**Post-ship action**: Elevate OBS-1 and OBS-2 to the immediate roadmap for TB-8/TB-9. The system's ledger is now constitutionally sound, but the evaluator's interaction with that ledger (tool dispatch and prompt context sharing) remains the last major source of Frame B constitutional friction.