# Gemini brainstorm — N agents × PPUT × difficulty
**Date**: 2026-04-25
**Mode**: think-partner (NOT audit), temp=0.7
**Elapsed**: 60.9s
**Prompt size**: 5,279 chars

---

Excellent. This is exactly the kind of setup where a thinking partner shines. Let's dive in and generate some ideas. I'll stick to your A-E structure.

### A. Critique the design + propose alternatives

Your current design (E1, E2, E3) is a solid, logical first step. It's economical and directly addresses your questions. My goal here is to stress-test it and suggest ways to get more signal for the same (or slightly more) cost.

**1. On Sample Size & N-Grid (E1):**

*   **Critique**: 5 problems is a very small sample. Theorem proving performance is notoriously high-variance; a single problem that happens to be unusually easy or hard for the model's "innate" knowledge can skew the entire aggregate. You might see a trend that's just an artifact of one problem.
*   **Alternative N-Grid**: The `{1, 3, 5, 8}` grid is fine, but it feels a bit arbitrary. A power-of-two grid `{1, 2, 4, 8}` is common in scaling experiments and makes it easier to spot log-linear relationships. It also gives you a data point at `N=2`, which could be interesting for coordination overhead hypotheses (see section B).
*   **Proposal for a richer E1**:
    *   Increase the problem set to **10-15 problems** from the same difficulty bucket (e.g., `mathd_algebra`).
    *   Reduce the N-grid to `{1, 2, 4, 8}`. This gives you 40-60 runs instead of 20.
    *   **Crucial Addition**: Your `N=1` is a different code path (`oneshot`). This is a major confounder. You **must** create a true `N=1_swarm` condition that runs the multi-agent orchestration logic with a single agent. Compare `N=1_oneshot` vs. `N=1_swarm` on a few problems to quantify the overhead of the swarm framework itself. All subsequent comparisons should use `N=1_swarm` as the baseline.

**2. On Difficulty Bucketing (E2):**

*   **Critique**: `mathd / algebra / aime` is a good, standard proxy for difficulty. The main weakness is that it's a coarse, categorical variable. Within "aime", there's likely a huge range of actual difficulty.
*   **Alternative Proxies**:
    *   **Continuous & Cheap**: Problem statement length (in tokens). Hypothesis: longer statements involve more concepts and are harder. Easy to collect.
    *   **Continuous & Structural**: Number of unique constants/axioms in the theorem statement. A proxy for conceptual complexity.
    *   **Misleading Proxies**: I would treat theorem-name lexical structure as likely misleading. A name like `sum_divisors_prime_pow` might be long but the proof is simple, while a short name like `bessels_inequality` could be monstrous.
*   **Proposal**: Stick with your `mathd/algebra/aime` buckets for now, but for every run, **log the continuous proxies** (statement length, etc.) as metadata. This lets you do a post-hoc analysis. You might find that `token_length` is a better predictor of solve time than the dataset bucket, giving you a more powerful way to analyze difficulty in the future.

**3. On Seeds & Noise:**

*   **Critique**: A single seed is risky given the non-determinism of the LLM backend (even at temp=0.2). A single lucky or unlucky generation path could create a false peak or trough in your N vs. PPUT curve.
*   **Proposal (The "Variance Probe")**: Instead of running everything with 3+ seeds (which triples the cost), be targeted. After your first run of E1/E2, identify the most surprising or pivotal data point (e.g., "Wow, N=5 did way better than N=8 on problem X"). Re-run just that specific `(problem, N)` configuration 4 more times with different seeds. This gives you a direct sense of the run-to-run variance for a small additional cost and tells you how seriously to take that surprising result.

**4. On Confounders:**

*   **`max_transactions` is a HUGE one.** It's a form of "right-censoring" in survival analysis. If N=3 and N=8 both fail at 200 transactions, you record them as equivalent failures. But maybe N=8 would have solved at tx=210 and N=3 would have needed tx=500. The `max_transactions` cap hides this information.
    *   **Proposal**: For every run that hits the 200 tx limit, add a binary flag `hit_max_tx` to your logs. When analyzing, you should separate `(solve_rate)` from `(PPUT_on_success)`. It's possible that increasing N improves solve rate but tanks PPUT on the problems it does solve. PPUT=0 conflates these.

### B. Generate hypotheses I haven't considered

**1. Alternative Metaphors to "Throttle":**

*   **"Search Party"**: Agents are explorers in a vast proof space.
    *   *Hypothesis*: For N > 1, the key benefit isn't just more proposals, but **diverse** proposals. The agents spread out and search different branches of the proof tree simultaneously. This predicts diminishing returns as N increases and the agents' search areas start to overlap.
    *   *Metric to add*: At each transaction, calculate the **semantic diversity** of the N proposed tactics. (e.g., using sentence-embedding cosine distance on the tactic strings). Does PPUT correlate with tactic diversity?
*   **"The Committee"**: Agents aren't just parallel workers; they influence each other (implicitly, by adding to the proof state).
    *   *Hypothesis*: There might be a "groupthink" or "premature convergence" effect. At high N, if a few agents propose a plausible but wrong first step, the shared success might steer all other agents down that dead-end path more quickly and forcefully than a single agent would. This could lead to a PPUT decline at high N.
*   **"Stochastic Resonance"**: (Speculative) Perhaps the system is noisy. A single agent can get stuck in a rut. Multiple agents add "noise" (different ideas) to the search process, which can "kick" the system out of a local minimum and into a more productive state. This predicts a non-linear benefit, where N=1 is bad, N=3 is much better, and N=8 is only slightly better than N=3.

**2. Non-Monotonic Relationships:**

*   **The N=2 Coordination Dip**: I love this idea. Let's flesh it out. At N=1, there's a single coherent strategy. At N=3+, a "majority" opinion can emerge from the proposals. At N=2, you have the highest chance of a deadlock: two agents with equally plausible but conflicting ideas (e.g., one wants to `rw [lemma_A]`, the other wants to `rw [lemma_B]`), causing the system to thrash. This would predict a PPUT curve that looks like: `N=1 > N=2 < N=3`. This is a very exciting and testable hypothesis.

**3. PPUT as the Wrong Metric:**

*   PPUT is a great top-line "business metric" for the system, but it's a composite of `solve_rate`, `token_cost`, and `wall_time`. To understand the *mechanism*, you must decompose it.
*   **Proposal**: Always log and plot these three components separately, in addition to PPUT.
    *   `Solve Rate (%)` vs. N
    *   `Median Tokens per SOLVED problem` vs. N
    *   `Median Time per SOLVED problem` vs. N
*   This will reveal the trade-offs. You might find that for hard problems, N=8 has a much higher solve rate, but the successful runs are incredibly expensive, leading to a lower PPUT than N=3, which solves fewer problems but does so efficiently. This is a much richer story than "PPUT went down."

### C. Connect to scaling laws / parallel computation theory

*   **Amdahl's Law is a perfect fit here.** The task of finding a proof has a serial component (the Lean verifier, which processes one tactic at a time) and a parallel component (the N agents thinking of tactics).
    *   **Hypothesis**: Let `S` be the serial fraction of the work (time spent in Lean) and `P` be the parallel fraction (time spent in LLM calls). The theoretical speedup is `1 / (S + P/N)`. You can actually measure `S` and `P` from your wall-time logs! Plot your observed speedup (`time_at_N=1 / time_at_N`) against this theoretical curve. A big divergence suggests your "Committee" or "Coordination" overheads are significant.
*   **Universal Scalability Law (USL)** is even better. It adds a term for coherence/crosstalk overhead: `Speedup(N) = N / (1 + α(N-1) + βN(N-1))`. The `β` term models the penalty for coordination. Fitting your data to this model could give you a quantitative measure of how much agent "crosstalk" is hurting performance.
*   **Extending N**: N=8 is definitely a small range. The most interesting parts of these curves (the knee, the plateau, the decline) often happen later.
    *   **Proposal**: Design a dedicated "Stress Test" experiment on a single, reliable, medium-difficulty problem. Run it with N={1, 2, 4, 8, 12, 16, 24, 32}. The goal isn't generalizability, but to find the breaking point where performance saturates or inverts due to API limits, context window pollution, or coordination overhead.
*   **`tx_budget × N` Interaction**: This is a fantastic angle. Is `1 agent × 200 tx` the same as `8 agents × 25 tx`? Almost certainly not. The first is a "deep" sequential search; the second is a "broad" parallel search.
    *   **Hypothesis**: "Deep" search (low N, high tx) is better for problems with long, dependent proof chains. "Broad" search (high N, low tx) is better for problems that can be broken into many independent sub-goals.

### D. Statistical rigor at small N

*   **Framework**: Forget frequentist significance testing for now. The goal is R&D, not a NeurIPS paper. The best framework is **estimation and visualization**.
    *   **Proposal**: Use **bootstrap confidence intervals**. For each (problem_type, N) condition, you'll have a small set of PPUT values. Resample this set with replacement 1000s of times, calculate the mean/median each time, and find the 2.5th and 97.5th percentiles. This gives you a robust 95% CI for your metric, even with non-normal data (which PPUT, with its zeros, definitely is). Plot these CIs as shaded regions or error bars. This tells you how certain you are about the observed differences.
*   **Partitioning Variance**: The variance *between problems* will almost certainly dominate the variance *between runs of the same problem*.
    *   **Proposal**: Your primary analysis unit should be the **per-problem improvement**. For each of the 10 problems in your new E1, calculate the PPUT at N=4 and N=1. You now have a set of 10 *differences*. You can bootstrap a CI on the median of these differences. This "pairs" the analysis and controls for the fact that some problems are just easier than others.
*   **Sequential Stopping**: Yes! Don't commit to the full experiment matrix in advance.
    *   **Proposal (Adaptive Experimentation)**: Start with E1 on N={1, 8} for all 10 problems. If the 95% CIs for PPUT massively overlap, you might conclude N has little effect for this problem class and stop. If N=8 is clearly better, then run N=4 to see if you're on a curve. If N=8 is worse, run N=4 to see if you've passed a peak. This lets the data guide your exploration.

### E. Any other angles

*   **The Shape of Time**: Plot a histogram of `transaction_at_solve` for each N. Does increasing N make the distribution more "front-loaded"? I.e., does it solve problems in the first few transactions much more often, while not helping much with problems that require a long chain of reasoning? This tells you *what kind* of speedup you're getting.
*   **Qualitative Analysis**: Don't just look at numbers. For one hard problem, save the full transcripts for N=1 and N=8 runs. Read them side-by-side. What is the N=8 system *doing* differently? Is it recovering from errors faster? Is one agent's dumb idea immediately refuted by another's better one? This is where true insight comes from.
*   **Is PPUT really `Progress / Cost / Time`?** The current formula is `1 / (Tokens * Time)`. A more standard approach might be `SolveRate / (Tokens + k*Time)`, where `k` is a constant to balance token cost vs time cost. Your current metric's units are a bit strange (`1000 / (tokens * sec)`). This isn't a critique, just a note that how you combine these costs is a subjective design choice, and it's worth thinking about if it reflects your true priorities.

---

### **3 Prioritized Concrete Experiments to Run NEXT**

Here are three experiments that build on your original ideas but incorporate the critiques and suggestions above.

1.  **E1-Prime: The Baseline & Scaling Shape**
    *   **Hypothesis**: The swarm framework has overhead, and PPUT will show a sub-linear, saturating relationship with N, possibly peaking before N=16.
    *   **Design**:
        *   Problems: 10 `mathd_algebra` problems.
        *   N values: `{1_oneshot, 1_swarm, 2, 4, 8, 16}`.
        *   Seeds: 1 primary seed. For the most interesting problem, do a 4-seed "variance probe" at N=1, 4, 16.
    *   **Key Metrics**: `Solve Rate`, `PPUT_on_success`, `Time-to-solve_on_success`. Plot bootstrap CIs for each metric vs. N. Compare `1_oneshot` vs `1_swarm` directly.

2.  **E4: Effort Equivalence Test**
    *   **Hypothesis**: For problems requiring deep sequential reasoning, a "deep" search (low N, high tx) will have a higher solve rate than a "broad" search (high N, low tx) for a fixed total number of agent steps.
    *   **Design**:
        *   Problems: 2 carefully selected "hard" `aime` problems that are known to require long proof chains.
        *   Configurations: Fix total agent steps at 240. Test `(N=2, max_tx=120)`, `(N=4, max_tx=60)`, `(N=8, max_tx=30)`.
        *   Seeds: 3 seeds per configuration due to expected high variance.
    *   **Key Metrics**: `Solve Rate` (primary), `Tactic Diversity` per transaction (secondary).

3.  **E2-Prime: The Difficulty Interaction Cross-Section**
    *   **Hypothesis**: The PPUT benefit from increasing N is negligible for easy problems but significant for hard problems.
    *   **Design**:
        *   Problems: 7 `mathd` (easy), 7 `aime` (hard).
        *   N values: Just the extremes: `{1_swarm, 8}`. (This is a 2x2 design with more samples per cell, making it more robust than your original 3x3).
        *   Seeds: 1 seed.
    *   **Key Metrics**: Plot `PPUT` vs a continuous difficulty proxy (e.g., problem token length) and color the points by N. Look for a divergence in the trend lines. Also, directly compare the distributions of `PPUT(N=8) - PPUT(N=1)` for the easy vs. hard buckets.