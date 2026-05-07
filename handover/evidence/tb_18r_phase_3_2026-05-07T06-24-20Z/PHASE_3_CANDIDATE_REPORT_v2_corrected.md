# TB-18R Phase 3 Candidate Evidence Report v2 (post-runner-fix 2026-05-07)

> **PENDING ROUND-3 DUAL AUDIT — NOT SHIPPED**
>
> Per architect ruling §1.5 + Q-P6 naming discipline: pre-final-audit reports MUST NOT carry "ship" / "shipped" naming. Awaits round-3 dual audit + architect §8 sign-off.
>
> **This v2 supersedes the v1 (`PHASE_3_CANDIDATE_REPORT.md`)** which carried an Option-A-orientation withdrawn 2026-05-07 per `feedback_no_workarounds_strict_constitution` after deeper investigation found the issue was a runner-script counting bug, not a constitution-text simplification. v1 preserved alongside for audit trail per `feedback_no_retroactive_evidence_rewrite`.

---

## §0 Header

- **Phase**: TB-18R Phase 3 — Technical Tape Validation, post-runner-bug-fix re-verification
- **Authority**:
  - Original Phase 3 launch directive: `handover/directives/2026-05-06_TB18R_PHASE_3_LAUNCH_DIRECTIVE.md`
  - TB-C0 SHIPPED FINAL 2026-05-07 (architect §8): `handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md`
  - Resolution authority: user 2026-05-07 "你自主决定 + 不凑活 + 不赶工"
- **Substrate HEAD**: `7c8dc548` (TB-C0 SHIPPED FINAL)
- **Evidence dir**: `handover/evidence/tb_18r_phase_3_2026-05-07T06-24-20Z/`
- **LLM batch run timestamp**: 2026-05-07T06:24:20Z (unchanged from v1)
- **Resolution / re-verification timestamp**: 2026-05-07T07:00:00Z
- **Smoke probe**: `tb_18r_phase_3_2026-05-07T06-20-45Z/`
- **Resolution OBS**: `handover/alignment/OBS_TB18R_INV1_NONLLM_TX_2026-05-07.md`

## §1 What changed since v1

v1 reported `inv1_match=False` for P04 (delta=-3) and P05 (delta=-1), and recommended Option A (treat architect §5 #1 as superseded by FC1-INV6 3-term — withdrawn after deeper investigation per CLAUDE.md line 80 mandate review).

v2 reports `inv1_match=True` for ALL 7 problems under the corrected formula. The change:

- **NOT a re-run of the LLM batch** — the chain externalization was always correct; same CAS evidence
- **Fixed the runner's invariant formula**:
  - **Bug**: runner passed `EXPECTED_COMPLETED = PPUT_RESULT.tx_count` (broader; includes admin scaffold)
  - **Fix**: runner now passes `EXPECTED_COMPLETED = PPUT_RESULT.tool_dist.step + parse_fail + llm_err` (each corresponds to an `r2_write_attempt_telemetry` callsite — the canonical LLM-Lean cycle count)
  - **Why P04/P05 specifically failed**: their evaluator tx_count = 12 included 3 / 1 architect-mandated admin scaffold txs (TB-6 atom-3 synthetic preseed + TB-C0 atom A.1 synthetic L4.E gate + sequencer system-terminal-summary) that don't externalize as `AttemptTelemetry`. P01/P02 happened to have all 12 tx slots be LLM cycles, masking the bug.
- **CLAUDE.md Report Standard line 80 clarified** (no constitution.md edit; CLAUDE.md is project instructions):
  - Canonical invariant explicit: `evaluator_reported_completed_llm_calls == l4 + l4e + capsule_anchored` (3-term FC1 line 33 alignment)
  - Formula spelled out: `step + parse_fail + llm_err`
  - Diagnostic field `evaluator_reported_tx_count_total` retained for visibility
- **New constitution gate test** `tests/constitution_runner_invariant_formula.rs` (4 tests; 4 REGRESSION GUARDs) — `bash scripts/run_constitution_gates.sh` now 68/0/1 GREEN

## §2 Architect §5 Phase 3 invariant audit — v2 results (corrected formula)

### §5 #1 — `chain_attempt_count == evaluator_reported_completed_llm_calls`

**7/7 PASS** ✅ (was 5/7 under buggy formula).

| Problem | step | parse_fail | llm_err | LLM_cycle | tx_count | chain_AT | delta | match |
|---------|------|-----------|---------|-----------|----------|----------|-------|-------|
| P01 mathd_numbertheory_1124 | 12 | 0 | 0 | 12 | 12 | 12 | 0 | ✅ |
| P02 numbertheory_2pownm1prime_nprime | 11 | 1 | 0 | 12 | 12 | 12 | 0 | ✅ |
| P03 mathd_algebra_107 | 1 | 0 | 0 | 1 | 1 | 1 | 0 | ✅ |
| P04 mathd_algebra_113 | 9 | 0 | 0 | 9 | 12 | 9 | 0 | ✅ |
| P05 mathd_algebra_114 | 11 | 0 | 0 | 11 | 12 | 11 | 0 | ✅ |
| P06 mathd_algebra_125 | 1 | 0 | 0 | 1 | 1 | 1 | 0 | ✅ |
| P07 mathd_algebra_141 | 1 | 0 | 0 | 1 | 1 | 1 | 0 | ✅ |

The 3-tx gap on P04 (= 12 - 9) and 1-tx gap on P05 (= 12 - 11) is now explicit as `non_llm_tx_diagnostic_gap` field — the gap reflects architect-mandated admin scaffold (TB-6 atom-3 + TB-C0 atom A.1 + system-terminal-summary), not dropped LLM cycles.

### §5 #2 — `id44 / id45 / id46 PASS on real evidence`

**7/7 PASS** ✅ (unchanged from v1).

### §5 #3 — `R4 invariant equation evaluable`

**7/7 evaluable + 7/7 delta=0 (Ok)** ✅ (was 7/7 evaluable + 5/7 Ok in v1; corrected formula closes P04/P05 to Ok).

### §5 #4 — `verdict_kind = PartialAccepted records on multi-iteration problems`

**Validated** ✅ (unchanged from v1). 3 problems exhibited `step_partial_ok > 0`:
- P01: 6 records
- P02: 5 records
- P05: 1 record

### §5 #5 — `dashboard substantive smoke`

**Validated** ✅ via workspace tests at HEAD `7c8dc548`: 1141 / 0 / 151 (unchanged).

### Constitution gates at HEAD

`bash scripts/run_constitution_gates.sh` = **68/0/1 GREEN** (was 64/0/1; +4 from new `constitution_runner_invariant_formula` gate registered post-fix).

## §3 What this v2 evidence demonstrates

1. **Phase 3 Phase 2 typed substrate is operational on post-TB-C0 HEAD**: typed `LeanVerdictKind` + `AttemptOutcome::PartialAccepted` records emit correctly across all 7 problems; id45 typed-consistency PASSES on every record.
2. **Architect §11 #1 hard gate satisfied 7/7** under canonical invariant.
3. **Bug introduced + bug fixed within the same TB**: runner-script counting bug (D-b') was the actual cause; identified, fixed, and locked behind regression-guard constitution gate.
4. **TB-C0 round-5/6/7 fixes did not regress Phase 2 substrate**.
5. **Constitution gate count increased**: 64 → 68 GREEN (mechanism added per `feedback_norm_needs_mechanism`).

## §4 What this v2 evidence does NOT demonstrate

1. **TB-18R FINAL ship eligibility** — still requires round-3 dual audit (Codex + Gemini) + architect §8 sign-off.
2. **M1 / M2 / M3 / NodeMarket / public-chain / TB-19 readiness** — separate TBs.
3. **A0 (22 files drift root cause)** — separate parallel investigation.

## §5 Round-3 dispatch readiness

A round-3 dispatch addendum will reference:
- v2 evidence (this report)
- OBS_TB18R_INV1_NONLLM_TX_2026-05-07 (resolution narrative)
- Corrected `architect_inv1_check_corrected.json` per problem
- Corrected `chain_invariant_corrected.json` per problem
- Corrected `PHASE_3_BATCH_SUMMARY_corrected.json`
- 68/0/1 constitution gate report
- v1 candidate report (preserved for audit trail; superseded)

Round-3 invocation is user-billed; awaits user explicit go.

## §6 Cross-references

- v1 candidate report (superseded; preserved): `PHASE_3_CANDIDATE_REPORT.md` (this dir)
- Resolution OBS: `handover/alignment/OBS_TB18R_INV1_NONLLM_TX_2026-05-07.md`
- Constitution gate runner (68/0/1): `bash scripts/run_constitution_gates.sh`
- New regression guard: `tests/constitution_runner_invariant_formula.rs`
- Original architect directive: `handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md` §5 Phase 3
- TB-C0 architect §8: `handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md`
- CLAUDE.md Report Standard line 80 (clarified): canonical invariant statement
- Memory: `feedback_no_workarounds_strict_constitution`, `feedback_class4_cannot_hide_in_class3`, `feedback_audit_after_evidence`, `feedback_norm_needs_mechanism`, `feedback_real_problems_not_designed`

---

**End of Phase 3 candidate v2. 7/7 PASS architect §11 #1 hard gate under canonical invariant. Awaits round-3 dual audit + architect §8.**
