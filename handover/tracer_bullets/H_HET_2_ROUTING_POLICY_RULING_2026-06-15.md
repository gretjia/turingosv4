# H-HET-2 Routing-Policy Ruling — `VERIFY_UCB_PRICE_PRIOR_FLOOR_V1` (architect, 2026-06-15)

**Verdict: APPROVED AS AMENDED.** This is the authoritative implementation contract
for the H-HET-2 dynamic model-budget router. Supersedes any conflicting framing in
the design-exploration doc (`H_HET_2_ROUTING_POLICY_DESIGN_2026-06-15.md`).

## Approved primary policy family

```
VERIFY_UCB_PRICE_PRIOR_FLOOR_V1
= outcome-driven deterministic UCB (reward = per-(model,target) Lean verify)
+ bounded target-local price prior for COLD-START ONLY
+ mandatory ε exploration floor
+ deterministic isqrt-only count bonus (no float/log table)
+ NO stochastic RNG in v1
+ full BudgetAllocationTelemetry to ChainTape/CAS
+ replay assertion: allocation_view == derive_from_tape(tape)
```

Rejected as primary: `PRICE_PROP`, `priced-softmax-reuse`, Thompson sampling.
`priced-softmax-reuse` allowed ONLY as a preregistered diagnostic ablation (not the
primary treatment, separate prereg).

## Amendment 1 — evidence wording correction (claim-boundary hygiene)

Do NOT write "H-HET-1 already showed price aggregation misroutes." Correct framing:

> H-HET-1 proves that fixed equal-split (round-robin) budget STARVES the uniquely
> capable model. It does NOT directly prove that price aggregation misroutes
> (H-HET-1 never ran a price-based allocator). Aggregate price is a WRONG-GRANULARITY
> PROXY for the H-HET-2 primary predicate (per-(model,target) union coverage), so
> price may inform cold-start but must not dominate the routing signal.

## Amendment 2 — honest policy name (§17.3)

Forbidden names: `hayek_softmax` / `priced_softmax` / `market_softmax`. This is NOT a
softmax distributor; it is a floored deterministic UCB. Approved name:
`verify_ucb_price_prior_floor_v1` / `VERIFY_UCB_PRICE_PRIOR_FLOOR_V1`.

## Amendment 3 — price is a target-local cold-start prior only

```
price_prior(m, T) = max price_yes of nodes on target T authored by model m
                    OR neutral prior if m has no target-local nodes yet
price_prior applies ONLY while n_pull(m,T) < N_cold AND verify_count(m,T) == 0.
After first verify OR after N_cold pulls, price weight decays to zero.
```
No global model-aggregate price anywhere. (Default `N_cold = 4`.)

## Amendment 4 — router policy is a top-level allocator, NOT in the proposer prompt (Art III.4 Goodhart shield)

UCB score, bonus, hard-failure threshold, decommission rule, price/verify weights
MUST NOT be broadcast to the proposer agent. They are written to tape (audit-visible)
but the proposer prompt still sees only allowed market signals / abstracted failures /
target context. The allocation metric must never become a proposer optimization target.

## Approved default parameters (prereg-draft starting point)

| param | value |
|---|---|
| W_VERIFY : W_PRICE | 8 : 1 |
| price_component cap | 1250 bps equivalent |
| N_cold | 4 pulls OR first verify, whichever first |
| C_UCB (count bonus) | 2500 bps |
| count bonus | `bonus(m,T) = C_UCB * isqrt_fixedpoint((N_T+1)/(n_mT+1))` — integer/isqrt, no log |
| ε_model (floor) | `min(0.10, 0.40 / |eligible_models|)` |
| N_hard_fail | 3 consecutive tape-recorded hard failures |
| cross-target transfer | NONE in primary; state resets per (target × seed) |
| budget fairness | per-target AND global, across tokens + microUSD + proposal-call cap + router overhead |
| tie-break | deterministic: lexicographic(model_id) or roster_order_hash; policy-hash pinned |

Score skeleton (all integer bps):
```
vr_bps(m,T)    = 10000 * (verify_count(m,T) + 1) / (pull_count(m,T) + 2)   // Beta(1,1) neutral prior
price_bps(m,T) = clamp(target_local_price_prior(m,T), 2500, 7500)
                 if pull_count(m,T) < N_cold AND verify_count(m,T) == 0 else 0
score(m,T)     = W_VERIFY*vr_bps + W_PRICE*price_bps + bonus_bps(m,T)
```

Decommission vs floor:
```
- A model exits the GUARANTEED ε-floor on a target only after N=3 consecutive
  tape-recorded hard failures under comparable context.
- Decommission-from-floor ≠ ban: it stays eligible for exploitation if its UCB
  score later wins.
- If B_target < |eligible_models| * N_hard_fail: NO model may be decommissioned
  from the floor on that target (small budgets distribute, not eliminate).
```
Hard-failure class INCLUDES: Lean-rejected / SorryBlocked / axiom-dirty / comparable
proof-search failure (each tape-recorded). EXCLUDES: provider_error, timeout,
rate_limit, parse_fallback, tool_infrastructure_failure, schema/replay failure.

## BudgetAllocationTelemetry — full field set (this round's additions folded in)

```rust
policy_family: "VERIFY_UCB_PRICE_PRIOR_FLOOR_V1"
policy_hash; policy_version
target_id; seed_id
eligible_model_set_hash
input_state_cid
price_vector_cid
abstracted_failure_features_cid
model_id
pull_count_model_target_before
verify_count_model_target_before
hard_failure_streak_before
total_pulls_target_before
vr_bps; price_bps; bonus_bps; score_bps
exploration_floor_state
floor_quota_remaining_before; floor_quota_remaining_after
selected_model_id
selection_reason          // FLOOR | UCB_SCORE | COLD_START | TIEBREAK
allocated_proposal_budget; allocated_token_budget
budget_remaining_before; budget_remaining_after
router_overhead_cid
rng_seed: None; rng_draw: None   // forward-compat; v1 is deterministic
```
Replay assertions:
```
derive_from_tape(tape).allocation_view    == recorded_allocation_view
derive_from_tape(tape).score_components   == recorded_score_components
derive_from_tape(tape).budget_remaining   == recorded_budget_remaining
```

## RoutingPolicyGenesisPin (NEW — does not ride §8; own Path-B declaration)

```rust
RoutingPolicyGenesisPin {
  policy_family: "VERIFY_UCB_PRICE_PRIOR_FLOOR_V1",
  policy_version, policy_hash,
  canonical_policy_config_cid,
  eligible_model_set_hash,
  target_pool_hash,
  budget_caps_hash,
  rng_mode: "deterministic_none",
  art_0_4_path: "B",
}
```
The `BudgetAllocationTelemetry` schema + this pin are new tape/CAS schema → gate-first,
real evidence, Veto-AI, repeating the Art 0.4 Path-B declaration (§8's Path-B does NOT
auto-cover this new routing schema).

## Paid-run gate (unchanged, restated)

Paid run BLOCKED until: BudgetAllocationTelemetry tape-canonical + Path-B + Veto-AI
PASS; RoutingPolicyGenesisPin sha-pinned; BestHOMO + target pool + budget caps +
primary metric + exploration floor + scientific-audit plan frozen; architect sign-off.

## Architect verbatim ruling block (for OBLIGATIONS / commit)

```
ARCHITECT ROUTING-POLICY FAMILY RULING — H-HET-2
Verdict: APPROVED AS AMENDED.
Primary policy family: VERIFY_UCB_PRICE_PRIOR_FLOOR_V1
- Lean verify outcome = primary per-(model,target) reward;
- bounded target-local price prior for cold-start only;
- mandatory ε exploration floor; deterministic isqrt-only count bonus;
- no stochastic RNG in v1; full BudgetAllocationTelemetry to ChainTape/CAS;
- replay assertion allocation_view == derive_from_tape(tape).
Defaults: W_VERIFY:W_PRICE=8:1; price cap 1250 bps; N_cold=4; C_UCB=2500 bps;
ε_model=min(0.10,0.40/|eligible|); N_hard_fail=3; per target×seed reset, no
cross-target transfer in primary; budget fairness per-target and global across
tokens/microUSD/proposal-calls/router overhead; tie-break deterministic + hash-pinned.
Wording: H-HET-1 did NOT prove price aggregation misroutes; it proved fixed
round-robin starves the uniquely capable model. Aggregate price = wrong-granularity
proxy risk, not a falsified mechanism.
Rejected as primary: PRICE_PROP, priced-softmax-reuse, Thompson.
Allowed secondary: priced-softmax-reuse as a preregistered diagnostic ablation only.
Paid run BLOCKED until BudgetAllocationTelemetry tape-canonical + Path-B + Veto-AI
PASS; RoutingPolicyGenesisPin sha-pinned; BestHOMO/target pool/budget caps/primary
metric/exploration floor/audit plan frozen; architect sign-off.
```
