# A04b - Corrected Batch-Open Budget-Elision Negative Control

Status: COMPLETE-INCONCLUSIVE

Evidence run tag:

`real4_prompt_a04b_batchopen_k0_budget_elision_2026-05-14T12-42-56Z`

Purpose:

Correct A04 by using `batch_open` scope so prior-task market context can exist,
then force visible context width to K=0.

Prompt / run deltas:

- `TURINGOS_PROMPT_VARIANT=v0`
- `TURINGOS_MARKET_ARENA_PROMPT=1`
- `TURINGOS_TB_N3_AUTO_MARKET=1`
- `TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=batch_open`
- `TURINGOS_TB_N3_MARKET_CONTEXT_K=0`

Problem set:

`handover/evidence/real4_prompt_market_activation_2026-05-14T12-03-04Z/problemsets/A04_budget_elision.txt`

Command:

```bash
env \
  TURINGOS_G_PHASE_DIRTY_OK=1 \
  TURINGOS_MARKET_ARENA_PROMPT=1 \
  TURINGOS_TB_N3_AUTO_MARKET=1 \
  TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=batch_open \
  TURINGOS_TB_N3_MARKET_CONTEXT_K=0 \
  TURINGOS_PROMPT_VARIANT=v0 \
  PER_PROBLEM_TIMEOUT_S=900 \
  bash scripts/run_g_phase_batch.sh \
    real4_prompt_a04b_batchopen_k0_budget_elision_2026-05-14T12-42-56Z \
    handover/evidence/real4_prompt_market_activation_2026-05-14T12-03-04Z/problemsets/A04_budget_elision.txt
```

Expected interpretation:

- No E2 claim.
- Expected useful signal is `PromptBudgetExceeded` or another explicit
  budget/context-elision reason.

Result:

Completed at 2026-05-14T12:47Z.

Evidence:

- Run directory:
  `handover/evidence/real4_prompt_a04b_batchopen_k0_budget_elision_2026-05-14T12-42-56Z/`
- `audit_tape` verdict:
  `PROCEED`
- `audit_dashboard` report:
  `handover/evidence/real4_prompt_a04b_batchopen_k0_budget_elision_2026-05-14T12-42-56Z/audit_dashboard_run_report.md`

Run metrics:

- Problems: 2.
- `mathd_algebra_125`: solved, verified, tx_count 1.
- `mathd_algebra_141`: not solved, hit max tx, tx_count 200.
- L4 entries: 7.
- L4.E entries: 16.
- CAS objects: 79.
- `buy_with_coin_router`: 0.
- `MarketDecisionTrace` objects: 0.
- submitted market decisions: 0.
- no-trade decisions: 0.

Interpretation:

- This is a valid run but still an inconclusive budget-elision negative
  control.
- `batch_open + K=0` did not emit `PromptBudgetExceeded`.
- Current behavior looks like K=0 suppresses the market decision window rather
  than recording a budget-elided no-trade trace.
- This should be treated as a test finding, not as E1/E2/E3 evidence.
