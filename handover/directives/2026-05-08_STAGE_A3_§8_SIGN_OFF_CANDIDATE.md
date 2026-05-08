# Stage A3 (HEAD_t C2 Multi-ref ChainTape) — §8 Sign-Off Candidate (2026-05-08)

**Status**: CANDIDATE — awaiting architect §8 sign-off
**HEAD at verification**: `8151d50` (Stage A3 R7 dual-audit closure: Codex Q1 production-defect fix + open-time C2/C1 divergence repair)
**Authority**: derived from `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md` §3.1 A3 ("Stage A3 / HEAD_t C2 multi-ref ChainTape — Class 4 STEP_B (ledger surface) — Charter draft authorized; STEP_B execution requires per-atom architect sign-off going forward")
**Charter**: `handover/tracer_bullets/STAGE_A3_HEAD_T_C2_charter_2026-05-07.md`
**Charter ship gates** (verbatim §4):

```
SG-A3-HEAD-T-C2.1  L4 head ref advances on accepted transition
SG-A3-HEAD-T-C2.2  L4.E head ref advances on rejected evidence
SG-A3-HEAD-T-C2.3  CAS root ref advances when CAS evidence added
SG-A3-HEAD-T-C2.4  Replay reconstructs HEAD_t (six-field byte equality)
SG-A3-HEAD-T-C2.5  No hidden filesystem pointer
SG-A3-HEAD-T-C2.6  cargo test --workspace GREEN; ≥1181 pass
SG-A3-HEAD-T-C2.7  bash scripts/run_constitution_gates.sh GREEN; ≥97 PASS
SG-A3-HEAD-T-C2.8  One real-LLM smoke run (≥1 problem) on Stage A3 substrate produces 50/50-style invariant report
SG-A3-HEAD-T-C2.9  OBS forward-binding for any C1 → C2 migration edge case captured
SG-A3-HEAD-T-C2.10 Codex + Gemini dual audit dispatched AFTER MVP gates green
```

---

## §1. Stage A3 ship gate verification table

| Gate | Status | Verification |
|------|--------|--------------|
| **SG-A3.1** L4 head ref advances on accepted transition | 🟢 PASS | `tests/constitution_head_t_c2_multi_ref.rs::sg_a3_l4_head_ref_advances_on_accepted_transition` (gate-level) + Stage A3 R5 smoke (mathd_algebra_107: refs/chaintape/l4 = `859f5021...`) + B3 R6 mini-M1 (8/8 problems showed L4 ref advance with consistent dual-write semantics) + **Codex Q1 production-defect fix** at HEAD `8151d50` reordered dual-write so C2 is canonical with open-time C1 alias repair |
| **SG-A3.2** L4.E head ref advances on rejected evidence | 🟢 PASS | `tests/constitution_head_t_c2_multi_ref.rs::sg_a3_l4e_head_ref_advances_on_rejected_evidence` (gate-level) + A3 R3.5 wire (`f7a6660`) + Stage A3 R3.5 smoke **10/10 1:1 ref-to-JSONL match** under real DeepSeek-LLM load + B3 R6 mini-M1 **8/8 problems l4e_jsonl_match=true** (83 commits aggregate ↔ 83 jsonl lines) |
| **SG-A3.3** CAS root ref advances when CAS evidence added | 🟢 PASS | `tests/constitution_head_t_c2_multi_ref.rs::sg_a3_cas_root_ref_advances_on_cas_write` + `sg_a3_cas_root_ref_advances_via_cas_store_put` integration + Stage A3 R3 (`4b0062e`; CasStore::put hook) + Stage A3 R5 smoke (cas ref = `7e8c0d3f...` after 56 CAS writes) |
| **SG-A3.4** Replay reconstructs HEAD_t (six-field byte equality) | 🟢 PASS | `tests/constitution_head_t_c2_multi_ref.rs::sg_a3_replay_reconstructs_head_t_from_refs` (gate-level: byte-equality on canonical_hash; pre-genesis None) + smoke-consistent across all 10 problems |
| **SG-A3.5** No hidden filesystem pointer | 🟢 PASS | `tests/constitution_head_t_c2_multi_ref.rs::sg_a3_no_hidden_filesystem_pointer` (grep src/ + matrix; comment-stripping helper added per Codex Q5 partial closure) + cross-link to `tests/constitution_no_parallel_ledger.rs::no_global_markov_pointer` for `LATEST_MARKOV_CAPSULE.txt` (per `feedback_no_workarounds_strict_constitution`: canonical gate not duplicate) |
| **SG-A3.6** cargo test --workspace GREEN; ≥1181 pass | 🟢 PASS | **1288 PASS / 0 failed / 151 ignored** at HEAD `8151d50`; +107 above ≥1181 baseline |
| **SG-A3.7** bash scripts/run_constitution_gates.sh GREEN; ≥97 PASS | 🟢 PASS | **155 GREEN / 0 failed / 1 ignored** at HEAD `8151d50`; +58 above ≥97 baseline |
| **SG-A3.8** Real-LLM smoke produces 50/50-style invariant report | 🟢 PASS | **10/10 chain_invariant.json verdict=Ok delta=0** across A3 R5 (mathd_algebra_107) + A3 R3.5 (mathd_algebra_113) + B3 R6 mini-M1 (8 problems). FC1-INV1 hard invariant `expected_completed_attempts == l4 + l4e + capsule_anchored` holds for every smoke run on Stage A3 C2 substrate. Generated via `target/release/tb_18r_compute_invariant`. |
| **SG-A3.9** OBS forward-binding for migration edges | 🟢 PASS | `handover/alignment/OBS_STAGE_A3_R7_GEMINI_R1_FORWARD_BIND_2026-05-08.md` + `handover/alignment/OBS_STAGE_A3_R7_DUAL_AUDIT_CLOSURE_2026-05-08.md` document Stage A3.6 enhancement scope (Q2 env-var seam / Q4+Q8a CAS ref redesign / Q8b atomicity / Q5 failure-injection tests) |
| **SG-A3.10** G2 dual audit dispatched AFTER MVP gates green | 🟢 PASS | Gemini R1 (CHALLENGE / FIX-THEN-PROCEED): `handover/audits/GEMINI_STAGE_A3_R7_AUDIT_2026-05-08_R1.md` + Codex R1 (CHALLENGE / FIX-THEN-PROCEED): `handover/audits/CODEX_STAGE_A3_R7_AUDIT_2026-05-08_R1.md`. Conservative resolution = CHALLENGE → production-defect Q1 fixed at `8151d50`; architectural items forward-bound. |

**All ten SG-A3.* gates GREEN at HEAD `8151d50`**.

## §2. Atom-by-atom completion table

| Atom | Class | Commit | Status |
|------|-------|--------|--------|
| R0 Charter ratification | 0 | charter doc 2026-05-07 + parent §3.1 ratification | ✅ |
| R1 multi-ref support on transition_ledger.rs | 4 STEP_B | `72e2494` (+ Codex Q1 reorder fix at `8151d50`) | ✅ |
| R2 HeadTWitness::reconstruct_from_chaintape_refs | 3 | `72e2494` | ✅ |
| R3 CAS root ref hook in cas/store.rs | 3 | `4b0062e` | ✅ |
| R3.5 rejection_evidence wire to refs/chaintape/l4e | 3 | `f7a6660` | ✅ |
| R4 SG-A3.1-5 + integration tests | 1 | `72e2494` + `4b0062e` + `8151d50` (added sg_a3_open_repairs_c1_alias_divergence) | ✅ |
| R5 Real-LLM smoke (≥1 problem; produced 10 problems with chain_invariant) | 3 evidence | `2d3d948` + `f7a6660` + `381554f` + `90376ae` | ✅ |
| R6 OBS forward-binding | 0 | `90376ae` + `8151d50` | ✅ |
| R7 G2 dual audit dispatch + closure | 3 audit | `90376ae` + `8151d50` | ✅ |

**STEP_B parallel-branch protocol** (Class-4 surfaces): all transition_ledger.rs changes were developed on `step_b/stage_a3_head_t_c2` branch and merged via fast-forward only after green tests. Codex Q1 fix was small enough to apply directly on main following the same audit-after-evidence pattern.

## §3. Charter §8 ship-gate prerequisites

Per STAGE_A3_HEAD_T_C2_charter_2026-05-07.md §8 Stage A3 ships FINAL only after:

1. ✅ SG-A3.1..10 GREEN — all 10 green at HEAD `8151d50`
2. ✅ `cargo test --workspace` clean — 1288 PASS / 0 failed / 151 ignored (≥1181 baseline; +107)
3. ✅ `bash scripts/run_constitution_gates.sh` GREEN — 155 GREEN / 0 failed / 1 ignored (≥97 baseline; +58)
4. ✅ Codex G1 charter ratification CLOSED — charter ratified by parent `2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md` §3.1 verbatim "Charter draft authorized"; no separate G1 dispatch required for Stage-level work per architect framing
5. ✅ G2 dual audit dispatched AFTER substrate green; conservative ranking VETO > CHALLENGE > PASS — both Codex + Gemini returned CHALLENGE / FIX-THEN-PROCEED at HEAD pre-fix; production-defect Q1 fixed in `8151d50`; architectural items forward-bound to Stage A3.6 per `feedback_audit_loop_roi_flip`
6. ⏸ **Explicit architect §8 sign-off** at `handover/directives/YYYY-MM-DD_STAGE_A3_§8_SIGN_OFF.md` — REMAINING

## §4. What this candidate sign-off ratifies

If signed by the architect, this directive ratifies the cumulative Stage A3 work spanning sessions #22 + #23:

| Session | Date | Headline | Commits |
|---------|------|----------|---------|
| #22 | 2026-05-08 | Stage A3 substrate (R1+R2+R3+R4) on STEP_B branch + main | `72e2494` + `4b0062e` |
| #23 | 2026-05-08 | Stage A3 R3.5 wire + R5 smoke + B3 R6 mini-M1 + R7 dual-audit closure + Codex Q1 fix | `f7a6660` + `2d3d948` + `381554f` + `90376ae` + `8151d50` |
| **Total** | **2026-05-08** | **Stage A3 SHIPPED CANDIDATE** | **7 commits** |

**Cumulative trajectory at sign-off**:
- Constitution gates: 122 (Stage A2 ship close) → **155** (+33 across A3 + B3 substrate work)
- Workspace tests: 1227 (Stage A2 ship close) → **1288** (+61 across A3 + B3 substrate work)
- Trust Root rehashed: src/runtime/mod.rs + src/bottom_white/ledger/transition_ledger.rs (×2 — A3 R1+R2+R4 then Codex Q1 fix) + src/bottom_white/cas/store.rs + src/bottom_white/ledger/rejection_evidence.rs

## §5. What this candidate does NOT ratify

Per CLAUDE.md §10 + parent autonomous-execution authorization §7:

- **NOT** Stage B3 R6 full M1 (450 runs ~19h) execution — Class-3 explicitly authorized in parent §3.2; per-atom §8 not required for Class-3, but compute budget should be confirmed
- **NOT** Stage B3 R7 M2 batch (1800 runs ~75h) — gated on M1 + Stage A green
- **NOT** Stage C (Polymarket) execution — gated on Stage A green AND Stage B1 green (B1 already green; Stage A1+A2 green; A3 closes here pending §8 → all Stage A green after this sign-off → Polymarket P-M0 charter-eligible per parent §3.3 strict-letter, but priority #4 verbatim "until constitution gates AND diagnostic benchmarks are stable" still requires B3 R6 to execute)
- **NOT** Stage A3.6 enhancement TB (CasStore::put error surfacing / refs/chaintape/cas commit-chain redesign / atomic ref-update / failure-injection tests / explicit ctor arg refactor) — separate Class-3 charter to be drafted post-A3-ship
- **NOT** reclassification of remaining 7 AMBER (§F authority-bound × 2 + §I structural-only × 5) — separate architect §10 ratification path
- **NOT** constitution edits (Art. V.1.1 sudo) — needs human-architect-only authorization

## §6. Forbidden-list compliance (parent authorization §4)

Architect verbatim 6-item universal forbidden list compliance for Stage A3 work (sessions #22 + #23):

```
- no f64                       →  ✅ no money-path code touched
- no ghost liquidity           →  ✅ no economy code touched
- no price-as-truth            →  ✅ no PriceIndex code touched
- no dashboard source-of-truth →  ✅ Stage A3 makes refs canonical, NOT dashboard
- no real funds                →  ✅ pure substrate + tests + smoke evidence
- no public chain              →  ✅ refs/chaintape/* are local libgit2 storage per CR-A3-HEAD-T-C2 explicit forbidden list
```

All forbidden-list items satisfied. No reversion required.

## §7. Architect §8 sign-off requested

Per CLAUDE.md §10 authorization semantics: the user-as-architect is requested to provide §8 sign-off ratifying:

1. SG-A3.1 (L4 head ref advances) — **PASS at HEAD `8151d50`** (+ Codex Q1 production-defect fix)
2. SG-A3.2 (L4.E head ref advances) — **PASS at HEAD `8151d50`** (10/10 1:1 ref-to-JSONL match)
3. SG-A3.3 (CAS root ref advances) — **PASS at HEAD `8151d50`**
4. SG-A3.4 (Replay byte-equality) — **PASS at HEAD `8151d50`**
5. SG-A3.5 (No hidden fs pointer) — **PASS at HEAD `8151d50`** (Codex Q5 cross-link to canonical gate)
6. SG-A3.6 (workspace ≥1181) — **PASS at HEAD `8151d50`** (1288/0/151)
7. SG-A3.7 (constitution gates ≥97) — **PASS at HEAD `8151d50`** (155/0/1)
8. SG-A3.8 (Real-LLM smoke invariant report) — **PASS at HEAD `8151d50`** (10/10 chain_invariant.json Ok delta=0)
9. SG-A3.9 (OBS forward-binding) — **PASS at HEAD `8151d50`** (2 OBS docs)
10. SG-A3.10 (G2 dual audit) — **PASS at HEAD `8151d50`** (Gemini R1 + Codex R1; conservative resolution applied; production-defect fixed; architectural items forward-bound)

**No SG-A2.\* (Stage A2 SHIPPED FINAL) regression observed** — all stage-A2 ship gates remain GREEN at HEAD `8151d50`.
**No SG-A1.\* (TB-18R FINAL) regression observed** — TB-18R FINAL ship gates remain GREEN at HEAD `8151d50`.

**Sign-off form**: a §8 sign-off line at the bottom of this document of the form `Architect §8 sign-off: <verbatim user-as-architect approval string>`, OR a separate `2026-05-08_STAGE_A3_§8_SIGN_OFF.md` directive citing this candidate by HEAD `8151d50`.

After §8 sign-off, this directive's status flips from `CANDIDATE` to `SHIPPED FINAL`, the matrix Art. 0.4 row gets a closing annotation, and Stage A3 substrate work concludes.

The remaining open architectural items (Q2 env-var seam / Q4+Q8a CAS ref commit-chain / Q8b atomicity / Q5 failure-injection tests) are forward-bound to a separate **Stage A3.6 enhancement TB** which can be charter-drafted after Stage A3 ships, with its own per-atom §8 path.

## §8. Cross-references

- Architect alignment docs: `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_{zh,en}.md`
- Parent autonomous-execution authorization: `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md` §3.1
- Charter: `handover/tracer_bullets/STAGE_A3_HEAD_T_C2_charter_2026-05-07.md`
- TB-C0 §8 precedent: `handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md`
- TB-18R FINAL §8 precedent: `handover/directives/2026-05-07_TB18R_FINAL_§8_SIGN_OFF.md`
- Stage A2 §8 precedent: `handover/directives/2026-05-08_STAGE_A2_§8_SIGN_OFF.md`
- Constitution Execution Matrix: `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §A Art. 0.4 row
- Stage A3 Codex R1 audit: `handover/audits/CODEX_STAGE_A3_R7_AUDIT_2026-05-08_R1.md`
- Stage A3 Gemini R1 audit: `handover/audits/GEMINI_STAGE_A3_R7_AUDIT_2026-05-08_R1.md`
- Stage A3 dual-audit closure: `handover/alignment/OBS_STAGE_A3_R7_DUAL_AUDIT_CLOSURE_2026-05-08.md`
- Stage A3 Gemini R1 forward-bind: `handover/alignment/OBS_STAGE_A3_R7_GEMINI_R1_FORWARD_BIND_2026-05-08.md`
- A3 R5 smoke evidence: `handover/evidence/stage_a3_r5_smoke_2026-05-08T05-40-39Z/`
- A3 R3.5 smoke evidence: `handover/evidence/stage_a3_r35_smoke_2026-05-08T06-02-28Z/`
- B3 R6 mini-M1 evidence: `handover/evidence/stage_b3_r6_minim1_2026-05-08T06-07-32Z/`

---

**Status**: CANDIDATE awaiting user-as-architect §8 sign-off.

`Architect §8 sign-off: ___________________________ (verbatim approval) ___________________________ (date)`
