# A02 - v3 Operating-Laws Prompt Variant

Status: COMPLETE

Evidence run tag:

`real4_prompt_a02_v3_k10_hardmix_2026-05-14T12-26-11Z`

Purpose:

Compare the existing v3 operating-laws prompt variant against A01 under the
same market context.

Prompt / run deltas from A01:

- `TURINGOS_PROMPT_VARIANT=v3`

All other market knobs remain the same as A01.

Command:

```bash
env \
  TURINGOS_G_PHASE_DIRTY_OK=1 \
  TURINGOS_MARKET_ARENA_PROMPT=1 \
  TURINGOS_TB_N3_AUTO_MARKET=1 \
  TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=batch_open \
  TURINGOS_TB_N3_MARKET_CONTEXT_K=10 \
  TURINGOS_PROMPT_VARIANT=v3 \
  PER_PROBLEM_TIMEOUT_S=900 \
  bash scripts/run_g_phase_batch.sh \
    real4_prompt_a02_v3_k10_hardmix_2026-05-14T12-26-11Z \
    handover/evidence/real4_prompt_market_activation_2026-05-14T12-03-04Z/problemsets/A01_hardmix.txt
```

Result:

Completed at 2026-05-14T12:31Z.

Evidence:

- Run directory:
  `handover/evidence/real4_prompt_a02_v3_k10_hardmix_2026-05-14T12-26-11Z/`
- `audit_tape` verdict:
  `PROCEED`
- `audit_dashboard` report:
  `handover/evidence/real4_prompt_a02_v3_k10_hardmix_2026-05-14T12-26-11Z/audit_dashboard_run_report.md`
- Persistence report:
  `handover/evidence/real4_prompt_a02_v3_k10_hardmix_2026-05-14T12-26-11Z/PERSISTENCE_BINDING_REPORT.json`

Run metrics:

- Problems: 3
- `mathd_algebra_107`: solved, verified, tx_count 1.
- `mathd_algebra_125`: not solved, hit max tx, tx_count 200.
- `mathd_algebra_141`: not solved, hit max tx, tx_count 200.
- L4 entries: 8.
- L4.E entries: 27.
- CAS objects: 142.
- `buy_with_coin_router`: 0.
- `cpmm_swap`: 0.
- `MarketDecisionTrace` objects: 6.
- submitted market decisions: 0.
- no-trade decisions: 6.
- no-trade reason distribution:
  - `NoPerceivedEdge`: 6.

Interpretation:

- E1: PASS. Market-visible no-trade evidence is present.
- E2: NOT PROVEN. No live agent-generated router action appeared.
- E3: NOT PROVEN.

Observation:

Compared with A01, the v3 operating-laws prompt condition was much faster and
produced fewer no-trade decision windows, but it did not change the market
action outcome. The no-trade mechanism remains `NoPerceivedEdge`.
