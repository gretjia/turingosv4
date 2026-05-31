# Multi-agent economy on Lean proving — the honest final verdict (price vs combination)

> 2026-05-31 · branch `claude/lean-market-baselines`. The calibrated, audit-grade conclusion after a full
> investigation: what the multi-agent economy IS and ISN'T on real Lean theorem proving. Every number
> below is from a frozen raw-data file, real Lean-verified, aggregated over seeds. FC1/FC2/FC3 untouched.

## Two findings, opposite signs — both real, both defensible

### FINDING 1 — PRICE is NOT the causal driver (a clean, literature-aligned NEGATIVE)
Four independent experiments, each a real Lean run aggregated over seeds, all converge: **price discovery
(capital-at-risk routing) does not causally beat a non-price baseline on Lean proving.**

| experiment | result | what it rules out |
|---|---|---|
| het4 H2 | realprice 2.83 ≈ shuffled 3.00 ≈ uniform 3.00 | price doesn't help when candidates are equal (Δ≈0) |
| LEAN-ALLOC (reasoner-$) | random 2.68 ≥ market 2.16 > shuffled 1.42 | price can't predict reasoner-repair success (s low) |
| compete (per-proof) | single-model all-NO; hetero a knife-edge cmp_ineq 4/4 vs 0/4 | price needs heterogeneous assessors AND is unstable |
| **PROBE-ALLOC (the decisive one)** | **market 3.00 = flatbid 3.00**; shuffled 3.75; roundrobin 2.25; uniform 1.75 | **the informative BID contributes nothing** |

The **flatbid firewall is decisive**: PROBE-ALLOC with constant (uninformative) bids closes exactly as
many conjuncts as PROBE-ALLOC with real informative bids (3.00 = 3.00). So whatever advantage the market
machinery has over uniform-random (3.00 vs 1.75) comes from the **STRUCTURE — a binding shared budget +
parallel bidding/scheduling — NOT from the price/capital signal**. shuffled ≥ market confirms it from the
other side. This is the cleanest possible negative for "price is the causal allocator."

### FINDING 2 — COMBINING complementary agents IS the real, validated value (a robust POSITIVE)
The multi-agent gain that survives every test is **combination of complementary limited agents**, proven
earlier and replay-/axiom-clean (`handover/reports/MARKET_EMERGENCE_FULL_REPORT_2026-05-31.md`,
`het_emergence_cells_2026-05-31.txt`): a market that COMBINES 4 specialists closes **market 3.81 > single
3.00 > single_spec 1.50** avg Lean-verified sub-goals (16 seeds); het4 deterministic **3/4 > 2/4 > 1/4**
on every seed. The whole strictly exceeds the best part; a lone specialist is capped at 1/4 by
construction. This is real emergence — but it comes from **routing each subtask to a complementary
specialist** (combination), not from price.

## Why — this matches the 2026 literature exactly (researched, not guessed)
The wall is the **Selection Bottleneck**: Q = s·O + (1−s)·M (arXiv:2603.20324) — selection beats random
only when candidate distinguishability Δ AND selector quality s are both high. On Lean proving:
- **Hong–Page E = M − D**: same-base-model agents sharing evidence ⇒ diversity D≈0 ⇒ a market aggregates
  nothing a single calibrated estimate couldn't.
- **Predicting "which attempt passes the kernel" is unreliable for formal proofs** (every prover —
  AlphaProof/HTPS/DeepSeek-Prover — samples-until-kernel-pass; LEAN-ALLOC's measured crux: error class
  did NOT predict repair, NEAR 0/2 vs FAR 1/3). With a cheap kernel, predict-then-route loses to
  check-everything.
- **A weak value/price signal is worse than none** (Nau pathology; PRM-MCTS ties Best-of-N at 10× cost;
  DeepSeek-Prover went reward-free). PROBE-ALLOC's flatbid=market is this result on our substrate.
The value lives in **complementarity** (different specialists cover different subtasks — real Δ), exactly
where Finding 2 wins, and NOT in price-as-value-estimate, exactly where Finding 1 loses.

## The calibrated claim TuringOS can defensibly make
> **TuringOS's proven multi-agent value is COMBINING complementary limited agents into a union no single
> agent covers** (market 3.81 > best-single 3.00 > floor 1.50, deterministic, every conjunct Lean +
> #print-axioms clean, replayable). The loss-bearing MarketTape + settlement + replay is the **auditable
> incentive / Sybil-resistance / provenance layer** — its value is *trust and accountability*, not raw
> performance. On this substrate, capital-priced routing is **not** the causal performance driver; a
> binding budget + complementary specialists is. This is consistent with the 2026 multi-agent literature.

What TuringOS should NOT claim: that price discovery makes the collective prove harder theorems than a
non-price scheduler. Four real experiments say it doesn't, and the literature says why.

## Constitutional discipline (first principle held throughout)
FC1/FC2/FC3 canonical hashes UNCHANGED (matrix_drift 3/3 every commit). No §6 restricted surface touched
(diff = src/bin/* diagnostic bins + src/sdk/actor.rs additive softmax + reports/fixtures/liveness only).
Integer money paths; f64 only in routing policy (chosen route on tape). Liveness 12/12. PR-only. Every
counted solve independently Lean-reverified + axiom-checked; every claim aggregated over seeds (single
runs gave false positives twice — both caught).

## What ships
The honest two-gate deliverable: **Finding 2 (combination economy) as the validated multi-agent result**,
**Finding 1 (price-not-causal) as a rigorous, literature-aligned negative that prevents over-claiming** —
plus the real, replayable Hayek market machinery (loss-bearing bets, settlement, MarketTape) as the
auditable substrate it genuinely is. A defensible, audit-grade truth, not a fabricated price win.
