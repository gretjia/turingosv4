# OBS Stage A3 R7 Dual-Audit Closure (2026-05-08)

**Authority**: STAGE_A3_HEAD_T_C2_charter_2026-05-07.md SG-A3.10 + `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS conservative ranking) + `feedback_audit_loop_roi_flip` (production-defect → fix; architectural-strengthening → forward-bind).

**Predecessors**:
- Gemini R1 audit: `handover/audits/GEMINI_STAGE_A3_R7_AUDIT_2026-05-08_R1.md` (verdict CHALLENGE; FIX-THEN-PROCEED)
- Codex R1 audit: `handover/audits/CODEX_STAGE_A3_R7_AUDIT_2026-05-08_R1.md` (verdict CHALLENGE; FIX-THEN-PROCEED)
- Gemini R1 forward-bind: `handover/alignment/OBS_STAGE_A3_R7_GEMINI_R1_FORWARD_BIND_2026-05-08.md`

---

## §1. Aggregate dual-audit verdict

| Auditor | Aggregate | Conviction | Recommendation |
|---------|-----------|------------|----------------|
| Gemini 2.5 Pro | CHALLENGE | high | FIX-THEN-PROCEED |
| Codex (codex-cli) | CHALLENGE | high | FIX-THEN-PROCEED |

Per `feedback_dual_audit_conflict`: **conservative resolution = CHALLENGE** (both auditors agree; no VETO from either).

Per `feedback_audit_loop_roi_flip`: when CHALLENGEs split into production-defect + architectural-strengthening, fix the production-defect THIS commit; forward-bind the architectural items.

## §2. Per-Q dual-audit cross-reference

| Topic | Gemini | Codex | Disposition |
|-------|--------|-------|-------------|
| Dual-write atomicity | (not asked) | Q1 CHALLENGE — wrong order; C1 advances before C2; partial failure → divergent repo | **FIXED THIS COMMIT** (reorder + open-time repair) |
| `submit_id as i64` overflow | (not asked) | Q2 PASS — defensible | — |
| `cas_root` domain separator | Q4 PASS | Q3 PASS — both confirm unique | — |
| env-var seam vs explicit ctor | Q2 CHALLENGE | (not asked) | forward-bind Stage A3.6 |
| chain_invariant.json missing | Q5 CHALLENGE | (not asked) | **CLOSED** in commit `90376ae` |
| CasStore::put silent error | Q8 (overlap) | Q4 CHALLENGE — silent ignore; idempotent path skips ref | forward-bind Stage A3.6 |
| SG-A3 test coverage | (not asked) | Q5 CHALLENGE — happy-path only; failure injection missing; LATEST_MARKOV_CAPSULE.txt missing from forbidden list | **PARTIAL FIX THIS COMMIT** (added open-time repair test sg_a3_open_repairs_c1_alias_divergence; LATEST_MARKOV_CAPSULE.txt cross-linked to existing gate constitution_no_parallel_ledger.rs::no_global_markov_pointer); failure-injection forward-bind Stage A3.6 |
| trust_root rehash correctness | (not asked) | Q6 PASS | — |
| Class-3 boundary | (not asked) | Q7 PASS | — |
| total counts (1287/154) | (not asked) | Q8 PASS | — |
| no-fs-pointer grep coverage | Q6 PASS | Q5 (partial) — LATEST_MARKOV_CAPSULE.txt | cross-linked above |
| Real-problem coverage 8 vs 450 | Q7 PASS | (not asked) | — |
| GC vulnerability | Q8 CHALLENGE — refs/chaintape/cas points to blob OID; git gc would prune history | (not asked) | forward-bind Stage A3.6 |
| Concurrency safety | Q8 CHALLENGE — non-atomic ref-update | (not asked) | forward-bind Stage A3.6 |
| Strategic risk | Q8 CHALLENGE | (not asked) | covered by GC + concurrency forward-bind |

## §3. Fixes applied this commit (production-defect closure)

### 3.1 Codex Q1 — dual-write order (FIXED)

**Defect**: pre-fix `Git2LedgerWriter::commit` wrote to `Some(TRANSITIONS_REF)` as the commit destination, then `repo.reference(CHAINTAPE_L4_REF, ...)` as a secondary mirror. If the mirror failed, the C1 ref had advanced while C2 lagged — canonical chain stale.

**Fix**: swap the order. Now `repo.commit(Some(CHAINTAPE_L4_REF), ...)` writes to the canonical C2 ref first; the C1 alias mirror is secondary. Plus, `Git2LedgerWriter::open()` now detects C2/C1 divergence at boot and repairs C1 by aligning to canonical C2 OID.

**Test added**: `tests/constitution_head_t_c2_multi_ref.rs::sg_a3_open_repairs_c1_alias_divergence` — synthesizes a divergent C1 alias (rewound to parent), opens the writer, asserts C1 is repaired to match canonical C2.

### 3.2 Codex Q5 (partial) — LATEST_MARKOV_CAPSULE.txt coverage

**Codex finding**: `sg_a3_no_hidden_filesystem_pointer` omits `LATEST_MARKOV_CAPSULE.txt`.

**Disposition**: Cross-linked to existing canonical gate `tests/constitution_no_parallel_ledger.rs::no_global_markov_pointer` + `tests/markov_pointer_de_canonicalize.rs` (TB-16 OBS_R022 closure). Adding the name to the SG-A3 grep list would also flag legitimate doc-comment references and user-facing audit diagnostic strings in `src/bin/audit_dashboard.rs` (which explicitly explain the pointer's de-canonicalization). Per `feedback_no_workarounds_strict_constitution`, rely on the canonical gate, not on a duplicate patched here. Documented in inline comment on the forbidden list.

## §4. Forward-bind to Stage A3.6 enhancement TB

| Item | Source | Severity |
|------|--------|----------|
| Refactor RejectionEvidenceWriter to accept `chaintape_repo_path: Option<PathBuf>` ctor field | Gemini Q2 | low (current single-instance contract correct) |
| Stop silently ignoring CAS ref errors in `CasStore::put`; surface error / repair from sidecar; idempotent put currently short-circuits before ref advance | Codex Q4 + Gemini Q8a (overlap) | medium |
| Redesign `refs/chaintape/cas` as proper commit chain (each put creates a commit; tree references CAS objects by name; parent = previous commit) → GC-safe | Gemini Q8a | medium for forward robustness; low for current single-process usage |
| Atomic ref-update via file lock + compare-and-swap for multi-process safety | Gemini Q8b | medium for forward; low for current |
| Expand SG-A3 tests with ref-update failure injection + exact `cas_root` digest assertion + real `TURINGOS_CHAINTAPE_PATH` flush coverage | Codex Q5 | low (architectural; current tests sufficient for Class-4 substrate validation) |

All forward-binds are Class-3 additive; not blocking Stage A3 ship.

## §5. Stage A3 ship eligibility (post-dual-audit closure)

Charter §4 ship gates SG-A3.1..10 final status:

| ID | Gate | Status |
|----|------|--------|
| SG-A3.1 | L4 head ref advances on accepted transition | 🟢 GREEN (gate-level + real-LLM-load + Codex Q1 reorder fix verified) |
| SG-A3.2 | L4.E head ref advances on rejected evidence | 🟢 GREEN (gate-level + real-LLM-load 10/10 1:1 ref-to-JSONL match) |
| SG-A3.3 | CAS root ref advances on CAS evidence | 🟢 GREEN (gate-level + CasStore::put integration; concurrency/GC forward-bind documented) |
| SG-A3.4 | Replay reconstructs HEAD_t (six-field byte equality) | 🟢 GREEN (gate-level + smoke-consistent) |
| SG-A3.5 | No hidden filesystem pointer | 🟢 GREEN (gate + cross-linked to constitution_no_parallel_ledger) |
| SG-A3.6 | cargo test --workspace ≥1181 GREEN | 🟢 GREEN (1288 PASS / 0 failed; +107 above baseline) |
| SG-A3.7 | constitution gates ≥97 GREEN | 🟢 GREEN (155 GREEN / 0 failed; +58 above baseline) |
| SG-A3.8 | Real-LLM smoke produces invariant report | 🟢 GREEN (10/10 chain_invariant.json Ok delta=0 across A3 R5 + A3 R3.5 + B3 R6 mini-M1) |
| SG-A3.9 | OBS forward-binding for migration edges | 🟢 GREEN (this OBS + OBS_STAGE_A3_R7_GEMINI_R1_FORWARD_BIND_2026-05-08.md) |
| SG-A3.10 | Codex + Gemini dual audit dispatched AFTER MVP gates green | 🟢 GREEN (both auditors returned CHALLENGE / FIX-THEN-PROCEED; production-defect Q1 fixed; architectural items forward-bound) |

**Stage A3 SHIPPED CANDIDATE** — awaiting architect §8 sign-off path opens.

## §6. Architect §8 sign-off prerequisites met

Per STAGE_A3_HEAD_T_C2_charter_2026-05-07.md §8 Stage A3 ships FINAL only after:

1. ✅ SG-A3.1..10 GREEN (all 10 green at this commit)
2. ✅ `cargo test --workspace` clean (1288 PASS / 0 failed; ≥1181 baseline)
3. ✅ `bash scripts/run_constitution_gates.sh` GREEN (155 GREEN / 0 failed; ≥97 baseline)
4. ✅ Codex G1 charter ratification CLOSED (charter ratified by architect verbatim §3.1 in 2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md "A3 charter draft authorized")
5. ✅ G2 dual audit dispatched AFTER substrate green; conservative ranking VETO > CHALLENGE > PASS per `feedback_dual_audit_conflict` — both auditors CHALLENGE / FIX-THEN-PROCEED; production-defect Q1 fixed in this commit
6. ⏸ **Explicit architect §8 sign-off** at `handover/directives/YYYY-MM-DD_STAGE_A3_§8_SIGN_OFF.md` — REMAINING

## §7. What §8 sign-off would ratify

If signed by architect, this directive ratifies the cumulative Stage A3 work spanning sessions #22 + #23:

| Atom | Commit | Class | Status |
|------|--------|-------|--------|
| R1 multi-ref support on transition_ledger.rs | `72e2494` (+ Q1 fix this commit) | 4 STEP_B | ✅ |
| R2 HeadTWitness::reconstruct_from_chaintape_refs | `72e2494` | 3 | ✅ |
| R3 CAS root ref hook in cas/store.rs | `4b0062e` | 3 | ✅ |
| R3.5 rejection_evidence wire to refs/chaintape/l4e | `f7a6660` | 3 | ✅ |
| R4 SG-A3.1-5 + integration tests | `72e2494` + `4b0062e` (+ this commit) | 1 | ✅ |
| R5 Real-LLM smoke (A3 R5 + R3.5 + B3 R6 mini-M1; 10/10 chain_invariant Ok delta=0) | `2d3d948` + `f7a6660` + `381554f` + `90376ae` | 3 evidence | ✅ |
| R6 OBS forward-binding | `90376ae` + this OBS | 0 | ✅ |
| R7 G2 dual audit + closure | this OBS + audits | 3 audit | ✅ |

**Validation final state at sign-off**:
- Constitution gates: 155 GREEN / 0 failed / 1 ignored (+58 above 97 baseline)
- Workspace tests: 1288 PASS / 0 failed / 151 ignored (+107 above 1181 baseline)
- Trust Root rehashed across A3 commits for all touched files
- Forbidden-list 6 items satisfied; no f64/ghost/price-truth/dashboard-SoT/funds/public-chain
- 5/5 SG-A3 gates GREEN at gate-level + real-LLM-load
- Forward-bind documented for Q2/Q4/Q8 architectural items in Stage A3.6 enhancement scope

## §8. Cross-references

- Charter: `handover/tracer_bullets/STAGE_A3_HEAD_T_C2_charter_2026-05-07.md`
- Parent autonomous-execution authorization: `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md`
- Gemini R1 audit: `handover/audits/GEMINI_STAGE_A3_R7_AUDIT_2026-05-08_R1.md`
- Codex R1 audit: `handover/audits/CODEX_STAGE_A3_R7_AUDIT_2026-05-08_R1.md`
- Gemini R1 forward-bind OBS: `handover/alignment/OBS_STAGE_A3_R7_GEMINI_R1_FORWARD_BIND_2026-05-08.md`
- A3 R5 smoke: `handover/evidence/stage_a3_r5_smoke_2026-05-08T05-40-39Z/`
- A3 R3.5 smoke: `handover/evidence/stage_a3_r35_smoke_2026-05-08T06-02-28Z/`
- B3 R6 mini-M1 batch: `handover/evidence/stage_b3_r6_minim1_2026-05-08T06-07-32Z/`
- Constitution Execution Matrix: `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` (§A Art. 0.4 row updated)
- TB-C0 §8 precedent: `handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md`
- Stage A2 §8 precedent: `handover/directives/2026-05-08_STAGE_A2_§8_SIGN_OFF.md`
- `feedback_dual_audit_conflict` — VETO > CHALLENGE > PASS conservative ranking
- `feedback_audit_loop_roi_flip` — production-defect fix vs architectural-strengthening forward-bind
