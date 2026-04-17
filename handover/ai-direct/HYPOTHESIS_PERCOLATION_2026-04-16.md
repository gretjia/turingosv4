# Hypothesis: Swarm Scaling Follows Percolation Phase Transition

**Proposed by**: User (2026-04-16 ~04:00 UTC)
**Status**: Framework established; N-scaling experiment queued post v3.3

## Core claim

Increasing N (number of agents) produces PPUT gain that is NOT linear, NOT super-linear in the naive sense, but follows a **percolation-like curve** — possibly logarithmic, possibly phase-transitional with a critical N_c.

Below N_c: adding agents = adding independent trials (Bernoulli).
Above N_c: collective error-landscape coverage creates "spanning cluster" in proof-strategy space → emergent cooperative solving.

## Physical analog: bond percolation on a lattice

- Each agent = a node in the proof-search graph
- Art. II.1 broadcast = bonds (edges) between nodes
- Bond probability depends on mechanism quality (classifier granularity, broadcast latency, error coverage)
- **N_c = critical agent count where giant connected component forms**
- N_c is NOT fixed: improving mechanism (Art. II.1 → II.2 → III.3) lowers N_c

## Testable predictions

1. **Control arm (broken Art. II.1)**: SolveRate(N) = 1 - (1-p)^N (Bernoulli). No phase transition.
2. **Treatment arm (fixed Art. II.1)**: SolveRate(N) deviates from Bernoulli above some N_c.
   - If log: PPUT(N) = a·log(N) + b (smooth, no sharp transition)
   - If percolation: PPUT(N) exhibits a "knee" at N_c (sharp transition)

## Proposed N-scaling experiment

### Design
- **N values**: 1, 2, 3, 5, 8, 13 (log-spaced, Fibonacci-ish; covers 1-order-of-magnitude range)
- **Sample**: same frozen seed=74677 (or subset N=20 for budget control)
- **Model**: deepseek-chat (fast + activates scaffold per user thesis)
- **Two arms per N**: control (main, broken broadcast) vs treatment (experiment branch, fixed broadcast)
- **Total evaluations**: 6 N-values × 2 arms × 50 problems = 600 (or 240 on N=20 subset)

### Metrics per N
- **Primary**: SolveRate(N)
- **Secondary**: Aggregate_PPUT(N)
- **Diagnostic**: 
  - `tape_depth(N)` — does depth increase with N? (percolation signal: spanning cluster = deeper chains)
  - `unique_error_classes_seen(N)` — does error-class coverage increase with N? (diversity signal)
  - `tx_to_OMEGA(N)` — does convergence speed up with N? (efficiency signal)
  - `n3_solve_minus_bernoulli_prediction(N)` — **excess** above independent-trial baseline = interaction signal

### Curve fitting (pre-registered)
After data collection:
- Fit Bernoulli model: p from N=1 SolveRate, predict all N
- Fit log model: a·log(N) + b
- Fit percolation model: Θ(N-N_c)·(N-N_c)^β (3 free params: N_c, scaling prefactor, β)
- Compare AIC/BIC across models → select

### What changes N_c (the iterative research program)

Each architectural fix potentially lowers N_c:
- Fix Art. II.1 broadcast → lower N_c (current Step-B)
- Fix Art. II.2 price signals → lower N_c (market correctly reflects proof quality)
- Fix Art. III.3 correlation shielding → lower N_c (diverse agent strategies)
- Fix Art. II.2.1 exploration/exploitation → lower N_c (Boltzmann temp tuned)

**Iterative cycle**:
1. Run N-scaling → find N_c
2. Diagnose bottleneck (which mechanism limits N_c?)
3. Step-B fix the bottleneck
4. Re-run N-scaling → verify N_c shifted
5. Repeat until N_c = 2 (minimal swarm enables cooperation)

## Cost estimate

| Scenario | Problems | N-values | Arms | Evals | Est. time (chat) | Est. $ |
|---|---|---|---|---|---|---|
| Full (50 probs × 6 N × 2 arms) | 50 | 6 | 2 | 600 | ~40h | ~$80 |
| Subset (20 probs × 6 N × 2 arms) | 20 | 6 | 2 | 240 | ~16h | ~$30 |
| Phase 1 (20 probs × 3 N × 2 arms) | 20 | 3 (1,3,8) | 2 | 120 | ~8h | ~$15 |

Recommend Phase 1 (sparse N-coverage) first to detect SHAPE, then fill in if shape is interesting.

## Connection to current experiments

- v3.1 (reasoner): N=1 data point (oneshot + n1 + n3) — Bernoulli baseline for reasoner
- v3.2 (chat, broken): N=1 and N=3 data points — Bernoulli baseline for chat
- v3.3 treatment (chat, fixed): N=3 data point — first treatment measurement
- N-scaling: extends to N=1,2,3,5,8,13 with both arms

v3.3 IS the first data point of the N-scaling experiment (treatment arm, N=3).

## User's additional insight: iterative code improvement

"现在的代码也并不能足够激发swarm的真正实力"

This means the N-scaling experiment is not just a measurement — it's a **diagnostic tool**. Each run reveals:
- WHERE the scaling curve bends (N_c location)
- WHY it bends (which mechanism is saturated)
- WHAT to fix next (targeted Step-B)

This is a **gradient descent on architecture** — each N-scaling run is an evaluation, each Step-B is a gradient step, N_c is the loss function.
