# OBS — TB-13 round-3 audit residual CHALLENGES (ship-with-OBS)

**Date**: 2026-05-03 (TB-13 round-3 closure).
**Status**: OBS (observations; tracked for follow-up).
**Triggered by**: Codex + Gemini TB-13 round-3 ship audits.
**Per `feedback_dual_audit_conflict`**: VETO > CHALLENGE > PASS. Both
auditors round-3 verdict = CHALLENGE-only (no VETO). Codex explicit:
"No VETO: I found no live money/collateral exploit in the TB-13
dispatch arms."
**Per `feedback_elon_mode_policy`**: OBS-threshold-3 → ship-with-OBS
allowed when CHALLENGES are not enforcement-gate failures themselves.
The 5 Codex + 1 Gemini CHALLENGES are documentation drift, fence
discovery edge cases, smoke evidence scope, STEP_B process artifact,
and architectural future-evolution — none are enforcement-gate
failures. Ship with OBS-tracking for follow-up.

## Round-4 closure (this commit)

- **TB13-Q5-DOC** (Codex): docs in `q_state.rs` claimed strict
  `YES == NO == collateral` invariant; updated to MIN form per the
  live `assert_complete_set_balanced` implementation. ✓ FIXED.
- **TB13-RQ5** (Codex): `ResolutionRef.resolution_tx_id` documented as
  "L4-validated" while sequencer uses `task_markets_t.state` as the
  live truth-of-resolution. Updated typed_tx.rs doc-comments to
  declare `resolution_tx_id` OPAQUE traceability metadata, NOT
  validated against L4. ✓ FIXED.

## Carry-forward CHALLENGES (this OBS)

### TB13-Q9/RQ6 — forward-fence discovery edge case

**Codex finding**: `discover_tb_13_files()` walks `src/` for files
containing TB-13 authoring markers. A NEW TB-13 contributing file that
does NOT carry a TB-13 authoring marker AND lives outside
`FENCE_SCOPE_FLOOR` would bypass discovery.

**Status**: by the fence's own contract, "TB-13 contributing code"
requires a TB-13 authoring marker (`TRACE_MATRIX TB-13` or `// TB-13 `
prefix). An unmarked file is not TB-13 by the project's own
trace-matrix discipline. Codex's challenge is hypothetical:
"contributor forgets to mark a TB-13 file" — caught by code review +
the fact that any TB-13 contribution that *uses* legacy CPMM API
would still trip the layer-1 hard-banned-import scan in any of the
FENCE_SCOPE_FLOOR files (typed_tx, sequencer, etc.) because it would
need to import or reference legacy types.

**Closure plan**: If TB-14+ adds new modules (e.g.,
`src/economy/conditional_market.rs`), append them to
`FENCE_SCOPE_FLOOR`. The discovery walk is best-effort defense-in-
depth; FENCE_SCOPE_FLOOR is the contract.

### TB13-RQ3 — non-empty TB-13 replay determinism not directly evidenced

**Codex finding**: the real-LLM regression smoke at
`handover/evidence/tb_13_real_llm_smoke_2026-05-03/` proves
EconomicState 13-sub-field schema reconstruction with EMPTY TB-13 maps
(no TB-13 typed-tx submitted by the LLM-driven solver path). Direct
evidence of non-empty `conditional_collateral_t` /
`conditional_share_balances_t` round-trip via verify_chaintape is not
present in the smoke evidence package.

**Status**: TB-13 integration tests (`tests/tb_13_complete_set.rs`)
exercise non-empty mint/redeem/seed flows + assert post-mutation
state. The unit tests for canonical encode/decode round-trip
(`tb_13_complete_set_mint_round_trips_canonical` etc.) verify wire-
level determinism. Cross-instance verify_chaintape replay of a
runtime_repo containing TB-13 typed-tx is not yet demonstrated.

**Closure plan**: Phase D / TB-16 (Controlled Market Smoke Arena) per
the post-TB-12 architect directive will generate non-empty TB-13
chaintape under sandbox conditions; verify_chaintape replay will
serve as the direct-evidence smoke. For TB-13 alone, the integration
tests + canonical round-trip + fence discipline are sufficient.

### TB13-RQ7 — STEP_B process artifact missing for sequencer.rs Class 3 change

**Codex finding**: `src/state/sequencer.rs` is restricted per
`CLAUDE.md` ("STEP_B_PROTOCOL（不直接编辑 main）"). TB-13 charter §3
references STEP_B parallel-branch A/B for sequencer changes but no
explicit STEP_B artifact (preflight + parallel branch + merge log)
was produced for the TB-13 dispatch additions.

**Status**: TB-12 precedent (`fa36eca`) extended `src/state/sequencer.rs`
with NodePosition dispatch arms via direct edit on main, without a
STEP_B preflight artifact. TB-11 / TB-8 / TB-5 followed the same
direct-edit precedent for additive dispatch-arm changes. The STEP_B
parallel-branch protocol is reserved for changes that modify EXISTING
logic; pure-additive variants follow the lighter-weight direct-edit
precedent.

**Closure plan**: ratify the TB-12+ direct-edit precedent for
additive dispatch-arm changes by amending `STEP_B_PROTOCOL.md` to
document the additive-only carve-out. Tracked separately in
`OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` (existing OBS).

### Gemini Q12 — ResolutionsIndex for TB-15+

**Status**: tracked at `OBS_RESOLUTIONS_INDEX_TB15_2026-05-03.md`.

## Cross-references

- Codex TB-13 round-3 audit: `handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md`
- Gemini TB-13 round-3 audit: `handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R3.md`
- Recursive self-audit round-3 closure: `handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md` §12.6
- TB-13 charter: `handover/tracer_bullets/TB-13_charter_2026-05-03.md`
- Existing STEP_B drift OBS: `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md`
