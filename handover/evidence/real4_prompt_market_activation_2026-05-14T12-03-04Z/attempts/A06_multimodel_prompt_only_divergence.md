# A06 - Multi-Model Prompt-Only Divergence Probe

Status: COMPLETE

Evidence run tag:

`real4_prompt_a06_multimodel_k10_2026-05-14T13-43-24Z`

Purpose:

If API availability permits, compare the same prompt-only market context across
at least three model families without making model-ranking claims.

Prompt / run deltas:

Same as A01, plus:

- `PHASE_D_HETERO_OK=1`
- `TURINGOS_G4_REQUIRED_MODEL_FAMILIES=3`
- `AGENT_MODELS` contains at least three model families.

Expected interpretation:

- E1 if decisions or no-trade reasons are visible by family.
- E2 only if a live agent-generated market action appears.
- E3 only if persistent role differentiation is observed.

Result:

Completed at 2026-05-14T13:53Z.

Evidence:

- Run directory:
  `handover/evidence/real4_prompt_a06_multimodel_k10_2026-05-14T13-43-24Z/`
- `audit_tape` verdict:
  `PROCEED`
- `audit_dashboard` report:
  `handover/evidence/real4_prompt_a06_multimodel_k10_2026-05-14T13-43-24Z/audit_dashboard_run_report.md`
- Persistence report:
  `handover/evidence/real4_prompt_a06_multimodel_k10_2026-05-14T13-43-24Z/PERSISTENCE_BINDING_REPORT.json`

Model identity:

- Required model families: 3.
- Observed model families: 4.
- Assignment:
  - `Agent_0`: deepseek / `deepseek-chat`
  - `Agent_1`: openai / `gpt-5.2`
  - `Agent_2`: claude / `claude-3-7-sonnet`
  - `Agent_3`: qwen / `qwen3-coder`
  - `Agent_4`: deepseek / `deepseek-chat`
  - `Agent_5`: openai / `gpt-5.2`
  - `Agent_6`: claude / `claude-3-7-sonnet`
  - `Agent_7`: qwen / `qwen3-coder`
  - `Agent_8`: deepseek / `deepseek-chat`
  - `Agent_9`: openai / `gpt-5.2`
- `no_hidden_model_switch`: PASS.

Run metrics:

- Problems: 3.
- `mathd_algebra_107`: solved, verified, tx_count 5.
- `mathd_algebra_125`: not solved, hit max tx, tx_count 200.
- `mathd_algebra_141`: not solved, hit max tx, tx_count 200.
- L4 entries: 8.
- L4.E entries: 80.
- CAS objects: 248.
- `buy_with_coin_router`: 0.
- `cpmm_swap`: 0.
- `MarketDecisionTrace` objects: 7.
- submitted market decisions: 0.
- no-trade decisions: 7.
- no-trade reason distribution:
  - `NoPerceivedEdge`: 7.

Activity by assigned model family:

- deepseek: 7 no-trade traces.
- openai: 0 market-decision traces.
- claude: 0 market-decision traces.
- qwen: 0 market-decision traces.

Interpretation:

- E1: PASS. Multi-family assignment and no-hidden-switch audit are present,
  and market-visible no-trade evidence is present.
- E2: NOT PROVEN. No live agent-generated router action appeared.
- E3: NOT PROVEN. The activity difference is only an activity observation, not
  persistent role differentiation.

Observation:

The multi-model prompt-only run produced more LLM errors than the single-model
runs and only deepseek-assigned agents reached recorded no-trade market
decision windows. This is a model-family activity divergence observation, not a
model quality ranking.

Command:

```bash
env \
  TURINGOS_G_PHASE_DIRTY_OK=1 \
  TURINGOS_MARKET_ARENA_PROMPT=1 \
  TURINGOS_TB_N3_AUTO_MARKET=1 \
  TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=batch_open \
  TURINGOS_TB_N3_MARKET_CONTEXT_K=10 \
  TURINGOS_PROMPT_VARIANT=v0 \
  PHASE_D_HETERO_OK=1 \
  TURINGOS_G4_REQUIRED_MODEL_FAMILIES=3 \
  AGENT_MODELS=deepseek-chat,gpt-5.2,claude-3-7-sonnet,qwen3-coder,deepseek-chat,gpt-5.2,claude-3-7-sonnet,qwen3-coder,deepseek-chat,gpt-5.2 \
  PER_PROBLEM_TIMEOUT_S=900 \
  bash scripts/run_g_phase_batch.sh \
    real4_prompt_a06_multimodel_k10_2026-05-14T13-43-24Z \
    handover/evidence/real4_prompt_market_activation_2026-05-14T12-03-04Z/problemsets/A01_hardmix.txt
```
