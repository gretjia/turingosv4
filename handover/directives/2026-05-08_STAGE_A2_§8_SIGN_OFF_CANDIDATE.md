# Stage A2 (Constitution AMBER Closure) — §8 Sign-Off Candidate (2026-05-08)

**Status**: SHIPPED FINAL — ratified by `handover/directives/2026-05-08_STAGE_A2_§8_SIGN_OFF.md` (architect §8 verbatim "好，确认可以 ship", 2026-05-08).
**HEAD at verification**: `4c9f767` (`tests/constitution_shielding_evidence_binding.rs` registered + `constitution_diversity` + `constitution_wilson_ci` registered to gate runner)
**Authority**: derived from `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md` §3.1 A2 ("Constitution AMBER closure — Class 1 — YES — execute as ship-eligible Class-1 work")
**Architect ship gates**: `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md` §A.A2 verbatim:

```
SG-A2.1 constitution gates >= current 97 and no regression.
SG-A2.2 all new gate files included in scripts/run_constitution_gates.sh.
SG-A2.3 every matrix promotion has a real witness.
SG-A2.4 no doc-only GREEN promotions.
```

---

## §1. Stage A2 ship gate verification table

| Gate | Status | Verification |
|------|--------|--------------|
| **SG-A2.1** constitution gates ≥ 97 + no regression | 🟢 PASS | Current `bash scripts/run_constitution_gates.sh` reports **122 passed, 0 failed, 1 ignored, all gates GREEN**. 122 ≥ 97 architect baseline. 0 failed = no regression. |
| **SG-A2.2** all new gate files registered to runner | 🟢 PASS | `tests/constitution_*.rs` (20 files) ↔ `scripts/run_constitution_gates.sh` GATES array (20 entries) — exact diff match. Session-#21 mid-verification gap (`constitution_diversity` + `constitution_wilson_ci` from session #19 unregistered) closed at HEAD `4c9f767`. |
| **SG-A2.3** every matrix promotion has a real witness | 🟢 PASS | 39/40 cited tests (`grep -oE 'tests/[a-z0-9_]+\.rs::[a-z0-9_]+' CONSTITUTION_EXECUTION_MATRIX.md`) are real `#[test] fn` definitions; the 1 hedge "(if exists)" is on §F Art. V.2 row which is currently 🟡 AMBER (NOT a promotion). All 39 cited GREEN-row tests pass under `cargo test --workspace` (1227 passed, 0 failed, 151 ignored). |
| **SG-A2.4** no doc-only GREEN promotions | 🟢 PASS | `tests/constitution_closure_3_no_trivial_asserts.rs` 3/3 PASS — scanner enforces `assert!(true)` / `assert_eq!(1,1)` / 7 other trivial-pass patterns absent across all 20 `tests/constitution_*.rs` files (self-verifying via `forbidden_patterns_list_is_load_bearing`). Matrix grep for `doc.?only` / `comment.?only` against GREEN rows returns empty. |

**All four SG-A2.* gates GREEN at HEAD `4c9f767`**.

## §2. Stage A1 (TB-18R FINAL) re-verification at current HEAD (no regression)

Since TB-18R was shipped at session #18 (commit `feec129` + architect §8 `2026-05-07_TB18R_FINAL_§8_SIGN_OFF.md`), session #19/#20/#21 work could in principle regress SG-A1.* gates. Re-verification at HEAD `4c9f767`:

| Gate | Status | Verification |
|------|--------|--------------|
| **SG-A1.1** P38 attempt equality green | 🟢 PASS | Wave 3 50p binding `wave3_50p_chain_invariant_all_pass` 50/50 verdict=Ok delta=0 (P38-class re-validated at 50p scale per matrix §A.0.4 row text) |
| **SG-A1.2** P49 attempt equality green | 🟢 PASS | Same Wave 3 50p binding (P49-class included at 50p scale; pre-TB-18R baseline P49 32-vs-1 mismatch eliminated) |
| **SG-A1.3** M0 mini-batch green | 🟢 PASS | `handover/evidence/m0_minif2f_harness_audit_2026-05-05/` contains `M0_RUN_MANIFEST.json` + per-problem dirs + `m0_runner.log` + `r1/` round evidence. MVP-3 `dashboard_regenerates_from_tape_cas` PASS confirms tape regeneration works at M0 scale. |
| **SG-A1.4** no fake accepted nodes | 🟢 PASS | `tests/constitution_fc1_runtime_loop.rs::fc1_no_fake_accepted_nodes` PASS at HEAD `4c9f767` |
| **SG-A1.5** every Lean reject in L4.E or anchored capsule | 🟢 PASS | `tests/constitution_predicate_gate.rs::predicate_failure_cannot_enter_l4` PASS + `tests/tb_18r_attempt_routes_to_l4_or_l4e.rs` 5/5 PASS at HEAD `4c9f767` |
| **SG-A1.6** chain facts derived from ChainTape/CAS not evaluator stdout | 🟢 PASS | `tests/constitution_tape_canonical_gate.rs::chain_derived_facts_not_evaluator_stdout` + `dashboard_regenerates_from_tape_cas` PASS + Wave 3 50p `wave3_50p_dashboard_regen_matches_chain` 50/50 PASS |
| **SG-A1.7** final dual audit PASS under VETO > CHALLENGE > PASS | 🟢 PASS (historical) | Session #18 archive: Codex 5-round audit VETO→CHALLENGE→CHALLENGE→PASS→PASS aggregate; Gemini R1 Q1-Q7 PASS / Q8 CHALLENGE-forward-bound to TB-18B. Not re-runnable autonomously today; verified by `handover/directives/2026-05-07_TB18R_FINAL_§8_SIGN_OFF.md` SG-18R closure record. |

**All seven SG-A1.* gates remain GREEN at HEAD `4c9f767`**.

## §3. What this candidate sign-off ratifies

If signed by the architect, this directive ratifies the cumulative Stage A2 work spanning sessions #19, #20, and #21:

| Session | Date | Headline | AMBER closed |
|---------|------|----------|-------------:|
| #19 | 2026-05-08 | Wilson 95% CI helper + DiversityReport helper + 8 row promotions via Wave 3 50p binding | 8 |
| #20 | 2026-05-08 | Closure-3 mechanical CR-C0.1 enforcement + memory-only-preseed Wave 3 50p binding | 3 |
| #21 | 2026-05-08 | Wave 3 50p CAS-index shielding evidence binding (§C Art. II.1 + §D Art. III.1-4 + §K shielding 4 mirror rows) + session-#19 unregistered gates fixed | 9 |
| **Total** | **2026-05-08** | **Stage A2 cumulative closure** | **20** |

**Matrix true-AMBER trajectory**: 28 (pre-#19) → 19 (#19) → 16 (#20) → **7** (#21).

**Constitution gates trajectory**: 90 (pre-#19) → 97 (architect baseline) → 101 (#20) → 110 (#21 CAS-index binding) → **122** (#21 + session-#19-unregistered-gates fixed).

**Workspace tests trajectory**: 1174 (pre-#19) → 1181 (#19) → 1214 (post-#19 fix) → 1218 (#20) → **1227** (#21).

## §4. What this candidate does NOT ratify

Per `feedback_class4_cannot_hide_in_class3` + CLAUDE.md §10:

- **NOT** Stage A3 (HEAD_t C2 multi-ref) execution — Class-4 STEP_B per `STAGE_A3_HEAD_T_C2_charter_2026-05-07.md`; per-atom architect §8 still required
- **NOT** Stage B3 (TB-18B / 100p / M2) execution — Class-3 explicitly authorized per parent `2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md` §3.2; `TB-18B_charter_2026-05-07.md` execution per-atom §8 needed
- **NOT** Stage C (Polymarket P-M0..P-M9) execution — gated on Stage A green AND Stage B1 green per parent §3.3
- **NOT** Stage D (real-world readiness) activation — directive draft only per parent §3.4
- **NOT** reclassification of remaining 7 AMBER rows from 🟡 → 🚫 N/A — that decision touches CLAUDE.md §10 Class-4 boundary (matrix legend semantic shift in protection-weakening direction); requires separate architect §10 ratification path

## §5. Remaining 7 AMBER (out of Stage A2 scope per architect "no-dependency static and parser/manifest" framing)

These 7 rows were classified by architect language at §A.A2 verbatim ("Close remaining no-dependency static and parser/manifest AMBER rows"). The 7 below have *dependencies* (architect signature) or are *structural-only-by-design* (procedural / not chain-witnessable). They are out of A2's defined scope and forward to either §10 ratification or forward-TB integration.

| Article | Row | Class |
|---------|-----|-------|
| §F Art. V.1.2 | ArchitectAI proposes (NOT direct write) | authority-bound (procedural witness = human signature pattern) |
| §F Art. V.2 | constitution boundaries | authority-bound (constitution.md hash drift = architect signature) |
| §I FC3 | Raw logs not in agent read view | structural-only-by-design (runtime prompt construction not asserted) |
| §I FC3 | Latest capsule = context only | structural-only-by-design |
| §I FC3 | Deep history requires override | structural-only-by-design (env-var grep, no runtime integration test) |
| §I FC3 | ArchitectAI proposes, no direct write | structural-only-by-design (mirror of §F V.1.2) |
| §I FC3 | JudgeAI veto-only | structural-only-by-design (judge role is procedural) |

Per CLAUDE.md §22 supreme order ("constitution > flowcharts > ChainTape/CAS > executable gates > reports") + edge-case rule from parent authorization §5: these 7 remain 🟡 AMBER **with explicit "by design" annotations in matrix row text** until architect §10 ratification path runs.

## §6. Architect ship gate dispatch

**Verbatim from architect 2026-05-07 alignment doc en §A.A2**:

```
SG-A2.1 constitution gates >= current 97 and no regression.    →  🟢 PASS (122 ≥ 97; 0 failed)
SG-A2.2 all new gate files included in scripts/run_constitution_gates.sh.  →  🟢 PASS (20/20 match)
SG-A2.3 every matrix promotion has a real witness.            →  🟢 PASS (39/39 GREEN-row tests real)
SG-A2.4 no doc-only GREEN promotions.                          →  🟢 PASS (closure-3 scanner clean)
```

**Aggregate Stage A2 verdict**: **🟢 ALL FOUR SHIP GATES GREEN AT HEAD `4c9f767`**.

## §7. Forbidden-list compliance (parent authorization §4)

Architect verbatim 6-item universal forbidden list compliance for Stage A2 work (sessions #19/#20/#21):

```
- no f64                       →  ✅ Sessions touched no money-path code
- no ghost liquidity           →  ✅ Sessions touched no economy code
- no price-as-truth            →  ✅ Sessions touched no PriceIndex code
- no dashboard source-of-truth →  ✅ Wave 3 50p binding ASSERTS dashboard regenerable; no SoT inversion
- no real funds                →  ✅ Pure tests + matrix + handover
- no public chain              →  ✅ Pure local CAS + ChainTape evidence
```

All forbidden-list items satisfied. No reversion required.

## §8. Architect §8 sign-off requested

Per CLAUDE.md §10 authorization semantics: the user-as-architect is requested to provide §8 sign-off ratifying:

1. SG-A2.1 (constitution gates ≥ 97 + no regression) — **PASS at HEAD `4c9f767`**
2. SG-A2.2 (all new gate files registered) — **PASS at HEAD `4c9f767`**
3. SG-A2.3 (every matrix promotion has real witness) — **PASS at HEAD `4c9f767`**
4. SG-A2.4 (no doc-only GREEN promotions) — **PASS at HEAD `4c9f767`**

**No SG-A1.* regression observed** — TB-18R FINAL ship gates remain GREEN at current HEAD.

**Sign-off form**: a §8 sign-off line at the bottom of this document of the form `Architect §8 sign-off: <verbatim user-as-architect approval string>`, OR a separate `2026-05-08_STAGE_A2_§8_SIGN_OFF.md` directive citing this candidate by HEAD `4c9f767`.

After §8 sign-off, this directive's status flips from `CANDIDATE` to `SHIPPED FINAL`, the matrix Stage A2 row gets a closing annotation, and Stage A2 work concludes the 28 → 7 cumulative AMBER closure.

The remaining 7 AMBER are out-of-A2-scope and forward to a separate architect §10 ratification path (recommended next: re-classify §F authority-bound + §I structural-only-by-design rows as 🚫 N/A with explicit per-row reasoning, OR forward-bind to TB-18B / TB-Wave12 integration tests).

## §9. Cross-references

- Architect alignment docs: `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_{zh,en}.md`
- Parent autonomous-execution authorization: `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md`
- TB-C0 §8 precedent: `handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md`
- TB-18R FINAL §8 precedent: `handover/directives/2026-05-07_TB18R_FINAL_§8_SIGN_OFF.md`
- Constitution Execution Matrix: `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
- Gate runner: `scripts/run_constitution_gates.sh`
- Gate report (machine-readable): `target/constitution_gate_report.json`
- Gate report (human-readable): `target/constitution_gate_report.md`
- Session #19 LATEST: `handover/ai-direct/LATEST.md` § "session end #19"
- Session #20 LATEST: `handover/ai-direct/LATEST.md` § "session end #20"
- Session #21 LATEST: `handover/ai-direct/LATEST.md` § "session end #21"

---

**Status**: SHIPPED FINAL.

`Architect §8 sign-off: 好，确认可以 ship (2026-05-08, user-as-architect verbatim; ratified separately at handover/directives/2026-05-08_STAGE_A2_§8_SIGN_OFF.md)`
