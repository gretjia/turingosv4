# REAL-11 Agent Economic Action Activation — Audited Execution Plan

## Summary

REAL-11 starts from the architect's post-REAL-10 ruling archived at:

```text
handover/directives/2026-05-15_REAL11_AGENT_ECONOMIC_ACTION_ACTIVATION_ARCHITECT_ORIGINAL.md
```

The controlling diagnosis is:

```text
market_tx_count 上升但 buy_with_coin_router = 0
```

REAL-10 is ratifiable only as clean, pinned, audited E1 controlled-market
evidence. It does not prove E2/E3/E4. REAL-11 therefore must not simply enlarge
the same REAL-8 A/B benchmark and must not rush into live REAL-6B. REAL-11 must
answer the smaller causal questions first:

```text
市场动作路径能否通过 scripted positive control？
Trader turn 是否真的有 actionable market？
Agent 是否看见 PnL / risk？
如果看见仍不交易，为什么？
```

REAL-11 success is not "force agents to trade." Success is either:

```text
E2 achieved by at least one live, non-scripted, agent-generated router/short tx;
or a clean, chain-backed explanation of why E2 is still absent.
```

## Non-Negotiable Claim Boundary

REAL-11 inherits `handover/alignment/EMERGENCE_METRICS_E1_E2_E3_E4.md`.

Allowed claims:

```text
REAL-10 clean evidence is canonical.
REAL-10 satisfies E1 for market-visible arms B/C/D.
REAL-10 shows structural market activity increases.
REAL-11 may prove the router positive-control path.
REAL-11 may prove whether Trader turns have actionable opportunity.
REAL-11 may prove whether Trader PromptCapsule exposes ChainTape-derived PnL/risk.
REAL-11 may achieve E2 only through live, non-scripted, agent-generated router/short action.
```

Forbidden claims:

```text
market_tx_count increase == emergence
scripted fixture == E2
role label == E3
small-n descriptive result == E4
live REAL-6B approved
market-caused solve improvement
model ranking
autonomous secondary market alive
real-world readiness
price-as-truth
forced trade
ghost liquidity
f64 economy
off-tape WAL as truth
private CoT recording
raw-log broadcast
dashboard/report as source of truth
```

## Risk And FC Mapping

Default package classification: **Class 3**.

Rationale:

```text
REAL-11 touches market/economic reports, CAS evidence, audit dashboards,
runner/evaluator behavior, router positive-control evidence, and live micro-probe
evidence. These are production evidence and economic surfaces.
```

Atom-specific risk:

| Atom | Risk | Reason |
|---|---:|---|
| Atom 0 | Class 0 | docs / evidence hygiene / ratification boundary only |
| Atom 1 | Class 2 | report/dashboard classification and tests; no tx schema |
| Atom 2 | Class 3 | router positive-control over economic transition evidence |
| Atom 3 | Class 2/3 | opportunity trace, CAS/report aggregation, live-turn evidence |
| Atom 4 | Class 2/3 | PromptCapsule/view/PnL visibility and risk-cap report checks |
| Atom 5 | Class 3 | live evidence micro-probe |
| Atom 6 | Class 0/3 | decision report; Class 3 if it closes a ship/evidence package |

Touched FC nodes/invariants:

```text
FC1 Runtime loop:
  rtool / role-scoped view -> Agent delta -> typed output / no-trade trace
  -> predicate / router admission -> L4 or L4.E / CAS fossil.

FC2 Boot / replay:
  pinned benchmark/evidence directories;
  same runtime_repo / CAS discipline for micro-probe;
  no fabricated genesis or rewritten historical evidence.

FC3 Meta / reporting:
  dashboard/report as materialized view;
  E1/E2/E3/E4 claim boundary;
  audit and decision gate derived from ChainTape + CAS.

Art. I:
  quantized market/action categories;
  E2/E3/E4 metrics as hard binary gates for claims.

Art. II:
  Trader receives broadcast market/PnL signals as scoped summaries.

Art. III:
  no raw prompt/CoT/log broadcast;
  PromptCapsule/MarketOpportunityTrace/MarketDecisionTrace are shielded,
  reconstructable evidence, not private monologue dumps.

Economy gates:
  no ghost liquidity;
  CTF conserved;
  no f64/f32 economy;
  price signal never affects Lean predicate.
```

Stop and re-ratify as **Class 4** before implementation if any worker needs:

```text
src/state/sequencer.rs
src/state/typed_tx.rs
canonical signing payloads
src/kernel.rs
src/bus.rs
src/sdk/tools/wallet.rs
src/bottom_white/cas/schema.rs
genesis_payload.toml
constitution.md / flowchart authority documents
live REAL-6B real-LLM AttemptPrediction
new typed transaction discriminants
sequencer admission changes
ObjectType enum changes
```

## Agent Topology

All agents use GPT-5.5. Only reasoning effort changes.

| Role | Reasoning | Responsibility | Notes |
|---|---|---|---|
| Orchestrator | high; xhigh on restricted/claim surfaces | Own truth order, final plan, task splitting, merge, claim boundary, audit payloads | This thread |
| Docs Worker | low | Preserve architect original, produce doc skeletons, copy exact SG tables | May not interpret claims |
| Metrics Worker | medium | Atom 1 classification helper/report/test scaffolding | No restricted source edits |
| Router Fixture Worker | high | Atom 2 scripted BuyWithCoinRouter positive-control tests/evidence | Must not change sequencer/typed_tx |
| Opportunity Worker | high | Atom 3 trace design/tests; ensure ChainTape/CAS derivation | Must avoid raw CoT and CAS ObjectType changes |
| PnL/View Worker | high | Atom 4 TraderView/PromptCapsule PnL visibility tests | Must prove ChainTape fold, not sidecar |
| Probe Worker | medium/high | Atom 5 runner/report micro-probe evidence | Medium for runner, high for evidence interpretation |
| Analysis Worker | high | Atom 6 decision gate and branch classification | Must not overclaim E2/E3/E4 |
| Audit Worker | xhigh | Independent clean-context audits at selected phase boundaries | Verdict: `PROCEED | CHALLENGE | VETO` |

Audit intervention points:

```text
Audit 1: plan alignment before code/evidence execution.
Audit 2: after Atom 1-2, because metric decomposition + router path are core.
Audit 3: after Atom 3-5, because opportunity/PnL/live evidence affect claims.
Audit 4: final ship review after gates + evidence.
```

No per-atom audit unless a worker touches restricted surfaces, changes Trust
Root, changes CAS schema/ObjectType, or attempts an E2/E3/E4 claim escalation.

## Harness Contract

Open a `turingos_dev` run when implementation starts:

```text
module: REAL-11 Agent Economic Action Activation
risk: 3
unit: atom
fc_nodes:
  FC1 market action / no-trade loop
  FC2 pinned evidence / replay discipline
  FC3 dashboard-report materialized views
  Art. I quantization
  Art. II selective broadcast
  Art. III shielding
  economy gates
intent:
  Move market activity analysis from structural market tx to agent economic action,
  without forced trade, price-as-truth, or live REAL-6B.
```

Allowed planned paths:

```text
handover/directives/2026-05-15_REAL11_AGENT_ECONOMIC_ACTION_ACTIVATION_ARCHITECT_ORIGINAL.md
handover/directives/2026-05-15_REAL11_AGENT_ECONOMIC_ACTION_ACTIVATION_EXECUTION_PLAN.md
handover/directives/2026-05-15_REAL10_NARROW_RATIFICATION_REAL11.md
handover/alignment/EMERGENCE_METRICS_E1_E2_E3_E4.md
handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md
handover/alignment/TRACE_FLOWCHART_MATRIX.md
handover/alignment/DECISION_REAL11_MARKET_TX_CATEGORY.md
handover/alignment/REAL11_TRACE_MATRIX_UPDATE.md
handover/alignment/OBS_REAL11_*.md
handover/audits/CODEX_REAL11_EXECUTION_PLAN_ALIGNMENT_REVIEW.md
handover/audits/CODEX_REAL11_EXECUTION_PLAN_ALIGNMENT_REVIEW_R2.md
handover/audits/CODEX_REAL11_EXECUTION_PLAN_ALIGNMENT_REVIEW_R3.md
handover/audits/CODEX_REAL11_ATOM1_2_REVIEW.md
handover/audits/CODEX_REAL11_OPPORTUNITY_PNL_PROBE_REVIEW.md
handover/audits/CODEX_REAL11_FINAL_IMPLEMENTATION_REVIEW.md
handover/reports/REAL11_MARKET_TX_CATEGORY_REPORT.md
handover/reports/REAL11_ROUTER_POSITIVE_CONTROL_REPORT.md
handover/reports/REAL11_MARKET_OPPORTUNITY_REPORT.md
handover/reports/REAL11_PNL_INCENTIVE_VISIBILITY_REPORT.md
handover/reports/REAL11_E2_MICRO_PROBE_REPORT.md
handover/reports/REAL11_DECISION_GATE_REPORT.md
src/runtime/market_tx_category.rs
src/runtime/market_opportunity_trace.rs
src/runtime/market_decision_trace_summary.rs
src/runtime/audit_assertions.rs
src/runtime/audit_views.rs
src/runtime/real5_roles.rs
src/runtime/agent_pnl.rs
src/runtime/prompt_capsule.rs
src/bin/audit_dashboard.rs
src/sdk/prompt.rs
experiments/minif2f_v4/src/bin/evaluator.rs
scripts/run_real11_router_positive_control.sh
scripts/run_real11_e2_micro_probe.sh
tests/constitution_real11_evidence_hygiene.rs
tests/constitution_real11_market_tx_category.rs
tests/constitution_real11_router_positive_control.rs
tests/constitution_real11_market_opportunity_trace.rs
tests/constitution_real11_trader_pnl_visibility.rs
tests/constitution_real11_e2_micro_probe.rs
tests/constitution_real11_claim_boundary.rs
tests/constitution_real11_no_live_real6b.rs
tests/constitution_real11_matrix_update.rs
handover/evidence/real11_*
handover/evidence/dev_self_hosting/*
```

If implementation can satisfy an atom with fewer paths, prefer the smaller
surface. Do not edit restricted surfaces listed above.

## Architect Atom Requirements — Verbatim Gate Source

The following atom tasks and SG lines are copied from the architect original and
are the minimum acceptance source.

### Atom 0 — REAL-10 narrow ratification + evidence hygiene

```text
任务

1. 归档 REAL-10 narrow ratification。
2. 标记 clean evidence dir 为唯一 conclusion-bearing evidence。
3. 标记 contaminated evidence dir 为 remediation-only。
4. 确认 stale-parent behavioral test 是否已补。

验收

SG-11.0.1 REAL-10 ratification file exists.
SG-11.0.2 invalid evidence dir is excluded from all reports.
SG-11.0.3 clean evidence dir is canonical.
SG-11.0.4 stale-parent behavioral test passes or forward OBS exists.
```

### Atom 1 — Metric decomposition: structural vs agent economic action

```text
新增分类

enum MarketTxCategory {
    StructuralMarketTx,
    AgentEconomicActionTx,
    ScriptedFixtureTx,
    ResolutionTx,
}

任务

将当前 report 中的 `market_tx_count` 拆成：

structural_market_tx_count
agent_economic_action_tx_count
scripted_fixture_tx_count
resolution_tx_count

验收

SG-11.1.1 REAL-10 report can be re-rendered with split market tx categories.
SG-11.1.2 buy_with_coin_router appears under AgentEconomicActionTx only if live non-scripted.
SG-11.1.3 scripted AttemptPrediction fixture does not count as E2.
SG-11.1.4 dashboard clearly separates structural market activity from agent economic action.
```

### Atom 2 — Router positive-control fixture

```text
设计

使用 deterministic scripted agent，不算 E2，只做 wire proof：

active market pool
scripted trader payload
BuyWithCoinRouterTx
audit_tape PROCEED
CTF conserved
PnL updated

验收

SG-11.2.1 scripted BuyYesWithCoinRouterTx enters L4.
SG-11.2.2 scripted BuyNo / short-equivalent path enters L4 or L4.E with explicit class.
SG-11.2.3 insufficient balance routes L4.E.
SG-11.2.4 missing pool routes L4.E.
SG-11.2.5 CTF conserved.
SG-11.2.6 no ghost liquidity.
SG-11.2.7 no f64 money path.
```

### Atom 3 — Market opportunity visibility audit

```text
新增：

MarketOpportunityTrace {
    agent_id,
    role,
    task_id,
    head_t,
    visible_markets,
    actionable_markets,
    available_balance,
    router_available,
    reason_if_no_actionable_market,
}

验收

SG-11.3.1 Every Trader turn has MarketOpportunityTrace.
SG-11.3.2 If actionable_markets = 0, NoTradeReason must not be generic.
SG-11.3.3 If actionable_markets > 0 and no trade, record NoPerceivedEdge / AgentDeclined / RiskCap / Balance / PromptBudget.
SG-11.3.4 Opportunity trace derives from ChainTape + CAS.
```

### Atom 4 — PnL / incentive visibility check

```text
任务

确保 TraderView 中明确包含：

available balance
open positions
realized PnL
unrealized PnL
risk cap
bankruptcy/autopsy summary

验收

SG-11.4.1 Trader PromptCapsule includes PnL summary.
SG-11.4.2 PnL summary derives from ChainTape fold, not sidecar.
SG-11.4.3 Below-risk-cap agents cannot execute risky market action.
SG-11.4.4 Low-balance status appears as signal, not as hidden state.
```

### Atom 5 — Live E2 micro-probe without REAL-6B

```text
运行：

3–5 tasks
same pinned config
market enabled
TraderView with PnL
no forced trade

验收

SG-11.5.1 If live non-scripted BuyWithCoinRouterTx occurs: E2 achieved.
SG-11.5.2 If no buy occurs: every no-buy turn has MarketOpportunityTrace + NoTradeReason.
SG-11.5.3 No forced trade.
SG-11.5.4 No price-as-truth.
SG-11.5.5 audit_tape PROCEED.
```

### Atom 6 — Decision gate

```text
分支 A：出现 live buy

E2 achieved.
进入 REAL-12：role differentiation / E3。

分支 B：无 live buy，但机会存在

Agent sees actionable market but abstains.
进入 Trader objective redesign:
  role objective stronger,
  PnL prompt more explicit,
  maybe risk-adjusted return prompt,
  still no forced trade.

分支 C：无 live buy，因为无 actionable market

进入 event timing redesign:
  prepare live REAL-6B Class-4 packet.

分支 D：router positive-control 失败

回到 substrate fix。
```

## Current Evidence Inputs

Use these exact REAL-10 inputs in Atom 0 / Atom 1 rerendering:

```text
canonical clean evidence:
  handover/evidence/real8x_market_ab_clean_20260515T141331Z/

invalid/remediation-only evidence:
  handover/evidence/real8x_market_ab_20260515T134453Z/

constitution gates:
  461 passed / 0 failed / 1 ignored

workspace tests:
  exit 0

targeted tests:
  constitution_real8_market_ab_benchmark: 9/9
  constitution_real10_trace_cleanup: 6/6
  constitution_real10_emergence_metrics: 4/4

Trust Root:
  pass

audit_tape:
  pass
```

REAL-10 arm facts:

| Arm | Condition | market_tx_count | buy_with_coin_router | solve_rate |
|---|---|---:|---:|---:|
| A | market disabled | 0 | 0 | 5/15 |
| B | market visible, no TaskOutcomeMarket | 10 | 0 | 5/15 |
| C | TaskOutcomeMarket enabled | 42 | 0 | 6/15 |
| D | TaskOutcomeMarket + scripted fixture | 38 | 0 | 4/15 |

Interpretation required by architect:

```text
market structure is active on tape.
agents are not yet trading.
buy_with_coin_router = 0 for all arms.
```

## Implementation Adapter For Low-Reasoning Workers

### General Worker Rules

1. Write or identify the failing/guarding test first.
2. Prefer additive modules and reports.
3. Do not modify restricted surfaces.
4. Do not rewrite old evidence.
5. Do not make dashboard counters a source of truth.
6. Every new report must list source evidence paths.
7. Every new "E2" claim must be blocked unless there is a live, non-scripted,
   agent-generated `BuyWithCoinRouterTx` or short-equivalent on ChainTape/CAS.
8. If a test cannot fail on a broken implementation, rewrite the test.

### Atom 0 Implementation Detail

Files:

```text
handover/directives/2026-05-15_REAL10_NARROW_RATIFICATION_REAL11.md
tests/constitution_real11_evidence_hygiene.rs
```

Required content:

```text
REAL-10 is ratified as clean controlled market evidence only.
Only `real8x_market_ab_clean_20260515T141331Z` is conclusion-bearing.
`real8x_market_ab_20260515T134453Z` is contamination/remediation-only.
E1 satisfied for B/C/D.
E2/E3/E4 not achieved.
buy_with_coin_router=0 in all arms.
No live REAL-6B approval.
```

Test assertions:

```text
the ratification file mentions the clean evidence path;
the ratification file marks the contaminated path invalid for conclusions;
all conclusion-bearing REAL-10 / REAL-11 reports and handover outputs treat
  the contaminated path only as invalid / remediation-only / excluded evidence;
no conclusion-bearing report uses the contaminated path in statistics, tables,
  arm summaries, E1/E2/E3/E4 verdicts, or next-step claims;
the contaminated path may appear only if the surrounding paragraph contains
  invalid, contamination, remediation-only, excluded, or not conclusion-bearing
  language;
stale-parent behavioral test artifact is present OR OBS exists.
```

Evidence-hygiene scan scope:

```text
handover/directives/2026-05-15_REAL10_NARROW_RATIFICATION_REAL11.md
handover/reports/REAL10_DECISION_GATE_REPORT.md
handover/reports/REAL10_VERIFICATION_SUMMARY.md
handover/evidence/real8x_market_ab_clean_20260515T141331Z/REAL10_DECISION_GATE_REPORT.md
handover/reports/REAL11_*.md
handover/ai-direct/LATEST.md
handover/tracer_bullets/TB_LOG.tsv
```

Test implementation guidance:

```text
Read every file in the scan scope that exists.
If `real8x_market_ab_20260515T134453Z` appears, inspect the same paragraph or
nearby 2 lines. The occurrence passes only when it is explicitly marked invalid,
contaminated, remediation-only, excluded, or not conclusion-bearing.
Fail if the contaminated path appears in an evidence table, metrics table,
E1/E2/E3/E4 verdict, canonical evidence list, or conclusion-bearing summary.
```

If stale-parent behavioral test is missing, create:

```text
handover/alignment/OBS_REAL11_STALE_PARENT_BEHAVIORAL_TEST_FORWARD.md
```

with reason and command evidence. Do not silently ignore the gap.

### Atom 1 Implementation Detail

Files:

```text
src/runtime/market_tx_category.rs
src/bin/audit_dashboard.rs
src/runtime/audit_assertions.rs
handover/alignment/DECISION_REAL11_MARKET_TX_CATEGORY.md
handover/reports/REAL11_MARKET_TX_CATEGORY_REPORT.md
tests/constitution_real11_market_tx_category.rs
```

If `src/runtime/market_tx_category.rs` can be avoided by putting pure helpers in
an existing non-restricted report module, prefer the smaller edit.

Data model:

```rust
pub enum MarketTxCategory {
    StructuralMarketTx,
    AgentEconomicActionTx,
    ScriptedFixtureTx,
    ResolutionTx,
}

pub struct MarketTxCategoryCounts {
    pub structural_market_tx_count: u64,
    pub agent_economic_action_tx_count: u64,
    pub scripted_fixture_tx_count: u64,
    pub resolution_tx_count: u64,
}
```

Classification rules:

```text
StructuralMarketTx:
  MarketSeedTx
  CpmmPoolTx
  TaskOutcomeMarket setup tx
  market infrastructure tx that creates context but is not an agent buy/sell

ResolutionTx:
  EventResolveTx
  MarketCloseTx / OracleResolveTx if present in fixture reports

ScriptedFixtureTx:
  any tx emitted by REAL-7/REAL-8/REAL-11 scripted positive-control flag;
  any buy/sell emitted without live-agent provenance;
  scripted AttemptPrediction fixture activity

AgentEconomicActionTx:
  BuyWithCoinRouterTx / short-equivalent / live liquidity action only when all
  of the following are true:
    live agent action source is recorded;
    not forced;
    not scripted;
    PromptCapsule or MarketDecisionTrace links the action to the agent turn;
    ChainTape/CAS has the L4 or L4.E anchor;
    audit_tape is PROCEED.
```

Conservative fallback:

```text
If provenance is missing, classify as ScriptedFixtureTx or unknown-non-E2.
Never classify missing-provenance router tx as AgentEconomicActionTx.
```

REAL-10 rerender expectations:

```text
agent_economic_action_tx_count = 0 for A/B/C/D
scripted_fixture_tx_count > 0 only where scripted fixture provenance exists
structural_market_tx_count explains B/C/D market_tx_count increase
```

Test cases:

```text
market_seed + cpmm_pool -> StructuralMarketTx
event_resolve -> ResolutionTx
scripted AttemptPrediction fixture -> ScriptedFixtureTx
BuyWithCoinRouter with scripted provenance -> ScriptedFixtureTx, not E2
BuyWithCoinRouter with live non-scripted provenance -> AgentEconomicActionTx
BuyWithCoinRouter with missing provenance -> not AgentEconomicActionTx
REAL-10 clean report rerenders with agent_economic_action_tx_count=0
dashboard includes all four split counters
```

### Atom 2 Implementation Detail

Files:

```text
scripts/run_real11_router_positive_control.sh
tests/constitution_real11_router_positive_control.rs
handover/reports/REAL11_ROUTER_POSITIVE_CONTROL_REPORT.md
handover/evidence/real11_router_positive_control_*/
```

Allowed implementation strategy:

```text
Use existing Stage C P-M6 / REAL-7 router plumbing.
Use existing `BuyWithCoinRouterTx` and quote helpers.
Use existing `scripted_positive_edge_trade` only as a role-level fixture marker.
Do not edit `src/state/sequencer.rs` or `src/state/typed_tx.rs`.
```

Required tests:

```text
positive control:
  create or load an active market pool;
  construct scripted BuyYesWithCoinRouterTx;
  submit through normal path;
  assert L4 acceptance;
  assert audit_tape/aggregate verdict PROCEED from mandatory evidence run;
  assert CTF conserved;
  assert PnL changes for buyer.

BuyNo/short equivalent:
  construct scripted BuyNo path;
  assert L4 acceptance OR L4.E with explicit class if environment lacks depth;
  no silent skip.

insufficient balance:
  construct router tx above balance;
  assert L4.E explicit class, preferably RouterInsufficientCoinBalance or mapped class;
  no L4 acceptance.

missing pool:
  construct route against absent EventId/pool;
  assert L4.E or pre-submit NoTradeReason::NoPool as designed;
  no silent skip.

conservation:
  assert no ghost liquidity;
  assert total CTF conserved;
  assert complete set balance invariant if shares involved.

no f64:
  grep source paths touched by Atom 2 for f64/f32 in money path;
  integer micro-units and u128 math only.
```

Mandatory evidence run:

```text
Atom 2 cannot close on unit tests alone.
It must create `handover/evidence/real11_router_positive_control_<UTC>/`.
It must run the scripted router path through the normal ChainTape runtime.
It must run `audit_tape` or the repo's existing aggregate-verdict path over the
runtime repo.
It must record aggregate verdict `PROCEED`.
It must write `REAL11_ROUTER_POSITIVE_CONTROL_REPORT.md`.
```

Recommended command shape:

```bash
bash scripts/run_real11_router_positive_control.sh real11_router_positive_control_<UTC>
```

Evidence directory must include:

```text
runtime_repo path or pointer
CAS path or pointer
aggregate_verdict.json or audit_tape verdict file
audit_dashboard_run_report.txt or equivalent dashboard materialized view
router_positive_control_manifest.json
REAL11_ROUTER_POSITIVE_CONTROL_REPORT.md
```

Router positive-control manifest must include:

```text
scripted_fixture=true
attempt_prediction_fixture_count=0
live_real6b_enabled=false
buy_yes_l4_accepted=true
buy_no_or_short_l4_or_l4e_explicit=true
insufficient_balance_l4e_explicit=true
missing_pool_l4e_or_no_trade_explicit=true
ctf_conserved=true
no_ghost_liquidity=true
no_f64_money_path=true
pnl_updated=true
audit_verdict=PROCEED
```

Report gate:

```text
REAL11_ROUTER_POSITIVE_CONTROL_REPORT.md must state:
  This is scripted positive control.
  It proves router wire path only.
  It does not count as E2.
```

Evidence report must state:

```text
This is scripted positive control.
It proves router wire path, not E2.
```

### Atom 3 Implementation Detail

Files:

```text
src/runtime/market_opportunity_trace.rs
src/runtime/market_decision_trace_summary.rs
experiments/minif2f_v4/src/bin/evaluator.rs
src/bin/audit_dashboard.rs
tests/constitution_real11_market_opportunity_trace.rs
handover/reports/REAL11_MARKET_OPPORTUNITY_REPORT.md
```

Schema adapter:

```rust
pub struct MarketOpportunityTrace {
    pub schema_version: String, // "real11.market_opportunity_trace.v1"
    pub agent_id: AgentId,
    pub role: AgentRole,
    pub task_id: TaskId,
    pub head_t: HeadT,
    pub visible_markets: Vec<EventId>,
    pub actionable_markets: Vec<EventId>,
    pub available_balance: MicroCoin,
    pub router_available: bool,
    pub reason_if_no_actionable_market: Option<NoTradeReason>,
    pub prompt_capsule_cid: Option<Cid>,
}
```

CAS/ObjectType rule:

```text
Do not add a new `ObjectType` in REAL-11 unless separately ratified.
If storing JSON in the existing AttemptTelemetry-compatible CAS slot, add an
explicit schema classifier like the MarketDecisionTrace classifier:
  recognized MarketOpportunityTrace JSON may be skipped by bulk AttemptTelemetry walker;
  unknown JSON must fail closed;
  unknown JSON must not silently skip.
If that classifier cannot be implemented without risky surface edits, store the
trace under an existing evidence/report CAS path and file OBS for future split.
```

Derivation rule:

```text
visible_markets/actionable_markets must be derived from Q snapshot + ChainTape/CAS
market state at the turn head, not from stdout strings.
available_balance and risk state come from ChainTape-derived economic state.
router_available means the invest tool and route construction path are available.
```

NoTrade coupling:

```text
If actionable_markets = 0:
  NoTradeReason must be specific: NoPool, AmountExceedsBalance, PromptBudgetExceeded,
  TooFastSolve, RouterRejected, or another stable taxonomy reason.
  Generic Unknown is allowed only with an OBS and must block ship if dominant.

If actionable_markets > 0 and no trade:
  record NoPerceivedEdge, AgentDeclined, RiskCap/Balance mapped to existing taxonomy,
  or PromptBudgetExceeded.
```

If `NoTradeReason` lacks `RiskCap` / `Balance` exact variants:

```text
Use existing `AmountExceedsBalance` for balance shortfall.
Use `RouterRejected` plus public summary for risk-cap/router admission rejection.
Do not append enum variants unless tests and dashboard column-order compatibility are updated.
```

Test cases:

```text
Trader turn with active pool and balance -> actionable_markets > 0.
Trader turn with no pool -> actionable_markets = 0 and reason NoPool.
Trader turn with low/zero balance -> specific balance/risk reason, not Unknown.
Trader turn with prompt budget hiding market -> PromptBudgetExceeded.
Every Trader turn in a fixture has exactly one MarketOpportunityTrace.
Trace includes prompt_capsule_cid when PromptCapsuleV2 exists.
Trace contains no raw prompt, raw completion, private CoT, or raw logs.
Trace summary in dashboard derives from CAS/ChainTape.
Unknown JSON classifier fails closed if shared slot is used.
```

### Atom 4 Implementation Detail

Files:

```text
src/runtime/agent_pnl.rs
src/runtime/real5_roles.rs
src/runtime/prompt_capsule.rs
experiments/minif2f_v4/src/bin/evaluator.rs
src/sdk/prompt.rs
tests/constitution_real11_trader_pnl_visibility.rs
handover/reports/REAL11_PNL_INCENTIVE_VISIBILITY_REPORT.md
```

Prefer tests over source edits if current code already satisfies the gate.

Required visible TraderView fields:

```text
available balance
open positions
realized PnL
unrealized PnL
risk cap
bankruptcy/autopsy summary
```

Derivation:

```text
balance / open positions / PnL / risk cap must come from ChainTape/Q fold and
existing `compute_agent_pnl` / risk-cap helpers, not a HashMap sidecar.
PromptCapsuleV2 read_set or visible_context_cid must link to the view bytes.
No raw prompt body is public by default.
```

Risk behavior:

```text
below risk cap:
  cannot execute risky Trader/Challenger market action;
  can still observe/read/abstain/solve/possibly verify.
```

Test cases:

```text
Trader PromptCapsule visible context includes balance/PnL/risk labels.
PromptCapsule read_set resolves the PnL summary CID.
PnL values match `compute_agent_pnl` on a constructed Q state.
No PnL HashMap sidecar is used as source of truth.
Low-balance status appears in TraderView as signal.
Low-balance risky action is blocked by existing risk/admission path or classified before submit.
Read-only view does not debit Coin.
No raw CoT/raw logs/raw Lean stderr in TraderView.
```

### Atom 5 Implementation Detail

Files:

```text
scripts/run_real11_e2_micro_probe.sh
tests/constitution_real11_e2_micro_probe.rs
tests/constitution_real11_no_live_real6b.rs
handover/evidence/real11_e2_micro_probe_<UTC>/
handover/reports/REAL11_E2_MICRO_PROBE_REPORT.md
```

Micro-probe constraints:

```text
3-5 tasks
same pinned config across tasks
market enabled
TaskOutcomeMarket enabled
TraderView with PnL enabled
MarketOpportunityTrace enabled
No live REAL-6B
No scripted buys in the live probe arm
No forced trade
No price-as-truth
```

Hard live-REAL-6B fail-closed gate:

```text
REAL-11 Atom 5 must fail before running and fail in post-run validation if any
live AttemptPrediction / REAL-6B path is enabled or observed.
```

Forbidden live/surrogate sentinels for Atom 5:

```text
env/config:
  TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION=1
  TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION=true
  TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE=1
  TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE=true

manifest/report/dashboard:
  ARM_CONDITION containing AttemptPrediction
  any config manifest containing TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION=1/true
  any config manifest containing TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE=1/true
  audit_dashboard_run_report scripted_attempt_prediction_market_count > 0

CAS/schema:
  schema_id == "real6b.attempt_prediction_fixture.v1"
  runtime::real6_attempt_prediction::REAL6B_SCHEMA_ID object count > 0

tx/report labels:
  SubmitCandidateTx / AttemptPredictionMarket / MarketCloseTx / OracleResolveTx
  appearing in micro-probe evidence as active live path
```

Allowed REAL-11 distinction:

```text
Atom 2 may use a deterministic scripted router positive-control fixture for
BuyWithCoinRouter only. Atom 2 still must not use AttemptPrediction.
Atom 5 micro-probe must use no scripted buys and no AttemptPrediction fixture.
```

Required test assertions:

```text
scripts/run_real11_e2_micro_probe.sh rejects
  TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION=1;
scripts/run_real11_e2_micro_probe.sh rejects
  TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE=1;
micro-probe config manifests contain no forbidden sentinel enabled;
micro-probe dashboard/report has scripted_attempt_prediction_market_count == 0;
micro-probe CAS metadata has no `real6b.attempt_prediction_fixture.v1` object;
the final REAL11_E2_MICRO_PROBE_REPORT records `live_real6b_enabled=false`;
any violation blocks E2 verdict and blocks REAL-11 close.
```

Recommended command shape:

```bash
TURINGOS_REAL6_TASK_OUTCOME_MARKET=1 \
TURINGOS_REAL5_ROLE_VIEWS=1 \
TURINGOS_REAL11_MARKET_OPPORTUNITY_TRACE=1 \
TURINGOS_REAL11_TRADER_PNL_VIEW=1 \
TURINGOS_REAL11_NO_SCRIPTED_BUYS=1 \
TURINGOS_REAL11_E2_MICRO_PROBE=1 \
bash scripts/run_real11_e2_micro_probe.sh real11_e2_micro_probe_<UTC>
```

If environment uses `run_g_phase_batch.sh`, wrap it in the REAL-11 script and
pin problem/model/budget/config manifests in the evidence directory.

Micro-probe report must include:

```text
run tag / runtime_repo / CAS path
problem list and hash
model assignment hash
budget/config hash
audit_tape verdict
Trader turn count
MarketOpportunityTrace count
NoTradeReason distribution
buy_with_coin_router count
live_non_scripted_router_tx_count
scripted_fixture_tx_count
agent_economic_action_tx_count
live_real6b_enabled=false
attempt_prediction_fixture_count=0
E2 verdict
decision branch A/B/C/D
```

E2 verdict rule:

```text
E2 achieved only if live_non_scripted_router_tx_count >= 1
and every qualifying tx has ChainTape/CAS anchor + PromptCapsule/trace provenance
and audit_tape PROCEED
and no forced/scripted flag.
```

No-buy rule:

```text
If buy_with_coin_router = 0, every no-buy Trader turn must have
MarketOpportunityTrace + NoTradeReason. Missing trace is a blocker.
```

### Atom 6 Implementation Detail

Files:

```text
handover/reports/REAL11_DECISION_GATE_REPORT.md
handover/alignment/REAL11_TRACE_MATRIX_UPDATE.md
handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md
handover/alignment/TRACE_FLOWCHART_MATRIX.md
handover/ai-direct/LATEST.md
handover/tracer_bullets/TB_LOG.tsv
tests/constitution_real11_matrix_update.rs
```

Update `LATEST.md` / `TB_LOG.tsv` only after implementation gates and audits
are complete. The decision report comes first.

Matrix update requirement:

```text
REAL-11 ship must update alignment metadata before close.
```

Implementation:

```text
1. Add or update a `CONSTITUTION_EXECUTION_MATRIX.md` row in the G/REAL section
   for REAL-11 Agent Economic Action Activation.
2. The row must cite the concrete REAL-11 tests:
   constitution_real11_evidence_hygiene
   constitution_real11_market_tx_category
   constitution_real11_router_positive_control
   constitution_real11_market_opportunity_trace
   constitution_real11_trader_pnl_visibility
   constitution_real11_e2_micro_probe
   constitution_real11_claim_boundary
   constitution_real11_no_live_real6b
   constitution_real11_matrix_update
3. The row must cite the canonical evidence directory and decision report after
   Atom 5/6 run.
4. The row must state the claim boundary: E2 achieved only if live non-scripted
   router/short action occurred; otherwise REAL-11 is a clean diagnostic.
5. Update `TRACE_FLOWCHART_MATRIX.md` only if REAL-11 changes FC enforcement.
   If no FC enforcement changes, create `REAL11_TRACE_MATRIX_UPDATE.md` saying
   no flowchart node/hash changed and list touched FC nodes as evidence/report
   surfaces. Do not edit flowchart hashes.
```

Matrix tests:

```text
constitution_real11_matrix_update asserts the execution matrix references
REAL-11 and every required REAL-11 test name.
If `TRACE_FLOWCHART_MATRIX.md` is unchanged, the test asserts
REAL11_TRACE_MATRIX_UPDATE.md exists and says no FC enforcement/hash change.
If TRACE_FLOWCHART_MATRIX is changed, the test asserts the changed rows mention
REAL-11 and do not alter canonical FC hashes without Class-4 ratification.
```

Decision report template:

```text
# REAL-11 Decision Gate Report

## Evidence
- canonical REAL-10 clean evidence
- REAL-11 router positive-control evidence
- REAL-11 micro-probe evidence
- targeted tests
- constitution gates
- audit verdicts

## E2 Verdict
ACHIEVED / NOT ACHIEVED

## Branch
A live buy -> REAL-12 role differentiation / E3
B opportunity exists but abstain -> Trader objective redesign
C no actionable opportunity -> prepare live REAL-6B Class-4 packet
D router fixture fails -> substrate fix

## Forbidden Claims
...
```

If branch C is selected:

```text
Do not implement live REAL-6B in REAL-11.
Create a future Class-4 packet only.
```

## Verification Matrix

Targeted tests:

```bash
cargo test --test constitution_real11_evidence_hygiene
cargo test --test constitution_real11_market_tx_category
cargo test --test constitution_real11_router_positive_control
cargo test --test constitution_real11_market_opportunity_trace
cargo test --test constitution_real11_trader_pnl_visibility
cargo test --test constitution_real11_e2_micro_probe
cargo test --test constitution_real11_claim_boundary
cargo test --test constitution_real11_no_live_real6b
cargo test --test constitution_real11_matrix_update
```

Existing relevant regressions:

```bash
cargo test --test constitution_real8_market_ab_benchmark
cargo test --test constitution_real10_trace_cleanup
cargo test --test constitution_real10_emergence_metrics
cargo test --test constitution_g3_pnl
cargo test --test constitution_tb_n3_invest_routing
cargo test --test constitution_polymarket_smoke
cargo test --test constitution_real5_prompt_capsule_v2
```

Broad checks:

```bash
bash scripts/run_constitution_gates.sh
cargo test --workspace --no-fail-fast -- --test-threads=1
```

Evidence checks:

```bash
bash scripts/run_real11_router_positive_control.sh real11_router_positive_control_<UTC>
bash scripts/run_real11_e2_micro_probe.sh real11_e2_micro_probe_<UTC>
cargo run --bin audit_tape -- <REAL11_RUNTIME_REPO_OR_ARGS>
cargo run --bin audit_dashboard -- --run-report <REAL11_RUNTIME_REPO_OR_ARGS>
```

If exact `audit_tape`/`audit_dashboard` invocation differs in the repo, use the
existing REAL-10/REAL-8X evidence command pattern and record the exact command.

Router positive-control evidence is mandatory:

```text
The `real11_router_positive_control_<UTC>` evidence directory must contain an
aggregate/audit verdict of PROCEED. Unit tests alone do not satisfy SG-11.2.
```

Overclaim tests must fail if any REAL-11 report claims:

```text
E2 without live non-scripted router/short tx
E3 from role labels
E4 from small-n descriptive data
market_tx_count increase as emergence
scripted positive-control as E2
live REAL-6B approval
market-caused solve improvement without E4 evidence
causal performance improvement without statistical support
model ranking
autonomous secondary market alive
autonomous prediction market
emergent agent economy
market-proven performance improvement
real-world readiness
agent economy beta without E2 at least once under live non-scripted condition
emergent market beta without E3 persistent role differentiation
price-as-truth
forced trade
ghost liquidity
f64 economy
off-tape WAL truth
private CoT recording
raw-log broadcast
dashboard as source of truth
```

## Audit Payloads

### Audit 1 — Plan Alignment

Reviewer input:

```text
task brief: REAL-11 plan alignment
risk class: planning Class 0, implementation default Class 3
architect original:
  handover/directives/2026-05-15_REAL11_AGENT_ECONOMIC_ACTION_ACTIVATION_ARCHITECT_ORIGINAL.md
plan:
  handover/directives/2026-05-15_REAL11_AGENT_ECONOMIC_ACTION_ACTIVATION_EXECUTION_PLAN.md
user requirement:
  extract all details, implementation details, verification requirements;
  plan must be detailed enough for low-reasoning coding agents;
  tests must catch errors;
  final plan must be audited against architect original and user request
required verdict:
  PROCEED | CHALLENGE | VETO
```

Output:

```text
handover/audits/CODEX_REAL11_EXECUTION_PLAN_ALIGNMENT_REVIEW.md
```

### Audit 2 — Atom 1-2 Review

Review:

```text
MarketTxCategory split correctness
E2 non-overclaim
router positive-control path
CTF/no ghost liquidity/no f64
no restricted surface edits
```

Output:

```text
handover/audits/CODEX_REAL11_ATOM1_2_REVIEW.md
```

### Audit 3 — Opportunity / PnL / Probe Review

Review:

```text
MarketOpportunityTrace completeness
Trader PromptCapsule PnL visibility
NoTradeReason coupling
micro-probe evidence and E2 classification
no live REAL-6B
no AttemptPrediction fixture sentinel in Atom 5 evidence/config/CAS/dashboard
```

Output:

```text
handover/audits/CODEX_REAL11_OPPORTUNITY_PNL_PROBE_REVIEW.md
```

### Audit 4 — Final Review

Review:

```text
all gates/evidence
decision branch
claim boundary
handover updates
restricted surfaces
```

Output:

```text
handover/audits/CODEX_REAL11_FINAL_IMPLEMENTATION_REVIEW.md
```

## Done Definition

REAL-11 implementation is not complete until:

```text
SG-11.0.* through SG-11.5.* have evidence or explicit forward OBS where allowed.
Atom 6 decision gate report exists.
Execution Matrix REAL-11 row exists and cites all REAL-11 tests/evidence.
Trace matrix is updated if FC enforcement changed, or
  REAL11_TRACE_MATRIX_UPDATE.md explicitly records no FC/hash change.
Targeted tests pass.
Constitution gates pass.
Workspace tests pass or any failure is proven unrelated and accepted by audit.
Router positive-control evidence exists with aggregate/audit verdict PROCEED.
Micro-probe evidence exists.
Micro-probe has live_real6b_enabled=false and attempt_prediction_fixture_count=0.
Clean-context Codex audit returns PROCEED.
No forbidden claim appears in reports/handover.
No restricted surface was touched without Class-4 ratification.
```

REAL-11 may close with E2 not achieved if it produces a clean branch-B/C/D
diagnosis. That is not failure; it is the intended scientific narrowing.
