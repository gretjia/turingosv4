# CODEX TB-18R G2 Ship Audit - 2026-05-06

## 1. Header

- auditor: Codex
- date: 2026-05-06
- target: TB-18R Tape Restoration
- gate: G2
- HEAD: 3964957 (`TB-18R PROVISIONAL SHIPPED - R6+R7 evidence + R4 6/6 empirical PASS`)
- scope: R1..R7 source/evidence, R3 and R5 OBS forward-bindings, workspace test gate, no source edits.

## 2. Inputs Reviewed

- `handover/audits/G2_TB_18R_DUAL_AUDIT_DISPATCH_2026-05-06.md:1-252` (full dispatch; Q1..Q15 at `:136-209`).
- `handover/tracer_bullets/TB-18R_charter_2026-05-06.md:1-792` (full charter; R0 closure `:626-641`; forbidden fixes `:728-750`).
- `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md:1-649` (full predecessor; M1 grandfathering `:604-609`).
- `handover/audits/CODEX_TB_18R_CHARTER_RATIFICATION_2026-05-06.md:1-126` (full G1 audit).
- `handover/alignment/OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06.md:1-57` (full OBS; G2 binding `:40-46`).
- `handover/alignment/OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md:1-90` (full OBS; facts/decision/G2 binding `:14-30`, `:34-47`, `:63-74`).
- `handover/ai-direct/TB-18R_R1_STEP_B_schema.md:1-12`, `:147-154`; `handover/ai-direct/TB-18R_R3_STEP_B_admission.md:1-6`, `:38-45`, `:133-153`, `:211-239`; `handover/ai-direct/TB-18R_R3FIX_STEP_B_cas_reload.md:1-80`; `handover/ai-direct/TB-18R_R4_STEP_B_invariant.md:1-8`, `:38-58`, `:82-107`; `handover/ai-direct/TB-18R_R5_preflight_audit_extension.md:1-8`, `:21-63`, `:75-87`, `:112-126`, `:137-168`.
- `src/state/typed_tx.rs:183-247`; `src/bottom_white/cas/schema.rs:40-113`; `src/runtime/attempt_telemetry.rs:1-60`, `:76-170`, `:207-289`, `:373-435`; `src/bottom_white/cas/store.rs:182-252`; `src/bottom_white/ledger/rejection_evidence.rs:142-185`, `:253-273`; `src/state/sequencer.rs:400-502`, `:3042-3105`, `:3107-3123`; `src/runtime/chain_derived_run_facts.rs:715-798`, `:800-912`; `src/runtime/mod.rs:187-200`, `:482-507`; `src/runtime/audit_assertions.rs:2527-2665`, `:2760-2856`.
- `experiments/minif2f_v4/src/bin/evaluator.rs:61-230`, `:2520-2655`, `:3098-3225`, `:3485-3763`; `experiments/minif2f_v4/src/bin/comprehensive_arena.rs:188-210`, `:948-965`.
- Tests: `tests/tb_18r_rejection_class_repr_stability.rs:23-95`; `tests/tb_18r_attempt_chain_root_payload_schema.rs:150-185`; `tests/tb_18r_dashboard_attempt_dag_replay.rs:1-38`; `experiments/minif2f_v4/tests/tb_16_comprehensive_arena_smoke.rs:56-72`.
- Evidence: `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/R6_BATCH_SUMMARY.json:1-82`; `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P01_mathd_algebra_107/chain_invariant.json:1-11`; `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P02_mathd_numbertheory_1124/chain_invariant.json:1-21`, `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P02_mathd_numbertheory_1124/verdict.json:309-390`; `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P03_numbertheory_2pownm1prime_nprime/chain_invariant.json:1-20`, `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P03_numbertheory_2pownm1prime_nprime/verdict.json:309-390`; `handover/evidence/tb_18r_r7_m0_2026-05-06/R7_BATCH_SUMMARY.json:1-94`; `handover/evidence/tb_18r_r7_m0_2026-05-06/P01_mathd_algebra_113/chain_invariant.json:1-11`; `handover/evidence/tb_18r_r7_m0_2026-05-06/P02_mathd_algebra_114/chain_invariant.json:1-11`, `handover/evidence/tb_18r_r7_m0_2026-05-06/P02_mathd_algebra_114/verdict.json:309-390`; `handover/evidence/tb_18r_r7_m0_2026-05-06/P03_mathd_algebra_125/chain_invariant.json:1-11`; `handover/evidence/tb_18r_r7_m0_2026-05-06/P04_mathd_algebra_141/chain_invariant.json:1-11`; `handover/evidence/tb_18r_r7_m0_2026-05-06/P05_aime_1983_p2/chain_invariant.json:1-11`.
- Commands: `git show --no-patch --format='%h %s' 9f8ce1f 35389d0 72a1b75 2ca1aed d34f428 5a09e2d 3964957`; `git status --short -- src experiments/minif2f_v4/src/bin/evaluator.rs experiments/minif2f_v4/src/bin/comprehensive_arena.rs tests handover/audits/CODEX_TB_18R_G2_SHIP_AUDIT_2026-05-06.md` (no output); `git diff --name-status 5338cea..HEAD -- handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z` (no output); `git show --name-status --oneline --no-renames 5338cea -- handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z`.
- Workspace test: `cargo test --workspace` at HEAD `3964957` returned exit 101 before producing the claimed `1047/1/150` workspace count. Observed failing integration test binary count: 1 passed / 1 failed / 0 ignored; failing test `comprehensive_arena_plan_only_emits_plan`. Source expects `ARENA_PLAN.md` at `experiments/minif2f_v4/tests/tb_16_comprehensive_arena_smoke.rs:56-72`, while the binary only prints the plan and exits on `--plan-only` at `experiments/minif2f_v4/src/bin/comprehensive_arena.rs:191-205`, `:958-962`.

## 3. Per-Question Verdicts Q1..Q15

### Q1 - Does R1 schema (AttemptTelemetry + LeanResult + TerminalAbortRecord) preserve `feedback_chaintape_externalized_proposal` ("1 LLM call = 1 Attempt Node") per charter Q2 PASS?

PASS. R1 creates per-cycle `AttemptTelemetry` with `AttemptKind::ExternalizedLlmCycle = 0`, documented as one LLM call to one attempt node, and keeps outcome separate (`src/runtime/attempt_telemetry.rs:98-168`). The schema carries `schema_version`, `attempt_id`, parsed `candidate_payload_cid`, `attempt_kind`, `outcome`, and optional `attempt_chain_root` (`src/runtime/attempt_telemetry.rs:207-289`). CAS object tags include `AttemptTelemetry`, `LeanResult`, and `TerminalAbortRecord` (`src/bottom_white/cas/schema.rs:90-110`). Commit: `9f8ce1f`.

### Q2 - Does R2 evaluator wire-up at all 6 paths preserve CR-18R.4 v2 (no raw LLM response in candidate_payload_cid; parsed external candidate bytes only; fixed sentinels on parse_fail / llm_err)?

PASS. The helper states raw LLM response is never passed and writes only `candidate_bytes` to CAS (`experiments/minif2f_v4/src/bin/evaluator.rs:82-122`). Omega-full uses parsed `payload.as_bytes()` (`:2533-2561`), omega-pertactic uses `tactic.as_bytes()` (`:3108-3134`), step_partial_ok uses `tactic.as_bytes()` (`:3504-3520`), step_reject uses `tactic.as_bytes()` (`:3581-3598`), parse_fail uses fixed sentinel `b"tb-18r-parse-fail-no-candidate"` (`:3641-3675`), and llm_err uses fixed sentinel `b"tb-18r-llm-err-no-candidate"` (`:3706-3739`). Commit: `35389d0`.

### Q3 - Does R3 RejectionClass tail-append {6,7,8,9} preserve byte-stable canonical hash for pre-R3 L4.E rows?

PASS. Existing variants remain `0..5`, new variants are tail-appended as `LeanFailed=6`, `ParseFailed=7`, `SorryBlocked=8`, `LlmError=9` (`src/bottom_white/ledger/rejection_evidence.rs:142-185`). L4.E hash uses `rejection_class as u8` (`src/bottom_white/ledger/rejection_evidence.rs:253-273`), and tests pin both pre-R3 and R3 ranges as disjoint (`tests/tb_18r_rejection_class_repr_stability.rs:23-95`). Commit: `72a1b75`.

### Q4 - R3 section 3.5 omega-path NO cutover: does this deviation align with Art.0.2 + Art.III.4 + TB-7 audit-chain backward compatibility? Or should omega path also cut over?

PASS. Omega path should not cut over at TB-18R. Revised R3 preflight requires omega `WorkTx.proposal_cid` to stay as ProposalTelemetry CID to preserve TB-7 audit walks (`handover/ai-direct/TB-18R_R3_STEP_B_admission.md:133-153`). Source matches: omega-full writes AttemptTelemetry to CAS (`experiments/minif2f_v4/src/bin/evaluator.rs:2533-2561`) but accepted `WorkTx` uses `tel_cid` from ProposalTelemetry (`:2615-2624`, `:2642-2650`); omega-pertactic does the same (`:3108-3134`, `:3186-3197`, `:3213-3221`). This is the correct compatibility choice.

### Q5 - R3 section 1.3 step_partial_ok CAS-only: is LeanPass-on-rejection-fence-respect correctly applied?

PASS. step_partial_ok is explicitly CAS-only, because it is `LeanPass` intermediate progress and must not enter the predicate-failure L4.E arm (`experiments/minif2f_v4/src/bin/evaluator.rs:3485-3494`). It writes AttemptTelemetry with `outcome=LeanPass` and `lean_result: Some((0, false))` but does not call `r3_emit_failure_path_worktx` in that block (`experiments/minif2f_v4/src/bin/evaluator.rs:3504-3527`). The sequencer guards against `LeanPass` reaching the rejection arm (`src/state/sequencer.rs:482-500`).

### Q6 - Does R3.fix CasStore::reload_index_from_sidecar correctly close the stale-cache bug without introducing new race conditions?

PASS. `reload_index_from_sidecar` reloads sidecar metadata into the in-memory index, idempotently and without disk writes (`src/bottom_white/cas/store.rs:182-215`). The refine helper performs the requested sequence: read attempt, on miss acquire write lock and reload, drop it, then reacquire read and retry (`src/state/sequencer.rs:448-470`). Single-writer sequencing is the standing assumption for logical time and commit critical section (`src/state/sequencer.rs:3107-3123`). Commit: `2ca1aed`.

### Q7 - R4 G1 equation populated verbatim: no alteration; exact equation, abort auxiliary, negative delta forbidden.

PASS. R4 documents and implements `evaluator_reported_completed_llm_calls == l4_work_attempt_count + l4e_work_attempt_count` with no tolerance (`src/runtime/chain_derived_run_facts.rs:715-741`). Code forbids negative delta under any halt (`:747-755`), requires clean halt `delta == 0` and no aborts (`:756-773`), and requires abort halt `expected_completed_attempts + attempt_aborted_count == l4_work_attempt_count + l4e_work_attempt_count` (`:775-795`). Commit: `d34f428`.

### Q8 - R4 drain barrier via ChaintapeBundle::shutdown() without new Sequencer public method.

PASS. `ChaintapeBundle::shutdown()` sends shutdown and awaits the driver handle (`src/runtime/mod.rs:187-200`). The driver closes the queue and drains all remaining submissions before return (`src/runtime/mod.rs:482-507`). R4's quiescence witness checks `next_submit_id - 1 == l4_count + l4e_count` post-drain (`src/runtime/chain_derived_run_facts.rs:800-838`). R4 preflight explicitly treats this as the "or equivalent" drain barrier (`handover/ai-direct/TB-18R_R4_STEP_B_invariant.md:82-107`).

### Q9 - R5 sampler privacy fence: does assert_44 avoid byte inspection of candidate_payload and never expose bytes?

PASS. assert_44 states it does not inspect candidate payload bytes (`src/runtime/audit_assertions.rs:2527-2535`). The implementation decodes `AttemptTelemetry`, calls `t.cas.get(&att.candidate_payload_cid).is_err()`, and emits only the CID in failure detail (`src/runtime/audit_assertions.rs:2550-2577`). It never logs or returns candidate bytes.

### Q10 - R5 attempt_chain_root schema validity: is Option<Hash> round-trip + zero-bytes admissible and not requiring omega-path WorkTx population?

PASS. `AttemptTelemetry.attempt_chain_root` is `Option<Hash>`, `None` for non-terminal attempts and not a `WorkTx` field (`src/runtime/attempt_telemetry.rs:279-288`; `src/state/typed_tx.rs:223-247`). assert_46 is schema-only and explicitly does not require omega-path population because omega `WorkTx.proposal_cid` stays ProposalTelemetry (`src/runtime/audit_assertions.rs:2623-2663`). The WorkTx canonical-shape test asserts TB-18R did not add `attempt_chain_root` to `WorkTx` (`tests/tb_18r_attempt_chain_root_payload_schema.rs:156-180`).

### Q11 - SG-18R.9 smoke-level closure: is R5 dashboard binary smoke acceptable for ship-gate or must full DAG render land before SHIPPED FINAL?

CHALLENGE. Full DAG render can remain OBS-forwarded as a presentation-tier refactor if the load-bearing tape/CAS assertions pass; OBS estimates the render at 4-6h and calls the load-bearing invariant assert_44/45/46 (`handover/alignment/OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md:20-30`, `:44-47`, `:72-84`). But the actual R5 smoke test does not invoke `audit_dashboard`; it only checks `Cargo.toml` and `src/bin/audit_dashboard.rs` exist (`tests/tb_18r_dashboard_attempt_dag_replay.rs:17-37`). Remediation: replace the smoke with an actual dashboard invocation against TB-18R evidence, or land the full DAG render if SG-18R.9 is interpreted literally.

### Q12 - R6/R7 evidence: does each per-run chain_invariant.json satisfy R4 invariant equation?

VETO. R7 satisfies the equation on all five runs: each `chain_invariant.json` has `delta=0`, matching `expected_completed_attempts == l4_work_attempt_count + l4e_work_attempt_count`, and clean halt classes (`handover/evidence/tb_18r_r7_m0_2026-05-06/P01_mathd_algebra_113/chain_invariant.json:1-11`; `handover/evidence/tb_18r_r7_m0_2026-05-06/P02_mathd_algebra_114/chain_invariant.json:1-11`; `handover/evidence/tb_18r_r7_m0_2026-05-06/P03_mathd_algebra_125/chain_invariant.json:1-11`; `handover/evidence/tb_18r_r7_m0_2026-05-06/P04_mathd_algebra_141/chain_invariant.json:1-11`; `handover/evidence/tb_18r_r7_m0_2026-05-06/P05_aime_1983_p2/chain_invariant.json:1-11`). R6 P01 also passes (`handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P01_mathd_algebra_107/chain_invariant.json:1-11`). R6 P02 and P03 are not evaluable: both were externally SIGKILLed before `PPUT_RESULT`, so evaluator-side counts are unavailable and `r4_invariant_equation_evaluable=false` (`handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P02_mathd_numbertheory_1124/chain_invariant.json:1-21`; `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P03_numbertheory_2pownm1prime_nprime/chain_invariant.json:1-20`). This fails the "each per-run" ship-gate condition. Remediation: rerun P38/P49 with the evidence-prescribed longer timeout (`handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P02_mathd_numbertheory_1124/chain_invariant.json:18`; `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P03_numbertheory_2pownm1prime_nprime/chain_invariant.json:17`) or emit TerminalAbortRecord-backed abort evidence.

### Q13 - R6/R7 evidence: does audit_tape verdict.json show R5 assertions 44/45/46 PASS on real chain data?

VETO. No. R6 P02 and P03 both show id44 PASS, id45 FAIL, id46 PASS, and verdict BLOCK (`handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P02_mathd_numbertheory_1124/verdict.json:309-390`; `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P03_numbertheory_2pownm1prime_nprime/verdict.json:309-390`). R7 P02 also shows id45 FAIL and verdict BLOCK (`handover/evidence/tb_18r_r7_m0_2026-05-06/P02_mathd_algebra_114/verdict.json:309-390`). The source cause is a real assertion/schema mismatch: `LeanResult` allows `verified=false` for partial verdict even when `exit_code=0` (`src/runtime/attempt_telemetry.rs:387-392`), step_partial_ok writes exactly `lean_result: Some((0, false))` (`experiments/minif2f_v4/src/bin/evaluator.rs:3518-3520`), but assert_45 requires `verified == (exit_code == 0)` (`src/runtime/audit_assertions.rs:2580-2621`). Remediation: make assert_45 partial-verdict-aware, then rerun audit_tape on R6/R7 evidence.

### Q14 - No retroactive M1 evidence rewrite: verify R0-R7 diffs do NOT modify pre-TB-18R M1 evidence.

CHALLENGE. R1-R7 after R0 do not modify the M1 evidence directory: `git diff --name-status 5338cea..HEAD -- handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z` produced no paths. However the literal "R0-R7 diffs do NOT modify" condition is false for R0: `5338cea` modified `MINIF2F_M1_BENCHMARK_REPORT.md` and added `README.md` in that directory. Charter authorizes those as grandfathering annotation/banner only (`handover/tracer_bullets/TB-18R_charter_2026-05-06.md:626-639`), and the VETO predecessor permits a README annotation while forbidding relabel/migration (`handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md:604-609`). Remediation: record the R0 annotation exception explicitly in the G2 ship packet so "no rewrite" is not misread as "zero path changes."

### Q15 - Class-4 carve-out compliance: R1 + R3 + R3.fix + R4 STEP_B; no Class-4 surface hidden inside R2 + R5.

PASS. Class-4 atoms have preflight records: R1 schema Class-4 STEP_B (`handover/ai-direct/TB-18R_R1_STEP_B_schema.md:1-6`, `:147-154`), R3 admission/RejectionClass STEP_B (`handover/ai-direct/TB-18R_R3_STEP_B_admission.md:1-6`, `:211-239`), R3.fix sequencer/CAS reload STEP_B (`handover/ai-direct/TB-18R_R3FIX_STEP_B_cas_reload.md:1-8`, `:40-55`), and R4 invariant Class-4 STEP_B (`handover/ai-direct/TB-18R_R4_STEP_B_invariant.md:1-8`, `:38-58`). R2's inspected evaluator changes are instrumentation and failure-path submission under the R3-ratified admission shape (`experiments/minif2f_v4/src/bin/evaluator.rs:176-230`, `:3552-3617`, `:3641-3699`, `:3706-3763`); R5 is audit assertion/test plumbing and explicitly Class-3 (`handover/ai-direct/TB-18R_R5_preflight_audit_extension.md:1-8`, `:137-168`). Documentation cleanup is still needed: R3 preflight has stale cutover text at `handover/ai-direct/TB-18R_R3_STEP_B_admission.md:38-45`, `:221-225`, contradicted by amended section `:133-153`.

## 4. OBS Forward-Binding Rulings

- `OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06`: CLOSED FOR R3 MECHANISM, NOT FOR SHIP. This G2 audit covered the two required deviations (`handover/alignment/OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06.md:40-46`). Omega NO-cutover is correct and source-backed (Q4). step_partial_ok CAS-only is correct and source-backed (Q5). R3.fix stale-cache reload is source-backed (Q6). Overall ship still VETOs on Q12/Q13/workspace tests, not on R3's two deviations.
- `OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06`: CHALLENGE. Full DAG render can be forward-bound as presentation-tier if load-bearing assertions pass (`handover/alignment/OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md:44-47`, `:72-84`). They do not pass on real evidence (Q13), and the smoke test does not invoke the dashboard binary (`tests/tb_18r_dashboard_attempt_dag_replay.rs:17-37`). SG-18R.9 is not final-clean.

## 5. Overall Verdict

VETO.

Blockers:

1. Q12: R6 P38/P49 chain invariant evidence is `NotEvaluable` / `r4_invariant_equation_evaluable=false` after external SIGKILL before evaluator count emission (`handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P02_mathd_numbertheory_1124/chain_invariant.json:1-21`; `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P03_numbertheory_2pownm1prime_nprime/chain_invariant.json:1-20`).
2. Q13: R6 P02/P03 and R7 P02 audit_tape `verdict.json` show id45 FAIL and verdict BLOCK, not 44/45/46 all PASS (`handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P02_mathd_numbertheory_1124/verdict.json:309-390`; `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P03_numbertheory_2pownm1prime_nprime/verdict.json:309-390`; `handover/evidence/tb_18r_r7_m0_2026-05-06/P02_mathd_algebra_114/verdict.json:309-390`).
3. Workspace test gate: `cargo test --workspace` failed with exit 101, observed 1 passed / 1 failed / 0 ignored in the failing integration test binary, and did not produce the claimed `1047/1/150` workspace count. The failing expectation is source-visible at `experiments/minif2f_v4/tests/tb_16_comprehensive_arena_smoke.rs:56-72`; the binary only prints and exits at `experiments/minif2f_v4/src/bin/comprehensive_arena.rs:191-205`, `:958-962`.

## 6. Suggested Remediation Atoms

1. `R5.fix-assert45-partial-verdict`: change assert_45 from `verified <-> exit_code == 0` to a partial-verdict-aware invariant consistent with `LeanResult` (`src/runtime/attempt_telemetry.rs:387-392`) and step_partial_ok (`experiments/minif2f_v4/src/bin/evaluator.rs:3518-3520`), then rerun audit_tape on all R6/R7 evidence.
2. `R6.fix-p38-p49-evaluable`: rerun R6 P38/P49 with `--per-problem-timeout-s 1800` or add TerminalAbortRecord-backed external-timeout evidence so Q12 is evaluable (`handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P02_mathd_numbertheory_1124/chain_invariant.json:18`; `handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/P03_numbertheory_2pownm1prime_nprime/chain_invariant.json:17`).
3. `workspace-test-fix`: make `comprehensive_arena --plan-only` write `ARENA_PLAN.md`, or correct the smoke test to match stderr-only behavior; rerun exact `cargo test --workspace`.
4. `R5.dashboard-smoke-fix`: replace the current file-existence test with an actual `audit_dashboard` invocation on TB-18R-shape evidence, or land the full attempt-DAG render (`tests/tb_18r_dashboard_attempt_dag_replay.rs:17-37`; `handover/alignment/OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md:20-30`).
5. `G2-doc-cleanup`: document the R0 M1 grandfathering annotation exception and remove stale omega-cutover wording from R3 preflight (`handover/tracer_bullets/TB-18R_charter_2026-05-06.md:626-639`; `handover/ai-direct/TB-18R_R3_STEP_B_admission.md:38-45`, `:221-225`, `:133-153`).
