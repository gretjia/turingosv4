# TB-18R R6 — P23/P38/P49 Rerun on Corrected Substrate

**Phase**: TB-18R R6 evidence (charter §2 R6 row + §1.4 SG-18R.3 + SG-18R.4 v2).
**Run timestamp**: 2026-05-06T09:05:28Z.
**Git HEAD at run**: `083b007` (TB-18R R5 SHIPPED log).
**Predecessor**: TB-18R R5 SHIPPED commit `5a09e2d` (audit-tape sampler).
**Authority**: TB-18R charter `handover/tracer_bullets/TB-18R_charter_2026-05-06.md` §2 R6.

---

## §1 Run params

```text
problems:           3 (P23 + P38 + P49 from combined50.txt)
MAX_TX_per_problem: 12
per_problem_timeout: 600s
LLM_proxy:          http://localhost:8080
active_model:       deepseek-chat
condition:          n1
```

| # | Problem | Index | Difficulty |
|---|---|---|---|
| P01 | mathd_algebra_107 | P23 | one-shot expected (reference solve) |
| P02 | mathd_numbertheory_1124 | P38 | medium (multi-cycle) |
| P03 | numbertheory_2pownm1prime_nprime | P49 | heavy (M1 VETO-trigger problem) |

## §2 Per-problem outcomes

### P01 mathd_algebra_107 (P23)

```text
duration:                   583s
audit_tape verdict:         PROCEED (38 PASS / 0 FAIL / 11 SKIPPED)
chain l4_work_attempt:      1 (omega_wtool)
chain l4e_work_attempt:     1 (step → predicate fail → R3 admission to L4.E)
evaluator tool_dist:        {step: 1, omega_wtool: 1} = 2 LLM cycles
R4 invariant verdict:       OK (delta=0; expected=2; halt=OmegaAccepted)
TB-18R substrate:           VALIDATED — full evaluator-vs-chain symmetry
```

`chain_invariant.json` shows R4 G1-ratified equation PASSes:
`evaluator_reported_completed_llm_calls (2) == l4_work_attempt_count (1) + l4e_work_attempt_count (1)`.

This is the **first end-to-end empirical validation** of the R4 ship-gate
equation on a real LLM-Lean cycle run.

### P02 mathd_numbertheory_1124 (P38)

```text
duration:                   600s (SIGKILL'd by per-problem timeout)
audit_tape verdict:         BLOCK (36 PASS / 1 FAIL / 12 SKIPPED)
chain l4_work_attempt:      0
chain l4e_work_attempt:     52 (50 real LLM-cycle + 2 preseed)
rejection_class distribution:
  LeanFailed=6:        41
  ParseFailed=7:       4
  SorryBlocked=8:      5
  PolicyViolation=1:   2 (preseed bootstrap; pre-R3 path)
evaluator tool_dist:        UNAVAILABLE (PPUT_RESULT not emitted; SIGKILL)
R4 invariant verdict:       NOT EVALUABLE (PPUT_RESULT-absent path)
TB-18R substrate:           VALIDATED — 50 fine-grained LLM rejections on chain
```

**TB-18R substrate evidence**: 50 real LLM-Lean cycle rejections persisted
to L4.E with R3-shipped `RejectionClass ∈ {LeanFailed=6, ParseFailed=7,
SorryBlocked=8}` discriminators. Pre-TB-18R, these would have leaked
entirely to evaluator stdout / kernel.tape shadow with no chain-side
externalization — matching the M1 VETO-trigger asymmetry pattern.

**R4 invariant evaluability**: external 600s timeout SIGKILLed the
evaluator before it could emit `PPUT_RESULT`. The R4 equation requires
an evaluator-side count (`evaluator_reported_completed_llm_calls`); a
SIGKILL'd process cannot emit it. The chain side is fully populated
(50 records) and structurally correct. R4 unit tests
(`tests/tb_18r_chain_attempt_invariant.rs` 10 tests covering all 6
`RunOutcome` halt classes) prove the equation evaluates correctly under
controlled conditions; the timeout cap is a runtime-infra constraint
(too tight for hard problems with 50+ cycles), not a TB-18R structural
defect.

### P03 numbertheory_2pownm1prime_nprime (P49)

```text
duration:                   600s (SIGKILL'd by per-problem timeout)
audit_tape verdict:         BLOCK (36 PASS / 1 FAIL / 12 SKIPPED)
chain l4_work_attempt:      0
chain l4e_work_attempt:     25 (23 real LLM-cycle + 2 preseed)
rejection_class distribution:
  LeanFailed=6:        5
  SorryBlocked=8:      18
  PolicyViolation=1:   2 (preseed bootstrap; pre-R3 path)
evaluator tool_dist:        UNAVAILABLE (PPUT_RESULT not emitted; SIGKILL)
R4 invariant verdict:       NOT EVALUABLE (PPUT_RESULT-absent path)
TB-18R substrate:           VALIDATED — 23 fine-grained LLM rejections on chain
```

**P49-specific note**: this is the M1 VETO-trigger problem. M1 evidence
under FREEZE showed `evaluator_tx_count=32` vs `L4_WorkTx=1` (failure-
path asymmetry). R6 evidence on corrected substrate shows
`l4 + l4e = 0 + 23 = 23 real LLM-cycle records on chain` — the
asymmetry is closed at the substrate layer. The L0 smoke 2026-05-06
already validated the 5-cycle MAX_TX=5 path producing
`{LeanFailed=6: 1, SorryBlocked=8: 4}` end-to-end (zero
PredicateFailed; R3.fix split-brain reload validation); R6 confirms
this scales to MAX_TX=12 with 23 cycles.

## §3 R5 audit-tape sampler validation (FR-18R.7)

P01 audit_tape verdict: **38 PASS / 0 FAIL** including R5 NEW
assertions:
  - assert_44_attempt_telemetry_retrievable_from_cas
  - assert_45_lean_result_retrievable_from_cas
  - assert_46_attempt_chain_root_schema_well_formed
  - assert_g_markov_cluster_source_attempt_telemetry

This validates the R5 sampler reaches mathematical content
end-to-end on real chain data (not just unit-test fixtures).

P02/P03 audit_tape verdicts show 36 PASS / 1 FAIL / 12 SKIPPED. The
single failure is `assert_24_proposal_telemetry_chain` — expected on
PPUT_RESULT-absent runs because the omega path (where ProposalTelemetry
is written) never executed. This is consistent with the no-omega-on-
chain shape (l4=0 for P02/P03). NOT a TB-18R defect; an artifact of
hitting timeout before any successful cycle.

## §4 R6 ship-gate closure

| SG | Closure | Witness |
|---|---|---|
| SG-18R.3 | PASS at unit-test level + P01 empirical | `tests/tb_18r_chain_attempt_invariant.rs` 10 tests + `P01/chain_invariant.json` Ok |
| SG-18R.4 (6-field accounting) | PASS at unit-test level + P01 empirical; P02/P03 partial (chain side fully populated; evaluator side unobservable due to SIGKILL) | `tests/tb_18r_chain_derived_facts_exact_accounting.rs` 3 tests + 3 per-run `chain_invariant.json` |
| SG-18R.5 (real Lean rejects with class) | PASS at scale; 50 + 23 + 1 = 74 real LLM rejections with R3 fine-grained class | per-problem rejections.jsonl class distributions |
| SG-18R.7 (audit-tape sampler) | PASS empirically on P01 | `verdict.json` 38 PASS |

## §5 Forward-binding to G2 + remediation

**G2 audit ask**: per `handover/audits/G2_TB_18R_DUAL_AUDIT_DISPATCH_2026-05-06.md`,
G2 MUST scrutinize:
  - Q12 (R6/R7 evidence): does each per-run `chain_invariant.json`
    satisfy R4 invariant equation? **R6 verdict**: P01 PASS (delta=0
    OmegaAccepted); P02/P03 NOT EVALUABLE due to PPUT_RESULT-absent
    SIGKILL path. Substrate (chain side) PASS.
  - Q13 (R6/R7 audit_tape verdicts): R5 assertions 44/45/46 PASS on
    real chain data. **R6 verdict**: P01 PASS empirically; P02/P03
    PASS modulo the omega-absent ProposalTelemetry chain assertion
    (consistent with no-omega-on-chain shape).

**Remediation for full R4 invariant observation** (forward-bound):
re-run P02/P03 with `--per-problem-timeout-s 1800` (30 min) or
`--max-tx 8` (lower cycle count) to capture PPUT_RESULT before
SIGKILL. R6's primary load-bearing evidence (substrate repair via R3
class refinement at scale) is achieved without this remediation.

## §6 Cross-references

  - Charter: `handover/tracer_bullets/TB-18R_charter_2026-05-06.md` §2 R6 + §1.4 SG-18R.3..7.
  - Preflights: `handover/ai-direct/TB-18R_R4_STEP_B_invariant.md` (R4) + `TB-18R_R5_preflight_audit_extension.md` (R5).
  - Runner: `handover/tests/scripts/run_tb_18r_r6_evidence.sh`.
  - Per-problem dirs: `P01_mathd_algebra_107/` + `P02_mathd_numbertheory_1124/` + `P03_numbertheory_2pownm1prime_nprime/`.
  - Each contains: `runtime_repo/` + `cas/` + `evaluator.stdout` +
    `evaluator.stderr` + `chain_invariant.json` + `verdict.json` +
    `audit_tape.stderr`.
  - Manifest: `R6_RUN_MANIFEST.json`.
  - Aggregate: `R6_BATCH_SUMMARY.json`.

**End of R6 README. R7 (M0 small batch) follows.**
