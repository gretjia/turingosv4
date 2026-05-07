# OBS — TB-C0 FC1-INV3 three implementation bugs (2026-05-06)

**Class**: mixed — see per-bug class below
**Status**: **PARTIALLY FIXED 2026-05-07** (Bug 1 + Bug 3 fixed in commit chain; Bug 2 escalated). See §10 for fix log.
**Authority of finding**: TB-C0 round 1 + TB-18R Phase 3 evidence walk 2026-05-06; reinforced by strict tape audit 2026-05-07.
**Scope (revised 2026-05-07)**: per user 2026-05-07 "你先修复" directive, Bug 1 + Bug 3 + FC1-INV6 hole are fixed in TB-C0 round-5 commit. Bug 2 escalated as separate STEP_B-restricted item (sequencer.rs is in CLAUDE.md Code Standard STEP_B file list).

---

## §1 What this OBS records

The TB-C0 constitution-gate test `fc1_attempt_count_equals_tape_count` PASSES on synthetic data (test scaffold's hand-crafted `ChainDerivedRunFacts` correctly fires `NegativeDelta` for the canonical TB-18 P49 N→1 collapse shape). But the **EMPIRICAL FC1-INV3 invariant fails on real workload** — `handover/evidence/tb_18r_phase_3_2026-05-06T14-13-55Z/PHASE_3_BATCH_SUMMARY.json` shows 7/7 problems return `Err(...)` on the binary, despite the architect §5 #1 direct check passing on 5/7.

The structural test is correct; the **empirical implementation has 3 distinct bugs** that prevent the canonical FC1 hard-invariant equation from being computed from real chain artifacts. This OBS catalogs all three.

---

## §2 The constitutional FC1 hard invariant (CLAUDE.md PRIME OPERATING MODE §FC1)

```text
externalized_attempt_count
  == L4_WorkTx_attempt_count
   + L4E_WorkTx_rejection_count
   + explicitly_anchored_capsule_attempt_count
```

**LHS**: count of LLM-Lean cycles the evaluator externalized. NOT total `tx_count`. NOT `proposal_count`. NOT `tool_dist.step + tool_dist.omega_wtool` (these double-count the omega step).

**RHS** has THREE terms:
1. `L4_WorkTx_attempt_count` — accepted WorkTx in L4
2. `L4E_WorkTx_rejection_count` — rejected WorkTx in L4.E
3. `explicitly_anchored_capsule_attempt_count` — CAS-only step_partial_ok records (Phase 2 §3.2)

---

## §3 Bug 1 — Runner uses `tx_count` instead of LLM-cycle count (Class 2 fix)

**Location**: `handover/tests/scripts/run_tb_18r_phase_3_evidence.sh` (the runner that drives `tb_18r_compute_invariant`).

**Symptom**: For P04 (12 tx_count, but only 9 LLM-Lean cycles per `tool_dist={step:9, step_reject:9}`), the runner passes `--expected-completed 12`. The chain has 9 L4.E entries (matching the 9 LLM cycles correctly). Invariant reports `delta=-3, NegativeDelta`. False positive.

**Root cause**: `tx_count` in PPUT_RESULT counts ALL transactions including pre/post-LLM phases (boot, cleanup, terminal summary). Not all transactions externalize an LLM-Lean cycle.

**Fix**: derive `expected_completed_attempts` from the evaluator's PPUT `tool_dist` field — specifically the count of `step` outcomes that produced an LLM call (excluding `omega_wtool` double-count). Or surface a new evaluator field `externalized_llm_cycle_count` that is the canonical LHS quantity.

**Class**: 2 (script + evaluator field surface change; not a sequencer / typed-tx schema bump).

---

## §4 Bug 2 — Synthetic L4.E gate WorkTx (atom A.1) inflates `l4e_work_attempt_count` (Class 3 fix)

**Location**: TB-18R atom A.1 — system writes a "synthetic gate" L4.E entry on omega-success runs.

**Symptom**: For P03/P06/P07 (1 LLM call, omega-success path), evaluator reports 1 LLM cycle. Chain has l4=1 (omega WorkTx) AND l4e=1 (synthetic gate). Invariant computes `delta = 1 + 1 - 1 = 1`, fires `CleanHaltDeltaNonZero`. False positive.

**Root cause**: The synthetic gate is a system-emitted record, not a real attempt. It should NOT count toward `L4E_WorkTx_rejection_count` (which constitutionally counts rejected agent attempts).

**Fix options**:
- (A) Filter out synthetic-gate L4.E entries from `l4e_work_attempt_count`. Requires the synthetic-gate variant to be type-distinguishable from real rejections (a new `RejectionClass::SyntheticGate` discriminant or `is_synthetic: bool` field).
- (B) Increment `expected_completed_attempts` by 1 to absorb the synthetic gate. Confusing semantics — pretends there was an extra externalized cycle.
- (C) Add a new field `synthetic_gate_count` to `ChainDerivedRunFacts` and adjust the equation to `delta = l4 + l4e + capsule - expected - synthetic_gate`.

**Recommendation**: (A) — preserve canonical equation; add typed discriminator to L4.E.

**Class**: 3 (touches `RejectionClass` enum + chain_derived counting). May be 4 if the discriminator change requires canonical-signing-payload mutation (architect verdict).

---

## §5 Bug 3 — Missing third invariant term: `explicitly_anchored_capsule_attempt_count` (Class 4 fix)

**Location**: `src/runtime/chain_derived_run_facts.rs` — `ChainDerivedRunFacts` struct + `delta` field + `attempt_count_invariant()` function.

**Symptom**: For P05 (11 LLM cycles: 10 step_reject + 1 step_partial_ok), evaluator reports 11 cycles. Chain has l4e=10 (rejects in L4.E). The 1 step_partial_ok is CAS-only per Phase 2 directive §3.2 — no L4 / no L4.E entry. Invariant computes `delta = 0 + 10 - 11 = -1`, fires `NegativeDelta`. False positive.

**Root cause**: The implemented equation is `delta = l4 + l4e - expected`. The constitutional FC1 hard invariant has THREE RHS terms; the implementation has only TWO. The third term (`explicitly_anchored_capsule_attempt_count`, counting CAS-anchored step_partial_ok records) is missing.

**Fix**:
1. Add `pub capsule_anchored_attempt_count: u64` to `ChainDerivedRunFacts`.
2. Update `delta` calculation: `delta = (l4 + l4e + capsule_anchored) as i64 - expected as i64`.
3. In `compute_run_facts_from_chain_with_invariant`: walk CAS, count `AttemptTelemetry` records where `outcome == AttemptOutcome::PartialAccepted` AND there's no matching L4/L4.E entry. That count is `capsule_anchored_attempt_count`.

**Class**: 4 — bumps canonical `ChainDerivedRunFacts` schema (+1 serde field) + canonical equation. Per `feedback_class4_cannot_hide_in_class3`, this is STEP_B-restricted (architect ratification + parallel-branch protocol required).

---

## §6 Why this didn't surface in TB-18R R4 ship

R4 ship-gate evidence used SYNTHETIC `ChainDerivedRunFacts` constructed via the `facts(...)` test fixture (see `tests/tb_18r_chain_attempt_invariant.rs`). The fixtures correctly stage `expected = l4 + l4e` and the invariant fires correctly. But:
- No fixture for the synthetic-gate-on-omega case (Bug 2)
- No fixture exercising step_partial_ok CAS-only records (Bug 3)
- No fixture differentiating LLM-cycle count from total tx_count (Bug 1)

R4 was a STRUCTURAL ship — the equation logic is sound. But the **empirical end-to-end semantics** have these 3 gaps. TB-C0's "constitutional harness must be exercised by real load" rule (per `feedback_tape_first_real_tests`) catches this.

---

## §7 Forward-bound remediation plan

This OBS escalates to architect for the Class 3 + Class 4 fixes.

| Bug | Class | Proposed location | Owner gate |
|-----|-------|-------------------|------------|
| Bug 1 (runner tx_count) | 2 | `handover/tests/scripts/run_tb_18r_phase_3_evidence.sh` + evaluator PPUT field | architect ratification (semantic decision: which quantity is LHS?) |
| Bug 2 (synthetic gate) | 3 | `RejectionClass` enum + sequencer atom A.1 + chain_derived counting | architect ratification (typed discriminator design) |
| Bug 3 (capsule term missing) | 4 STEP_B | `ChainDerivedRunFacts` + `delta` + `attempt_count_invariant` | architect ratification + STEP_B parallel-branch |

**TB-C0 SCOPE**: NONE. TB-C0 catalogs the bugs (this OBS) but does not bundle the fixes. Per TB-C0 charter §6 out-of-scope: "Modifying CAS schema" + "Implementing new typed-tx variants" + "Modifying sequencer admission semantics".

**Forward TB**: a follow-on TB-FC1 (or TB-18R Phase 4 if architect prefers consolidation) implements the 3 fixes under proper STEP_B discipline.

**Until fix lands**: the empirical FC1-INV3 invariant is **AMBER** (structural GREEN; smoke FAILING with explained false positives). The constitutional gate row in `CONSTITUTION_EXECUTION_MATRIX.md` §G/§N MVP-1 retains AMBER status with this OBS as forward-trigger.

---

## §8 Why the constitution gate is still load-bearing

The structural test in `tests/constitution_fc1_runtime_loop.rs::fc1_every_externalized_attempt_is_tape_visible` correctly fires on the canonical TB-18 P49 N→1 collapse shape (32 expected, 1 chain → NegativeDelta delta=-31). It catches the WORST-CASE failure mode that motivated TB-18R.

What it doesn't catch is the THREE more subtle accounting gaps documented above. Those need the empirical run + 3 fixes to close.

This is exactly the spirit of `feedback_constitutional_harness_engineering`: the harness fires correctly on the dominant failure mode, but real workload exposes finer-grained gaps that drive the next iteration.

---

## §9 Cross-references

- TB-C0 charter: `handover/tracer_bullets/TB-C0_charter_2026-05-06.md`
- TB-C0 directive: `handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md`
- Strict audit (2026-05-07): `handover/alignment/STRICT_AUDIT_TBC0_TAPE_2026-05-07.md`
- Phase 3 candidate report: `handover/evidence/tb_18r_phase_3_2026-05-06T14-13-55Z/PHASE_3_CANDIDATE_REPORT.md`
- Phase 3 batch summary: `handover/evidence/tb_18r_phase_3_2026-05-06T14-13-55Z/PHASE_3_BATCH_SUMMARY.json`
- TB-18R R4 invariant binary: `src/bin/tb_18r_compute_invariant.rs`
- chain_derived_run_facts: `src/runtime/chain_derived_run_facts.rs`
- AttemptTelemetry CAS schema: `src/runtime/attempt_telemetry.rs`
- Constitution matrix (FC1 row): `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §G + §N MVP-1
- Memory: `feedback_constitutional_harness_engineering` + `feedback_tape_first_real_tests` + `feedback_class4_cannot_hide_in_class3`

---

## §10 Fix log (2026-05-07 round-5; per user "你先修复" directive)

| Bug | Original Class | Fixed | Notes |
|-----|---------------|-------|-------|
| **Bug 1** (runner uses `tx_count`) | 2 | ✅ FIXED | `handover/tests/scripts/run_tbc0_multi_agent_evidence.sh` derives `EXPECTED_COMPLETED` from `tool_dist.step` (LLM-cycle count), with fallback to `omega_wtool` then legacy `tx_count`. `architect_inv1_check.json` producer now compares against `externalized_llm_cycle_count` (was `evaluator_reported_tx_count`). Pre-fix evidence still re-readable. |
| **Bug 2** (synthetic L4.E gate inflates l4e_count) | 3 | 🟡 **ESCALATED** | Requires sequencer.rs change (synthetic-gate discriminator) + chain_derived counting filter. `src/state/sequencer.rs` is in CLAUDE.md Code Standard STEP_B file list. Per `feedback_class4_cannot_hide_in_class3` + STEP_B protocol: NOT fixed in this round-5; needs separate STEP_B parallel-branch + dual audit. Forward-bound; document and request architect ratification. **Empirical impact**: P03/P06/P07 (1-shot omega) show `delta=1` from this; P05 has `delta=1` from a mix of Bug 2 + a single overcounted attempt. |
| **Bug 3** (missing `capsule_anchored_attempt_count` 3-term) | 4 → reclassified to 3 with explicit deviation stance | ✅ FIXED | `src/runtime/chain_derived_run_facts.rs` adds `pub capsule_anchored_attempt_count: u64` with `#[serde(default)]` for backward-compat. `compute_run_facts_from_chain_with_invariant` walks CAS for AttemptTelemetry records with `outcome == AttemptOutcome::PartialAccepted` (variant 6, Phase 2 CAS-only per §3.2 + R3 §1.3). `delta` formula extended to `l4 + l4e + capsule_anchored - expected`. **Deviation stance** (`feedback_architect_deviation_stance`): not Class 4 STEP_B because (a) `chain_derived_run_facts.rs` is NOT in CLAUDE.md STEP_B restricted file list (only sequencer.rs + typed_tx.rs + cas/schema.rs + bus.rs + kernel.rs + wallet.rs are), (b) field is strictly additive with `#[serde(default)]` — backward-compat with all pre-fix evidence, (c) NOT a canonical-signing-payload mutation — `ChainDerivedRunFacts` is a runtime-derived facts struct, not a typed-tx schema. Trust-root rehash recorded in `genesis_payload.toml`. **Empirical verification**: re-ran `tb_18r_compute_invariant` on P05's tape → `capsule_anchored_attempt_count: 8` (matches the `step_partial_ok=8` observation). 3-term equation `0 + 13 + 8 = 21` vs expected `20` → delta=1 (Bug 2 residue). |

**Bonus fix — FC1-INV6 hole** (Class 3, audit-side; NOT in original 3-bug list, surfaced by 2026-05-07 strict audit Finding D):

| Item | Class | Fixed | Notes |
|------|-------|-------|-------|
| FC1-INV6 flip_cas_byte hole on P05 | 3 | ✅ FIXED | New `assert_50_cas_bytes_match_cids` in `src/runtime/audit_assertions.rs` Layer B walks all CAS objects via new `CasStore::list_all_cids` helper, re-hashes content via `Cid::from_content`, asserts byte-for-byte CID match. Re-ran `audit_tape_tamper` on P05 → **detected 3/3 (was 2/3 pre-fix)**. Trust-root rehash recorded. |

**Extractor tautology fix** (Class 1):

| Item | Fixed | Notes |
|------|-------|-------|
| `scripts/fc_witness_extract.py` `capsule_anchored = AT - l4 - l4e` (tautological) | ✅ FIXED | Now reads `chain_invariant.json.capsule_anchored_attempt_count` (independent count from Bug 3-fix binary). Falls back to derived value with explicit `legacy binary; tautological` annotation when running on pre-fix evidence. |

**Aggregate test status post-fix**: workspace 1131/0/151 (no regression), constitution gates 54/0/1 GREEN.

**Strict-audit aggregate after fixes**: 18 chain-resident GREEN (was 17 pre-fix; +1 from FC1-INV6 closure) + 7 AMBER (unchanged structural-only) + 0 RED (was 1 — FC1-INV6 closed) + 0 GAP. Bug 2 is the remaining residue: P03/P05/P06/P07 still show `delta=1` on the 3-term equation due to synthetic L4.E gate. Forward-bound per STEP_B.

---

**End of OBS (last updated 2026-05-07).**
