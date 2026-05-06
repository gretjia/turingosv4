# TB-18R G2 Dual Audit — Merged Verdict

**Date**: 2026-05-06
**Gate**: G2 (final ship audit; charter §2 G2 row + SG-18R.13)
**HEAD audited**: `3964957` (TB-18R PROVISIONAL SHIPPED)
**Conservative ranking**: VETO > CHALLENGE > PASS per `feedback_dual_audit_conflict`.

---

## §0 Merged Overall Verdict

**VETO**.

Codex (source + test-grounded) returned VETO on three hard blockers; Gemini (text/schema inspection) returned 15/15 PASS. Per conservative ranking and `feedback_dual_audit_conflict`, **VETO wins** — Codex's blockers are real and grounded in actual file:line + workspace test execution that Gemini did not perform.

SHIPPED FINAL is **blocked**. `TB-18R PROVISIONAL SHIPPED 2026-05-06 commit 3964957` does **not** clear G2 in current state. Remediation atoms required before re-audit.

---

## §1 Per-Auditor Summary

| Auditor | Model | Output file | Overall |
|---|---|---|---|
| Codex | (codex-rescue subagent) | `handover/audits/CODEX_TB_18R_G2_SHIP_AUDIT_2026-05-06.md` | **VETO** |
| Gemini | gemini-2.5-pro | `handover/audits/GEMINI_TB_18R_G2_SHIP_AUDIT_2026-05-06.md` | PASS |

**Method asymmetry**: Codex inspected actual source files at file:line, ran `cargo test --workspace`, and read the per-run `chain_invariant.json` + `verdict.json` evidence end-to-end. Gemini reviewed the inlined dispatch + charter + concatenated source slices but did not execute tests. Codex's findings are runtime-validated; Gemini's are static.

---

## §2 Per-Question Merged Verdicts (Q1..Q15)

| Q | Codex | Gemini | Merged | Notes |
|---|---|---|---|---|
| Q1 | PASS | PASS | **PASS** | R1 schema preserves 1-LLM-call=1-AttemptNode. |
| Q2 | PASS | PASS | **PASS** | R2 6-path wire-up; parsed bytes + fixed sentinels. |
| Q3 | PASS | PASS | **PASS** | RejectionClass tail-append {6,7,8,9}; canonical hash stable. |
| Q4 | PASS | PASS | **PASS** | Omega NO-cutover correct (TB-7 audit chain compat). |
| Q5 | PASS | PASS | **PASS** | step_partial_ok CAS-only (LeanPass fence respected). |
| Q6 | PASS | PASS | **PASS** | `reload_index_from_sidecar` correct; single-writer guarantees no deadlock. |
| Q7 | PASS | PASS | **PASS** | R4 G1 equation populated verbatim; no tolerance; abort auxiliary present. |
| Q8 | PASS | PASS | **PASS** | `ChaintapeBundle::shutdown()` satisfies "or equivalent" drain barrier. |
| Q9 | PASS | PASS | **PASS** | assert_44 privacy fence preserved (CID-only failure detail). |
| Q10 | PASS | PASS | **PASS** | `attempt_chain_root` Option<Hash> schema admissible; not on omega WorkTx. |
| **Q11** | **CHALLENGE** | PASS | **CHALLENGE** | Dashboard smoke is file-existence only; does not invoke `audit_dashboard`. |
| **Q12** | **VETO** | PASS | **VETO** | R6 P02/P03 `r4_invariant_equation_evaluable=false` (external SIGKILL before evaluator count emission). |
| **Q13** | **VETO** | PASS | **VETO** | R6 P02/P03 + R7 P02 `verdict.json` show **id45 FAIL → BLOCK** (assert_45 vs step_partial_ok semantic mismatch). |
| **Q14** | CHALLENGE | PASS | **CHALLENGE** | R0 (`5338cea`) modified `MINIF2F_M1_BENCHMARK_REPORT.md` + added README — authorized as grandfathering annotation but the literal "no path changes" claim is false; needs explicit exception in ship packet. |
| Q15 | PASS | PASS | **PASS** | Class-4 carve-out clean; minor doc-cleanup needed on R3 preflight stale cutover wording. |

**Tally (merged)**: 11 PASS / 2 CHALLENGE / **2 VETO**.

Workspace test gate (charter SG-18R.13 ship dependency) **failed independently of Q1..Q15**: `cargo test --workspace` returned exit 101 (`comprehensive_arena_plan_only_emits_plan`). This is a third blocker per Codex §5.

---

## §3 Blockers (must clear before SHIPPED FINAL)

### Blocker 1 — Q13: assert_45 partial-verdict semantic mismatch
**Auditor**: Codex.
**Symptom**: R6 P02/P03 + R7 P02 `audit_tape verdict.json` shows `id45 FAIL` and overall `verdict: BLOCK`.
**Root cause**: `LeanResult` allows `verified=false` for partial verdict even when `exit_code=0` (`src/runtime/attempt_telemetry.rs:387-392`); step_partial_ok writes exactly `lean_result: Some((0, false))` (`experiments/minif2f_v4/src/bin/evaluator.rs:3518-3520`); but `assert_45` requires `verified == (exit_code == 0)` (`src/runtime/audit_assertions.rs:2580-2621`). assert_45 is wrong; the runtime semantics are correct.
**Remediation atom**: `R5.fix-assert45-partial-verdict` — make assert_45 partial-verdict-aware; rerun audit_tape on all R6/R7 evidence.
**Class**: 3 (audit-assertion only).

### Blocker 2 — Q12: R6 P02/P03 not evaluable
**Auditor**: Codex.
**Symptom**: `chain_invariant.json` shows `r4_invariant_equation_evaluable=false` because `numbertheory_1124` and `numbertheory_2pownm1prime_nprime` were externally SIGKILLed before `PPUT_RESULT` emission.
**Root cause**: per-problem timeout too short for these two heavy problems.
**Remediation atom**: `R6.fix-p38-p49-evaluable` — rerun with `--per-problem-timeout-s 1800` (the evidence already names this remediation in the chain_invariant.json suggestions block) **or** emit `TerminalAbortRecord`-backed external-timeout evidence + named abort halt class.
**Class**: 3 (evidence rerun).

### Blocker 3 — Workspace test gate
**Auditor**: Codex.
**Symptom**: `cargo test --workspace` fails with exit 101 at HEAD `3964957`. Test `comprehensive_arena_plan_only_emits_plan` (`experiments/minif2f_v4/tests/tb_16_comprehensive_arena_smoke.rs:56-72`) expects `ARENA_PLAN.md` to be written; the binary only prints to stdout and exits (`experiments/minif2f_v4/src/bin/comprehensive_arena.rs:191-205`, `:958-962`).
**Root cause**: test/binary contract drift; one of the two must change.
**Remediation atom**: `workspace-test-fix` — either make `--plan-only` write `ARENA_PLAN.md`, or correct the smoke test to match stdout-only behavior. Rerun `cargo test --workspace` to clear gate.
**Class**: 1 (test or binary surface, no kernel/economy touch).

The TB_LOG and ship report claim of `1047/1/150` is **not currently reproducible** at HEAD `3964957`.

---

## §4 Non-Blocker Remediations (CHALLENGE)

### CHALLENGE 1 — Q11: SG-18R.9 dashboard smoke insufficient
The R5 smoke test (`tests/tb_18r_dashboard_attempt_dag_replay.rs:17-37`) only verifies `Cargo.toml` + `src/bin/audit_dashboard.rs` exist on disk; it does not invoke the dashboard binary. SG-18R.9 closure at this level is insufficient even per the OBS deferral framing (which assumed load-bearing assertions pass — they don't, see Blocker 1).
**Remediation atom**: `R5.dashboard-smoke-fix` — replace the file-existence smoke with an actual `audit_dashboard` invocation against TB-18R-shape evidence, **or** land the full attempt-DAG render.
**Class**: 1.

### CHALLENGE 2 — Q14: R0 grandfathering annotation needs explicit ship-packet exception
R0 (`5338cea`) modified `MINIF2F_M1_BENCHMARK_REPORT.md` and added `README.md` inside `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/`. The charter and VETO authorize this as annotation-only (no L4/L4.E/CAS root rewrite), but the dispatch Q14 literal claim "R0-R7 diffs do NOT modify pre-TB-18R M1 evidence" is technically false. Additionally, R3 preflight has stale "omega-path cutover" wording at `handover/ai-direct/TB-18R_R3_STEP_B_admission.md:38-45`, `:221-225` contradicted by amended `:133-153` (Q15 doc-cleanup tail).
**Remediation atom**: `G2-doc-cleanup` — annotate the R0 grandfathering exception in the G2 ship packet; remove stale R3 omega-cutover wording.
**Class**: 0.

---

## §5 OBS Forward-Binding Rulings

- **OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06**: closed for R3 mechanism (Q4 + Q5 + Q6 all PASS). Open for ship: not the cause of merged VETO.
- **OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06**: **CHALLENGE remains** — the OBS deferral is conditional on load-bearing assertions (44/45/46) passing on real evidence; assert_45 is currently broken (Blocker 1), and the smoke test does not actually invoke the dashboard (CHALLENGE 1). Both must be resolved before this OBS closes.

---

## §6 Suggested Remediation-Atom Slate (R8…)

| Atom | Class | Blocker | Effort |
|---|---|---|---|
| R8 `R5.fix-assert45-partial-verdict` | 3 | Q13 | small (assertion logic + audit_tape rerun) |
| R9 `R6.fix-p38-p49-evaluable` | 3 | Q12 | medium (rerun heavy problems with extended timeout) |
| R10 `workspace-test-fix` | 1 | workspace gate | small (one binary or one test) |
| R11 `R5.dashboard-smoke-fix` | 1 | Q11 | small-to-medium (binary invocation harness) |
| R12 `G2-doc-cleanup` | 0 | Q14 + Q15 doc tail | trivial |

Ordering: R10 first (gates further runs); then R8 (unblocks rerun audit_tape on existing R6/R7 evidence); then R9 (regenerates the two non-evaluable runs); then R11 + R12 (doc/test polish).

---

## §7 Architect §8 Sign-Off Required

Per dispatch §2 + TB-17 / TB-18 PROVISIONAL pattern: this merged verdict is delivered to the architect for §8 sign-off. The architect can:

- **(A)** Authorize remediation slate R8..R12, then re-audit at G2 round 2 after fixes ship. SHIPPED FINAL deferred until G2 round 2 PASS.
- **(B)** Override merged VETO with explicit reasoning + named carve-outs for each blocker (very high bar; requires reconciling with `feedback_dual_audit_conflict` and Class-4 ship-gate discipline).
- **(C)** Roll back PROVISIONAL SHIPPED label (revert ship doc claim; TB-18R remains Class-4 in-flight).

Default expectation given pattern + conservative ranking: **path (A)**.

---

## §8 Cross-References

- Dispatch: `handover/audits/G2_TB_18R_DUAL_AUDIT_DISPATCH_2026-05-06.md`.
- Codex verdict: `handover/audits/CODEX_TB_18R_G2_SHIP_AUDIT_2026-05-06.md`.
- Gemini verdict: `handover/audits/GEMINI_TB_18R_G2_SHIP_AUDIT_2026-05-06.md`.
- G1 audit (Codex): `handover/audits/CODEX_TB_18R_CHARTER_RATIFICATION_2026-05-06.md`.
- Charter: `handover/tracer_bullets/TB-18R_charter_2026-05-06.md`.
- VETO predecessor: `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md`.
- OBS forward-bindings:
  - `handover/alignment/OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06.md`.
  - `handover/alignment/OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md`.
- Conflict-resolution rule: `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS).

**End of merged G2 verdict. Awaits architect §8 sign-off.**
