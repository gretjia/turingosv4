# REAL-12 Agent Economic Action Activation Probe

Date: 2026-05-15 UTC

Branch/worktree:

```text
branch: codex/real12-economic-action-probes
worktree: /home/zephryj/projects/turingosv4-real12-action-probes
base: main ac4f9367 REAL-11 agent economic action activation
```

## Claim Boundary

REAL-12 is an experimental branch on top of completed REAL-11. It tests the
weak point exposed by REAL-10/REAL-11:

```text
market_tx_count > 0
buy_with_coin_router = 0
```

It does not claim E2. All true-problem probes below preserve:

```text
no forced trade
no price-as-truth
no scripted buys
live REAL-6B disabled
audit_tape PROCEED
```

## Code Findings

The first concrete bug was a schema/affordance mismatch:

```text
Prompt advertised bid_task.
REAL-5 role gateway treated bid_task as abstain/CAS-only.
Evaluator did not execute bid_task through the task-outcome router.
Role-turn trace did not classify bid_task as market decision.
Trader allowed_tools did not include bid_task.
```

REAL-12 repairs that mismatch:

```text
bid_task -> RoleAction::Invest
Trader bid_task -> BuyWithCoinRouterTx gateway
Solver bid_task -> PolicyViolation
Trader allowed_tools includes bid_task
role-turn traces classify bid_task as MarketDecision
```

The repair is not an emergence claim. It only means an agent that emits the
advertised `bid_task` schema can now reach the existing economic action path.

## True-Problem Probe Matrix

| Probe | Role order / prompt condition | audit | opportunity traces | bid_task | invest | router buys | no perceived edge | role policy violations | solve |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `real12_task_market_probe_20260515T182104Z` | default `Solver,Trader,...` | PROCEED | 0 | 0 | 0 | 0 | 0 | 0 | 3/3 |
| `real12_task_market_probe_trader_first_20260515T182954Z` | `Trader,Solver,...` before role-boundary patch | PROCEED | 3 | 0 | 0 | 0 | 3 | 3 | 3/3 |
| `real12_task_market_probe_trader_first_boundary_20260515T183307Z` | Trader first + role action boundary | PROCEED | 4 | 0 | 0 | 0 | 12 | 5 | 2/3 |
| `real12_task_market_probe_trader_objective_20260515T183722Z` | Trader first + ungated objective text | PROCEED | 5 | 0 | 0 | 0 | 17 | 8 | 2/3 |
| `real12_task_market_probe_trader_boundary_gated_20260515T184026Z` | Trader first + objective off | PROCEED | 4 | 0 | 0 | 0 | 12 | 5 | 2/3 |
| `real12_task_market_probe_trader_objective_gated_20260515T184205Z` | Trader first + objective on | PROCEED | 5 | 0 | 0 | 0 | 17 | 9 | 2/3 |

<!-- evidence:
real12_task_market_probe_20260515T182104Z:
MarketOpportunityTrace=0 market_seed=6 cpmm_pool=6 event_resolve=2 bid_task=0 invest=0 buy_with_coin_router=0 no_perceived_edge=0 role_policy_violation=0 audit=PROCEED solve=3/3
real12_task_market_probe_trader_first_20260515T182954Z:
MarketOpportunityTrace=3 market_seed=6 cpmm_pool=6 event_resolve=2 bid_task=0 invest=0 buy_with_coin_router=0 no_perceived_edge=3 role_policy_violation=3 audit=PROCEED
real12_task_market_probe_trader_first_boundary_20260515T183307Z:
MarketOpportunityTrace=4 market_seed=5 cpmm_pool=5 event_resolve=2 bid_task=0 invest=0 buy_with_coin_router=0 no_perceived_edge=12 role_policy_violation=5 audit=PROCEED
real12_task_market_probe_trader_objective_20260515T183722Z:
MarketOpportunityTrace=5 market_seed=5 cpmm_pool=5 event_resolve=3 bid_task=0 invest=0 buy_with_coin_router=0 no_perceived_edge=17 role_policy_violation=8 audit=PROCEED
real12_task_market_probe_trader_boundary_gated_20260515T184026Z:
MarketOpportunityTrace=4 market_seed=5 cpmm_pool=5 event_resolve=2 bid_task=0 invest=0 buy_with_coin_router=0 no_perceived_edge=12 role_policy_violation=5 audit=PROCEED solve=2/3 failed_branch_count=12
real12_task_market_probe_trader_objective_gated_20260515T184205Z:
MarketOpportunityTrace=5 market_seed=5 cpmm_pool=5 event_resolve=3 bid_task=0 invest=0 buy_with_coin_router=0 no_perceived_edge=17 role_policy_violation=9 audit=PROCEED solve=2/3 failed_branch_count=17
-->

## Interpretation

REAL-12 narrows the failure mode:

```text
Not merely: no TaskOutcomeMarket exists.
Not merely: Trader never has a market-visible turn.
Not merely: bid_task cannot route.

Now observed:
  Trader-first runs create MarketOpportunityTrace.
  The repaired bid_task path is available.
  Agents still emit 0 bid_task / 0 invest / 0 router buys.
  NoTradeReason is dominated by NoPerceivedEdge.
```

The opt-in `TURINGOS_REAL12_TRADER_OBJECTIVE=1` prompt experiment did not
improve economic action. It increased opportunity/no-trade traces and did not
produce any live router tx. It is therefore kept as an opt-in experiment, not
as default ship behavior.

## Current Best Diagnosis

The current bottleneck is not the market transaction substrate. It is agent
economic conviction:

```text
TaskOutcomeMarket exists.
Trader can see opportunity.
The legal bid_task route exists.
But the model does not perceive an exploitable edge at the visible task price.
```

This is consistent with the architect's broader diagnosis: structural market
activity is not the same as live agent economic action. The next lawful design
must create better information/uncertainty timing or stronger but still
non-forcing utility, rather than merely adding more prompt text.

## Next Recommendations

1. Do not claim E2 from REAL-12.
2. Keep `TURINGOS_REAL12_TRADER_OBJECTIVE` opt-in/off by default.
3. Treat the `bid_task` gateway repair as a valid substrate hardening candidate,
   but because `evaluator.rs` is Trust Root pinned, require Class-4 review
   before any main merge.
4. For emergence, prefer one of these next experiments:
   - harder tasks where Trader has more pre-resolution turns;
   - ChainTape-derived difficulty/solver-prior signal in TraderView;
   - separate Class-4 live REAL-6B packet if actionable windows remain too late;
   - multi-model trader-only micro-probe to test whether abstention is model-family specific.

## Verification So Far

```text
cargo test --test constitution_real12_task_market_action --no-fail-fast -- --test-threads=1
  result: 5 passed / 0 failed

cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
  result: passed

all six true-problem probes:
  audit_tape verdict: PROCEED
```

<!-- verification evidence:
targeted test final pass observed after opt-in objective gating: 5 passed / 0 failed.
Trust Root verify final pass after evaluator hash update to 02a1895617334110da1562d9daf58d9a1f6461791bbdbfb9ae884c74ca38b910.
-->
