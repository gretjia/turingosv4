# TB-18R G2 Dual Audit Dispatch — Codex + Gemini Ship Audit

**Date**: 2026-05-06
**Gate**: G2 (final ship audit; charter §2 G2 row + SG-18R.13)
**Scope**: R0..R7 + 2 OBS forward-bindings.
**Conservative ranking**: VETO > CHALLENGE > PASS per `feedback_dual_audit_conflict`.

---

## §1 Audit Ask (verbatim for both Codex + Gemini)

```text
TB-18R Tape Restoration final ship audit (Gate 2). Class 4 ship-gate.

Predecessor:
  - TB-18 PROVISIONAL SHIPPED 2026-05-05 (commit 15b662c).
  - TB-18 M1 evidence triggered VETO 2026-05-06 (per-LLM-call
    failure-path asymmetry: omega externalizes but
    step_reject/parse_fail/llm_err leak to evaluator stdout only).
  - VETO archive at handover/architect-insights/
    TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md.

TB-18R substrate (R0..R7 atoms shipped to main):

  R0 (Class 0): charter v2 + grandfathering README on M1 evidence.
     handover/tracer_bullets/TB-18R_charter_2026-05-06.md.

  G1 (Class 3 audit): Codex charter ratification CLOSED 2026-05-06.
     CHALLENGE-but-ship-clean; 7 remediations applied as charter v2.

  R1 (Class 4 STEP_B): typed-tx + CAS schema additions.
     AttemptTelemetry + LeanResult + TerminalAbortRecord schemas.
     AttemptKind/Outcome split; schema_version=1; attempt_chain_root
     on AttemptTelemetry (Design B; WorkTx canonical preserved).
     Commit 9f8ce1f via merge bbee847.

  R2 (Class 3): evaluator hot path externalization.
     6 evaluator paths instrumented (omega-full @ ~2317 /
     omega-pertactic @ ~2861 / step_partial_ok @ ~3236 /
     step_reject @ ~3263 / parse_fail @ ~3275 / llm_err @ ~3289).
     CR-18R.4 v2 privacy fence preserved via parsed-candidate-only
     on paths 1-4 + fixed sentinels on paths 5-6.
     Commit 35389d0.

  R3 (Class 4 STEP_B): sequencer L4.E admission expansion.
     RejectionClass tail-append {LeanFailed=6, ParseFailed=7,
     SorryBlocked=8, LlmError=9} stable-repr-u8 per Codex Q8.
     Sequencer Design D: refine_rejection_class_via_attempt_telemetry
     reads AttemptTelemetry on predicate_passes=false arm; legacy
     ProposalTelemetry → fall-back PredicateFailed.
     Class 3 evaluator wire-up: 3 failure paths submit
     predicate_passes=false WorkTx; omega paths NO cutover (R3 §3.5
     amended preserves TB-7 audit chain).
     Commit 72a1b75 via merge 66dde84.

  R3.fix (Class 4 STEP_B): CasStore split-brain reload.
     Surgical patch closing L0 smoke 2026-05-06 stale-cache bug —
     long-lived sequencer.cas in-memory index was stale wrt
     evaluator-side handle writes. Fix = NEW pub fn
     CasStore::reload_index_from_sidecar + retry-on-miss in
     refine helper.
     Commit 2ca1aed via merge f2e73f6.
     L0 smoke re-verify: P49 MAX_TX=5 produces chain rejection_class
     histogram {LeanFailed=6: 1, SorryBlocked=8: 4} — zero
     PredicateFailed (was 5/5 PredicateFailed pre-fix); R3 charter
     promise of fine-grained class on L4.E EMPIRICALLY VALIDATED
     end-to-end.

  R4 (Class 4 STEP_B; G1-ratified canonical contract):
     chain_derived_run_facts attempt_count_invariant() ship-gate
     equation. Implementation populates ratified spec WITHOUT
     alteration per charter §0.A Q1+Q4.
     ChainDerivedRunFacts +6 fields all #[serde(default)]:
       expected_completed_attempts / l4_work_attempt_count /
       l4e_work_attempt_count / attempt_aborted_count / delta /
       terminal_halt_class.
     NEW pub types: AttemptCountInvariantInputs +
       AttemptCountInvariantViolation + DrainBarrierViolation +
       ChainDerivedError::DrainBarrier.
     NEW pub fns: compute_run_facts_from_chain_with_invariant +
       attempt_count_invariant + verify_chain_quiescent_post_drain.
     G1 equation populated verbatim:
       evaluator_reported_completed_llm_calls
         == l4_work_attempt_count + l4e_work_attempt_count
       (clean halt; delta==0 AND aborted==0 required) +
       auxiliary `expected + aborted == l4 + l4e` (abort halt).
     Negative delta forbidden under any halt class.
     terminal_halt_class reuses constitutional RunOutcome.
     Drain barrier honored via existing ChaintapeBundle::shutdown()
     + next_submit_id_peek primitives — NO new Sequencer public
     method (avoids redundant Class-4 sequencer touch).
     +16 R4 tests; cargo test --workspace 1038/1/150.
     Commit d34f428 via merge 41aae74; log ae7681f.

  R5 (Class 3): audit-tape sampler reaches AttemptTelemetry +
     LeanResult mathematical content.
     audit_assertions.rs +5 Layer G/H assertions:
       assert_44_attempt_telemetry_retrievable_from_cas (FR-18R.7) +
       assert_45_lean_result_retrievable_from_cas (FR-18R.7) +
       assert_46_attempt_chain_root_schema_well_formed (FR-18R.8) +
       assert_g_markov_cluster_source_attempt_telemetry (FR-18R.6) +
       assert_47/48 Layer H tamper stubs (FR-18R.7).
     cas/store.rs +list_cids_by_object_type Class 3 helper.
     +9 R5 tests; cargo test --workspace 1047/1/150.
     Commit 5a09e2d.

  R6 (Class 3 evidence): P23 + P38 + P49 rerun on corrected
     substrate. handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/.
     Per-run chain_invariant.json: R4 invariant verdict per problem
     (delta=0 for clean halt; aborted=0; halt class). audit_tape
     verdict.json with R5 assertions 44/45/46 PASS confirmation.

  R7 (Class 3 evidence): M0 small batch (5 problems) on corrected
     substrate. handover/evidence/tb_18r_r7_m0_2026-05-06/.
     Same per-run shape as R6.

OBS forward-binding (G2 MUST cover):

  1. handover/alignment/OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06.md —
     R3 between-gate Codex audit infra-failed (PID killed silently
     mid-investigation). G2 MUST scrutinize R3 + R3.fix as a unit.
     Two preflight deviations specifically flagged:
       a) R3 §3.5 omega-path NO cutover (preserves TB-7 audit chain
          backward compat; AttemptTelemetry coexists with
          ProposalTelemetry).
       b) R3 §1.3 step_partial_ok stays CAS-only (LeanPass-on-
          rejection-fence-respect; failure-paths-only routing to
          L4.E).

  2. handover/alignment/OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md —
     SG-18R.9 full §17 dashboard DAG render deferred to forward TB.
     R5 closes SG-18R.9 at smoke level via
     tests/tb_18r_dashboard_attempt_dag_replay.rs. G2 MUST rule on
     whether smoke-level closure is acceptable for SG-18R.13.

Your audit MUST address (15 questions):

  Q1  Does R1 schema (AttemptTelemetry + LeanResult +
      TerminalAbortRecord) preserve `feedback_chaintape_externalized_proposal`
      ("1 LLM call = 1 Attempt Node") per charter Q2 PASS?

  Q2  Does R2 evaluator wire-up at all 6 paths preserve CR-18R.4 v2
      (no raw LLM response in candidate_payload_cid; parsed external
      candidate bytes only; fixed sentinels on parse_fail / llm_err)?

  Q3  Does R3 RejectionClass tail-append {6,7,8,9} preserve byte-
      stable canonical hash for pre-R3 L4.E rows? (Codex Q8 ratified
      stable-repr-u8 invariant.)

  Q4  R3 §3.5 omega-path NO cutover (per OBS forward-binding 1.a):
      does this deviation align with Art.0.2 + Art.III.4 + the
      TB-7 audit chain backward-compat constraint? Or should the
      omega path also cut over to AttemptTelemetry CID?

  Q5  R3 §1.3 step_partial_ok CAS-only (per OBS forward-binding 1.b):
      is LeanPass-on-rejection-fence-respect correctly applied?

  Q6  Does R3.fix CasStore::reload_index_from_sidecar correctly
      close the stale-cache bug without introducing new race
      conditions? Lock promotion sequence in refine helper:
      drop read → acquire write → reload → drop write → re-acquire
      read → retry. Is single-writer-per-sequencer-spec sufficient
      to prevent deadlock?

  Q7  R4 G1 equation populated verbatim (charter §0.A Q1+Q4):
      verify NO alteration. Specifically:
        a) `evaluator_reported_completed_llm_calls ==
           l4_work_attempt_count + l4e_work_attempt_count` is the
           exact equation (no ±N tolerance, no approximation).
        b) Auxiliary: `expected + aborted == l4 + l4e` for abort
           halt.
        c) Negative delta forbidden under any halt class.

  Q8  R4 drain barrier discharged via ChaintapeBundle::shutdown()
      without new Sequencer public method: does this satisfy the
      "or equivalent" charter language per FR-18R.3 v2? Or must a
      new Sequencer::drain_until_quiescent() exist?

  Q9  R5 sampler privacy fence (CR-18R.4 v2): does
      assert_44_attempt_telemetry_retrievable_from_cas correctly
      avoid byte inspection of candidate_payload? Verify the
      assertion API never exposes the bytes.

  Q10 R5 attempt_chain_root schema validity (assert_46): is the
      Option<Hash> round-trip + zero-bytes admissible per R3 §3.5
      amended? Verify schema-only test does NOT require populated
      attempt_chain_root on actual omega-path WorkTx.

  Q11 SG-18R.9 smoke-level closure (per OBS forward-binding 2): is
      R5's dashboard binary smoke test acceptable for ship-gate or
      must full §17 DAG render land in R5 before SHIPPED FINAL?

  Q12 R6/R7 evidence: does each per-run chain_invariant.json
      satisfy R4 invariant equation? Specifically delta=0 for
      clean-halt class and named abort with TerminalAbortRecord
      otherwise.

  Q13 R6/R7 evidence: does audit_tape verdict.json show R5
      assertions 44/45/46 PASS on real chain data (not just unit
      test fixtures)?

  Q14 No retroactive M1 evidence rewrite (CR-18R.1 +
      `feedback_no_retroactive_evidence_rewrite`): verify R0-R7
      diffs do NOT modify pre-TB-18R M1 evidence at
      handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/.

  Q15 Class-4 carve-out compliance: R1 + R3 + R3.fix + R4 each
      passed STEP_B preflight + parallel-branch worktree. Verify
      no Class-4 surface hides inside Class-3 atom (R2 + R5).

Verdict format:
  PASS / CHALLENGE / VETO with per-Q breakdown.
  CHALLENGE remediations should be actionable (what code/test/doc
  changes; which atom).
  VETO blocks SHIPPED FINAL; user can authorize remediation atom
  after VETO.

Conservative ranking applies per `feedback_dual_audit_conflict`:
  VETO > CHALLENGE > PASS.
```

---

## §2 Forward-binding to architect § sign-off (TB-17 / TB-18 PROVISIONAL pattern)

Per `feedback_audit_after_evidence` + TB-17 §8 PROVISIONAL pattern: TB-18R
ship status is **PROVISIONAL** until architect signs off on G2 dual-audit
verdict. Following this dispatch, the SHIPPED FINAL gate is:

1. G2 dispatch (Codex + Gemini) — this doc.
2. G2 verdict reception.
3. Architect § sign-off on verdict (conservative ranking applied).
4. Ship doc commit + TB_LOG SHIPPED row + memory updates.

## §3 Cross-references

  - Charter: handover/tracer_bullets/TB-18R_charter_2026-05-06.md.
  - G1 audit: handover/audits/CODEX_TB_18R_CHARTER_RATIFICATION_2026-05-06.md.
  - Preflights:
    - R1: handover/ai-direct/TB-18R_R1_STEP_B_schema.md.
    - R3: handover/ai-direct/TB-18R_R3_STEP_B_admission.md.
    - R3.fix: handover/ai-direct/TB-18R_R3FIX_STEP_B_cas_reload.md.
    - R4: handover/ai-direct/TB-18R_R4_STEP_B_invariant.md.
    - R5: handover/ai-direct/TB-18R_R5_preflight_audit_extension.md.
  - OBS forward-bindings:
    - handover/alignment/OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06.md.
    - handover/alignment/OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md.
  - Evidence:
    - R6: handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/.
    - R7: handover/evidence/tb_18r_r7_m0_2026-05-06/.

**End of G2 dispatch ask. Awaits Codex + Gemini verdicts.**
