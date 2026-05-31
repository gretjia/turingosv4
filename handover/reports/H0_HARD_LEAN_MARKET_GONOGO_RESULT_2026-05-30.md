# H0 pilot result — Hard Lean Market Go/No-Go

> 2026-05-30 · branch `claude/lean-market-baselines` (PR #220) · binary `src/bin/lean_market_agent.rs`
> Pre-registration: `handover/preregistration/HARD_LEAN_MARKET_GONOGO_PREREG_2026-05-30_v2.md`
> Verdict: **NO-GO** (robust, replay-verified) for Thesis A on monolithic Lean theorems at this budget.

## 1. Verdict

**The price-routed market is NOT stronger than a single agent — it is weaker.** Under equal model
(deepseek-chat), equal budget (8 proof attempts), equal wall-time, equal verifier (Lean kernel,
`Verified` only), on 3 pre-selected headroom theorems × 4 seeds (72 cells, all replay-green), every
pre-registered NO-GO condition was met.

## 2. Data (solve-rate @ equal budget, all cells replay-verified)

| arm | solved | rate | |
|---|---|---|---|
| **B1 single-agent** (1 agent × 8 sequential rounds, Lean feedback) | **7/12** | **58%** | ← best |
| A0 shuffled-price (market, price signal permuted) | 6/12 | 50% | |
| **M0 MARKET** (informed-Bear price routing) | **5/12** | **42%** | treatment |
| B2 parallel (Bulls-only, no shared tape) | 4/12 | 33% | |
| M1 random-Bear (market, random short) | 4/12 | 33% | |
| B6 skeptic-rerank (critic-matched, argmin doubt) | 3/12 | 25% | ← worst |

Per-theorem (M0 vs A0): cube4 1-1, mono 2-2, monotone_glue 2-3. Headroom theorems calibrated at
single-agent 33% (K=3). **Replay gate: 29/29 OMEGA cells reconstruct clean from ChainTape+CAS
(`replay_failure=null`, `economic_state_reconstructed=true`); every OMEGA a real Lean `Verified`
(LeanJudge rejects `sorry`/`admit`/`native_decide`).**

## 3. Pre-registered NO-GO conditions — all met

- ✅ "M0 fails to beat B1_single_agent at equal total budget" — **single 7 > market 5**.
- ✅ "A0_shuffled / B3_no_price tie M0 within noise" — **A0 6 ≈ M0 5** (A0 *higher*).
- ✅ "M0 fails to beat B2_parallel" — **market 5 ≈ parallel 4**.
- ✅ "M1_random_Bear ties M0" — **M1 4 ≈ M0 5**.
- ✅ No WEAK-GO escape: time-to-first-proof shows no M0 advantage either (M0 5-31s, A0 5-16s when solved).

This is not a goalpost-moved or under-powered fluke: the *direction* is wrong (single > market), and
it is consistent across all three theorems.

## 4. Why — the mechanism, from per-cell evidence

Every cell is **bimodal: solved fast (1-2 attempts, ~5-16s, <2k tokens) or never (burns all 8,
~4-9k tokens)**. There is no "assemble-through-branching" middle state. Consequences:

1. **These proofs are solved by a lucky/skilled fresh attempt, not deep collective search.** So the
   market's core mechanism — route budget to the most promising *partial* node — has nothing to bite
   on: extending a failed prior attempt is worse than a fresh attempt.
2. **Single-agent sequential refinement (one chain, 8 tries, Lean error feedback) is the BEST use of
   budget** for this task type — it is a *depth* problem (iterate one proof against the compiler),
   not a *breadth* problem (search many branches). The market *distributes* budget across parallel
   branches, giving each fewer sequential refinement steps → it actively hurts.
3. **The LLM skeptic is anti-informative** (B6 worst, 3/12): its doubt mis-ranks which node to extend.

5. **Deepest insight — the strong verifier removes the signal the market needs.** The market routes on
   **price**, which is supposed to reflect *graded partial progress*. But a Lean kernel verdict is
   **binary** (compiles or not) — there is no graded partial-progress signal on a monolithic theorem.
   The v3 ζ "emergent DAG" formed under a *weak* judge that let partial sentence-steps accumulate;
   the strong kernel that makes this experiment rigorous **also collapses the partial-progress
   gradient the market exploits.** The rigor and the mechanism are, on monolithic theorems, in tension.

## 5. Honest scope of the claim

This is a robust NO-GO **for: a price-routed market over full-proof-attempt nodes, on monolithic Lean
theorems, under a binary kernel verdict, at single-digit-attempt budget.** It does **not** test:
- **decomposable / graded-progress tasks** where verified sub-lemmas compose and are reusable across
  branches (the market's plausible natural home — untested, and would require a *different* mechanism
  that assembles verified sub-results, not one that refines full-proof attempts);
- **weak-verifier regimes** (where the v3 emergence was seen — but those are exactly the weak-judge
  OMEGA the constitution forbids counting).

## 6. Recommendation (architect's decision)

The market mechanism, as built, does not amplify proving on this task class. Three honest forward paths:

- **A. Accept the NO-GO + re-position.** Per prereg §1: TuringOS's defensible claim becomes the
  **auditable / replayable / constitutional substrate** (every attempt, price, failure, and OMEGA is
  tape-reconstructable — *that* held perfectly: 29/29 replay-green), **not** "the market out-thinks a
  single agent." Honest, and the replay/audit property is genuinely strong + unique.
- **B. Test the market's actual hypothesis once, properly:** build a **sub-lemma-assembly** task +
  mechanism (agents propose+verify intermediate lemmas; the market routes on *verified sub-progress*;
  a final proof composes them) on 1-2 genuinely multi-step theorems. This is the only setup where
  routing-on-progress can bite. If the market loses there too → strong global NO-GO.
- **C. One diagnostic run** to confirm §4.5: rerun M0 vs single under a *graded* judge (e.g. count of
  verified sub-goals) to show price-routing only helps when progress is graded — converting the
  insight into evidence.

My honest lean: the result is real and should not be spun as anything but a NO-GO for the
strong-claim. But before the *global* conclusion "the market idea is wrong," path **B** is worth one
clean shot, because §4.5 says we may have tested the market on precisely the task structure (binary,
monolithic) where it *cannot* help — single-agent sequential refinement is simply optimal there.

## 7. What stands regardless of the verdict

The **substrate** is proven and unique: integer-CPMM market, informed-Bear price discovery, 10-arm
matrix, LeanJudge strong verifier (no-`sorry`), on-chain VerifyTx+VerificationResult, and **100%
deterministic replay** of every counted cell from ChainTape+CAS alone. The negative capability result
does not diminish the auditable-substrate result — it sharpens what to claim.
