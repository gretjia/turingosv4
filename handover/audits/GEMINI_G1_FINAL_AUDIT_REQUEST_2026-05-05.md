# TB-18 Atom G1 — Gemini final ship audit request

**Status**: REQUEST FILED — awaiting Gemini external audit run by user.
**Filed**: 2026-05-05.
**Authority**: TB-18 charter §1.4 SG-18.16 + architect TB-18 ratification ruling §2.1 + Q7. Class 3 dual external audit per `feedback_dual_audit` (kernel-only would be self-audit OK; this is production wire-up + economic-mutator territory → full dual mandatory).

---

## §1 Why this audit exists

This is the **Gemini half** of the G1 dual external audit. The Codex half is filed at `handover/audits/CODEX_G1_FINAL_AUDIT_REQUEST_2026-05-05.md` (read its §1-§6 first; this doc adds Gemini-specific framing).

Per `feedback_dual_audit_conflict`: if Codex and Gemini disagree, conservative verdict (VETO > CHALLENGE > PASS) wins. This is the structural safeguard against single-auditor blind spots.

## §2 Scope (mirrors Codex G1 — see §2 of `CODEX_G1_FINAL_AUDIT_REQUEST_2026-05-05.md`)

Same surface: substrate atoms 0/E/A/H0/D-design/C/B-design/B-impl/F + G0 CHALLENGE-resolved (commit `c9e0dc1`) + atom H M0 retry (commit `2bc712e`) + atom H M1 (this run).

## §3 Audit questions — Gemini-specific framing

Codex's prompt is mechanically focused (literal greps, sha256 recompute, deterministic audit). Gemini's prompt below leans into structural / philosophical questions where Gemini has historically caught issues Codex did not (TB-12 entropy fence; TB-15 Markov inheritance scope).

```text
TB-18 Final Ship Dual Audit — Gemini side. Codex parallel audit covers literal verification + Q1-Q10 in `CODEX_G1_FINAL_AUDIT_REQUEST_2026-05-05.md`. Your job is to add the structural / cross-cutting layer.

Read these inputs first:
  handover/audits/CODEX_G1_FINAL_AUDIT_REQUEST_2026-05-05.md  (Codex scope + Q1-Q10)
  handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_VERDICT_R2_2026-05-05.md  (G0 R2 PASS; what Codex already verified)
  handover/tracer_bullets/TB-18_charter_2026-05-05.md  (binding spec)
  handover/directives/2026-05-05_TB18_CHARTER_RATIFICATION_ARCHITECT_RULING.md  (architect ruling)
  handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/MINIF2F_M1_BENCHMARK_REPORT.md  (M1 report)
  handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/EVIDENCE_INDEX.json  (50-problem catalog)
  constitution.md  (Art. I.2 + Art. II.2.1 + Art. IV)

Verify each:

GQ1. Statistical claims integrity — does the M1 report's framing match what the data supports?
    - The report says "no regression vs M0 retry" because Wilson CI on 17/50 = [22.4%, 47.8%] contains M0's 7/20 = 35%.
      Verify this is statistically valid; do NOT accept the framing on face value.
    - The report says "G0 fix shipped at c9e0dc1 does NOT regress harness behavior" — what would constitute a regression
      that this run would NOT detect? (Type II error analysis.)
    - The cluster solve rates (75% mathd_numbertheory / 43% mathd_algebra / 0% aime+imo+induction+algebra_) — are these
      consistent with the MiniF2F community baseline, or is the spread anomalous?

GQ2. Halt-reason taxonomy semantics (Art. IV):
    - The report classifies all 9 timeouts as WallClockCap. But architect §2.5 (atom A) added DegradedLLM as a 6th
      RunOutcome variant precisely because pure WallClockCap in a slow-LLM scenario is operationally indistinguishable
      from DegradedLLM. Verify: are some of the 9 "WallClockCap" actually DegradedLLM that didn't fire because the
      external SIGTERM killed before the per-LLM-call budget tracker could trigger?
    - If yes, this is a known measurement gap (the tracker's threshold + the external timeout race), but the report
      should disclose this gap, not paper over it.
    - Recommend: VETO if the gap is undisclosed; CHALLENGE if disclosed but ambiguously.

GQ3. assert_27 G0 fix — broader semantic question:
    - The fix verified `cap.terminal_reason.to_run_outcome() == ts.run_outcome`. 24 MaxTxExhausted chains exercise
      this. But the test of the FIX is whether it would catch a NEW class of bug, not just the historical one.
    - Hypothetical: if a future evaluator code path emits TerminalSummary with run_outcome=DegradedLLM but the
      EvidenceCapsule was constructed with terminal_reason=MaxTxExhausted (e.g. capsule built before the halt
      reason was finalized), does assert_27 catch it? Is the projection contract (ExhaustionReason → RunOutcome)
      injective enough to catch every meaningful misalignment, or are there equivalence classes that mask drift?
    - Specifically: ProtocolCollapse → ErrorHalt and SolverGiveUp → ErrorHalt are NOT injective (two reasons project
      to one outcome). Does this matter for the assert_27 contract?

GQ4. Evidence sampling adequacy:
    - 11/50 sampled with full tarball; 39/50 retain only verdict.json + tamper_report + evaluator.stdout.
    - Is this sample sufficient for an auditor to reconstruct the full halt-reason distribution claim? Specifically:
      can a future auditor verify the claim "9 chains had l4_count=2 with no TerminalSummary" without re-running
      the full 50?
    - If a non-sampled chain's verdict.json is sufficient (because tape_root.l4_count + tx_kind_counts are in the
      verdict), then yes. Verify this is the case for at least 5 non-sampled problems.

GQ5. Architect amendment §2.7 lifecycle-append-only invariant:
    - Atom D-design declared this as a TB-19+ Class 4 forward trigger (no impl in TB-18). The M1 batch did NOT
      add any market lifecycle txs (no MarketSeed / CompleteSetMint / CompleteSetRedeem in the per-problem chains).
    - Verify: is the architect §2.7 invariant correctly preserved across M1? (Spot-check 3 problem chains;
      confirm zero market-lifecycle tx kinds.)

GQ6. Charter PRE-17.5 EXCLUSION (architect §B.9.5):
    - PRE-17.5 was explicitly excluded from TB-18. Verify the M1 substrate did not silently re-introduce it
      (e.g. by referencing or invoking the multi-org / multi-market path that PRE-17.5 would have unlocked).
    - Spot-check: any chain that emitted MarketSeed → confirms PRE-17.5 leakage. Expected: zero.

GQ7. Constitution alignment (Art. I.1 + Art. I.2):
    - Art. I.1: "Compile-loop closure: Proposal → Ground-Truth Feedback → Logging → Capability Compilation → ↑H-VPPUT".
      Did the M1 atom H exercise the full compile loop, or only fragments? Specifically:
        Proposal: ✓ (LLM-driven Work tx)
        Ground-Truth Feedback: ✓ (Lean compile via Verify tx → OmegaConfirm or sub-omega)
        Logging: ✓ (chain-backed L4 + CAS)
        Capability Compilation: ⚠️ (n=1 single-agent; no Markov capsule inheritance across problems; M2 territory)
        ↑H-VPPUT: ⚠️ (no cross-problem H-VPPUT measurement; isolated per-problem PPUT only)
      The "Capability Compilation" + "↑H-VPPUT" stages are what M2 (n5+ / 100+ problems / Markov-inheritance enabled)
      will exercise. M1 is foundational but does NOT fully close the compile loop. Verify the report's framing
      acknowledges this scope honestly.

GQ8. ChainTape externalized-proposal discipline (`feedback_chaintape_externalized_proposal`):
    - 1 LLM call → 1 compound payload = 1 Attempt Node. Verify: each per-problem chain's Work tx count is consistent
      with this discipline. (Solved chains have Work=1; some MaxTxExhausted chains may have Work=0 because the LLM
      never produced a substantive tactic block. Verify NO chain has Work > 1 with each Work being a fragment of
      a single LLM response — that would violate the externalization contract.)

GQ9. β-A in-tape resolution honor (architect Q4 STOP gate):
    - Atom F r2 evidence was regenerated post-G0 with β-A FEASIBLE. M1 chains are isolated genesis chains (each
      problem is its own runtime_repo + cas; no Markov inheritance across problems). Verify: no chain re-introduced
      a global LATEST_MARKOV_CAPSULE.txt sidecar; all 50 are pure genesis.
    - Spot-check: `find handover/evidence/tb_18_minif2f_m1_*/ -name "LATEST_MARKOV_CAPSULE.txt"` should return zero.

GQ10. Honesty audit:
    - The report says "Provisional ship status: TB-18 M1 atom H sub-stage 2 evidence is COMPLETE" and forwards
      to G1 + architect § sign-off for terminal ship.
    - The report does NOT claim public-launch-readiness, real-world-formal-verification capability, or comparison
      to community SOTA. Verify: no overclaim. Per `feedback_minif2f_scaling_policy`: M1 is harness-prep, not benchmark.

Output verdict format:
  OVERALL: VETO | CHALLENGE | PASS
  Per-question (GQ1-GQ10): VETO | CHALLENGE | PASS + rationale (Gemini structural perspective).
  Recommended remediations (if VETO/CHALLENGE).

Save your verdict to:
  handover/audits/GEMINI_G1_FINAL_VERDICT_2026-05-05.md
```

## §4 Cross-references

Same as Codex side; see `CODEX_G1_FINAL_AUDIT_REQUEST_2026-05-05.md` §6.

---

**Awaiting external Gemini audit invocation.** Parallel with Codex G1.
