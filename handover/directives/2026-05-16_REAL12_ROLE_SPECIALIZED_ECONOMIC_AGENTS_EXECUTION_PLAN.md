# REAL-12 / REAL-13 Execution Plan — Role-Specialized Economic Agents

Date: 2026-05-16 UTC

## Summary

REAL-12 implements role-specialized economic agents after REAL-10 and REAL-11
narrowly established that market structure is active but live Agent economic
action remains absent.

Core boundaries:

- REAL-10 is E1 only: market visibility / structure activity.
- REAL-11 is diagnostic/substrate clarification only: scripted router positive-control works; E2 is not achieved.
- REAL-12 forces economic roles and economic judgment, not economic action.
- A trade cannot be forced and then counted as E2.

## Orchestration

- Orchestrator: GPT-5.5 high/xhigh.
- Docs Worker: GPT-5.5 low.
- Schema/Views Worker: GPT-5.5 medium.
- Gateway/Judgment Worker: GPT-5.5 high.
- Probe Worker: GPT-5.5 medium.
- Analysis Worker: GPT-5.5 high.
- Audit Worker: GPT-5.5 xhigh.

Audit points:

1. plan alignment;
2. after Atom 1-3 schema/gateway/judgment;
3. after Atom 4-5 evidence;
4. final ship review.

## Harness

`turingos_dev` package:

```text
run_id: dev_1778893416667_2411521
module: REAL-12 Role-Specialized Economic Agents
risk: 4
fc:
  FC1 role action loop
  FC2 role assignment / PromptCapsule replay
  FC3 dashboard/audit materialized view
  Art. III shielding
```

Stop immediately if the implementation needs sequencer admission, TypedTx
schema/discriminant, canonical signing payload, wallet, kernel, bus, or live
REAL-6B.

## Atom 0 — REAL-10 / REAL-11 Narrow Ratification

Create:

```text
handover/directives/2026-05-16_REAL10_REAL11_NARROW_RATIFICATION.md
```

Required statements:

- REAL-10 proves E1 only, not E2/E3/E4.
- REAL-11 proves router scripted positive-control works.
- REAL-11 patched live probe still has `buy_with_coin_router=0`.
- Scripted buys are not E2.
- No live REAL-6B approval.
- No forced trade / no price-as-truth / no ghost liquidity.

Gate:

```text
tests/constitution_real12_claim_boundary.rs
```

Must fail on spontaneous-emergence, causality, model-ranking, live REAL-6B,
scripted-buy-as-E2, price-as-truth, or forced-trade claims.

## Atom 1 — Role Specialization Schema

Add:

```rust
BullTrader
BearTrader
```

Compatibility:

- Existing `Trader` remains as a legacy role only.
- REAL-12 prompts/tests use `BullTrader` and `BearTrader`.

Allowed action matrix:

```text
BullTrader:
  buy_yes
  abstain

BearTrader:
  buy_no
  abstain

Solver:
  WorkTx / proof action only

Verifier:
  VerifyTx only

Challenger:
  ChallengeTx only
```

`bid_task` is not canonical Bull/Bear schema. If retained for legacy, it must
normalize to market-only EconomicJudgment and never route to WorkTx/proof,
VerifyTx, or ChallengeTx.

Gate:

```text
tests/constitution_real12_role_specialization.rs
```

Must prove Bull/Bear illegal actions route L4.E PolicyViolation /
RoleActionNotAllowed.

## Atom 2 — Role-Specific Views

Views derive from ChainTape/CAS/QState only.

BullTraderView includes YES price, TaskOutcome YES market, NodeSurvive YES
market, balance, realized/unrealized PnL, risk cap, liquidity/depth,
deadline/budget remaining.

BearTraderView includes NO price, unsolved-task risk, candidate weakness
signals, challenge status, failed attempts, market depth, balance, PnL, risk
cap.

SolverView includes Lean goal, local proof context, local errors, and limited
market summary only.

Gate:

```text
tests/constitution_real12_role_views.rs
```

Must prove broadcast/shielding/read-set behavior and no raw logs/private CoT.

## Atom 3 — Mandatory EconomicJudgment

Every Bull/Bear turn must create CAS-backed judgment:

```rust
EconomicJudgment {
    agent_id,
    role,
    visible_markets,
    chosen_market,
    intended_side,
    intended_amount,
    action,
    reason,
    observed_price,
    estimated_probability_band,
    expected_value_sign,
    liquidity_depth,
    balance_available,
    risk_cap,
    oracle_or_deadline_risk,
    prompt_capsule_cid,
}
```

Reasons:

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

Validation:

- Buy/Short requires positive EV basis.
- Bull cannot choose NO side.
- Bear cannot choose YES side unless future ratification explicitly allows it.
- Abstain requires structured reason.
- No private CoT.

Gate:

```text
tests/constitution_real12_economic_judgment.rs
```

## Atom 4 — Scripted Bull/Bear Positive-Control

Scripted controls are not E2:

```text
BullTrader scripted BuyYes -> L4 accepted.
BearTrader scripted BuyNo -> L4 accepted.
```

Bear BuyNo success is mandatory. A short-equivalent L4.E cannot satisfy this
gate.

Gate:

```text
tests/constitution_real12_bull_bear_positive_control.rs
```

Must prove ChainTape-derived PnL/positions, CTF conservation, no ghost
liquidity, no f64/f32 money path, audit_tape PROCEED.

## Atom 5 — Live Role-Specialized Micro-Probe

Run 3-5 MiniF2F tasks:

```text
roles:
  Solver
  BullTrader
  BearTrader
  Verifier
  Challenger

market enabled
role views enabled
no forced trade
live REAL-6B disabled
no scripted buys
```

Runner records:

```text
bull_judgment_count
bear_judgment_count
buy_yes_router_count
buy_no_router_count
abstain_reason_distribution
live_non_scripted_router_tx_count
```

Gate:

```text
tests/constitution_real12_live_micro_probe.rs
```

## Atom 6 — Decision Gate

- live non-scripted buy/short -> E2 candidate; proceed REAL-13 E3 study.
- actionable markets but no live action -> REAL-13A expected-value scaffolding.
- no actionable market -> REAL-13B event timing / live REAL-6B Class-4 packet.
- positive-control fail -> substrate fix.

## Verification

Targeted:

```bash
cargo test --test constitution_real12_claim_boundary
cargo test --test constitution_real12_role_specialization
cargo test --test constitution_real12_role_views
cargo test --test constitution_real12_economic_judgment
cargo test --test constitution_real12_bull_bear_positive_control
cargo test --test constitution_real12_live_micro_probe
```

Ship:

```bash
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
bash scripts/run_constitution_gates.sh
cargo test --workspace --no-fail-fast -- --test-threads=1
```

Evidence:

```text
audit_tape PROCEED
all reported facts regenerate from ChainTape/CAS
no E2 claim unless live non-scripted router tx exists
no E3 claim unless persistent behavioral differentiation exists
no E4 claim unless statistical support exists
```

## Forbidden

```text
No forced trade as E2 evidence.
No price-as-truth.
No ghost liquidity.
No f64/f32 money.
No off-tape WAL as truth.
No private CoT recording.
No raw-log broadcast.
No dashboard/report as source of truth.
No live REAL-6B in REAL-12.
No sequencer / TypedTx / signing payload changes without separate Class-4 ratification.
```
