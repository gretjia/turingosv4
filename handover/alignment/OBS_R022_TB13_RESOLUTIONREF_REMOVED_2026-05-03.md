# OBS R-022 — TB-13 ResolutionRef removed (TRACE_MATRIX backlink retired)

**Date**: 2026-05-03 (TB-13 Atom 6 round-5 — Codex RQ5 remediation).
**Triggered by**: pre-commit hook R-022 (TRACE_MATRIX pub-symbol-block).
**Removed symbol**: `pub struct ResolutionRef { resolution_tx_id, claimed_outcome }` in `src/state/typed_tx.rs`.
**Removed backlink**: `/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): system-resolution reference embedded in CompleteSetRedeemTx`.

## Why removal is correct

ResolutionRef carried two dead fields:

- `resolution_tx_id: TxId` — documented in round-4 as opaque traceability metadata, NOT validated against L4 (the sequencer ignores it; `task_markets_t[event_id.0].state` is the live truth-of-resolution). A field that no code consumes is wire weight, not architecture.
- `claimed_outcome: OutcomeSide` — a redundant copy of `CompleteSetRedeemTx.outcome`. Used only by an inner-consistency gate `redeem.outcome != redeem.resolution_ref.claimed_outcome → InvalidResolutionRef`. That gate was dead defense-in-depth: the agent signature covers both fields, so any tampering that desynchronized them would break the signature first.

Codex round-3 RQ5 raised this as the contract drift between doc and code. The previous session's round-4 closure was a doc-only fix that left the dead wire field locked into the on-disk encoding. This round-5 commit is the structural fix: drop the wrapper entirely, rely on `redeem.outcome` as the sole claim, preserve the state-mismatch rejection path via the existing match arm.

## Why the TRACE_MATRIX entry does not need a replacement

The TB-13 typed-tx surface is still flowchart-anchored — `CompleteSetRedeemTx` itself retains its `TRACE_MATRIX TB-13 Atom 1 (architect §4.3 + FR-13.4..5 + SG-13.5..6)` backlink at `src/state/typed_tx.rs`. ResolutionRef was an internal helper type without an independent flowchart role. Removing the type cleanly removes the backlink — there is no orphan code; the role it carried (claimed-outcome assertion) is now expressed directly via `CompleteSetRedeemTx.outcome`.

## Behavioral preservation evidence

- `cargo test --workspace = 789 passed / 0 failed / 150 ignored` (= round-3 baseline; no regression).
- SG-13.5 (`sg_13_5_redeem_unavailable_before_outcome_resolution`): PASS unchanged — Open / Expired states still rejected with `RedeemBeforeResolution`.
- SG-13.6 (`sg_13_6_redeem_after_yes_outcome_pays_yes_not_no`): PASS unchanged — including both mismatch checks (Finalized + outcome=No → `InvalidResolutionRef`; Bankrupt + outcome=Yes → `InvalidResolutionRef`). The match arm using `(market_state, redeem.outcome)` produces identical rejection behavior to the previous `(market_state, claimed_outcome)` form because all fixtures set `claimed_outcome = outcome` (the inner-consistency check could never independently fire on an honestly-signed tx).

## Wire-format note

This is a wire-format break. Per `feedback_no_retroactive_evidence_rewrite`: no production rows exist for TB-13 typed-tx; no migration is needed. Future TB-13 chaintape entries will use the 8-field `CompleteSetRedeemTx` shape directly.

## Cross-references

- Codex audit RQ5 finding: `handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md` (round-3).
- Round-4 doc-only fix (now superseded by structural fix): `handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md` §12.7 TB13-RQ5.
- Fix handoff that mandated this round: `handover/ai-direct/TB-13_FIX_HANDOFF_2026-05-03.md` §3.3.
