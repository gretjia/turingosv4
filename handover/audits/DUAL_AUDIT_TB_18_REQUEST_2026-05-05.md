# TB-18 Atom G1 — Full Codex + Gemini ship audit request (post-H-ladder; final ship gate)

**Status**: REQUEST FILED — awaiting Codex + Gemini external audit run by user. Architect §2.1 Q7 verdict: G1 = full Codex+Gemini AFTER H (final ship audit covering ALL atoms incl. M-ladder evidence).
**Filed**: 2026-05-05.
**Authority**: TB-18 charter §1.4 SG-18.16 + architect TB-18 ratification ruling §2.1 + Q7 + §1 Q7 final dual audit timing.

---

## §1 Why this audit exists

Per architect §2.1 + Q7: G0 = pre-H Codex micro-audit; **G1 = post-H full dual audit covering ALL atoms including H M-ladder evidence**. Final ship audit MUST cover H — otherwise the M-ladder benchmark report ships unaudited.

Conservative resolution per `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## §2 Scope — what to audit

ALL TB-18 atoms shipped at G1 invocation time:

| Commit range | Atoms |
|---|---|
| d3c8d78..7bb18b4 | Substrate (Atom 0 / E / A / H0 / D-design / C / B-design) |
| `<G0 verdict commit>` | Atom G0 verdict + any pre-H remediations |
| `<H ladder commits>` | Atom H sub-stages: M0 retry + M1 (50-100, n1/n3) + M2 (100+, n5, observe-only) + BenchmarkManifest + EvidencePackagingPolicy + benchmark report |
| `<TB-18.B-impl commit>` (if landed before G1) | Atom B substantive comprehensive_arena (else atom B = design-complete only) |
| `<atom F commit>` (if landed) | Atom F single-chain 13/13 evidence (depends on B-impl) |
| `<atom G0 micro-audit verdict commit>` | G0 audit response artifact |

## §3 Audit questions (Codex + Gemini prompt template)

```text
TB-18 Final Ship Audit (Codex + Gemini; conservative VETO > CHALLENGE > PASS)
Target: ALL TB-18 commits on branch main (turingosv4 repo); HEAD at audit time.
Architect ruling 2026-05-05 + 2026-05-05 TB-17 ruling are binding spec.

Read:
  handover/directives/2026-05-05_TB18_CHARTER_RATIFICATION_ARCHITECT_RULING.md
  handover/tracer_bullets/TB-18_charter_2026-05-05.md
  handover/manifests/TB-18_BENCHMARK_MANIFEST.json
  handover/policies/TB-18_EVIDENCE_PACKAGING_POLICY.md
  handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_*.md (G0 verdict)
  handover/proposals/TB-18_ATOM_*_DESIGN_*.md
  handover/whitepapers/MINIF2F_M2_BENCHMARK_REPORT.md (if M-ladder ran)

Then verify:

Q1. SG-18.1..16 satisfaction (or honest-deferral):
    - Walk all 16 numbered ship gates per architect §4. Mark each as
      GREEN / RED / DEFERRED-WITH-FORWARD-TRIGGER. RED = ship blocker.

Q2. CR-18.1..10 compliance:
    - Walk all 10 constitutional requirements. Each must hold or be
      explicitly carved out per architect Q3/Q4/Q6 verdicts.

Q3. FR-18.1..10 functional delivery:
    - Walk all 10 functional requirements. Implemented vs deferred status.

Q4. Architect ship-claim narrowing rule (Q2 verbatim):
    - Verify TB-18 ship doc cites narrowed claim if atom D-impl SKIPPED:
      "formal benchmark substrate partially closed; lifecycle-order
      constraint remains Class 4 forward trigger."
    - Same for atom B-impl: "single-chain 13/13 remains as TB-18.B-impl
      follow-up."

Q5. Architect §2.1-§2.10 hidden-issue coverage:
    - §2.1 audit-order (G0 + G1 split): verified.
    - §2.2 BenchmarkManifest: filed and pinned.
    - §2.3 EvidencePackagingPolicy: filed; replay integrity check feasible.
    - §2.4 M0 failure-mode coverage (solved + unsolved + LLM-degraded +
      Lean failure + EvidenceCapsule emission + no fake accepted):
      walk each on M0 retry evidence.
    - §2.5 DegradedLLM evidence emission (no backdoor): cite Atom A wiring.
    - §2.6 deferred-finalize idempotency: 5-gate analysis.
    - §2.7 lifecycle no-overwrite invariant: status + forward trigger.
    - §2.8 one-process-one-chain (atom B): status + forward trigger.
    - §2.9 real-world disclaimer: present in benchmark report.
    - §2.10 benchmark contamination disclosure: present in report.

Q6. Architect §4 SG-18.14 disclaimer presence:
    - Benchmark report contains "Formal benchmark capacity only.
      Not real-world readiness. No real-world domain. No real funds.
      No public settlement." (verbatim per architect §2.9).

Q7. Anything blocked or surprising:
    - Workspace test green (cargo test --workspace --release; canonical
      reporting per feedback_workspace_test_canonical).
    - No regressions vs TB-17 ship 939/0/150 baseline (allowance: TB-18
      adds tests; counts grow).
    - No retroactive evidence rewrite (per feedback_no_retroactive_evidence_
      rewrite).

Output verdict format:
  OVERALL: VETO | CHALLENGE | PASS (conservative outcome wins)
  Codex verdict: ...
  Gemini verdict: ...
  Per-question (Q1-Q7): VETO | CHALLENGE | PASS + rationale
  Recommended pre-ship remediations (if VETO/CHALLENGE).
  Recommendation for architect § sign-off: READY | CONDITIONAL | NOT-READY.
```

## §4 What G1 verdict gates

Per TB-18 charter SG-18.16: "Final Codex + Gemini audit passes under VETO > CHALLENGE > PASS."

- **VETO** (either auditor) → ship BLOCKED until resolved (re-audit cycle).
- **CHALLENGE** (either auditor; not VETO) → ship doc captures CHALLENGE response (commit message + ship doc §appendix); ship proceeds.
- **PASS** (both auditors) → ship clean.

Per `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS — conservative outcome wins. If Codex says PASS but Gemini VETOs, treat as VETO.

## §5 Architect § sign-off (final ship)

Per architect ruling Atom 12 verbatim: "TB-18 不能算完全 shipped，直到 human architect 签署 readiness report" (extending TB-17 §8 pattern). Ship verdict format mirrors TB-17 §8: CONDITIONAL with caveats list.

## §6 Cross-references

- TB-18 charter §1.4 SG-18.16 + SG-18.E
- Architect TB-18 ratification ruling §2.1 + Q7 + §1 (all Q1-Q7) + §4 (FR/CR/SG)
- Memory: `feedback_dual_audit` + `feedback_dual_audit_conflict` + `feedback_audit_after_evidence` + `feedback_audit_loop_roi_flip` (Q7 batch-audit compromise)

---

**Awaiting external Codex + Gemini dual audit invocation.** TB-18 ship CONDITIONAL on G1 verdict + architect § sign-off.
