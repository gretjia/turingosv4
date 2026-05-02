# Checkpoint TB-7R #1 — CAS race + payload CAS

**Date**: 2026-05-02
**Phase**: TB-7R Checkpoint 1 of 3 (per verdict C7).
**Status**: GREEN — closes via existing commits, no new code required for this checkpoint.

---

## What changed (relative to TB-7.5 ship-state)

| Item | Commit | Source charter | Verdict mapping |
|---|---|---|---|
| CAS index sidecar atomic write | `c0ec514` (TB-7.6) | TB-7.6 charter §1 | Verdict §8 / "Commit 1: CAS race" |
| Disk-level tamper battery (I90d/e/f) | `c0ec514` | TB-7.6 charter §2 | tamper-detectability for sidecar |
| Proposal payload bytes → CAS | `a39c31b` (TB-7.7 D1) | TB-7.7 charter §2 D1 | Verdict §8 / "Commit 2: payload CAS" |
| `ProposalTelemetry.proposal_artifact_cid` resolve guarantee | `a39c31b` | TB-7.7 charter §2 D1 ship gate | Verdict §11 acceptance clause |

## What tests pass

`cargo test --workspace` at HEAD `e9cb023`: **698 passed / 0 failed / 150 ignored** (carried forward from D7 PENDING handover; not re-run for this checkpoint since no code changed).

Re-run will happen as part of Checkpoint 2 prep (after Deliverables A-D ship), per `feedback_workspace_test_canonical`.

## What remains red

Nothing red for Checkpoint 1's narrow scope. For the TB-7R arc as a whole:

- Deliverable B (ChainTape-mode fail-closed) — pending, blocks Checkpoint 2.
- Deliverable C (genesis_report emission) — pending.
- Deliverable D (on-chain TaskOpen + EscrowLock) — pending; may trigger STEP_B if it touches `src/state/sequencer.rs`.

## Kill-criterion status (per TB-7R charter §3 phase declarations)

| Kill criterion | Phase | Status at Checkpoint 1 |
|---|---|---|
| P1:1 (CAS race makes ledger un-replayable) | P1 | NOT TRIGGERED (fixed at `c0ec514`) |
| P1:2 (payload not retrievable from CAS) | P1 | NOT TRIGGERED (D1 fix at `a39c31b`) |
| P1:3 (signature verify breaks) | P1 | NOT TRIGGERED (still GREEN per `replay_report.json` of all evidence dirs) |
| P1:4 (replay reconstruction breaks) | P1 | NOT TRIGGERED |
| P3:1-3 (RSP carry-forward conservation) | P3 | NOT EVALUATED at this checkpoint (settlement out of scope) |

## Carry-forward to Checkpoint 2

Checkpoint 2 will record: parent_tx wire (already at `a39c31b` D2) + VerificationResult CAS (`89cd448` D4) + L4/L4.E purity (audit `L4_PURITY_AUDIT_TB7R_2026-05-02.md`: ZERO violations) — all in addition to the new TB-7R Deliverables B+C+D code.

## Cross-references

- TB-7R charter: `handover/tracer_bullets/TB-7R_charter_2026-05-01.md` §5
- L4 purity audit: `handover/audits/L4_PURITY_AUDIT_TB7R_2026-05-02.md`
- TB-7.6 commit: `c0ec514`
- TB-7.7 D1+D2 commit: `a39c31b`
- TB-7.7 D4 commit: `89cd448`
