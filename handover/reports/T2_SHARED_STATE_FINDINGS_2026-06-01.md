# T2 Shared-State Price-Routing — Findings (2026-06-01)

> **Status: SCAFFOLD — methodology + audit captured; 12-seed counted results filled on sweep completion.**
> Mission proposition (architect, compressed): *given identical models / budget / verifier / task, does a
> PRICE-routed allocation of a scarce reasoner-repair budget bank more verified Lean theorems than a central
> coordinator — and is the win causal vs price-destruction ablations?*

## 1. What this measures (and the confound it removes)

The TuringOS thesis under test (Hayek core): a **decentralized price** that aggregates bettors' beliefs about
repair-success allocates a scarce compute budget **better than central planning** (a coordinator LLM) or **no
price** (shuffled / flatbid / random). The cleanest substrate: a pool of Lean theorems, a cheap proposer that
**free-banks** the easy ones, leaving a **residual** of failed attempts; a fixed reasoner-repair budget **B**;
six allocation policies racing to bank the most repairs within B.

**The confound we removed (the load-bearing fix).** The earlier per-arm `run_alloc` re-ran the *stochastic*
free-bank for **every arm**, so each arm faced a **different residual set** (free-bank luck ±7 theorems) that
swamped the thin routing signal (~3–4 repairable). The constitution's price-coordination could never show
through that noise. `run_alloc_shared` computes the free-bank + betting + coordinator-rank + per-residual
repair **once per seed**; the six arms are **deterministic allocation policies over the IDENTICAL state**
(residuals, prices, repair-success, repair-cost) — only the **order** they spend B differs. So `banked@B`
isolates **pure routing**: any arm difference is the allocation decision, nothing else.

## 2. Pre-registration + audit (locked before reading counted results)

- **Prereg (v2, SHA-pinned):** `handover/preregistration/T2_SHARED_STATE_PREREG_2026-06-01.json`
  (sha256 `0b44dddf…`). Proposer/bettor `deepseek-v4-flash`; reasoner `qwen3.7-max`; pool subset 24; **B = 1000**
  reasoner-completion-tokens; arms `market / coordinator / shuffled / flatbid / random / index`; **seeds 8–19**
  counted (seed 7 = pilot). v1 `T2_COUNTED_SWEEP_PREREG` retained unmutated as the abandoned confounded design.
- **G2 exit gate met (real run + byte-equal replay, 真题真跑):** seed-7 smoke, all 6 arms
  `verify_market_tape replay_clean=true`, with banked / cost / cost_of_pass / llm_calls / tokens **byte-equal**
  derived-vs-manifest (e.g. market: micro_usd 11969==11969, llm_calls 181==181, tokens 14205==14205).
- **Clean-context audit — PROCEED** (independent witness, no implementation transcript). Independently re-ran
  the verifier on the market arm AND a foil (both byte-exact) and reconstructed per-residual cost/repair-ok +
  per-arm afforded sets directly from the tapes. Verified: (1) shared-state integrity (free-bank/betting/coord/
  repair computed once, identical across all 6 arms); (2) `banked@B` pure-routing via a **symmetric** budget gate
  (`spent + rep_cost > B → skip`, identical across arms); (3) replay/cost byte-exact + integer-only; (4) no
  market-favoring bias (shuffled = permuted price, flatbid = constant stake, coordinator sees the same
  failed-body+error info the bettors price); (5) determinism (single seeded RNG consumed only by shuffled→random
  in fixed order); (6) verdict honesty (NO-GO requires a foil delta **strictly < 0**; positive-but-nonsignificant
  = INCONCLUSIVE). Three **non-blocking** findings (analyzer v1 defaults — **fixed**; cosmetic Resolve-index
  label; chat/reasoner token-label bundling — neither affects `banked@B` or replay).

## 3. Metric

`banked@B` = axiom-clean theorems banked at the fixed reasoner-token budget (pass-rate at equal compute;
least gameable). Complementary **routing-capture** = `(banked@B − free) / repairable` normalizes for per-seed
routing *room* (free + repairable are shared per-seed constants; only the captured numerator varies by order).

## 4. Counted results (seeds 8–19) — [FILL ON COMPLETION]

Command:
```
python3 scripts/analyze_t2_sweep.py --dir handover/evidence/t2_shared_sweep_2026-06-01 \
  --prefix t2s --seeds 8,9,10,11,12,13,14,15,16,17,18,19 --arms market,coordinator,shuffled,flatbid,random,index
python3 scripts/analyze_t2_routing_efficiency.py --dir handover/evidence/t2_shared_sweep_2026-06-01 --prefix t2s
```

| arm | mean banked@B | mean routing-capture | replay all-green |
|---|---|---|---|
| market | [FILL] | [FILL] | [FILL] |
| coordinator | [FILL] | [FILL] | [FILL] |
| shuffled | [FILL] | [FILL] | [FILL] |
| flatbid | [FILL] | [FILL] | [FILL] |
| random | [FILL] | [FILL] | [FILL] |
| index | [FILL] | [FILL] | [FILL] |

Verdict A causal gates (paired Wilcoxon, Holm @ α=0.05): market vs coordinator [FILL]; vs shuffled (PRIMARY)
[FILL]; vs flatbid [FILL]. **n = [FILL] paired seeds. Per-seed routing room (repairable): [FILL].**

## 5. Two-level verdict — [FILL ON COMPLETION]

- **Verdict A (price-causal efficiency):** [GO / INCONCLUSIVE / NO-GO]. [FILL: market > coordinator AND
  shuffled (PRIMARY) AND flatbid, all Holm-p<α, replay-green?]
- **Verdict B (institutional governance, the floor):** market replay-GREEN [FILL] ∧ Sybil-resistant (TP-3
  frozen-competence design, `reputation_constitutional.rs` substrate) ∧ Goodhart-shielded (TP-1 16-site
  PredicateId-leak falsifier) ∧ failures-on-tape. A is **never** inferred from B.

## 6. If signal is weak (role-correction frame) — [conditional]

Per the architect: a weak market signal is a prompt to **improve the code's fidelity to the constitution**, not
to declare price-doesn't-help. The first lever is **betting informativeness** — make the price aggregate
repair-success likelihood (the constitution requires an *informative* price). The routing-capture diagnostic
(§3) localizes this: if `market ≈ shuffled/flatbid/random` on capture, the **betting** is the lever, not the
routing mechanism. [FILL: which case obtained; next atom.]
