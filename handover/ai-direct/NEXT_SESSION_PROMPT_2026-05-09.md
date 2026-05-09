# Next Session Boot Prompt — 2026-05-09 close

> Paste the **`USER PROMPT`** block at the bottom into the next Claude session. Everything above it is context for cold-start orientation if you read this file directly.

---

## State at session #27 close (2026-05-09)

- **HEAD**: `d15b868` (pushed to `origin/main`)
- **Constitution gates**: `212 PASS / 0 FAIL / 1 ignored` (was 175 at session start; +37 / +21%)
- **Stage C Polymarket**: all 10 P-M atoms (P-M0..P-M9) 🟢 SHIPPED at gate level
- **Workspace tests**: 1313+ passed; 1 pre-existing env-var flake unrelated to Stage C
- **Plan executed**: `/home/zephryj/.claude/plans/cozy-waddling-raven.md` Steps 0-13 all complete locally
- **Two §8 packets PENDING architect ratification**:
  1. Class-4 batch (P-M2+P-M4+P-M6): `handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM2_PM4_PM6_BATCH_§8_PACKET.md`
  2. Stage C overall: `handover/directives/2026-05-09_STAGE_C_POLYMARKET_OVERALL_§8_PACKET.md`

## Critical files for cold-start orientation

Read in this order on next session boot:
1. `CLAUDE.md` (project constitution / operating mode)
2. `constitution.md` (top-level law)
3. **This file** (`handover/ai-direct/NEXT_SESSION_PROMPT_2026-05-09.md`)
4. `handover/ai-direct/LATEST.md` — top-of-file "🚧 Open after Polymarket" block (committed Step 1)
5. `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` (gate-level CI view; should still be 0 AMBER / 0 RED)
6. `handover/alignment/CONSTITUTION_LANDING_MANIFEST_2026-05-09.md` (layer-organized real-status view; **NEEDS REGEN** per §15 triggers — see §3 below)
7. The two §8 packets from §1 above (read both fully before any architect-side action)

## Branching state

```
main:                   d15b868 (pushed) ← canonical
feat/p-m2-completeset-merge   (merged via bac20ba; can be deleted post-§8)
feat/p-m4-cpmm-pool           (merged via 8c74034; can be deleted post-§8)
feat/p-m5-cpmm-swap           (merged via 6d3cb0c; can be deleted post-§8)
feat/p-m6-router              (merged via 7395aaa; can be deleted post-§8)
```

## §1 — Likely paths the next session takes

### Path A: Architect §8 lands GREEN (most likely)

1. File `handover/directives/2026-05-09_STAGE_C_POLYMARKET_§8_SIGN_OFF.md` with verbatim multi-clause ratification (precedent: `handover/directives/2026-05-08_STAGE_A3_§8_SIGN_OFF.md` for canonical phrasing).
2. Regenerate `CONSTITUTION_LANDING_MANIFEST_2026-05-09.md` per §15 triggers — see §3 below for the regeneration delta.
3. Update `LATEST.md` to mark Stage C SHIPPED FINAL (mirror the Stage A3 §8 sign-off entry pattern).
4. Memory `MEMORY.md` update: add Stage C Polymarket SHIPPED FINAL row to "Active state (动态; TB_LOG.tsv 为准)" section.
5. Optionally: clean up the 4 merged feature branches (`git branch -d feat/p-m2-completeset-merge feat/p-m4-cpmm-pool feat/p-m5-cpmm-swap feat/p-m6-router`).

### Path B: Architect §8 returns CHALLENGE / partial veto

Both §8 packets include explicit rollback instructions:
- Batch packet §6 (CR-StageC-PM.16 deviation note + risk acceptance) + §10 (Architect §8 Sign-Off Request — partial-veto handling)
- Overall packet §10 (recommended cadence — A → B → C → D)

Rollback options span: full revert (`git revert d15b868~12..d15b868`), per-atom revert (revert single merge commit), or in-place patch (continue on main).

### Path C: User wants to keep going on forward-bound work (don't wait for §8)

Pick from `LATEST.md` "🚧 Open after Polymarket" block. Top candidates by ROI:

| Forward item | Class | ETA | ROI |
|--------------|-------|-----|-----|
| **C.5 PromptCapsule evaluator wire-up** | 3 | ~1-2 days | High — closes manifest C.5 PARTIAL-S; required for full FC1 hard-invariant strictness |
| **B.4 CAS Merkle redesign** (Stage A3.6 enhancement TB) | 3-4 | ~3-5 days | High — closes manifest B.4 KNOWN-GAP; replay strict-proof |
| J.2 Full M1 (450 cells) | runner-only | ~3 days wall + ¥budget | Medium — substrate-stability data; no runtime correctness gain |
| J.5 4 replay sampling tests | 1 | ~1 day | Low — gate-level; gated on M2 evidence |
| K.* Stage D readiness directive package | architect Class-4 | architect timeline | Low — architect-side; decoupled |

**DO NOT START** on forward-bound work until either (a) Stage C overall §8 ratifies OR (b) user explicitly authorizes parallel work.

## §2 — Pre-action gate (mandatory before any first edit)

Per `MEMORY.md` "MUST CHECK BEFORE":
- **Before drafting any new TB charter / dispatching G1 audit / picking next atom**: invoke `/constitution-landing-check` skill — surfaces AMBER rows + classifies addressability.
- **Before any `bash run_*.sh` runner script**: invoke `/runner-preflight` — 7-stage tree/binary/evidence/Class/FC-trace/charter/audit-rounds gate.

Stage C work is forward execution against an already-ratified charter; the constitution-landing-check at session #27 returned PROCEED. If next session opens new charter work (post-Stage-C), the gate must fire fresh.

## §3 — Manifest regeneration delta (per §15 triggers)

The next session that runs after Stage C overall §8 lands MUST regenerate `CONSTITUTION_LANDING_MANIFEST_2026-05-09.md`. The delta from the current manifest:

| Manifest row | Old status | New status | Trigger |
|--------------|------------|------------|---------|
| §9 L9 — I.0 P-M0 quarantine | 🟢 DONE (label pending) | 🟢 DONE | charter §3.1 SHIPPED label landed (commit `e632a82`) |
| §9 L9 — I.1 P-M1 CompleteSet hardening | 🟢 DONE | 🟢 DONE | manifest already matches (verbatim binding confirmed `e0ed12c`) |
| §9 L9 — I.2 P-M2 CompleteSetMergeTx | ⚪ NOT-STARTED | 🟢 DONE | commit `bac20ba`; 5/5 verbatim GREEN |
| §9 L9 — I.3 P-M3 MarketSeed hardening | 🟡 PARTIAL-W | 🟢 DONE | commit `a227189`; closes D.4 |
| §9 L9 — I.4 P-M4 CpmmPool | ⚪ NOT-STARTED | 🟢 DONE | commit `8c74034`; 4/4 verbatim GREEN |
| §9 L9 — I.5 P-M5 Share-only swap | ⚪ NOT-STARTED | 🟢 DONE | commit `6d3cb0c`; 6/6 verbatim GREEN |
| §9 L9 — I.6 P-M6 Mint-and-Swap Router | ⚪ NOT-STARTED | 🟢 DONE | commit `7395aaa`; 9/9 verbatim GREEN |
| §9 L9 — I.7 P-M7 PriceIndex | ⚪ NOT-STARTED | 🟢 DONE | commit `ba3a35d`; 4/4 verbatim GREEN |
| §9 L9 — I.8 P-M8 audit_tape views | ⚪ NOT-STARTED | 🟢 DONE | commit `48675a4`; 3/3 verbatim GREEN |
| §9 L9 — I.9 P-M9 controlled smoke | ⚪ NOT-STARTED | 🟢 DONE | commit `17230ca`; 6/6 SG-StageC-PM.9 gates PASS |
| §4 L4 — D.4 No ghost liquidity | 🟡 PARTIAL-W | 🟢 DONE | commit `a227189`; verbatim binding 5/5 |
| §10 L10 — J.3 M2 batch | 🟡 RUNNING (31 cells) | ⚪ KILLED-FORWARD-BOUND | commit `f4e5c44`; killed at cell 49 |
| §12 Summary statistics | 55 DONE / 4 PARTIAL / 16 NOT-STARTED / 1 KNOWN-GAP | **65 DONE / 1 PARTIAL / 6 NOT-STARTED / 1 KNOWN-GAP** | per-row deltas above |

Items NOT changed by Stage C:
- B.4 CAS root commit-chain — still KNOWN-GAP (Stage A3.6 enhancement TB)
- C.5 PromptCapsule evaluator wire-up — still PARTIAL-S (forward post-Polymarket)
- J.2 Full M1 — still NOT-DONE (forward post-Polymarket)
- J.5 replay sampling tests — still NOT-STARTED
- K.* Stage D — still NOT-STARTED / GATED

## §4 — Forward open items (no drift; recorded in LATEST.md)

The "🚧 Open after Polymarket" block at top of `LATEST.md` is the canonical tech-debt log. Each row references manifest entry + estimated work + non-blocking rationale. Do NOT pick up these items until Stage C overall §8 ships.

## §5 — Memory entries to update post-§8

When Stage C ships FINAL, update `MEMORY.md` (per `feedback_kolmogorov_compression` lossless rule):

```markdown
- **Stage C Polymarket SHIPPED FINAL 2026-05-XX** (architect §8: `handover/directives/2026-05-XX_STAGE_C_POLYMARKET_§8_SIGN_OFF.md`; ...). All 10 P-M atoms (P-M0..P-M9) 🟢 SHIPPED at HEAD `<commit>`. Constitution gates: 175 → 212 (+37 new gates / +21%). 50 verbatim tests across 10 architect-mandated names (§7.2-7.10). Class-4 batch §8 (P-M2+P-M4+P-M6) ratified per CR-StageC-PM.16 deviation cadence. P-M9 controlled smoke evidence at `handover/evidence/stage_c_pm9_controlled_smoke_20260509T042633Z/` with verdict PASS + 6/6 SG-StageC-PM.9 gates GREEN. Substrate-stable @ 109 cumulative cells (Wave 3 50p + A3 R3.5 + B3 R6 mini-M1 + B3 R7 M2 partial 49). FC1 hard invariant continuous.
```

Also update the "Active charter" line + freeze-lift notice if applicable.

## §6 — Key references (canonical sources)

| Reference | Purpose |
|-----------|---------|
| `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md` | Charter (per-phase SHIPPED markers updated) |
| `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md` §7.1-7.10 | Architect engineering manual (verbatim spec for each atom) |
| `handover/decisions/2026-05-09_M2_KILL_AND_SUBSTRATE_STABLE_DECLARATION.md` | M2 kill decision rationale + substrate-stable cumulative-109-cell evidence |
| `/home/zephryj/.claude/plans/cozy-waddling-raven.md` | Plan executed in session #27 (Steps 0-13 all complete locally) |
| `tests/constitution_completeset_merge.rs` (P-M2) | 5 verbatim tests |
| `tests/constitution_marketseed_hardening.rs` (P-M3) | 5 verbatim tests; closes D.4 |
| `tests/constitution_cpmm_pool.rs` (P-M4) | 4 verbatim tests |
| `tests/constitution_cpmm_swap.rs` (P-M5) | 6 verbatim tests |
| `tests/constitution_router_buy_with_coin.rs` (P-M6) | 9 verbatim tests |
| `tests/constitution_price_index_signal_only.rs` (P-M7) | 4 verbatim tests |
| `tests/audit_tape_views.rs` (P-M8) | 3 verbatim tests |
| `tests/stage_c_pm9_controlled_smoke.rs` (P-M9) | 1 integration smoke; 6 SG gates |

---

## USER PROMPT (paste this into next Claude session)

```
Stage C Polymarket fast-path landed in session #27 (2026-05-09). HEAD `d15b868` pushed
to origin/main. Constitution gates 212/0/1 (was 175). All 10 P-M atoms SHIPPED at gate
level; two §8 packets pending architect ratification:
- handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM2_PM4_PM6_BATCH_§8_PACKET.md
- handover/directives/2026-05-09_STAGE_C_POLYMARKET_OVERALL_§8_PACKET.md

Read first:
1. handover/ai-direct/NEXT_SESSION_PROMPT_2026-05-09.md (this prompt's source; full
   context + path A/B/C decision tree + manifest regen delta)
2. handover/ai-direct/LATEST.md top "🚧 Open after Polymarket" block
3. handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md
4. The two §8 packets above

Tell me what you want to do:
(a) architect §8 ratified — file the §8 sign-off + regenerate manifest + update LATEST.md
(b) architect §8 vetoed / challenged — apply rollback per packet §6/§10 instructions
(c) work forward-bound items in parallel — pick from "Open after Polymarket" (NOT
    recommended until §8 ships, but viable if explicitly authorized)
(d) something else — describe it
```

---

**End of next-session boot prompt.**
