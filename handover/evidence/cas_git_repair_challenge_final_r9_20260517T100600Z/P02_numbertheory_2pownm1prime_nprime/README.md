# TB-18R G2 round-2 R9 — P02_numbertheory_2pownm1prime_nprime

**Phase**: TB-18R G2 round-2 R9 evidence (P38/P49 evaluable rerun on R8-fix substrate).
**Authority**: G2 verdict §3 Blocker 2 + §6 R9; charter §2 R6 row + §1.4 SG-18R.4 v2.
**Date**: 2026-05-17T10-07-33Z
**Git HEAD**: c85dacfafcd1aa99047cee6af4f5799670403f8f
**Predecessor**: R6 SIGKILL evidence (handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P02_numbertheory_2pownm1prime_nprime/).

## Run params

- problem: `numbertheory_2pownm1prime_nprime`
- problem file: `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/numbertheory_2pownm1prime_nprime.lean`
- MAX_TRANSACTIONS: 12 (was MAX_TX_OVERRIDE in r6 runner — a no-op)
- per-problem timeout: 1800s (3x prior 600s)
- LLM proxy: http://localhost:8080
- active model: deepseek-chat
- condition: n1
- duration: 716s
- evaluator exit code: 0

## R4 invariant verdict

Refreshed by `tb_18r_postprocess_invariant_v4.sh` after the run to use the
structured PPUT expected-count helper.

```
{
  "attempt_aborted_count": 0,
  "capsule_anchored_attempt_count": 3,
  "delta": 0,
  "expected_completed_attempts": 8,
  "invariant_verdict": "Ok",
  "l4_work_attempt_count": 0,
  "l4e_work_attempt_count": 5,
  "preflight": "handover/ai-direct/TB-18R_R4_STEP_B_invariant.md",
  "tb_18r_r4_invariant_equation": "evaluator_reported_completed_llm_calls == l4_work_attempt_count + l4e_work_attempt_count + capsule_anchored_attempt_count",
  "tbc0_strict_audit_fix": "STRICT_AUDIT_TBC0_TAPE_2026-05-07.md Finding C — Bug 3 (capsule_anchored 3-term)",
  "terminal_halt_class": "MaxTxExhausted"
}
```
