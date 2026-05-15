# REAL-11 Traceability Update

Date: 2026-05-15

## FC Mapping

REAL-11 touches:

```text
FC1: externalized role/economic action loop
FC2: Trust Root / replay authority for pinned evaluator and dashboard surfaces
FC3: dashboard/report as materialized view from ChainTape + CAS
Art. II: selective broadcast of market/PnL/risk signals
Art. III: shielding of raw prompt/completion/CoT and private diagnostics
Economy: integer-only market action classification and router positive control
```

## New Surfaces

| Surface | FC / invariant | Evidence |
| --- | --- | --- |
| `src/runtime/market_tx_category.rs` | FC3 materialized metrics distinguish structural market activity from live agent economic action | `tests/constitution_real11_market_tx_category.rs` |
| `src/runtime/market_opportunity_trace.rs` | FC1/FC3 Trader turn opportunity trace is CAS-anchored and typed | `tests/constitution_real11_market_opportunity_trace.rs` |
| `scripts/run_real11_router_positive_control.sh` | Economy/router positive control, not E2 | `tests/constitution_real11_router_positive_control.rs`; `handover/evidence/real11_router_positive_control_20260515T172419Z_r2b/` |
| `scripts/run_real11_e2_micro_probe.sh` | no-forced-trade live E2 diagnostic with no live REAL-6B | `tests/constitution_real11_e2_micro_probe.rs`; patched canonical `handover/evidence/real11_e2_micro_probe_20260515T172707Z_r2_max10/`; supplemental actionable-opportunity diagnostic `handover/evidence/real11_e2_micro_probe_20260515T165855Z/` |
| `experiments/minif2f_v4/src/bin/evaluator.rs` | env-gated MarketOpportunityTrace CAS write on Trader turns | Trust Root rehash + `tests/constitution_real11_market_opportunity_trace.rs` |
| `src/bin/audit_dashboard.rs` | FC3 materialized count of MarketOpportunityTrace CAS records | Trust Root rehash + E2 micro-probe dashboard |
| `src/runtime/real6_conviction_budget.rs` | scoped PnL/risk/bankruptcy/autopsy summary in Trader view | `tests/constitution_real11_trader_pnl_visibility.rs` |

## Kill Conditions

REAL-11 row goes RED if any of the following happen:

```text
market_tx_count is reported without StructuralMarketTx / AgentEconomicActionTx split;
scripted fixture is counted as E2;
MarketOpportunityTrace is absent for observed Trader market turns;
actionable_markets > 0 and no trade lacks classified NoTradeReason;
PnL/risk summary becomes a HashMap sidecar source of truth;
router positive-control violates CTF conservation or ghost-liquidity guard;
price affects Lean predicate or L4/L4.E admission;
raw prompt/completion/CoT enters public records;
live REAL-6B is enabled without separate Class-4 ratification.
```

## Remediation Evidence

After clean-context audit CHALLENGE, Atom 2 was upgraded from unit-only
evidence to runtime evidence:

```text
handover/evidence/real11_router_positive_control_20260515T172419Z_r2b/
```

This directory contains:

```text
runtime_repo/
cas/
aggregate_verdict.json = audit_tape PROCEED
audit_dashboard_run_report.txt
REAL11_ROUTER_POSITIVE_CONTROL_VERDICT.json = PROCEED
buy_with_coin_router = 6
```

Those buys are scripted TaskOutcomeMarket positive controls and remain not E2.
