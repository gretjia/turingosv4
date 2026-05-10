# Next Session Boot Prompt — 2026-05-10 session #33 close (post forward-defense + M0 batch)

> Paste the **`USER PROMPT`** block at the bottom into the next Claude session.

---

## State at session #33 close (2026-05-10)

- **HEAD on `origin/main`**: `ed0555f` (3 commits ahead of session #32 boot `bf45a2b`).
- **Stage C Polymarket: SHIPPED FINAL** (preserved from session #32; full P-M2..P-M9 + Phase F.9 overall §8).
- **Constitution gates**: `259/0/1` (was 241 at session #32 close; +18 across two new gates this session).
- **Workspace tests** (`--test-threads=1`): `1403/0/151` (was ~1390 at session #32; +9 from tamper-3-of-3 gate, with 1 ignored→passed via P-M7 doctest hygiene fix).
- **Trust Root**: PASS (post `src/runtime/mod.rs` rehash for `pub mod audit_tamper`).
- **FC1 / FC2 / FC3**: all GREEN.

## What landed this session (#33)

| Phase | Atom | Class | Outcome |
|-------|------|-------|---------|
| Hygiene | Stage C P-M7 doctest mark `text` + TB-13 `RationalPrice` token exemption | 1-2 | SHIPPED (`8de75aa`) — closed 2 pre-existing P-M7 ship gaps; workspace baseline restored |
| (d) | `constitution_admission_no_fail_open_default` source-grep gate | 1-2 | SHIPPED (`5e6d7c7`) — Stage C R2 Q10 forward defense; +9 tests catch fail-open `unwrap_or(...Open)` co-occurrence in sequencer admission |
| (a) | M0 mini smoke (full 20-problem batch, real DeepSeek + real Lean) | 2 | RUN COMPLETE — 16 PROCEED / 0 BLOCK / 4 ERROR / 8 solved / 7 exhausted / 5 error_or_no_pput; ~30 min wall-clock; ~$1-3 spend |
| Tamper | `audit_tape_tamper` 3/3 multi-ref drift fix + `constitution_audit_tamper_3_of_3` gate | 1-2 | SHIPPED (`ed0555f`) — closed M0-surfaced 1/3 universal regression; library refactor `src/runtime/audit_tamper.rs` + 9 forward-defense tests + OBS_R022 doc for backlink relocation |

## §1 — Critical files for cold-start orientation

Read in this order:

1. **`CLAUDE.md`** — project constitution (§4 strategic decisions; §10 Class-4 authorization; §22 read order).
2. **`handover/ai-direct/LATEST.md`** — top "✅ Session #33 close" block.
3. **`MEMORY.md`** — "MUST CHECK BEFORE" pre-action gates + "IN-FLIGHT CONSTITUTIONAL LANDING GAPS" section (project_economy_prompt_landing_gap).
4. **`handover/evidence/m0_minif2f_harness_audit_2026-05-10_post_stage_c/M0_BATCH_SUMMARY.json`** — fresh M0 batch summary.

## §2 — Pre-action gate (mandatory at next session start)

Per `MEMORY.md`:
- `/constitution-landing-check` — should return PROCEED (matrix unchanged at 0 AMBER).
- `/runner-preflight` — IF starting any `bash run_*.sh` runner script (M1 batch path).

## §3 — Forward queue (post session #33)

**STRICT-CONSTITUTION HOLD on Forward item 1** (per user 2026-05-10 verbatim "我要宪法的完整落地，我不要凑活，但是可以等M0实际结果来决定v4的机制如何修正"). The M0 evidence required to decide is now available — `handover/evidence/m0_minif2f_harness_audit_2026-05-10_post_stage_c/`. Boot prompt RECOMMENDS confirming user direction before starting Item 1 work.

| Item | Class | Status / cost-risk |
|------|-------|----------|
| **(1) Economy-aware agent prompt — landing decision** | TBD | DEFERRED (gated on user picking Option A / B / C). Reference: `memory/project_economy_prompt_landing_gap.md` + `~/projects/turingosv3/experiments/zeta_sum_proof/prompt/skill.txt`. v4 prompt currently advertises `invest` tool that is runtime-disabled (TB-9 collapse). N=1 chain tape shows ZERO agent-initiated economic action — agents don't perceive the economy. Decision is constitutional, not cosmetic. |
| **(2) M0 4/20 ERROR root-cause** | 1-2 | OPEN — 4 problems errored on `audit_tape` post-evaluator. Inspect `handover/evidence/m0_*/P*_*/audit_tape.stderr` + `evaluator.stderr` to triage. Could be per-problem evaluator issue (Lean stderr edge cases), audit-load issue, or chain-tape construction issue. |
| **(3) L4.E body integrity verification** | 2-3 | OPEN — `audit_assertions::run_all_assertions` doesn't deep-verify L4.E rejection_record blob bodies. Tampering an L4.E body is silent at audit-time. Forward defense; closes the documented gap from session #33's `tests/constitution_audit_tamper_3_of_3.rs::l4_refs_is_strict_subset_of_chain_refs_excluding_l4e` comment. |
| **(4) M1 mini batch (8p × n3)** | 2-3 | ELIGIBLE per session #32 user grant ("授权调用 LMM API + 自主开展真题测试"); RECOMMENDED to defer until Item 1 resolved so M1 runs with correct economy prompt. ~$5-20 / ~1-3 hours. |
| Stage D real-world readiness | architect | DEFERRED behind explicit architect ship gate. |
| C.5 PromptCapsule evaluator wire-up | 3 | Forward per CLAUDE.md §4.3; not blocking. |
| B.4 CAS Merkle redesign | 3-4 | Stage A3.6 enhancement TB; not blocking. |

**Recommended next-session path**: confirm with user (a) preference between Item 1 Option A/B/C/staged, and (b) whether to address Item 2 (M0 ERROR triage; cheap, ~30 min) before Item 4 (M1 batch; multi-$/multi-hour).

## §4 — Mechanism additions this session (forward defense)

- **NEW `tests/constitution_admission_no_fail_open_default.rs`** — 9 tests (1 main + 8 self-checks) catching fail-open `unwrap_or(<state>::Open)` patterns in `src/state/sequencer.rs`. Stage C R2 Q10 class mechanically prevented from regressing.
- **NEW `src/runtime/audit_tamper.rs`** — library API for tamper primitives with reachability-aware L4 blob selection + dual-ref truncation. Replaces stale TB-16-era in-binary primitives.
- **NEW `tests/constitution_audit_tamper_3_of_3.rs`** — 9 tests proving tamper coverage at architect §B.9.3 mandated 3/3. Catches future drift (e.g. new Stage D ref name without updating `CHAIN_REFS`) at gate-time.
- **NEW `memory/project_economy_prompt_landing_gap.md`** — codifies the in-flight constitutional gap + user's strict-landing preference. Persists across sessions.
- **NEW `handover/alignment/OBS_R022_AUDIT_TAMPER_LIBRARY_RELOCATION_2026-05-10.md`** — justifies the TRACE_MATRIX backlink relocation (binary → library).
- **MEMORY.md** index updated with "IN-FLIGHT CONSTITUTIONAL LANDING GAPS" section.

## §5 — Validation baseline at session #33 close

| Check | Value |
|-------|-------|
| HEAD | `ed0555f` (origin/main after push) |
| Constitution gates | 259/0/1 |
| Workspace tests | 1403/0/151 |
| Trust Root | PASS |
| Phase E binding gates | E.1 LANDED for all P-M atoms; E.2 P-M6 LANDED; E.3 strict-equality enforced |
| FC1 / FC2 / FC3 | all GREEN |
| `audit_tape_tamper` 3/3 (post-fix) | empirically validated on M0 P01 + P05 |

## §6 — Memory verification at next-session start

Verify these are present in `MEMORY.md` (added 2026-05-10 session #33):
- **"IN-FLIGHT CONSTITUTIONAL LANDING GAPS"** section header above "USER & PHILOSOPHY".
- **`project_economy_prompt_landing_gap.md`** memory file with Option A/B/C/staged forward path.

If missing, restore from session #33 commits.

---

## USER PROMPT (paste this into next Claude session)

```
Session #33 closed 2026-05-10 at HEAD `ed0555f` on origin/main (3 commits
on top of session #32 boot `bf45a2b`).

What landed this session:
- (d) constitution_admission_no_fail_open_default gate (Stage C R2 Q10
  forward defense; 9 tests; gates 241→250).
- Stage C P-M7 ship hygiene (doctest + TB-13 RationalPrice exemption;
  workspace clean post-ship).
- (a) M0 mini smoke (full 20-problem batch, real DeepSeek + real Lean,
  ~30 min, ~$1-3): 16 PROCEED / 0 BLOCK / 4 ERROR / 8 solved /
  7 exhausted; chain tape evidence at
  handover/evidence/m0_minif2f_harness_audit_2026-05-10_post_stage_c/.
- audit_tape_tamper 3/3 multi-ref drift fix + new
  constitution_audit_tamper_3_of_3 gate (closed M0-surfaced 1/3
  universal regression; gates 250→259; workspace 1394→1403; Trust Root
  PASS).

Constitution gates 259/0/1; workspace 1403/0/151; Trust Root PASS.

In-flight strict-constitution holds (per "我不要凑活" + auto-mode):
1. Economy-aware agent prompt landing — Option A (v3-style explicit LAW
   1/2/3) / B (minimal awareness) / C (TB-12+ synchronized) DEFERRED
   until user picks path. M0 evidence now available for the decision.
2. L4.E body integrity verification — audit doesn't deep-verify L4.E
   rejection_record bodies; documented forward gap.
3. M0 4/20 ERROR root-cause not yet investigated.

Read first:
1. handover/ai-direct/NEXT_SESSION_PROMPT_2026-05-10_post_session_33.md
   (this prompt's source; full context + forward queue + cost guidance)
2. handover/ai-direct/LATEST.md "✅ Session #33 close" block
3. MEMORY.md (IN-FLIGHT CONSTITUTIONAL LANDING GAPS section +
   project_economy_prompt_landing_gap.md)
4. handover/evidence/m0_minif2f_harness_audit_2026-05-10_post_stage_c/
   M0_BATCH_SUMMARY.json (fresh M0 evidence)

Tell me what you want to do:
(a) Economy prompt landing — pick Option A / B / C / staged for
    project_economy_prompt_landing_gap.md. Reference v3
    ~/projects/turingosv3/experiments/zeta_sum_proof/prompt/skill.txt.
(b) M0 4/20 ERROR root-cause triage — Class 1-2; ~30 min; inspect
    handover/evidence/m0_*/P*_*/audit_tape.stderr + evaluator.stderr.
    No money spend.
(c) L4.E body integrity verification — Class 2-3 forward defense;
    extends audit_assertions to deep-verify L4.E rejection_record
    bodies. ~1-2 hours; no money spend.
(d) M1 mini batch (8p × n3) — Class 2-3; ~$5-20; ~1-3 hours; only
    after (a) so prompt is correct. RECOMMENDED last.
(e) Something else — describe it.
```

---

**End of session #33 close boot prompt.**
