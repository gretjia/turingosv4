# REAL-13A Expected-Value Scaffolding Execution Plan

> Superseded terminology note (2026-05-16): the worker-facing REAL-13 package
> uses `EVDecisionTrace` as the canonical schema name. Earlier
> `ExpectedValueDecomposition` wording in this file is historical planning
> context only. Implementers must follow
> `2026-05-16_REAL13_MARKET_PRESSURE_LOOP_EXECUTION_PLAN.md`.

## Summary

REAL-13A follows directly from the REAL-12 decision gate:

```text
actionable markets but no live action -> REAL-13A expected-value scaffolding
```

The canonical REAL-12 micro-probe produced:

```text
MarketOpportunityTrace count = 4
economic_judgment_total = 4
bull_judgment_count = 2
bear_judgment_count = 2
abstain_reason_distribution = {"NoPerceivedEdge": 4}
buy_with_coin_router = 0
agent_economic_action_tx_count = 0
live_non_scripted_router_tx_count = 0
audit_tape = PROCEED
E2 NOT ACHIEVED
```

REAL-13A therefore does not add a new market mechanism. It makes Bull/Bear
probability and expected-value reasoning explicit, public, typed, and
CAS/ChainTape reconstructable while preserving:

```text
no forced trade
price as signal only
no live REAL-6B
no scripted action counted as E2
no E2/E3/E4 overclaim
```

## Orchestration

All agents use GPT-5.5; only reasoning depth changes.

| Role | Model / depth | Responsibility |
| --- | --- | --- |
| Orchestrator | GPT-5.5 high, xhigh for risk calls | Truth order, Class-4 boundary, FC mapping, audit scheduling, final claim boundary. |
| Docs Worker | GPT-5.5 low | Mechanical provenance, ratification, report templates, forbidden-claim text. No semantic reinterpretation. |
| EV Schema/View Worker | GPT-5.5 medium | Additive expected-value structs, CAS sidecar shape, Bull/Bear EV views, report counters. |
| Gateway/Judgment Worker | GPT-5.5 high | Role action gateway, EV validation, buy/short gating, L4/L4.E behavior. |
| Probe Worker | GPT-5.5 medium | Scripted positive controls and live micro-probe runner. |
| Analysis Worker | GPT-5.5 high | E2/E3/E4 decision gate, reason distribution, final evidence interpretation. |
| Audit Worker | GPT-5.5 xhigh | Clean-context audit only; verdict `PROCEED | CHALLENGE | VETO`. |

Audit points:

1. Plan alignment before implementation.
2. After Atoms 1-4, because schema/view/gateway/reporting define the scaffold.
3. After Atom 6 live micro-probe evidence, because interpretation risk is highest.
4. Final implementation review before ship.

Do not require a separate audit after every atom unless a worker touches a
restricted surface, changes Trust Root, or tries to escalate an E2/E3/E4 claim.

## Risk, FC Mapping, And Stop Conditions

Default implementation risk:

```text
Class 3 if additive runtime/evaluator/report/probe code only.
Class 4 if Trust Root-pinned files, PromptCapsule authority, genesis authority,
or any replay-authority surface is modified.
```

FC mapping:

```text
FC1: role action loop, EconomicJudgment -> typed action/abstain -> L4/L4.E/CAS
FC2: PromptCapsule/view replay; role assignment and read set reconstructability
FC3: dashboard/audit/report as materialized views derived from ChainTape/CAS
Art. II: broadcast price/PnL/EV signals without making price truth
Art. III: shield raw CoT, raw logs, private diagnostics, and dashboard-only facts
Economy: integer-only money and conservation gates
```

Stop immediately if implementation requires:

```text
sequencer admission changes
TypedTx schema/discriminant changes
canonical signing payload changes
wallet backend changes
kernel.rs or bus.rs changes
CAS ObjectType schema changes
live REAL-6B
forced trade as E2 evidence
```

## Phase 0 — Provenance And Harness

Create or preserve:

```text
handover/directives/2026-05-16_REAL13A_EXPECTED_VALUE_SCAFFOLDING_ARCHITECT_SOURCE.md
handover/directives/2026-05-16_REAL13A_EXPECTED_VALUE_SCAFFOLDING_EXECUTION_PLAN.md
handover/audits/CODEX_REAL13A_EXECUTION_PLAN_ALIGNMENT_REVIEW.md
```

Open `turingos_dev` for implementation only after user approval:

```bash
turingos_dev open \
  --title "REAL-13A Expected-Value Scaffolding" \
  --module "REAL-13A Expected-Value Scaffolding" \
  --risk 3 \
  --fc "FC1 role action loop,FC2 PromptCapsule replay,FC3 dashboard/audit materialized view,Art II broadcast,Art III shielding" \
  --unit atom \
  --intent "Make Bull/Bear expected-value reasoning explicit and reconstructable without forced trades or live REAL-6B."
```

If the implementation touches Trust Root or replay-authority surfaces, re-open
or upgrade the run as Class 4 with explicit atom ratification.

## Atom 0 — REAL-12 To REAL-13A Branch Ratification

Create:

```text
handover/directives/2026-05-16_REAL12_TO_REAL13A_BRANCH_RATIFICATION.md
```

Must state:

```text
REAL-12 canonical evidence path:
  handover/evidence/real12_role_specialized_micro_probe_20260516T023351Z/

REAL-12 proved:
  role-specialized EconomicJudgment exists
  Bull/Bear turns are covered
  actionable market context exists
  audit_tape PROCEED

REAL-12 did not prove:
  E2 live non-scripted economic action
  E3 persistent role differentiation
  E4 causal performance signal

Branch reason:
  actionable markets exist, but all Bull/Bear judgments abstained as
  NoPerceivedEdge; proceed to REAL-13A expected-value scaffolding.
```

Targeted test:

```bash
cargo test --test constitution_real13a_claim_boundary
```

The test must fail if any report or ratification claims:

```text
spontaneous emergence
causal improvement
model ranking
live REAL-6B approval
scripted buy counted as E2
price-as-truth
forced trade
```

## Atom 1 — ExpectedValueDecomposition Schema

Add an additive CAS-backed schema. Do not modify TypedTx. Do not modify CAS
ObjectType discriminants. Prefer a JSON/CAS sidecar with an explicit
`schema_version`.

Suggested shape:

```rust
pub struct ExpectedValueDecomposition {
    pub schema_version: String, // "real13a.expected_value_decomposition.v1"
    pub agent_id: AgentId,
    pub role: AgentRole, // BullTrader or BearTrader
    pub task_id: TaskId,
    pub event_id: EventId,
    pub side: YesNo,
    pub observed_price: RationalPrice,
    pub implied_probability_bps: i64,
    pub estimated_probability_band_bps: ProbabilityBandBps,
    pub expected_value_sign: ExpectedValueSign,
    pub edge_bps: Option<i64>,
    pub max_risk_micro: MicroCoin,
    pub recommended_amount_micro: Option<MicroCoin>,
    pub liquidity_depth_micro: MicroCoin,
    pub deadline_or_oracle_risk: OracleOrDeadlineRisk,
    pub uncertainty_reason: Option<EconomicReason>,
    pub source_view_cid: Cid,
    pub prompt_capsule_cid: Cid,
    pub public_summary: String,
}
```

Supporting enums:

```rust
pub enum ExpectedValueSign {
    Positive,
    Neutral,
    Negative,
    Unknown,
}

pub struct ProbabilityBandBps {
    pub low_bps: i64,
    pub high_bps: i64,
}

pub enum OracleOrDeadlineRisk {
    Low,
    Medium,
    High,
    Unknown,
}
```

Validation rules:

```text
No f64/f32.
Probability bands are integer bps in [0, 10000].
implied_probability_bps is integer bps in [0, 10000].
low_bps <= high_bps.
observed_price is rational/integer-derived.
edge_bps is integer bps.
public_summary must not contain raw prompt, raw CoT, raw Lean stderr, or raw logs.
source_view_cid and prompt_capsule_cid must resolve.
BullTrader side must be YES.
BearTrader side must be NO unless future ratification explicitly allows another short-equivalent.
```

Targeted test:

```bash
cargo test --test constitution_real13a_ev_schema
```

The test must include negative fixtures:

```text
float probability rejected
out-of-range bps rejected
low_bps > high_bps rejected
missing observed_price rejected
missing prompt_capsule_cid rejected
raw CoT / raw log markers rejected
BullTrader NO side rejected
BearTrader YES side rejected
```

## Atom 2 — Bull/Bear EV-Scoped Views

Extend derived views so EV reasoning uses public, scoped, replayable facts.
Do not widen Solver into a full market dashboard.

BullTrader EV view includes:

```text
YES price
TaskOutcome YES market
NodeSurvive YES market if present
observed market depth
available balance
realized/unrealized PnL
risk cap
deadline/budget remaining
verification status
challenge status
recent accepted WorkTx summary
```

BearTrader EV view includes:

```text
NO price
short-equivalent route if supported
unsolved-task risk
candidate weakness signals
failed attempt summaries
challenge status
observed market depth
available balance
realized/unrealized PnL
risk cap
deadline/budget remaining
```

Solver view remains limited:

```text
Lean goal
local proof context
local public error summaries
minimal market summary only
no full market dashboard
```

PromptCapsule requirements:

```text
stores EV view hash
stores read_set containing the EV view/source CIDs
does not store raw prompt body by default
does not include raw logs, private CoT, raw diagnostics, or dashboard-only facts
```

Targeted test:

```bash
cargo test --test constitution_real13a_ev_views
```

The test must prove:

```text
Bull view broadcasts long-side EV signals.
Bear view broadcasts short-side EV signals.
Solver view excludes full market dashboard noise.
PromptCapsule read_set resolves.
Views derive from ChainTape/CAS/QState, not stdout/dashboard counters.
Forbidden raw diagnostics do not enter prompts.
```

## Atom 3 — EV-Aware Judgment Gateway

Extend EconomicJudgment so each Bull/Bear turn has either:

```text
EconomicJudgment + ExpectedValueDecomposition
```

or a structured abstain reason explaining why EV cannot be estimated.

Rules:

```text
Buy/Short requires ExpectedValueSign::Positive.
Buy/Short requires chosen_market, intended_side, intended_amount, observed_price,
and prompt_capsule_cid.
Buy/Short requires amount <= max_risk_micro and balance/risk-cap permits it.
Positive EV is necessary for router submission but never sufficient to override
sequencer, predicates, risk cap, liquidity, or audit gates.
Neutral/Negative/Unknown EV cannot route to Buy/Short.
Missing EV basis must become Abstain with a structured reason.
Abstain is always allowed, but must carry EconomicReason.
No private CoT.
```

Allowed abstain reasons:

```text
NoPerceivedEdge
NoActionableMarket
InsufficientBalance
RiskCapExceeded
LiquidityTooLow
ExpectedValueNegative
PromptBudgetExceeded
UnresolvedOracleRisk
RolePolicyBlocked
Unknown
```

`Unknown` policy:

```text
Unknown is allowed only as a fail-closed diagnostic.
REAL-13A ship evidence must report Unknown count.
Unknown > 0 is CHALLENGE unless explicitly explained in a forward OBS.
```

Targeted test:

```bash
cargo test --test constitution_real13a_ev_gateway
```

The test must prove:

```text
positive EV with valid risk/balance may route to router candidate
negative EV routes to Abstain
unknown EV routes to Abstain
missing EV basis routes to Abstain
positive EV never changes Lean predicate
BullTrader cannot choose NO
BearTrader cannot choose YES
Solver cannot submit router action
illegal role action routes L4.E PolicyViolation or RoleActionNotAllowed
```

## Atom 4 — EV Reporting And Dashboard Materialized View

Add report/dashboard fields derived from ChainTape/CAS, not stdout-only
counters:

```text
ev_decomposition_count
bull_ev_decomposition_count
bear_ev_decomposition_count
positive_ev_count
neutral_ev_count
negative_ev_count
unknown_ev_count
missing_ev_basis_count
router_blocked_missing_ev_basis_count
router_blocked_risk_cap_count
router_blocked_liquidity_count
ev_abstain_reason_distribution
live_non_scripted_router_tx_count
buy_yes_router_count
buy_no_router_count
```

Classification:

```text
Structural market tx != AgentEconomicActionTx.
Scripted fixture tx != E2.
Live non-scripted router tx is E2 candidate only after audit.
EV decomposition alone is not E2.
Positive EV alone is not E2.
```

Targeted test:

```bash
cargo test --test constitution_real13a_ev_reporting
```

The test must fail if:

```text
dashboard/report treats EV as source of truth
stdout-only counters become conclusion-bearing
scripted tx counted as E2
positive EV counted as E2 without router tx
price treated as predicate truth
```

## Atom 5 — Scripted EV Positive-Control

Run deterministic scripted controls. They prove the wire, not E2.

Required fixtures:

```text
BullTrader scripted positive EV -> BuyYesWithCoinRouterTx -> L4 accepted.
BearTrader scripted positive EV -> BuyNoWithCoinRouterTx -> L4 accepted.
BullTrader negative EV -> Abstain, no router tx.
BearTrader unknown EV -> Abstain, no router tx.
Insufficient balance positive EV -> L4.E or blocked trace with explicit class.
Liquidity too low positive EV -> L4.E or blocked trace with explicit class.
```

Acceptance:

```text
CTF conserved.
No ghost liquidity.
No f64/f32 money path.
PnL/positions derive from ChainTape/CAS.
audit_tape PROCEED.
Scripted positive-control is marked scripted-not-E2.
```

Targeted test:

```bash
cargo test --test constitution_real13a_ev_positive_control
```

## Atom 6 — Live EV Micro-Probe

Run a 3-5 MiniF2F task probe:

```text
roles:
  Solver
  BullTrader
  BearTrader
  Verifier
  Challenger

market enabled
TaskOutcomeMarket enabled
role views enabled
EV scaffolding enabled
no forced trade
live REAL-6B disabled
no scripted buys
price as signal only
```

Suggested environment gate:

```bash
TURINGOS_REAL13A_EV_SCAFFOLDING=1
TURINGOS_REAL12_TASK_MARKET_AFFORDANCE=1
TURINGOS_REAL12_TRADER_OBJECTIVE=1
TURINGOS_REAL6B_LIVE=0
```

Runner must record:

```text
bull_ev_judgment_count
bear_ev_judgment_count
ev_decomposition_count
positive_ev_count
neutral_ev_count
negative_ev_count
unknown_ev_count
missing_ev_basis_count
router_blocked_missing_ev_basis_count
router_blocked_risk_cap_count
router_blocked_liquidity_count
buy_yes_router_count
buy_no_router_count
live_non_scripted_router_tx_count
abstain_reason_distribution
ev_reason_distribution
audit_tape verdict
```

Acceptance:

```text
Every Bull/Bear turn has EconomicJudgment.
Every Bull/Bear turn has ExpectedValueDecomposition or structured EV abstain.
At least one Trader sees actionable market.
No forced trade.
No price-as-truth.
No live REAL-6B.
No scripted buys.
audit_tape PROCEED.
```

Targeted test:

```bash
cargo test --test constitution_real13a_live_ev_micro_probe
```

Interpretation:

```text
If live non-scripted buy/short appears:
  E2 candidate, pending clean-context audit. Do not auto-claim.

If no live action and positive_ev_count = 0:
  Models still do not infer positive edge under current market/event structure.

If positive_ev_count > 0 but no live action:
  Classify blocker as risk cap, balance, liquidity, policy, or parser/gateway.

If no actionable market:
  Branch to REAL-13B event timing / live REAL-6B Class-4 packet.
```

## Atom 7 — Decision Gate

Write:

```text
handover/reports/REAL13A_EXPECTED_VALUE_SCAFFOLDING_DECISION_GATE_REPORT.md
```

Decision branches:

```text
A. Live non-scripted BuyYes/BuyNo/short appears
   -> E2 candidate only, pending clean-context audit; proceed to REAL-13
      persistent role differentiation only after audit.

B. No live action and no positive EV
   -> EV scaffolding made the reason explicit; do not force trade. Consider
      objective wording or event-timing redesign, but no E2 claim.

C. Positive EV appears but router is blocked by risk/balance/liquidity
   -> Adjust budget/risk/liquidity only with explicit architect approval.

D. Positive EV appears but parser/gateway blocks unexpectedly
   -> Fix gateway/reporting bug before more experiments.

E. No actionable market appears
   -> Prepare REAL-13B / live REAL-6B Class-4 packet.

F. Scripted EV positive-control fails
   -> Return to substrate fix.
```

Report must include:

```text
evidence paths
exact command outputs
audit_tape verdict
EV count table
abstain reason table
router action table
E2/E3/E4 claim boundary
next branch recommendation
```

## Verification

Targeted tests:

```bash
cargo test --test constitution_real13a_claim_boundary
cargo test --test constitution_real13a_ev_schema
cargo test --test constitution_real13a_ev_views
cargo test --test constitution_real13a_ev_gateway
cargo test --test constitution_real13a_ev_reporting
cargo test --test constitution_real13a_ev_positive_control
cargo test --test constitution_real13a_live_ev_micro_probe
```

Ship-level checks:

```bash
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
bash scripts/run_constitution_gates.sh
cargo test --workspace --no-fail-fast -- --test-threads=1
```

Evidence checks:

```text
audit_tape PROCEED over scripted positive-control evidence
audit_tape PROCEED over live EV micro-probe evidence
all reported facts regenerate from ChainTape/CAS
turingos_dev validate
turingos_dev close
clean-context Codex implementation review = PROCEED
```

Overclaim tests must fail on:

```text
autonomous emergence
causal improvement
model ranking
real-world readiness
live REAL-6B approval
price-as-truth
forced trade
ghost liquidity
off-tape WAL truth
private CoT recording
raw-log broadcast
scripted tx counted as E2
positive EV counted as E2 without router tx
```

## Forbidden

```text
No forced trade as E2 evidence.
No price-as-truth.
No ghost liquidity.
No f64/f32 money or probability paths.
No off-tape WAL as truth.
No private CoT recording.
No raw-log broadcast.
No dashboard/report/stdout as source of truth.
No live REAL-6B in REAL-13A.
No sequencer admission changes.
No TypedTx schema/discriminant changes.
No canonical signing payload changes.
No wallet backend changes.
No kernel.rs or bus.rs changes.
No CAS ObjectType schema changes.
No E2 claim unless live non-scripted router/short tx exists and audit passes.
No E3 claim unless persistent behavioral role differentiation exists.
No E4 claim unless statistical support exists.
```

## Route After REAL-13A

```text
REAL-12:
  completed; role-specialized EconomicJudgment exists; E2 absent.

REAL-13A:
  expected-value scaffolding; make Bull/Bear EV reasoning explicit.

If E2 candidate appears:
  REAL-13: persistent role differentiation / E3.

If EV remains absent or negative:
  refine objective scaffolding or prepare event timing redesign without E2 claim.

If no actionable market window:
  REAL-13B: event timing / live REAL-6B Class-4 packet.

REAL-14:
  larger A/B benchmark with E2/E3-specific arms.

REAL-15:
  whitepaper / beta launch synthesis.
```
