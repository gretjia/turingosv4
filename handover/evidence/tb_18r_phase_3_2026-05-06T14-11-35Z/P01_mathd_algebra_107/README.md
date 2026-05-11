# TB-18R Phase 3 — P01_mathd_algebra_107

**Phase**: TB-18R Phase 3 (Technical Tape Validation on typed PartialAccepted substrate)
**Authority**: `handover/directives/2026-05-06_TB18R_PHASE_3_LAUNCH_DIRECTIVE.md`
**Date**: 2026-05-06T14-11-35Z
**Git HEAD**: 55a0935213a13b87556da1a57540117cec3438af
**Substrate**: HEAD includes Phase 2 commit `3f51667` (LeanVerdictKind + AttemptOutcome::PartialAccepted)

## Run params

- problem: `mathd_algebra_107`
- problem file: `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_107.lean`
- MAX_TRANSACTIONS: 12
- per-problem timeout: 1800s
- LLM proxy: http://localhost:8080
- active model: deepseek-chat
- condition: n1
- duration: 10s
- evaluator exit code: 0

## Architect §5 invariants (per-run signal)

- audit_tape verdict: `PROCEED`
- "invariant_verdict": "Err(TB-18R FR-18R.3 violation: clean halt OmegaAccepted requires delta=0 but delta=1 (l4=1, l4e=1, expected=1))"
- verdict_kind=PartialAccepted record count: 0 (multi-iteration tape-derived)

## R4 invariant verdict

```
{
  "attempt_aborted_count": 0,
  "delta": 1,
  "expected_completed_attempts": 1,
  "invariant_verdict": "Err(TB-18R FR-18R.3 violation: clean halt OmegaAccepted requires delta=0 but delta=1 (l4=1, l4e=1, expected=1))",
  "l4_work_attempt_count": 1,
  "l4e_work_attempt_count": 1,
  "preflight": "handover/ai-direct/TB-18R_R4_STEP_B_invariant.md",
  "tb_18r_r4_invariant_equation": "evaluator_reported_completed_llm_calls == l4_work_attempt_count + l4e_work_attempt_count",
  "terminal_halt_class": "OmegaAccepted"
}
```

## verdict_kind summary

```
{
  "total_lean_result_records": 1,
  "by_verdict_kind": {
    "Verified": 0,
    "Failed": 0,
    "PartialAccepted": 0,
    "SorryBlocked": 0,
    "_legacy_no_kind": 0,
    "_decode_error": 1
  }
}
```

## Architect §5 #1 direct check (chain_attempt_count == evaluator_reported_tx_count)

```
{
  "architect_inv_1": "chain_attempt_count == evaluator_reported_tx_count",
  "chain_attempt_count": 1,
  "evaluator_reported_tx_count": 1,
  "match": true,
  "attempt_outcomes": {},
  "delta": 0
}
```
