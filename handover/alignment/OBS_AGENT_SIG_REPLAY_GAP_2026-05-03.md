# OBS — Codebase-wide agent-signature verification gap (forward-dep, CO P2.x AgentRegistry)

**Date**: 2026-05-03 (TB-13 round-2 audit remediation).
**Status**: OBS (observation; tracked for future codebase-wide pass).
**Triggered by**: Codex TB-13 ship audit round-1 VETO TB13-V2 (`handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md`).

## Summary

`AgentSignature` fields on agent-submitted typed-tx variants are NOT live-verified at the sequencer ingress (`submit_agent_tx`) NOR at apply time (`apply_one`) for ANY variant. Verification happens ONLY at chain-replay time via `verify_chaintape::Gate 4` (`src/runtime/verify.rs`) — and Gate 4 historically covered only `WorkTx` + `VerifyTx` per TB-7 ARCHITECT_RULING D3 narrowed scope.

This is a **codebase-wide forward dependency** (typed_tx.rs:813 comment: "actual `verify_agent_signature` ... + agent-pubkey-registry lookup is CO P2.x AgentRegistry territory"). It is NOT a TB-13 regression; TB-13 inherits the existing pattern.

## TB-13 round-2 partial remediation

Per `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS), TB-13 round-2 raised the bar for its three new agent-signed variants:

- **Replay-time (Gate 4) coverage extended** (`src/runtime/verify.rs:406-501`):
  - `CompleteSetMint` → verify against owner's pubkey
  - `CompleteSetRedeem` → verify against owner's pubkey
  - `MarketSeed` → verify against provider's pubkey

- **Submit-time / apply-time coverage**: NOT extended. The sequencer does not currently load an `AgentPubkeyManifest` at construction time (it loads `PinnedSystemPubkeys` only); plumbing agent-pubkey verification into `submit_agent_tx` is a substantial refactor that would mirror the same gap in `Challenge`, `TaskOpen`, `EscrowLock` variants. **This is the OBS**.

## Agent-signed variants currently unverified at submit/apply time

| Variant            | Replay-time (Gate 4)                  | Submit-time (`submit_agent_tx`) | Apply-time (`apply_one`) |
| ------------------ | ------------------------------------- | ------------------------------- | ------------------------ |
| `WorkTx`           | ✓ verified (TB-7)                     | ✗ deferred                      | ✗ deferred               |
| `VerifyTx`         | ✓ verified (TB-7)                     | ✗ deferred                      | ✗ deferred               |
| `ChallengeTx`      | ✗ deferred                            | ✗ deferred                      | ✗ deferred               |
| `TaskOpenTx`       | ✗ deferred                            | ✗ deferred                      | ✗ deferred               |
| `EscrowLockTx`     | ✗ deferred                            | ✗ deferred                      | ✗ deferred               |
| `CompleteSetMintTx` (TB-13)   | ✓ verified (TB-13 round-2)  | ✗ deferred                      | ✗ deferred               |
| `CompleteSetRedeemTx` (TB-13) | ✓ verified (TB-13 round-2)  | ✗ deferred                      | ✗ deferred               |
| `MarketSeedTx` (TB-13)        | ✓ verified (TB-13 round-2)  | ✗ deferred                      | ✗ deferred               |

System-emitted variants (`FinalizeReward`, `TaskExpire`, `TerminalSummary`, `ChallengeResolve`, `TaskBankruptcy`) ARE verified at apply-time via `system_message_for_verification` + `verify_emitted_system_tx_signature`; their auth model is settled.

## Threat model under current state

Under the current model, a forged agent signature would:

1. Pass `submit_agent_tx` (no signature check at admission).
2. Pass `apply_one` stage 1.5 (system-only verification skips agent variants).
3. Be appended to L4 with a state mutation taking effect.
4. Be **caught at replay time** (Gate 4 sets `agent_signatures_verified=false` in `replay_report.json`).

For Class 3 (money/collateral) variants, this is post-hoc detection — the state has already mutated by the time a replay catches the forgery. Real-time rejection requires submit-time / apply-time verification.

For TB-13 specifically, the replay-time coverage (this round-2 fix) means a forged TB-13 signature is detectable. Pre-fix, the tx and forgery were both indistinguishable from genuine.

## Closure plan (CO P2.x AgentRegistry)

Future codebase-wide pass should:

1. Plumb `AgentPubkeyManifest` (or successor `AgentRegistry`) into `Sequencer::new` alongside `PinnedSystemPubkeys`.
2. Add submit-time agent-signature verification for ALL agent-signed variants (Work, Verify, Challenge, TaskOpen, EscrowLock, CompleteSetMint, CompleteSetRedeem, MarketSeed).
3. Add `TransitionError::SignatureInvalid` rejection path at submit_agent_tx (variant already exists at `src/state/typed_tx.rs:1688` but is currently dead code — Codex round-1 noted this as defense-in-depth wired but never set).
4. Extend Gate 4 in `verify.rs` to cover Challenge / TaskOpen / EscrowLock / TaskBankruptcy alignment with the submit-time set.
5. Update fixture builders to use real keypair-derived signatures (current TB-3..TB-13 fixtures use `[0u8; 64]` placeholders that would fail real verification).

## Cross-references

- Codex TB-13 round-1 audit: `handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md` (TB13-V2 VETO finding)
- TB-13 round-2 remediation commits: see RECURSIVE_AUDIT_TB_13_2026-05-03.md §10 (round-2 update)
- typed_tx.rs forward-dep comment: `src/state/typed_tx.rs:813`
- TB-7 ARCHITECT_RULING D3 (narrowed scope): `handover/architect-insights/RULING_TB7_*.md`
- verify.rs Gate 4: `src/runtime/verify.rs:406-501`
- Existing test fixtures using placeholder signatures: `tests/tb_3_*.rs`, `tests/tb_4_*.rs`, ..., `tests/tb_13_complete_set.rs:128`
