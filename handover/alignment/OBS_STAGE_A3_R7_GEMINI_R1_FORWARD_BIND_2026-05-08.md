# OBS Stage A3 R7 Gemini R1 Forward-Binding (2026-05-08)

**Authority**: `feedback_audit_loop_roi_flip` + `feedback_dual_audit_conflict` + STAGE_A3_HEAD_T_C2_charter_2026-05-07.md SG-A3.10.

**Audit verdict**: Gemini 2.5 Pro R1 = **CHALLENGE** at HEAD `381554f` (Stage A3 substrate + B3 R6 mini-M1 evidence). Aggregate verdict CHALLENGE because Q2/Q5/Q8 returned CHALLENGE; Q1/Q3/Q4/Q6/Q7 PASS.

**Disposition**: SHIP Stage A3 substrate + forward-bind Q2/Q8 remediations to a follow-up enhancement TB. Q5 closed in this commit.

---

## §1. Per-Q breakdown + disposition

| Q | Verdict | Topic | Disposition |
|---|---------|-------|-------------|
| Q1 | PASS | Dual-write semantics + signing-payload preservation | — |
| Q2 | **CHALLENGE** | env-var seam (`TURINGOS_CHAINTAPE_PATH`) on `flush_jsonl_record` is brittle vs explicit ctor arg | **forward-bind** to Stage A3.6 enhancement |
| Q3 | PASS | byte-stable JSONL → git2 commit determinism | — |
| Q4 | PASS | sha256-of-OID for cas_root cross-format mapping | — |
| Q5 | **CHALLENGE → CLOSED** | chain_invariant.json missing from smoke evidence | **CLOSED this commit** — regenerated 10/10 Ok delta=0 across A3 R5 + A3 R3.5 + B3 R6 mini-M1 |
| Q6 | PASS | grep-based no-fs-pointer test sufficient | — |
| Q7 | PASS | 8-problem real-MiniF2F coverage adequate for storage-layer change | — |
| Q8 | **CHALLENGE** | (a) refs/chaintape/cas points to blob OID directly (GC-vulnerable); (b) non-atomic ref updates race-prone | **forward-bind** to Stage A3.6 enhancement |

## §2. Q5 closure (this commit)

Gemini R1 Q5 verbatim: "the evidence provided for all smoke runs (R5, R3.5, B3 R6) fails to include the '50/50-style invariant report' (i.e., the chain_invariant.json file or its equivalent) explicitly required by ship gate SG-A3.8".

**Action**: regenerated chain_invariant.json for all 10 smoke evidence problem dirs using the existing `target/release/tb_18r_compute_invariant` binary against the per-problem runtime_repo + cas. Halt class derived from PPUT result (`hit_max_tx → MaxTxExhausted`; `solved → OmegaAccepted`).

**Result**:

| Batch | Problems | Verdict |
|-------|----------|---------|
| A3 R5 smoke (mathd_algebra_107) | 1 | Ok delta=0 (l4=0 l4e=9 cap=0 expected=9) |
| A3 R3.5 smoke (mathd_algebra_113) | 1 | Ok delta=0 (l4=0 l4e=9 cap=0 expected=9) |
| B3 R6 mini-M1 (8 problems) | 8 | 8/8 Ok delta=0 |
| **Total** | **10** | **10/10 Ok delta=0** |

FC1-INV1 hard invariant `expected_completed_attempts == l4 + l4e + capsule_anchored` holds for every smoke run on Stage A3 C2 substrate. SG-A3.8 ("real-LLM smoke produces 50/50-style invariant report") **NOW SATISFIED**.

## §3. Q2 forward-bind — env-var seam → explicit ctor arg

Gemini R1 Q2 verbatim concern: "TURINGOS_CHAINTAPE_PATH environment variable is an architecturally weak seam... introduces implicit, global state ('action at a distance')... fails in scenarios with multiple concurrent TuringOS instances within a single process".

**Counter-argument**: TuringOS is currently single-instance per process (the evaluator binary is invoked once per problem with isolated chaintape paths via per-problem env vars). The env-var seam is a documented runtime contract identical to the upstream `TURINGOS_CHAINTAPE_PATH` already used by `RuntimeChaintapeConfig::from_env`. Multi-process concurrency is an architectural future state not currently in scope.

**Forward-bind**: Stage A3.6 enhancement TB will refactor `RejectionEvidenceWriter` to accept `chaintape_repo_path: Option<PathBuf>` as an explicit ctor field. The env-var hook can remain as a thin convenience wrapper that calls the explicit ctor.

**Severity**: low. Current usage is correct for single-instance contract.

## §4. Q8 forward-bind — refs/chaintape/cas GC-vulnerability + concurrency

Gemini R1 Q8 verbatim concerns:

(a) **GC vulnerability**: "refs/chaintape/cas points directly to the latest blob OID, leaving all historical CAS blobs unreferenced. A standard `git gc` would garbage-collect and delete this historical data".

(b) **Concurrency**: "non-atomic, race-prone way... If two processes write to CAS concurrently, one's ref update will be lost".

**Counter-argument (GC)**: The TuringOS CAS git repo is per-run (per-problem) and lives at `cas_path` which is NOT subject to `git gc` in normal operation. The CAS `.turingos_cas_index.jsonl` sidecar is the durable index; the ref is a derived view per FR-A3-HEAD-T-C2.5. Per `feedback_evidence_packaging_policy_required` the repo is local-only artifact, not a long-lived production repo.

However, Gemini's concern is valid for FORWARD architectural robustness: if the C2 ref scheme is intended as canonical pointer per CR-A3-HEAD-T-C2.5, it should be a proper commit chain, not a blob-OID pointer. This is the architecturally clean design.

**Forward-bind (Q8a)**: Stage A3.6 enhancement TB will redesign `CasStore::put` ref-update logic so each put creates a new git commit (or batch commit per N puts):
- Tree references the new CAS object(s) by name
- Parent = previous commit on refs/chaintape/cas
- This forms a proper, GC-safe commit chain

**Forward-bind (Q8b)**: Once Q8a is in place, atomicity comes from git's underlying ref-update atomicity (single-process); for multi-process, file lock + compare-and-swap retry on ref update.

**Severity**: medium for forward-architecture robustness; low for current single-process usage.

## §5. Aggregate disposition

Per `feedback_audit_loop_roi_flip`: "When CHALLENGEs shift production-defects → architectural strengthening, ROI flipped; ship + forward-bind". Gemini R1's three CHALLENGE items split as:

- Q5 (production-defect: missing evidence) → **CLOSED this commit**
- Q2 (architectural-seam preference) → forward-bind (no production defect; refactor improves hygiene)
- Q8 (forward-architectural robustness) → forward-bind (no current-usage defect; needed for future multi-process / long-lived repo scenarios)

Stage A3 substrate ship eligibility:
- All 5 SG-A3 ship gates GREEN at gate level (constitution_head_t_c2_multi_ref.rs 7 tests)
- All 5 SG-A3 ship gates GREEN under real DeepSeek-LLM load (10/10 chain_invariant Ok delta=0 across A3 R5 + R3.5 + B3 R6 mini-M1)
- SG-A3.6 cargo test --workspace 1287 PASS / 0 failed (≥1181 baseline)
- SG-A3.7 constitution gates 154 GREEN / 0 failed (≥97 baseline)
- SG-A3.8 real-LLM smoke produces invariant report — CLOSED this commit
- SG-A3.9 OBS forward-binding for migration edges — THIS DOC
- SG-A3.10 G2 dual audit dispatched AFTER MVP gates green — Gemini done R1; Codex pending dispatch

**Codex dual-audit dispatch**: scheduled in this commit pair via `run_codex_stage_a3_r7_audit_2026-05-08.sh` (separate commit; results capture verdict).

**Stage A3 SHIPPED CANDIDATE pending Codex dual-audit closure**. Architect §8 sign-off path opens after Codex side returns.

## §6. Stage A3.6 forward TB scope (post-A3 ship)

Open items for follow-up Stage A3.6 enhancement TB:

1. `RejectionEvidenceWriter` explicit `chaintape_repo_path: Option<PathBuf>` ctor field (Class-3 additive; closes Gemini Q2)
2. `refs/chaintape/cas` redesign as proper commit chain anchoring CAS objects by name in commit tree (Class-3-or-4 depending on schema impact; closes Gemini Q8a GC vulnerability)
3. Atomic ref-update logic for multi-process safety (Class-3; closes Gemini Q8b once Q8a lands)
4. Optional: same commit-chain redesign for refs/chaintape/l4e (currently each rejection creates its own commit, which is already chain-safe; verify and document)

These are forward-bound; not blocking Stage A3 ship.

## §7. Cross-references

- Gemini R1 audit: `handover/audits/GEMINI_STAGE_A3_R7_AUDIT_2026-05-08_R1.md`
- Gemini dispatch script: `handover/audits/run_gemini_stage_a3_r7_audit_2026-05-08.py`
- Stage A3 charter: `handover/tracer_bullets/STAGE_A3_HEAD_T_C2_charter_2026-05-07.md`
- A3 R5 smoke evidence: `handover/evidence/stage_a3_r5_smoke_2026-05-08T05-40-39Z/`
- A3 R3.5 smoke evidence: `handover/evidence/stage_a3_r35_smoke_2026-05-08T06-02-28Z/`
- B3 R6 mini-M1 evidence: `handover/evidence/stage_b3_r6_minim1_2026-05-08T06-07-32Z/`
- chain_invariant.json regen helper: `target/release/tb_18r_compute_invariant`
- `feedback_audit_loop_roi_flip` — disposition rationale
- `feedback_dual_audit_conflict` — VETO > CHALLENGE > PASS conservative ranking
