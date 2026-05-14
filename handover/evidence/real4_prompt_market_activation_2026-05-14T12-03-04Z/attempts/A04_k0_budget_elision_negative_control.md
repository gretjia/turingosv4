# A04 - Budget-Elision Negative Control

Status: COMPLETE-INCONCLUSIVE

Evidence run tag:

`real4_prompt_a04_k0_budget_elision_2026-05-14T12-39-23Z`

Purpose:

Confirm that zero visible market context is classified as a budget/context
elision condition rather than being confused with spontaneous abstention.

Prompt / run deltas:

- `TURINGOS_PROMPT_VARIANT=v0`
- `TURINGOS_MARKET_ARENA_PROMPT=1`
- `TURINGOS_TB_N3_AUTO_MARKET=1`
- `TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=same_task`
- `TURINGOS_TB_N3_MARKET_CONTEXT_K=0`

Problem set:

`handover/evidence/real4_prompt_market_activation_2026-05-14T12-03-04Z/problemsets/A04_budget_elision.txt`

Command:

```bash
env \
  TURINGOS_G_PHASE_DIRTY_OK=1 \
  TURINGOS_MARKET_ARENA_PROMPT=1 \
  TURINGOS_TB_N3_AUTO_MARKET=1 \
  TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=same_task \
  TURINGOS_TB_N3_MARKET_CONTEXT_K=0 \
  TURINGOS_PROMPT_VARIANT=v0 \
  PER_PROBLEM_TIMEOUT_S=900 \
  bash scripts/run_g_phase_batch.sh \
    real4_prompt_a04_k0_budget_elision_2026-05-14T12-39-23Z \
    handover/evidence/real4_prompt_market_activation_2026-05-14T12-03-04Z/problemsets/A04_budget_elision.txt
```

Expected interpretation:

- No E2 claim.
- Useful only if no-trade reason attribution is clean.

Result:

Completed at 2026-05-14T12:42Z.

Evidence:

- Run directory:
  `handover/evidence/real4_prompt_a04_k0_budget_elision_2026-05-14T12-39-23Z/`
- `audit_tape` verdict:
  `PROCEED`
- `audit_dashboard` report:
  `handover/evidence/real4_prompt_a04_k0_budget_elision_2026-05-14T12-39-23Z/audit_dashboard_run_report.md`

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

- This is a valid run but an inconclusive negative control.
- It did not test budget elision because `same_task` scope did not make the
  prior task's market visible to the second task.
- No E1/E2/E3 claim should be made from this attempt.

Correction:

Run A04b with `TURINGOS_TB_N3_MARKET_CONTEXT_SCOPE=batch_open` and
`TURINGOS_TB_N3_MARKET_CONTEXT_K=0`.
