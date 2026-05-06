# TB-18R Phase 3 — P04_mathd_algebra_113

**Phase**: TB-18R Phase 3 (Technical Tape Validation on typed PartialAccepted substrate)
**Authority**: `handover/directives/2026-05-06_TB18R_PHASE_3_LAUNCH_DIRECTIVE.md`
**Date**: 2026-05-06T14-13-55Z
**Git HEAD**: 55a0935213a13b87556da1a57540117cec3438af
**Substrate**: HEAD includes Phase 2 commit `3f51667` (LeanVerdictKind + AttemptOutcome::PartialAccepted)

## Run params

- problem: `mathd_algebra_113`
- problem file: `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_113.lean`
- MAX_TRANSACTIONS: 12
- per-problem timeout: 1800s
- LLM proxy: http://localhost:8080
- active model: deepseek-chat
- condition: n1
- duration: 83s
- evaluator exit code: 0

## Architect §5 invariants (per-run signal)

- audit_tape verdict: `PROCEED`
- "invariant_verdict": "Err(TB-18R FR-18R.3 violation: delta<0 forbidden (delta=-2, halt=MaxTxExhausted) — attempt vanished pre-chain)"
- verdict_kind=PartialAccepted record count: 0 (multi-iteration tape-derived)

## R4 invariant verdict

```
{
  "attempt_aborted_count": 0,
  "delta": -2,
  "expected_completed_attempts": 12,
  "invariant_verdict": "Err(TB-18R FR-18R.3 violation: delta<0 forbidden (delta=-2, halt=MaxTxExhausted) — attempt vanished pre-chain)",
  "l4_work_attempt_count": 0,
  "l4e_work_attempt_count": 10,
  "preflight": "handover/ai-direct/TB-18R_R4_STEP_B_invariant.md",
  "tb_18r_r4_invariant_equation": "evaluator_reported_completed_llm_calls == l4_work_attempt_count + l4e_work_attempt_count",
  "terminal_halt_class": "MaxTxExhausted"
}
```

## verdict_kind summary

```
{
  "cas_object_type_counts": {
    "ProposalPayload": 15,
    "Generic": 4,
    "LeanResult": 9,
    "AttemptTelemetry": 9,
    "CompressedRunLog": 1,
    "EvidenceManifest": 1,
    "EvidenceCapsule": 1
  },
  "lean_result_count": 9,
  "attempt_telemetry_count": 9,
  "tool_dist": {
    "step_reject": 9,
    "step": 9
  },
  "step_partial_ok_count": 0,
  "omega_wtool_count": 0,
  "step_reject_count": 9,
  "parse_fail_count": 0,
  "note": "verdict_kind decoded indirectly via id45 PASS in audit_tape (assert_45 4-arm typed match over every LeanResult); step_partial_ok > 0 indicates AttemptOutcome::PartialAccepted records emitted per Phase 2 directive \u00a75.2"
}
```

## Architect §5 #1 direct check (chain_attempt_count == evaluator_reported_tx_count)

```
{
  "architect_inv_1": "chain_attempt_count == evaluator_reported_tx_count",
  "chain_attempt_count": 9,
  "evaluator_reported_tx_count": 12,
  "match": false,
  "attempt_outcomes": {},
  "delta": -3
}
```
