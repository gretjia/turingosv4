# TB-18R G2 Round-3 Dual Audit Dispatch — Final Cumulative Phase 1 + 2 + 3 Review

**Date**: 2026-05-06
**Gate**: G2 round-3 (final dual audit per architect ruling §8 directive items 8–9; charter §2 G2 row + SG-18R.13)
**HEAD audited**: TBD — Phase 3 evidence commit (will be post-`55a0935`; updated when Phase 3 evidence ships)
**Predecessors audited**:
- `eb2b932` (round-2 candidate; merged VETO at round-1, reviewed at round-2 dispatch)
- `8487bd6` (Phase 1 process repair; Class 0 docs)
- `3f51667` (Phase 2 typed PartialAccepted; Class 4 schema bump)
- `55a0935` (session #8 handover-update)
- Phase 3 evidence commit (TBD)
**Auditors**: Codex (source-grounded + workspace-test-grounded) + Gemini (text/schema review)
**Conservative ranking**: VETO > CHALLENGE > PASS per `feedback_dual_audit_conflict`. Any single VETO across Q1..Q15 + Q-P1..Q-P6 + new round-3 questions → merged VETO.

> **Naming discipline (per parent ruling §1.5 + Q-P6)**: this is a DISPATCH not a SHIP REPORT. Round-3 verdict files must NOT carry "ship" / "shipped" naming until architect §8 sign-off.

---

## §0 What this dispatch is — and is NOT

**This is the FINAL dual audit per architect ruling §5 + §8.** Its scope:

1. **Cumulative Phase 1 + Phase 2 + Phase 3 review**: ratify the entire correction trajectory from round-1 VETO through to Phase 3 evidence on the typed substrate.
2. **Phase 2 schema bump verification**: confirm the typed PartialAccepted (`LEAN_RESULT_SCHEMA_ID v1→v2`, `LeanVerdictKind` enum, `AttemptOutcome::PartialAccepted=6`) is correctly implemented per FC-first analysis Option B.
3. **Phase 3 evidence verification**: confirm P38 + P49 + M0 mini-batch on typed substrate satisfy architect ruling §5 five invariants.
4. **Round-2 carry-forward closure**: explicitly close all six Q-P process gaps (round-2 dispatch §3) given Phase 1 docs landed on `main`.

**This is NOT** a re-audit of round-1 / round-2 evidence per se. Where round-2 dispatch's Q1..Q15 + Q-P1..Q-P6 already received PASS / CHALLENGE-resolved verdicts, do not re-derive — but DO surface any drift introduced by Phase 1+2+3 commits.

---

## §1 Round-2 verdict status (closure ledger)

Per round-2 dispatch + architect ruling §4 Q-P1..Q-P6 rulings + Phase 1 docs landed at `8487bd6`:

| Q-P | Issue | Round-2 status | Phase 1 closure artifact | Round-3 verification |
|-----|-------|----------------|--------------------------|----------------------|
| Q-P1 | `"fix"` ≠ §8 sign-off | OPEN | `2026-05-06_TB18R_R8_R12_RATIFICATION_ADDENDUM.md` | confirm addendum exists + cites architect ruling |
| Q-P2 | assert_45 α vs β | CHALLENGE | `FC_FIRST_ANALYSIS_ASSERT45_PARTIAL_VERDICT_2026-05-06.md` + Phase 2 directive | confirm FC-first verdict + Phase 2 implements Option B |
| Q-P3 | R3 `[SUPERSEDED]` markers | CHALLENGE | `OBS_TB18R_R3_PREFLIGHT_SUPERSESSION_2026-05-06.md` (or `handover/alignment/`) | confirm OBS file exists with old/new authority text |
| Q-P4 | Q14 distill risk | CHALLENGE | round-2 candidate report verbatim quote inserts | confirm verbatim quotes from charter §0.A + VETO :604-609 are present |
| Q-P5 | No FC-first trace | CHALLENGE | `FC_FIRST_ANALYSIS_ASSERT45_PARTIAL_VERDICT_2026-05-06.md` (covers Q-P5 too) | confirm 4 constitutional questions answered with reasoning |
| Q-P6 | Premature "Ship Report" title | OPEN | round-2 ship report banner-prefixed `PENDING ROUND-2 DUAL AUDIT — NOT SHIPPED` | confirm banner + tracer-bullet ship report banner |

**Round-3 expectation**: all six Q-P → CLOSED. Auditors verify each Phase 1 closure artifact exists, has the required content, and is consistent with the parent ruling.

---

## §2 Phase 2 schema bump audit (NEW)

Verify the typed PartialAccepted implementation at commit `3f51667` per Phase 2 directive §1–§5 + FC-first analysis §2.4 Option B.

### Q-A1 — LeanVerdictKind discriminant stability

`LeanVerdictKind` is `#[repr(u8)]` with `Verified=0, Failed=1, PartialAccepted=2, SorryBlocked=3`. Tail-additive (no renumbering possible). Mirrors R3 RejectionClass pattern.

Witness tests: `tests/tb_18r_lean_verdict_kind_repr_stability.rs`.

### Q-A2 — LEAN_RESULT_SCHEMA_ID v1 → v2 + legacy decode

`LEAN_RESULT_SCHEMA_ID` constant bumped v1 → v2. Pre-Phase-2 LeanResult records (R6/R7/M1 evidence) MUST decode under post-Phase-2 build via `LeanResult::derive_verdict_kind_from_legacy_fields`.

**Load-bearing**: `feedback_no_retroactive_evidence_rewrite` requires no edits to historical CAS objects. Phase 3 substrate must read pre-Phase-2 records correctly.

### Q-A3 — AttemptOutcome::PartialAccepted = 6 tail-add

Variants 0..5 unchanged (LeanPass / LeanFail / ParseFail / SorryBlock / LlmErr + any pre-existing). PartialAccepted = 6 appended. step_partial_ok emitter flips from LeanPass → PartialAccepted.

Witness: `tests/tb_18r_attempt_outcome_partial_accepted_repr_stability.rs`.

### Q-A4 — assert_45 4-arm typed match

`assert_45_lean_result_retrievable_from_cas` retyped to 4-arm match via `is_verdict_kind_consistent`:

```text
Verified         ↔ exit_code == 0  ∧ verified ∧ error_class.is_none()
Failed           ↔ exit_code != 0  ∧ ¬verified ∧ error_class.is_some()
PartialAccepted  ↔ exit_code == 0  ∧ ¬verified ∧ error_class.is_none()
SorryBlocked     ↔ exit_code == 0  ∧ ¬verified ∧ error_class == Some(SorryBlocked)
```

Each arm is exact (==), not implication. Drift bug (e.g., `verdict_kind=Verified` but `verified=false`) MUST FAIL.

Witness: `tests/tb_18r_lean_verdict_kind_consistency.rs` (drift rejection + legacy fixture PASS).

### Q-A5 — Sequencer guard for AttemptOutcome::PartialAccepted

`refine_rejection_class_via_attempt_telemetry` (`src/state/sequencer.rs`) handles `PartialAccepted` correctly: panic-in-debug if it reaches the rejection arm; fallback-in-release. step_partial_ok stays CAS-only (no L4 / no L4.E entry per R3 §1.3).

**Class-4 sub-question**: `src/state/sequencer.rs` IS in CLAUDE.md STEP_B_PROTOCOL list. Was Phase 2's sequencer touch performed under STEP_B parallel-branch, or as direct edit? If direct edit on `main`, Q-A5 = VETO (STEP_B violation).

(Phase 2 directive §1 claims sequencer touch is the AttemptOutcome::PartialAccepted guard arm only, justified as additive non-admission-changing. Auditors must rule whether this counts as a STEP_B file modification requiring parallel-branch.)

### Q-A6 — Emitter migration: 6 callsites with explicit verdict_kind

`r2_write_attempt_telemetry` (`evaluator.rs`) signature gains optional `verdict_kind: Option<LeanVerdictKind>`. All 6 production callsites pass an explicit `Some(...)`:
- omega-full / omega-pertactic → `Verified`
- step_reject (sorry-block) → `SorryBlocked`
- step_reject (lean-error) → `Failed`
- parse_fail → `Failed`
- llm_err → `verdict_kind` arg unused
- step_partial_ok → `PartialAccepted` (and `outcome: AttemptOutcome::PartialAccepted`)

Backward-compat: `derive_verdict_kind_from_fields` for `verdict_kind: None`.

### Q-A7 — Workspace test count: 1077 / 0 / 150 reproducible

Run `cargo test --workspace` at HEAD `3f51667` (or `55a0935`). Expected: **1077 passed / 0 failed / 150 ignored**. Any deviation → Q-A7 finding.

### Q-A8 — FC3 cross-edge audit

Phase 2 must not perturb economic-flow paths. Verify:
- `src/state/sequencer.rs` admission's `predicate_passes` derivation: independent of `LeanResult.verified`
- `src/sdk/tools/wallet.rs` stake/reward: depends on WorkTx admission, not LeanResult
- `src/runtime/chain_derived_run_facts.rs` ship-gate equation: counts L4 vs L4.E by chain identity
- `experiments/minif2f_v4/src/bin/evaluator.rs::compute_pput_m`: reads `verified` from `B4Result`

If any economic-flow site now depends on `verdict_kind`, Q-A8 = CHALLENGE (FC3 cross-edge entered).

### Q-A9 — Composite ratification trail acceptability for Class 4

Phase 2 ratification trail is composite (architect ruling enumerated both options + user umbrella + user explicit autonomy clarification + FC-first analysis + self-recorded directive). Architect ruling §5 Phase 2 leaning: β-with-typing.

Auditors rule: is this composite trail sufficient ratification for `LEAN_RESULT_SCHEMA_ID v1→v2` + new enum types, OR is line-item architect Option-B ratification required?

---

## §3 Phase 3 evidence audit (NEW)

Verify Phase 3 evidence at `handover/evidence/tb_18r_phase_3_<timestamp>/` per architect ruling §5 Phase 3 + Phase 3 launch directive §3.

### Q-B1 — Architect §5 #1: chain_attempt_count == evaluator_reported_tx_count

For each Phase 3 problem, verify `architect_inv1_check.json` shows `match: true`. The Python script counts `AttemptTelemetry` records via `.turingos_cas_index.jsonl` and compares to `tx_count` from PPUT_RESULT.

Auditor verification: spot-check 2 problems' CAS index manually + recompute.

### Q-B2 — Architect §5 #2: id44 / id45 / id46 PASS on real evidence

For each Phase 3 problem, `verdict.json` MUST have:
- id=44 (`attempt_telemetry_retrievable_from_cas`) result=`Pass`
- id=45 (`lean_result_retrievable_from_cas`) result=`Pass`
- id=46 (`attempt_chain_root_schema_well_formed`) result=`Pass`

id45 is the load-bearing typed-consistency check: assert_45 walks all LeanResult records and runs the 4-arm typed match. Any verdict_kind drift → id45 FAIL.

### Q-B3 — Architect §5 #3: R4 invariant equation evaluable

For each Phase 3 problem, `chain_invariant.json` MUST exist with all 6 fields populated (no SIGKILL pre-PPUT_RESULT abort). The `invariant_verdict` field may be `Ok` or `Err(...)` — both count as "evaluable".

**Note on synthetic L4.E gate artifact**: For OmegaAccepted runs with no `step_partial_ok`, the binary's strict `delta=0` check produces `Err(... delta=1 ...)` because the synthetic L4.E gate WorkTx (per atom A.1) adds +1 to `l4e_count` without a corresponding tx_count increment. This is a pre-Phase-2 substrate convention, not a Phase 2 defect. The architect §5 #1 direct check (Q-B1) bypasses this artifact.

Auditors should rule whether the invariant binary's strict-delta-zero contract should be relaxed in a follow-on TB to account for the synthetic gate, or whether the architect §5 #1 direct check via `architect_inv1_check.json` is sufficient.

### Q-B4 — Architect §5 #4: verdict_kind=PartialAccepted records on multi-iteration problems

For Phase 3 multi-iteration problems where `step_partial_ok` fires (`tool_dist.step_partial_ok > 0`), the typed substrate MUST emit `LeanResult` records with `verdict_kind = PartialAccepted`. These records are canonical-encoded in CAS, but their typed-consistency is verified indirectly via id45 PASS.

Auditor verification: identify Phase 3 problems with `step_partial_ok > 0`. Confirm id45 PASS on those problems. Confirm `LeanResult` count in CAS index ≥ `step_partial_ok` count + omega + reject + parse_fail.

### Q-B5 — Architect §5 #5: dashboard substantive smoke

`tests/tb_18r_dashboard_attempt_dag_replay.rs` test must PASS in workspace (covered by Q-A7 1077/0/150). If Q-A7 PASS → Q-B5 PASS.

### Q-B6 — No retroactive M1 / R6 / R7 evidence rewrite

Verify `git diff 55a0935..HEAD -- handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/` produces no changes. Verify Phase 3 evidence dir is fresh (not overlapping any pre-Phase-3 evidence).

### Q-B7 — Phase 3 launch directive properly archived

`handover/directives/2026-05-06_TB18R_PHASE_3_LAUNCH_DIRECTIVE.md` exists with verbatim user authorization + Q-P1 conformance section + scope envelope. Per `feedback_kolmogorov_compression`: original user message must be exactly reconstructable from the archive.

---

## §4 Cumulative ship eligibility (NEW)

### Q-C1 — TB-18R FINAL ship readiness

Given the cumulative state at HEAD (Phase 3 evidence commit), is TB-18R eligible for FINAL ship?

Required conditions:
- All Q1..Q15 (round-2) → PASS / CHALLENGE-resolved
- All Q-P1..Q-P6 → CLOSED
- All Q-A1..Q-A9 (Phase 2 schema) → PASS
- All Q-B1..Q-B7 (Phase 3 evidence) → PASS
- Workspace test 1077 / 0 / 150 reproducible
- No FREEZE-list items externalized

If all conditions hold → Q-C1 = SHIP-ELIGIBLE-PENDING-§8.
If any conditions fail → Q-C1 = NOT-SHIP-ELIGIBLE; remediation path required.

**Q-C1 = SHIP-ELIGIBLE does NOT promote to ship.** Architect §8 sign-off is the final gate per ruling §8 directive item 9.

### Q-C2 — Architect §8 sign-off prep

If Q-C1 = SHIP-ELIGIBLE, the merged round-3 verdict file becomes the input for architect §8. Auditors should explicitly recommend whether §8 should grant TB-18R FINAL ship or hold for further work.

Per Q-P1 ruling: **single-word user inputs ("fix" / "go" / "ok") MUST NOT be parsed as architect §8 sign-off.** §8 must be an explicit multi-clause architect/user directive archived under `handover/directives/`.

### Q-C3 — Post-ship FREEZE unlock conditions

Currently FROZEN per architect ruling §3 expanded FREEZE:
- M1 public benchmark report
- M2 / M3 scale-up
- TB-19 real-world pilot design
- NodeMarket / PriceIndex claims based on M1
- Any formal H-VPPU conclusion
- Any "formal benchmark passed" externalization

If TB-18R ships with §8: FREEZE lifts on **TB-18R FINAL ship gate alone** — NOT on every individual frozen item. M1 public benchmark + M2 / M3 + TB-19 each need their own gating decision (separate TB charters or architect directives).

Auditors recommend explicit FREEZE-unlock language for the §8 sign-off draft.

---

## §5 Inputs to read

**Phase 1 + Phase 2 + Phase 3 source** (cumulative at HEAD):
- `git diff 3964957..HEAD --stat` (round-1 baseline → cumulative)
- `git log --oneline 3964957..HEAD` (8 commits expected)
- `src/runtime/attempt_telemetry.rs` (Phase 2 schema + LeanVerdictKind + AttemptOutcome::PartialAccepted)
- `src/runtime/audit_assertions.rs` (Phase 2 assert_45 retype)
- `src/state/sequencer.rs` (Phase 2 PartialAccepted guard)
- `experiments/minif2f_v4/src/bin/evaluator.rs` (Phase 2 emitter migration)

**Phase 3 evidence**:
- `handover/evidence/tb_18r_phase_3_<timestamp>/PHASE_3_BATCH_SUMMARY.json`
- `handover/evidence/tb_18r_phase_3_<timestamp>/PHASE_3_RUN_MANIFEST.json`
- `handover/evidence/tb_18r_phase_3_<timestamp>/<P##>_<problem>/{verdict.json,chain_invariant.json,verdict_kind_summary.json,architect_inv1_check.json,README.md}`

**Authorizing documents** (read FULLY):
- `handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md`
- `handover/directives/FC_FIRST_ANALYSIS_ASSERT45_PARTIAL_VERDICT_2026-05-06.md`
- `handover/directives/2026-05-06_TB18R_PHASE_2_REMEDIATION_DIRECTIVE.md`
- `handover/directives/2026-05-06_TB18R_PHASE_3_LAUNCH_DIRECTIVE.md`
- `handover/directives/2026-05-06_TB18R_R8_R12_RATIFICATION_ADDENDUM.md`
- `handover/post-mortems/ROOT_CAUSE_TB18_DELAY_2026-05-06.md`
- `handover/alignment/OBS_TB18R_R3_PREFLIGHT_SUPERSESSION_2026-05-06.md`

**Predecessor audit dispatches + verdicts** (cross-reference):
- `handover/audits/G2_TB_18R_DUAL_AUDIT_DISPATCH_2026-05-06.md` (round-1)
- `handover/audits/G2_TB_18R_DUAL_AUDIT_VERDICT_2026-05-06.md` (round-1 merged VETO)
- `handover/audits/G2_TB_18R_ROUND_2_DUAL_AUDIT_DISPATCH_2026-05-06.md` (round-2)

**Charter + governing memory**:
- `handover/tracer_bullets/TB-18R_charter_2026-05-06.md`
- `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md`
- CLAUDE.md (Code / Audit / Report / Reproducibility / Alignment Standards + STEP_B_PROTOCOL)
- Memory rules cited:
  - `feedback_no_workarounds_strict_constitution`
  - `feedback_fc_first_problem_handling`
  - `feedback_class4_cannot_hide_in_class3`
  - `feedback_dual_audit_conflict`
  - `feedback_kolmogorov_compression`
  - `feedback_no_retroactive_evidence_rewrite`
  - `feedback_audit_after_evidence`
  - `feedback_workspace_test_canonical`
  - `feedback_step_b_protocol`

**Workspace test reproduction**:
- Run `cargo test --workspace` at HEAD. Expected: 1077 / 0 / 150.

---

## §6 Conservative ranking + auditor independence

Both auditors operate independently. Do not coordinate verdicts. Merged verdict computed by:
1. Per-question / per-process-gap merge: VETO > CHALLENGE > PASS.
2. Across all dimensions: any single VETO → merged VETO.

Specifically, even if all Q1..Q-A9 + Q-B1..Q-B7 land PASS, a single Q-C / Q-P (process) VETO produces merged VETO.

Auditors are encouraged to disagree with predecessor recommendations. Round-2 dispatch and FC-first analysis can be challenged at round-3 if independent re-analysis warrants.

---

## §7 Output format

Each auditor produces one file:
- `handover/audits/CODEX_TB_18R_G2_ROUND_3_AUDIT_2026-05-06.md`
- `handover/audits/GEMINI_TB_18R_G2_ROUND_3_AUDIT_2026-05-06.md`

Required structure:

```text
# <Auditor> TB-18R G2 Round-3 Final Cumulative Audit — 2026-05-06

## 1. Header
- auditor: <name>
- model: <model id>
- date: 2026-05-06
- HEAD: <Phase 3 evidence commit>
- scope: cumulative Phase 1 + 2 + 3 review (process + schema + evidence + ship eligibility)

## 2. Inputs reviewed
<verbatim file:line citations of every artifact actually read>

## 3. Round-2 Q-P closure verification (Q-P1..Q-P6)
For each, output: CLOSED | OPEN | RE-OPENED + grounded reasoning citing Phase 1 artifact.

## 4. Phase 2 schema bump verdicts (Q-A1..Q-A9)
For each, output: PASS | CHALLENGE | VETO + grounded reasoning.

## 5. Phase 3 evidence verdicts (Q-B1..Q-B7)
For each, output: PASS | CHALLENGE | VETO + grounded reasoning.

## 6. Cumulative ship eligibility verdicts (Q-C1..Q-C3)
For each, output: SHIP-ELIGIBLE | NOT-SHIP-ELIGIBLE | RECOMMENDATION + grounded reasoning.

## 7. Workspace test reproduction
<observed counts at HEAD>

## 8. Overall verdict
PASS | CHALLENGE | VETO + tally + ship recommendation.

## 9. Suggested remediation (if applicable)
If VETO or CHALLENGE: enumerate atoms / scope.
```

After both files exist, Claude orchestrator computes merged verdict at `handover/audits/G2_TB_18R_ROUND_3_DUAL_AUDIT_VERDICT_2026-05-06.md` per `feedback_dual_audit_conflict` conservative ranking, and surfaces to architect for **explicit §8 sign-off**.

---

## §8 Cross-references

- Architect ruling: `handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md`
- Phase 2 directive: `handover/directives/2026-05-06_TB18R_PHASE_2_REMEDIATION_DIRECTIVE.md`
- Phase 3 directive: `handover/directives/2026-05-06_TB18R_PHASE_3_LAUNCH_DIRECTIVE.md`
- Round-1 verdict: `handover/audits/G2_TB_18R_DUAL_AUDIT_VERDICT_2026-05-06.md`
- Round-2 dispatch: `handover/audits/G2_TB_18R_ROUND_2_DUAL_AUDIT_DISPATCH_2026-05-06.md`
- TB-18 delay post-mortem: `handover/post-mortems/ROOT_CAUSE_TB18_DELAY_2026-05-06.md`
- TB log: `handover/tracer_bullets/TB_LOG.tsv`

**End of round-3 dispatch. Awaits Codex + Gemini independent audits + merged verdict for architect §8 sign-off decision on TB-18R FINAL ship.**
