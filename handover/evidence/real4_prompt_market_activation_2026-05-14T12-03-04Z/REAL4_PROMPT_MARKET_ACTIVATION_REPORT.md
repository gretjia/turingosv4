# REAL-4 Prompt Market Activation Report

Status: IN PROGRESS
Campaign root: `handover/evidence/real4_prompt_market_activation_2026-05-14T12-03-04Z/`
Harness run: `dev_1778760095289_432118`

## Current Summary

REAL-4 is a prompt-only / run-condition-only experiment campaign. It tests
whether existing prompt and market-context controls can stimulate spontaneous
agent trading on real MiniF2F problems.

No architecture code has been changed for this campaign.

## Round-1 Result Table

| Attempt | Main delta | MarketDecisionTrace | Submitted | BuyWithCoinRouter | Dominant reason | Interpretation |
| --- | --- | ---: | ---: | ---: | --- | --- |
| A01 | `v0`, `batch_open`, K=10 | 29 | 0 | 0 | `NoPerceivedEdge` | E1 only |
| A02 | `v3`, `batch_open`, K=10 | 6 | 0 | 0 | `NoPerceivedEdge` | E1 only |
| A03 | `v4`, `batch_open`, K=10 | 4 | 0 | 0 | `NoPerceivedEdge` | E1 only |
| A04 | `v0`, `same_task`, K=0 | 0 | 0 | 0 | none emitted | inconclusive |
| A04b | `v0`, `batch_open`, K=0 | 0 | 0 | 0 | none emitted | inconclusive |
| A05 | `v0`, `batch_open`, K=10, 5-task persistence | 47 | 0 | 0 | `NoPerceivedEdge` | E1 only |
| A06 | `v0`, `batch_open`, K=10, 4 model families | 7 | 0 | 0 | `NoPerceivedEdge` | E1 only |

Round-1 conclusion:

- Market visibility and no-trade tracing work when market context is shown.
- Existing prompt variants changed speed, token use, and no-trade trace count.
- None of the prompt-only variants produced spontaneous live trading.
- The only observed explicit abstention cause was `NoPerceivedEdge`.
- K=0 did not produce `PromptBudgetExceeded`; it suppressed the market
  decision window in these runs.
- Longer persistence increased no-trade evidence density but did not induce
  spontaneous trade.
- Multi-family assignment was witnessed with no hidden model switch, but no
  spontaneous trade emerged.

Therefore REAL-4 Round 1 remains E1-only.

## Attempt A01

Run:

`handover/evidence/real4_prompt_a01_v0_k10_hardmix_2026-05-14T12-03-04Z/`

Condition:

- `TURINGOS_PROMPT_VARIANT=v0`
- `TURINGOS_MARKET_ARENA_PROMPT=1`
- `TURINGOS_TB_N3_AUTO_MARKET=1`
- `TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=batch_open`
- `TURINGOS_TB_N3_MARKET_CONTEXT_K=10`

Problem set:

- `mathd_algebra_107`
- `mathd_algebra_125`
- `mathd_algebra_141`

Evidence:

- `audit_tape` verdict: `PROCEED`
- Persistence report: passing, 5 witnesses.
- `no_hidden_model_switch`: pass in audit assertions.

Market result:

- `MarketDecisionTrace` objects: 29.
- submitted market decisions: 0.
- `buy_with_coin_router`: 0.
- no-trade decisions: 29.
- no-trade reason distribution: `NoPerceivedEdge` = 29.

Interpretation:

- A01 is E1 evidence.
- A01 is not E2 evidence.
- A01 is not E3 evidence.

Mechanism reading:

The agent saw market context and repeatedly declined to invest. The dominant
mechanism label is `NoPerceivedEdge`, not parser failure, router rejection, or
insufficient balance.

## Next Attempts

## Attempt A02

Run:

`handover/evidence/real4_prompt_a02_v3_k10_hardmix_2026-05-14T12-26-11Z/`

Condition:

- `TURINGOS_PROMPT_VARIANT=v3`
- `TURINGOS_MARKET_ARENA_PROMPT=1`
- `TURINGOS_TB_N3_AUTO_MARKET=1`
- `TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=batch_open`
- `TURINGOS_TB_N3_MARKET_CONTEXT_K=10`

Evidence:

- `audit_tape` verdict: `PROCEED`
- Persistence report: passing, 5 witnesses.

Market result:

- `MarketDecisionTrace` objects: 6.
- submitted market decisions: 0.
- `buy_with_coin_router`: 0.
- no-trade decisions: 6.
- no-trade reason distribution: `NoPerceivedEdge` = 6.

Interpretation:

- A02 is E1 evidence.
- A02 is not E2 evidence.
- A02 is not E3 evidence.

Comparison to A01:

- A02 reduced runtime and token use materially.
- A02 reduced no-trade trace count from 29 to 6.
- A02 did not change the market action outcome.
- The dominant mechanism remains `NoPerceivedEdge`.

## Next Attempts

## Attempt A03

Run:

`handover/evidence/real4_prompt_a03_v4_k10_hardmix_2026-05-14T12-32-23Z/`

Condition:

- `TURINGOS_PROMPT_VARIANT=v4`
- `TURINGOS_MARKET_ARENA_PROMPT=1`
- `TURINGOS_TB_N3_AUTO_MARKET=1`
- `TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=batch_open`
- `TURINGOS_TB_N3_MARKET_CONTEXT_K=10`

Evidence:

- `audit_tape` verdict: `PROCEED`
- Persistence report: passing, 4 witnesses.

Market result:

- `MarketDecisionTrace` objects: 4.
- submitted market decisions: 0.
- `buy_with_coin_router`: 0.
- no-trade decisions: 4.
- no-trade reason distribution: `NoPerceivedEdge` = 4.

Interpretation:

- A03 is E1 evidence.
- A03 is not E2 evidence.
- A03 is not E3 evidence.

Comparison across A01-A03:

- A01 `v0`: 29 no-trade traces, all `NoPerceivedEdge`.
- A02 `v3`: 6 no-trade traces, all `NoPerceivedEdge`.
- A03 `v4`: 4 no-trade traces, all `NoPerceivedEdge`.
- All three audit cleanly with `PROCEED`.
- None produced live agent-generated router activity.

Preliminary mechanism finding:

The existing prompt variants can change proof-search cadence and how many
market decision windows are reached, but they do not yet create a perceived
market edge strong enough for spontaneous investment.

## Next Attempts

## Attempt A04

Run:

`handover/evidence/real4_prompt_a04_k0_budget_elision_2026-05-14T12-39-23Z/`

Condition:

- `TURINGOS_PROMPT_VARIANT=v0`
- `TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=same_task`
- `TURINGOS_TB_N3_MARKET_CONTEXT_K=0`

Market result:

- `MarketDecisionTrace` objects: 0.
- submitted market decisions: 0.
- `buy_with_coin_router`: 0.
- no-trade decisions: 0.

Interpretation:

- A04 is a valid run but an inconclusive negative control.
- It did not test prompt budget elision because `same_task` scope did not make
  the prior task's market visible to the second task.
- Correction: A04b changes scope to `batch_open` while keeping K=0.

## Next Attempts

## Attempt A04b

Run:

`handover/evidence/real4_prompt_a04b_batchopen_k0_budget_elision_2026-05-14T12-42-56Z/`

Condition:

- `TURINGOS_PROMPT_VARIANT=v0`
- `TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=batch_open`
- `TURINGOS_TB_N3_MARKET_CONTEXT_K=0`

Market result:

- `MarketDecisionTrace` objects: 0.
- submitted market decisions: 0.
- `buy_with_coin_router`: 0.
- no-trade decisions: 0.

Interpretation:

- A04b is a valid run but still inconclusive as budget-elision evidence.
- K=0 appears to suppress the market decision window entirely.
- This means the current prompt-only harness cannot use K=0 to test
  `PromptBudgetExceeded` without changing code or choosing a different existing
  run condition.

Test finding:

The no-trade classifier distinguishes `NoPerceivedEdge` well when market blocks
are visible, but the K=0 negative-control path did not produce an explicit
budget-elision trace in A04 or A04b.

## No-Architecture-Change Boundary

The following were not changed:

- market architecture
- sequencer admission
- typed transaction schema
- wallet backend
- CAS schema
- Trust Root files
- source-level prompt construction

If the next round wants stronger prompt wording than existing
`TURINGOS_PROMPT_VARIANT` options allow, that should be treated as a separate
source-level prompt package and not hidden inside this prompt-only campaign.

## Next Attempts

## Attempt A05

Run:

`handover/evidence/real4_prompt_a05_easy_then_hard_2026-05-14T13-15-46Z/`

Condition:

- `TURINGOS_PROMPT_VARIANT=v0`
- `TURINGOS_MARKET_ARENA_PROMPT=1`
- `TURINGOS_TB_N3_AUTO_MARKET=1`
- `TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=batch_open`
- `TURINGOS_TB_N3_MARKET_CONTEXT_K=10`
- 5-task easy-then-hard problem set.

Evidence:

- `audit_tape` verdict: `PROCEED`
- Persistence report: passing, 5 witnesses.
- Node positions persisted across 5 tasks.
- Autopsy capsules accumulated from 0 to 4.

Market result:

- `MarketDecisionTrace` objects: 47.
- submitted market decisions: 0.
- `buy_with_coin_router`: 0.
- no-trade decisions: 47.
- no-trade reason distribution: `NoPerceivedEdge` = 47.

Interpretation:

- A05 is E1 evidence.
- A05 is not E2 evidence.
- A05 is not E3 evidence.

Mechanism reading:

Persistent state and longer hard-task exposure made abstention more visible,
but the agent still did not perceive a market edge worth investing in.

## Next Attempts

## Attempt A06

Run:

`handover/evidence/real4_prompt_a06_multimodel_k10_2026-05-14T13-43-24Z/`

Condition:

- `TURINGOS_PROMPT_VARIANT=v0`
- `TURINGOS_MARKET_ARENA_PROMPT=1`
- `TURINGOS_TB_N3_AUTO_MARKET=1`
- `TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=batch_open`
- `TURINGOS_TB_N3_MARKET_CONTEXT_K=10`
- `PHASE_D_HETERO_OK=1`
- `TURINGOS_G4_REQUIRED_MODEL_FAMILIES=3`
- `AGENT_MODELS` assigned 4 observed model families:
  deepseek, openai, claude, qwen.

Evidence:

- `audit_tape` verdict: `PROCEED`
- `no_hidden_model_switch`: PASS
- Persistence report: passing, 5 witnesses.

Market result:

- `MarketDecisionTrace` objects: 7.
- submitted market decisions: 0.
- `buy_with_coin_router`: 0.
- no-trade decisions: 7.
- no-trade reason distribution: `NoPerceivedEdge` = 7.

Activity by assigned model family:

- deepseek: 7 no-trade traces.
- openai: 0 market-decision traces.
- claude: 0 market-decision traces.
- qwen: 0 market-decision traces.

Interpretation:

- A06 is E1 evidence.
- A06 is not E2 evidence.
- A06 is not E3 evidence.
- The model-family difference is activity divergence only; it is not a model
  ranking and not role differentiation.

Mechanism reading:

Multi-family assignment did not by itself create spontaneous market action.
The run also recorded more `llm_err` activity than the single-model runs,
suggesting API/model availability or response compatibility may be part of the
mechanism bottleneck.

## Next Attempts

Recommended next prompt-only attempts:

1. If budget-elision must be tested, design a source-level classifier test in a
   separate code-change package rather than pretending A04/A04b proved it.
2. If stronger prompt wording is desired, prepare an architect-facing
   source-level prompt packet rather than changing `src/sdk/prompt.rs` inside
   this prompt-only experiment.
3. If spontaneous trading remains the target without forced trades, the next
   non-architecture experiment should vary task selection toward problems with
   visible partial-progress uncertainty and multiple open node markets, because
   the current mechanism bottleneck is consistently `NoPerceivedEdge`.

Do not claim spontaneous trading until E2 appears.
