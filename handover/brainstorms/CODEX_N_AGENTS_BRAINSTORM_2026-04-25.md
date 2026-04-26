# Codex brainstorm — N agents × PPUT × difficulty
**Date**: 2026-04-25
**Mode**: think-partner (NOT audit)
**Prompt size**: 4949 chars

---

Reading prompt from stdin...
OpenAI Codex v0.124.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: danger-full-access
reasoning effort: xhigh
reasoning summaries: none
session id: 019dc751-0085-7a00-afff-3553c2af4a84
--------
user
# Brainstorm partner — N agents × PPUT × difficulty

**Role**: think-partner, NOT auditor. No PASS/CHALLENGE/VETO. Generative, divergent, idea-rich. Your job: critique the user's experiment design + propose additions + flag blind spots. Speculation is welcome, just label it as such.

**Background**: TuringOS v4 is a multi-agent LLM swarm for Lean4 theorem proving (MiniF2F). Each "run" is a single problem attempt where N agents propose tactics + a Lean compiler ground-truths. PPUT = progress / cost / time = `1[Lean accepts] / (total_tokens × wall_time_sec / 1000)`. Constitutional anchor: Proposal (LLM) → ∏p ground-truth (Lean) → Logging → Capability Compilation. Solo researcher, R&D phase, iterative.

**Three research questions the user wants answered**:
- **Q1**: For same-difficulty problems, does increasing N (number of agents) have a PPUT ceiling? Is there a plateau? Decline? Power-law?
- **Q2**: As difficulty increases, does increasing N speed up solving (higher PPUT)? Or is N less helpful on hard problems?
- **Q3**: Like a car (N = throttle): is there a regime where increasing N **sacrifices PPUT** but **improves solve speed (1/wall_time)**? I.e., does N let you trade efficiency for speed? Does this mean N should be a user-facing knob?

**User's constraint**: still in R&D, small targeted experiments preferred over big batches. Each test should answer one specific question. Code may change between experiments — designs robust to mid-stream code adjustments are valued.

**My (Claude's) draft experiment design**:

E1 — Same-difficulty PPUT ceiling
- 5 mathd_algebra problems × N ∈ {1, 3, 5, 8} × seed=31415 = 20 runs (~$1, ~45min)
- Plot PPUT vs N per problem + aggregate

E2 — Difficulty × N matrix
- 9 problems (3 easy mathd / 3 medium algebra / 3 hard aime) × N ∈ {1, 3, 8} × seed=31415 = 27 runs (~$2-3, 3-6h)
- Heatmap PPUT vs (difficulty, N); solve_rate vs (difficulty, N)

E3 — Throttle regime (Q3)
- Reuse E2 data; plot solve-time vs N AND PPUT vs N on dual-axis per difficulty bucket
- Look for points where solve-time↓ but PPUT↓ (the throttle regime)

**Key technical details**:
- N=1 is currently `CONDITION=oneshot` (1 LLM call, no tape) — different code path from `n*` swarm. Comparison concern.
- Boltzmann seed [BOLTZMANN_SEED env] only seeds agent-routing RNG; LLM sampling at temp=0.2 is non-deterministic.
- max_transactions = 200 hard-coded; hard problems hit it.
- Per-tx wall-clock ~10s avg (LLM call + tool + Lean verify).
- Easy problems solve in tx 1-5; hard problems take 100-200 tx or fail.
- Cost: ~1000 tokens per tx avg (per agent prompt + completion).
- PPUT = 1 / (total_tokens × time_sec / 1000) when SOLVED, else 0.

## What I want from you (be generative, not skeptical)

### A. Critique the design + propose alternatives
- Is "5 problems × 5 N values" enough power for E1? What N grid would you choose differently?
- Is the difficulty bucketing (mathd / algebra / aime) the right operationalization? What proxy for difficulty would you use?
- Single seed vs multi-seed for these small experiments — how to bound noise without inflating cost?
- Any confounders I'm missing (problem-text length, theorem name distribution, library imports, etc.)?

### B. Generate hypotheses I haven't considered
- The user's intuition is "throttle". What OTHER metaphors/regimes might be in play? E.g., diversity-vs-homogeneity, exploration-exploitation, parallel-vs-serial information bottleneck?
- Are there NON-monotonic relationships in N → PPUT that would surprise us? (e.g., minimum at N=2 from agent-confusion before benefits from N=3+ kick in)
- Could PPUT be the WRONG metric for some questions? (e.g., for "speed-up regime", solve-time alone matters)

### C. Connect to scaling laws / parallel computation theory
- Is there a published functional form for "agents × task difficulty → speedup" we should test against? (Amdahl's law, Brent's theorem, Universal Scalability Law, etc.)
- N=1 to N=8 is small range — would you suggest extending up to N=16 or N=32 to find the saturation point?
- What's the role of max_transactions in this picture? Is there a `tx_budget × N` interaction?

### D. Constitutional & engineering considerations
- The user wants per-line constitutional alignment (DO-178C-style) on the swarm hot loop. What runtime tracing / coverage tooling would you use to know which lines fire on a real run?
- Are there code paths in the swarm loop that should be considered "infrastructure not capability" and therefore exempt from the PPUT calculation? (e.g., tape-snapshot serialization)

### E. Any other angles
Whatever you'd want to discuss with a research collaborator over coffee. No format requirement. Be specific where useful, speculative where useful.

---

End with **3 prioritized concrete experiments you'd run NEXT** (in addition to or instead of E1/E2/E3). For each: name, hypothesis, sample size, key metric. Brief.


codex
I’ll treat this as experiment-design collaboration rather than code review. I’m going to focus on sharper observables, cheaper ways to bound variance, and designs that survive code changes midstream.
codex
Your E1/E2/E3 are good scouting designs, but I’d tighten what each can actually claim. The biggest issue is not sample size; it is isolation. Right now `N=1` is `oneshot`, so any curve from `N=1 → 3 → 5 → 8` mixes “number of agents” with “different runtime architecture.” I’d either add a true `swarm_N=1` condition or treat oneshot as a separate baseline, not a point on the N curve.

**A. Design Critique**

For E1, `5 problems × 4 N values × 1 seed` is enough to notice a large effect, but not enough to estimate a ceiling shape. I’d prefer a slightly denser N grid over more problems if the goal is “where does saturation begin”:

`N ∈ {1_swarm, 2, 3, 5, 8, 13}`

or, if implementation cost matters:

`N ∈ {1_swarm, 3, 5, 8, 13}`

The jump from 1 to 3 is too large if you care about non-monotonicity. `N=2` is interesting because it may expose coordination overhead before diversity benefits appear.

For difficulty, `mathd / algebra / aime` is a useful label but not a clean operationalization. I’d define difficulty empirically from a cheap pilot:

- baseline solve probability under `N=1_swarm` or `N=3`
- transaction-to-first-solve under a fixed small budget
- historical solve rate if you already have logs
- proof length / tactic depth if known
- statement token length and imported-library footprint as covariates, not difficulty itself

I’d avoid calling all `aime` hard and all `mathd` easy. Some AIME problems may be template-like; some algebra problems may be brittle for Lean.

Single seed is okay for directional scouting, but because LLM sampling is still nondeterministic, one seed mostly controls routing, not the model. A cheap noise-bounding pattern:

- Run one full paired grid with `seed=31415`.
- Replicate only the ambiguous or surprising cells.
- Include 1 duplicated same-seed run to measure provider nondeterminism.
- Include 1 different-seed run to measure routing variance.
- Randomize or interleave run order by problem and N to reduce backend/model-time drift.

Confounders I’d explicitly log:

- code SHA / dirty diff hash
- prompt hash, system prompt hash, theorem hash
- model name/version if available
- total prompt tokens vs completion tokens
- wall time split: LLM latency, Lean verification, orchestration, logging
- Lean cold-start vs warm verification time
- theorem statement length
- theorem name leakage / recognizable MiniF2F names
- import set and cache state
- tape length over time
- duplicate tactic rate across agents
- accepted intermediate tactic count, if applicable
- whether agents run serially or concurrently

The product metric `tokens × wall_time` is also important. It double-penalizes high-N parallelism. That may be right for “efficiency,” but it is not the same as “cheap” or “fast.”

**B. Extra Hypotheses**

The throttle metaphor is plausible, but I’d also consider these regimes:

- **Lottery-ticket regime**: each agent is a partially independent draw; solve probability follows something like `1 - (1-p)^N`, then saturates.
- **Congestion regime**: more agents increase prompt/tape/coordination overhead, so throughput eventually declines.
- **Diversity bottleneck**: raw N does not matter; effective diversity matters. `N=8` identical agents may behave like `N_eff=2`.
- **Depth-vs-breadth regime**: hard problems need long serial proof search, so more agents help less than more transactions.
- **Burst regime**: high N is useful only early, to find a promising direction; later it wastes tokens.
- **Compiler bottleneck regime**: if Lean verification or orchestration is serial, N improves candidate generation but not end-to-end latency.
- **Tape pollution regime**: more agents create more failed attempts, which degrade future prompts.

Non-monotonic possibilities worth looking for:

- `N=1` best PPUT on easy problems.
- `N=2` worse than both `N=1` and `N=3` because it adds coordination without enough diversity.
- `N=5` peak PPUT, `N=8+` faster but less efficient.
- `N=8` better on medium problems, worse on very hard ones because hard tasks need depth, not width.
- `N=13/16` worse than `N=8` because prompt/tape overhead dominates.

I would not rely on PPUT alone. Split the dashboard into:

- solve rate
- wall time to solve
- tokens to solve
- dollars per solve
- conditional PPUT among solved runs
- censored time-to-solve for failures
- duplicate proposal rate
- accepted-progress-per-token, if you can define Lean-grounded partial progress

For Q3 especially, the real object is a Pareto frontier: latency vs cost vs solve probability. PPUT is one projection of that frontier, not the frontier itself.

**C. Scaling-Law Lens**

Useful forms to test, lightly:

- **Amdahl’s law**: speedup limited by serial fraction. Good for Lean/compiler/orchestration bottlenecks.
- **Brent’s theorem**: runtime bounded by total work divided by processors and by critical path length. Good metaphor for proof search depth.
- **Universal Scalability Law**: captures both contention and retrograde scaling. Very relevant if agents contend through shared tape, verifier, aggregator, or prompt context.
- **Portfolio/random-restart model**: `P_solve(N) = 1 - ∏(1 - p_i)`. Good if agents are independent tactic generators.
- **Effective-N model**: correlated agents behave like fewer agents. Track diversity so you can estimate `N_eff`.

`N=1..8` is probably too small to identify a law, but enough to reject simple monotonic stories. I would only extend to `N=16` or `N=32` after seeing `N=8` still improve wall-time or solve rate. Use those large N values on 1-2 sentinel problems first.

`max_transactions=200` is a major interaction. Fixed tx budget means total proposal budget may scale with N. That is fine if the question is “what happens when I turn the swarm up,” but not if the question is “is N intrinsically better.” You need at least one budget-normalized test:

- fixed transaction budget: `tx=200` for all N
- fixed proposal budget: `N × tx = constant`
- fixed token budget
- fixed wall-clock budget

Those answer different questions.

**D. Constitutional / Engineering**

For per-line constitutional alignment, I’d use runtime coverage plus semantic tracing.

If this is Python, use `coverage.py` with branch coverage and dynamic contexts like:

`problem_id / N / condition / seed / transaction_id`

Then add OpenTelemetry-style spans or structured JSONL events for the constitutional stages:

`Proposal → Lean ground truth → Logging → Capability compilation`

Each event should include `run_id`, `tx_id`, `agent_id`, `problem_id`, `N`, prompt hash, proposal hash, Lean result, token counts, and monotonic timestamps.

For PPUT, I’d report two versions:

- **User-facing PPUT**: includes everything on the critical path.
- **Capability PPUT**: includes LLM proposal, agent coordination, and Lean verification, but excludes pure observability/storage overhead.

I would not silently exempt tape serialization if it grows with N and affects latency. That is part of the system behavior. But I would break it out so you can tell whether N is failing because of reasoning, verification, or infrastructure.

**Other Angles**

Because code may change mid-stream, structure experiments as small paired blocks. A valid block is: same commit, same candidate problem set, all N values run close together, with hashes logged. If code changes, start a new block and keep 1-2 sentinel problems repeated across blocks.

Also, consider testing policies, not just static N. A likely winner may be:

`burst N=8 for first 5-10 transactions, then decay to N=3 or N=1`

That directly matches the “throttle” idea better than fixed N.

**3 Experiments I’d Run Next**

1. **E0: N=1 Path Calibration**  
   Hypothesis: apparent N gains/losses are partly caused by `oneshot` vs swarm-loop differences.  
   Sample: 6 problems × `{oneshot, swarm_N=1}` × 2 repeats = 24 runs.  
   Key metric: solve rate, wall time, tokens, PPUT, and overhead split.

2. **E1b: Paired N Curve on Medium Problems**  
   Hypothesis: PPUT peaks at low/intermediate N, while wall-time may keep improving.  
   Sample: 6 empirically medium problems × `N={1_swarm,2,3,5,8,13}` × 1 seed, then replicate surprising cells.  
   Key metric: paired PPUT delta, time-to-solve, solve probability, duplicate tactic rate.

3. **E4: Budget-Normalized Throttle Test**  
   Hypothesis: high N mostly buys latency via more parallel proposals, not better efficiency.  
   Sample: 4 problems × `N={1,4,8}` under fixed `tx=200` and fixed `N×tx` budget = 24 runs.  
   Key metric: Pareto frontier of solve rate vs wall time vs tokens, with censored failures shown separately.
tokens used
18,610
Your E1/E2/E3 are good scouting designs, but I’d tighten what each can actually claim. The biggest issue is not sample size; it is isolation. Right now `N=1` is `oneshot`, so any curve from `N=1 → 3 → 5 → 8` mixes “number of agents” with “different runtime architecture.” I’d either add a true `swarm_N=1` condition or treat oneshot as a separate baseline, not a point on the N curve.

**A. Design Critique**

For E1, `5 problems × 4 N values × 1 seed` is enough to notice a large effect, but not enough to estimate a ceiling shape. I’d prefer a slightly denser N grid over more problems if the goal is “where does saturation begin”:

`N ∈ {1_swarm, 2, 3, 5, 8, 13}`

or, if implementation cost matters:

`N ∈ {1_swarm, 3, 5, 8, 13}`

The jump from 1 to 3 is too large if you care about non-monotonicity. `N=2` is interesting because it may expose coordination overhead before diversity benefits appear.

For difficulty, `mathd / algebra / aime` is a useful label but not a clean operationalization. I’d define difficulty empirically from a cheap pilot:

- baseline solve probability under `N=1_swarm` or `N=3`
- transaction-to-first-solve under a fixed small budget
- historical solve rate if you already have logs
- proof length / tactic depth if known
- statement token length and imported-library footprint as covariates, not difficulty itself

I’d avoid calling all `aime` hard and all `mathd` easy. Some AIME problems may be template-like; some algebra problems may be brittle for Lean.

Single seed is okay for directional scouting, but because LLM sampling is still nondeterministic, one seed mostly controls routing, not the model. A cheap noise-bounding pattern:

- Run one full paired grid with `seed=31415`.
- Replicate only the ambiguous or surprising cells.
- Include 1 duplicated same-seed run to measure provider nondeterminism.
- Include 1 different-seed run to measure routing variance.
- Randomize or interleave run order by problem and N to reduce backend/model-time drift.

Confounders I’d explicitly log:

- code SHA / dirty diff hash
- prompt hash, system prompt hash, theorem hash
- model name/version if available
- total prompt tokens vs completion tokens
- wall time split: LLM latency, Lean verification, orchestration, logging
- Lean cold-start vs warm verification time
- theorem statement length
- theorem name leakage / recognizable MiniF2F names
- import set and cache state
- tape length over time
- duplicate tactic rate across agents
- accepted intermediate tactic count, if applicable
- whether agents run serially or concurrently

The product metric `tokens × wall_time` is also important. It double-penalizes high-N parallelism. That may be right for “efficiency,” but it is not the same as “cheap” or “fast.”

**B. Extra Hypotheses**

The throttle metaphor is plausible, but I’d also consider these regimes:

- **Lottery-ticket regime**: each agent is a partially independent draw; solve probability follows something like `1 - (1-p)^N`, then saturates.
- **Congestion regime**: more agents increase prompt/tape/coordination overhead, so throughput eventually declines.
- **Diversity bottleneck**: raw N does not matter; effective diversity matters. `N=8` identical agents may behave like `N_eff=2`.
- **Depth-vs-breadth regime**: hard problems need long serial proof search, so more agents help less than more transactions.
- **Burst regime**: high N is useful only early, to find a promising direction; later it wastes tokens.
- **Compiler bottleneck regime**: if Lean verification or orchestration is serial, N improves candidate generation but not end-to-end latency.
- **Tape pollution regime**: more agents create more failed attempts, which degrade future prompts.

Non-monotonic possibilities worth looking for:

- `N=1` best PPUT on easy problems.
- `N=2` worse than both `N=1` and `N=3` because it adds coordination without enough diversity.
- `N=5` peak PPUT, `N=8+` faster but less efficient.
- `N=8` better on medium problems, worse on very hard ones because hard tasks need depth, not width.
- `N=13/16` worse than `N=8` because prompt/tape overhead dominates.

I would not rely on PPUT alone. Split the dashboard into:

- solve rate
- wall time to solve
- tokens to solve
- dollars per solve
- conditional PPUT among solved runs
- censored time-to-solve for failures
- duplicate proposal rate
- accepted-progress-per-token, if you can define Lean-grounded partial progress

For Q3 especially, the real object is a Pareto frontier: latency vs cost vs solve probability. PPUT is one projection of that frontier, not the frontier itself.

**C. Scaling-Law Lens**

Useful forms to test, lightly:

- **Amdahl’s law**: speedup limited by serial fraction. Good for Lean/compiler/orchestration bottlenecks.
- **Brent’s theorem**: runtime bounded by total work divided by processors and by critical path length. Good metaphor for proof search depth.
- **Universal Scalability Law**: captures both contention and retrograde scaling. Very relevant if agents contend through shared tape, verifier, aggregator, or prompt context.
- **Portfolio/random-restart model**: `P_solve(N) = 1 - ∏(1 - p_i)`. Good if agents are independent tactic generators.
- **Effective-N model**: correlated agents behave like fewer agents. Track diversity so you can estimate `N_eff`.

`N=1..8` is probably too small to identify a law, but enough to reject simple monotonic stories. I would only extend to `N=16` or `N=32` after seeing `N=8` still improve wall-time or solve rate. Use those large N values on 1-2 sentinel problems first.

`max_transactions=200` is a major interaction. Fixed tx budget means total proposal budget may scale with N. That is fine if the question is “what happens when I turn the swarm up,” but not if the question is “is N intrinsically better.” You need at least one budget-normalized test:

- fixed transaction budget: `tx=200` for all N
- fixed proposal budget: `N × tx = constant`
- fixed token budget
- fixed wall-clock budget

Those answer different questions.

**D. Constitutional / Engineering**

For per-line constitutional alignment, I’d use runtime coverage plus semantic tracing.

If this is Python, use `coverage.py` with branch coverage and dynamic contexts like:

`problem_id / N / condition / seed / transaction_id`

Then add OpenTelemetry-style spans or structured JSONL events for the constitutional stages:

`Proposal → Lean ground truth → Logging → Capability compilation`

Each event should include `run_id`, `tx_id`, `agent_id`, `problem_id`, `N`, prompt hash, proposal hash, Lean result, token counts, and monotonic timestamps.

For PPUT, I’d report two versions:

- **User-facing PPUT**: includes everything on the critical path.
- **Capability PPUT**: includes LLM proposal, agent coordination, and Lean verification, but excludes pure observability/storage overhead.

I would not silently exempt tape serialization if it grows with N and affects latency. That is part of the system behavior. But I would break it out so you can tell whether N is failing because of reasoning, verification, or infrastructure.

**Other Angles**

Because code may change mid-stream, structure experiments as small paired blocks. A valid block is: same commit, same candidate problem set, all N values run close together, with hashes logged. If code changes, start a new block and keep 1-2 sentinel problems repeated across blocks.

Also, consider testing policies, not just static N. A likely winner may be:

`burst N=8 for first 5-10 transactions, then decay to N=3 or N=1`

That directly matches the “throttle” idea better than fixed N.

**3 Experiments I’d Run Next**

1. **E0: N=1 Path Calibration**  
   Hypothesis: apparent N gains/losses are partly caused by `oneshot` vs swarm-loop differences.  
   Sample: 6 problems × `{oneshot, swarm_N=1}` × 2 repeats = 24 runs.  
   Key metric: solve rate, wall time, tokens, PPUT, and overhead split.

2. **E1b: Paired N Curve on Medium Problems**  
   Hypothesis: PPUT peaks at low/intermediate N, while wall-time may keep improving.  
   Sample: 6 empirically medium problems × `N={1_swarm,2,3,5,8,13}` × 1 seed, then replicate surprising cells.  
   Key metric: paired PPUT delta, time-to-solve, solve probability, duplicate tactic rate.

3. **E4: Budget-Normalized Throttle Test**  
   Hypothesis: high N mostly buys latency via more parallel proposals, not better efficiency.  
   Sample: 4 problems × `N={1,4,8}` under fixed `tx=200` and fixed `N×tx` budget = 24 runs.  
   Key metric: Pareto frontier of solve rate vs wall time vs tokens, with censored failures shown separately.
