# TB-18R G2 round-2 R9 — P38 + P49 evaluable rerun on R8-fix substrate

**Phase**: TB-18R G2 round-2 R9 evidence (Q12 VETO closure).
**Authority**: G2 round-1 verdict §3 Blocker 2 + §6 R9; charter §2 R6 row + §1.4 SG-18R.4 v2.
**Run timestamp**: 2026-05-06T11-13-27Z.
**Git HEAD at run**: `095a622` (TB-18R G2 round-2 R8 + R10 atom commit).
**Predecessor evidence**: `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/` (P38/P49 SIGKILL'd at 600s, evaluable=false).

---

## §1 Why this rerun was needed

Per G2 round-1 Codex VETO Q12: R6 P02 (`mathd_numbertheory_1124`) + P03 (`numbertheory_2pownm1prime_nprime`) carried `r4_invariant_equation_evaluable=false` because the evaluator was SIGKILL'd by the 600s per-problem timeout before emitting `PPUT_RESULT`. With no evaluator-side count, the R4 invariant equation
`evaluator_reported_completed_llm_calls == l4_work_attempt_count + l4e_work_attempt_count`
could not be evaluated.

## §2 Root cause (compound)

1. **MAX_TX cap was a no-op**: The R6 runner script (`run_tb_18r_r6_evidence.sh`) sets `MAX_TX_OVERRIDE="$MAX_TX"`, but the evaluator reads `MAX_TRANSACTIONS` (see `experiments/minif2f_v4/src/bin/evaluator.rs:1704`). `MAX_TX_OVERRIDE` is unread → cap unenforced → P38 ran 50 LLM cycles (intended 12) before timeout.
2. **600s timeout too tight for hard problems** even at the intended 12-cap, since each cycle is 10–25s on heavy heuristics.

## §3 R9 fix

Dedicated R9 runner `handover/tests/scripts/run_tb_18r_r9_evidence.sh`:
- `MAX_TRANSACTIONS=12` (correct env var; enforces the per-problem cap).
- `PER_PROBLEM_TIMEOUT_S=1800` (3× R6).
- Otherwise identical to R6 runner.

## §4 Run params

```text
problems:           2 (P38 + P49)
MAX_TRANSACTIONS:   12 per problem (now enforced)
per_problem_timeout: 1800s
LLM_proxy:          http://localhost:8080
active_model:       deepseek-chat
condition:          n1
git_head:           095a622
```

| # | Problem | Index | Duration | Halt |
|---|---|---|---|---|
| P01 | mathd_numbertheory_1124 | P38 | 193s | MaxTxExhausted |
| P02 | numbertheory_2pownm1prime_nprime | P49 | 222s | MaxTxExhausted |

Both terminated cleanly under the 12-cap; neither hit timeout.

## §5 Per-problem invariant results (post `tb_18r_postprocess_invariant_v4.sh`)

The runner-script's initial extraction of `EXPECTED_COMPLETED` from PPUT_RESULT used a stale field-name cascade (`completed_llm_calls` → `externalized_llm_calls` → `proposal_count`); none match the v4 PPUT_RESULT shape, so the runner's first invariant write had `expected=0`. The canonical v4 postprocess (`handover/tests/scripts/tb_18r_postprocess_invariant_v4.sh`) extracts the correct count (`step_reject + parse_fail + llm_err + sorry_block + omega + complete + complete_via_tape + preseed_work`; `step_partial_ok` excluded per R3 §1.3 amended CAS-only path) and rewrites `chain_invariant.json` with the proper expected count.

| Tag | l4 | l4e | expected | delta | halt | verdict |
|---|---|---|---|---|---|---|
| P01_mathd_numbertheory_1124 | 0 | 12 | 12 | 0 | MaxTxExhausted | **Ok** |
| P02_numbertheory_2pownm1prime_nprime | 0 | 13 | 13 | 0 | MaxTxExhausted | **Ok** |

Both pass the R4 invariant equation (delta=0; clean halt). **Q12 VETO closed**.

## §6 R8 audit_tape on R9 evidence

`audit_tape` (with R8 partial-verdict-aware assert_45) on each R9 run produced verdict=PROCEED, 38 PASS / 0 FAIL / 11 SKIPPED (id44/id45/id46 all Pass on real chain data). See per-problem `verdict.json`.

## §7 Cross-References

- G2 round-1 verdict: `handover/audits/G2_TB_18R_DUAL_AUDIT_VERDICT_2026-05-06.md`.
- R6 predecessor evidence: `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/`.
- R8 audit_tape rerun: `handover/evidence/tb_18r_r8_assert45_rerun_2026-05-06/`.
- R9 runner: `handover/tests/scripts/run_tb_18r_r9_evidence.sh`.
- v4 postprocess: `handover/tests/scripts/tb_18r_postprocess_invariant_v4.sh`.
- G2 round-2 ship report: `handover/audits/TB-18R_G2_ROUND_2_SHIP_REPORT_2026-05-06.md`.
