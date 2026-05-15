# Clean-Context Codex Audit: REAL-11 Atom 1-2

Date: 2026-05-15

Scope: REAL-11 Atom 1-2 only: market tx category split and router positive-control fixture.

Risk / FC mapping reviewed:

- Atom 1: Class 2 report-side category split; FC1/FC3 materialized market-action metrics.
- Atom 2: Class 3 router positive-control evidence; FC1 router action to L4/L4.E, FC2 evidence/replay discipline, FC3 scripted-not-E2 claim boundary, economy gates.

## Findings

1. [P1] Atom 2 evidence still closes on cargo tests, not on the required normal ChainTape runtime evidence path. The execution plan says Atom 2 "cannot close on unit tests alone", must run the scripted router path through the normal ChainTape runtime, must run `audit_tape` or aggregate-verdict over the runtime repo, and must include runtime_repo/CAS pointers plus dashboard materialized evidence (`handover/directives/2026-05-15_REAL11_AGENT_ECONOMIC_ACTION_ACTIVATION_EXECUTION_PLAN.md:700-727`). The runner instead executes only `cargo test --test constitution_real11_router_positive_control` (`scripts/run_real11_router_positive_control.sh:53-60`), and its mandatory artifacts are limited to `manifest.json`, `aggregate_verdict.json`, report, stdout, and stderr (`scripts/run_real11_router_positive_control.sh:14-19`, `scripts/run_real11_router_positive_control.sh:100-106`). The test harness uses `InMemoryLedgerWriter` and a temp CAS (`tests/constitution_real11_router_positive_control.rs:55-60`), and the recorded evidence manifest also names only the cargo test command and those five artifacts (`handover/evidence/real11_router_positive_control_20260515T165147Z/manifest.json:9-25`). This proves the Sequencer/router unit path, but it does not satisfy the mandatory FC2 replay/evidence contract for Atom 2.

2. [P1] The production live-agent invest path still contains a float money parse/conversion that the Atom 2 no-f64 gate does not cover. Agent actions deserialize `amount` as `Option<f64>` (`src/sdk/protocol.rs:11-19`), and the evaluator invest branch converts it with `action.amount.unwrap_or(0.0) as i64` before constructing the router tx (`experiments/minif2f_v4/src/bin/evaluator.rs:5188-5208`, `experiments/minif2f_v4/src/bin/evaluator.rs:5246-5275`). The Atom 2 test for SG-11.2.7 only greps `src/state/router_quote.rs` and `src/runtime/agent_pnl.rs` (`tests/constitution_real11_router_positive_control.rs:413-425`), so the current PASS does not cover the actual live agent router input path. This is directly relevant to the requested "no f64 money path" invariant and should block claiming Atom 2 fully closed.

3. [P2] SG-11.1.4 asks for dashboard separation of structural market activity from agent economic action, but Atom 1 currently wires the split only into the helper/report path. `src/runtime/market_tx_category.rs` exposes the split helper and hard-coded REAL-10 re-render (`src/runtime/market_tx_category.rs:37-70`, `src/runtime/market_tx_category.rs:112-154`), and the report shows the four columns with E2 false (`handover/reports/REAL11_MARKET_TX_CATEGORY_REPORT.md:15-34`). I did not find `market_tx_category` / split-counter usage in `src/bin/audit_dashboard.rs`; the grep hits are limited to the helper, tests, and reports. If "dashboard" in SG-11.1.4 is literal, this remains a materialized-view gap rather than a production admission defect.

## Positive Checks

- The category logic is conservative for router/short/sell actions: only live, non-scripted, non-forced actions with prompt/trace link, ChainTape anchor, and `audit_proceed=true` become `AgentEconomicActionTx`; scripted/missing/forced router actions stay `ScriptedFixtureTx` (`src/runtime/market_tx_category.rs:50-63`).
- REAL-10 re-render keeps `agent_economic_action_tx_count=0` for all arms and splits the scripted AttemptPrediction fixture away from E2 (`src/runtime/market_tx_category.rs:116-151`; `handover/reports/REAL11_MARKET_TX_CATEGORY_REPORT.md:17-26`).
- Router unit coverage includes BuyYes L4 acceptance, CTF conservation, complete-set balance/no-ghost checks, and PnL movement (`tests/constitution_real11_router_positive_control.rs:219-263`).
- Router unit coverage includes BuyNo/short-equivalent L4 acceptance and NO exposure (`tests/constitution_real11_router_positive_control.rs:265-301`).
- Insufficient balance and missing pool both reach explicit rejection evidence after pre-submit classification (`tests/constitution_real11_router_positive_control.rs:303-365`, `tests/constitution_real11_router_positive_control.rs:368-411`).
- The router report and evidence manifest correctly state the scripted fixture is not E2 (`handover/reports/REAL11_ROUTER_POSITIVE_CONTROL_REPORT.md:5-7`; `handover/evidence/real11_router_positive_control_20260515T165147Z/aggregate_verdict.json:3-8`).

## Audit Question Answers

1. Scripted fixture remains clearly not E2: yes, in the category report, router report, manifest, and aggregate verdict.
2. Category logic avoids counting scripted/structural/resolution tx as `AgentEconomicActionTx`: yes for the helper and REAL-10 re-render reviewed.
3. Router positive-control coverage: partially. Unit tests cover BuyYes, BuyNo/short, insufficient balance, missing pool, CTF conservation, no ghost liquidity, and a narrow no-f64 grep. The required runtime_repo/CAS evidence path is missing, and the live invest parser still has an uncovered f64 money conversion.
4. Production defects / claim-boundary risks: no E2 overclaim found in the reviewed reports. The blocking risks are the Atom 2 evidence-contract gap and the uncovered f64 production invest path.

## Verification

Re-ran:

```text
cargo test --test constitution_real11_evidence_hygiene --test constitution_real11_market_tx_category --test constitution_real11_claim_boundary --test constitution_real11_router_positive_control --no-fail-fast -- --test-threads=1
```

Result: 13 passed / 0 failed across the four target test binaries. Existing warnings only.

## Verdict

CHALLENGE
