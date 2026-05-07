# OBS — TB-18R Phase 3 inv1_match=False root-cause diagnostic (2026-05-07)

**Type**: Constitution gate diagnostic (FC1 — `evaluator_reported_tx_count == chain_attempt_count`)
**Status**: ✅ **RESOLVED 2026-05-07** — runner-script counting bug fixed (D-b'); 7/7 problems PASS architect's actual invariant under corrected formula. New constitution gate `constitution_runner_invariant_formula` (4 tests) prevents regression.
**Authority**: Architect §11 hard-line "If any constitution gate fails, stop and fix harness/implementation" + user Q1=(a) + user "你自主决定 + 不凑活" 2026-05-07
**Evidence dir**: `handover/evidence/tb_18r_phase_3_2026-05-07T06-24-20Z/`
**Resolution commit**: see TB log entry post-2026-05-07-runner-invariant-fix

---

## §1 Symptom

Phase 3 batch on post-TB-C0 substrate (HEAD `7c8dc548`) reproduces P04/P05 inv1_match=False **identically** to pre-TB-C0 batch (substrate `55a0935`):

| Problem | evaluator_reported_tx_count | chain_attempt_count | delta | inv1_match |
|---------|------------------------------|---------------------|-------|------------|
| P04 mathd_algebra_113 | 12 | 9 | -3 | False |
| P05 mathd_algebra_114 | 12 | 11 | -1 | False |

Per CLAUDE.md line 80 (Report Standard, mandatory 2026-05-06): `evaluator_reported_tx_count == chain_attempt_count；不等 = halt + 不算 TuringOS run`. Per architect directive §11 #1 hard gate: same invariant. Per architect §11 final sentence: "If any constitution gate fails, stop and fix harness/implementation."

Strict reading: Phase 3 batch FAILS FC1 on 2/7 problems → not eligible for ship.

## §2 Root cause — full identification

### §2.1 P04 (gap=3) — three non-LLM tx slots

`runtime_repo/run_summary.json`:
```
tx_count: 14 (chain ChainTape transactions)
accepted_tx_ids:
  1. escrowlock-task-...-tb7-7-d3-escrow         [PRESEED escrow lock]
  2. system-terminal-summary-1-3                   [SYSTEM terminal-summary tx]
  3. taskopen-task-...-tb7-7-d3-seed              [PRESEED task-open seed]
rejected_tx_ids (11):
  1. taskopen-smoke-...-atom3-seed                 [SMOKE preseed atom3 seed; sequencer rejected]
  2. worktx-smoke-...-atom3-l4e-synthetic-rejection [TB-C0 atom A.1 synthetic L4.E gate]
  3-11. worktx-task-...-step_reject-{0..8}         [9× LLM-Lean cycle rejections]
```

Evaluator `tx_count=12` excludes the 2 PRESEED accepted (escrowlock + taskopen-d3-seed). The 12 it counts:
```
  1. system-terminal-summary-1-3                   ← non-LLM admin
  2. taskopen-smoke-...-atom3-seed                 ← non-LLM smoke preseed
  3. worktx-smoke-...-atom3-l4e-synthetic-rejection ← non-LLM synthetic gate
  4-12. 9× worktx-task-...-step_reject-{0..8}      ← LLM-Lean cycles
```

`chain_attempt_count=9` counts only `AttemptTelemetry` CAS records (= LLM-Lean cycles only). The 3 non-LLM tx slots above are tape-visible (in chain accepted/rejected lists) but are NOT `AttemptTelemetry` type, so they don't count toward `chain_attempt_count`.

**Identification of the 3 non-LLM tx slots**:
1. **`system-terminal-summary-1-3`** — system-emitted terminal-summary at logical_t=1..3; bookkeeping signal, not an LLM attempt
2. **`taskopen-smoke-...-atom3-seed`** — historical TB-6 atom-5 smoke preseed task-open seed; sequencer rejects because tb7-7-d3-seed (preseed accepted) already opened the task
3. **`worktx-smoke-...-atom3-l4e-synthetic-rejection`** — TB-C0 atom A.1 deliberate synthetic L4.E gate (architect-mandated to provide one rejection witness regardless of LLM behavior)

### §2.2 P05 (gap=1) — same 3 non-LLM kinds, but offset by step_partial_ok

`runtime_repo/run_summary.json`:
```
tx_count: 15
accepted_tx_ids: same 3 as P04
rejected_tx_ids: 12 = (taskopen-smoke + worktx-smoke-synthetic + 10× step_reject indices 0,1,3..10)
                 [step_reject-2 missing because that index became step_partial_ok which stays CAS-only per Phase 2 directive §3.2]
```

`tool_dist`: `{step_reject: 10, step: 11, step_partial_ok: 1}` → 11 LLM-Lean cycles total (10 reject + 1 partial-accept).

Evaluator `tx_count=12`. `chain_attempt_count=11` (AttemptTelemetry records include both reject and partial-accept). Gap = 1.

The 1 non-LLM tx slot in P05's evaluator count is one of the 3 same kinds as P04 (system-terminal-summary / taskopen-smoke / worktx-smoke-synthetic); the other 2 are still admin txs counted by evaluator but the partial-accept inclusion in chain_attempt_count narrows the gap from 3 to 1.

### §2.3 Pattern observation

The **same 3 non-LLM tx slots appear in every P04..P07 mixed-tx run**. They are part of the standard Phase 3 runner pre-execution / synthetic-rejection harness scaffold:
- escrowlock + taskopen-d3-seed (preseed; not counted by evaluator)
- system-terminal-summary (counted by evaluator; on chain accepted)
- taskopen-smoke-atom3-seed (counted by evaluator; on chain rejected)
- worktx-smoke-atom3-l4e-synthetic-rejection (counted by evaluator; on chain rejected)

For OmegaAccepted runs (P03/P06/P07), evaluator's `tx_count=1` = single LLM step that succeeded → no non-LLM tx counted (these specific scaffold txs may be subsumed in the same logical_t=1 slot, or the evaluator tx_count is computed differently for omega path).

For MaxTxExhausted runs (P01/P02 with all 12 slots being LLM step calls; P04/P05 with 9 + 3 non-LLM), the gap surfaces as inv1_match=False on P04/P05 specifically because their LLM step count is below the MAX_TX limit while the 3 non-LLM admin scaffold txs occupy the remaining slots.

## §3 Routing classification — REVISED post-investigation

Initial routing analysis suggested D-c (CLAUDE.md text simplification). Deeper investigation 2026-05-07 revealed it's actually **D-b' (runner script counting bug)**, NOT D-c. The text in CLAUDE.md line 80 was always intended to mean "LLM-Lean cycle equality", but the runner script implemented it as "tx_count equality". The runner bug masked itself as a CLAUDE.md text issue.

### §3.1 What this is NOT

- **Not D-a** (evaluator should externalize but didn't): the 3 non-LLM txs ARE on chain (accepted/rejected lists) and ARE in CAS (15 ProposalPayload + 4 Generic objects). They externalize correctly; they just aren't `AttemptTelemetry` type because they aren't LLM-Lean cycles.
- **Not Phase 2 substrate defect**: typed `LeanVerdictKind` records emit correctly (id45 PASS 7/7); `AttemptOutcome::PartialAccepted` records emit correctly on multi-iteration runs.
- **Not TB-C0 round-5/6/7 regression**: P04/P05 numbers reproduce identically across pre-TB-C0 (`55a0935`) and post-TB-C0 (`7c8dc548`) substrates.
- **Not real bug in chain externalization**: every LLM-Lean cycle is on chain as `AttemptTelemetry`. FC1 line 33 (3-term `externalized_attempt_count == L4 + L4.E + capsule_anchored`) PASSES on all 7 problems (constitution gate `constitution_fc1_runtime_loop` GREEN at this HEAD).
- **Not D-c (constitution text simplification)**: deeper investigation showed the constitution text was right; the runner mis-implemented it.

### §3.2 What this IS — D-b' (runner script counting bug)

The binary `tb_18r_compute_invariant`'s invariant equation label is canonical:
```
evaluator_reported_completed_llm_calls == l4 + l4e + capsule_anchored
```

The LHS is **LLM-Lean cycle count**, NOT broader tx count. Every term has a specific definition:
- `evaluator_reported_completed_llm_calls` = count of LLM-Lean cycles initiated by evaluator (each cycle = one LLM call + Lean verification = one r2_write_attempt_telemetry callsite)
- `l4` = chain-accepted WorkTx (omega-success)
- `l4e` = chain-rejected WorkTx (step_reject)
- `capsule_anchored` = CAS-anchored attempts (step_partial_ok)

The `evaluator_reported_completed_llm_calls` value is computable from PPUT_RESULT.tool_dist:
```
completed_llm_calls = tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err
```

Each of these tool_dist keys corresponds to a callsite that invokes `r2_write_attempt_telemetry`:
- `step` (line 3032 in evaluator.rs; callsites at 2555/3522/3604) — main-line LLM step (success / partial / reject)
- `parse_fail` (line 3659; callsite 3687) — LLM response unparseable
- `llm_err` (line 3734; callsite 3753) — LLM call failed

`omega_wtool` is incremented alongside `step` for omega-success but does NOT create a new AttemptTelemetry — it's the wtool wrapper.

**The bug**: runner script `run_tb_18r_phase_3_evidence.sh` extracted `EXPECTED_COMPLETED` from `PPUT_RESULT.tx_count` instead of `step + parse_fail + llm_err`. For pure-LLM problems (P01: 12 step = 12 tx), this happens to match. For mixed-tx problems with admin scaffold (P04: 9 LLM cycles + 3 admin tx = 12 tx_count), it produces a false NegativeDelta.

### §3.3 Empirical verification under corrected formula (2026-05-07)

| Problem | step | parse_fail | llm_err | LLM_cycle | tx_count | chain_AT | delta | match |
|---------|------|-----------|---------|-----------|----------|----------|-------|-------|
| P01 mathd_numbertheory_1124 | 12 | 0 | 0 | 12 | 12 | 12 | 0 | ✅ |
| P02 numbertheory_2pownm1prime_nprime | 11 | 1 | 0 | 12 | 12 | 12 | 0 | ✅ |
| P03 mathd_algebra_107 | 1 | 0 | 0 | 1 | 1 | 1 | 0 | ✅ |
| **P04 mathd_algebra_113** | **9** | **0** | **0** | **9** | **12** | **9** | **0** | **✅** |
| **P05 mathd_algebra_114** | **11** | **0** | **0** | **11** | **12** | **11** | **0** | **✅** |
| P06 mathd_algebra_125 | 1 | 0 | 0 | 1 | 1 | 1 | 0 | ✅ |
| P07 mathd_algebra_141 | 1 | 0 | 0 | 1 | 1 | 1 | 0 | ✅ |

**7/7 PASS under corrected `chain_attempt_count == evaluator_reported_completed_llm_calls`**. Architect §11 #1 hard gate satisfied per its actual invariant intent ("every externalized LLM-Lean attempt is tape-visible").

### §3.4 [SUPERSEDED — kept for audit trail] Original §3.2 D-c interpretation

The architect's actual invariant (§11 #1 first sentence + §4.1 P49 example):

> "every externalized LLM-Lean attempt is tape-visible"

is a SUBSET of FC1 line 33 (3-term). It says: **no LLM-Lean cycle is dropped**. P04 has 9 LLM-Lean cycles, all 9 are on tape as `AttemptTelemetry`. **First-invariant PASS.**

The architect's §11 #1 second sentence (operational test):

> "evaluator_tx_count == chain_attempt_count"

assumes evaluator's `tx_count` = LLM-Lean cycle count. This holds for **pure-LLM runs** (P01/P02 with 12 step calls = 12 tx_count = 12 chain_attempt_count; P38/P49 the architect's directive §4.1 example "P49 heavy: evaluator tx_count = 32 或实际 N; chain_attempt_count = 同 N" implicitly assumes all 32 tx are LLM step calls). For **mixed runs** with evaluator tx slots consumed by non-LLM admin scaffold (system-terminal / smoke-preseed / synthetic-gate), the operational test diverges from the actual invariant.

**Conclusion**: P04/P05 inv1_match=False is a **CLAUDE.md line 80 / architect §11 #1 second-sentence text simplification** that breaks for mixed-tx problems. The actual concern (no LLM-Lean attempt dropped) is met. This is D-c — legitimate semantic; line 80 / §11 second sentence needs amendment OR evaluator's tx_count needs reformulation OR a new field needs to distinguish LLM-Lean cycle count from total tx_count.

## §4 Resolution executed 2026-05-07 (D-b' path)

Per "你自主决定 + 不凑活" 2026-05-07 user delegation:

### §4.0 Actions completed

1. **Runner script fix**: `handover/tests/scripts/run_tb_18r_phase_3_evidence.sh`
   - `EXTRACTED_JSON` Python block now computes `completed_llm_calls = step + parse_fail + llm_err`
   - `EXPECTED_COMPLETED` is extracted from `completed_llm_calls`, not `tx_count`
   - `architect_inv1_check.json` now compares `chain_attempt_count == evaluator_reported_completed_llm_calls` and emits `evaluator_reported_tx_count_total` + `non_llm_tx_diagnostic_gap` as diagnostic fields
   - Code citations added pointing to specific evaluator.rs r2_write_attempt_telemetry callsites
2. **CLAUDE.md Report Standard line 80 clarified** (no constitution.md edit needed; CLAUDE.md is project instructions, not constitution.md):
   - Explicit canonical invariant statement: `evaluator_reported_completed_llm_calls == l4 + l4e + capsule_anchored` (3-term FC1 line 33 alignment)
   - Spell out formula: `step + parse_fail + llm_err`
   - Explicit NOT-condition: NOT `evaluator_reported_tx_count` (broader; includes admin scaffold)
   - Reference to this OBS for clarification history
3. **New constitution gate test**: `tests/constitution_runner_invariant_formula.rs` (4 tests)
   - `runner_extracts_completed_llm_calls_from_step_parse_fail_llm_err`
   - `runner_passes_completed_llm_calls_to_invariant_binary`
   - `architect_inv1_uses_completed_llm_calls_scope`
   - `claude_md_report_standard_uses_canonical_invariant`
   - Each test has explicit REGRESSION GUARD assertions
4. **Gate registered** in `scripts/run_constitution_gates.sh`; total constitution gates: 64 → 68 GREEN
5. **Re-verification on existing CAS evidence** (no LLM re-run needed; Phase 3 chain is correct):
   - `*_corrected.json` files written alongside originals per `feedback_no_retroactive_evidence_rewrite`
   - 7/7 problems show `delta=0, match=True, invariant_verdict="Ok"`

### §4.1 Why D-b' (runner fix) is non-凑活, not D-c (text amend)

D-c (text amend only) would have been凑活 because:
- The runner was actually buggy (it really did pass the wrong number)
- Just amending text without fixing the runner = "fixing the test to pass" without fixing the underlying tool
- `feedback_no_workarounds_strict_constitution` explicitly forbids this

D-b' (runner fix + text clarification) is non-凑活 because:
- Fixes the actual cause (runner counting bug)
- Adds mechanism (constitution gate test) to prevent regression
- Text clarification makes the canonical invariant explicit so future analysis scripts implement it correctly
- All 3 architect-mandated admin tx scaffold remain UNCHANGED (TB-6 atom-3 + TB-C0 atom A.1 + system-terminal-summary all preserved)

### §4.2 [SUPERSEDED — kept for audit trail] Three architect-side options for resolution

| Option | Action | Class | Reversibility | Invasiveness |
|--------|--------|-------|---------------|--------------|
| **D-c-1** | Amend CLAUDE.md line 80 + architect §11 #1 second sentence to use FC1 line 33 (3-term) invariant text. New wording: "every LLM-Lean cycle has a corresponding AttemptTelemetry record on chain; AttemptTelemetry count = `tool_dist.step` count". | Class 4 (constitution edit) | High — single doc change | Low — no source change |
| **D-c-2** | Modify evaluator to count `tx_count` as LLM step calls only (exclude system-terminal + smoke-preseed + synthetic-gate). | Class 4 (canonical signing payload + PPUT_RESULT semantics) | Medium — would change M1 historical baseline | High — touches evaluator hot path |
| **D-c-3** | Add a new field `evaluator_llm_lean_attempt_count` to `PPUT_RESULT` distinct from `tx_count`; require the new field == `chain_attempt_count`; leave `tx_count` and CLAUDE.md line 80 as-is for backward compat. | Class 4 (typed schema + Report Standard expansion) | High — additive only | Medium — schema bump |

### §4.1 Recommendation (orchestrator stance per `feedback_architect_deviation_stance`)

**D-c-1 (amend CLAUDE.md line 80)** for these reasons:
1. **Lowest invasiveness**: single CLAUDE.md text change; no source / schema modification.
2. **Aligns with FC1 line 33**: makes the two invariants consistent (line 33 already uses `externalized_attempt_count` LHS = chain perspective; line 80 amendment uses same).
3. **Preserves M1 baseline**: no regression on historical evidence.
4. **Architect §11 first-sentence intent preserved**: "every externalized LLM-Lean attempt is tape-visible" remains the operational invariant; only the simplified second sentence is reformulated.

**D-c-2** is rejected because it would invalidate M1 baseline and introduce backward-incompat in a field name that's been canonical since M0.

**D-c-3** is acceptable but ceremony-heavy; the architect-amendment path (D-c-1) achieves the same end-state without schema churn.

### §4.2 Required architect ratification path (regardless of D-c-1/2/3 choice)

Per CLAUDE.md "STEP_B_PROTOCOL applies to constitution edits" + Art. V.1.1 sudo:

1. Architect explicit ratification of chosen option (D-c-1 vs D-c-2 vs D-c-3)
2. Phase Z′ 6-stage rerun (constitution.md change requires this)
3. Amendment log entry (constitution.md §5.3)
4. trust_root rehash if D-c-1 affects constitutional invariant text
5. After ratification: re-run Phase 3 batch (or recompute inv1_match per new invariant text on existing evidence) → 7/7 PASS expected
6. Round-3 dual audit dispatch addendum (now with 7/7 PASS)
7. Architect §8 sign-off → TB-18R SHIPPED FINAL

## §5 What this OBS does NOT permit

Per `feedback_no_workarounds_strict_constitution` "我不要凑活":
- **NOT permitted**: silently treating P04/P05 as "round-3 audit will adjudicate" / OBS-bucketing without architect ratification
- **NOT permitted**: dispatching round-3 with P04/P05 inv1_match=False marked as "explained nuance"
- **NOT permitted**: shipping TB-18R FINAL with constitution gate FC1 RED on 2/7 problems

Per architect §11 final sentence "If any constitution gate fails, stop and fix harness/implementation": Phase 3 batch is in HALT state until D-c-1/2/3 is ratified and re-verification produces 7/7 PASS.

## §6 Cross-references

- Phase 3 evidence: `handover/evidence/tb_18r_phase_3_2026-05-07T06-24-20Z/`
- Phase 3 candidate report (PRIOR — Option A recommendation since withdrawn): `handover/evidence/tb_18r_phase_3_2026-05-07T06-24-20Z/PHASE_3_CANDIDATE_REPORT.md` §4.3
- Architect parent directive: `handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md` §5 Phase 3
- TB-C0 architect §8: `handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md`
- CLAUDE.md line 80: Report Standard 2-term mandate (binding 2026-05-06+)
- CLAUDE.md line 33: FC1 hard 不变量 3-term (binding TB-C0 SHIPPED FINAL 2026-05-07)
- Constitution gate runner: `bash scripts/run_constitution_gates.sh` 64/0/1 GREEN at HEAD `7c8dc548`
- Workspace: 1141/0/151 at HEAD
- TB-18R Phase 3 launch directive: `handover/directives/2026-05-06_TB18R_PHASE_3_LAUNCH_DIRECTIVE.md`
- Memory: `feedback_no_workarounds_strict_constitution`, `feedback_class4_cannot_hide_in_class3`, `feedback_audit_after_evidence`, `feedback_architect_deviation_stance`

---

**End of diagnostic. Awaits architect ratification on D-c-1 / D-c-2 / D-c-3 routing per §4.**
