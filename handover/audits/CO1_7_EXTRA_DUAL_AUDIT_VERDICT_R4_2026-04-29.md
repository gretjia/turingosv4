# CO1.7-extra Dual External Audit — Round-4 Merged Verdict ✅ PASS/PASS

**Date**: 2026-04-29
**Target**: `CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md` v1.2 at HEAD `13bfb7e`
**Audits**: Codex r4 (`CODEX_CO1_7_EXTRA_ROUND4_AUDIT_2026-04-29.md`) + Gemini r4 (`GEMINI_CO1_7_EXTRA_ROUND4_AUDIT_2026-04-29.md`)

## Verdict matrix

| Audit | Verdict | Conviction |
|---|---|---|
| Codex | **PASS** | High |
| Gemini | **PASS** | High |

**Conservative-merged verdict**: **PASS / PASS** ✅ — pre-implementation gate cleared per CLAUDE.md "Audit Standard".

## Round-3 must-fix closure (per round-4 review)

| R3 B | Codex r4 status | Gemini r4 status |
|---|---|---|
| B1 (`&**writer_w` → `&*writer_w`) | ✅ PASS at spec line 102 | ✅ PASS (mechanical fix; no architectural impact) |
| B2 (`pub(crate)` → `pub`) | ✅ PASS at spec line 84; FC-trace present at line 79; no over-exposure | ✅ PASS ("`pub` is appropriate for constitutional-anchor helper") |
| B3 (stale Kernel refs at line 14 + 395) | ✅ PASS — preface 14 says "Kernel UNTOUCHED"; § 6 line 408 file list excludes kernel.rs | ✅ PASS |
| B4 (serde-skip conditional + LoC sync) | ✅ PASS — § 2.1 lines 175-182 conditional comment matches source state; LoC synced 210-300 | ✅ PASS |

## New findings (non-blocking editorial nits — Codex r4 Q5)

These do NOT affect the PASS verdict. Listed for v1.2.1 cleanup:

| ID | Source | Issue |
|---|---|---|
| **N1** | Codex Q5 | spec line 69 still labels the snippet `NEW pub(crate) helper` (stale label) while lines 81-84 unambiguously declare `pub fn`. Visual contradiction. |
| **N2** | Codex Q5 | spec line 66 names test file `tests/co1_7_extra_head_t_advancement.rs`; § 3.3 line 308 uses `tests/co1_7_extra_sequencer_head_t_advancement.rs`. File name inconsistency. |

ArchitectAI applies both nits as v1.2.1 (no re-audit needed; Codex explicitly framed as "non-blocking nits", and v1.2.1 introduces zero new claims to audit).

## Where audits agreed (round-4)

- ✅ All 4 round-3 B-fixes correctly applied and verified at source-level
- ✅ No new architectural defects introduced by mechanical patches
- ✅ Spec is implementable end-to-end with no further blockers
- ✅ Both audits PASS with high conviction

## Where audits disagreed (round-4)

None. Both verdicts converged on PASS/High with consistent reasoning. Codex caught the 2 editorial nits Gemini missed, but framed them as non-blocking.

## Cumulative audit history (CO1.7.5 → CO1.7-extra)

| Round | Codex | Gemini | Conservative | Outcome |
|---|---|---|---|---|
| **r1** (bundled CO1.7.5 v1) | CHALLENGE/High | CHALLENGE/High | CHALLENGE | scope split via Occam (B2 decomposition) → CO1.7-extra atom carved out |
| **r2** (CO1.7-extra v1) | CHALLENGE/High | CHALLENGE/High | CHALLENGE | 10 MFs (MF1-MF10) applied → v1.1 |
| **r3** (CO1.7-extra v1.1) | CHALLENGE/High | PASS/High | CHALLENGE | 4 mechanical fixes (B1-B4) applied → v1.2 |
| **r4** (CO1.7-extra v1.2) | **PASS/High** | **PASS/High** | ✅ **PASS/PASS** | pre-implementation gate cleared |

Total rounds: 4 (CO1.7 spec also took 3 rounds; CO1.7.5 bundled→split→4-rounds is comparable).

## Audit cost summary

- Codex r4: 30,269 tokens (much smaller than r3's 140k — round-4 was tightly focused)
- Gemini r4: similar reduction
- Estimated round cost: ~$2-4
- **Cumulative project audit spend**: ~$196-314 / $890 mid-budget (~22-35%)
- CO1.7-extra atom-only spend: ~$25-43 across 3 audit rounds (r2 + r3 + r4; r1 was on bundled CO1.7.5 not CO1.7-extra)

## Status going forward — PRE-IMPLEMENTATION GATE CLEARED ✅

1. **CO1.7-extra v1.2.1** (with 2 editorial nits cleaned): final spec; ready for implementation. No further audit needed.
2. **CO1.7-extra-impl** (Task #4 unblocked): may now proceed per spec § 1-3:
   - D2: `advance_head_t` helper extraction in `src/state/sequencer.rs` + apply_one stage 9 patch + required trait method `head_commit_oid_hex` + 2 impl declarations in `src/bottom_white/ledger/transition_ledger.rs` + stale-comment updates at sequencer.rs:178-184/:357-361
   - D3: TuringBus single-file STEP_B ceremony — A/B branches add field + constructor + forwarder (~50-80 LoC)
   - D4: 3 substrate-independent integration tests
   - LoC ~210-300; calendar 1-2 days
3. **STATE_TRANSITION_SPEC v1.5 housekeeping issue**: file as part of CO1.7-extra atom closure per § 0.4 commitment.
4. **LATEST.md correction**: still pending; Wave 6 #1 ~30-40% diagnosis confirmed across 4 audit rounds.
5. **Future CO1.7.5 spec** (transition bodies; Task #6): gated on CO P2.x substrate atoms reaching individual PASS/PASS.

## Achievement annotation

CO1.7-extra is the SECOND atom (after CO1.7) to clear pre-implementation gate via dual-audit. It exemplifies the project's iterative audit pattern:
- r1 found systemic issue (substrate gap) → architectural Occam scope-split (NOT a patch round)
- r2-r4 progressive refinement → 10 MFs → 4 B-fixes → 2 nits
- Final atom is ~3× smaller than the original bundled scope, with cleaner architecture (TuringBus owns Sequencer; Kernel preserves "pure topology") and stronger guarantees (compiler-enforced trait method).
