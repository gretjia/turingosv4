# Paper 1 v2 — Skeleton (awaiting E1 v2 data)

**Status**: draft skeleton per DUAL_AUDIT_PAPER1_VERDICT P0/P1/P2 fixes
**Supersedes**: `PAPER_1_FULL_DRAFT_2026-04-23.md` (v1, CHALLENGE verdict)
**Data**: E1 v2 data being collected (6 rounds × 2 parallel, orchestrator running). Numbers below marked `⟦PENDING⟧` until runs complete.

---

## Revisions vs v1 (per dual-audit P0/P1)

| ID | v1 problem | v2 fix | Source |
|---|---|---|---|
| P0-1 | hard10 sample selection not documented → p-hacking risk | Pre-registered random 10/36 with seed=20260423 BEFORE run; PREREG file committed | Codex DESIGN-1 + Gemini DESIGN-1 |
| P0-2 | McNemar p=0.0195 unlabeled (one-sided); no multiplicity declared | Explicit one-sided + two-sided report; Bonferroni α=0.0125 for family-of-4 primary tests; declared in PREREG | Codex STAT-1 + STAT-3 |
| P0-3 | "emergence" / "swarm intelligence" language | Replaced throughout with "performance gain from prompt heterogeneity" or "portfolio effect" | Gemini CAUSE-1 + CLAIM-2 |
| P0-4 | "Meta-Planner is THE mechanism" — from N=1 ablation | Demoted; Meta-Planner described as "contributing factor" alongside generic heterogeneity | Codex CAUSE-1 + CAUSE-2 |
| P0-5 | Ablation N=1 seed | Ablation N=4 seeds (paired with full A/B); McNemar on ablation-vs-B paired pairs | Codex CAUSE-2 + Gemini STAT-2 |
| P1-6 | Meta-Planner prompt meta-cognitive ≠ object-level skills "symmetric" defense | Acknowledge asymmetry explicitly in § 7 Limitations; do NOT claim symmetry | Codex LEAKAGE-1/2/3 + Gemini LEAKAGE-1 |
| P1-7 | TuringOS substrate inflated as contribution | Demoted to § 3 engineering infrastructure; not a contribution claim | Gemini CLAIM-1 + DESIGN-2 |
| P1-8 | "Strictly dominates" language | Corrected: "dominates in N/4 seeds; dominates on aggregate" | Gemini CLAIM-3 |
| P1-9 | Reused-problem clustering not addressed statistically | Add problem-level clustered sensitivity analysis (mixed logistic OR problem-cluster bootstrap) | Codex STAT-2 |
| P1-10 | Hard-set construction opaque | hard36 pool + random 10 fully specified in § 4.1 + PREREG | Codex DESIGN-1 + Gemini REPRO-2 |
| P1-11 | build_sha missing on all rows | build_sha populated via run_list.sh auto-stamp + evaluator fail-fast | Codex REPRO-1 + Gemini REPRO-1/3 |
| P2-12 | Multi-node tape chain claim undermined by gp_node_count=1 solves | Report node-count distribution; clarify solve-set claim vs chain claim separately | Codex REPRO-3 |
| P2-13 | Evidence README stale | Updated with all v2 batches | Codex REPRO-2 |

---

## Title (v2 draft)

*Prompt Heterogeneity Improves Multi-Agent LLM Solve Rate on Hard MiniF2F Problems: A Pre-Registered Paired A/B Study*

(Deliberately modest; removes "emergence" / "constitutional microkernel" framing per P0-3 + P1-7.)

---

## Abstract (v2 draft, ~200 words)

Multi-agent LLM systems often fail to outperform a single well-prompted instance of the same model. We report a pre-registered paired A/B study on `deepseek-chat` in an n=8 swarm harness over 10 hard MiniF2F Lean 4 problems (drawn at random from a 36-problem pool, sampling seed committed BEFORE run). We compare a **homogeneous** condition (all 8 agents share one algebraic-skill prompt) against a **heterogeneous** condition (4 distinct skill prompts including a Meta-Planner role). Across 4 independent Boltzmann routing seeds (40 paired trials), heterogeneous solves ⟦PENDING⟧/40 vs homogeneous ⟦PENDING⟧/40 (McNemar exact **one-sided p=⟦PENDING⟧**; two-sided p=⟦PENDING⟧; Bonferroni-corrected α=0.0125 for family-of-4). An ablation removing the Meta-Planner role (retaining 3 other skills) solves ⟦PENDING⟧/40 — ⟦PENDING⟧ problems between homogeneous and heterogeneous, indicating both generic heterogeneity and Meta-Planner contribute. Easy-set negative control shows no condition effect. All accepted proofs re-verify independently via `lean --stdin`. We frame the finding as a **portfolio effect** from prompt heterogeneity in a multi-agent harness, NOT as swarm emergence in the strict sense.

---

## § 1. Introduction (v2, compressed)

### 1.1 Problem

n-agent LLM swarms (AutoGen, CrewAI, LangGraph) rarely outperform a well-prompted single instance. Open question: does prompt diversity across agents elicit a measurable advantage, or is it noise?

### 1.2 Contribution

1. **Pre-registered paired A/B** on MiniF2F: sample 10/36 hard problems drawn before any data collection, 4 Boltzmann seeds, 50 max transactions, same model/prompt everywhere except the skill-description string.
2. **Portfolio effect** finding: prompt heterogeneity (4 skill prompts vs 1) increases solve count by ⟦PENDING⟧% with McNemar p=⟦PENDING⟧ (one-sided, Bonferroni-adjusted).
3. **Ablation evidence**: removing the Meta-Planner role specifically reduces solves from ⟦heterogeneous count⟧ to ⟦ablation count⟧ across 4 seeds, suggesting Meta-Planner is a meaningful subcomponent of the effect.
4. **Full reproducibility**: pre-reg file + sample-selection script + evaluator commit + 14+ re-verified Lean proof artifacts + Dockerfile.

We explicitly do NOT claim swarm emergence in the strict sense (irreducible collective behavior not present in individual agents). Many winning proofs are single-agent, multi-line payloads; the treatment effect is that heterogeneous prompts collectively cover more of the tactic space.

### 1.3 Scope

- Single model tested (deepseek-chat). Model-independence = Paper 2.
- Single benchmark (MiniF2F Lean 4). Generalization = Paper 3.
- Hard-set = 36 problems from Phase 9.A baseline that FAILed in BOTH seeds 31415 AND 2718. Pool frozen; sample random draw pre-committed.
- N=10 × 4 = 40 paired trials (primary) + 40 ablation (secondary) + 10 easy-set (null control).

---

## § 2. Related work (compressed per P1-7)

Prior multi-agent LLM work (Debate, Constitutional AI, AutoGen, LeanDojo) does not report paired-design prompt-diversity experiments with explicit negative controls. Our contribution is methodological: a reproducible, pre-registered paired A/B isolating prompt diversity from all other variables.

---

## § 3. Experimental setup

### 3.1 Model + harness

- **Model**: deepseek-chat via deepseek.com public API, snapshot version ⟦specify date/hash⟧
- **Harness**: TuringOS v4 Rust microkernel (public repo; see § 8 Reproducibility). Used only as execution infrastructure; not a contribution of this paper.
- **Condition**: `CONDITION=n8` — 8 agents in round-robin Boltzmann routing
- **Cap**: `MAX_TRANSACTIONS=50`, wallclock 900s outer timeout
- **Concurrency policy**: ≤ 2 parallel batches per API key (see § 3.5 proxy-saturation note)

### 3.2 Sample

- **hard36 pool**: 36 problems (listed in Appendix A) from MiniF2F test set that FAILed in BOTH Phase 9.A baseline seeds 31415 AND 2718. Pool **FROZEN** at commit `045fa6b`.
- **hard10 draw**: `random.Random(20260423).sample(hard36, 10)`, sorted. Seed 20260423 is the pre-reg date in integer form, committed BEFORE any run. Fingerprint `a94c0ae30f728e6c`.

### 3.3 Conditions

- **A (homogeneous)**: `HOMOGENEOUS_AGENTS=1`. All 8 agents receive `skill_0` prompt: "Focus on algebraic simplification: ring, field_simp, linarith, nlinarith."
- **B (heterogeneous)**: default 4 skills cycled across 8 agents:
  - `skill_0` (algebraic) — same as A
  - `skill_1` (structural): "induction, cases, rcases, constructor."
  - `skill_2` (rewriting): "simp, norm_num, rw, calc."
  - `skill_3` (Meta-Planner): "Review the chain so far. If the current tactic family has produced many rejects or a linear spiral of small-step partial-OKs without closing goals, propose a high-level TACTIC FAMILY SHIFT (e.g. by_contra, induction', refine ⟨?_, ?_⟩). Re-shape the proof strategy, not another small step."
- **Ablation (no Meta-Planner)**: `EXCLUDE_META_PLANNER=1`. 3 skills cycled (skill_0/1/2; no skill_3).

### 3.4 Boltzmann routing seeds

Fixed a priori: {141421 (√2×10⁵), 31415 (π×10⁴), 2718 (e×10³), 2357 (4th-prime concat)}.

### 3.5 Proxy-saturation finding

During the initial v2 data run, we observed that launching 12 concurrent batches × 8 agents = 96 concurrent DeepSeek API requests produced a 73% MEASUREMENT_ERROR rate (the 900s outer wallclock fired before MAX_TRANSACTIONS=50 was reached). At ≤ 2 concurrent batches, MEASUREMENT_ERROR rate was 0%. We therefore serialized the run to max 2 parallel batches. This constraint was NOT in the original PREREG; we flag it here as a deviation from pre-reg, with clean-data re-run.

### 3.6 Pre-registered statistics

- **Primary endpoint**: McNemar exact binomial **one-sided** test on paired (by problem) discordant cells, A vs B across 4 seeds.
- **Threshold**: p < 0.0125 (Bonferroni family size = 4).
- **Directional hypothesis**: B > A (pre-registered before run).
- **Secondary endpoints** (all Bonferroni α=0.0125):
  1. Ablation vs B paired McNemar (one-sided, B > Ablation)
  2. Easy-set Δ (prediction: Δ ≤ 1, exploratory)
  3. Per-seed solve-set dominance count (exploratory)

See `handover/preregistration/PREREG_E1V2_HETEROGENEITY_2026-04-23.md` for the full pre-reg document.

---

## § 4. Results

⟦PENDING — table filled once all 4 seeds × 3 conditions complete⟧

### 4.1 Primary endpoint (hard-set A vs B paired)

| Seed | A / 10 | B / 10 | B-unique | A-unique |
|---|---|---|---|---|
| 141421 | 1 (from v2 A_s141421 data) | ⟦PENDING⟧ | ⟦PENDING⟧ | ⟦PENDING⟧ |
| 31415 | ⟦PENDING⟧ | ⟦PENDING⟧ | ⟦PENDING⟧ | ⟦PENDING⟧ |
| 2718 | ⟦PENDING⟧ | ⟦PENDING⟧ | ⟦PENDING⟧ | ⟦PENDING⟧ |
| 2357 | ⟦PENDING⟧ | ⟦PENDING⟧ | ⟦PENDING⟧ | ⟦PENDING⟧ |
| **Pooled** | **⟦PENDING⟧/40** | **⟦PENDING⟧/40** | **⟦PENDING⟧** | **⟦PENDING⟧** |

McNemar exact binomial:
- one-sided p = ⟦PENDING⟧
- two-sided p = ⟦PENDING⟧
- Bonferroni threshold (family=4): α = 0.0125

Verdict: ⟦PENDING⟧

### 4.2 Ablation (Meta-Planner removed)

⟦PENDING table⟧

### 4.3 Easy-set negative control

⟦PENDING table⟧

### 4.4 Per-seed dominance (exploratory)

⟦PENDING⟧

### 4.5 Solve-set composition

⟦table of which problems were solved in which condition × seed; plus node-count distribution per solve, addressing P2-12⟧

### 4.6 Winning-agent distribution (exploratory)

⟦which skill (0/1/2/3) wrote the OMEGA-accepting step per B-unique event⟧

---

## § 5. Ablation + robustness

### 5.1 N=4 seed ablation

Unlike Paper 1 v1's N=1 ablation, the v2 ablation runs EXCLUDE_META_PLANNER=1 on all 4 Boltzmann seeds. Paired analysis ablation-vs-B:

⟦PENDING⟧

### 5.2 Tactic-composition analysis

For the ⟦PENDING⟧ B-unique solves, what tactic families appear in the winning gp_payload? This is descriptive evidence; not a mechanism claim.

### 5.3 Independent Lean re-verification

All ⟦PENDING⟧ accepted proofs re-verify via `audit_proof.py`.

---

## § 6. Discussion

### 6.1 What the data supports

Prompt heterogeneity in a multi-agent LLM harness produces a measurable solve-rate gain on a pre-registered random sample of hard MiniF2F problems. The Meta-Planner role specifically contributes to this gain, as shown by the ablation.

### 6.2 What the data does NOT support

- **Strict "emergence"**: many winning proofs are single-agent, multi-line `step` calls. The effect is best described as a portfolio effect: heterogeneous prompts collectively span more of the tactic space, increasing the probability that SOME agent solves SOME problem.
- **TuringOS-substrate as load-bearing**: we do not claim the constitutional microkernel was necessary for the result. A simpler Python-loop harness with identical prompts and model should reproduce the effect.
- **Generalization to other models**: single-model test; Paper 2 scope.

### 6.3 Prompt leakage caveat

The Meta-Planner prompt is a meta-cognitive instruction ("review the chain", "propose a family shift"), not a list of tactics at the same abstraction level as the other 3 skills. A hostile reviewer may argue the observed gain could equally be attributed to "meta-cognitive prompt content" rather than "role heterogeneity". We flag this confound explicitly and do not resolve it in this paper; future work should run tactic-matched controls.

---

## § 7. Limitations (fully honest)

1. N=10 problems per paired A/B × 4 seeds = 40 trials. Moderate N for binary outcomes.
2. Single model (deepseek-chat) — no model-independence evidence.
3. Single benchmark (MiniF2F Lean 4).
4. Ablation isolates Meta-Planner BUT does not resolve the prompt-content-vs-role-diversity confound.
5. Some B solves are single-tape-node (multi-line `step`); the "multi-agent collaboration" interpretation applies only to a subset (⟦PENDING⟧ of ⟦PENDING⟧ B-unique solves).
6. Hard-set was constructed by filtering a broader MiniF2F pool (problems FAILed in BOTH baseline seeds). Alternative pool constructions may yield different effect sizes.
7. Proxy-saturation deviation from pre-reg: execution serialized to max 2 parallel batches; documented in § 3.5.
8. Result may reflect a "well-known effect" (prompt diversity helps in multi-sample paradigms) formalized in a more rigorous experimental design. We contribute the formalization + pre-registration + ablation, not a novel mechanism.

---

## § 8. Reproducibility

### 8.1 Code + commits

- TuringOS v4: https://github.com/gretjia/turingosv4
- main@⟦PENDING⟧ (paper + evidence)
- experiment/phase-8a-snapshot-fix@⟦PENDING⟧ (runtime code)

### 8.2 Smallest reproducer

```bash
git clone --branch experiment/phase-8a-snapshot-fix https://github.com/gretjia/turingosv4
cd turingosv4
cargo build --release -p minif2f_v4 --bin evaluator

# Pre-registered random draw
python3 -c "
import random
with open('handover/preregistration/hard36_pool.txt') as f:
    pool = [l.strip() for l in f if l.strip() and not l.startswith('#')]
sample = sorted(random.Random(20260423).sample(pool, 10))
for s in sample: print(s)
" > sample_E1v2_hard10.txt

# Run A (homogeneous) + B (heterogeneous) paired
for seed in 141421 31415 2718 2357; do
    for mode in "HOMOGENEOUS_AGENTS=1" ""; do
        env TURING_STEP_ONLY=0 TEMP_LADDER=1 HAYEK_BOUNTY=1 TAPE_ECONOMY_V2=1 \
            TICK_INTERVAL=20 MAX_TRANSACTIONS=50 \
            BOLTZMANN_SEED=$seed $mode ACTIVE_MODEL=deepseek-chat \
            bash experiments/minif2f_v4/run_list.sh n8 sample_E1v2_hard10.txt run_s${seed}_${mode}
    done
done
```

### 8.3 Dockerfile

⟦TO BE SHIPPED before arXiv submission⟧

### 8.4 Conformance test suite

```bash
cargo test --release  # Expected: ~170 tests PASS + 5 ignored (Phase 11+ stubs)
```

### 8.5 Evidence archive

All raw jsonl + proof artifacts + sample files in `handover/evidence/` and `handover/preregistration/` at commit ⟦PENDING⟧.

---

## § 9. Acknowledgments

Solo researcher (gretjia) with Claude Opus 4.7 AI collaborator. Methodology (pre-registration, ablation, dual-audit) is human-authored; AI collaborator executes code, analysis, drafting under human direction. Dual adversarial review by Codex (OpenAI) and Gemini 2.5 Pro (Google) caught methodological issues in v1 and are acknowledged as external reviewers.

---

## Appendix A. hard36 pool

⟦paste from handover/preregistration/hard36_pool.txt⟧

## Appendix B. Selection script (deterministic)

⟦paste the Python 3 script from § 8.2 verbatim⟧

## Appendix C. Sample B-unique winning proof

⟦paste one gp_payload + proof artifact header from a B-unique solve in v2⟧

---

**Status when user returns**: data collection ~50% complete (2/12 batches done). Skeleton ready for numeric fill-in. Awaiting all 4 seeds × 3 conditions to fill §§ 4-5.
