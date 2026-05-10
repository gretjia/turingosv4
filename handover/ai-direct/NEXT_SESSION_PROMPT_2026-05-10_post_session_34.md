# Next Session Boot Prompt — 2026-05-10 session #34 close (post strict-constitution sweep)

> Paste the **`USER PROMPT`** block at the bottom into the next Claude session.

---

## State at session #34 close (2026-05-10)

- **HEAD on `origin/main`**: `fc3f359` (7 commits ahead of session #33 close `ed0555f`).
- **Stage C Polymarket: SHIPPED FINAL** (preserved from session #32).
- **Constitution gates**: `267/0/1` (was 259 at session #33 close; +8: tamper-3-of-3 9→10 + new `constitution_l4e_body_integrity` 0→7).
- **Workspace tests** (`--test-threads=1`): `1418/0/151` (was 1403 at session #33 close; +8 from L4.E body integrity gate + 1 from tamper sister test; +7 from prompt-variant tests).
- **Trust Root**: PASS (post `src/bottom_white/ledger/rejection_evidence.rs` rehash `f305f621 → 32679870`).
- **`CONSTITUTION_EXECUTION_MATRIX.md`**: 0 RED + 0 AMBER (current).
- **`TRACE_FLOWCHART_MATRIX.md`**: 0 RED + 0 AMBER + 37 GREEN + 3 N/A (post 8-AMBER → ✅ promotion this session).
- **FC1 / FC2 / FC3**: all GREEN.

## What landed this session (#34)

| Phase | Atom | Class | Outcome |
|-------|------|-------|---------|
| Strict-constitution (1) | L4.E body integrity verification | 2 | SHIPPED (`4775620`) — `assert_51_l4e_git_attestation_matches_jsonl` Layer B; closes session-#33-documented forward gap |
| Operational triage | M0 4/20 ERROR root-cause + memory | 1 | TRIAGED (`5561b66`) — single shared cause TRUST_ROOT_TAMPERED; new `feedback_no_concurrent_dev_during_batch.md`; NOT a TuringOS bug |
| Experimental | Prompt-variant harness `TURINGOS_PROMPT_VARIANT={v0\|v1\|v2\|v3\|v4}` | 2 | SHIPPED (`9b8c847`) — opt-in; v0 = unchanged baseline; 7 new variant tests |
| Experimental | Prompt-variant 5×4=20-run experiment | 2 | RUN COMPLETE (`41e8e61`) — clean negative at N=1 T=0.2 deepseek-chat; every cell byte-identical; recommendation = land v1 schema cleanup, forward-bind agent-economy to TB-12+ runtime |
| Strict-constitution (2) | Comprehensive verification + 8 FC AMBER → ✅ promotions | 1-2 | SHIPPED (`c0c36b4`) — full audit at HEAD `41e8e61`; `COMPREHENSIVE_VERIFICATION_REPORT_2026-05-10_session34.md` |
| Handover | LATEST.md session #34 close block (full state) | 0 | SHIPPED (`fc3f359`) |

## §1 — Critical files for cold-start orientation

Read in this order:

1. **`CLAUDE.md`** — project constitution (§4 strategic decisions; §10 Class-4 authorization; §22 read order).
2. **`handover/ai-direct/LATEST.md`** — top "✅ Session #34 close" block.
3. **`handover/alignment/COMPREHENSIVE_VERIFICATION_REPORT_2026-05-10_session34.md`** — full audit at HEAD `c0c36b4`: constitution + FC + per-stage architect ship gates.
4. **`MEMORY.md`** — "MUST CHECK BEFORE" pre-action gates + new `feedback_no_concurrent_dev_during_batch.md`.
5. **`handover/alignment/PROMPT_VARIANT_EXPERIMENT_RESULTS_2026-05-10.md`** — empirical clean-negative on the prompt-variant question; closes boot-prompt option (a).

## §2 — Pre-action gate (mandatory at next session start)

Per `MEMORY.md`:
- `/constitution-landing-check` — should return PROCEED (matrix unchanged at 0 AMBER).
- `/runner-preflight` — IF starting any `bash run_*.sh` runner script (M1 or M2 batch path).
- Per `feedback_no_concurrent_dev_during_batch` (NEW this session): if any batch is in flight, do NOT modify any Trust-Root-pinned source file until the batch completes.

## §3 — Forward queue (post session #34, strict-constitution framing)

**The user explicitly rejected cost/ease framing this session** ("我不想听到哪种更简单，哪种更 cheap 这样的言论...我需要的是宪法以及宪法中三个 flow chart 的完整落地，还有架构师设计的 ship gate 的完整的验证通过"). Forward queue is bound to architect spec / constitutional clauses, NOT to convenience.

| Item | Constitutional binding | Status |
|------|------------------------|--------|
| **(A) Run M2 (100-problem benchmark) under SG-B3.1-6 + EvidencePackagingPolicy** | Architect §Stage B spec (`handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md`); `feedback_minif2f_scaling_policy` (M0→M1→M2→M3 ladder); `feedback_benchmark_manifest_required`; `feedback_evidence_packaging_policy_required`; CLAUDE.md §B.9.1 binding conditions | **OPEN — only architect-designed ship-gate set still requiring real-evidence binding at HEAD `fc3f359`.** Substrate ready (TB-18B atoms R1+R2+R3+R4+R5 LANDED 2026-05-08; BenchmarkManifest + AggregateReport with Wilson 95% CI + diversity helpers + PCP corpus phase-2 + Art. 0.2 10-commit). Precondition "Only after B1/B2 green" satisfied. M1 (10-30p × n3) is the optional precursor. |
| (B) Stage D real-world readiness | Architect §B.9.1 explicit forbid + CLAUDE.md §20 freeze conditions | DEFERRED behind explicit architect ship gate (no spec exists yet). |
| (C) PromptCapsule evaluator wire-up | CLAUDE.md §4.3 G-016 / G-019 / G-021 / G-028 prompt persistence Class-3 PromptCapsule + L4 anchor by default | OPEN forward Class-3 work. |
| (D) CAS Merkle redesign | Stage A3.6 enhancement TB (Codex Q1+Q2 from A3 R7); CR-A3-HEAD-T-C2.6 deferred | DEFERRED — does not affect current operations. |
| (E) Economy-aware agent prompt landing (boot-prompt-from-#33 option a) | Originally framed as Option A/B/C/staged; **session #34 prompt-variant experiment produced empirical clean-negative evidence** showing no prompt variant moves the metric at N=1 T=0.2 deepseek-chat. Constitutional landing of "agent perceives the economy" is forward-bound to TB-12+ runtime work (NodeMarket / Polymarket-agent-bridge). | **EMPIRICALLY CLOSED at this configuration** — agent-economy landing is a runtime-tools question, NOT a prompt-text question. |
| (F) Land v1 prompt schema cleanup (drop unused `invest`/`search`/`post` from default schema) | Empirical safety per session #34 experiment (v1 byte-identical to v0); ~75 input tokens saved per LLM call; **NOT a ship-gate satisfier** | OPEN — housekeeping. Touches `current_prompt_variant()` default; Class-2; not constitutionally load-bearing. |
| (G) Mid-batch Trust Root re-check (B-followup from session #33) | `feedback_no_concurrent_dev_during_batch` mechanism enhancement; existing fail-closed panic IS the detection mechanism; this only improves diagnostic | OPEN — no constitutional gap. |

**Recommended next-session path**: confirm with user whether to attack (A) M2 — the only architect-designed ship-gate set still requiring real-evidence binding — and if so, whether to use M1 (~10-30p × n3) as the precursor.

## §4 — Mechanism additions this session (forward defense)

- **NEW `src/runtime/audit_assertions.rs::assert_51_l4e_git_attestation_matches_jsonl`** (Layer B; FC1-N34 + FC1-N35 + FC2-INV1) — closes session-#33-documented L4.E silent-tamper class.
- **NEW `src/bottom_white/ledger/rejection_evidence.rs::parse_and_verify_jsonl_record_bytes`** — pure additive read-only audit-side helper; Trust Root rehashed `f305f621 → 32679870`.
- **NEW `src/runtime/audit_tamper.rs::{L4E_REFS, flip_largest_reachable_l4e_blob}`** — sibling tamper primitive for the L4.E side.
- **NEW `tests/constitution_l4e_body_integrity.rs`** (7 tests; uses real M0 P01 evidence per `feedback_real_problems_not_designed`).
- **NEW `tests/constitution_audit_tamper_3_of_3.rs::l4e_refs_is_strict_subset_of_chain_refs_l4e_only`** (1 test; tamper-3-of-3 9→10).
- **NEW `src/sdk/prompt.rs` variant harness** — `TURINGOS_PROMPT_VARIANT` env var; 5 variants (v0/v1/v2/v3/v4); 7 new tests; default v0 = bit-identical to pre-session-#34.
- **NEW memory `feedback_no_concurrent_dev_during_batch.md`** — codifies the M0 4/20 ERROR lesson; cross-references `feedback_pre_runner_checklist`.
- **UPDATED `project_economy_prompt_landing_gap.md`** — empirical decision recorded; original "in-flight" framing preserved for audit trail.
- **UPDATED `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`** — NEW §G FC1 row "L4.E body integrity — git-side attestation matches JSONL" (🟢 GREEN).
- **UPDATED `handover/alignment/TRACE_FLOWCHART_MATRIX.md`** — 8 stale 🟡 → ✅ promotions with binding citations (FC1-N1/N5/N7/N9/N13 + FC3-N31/N33/N39).
- **NEW `handover/alignment/COMPREHENSIVE_VERIFICATION_REPORT_2026-05-10_session34.md`** — per-stage architect ship-gate audit; 9/10 sets GREEN at HEAD; SG-B3.1-6 (M2) is the single open set.
- **NEW `handover/alignment/PROMPT_VARIANT_EXPERIMENT_PLAN_2026-05-10.md`** + **`PROMPT_VARIANT_EXPERIMENT_RESULTS_2026-05-10.md`** — clean-negative empirical evidence for the prompt-landing decision.
- **NEW `handover/alignment/M0_4_OF_20_ERROR_TRIAGE_2026-05-10.md`** — operational, not a TuringOS bug.

## §5 — Validation baseline at session #34 close

| Check | Value |
|-------|-------|
| HEAD | `fc3f359` (origin/main; pushed) |
| Constitution gates | 267/0/1 |
| Workspace tests | 1418/0/151 |
| Trust Root | PASS |
| `CONSTITUTION_EXECUTION_MATRIX.md` | 0 current RED + 0 current AMBER |
| `TRACE_FLOWCHART_MATRIX.md` | 0 RED + 0 AMBER + 37 GREEN + 3 N/A |
| FC1 / FC2 / FC3 | all GREEN |
| Architect ship-gate sets verified at HEAD | 9 / 10 (SG-B3.1-6 / M2 is the single open set) |

## §6 — Memory verification at next-session start

Verify these are present in `MEMORY.md` (session-#34 additions):
- **`feedback_no_concurrent_dev_during_batch.md`** index entry under DEVELOPMENT DISCIPLINE.
- **`project_economy_prompt_landing_gap.md`** memory file with the EMPIRICAL DECISION update header from session #34.

If missing, restore from session #34 commits.

---

## USER PROMPT (paste this into next Claude session)

```
Session #34 closed 2026-05-10 at HEAD `fc3f359` on origin/main (7 commits
on top of session #33 boot `ed0555f`).

What landed this session:
- (c) L4.E body integrity verification — assert_51 Layer B + audit-side
  helper `parse_and_verify_jsonl_record_bytes` + L4E_REFS + sibling
  tamper primitive + 7-test gate. Closes session-#33-documented forward
  gap (silent L4.E git-side tamper). Trust Root rehashed
  rejection_evidence.rs `f305f621 → 32679870`.
- (b) M0 4/20 ERROR triage — single shared cause TRUST_ROOT_TAMPERED on
  src/runtime/mod.rs (file modified mid-batch session #33). Operational,
  not a TuringOS bug. New memory feedback_no_concurrent_dev_during_batch.md.
- Prompt-variant experiment harness (TURINGOS_PROMPT_VARIANT={v0..v4}
  opt-in env var; v0 = unchanged baseline) + 5×4=20-run experiment →
  CLEAN NEGATIVE at N=1 T=0.2 deepseek-chat. Memory updated:
  recommendation = land v1 (schema cleanup); forward-bind "agent
  perceives economy" to TB-12+ runtime tools.
- Comprehensive verification at HEAD `41e8e61`: TRACE_FLOWCHART_MATRIX.md
  8 stale 🟡 → ✅ promotions (FC1-N1/N5/N7/N9/N13 + FC3-N31/N33/N39 with
  Wave 3 50p / FC3 evidence binding / M0 P01-P16 citations); per-stage
  architect ship-gate audit (9/10 sets GREEN; SG-B3.1-6 / M2 100p
  benchmark is the single open set).

Constitution gates 267/0/1; workspace 1418/0/151; Trust Root PASS;
constitution matrix 0 current RED + 0 current AMBER; FC matrix 0 RED +
0 AMBER + 37 GREEN + 3 N/A.

Two strict-constitution holds (verbatim user direction this session):
1. "我现在在引擎的开发阶段，我不要凑合，我需要的是宪法约定的内容全部
   真实落地且可被验证" — drove L4.E body integrity landing.
2. "我不想听到哪种更简单，哪种更 cheap 这样的言论...我需要的是宪法以及
   宪法中三个 flow chart 的完整落地，还有架构师设计的 ship gate 的完整
   的验证通过" — drove the comprehensive verification + FC promotions.
   FORWARD WORK MUST BE FRAMED IN CONSTITUTIONAL / FC / SG TERMS, NOT
   COST / EASE.

Read first:
1. handover/ai-direct/NEXT_SESSION_PROMPT_2026-05-10_post_session_34.md
   (this prompt's source; full context + forward queue + spec citations)
2. handover/ai-direct/LATEST.md "✅ Session #34 close" block
3. handover/alignment/COMPREHENSIVE_VERIFICATION_REPORT_2026-05-10_session34.md
   (per-stage architect ship-gate audit; SG-B3.1-6 is the single open set)
4. MEMORY.md (NEW feedback_no_concurrent_dev_during_batch.md +
   updated project_economy_prompt_landing_gap.md)

Tell me what you want to do (constitutional framing only):
(A) Run M2 (100-problem benchmark) under SG-B3.1-6 +
    EvidencePackagingPolicy. RECOMMENDED — only architect-designed
    ship-gate set still requiring real-evidence binding. Substrate
    ready; precondition "B1+B2 green" satisfied at HEAD. M1 (10-30p × n3)
    is the optional architect-described precursor per
    feedback_minif2f_scaling_policy.
(B) Stage D real-world readiness — DEFERRED behind explicit architect
    ship gate (no spec exists).
(C) PromptCapsule evaluator wire-up — CLAUDE.md §4.3 G-016/019/021/028
    Class-3 forward; not blocking.
(D) CAS Merkle redesign — Stage A3.6 enhancement TB; not blocking.
(F) Land v1 prompt schema cleanup — empirical-safety housekeeping; NOT
    a ship-gate satisfier; touches current_prompt_variant() default.
(G) Mid-batch Trust Root re-check — diagnostic improvement to the
    fail-closed panic mechanism; no constitutional gap.
(H) Something else — describe it (in constitutional / FC / SG terms).
```

---

**End of session #34 close boot prompt.**
