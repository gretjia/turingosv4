# TB-18R Round-3 Dual Audit Dispatch — Addendum 2026-05-07

**Date**: 2026-05-07
**Parent dispatch**: `handover/audits/G2_TB_18R_ROUND_3_DUAL_AUDIT_DISPATCH_2026-05-06.md` (authored against substrate `55a0935`)
**Why addendum**: Phase 3 was re-run on post-TB-C0 substrate `7c8dc548`. Initial 2/7 inv1_match=False on P04/P05 was identified + resolved as runner-script counting bug (D-b'). Auditors should review the v2 evidence package, not v1.

---

## §0 Substrate change

| | Pre (parent dispatch) | Post (this addendum) |
|---|----------------------|----------------------|
| HEAD | `55a0935` (Phase 1+2 only) | `7c8dc548` (TB-C0 SHIPPED FINAL: Phase 1+2+TB-C0 round-1..8) |
| Constitution gates | did not exist | 68 / 0 / 1 GREEN |
| Workspace tests | 1077 / 0 / 150 | 1141 / 0 / 151 |
| Phase 3 evidence dir | `tb_18r_phase_3_2026-05-06T14-13-55Z/` | `tb_18r_phase_3_2026-05-07T06-24-20Z/` |
| Phase 3 status (initial) | 5/7 inv1_match=True | 5/7 inv1_match=True (REPRODUCED) |
| Phase 3 status (post-fix) | n/a | **7/7 inv1_match=True** (corrected formula) |

The 5/7 → 7/7 transition is NOT due to chain change; chain externalization was always correct. The transition is due to the runner script being fixed to compute `EXPECTED_COMPLETED` from `step + parse_fail + llm_err` (LLM-Lean cycle count) instead of `tx_count` (broader; includes admin scaffold).

## §1 What auditors should review (added vs parent dispatch)

In addition to parent dispatch §1-9 (which apply to v2 evidence too), auditors are asked to verify:

### §1.1 Runner counting bug resolution

**Read in order**:
1. `handover/alignment/OBS_TB18R_INV1_NONLLM_TX_2026-05-07.md` — full diagnostic narrative
2. `handover/evidence/tb_18r_phase_3_2026-05-07T06-24-20Z/PHASE_3_CANDIDATE_REPORT_v2_corrected.md` — v2 evidence report
3. `tests/constitution_runner_invariant_formula.rs` — new regression-guard test (4 tests, REGRESSION GUARDs explicit)
4. `handover/tests/scripts/run_tb_18r_phase_3_evidence.sh` — runner script (look for `completed_llm_calls = step + parse_fail + llm_err` block)
5. `CLAUDE.md` Report Standard line 80 (clarified text)

**Verify**:
- [ ] OBS narrative correctly identifies 3 architect-mandated admin tx kinds (TB-6 atom-3 + TB-C0 atom A.1 + system-terminal-summary) as the source of evaluator's tx_count gap
- [ ] Resolution is D-b' (runner-script counting bug fix), NOT D-c (constitution-text amend); fix is in script + new gate, not workaround text amendment
- [ ] CLAUDE.md clarification is purely textual (project instructions; not constitution.md edit; no Phase Z′ rerun required)
- [ ] New constitution gate `constitution_runner_invariant_formula` has 4 tests, each with explicit REGRESSION GUARD assertions
- [ ] `bash scripts/run_constitution_gates.sh` returns 68/0/1 GREEN at HEAD
- [ ] No source code (`src/`) change required for the resolution; runner-script + new test + CLAUDE.md clarification only

### §1.2 v2 evidence package completeness

For `handover/evidence/tb_18r_phase_3_2026-05-07T06-24-20Z/`, verify:

- [ ] Per-problem `chain_invariant_corrected.json` shows `delta=0, invariant_verdict="Ok"` for all 7 problems
- [ ] Per-problem `architect_inv1_check_corrected.json` shows `match=True, chain_attempt_count == evaluator_reported_completed_llm_calls`
- [ ] `PHASE_3_BATCH_SUMMARY_corrected.json` aggregate: `match_pass_count: 7, delta_zero_count: 7, all_pass: true`
- [ ] v1 files (`chain_invariant.json`, `architect_inv1_check.json`, `PHASE_3_BATCH_SUMMARY.json`, `PHASE_3_CANDIDATE_REPORT.md`) preserved alongside v2 per `feedback_no_retroactive_evidence_rewrite` audit-trail discipline
- [ ] PartialAccepted records emit correctly on multi-iteration runs (P01: 6, P02: 5, P05: 1)

### §1.3 Architect §11 hard-line conformance

Verify per architect TB-C0 directive §11 + §13:

- [ ] FC1 hard gate: every externalized LLM-Lean attempt is tape-visible — 7/7 problems (chain has every LLM-Lean cycle as `AttemptTelemetry`)
- [ ] FC2: run replayable from `genesis_report + ChainTape + CAS` — verifiable per problem dir
- [ ] FC3: `EvidenceCapsule` derived from chain+CAS — per `verdict_kind_summary.json` shows EvidenceCapsule type present
- [ ] Predicate gate: predicate pass → L4 / fail → L4.E — `id45 PASS 7/7` confirms typed routing
- [ ] Tape canonical gate: dashboard regenerable from chain+CAS — workspace tests cover

## §2 Auditor question rubric (additive)

| ID | Question | Expected verdict |
|----|----------|------------------|
| Q-R3-A1 | Does the v2 PHASE_3_BATCH_SUMMARY_corrected.json show all 7 problems passing match/delta=0? | PASS |
| Q-R3-A2 | Does `tests/constitution_runner_invariant_formula.rs` actually FAIL if EXPECTED_COMPLETED is reverted to tx_count? | PASS (test enforces; verify by reasoning about the negation assertions) |
| Q-R3-A3 | Is the resolution D-b' (runner fix + mechanism) sufficient to prevent recurrence vs D-c (text amend only) which would be 凑活? | PASS — D-b' executed; D-c would have been workaround |
| Q-R3-A4 | Does `feedback_no_retroactive_evidence_rewrite` hold? Are v1 files preserved? | PASS — v1 files (chain_invariant.json, architect_inv1_check.json, PHASE_3_BATCH_SUMMARY.json, PHASE_3_CANDIDATE_REPORT.md) untouched; v2 alongside |
| Q-R3-A5 | Was any `src/` source code changed for the resolution? | NO (intentional — runner script + test + docs only) |
| Q-R3-A6 | Does the new constitution gate have explicit REGRESSION GUARDs (assertions that fail if the bug recurs)? | PASS — each test has `assert!(!script.contains(...))` regression-guard pattern |
| Q-R3-A7 | Does CLAUDE.md line 80 amendment require Phase Z′ rerun per Art. V.1.1? | NO — CLAUDE.md is project instructions, not constitution.md; clarification is in scope of project ops |
| Q-R3-A8 | Does the parent dispatch's Q1-Q15 + Q-P1-P6 + Q-A/B/C apply to v2 evidence with no change? | PASS — v2 is same chain externalization; only the runner-side analysis was buggy |

## §3 Conservative-ranking note

If any auditor returns VETO on Q-R3-A1..A8 (or any parent-dispatch question against v2 evidence), VETO wins per `feedback_dual_audit_conflict`. Round-4 dispatch addendum + remediation required.

If both auditors PASS Q-R3-A1..A8 + parent dispatch applies cleanly, addendum verdict = PASS → architect §8 path opens.

## §4 What this addendum does NOT change

- Parent dispatch §1-9 substantive questions
- Round-3 = final round before architect §8 (per parent dispatch)
- TB-18R FINAL ship still requires architect explicit §8 sign-off (multi-clause; not single-word per Q-P1)
- FREEZE list status (TB-C0 §8 lifted 2026-05-07; TB-18R FINAL ships into the post-freeze landscape)

## §5 Cross-references

- Parent dispatch: `handover/audits/G2_TB_18R_ROUND_3_DUAL_AUDIT_DISPATCH_2026-05-06.md`
- Resolution OBS: `handover/alignment/OBS_TB18R_INV1_NONLLM_TX_2026-05-07.md`
- v2 candidate report: `handover/evidence/tb_18r_phase_3_2026-05-07T06-24-20Z/PHASE_3_CANDIDATE_REPORT_v2_corrected.md`
- v1 candidate report (superseded; preserved): `handover/evidence/tb_18r_phase_3_2026-05-07T06-24-20Z/PHASE_3_CANDIDATE_REPORT.md`
- TB-C0 architect §8: `handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md`
- Architect parent ruling: `handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md` §5
- Constitution gates report: `target/constitution_gate_report.json` (68/0/1 GREEN at HEAD `7c8dc548`+post-runner-fix)
- New regression guard: `tests/constitution_runner_invariant_formula.rs`

---

## §6 v3 evidence supersession (2026-05-07 evening session)

**Status**: v2 evidence package (substrate `7c8dc548`, runner-counting-bug fix applied via `*_corrected.json` post-processing) was the canonical artifact when this addendum was first authored. Subsequent commits landed:
- `3eb4f71` — Phase 3 runner counting bug fix lifted into the runner script itself (no longer requires post-hoc `_corrected.json`)
- `cf7cb48` — A0 evidence-drift root-cause fix (env-gate test writes to committed evidence)
- `64745bb` — A0 followup (manifest rehash for `tb_7_atom6_smoke.rs`)
- `11b987b` — handover update; ShipGate PASS verdict

Per architect `§10` + `§9` mandate (real test FIRST on ship HEAD), Phase 3 was re-run on substrate `11b987b` to eliminate the 4-commit drift between v2 substrate and the round-3 audit HEAD. **v3 evidence is now the canonical artifact for round-3 audit.**

### §6.1 v3 substrate change

| | v2 (parent of this addendum) | v3 (canonical for round-3) |
|---|----------------------|----------------------|
| Substrate HEAD | `7c8dc548` (TB-C0 SHIPPED FINAL) | `11b987b` (ShipGate PASS; round-3 ship HEAD) |
| Drift from round-3 ship HEAD | 4 commits behind | **0 (HEAD == substrate)** |
| Runner counting fix in script | No (lifted via `_corrected.json` post-processing) | **Yes** (canonical formula in script from genesis) |
| `_corrected.json` post-processing | Required | **None** (natural pass) |
| Constitution gates | 68 / 0 / 1 GREEN | **70 / 0 / 1 GREEN** (+2 gates: `runner_invariant_formula`, `no_evidence_drift_in_tests`) |
| Phase 3 evidence dir | `tb_18r_phase_3_2026-05-07T06-24-20Z/` | **`tb_18r_phase_3_2026-05-07T08-33-05Z/`** |
| inv1_match=True | 5/7 (raw) → 7/7 (post-corrected) | **7/7 (natural)** |
| chain_invariant delta=0 | 5/7 (raw) → 7/7 (post-corrected) | **7/7 (natural)** |
| Solve count | 3/7 (43%) | 5/7 (71%) — LLM stochasticity |

### §6.2 What auditors should review for v3 (replaces §1.2 + §2 against v3 evidence)

**Read in order**:
1. v3 candidate report: `handover/evidence/tb_18r_phase_3_2026-05-07T08-33-05Z/PHASE_3_CANDIDATE_REPORT_v3.md`
2. v3 batch summary: `handover/evidence/tb_18r_phase_3_2026-05-07T08-33-05Z/PHASE_3_BATCH_SUMMARY.json`
3. v3 run manifest: `handover/evidence/tb_18r_phase_3_2026-05-07T08-33-05Z/PHASE_3_RUN_MANIFEST.json`
4. Per-problem `chain_invariant.json` + `architect_inv1_check.json` (no `_corrected.json` files needed)
5. OBS docs unchanged: `handover/alignment/OBS_TB18R_INV1_NONLLM_TX_2026-05-07.md` + `handover/alignment/OBS_EVIDENCE_DRIFT_ROOT_CAUSE_2026-05-07.md`
6. `target/constitution_gate_report.json` — 70/0/1 GREEN at HEAD `11b987b`

**Verify (replaces v2 §1.2 checklist)**:
- [ ] Per-problem `chain_invariant.json` (NOT `_corrected.json`) shows `delta=0, invariant_verdict="Ok"` for all 7 problems
- [ ] Per-problem `architect_inv1_check.json` (NOT `_corrected.json`) shows `match=True`
- [ ] `PHASE_3_BATCH_SUMMARY.json` (NOT `_corrected.json`): 7/7 audit=PROCEED, 7/7 invariant_verdict=Ok, 7/7 delta=0, evaluator_failures_excluding_timeout=0
- [ ] No `*_corrected.json` files exist in v3 evidence dir (verifies natural pass)
- [ ] v3 substrate HEAD == round-3 audit HEAD (`11b987b`) — no drift
- [ ] Constitution gates 70/0/1 GREEN at this HEAD
- [ ] Smoke probe preceded batch: `tb_18r_phase_3_2026-05-07T08-30-43Z/` (1-problem `mathd_algebra_107`, audit=PROCEED, id45=Pass, inv1_match=True)
- [ ] P04 `mathd_algebra_113` `non_llm_tx_diagnostic_gap=3` correctly informational, not a violation (canonical FC1-INV1 case)

### §6.3 Auditor question rubric for v3 (additive to §2)

| ID | Question | Expected verdict |
|----|----------|------------------|
| Q-R3-V3-1 | Does v3 substrate == round-3 audit HEAD? | PASS — `11b987b` both |
| Q-R3-V3-2 | Are there any `*_corrected.json` files in v3 evidence dir? | NO — natural pass |
| Q-R3-V3-3 | Do all 7 v3 `chain_invariant.json` files show `delta=0, invariant_verdict=Ok` directly (not via post-processing)? | PASS |
| Q-R3-V3-4 | Does v3 demonstrate that the canonical FC1-INV1 3-term formula is in the runner script from genesis (not applied post-hoc)? | PASS — `tests/constitution_runner_invariant_formula.rs` GREEN at this HEAD |
| Q-R3-V3-5 | Does the user 2026-05-07 directive ("every word in constitution countable, real problem from web") apply to this evidence? | YES — but v3 covers FC1-INV1 only; constitution-wide every-clause coverage is a separate post-v3 deliverable (see v3 report §5). v3 does NOT claim constitution-wide every-clause-countable coverage; it claims FC1-INV1 + adjacent §11 hard-gate coverage on real MiniF2F problems with no manipulation. |
| Q-R3-V3-6 | Are v2 evidence files preserved (audit trail)? | PASS — `tb_18r_phase_3_2026-05-07T06-24-20Z/` untouched |

### §6.4 Conservative-ranking note for v3

Same as §3: if any auditor returns VETO on Q-R3-V3-1..6 or any parent-dispatch / v2 question against v3 evidence, VETO wins. If both auditors PASS, addendum verdict = PASS → architect §8 path opens.

### §6.5 Cross-references for v3 (additive)

- v3 evidence dir: `handover/evidence/tb_18r_phase_3_2026-05-07T08-33-05Z/`
- v3 candidate report: `handover/evidence/tb_18r_phase_3_2026-05-07T08-33-05Z/PHASE_3_CANDIDATE_REPORT_v3.md`
- v3 smoke probe dir: `handover/evidence/tb_18r_phase_3_2026-05-07T08-30-43Z/`
- v3 substrate HEAD: `11b987bb58f5cf535b1ffccf07c1e9e66ce68dac`
- Constitution gates 70/0/1 GREEN at v3 substrate: `target/constitution_gate_report.json`
- A0 evidence-drift fix commit: `cf7cb48` (cf. `handover/alignment/OBS_EVIDENCE_DRIFT_ROOT_CAUSE_2026-05-07.md`)
- Runner counting fix commit: `3eb4f71` (cf. `handover/alignment/OBS_TB18R_INV1_NONLLM_TX_2026-05-07.md`)

---

**End of round-3 dispatch addendum. v3 supersedes v2 as canonical round-3 evidence. Awaits user-billed Codex + Gemini round-3 invocation.**
