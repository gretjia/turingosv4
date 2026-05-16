# REAL-13 Market Pressure Loop Execution Plan

## Summary

REAL-13 rebuilds v3's useful pressure loop under v4 constitution:

```text
market review turn
-> EVDecisionTrace
-> optional voluntary router action
-> ChainTape/CAS evidence
-> PnL / report feedback
```

It does not relax predicate gates, force trades, add ghost liquidity, or approve
live REAL-6B.

Current branch facts from REAL-12:

```text
evidence: handover/evidence/real12_role_specialized_micro_probe_20260516T023351Z/
MarketOpportunityTrace=4
economic_judgment_total=4
bull_judgment_count=2
bear_judgment_count=2
abstain_reason_distribution={"NoPerceivedEdge":4}
buy_with_coin_router=0
live_non_scripted_router_tx_count=0
audit_tape=PROCEED
E2 NOT ACHIEVED
```

## Phase 1 Implemented In This Package

This package lands the first safe scaffold:

```text
REAL-13A EVDecisionTrace CAS Generic sidecar.
REAL-13B MarketReviewWindow / Response / Summary sidecars.
REAL-13C DisplayCoin fixed-point parser/formatter.
REAL-13D report-side signal purification.
```

Explicitly not implemented here:

```text
live REAL-6B
sequencer admission changes
TypedTx schema/discriminant changes
canonical signing payload changes
wallet/kernel/bus/CAS ObjectType changes
forced trade
full async
E2/E3/E4 claim
```

## Worker Contract

Implementation/audit workers may all use GPT-5.5 with depth switches. Runtime
agents are different: REAL-13G requires a model assignment manifest with at
least three model families and no hidden model switch before any model-family
behavior claim.

## Current Code Anchors

```text
src/runtime/economic_judgment.rs
  existing REAL-12 EconomicJudgment and ExpectedValueSign.

src/runtime/real5_roles.rs
  existing BullTrader / BearTrader role variants and tool policy.

src/runtime/market_opportunity_trace.rs
  existing MarketOpportunityTrace CAS Generic sidecar pattern.

scripts/run_real12_task_market_probe.sh
  existing fail-closed sentinels for no live REAL-6B / no scripted buys.
```

## Tests Added In This Package

```bash
cargo test --test constitution_real13a_ev_decision_trace
cargo test --test constitution_real13a_display_coin
cargo test --test constitution_real13b_market_review_window
cargo test --test constitution_real13d_signal_purification
```

These gates prove:

```text
EVDecisionTrace round-trips through Generic CAS with schema id.
EV bps fields are integer bounded in [0,10000].
No f64/f32 or private/raw material appears in EV trace.
Bull/Bear side policy is enforced.
Abstain requires structured EV reason.
DisplayCoin parses decimal strings to integer micro only.
DisplayCoin rejects scientific notation, floaty forms, padding, signs, overflow.
MarketReviewResponse requires EVDecisionTrace or NoResponseTrace CID.
MarketReview ordering is deterministic by agent_id.
FullAsyncExperimental is unsafe-only.
WorkTx/Challenge/Verify bonds are separated from voluntary market position.
E2 counts only BuyWithCoinRouter-style voluntary market action.
```

## Route After This Package

Next atoms should wire the scaffold into runner/evaluator/report paths:

```text
1. Write EVDecisionTrace per Bull/Bear review turn.
2. Add MarketReviewWindow creation in evaluator/orchestrator only.
3. Add run-report EV and review-window tables derived from CAS.
4. Add live REAL-13H integrated probe.
5. Run clean-context audit before any E2 candidate claim.
```

## Forbidden Claims

```text
No E2 claim unless live non-scripted agent-generated router/short action exists.
No E3 claim unless divergence persists across >=2 tasks/batches.
No E4 claim without statistical support.
Positive EV alone is not E2.
EVDecisionTrace alone is not E2.
Scripted positive-control is not E2.
```
