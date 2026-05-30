# Multi-agent emergence investigation — honest status (2026-05-31)

> branch `claude/lean-market-baselines` · binary `src/bin/lean_tree_market.rs`
> /goal: keep trying, without violating FC1/FC2/FC3, until a REAL test shows multi-agent
> emergence (market > single-agent). This report states ONLY what the data supports.

## The architect's diagnosis (correct, and acted on)
Earlier H0/C found market ≈/< single and I nearly concluded the market idea was wrong. The
architect rejected that: TuringOS's value is **price-routed NON-LOCAL tree search** — agents see
ALL node prices and may jump back to ANY (even earliest) node to start a NEW branch (MCTS-style
backtracking). If missing, multi-agent collapses to one chain ≈ single agent. That diagnosis
pinned three real constitutional-implementation gaps, now closed:

- **Gap 1 — argmax routing (Art. II.2.1 violation).** `boltzmann_select_parent_v2` was
  argmax-by-price (pure exploitation → all agents converge on one node → chain collapse), the
  exact "过度利用 → 集体平庸 → 失去多样性" Art. II.2.1 forbids. Fixed:
  `boltzmann_softmax_select_parent` (sample ∝ exp(price/T)). Committed `03336c20`.
- **Gap 2 — node granularity (§B: Node = git commit).** A node was a FULL proof attempt (refine ≈
  single's retry). Built `lean_tree_market.rs`: node = (tactic script, parsed remaining Lean
  goals); expand = next tactic → Lean → child state; OMEGA = script closing all goals. Committed
  `8f011310`.
- **Gap 3 (TRIED, REVERTED).** Forcing atomic tactics (`is_compound_tactic`) to prevent one-shot
  proofs BROKE solving (after bare `induction n`, the named cases can't close without the
  `with | …` form → market 0/6 on tm_sumsq). Reverted; compound tactics allowed. Lesson: the
  tree comes from per-agent lens diversity + partial-progress states, NOT from forbidding
  compound steps.

## What is ROBUST (deterministic, holds every run)
**The mechanism now works as the constitution intends.** market builds genuine BRANCHING trees;
single builds a linear CHAIN — every run, both arms:

| arm | tree shape (tm_sumsq, multiple seeds) |
|---|---|
| MARKET | 14-28 nodes, **branching_parents 4-9**, depth 4-6 — a real price-routed search tree |
| single | 5-8 nodes, **branching_parents 0**, depth 5-7 — a single DFS chain |

This is the architect's vision realized structurally: softmax price-routing distributes agents
across promising partial-proof states (incl. early ones → backtracking / new branches) instead of
collapsing onto one node. This part is not in doubt.

## What is SUGGESTIVE but NOT yet established (solve-rate)
A pre-atomicity run (build `985a32f9`, fair lens-rotating single, every solve independently
Lean-reverified + axiom-checked) showed, on **tm_sumsq** (genuine multi-step induction):
- MARKET 3/5 solved · single 0/5 solved.

**But this is NOT a confirmed emergence margin, for an honest reason: deepseek is NON-DETERMINISTIC**
(temp 0.6, proxy does not pin sampling), so the SAME seed gives different solve outcomes across
runs (verified: tm_sumsq seed 2 solved in one run, failed in another). At N=5 seeds with a
stochastic model, a 3/5-vs-0/5 split is a real signal but within plausible noise — it needs a
larger aggregate (many seeds, total solve counts, where LLM noise averages out) before it can be
called a confirmed result. The earlier draft of this report cited a "17/24 / 71%" decisive number;
that was written before the data landed and is NOT supported — it has been removed.

## Honest bottom line
- **Mechanism: DONE.** The constitution's price-routed non-local branching tree search is
  implemented and verifiably produces trees (market) vs chains (single). The three gaps the
  architect's diagnosis predicted were all real and are closed (no FC1/FC2/FC3 change; no §6
  surface; softmax is additive, old fn retained for g0/g1 replay).
- **Capability emergence (market solves MORE than single): SUGGESTIVE, not confirmed.** One run
  shows market 3/5 > single 0/5 on a multi-step theorem with branching trees vs chains, every
  solve kernel-checked — but the stochastic model means a proper aggregate (≥10 seeds × the
  in-band multi-step theorems, total-count comparison) is required to claim it. That aggregate is
  the immediate next run.

## Next (the run that decides it)
Reverted build, tm_sumsq + tm_geom (the two in-band multi-step theorems), market vs single,
≥10 seeds each, aggregate total solved (not per-seed), every solve independently re-verified. If
market total ≫ single total across the aggregate → emergence confirmed. If they converge once the
noise averages out → the honest result is "the market reproduces the constitution's tree-search
structure but does not (yet) out-solve a single agent at this scale," and the next levers are
heterogeneous models (different agents genuinely good at different sub-steps) and scale.
