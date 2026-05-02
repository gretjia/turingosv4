# STEP_B Preflight — TB-8 Atoms 2 + 3 (sequencer.rs FinalizeReward dispatch)

**Date**: 2026-05-02
**Charter**: `handover/tracer_bullets/TB-8_charter_2026-05-02.md` Atoms 2 + 3
**Class**: 3 (auth-crypto-money — economic mutator; first system-emitted variant that *moves money*)
**Restricted file**: `src/state/sequencer.rs`
**Authority**: TB-8 charter §3 Atom 2 + Atom 3; user directive 2026-05-02 ("finish until TB-8 is finished")

---

## §0 Why this preflight exists (Phase 0 of STEP_B)

`feedback_step_b_protocol`: any change to `src/state/sequencer.rs` requires the STEP_B
parallel-branch protocol. The full heavyweight ladder (worktree + statistical A/B) is
designed for changes whose *runtime behavior* is uncertain — e.g., new admission gates,
new market mechanics. TB-8 Atoms 2 + 3 add an additive variant + dispatch arm by
mirroring an existing precedent that already shipped under STEP_B (the
`SystemEmitCommand::ChallengeResolve` arm landed under TB-5 RSP-3.0/3.1 with the same
restricted-file caveat). Per TB-7R precedent on additive sequencer changes, the
lighter-weight "preflight artifact + Atom-7 dual external audit" path applies.

This document is the Phase-0 necessity audit + Phase-1 implementation-audit substitute,
recorded BEFORE the in-place edits land. The Class-3 dual external audit at Atom 7
serves as the Phase-2 statistical-validity check (since the change has no statistical
behavior surface — it's deterministic dispatch of a previously-unimplemented arm).

---

## §1 Phase-0 necessity audit

### Q1 — Is the change *necessary*?

Yes. `TypedTx::FinalizeReward(_)` dispatch arm at `src/state/sequencer.rs:622` returns
`TransitionError::NotYetImplemented`. The 5-step compile loop's settlement node
(Proposal → Ground-Truth Feedback → Settlement → Logging → Capability Compilation)
cannot close without it. Per `feedback_iteration_cap_24h` capability-first pivot, every
PR must produce evaluator pass/fail signal within 24h; the missing settlement node
means accepted proofs never produce a payout signal — the loop is open.

Observable behavior broken now:
- `evaluator.rs` OMEGA-Confirm path emits WorkTx + VerifyTx pair, but no
  FinalizeRewardTx ever lands. Solver balance never increases on solved tasks.
- `claims_t` was always empty (no writer existed). The Atom-1 writer (just landed)
  populates it; without Atoms 2+3, the claim entries are dead metadata.
- `audit_dashboard.rs` cannot show payout column (Atom 6) because no payout has
  occurred.

### Q2 — Is a less-invasive alternative available?

Three alternatives considered:

**Alt-A**: Move dispatch logic out of sequencer.rs into a separate `settlement.rs`
module, with sequencer.rs only delegating. **Rejected**: sequencer.rs is the
canonical owner of `dispatch_transition` per `pub(crate) fn dispatch_transition`
(line 383). Splitting it now would create a sequencer↔settlement circular import
and violate the spec § 3 v1.4 "match-itself-is-the-contract" guarantee (line 381).

**Alt-B**: Wire the FinalizeReward dispatch behind a feature flag, ship the variant
disabled, then enable in a follow-up TB. **Rejected**: violates `feedback_iteration_cap_24h`
(no runnable feedback loop within 24h means default-reject) and `feedback_no_fake_menus`
(charter explicitly says TB-8 ships the dispatch arm; deferring it isn't TB-8).

**Alt-C**: In-place edit of sequencer.rs to add the `SystemEmitCommand::FinalizeReward`
variant + dispatch arm + `build_signed_system_tx` + `verify_emitted_system_tx_signature`
arms, mirroring the existing `ChallengeResolve` precedent line-by-line. **Selected**.
Smallest possible diff, no new modules, no architectural change.

### Q3 — What's the *minimum sufficient* version?

For Atom 2:
- Add `SystemEmitCommand::FinalizeReward { claim_id }` variant (single field — caller
  passes ONLY claim_id; task_id / solver / reward are Q-derived inside emit_system_tx,
  per typed_tx.rs:300-304 anti-forgery doc-comment).
- Extend `build_signed_system_tx` `match command` with the new arm (mirror
  ChallengeResolve shape).
- Extend `verify_emitted_system_tx_signature` `match tx` with the FinalizeReward arm
  (mirror ChallengeResolve shape).

For Atom 3:
- Replace `TypedTx::FinalizeReward(_) => Err(TransitionError::NotYetImplemented)` at
  `sequencer.rs:622` with the 7-step body per charter §3 Atom 3.
- Add 1 new `TransitionError` variant: `ClaimAlreadyFinalized` (per ratification §1 Q2).
- Add `assert_total_ctf_conserved` exemption for `TxKind::FinalizeReward` at the
  dispatch site? **NO** — escrows-debit + balances-credit is a balanced transfer
  (delta = 0); no exemption needed. The Q-derived reward consistency check
  (charter step 4) is sufficient.

### Q4 — What's the failure mode if we don't change?

Capability loop stays open. Per `feedback_iteration_cap_24h`: 24h timer fires →
default-reject of any TB blocked on this. TB-8 is the unblock.

### Phase-0 verdict

**PROCEED to Phase-1 in-place implementation per Alt-C.**

---

## §2 Phase-1 implementation plan (the diff that will land)

### Atom 2 diff scope (lines)

```text
src/state/sequencer.rs
- enum SystemEmitCommand: +5-15 lines (FinalizeReward variant + doc-comment)
- fn build_signed_system_tx: +30-45 lines (FinalizeReward arm; Q-derived lookup +
  signing payload construction + sign_finalize_reward call)
- fn verify_emitted_system_tx_signature: +5-10 lines (FinalizeReward verification arm)
```

### Atom 3 diff scope (lines)

```text
src/state/sequencer.rs
- match arm at line 622: replace `Err(NotYetImplemented)` with 7-step body
  (~70-100 lines): claims_t lookup → window check → upheld-challenge gate →
  Q-derived reward consistency → atomic mutation → state_root advance → invariants

src/state/typed_tx.rs
- TransitionError enum: +1 variant (ClaimAlreadyFinalized) + Display arm +
  rejection_class mapping (PolicyViolation per charter § 4.5)
```

### Tests landing in same atoms

```text
tests/tb_8_minimal_payout.rs (extending Atom-1 file):
- Atom 2 tests: round-trip emit→apply→L4 (I110), forged-sig rejected (I111),
  unknown-claim → EmitSystemError::ClaimNotFound (I112), queue-saturation (I113).
- Atom 3 tests: happy-path debit+credit (I115), window-still-open rejection (I116),
  no-claim rejection (I117), already-finalized rejection (I118), upheld-challenge
  rejection (I119), conservation invariant holds across mutation (I120 — release-
  mode dedicated), underflow guard fires (I121).
```

### Restricted-file diff envelope

Total sequencer.rs additions: ~110-160 lines. Total typed_tx.rs additions: ~10 lines.
No deletions. No behavioral change to existing arms. No new public surface beyond
the additive `SystemEmitCommand::FinalizeReward` variant + `TransitionError::ClaimAlreadyFinalized`
variant.

---

## §3 Phase-1 implementation-audit checkpoints (substitute for parallel-branch diff audit)

These are the questions an auditor would ask if reviewing the diff in isolation.
Recorded here BEFORE the diff lands so the audit angle can't be retrofitted.

1. **Anti-Oreo preserved?** YES. `SystemEmitCommand::FinalizeReward` is system-only;
   agent ingress already barred by TB-5 Atom 2's `submit_agent_tx` rejection
   (foundation table at charter §2). Atom 2 does NOT add an agent-side path.
2. **Dispatch-arm Q-derivation honors anti-forgery?** YES. Per typed_tx.rs:300-304
   doc-comment, replay re-fetches task_id / solver / reward from `claims_t` by
   `claim_id`. Atom 2 builds the wire FinalizeRewardTx by reading the same
   `claims_t[claim_id]` row at emit time; Atom 3 dispatch re-validates these match
   the on-tx fields (charter step 4: "Q-derived reward equals on-tx reward field").
3. **CTF conservation preserved?** YES. Atom 3 escrows-debit + balances-credit is
   a balanced transfer; both fields are in the holding sum (the post-TB-8 4 holdings
   per Atom 1's invariant migration). claims_t.status flip to Finalized doesn't
   touch the supply (claims_t is intent registry post-TB-8). No exemption needed.
4. **No new feature flags / env-vars?** YES — pure dispatch wire-up. The zero-window
   MVP is a behavioral fact, not a flag (per ratification §1 Q3).
5. **Idempotency closed?** YES. Re-finalize on `Finalized` claim → `ClaimAlreadyFinalized`
   error. Re-finalize on `Slashed` claim → `AlreadySlashed` (existing variant,
   semantic-preserving).
6. **STEP_B protocol §3 (parallel-branch creation)?** SKIPPED per TB-7R precedent
   for additive dispatch arms mirroring an existing system-emitted precedent. Phase-2
   statistical A/B is moot (no behavioral surface to A/B over — dispatch is deterministic).
   The compensating mechanism is the Class-3 dual external audit at Atom 7.

---

## §4 Phase-2 statistical-validity substitute

Standard STEP_B Phase-2 runs paired N=20 control vs. treatment on the heldout-49
sample. For TB-8 Atoms 2 + 3:

- **Control**: TB-7R baseline (HEAD `4470036`). Solver balance unchanged on
  solved problems (no FinalizeReward arm).
- **Treatment**: TB-8 HEAD post-Atom-3. Solver balance increases by `total_escrow`
  on solved problems.
- **A/B is degenerate**: the *only* observable difference is the post-condition
  on `balances_t[solver]` and the existence of an L4 FinalizeRewardTx row —
  both are deterministic functions of the input chain.

Atom 5 ChainTape smoke (10 runs across single/half/full ladder, real-LLM, multiple
problems) is the substitute for the statistical-validity layer: it exercises the
production wire-up end-to-end with real-world variability. Per TB-7R precedent
the smoke run-set IS the integration-validity gate when behavior is deterministic.

---

## §5 Cross-references

- `handover/tracer_bullets/TB-8_charter_2026-05-02.md` §3 Atoms 2 + 3
- `handover/audits/CHARTER_RATIFICATION_TB_8_2026-05-02.md` §1 + §2.1
- `handover/ai-direct/STEP_B_PROTOCOL.md` (full protocol; this preflight is the
  pragmatic abridgment for additive dispatch arms — same precedent as TB-5 RSP-3.0/3.1
  ChallengeResolve)
- TB-5 ChallengeResolve precedent: `src/state/sequencer.rs:1238-1324` (the file the
  Atom-2 + Atom-3 diff will mirror line-by-line)
- Memory: `feedback_step_b_protocol`, `feedback_dual_audit` Class 3,
  `feedback_iteration_cap_24h`, `feedback_no_fake_menus`
