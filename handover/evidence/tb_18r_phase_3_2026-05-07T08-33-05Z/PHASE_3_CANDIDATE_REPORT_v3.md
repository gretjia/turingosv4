# TB-18R Phase 3 Candidate Evidence Report v3 — 2026-05-07 (fresh re-run on ship HEAD `11b987b`)

> **PENDING ROUND-3 DUAL AUDIT — NOT SHIPPED**
>
> Per architect ruling §1.5 + Q-P6 naming discipline: pre-final-audit reports MUST NOT carry "ship" / "shipped" naming. This is a candidate report; ship status awaits round-3 dual audit + architect §8 sign-off.

> **WHY v3** (not a re-issue of v2): v2 evidence (`tb_18r_phase_3_2026-05-07T06-24-20Z/`, substrate `7c8dc548`) was generated 4 commits behind ship HEAD. Two bug fixes landed AFTER v2 — Phase 3 runner counting bug (`3eb4f71`) and A0 evidence-drift fix (`cf7cb48`) — followed by the architect `§10` + `§9` mandate to re-run real test on ship HEAD. v3 = fresh real test on substrate `11b987b` with the corrected runner from genesis (no `*_corrected.json` post-processing). Eliminates "HEAD drift" + "post-hoc corrected" axes that round-3 auditors might otherwise challenge as separate.

---

## §0 Header

- **Phase**: TB-18R Phase 3 — Technical Tape Validation (P38 + P49 + M0 mini-batch on typed `PartialAccepted` substrate; **fresh re-run on ship HEAD**)
- **Authority**:
  - Original Phase 3 launch directive: `handover/directives/2026-05-06_TB18R_PHASE_3_LAUNCH_DIRECTIVE.md`
  - v3 re-run authority: architect `§10` (real test FIRST on ship HEAD) + `§9` (no shortcut) + 不凑活 + 不赶工 (jointly); user explicit "确认" 2026-05-07 evening session.
  - Bug fixes between v2 and v3 substrate: `3eb4f71` (Phase 3 runner counting) + `cf7cb48` (A0 evidence-drift root cause) + `64745bb` (A0 followup — manifest rehash) + `11b987b` (handover update HEAD).
- **Architect ruling parent**: `handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md` (§5 Phase 3)
- **Substrate HEAD**: `11b987bb58f5cf535b1ffccf07c1e9e66ce68dac` (ShipGate PASS verdict per session #12 of `handover/ai-direct/LATEST.md`)
- **Run timestamp**: 2026-05-07T08:33:05Z
- **Wallclock total**: ~8.4 minutes (502s sum-of-per-problem dur)
- **Run params**: `MAX_TRANSACTIONS=12`, `PER_PROBLEM_TIMEOUT_S=1800`, model=`deepseek-chat`, condition=`n1`
- **Constitution gates (re-verified at HEAD on clean working tree)**: **70 / 0 / 1 GREEN** (`bash scripts/run_constitution_gates.sh`; 6 new gates vs v2's 64 — `tape_canonical`, `fc3_inv1_capsule_integrity_regen`, `art_v3_amendment_log`, `runner_invariant_formula`, `no_evidence_drift_in_tests`, plus expansions). Authoritative: `target/constitution_gate_report.json`.
- **Smoke probe (preceding this batch)**: `tb_18r_phase_3_2026-05-07T08-30-43Z/` — 1-problem `mathd_algebra_107`, dur=128s, OmegaAccepted, audit=PROCEED, id45=Pass, step_partial_ok=0, inv1_match=True, chain_invariant delta=0 (Ok)

## §1 Problem set + per-problem signals

| # | Problem | M1 idx | tx_count | completed_llm_calls | halt | solved | dur(s) | audit | id45 | step_partial_ok | inv1_match | chain_invariant |
|---|---------|--------|----------|---------------------|------|--------|--------|-------|------|-----------------|------------|-----------------|
| P01 | mathd_numbertheory_1124 | P38 | 8  | 8  | OmegaAccepted    | True  | 67  | PROCEED | Pass | 0 | True | **delta=0 (Ok)** |
| P02 | numbertheory_2pownm1prime_nprime | P49 | 12 | 12 | MaxTxExhausted   | False | 136 | PROCEED | Pass | 0 | True | **delta=0 (Ok)** |
| P03 | mathd_algebra_107 | M0_1 | 1  | 1  | OmegaAccepted    | True  | 19  | PROCEED | Pass | 0 | True | **delta=0 (Ok)** |
| P04 | mathd_algebra_113 | M0_2 | 12 | 9  | MaxTxExhausted   | False | 98  | PROCEED | Pass | 0 | **True** | **delta=0 (Ok)** |
| P05 | mathd_algebra_114 | M0_3 | 5  | 5  | OmegaAccepted    | True  | 159 | PROCEED | Pass | 1 | True | **delta=0 (Ok)** |
| P06 | mathd_algebra_125 | M0_4 | 1  | 1  | OmegaAccepted    | True  | 11  | PROCEED | Pass | 0 | True | **delta=0 (Ok)** |
| P07 | mathd_algebra_141 | M0_5 | 1  | 1  | OmegaAccepted    | True  | 12  | PROCEED | Pass | 0 | True | **delta=0 (Ok)** |

**Aggregate** — all architect §11 hard gates GREEN:
- 7/7 audit_tape PROCEED ✓
- 7/7 id45 (`lean_result_retrievable_from_cas`) PASS ✓
- **7/7 inv1_match=True** ✓ (vs v2's 5/7 — natural pass on ship HEAD)
- **7/7 chain_invariant.json delta=0 (Ok)** ✓ (vs v2's 5/7)
- 7/7 evaluator_failures_excluding_timeout=0 ✓
- 5/7 problems solved (71% solve rate; up from v2's 43% — LLM stochasticity)
- 1/7 step_partial_ok > 0 (P05) → 1 `AttemptOutcome::PartialAccepted` record on typed substrate

**No `*_corrected.json` post-processing was needed**: the runner script's invariant formula (`evaluator_reported_completed_llm_calls = tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err`) is canonical post-`3eb4f71` and was applied from genesis on this batch. Distinct from v2 where the runner was buggy at the time and a separate `PHASE_3_BATCH_SUMMARY_corrected.json` was authored to reflect the corrected counts post-hoc.

## §2 Architect §11 invariant audit (post-TB-C0 canonical formulation)

### §2.1 — FC1-INV1 canonical 3-term invariant

Canonical equation (per CLAUDE.md PRIME OPERATING MODE):

```
evaluator_reported_completed_llm_calls
  == l4_work_attempt_count + l4e_work_attempt_count + capsule_anchored_attempt_count
```

where `evaluator_reported_completed_llm_calls = tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err` (each corresponds to one `r2_write_attempt_telemetry` callsite).

**7/7 PASS** — every problem reports `delta=0`, `invariant_verdict=Ok` from `tb_18r_compute_invariant` binary at HEAD `11b987b`.

| Problem | l4_work | l4e_work | capsule_anchored | sum | completed_llm | delta | verdict |
|---------|---------|----------|------------------|-----|---------------|-------|---------|
| P01 | 8 | 0 | 0 | 8 | 8 | 0 | Ok |
| P02 | 12 | 0 | 0 | 12 | 12 | 0 | Ok |
| P03 | 1 | 0 | 0 | 1 | 1 | 0 | Ok |
| P04 | 9 | 0 | 0 | 9 | 9 | 0 | Ok |
| P05 | 5 | 0 | 0 | 5 | 5 | 0 | Ok |
| P06 | 1 | 0 | 0 | 1 | 1 | 0 | Ok |
| P07 | 1 | 0 | 0 | 1 | 1 | 0 | Ok |

### §2.2 — `architect_inv1_check.json` direct check

Direct comparison `chain_attempt_count == evaluator_reported_completed_llm_calls`:

**7/7 match=True**.

| Problem | chain | completed_llm | tx_count_total | non_llm_tx_diagnostic_gap | match |
|---------|-------|---------------|----------------|---------------------------|-------|
| P01 | 8  | 8  | 8  | 0 | True |
| P02 | 12 | 12 | 12 | 0 | True |
| P03 | 1  | 1  | 1  | 0 | True |
| **P04** | **9**  | **9**  | **12** | **3** | **True** |
| P05 | 5  | 5  | 5  | 0 | True |
| P06 | 1  | 1  | 1  | 0 | True |
| P07 | 1  | 1  | 1  | 0 | True |

**P04 mathd_algebra_113 — canonical non-LLM-tx case**:
- `tx_count_total=12` includes 3 architect-mandated administrative transactions (TB-6 atom-3 synthetic preseed + TB-C0 atom A.1 synthetic L4.E gate + sequencer system-terminal-summary) per `handover/alignment/OBS_TB18R_INV1_NONLLM_TX_2026-05-07.md`.
- `completed_llm_calls=9` (the 3-term `step + parse_fail + llm_err`) is the canonical denominator for FC1-INV1 — it counts only externalized LLM-Lean cycles.
- Chain attempt count = 9 = LLM-Lean cycles externalized to the tape. Perfect parity. **`non_llm_tx_diagnostic_gap=3` is informational, not a violation**: this is exactly the case TB-18R round-2 raised and that the 3-term canonical formula resolves.

### §2.3 — id45 typed-substrate consistency

**7/7 PASS** — every `LeanResult` record's `verdict_kind` field passes the 4-arm typed match (`Verified` / `Failed` / `PartialAccepted` / `SorryBlocked`); audit_tape `passed=39 failed=0 halted=0 skipped=11` per problem.

### §2.4 — `verdict_kind = PartialAccepted` records

**P05 mathd_algebra_114** — step_partial_ok=1 → 1 `AttemptOutcome::PartialAccepted` record emitted on typed substrate. Verified indirectly via id45 PASS.

(Other 6 problems had no partial-accept attempts in this LLM run.)

### §2.5 — `evaluator_failures_excluding_timeout`

**0** across all 7 problems. No SIGKILL, no panic, no abort.

## §3 Comparison with v2 batch (substrate `7c8dc548`)

| Signal | v2 (`7c8dc548`, 2026-05-07T06:24:20Z) | v3 (`11b987b`, 2026-05-07T08:33:05Z) | Delta |
|--------|----------------------------------------|----------------------------------------|-------|
| Substrate HEAD | `7c8dc548` (TB-C0 ship HEAD, 4 commits behind round-3 ship HEAD) | `11b987b` (round-3 ship HEAD, ShipGate PASS) | +4 commits |
| Bug fixes incorporated | TB-C0 round-1..8 only | TB-C0 + Phase 3 runner counting (`3eb4f71`) + A0 evidence-drift (`cf7cb48`) + manifest rehash (`64745bb`) + handover (`11b987b`) | +4 |
| audit_tape PROCEED | 7/7 | 7/7 | unchanged |
| id45 PASS | 7/7 | 7/7 | unchanged |
| inv1_match=True | 5/7 (P04/P05 fail under v2 runner formula) | **7/7** (natural pass under canonical 3-term formula) | **+2** |
| chain_invariant delta=0 | 5/7 | **7/7** | **+2** |
| `*_corrected.json` post-processing required | Yes (`PHASE_3_BATCH_SUMMARY_corrected.json` authored after the fact) | **No** (canonical formula in runner from genesis) | clean |
| solve count | 3/7 (43%) | 5/7 (71%) | +2 (LLM stochasticity) |
| step_partial_ok records | 12 total (P01:6 + P02:5 + P05:1) | 1 total (P05:1) | -11 (LLM stochasticity — fewer multi-iteration runs because more solved on first try) |
| Constitution gates | 64/0/1 GREEN | 70/0/1 GREEN | +6 gates (canonical-tape, capsule-integrity-regen, amendment-log, runner-invariant-formula, no-evidence-drift, etc.) |
| Wallclock total | ~7.6 min | ~8.4 min | +0.8 min |

**Interpretation**:
- v2 `inv1_match` 5/7 was the symptom of the runner script's stale formula (`tx_count_total` instead of `completed_llm_calls`). v2 *had* the data needed to compute the correct invariant — `PHASE_3_BATCH_SUMMARY_corrected.json` was authored to show the post-hoc corrected counts. v3 simply applies the canonical formula from genesis, naturally producing 7/7.
- The 4-commit drift between v2 substrate and round-3 ship HEAD is closed by v3. Round-3 auditors evaluate **evidence on the same HEAD they audit**.
- LLM stochasticity changed the solve mix (P01 went 12-tx-MaxTx→8-tx-Omega; P05 went 12-tx-MaxTx→5-tx-Omega) but the architect §11 hard gates (PROCEED, id45, inv1_match, delta) are invariant to which terminal halt class the LLM happens to land in.
- The constitutional gate count went from 64→70 because TB-C0 followups added the runner-invariant-formula gate, the no-evidence-drift gate, the canonical-tape gate, the capsule-integrity-regen gate, and amendment-log gate. All 70 GREEN at substrate `11b987b`.

## §4 What v3 evidence demonstrates

1. **Phase 2 typed substrate is operational on ship HEAD `11b987b`**: typed `LeanVerdictKind` + `AttemptOutcome::PartialAccepted` records emit correctly across all 7 problems; id45 typed-consistency PASSES on every record.
2. **TB-C0 invariant + runner counting + evidence-drift fixes are jointly stable**: id45 PASS rate 7/7; binary `tb_18r_compute_invariant` reports `Ok` on every problem; `architect_inv1_check.json` reports `match=True` on every problem; `chain_invariant.json` reports `delta=0` on every problem.
3. **Canonical FC1-INV1 3-term invariant is naturally satisfied on real MiniF2F problems on ship HEAD**: no post-hoc *_corrected.json files; no "as if" reframing; the runner emits canonical evidence from genesis.
4. **Non-LLM admin tx accounting is now ground-truth**: P04's 3-tx admin scaffold gap is correctly *informational* (recorded as `non_llm_tx_diagnostic_gap=3`) and does NOT trigger a false-negative on inv1.
5. **Constitution gates 70/0/1 GREEN**: CR-C0.10 (every feature TB merge requires GREEN gates) is satisfied for any TB-18R FINAL ship from this evidence.

## §5 What v3 evidence does NOT demonstrate

1. **Constitution-wide every-clause-countable coverage** — per user 2026-05-07 directive ("the test need to test every word in constitution is countable... the real problem you can find on web"), this batch covers FC1-INV1 + adjacent §11 gates. A separate constitution-coverage audit (clause-by-clause map → real-problem witness from web research) is the next deliverable; gaps surface BLOCKERs to user before round-3 dispatch can claim full constitutional coverage.
2. **TB-18R FINAL ship eligibility** — requires round-3 dual audit (Codex + Gemini independent) + architect §8 sign-off. v3 evidence enables round-3 dispatch without HEAD-drift challenge axis.
3. **M1 / M2 / M3 / NodeMarket / public-chain / TB-19 readiness** — TB-C0 freeze lifted 2026-05-07; those TBs require their own charters and gate paths.
4. **Resolution of TB-C0 forward-bound items** (Art. 0.4 git-style HEAD_t / FC3-INV3-INV5-INV7-INV8 strengthening / continuation-Markov smoke) — separate from TB-18R FINAL ship.

## §6 Position on §5 #1 vs FC1-INV1 (orchestrator stance)

The architect's original §5 #1 2-term invariant `chain_attempt_count == evaluator_reported_tx_count` (v2 framing) used `tx_count_total` as RHS. v2 §4.3 of the candidate report flagged this as superseded by the post-TB-C0 canonical FC1-INV1 3-term invariant `completed_llm_calls == L4 + L4.E + capsule_anchored`. This v3 evidence reinforces the v2 §4.3 position by demonstrating that:
- Under the canonical formula, P04's "gap" is correctly informational.
- The ship HEAD's runner already encodes the canonical formula (constitution gate `runner_invariant_formula` GREEN at HEAD `11b987b`).
- Constitution gate `art_v3_amendment_log` is GREEN, indicating the amendment is canonical (not pending).

**Round-3 adjudication request stands** (carried forward from v2 §4.3 unchanged): treat §5 #1 as superseded by FC1-INV1 (Option A); the strict 2-term reading (Option B) is no longer the canonical post-TB-C0 invariant.

## §7 Round-3 dispatch readiness

- v3 evidence dir: `handover/evidence/tb_18r_phase_3_2026-05-07T08-33-05Z/`
- v3 substrate HEAD == round-3 ship HEAD: `11b987b` (no drift)
- v3 method: canonical runner from genesis, no `*_corrected.json` post-processing
- Constitution gates 70/0/1 GREEN evidence: `target/constitution_gate_report.json` at this HEAD
- Round-3 dispatch addendum will be updated to point auditors at this dir (replacing v2 evidence reference) — see `handover/audits/G2_TB_18R_ROUND_3_DUAL_AUDIT_DISPATCH_ADDENDUM_2026-05-07.md`
- Round-3 external invocation (Codex + Gemini per `feedback_dual_audit`) is **user-billed and user-triggered**; orchestrator will request explicit user authorization before dispatch.

## §8 Cross-references

- Phase 3 launch directive (original): `handover/directives/2026-05-06_TB18R_PHASE_3_LAUNCH_DIRECTIVE.md`
- Architect parent ruling: `handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md`
- TB-C0 architect §8 sign-off: `handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md`
- Phase 3 runner counting bug diagnosis: `handover/alignment/OBS_TB18R_INV1_NONLLM_TX_2026-05-07.md`
- A0 evidence-drift diagnosis: `handover/alignment/OBS_EVIDENCE_DRIFT_ROOT_CAUSE_2026-05-07.md`
- v2 batch evidence: `handover/evidence/tb_18r_phase_3_2026-05-07T06-24-20Z/`
- v2 candidate report: `handover/evidence/tb_18r_phase_3_2026-05-07T06-24-20Z/PHASE_3_CANDIDATE_REPORT.md`
- Per-problem evidence (this batch): `handover/evidence/tb_18r_phase_3_2026-05-07T08-33-05Z/<P##>_<problem>/`
- Run manifest (this batch): `handover/evidence/tb_18r_phase_3_2026-05-07T08-33-05Z/PHASE_3_RUN_MANIFEST.json`
- Batch summary (this batch): `handover/evidence/tb_18r_phase_3_2026-05-07T08-33-05Z/PHASE_3_BATCH_SUMMARY.json`
- Smoke probe (this batch): `handover/evidence/tb_18r_phase_3_2026-05-07T08-30-43Z/`
- Constitution gate report: `target/constitution_gate_report.json` (70/0/1 GREEN at HEAD `11b987b`)
- Runner script: `handover/tests/scripts/run_tb_18r_phase_3_evidence.sh`
- User directive on every-clause-countable real problems (2026-05-07): see `feedback_real_problems_not_designed.md` 2026-05-07 strengthening section

---

**End of Phase 3 candidate evidence report v3 (2026-05-07 fresh re-run on ship HEAD). Awaits user authorization for round-3 dual audit dispatch (Codex + Gemini external invocation) + architect §8 sign-off.**

FC-trace: FC1-INV1
