# REAL-12 Role-Specialized Economic Agents Implementation Report

Date: 2026-05-16

## Claim Boundary

REAL-12 implements role-specialized economic agents and mandatory economic
judgment traces for BullTrader / BearTrader turns.

This report **does claim**:

- BullTrader / BearTrader roles exist as role-specialized economic agents.
- BullTrader / BearTrader role policy is enforced by the role gateway.
- BullTrader / BearTrader views expose scoped market-side signals.
- EconomicJudgment records are CAS-backed and ChainTape-anchored through
  runtime evidence.
- Scripted Bull/Bear positive controls prove the router path remains usable.
- The live REAL-12 micro-probe produced economic judgments but no live
  non-scripted router action.

This report **does not claim**:

- E2 spontaneous market action.
- E3 persistent role differentiation.
- E4 causal performance improvement.
- live REAL-6B approval.
- forced trade as evidence.
- price-as-truth.

## Touched FC Nodes And Risk

Risk class: Class 4 package, because the package touches Trust Root-pinned
runtime/evaluator surfaces and role-action evidence authority.

FC mapping:

- FC1 role action loop: role-scoped output is parsed, typed, routed, and
  rejected through L4/L4.E instead of silently ignored.
- FC2 role assignment / PromptCapsule replay: role-derived views and prompt
  capsules preserve scoped read sets and model/task context.
- FC3 dashboard/audit materialized view: REAL-12 reports derive from
  ChainTape/CAS, not stdout as source of truth.
- Art. III shielding: raw prompt/completion/private CoT/raw logs remain out of
  public role views and EconomicJudgment payloads.

## Implementation Summary

- Added `BullTrader` and `BearTrader` to `AgentRole`.
- Added role helpers: `MarketSide`, `MarketBias`, `market_bias`, and
  `is_trader_like`.
- Split Bull/Bear default tools:
  - BullTrader: `buy_yes`, `abstain`
  - BearTrader: `buy_no`, `abstain`
- Extended role views:
  - BullTraderView: YES price, TaskOutcome YES market, NodeSurvive YES market,
    balance, realized/unrealized PnL, risk cap, liquidity/depth,
    deadline/budget remaining.
  - BearTraderView: NO price, unsolved-task risk, candidate weakness signals,
    challenge status, failed attempts, market depth, balance, PnL, risk cap.
  - SolverView keeps only limited market summary.
- Added `EconomicJudgment` CAS schema:
  `real12.economic_judgment.v1`.
- Added validator rules:
  - Buy/Short requires chosen market, positive amount, observed price,
    probability band, and positive expected-value sign.
  - BullTrader cannot choose NO.
  - BearTrader cannot choose YES.
  - Abstain must have a structured non-Unknown reason.
  - private CoT / raw prompt / raw completion / raw logs are rejected.
- Extended evaluator parsing for `buy_yes` / `buy_no`.
- Extended live runner metrics:
  `bull_judgment_count`, `bear_judgment_count`,
  `buy_yes_router_count`, `buy_no_router_count`,
  `live_non_scripted_router_tx_count`, and
  `abstain_reason_distribution`.

## Evidence

Canonical REAL-12 live micro-probe evidence:

```text
handover/evidence/real12_role_specialized_micro_probe_20260516T023351Z/
```

Command:

```bash
REAL12_PROBLEM_SET=mini \
TURINGOS_REAL12_TASK_MARKET_AFFORDANCE=1 \
TURINGOS_REAL12_TRADER_OBJECTIVE=0 \
MAX_TRANSACTIONS=10 \
PER_PROBLEM_TIMEOUT_S=300 \
bash scripts/run_real12_task_market_probe.sh real12_role_specialized_micro_probe_20260516T023351Z
```

Evidence facts:

```text
tasks = 3
roles = Solver,BullTrader,BearTrader,Verifier,Challenger
audit_tape = PROCEED
PERSISTENCE_BINDING_REPORT.is_passing = true
market_seed = 5
cpmm_pool = 5
event_resolve = 2
MarketOpportunityTrace count = 4
economic_judgment_total = 4
bull_judgment_count = 2
bear_judgment_count = 2
abstain_structured_reason_count = 4
economic_judgment_coverage_ok = true
economic_judgment_required_trader_turns = 4
economic_judgment_linked_trader_turns = 4
agent_economic_action_tx_count = 0
buy_yes_router_count = 0
buy_no_router_count = 0
live_non_scripted_router_tx_count = 0
buy_with_coin_router = 0
E2 verdict = E2 NOT ACHIEVED
```

No-trade distribution:

```json
{
  "NoPerceivedEdge": 4,
  "zero_amount": 0,
  "no_pool": 0,
  "amount_exceeds_balance": 0,
  "prompt_budget_exceeded": 0,
  "router_rejected": 0
}
```

Interpretation:

```text
REAL-12 moved the system from generic Trader visibility to role-specialized
Bull/Bear economic judgment. It did not cause live non-scripted trading.
Because task-level markets existed and NoPool did not dominate, the next
branch is REAL-13A expected-value scaffolding rather than immediate live
REAL-6B.
```

Remediation-only evidence:

```text
handover/evidence/real12_role_specialized_micro_probe_20260516T023050Z/
```

This earlier run is not conclusion-bearing. It failed because the evaluator
Trust Root hash was stale after implementation changes. The Trust Root was
rehashed, verified, and the canonical 02:33:51Z run above replaced it for
conclusions.

## Verification

Targeted REAL-12 tests:

```bash
cargo test --test constitution_real12_claim_boundary \
  --test constitution_real12_role_specialization \
  --test constitution_real12_role_views \
  --test constitution_real12_economic_judgment \
  --test constitution_real12_bull_bear_positive_control \
  --test constitution_real12_live_micro_probe \
  --test constitution_real12_task_market_action \
  -- --test-threads=1
```

Result:

```text
25 passed / 0 failed
```

Regression tests:

```bash
cargo test --test constitution_real12_task_market_action \
  --test constitution_real5_role_assignment \
  --test constitution_real5_role_scoped_view \
  --test constitution_real5_typed_generation_gateway \
  --test constitution_real11_market_opportunity_trace \
  --test constitution_real11_e2_micro_probe \
  -- --test-threads=1
```

Result:

```text
all passed
```

Additional focused regressions:

```bash
cargo test --test constitution_real6_conviction_budget -- --test-threads=1
cargo test --test constitution_real5_trader_activation \
  --test constitution_real5_role_based_smoke \
  --test constitution_real6_task_outcome_market \
  -- --test-threads=1
```

Result:

```text
4/4 passed
25/25 passed
```

Trust Root:

```bash
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
```

Result:

```text
1 passed / 0 failed
```

Constitution gates:

```bash
bash scripts/run_constitution_gates.sh
```

Result:

```text
461 passed / 0 failed / 1 ignored
```

Workspace:

```bash
cargo test --workspace --no-fail-fast -- --test-threads=1
```

Result:

```text
exit 0
```

Note: the first constitution-gate attempt in this worktree failed because the
git worktree lacked historical untracked evidence fixtures. I copied the
missing fixtures from `/home/zephryj/projects/turingosv4/handover/evidence`
using `rsync -a --ignore-existing`, preserving old evidence byte-for-byte and
not overwriting any existing evidence. After fixture hydration, gates passed.

## Cleanup

An independent cleanup agent removed only regenerable build-cache files from
the original worktree target directory and did not touch evidence, directives,
audits, reports, ChainTape, CAS, TB_LOG, or LATEST. It left the active REAL-12
worktree target directories intact because they were being used for verification.

## Open Scientific Result

REAL-12 is a diagnostic success and a generative-economy negative result:

```text
role-specialized economic judgment is now visible,
but live agent economic action remains absent.
```

Next branch:

```text
REAL-13A expected-value scaffolding:
  make Bull/Bear probability and EV reasoning more explicit,
  still without forced trade,
  still with price as signal only.
```
