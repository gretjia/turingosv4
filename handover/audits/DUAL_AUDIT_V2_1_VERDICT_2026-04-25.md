# Paper 1 v2.1 Dual-Audit Verdict (Round 3)

**Date**: 2026-04-25
**Target**: `PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md` at commit `d349a86`
**Codex verdict**: PASS
**Gemini verdict**: PASS
**Merged verdict**: **PASS** (per VETO > CHALLENGE > PASS — both at PASS so merge at PASS)

---

## Audit-history arc

| Round | Target | Commit | Codex | Gemini | Merged |
|---|---|---|---|---|---|
| 1 | Paper 1 v1 | `2687882` | CHALLENGE | CHALLENGE | CHALLENGE |
| 2 | Paper 1 v2 | `210f19b` | CHALLENGE | CHALLENGE | CHALLENGE |
| **3** | **Paper 1 v2.1** | **`d349a86`** | **PASS** | **PASS** | **PASS** |

This is the first PASS verdict in the dual-audit history of Paper 1. The paper is **arXiv-submittable** modulo the residual P1 hygiene items below — none of which either auditor would block on, and Gemini explicitly states "Top 3 must-fix items: None. The paper is arXiv-ready."

---

## § 1. Round-2 P0 closure assessment (both auditors agree)

| ID | Round-2 P0 | v2.1 fix | Codex | Gemini |
|---|---|---|---|---|
| v2.1-P0-A | Document `mathd_algebra_246` drift; hard-9 restatement | § 4.7 + drift-robust Table | closed | closed |
| v2.1-P0-B | Cut "primary effect attributable to generic prompt heterogeneity" | abstract + § 4.1 + § 6.1 reframed | closed | closed |
| v2.1-P0-C | Cut "tripled absolute solve count (3× from 4 to 12)" | replaced with "4 B-unique" framing | closed | closed |
| v2.1-P0-D | Reconcile pre-reg family — declare closed testing family | § 3.6 reconciliation note (post-hoc, transparent) | partially closed (P1) | closed |
| v2.1-P0-E | Stabilize artifacts — final commit/tag, move jsonl out of `.claude/worktrees/` | 12 jsonl copied to `handover/evidence/v2/` | partially closed (P1, tag still pending) | closed |

**Both auditors agree all 5 round-2 P0 blockers are closed at the substantive level.** The two "partially closed (P1)" items from Codex are wording/cleanup residuals on items Gemini scored fully closed.

## § 2. Residual P1 items (cross-confirmed by Codex, Gemini explicitly de-gates)

These are NOT round-3 blockers. Codex flags as "should fix in v2.2 / camera-ready" and Gemini explicitly says "should NOT be promoted to P0 status." They are listed here so the user can decide whether to do a tiny `v2.1.1` cleanup pass before tagging `paper1-v2.1` for arXiv.

### Codex-discovered round-3 P1 items (not in round-2)

| ID | Issue | Fix |
|---|---|---|
| v3.1-P1-α | Family wording inconsistent: abstract "family of three", § 3.6 "family size = 3", Table 4.1 "family=4" | Pick one (recommend "family of three with α=0.0125 retained from original family-of-4 plan"); update Table 4.1 caption + abstract |
| v3.1-P1-β | § 2 says study "isolating prompt diversity from all other variables" — contradicts Limitation 12 | Change "from all other variables" → "from all other variables held in the harness" or remove |
| v3.1-P1-γ | Appendix C references `.claude/worktrees/...` extraction path + claims `.lean` files in `handover/evidence/v2/` (only `.jsonl` present there) | Either move proof artifacts to `handover/evidence/v2/proofs/` or correct the path claim |

### v2.2 items both auditors recommend keeping deferred

| ID | Item | Codex stance | Gemini stance |
|---|---|---|---|
| P1-A | Problem-cluster sensitivity analysis | keep deferred | keep deferred |
| P1-D | Per-condition token-budget table | keep deferred | keep deferred (importance diminished after v2.1 reframe) |
| P1-E | Docker build/run transcript | keep deferred | keep deferred |
| P2-B | Appendix C node-count + winning-agent extraction | keep deferred | keep deferred |

## § 3. Submission decision tree

**Path A — Tag and submit immediately** (Gemini-aligned):
- `git tag paper1-v2.1 d349a86`
- Submit to arXiv.
- Address v3.1-P1-α/β/γ + v2.2 items in camera-ready / final-author-version.

**Path B — Quick v2.1.1 cleanup, then tag** (Codex-aligned):
- ~30 min edit pass: fix the 3 wording/path inconsistencies (P1-α/β/γ).
- Re-stat raw jsonl that the abstract claim is internally consistent, no statistical change needed.
- Commit as v2.1.1, tag `paper1-v2.1.1`, submit to arXiv.

Both paths are defensible. Path B is preferable because the inconsistencies are reviewer-visible and trivially cheap to fix; the marginal cost is ~30 min.

## § 4. Notepad / memory updates

- F-2026-04-25-01: Round-3 dual-audit returned PASS/PASS. Paper 1 is arXiv-ready (modulo optional v2.1.1 hygiene pass).
- C-070 (pre-submission dual-audit + pre-reg + multiplicity + N≥3 ablation gate) **validated**: the harness + pre-reg + drift-disclosure regime survived 3 rounds of independent adversarial audit, ending in PASS.
- The `mathd_algebra_246` drift event is now **archived as a paper finding**, not a confound — § 4.7 turns it into a documented model-drift report.

## § 5. Per dual-audit conflict rule

`feedback_dual_audit_conflict.md` says: "VETO > CHALLENGE > PASS — conservative verdict wins." Both at PASS → merged PASS. No conflict to resolve.

The Gemini "no must-fix" verdict and Codex "PASS with 3 P1 cleanups" are **not in conflict**: Codex's P1s are cleanup, not gate. Both auditors land on the same submission decision: arXiv-ready now.
