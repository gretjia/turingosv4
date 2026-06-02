# P1 Real-Value v4 — CORRECTED Findings (2026-06-02)

> The post-VETO corrective primary experiment. Binary `cb89a5d6`; prereg
> `P1_REALVALUE_V4_CORRECTED_PREREG_2026-06-02.json` (SHA-locked `d80ba8cd`). 732 cells, all
> `verify_chaintape` replay-clean; every crack axiom-clean via the inline `#print axioms` gate.
> Supersedes the v3 / scaleup result (`P1_CONFOUNDED_SUPERSEDED_2026-06-02.md`).

## 0. The honest headline

On 13 Lean theorems where a single chain **robustly fails (single 0/6 seeds = 0/78 cells)**, at equal
budget (24 proposals/cell), real Lean kernel + inline axiom gate:

**The lever is NON-LOCAL TREE SEARCH (restart/branch from any earlier node) — NOT the loss-bearing
price, NOT the agent count, NOT free-choice routing.** A single agent that may revisit any of its own
earlier attempts (`single_tree_no_price`, 5.1%, 329K tokens/solve) cracks **more** hard theorems, **more
cheaply**, than the price market (2.6%, 1.13M/solve), free-choice autonomous (1.3%, 3.09M/solve), and 4
independent linear chains (`parallel`, 2.6%). **Adding the real price signal DECREASES the solve rate
(−3.8pp vs shuffled price) — the single biggest negative delta in the decomposition.**

**Neither pre-registration confirms. Nothing is statistically significant** (all McNemar `p_holm = 1.0`;
cracks are sparse). The finding is a **consistent directional pattern + a clean efficiency ordering**, not
a powered significant claim.

### Solve rate on the 78 hard cells (the reliability layer)

| arm | mechanism | solved | rate | Wilson 95% | tok/solve |
|---|---|---|---|---|---|
| `single` | one linear chain | 0/78 | 0.0% | [0, 4.7%] | — |
| `single_restart` | 1 agent, root-restart or last | 2/78 | 2.6% | [0.7, 8.9%] | 584K |
| **`single_tree_no_price`** | **1 agent, restart from ANY own node, no price** | **4/78** | **5.1%** | [2.0, 12.5%] | **329K (best)** |
| `parallel` | 4 independent linear chains | 2/78 | 2.6% | [0.7, 8.9%] | 659K |
| `parallel_restart` | 4 indep chains + root-restart | 2/78 | 2.6% | [0.7, 8.9%] | 546K |
| **`no_price`** | 4 agents, shared tree, RANDOM restart | **6/78** | **7.7%** | [3.6, 15.8%] | 344K |
| `shuffled_price` | 4 agents, shared tree, PERMUTED-price softmax | 5/78 | 6.4% | [2.8, 14.1%] | 431K |
| `market` | 4 agents, shared tree, REAL-price softmax (Path-2) | 2/78 | 2.6% | [0.7, 8.9%] | 1.13M |
| `autonomous` | 4 agents, shared tree, LLM free-choice (Path-1) | 1/78 | 1.3% | [0.2, 6.9%] | 3.09M (worst) |

## 1. Why v4 (the v3 result was a DOUBLE artifact)

The v3 headline ("`autonomous` cracked hard theorems `single`+`market` could not") rested on two flaws,
both found and fixed here:

1. **Confound B (external auditors, fatal):** `autonomous`'s proof prompt fed the *full search landscape*
   (top-6 full bodies + errors) while `market` saw one parent — the "crack" could be 6× richer context,
   not free-choice routing. **Fixed (F1):** `autonomous` is now two calls — Stage-1 routes on a compact
   summary (no bodies/errors), Stage-2 builds the proof via the *byte-identical* `build_prompt` market
   uses (enforced by a self-test: `sha(stage2)==sha(market)`).
2. **Unreliable hard floor (deeper, found in v4 validation):** `single` gets 24 attempts/cell; the v4
   floor shows **`single` SOLVES `lm_ineq1`** (the v3 flagship "crack") axiom-clean, the *same* AM-GM
   route v3 credited to `autonomous`. v3's "single 0/3" was a ~5% small-sample fluke. **Fixed:** the hard
   floor is re-established here at `single` 0/**6**; `lm_ineq1` (1/6), `lm_det_zero` (6/6) and 3 others
   are correctly **excluded**. 13 of 18 candidates are robustly hard.

Plus the auditor-mandated controls (`parallel`, `shuffled_price`, `no_price`) — **never run in v3** — are
run here, and the inline `#print axioms` gate (F5) makes every "Verified" axiom-clean by construction.

## 2. The three layers

**LAYER 1 — Existence (PASS).** 24 axiom-clean cracks across the multi-arm policies on single-0 theorems
(`lm_ineq2`, `lm_ineq3`, `lm_natdeg_pow`, `lm_nt_gcd2`, `lm_f`, `lm_fact`, `lm_deriv1`). So **organization
of *some* kind beats a single chain** — real, replay-clean, kernel-verified, axiom-clean. (6 of 13 hard
theorems — `lm_coeff_mul`, `lm_c`, `lm_e`, `lm_lim1`, `lm_nt_cop_cubic`, `lm_probe1` — are cracked by
**nobody**: genuinely hard for this model at this budget.)

**LAYER 2 — Reliability (table above).** The ordering is consistent: non-local-tree arms (5–8%) >
linear-chain arms (0–2.6%); price/free-choice routing (1.3–2.6%) does **not** beat random/heuristic
restart. CIs overlap (sparse data) — directional, not significant.

**LAYER 3 — Causal / pre-registered (both NOT confirmed).**
- **PREREG_1 (market-Hayek): NOT CONFIRMED.** `market` CONFIRMED_WIN = 1 theorem (`lm_fact`), but `market`
  **loses** to `shuffled_price` (2 vs 5) and `no_price` (2 vs 6). `p_holm = 1.0`. The loss-bearing
  price-routing claim is **not supported** — real price does worse than destroyed/random price.
- **PREREG_2 (autonomous-freechoice): NOT CONFIRMED.** `autonomous` uniquely cracks `lm_deriv1` (real,
  axiom-clean `[Classical.choice, Quot.sound, propext]`) — 1 theorem, `p_holm = 1.0`, and `autonomous` is
  the **lowest-productivity, highest-cost** arm. Genuine free choice (0% route hallucination) that doesn't
  help.

### The decomposition (what the topology contrasts isolate)

```
single → single_restart            +2.6pp   root-restart helps a little
single_restart → single_tree_no_price +2.6pp branching from ANY own node helps more
single_tree_no_price → parallel    −2.6pp   1 agent w/ non-local tree BEATS 4 independent linear chains
parallel_restart → no_price        +5.1pp   a SHARED non-local tree is the big jump
shuffled_price → market            −3.8pp   adding the REAL price signal HURTS (biggest negative)
market → autonomous                −1.3pp   free-choice doesn't help either
```

**Read:** non-locality (revisit/branch from earlier nodes) is the lever; sharing the tree across agents
amplifies it; **price-routing and free-choice routing of that non-locality add nothing and point-wise
subtract** — they over-concentrate the search and lose the diversity that random/shuffled restart keeps.

## 3. What it means

- **The constitution's specific value claim — that a *loss-bearing price* routes attention better — is
  NOT supported in this regime.** Random restart (`no_price`) and permuted-price restart (`shuffled_price`)
  crack *more* hard theorems than real-price restart (`market`), at a fraction of the cost.
- **The architect's "non-local restart from early nodes" intuition IS vindicated** — it is the actual
  lever (`single` 0% → `single_tree_no_price` 5.1%). But it needs neither the price nor the swarm: a
  *single* agent revisiting its own tree is the cheapest, near-best policy.
- **Cost compounds the verdict:** the price/free-choice arms are 3–9× more expensive per solve (`market`
  1.13M, `autonomous` 3.09M vs `single_tree_no_price` 329K) — the price machinery (bear shorts) and the
  autonomous route call add tokens without adding solves.

## 4. Honest limitations + scope

- **Not significant.** Cracks are sparse (max 6/78); every paired test is `p_holm = 1.0`. The directional
  pattern is consistent and mechanistically sensible but **underpowered**. To power the *positive*
  non-locality finding, add seeds on the crackable band (`lm_ineq2`, `lm_ineq3`, `lm_natdeg_pow`).
- **Regime-scoped.** This tests price-**routing** of a homogeneous-agent, abundant-budget proof-tree
  search. It does **not** refute price-**allocation of a scarce shared budget across complementary
  specialists** (a different mechanism, tested in LEAN-ALLOC / PROBE-ALLOC). The NO-GO is for *price as a
  router in this regime*, not for every market claim.
- **Strong model.** `deepseek-v4-pro` solves many lm_ theorems alone; 6/13 hard ones are beyond every arm.
  A genuinely harder theorem set might widen the dynamic range (a separate effort).

## 5. Discipline (§17)

FC1/FC2/FC3 untouched; no §6 surface; integer money; PR-only; prereg SHA-locked before any counted cell;
all 732 cells `verify_chaintape` replay-clean; cracks axiom-clean by the inline gate; hard floor
re-established in-run (not inherited); both routing algorithms decoupled to a byte-identical proof prompt
(self-test enforced); the headline is a scoped, non-causal NO-GO + a directional (non-significant) lever
finding — no PROVEN/causal claim. A fresh clean-context audit (§17 G5) runs on this data before any PR.
