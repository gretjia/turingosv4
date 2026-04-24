# Codex Paper 1 v2 Round-2 Adversarial Audit

Date: 2026-04-24  
Target commit: `210f19b`  
VERDICT: CHALLENGE

## 1. STAT — statistical: multiplicity, power, exact-test assumptions, family-of-4 definition

The primary A/B result is no longer the central blocker: the aggregate B vs A table reports 12/40 vs 4/40 with b=8, c=0, one-sided p=0.00391 and two-sided p=0.00781, under alpha=0.0125 (`handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:125-140`; JSON confirms `b_x_only=8`, `c_y_only=0`, `rejects_null...=true` at `handover/preregistration/E1v2_RESULTS_2026-04-24.json:357-370`). That is a real improvement over v1.

The weakness is that the statistical frame is still underspecified for repeated problem identities. The 40 paired trials are 10 problems crossed with 4 routing seeds, not 40 independently sampled problems. The draft acknowledges reused-problem clustering as a prior fix and says it will add a problem-level clustered sensitivity analysis (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:21-23`), but no such analysis is actually reported in Results. Since the distinct B-minus-A solve set is only four problem identities (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:180-186`), the inference could be fragile to problem-level clustering. Hypothesis: the primary McNemar p-value will remain directionally favorable, but the paper needs a problem-cluster bootstrap or a mixed model with problem random effects before claiming robustness.

The family-of-4 definition remains internally inconsistent. The prereg says family size is “2 primary inferential tests + 2 secondary inferential tests,” including per-seed containment (`PREREG_E1V2_HETEROGENEITY_2026-04-23.md:76-84`), while the endpoint list calls per-seed containment “Exploratory, NOT inferentially tested” (`PREREG_E1V2_HETEROGENEITY_2026-04-23.md:40-42`). The draft then lists secondary endpoints where easy-set and per-seed dominance are exploratory (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:107-116`). This does not invalidate the conservative alpha, but it makes the prereg look patched rather than principled. State a single closed testing family and mark the rest descriptive.

Power is too close for the ablation claims. Abl vs A is 10/40 vs 4/40, b=6, c=0, p=0.01563, missing Bonferroni (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:142-155`; JSON `handover/preregistration/E1v2_RESULTS_2026-04-24.json:371-383`). B vs Abl is null, b=4, c=2, p=0.34375 (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:157-168`; JSON `handover/preregistration/E1v2_RESULTS_2026-04-24.json:384-396`). The draft mostly respects this, but “primary effect is attributable to generic prompt heterogeneity” in the abstract overreads a secondary test that missed the declared threshold (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:40`).

## 2. DESIGN — sample construction, pool filtering, negative controls, ablation scope

The hard10 draw is now much better documented: hard36 was frozen, sampled with `random.Random(20260423)`, and fingerprinted (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:84-88`, `PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:307-357`; prereg `PREREG_E1V2_HETEROGENEITY_2026-04-23.md:44-60`). The residual design issue is upstream pool filtering. The pool consists of problems failed by two prior baseline seeds at a higher cap (`PREREG_E1V2_HETEROGENEITY_2026-04-23.md:48-49`; draft `PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:86-87`, `PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:309-310`). That creates a hard, selected regime where a homogeneous algebra prompt may be especially disadvantaged. This is acceptable if framed as “conditional on this filtered hard pool,” not as a general MiniF2F effect.

The easy-set negative control is too weak for v2. The prereg prediction says easy-set A=B=10/10, delta=0 with pass iff delta<=1 (`PREREG_E1V2_HETEROGENEITY_2026-04-23.md:36-38`), but the draft uses old v1 data, reports A=9/10 and B=9/10, and explicitly does not rerun under the stamped v2 harness (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:170-172`). That is a useful sanity check, not a v2 negative control. Either rerun it or demote it out of the prereg family.

The ablation scope is improved from N=1 to four seeds, but it still removes a role with qualitatively different content and changes the cycling structure from 4 skills across 8 agents to 3 skills across 8 agents (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:89-98`). That means the ablation mixes “Meta-Planner content removed” with “role distribution changed.” It cannot isolate the Meta-Planner role.

## 3. CAUSE — portfolio effect framing and claim scope

“Portfolio effect” is defensible as a descriptive frame if kept narrow: B solves more problem identities than A, and the draft explicitly rejects strict emergence (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:220-228`). The best causal statement is “heterogeneous prompt assignment improved solve rate in this harness/sample.”

The remaining overreach is attribution. The abstract says the primary effect is attributable to generic prompt heterogeneity (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:40`), but the direct non-Meta heterogeneity comparison misses Bonferroni and the Meta-Planner marginal effect is null. The data support B>A, with Abl in between. They do not cleanly decompose the cause into generic heterogeneity versus meta-cognitive prompting versus cycling/coverage effects.

The draft also admits that winning-agent analysis is preliminary and not causal (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:190-193`). Keep it in an appendix only unless finalized.

## 4. LEAKAGE — token budget symmetry, prompt-content fairness, Meta-Planner layer

The prompt-content fairness caveat is now visible, but not solved. A uses only the algebraic tactic prompt; B includes structural, rewriting, and a Meta-Planner instruction that tells the agent to review history and shift tactic family (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:91-97`). That is not merely role diversity; it gives B additional high-level search control. The draft acknowledges this exact confound (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:230-233`, `PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:241-242`), which is good, but the title and abstract still invite readers to interpret the result as prompt heterogeneity simpliciter.

Token budget symmetry is asserted only indirectly through same model, cap, and harness (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:52-55`, `PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:78-82`). I did not see per-condition prompt-token counts, total generated tokens, or API spend/latency distributions in the listed files. Hypothesis: B may consume different context because the Meta-Planner reviews chain state. Add token/accounting tables or avoid “same budget” phrasing.

## 5. REPRO — prereg discipline, BUILD_SHA stamping, artifact links, Docker plausibility

The prereg discipline is improved but not clean. The concurrency policy was absent from prereg and added after a 73% measurement-error incident (`PROXY_SATURATION_FINDING_2026-04-24.md:11-27`, `PROXY_SATURATION_FINDING_2026-04-24.md:88-97`). The deviation is honestly documented, and the final JSON reports zero measurement error by condition (`E1v2_RESULTS_2026-04-24.json:408-412`). That is acceptable, but the paper should state discarded-run paths and exclusion criteria in methods, not just in a side note (`PROXY_SATURATION_FINDING_2026-04-24.md:100-115`).

BUILD_SHA stamping is present in the aggregate rows (`E1v2_RESULTS_2026-04-24.json:27-29`, `E1v2_RESULTS_2026-04-24.json:60-62`, `E1v2_RESULTS_2026-04-24.json:206-208`), but reproducibility claims are muddied by multiple commits: draft status names runtime `29ab43a` (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:5`), code section names main `f874bd8` and runtime `29ab43a` (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:252-257`), while this audit targets `210f19b`. Resolve this before submission with one immutable artifact commit/tag.

Artifact links are not yet reviewer-grade. The draft points raw logs to a `.claude/worktrees/...` path (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:295-298`), which is not a stable archive location. It also says Appendix C/node-count extraction is deferred (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:188-193`, `PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:359-375`). The Dockerfile is described but not evidenced by a build/run transcript in the listed files (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:285-287`).

## 6. CLAIM — abstract/introduction overclaim, limitations completeness

The limitations section is unusually candid and covers many v1 problems (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:236-247`). The remaining issue is the abstract/title/introduction still read stronger than the evidence. “Improves” is acceptable only with the hard-pool qualifier in the title and first sentence. “Triples” is arithmetically true but rhetorically high leverage from 4 to 12 solves; pair it with absolute counts every time (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:50-55`).

The “Full reproducibility” contribution is premature while the easy control is old, artifact paths are unstable, Docker is unverified, and the final line of the draft still says data collection is 50% complete (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:55`, `PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:295-298`, `PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:379`). That stale status line must be removed; it undermines confidence in the draft hygiene.

One-line VERDICT: CHALLENGE

| Priority | Category | Change | Rationale |
|---|---|---|---|
| P0 | STAT | Add the promised problem-cluster sensitivity analysis or demote exact McNemar to the primary descriptive test with clustering caveat. | Current p-value treats 40 problem-seed pairs too independently for a 10-problem repeated design. |
| P0 | CLAIM | Cut “primary effect is attributable to generic prompt heterogeneity.” | Abl vs A misses Bonferroni; B vs Abl is null; causal decomposition is unresolved. |
| P0 | REPRO | Replace `.claude/worktrees` artifact paths with stable repo/archive paths and publish one final commit/tag. | Current evidence links are not reviewer-reproducible. |
| P1 | DESIGN | Rerun easy-set control under v2 stamped harness or remove it from the inferential family. | Reused v1 A=9/10, B=9/10 is not the preregistered v2 negative control. |
| P1 | STAT | Reconcile the family-of-4 definition. | Prereg simultaneously labels per-seed containment inferential and exploratory. |
| P1 | LEAKAGE | Add prompt-token/context/generated-token accounting by condition. | “Same budget except skill string” is not demonstrated. |
| P1 | REPRO | Add Docker build/run transcript or demote Docker claim. | Dockerfile plausibility is asserted, not verified in read artifacts. |
| P2 | CLAIM | Remove stale “data collection 50% complete” footer. | It contradicts the filled numeric draft. |

Top 3 must-fix items before arXiv submission:

1. Add problem-cluster sensitivity analysis for the A/B result.
2. Stabilize and verify the evidence bundle: final commit/tag, raw logs, proof files, Docker transcript.
3. Rewrite abstract/introduction so only B>A is confirmatory; all mechanism and generic-heterogeneity decomposition is exploratory.

Specific claim to cut entirely: “The primary effect is attributable to generic prompt heterogeneity” (`PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:40`).
