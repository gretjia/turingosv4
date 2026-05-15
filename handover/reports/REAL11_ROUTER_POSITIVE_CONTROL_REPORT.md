# REAL-11 Router Positive-Control Report

source evidence path: `/home/zephryj/projects/turingosv4/handover/evidence/real11_router_positive_control_20260515T172419Z_r2b`

This scripted positive control is not E2. It proves router wiring only.
E2 remains false unless a live, non-scripted, agent-generated router/short
action is observed on ChainTape/CAS.

## Command

```text
cargo test --test constitution_real11_router_positive_control -- --test-threads=1
```

test_exit_code: `0`
runtime_exit_code: `0`
dashboard_exit_code: `0`
audit_tape verdict: `PROCEED`
aggregate_verdict: `PROCEED`
test_status: `PASS`

## Runtime Chain Evidence

```text
runtime_repo: /home/zephryj/projects/turingosv4/handover/evidence/real11_router_positive_control_20260515T172419Z_r2b/runtime_repo
CAS path: /home/zephryj/projects/turingosv4/handover/evidence/real11_router_positive_control_20260515T172419Z_r2b/cas
audit_dashboard: /home/zephryj/projects/turingosv4/handover/evidence/real11_router_positive_control_20260515T172419Z_r2b/audit_dashboard_run_report.txt
market_seed: 6
cpmm_pool: 6
buy_with_coin_router: 6
scripted_task_outcome_buys: Agent_1:Agent_2:1000
```

This runtime evidence uses scripted TaskOutcomeMarket buys as a positive
control. It proves the normal ChainTape/CAS/audit-dashboard path can carry
router actions; it is explicitly not E2.

## Claim Boundary

```text
scripted fixture == not E2
no forced trade
no price-as-truth
no ghost liquidity
no f64 economy
no private CoT recording
no raw-log broadcast
dashboard/report is a materialized view, not source of truth
```

## SG Coverage

| Gate | Status | Evidence |
| --- | --- | --- |
| SG-11.2.1 scripted BuyYesWithCoinRouterTx enters L4 | PASS | `cargo_test.stdout` + `aggregate_verdict.json` |
| SG-11.2.2 scripted BuyNo / short-equivalent enters L4 or explicit L4.E | PASS | `cargo_test.stdout` + `aggregate_verdict.json` |
| SG-11.2.3 insufficient balance routes L4.E / pre-submit classification | PASS | `cargo_test.stdout` / `cargo_test.stderr` |
| SG-11.2.4 missing pool routes NoPool / L4.E | PASS | `cargo_test.stdout` / `cargo_test.stderr` |
| SG-11.2.5 CTF conserved | PASS | `cargo_test.stdout` / `cargo_test.stderr` |
| SG-11.2.6 no ghost liquidity | PASS | `cargo_test.stdout` / `cargo_test.stderr` |
| SG-11.2.7 no f64 money path | PASS | `cargo_test.stdout` / `cargo_test.stderr` |

## Audit Readiness

positive_control_verdict: `PROCEED`

The manifest records aggregate verdict now and keeps a separate slot for later
clean-context audit:
`audit_ready.clean_context_audit_verdict`.
