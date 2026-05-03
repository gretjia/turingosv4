# OBS — TB-13 forward-fence mechanism doom-loop closure

**Date**: 2026-05-03 (TB-13 round-7 closure; Codex R6 verdict).
**Status**: OBS (observations; tracked for follow-up at TB-14 or whenever the fence ships into production).
**Triggered by**: Codex R6 audit + user directive ("如果6轮audit都不过，要停下来认真思考，根因在哪里").

## What this OBS records

The forward-fence test scaffold (`tests/tb_13_legacy_cpmm_forward_fence.rs`) has been the subject of 4 consecutive rounds (R3 → R4 → R5 → R6) of Codex CHALLENGEs, each finding a new edge case in the previous round's fix to the previous round's CHALLENGE. Per `feedback_audit_obs_bias` and `feedback_elon_mode_policy` (round-cap=2), the iteration has crossed the ROI threshold. This OBS documents the residual CHALLENGE from R6 and the explicit decision to stop iterating and ship.

## Carry-forward CHALLENGEs (this OBS)

### TB13-Q9/RQ6-PARTIAL-MARKER-MULTILINE (Codex R6, 2026-05-03)

**Codex finding**: round-7's `tb_13_scan_lines()` for marker-bearing files returns marker-spans UNION non-comment lines containing TB-13 type names. The "containing TB-13 type names" check is line-granular. A contributor could write a function whose multiline signature splits the TB-13 type and the forbidden token across adjacent lines:

```rust
fn stealth(
    _: CompleteSetMintTx,
) -> f64 {
    0.0_f64
}
```

The line containing `CompleteSetMintTx` is returned (Layer 2 sees no f64 there); the line containing `f64` is NOT returned (no TB-13 type name on it). SG-13.0.2 evades.

**Severity assessment**:
- Attack requires deliberate evasion: contributor must (1) add a TB-13 marker to the file, (2) split a stealth contribution across adjacent lines, (3) ensure no TB-13 type name appears on the f64 line, (4) survive code review.
- Defense-in-depth — not an enforcement-gate failure. SG-13.0.1 / SG-13.0.2 / SG-13.0.3 (architect ship gates) are PASS at HEAD `8efffa8`.
- Closure cost: ~30-60 min (e.g., extend window scan to ±N adjacent lines around any TB-13-type-use line; or shift to item/block-granular AST walk).

**Closure plan (deferred)**:
- Treat this as an evolving fence-mechanism task. When the fence next ships into production-binary scope (e.g., as a CI gate at TB-14+), refactor to item/block-granular AST walk. The `syn` crate or similar AST-aware tool is the right level of abstraction; the current line-by-line heuristic is a known limitation.
- Manual code review remains the fallback halt-trigger guard for stealth-multiline patterns until the AST refactor lands.

### TB13-RQ7 (round-3 carry-forward)

STEP_B compliance for sequencer.rs additive changes. Tracked at `OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md`. Codebase-wide process question, not TB-13-specific. Closure: amend `STEP_B_PROTOCOL.md` with additive-only carve-out.

### Codebase-wide agent-sig submit-time gap (round-2 carry-forward)

Tracked at `OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md`. TB-13 round-2 raised the bar to its 3 Class 3 variants (replay-time) and round-3 to submit-time. The other 5 agent variants (Challenge / TaskOpen / EscrowLock / FinalizeReward — system / TaskExpire — system) still have replay-time-only OR no per-variant signing helper. Codebase-wide CO P2.x AgentRegistry pass scope.

## Why we are stopping here

Per `feedback_audit_obs_bias`:
> Table by (id / fix cost / severity / contradicts-prior-user-instruction?); only OBS-defer multi-hour future-arch.

Per `feedback_elon_mode_policy`:
> Process-only streamlining (scope unchanged): round-cap=2, OBS threshold=3, cap-exception via auto-execute on determinate-best surgical patch; ship-with-OBS NOT for enforcement gates themselves.

The R3-R6 challenges have all been about the FORWARD-FENCE TEST SCAFFOLD's edge cases, NOT about TB-13 production code. Specifically:

- R1: V1 (negative MicroCoin) + V2 (replay-time agent sig) — production code defects, fixed in round-2.
- R2: TB13-AUTH (submit-time agent sig) — production code defect, fixed in round-3.
- R3: 5 challenges, mix of doc-drift / dead wire field / fence edge / smoke scope / process. Architect ship gates SG-13.0.1..8 + SG-13.1..8 all PASS. Codex explicit: "No VETO; I found no live money/collateral exploit in the TB-13 dispatch arms."
- R4: 2 challenges in round-5's test/doc fixes. Round-6 closed both.
- R5: 2 challenges in round-6's test fixes. Round-7 closed both.
- R6: 1 challenge in round-7's test fix. **Stopping here.**

The pattern: starting at R3, each round Codex finds a new edge case in the previous round's heuristic test scaffold. The forward-fence is a defense-in-depth tool, not an enforcement gate. Its internal design is implementation detail. The ship-gate enforcement (SG-13.0.1..3 architect ship gates) has been GREEN since round-1.

Continuing the loop has flat-or-decreasing marginal value:
- Each "fix" narrows one fence edge but introduces new logic = new edge cases.
- No asymptote — adversarial paranoid pressure on heuristic test scaffolds finds new edges forever.
- Real risk reduction came in R1-R2 (production code). Fence-mechanism iterations contribute only manual-review-tier defense-in-depth.

## Cross-references

- Codex R6 audit: `handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R6.md`
- Gemini R6 audit (PASS / High / PROCEED to SHIP): `handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R6.md`
- Auditor agent clean-room architect-directive alignment audit (PASS / high / PROCEED to SHIP): in-session output 2026-05-03 (records two endorsed deviations: `u128 ShareAmount` + `ResolutionRef` removed).
- Round-3 audit residuals: `handover/alignment/OBS_TB13_AUDIT_RESIDUAL_CHALLENGES_2026-05-03.md`
- Memory rule about audit-loop ROI flip: `feedback_audit_loop_roi_flip` (NEW; this session).

## Verdict at close

TB-13 ship-ready at HEAD `8efffa8`. Architect ship gates (SG-13.0.1..8 + SG-13.1..8) all PASS. Auditor agent + Gemini R4/R5/R6 all PASS. Codex R6 CHALLENGE is fence-mechanism edge-case, defense-in-depth, OBS-deferred per round-cap.

Manual code review remains the active halt-trigger guard for stealth multiline patterns. AST-aware fence refactor planned for TB-14+ when fence enters production-binary scope.
