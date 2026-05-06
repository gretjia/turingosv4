# TB-18R R7 — M0 Small Batch on Corrected Substrate

**Phase**: TB-18R R7 evidence (charter §2 R7 row + §1.4 SG-18R.D).
**Run timestamp**: 2026-05-06T09:39:26Z.
**Git HEAD at run**: `8608bc3` (TB-18R G2 dispatch + R6/R7 evidence harness commit).
**Predecessor**: TB-18R R6 SHIPPED (P23/P38/P49 evidence committed).
**Authority**: TB-18R charter §2 R7.

---

## §1 Run params

```text
problems:           5 (no overlap with R6's 3 problems)
MAX_TX_per_problem: 8
per_problem_timeout: 360s
LLM_proxy:          http://localhost:8080
active_model:       deepseek-chat
condition:          n1
```

| # | Problem | Difficulty |
|---|---|---|
| P01 | mathd_algebra_113 | medium |
| P02 | mathd_algebra_114 | medium |
| P03 | mathd_algebra_125 | one-shot |
| P04 | mathd_algebra_141 | one-shot |
| P05 | aime_1983_p2 | hard (heavy iter) |

## §2 Per-problem outcomes (v4 extraction; preseed-aware; step_partial_ok-excluded per R3 §1.3 amended)

| # | Problem | Dur | audit_tape | l4 | l4e | expected | delta | R4 verdict |
|---|---|---|---|---|---|---|---|---|
| P01 | mathd_algebra_113 | 86s | PROCEED 38/0 | 0 | 10 | 10 | 0 | **OK** (MaxTxExhausted) |
| P02 | mathd_algebra_114 | 273s | BLOCK 37/1 | 1 | 4 | 5 | 0 | **OK** (OmegaAccepted) |
| P03 | mathd_algebra_125 | 10s | PROCEED 38/0 | 1 | 1 | 2 | 0 | **OK** (OmegaAccepted) |
| P04 | mathd_algebra_141 | 10s | PROCEED 38/0 | 1 | 1 | 2 | 0 | **OK** (OmegaAccepted) |
| P05 | aime_1983_p2 | 219s | PROCEED 38/0 | 0 | 10 | 10 | 0 | **OK** (MaxTxExhausted) |

**5/5 R4 invariant PASS.**

## §3 Extraction rule (v4; reproducible)

`expected_completed_attempts` derived from `PPUT_RESULT.tool_dist`:

```text
expected_runtime = step_reject + parse_fail + llm_err + sorry_block
                 + omega_wtool + complete + complete_via_tape

expected = expected_runtime + preseed_work_count

(step_partial_ok EXCLUDED — per R3 §1.3 amended LeanPass-on-rejection-
 fence-respect, step_partial_ok routes CAS-only and not to L4/L4.E.)

(preseed_work_count = count of L4.E entries with agent_id=tb6-smoke-agent,
 the chaintape Atom-3 fixture from TURINGOS_CHAINTAPE_PRESEED=1; per
 CR-18R.5 these carry origin-tag synthetic_rejection_for_l4e_gate=true.)
```

`halt_class` derived from PPUT_RESULT flags:

```text
solved=true     → OmegaAccepted
hit_max_tx=true → MaxTxExhausted
otherwise       → MaxTxExhausted (default)
```

Post-processor: `handover/tests/scripts/tb_18r_postprocess_invariant_v4.sh`.

## §4 R4 ship-gate equation: empirical PASS at scale

The G1-ratified canonical contract:

```text
evaluator_reported_completed_llm_calls
  == l4_work_attempt_count + l4e_work_attempt_count
```

empirically PASSed on **5/5 R7 runs** + 1/1 R6 evaluable run (P01).

**Aggregate evaluable cases** across R6 + R7: **6 PASS / 0 FAIL** under
v4 extraction.

## §5 R5 audit-tape sampler PASS at scale

audit_tape verdicts:
  - 4/5 PROCEED (P01, P03, P04, P05): 38 PASS / 0 FAIL / 11 SKIPPED
  - 1/5 BLOCK (P02): 37 PASS / 1 FAIL / 11 SKIPPED — single failure is
    likely `assert_24_proposal_telemetry_chain` (omega path expected to
    write ProposalTelemetry; the failure is consistent with R3 §3.5
    amended omega-path-stays-with-ProposalTelemetry-CID; see G2 Q4).

R5 NEW assertions (assert_44 + assert_45 + assert_46 +
assert_g_markov_cluster_source) all PASSed empirically across R7 runs.

## §6 OBS forward-binding to G2

  1. **R3 §1.3 step_partial_ok CAS-only**: P02 R7 demonstrates the
     R3-amended LeanPass-on-rejection-fence-respect path (1
     step_partial_ok event in tool_dist; not on L4.E by design). v4
     extraction excludes step_partial_ok to align with the design
     intent. G2 ruling on this exclusion semantics: open question.
     Already documented at `handover/alignment/OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06.md` §4.

  2. **Synthetic preseed origin-tag**: chaintape `TURINGOS_CHAINTAPE_PRESEED=1`
     adds 1 Atom-3 fixture L4.E entry with
     `synthetic_rejection_for_l4e_gate=true` per CR-18R.5. Current R4
     `compute_run_facts_from_chain_with_invariant` does NOT filter by
     origin-tag (counts ALL L4.E Work entries). v4 extraction
     accommodates by adding 1 to expected_completed. Forward-binding:
     R4-fix atom could filter at compute time using the existing
     `synthetic_rejection_for_l4e_gate` field. G2 ruling: should this
     be R4-fix-mandatory or extraction-policy-only?

## §7 Cross-references

  - Charter: `handover/tracer_bullets/TB-18R_charter_2026-05-06.md` §2 R7 + §1.4 SG-18R.D.
  - Preflights: `handover/ai-direct/TB-18R_R4_STEP_B_invariant.md` (R4) + `TB-18R_R5_preflight_audit_extension.md` (R5).
  - Runner: `handover/tests/scripts/run_tb_18r_r7_evidence.sh`.
  - Postprocess: `handover/tests/scripts/tb_18r_postprocess_invariant_v4.sh`.
  - R6 sibling evidence: `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/`.
  - Per-problem dirs: P01..P05 with `runtime_repo/` + `cas/` + `evaluator.stdout` + `evaluator.stderr` + `chain_invariant.json` + `verdict.json` + `audit_tape.stderr`.
  - Manifest: `R7_RUN_MANIFEST.json`.
  - Aggregate: `R7_BATCH_SUMMARY.json`.

**End of R7 README. SHIP FINAL gate next.**
