# Paper 1 v2 Dual-Audit Verdict (Round 2)

**Date**: 2026-04-24
**Target**: `PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md` at commit `210f19b`
**Codex verdict**: CHALLENGE
**Gemini verdict**: CHALLENGE
**Merged verdict**: **CHALLENGE** (per VETO > CHALLENGE > PASS — both at CHALLENGE so merge at CHALLENGE)

Round 1 verdict was CHALLENGE/CHALLENGE (v1, commit `2687882`). Round 2 on v2 is again CHALLENGE/CHALLENGE — the paper is materially better but still not arXiv-submittable without another revision (v2.1).

---

## § 1. Major new finding from Round 2 (Gemini catch)

### 1.1 mathd_algebra_246 anomaly — model drift

Gemini DESIGN-1 flagged that `mathd_algebra_246` solves in 100% of A (homogeneous) runs on all 4 seeds. Investigation confirms:

| Seed | Phase 9.A (2026-04-22/23) | E1 v2 A (2026-04-24) |
|---|---|---|
| 141421 | SOLVED (21.4s, tx=2) | SOLVED (12.7s, tx=1) |
| 31415  | **FAILED** (607s, tx=50) | **SOLVED** (11.5s, tx=1) |
| 2718   | **FAILED** (594s, tx=50) | **SOLVED** (12.6s, tx=1) |
| 2357   | not tested in 9.A | SOLVED (23.3s, tx=2) |

Both Phase 9.A and v2 A use identical harness settings (n=8, tx_cap=50, homogeneous algebraic skill); only the DeepSeek model could have changed. Conclusion: **`deepseek-chat` drifted between 2026-04-22 and 2026-04-24**, making `mathd_algebra_246` solvable from seeds that previously failed. This validates v2 Limitation #9 (model drift unverifiable) as a real, observed phenomenon in this experiment.

### 1.2 Impact on primary endpoint

McNemar is conditional on discordant pairs. `mathd_algebra_246` was **concordant-solved** (both A and B solved it) in all 4 seeds, so excluding it from the analysis changes b=8, c=0 to b=8, c=0 — **primary p-value is unchanged at 0.00391**. Same for Abl-vs-A (p=0.01563) and B-vs-Abl (p=0.34375).

The interpretation changes significantly:
- **Old headline**: "B=12/40 vs A=4/40, 3× tripled solve count" — inflated by a now-easy problem.
- **New headline**: "B solved 4 distinct hard problems A never solved; A solved 0 hard problems B did not" — robust to drift and better descriptive.

---

## § 2. Merged P0 (blockers for arXiv submission)

Union of Codex P0 + Gemini P0, with both flagging similar issues:

| ID | Source | Change | Action |
|---|---|---|---|
| v2.1-P0-A | Gemini DESIGN-1 (P0) | Document mathd_algebra_246 model-drift anomaly in its own § 4.7; present hard-9-subset numbers alongside hard-10-full | Add new subsection to § 4 |
| v2.1-P0-B | Codex CLAIM (P0) + Gemini CAUSE/CLAIM (P0/P1) | Cut "primary effect is attributable to generic prompt heterogeneity" from abstract + § 4.1 | Edit abstract + § 4.1 + § 6.1 |
| v2.1-P0-C | Gemini CLAIM (P0) | Cut "Effect size: tripled absolute solve count (3× from 4 to 12)"; replace with "4 B-unique hard problems" framing | Edit § 4.1 + abstract |
| v2.1-P0-D | Gemini STAT (P0) + Codex STAT (P1) | Reconcile pre-registered family: declare the closed testing family explicitly (3 primary/secondary inferential tests on hard set; easy-set + per-seed = descriptive) | Edit § 3.6 + Limitation |
| v2.1-P0-E | Codex REPRO (P0) | Stabilize artifacts: one final commit/tag, move raw jsonl out of `.claude/worktrees/` | Tag a paper-final commit after v2.1 edits, copy logs into `handover/evidence/v2/` |

## § 3. Merged P1 (strongly recommended before submission)

| ID | Source | Change |
|---|---|---|
| v2.1-P1-A | Codex STAT | Add problem-cluster sensitivity analysis (mixed-logistic OR cluster-bootstrap) OR demote claim to "McNemar under paired design; no independent-problem claim" |
| v2.1-P1-B | Both DESIGN (P1) | Either re-run easy-set under BUILD_SHA 29ab43a, or demote easy-set to "historical descriptive control" (not part of pre-reg inferential family) |
| v2.1-P1-C | Both CAUSE/LEAKAGE | Sharpen causal claim: "portfolio of prompts including one meta-cognitive" — NOT "generic heterogeneity" |
| v2.1-P1-D | Codex LEAKAGE (P1) | Add token-accounting table by condition OR stop claiming "same budget except skill string" |
| v2.1-P1-E | Codex REPRO (P1) | Add Dockerfile build/run transcript or demote Docker section to "optional reproducer" |

## § 4. Merged P2

| ID | Source | Change |
|---|---|---|
| v2.1-P2-A | Codex CLAIM (P2) | Remove stale "data collection 50% complete" footer on final line |
| v2.1-P2-B | Both REPRO (P2) | Complete Appendix C node-count + winning-agent analysis before submission |

---

## § 5. Response plan (this session)

Apply P0 edits (A–E) + P1-C (causal sharpening) + P1-B (easy-set demote) + P2-A (stale footer) immediately. Defer P1-A (problem-cluster analysis), P1-D (token accounting), P1-E (Docker transcript), and P2-B (Appendix C) as labeled "v2.2 polishing" tasks — the paper's *headline statistical claim survives drift* (McNemar unchanged) and is publishable with appropriate scope; the deferred items tighten but do not gate the submission.

**Per the dual-audit conflict rule (feedback_dual_audit_conflict.md)**: when both audits are CHALLENGE, conservative merge = accept every P0 from either. That is what § 2 does.
