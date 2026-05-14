# A01 - v0 Wide Batch-Open Market Context

Status: COMPLETE

Evidence run tag:

`real4_prompt_a01_v0_k10_hardmix_2026-05-14T12-03-04Z`

Purpose:

Give agents wider public market context while preserving the existing
do-not-force-trade rule.

Prompt / run deltas:

- `TURINGOS_PROMPT_VARIANT=v0`
- `TURINGOS_MARKET_ARENA_PROMPT=1`
- `TURINGOS_TB_N3_AUTO_MARKET=1`
- `TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=batch_open`
- `TURINGOS_TB_N3_MARKET_CONTEXT_K=10`

Problem set:

`handover/evidence/real4_prompt_market_activation_2026-05-14T12-03-04Z/problemsets/A01_hardmix.txt`

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
    real4_prompt_a01_v0_k10_hardmix_2026-05-14T12-03-04Z \
    handover/evidence/real4_prompt_market_activation_2026-05-14T12-03-04Z/problemsets/A01_hardmix.txt
```

Expected interpretation:

- E1 if market decision or no-trade traces are visible and auditable.
- E2 only if a live agent-generated router action appears.
- E3 not expected from this attempt.

Result:

Completed at 2026-05-14T12:24Z.

Evidence:

- Run directory:
  `handover/evidence/real4_prompt_a01_v0_k10_hardmix_2026-05-14T12-03-04Z/`
- `audit_tape` verdict:
  `PROCEED`
- `audit_dashboard` report:
  `handover/evidence/real4_prompt_a01_v0_k10_hardmix_2026-05-14T12-03-04Z/audit_dashboard_run_report.md`
- Persistence report:
  `handover/evidence/real4_prompt_a01_v0_k10_hardmix_2026-05-14T12-03-04Z/PERSISTENCE_BINDING_REPORT.json`

Run metrics:

- Problems: 3
- `mathd_algebra_107`: solved, verified, tx_count 2.
- `mathd_algebra_125`: not solved, hit max tx, tx_count 200.
- `mathd_algebra_141`: not solved, hit max tx, tx_count 200.
- L4 entries: 8.
- L4.E entries: 73.
- CAS objects: 355.
- `buy_with_coin_router`: 0.
- `cpmm_swap`: 0.
- `MarketDecisionTrace` objects: 29.
- submitted market decisions: 0.
- no-trade decisions: 29.
- no-trade reason distribution:
  - `NoPerceivedEdge`: 29.

Interpretation:

- E1: PASS. Market context was visible enough to generate traceable
  `NoTrade/NoPerceivedEdge` decisions.
- E2: NOT PROVEN. No live agent-generated router action appeared.
- E3: NOT PROVEN. This single-model condition does not establish role
  differentiation.

Observation:

Widening batch-open market context to K=10 did not stimulate spontaneous
trading. It did convert the zero-trade outcome into auditable no-trade
evidence, concentrated on `NoPerceivedEdge`.
