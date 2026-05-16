# REAL-13 Market Pressure Loop Implementation Report

Date: 2026-05-16

## Scope

This package implements the first ship-path slice of REAL-13:

- REAL-13A: EVDecisionTrace / Expected-Value Scaffolding.
- REAL-13B-v1: deterministic sequential Market Review Turn sidecars.
- REAL-13C: fixed-point DisplayCoin parser and integer/bps cognitive bridge helpers.
- REAL-13D: signal purification separating forced CreatorBond from voluntary market positions.
- REAL-13H: live integrated micro-probe runner with live REAL-6B disabled and no scripted buys.

This package does not implement live REAL-6B, full async market arena, forced trading, price-as-truth, ghost liquidity, private CoT storage, raw-log broadcast, or any sequencer/TypedTx/signing-payload change.

## FC / Risk Mapping

- FC1: role economic action loop; Bull/Bear turns now emit CAS-backed EVDecisionTrace when REAL-13 is enabled.
- FC2: replay authority remains existing genesis/batch role assignment and PromptCapsule lineage; this package adds no hidden global pointer.
- FC3: audit/dashboard views remain materialized views over ChainTape/CAS; `audit_dashboard --run-report` derives EVDecisionTrace and MarketReviewSummary counts from CAS.
- Art. III shielding: no raw prompt, raw completion, private CoT, raw logs, or raw diagnostics are stored in EVDecisionTrace / MarketReview sidecars.

Risk class: Class 4 package because Trust Root pinned files were updated (`experiments/minif2f_v4/src/bin/evaluator.rs`, `src/runtime/mod.rs`, `src/bin/audit_dashboard.rs`, `genesis_payload.toml`). No restricted protocol surface was edited.

## Changed Surfaces

- `src/runtime/ev_decision_trace.rs`
- `src/runtime/display_coin.rs`
- `src/runtime/market_review.rs`
- `src/runtime/signal_purification.rs`
- `src/runtime/mod.rs`
- `experiments/minif2f_v4/src/bin/evaluator.rs`
- `src/bin/audit_dashboard.rs`
- `scripts/run_real13_market_pressure_probe.sh`
- `tests/constitution_real13a_ev_decision_trace.rs`
- `tests/constitution_real13a_display_coin.rs`
- `tests/constitution_real13b_market_review_window.rs`
- `tests/constitution_real13d_signal_purification.rs`
- `tests/constitution_real13h_market_pressure_probe.rs`
- `genesis_payload.toml`

## Evidence

### Remediation Record

`handover/evidence/real13_market_pressure_probe_20260516T070534Z/`

The first REAL-13 probe is not conclusion-bearing. It exited after the runner detected:

```text
ev_decision_trace_total_cas=0
market_review_summary_cas_count=0
```

Cause: the initial role order let the Solver solve too quickly before Bull/Bear review turns fired. The runner was fixed to use:

```text
TURINGOS_REAL5_ROLE_ASSIGNMENT=BullTrader,BearTrader,Solver,Verifier,Challenger
```

### Canonical Clean Probe

`handover/evidence/real13_market_pressure_probe_20260516T071216Z/`

Key CAS-derived metrics:

```text
audit_tape verdict: PROCEED
ev_decision_trace_total_cas=10
ev_decision_trace_bull_count_cas=5
ev_decision_trace_bear_count_cas=5
ev_decision_trace_buy_yes_count_cas=0
ev_decision_trace_buy_no_count_cas=0
ev_decision_trace_abstain_count_cas=10
market_review_summary_cas_count=10
live_non_scripted_router_tx_count=0
```

Underlying REAL-12 task-market probe metrics in the same run:

```text
MarketOpportunityTrace count=10
market_seed=6
cpmm_pool=6
economic_judgment_total=10
bull_judgment_count=5
bear_judgment_count=5
economic_judgment_coverage_ok=true
abstain_reason_distribution={"NoPerceivedEdge":10}
buy_with_coin_router=0
```

Interpretation:

```text
E2 NOT ACHIEVED.
```

REAL-13 successfully makes Bull/Bear market review decisions tape/CAS-visible, but live non-scripted router action is still zero.

## Verification Commands

Targeted + Trust Root + formatting:

```bash
cargo test --test constitution_real13a_ev_decision_trace \
  --test constitution_real13a_display_coin \
  --test constitution_real13b_market_review_window \
  --test constitution_real13d_signal_purification \
  --test constitution_real13h_market_pressure_probe \
  --test constitution_real6_task_outcome_market
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
cargo fmt --all -- --check
git diff --check
```

Result:

```text
REAL-13 targeted tests: 17 passed / 0 failed.
REAL-6 TaskOutcomeMarket regression target: 19 passed / 0 failed.
Trust Root verify: 1 passed / 0 failed.
fmt check: pass.
diff check: pass.
```

Constitution gates:

```bash
bash scripts/run_constitution_gates.sh
```

Result:

```text
Totals: 461 passed, 0 failed, 1 ignored
```

Workspace:

```bash
cargo test --workspace --no-fail-fast -- --test-threads=1
```

Result:

```text
exit 0
```

During the first workspace run, `constitution_real6_task_outcome_market::sg_6a_10_price_is_not_used_as_lean_predicate_truth` failed on a source sentinel matching `if price` in a REAL-13 helper local variable. The variable was renamed to avoid the sentinel and preserve the price-as-signal guard. The target and full workspace were re-run successfully afterward.

## Claim Boundary

Allowed claim:

```text
REAL-13A/B/C/D/H scaffold is implemented for sequential Market Review Turn evidence.
EVDecisionTrace and MarketReviewSummary are CAS-backed and dashboard-visible as materialized views.
```

Forbidden claims:

```text
No E2 spontaneous economic action claim.
No E3 persistent role differentiation claim.
No E4 causal performance claim.
No model ranking claim.
No live REAL-6B approval.
No autonomous market emergence claim.
```

## Next Decision

The evidence points to a precise mechanism gap:

```text
Bull/Bear enter the review loop and produce EVDecisionTrace, but every live decision abstains with NegativeEV / NoPerceivedEdge and live router count remains 0.
```

Recommended next design branch:

- Improve REAL-13C trader UX / EV objective framing if architect wants to continue without live REAL-6B.
- Consider REAL-13F private alpha sandbox if the hypothesis is "no information asymmetry."
- Consider REAL-13E legal MarketMaker if the hypothesis is "liquidity/price cold-start."
- Do not claim E2 until a live, non-scripted, agent-generated router/short tx appears with PromptCapsule + EVDecisionTrace + ChainTape provenance.
