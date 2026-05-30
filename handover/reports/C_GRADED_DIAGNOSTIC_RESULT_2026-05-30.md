# C diagnostic result — §4.5 graded-progress hypothesis REFUTED → global NO-GO (path A)

> 2026-05-30 · binary `src/bin/lean_graded_diag.rs` (commit `ee37216f`+) · PR #220
> Architect chose path C ("做完 C 决定性") to test whether the market only helps when progress is
> GRADED (the regime H0's monolithic theorems couldn't provide). Verdict: **refuted — the market
> ties/loses single-agent across ALL task structures.** → path A.

## 1. The decisive pattern (market never beats single-agent)

| task structure | market | single | note |
|---|---|---|---|
| **monolithic theorem** (H0, binary kernel) | 5/12 | **7/12** | single wins |
| **easy graded** (grad_nt/alg/ord, k=6) | 6/6 | 6/6 | both solve trivially |
| **moderate graded** (grad_calibrated, k=6, seeds 1-3) | 6/6 | 6/6 | both solve |
| **hard graded** (grad_hard = headroom thms as conjuncts, k=3) | 0/3 | 0/3 | both fail |

The market never wins. On the only structure with a difference, **single-agent wins**. Confirmed
across seeds (grad_calibrated 6/6 vs 6/6 on seeds 1, 2, 3).

## 2. Why — two root causes (deeper than "no gradient")

**(a) Conjunct difficulty is BIMODAL — the graded-pooling regime barely exists.** Across many
hand-built + headroom-derived conjuncts, deepseek-chat is either GOOD at a goal (~70-90% single
shot → single-agent just closes it) or BAD (~0-10% → nobody closes it). The "moderate AND diverse"
middle — where partial progress accumulates and pooling could help — is vanishingly thin for real
Lean goals. When conjuncts are easy enough to grade, single-agent sequential closure already
suffices; when hard, there is no partial progress to pool. There is no Goldilocks regime in
between for the market to exploit.

**(b) HOMOGENEOUS agents → no pooling benefit (the implicit §4.5 assumption fails).** The market's
"pool partial progress across agents" advantage requires that different agents can close different
sub-goals — i.e. HETEROGENEOUS capabilities. But the experiment's agents are all the *same model*
(deepseek-chat) varied only by temperature. Any agent can do whatever any other can, so distributing
work across agents (market) is never better than one agent doing it sequentially (single). With a
homogeneous pool, single-agent sequential refinement is the optimal budget allocation — exactly what
H0 already showed (single 7 > market 5).

## 3. Decisive conclusion

**The price-routed market does not amplify a homogeneous agent pool on Lean proving — on monolithic
theorems (no gradient), easy/moderate graded tasks (single suffices), or hard graded tasks (nothing
to pool). The §4.5 "graded progress rescues the market" hypothesis is REFUTED. This is a global
NO-GO for the strong capability claim → path A.**

## 4. The one regime NOT refuted (a different claim, deferred)

The market's only remaining plausible niche is **HETEROGENEOUS agents** — e.g. deepseek + a
reasoning model + a specialist prover, each strong on different sub-goals, where the market COMBINES
their complementary strengths. That is a *different* thesis ("the market is an optimal combiner of
diverse solvers", not "the market amplifies search") and a *different* experiment. It is not tested
here and would be the only honest way to revive a market-capability claim. Recommend the architect
decide separately whether that is worth a future thrust; it does not change the current NO-GO.

## 5. Path A — what TuringOS defensibly IS

The capability claim "the market out-thinks a single agent" is refuted. The **substrate** claim
stands and is unique and fully evidenced: every attempt, price, informed-Bear short, failure, and
OMEGA is **deterministically reconstructable from ChainTape + CAS** (H0: 29/29 OMEGA cells
replay-green; every OMEGA a real Lean `Verified`, no `sorry`). TuringOS's honest, defensible
position: an **auditable, replayable, constitutional substrate** for LLM/AGI agent work — not a
market that thinks better. The negative capability result sharpens the claim rather than weakening
the substrate.

## 6. Diagnostic scope honesty

`lean_graded_diag` is diagnostic-only (not OMEGA, no chain/CAS/replay; allows decide/native_decide
since proof-method purity is irrelevant to the routing question). It is a clean mechanism probe, not
a constitutional run. The strong no-`sorry`/no-`native_decide` LeanJudge stays in force for the real
market. The H0 result (the headline NO-GO) IS fully constitutional + replay-verified; C corroborates
its root cause.
