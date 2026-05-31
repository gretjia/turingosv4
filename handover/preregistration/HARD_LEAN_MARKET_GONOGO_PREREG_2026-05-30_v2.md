# Pre-registration v2 — Hard Lean Market Go/No-Go (informed-Bear ratification + ablation expansion)

**Supersedes:** `HARD_LEAN_MARKET_GONOGO_PREREG_2026-05-30.md` (v1) for §4 (arms), §6 (metrics),
§7 (decision rule). v1 §1-§3, §5, §8-§9 remain in force unchanged.
**Date registered:** 2026-05-30
**Risk class:** 0 (this document)
**Trigger:** architect ratification of the informed-Bear price-discovery mechanism (this session),
with bounded approval + 7 frozen rules + 3 new ablation arms.
**Status:** REGISTERED — LOCKED before any counted run. No counted run has begun under v2.

> **Why v2.** v1 §4 had Bulls posting WorkTx-Long and a *fixed* ChallengeTx-Short. A faithful
> `distinct_price_ratios` metric exposed price collapse to **1 distinct price**: every Long
> self-reports ~100% confidence, prices pin at max, Boltzmann routing degenerates to random —
> a *Bulls-only voting machine*, not a market. MARKET vs A0 in that regime would be a **false
> no-go** (testing parallel sampling, not price-routing). The fix (P0-E, PR #220): a
> wallet-limited **informed Bear** — an independent skeptic LLM that prices downside risk via
> integer Shorts — restored Bull-vs-Bear price discovery (spread 0.41-0.80, ≥6 distinct prices).

## A. Architect ratification (binding text)

> Approve wallet-limited **informed Bear shorts** as the market **price-discovery** mechanism.
> The Bear is a **bettor, not a judge**. Bear token/cost counts toward budget. Settlement's sole
> source remains the **Lean kernel**. Bear prompt, stake mapping, model, temperature, parser, and
> wallet cap must be **pre-registered** and written to **tape/replay**.

This is not a hack: it restores the market from a Bulls-only vote to a Bull-vs-Bear prediction
market. The boundary that keeps it a *market* and not an *LLM grader*: **the Bear never emits
`accepted`/`rejected`/`omega`/`proof_valid`** — only a priced risk estimate.

## B. The 7 frozen Bear rules (LOCKED)

1. **Bear calls count toward total budget.** MARKET's Bull+Bear tokens, latency, and USD cost all
   enter the MARKET account. Fairness is enforced two ways, **both required for the headline**:
   (a) *budget-inclusive* — A0/baselines get token budget equal to MARKET's Bull+Bear total; and
   (b) *critic-matched* — the **B6 skeptic-rerank** baseline gives a non-market arm the *same*
   skeptic resource (propose→critique→revise). A MARKET win that does not also beat B6 is "a
   critic helped," not "the market helped."
2. **Bear affects price only, never the Lean verdict.** Bear MAY output `p_fail`, `short_amount`,
   `skeptic_rationale`, `model_id`, `prompt_version`, `normalization_version`, `wallet_delta`.
   Bear MUST NOT output `accepted` / `rejected` / `omega` / `proof_valid`. Settlement = LeanJudge
   kernel verdict only.
3. **Bear is wallet-constrained.** Bear shorts from its own wallet; a failed short has an economic
   consequence; total Bear risk exposure is capped; stake↔doubt mapping is **integer** and
   replayable; bankruptcy / cap / liveness all go on tape. (A free, unbounded short is a hidden
   re-ranker, not a market participant.)
4. **Bear parameters pre-registered + on tape (see §C).** No tuning Bear after seeing theorem
   outcomes. Any change → new experiment number.
5. **Bear sees no private verifier information.** Allowed: candidate proof, public history nodes,
   public Lean error feedback. Forbidden: hidden oracle, future verifier result, other seeds'
   results, held-out stats. (Critical for the later SWE-bench shadow-run: the Bear must never see
   hidden-test output, or it becomes a verifier side-channel.)
6. **Bear information quality is measured separately, before any headline:** Brier score on
   `p_fail`, ECE / calibration curve, AUC / rank-correlation of doubt vs Lean outcome,
   price→verifier-accept correlation, and **informed-Bear vs random-Bear**. A Bear whose doubt is
   uncorrelated with the Lean outcome only injects noise.
7. **A skeptic-only baseline (B6) is mandatory** (see §D) — to prevent attributing a critic-heuristic
   gain to "the market."

## C. Bear parameter block (LOCKED — written verbatim into every run manifest)

```
bear_model            = deepseek-chat            # same base model family as Bulls (Thesis A)
bear_temperature      = 0.0                       # deterministic skeptic
bear_prompt_hash      = <sha256 of the frozen skeptic prompt, recorded at run start>
bear_context_policy   = {candidate_proof, public_prior_nodes, public_lean_error}  # rule 5 allow-list
p_fail_parser         = strict-json:"p_fail" in [0,1]; malformed -> abstain (no short)
p_fail_to_short_amount= integer: short_micro = round(MIN_SHORT + p_fail*(MAX_SHORT-MIN_SHORT))
min_short             = 250 (micro)    # MIN_SHORT_MICRO
max_short             = 8000 (micro)   # MAX_SHORT_MICRO (matches the validated P0-E code constant)
wallet_cap            = per-Bear escrow cap; exhausted Bear abstains (logged, on tape)
timeout_policy        = bear LLM timeout -> abstain (no short), counted as a Bear call in budget
malformed_json_policy = abstain (no short), counted in budget + parse_fail telemetry
```
Any edit to this block → a NEW `..._vN.md` before the next counted run.

## D. Arms (REPLACES v1 §4)

| arm | definition |
|---|---|
| **M0 MARKET informed-Bear** (treatment) | price-routed loop; Bull WorkTx-Long + wallet-limited **informed-Bear** ChallengeTx-Short priced by `p_fail`; Boltzmann routing over CPMM price; Lean settles. |
| **M1 MARKET random-Bear** (ablation) | identical to M0 but Bear `p_fail` ~ Uniform(0,1) (no skeptic call). Tests whether merely *scattering* prices is enough. |
| **M2 MARKET fixed-Bear** (ablation) | identical to M0 but every Short is the v1 fixed amount (no `p_fail`). Tests whether a constant counter-stake is enough. |
| **A0 SHUFFLED-PRICE** (decisive ablation) | byte-identical to M0 except the price vector is permuted before parent-selection. Only the routing signal is destroyed. |
| **B1 single-agent** | one agent, equal total budget; best of its own chain. |
| **B2 parallel Bulls-only** | N independent agents, no shared tape, no prices; first kernel-`Verified` wins. |
| **B3 no-price multi-agent** | N agents share the DAG; uniform-random parent (prices stripped). |
| **B4 strict majority-vote** | N independent proofs; accept only if ≥k agree (Lean still gates). |
| **B5 best-first** | shared tape, parent = argmax(progress heuristic), no betting/short/CPMM. |
| **B6 skeptic-rerank** (critic-matched, NEW) | **no market**: the *same* Bear skeptic scores candidate proofs by `p_fail`; pick / continue lowest-`p_fail`. Isolates the skeptic heuristic from the market. |

Budget note: M0/M1/M2 Bull+Bear tokens, and B6 critic tokens, all count toward each arm's equal
total budget (rule 1). Bear/critic latency counts toward the wall-time cap.

## E. Two-level experiment structure (naming correction)

The single-theorem run is a **pilot**, not the headline.

- **H0 — decisive pilot:** 1-3 headroom theorems × 8 seeds × the arm set below. Question: *is M0
  clearly not A0 (and not B6/M1)?* Fast mechanism go/no-go. **Not** a headline; one theorem can win
  on selection bias / prompt luck / a Bear heuristic coincidence.
- **H1 — headline suite (only after H0 passes):** 10-30 headroom theorems × 4-8 seeds; the
  required-baseline set; solve-rate / cost-per-solved / time-to-first-proof; held-out split decides.

## F. Minimal H0 arm matrix (no 90-agent runs)

`A0, B2 (parallel), B4 (majority), B5 (best-first), B6 (skeptic-rerank), M0 (informed-Bear),
M1 (random-Bear), M2 (fixed-Bear)`. Diagnostic reads:
- M0 beats A0 but loses to **B6** → the skeptic is strong, the market is weak.
- M0 == **M1** (random-Bear) → it was just price noise, not informed pricing.
- M0 == **M2** (fixed-Bear) → a constant counter-stake suffices; the `p_fail` signal adds nothing.
- M0 beats A0/B2/B4/B5/**B6/M1/M2** → real market evidence.

## G. Decision rule v2 (REPLACES v1 §7 — binding)

```json
{
  "decision_rule": "hard-lean-market-gonogo/v2",
  "registered": "2026-05-30",
  "level": "H0 pilot gates entry to H1; only H1 is a headline",
  "GO_requires_all": [
    "M0 resolve-rate >= A0 + 20pp  OR  cost_adjusted_solved >= 1.5x",
    "M0 strictly beats B2_parallel (else just more sampling)",
    "M0 strictly beats B5_best_first (else just a search heuristic)",
    "M0 strictly beats B6_skeptic_rerank (else just a critic heuristic)",
    "M0 strictly beats M1_random_Bear (else just price noise)",
    "M0 strictly beats A0_shuffled and B3_no_price (paired p<0.05, Holm-Bonferroni)",
    "100% counted cells replay-green; every OMEGA a real Lean Verified (no sorry)",
    "Bear token/cost counted in MARKET budget AND baselines got budget-equal-or-critic-matched resource",
    "price correlates with final verifier outcome / useful routing (Bear AUC and price->accept corr reported, non-trivial)"
  ],
  "WEAK_GO_if": "resolve not significantly higher, BUT time-to-first-proof OR cost-per-solved clearly beats baselines, OR failed-branch reuse + price-routing efficiency clearly beat no-price -> 'cheaper / faster / more auditable solver substrate, not a stronger solver'; triggers the Thesis-B cross-model fast-follow",
  "NO_GO_if_any": [
    "M0 beats A0 but NOT B2_parallel or B5_best_first",
    "M0 wins only because it spent more Bear tokens (budget not equalized)",
    "B6_skeptic_rerank ties or beats M0",
    "M1_random_Bear ties M0 (informed pricing adds nothing)",
    "price uncorrelated with Lean accept",
    "replay incomplete on any counted cell",
    "single-theorem win that does not replicate across the H1 multi-theorem suite"
  ]
}
```

## H. Ordered execution (architect)

1. **Freeze Bear mechanism** — this document. ✔
2. **Budget accounting for Bear calls** — Bull+Bear tokens/cost/latency into the MARKET manifest;
   baselines budget-equalized; B6 critic-matched.
3. **Add B6 skeptic-rerank + M1 random-Bear + M2 fixed-Bear** ablation arms.
4. **Curate headroom theorems** (criteria below).
5. **Run H0** (1-3 theorems × 8 seeds × the arm set), replay-green only.
6. **If H0 passes → H1** multi-theorem headline suite.

## I. Headroom theorem selection (LOCKED criteria — "the hard that separates mechanisms," not "the hardest")

Target: single-agent pass@1 ∈ **10-40%**; pass@k not so high that parallel sampling alone solves it;
medium-to-long proof; **multiple lemma paths (branching)**; Lean produces *usable* errors (not pure
timeout); search topology has local optima / decoy tactics; **avoid** common MiniF2F / textbook /
famous-informal-proof items (contamination); Lean-kernel deterministic accept/reject.
**Exclude:** single-agent-always-passes; all-agents-fail; one-line-`simp`; huge-Mathlib-gap;
famous theorems likely memorized. The goal is **branching proof search**, not mathematical flourish.

## J. Framing (external)

Never: "we added an LLM judge to grade proofs." Always: *"wallet-limited informed Bear participants
express downside risk via integer Short positions; settlement remains exclusively Lean-kernel-based."*
The deliverable is a **prover-amplification substrate** judged by **uplift over budget-matched
baselines**, not a raw proof rate.
