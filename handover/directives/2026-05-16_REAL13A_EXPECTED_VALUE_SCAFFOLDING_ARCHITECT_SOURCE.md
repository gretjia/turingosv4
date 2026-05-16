# REAL-13A Expected-Value Scaffolding Architect Source

This file preserves the local source facts that authorize drafting REAL-13A.
It is not an implementation plan and does not authorize code changes by itself.

## Source 1 — REAL-12 Architect Decision Gate

From `handover/directives/2026-05-16_REAL12_ROLE_SPECIALIZED_ECONOMIC_AGENTS_ARCHITECT_ORIGINAL.md`:

```text
Atom 6 — Decision gate

如果出现 live non-scripted buy/short

E2 achieved.
进入 REAL-13：role differentiation / E3。

如果无 live buy，但 EconomicJudgment 显示 no perceived positive EV

说明模型没有足够交易判断能力或市场 event 没有 edge。
进入 REAL-13A：Trader objective / expected value scaffolding。

如果无 actionable market

进入 REAL-13B：Event Timing Redesign / live REAL-6B ratification。

如果 positive-control 失败

回 substrate fix。
```

From `handover/directives/2026-05-16_REAL12_ROLE_SPECIALIZED_ECONOMIC_AGENTS_EXECUTION_PLAN.md`:

```text
## Atom 6 — Decision Gate

- live non-scripted buy/short -> E2 candidate; proceed REAL-13 E3 study.
- actionable markets but no live action -> REAL-13A expected-value scaffolding.
- no actionable market -> REAL-13B event timing / live REAL-6B Class-4 packet.
- positive-control fail -> substrate fix.
```

## Source 2 — REAL-12 Canonical Evidence

From `handover/reports/REAL12_TASK_MARKET_PROBE_REPORT.md`:

Evidence-local copy:

```text
handover/evidence/real12_role_specialized_micro_probe_20260516T023351Z/REAL12_TASK_MARKET_PROBE_REPORT.md
```

```text
# REAL-12 Task-Market Action Probe Report

run_tag: `real12_role_specialized_micro_probe_20260516T023351Z`
runtime_repo: `/home/zephryj/projects/turingosv4-real12-action-probes/handover/evidence/real12_role_specialized_micro_probe_20260516T023351Z/runtime_repo`
CAS path: `/home/zephryj/projects/turingosv4-real12-action-probes/handover/evidence/real12_role_specialized_micro_probe_20260516T023351Z/cas`
audit_tape verdict: `PROCEED`
```

```text
No forced trade
No price-as-truth
No scripted buys
scripted_positive_control_is_not_e2=true
live_real6b_enabled=false
attempt_prediction_fixture_count=0
TURINGOS_REAL12_TASK_MARKET_AFFORDANCE=1
TURINGOS_REAL12_TRADER_OBJECTIVE=0
No ghost liquidity
No f64/f32 money path
```

```text
| MarketOpportunityTrace count | 4 |
| market_seed | 5 |
| cpmm_pool | 5 |
| event_resolve | 2 |
| bid_task_attempted | 0 |
| invest_attempted | 0 |
| invest_submitted | 0 |
| buy_with_coin_router | 0 |
| buy_yes_router_count | 0 |
| buy_no_router_count | 0 |
| agent_economic_action_tx_count | 0 |
| live_non_scripted_router_tx_count | 0 |
| economic_judgment_total | 4 |
| bull_judgment_count | 2 |
| bear_judgment_count | 2 |
| abstain_structured_reason_count | 4 |
| economic_judgment_coverage_ok | true |
| economic_judgment_required_trader_turns | 4 |
| economic_judgment_linked_trader_turns | 4 |
| no_trade_no_perceived_edge | 10 |
| no_trade_zero_amount | 0 |
| no_trade_no_pool | 0 |
| no_trade_amount_exceeds_balance | 0 |
```

```json
{
  "NoPerceivedEdge": 4
}
```

```text
`E2 NOT ACHIEVED`
```

```text
This probe tests whether the advertised task-level market action affordance
causes live agents to emit `bid_task` or `invest`. It does not force trades,
does not enable live REAL-6B, and does not allow price to affect Lean predicates.

Scripted actions cannot satisfy E2. A live non-scripted router tx requires
ChainTape/CAS evidence, PromptCapsule/trace provenance, and audit_tape PROCEED.
This report derives EconomicJudgment counts from CAS schema
`real12.economic_judgment.v1` and Bull/Bear turn coverage from
`real5.role_turn_trace.v1`; stdout tool_dist is diagnostic only.
```

## Source 3 — Current Dynamic Handover

From `handover/ai-direct/LATEST.md`:

```text
Recommended branch is REAL-13A expected-value scaffolding: make
Bull/Bear probability and EV reasoning more explicit, still without forced
trade and still with price as signal only.
```

```text
Defer live REAL-6B unless future evidence shows no actionable market window
after role-specialized EV scaffolding.
```

## Source 4 — Claim Boundary Carried Forward

REAL-13A must preserve these non-claims from REAL-12:

```text
E2 is not achieved: no live non-scripted agent-generated router/short action.
E3 is not achieved: no persistent behavioral role differentiation claim.
E4 is not achieved: no causal performance signal claim.
No live REAL-6B approval.
No forced trade, price-as-truth, ghost liquidity, f64/f32 money path,
off-tape WAL truth, private CoT recording, raw-log broadcast, model ranking,
autonomous secondary market, or real-world readiness claim.
```

## Source-Derived REAL-13A Interpretation

REAL-13A is authorized only as a planning target for:

```text
Trader objective / expected value scaffolding
```

because the REAL-12 branch condition is:

```text
actionable markets but no live action
```

and the observed structured reason distribution is:

```text
NoPerceivedEdge: 4
```

REAL-13A is not:

```text
live REAL-6B
forced trade
larger A/B benchmark
E2/E3/E4 ratification
typed transaction schema work
sequencer admission work
```
