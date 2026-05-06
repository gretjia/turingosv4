# TB-18R G2 round-2 R9 — P01_mathd_numbertheory_1124

**Phase**: TB-18R G2 round-2 R9 evidence (P38/P49 evaluable rerun on R8-fix substrate).
**Authority**: G2 verdict §3 Blocker 2 + §6 R9; charter §2 R6 row + §1.4 SG-18R.4 v2.
**Date**: 2026-05-06T11-13-27Z
**Git HEAD**: 095a62264d10b9bc56af5729872320daa587a352
**Predecessor**: R6 SIGKILL evidence (handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P01_mathd_numbertheory_1124/).

## Run params

- problem: `mathd_numbertheory_1124`
- problem file: `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_numbertheory_1124.lean`
- MAX_TRANSACTIONS: 12 (was MAX_TX_OVERRIDE in r6 runner — a no-op)
- per-problem timeout: 1800s (3x prior 600s)
- LLM proxy: http://localhost:8080
- active model: deepseek-chat
- condition: n1
- duration: 193s
- evaluator exit code: 0

## R4 invariant verdict

```
{
  "attempt_aborted_count": 0,
  "delta": 12,
  "expected_completed_attempts": 0,
  "invariant_verdict": "Err(TB-18R FR-18R.3 violation: clean halt MaxTxExhausted requires delta=0 but delta=12 (l4=0, l4e=12, expected=0))",
  "l4_work_attempt_count": 0,
  "l4e_work_attempt_count": 12,
  "preflight": "handover/ai-direct/TB-18R_R4_STEP_B_invariant.md",
  "tb_18r_r4_invariant_equation": "evaluator_reported_completed_llm_calls == l4_work_attempt_count + l4e_work_attempt_count",
  "terminal_halt_class": "MaxTxExhausted"
}
```
