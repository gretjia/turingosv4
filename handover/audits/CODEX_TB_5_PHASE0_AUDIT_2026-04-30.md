**Audit Mode**: Codex-only (single-auditor) per directive supplement
2026-04-30 (handover/directives/2026-04-30_TB5_audit_mode_supplement.md).
Gemini strategic-tier (gemini-2.5-pro or stronger) was unavailable at
audit time due to MODEL_CAPACITY_EXHAUSTED 429 errors on the
cloudcode-pa.googleapis.com endpoint. Degraded Gemini was deliberately
NOT invoked as substitute per parent directive § 4 Q4 ('不要把 degraded
Gemini 当作完整战略审计').

This Codex verdict IS the ship-gate authority for the audited subject.
There is no second strategic auditor to merge against. Per
feedback_dual_audit_conflict conservative-merge, single Codex VETO /
CHALLENGE / PASS controls. If strategic-tier Gemini becomes available
post-ship, an opportunistic supplemental verdict may be appended but
does NOT override this round's verdict.

## Q1 - Ingress-Barrier Soundness (TB-5.0 Substrate): PASS

The v2 redesign closes the specific round-1 VETO path at the planned public ingress. The current `TypedTx` enum has nine variants at HEAD: `Work`, `Verify`, `Challenge`, `Reuse`, `FinalizeReward`, `TaskExpire`, `TerminalSummary`, `TaskOpen`, and `EscrowLock` (`src/state/typed_tx.rs:739-755`). The current system-emitted variants with `SystemSignature` fields are `FinalizeRewardTx`, `TaskExpireTx`, and `TerminalSummaryTx` (`src/state/typed_tx.rs:292-323`, `src/state/typed_tx.rs:325-337`, `src/state/typed_tx.rs:339-363`). The TB-5 planned fourth system variant is `ChallengeResolveTx` with `system_signature` (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md` §5.1, lines 418-433). Preflight §3.2 rejects exactly those four variants before queue insertion (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:117-141`), matching charter v2 §4.2's system-variant list (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:94-108`).

`ReuseTx` is intentionally not included in the system-forbidden list: charter v2 §4.2 lists `ReuseTx` among agent-accepted variants (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:96-101`), and the preflight mirrors that (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:126-131`). The fact that `ReuseTx::submitter_id()` returns `None` (`src/state/typed_tx.rs:804-807`) is therefore not, by itself, a system-emitted classification.

The construction guarantee for `emit_system_tx` is sound for TB-5's active system emission path: preflight §3.3 accepts a `SystemEmitCommand`, builds the typed tx internally, verifies it, then queues it (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:148-176`). The command only lets the caller supply `target_challenge_tx_id` and `resolution` (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md` §3.4, lines 181-189). The preflight's `build_signed_system_tx` sketch derives `tx_id`, `parent_state_root`, `epoch`, and `timestamp_logical` from sequencer state, installs a placeholder signature, computes the signing payload digest, signs internally, and only then returns `TypedTx::ChallengeResolve` (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:345-371`). A caller cannot pass an arbitrary `system_signature` through this API.

I found no public third queue-write path in the current source. `queue_tx` is a private `Sequencer` field (`src/state/sequencer.rs:795-817`), current public `submit` is the only public queue sender and constructs `SubmissionEnvelope` before `try_send` (`src/state/sequencer.rs:859-875`), and `TuringBus::submit_typed_tx` delegates to `seq.submit(tx)` (`src/bus.rs:135-141`). `try_apply_one` only drains an existing receiver and calls `apply_one` (`src/state/sequencer.rs:903-910`); it is not a public queue insertion path. Preflight §3.2 also correctly narrows the legacy `Sequencer::submit` alias to delegate to `submit_agent_tx` (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:144-146`).

No remediation required for Q1.

## Q2 - Defense-In-Depth At `apply_one` (TB-5.0 Verification): CHALLENGE

The planned verification uses the right primitive shape. The existing verifier is `verify_system_signature(sig, message, epoch, pinned_pubkeys) -> bool` and fails closed on missing epoch key, invalid public key bytes, or Ed25519 verification failure (`src/bottom_white/ledger/system_keypair.rs:501-517`). Preflight §4.5 calls it with `&sig`, `&message`, `epoch`, and `&self.pinned_pubkeys` (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:388-397`). Preflight §4.3 also correctly adds `CanonicalMessage::ChallengeResolveSigning([u8; 32])` and a `canonical_digest()` arm (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:297-324`).

However, the stage 1.5 sketch is under-specified for all system variants. It says `system_signature_digest_of` handles "system variants" (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:401-403`), but the executable snippet constructs `CanonicalMessage::ChallengeResolveSigning(digest)` with only a comment "or per-variant arm" (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:390-394`). Current system-signing domains already distinguish `FinalizeRewardSigning`, `TaskExpireSigning`, and `TerminalSummarySigning` in `CanonicalMessage` (`src/bottom_white/ledger/system_keypair.rs:235-243`) and in `canonical_digest()` (`src/bottom_white/ledger/system_keypair.rs:472-483`). A forward-compatible stage must be an exhaustive per-variant match over `ChallengeResolve`, `FinalizeReward`, `TaskExpire`, and `TerminalSummary`, each mapped to its own `CanonicalMessage`.

There is also an L4.E routing bug in the plan. Current `apply_one` records L4.E only inside the `dispatch_transition` error arm (`src/state/sequencer.rs:945-1024`). Preflight §4.5 inserts signature verification before dispatch and returns directly on failure (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:385-397`). That direct return is fail-closed for Q mutation, but it bypasses the existing rejection-evidence writer. This contradicts preflight §4.5's own statement that `InvalidSystemSignatureLive` routes through L4.E (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:401-403`) and charter v2 §4.9's error-class mapping (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:246-258`).

Concrete remediation: amend preflight §4.5 and §7.4 to factor a local `record_rejection(submit_id, &q_snapshot, &tx, TransitionError)` helper out of current `apply_one` lines 956-1024, then call it from both stage 1.5 verification failures and dispatch failures before returning `ApplyError::Transition`. Replace the placeholder `CanonicalMessage::ChallengeResolveSigning(digest) // or per-variant arm` with an explicit exhaustive helper:

```rust
match tx {
    TypedTx::ChallengeResolve(t) => (..., CanonicalMessage::ChallengeResolveSigning(t.to_signing_payload().canonical_digest()), ...),
    TypedTx::FinalizeReward(t) => (..., CanonicalMessage::FinalizeRewardSigning(t.to_signing_payload().canonical_digest()), ...),
    TypedTx::TaskExpire(t) => (..., CanonicalMessage::TaskExpireSigning(t.to_signing_payload().canonical_digest()), ...),
    TypedTx::TerminalSummary(t) => (..., CanonicalMessage::TerminalSummarySigning(t.to_signing_payload().canonical_digest()), ...),
    _ => None,
}
```

Extend U28/I66 to assert both `InvalidSystemSignatureLive` and one L4.E row, and add per-variant zero-signature coverage for `FinalizeReward`, `TaskExpire`, and `TerminalSummary`.

## Q3 - ChallengeResolve Dispatch Correctness (TB-5.1): PASS

The planned `Released` arm is conservation-correct. It looks up the target challenge case, rejects missing targets, rejects already-resolved cases, credits the challenger balance by `case.bond`, sets `case.bond = MicroCoin::zero()`, and flips `case.status = ChallengeStatus::Released` (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md` §7.2, lines 624-658). This matches charter v2 §4.6's required mutations and CTF round-trip (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:186-214`). The existing monetary invariant sums `challenge_cases_t.bond` as the fifth holding (`src/economy/monetary_invariant.rs:95-103`, `src/economy/monetary_invariant.rs:118-137`), so setting the bond to zero and crediting balances closes the debit/credit pair.

The planned `UpheldDeferred` arm is marker-only: it mutates only `case.status = ChallengeStatus::UpheldDeferred` and preserves the bond and all other fields (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:659-666`). This matches charter v2 §4.7 (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:216-235`). Since status is not a holding in the current 5-holding sum (`src/economy/monetary_invariant.rs:95-103`), `assert_total_ctf_conserved` is trivially preserved.

Idempotency is sound for both outcomes because the status gate runs before the outcome match (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:629-641`). Once either `Released` or `UpheldDeferred` flips status away from `Open`, a second resolve rejects as `AlreadyResolved`. `ChallengeNotFound` also rejects before `q_next` is created (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:629-634`). Current `apply_one` dispatch rejections are written to L4.E without mutating Q (`src/state/sequencer.rs:945-1024`), subject to the Q2 remediation for pre-dispatch signature failures.

No remediation required for Q3.

## Q4 - Schema + Q-Shape Additivity (Charter §4.4 + Preflight §5 + §6): CHALLENGE

The planned schema additions are mostly additive. Preflight §5.1 adds `ChallengeResolveTx`, `ChallengeResolution`, the six-field signing payload, `DOMAIN_SYSTEM_CHALLENGE_RESOLVE`, and `HasSubmitter for ChallengeResolveTx -> None` (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:407-480`). Preflight §5.2 adds the tenth `TypedTx` variant plus `tx_kind()` and `HasSubmitter for TypedTx` arms, and adds `TxKind::ChallengeResolve` in the L4 ledger enum (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:482-520`). This is the correct cascade relative to current `TypedTx` and `TxKind` definitions (`src/state/typed_tx.rs:745-772`, `src/bottom_white/ledger/transition_ledger.rs:48-67`).

`ChallengeCase.status` is planned as an additive serde-default field with `Default = Open`, and `ChallengeStatus` has `Open | Released | UpheldDeferred` under `#[repr(u8)]` (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md` §6.2, lines 552-590). The current `ChallengeCase` has only `challenger`, `bond`, `opened_at_round`, and `target_work_tx` (`src/state/q_state.rs:347-357`), so the proposed `#[serde(default)] pub status` is a genuine entry-shape additive migration. The 9-sub-field `EconomicState` shape remains unchanged (`src/state/q_state.rs:151-166`), and the 5-holding CTF invariant remains based on balances, escrows, stakes, claims, and `challenge_cases_t.bond` (`src/economy/monetary_invariant.rs:95-103`, `src/economy/monetary_invariant.rs:118-137`).

The blocking issue is that the preflight incorrectly marks `src/economy/monetary_invariant.rs` as zero-touch (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:71-75`, `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:818-819`). Adding `TypedTx::ChallengeResolve` will make the current exhaustive match in `assert_no_post_init_mint` non-compiling unless a `TypedTx::ChallengeResolve(_) => Ok(())` arm is added (`src/economy/monetary_invariant.rs:209-227`). The existing invariant test table also needs to include the new variant if it keeps claiming all K5 variants (`src/economy/monetary_invariant.rs:342-360`). This is not a change to the 5-holding count, but it is a required cascade edit.

Secondary remediation: preflight §2 currently says `typed_tx.rs` gets `ChallengeStatus` while `q_state.rs` also gets `ChallengeStatus enum re-import or local def` (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:39-49`). Amend this to define exactly one `ChallengeStatus` type, preferably in `q_state.rs` next to `ChallengeCase`, and import that same type in dispatch. Do not create duplicate `typed_tx::ChallengeStatus` and `q_state::ChallengeStatus` types.

Concrete remediation: amend preflight §2, §9, and §10 to allow a minimal `src/economy/monetary_invariant.rs` touch: add the `ChallengeResolve` arm to `assert_no_post_init_mint` and update the invariant unit test fixture list. Keep `total_supply_micro` unchanged.

## Q5 - Four Anti-Drift Renames (Charter §4.11): CHALLENGE

Three of the four anti-drift renames are operationally expressed. `resolve != judge` holds because the planned `ChallengeResolve` dispatch arm does not call a predicate evaluator or inspect `counterexample_cid`; it only checks parent root, challenge existence, status, and the supplied resolution enum (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:621-681`). This matches charter v2 §4.10 (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:260-267`). `release != settlement` holds because `Released` only credits the challenger bond and updates the challenge case, with no task finalization or reward payout (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:641-658`; charter v2 §4.8 at `handover/tracer_bullets/TB-5_charter_2026-04-30.md:237-245`). Preflight I78 explicitly checks solver/verifier stakes are not released (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:785-786`). `UpheldDeferred != slash` holds because the planned arm flips only status and preserves the bond (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:659-666`; charter v2 §4.7 at `handover/tracer_bullets/TB-5_charter_2026-04-30.md:216-235`).

The fourth rename, `system_signature != schema-only field`, carries the Q2 challenge. Charter v2 §4.3 requires live verification or internal construction (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:116-124`), and preflight §3.3/§4.5 supplies the intended construction plus verification (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:148-176`, `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:377-403`). But the apply-side verification must be amended to exhaustively cover all system variants and route invalid signatures into L4.E before this rename is fully enforced.

Concrete remediation: same as Q2, plus add tests that prove `FinalizeReward`, `TaskExpire`, `TerminalSummary`, and `ChallengeResolve` cannot reach dispatch with a bad signature through any queue-bypass harness.

## Q6 - Forbidden Lines + Test Count Cross-Validation: CHALLENGE

Random spot-check sample from charter v2 §6 used forbidden lines #28, #24, #30, #32, and #31 (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:377-386`). Results:

- #28, no `system_signature` field without live verification: intended enforcement exists in preflight §4.5 (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:377-403`), but it inherits the Q2 challenge for exhaustive per-variant verification and L4.E routing.
- #24, no solver YES stake release on `ChallengeResolveTx`: enforced by construction, since the planned `Released` arm mutates balances and the challenge case only, and the `UpheldDeferred` arm mutates status only (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:641-666`); I78 covers solver/verifier stake non-release (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:785-786`).
- #30, `release != settlement`: enforced by the same dispatch construction and I78/I79 boundary tests (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:777-788`).
- #32, no P5/P6/P7/P8 ship-gate work: preflight §2 and §9 constrain touched files to TB-5 state/ledger/test surfaces and mark unrelated files untouched (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:71-82`, `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:809-821`); I87 adds a P6 diff scanner (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:796-797`).
- #31, `UpheldDeferred != slash`: enforced by construction and I75/I76 marker-only tests (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:782-784`).

The blocking Q6 issue is test-list drift between charter §5.3 and preflight §8. Charter §5.3 targets about 30 tests and post-ship 601/601 (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:309-354`), while preflight §8 targets about 33 tests and post-ship 604/604 (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:732-799`). The sets do not match:

- Charter names `challenge_resolve_canonical_digest_deterministic` and `challenge_resolve_signing_payload_excludes_signature` (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:311-323`); preflight names `challenge_resolve_canonical_digest_is_deterministic` and `challenge_resolve_signing_payload_excludes_signature_field_count_6` (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:734-740`).
- Charter includes `emit_system_tx_rejects_wrong_epoch_signature` (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:319-323`); preflight §8 does not list that test and instead lists `apply_one_rejects_zero_signature_system_variant_with_pinned_pubkey_check`, `legacy_submit_alias_delegates_to_submit_agent_tx_and_rejects_system_variants`, and `submit_id_and_emit_id_advance_independently` (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:750-772`).
- Charter's integration numbering is I45-I58 (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:318-352`); preflight's integration numbering is I60-I87 (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:760-797`).
- Preflight adds boundary tests I78-I79 and anti-drift tests I82-I87 that are not present as named tests in charter §5.3 (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:785-797`; charter list at `handover/tracer_bullets/TB-5_charter_2026-04-30.md:333-352`).

Concrete remediation: make charter §5.3 and preflight §8 a single stable test matrix with identical IDs, names, count, and target total. Keep the richer preflight list if desired, but amend the charter to match it exactly.

## Q7 - Atom Plan Executability (Preflight §10): CHALLENGE

The high-level dependency intent is correct: substrate before resolution, ABI before dispatch, Released before UpheldDeferred, anti-drift after implementation, audit last (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:406-423`; `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:825-840`). I found no need for a VETO-level redesign of the overall direction.

There is a concrete atom-order cycle. Preflight §10 says Atom 3 implements `emit_system_tx`, `Sequencer.pinned_pubkeys`, stage 1.5 verification, and the `CanonicalMessage` signer additions, while Atom 4 adds `ChallengeResolveTx`, `ChallengeResolution`, the signing payload, and `DOMAIN_SYSTEM_CHALLENGE_RESOLVE` (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:831-834`). But the Atom 3 sketch in §4.4 constructs `ChallengeResolveTx`, uses `SystemEmitCommand::ChallengeResolve`, uses `ChallengeResolution`, calls `tx.to_signing_payload()`, and returns `TypedTx::ChallengeResolve(tx)` (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:342-374`). Those types and methods do not exist until Atom 4. A compile-green Atom 3 cannot land before Atom 4 as currently written.

There is also a compile executability gap from Q4: preflight §9 says `src/economy/monetary_invariant.rs` remains zero-touch (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:818-819`), but adding a tenth `TypedTx` variant requires updating the exhaustive `assert_no_post_init_mint` match (`src/economy/monetary_invariant.rs:209-227`).

Concrete remediation: either reorder Atom 4 before Atom 3, or split Atom 4 so the minimal `ChallengeResolveTx` ABI, `ChallengeResolution`, signing payload, and `TypedTx`/`TxKind` variants land before Atom 3's `emit_system_tx`. Also amend §9/§10 to include the minimal `monetary_invariant.rs` cascade. After that, Atom 5 before Atom 6 and Atom 7 after Atoms 2-6 remain sequenceable (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:834-837`).

## Q8 - Charter v2 §4 Decision Blocks To Preflight Operational Expression: CHALLENGE

Operational expression coverage:

- Charter §4.1, `ChallengeResolveTx` first-class system-only variant, is expressed in preflight §5.1-§5.2 (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:62-92`; `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:407-520`).
- Charter §4.2, two-channel ingress, is expressed in preflight §3.2-§3.6 (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:94-115`; `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:109-246`).
- Charter §4.3, live signature verification, is expressed in preflight §4.2-§4.5 but incomplete due to Q2 (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:116-124`; `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:272-403`).
- Charter §4.4, 9-sub-field EconomicState and 5-holding CTF, is expressed in preflight §6.2 but incomplete due to the missing `monetary_invariant.rs` cascade from Q4 (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:126-137`; `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:535-595`).
- Charter §4.5, ChallengeResolve schema, is expressed in preflight §5.1 (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:138-184`; `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:407-480`).
- Charter §4.6 and §4.7, `Released` and `UpheldDeferred`, are expressed in preflight §7.2-§7.3 (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:186-235`; `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:621-697`).
- Charter §4.8, no Solver/Verifier state mutation, is expressed by dispatch construction and I78/I79 (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:237-245`; `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:641-666`, `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:785-786`).
- Charter §4.9, `SystemTxForbiddenOnAgentIngress`, is expressed in preflight §3.2, §3.5, and §7.4 (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:246-258`; `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:109-146`, `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:199-230`, `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:699-728`).
- Charter §4.10, no window-close, round-tick, or counterexample evaluation, is expressed by absence in the dispatch sketch (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:260-267`; `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:621-681`). Optional improvement: add an explicit named test for no `current_round` mutation.
- Charter §4.11, four anti-drift renames, is expressed in preflight §8.5 and by the dispatch construction, but incomplete for `system_signature != schema-only` until Q2 is fixed (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:268-277`; `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:790-797`).

Concrete remediation: apply the Q2/Q4/Q6/Q7 amendments, then refresh Q8 cross-references so every charter §4 block points to exact preflight code/test rows.

## Overall TB-5 v2 Verdict: CHALLENGE

Conservative rollup: Q1 PASS, Q2 CHALLENGE, Q3 PASS, Q4 CHALLENGE, Q5 CHALLENGE, Q6 CHALLENGE, Q7 CHALLENGE, Q8 CHALLENGE. There is no VETO in this round because I did not find a fifth system variant missing from the rejection list, a public path by which an agent can push a system-emitted variant into the queue after the planned alias narrowing, or a dispatch design that would accept forged `ChallengeResolveTx` without signature verification. The original VETO substance is closed in direction.

The preflight is not ready to gate Atom 2 implementation until the CHALLENGE items are amended.

## Top-3 Must-Fix Items

1. Fix apply-side signature verification: explicit per-system-variant `CanonicalMessage` mapping, plus L4.E recording for stage 1.5 failures before returning `InvalidSystemSignatureLive`.
2. Fix atom executability: resolve the Atom 3 before Atom 4 `ChallengeResolveTx` dependency, and permit the required `src/economy/monetary_invariant.rs` cascade edit.
3. Reconcile charter §5.3 and preflight §8 into one exact test matrix with identical IDs, names, count, and post-ship target.

## Optional Improvements

- Add an explicit integration test that `ChallengeResolveTx` does not mutate `q.q_t.current_round`, complementing charter v2 §4.10 (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:260-267`).
- Add an explicit `UpheldDeferred` test that solver and verifier stakes remain byte-identical, parallel to I78's `Released` boundary check (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:785-786`).
- In charter §4.11, replace "encoded as forbidden lines in §5 + as CI tests in §4.7" with the actual sections. The forbidden list is in charter §6 (`handover/tracer_bullets/TB-5_charter_2026-04-30.md:366-388`), and the current preflight anti-drift tests are in §8.5 (`handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md:790-797`).

## Verification-Anchor Results

`cargo test --workspace 2>&1 | grep '^test result:' | awk '{p+=$4; f+=$6} END {print "PASS="p, "FAIL="f}'`

```text
PASS=571 FAIL=0
```

Matches expected TB-4 baseline `PASS=571 FAIL=0`.

`sha256sum src/state/sequencer.rs src/state/typed_tx.rs src/state/q_state.rs`

```text
783e2291c56871a028b860a6fe323d3adc5c00c0e82626cbb05dd40928196bfb  src/state/sequencer.rs
9e0044486d3e53ff4768c4bfbb0e19c873f6f29ff142b3b5e130888f0bcd1593  src/state/typed_tx.rs
9d1ce20dd607f252efa4b6617451228bd9e5f2327b9e4c4a6ce1e3596bdab76a  src/state/q_state.rs
```

Matches `genesis_payload.toml` trust-root entries for those files (`genesis_payload.toml:227-229`).

`grep -rn 'NoStakeTx\|VerifierBondTx\|ChallengeStakeTx\|VerifierStakeTx' src/`

```text
<zero hits; grep exited 1>
```

Matches expected TB-4 anti-drift result.

`cargo test --test tb_3_bridge_deletion_invariant 2>&1 | tail -5`

```text
test scanner_positive_control_finds_known_match ... ok
test bridge_pattern_does_not_resurrect_in_src ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

Matches expected 2 PASS / 0 FAIL.

## Round-3 Narrowing Recommendation

Warranted after amendment, but narrow it to Q2, Q4, Q6, and Q7 only. Specifically re-audit: exhaustive system-signature verification plus L4.E routing, atom sequencing/compile-green file budgets, `monetary_invariant.rs` cascade handling, and the unified charter/preflight test matrix.
