# TB — Dynamic Model-Budget Market (H-HET-2) — CHARTER (draft v2, architect-redlined)

**Status:** DRAFT for architect review (v2, incorporating the 2026-06-15 architect
redline). §8 `ProposalTelemetry.model_id` hard gate is **SATISFIED** and **Veto-AI
PASS** is recorded (commit `51c1d602`; per-proposal model provenance is now
tape-canonical). **No H-HET-2 paid run** until ALL freeze gates in §11 pass:
charter frozen + `BudgetAllocationTelemetry` tape gate + BestHOMO control defined +
target pool frozen with leakage guard + paired primary metric frozen + budget
fairness (tokens+microUSD+calls+router overhead) frozen + bandit/pricing policy
sha-pinned + audit split + Veto-AI constitutional PASS + architect sign-off.

**Authority (anchored on SHAs, not a bare date — avoid future-dated authority):**
architect independent audit + redline 2026-06-15; §8 satisfied by commit
`51c1d602` (Veto-AI PASS); carrier freeze `f73163f4`; experiment evidence
`61df15ed`. Direction grounded in the V3 "Good market example" (ζ-Sum Run 6:
90 agents × 6000 tx, Qwen2.5-7B reaches OMEGA on an 18-step golden path, DAG
reconstructed from tape) — see the **V3 budget-regime prior** in §3.

---

## 1. Why this experiment exists (what the H-HET-1 pilot did and did NOT show)

The H-HET-1 pilot (commit `61df15ed`, audited NO-VIOLATION) is a *scoped null*: on
a det-family band, at 3 tx/agent (NA=4, NR=3 = 12 proposals/cell), with proofs of
golden-path depth mean 1.1 / max 2 (one-shot theorems), a fixed round-robin
heterogeneous market did not beat the best single model (Q397) on solve rate
(Wilson CIs overlap) or token-economics (Q397-homo dominated PPUT).

That null says nothing about the architecture's thesis, for two structural reasons
(stated as a *prior*, not a proven law):

- **Budget regime.** The V3 budget-regime prior (a single-run observation, not a
  multi-point-replicated scaling law): deep proof search appears to need
  `tx_budget ≳ agents × 20` — V3 got depth 5 / NO proof at ~3.3 tx/agent and depth
  18 / OMEGA at ~19 tx/agent. 3 tx/agent is the failure regime; a collaborative
  market cannot express value when each agent gets ~3 turns.
- **Depth regime.** Det-family theorems are one-shot (`simp [...]; norm_num`). A
  market's value is collaborative *chain-building* on theorems no single model
  one-shots; shallow targets let a strong single model win trivially.

The genuine residual signal from H-HET-1 is **complementary coverage**: at the
homogeneous level DSHOMO uniquely solves `{lm_det_zero, lm_det_3x3}` and Q397HOMO
uniquely solves `lm_det_2x2`; no single model covers all. The current carrier routes
*which node*, never *which model gets budget*, so it cannot convert latent coverage
into a win. **This charter builds and tests the missing mechanism.**

## 2. Treatment vs control (the lever is budget ROUTING, not the roster)

- **TREATMENT — dynamic model-budget market.** A priced/bandit carrier that
  reallocates the scarce resource (proposal-call / token budget) toward whichever
  model the price + abstracted-failure signal favors (Art II.2 price broadcast +
  Art II.2.1 explore/exploit + Art II.1 abstracted failure memory via Librarian).
- **CONTROL-1 — BestHOMO (REDLINE #1, was Q397HOMO).** "Best single model" is NOT
  assumed to be Q397 on H-HET-2 targets — Q397 was best only in the H-HET-1
  shallow one-shot regime, and H-HET-1 already showed DeepSeek uniquely solves some
  theorems Q397 cannot. So CONTROL-1 is **either** (a) ALL eligible single-model
  homogeneous arms run at the same total budget (BestHOMO = the max over them),
  **or** (b) a single BestHOMO model pre-selected on a DISJOINT frozen calibration
  set before any confirmatory seed. **Q397HOMO remains mandatory** (it dominated
  H-HET-1) but is NOT assumed globally best. The headline compares TREATMENT vs
  BestHOMO, not vs Q397HOMO. Without this, beating Q397 only shows "dynamic market
  beats the best *shallow* model," not "beats the best single model."
- **CONTROL-2 — fixed round-robin heterogeneous** (the H-HET-1 carrier) at the same
  budget. Isolates "dynamic routing" from "heterogeneous roster."

The claim is NOT "heterogeneity helps." It is: **dynamic budget routing converts
complementary coverage into capability BestHOMO cannot match at equal-or-lower
total cost.**

## 3. Targets — deep, hard, budget-Goldilocks, with a leakage guard (REDLINE #4)

Pre-select theorems where (a) no single model one-shots them, AND (b) the
budget-Goldilocks property holds (some model 0/K and another ≥1/K at the
experiment's per-agent budget). **Leakage guard:** the target pool, difficulty
labels, depth labels, and BestHOMO selection are derived ONLY from disjoint
calibration seeds / historical data. The confirmatory K≥12 seeds are NEVER used for
target inclusion, difficulty/depth labeling, or BestHOMO selection; any target whose
inclusion depends on confirmatory outcomes is excluded from the primary metric.
**Depth proxy:** "deep" is defined PRE-RUN via reference-proof decomposition, known
fixture metadata, or a frozen calibration solve-DAG — NEVER via the post-hoc solved
DAG. Source pool = the deeper theorems in `tests/fixtures/lean_theorems_pool.jsonl`
+ V3-class multi-step targets, EXCLUDING the det-family one-liners. Budget per the
V3 budget-regime prior: **tx/agent ≳ 20** (e.g. NA=4..8, NR≥20, or larger agent
pools à la V3's 90 agents if rate limits allow). Budget must BIND
(unsolved-at-budget is a real outcome, not a tooling artifact).

## 4. Primary claim + metrics — paired, frozen before any paid run (REDLINE #3, #5)

**Primary unit = the seed×target paired cell.** A single "≥1 theorem once" result
is an OBSERVED witness only and may NOT trigger a SUPPORTED/PROVEN headline (H-HET-1
showed a one-cell swing is sampling noise at K=3).

- **Primary A (confirmatory, paired):** `Δ_union = Σ I[TREAT solved ∧ BestHOMO
  unsolved] − Σ I[BestHOMO solved ∧ TREAT unsolved]` over target×seed cells, tested
  with paired exact sign / McNemar OR within-seed Wilcoxon (preregistered choice).
- **Primary B (existence witness):** ≥1 target where TREAT solves in ≥m/12 seeds AND
  BestHOMO solves in 0/12 (or statistically fewer paired seeds), replay-clean +
  axiom-clean.
- **Economic dominance (REDLINE #5):** equal budget = same-or-lower total
  input+output **tokens** AND same-or-lower **microUSD** (model-specific rates
  differ — Q397/DS/Q32/GLM are not equally priced) AND same-or-lower **proposal-call
  cap** AND **all router/model-selection LLM/tool overhead counted**. The capability
  metric may be token-pure; the *economic-dominance* claim requires BOTH token and
  microUSD non-inferiority.
- **Secondary:** serial (uncontended) PPUT (ΣPPUT, Mean-PPUT(solved)) — never
  concurrency-contaminated wall-clock as primary; per-model budget share vs
  per-model verify rate (did budget flow to the winning model?); golden-path tokens.
- **No PROVEN headline** without §17 G1–G6.

## 5. Hard gates (architect, including REDLINE #2 new gate 5.4)

1. **`model_id` tape-canonical — SATISFIED** (commit `51c1d602`, Veto-AI PASS).
2. **Serial / token-pure primary metric pre-registered** (§4); contaminated
   wall-clock PPUT may not be the canonical primary.
3. **Dynamic policy frozen before paid run** — the bandit/pricing rule sha-pinned;
   reintroducing it post-hoc is p-hacking + Goodhart risk (Art III.4).
4. **5.4 BudgetDecision must be tape-canonical (REDLINE #2 — the core treatment must
   be auditable).** `model_id` records WHO produced a proposal; the H-HET-2 treatment
   is HOW budget is routed, so every routing tick MUST emit `BudgetAllocationTelemetry`
   to ChainTape/CAS: `policy_hash`, `policy_version`, `input_state_cid`, visible price
   vector, abstracted failure features, per-model score, exploration-floor state, RNG
   seed/draw (if stochastic), selected `model_id`, allocated proposal/token budget,
   and reason code. Replay assertion: `allocation_view == derive_from_tape(tape)`.
   Without this the DAG artifact only shows where budget *went*, not that it went
   there *by the frozen policy* — the treatment would be unauditable (Art 0.2).

## 6. Power

K ≥ 12 preregistered seeds, within-seed Wilcoxon (or McNemar/sign) pairing
(TREATMENT vs each control on the SAME seed/target). Goldilocks pool pre-selected
per §3 (leakage-guarded). Stable across a later-day re-run (no single-day flip).

## 7. Required artifact — tape-reconstructed DAG (the V3 deliverable)

Per solved target, emit — **reconstructed from the frozen tape, not the manifest
sidecar** — a V3-style report: citation DAG (roots → golden path → branches),
golden-path steps with author agent + **model** (tape-canonical via §8) + the
**budget-allocation decision** that funded each step (via §5.4), role/model activity,
trading/price breakdown, whale/contested nodes. `assert view == derive_from_tape(tape)`.
This is the direct visual test of whether budget flowed to the winning model.

## 8. FC trace + constitution alignment

- **Art II.2 (price broadcast drives emergence)** — finally implemented at the level
  that matters: budget (the scarce resource) is priced and routed, not just node
  prices broadcast. H-HET-1's carrier was a half-implementation.
- **Art II.2.1 explore/exploit — testable exploration floor (REDLINE #7).** Each
  eligible model receives ≥ ε of proposal budget per target until either (a) it
  accumulates N consecutive tape-recorded hard failures under comparable context, or
  (b) preregistered budget exhaustion. ε, N, and the failure-class definitions are
  frozen before the paid run (not narrative — a gate).
- **Art III.3/III.4** — shield horizontal correlation (keep models decorrelated);
  keep the allocation metric non-gameable (the score inputs are tape-recorded so
  Goodhart is auditable).
- **Art 0.2** — model provenance (§8 ✅) AND budget-decision provenance (§5.4) both
  tape-canonical.
- **Art 0.4 PATH DECLARATION (REDLINE #8 — now a known status, not a TODO):** §8
  landed under **Path B** for the carrier's current `Git2LedgerWriter`-backed L4
  substrate (commit `51c1d602`); that local CAS-schema repair does NOT close the full
  `Q_t=⟨q_t,HEAD_t,tape_t⟩` debt (HEAD_t witness, q_t serialization, rtool/wtool HEAD
  axis remain open). Any new routing/tape-schema commit (e.g. `BudgetAllocationTelemetry`)
  must repeat the Path-B declaration or explicitly declare a different sudo-approved path.
- FC1 N7 (predicate verify) / N11 (output-evidence) via the Lean judge; FC1 N5
  (broadcast injection) via the priced budget signal.

## 9. Predecessor / dependency

1. **§8 `model_id` — DONE** (commit `51c1d602`, Veto-AI PASS). Hard-gate 5.1 cleared.
2. **`BudgetAllocationTelemetry` schema (gate 5.4) MUST land before any paid run** —
   a new tape/CAS schema (Class 3), gate-first + real evidence + Veto-AI, with its
   own Art 0.4 Path-B declaration.
3. Freeze branch `claude/het-carrier-freeze` (`51c1d602`) is pushed; merge to the main
   experiment path remains gated (architect call).
4. New carrier routing logic = Class 2-3 change to `lean_market_agent.rs`
   (model-budget allocation) — gate-first, real evidence, clean-context audit.

## 10. Done definition + audit split (REDLINE #6)

Touched FC nodes + risk class stated; the dynamic-routing carrier passes unit +
constitution gates; a frozen-policy, prereg'd, K≥12 paid run lands replay-clean +
axiom-clean evidence; the tape-reconstructed DAG (incl. budget decisions) is produced;
primary metric (paired union delta + existence witness) computed; §17 honored.
OBLIGATIONS reconciled. **Post-data review is TWO separate tracks:**
- **Veto-AI** — constitutional PASS/VETO **only** (Art V.1.3; no quality/perf/coverage
  opinions).
- **Scientific/economic clean-context audit** — recomputes solve counts, paired
  statistics, PPUT/token/microUSD, DAG reconstruction, headline validity; verdict
  domain `{SUPPORTED | NOT-SUPPORTED | ANALYSIS-ERROR | RECONSTRUCTION-FAILURE}`.

## 11. Freeze-gate criterion (architect's 8 points — ALL required to freeze)

1. §8 `model_id` tape-canonical — **SATISFIED** (`51c1d602`, Veto-AI PASS).
2. `BudgetAllocationTelemetry` / router decision tape-canonical (§5.4) — **OPEN**.
3. BestHOMO control defined (all-homo primary OR disjoint-calibration BestHOMO) — **OPEN**.
4. Target pool frozen + leakage guard + depth proxy frozen pre-confirmatory — **OPEN**.
5. Primary metric frozen: paired union delta + existence-witness threshold (not single ≥1) — **OPEN**.
6. Budget fairness frozen: tokens + microUSD + proposal calls + router overhead — **OPEN**.
7. Bandit/pricing policy sha-pinned; exploration floor numeric (ε, N); no hindsight;
   all policy inputs reconstructible from tape — **OPEN**.
8. Audit split: Veto-AI constitutional only; scientific audit separately recomputes — **OPEN** (encoded in §10).

> Pre-existing, tracked separately (NOT an H-HET-2 freeze blocker, but must NOT be
> normalized indefinitely before merge / final paid run): the `.claude/hooks/judge.sh`
> stale trust-root manifest hash (Class-4 inconsistency at HEAD; clean@HEAD, from
> commit `92c6ffe6`).
