# Codex Paper 1 v2.1 Adversarial Audit (round 3)
**Date**: 2026-04-25
**Target commit**: d349a86
**Verdict**: PASS

---

## Executive summary

- v2.1 closes the two biggest round-2 blockers cleanly: it documents the `mathd_algebra_246` drift event in its own subsection and gives the hard-9 restatement (`0/36` for A vs `8/36` for B, same `b=8, c=0, p=0.00391`) at [handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:203-230](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:203).
- The main causal/claim-language cleanup also landed: the abstract and discussion now frame the result as a gain from “a portfolio of prompts including one meta-cognitive instruction,” and explicitly deny a cleanly isolated “generic heterogeneity” claim at [handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:40](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:40) and [handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:260-272](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:260).
- The numbers in the paper match the machine-readable aggregate and the raw v2 bundle: pooled `B_vs_A` is `b_x_only=8`, `c_y_only=0`, one-sided `0.003906`; `Abl_vs_A` is `6/0`, `0.015625`; `B_vs_Abl` is `4/2`, `0.34375`; all 12 raw v2 jsonl files exist under `handover/evidence/v2/`, 10 rows each, BUILD_SHA `29ab43a` [E1v2_RESULTS_2026-04-24.json:357-395](/home/zephryj/projects/turingosv4/handover/preregistration/E1v2_RESULTS_2026-04-24.json:357), [E1v2_RESULTS_2026-04-24.json:15-209](/home/zephryj/projects/turingosv4/handover/preregistration/E1v2_RESULTS_2026-04-24.json:15).
- Residual issues are now mostly P1 hygiene, not submission-blocking science: the family language is still internally inconsistent (`family size = 3` in §3.6, `family=4` in Table 4.1, “pre-registered family of three” in the abstract) at [handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:109-123](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:109), [handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:145](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:145), [handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:40](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:40).
- Reproducibility is improved but not perfectly cleaned: the raw jsonl move is real [handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:338-348](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:338), but the paper still says the release tag is “to be cut” and Appendix C still contains a `.claude/worktrees/...` path while claiming `.lean` artifacts live in `handover/evidence/v2/` [handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:298-299](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:298), [handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:348](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:348), [handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:413-426](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:413).

## STAT — partially closed (residual P1)

v2.1 fixes the round-2 statistical blocker that mattered most for the headline: the hard-9 restatement is present, and the raw aggregate still supports `B > A` with `b=8`, `c=0`, one-sided `p=0.003906` [paper:203-230](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:203), [results JSON:357-369](/home/zephryj/projects/turingosv4/handover/preregistration/E1v2_RESULTS_2026-04-24.json:357). The family is now explicitly declared in §3.6, which is real progress [paper:107-123](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:107).

Residual gap: the paper still has internal multiplicity wording drift. §3.6 says “family size = 3” but keeps `α = 0.0125` as a carry-over from the original family-of-4 plan; Table 4.1 still labels the threshold as “family=4”; the abstract says “pre-registered family of three hard-set tests,” which is not literally true because §3.6 itself calls the 3-test family a post-hoc clarification [paper:109-123,145,40](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:109). I would not block submission on this because the primary endpoint clears `0.0125` under any family size from 1 to 4, but the wording should be made self-consistent.

Residual gap: Limitation 11 still defers the problem-cluster sensitivity analysis instead of running it [paper:288-289](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:288). I would keep this as P1, not promote it to P0, because the claim is now scoped to the paired design and the directional evidence is stable across all four seeds.

## DESIGN — closed

The round-2 design concern was the contaminated “hard” sample and the status of the easy control. v2.1 addresses both directly. The drifted problem is surfaced, explained, and separated from the robust headline in §4.7 [paper:203-230](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:203). The easy-set is explicitly demoted out of the inferential family and labeled historical/descriptive only [paper:117-120,179-181,287](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:117), which resolves the specific round-2 complaint rather than pretending the control is v2-clean.

The remaining design limitations are now honestly framed as scope conditions: a hard pool filtered from Phase 9.A failures, one model, one benchmark [paper:61-64,283](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:61).

## CAUSE — partially closed (residual P1)

The major round-2 causal overclaim is fixed. The abstract no longer says the primary effect is attributable to generic prompt heterogeneity, and §6.1 now says the defensible claim is “heterogeneity plus a meta-cognitive prompt helps” while decomposition remains unresolved [paper:40,260-272](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:40).

Residual gap: §2 still says the study is “isolating prompt diversity from all other variables” [paper:70](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:70). That sentence is inconsistent with the paper’s own Limitation 12 and prompt-leakage caveat, which explicitly admit the meta-cognitive/object-level asymmetry [paper:270-289](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:270). This is a wording-cleanup issue, not a fresh P0.

## LEAKAGE — partially closed (residual P1)

v2.1 now squarely discloses the key leakage/confound issue: the Meta-Planner prompt is a higher-abstraction instruction and the experiment cannot distinguish peer-level heterogeneity from adding one meta-cognitive layer [paper:164-177,270-289](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:164). That is the right scientific posture.

Residual gap: the paper still does not include the per-condition token-budget table that round 2 deferred, and the “same model/prompt everywhere except the skill-description string” line remains stronger than the actual fairness analysis [paper:52,272-289](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:52). I would keep the token-table item deferred; it should not be promoted to P0 so long as the wording stays careful.

## REPRO — partially closed (residual P1)

The important reproducibility improvements are real. The paper now points the raw v2 logs to `handover/evidence/v2/` [paper:340-343](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:340), and on-disk inspection at this commit confirms exactly 12 files there, matching the expected `(4 seeds × 3 conditions)` bundle. The aggregate JSON also shows zero measurement errors and BUILD_SHA `29ab43a` across runs [results JSON:15-209,408-411](/home/zephryj/projects/turingosv4/handover/preregistration/E1v2_RESULTS_2026-04-24.json:15).

Residual gap: artifact stabilization is not fully finished. The paper still says “final tag pending” and “to be cut” for `paper1-v2.1` [paper:298-299,348](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:298), and `git tag --points-at d349a86` is empty as of 2026-04-25. Appendix C still uses a `.claude/worktrees/phase-8a-snapshot/...` jsonl path [paper:418](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:418), which undercuts the “stable paths” story. Also, the paper says per-problem proof artifacts “live at `handover/evidence/v2/<problem>_s<seed>_B.lean`” and that proof `.lean` files are archived there [paper:413,426](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:413), but `handover/evidence/v2/` at this commit contains only the 12 `.jsonl` files. That is a real reproducibility overstatement, but I would still score it P1 rather than P0 because the primary raw logs are now in-repo and the paper no longer depends on Docker for minimal reproduction.

## CLAIM — partially closed (residual P1)

The high-risk round-2 claim issues are fixed where they mattered most. The “tripled absolute solve count” framing is gone from the abstract and §4.1, replaced with the drift-robust “4 distinct problems never solved by A” language [paper:40,147-149,218-230](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:40). The discussion now explicitly rejects both “generic peer-level prompt heterogeneity” and “Meta-Planner is the mechanism” [paper:262](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:262).

Residual gap: there are still a few claim-strength mismatches between cautious sections and overstated lines elsewhere, chiefly the abstract’s “pre-registered family of three hard-set tests,” §2’s “isolating prompt diversity from all other variables,” and the Appendix C proof-location claim [paper:40,70,413-426](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:40). Those should be tightened before any camera-ready version, but they no longer overturn the paper’s central empirical claim.

## Per-P0-blocker closure assessment

### v2.1-P0-A — closed

Closed as requested. v2.1 adds a dedicated drift subsection for `mathd_algebra_246`, calls it a “real, observed model-drift event,” and provides the hard-9 restatement with `A = 0 / 36`, `B = 8 / 36`, unchanged `b=8, c=0` [paper:203-230](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:203). The raw evidence also supports the before/after drift table: in v2 A, `mathd_algebra_246` is solved on all four seeds, while Phase 9.A failed on seeds `31415` and `2718` and solved on `141421` [results JSON:16-209](/home/zephryj/projects/turingosv4/handover/preregistration/E1v2_RESULTS_2026-04-24.json:16), with the corresponding phase-9A files under `handover/evidence/phase9a_jsonl/`.

### v2.1-P0-B — closed

Closed. The forbidden attribution sentence is gone. The abstract now says the finding is from “a portfolio of prompts including one meta-cognitive instruction” and “NOT a cleanly isolated ‘generic heterogeneity’ effect” [paper:40](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:40). §4.2.1 and §6.1 repeat the same narrower interpretation [paper:164-177,260-272](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:164).

### v2.1-P0-C — closed

Closed. The “tripled absolute solve count (3x from 4 to 12)” headline is cut from the abstract and §4.1. In its place, the paper now states that B solved “4 distinct hard problems” never solved by A and explicitly warns that `12/40 vs 4/40` “should not be interpreted as a 3× effect size” because of drift [paper:40,147-149,218-230](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:147).

### v2.1-P0-D — partially closed (residual P1)

Mostly fixed, but not perfectly. v2.1 does declare a closed family explicitly in §3.6 and transparently labels it a “post-hoc clarification of the pre-reg” [paper:109-123](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:109). That addresses the substantive round-2 critique.

Residual issue: the wording is still internally inconsistent. The abstract says “pre-registered family of three hard-set tests” [paper:40](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:40), but the prereg itself says family-of-4 [prereg:76-84](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_E1V2_HETEROGENEITY_2026-04-23.md:76), and Table 4.1 still says “family=4” [paper:145](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:145). This is now a cleanup issue, not a blocker.

### v2.1-P0-E — partially closed (residual P1)

Partially fixed. The raw v2 jsonl files are no longer only in `.claude/worktrees`; the stable in-repo copies exist under `handover/evidence/v2/` and are documented in §8.5 [paper:338-343](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:338). That closes the most important part of the blocker.

Residual issue: the “final commit/tag” part is not done yet. The paper still says the final tag is pending/to be cut [paper:298-299,348](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:298), and Appendix C still references `.claude/worktrees/...` [paper:418](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:418). I would not block submission on this alone, but the reproducibility appendix should be cleaned before treating the artifact bundle as final.

## Opinion on deferred v2.2 items

- P1-A cluster sensitivity: keep deferred, do not promote to P0. Limitation 11 is explicit, and the main paired claim remains robust under the hard-9 restatement and the 4/4-seed directional consistency [paper:288-289,220-230](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:288).
- P1-D token-budget table: keep deferred, do not promote to P0. It would strengthen fairness analysis, but the paper now admits the meta-cognitive confound directly [paper:270-289](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:270).
- P1-E Docker transcript: keep deferred, do not promote to P0. Docker is framed as convenience-only, and the minimal host-local reproducer is the actual primary path [paper:301-330](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:301).
- P2-B Appendix C node-count / winning-agent extraction: keep deferred, do not promote to P0. It affects interpretation of “multi-agent collaboration,” not the primary `B > A` solve-rate result, though the appendix path claims should be cleaned now [paper:197-201,282,410-426](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:197).

## New flaws discovered in round 3

1. **New P1 STAT/CLAIM inconsistency**: the paper simultaneously says “family size = 3” (§3.6), “family=4” (Table 4.1), and “pre-registered family of three hard-set tests” (abstract) [paper:109-145,40](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:109). This does not change the primary accept/reject decision, but it is still an avoidable internal contradiction.
2. **New P1 CAUSE wording slip**: §2 says the study isolates prompt diversity “from all other variables” [paper:70](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:70), which conflicts with the paper’s own prompt-asymmetry caveat and Limitation 12 [paper:270-289](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:270).
3. **New P1 REPRO mismatch**: Appendix C still contains a `.claude/worktrees/...` extraction path and claims proof `.lean` files are in `handover/evidence/v2/` [paper:413-426](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:413), but that directory at this commit contains only the 12 raw `.jsonl` files. The paper should either move the proof artifacts there or correct the location claim.

## Final

VERDICT: PASS

Deferred-item opinion: keep P1-A, P1-D, P1-E, and P2-B deferred; none should be promoted to round-3 P0, but the family-language cleanup and Appendix C artifact-path cleanup should happen in the next revision.
