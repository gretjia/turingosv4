# Clean-Context Codex Audit: REAL-11 Atom 3-6

Date: 2026-05-15

Scope: REAL-11 Atom 3-6: MarketOpportunityTrace, PnL/risk visibility, E2 micro-probe, and decision gate.

Risk / FC mapping reviewed:

- Atom 3: Class 2/3; FC1 Trader turn visibility and FC3 CAS/dashboard materialized view.
- Atom 4: Class 2/3; Art. II scoped PnL/risk broadcast and Art. III shielding.
- Atom 5: Class 3; FC2 evidence/replay discipline and no-live-REAL-6B/no-scripted/no-forced-trade micro-probe.
- Atom 6: Class 0/3; FC3 decision report and E1/E2/E3/E4 claim boundary.

## Findings

1. [P1] Atom 5 does not fail closed against scripted TaskOutcome router buys, and would misclassify them as live E2 if they occur. The micro-probe runner rejects `TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION` and `TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE` only (`scripts/run_real11_e2_micro_probe.sh:24-31`), then exports REAL-11 probe flags without rejecting or unsetting `TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS` (`scripts/run_real11_e2_micro_probe.sh:33-41`). The evaluator still honors `TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS` by constructing scripted TaskOutcomeMarket YES/NO `BuyWithCoinRouterTx` submissions (`experiments/minif2f_v4/src/bin/evaluator.rs:1965-2044`). If any router buy is observed, the runner sets `live_non_scripted_router_tx_count="$buy_with_coin_router"` and declares E2 `ACHIEVED` solely because Atom 5 was intended to have no scripted buys (`scripts/run_real11_e2_micro_probe.sh:104-115`). The post-run check only rejects AttemptPrediction fixture count (`scripts/run_real11_e2_micro_probe.sh:117-120`), and the batch manifest does not record the REAL-11 no-scripted/live sentinels (`handover/evidence/real11_e2_micro_probe_20260515T165855Z/G_PHASE_BATCH_MANIFEST.json:1-31`). This violates the Atom 5 requirement "No scripted buys in the live probe arm" and can produce a false E2 branch under environment contamination.

## Positive Checks

- MarketOpportunityTrace is a shielded CAS record. The schema contains typed IDs/counts, balance, router availability, NoTradeReason, and a PromptCapsule CID, with no raw prompt/completion/CoT fields (`src/runtime/market_opportunity_trace.rs:30-43`). It derives from `QState` economic state (`src/runtime/market_opportunity_trace.rs:45-112`) and writes to CAS as `ObjectType::Generic` with schema id `real11.market_opportunity_trace.v1` (`src/runtime/market_opportunity_trace.rs:132-147`). The reviewed evidence has one such CAS record (`handover/evidence/real11_e2_micro_probe_20260515T165855Z/cas/.turingos_cas_index.jsonl:50`).
- Evaluator trace emission is fail-closed on missing ChainTape bundle, missing QState snapshot, CAS open failure, or CAS put failure (`experiments/minif2f_v4/src/bin/evaluator.rs:3822-3881`). I found no sequencer admission, typed transaction schema/discriminant, or signing payload edits in the reviewed Atom 3-6 diff.
- PnL/risk visibility is derived from replay/QState paths, not a new sidecar truth. The prompt view renders `compute_agent_pnl` over `QState` (`src/sdk/your_position.rs:68-160`), ConvictionBudget derives from `compute_agent_pnl` plus risk-cap helper (`src/runtime/real6_conviction_budget.rs:52-62`), and dashboard PnL trajectory is computed from runtime repo + CAS paths (`src/bin/audit_dashboard.rs:2549-2582`).
- The concrete 2026-05-15 16:58:55Z micro-probe evidence correctly concludes E2 not achieved: audit_tape is `PROCEED` (`handover/evidence/real11_e2_micro_probe_20260515T165855Z/aggregate_verdict.json:398-416`), `buy_with_coin_router=0` (`.../aggregate_verdict.json:11-24`), `scripted_attempt_prediction_market_count=0` (`handover/evidence/real11_e2_micro_probe_20260515T165855Z/audit_dashboard_run_report.txt:468-475`), and the report states `live_non_scripted_router_tx_count=0` / `NOT ACHIEVED` (`handover/reports/REAL11_E2_MICRO_PROBE_REPORT.md:20-44`).
- Branch B is supported for this specific run, subject to the scripted-buy fail-closed fix above: Agent_1's CAS-anchored MarketOpportunityTrace shows `actionable=3`, `router_available=true`, and `balance=1000000` (`handover/reports/REAL11_E2_MICRO_PROBE_REPORT.md:32-34`), while the decision report keeps E2/E3/E4 and live REAL-6B claims out of scope (`handover/reports/REAL11_DECISION_GATE_REPORT.md:97-150`).

## Audit Question Answers

1. MarketOpportunityTrace CAS anchored and shielded: yes for the trace itself. It is CAS-indexed with schema id and does not inline raw prompt/completion/CoT.
2. Evaluator fail-safety / restricted surfaces: mostly yes for Atom 3 trace emission and no sequencer/typed/signing changes. The remaining fail-safety gap is Atom 5 scripted-buy contamination, not restricted-surface drift.
3. PnL/risk visibility source: yes, it derives from ChainTape/QState/replay helpers rather than a new sidecar source of truth.
4. Micro-probe no-live/no-scripted/no-forced and E2 conclusion: the actual evidence accurately says E2 `NOT ACHIEVED`, but the runner does not enforce the full no-scripted-buy contract and can falsely classify scripted TaskOutcome buys as live E2.
5. Branch B evidence: yes for the reviewed run: actionable opportunity existed, router was available, balance was positive, PnL/risk was visible, and the agent abstained with `NoPerceivedEdge` evidence.

## Verification

Re-ran:

```text
cargo test --test constitution_real11_market_opportunity_trace --test constitution_real11_trader_pnl_visibility --test constitution_real11_e2_micro_probe --test constitution_real11_no_live_real6b --test constitution_real11_matrix_update --no-fail-fast
```

Result: 17 passed / 0 failed across the five target test binaries. Existing warnings only.

## Verdict

CHALLENGE
