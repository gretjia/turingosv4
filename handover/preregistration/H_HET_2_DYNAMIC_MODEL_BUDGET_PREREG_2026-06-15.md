# H-HET-2 Dynamic Model-Budget Market — Preregistration (DRAFT)

**Status: DRAFT — awaiting architect sign-off. NO paid confirmatory run is
authorized by this document.** This prereg freezes everything that can be frozen
from the completed mechanism (policy hash, parameters, metric definitions, budget
fairness, audit split, power, BestHOMO procedure, target-selection procedure +
leakage guard). The two items that genuinely cannot be frozen yet — the concrete
frozen target list (requires a disjoint *deep-theorem* calibration pass) and the
architect sign-off — are listed OPEN in §9.

Authority chain (SHA-anchored, not bare-dated):
- Controlling charter: `handover/tracer_bullets/TB_DYNAMIC_MODEL_BUDGET_MARKET_charter_2026-06-15.md` (APPROVED AS AMENDED v2).
- Controlling policy ruling: `handover/tracer_bullets/H_HET_2_ROUTING_POLICY_RULING_2026-06-15.md` (`VERIFY_UCB_PRICE_PRIOR_FLOOR_V1`, APPROVED AS AMENDED).
- §8 `model_id` tape-canonical: commit `51c1d602` (Veto-AI PASS).
- BudgetAllocationTelemetry + routing mechanism (gate 5.4): commits `faee8c68` (atom 1) + `f897fb1a` (atoms 2–4), branch `claude/het-carrier-freeze`.
- Carrier freeze: `f73163f4`. Experiment evidence (H-HET-1 pilot): `61df15ed`.

---

## 1. Hypothesis (frozen claim boundary)

**H-HET-2:** *Dynamic model-budget routing converts complementary coverage into
capability that the best single model (BestHOMO) cannot match at equal-or-lower
total cost.*

This is NOT "heterogeneity helps." The lever under test is **budget ROUTING**, not
the roster. Per the ruling's Amendment 1: H-HET-1 proved fixed round-robin STARVES
the uniquely-capable model; it did NOT prove price aggregation misroutes (no
price-allocator was ever run). Aggregate price is a wrong-granularity proxy for the
primary predicate (per-(model,target) union coverage), so price informs cold-start
only and must not dominate routing.

A SUPPORTED headline requires **Primary A positive under the preregistered paired
test AND ≥1 Primary-B existence witness** (§4). A single "≥1 theorem once" result is
OBSERVED-only and never triggers SUPPORTED/PROVEN. No PROVEN headline without §17
G1–G6.

## 2. Frozen policy (sha-pinned) — gate #7

The routing policy is `VERIFY_UCB_PRICE_PRIOR_FLOOR_V1`, implemented in
`src/runtime/routing_policy.rs` (`RoutingPolicyConfig::default()`), deterministic,
no RNG. **Frozen `policy_hash` (sha256 of canonical_encode of the default config):**

```
FROZEN_POLICY_HASH = 9fb0f612df2054049a3799869aafe6c401eb8c72c27a1e581d3ed901913f263a
policy_family      = VERIFY_UCB_PRICE_PRIOR_FLOOR_V1
policy_version     = turingosv4.ucb_budget.v1
rng_mode           = deterministic_none      (rng_seed=None, rng_draw=None)
art_0_4_path       = B
```

Frozen parameters (integer/bps; any change re-mints the hash → re-prereg required):

| param | frozen value |
|---|---|
| W_VERIFY : W_PRICE | 8 : 1 |
| price_component cap | 1250 bps |
| N_cold | 4 pulls OR first verify, whichever first |
| price clamp [lo, hi] | [2500, 7500] bps |
| C_UCB (count bonus) | 2500 bps |
| count bonus | `bonus(m,T) = C_UCB · isqrt_fixedpoint((N_T+1)/(n_mT+1))` — integer/isqrt, no log/float |
| vr_bps | `10000 · (verify+1)/(pull+2)` — Beta(1,1) neutral prior |
| ε_model (exploration floor) | `min(0.10, 0.40 / |eligible_models|)` — exact integer-rational |
| N_hard_fail | 3 consecutive tape-recorded hard failures |
| cross-target transfer | NONE in primary; state resets per (target × seed) |
| tie-break | deterministic Lexicographic(model_id); policy-hash pinned |

Hard-failure class (burns toward N_hard_fail) INCLUDES Lean-rejected / SorryBlocked /
axiom-dirty / comparable proof-search failure. EXCLUDES provider_error, timeout,
rate_limit, parse_fallback, tool/schema/replay failure (infra noise must not burn a
model's exploration budget). Decommission-from-floor ≠ ban (the model stays eligible
for exploitation if its UCB score later wins). If `B_target < |eligible| · N_hard_fail`,
NO model may be decommissioned from the floor on that target (small budgets distribute,
not eliminate).

## 3. Treatment & controls (frozen) — gate #3

- **TREATMENT** — dynamic model-budget market: `Policy::VerifyUcbPriceFloor` reallocates
  scarce proposal-call/token budget per the frozen policy (Art II.2 + II.2.1 + II.1).
- **CONTROL-1 — BestHOMO** (procedure frozen; concrete pick is calibration-derived):
  either (a) ALL eligible single-model homogeneous arms run at the same total budget,
  BestHOMO = max over them; OR (b) one BestHOMO model pre-selected on a DISJOINT frozen
  calibration set before any confirmatory seed. **Q397HOMO is mandatory** but NOT
  assumed globally best. The headline compares TREATMENT vs BestHOMO, never vs a
  fixed-assumed Q397HOMO.
- **CONTROL-2 — fixed round-robin heterogeneous** (the H-HET-1 carrier) at the same
  budget; isolates "dynamic routing" from "heterogeneous roster."

**Roster freeze:** eligible roster frozen BEFORE calibration and confirmatory seeds:
DeepSeek-V4-Pro, Qwen3-32B, GLM-4.5-Air, Qwen3.5-397B-A17B. No post-calibration
admission, removal, alias substitution, fallback model, or provider-side silent
replacement enters the primary metric; `eligible_model_set_hash` (on
`BudgetAllocationTelemetry` + `RoutingPolicyGenesisPin`) makes drift tape-detectable.

## 4. Primary claim & metrics (frozen) — gates #5, #6

**Primary unit = the seed×target paired cell.**

- **Primary A (confirmatory, paired):** `Δ_union = Σ I[TREAT solved ∧ BestHOMO unsolved]
  − Σ I[BestHOMO solved ∧ TREAT unsolved]` over seed×target cells. **Default test =
  exact paired sign / McNemar over discordant seed×target cells.** Wilcoxon ONLY for
  preregistered per-seed aggregate deltas or continuous/token-economic secondary
  metrics.
- **Primary B (existence witness):** ≥1 target where TREAT solves in ≥ **m/12 seeds,
  m = 6 (minimum; a later prereg may raise, never below 6)** AND BestHOMO solves in
  0/12 (or statistically fewer paired seeds), replay-clean + axiom-clean. Multiple
  independent Primary-B targets → preregistered multiplicity control (Holm).
- **SUPPORTED headline = Primary A positive (preregistered paired test) AND ≥1
  Primary-B witness**, unless this prereg explicitly downgrades Primary B to secondary.
- **Economic dominance:** equal budget = same-or-lower total input+output **tokens**
  AND same-or-lower **microUSD** (per-model rates differ) AND same-or-lower
  **proposal-call cap** AND **all router/model-selection LLM+tool overhead counted**
  (via `router_overhead_cid`). Capability metric may be token-pure; the
  economic-dominance claim requires BOTH token and microUSD non-inferiority.
- **Secondary:** serial (uncontended) PPUT (ΣPPUT, Mean-PPUT(solved)) — never
  concurrency-contaminated wall-clock as primary; per-model budget share vs per-model
  verify rate; golden-path tokens.

## 5. Target pool (procedure frozen; concrete list OPEN — gate #4)

Pre-select theorems where (a) no single model one-shots them, AND (b) the
budget-Goldilocks property holds (some model 0/K, another ≥1/K at the experiment's
per-agent budget). **Source pool = the DEEPER theorems in
`tests/fixtures/lean_theorems_pool.jsonl` + V3-class multi-step targets, EXCLUDING
the det-family one-liners** (the H-HET-1 Goldilocks band was det-family/shallow and
is excluded here).

**Leakage guard (binding):** target inclusion, difficulty labels, depth labels, and
BestHOMO selection derive ONLY from disjoint calibration seeds / historical data. The
confirmatory K≥12 seeds are NEVER used for inclusion, labeling, or BestHOMO selection;
any target whose inclusion depends on confirmatory outcomes is excluded from the
primary metric. **Depth proxy** = PRE-RUN reference-proof decomposition / fixture
metadata / frozen calibration solve-DAG — NEVER the post-hoc solved DAG.

**Why the concrete list is OPEN:** the H-HET-1 calibration (134-call sweep + 84-call
K=3 pilot + 45-cell carrier pilot) was all det-family/shallow, which this charter
excludes. A NEW *disjoint deep-theorem* calibration pass (tx/agent ≳ 20 per the V3
budget-regime prior) is required to (i) verify no single model one-shots each
candidate and (ii) confirm budget-Goldilocks, BEFORE the list can be frozen. That
calibration pass is itself part of the architect-gated paid prep.

## 6. Budget regime & power (frozen) — gate #6, charter §6

- **Budget:** tx/agent ≳ 20 (V3 budget-regime prior — a single-run observation, NOT a
  replicated scaling law: V3 got depth-5/NO at ~3.3 tx/agent, depth-18/OMEGA at ~19
  tx/agent). e.g. NA = 4..8, NR ≥ 20, or larger agent pools if rate limits allow.
  Budget must BIND (unsolved-at-budget is a real outcome, not a tooling artifact).
- **Power:** K ≥ 12 preregistered seeds; within-seed pairing (TREATMENT vs each control
  on the SAME seed/target). Stable across a later-day re-run (no single-day flip).

## 7. Required artifact — tape-reconstructed DAG (V3 deliverable)

Per solved target, emit FROM THE FROZEN TAPE (not the manifest sidecar) a V3-style
report: citation DAG (roots → golden path → branches), golden-path steps with author
agent + model (tape-canonical via §8) + the budget-allocation decision that funded
each step (via §5.4 `BudgetAllocationTelemetry`), role/model activity, price breakdown,
contested nodes. `assert view == derive_from_tape(tape)`. This is the direct visual
test of whether budget flowed to the winning model.

## 8. Audit split (frozen) — gate #8

Post-data review is TWO separate tracks:
- **Veto-AI** — constitutional PASS/VETO only (Art V.1.3; no quality/perf/coverage
  opinions).
- **Scientific/economic clean-context audit** — recomputes solve counts, paired
  statistics, PPUT/token/microUSD, DAG reconstruction, headline validity; verdict
  domain `{SUPPORTED | NOT-SUPPORTED | ANALYSIS-ERROR | RECONSTRUCTION-FAILURE}`.

## 9. §11 paid-run freeze-gate status

| # | gate | status |
|---|---|---|
| 1 | §8 `model_id` tape-canonical | **SATISFIED** (`51c1d602`, Veto-AI PASS) |
| 2 | `BudgetAllocationTelemetry` / router decision tape-canonical (§5.4) | **SATISFIED** (`faee8c68`+`f897fb1a`; gate-5.4 `constitution_budget_decision_tape_canonical` 5/5; Veto-AI PASS) |
| 3 | BestHOMO control defined (all-homo OR disjoint-calibration) | **FROZEN (procedure)** — §3 |
| 4 | Target pool frozen + leakage guard + depth proxy | **OPEN** — needs disjoint deep-theorem calibration pass (§5); procedure + guards frozen |
| 5 | Primary metric frozen (paired union delta + existence witness) | **FROZEN** — §4 |
| 6 | Budget fairness frozen (tokens + microUSD + proposal calls + router overhead) | **FROZEN** — §4 |
| 7 | Bandit/pricing policy sha-pinned; ε, N numeric; tape-reconstructible | **SATISFIED** — `policy_hash` §2 |
| 8 | Audit split (Veto-AI constitutional only; scientific audit separate) | **FROZEN** — §8 |

**Remaining true blockers before any paid confirmatory run:** gate #4 concrete
frozen target list (via a disjoint deep-theorem calibration pass) + **architect
sign-off**. Pre-existing-and-tracked (NOT an H-HET-2 blocker but must not be
normalized indefinitely before merge): the `.claude/hooks/judge.sh` stale trust-root
manifest hash (Class-4 inconsistency at HEAD, clean@HEAD from `92c6ffe6`).

## 10. Claim-integrity (§17) commitments

G1 recompute-from-tape (`derive_from_tape == manifest`, byte-equal) for solve counts,
score components, budget remaining, and the DAG; G2 real models + real Lean verifier;
G3 fair equal-budget controls (BestHOMO, round-robin het); G4 K ≥ 12 preregistered
seeds + paired stats + later-day re-run stability; G5 clean-context scientific audit
AFTER data lands, persisted under `handover/audits/` referencing run-manifest SHA(s);
G6 no pass-condition asserted against a compile-time literal. A missing box ⇒ the only
legal output is a scoped, non-causal statement.
