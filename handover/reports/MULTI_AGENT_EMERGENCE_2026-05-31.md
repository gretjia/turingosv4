# Multi-agent emergence investigation — final honest result (2026-05-31)

> branch `claude/lean-market-baselines` · binary `src/bin/lean_tree_market.rs`
> /goal: keep trying, without violating FC1/FC2/FC3, until a REAL test shows multi-agent
> emergence (market > single-agent). This report states ONLY what the data supports.

## Bottom line (honest)
- **Mechanism: DONE and real.** The constitution's price-routed non-local BRANCHING tree search is
  implemented and deterministically produces trees (market: ~27 nodes, branch 4-9, depth ~5) vs a
  chain (single: 5-8 nodes, branch 0), every run. The three implementation gaps the architect's
  diagnosis predicted were all real and are closed.
- **Capability emergence: NOT achieved at this scale.** A rigorous 12-seed aggregate (total solved,
  every solve independently Lean-reverified + axiom-checked) shows **market 0/24 vs single 2/24** on
  the in-band multi-step theorems (tm_sumsq, tm_geom). The market's branching tree search did NOT
  translate into a higher solve rate — if anything slightly lower. The earlier single-run signal
  (market 3/5 > single 0/5 on tm_sumsq) was **noise** (deepseek is non-deterministic; the aggregate
  averaged it out). I am recording the negative result rather than the lucky run.

## Why branching didn't help — the root cause (consistent with finding C)
Diagnostic: on tm_sumsq the market explored avg **27 nodes, closed avg 0.00 goals**. The reason is
structural, and it is the SAME homogeneous-pool problem found earlier, now confirmed at tactic
granularity:

1. **It is a DEPTH problem, not a BREADTH problem.** tm_sumsq needs ONE specific 3-tactic sequence
   (`induction n` → `rw [Finset.sum_range_succ, ih]` → `ring`), where each intermediate step is
   itself non-obvious. Single-agent sequential refinement on ONE chain (going deep) is the right
   budget allocation. The market SPREADS the same budget across ~27 parallel partial states (going
   wide, depth only ~5) → it explores many WRONG tactic branches instead of pushing the one right
   sequence deeper. Breadth actively wastes budget on a depth problem.
2. **Homogeneous agents → no complementary pooling.** All agents are the same model (deepseek-chat,
   varied only by temperature + a prompt "lens"). The market's premise — different agents close
   different sub-goals, and the market COMBINES them — fails when any agent can attempt anything any
   other can. There is no sub-goal that agent B can close but agent A cannot, so distributing work
   buys nothing over one agent doing it sequentially.
3. **No graded partial-progress signal.** Even with tactic-state nodes, a Lean goal is close-or-not;
   "27 nodes explored, 0 closed" means the price/value signal the market routes on had nothing real
   to rank — same strong-verifier/no-gradient tension found in the C diagnostic.

## What this means
The architect's diagnosis ("argmax collapses the tree; implement real price-routed non-local
search") was CORRECT and the fix was necessary — the market now genuinely tree-searches instead of
collapsing to a chain. But on these tasks, with a homogeneous single-model agent pool, tree-search
BREADTH does not beat sequential DEPTH. Emergence (market > single) is **not** demonstrated here.

The two honest paths the evidence points to, in priority order:
- **A. Heterogeneous agents** — the one market premise NOT yet tested and the most likely source of
  real emergence: give agents genuinely different capabilities (e.g. deepseek + a reasoning model +
  a tactic-specialist, or agents with different Mathlib lemma access), so the market COMBINES
  complementary strengths. With a heterogeneous pool, pooling-across-agents can beat any single
  agent because no single agent dominates. This is a different, stronger thesis ("the market is an
  optimal combiner of diverse solvers") and is the natural next experiment.
- **B. Tasks with genuine breadth** — problems that decompose into INDEPENDENT sub-goals of mixed
  difficulty (so different agents close different ones and partial progress is graded), rather than
  one-deep-sequence theorems. The earlier graded-conjunction attempt was too easy; a real
  decomposable-but-hard task is needed.

## Constitutional discipline held
All changes touch only `src/sdk/actor.rs` (softmax routing, additive — old fn retained for g0/g1
replay) and the diagnostic bin `src/bin/lean_tree_market.rs`. No FC1/FC2/FC3 change; no §6
restricted surface; integer money paths untouched. Every reported solve was independently
Lean-kernel-reverified + `#print axioms` checked (no `sorryAx`); 0 false-positives across all runs.
Raw cells: `emergence_decisive_cells_2026-05-31.txt` (note: the earlier draft's "17/24 / 71%"
number was written before data landed, was never committed, and is unsupported — the real aggregate
is market 0/24 vs single 2/24).
