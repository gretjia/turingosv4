# A05 - Easy-Then-Hard Persistence Probe

Status: COMPLETE

Evidence run tag:

`real4_prompt_a05_easy_then_hard_2026-05-14T13-15-46Z`

Purpose:

Seed earlier problem state, then observe whether later hard tasks create
market decisions under persistent runtime state.

Prompt / run deltas:

Same as A01.

Problem set:

`handover/evidence/real4_prompt_market_activation_2026-05-14T12-03-04Z/problemsets/A05_easy_then_hard.txt`

Command:

```bash
env \
  TURINGOS_G_PHASE_DIRTY_OK=1 \
  TURINGOS_MARKET_ARENA_PROMPT=1 \
  TURINGOS_TB_N3_AUTO_MARKET=1 \
  TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=batch_open \
  TURINGOS_TB_N3_MARKET_CONTEXT_K=10 \
  TURINGOS_PROMPT_VARIANT=v0 \
  PER_PROBLEM_TIMEOUT_S=900 \
  bash scripts/run_g_phase_batch.sh \
    real4_prompt_a05_easy_then_hard_2026-05-14T13-15-46Z \
    handover/evidence/real4_prompt_market_activation_2026-05-14T12-03-04Z/problemsets/A05_easy_then_hard.txt
```

Expected interpretation:

- E1 if all market-visible turns are traced.
- E2 only if a live agent-generated market action appears.
- If no trades appear, the no-trade reason distribution is the main output.

Result:

Completed at 2026-05-14T13:42Z.

Evidence:

- Run directory:
  `handover/evidence/real4_prompt_a05_easy_then_hard_2026-05-14T13-15-46Z/`
- `audit_tape` verdict:
  `PROCEED`
- `audit_dashboard` report:
  `handover/evidence/real4_prompt_a05_easy_then_hard_2026-05-14T13-15-46Z/audit_dashboard_run_report.md`
- Persistence report:
  `handover/evidence/real4_prompt_a05_easy_then_hard_2026-05-14T13-15-46Z/PERSISTENCE_BINDING_REPORT.json`

Run metrics:

- Problems: 5.
- `mathd_algebra_107`: solved, verified, tx_count 1.
- `mathd_algebra_113`: not solved, hit max tx, tx_count 200.
- `mathd_algebra_114`: not solved, hit max tx, tx_count 200.
- `mathd_algebra_125`: not solved, hit max tx, tx_count 200.
- `mathd_algebra_141`: not solved, hit max tx, tx_count 200.
- L4 entries: 10.
- L4.E entries: 109.
- CAS objects: 505.
- `buy_with_coin_router`: 0.
- `cpmm_swap`: 0.
- `MarketDecisionTrace` objects: 47.
- submitted market decisions: 0.
- no-trade decisions: 47.
- no-trade reason distribution:
  - `NoPerceivedEdge`: 47.

Persistence:

- `persistence_passing`: true.
- Witnesses: 5.
- Node positions persisted: count 0 -> 1 across 5 tasks.
- Autopsy capsules accumulated: 0 -> 4.
- Model identity: `deepseek-chat` stable across 5 tasks.

Interpretation:

- E1: PASS. The longer persistent run produced many auditable no-trade
  decisions.
- E2: NOT PROVEN. No live agent-generated router action appeared.
- E3: NOT PROVEN. Single-model persistence did not show role differentiation.

Observation:

Persistence and more hard tasks increased no-trade evidence density, but did
not convert `NoPerceivedEdge` into spontaneous trading.
