# Next Session Boot Prompt — 2026-05-09 session #32 close (post Stage C SHIPPED FINAL)

> Paste the **`USER PROMPT`** block at the bottom into the next Claude session.

---

## State at session #32 close (2026-05-09)

- **HEAD on `origin/main`**: `592db06` (Stage C overall §8 sign-off + LATEST.md + R3 audit transcripts).
- **Stage C Polymarket: SHIPPED FINAL** — full P-M2..P-M9 sequence + Phase F.9 overall §8 cap. All 8 P-M atoms shipped; Class-4 atoms (P-M2 + P-M4 + P-M6) each got per-atom §8; Stage C overall §8 ratified per multi-clause user authorization + R3 dual audit PASS/PASS.
- **Constitution gates**: `241/0/1` (was 175 pre-Stage-C; +66).
- **Workspace tests**: ~1390/0/151 (was 1308 pre-Stage-C; +80+).
- **Trust Root**: PASS.
- **All 4 session #27 batch §8 VETO defects + 2 Q10 issues**: CLOSED.
- **New feedback memory**: `feedback_admission_fail_closed_default.md` (Codex Stage C R2 Q10 lesson).

## What landed this session (#32)

| Phase | Atom | Class | Outcome |
|-------|------|-------|---------|
| F.4 | P-M5 CpmmSwap | 3 | SHIPPED |
| F.5 | P-M6 BuyWithCoinRouter | 4 STEP_B | SHIPPED FINAL (per-atom §8 R1 PASS first-try) |
| F.6 | P-M7 PriceIndex from CPMM | 1-2 | SHIPPED |
| F.7 | P-M8 Audit views | 1-2 | SHIPPED |
| F.8 | P-M9 Controlled market smoke | 2-3 | SHIPPED |
| F.9 | Stage C overall §8 | 4 ship-cap | SHIPPED FINAL (R3 PASS post-R2 fail-closed remediation) |

## §1 — Critical files for cold-start orientation

Read in this order:

1. **`CLAUDE.md`** — project constitution (§4 strategic decisions; §10 Class-4 authorization; §22 read order).
2. **`handover/ai-direct/LATEST.md`** — top "✅ Stage C Polymarket SHIPPED FINAL" block.
3. **`handover/directives/2026-05-09_STAGE_C_POLYMARKET_OVERALL_§8_SIGN_OFF.md`** — verbatim sign-off.
4. **`handover/audits/CODEX_STAGE_C_OVERALL_AUDIT_2026-05-09_R3.md`** + companion R1/R2 — 3-round audit lineage (Q10 closure).
5. **`MEMORY.md`** — "MUST CHECK BEFORE" pre-action gates + Stage C SHIPPED FINAL row at top.

## §2 — Pre-action gate (mandatory at next session start)

Per `MEMORY.md`:
- `/constitution-landing-check` — should return PROCEED (matrix unchanged; 0 AMBER).
- `/runner-preflight` — IF starting any `bash run_*.sh` runner script (real-problem testing path).

## §3 — Forward queue (post Stage C SHIPPED FINAL)

| Item | Class | Status | Cost / risk |
|------|-------|--------|-------------|
| **Real-problem testing M0 mini smoke** | 2 | ELIGIBLE NOW per user clause 2 grant; deferred from session #32 due to multi-$ irreversible spend + multi-hour wall-clock | ~$1-5 LLM API; ~30-60 min |
| **Real-problem testing M1 mini batch** | 2-3 | ELIGIBLE post-M0-smoke clean | ~$5-20; ~1-3 hours |
| **Real-problem testing M2 batch (50p × n3)** | 3 | ELIGIBLE post-M1 clean | ~$50-200; ~6-12 hours |
| LP unwind / PoolStatus::Resolved/Closed lifecycle | 3-4 | DEFERRED to Stage D readiness gate | architect explicit auth required |
| Stage D real-world readiness | architect | DEFERRED behind explicit ship gate | K.1-6 readiness reports required |
| C.5 PromptCapsule evaluator wire-up | 3 | Forward per CLAUDE.md §4.3 | not Stage-C scope |
| B.4 CAS Merkle redesign | 3-4 | Stage A3.6 enhancement TB | not Stage-C scope |

**Recommended next-session path**: confirm with user whether to proceed with real-problem M0/M1/M2 batch (cost-monitored) OR pivot to a different forward item. The user's session #32 authorization scope `直到polymarket全部落地并自主开展真题测试` covers real-problem testing in principle, but multi-$/multi-hour batches benefit from per-batch user oversight (cost-prudence carve-out from CLAUDE.md "Executing actions with care").

## §4 — Mechanism additions this session (forward defense)

- New constitution gate idea (NOT yet implemented — recommended for next session): `constitution_admission_no_fail_open_default` — source-grep gate forbidding `unwrap_or(*State::Open)` in sequencer admission arms. Would have caught R2 Q10 fail-open default at gate-time before Codex's R2 audit. Implementation: ~30 lines; mirror P-M5/P-M6 source-grep gate pattern.
- New feedback memory: `feedback_admission_fail_closed_default.md` — codifies the R2 Q10 lesson; prevents recurrence on future admission gates.

## §5 — Validation baseline at session #32 close

| Check | Value |
|-------|-------|
| HEAD | `592db06` |
| Constitution gates | 241/0/1 |
| Workspace tests | ~1390/0/151 |
| Trust Root | PASS |
| Phase E binding gates | E.1 LANDED for all P-M atoms; E.2 P-M6 LANDED; E.3 strict-equality enforced |
| FC1 / FC2 / FC3 | all GREEN |

## §6 — Memory verification at next-session start

Verify these are present in `MEMORY.md` (added 2026-05-09 session #32):
- **Stage C Polymarket SHIPPED FINAL row** (canonical current state).
- **P-M5 SHIPPED row** (Phase F.4; Class-3).
- **P-M6 SHIPPED FINAL row** (Phase F.5; Class-4 STEP_B; per-atom §8 R1 PASS first-try).
- **`feedback_admission_fail_closed_default.md`** memory file.
- **MEMORY.md index** entry for the new feedback.

If any are stale, restore from session #32 work.

---

## USER PROMPT (paste this into next Claude session)

```
Stage C Polymarket SHIPPED FINAL in session #32 (2026-05-09) at HEAD
`592db06` (full P-M2..P-M9 + Phase F.9 overall §8 cap).

Architect §8 multi-clause user grant + R3 dual audit PASS:
- R1: Codex 9/10 + Q10 CHALLENGE / Gemini 10/10 PASS → CHALLENGE (event-state gate gap).
- R2: Codex 9/10 + Q10 CHALLENGE (fail-open default) / Gemini 10/10 PASS → CHALLENGE.
- R3: Codex 10/10 PASS / Gemini PASS → PASS first-try post fail-closed `ok_or(EventNotOpen)?` fix.

All 4 session #27 batch §8 VETO defects + 2 Q10 issues CLOSED.
Constitution gates 175 → 241 (+66); workspace 1308 → ~1390 (+80+);
Trust Root PASS.

Read first:
1. handover/ai-direct/NEXT_SESSION_PROMPT_2026-05-09_post_stage_c_ship.md
   (this prompt's source; full context + forward queue + cost-prudence
   guidance for real-problem testing)
2. handover/ai-direct/LATEST.md "✅ Stage C Polymarket SHIPPED FINAL" block
3. handover/directives/2026-05-09_STAGE_C_POLYMARKET_OVERALL_§8_SIGN_OFF.md
4. handover/audits/CODEX_STAGE_C_OVERALL_AUDIT_2026-05-09_R3.md +
   GEMINI_STAGE_C_OVERALL_AUDIT_2026-05-09_R3.md (3-round audit lineage)
5. MEMORY.md (Stage C SHIPPED FINAL row at top + new
   feedback_admission_fail_closed_default entry)

Tell me what you want to do:
(a) Real-problem testing M0 mini smoke — ~$1-5 LLM API spend; ~30-60 min;
    proves post-Stage-C harness still works end-to-end with real Lean +
    real DeepSeek calls. Per `feedback_smoke_before_batch` gate before
    any larger M1/M2 batch. RECOMMENDED FIRST per cost-prudence.
(b) Real-problem testing M1 mini batch (8p × n3) — ~$5-20; ~1-3 hours;
    only after (a) clean.
(c) Real-problem testing M2 (50p × n3) — ~$50-200; ~6-12 hours; only
    after (b) clean. Multi-hour wall-clock + per-batch monitoring
    recommended.
(d) Implement `constitution_admission_no_fail_open_default` gate — would
    have caught R2 Q10 fail-open default at gate-time. ~30 lines source-
    grep. Class 1-2; no §8.
(e) LP unwind / PoolStatus::Resolved/Closed lifecycle — Stage D
    readiness scope; needs architect explicit ship gate.
(f) C.5 PromptCapsule evaluator wire-up — forward per CLAUDE.md §4.3.
(g) Something else — describe it.
```

---

**End of post Stage C SHIPPED FINAL boot prompt.**
