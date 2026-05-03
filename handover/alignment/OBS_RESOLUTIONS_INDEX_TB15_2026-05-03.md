# OBS — ResolutionsIndex evolution for TB-15+ (Gemini Q12 CHALLENGE)

**Date**: 2026-05-03 (TB-13 round-3 audit closure).
**Status**: OBS (observation; tracked for future TB).
**Triggered by**: Gemini TB-13 ship audit round-3 Q12 CHALLENGE
(`handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R3.md` §Q12).
**Conflict-resolution rule**: Codex round-3 explicit "No VETO"; Gemini Q12
explicitly "non-blocking, recommends OBS rather than blocks ship".

## Summary

TB-13's `ResolutionRef` model couples redemption logic to specific
resolution-emitting transaction types (`TaskBankruptcyTx` for No,
`FinalizeRewardTx` for Yes). The validation lives in the
`CompleteSetRedeemTx` dispatch arm at `src/state/sequencer.rs:1882-1900`
as a hardcoded `match (market_state, claimed_outcome)`.

**Forward concern**: TB-15+ may introduce additional resolution
mechanisms (ChallengeCourt rulings, oracle updates, multi-resolver
quorum). Each new resolver requires modifying the `CompleteSetRedeemTx`
dispatch arm to recognize the new tx type / state mapping.

A more abstract long-term design — proposed by Gemini — is a canonical
`ResolutionsIndex` in `QState`:

```rust
/// Maps event_id → resolved outcome side. Set by ANY system-emitted
/// resolution transaction (TaskBankruptcy, FinalizeReward, future
/// ChallengeCourt, future Oracle, etc.). Read-only after set.
pub struct ResolutionsIndex(pub BTreeMap<EventId, OutcomeSide>);
```

The redemption logic would then be a single, decoupled lookup:
```rust
let outcome = q.economic_state_t.resolutions_t.0
    .get(&redeem.event_id)
    .ok_or(TransitionError::RedeemBeforeResolution)?;
if redeem.outcome != *outcome {
    return Err(TransitionError::InvalidResolutionRef);
}
```

New resolvers add a write site to populate `resolutions_t` without
touching the redemption arm.

## Why this is OBS, not blocking

Per Gemini R3: "The current implementation is not a bug but represents
an architectural choice that incurs future refactoring debt. ...The
recommended 'fix' is to add an OBS to the project backlog to track the
redesign of the resolution mechanism for TB-15, rather than blocking
the TB-13 ship."

Per Codex R3: "No VETO: I found no live money/collateral exploit in the
TB-13 dispatch arms." Codex's RQ5 challenge on `resolution_tx_id`
opacity was addressed in round-4 doc remediation (2026-05-03 commit
post-this-OBS).

## Closure plan (TB-15 prerequisite)

- Extend `EconomicState` with `resolutions_t: ResolutionsIndex` (14th
  sub-field; backward-compat `#[serde(default)]`).
- Refactor `TaskBankruptcyTx` and `FinalizeRewardTx` dispatch arms to
  populate `resolutions_t` alongside their existing state-flip logic.
- Refactor `CompleteSetRedeemTx` dispatch arm to consult `resolutions_t`
  instead of pattern-matching on `task_markets_t.state`.
- Make `ResolutionRef.resolution_tx_id` validation explicit (lookup in
  L4, verify it's the actual tx that resolved this event) OR drop the
  field as opaque metadata per Codex round-3 RQ5.

## Cross-references

- Gemini TB-13 R3 audit Q12 CHALLENGE: `handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R3.md`
- Codex TB-13 R3 audit RQ5 CHALLENGE: `handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md`
- Recursive self-audit round-3 closure: `handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md` §12.6
- TB-13 charter: `handover/tracer_bullets/TB-13_charter_2026-05-03.md`
- Architect 2026-05-02 supplementary directive (TB-15 scope): `handover/directives/2026-05-02_TB11_TO_TB17_SUPPLEMENTARY_DIRECTIVE.md`
