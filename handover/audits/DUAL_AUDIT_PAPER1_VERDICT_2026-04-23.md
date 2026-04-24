# Paper 1 Dual-Audit Verdict — 2026-04-23

**Reviewers**: Codex (structured 6-category scan) + Gemini 2.5 Pro (independent 6-category scan)
**Inputs**: `PAPER_1_FULL_DRAFT_2026-04-23.md` + `E1_FINAL_4SEEDS_2026-04-23.md` + evidence archive + our own adversarial audit
**Rule** (CLAUDE.md § Audit Standard): VETO > CHALLENGE > PASS

---

## § 1. Verdicts

| Reviewer | Verdict | Key demand |
|---|---|---|
| **Codex** | CHALLENGE | Demote mechanism claim, fix McNemar label, tactic-balanced controls |
| **Gemini 2.5 Pro** | CHALLENGE | Justify N=10 selection, demote "emergence" language, expand ablation |

**Conservative verdict: CHALLENGE (dual-confirmed).**

Paper 1 is NOT ready for arXiv submission. Revisions required before next audit round.

---

## § 2. Merged required changes (by priority)

Both reviewers raised these independently — strong signal each is a genuine weakness.

### P0 — blockers (must fix before resubmit)

| # | Theme | Action | Source |
|---|---|---|---|
| P0-1 | Problem selection transparency | Document HOW the 10 hard problems were picked from the 36-problem pool. Ideally: pre-registered random sampling with seed + script. If no pre-reg possible, redo on full 36. | Codex DESIGN-1 + Gemini DESIGN-1 |
| P0-2 | McNemar stat labeling | Label p=0.0195 explicitly as **one-sided** (directional, B > A alternative). Also report two-sided p=0.0391. Declare the primary endpoint + multiplicity-testing family. | Codex STAT-1 + Codex STAT-3 |
| P0-3 | Demote "emergence" / "swarm intelligence" language | Replace with "performance gains from prompt heterogeneity" or "portfolio effect from diverse role prompts". The data shows synergy, not emergence in the strict sense. | Gemini CAUSE-1 + Gemini CLAIM-2 + Codex CLAIM-1 |
| P0-4 | Demote mechanism claim to exploratory | Delete "This identifies the meta-strategic role … as the mechanism for hard-problem emergence" (abstract + § 1.2 item 2). Replace with "One-seed ablation (N=1 seed) shows **removing** Meta-Planner reduces solves on seed 141421 from 3 to 2; both heterogeneity and Meta-Planner may contribute." | Codex CAUSE-1/2 + Codex CLAIM-2 + Gemini CAUSE-2 + Gemini STAT-2 |
| P0-5 | Expand ablation to 4 seeds or label as exploratory | Either: run EXCLUDE_META_PLANNER on seeds 31415 + 2718 + 2357 (N=+30 more trials, $6, ~1.5h); OR move ablation to § 7 Future Work. | Codex CAUSE-2 + Gemini STAT-2 + Gemini CLAIM-2 |

### P1 — required revisions

| # | Theme | Action | Source |
|---|---|---|---|
| P1-6 | Prompt leakage — Meta-Planner is meta-cognitive | Acknowledge Meta-Planner prompt is not "symmetric with skill_0"; it introduces meta-level strategy instruction ("review chain", "propose family shift") absent from object-level skills. Either run tactic-balanced controls (pure-tactic Meta-Planner vs. meta-instruction Meta-Planner) or demote language. | Codex LEAKAGE-1/2/3 + Gemini LEAKAGE-1 |
| P1-7 | TuringOS substrate relevance | Either (a) show the substrate was necessary for the result (e.g., simple-Python-loop baseline), or (b) demote substrate from "contribution" to "engineering infrastructure description". Currently reads as "two separate papers." | Gemini CLAIM-1 + Gemini DESIGN-2 |
| P1-8 | Strict-containment accuracy | Update abstract + § 4.2: "B strictly dominates in 3/4 seeds and dominates on aggregate (7/40 vs 14/40, McNemar one-sided p=0.0195 / two-sided p=0.0391)". Cannot claim strict containment without seed-2357 caveat. | Gemini CLAIM-3 + Codex CLAIM-2 |
| P1-9 | Problem-cluster sensitivity analysis | Add mixed-effects or problem-cluster bootstrap to quantify non-independence of reusing 10 problems × 4 seeds. | Codex STAT-2 |
| P1-10 | Clarify hard-set construction + pool origin | List all 36 problems in supplementary; list selection method for 10 explicitly. | Codex DESIGN-1 + Gemini REPRO-2 |
| P1-11 | Build provenance (commits + model snapshot) | Tag a single release commit for the paper; specify deepseek-chat snapshot date/version. § 1.3 says "snapshot referenced in § 5" but § 5 has no such reference — dangling pointer. | Codex REPRO-1 + Gemini REPRO-1/3 |

### P2 — acceptable-with-revision

| # | Theme | Action | Source |
|---|---|---|---|
| P2-12 | Separate solve-set claim from multi-node-tape-chain claim | Report node-count distribution; clarify only `mathd_algebra_44` × 3 and `imo_1962_p2` × 2 are genuine multi-node chains. | Codex REPRO-3 |
| P2-13 | Evidence archive index | Update `handover/evidence/README.md` to include seed 2357 + ablation batches. Current README maps only 8 original E1 batches. | Codex REPRO-2 |

---

## § 3. Both reviewers' specific-cut recommendations

- **Codex**: cut "This identifies the meta-strategic role - not arbitrary heterogeneity - as the mechanism for hard-problem emergence."
- **Gemini**: cut "This is the first application of aerospace-grade MBSE traceability to an LLM system."

Both are defensible cuts. Do both.

---

## § 4. What both reviewers AGREE is actually defensible

- Solve-rate gain from B vs A on this specific hard-10 sample, McNemar one-sided p=0.0195
- Easy-set negative control (Δ=0) as a saturated specificity check
- Honest methodological-discipline disclosure (the 2 constitutional bugs caught during runs are legitimate evidence of the Report Standard working)
- 16/16 Lean re-verify as independent validity proof
- Phase Z/Z' traceability matrix + conformance tests are valid engineering contributions (if demoted from "first application")

---

## § 5. Estimated rework effort

| Task | Effort | Cost |
|---|---|---|
| Ablation on 3 more seeds (P0-5) | ~1.5h wallclock | ~$6 |
| Random-sample 10 of 36 + reseed run (P0-1 + P1-10) | ~2h | ~$8 |
| Draft v2 rewrite (abstract + § 1.2 + § 4.4 + § 7.1 + § 8.5 references) (P0-3, P0-4, P1-6, P1-7, P1-8, P2-12) | ~2h doc | 0 |
| Stat reanalysis with clustering + multiplicity (P0-2 + P1-9) | ~2h | 0 |
| Build provenance rerun (P1-11) | ~2h regen | ~$8 |
| Update evidence README (P2-13) | ~15min | 0 |
| **Total** | **~10h + $22** | |

---

## § 6. Proposed path forward

1. **Accept dual-audit CHALLENGE verdict**; Paper 1 held at "CHALLENGE" status, NOT ready for arXiv
2. **Execute P0 fixes in order**:
   - P0-1: random-sample 10 of 36 using pre-committed seed + script — this subsumes "hard10 bias" concern
   - P0-5: run ablation on remaining 3 seeds for full 4-seed ablation
   - Parallel: P0-2 + P0-3 + P0-4 rewrite in paper
3. **Paper draft v2** addressing all P0 + P1
4. **Second dual-audit round** on v2. If both → PASS or CHALLENGE-with-minor-fixes → arXiv ready.

Conservative timeline: **~2-3 days** of focused work including reruns + redraft + second audit round.

---

## § 7. Specific paper rewrites (already planned)

**Abstract rewrite sketch** (addresses P0-3, P0-4, P1-8):

> Multi-agent LLM systems often fail to outperform a single well-prompted instance of the same model. We show that varying the skill-description prompts across agents in a fixed n=8 swarm produces a measurable solve-rate gain on a selected MiniF2F Lean 4 hard-problem sample. In paired A/B trials across 4 independent Boltzmann routing seeds (30 → 40 trials), a 4-role heterogeneous swarm solves 14/40 problems vs 7/40 for a homogeneous 1-role swarm (McNemar one-sided p=0.0195; two-sided p=0.0391; B strictly dominates solve set in 3/4 seeds). An easy-set negative control (10 problems solved in all baseline seeds) shows Δ=0, consistent with the effect being specific to hard problems rather than generic inflation. A single-seed ablation removing the Meta-Planner role reduces solves 3 → 2 on seed 141421, suggesting the meta-strategic prompt may specifically enable the IMO-class problem; larger ablation needed for conclusive mechanism. We do NOT claim swarm emergence; we claim a portfolio effect from prompt heterogeneity in a multi-agent harness, with residual confounding between "role diversity" and "meta-cognitive prompt content". The underlying TuringOS v4 Rust microkernel and its flowchart-to-code traceability matrix are supporting engineering infrastructure, not core to the experimental claim. All raw data, 16/16 independently Lean-reverified proof artifacts, and a smallest-single-command reproducer are released.

---

## § 8. Sign-off

**Author** (me, Claude Opus 4.7 on behalf of gretjia): acknowledge CHALLENGE. Paper 1 v1 is not arXiv-ready. Executing revisions per § 6 timeline before next audit round.

**No arXiv submission attempted until second dual-audit PASS.**
