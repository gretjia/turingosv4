# P4-lite — price-causality test (H2): honest interim result + diagnosis

> 2026-05-31 · branch `claude/lean-market-baselines` · bin `src/bin/lean_hayek_market.rs` (commit 8943587a)
> Architect directive: prove PRICE DISCOVERY is the CAUSAL routing mechanism (not generic
> branching/parallelism). The decisive ablation: RealPrice vs ShuffledPrice vs CentralScore.

## What was built (real, validated)
A self-contained Class-1/2 Hayekian market bin grounded against code by a 5-agent workflow. The
ECONOMICS are genuinely real (this is the part the architect demanded vs the rejected informed-Bear
SCORING bypass), and the tape is replayable:
- **Real loss-bearing bets + settlement.** Each agent has a FINITE wallet (opportunity cost); a YES
  bet stakes integer micro-capital on its proof; a Lean REJECT forfeits the stake, a Lean ACCEPT
  settles (proposer takes the NO pool). Validated het4 seed-1: `realized_pnl_micro =
  [-40000, 0, -61950, +3000]` — capital genuinely lost on failed proofs and paid out on success.
- **MarketTape-lite.** Append-only, prev_hash-chained; `verify_chain()` green on 100% of cells
  (24/24 + 12/12); price re-derived from Invest events alone (`derive_pools`) — node.score never
  authoritative (Art. 0.2). Money integer throughout; f64 only in the routing softmax POLICY (the
  selected claim is taped → replay reads the choice).

## The result — H2 NOT supported on het4 (honest negative)
**Non-binding budget** (verify_budget=6 ≥ k=4), het4 × 6 seeds, all chains replay:
```
  uniform    3.00/4    shuffled  3.00/4    realprice 2.83/4    central 2.83/4
```
**Binding budget** (verify_budget=2 < k=4), het4 × 4 seeds, all chains replay:
```
  realprice 1.50    shuffled 1.50    central 1.25
```
RealPrice ≈ ShuffledPrice in BOTH regimes. **Price is not load-bearing on this task.**

## Diagnosis (why — and why it is NOT a mechanism failure)
Price routing has causal force only when the scheduler must choose WHICH funded work to verify under
scarcity **AND the funded options differ in true value**. het4 lacks the second condition:
- The het4 specialists (omega/ring/induction/nlinarith) each match exactly one conjunct, so every
  FUNDED proof is essentially correct. When all funded options are equally good, the routing ORDER is
  irrelevant — any policy closes the same count. Destroying the price signal (shuffled) costs nothing
  because there was no discriminative signal to destroy.
- Even with a binding budget, `closed` is governed by "which claims accumulated a funded proof," not
  "which funded proof we verified first." So budget scarcity alone does not make price causal.

This is the SAME lesson that has recurred all phase (an easy/low-variance task hides the mechanism),
now at the price layer: **a task with no funded-proof-quality variance cannot test price causality.**

## What price causality actually requires (the honest next step — NOT hand-tuning a win)
Price (competitive YES/NO bets + NO shorting) earns its keep when it must separate a TRUE proof from a
CONFIDENT-BUT-WRONG one under scarcity. That needs genuine **funded-proof-quality variance**, which the
honest source is model MIScalibration, not a fabricated trap (the architect explicitly forbids tuning a
conjunction until the market wins). Two structural changes, in order:
1. **Same-claim competition.** Let multiple agents bid on the SAME claim with proofs of varying quality
   (not one specialist per claim). Price then aggregates "which proof to trust"; NO shorts discredit
   confident-but-wrong proofs (architect H3). This is a harness-structure change (multi-proposer per
   claim + verify the price-selected PROOF, not just the price-selected claim), Class 1-2.
2. **Harder conjuncts** where a single specialist is only ~40-60% reliable, so YES bets carry real risk
   and the price has something to be right or wrong ABOUT. Use a frozen heldout generator (no
   hand-tuning), report calibration (for all "70% confidence YES", did YES win ~70%?).

Only after price shows causal force on a task WITH funded-proof-quality variance is H2 answerable. If it
still ties there, that is a real NO-GO: the win (if any) is branching/parallelism, not price — and we
report it honestly and pivot the framing (auditable substrate, not "the market thinks").

## Status vs the architect's 6 hypotheses
- H6 (constitutional replay): **PASS at lite grade** — wallet/price/route/failure/verify/LLM-call all on
  the MarketTape, chain verifies 100%, price re-derived from Invest events. (Full ChainTape/CAS port = P3.)
- H2 (price causality): **NOT YET ANSWERABLE on het4** — task has no funded-proof variance. Needs the
  same-claim-competition harness + a real-risk task. Honest interim: no price effect observed.
- H1/H3/H4/H5: pending the variance-bearing harness.

The economics + tape foundation is real and committed (8943587a). The remaining work is making the TASK
able to discriminate price — without fabricating the discrimination.

## Update — COMPETE mode built + run (the honest variance source)
Rather than hand-tune a trap, I built `compete` mode (cmp_* tasks): ONE hard goal, 6 agents each propose
a proof of naturally-varying TRUE quality (model miscalibration is the honest variance source), every
agent places loss-bearing YES/NO capital on every proof (peer assessment, never seeing the Lean oracle),
and under a scarce verify budget the router picks WHICH PROOF to verify. This DID produce real price
variance — e.g. cmp_sum `price_pm=[4, 508]` (market correctly bearish on a wrong proof), cmp_ineq funds
YES 374k–425k vs NO 174k–225k. Tape replays 100%, capital genuinely settles.

**But a deeper honest finding emerged (worth more than a forced win):** with a SINGLE homogeneous model,
peer-assessment betting is often pathologically miscalibrated. On cmp_pow, `yes_pools=[0,0,0,0,0,0]` —
EVERY bettor shorted EVERY proof; the "highest-priced" proof was merely the least-shorted, not a positive
signal, and ground truth (proof 3 correct) was ranked 4th by price. The model is uniformly underconfident
as an ASSESSOR of its own family's proofs, so YES never funds and price degenerates to a NO-volume
ranking. This is goal-dependent (cmp_ineq funds YES well; cmp_pow does not), not a coding bug.

**The real conclusion (decision point for the architect):** price-as-causal-signal needs INFORMATIVE
bets, and a single model assessing its own proofs is too correlated/miscalibrated to reliably produce
them — this is exactly the architect's **H4 (heterogeneity)**. The clean path to H2 is therefore THROUGH
P2/H4: heterogeneous ASSESSORS (deepseek-chat + deepseek-reasoner, or tool-checkers) whose independent
judgments make the price informative. Forcing a single-model price win by tuning cmp_* tasks until
realprice > shuffled would be precisely the hand-tuned victory the architect forbade — so I am NOT doing
that. Instead: the economics/tape/compete harness is real and committed; the next honest experiment is
heterogeneous-assessor price (merging P2 model-heterogeneity INTO the betting layer), which is where H2
and H4 become jointly testable.

### Committed artifacts
- `src/bin/lean_hayek_market.rs` (commit 8943587a + compete mode): real loss-bearing market + MarketTape
  + compete mode. Liveness 12/12, R-022 green.
- This report: the honest H2-on-het4 negative + the compete-mode miscalibration finding + the
  heterogeneity decision point.

## Update 2 — LEAN-ALLOC (price allocates the scarce reasoner-repair budget) — honest mixed result
Per the /goal authorization to ALTER the mechanism toward what proves economy validity, a 4-agent design
workflow picked the strongest benchmark: price allocates the SCARCE EXPENSIVE resource (deepseek-reasoner
repair calls), measured as VERIFIED THEOREMS BANKED PER REASONER-DOLLAR (the literal Hayek thesis + the
product thesis "cheap market beats one strong agent per dollar"). Built `run_alloc` (commit 75a6ee57):
cheap chat panel + free Lean-bank → residual → heterogeneous-assessor price → reasoner repairs spent in
the arm's order under a token budget B → settle. Cost split per model, integer micro-USD; replayable tape.

**De-risk MEASURED real:** chat free-banks only 7/44 pool theorems → residual 37 (genuine reasoner headroom).

**Causal smoke (market vs shuffled vs random, SAME 16 theorems, SAME budget, ONLY routing differs, 2 seeds):**
```
  random     avg_banked=5.0   per_reasoner_ktok=2.680   <- HIGHEST
  market     avg_banked=4.0   per_reasoner_ktok=2.164
  shuffled   avg_banked=3.0   per_reasoner_ktok=1.417
```
market > shuffled (price beats destroyed-price) BUT **random ≥ market** — a RED FLAG. If uniform-random
allocation matches/beats price allocation, the price is NOT the causal advantage. Honest reading: at N=2
this is market ≈ random ≈ noise; NOT a clean price win.

**Root-cause diagnosis (the real obstacle, same family as het4):** the price was asked to predict "will a
reasoner repair of this attempt succeed" — but reasoner repair success on this residual has little
PREDICTABLE variance (it's nearly a coin flip per theorem, uncorrelated with anything an assessor can see),
so routing order barely matters. No predictable repair-EV variance ⟹ price has nothing to discriminate ⟹
random ties price. This is the het4 lesson at the allocation layer.

**Next mechanism alteration (authorized; FC/§6 untouched):** make the price predict something an assessor
CAN judge and that DOES vary — e.g. "how close is this failed attempt to correct" read from the Lean error
class (a `type mismatch` / `unsolved goals` is near; an `unknown identifier` / parse error is far), and
tighten the budget so only ~2-3 repairs fit (real scarcity). Only if price then beats BOTH shuffled AND
random over ≥8 seeds is the economy's causal value proven. If it still ties random, the honest conclusion
is that on Lean-repair allocation the multi-agent gain (if any) is parallelism, not price — and we report
that and reframe (auditable substrate, not "price thinks"). Not chasing a tuned win.
