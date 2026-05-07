# TB-18R Phase 3 — P01_mathd_algebra_107

**Phase**: TB-18R Phase 3 (Technical Tape Validation on typed PartialAccepted substrate)
**Authority**: `handover/directives/2026-05-06_TB18R_PHASE_3_LAUNCH_DIRECTIVE.md`
**Date**: 2026-05-07T13-08-06Z
**Git HEAD**: 9007e1a5ff78ec23d8a211decf1b196c555f5bab
**Substrate**: HEAD includes Phase 2 commit `3f51667` (LeanVerdictKind + AttemptOutcome::PartialAccepted)

## Run params

- problem: `mathd_algebra_107`
- problem file: `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_107.lean`
- MAX_TRANSACTIONS: 12
- per-problem timeout: 1800s
- LLM proxy: http://localhost:8080
- active model: deepseek-chat
- condition: n1
- duration: 12s
- evaluator exit code: 0

## Architect §5 invariants (per-run signal)

- audit_tape verdict: `PROCEED`
- "invariant_verdict": "Ok"
- verdict_kind=PartialAccepted record count: 0 (multi-iteration tape-derived)

## R4 invariant verdict

```
{
  "attempt_aborted_count": 0,
  "capsule_anchored_attempt_count": 0,
  "delta": 0,
  "expected_completed_attempts": 1,
  "invariant_verdict": "Ok",
  "l4_work_attempt_count": 1,
  "l4e_work_attempt_count": 0,
  "preflight": "handover/ai-direct/TB-18R_R4_STEP_B_invariant.md",
  "tb_18r_r4_invariant_equation": "evaluator_reported_completed_llm_calls == l4_work_attempt_count + l4e_work_attempt_count + capsule_anchored_attempt_count",
  "tbc0_strict_audit_fix": "STRICT_AUDIT_TBC0_TAPE_2026-05-07.md Finding C — Bug 3 (capsule_anchored 3-term)",
  "terminal_halt_class": "OmegaAccepted"
}
```

## verdict_kind summary

```
{
  "cas_object_type_counts": {
    "ProposalPayload": 8,
    "Generic": 5,
    "LeanResult": 1,
    "AttemptTelemetry": 1
  },
  "lean_result_count": 1,
  "attempt_telemetry_count": 1,
  "tool_dist": {
    "step": 1,
    "omega_wtool": 1
  },
  "step_partial_ok_count": 0,
  "omega_wtool_count": 1,
  "step_reject_count": 0,
  "parse_fail_count": 0,
  "note": "verdict_kind decoded indirectly via id45 PASS in audit_tape (assert_45 4-arm typed match over every LeanResult); step_partial_ok > 0 indicates AttemptOutcome::PartialAccepted records emitted per Phase 2 directive \u00a75.2"
}
```

## Architect §5 #1 direct check (chain_attempt_count == evaluator_reported_tx_count)

```
{
  "architect_inv_1": "chain_attempt_count == evaluator_reported_completed_llm_calls",
  "chain_attempt_count": 1,
  "evaluator_reported_completed_llm_calls": 1,
  "evaluator_reported_tx_count_total": 1,
  "non_llm_tx_diagnostic_gap": 0,
  "match": true,
  "attempt_outcomes": {},
  "delta": 0,
  "resolution_ref": "handover/alignment/OBS_TB18R_INV1_NONLLM_TX_2026-05-07.md"
}
```
