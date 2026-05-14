# REAL-4 Prompt-Only Market Activation Test Plan

Status: ACTIVE
Created: 2026-05-14T12:03:04Z
Harness run: `dev_1778760095289_432118`

## Objective

Design and run new real-problem tests that continuously probe whether the
existing market mechanism can stimulate spontaneous agent trading.

This package is prompt-only / run-condition-only. It does not change market
architecture, transaction schema, sequencer admission, wallet semantics, CAS
schema, or Trust Root authority.

## Risk And FC Mapping

Risk class: 2

Touched nodes:

- FC1: agent prompt / action loop, especially proof / verify / invest /
  abstain choice under market-visible context.
- FC3: evidence and report views, especially attempt ledger, market-decision
  trace counts, no-trade reasons, and run reports.

Not touched:

- FC2 boot / genesis authority.
- Sequencer admission.
- Typed transaction schema.
- Canonical signing payload.
- Wallet backend.
- CAS object schema.
- Trust Root pinned source surfaces.

If a future round requires source-level prompt construction changes in
`src/sdk/prompt.rs`, stop and reclassify before editing. This plan is designed
to exhaust existing prompt and run-condition knobs first.

## Hard Boundaries

- Do not force agents to trade.
- Do not add new market mechanics.
- Do not claim E2 unless a live agent-generated router transaction appears.
- Do not claim E3 unless persistent role differentiation is observed.
- Do not use price as truth. Price is only a signal.
- Do not store raw CoT or raw hidden prompt/completion bodies in new records.
- Do not rewrite old ChainTape, CAS, or evidence.

## Existing Prompt And Run Knobs

These knobs already exist and are allowed for this test package:

- `TURINGOS_MARKET_ARENA_PROMPT=1`
  - Adds the existing market decision frame.
  - The current frame says proof, verify, invest, and abstain are legal choices.
  - It says to use invest only when public market context gives a perceived
    edge.
  - It says not to force a trade and that price is signal, not truth.

- `TURINGOS_PROMPT_VARIANT`
  - `v0`: baseline prompt.
  - `v1`: drops unused tools while keeping invest.
  - `v2`: tactic-search guidance.
  - `v3`: operating laws.
  - `v4`: tactic guidance plus recent rejects.

- `TURINGOS_TB_N3_AUTO_MARKET=1`
  - Auto-emits node-survive markets.

- `TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE`
  - `same_task`: only same-task market context.
  - `batch_open`: cross-problem open market context.

- `TURINGOS_TB_N3_MARKET_CONTEXT_K`
  - Maximum market context width presented to the agent.
  - `0` is a negative-control budget-elision condition.

## Evidence Layout

Campaign root:

`handover/evidence/real4_prompt_market_activation_2026-05-14T12-03-04Z/`

Required files:

- `ATTEMPT_LEDGER.tsv`
- `README.md`
- `problemsets/*.txt`
- `attempts/Axx_*.md`

Every attempt must record:

- attempt id
- objective
- prompt variant
- market arena prompt flag
- market context scope
- market context K
- problem set
- model assignment / family requirement
- exact command
- evidence run tag
- metrics
- interpretation as E1 / E2 / E3

## Metrics

Primary metrics:

- `submitted_market_traces`
- `buy_with_coin_router`
- `market_decision_trace_count`
- `no_trade_reason_count`
- `no_trade_reason_distribution`
- `invest_attempted`
- `invest_submitted`
- `l4e_market_rejections`
- `audit_tape_result`

Secondary metrics:

- solved / rejected / timeout pattern by problem
- market context was present or budget-elided
- model families observed
- hidden-switch verdict when G4.2 model identity is present

## Experiment Matrix

### A00 - REAL-3 Reference

Use existing REAL-3 full mini evidence as the control reference:

`handover/evidence/g_phase_real_3_full_mini_2026-05-14T10-15-35Z/`

Expected interpretation:

- E1: yes, market visibility and no-trade trace.
- E2: no, no spontaneous live agent-generated router action.
- E3: no, no role differentiation claim.

### A01 - Wide Batch-Open Market Context

Purpose: give agents more public market context without forcing a trade.

Run condition:

- `TURINGOS_PROMPT_VARIANT=v0`
- `TURINGOS_MARKET_ARENA_PROMPT=1`
- `TURINGOS_TB_N3_AUTO_MARKET=1`
- `TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=batch_open`
- `TURINGOS_TB_N3_MARKET_CONTEXT_K=10`

Problem set:

`problemsets/A01_hardmix.txt`

Expected signal:

- If E2 remains absent, the no-trade distribution should explain whether the
  agent saw context but found no perceived edge.

### A02 - Operating-Laws Prompt Variant

Purpose: test whether stronger existing operating-law prompt structure makes
market abstention or investment more explicit.

Run condition:

- Same as A01, but `TURINGOS_PROMPT_VARIANT=v3`.

Problem set:

`problemsets/A01_hardmix.txt`

Expected signal:

- Compare no-trade reason distribution against A01.
- Do not interpret lower proof success as market emergence.

### A03 - Recent-Rejects Prompt Variant

Purpose: test whether recent-reject context changes perceived market edge or
abstention behavior.

Run condition:

- Same as A01, but `TURINGOS_PROMPT_VARIANT=v4`.

Problem set:

`problemsets/A01_hardmix.txt`

Expected signal:

- Look for live invest attempts, not just more verbose abstentions.

### A04 - Market Context Budget-Elision Negative Control

Purpose: verify the classifier distinguishes prompt budget elision from
agent-declined/no-edge abstention.

Run condition:

- `TURINGOS_PROMPT_VARIANT=v0`
- `TURINGOS_MARKET_ARENA_PROMPT=1`
- `TURINGOS_TB_N3_AUTO_MARKET=1`
- `TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=same_task`
- `TURINGOS_TB_N3_MARKET_CONTEXT_K=0`

Problem set:

`problemsets/A04_budget_elision.txt`

Expected signal:

- No E2 claim.
- `PromptBudgetExceeded` or equivalent budget-elision no-trade reason should
  appear when context was available but not shown.

### A05 - Easy-Then-Hard Persistence Probe

Purpose: seed earlier market state, then test whether later hard tasks make
agents consider investment.

Run condition:

- Same as A01.

Problem set:

`problemsets/A05_easy_then_hard.txt`

Expected signal:

- Problem `k+1` must start from the same runtime repo head as problem `k`.
- If no trades happen, no-trade reasons should identify no perceived edge,
  prompt budget, insufficient balance, or another explicit class.

### A06 - Multi-Model Prompt-Only Divergence Probe

Purpose: if API availability permits, run the same prompt condition with at
least three model families and compare market-decision / no-trade behavior by
model family.

Run condition:

- Same as A01.
- `PHASE_D_HETERO_OK=1`
- `TURINGOS_G4_REQUIRED_MODEL_FAMILIES=3`
- `AGENT_MODELS` contains at least three model families.

Expected signal:

- No hidden-switch claim unless G4.2 evidence is present.
- No model ranking claim.
- Only activity divergence may be reported.

## Interpretation Ladder

E1 - Market visibility:

- Market context is present.
- Each market-visible turn yields a market decision trace or no-trade reason.
- Audit and dashboard can regenerate from ChainTape/CAS.

E2 - Market action:

- At least one live agent-generated `BuyWithCoinRouterTx` or short-equivalent
  action is accepted, or an attempted market action is rejected into L4.E with
  explicit reason.

E3 - Emergent role:

- Persistent cross-problem differences appear across agents or model families,
  such as solver / verifier / bull / bear / abstainer behavior.

REAL-4 should expect E1 by default and only upgrade to E2/E3 if the evidence
actually appears.

## Stop Conditions

Stop and consult before continuing if an attempt requires:

- Source-level prompt construction edits.
- New transaction type.
- Wallet, market, CAS, or sequencer code changes.
- Trust Root rehash.
- Forced-trade prompt language.
- E2/E3 claims without evidence.
