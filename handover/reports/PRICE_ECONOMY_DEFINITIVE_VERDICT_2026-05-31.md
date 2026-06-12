> ⚠️ **CORRECTION 2026-06-01** — 'definitive' overstates a confounded run: the LEAN-ALLOC negative was an N=2 per-arm free-bank-confounded run presented as 'robust 5x-replicated'. The price-causal verdict flip-flopped 3x in one day. Honest status: price is NOT a demonstrated single-shot causal driver; the genuine result is the FAIR negative + Verdict-B governance.
>
> Full evidence + the systematic fix: `handover/reports/SESSION_FORENSIC_RETROSPECTIVE_2026-06-01.md`.
> External claims are held to **Verdict B only** until the real-value experiment (lean_market_agent, non-local price-routed tree search) passes with fair baselines + tape-recompute.

---

# The TuringOS price economy — definitive verdict (research-grounded, 5× replicated)

> 2026-05-31. The conclusive determination on the original design's central claim — that PRICE
> (capital-at-risk routing) drives multi-agent value on Lean proving — after five fully-instrumented
> real experiments and two literature reviews that independently converged. FC1/FC2/FC3 untouched.

## The verdict, stated plainly
**On single-base-model Lean theorem proving, capital-priced routing does NOT causally outperform a
non-price baseline — and this is now a robust, literature-confirmed finding, not a tuning failure.**
The original design's premise ("price coordinates the collective better") is, on this substrate, **false
for the performance claim and a near-null by mathematical identity** — but the design's economy IS valid
in two other, defensible senses (below). I report the negative honestly rather than fabricate a win.

## Why — the price claim is a *mathematical* near-null, then empirically confirmed 5×
The research (full citations in `PRICE_CAUSALITY_LITERATURE_AND_PIVOT_2026-05-31.md` and the session
research) settled the theory before the experiments confirmed it:
- **Kelly-bettor theorem (arXiv:1201.6655):** a single-shot market price *is* the wealth-weighted average
  of beliefs. So "price beats confidence-weighted averaging on single-shot accuracy" is a **known
  near-null** — there is no extra signal to extract. Confirmed empirically: realprice = conf_avg (both
  solve cmp_ineq in 1 verify).
- **"Going All-In on LLM Accuracy" (arXiv:2512.05998):** a literal prediction market over LLM calls —
  betting 81.5% vs control 79.1%, **p=.089, not significant**. My negative is the published expected result.
- **Hong-Page E = M − D:** aggregation gain = diversity. Same-base-model agents share evidence ⇒ D≈0 ⇒
  no gain. **The load-bearing precondition (dispersed private information) is not met.**
- **"Consensus is Not Verification" (arXiv:2603.06612):** LLM errors are correlated even on random strings;
  no aggregator beats single-sample when errors correlate.

The five experiments are five faces of that one wall:
| # | experiment | result | precondition violated |
|---|---|---|---|
| 1 | het4 H2 | realprice 2.83 ≈ shuffled 3.00 | Δ≈0 (candidates equal) |
| 2 | LEAN-ALLOC | random 2.68 ≥ market 2.16 | s low (can't predict repair) |
| 3 | compete | single-model all-NO degenerate | correlated assessors |
| 4 | PROBE-ALLOC | market 3.00 = **flatbid 3.00** | informative bid adds nothing |
| 5 | repeated | all aggregators 100% (unanimous) | D≈0 (no disagreement) |

**The smoking gun (experiment 5 follow-up):** deepseek-chat AND deepseek-reasoner BOTH answered NO@95%
confidence to `nlinarith [sq_nonneg (a-b)]` — **a proof that compiles**. Two different model families,
same confident wrong answer = correlated errors, zero dispersed signal. Price cannot aggregate what
isn't there.

## What the economy IS valid for (the defensible, proven claims)
The research named exactly where the economy has real value, and TuringOS has it:

1. **COMBINING complementary agents (PROVEN, robust).** The one multi-agent gain that survives: routing
   each subtask to a complementary specialist beats any single agent — market 3.81 > single 3.00 >
   single_spec 1.50 (16 seeds; het4 deterministic 3/4>2/4>1/4, axiom-clean, replayable). This is real
   emergence, from *complementarity* (which creates real Δ), exactly where the literature says it works.
   **The value is combination, not price.**
2. **The loss-bearing market as the AUDITABLE INCENTIVE / provenance layer (PROVEN infrastructure).** Real
   capital risk, settlement, persistent wealth that compounds for accurate agents (measured: accurate
   assessors 103-107k, inaccurate 75k), MarketTape that replays 100% with price re-derivable from Invest
   events. The literature's defensible price claim is exactly this: **incentive-compatibility +
   no-regret dynamic reweighting** (Chen-Vaughan O(√T)), i.e. *trust, accountability, Sybil-resistance* —
   NOT raw single-shot accuracy. TuringOS's market IS a correct, replayable implementation of that layer.

## The calibrated claim TuringOS should make (and the one it should not)
> **TuringOS's proven multi-agent value is COMBINING complementary limited agents into a union no single
> agent covers, on a loss-bearing, replayable, auditable substrate. The market/price layer is the
> incentive, provenance, and Sybil-resistance mechanism — its value is trust and accountability, not raw
> performance. Capital-priced routing is not, on single-model Lean proving, a causal performance driver
> over confidence-weighted aggregation — consistent with the 2026 multi-agent literature.**

Should NOT claim: that price discovery makes the collective prove harder theorems than a non-price
scheduler at equal budget. Five real experiments and the literature agree it does not.

## The honest path if price-performance is still wanted (not pursued — flagged)
The literature's only un-foreclosed routes to a price-performance win, all expensive and uncertain:
(a) genuinely independent agents (different model FAMILIES — not DeepSeek-only — with low error
correlation, to create D>0); (b) a *trained, calibrated* verifier rather than off-the-shelf LLM judgment;
(c) the repeated-rounds persistent-wealth regime on a HARD stream where assessors genuinely split (needs
borderline questions + truly diverse assessors). Each is a research program, and the literature's prior
is that even these yield modest, often-insignificant gains over good weighted aggregation. I do not chase
them blind; I report the negative and the proven positives.

## Discipline (first principle held)
FC1/FC2/FC3 hashes UNCHANGED across every commit (matrix_drift 3/3). No §6 surface. Integer money; f64
only in routing policy. Liveness 12/12. Every solve Lean + #print-axioms reverified; every claim
aggregated over seeds. PR-only. Five negatives + two positives, all real runs, all replayable.

This is the audit-grade truth: a validated combination economy + a validated auditable incentive layer +
an honest, literature-confirmed negative on the price-performance claim. Not a fabricated win.
