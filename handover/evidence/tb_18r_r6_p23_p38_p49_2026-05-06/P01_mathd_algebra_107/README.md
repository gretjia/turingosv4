# TB-18R R6 — P01_mathd_algebra_107

**Phase**: TB-18R R6 evidence (P49-class rerun on corrected substrate).
**Authority**: TB-18R charter §2 R6 row + §1.4 SG-18R.3 + SG-18R.4 v2.
**Date**: 2026-05-06T09-05-28Z
**Git HEAD**: 083b0073b48d228196502c0dfe900ddeaef1e834
**Predecessor**: TB-18R R5 SHIPPED commit 5a09e2d.

## Run params

- problem: `mathd_algebra_107`
- problem file: `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_107.lean`
- MAX_TX: 12
- per-problem timeout: 600s
- LLM proxy: http://localhost:8080
- active model: deepseek-chat
- condition: n1
- duration: 583s
- evaluator exit code: 0

## Outputs

- `evaluator.stdout` / `evaluator.stderr` — evaluator output.
- `runtime_repo/` — chaintape L4 + L4.E git repo.
- `cas/` — CAS object store (AttemptTelemetry + LeanResult +
  ProposalTelemetry + RejectedSubmissionRecord etc.).
- `chain_invariant.json` — R4 invariant facts (FR-18R.4 v2 6-field
  accounting + verdict).
- `verdict.json` — audit_tape battery verdict (R5 assertions 44/45/46
  PASS confirmation on real chain data).

## R4 invariant verdict

```
{
  "attempt_aborted_count": 0,
  "delta": 2,
  "expected_completed_attempts": 0,
  "invariant_verdict": "Err(TB-18R FR-18R.3 violation: clean halt MaxTxExhausted requires delta=0 but delta=2 (l4=1, l4e=1, expected=0))",
  "l4_work_attempt_count": 1,
  "l4e_work_attempt_count": 1,
  "preflight": "handover/ai-direct/TB-18R_R4_STEP_B_invariant.md",
  "tb_18r_r4_invariant_equation": "evaluator_reported_completed_llm_calls == l4_work_attempt_count + l4e_work_attempt_count",
  "terminal_halt_class": "MaxTxExhausted"
}
```
