# TB-18 Atom G1 — Codex final ship audit request

**Status**: REQUEST FILED — awaiting Codex external audit run by user.
**Filed**: 2026-05-05.
**Authority**: TB-18 charter §1.4 SG-18.16 + architect TB-18 ratification ruling §2.1 + Q7 (G0 = pre-H micro-audit; G1 = post-H final ship audit).

---

## §1 Why this audit exists

Architect §2.1 verbatim:
> 如果 G 是 dual external audit, 而 H 是 M-ladder benchmark report, 那么 G before H 不能审计 H. 所以要么 G0 = pre-H micro-audit + H = M-ladder + G1 = final dual audit, 要么直接把 G 移到 H 后.

G0 already shipped (R1 CHALLENGE → R2 PASS). H atom shipped: M0 retry (substrate validation) + M1 50-problem benchmark (this report). G1 is the final dual audit covering the full TB-18 ship surface: substrate (atoms 0/E/A/H0/D-design/C/B-design/B-impl/F) + G0 CHALLENGE-resolved fix (commit `c9e0dc1`) + atom H M1 evidence.

## §2 Scope — full TB-18 ship surface

**Substrate atoms** (already covered by G0 R2 PASS; G1 re-checks integrity):
| Commit | Atom | What |
|---|---|---|
| `d3c8d78` | Atom 0 | TB-18 charter ratified-with-amendments |
| `8ad7a1d` | Atom E | OBS_R023 closure (RunOutcome propagation) |
| `13a5ee0` | Atom A | drive_task API + per-LLM-call budget + RunOutcome::DegradedLLM |
| `5c40d06` | Atom H0 | M0 small preflight (3-problem substrate validation) |
| `c025cdb` | Atom D-design | Class 4 escalation refusal + Path C multi-task verdict |
| `ae9530f` | Atom C | Deferred-finalize idempotency |
| `7bb18b4` | Atom B-design | SharedChain spec + 6-task arena manifest |
| `15b662c` | Atom B-impl | Single-process / single-runtime / single-CAS / 13/13 tx kinds in 2.8s |
| `0c3a5e1` | Atom F | Single-chain 13/13 audit verdict + β-A feasibility FEASIBLE |
| `c9e0dc1` | **G0 CHALLENGE-resolved** | assert_27 reason↔outcome consistency + comprehensive_arena helper parameterized + r2 evidence regenerated |

**Atom H ship surface (NEW IN G1 SCOPE)**:
- M0 retry (`2bc712e`): 20 problems × 120s timeout × n=1; 7 solved + 7 MaxTxExhausted + 6 timeout
- **M1** (this run): 50 problems × 600s × n=1; 17 solved + 24 MaxTxExhausted + 9 WallClockCap
- BenchmarkManifest (`handover/manifests/TB-18_M1_BENCHMARK_MANIFEST.json`, manifest_id `652890ec...`)
- M1 benchmark report (`handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/MINIF2F_M1_BENCHMARK_REPORT.md`)
- Sampled evidence (11/50 with runtime_repo.dotgit.tar.gz + cas.dotgit.tar.gz; per `feedback_evidence_packaging_policy_required`)
- EVIDENCE_INDEX.json catalog (50/50 problems; outcome + halt_reason + sha256)

## §3 Audit questions — Codex G1 prompt template

```text
TB-18 Final Ship Dual Audit (architect §2.1 G1 — Codex side; Gemini runs parallel)
Target: TB-18 substrate (commits d3c8d78..0c3a5e1) + G0 CHALLENGE-resolved (c9e0dc1) + atom H M0 retry (2bc712e) + atom H M1 (commit pending).

Read these inputs first:
  handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_VERDICT_R2_2026-05-05.md  (G0 R2 PASS — re-check don't re-audit)
  handover/tracer_bullets/TB-18_charter_2026-05-05.md  (binding spec)
  handover/directives/2026-05-05_TB18_CHARTER_RATIFICATION_ARCHITECT_RULING.md  (architect ruling)
  handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/MINIF2F_M1_BENCHMARK_REPORT.md  (M1 report)
  handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/M1_RUN_MANIFEST.json  (frozen manifest)
  handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/EVIDENCE_INDEX.json  (50-problem catalog)
  handover/evidence/tb_18_b_phase4_2026-05-05/r2/  (atom B-impl post-G0)
  handover/evidence/tb_18_single_chain_13_of_13/r2/  (atom F post-G0)

Verify each:

Q1. Manifest discipline (FR-18.13 / SG-18.13):
    - manifest_id is deterministic sha256 over canonical JSON minus manifest_id field;
      recompute from M1_RUN_MANIFEST.json and confirm == 652890ecc158d7139fb699b682298605bad3857ad3fb37a74dc6fd5a83fd57af.
    - manifest.turingosv4_commit == c9e0dc1; verify HEAD-at-launch matched.
    - frozen_problems_sha256 in manifest matches sha256(m1_problems.txt).

Q2. Halt-reason taxonomy (Art. IV):
    - 17 OmegaAccepted (l4=5; FinalizeReward present; PPUT_RESULT.solved=true)
    - 24 MaxTxExhausted (l4=3; TerminalSummary tx; capsule reason=MaxTxExhausted)
    - 9 WallClockCap (l4=2; no TerminalSummary; SIGTERM at 600s)
    - 0 ComputeCapViolated, 0 ErrorHalt, 0 DegradedLLM
    - Spot-check: pick 3 problems from each halt class; verify per-problem verdict.json + tx_kind_counts.

Q3. assert_27 production validation (G0 stricter check on real M1 chains):
    - For 24 MaxTxExhausted chains: capsule terminal_reason=MaxTxExhausted, TerminalSummary run_outcome=MaxTxExhausted → consistent.
    - For 17 OmegaAccepted chains: most do not emit TerminalSummary (Omega path); those that do must have consistent capsule.
    - Spot-check: extract at least 1 capsule cid per halt class from the chain; canonical-decode; verify
      cap.terminal_reason.to_run_outcome() == ts.run_outcome.

Q4. Replay determinism + tamper detection (CR-18.5/6):
    - 50/50 verdict.json byte-identical to verdict_replay.json (audit_dashboard --json or per-problem cmp -s).
    - 38/50 tamper 3/3 detected; 12/50 tamper 2/3 (DEGRADED). Verify the 12 split:
      9 WallClockCap (l4=2) + 3 OmegaAccepted (P39/P43/P49). Cross-check: the unwalked-largest-CAS-object root
      cause hypothesis is consistent with the data, OR identify a different root cause.

Q5. Statistical reporting (Art. I.2 + C-053 + C-061):
    - n=50; solved=17; solve rate 34%; 95% Wilson CI [22.4%, 47.8%]; Σ PPUT (solved) = 123.6869; Mean PPUT (solved) = 7.2757.
    - Verify the Wilson CI computation independently (z=1.96; standard formula).
    - Verify Mean PPUT = ΣPPUT / n_solved.
    - Cluster solve rates: mathd_numbertheory 75% / mathd_algebra 43% / amc12 11% / aime/imo/induction/algebra_ 0%.

Q6. n=1 reporting carve-outs (Art. II.2.1 + C-052):
    - parent_selection_entropy + pairwise_payload_diversity_mean correctly reported as N/A (n=1 condition).
    - Reputation distribution correctly reported as N/A (single-agent).
    - Confirm the report does NOT make multi-agent claims that n=1 doesn't support.

Q7. Charter SG closure:
    - SG-18.5 atom H executes — verify 50/50 chain-backed evaluator runs.
    - SG-18.13 BenchmarkManifest pinned — verify gate 4 commit drift check semantics in run_tb_18_atom_h_m1_2026-05-05.sh.
    - SG-18.14 EvidencePackagingPolicy — sample strategy declared; tarballs + sha256 present for sampled set.
    - SG-18.15 G0 audit — ✅ R2 PASS already verified.

Q8. Audit-tape coverage gaps + tamper-DEGRADED root cause:
    - Confirm or refute the M1 report's diagnosis: audit_tape_tamper picks the largest CAS object regardless of audit-walk reachability;
      when that object is not in any assertion's walk path, corruption isn't detected.
    - This is reported as a non-blocker forward-improvement candidate. Verify this characterization is correct, OR escalate if the
      pattern indicates a real verifier deficiency (e.g. an assertion that SHOULD walk that object but doesn't).

Q9. Class envelope (feedback_class4_cannot_hide_in_class3):
    - git diff src/state/sequencer.rs src/state/typed_tx.rs src/kernel.rs src/bus.rs src/sdk/tools/wallet.rs across full TB-18 range
      (d3c8d78..HEAD) returns no canonical-state-mutating changes (Atom A added DegradedLLM enum variant only; canonical-signing-payload
      shape preserved per genesis_payload.toml rehash entries).
    - No new TypedTx variants beyond DegradedLLM RunOutcome discriminant.

Q10. Anything missed:
    - Any SG / FR / CR / Q1-Q7 from the architect's TB-18 ratification ruling §3+§4 that the atom H ship surface forgot?
    - Particularly: PRE-17.5 was EXCLUDED by architect; verify the M1 batch did not silently re-introduce it.

Output verdict format:
  OVERALL: VETO | CHALLENGE | PASS
  Per-question (Q1-Q10): VETO | CHALLENGE | PASS + rationale
  Recommended remediations (if VETO/CHALLENGE).
  
Save your verdict to:
  handover/audits/CODEX_G1_FINAL_VERDICT_2026-05-05.md
```

## §4 What G1 verdict gates

Per TB-18 charter SG-18.16:
- **VETO** → atom H ship status REVERTED; substrate carries forward, M1 evidence quarantined, dual audit re-issue.
- **CHALLENGE** → atom H proceeds with documented response (commit captures CHALLENGE-resolved status).
- **PASS** → atom H + TB-18 unblocked for architect § sign-off.

## §5 How to execute

**This audit cannot be self-issued by the AI-coder.** Per CLAUDE.md + memory `feedback_dual_audit`: external audits are user-invoked.

**To execute G1**:
- User runs `/ultrareview <branch>` OR invokes Codex CLI against the TB-18 ship range with the Q1-Q10 prompt template above.
- Verdict file lands at `handover/audits/CODEX_G1_FINAL_VERDICT_2026-05-05.md`.
- Gemini parallel audit lands at `handover/audits/GEMINI_G1_FINAL_VERDICT_2026-05-05.md` (request doc filed alongside).
- Per `feedback_dual_audit_conflict`: if Codex and Gemini disagree, conservative verdict (VETO > CHALLENGE > PASS) wins.

## §6 Cross-references

- TB-18 charter §1.4 SG-18.16
- Architect TB-18 ratification ruling §2.1 + Q7
- G0 R2 verdict: `handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_VERDICT_R2_2026-05-05.md`
- M1 report: `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/MINIF2F_M1_BENCHMARK_REPORT.md`
- TB-18 commit range: `d3c8d78..c9e0dc1` (substrate + G0 fix) + atom H ship commit (forthcoming)
- Memory: `feedback_dual_audit` · `feedback_dual_audit_conflict` · `feedback_audit_after_evidence` · `feedback_class4_cannot_hide_in_class3`

---

**Awaiting external Codex audit invocation.** Substrate sealed at HEAD `c9e0dc1` + atom H ship commit (pending).
