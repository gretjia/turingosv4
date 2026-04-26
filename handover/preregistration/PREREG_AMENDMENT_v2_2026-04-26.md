# PREREG Amendment v2 — PPUT-CCL Arc Rescope (D1=C MVP-Pivot)

> **Status**: DRAFT awaiting user formal sudo on wake.
>
> **Supersedes**: amendment v1 (`PREREG_AMENDMENT_p0_defer_2026-04-25.md`) for arc-state question only; p_0 calibration deferral remains active.
>
> **Drafted**: 2026-04-26 night shift, ArchitectAI auto-research mode.

---

## § 1 Why amend

Original PREREG (`PREREG_PPUT_CCL_2026-04-26.md`) assumed Phase C (5-mode ablation) runs on the v4 kernel **as-is**. Post-2026-04-26 ultrathink:
- Constitution Art. 0-0.4 amendments declared current kernel violates Tape Canonical Axiom (24 V-violations)
- Economic chapter mandates 12 economic invariants + 9 RSP modules currently absent
- White paper architecture chapter mandates 反奥利奥 3-layer + Q_t 9-component structure not yet implemented

Running Phase C on the unrefactored kernel would produce results that are **not architecturally meaningful** — even if H1-H4 hypotheses statistically resolved, the underlying kernel does not satisfy white paper structure.

Three options were on the table (per CO_MEGA_PLAN_v3.1 § 8 D1):

- **A — PAUSE**: heldout-54 reserved; arc resumes after CO Phase 2 exit (~17-21 weeks)
- **B — NEGATIVE**: declare arc failed; new PREREG written post-CO; heldout-54 not reserved
- **C — MVP-pivot**: abbreviated Phase C run after **CO Phase 1 only** (no full RSP); compromise

User auto-research delegation 2026-04-26 night defaults to **C** per ArchitectAI recommendation.

---

## § 2 Amended arc state

### Phase A (PREREG draft + foundation)
- **Status**: COMPLETED 2026-04-26 (commits up to 5568a78)
- **Use**: results PRESERVED but **not used inferentially** (kernel was pre-CO refactor)
- **Reason**: Phase A produced PREREG doc + budgeting + tooling; that scaffolding survives the refactor

### Phase B (kernel instrumentation)
- **Status**: COMPLETED 2026-04-26 (B1-B7-extra)
- **Use**: results PRESERVED but **not used inferentially**
- **Reason**: instrumentation added jsonl schema, post-hoc verifier, wall clock, etc.; surviving artifacts will be re-validated post-CO P1; no inferential dependency

### Phase C (5-mode ablation, 100 jsonl rows)
- **Original schedule**: 2026-04-26 → 2026-04-29 (3 days)
- **Status**: SUSPENDED ahead of CO Phase 1
- **Restart gate**: CO P1.14 exit dual audit PASS/PASS
- **Restart scope (MVP-pivot)**:
  - 5 modes ablation **preserved** (Full / SoftLaw / Homogeneous / Panopticon / Amnesia per `experiments/.../src/experiment_mode.rs`)
  - hard-10 problem set **preserved** (sealed `PPUT_CCL_HARD10_2026-04-26.json`)
  - **Reduced**: 2 seeds → 1 seed initially (50 jsonl rows instead of 100); upgrade to 2 seeds if early signal warrants
  - **Not run**: full RSP-economy effects (CO Phase 2); Phase C runs against CO P1 kernel only (Anti-Oreo + ChainTape + predicate registry; economy still partial — current invest/short scaffolding)
- **Conformance test before C2 batch**: `tests/q_state_reconstruct.rs` + `tests/anti_oreo_layer_audit.rs` + `tests/chain_tape_L0..L6.rs` + 24 V conformance must all PASS
- **Earliest restart date**: ~10 weeks from CO P0.7 audit PASS (CO P1 takes 8-10 weeks)

### Phase D (shadow CCL on heldout-49)
- **Status**: NOT STARTED, deferred
- **Restart gate**: Phase C MVP results posted; if early signal supports H1-H4 (or rejects them clearly), Phase D launches; otherwise re-plan
- **Earliest restart date**: ~12 weeks from CO P0.7 audit PASS

### Phase E (sealed eval on heldout-54)
- **Status**: NOT STARTED, **heldout-54 reservation HONORED**
- **Restart gate**: depends on Phase D outcome AND CO P2 exit (full RSP economy live)
- **Earliest restart date**: ~17-21 weeks from CO P0.7 audit PASS (basically v4 ship date)

---

## § 3 Heldout-54 sealing — explicit reservation

Per original PREREG § 6, heldout-54 is reserved for Phase E sealed eval. Under D1=C (MVP-pivot), this reservation **remains intact**:

- heldout-54 problem IDs continue locked; no querying / sampling / training-on permitted
- splits file `handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json` SHA preserved in TR
- post-CO P1.14 audit must include explicit heldout-54 sealing re-validation

If at any point user reverses to D1=B (NEGATIVE), heldout-54 reservation lifts and a new PREREG starts fresh.

---

## § 4 Budget impact

| Phase | Pre-amendment | Post-amendment v2 |
|---|---|---|
| Phase A spend | ~$15-20 | unchanged |
| Phase B spend | ~$3-5 | unchanged |
| Phase A→B exit audit | ~$80 | unchanged |
| **Sub-total pre-CO** | ~$100 | ~$100 |
| **CO Phase 0+1+2** (per Plan v3.1 + Protocol) | n/a | $435-950 |
| Phase C MVP (50 rows) | n/a (was $50-80 for 100 rows) | $25-40 (50 rows post-CO P1) |
| Phase D shadow (heldout-49) | $30-50 | $30-50 (post-Phase C) |
| Phase E sealed (heldout-54) | $30-50 | $30-50 (post-CO P2) |
| **Total v4 arc** | ~$210-280 | **~$620-1190** |

Budget cap of $500 (original) is **insufficient** under MVP-pivot. Amendment v2 raises cap to **$1,200** with the same weekly burn-rate review process.

---

## § 5 Inferential validity claim adjustments

> **Reframe per Gemini CO P0.7 audit Q10 CHALLENGE (both runs)**: the 50-row × 1-seed Phase C MVP run is **NOT a hypothesis test**. It is a **post-refactor integration sanity check** with the following limited claims:
> - Catches **catastrophic regressions** (e.g., a mode crashes on >50% of cells)
> - Verifies **end-to-end pipeline plumbing** post-CO P1 refactor (5 modes execute, jsonl emits, kernel boots)
> - Provides **directional signal** on whether H1-H4 are still plausible to test in Phase D
>
> **Forbidden claims from MVP run**:
> - "H1-H4 are supported / rejected"
> - "Phase C results are statistically significant"
> - "PPUT-CCL hypothesis is validated"
>
> The MVP run is a **gate to Phase D**, not a substitute for Phase D.

- Phase C MVP (50 rows × 1 seed) at α=0.05: McNemar test on H1 needs 9-10/10 wins to reject; with 1 seed instead of 2, wins are correlated to seed-specific noise. **Mitigation**: pre-register seed selection criterion; if Phase C MVP fails the **directional signal** check (e.g., < 6/10 wins for the "Full" mode), re-launch with a 2nd seed before declaring NEGATIVE. Phase D required for any inferential claim.
- Phase D / E inferential validity unchanged from original PREREG.

---

## § 6 What this amendment does NOT change

- PREREG § 5.5 calibration (still deferred per amendment v1; p_0 = 0.10 ceiling)
- 7 red-line check unchanged
- Statistical method unchanged (McNemar paired sign + Holm-Bonferroni at α=0.05)
- hard-10 + heldout-49 + heldout-54 problem IDs unchanged

---

## § 7 Enactment requirements

User on wake must:

1. Confirm D1=C (or override to A/B)
2. cp this draft → `handover/preregistration/PREREG_AMENDMENT_v2_2026-04-26.md` (already that path; just review)
3. Update Trust Root manifest (CO P0.6) entry for this file
4. Update `handover/ai-direct/LATEST.md` next-session boot sequence to reflect arc state SUSPENDED
5. Acknowledge new $1,200 budget cap (or override to lower)

— ArchitectAI, 2026-04-26 night DRAFT
