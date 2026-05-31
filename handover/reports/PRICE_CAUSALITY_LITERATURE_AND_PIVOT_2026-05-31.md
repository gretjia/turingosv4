# Price-causality — the wall is a named phenomenon; the pivot that follows from it

> 2026-05-31 · branch `claude/lean-market-baselines`. After 3 honest negatives, I researched the
> literature (instead of more blind iteration) AND ran a cold strategy workflow. They INDEPENDENTLY
> converged on the same diagnosis + the same next experiment. This records both. FC1/FC2/FC3 untouched.

## The wall has a name: the Selection Bottleneck. Q = s·O + (1−s)·M
"When Agents Disagree: The Selection Bottleneck in Multi-Agent LLM Pipelines" (arXiv:2603.20324)
formalizes generate-then-select as **Q(T,s) = s·O(T) + (1−s)·M(T)** where O = best-candidate quality,
M = mean (random) quality, **Δ = O−M = candidate distinguishability**, **s = selector quality**.
Routing/selection beats random **only when s is high AND Δ is non-trivial**. Their measured numbers:
diverse pool + judge = 0.810 win-rate; **homogeneous pool + judge = 0.512 (≈ coin flip)**; synthesis
(blending) = 0.179 (worse than baseline). Crossover threshold s* ≈ 0.567.

**My three negatives map exactly onto this equation — they are not bad luck, they are algebra:**
- **het4 H2** (realprice 2.83 ≈ shuffled 3.00): Δ ≈ 0 — all funded proofs correct, nothing to distinguish.
- **LEAN-ALLOC** (random 2.68 ≥ market 2.16): s low — the Lean error class does NOT predict reasoner-repair
  success (measured crux: NEAR-miss repaired 0/2, FAR-miss 1/3; repair ~random). Confirmed by literature:
  Best-of-N collapses to majority/random under an uninformative reward model (Gao 2210.10760 Goodhart;
  "Majority of the Bests" 2511.18630).
- **compete** (single-model all-NO degeneration; hetero fixed it but knife-edge): low s from self-assessment
  correlation, raised by heterogeneous assessors — matches "juries > single judge" (2404.18796).

**Three hard literature constraints I had been violating:**
1. **Hong–Page: E = M − D** (group error = avg individual error − diversity). Same-model agents sharing
   evidence ⇒ D ≈ 0 ⇒ market aggregation buys NOTHING over one calibrated estimate. Mathematical identity.
2. **Predicting "which candidate passes the verifier" is unreliable for formal proofs** — every theorem-
   proving system (LeanDojo/HTPS/AlphaProof/DeepSeek-Prover) treats the kernel as the ONLY trustworthy
   judge and samples-until-pass. If the Lean check is cheap, predict-then-route is dominated by
   check-everything. (My LEAN-ALLOC asked price to predict repair success — exactly this hard case.)
3. **A weak value function is WORSE than no search** (Nau lookahead pathology; "Limits of PRM-Guided Tree
   Search" 2510.20272: PRM-MCTS ties Best-of-N at 10× cost because interior-node value correlation ~0.37
   is too weak). DeepSeek-Prover-V1.5 went REWARD-FREE (RMaxTS, diversity-seeking) precisely because the
   value signal was too weak to trust — when value is weak, maximize coverage, don't trust the price.

Sources: arXiv 2603.20324, 2502.00674 (Self-MoA: diversity can hurt), 2510.20272, 2205.11491 (HTPS),
2408.08152 (RMaxTS), 2305.20050 (Let's Verify), 2110.14168 (Cobbe verifiers), 2210.10760 (overoptimization),
2404.18796 (juries), Hong-Page PNAS. Full annotated list in the session research.

## The escape hatch the literature points to (and where price CAN be causal)
Routing/selection causally beats random ONLY by (1) creating real Δ — pool GENUINELY COMPLEMENTARY,
low-error-correlation agents — AND (2) pushing s above s* with heterogeneous/calibrated assessment. The
ONE place on the Lean substrate with both real loss-bearing price AND predictable variance is the
**specialist↔subtask match**: a omega-specialist can SEE (cheap, pre-Lean, reliable — the SKIP
self-selection already proves it) that a linear-nat goal is its family and a ring identity is not. That
predictable, exploitable Δ is exactly what all 3 negatives lacked.

## The decisive next experiment (research + strategy workflow agreed)
**PRICE AS ALLOCATOR of a SHARED BINDING PROBE BUDGET over complementary specialists.** The fix that
makes price causal where het4 failed: bind the **PROBE** (the proof-GENERATING LLM attempt), NOT the
verify — because het4's `closed` was governed by which conjunct got a funded proof (all correct), so
verify-order was irrelevant. When SKIP/wrong-family/fail each consume a scarce shared probe, MISallocation
costs a closed conjunct, and routing order finally matters.

Mechanism (≈150 LOC additive into lean_hayek_market.rs, Class 1-2, no FC/§6): het6 conjunction; 4
specialists place loss-bearing YES bids per open conjunct; price = (YES+α)/(YES+NO+2α) routes each of B
shared probes (B = k+2 = 8, pre-registered) to the highest-priced (conjunct,specialist) pair; Lean settles;
repeat until B exhausted. Every close Lean + #print-axioms reverified; MarketTape replays.

**Arms + the causal firewall:** market / roundrobin (het-emergence baseline, same B) / shuffled (price
permuted) / **flatbid (constant bids — THE firewall: if market ≈ flatbid, the gain is scarcity/structure,
NOT the informative bid)** / uniform (floor) / single_strong (reasoner given B — guards crippled-specialist
artifact). GO requires market ≥ roundrobin+1 AND ≥ shuffled+1 AND ≥ **flatbid+1** conjunct, ≥12 seeds,
majority-of-seeds, replicated on a held-out het-N, every close axiom-clean.

**Honest two-gate reporting (why this is safe to run):** even if price ties, we SHIP the already-proven
**combination economy** (het4/het6 market 3.81>3.00>1.50, deterministic floor, all Lean+axiom clean) as
the validated multi-agent claim — "TuringOS's proven value is COMBINING complementary limited agents into
a union no single agent covers; the loss-bearing MarketTape/settlement/replay is the auditable
incentive/Sybil layer, not the performance driver." A price-NO does not contaminate the combination-YES.
This is the calibrated, audit-proof posture the literature supports — not an over-claim.
