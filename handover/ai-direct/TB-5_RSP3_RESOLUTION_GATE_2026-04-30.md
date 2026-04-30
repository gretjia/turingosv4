# TB-5 STEP_B Phase-0 Preflight — System-Emitted Resolution Gate + Challenge Bond Release

**Date**: 2026-04-30
**Status**: Atom 1 deliverable; line-grounded vs main HEAD `0b76307` (post charter v2 + book-keeping)
**Charter (binding)**: `handover/tracer_bullets/TB-5_charter_2026-04-30.md` DRAFT v2
**Directives (binding)**:
- `handover/directives/2026-04-30_TB5_VETO_redesign_directive.md` (substantive redesign authority)
- `handover/directives/2026-04-30_TB5_audit_mode_supplement.md` (audit-mode policy supplement; **Codex-only round-2 authorized**)
**Templated on**: `handover/ai-direct/TB-4_RSP2_ADMISSION_SURFACE_2026-04-30.md` (TB-4 preflight v3 shape; A1-patched)

---

## §0 Audit-mode policy for this preflight (per supplement)

Per `handover/directives/2026-04-30_TB5_audit_mode_supplement.md` (user authorization 2026-04-30 "不用等gemini，使用codex就可以"):

```text
TB-5 v2 STEP_B Phase-0 audit  →  Codex-only round 2 (single-auditor).
Strategic-tier Gemini unavailable (429 MODEL_CAPACITY_EXHAUSTED).
Degraded Gemini deliberately NOT invoked as substitute.
Codex full-fidelity verdict IS the ship-gate authority for this round.
```

The Codex-only verdict will land at `handover/audits/CODEX_TB_5_PHASE0_AUDIT_2026-04-30.md` with the verdict-file caveat header from supplement § 6 mandatorily included.

---

## §1 Why this preflight exists

Per CLAUDE.md "Code Standard" + STEP_B_PROTOCOL.md, any change to `src/state/sequencer.rs` (STEP_B-restricted per TB-2 P1-A) requires a parallel-branch experimental write with a Phase-0 necessity audit. TB-5 v2 also touches `src/state/typed_tx.rs` (institutional change per C-031: 1 new TypedTx variant + 4 new TransitionError variants + 2 new enum types) and `src/state/q_state.rs` (additive `status` on `ChallengeCase`) and `src/bottom_white/ledger/system_keypair.rs` (institutional: PinnedSystemPubkeys promoted from replay-side to dispatch-side; new `CanonicalMessage::ChallengeResolveSigning` variant).

This preflight pins **exact line refs against main HEAD `0b76307`** so that Phase-1c diff audit + Atom 8 self-audit can verify each change is at its declared site, no scope creep, and the **system-tx ingress barrier is constitutionally enforced** (charter v2 §4.2 + directive §6 binding).

---

## §2 Scope summary (binding to charter v2)

```text
Touched files (4):
  src/state/typed_tx.rs                       — +ChallengeResolveTx struct +
                                                  ChallengeResolution enum (NOT ChallengeStatus —
                                                  that lives in q_state.rs per Q4 amendment) +
                                                  signing payload + DOMAIN_SYSTEM_CHALLENGE_RESOLVE
                                                  + 4 new TransitionError variants + Display arms +
                                                  golden digests + HasSubmitter impl + TypedTx
                                                  variant + tx_kind arm
  src/state/q_state.rs                        — ChallengeCase: +status: ChallengeStatus
                                                  (additive serde-default; default=Open) +
                                                  ChallengeStatus enum DEFINED HERE (single
                                                  source of truth; sequencer.rs imports from
                                                  q_state per Q4 amendment) + Default impl
                                                  extension
  src/state/sequencer.rs                      — submit_agent_tx() + emit_system_tx() API split
                                                  + ingress classification for system-vs-agent
                                                    variants + SystemTxForbiddenOnAgentIngress
                                                    fail-closed gate at submit_agent_tx entry
                                                  + Sequencer.pinned_pubkeys field +
                                                    Sequencer::new constructor signature bump
                                                  + apply_one live signature verification step
                                                    for system variants (defense-in-depth)
                                                  + ChallengeResolve dispatch arm + state-root
                                                    domain const + helper
                                                  + rejection_class_for + public_summary_for
                                                    table extension
                                                  + ~12 new in-crate unit tests
  src/bottom_white/ledger/system_keypair.rs   — +CanonicalMessage::ChallengeResolveSigning([u8;32])
                                                  + sign_challenge_resolve() crate helper
                                                    (analog of sign_finalize_reward / sign_task_expire)
                                                  + canonical_digest() arm

  src/bottom_white/ledger/transition_ledger.rs  — TxKind +ChallengeResolve variant; cascading match
                                                  audits across all consumers

Cascade-only touch (per Codex round-2 Q4 amendment; binding):
  src/economy/monetary_invariant.rs            — MINIMAL CASCADE: add
                                                  TypedTx::ChallengeResolve(_) => Ok(()) arm
                                                  to assert_no_post_init_mint exhaustive match
                                                  (line 214-227); update K5-test fixture at
                                                  line 348-356 to include new variant. 5-holding
                                                  count UNCHANGED. total_supply_micro UNCHANGED.

Untouched (Phase-1c verifies absence of touch):
  src/bottom_white/ledger/rejection_evidence.rs  (no new RejectionClass variants)
  src/bottom_white/cas/*                          (no schema changes)
  src/kernel.rs / src/bus.rs / src/sdk/tools/wallet.rs   (no edits)

New files:
  tests/tb_5_system_ingress_barrier.rs         (TB-5.0 substrate tests; ~10 integration tests)
  tests/tb_5_challenge_resolve_surface.rs      (TB-5.1 resolution tests; ~12 integration tests)
  tests/tb_5_anti_drift.rs                     (TB-5.2 anti-drift CI scanner extension; ~6 tests)
  handover/audits/RECURSIVE_AUDIT_TB_5_2026-04-30.md (Atom 8 self-audit)
  handover/evidence/tb_5_smoke_2026-04-30/      (Atom 8 smoke evidence; non-blocking per directive § 4 Q4 inheritance)
```

---

## §3 Two-channel ingress API design (TB-5.0; charter v2 § 4.2)

### §3.1 Current (HEAD `0b76307`) Sequencer ingress shape

`src/state/sequencer.rs:863-882`:

```rust
pub async fn submit(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
    // K1: increment submit_id atomically; ALWAYS assigned at submit time
    // (whether or not the tx is later accepted).
    let submit_id = self.next_submit_id.fetch_add(1, Ordering::SeqCst);
    let envelope = SubmissionEnvelope { submit_id, tx };
    match self.queue_tx.try_send(envelope) {
        Ok(()) => Ok(SubmissionReceipt { submit_id }),
        Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => Err(SubmitError::QueueFull),
        Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => Err(SubmitError::QueueClosed),
    }
}
```

This is the **agent-trusted ingress** that the VETO targeted: it accepts any `TypedTx` variant including system-emitted ones (`ChallengeResolveTx` / `FinalizeRewardTx` / `TaskExpireTx` / `TerminalSummaryTx`), and dispatch never verifies their `system_signature`. After TB-5.0:

### §3.2 Atom-2 shape — `submit_agent_tx` (narrow + reject system variants)

```rust
/// TRACE_MATRIX TB-5 charter v2 § 4.2 — agent-only ingress.
/// Accepts ONLY agent-submitted variants; rejects system-emitted variants
/// at submit time with TransitionError::SystemTxForbiddenOnAgentIngress
/// before the queue receives the envelope. This is the constitutional
/// Anti-Oreo "agent ≠ direct state writer" boundary, structurally enforced.
pub async fn submit_agent_tx(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
    // Reject system-emitted variants pre-queue.
    match &tx {
        TypedTx::ChallengeResolve(_)
        | TypedTx::FinalizeReward(_)
        | TypedTx::TaskExpire(_)
        | TypedTx::TerminalSummary(_) => {
            return Err(SubmitError::SystemTxForbiddenOnAgentIngress);
        }
        TypedTx::Work(_)
        | TypedTx::Verify(_)
        | TypedTx::Challenge(_)
        | TypedTx::Reuse(_)
        | TypedTx::TaskOpen(_)
        | TypedTx::EscrowLock(_) => {} // agent variants — proceed
    }
    // K1 path unchanged.
    let submit_id = self.next_submit_id.fetch_add(1, Ordering::SeqCst);
    let envelope = SubmissionEnvelope { submit_id, tx };
    match self.queue_tx.try_send(envelope) {
        Ok(()) => Ok(SubmissionReceipt { submit_id }),
        Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => Err(SubmitError::QueueFull),
        Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => Err(SubmitError::QueueClosed),
    }
}
```

`SubmitError` gets a new variant `SystemTxForbiddenOnAgentIngress` (or this becomes a new top-level error type — see §3.5).

`Sequencer::submit` is renamed-and-aliased: the legacy `pub async fn submit(...)` body is changed to `self.submit_agent_tx(tx).await` (delegating; preserves test backward-compat).

### §3.3 Atom-3 shape — `emit_system_tx` (system-only; signs + verifies internally)

```rust
/// TRACE_MATRIX TB-5 charter v2 § 4.2 — system-only ingress.
/// Constructs the typed tx + signs internally with the runtime's
/// system_keypair + verifies via PinnedSystemPubkeys before pushing
/// to the queue. Can never be invoked with a forged signature
/// because it constructs the signature from the runtime's own keypair.
pub async fn emit_system_tx(
    &self,
    command: SystemEmitCommand,
) -> Result<SystemEmitReceipt, EmitSystemError> {
    // 1. Build the typed tx struct from the command.
    let tx = self.build_signed_system_tx(command)?;
    // 2. Defense-in-depth: verify against pinned pubkeys (sanity check that
    //    the just-signed signature passes verification under the pinned
    //    key for the current epoch).
    self.verify_system_tx_signature(&tx)?;
    // 3. Allocate emit_id (separate counter from submit_id; see §3.5).
    let emit_id = self.next_emit_id.fetch_add(1, Ordering::SeqCst);
    let envelope = SubmissionEnvelope { submit_id: emit_id, tx };
    // 4. Push to shared queue.
    match self.queue_tx.try_send(envelope) {
        Ok(()) => Ok(SystemEmitReceipt { emit_id }),
        Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => Err(EmitSystemError::QueueFull),
        Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => Err(EmitSystemError::QueueClosed),
    }
}
```

### §3.4 SystemEmitCommand schema

```rust
/// TRACE_MATRIX TB-5 charter v2 § 4.5 — high-level system-emit command.
/// Inputs that emit_system_tx accepts; the typed tx struct is constructed
/// + signed inside emit_system_tx, never by the caller.
#[derive(Debug, Clone)]
pub enum SystemEmitCommand {
    ChallengeResolve {
        target_challenge_tx_id: TxId,
        resolution: ChallengeResolution,
    },
    // Future RSP-3.2 / RSP-4 additions (not in TB-5):
    // FinalizeReward { ... }
    // TaskExpire { ... }
    // TerminalSummary { ... }
    // SlashTx { ... }  (RSP-3.2)
    // SettlementTx { ... }  (RSP-4)
}
```

### §3.5 Error variants (TB-5.0 + TB-5.1 new)

Existing `SubmitError` (sequencer.rs:583+):
```rust
#[derive(Debug)]
pub enum SubmitError {
    QueueFull,
    QueueClosed,
    SystemTxForbiddenOnAgentIngress,  // ← TB-5 NEW (renamed from charter v1 SystemSignatureForbiddenAtAgentSubmit)
}
```

NEW `EmitSystemError`:
```rust
#[derive(Debug)]
pub enum EmitSystemError {
    QueueFull,
    QueueClosed,
    /// Signing the constructed tx with the system keypair failed.
    SignatureConstruction(KeypairError),
    /// Verification of the just-signed signature failed (pinned-pubkey mismatch
    /// for the current epoch). Should not happen in production but defends
    /// against keypair/pubkey-pinning desync.
    InvalidSystemSignatureLive,
}
```

NEW `TransitionError` variants in `src/state/typed_tx.rs`:
- `SystemTxForbiddenOnAgentIngress` — for cases where a system variant somehow reaches dispatch (defensive; should be unreachable post-Atom 2). Maps to `L4ERejectionClass::PolicyViolation`.
- `ChallengeNotFound` — `target_challenge_tx_id` not in `challenge_cases_t`. Maps to `PolicyViolation`.
- `AlreadyResolved` — `case.status != Open` at resolve time. Maps to `PolicyViolation`.
- `InvalidSystemSignatureLive` — apply_one defense-in-depth signature verification failed. Maps to `PolicyViolation`.

`Sequencer.next_emit_id: AtomicU64` is added as a new field (parallel to `next_submit_id`); starts at 1.

### §3.6 SubmissionEnvelope shape (UNCHANGED)

`src/state/sequencer.rs:669+`:
```rust
pub struct SubmissionEnvelope {
    pub submit_id: u64,
    pub tx: TypedTx,
}
```

The shared queue uses this for both ingress paths. The `submit_id` field carries either an agent submit_id or a system emit_id (separate atomic counters; collisions impossible because `next_submit_id` and `next_emit_id` advance in their own namespaces — though they may happen to share numeric values, which is fine because the variant TYPE distinguishes origin at dispatch).

**Atom-2 design alternative considered + rejected**: adding `origin: SubmissionOrigin { Agent, System }` enum field. Rejected because dispatch_transition exhaustive-matches by variant TYPE which is already the canonical origin signal; an explicit origin tag would be redundant duplicate-source-of-truth.

---

## §4 PinnedSystemPubkeys integration (TB-5.0 substrate; Atom 3)

### §4.1 Current Sequencer construction

`src/state/sequencer.rs:831-857`:

```rust
pub fn new(
    cas: Arc<RwLock<CasStore>>,
    keypair: Arc<Ed25519Keypair>,
    epoch: SystemEpoch,
    writer: Arc<RwLock<dyn LedgerWriter>>,
    rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
    predicate_registry: Arc<PredicateRegistry>,
    tool_registry: Arc<ToolRegistry>,
    initial_q: QState,
    queue_capacity: usize,
) -> (Self, tokio::sync::mpsc::Receiver<SubmissionEnvelope>) {
    // ...
}
```

### §4.2 Atom-3 shape

```rust
pub fn new(
    cas: Arc<RwLock<CasStore>>,
    keypair: Arc<Ed25519Keypair>,
    epoch: SystemEpoch,
    writer: Arc<RwLock<dyn LedgerWriter>>,
    rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
    predicate_registry: Arc<PredicateRegistry>,
    tool_registry: Arc<ToolRegistry>,
    pinned_pubkeys: Arc<PinnedSystemPubkeys>,    // ← TB-5 NEW
    initial_q: QState,
    queue_capacity: usize,
) -> (Self, tokio::sync::mpsc::Receiver<SubmissionEnvelope>) {
    // pinned_pubkeys stored on self.pinned_pubkeys: Arc<PinnedSystemPubkeys>
    // self.next_emit_id: AtomicU64::new(1)
    // ...
}
```

For test fixtures: derive `pinned_pubkeys` from the same keypair used for signing — pin `self.keypair`'s public key under `epoch`. This way the pinned-pubkey verification is satisfied by-construction in tests.

For production: `pinned_pubkeys` comes from `genesis_payload.toml [system_pubkeys]` per existing `verify_system_pubkeys` machinery in system_keypair.rs:532.

### §4.3 New CanonicalMessage variant

`src/bottom_white/ledger/system_keypair.rs:225-253`:

```rust
pub enum CanonicalMessage {
    RejectedAttemptSummary(RejectedAttemptSummary),
    TerminalSummarySigning([u8; 32]),
    FinalizeRewardSigning([u8; 32]),
    TaskExpireSigning([u8; 32]),
    EpochRotationProof(EpochRotationProof),
    LedgerEntrySigning([u8; 32]),
    /// TRACE_MATRIX TB-5 charter v2 § 4.5 — challenge-resolve signing payload digest.
    /// Opaque [u8; 32] from `state::typed_tx::ChallengeResolveSigningPayload::canonical_digest()`.
    /// Same opaque-digest pattern as TerminalSummarySigning; avoids circular
    /// `system_keypair ↔ state` dependency.
    ChallengeResolveSigning([u8; 32]),  // ← TB-5 NEW
}
```

Plus a new arm in `canonical_digest()` (system_keypair.rs around line 480-498):

```rust
CanonicalMessage::ChallengeResolveSigning(digest) => {
    h.update(b"ChallengeResolveSigning");
    h.update(digest);
}
```

### §4.4 New crate-only signer

In the existing `predicate_runner` mod-style pattern, add a new crate-only signer for ChallengeResolveTx. Or simply expose `sign_challenge_resolve` as `pub(crate)` next to the existing system signers. (TB-5 atom 4 picks the simpler option.)

```rust
pub(crate) fn sign_challenge_resolve(
    keypair: &Ed25519Keypair,
    digest: [u8; 32],
) -> Result<SystemSignature, KeypairError> {
    sign_system_message_inner(
        keypair,
        &CanonicalMessage::ChallengeResolveSigning(digest),
    )
}
```

`Sequencer::build_signed_system_tx` consumes this:

```rust
fn build_signed_system_tx(
    &self,
    command: SystemEmitCommand,
) -> Result<TypedTx, EmitSystemError> {
    use crate::bottom_white::ledger::system_keypair::sign_challenge_resolve;
    match command {
        SystemEmitCommand::ChallengeResolve { target_challenge_tx_id, resolution } => {
            let q_snap = self.q.read().map_err(|_| EmitSystemError::SignatureConstruction(
                KeypairError::Internal("q lock poisoned".into())))?;
            let logical_t_for_id = self.next_logical_t.load(Ordering::SeqCst) + 1;
            let mut tx = ChallengeResolveTx {
                tx_id: TxId(format!("system-challenge-resolve-{}", logical_t_for_id)),
                parent_state_root: q_snap.state_root_t,
                target_challenge_tx_id,
                resolution,
                epoch: self.epoch,
                timestamp_logical: logical_t_for_id,
                system_signature: SystemSignature::from_bytes([0u8; 64]),  // placeholder
            };
            drop(q_snap);
            // sign payload
            let payload = tx.to_signing_payload();
            let digest = payload.canonical_digest();
            let sig = sign_challenge_resolve(&self.keypair, digest)
                .map_err(EmitSystemError::SignatureConstruction)?;
            tx.system_signature = sig;
            Ok(TypedTx::ChallengeResolve(tx))
        }
    }
}
```

### §4.5 Live signature verification call site (apply_one defense-in-depth) — AMENDED post Codex round-2 Q2 CHALLENGE

Per charter v2 § 4.3 + § 6 #28:

> system_signature is NOT a schema-only field. It MUST be live-verified at the ingress gate, OR constructed inside emit_system_tx from the system keypair.

The construction guarantee from § 3.3 covers ingress; the apply_one verification is **defense-in-depth** for any path that bypasses emit_system_tx (currently: none; future: hardened against accidental queue-bypass).

**Round-2 Codex Q2 amendment (binding)**:
- Stage 1.5 verification MUST be **exhaustive per-system-variant**, not just `ChallengeResolveSigning`. The four current system variants (`ChallengeResolve`, `FinalizeReward`, `TaskExpire`, `TerminalSummary`) each have their own `CanonicalMessage` arm; each must be matched correctly.
- Stage 1.5 verification failures MUST route through the **same L4.E rejection-evidence path** that dispatch failures use (`apply_one` lines 956-1024). Direct return without recording is forbidden — that bypass would contradict charter § 4.9 + this preflight § 4.5's own L4.E routing claim.

**Implementation: factor a `record_rejection` helper out of existing `apply_one` at `src/state/sequencer.rs:945-1024`** (the rejection-writer arm of the dispatch match). Both stage 1.5 sig failures AND dispatch failures call this helper before returning `ApplyError::Transition`.

```rust
// New helper extracted from apply_one rejection arm (sequencer.rs:945-1024).
// Records L4.E rejection evidence: CAS-puts canonical-encoded tx payload +
// diagnostic + appends RejectedSubmissionRecord. K1: NO logical_t advance,
// NO state_root_t advance. Same shape as the existing inline rejection arm.
fn record_rejection(
    &self,
    submit_id: u64,
    q_snapshot: &QState,
    tx: &TypedTx,
    transition_err: &TransitionError,
) -> Result<(), ApplyError> {
    let payload_bytes = canonical_encode(tx)
        .map_err(|e| ApplyError::PayloadEncode(e.to_string()))?;
    let creator = format!("sequencer.rejection_path.epoch-{}", self.epoch.get());
    let rejection_logical_t = self.next_logical_t.load(Ordering::SeqCst);
    let tx_payload_cid = {
        let mut cas_w = self.cas.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
        cas_w.put(&payload_bytes, ObjectType::ProposalPayload, &creator,
                  rejection_logical_t, Some("TypedTx.v1".to_string()))?
    };
    let diag_bytes = transition_err.to_string().into_bytes();
    let raw_diagnostic_cid = {
        let mut cas_w = self.cas.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
        Some(cas_w.put(&diag_bytes, ObjectType::Generic, &creator,
                       rejection_logical_t, Some("TransitionError.display.v1".to_string()))?)
    };
    let agent_id = tx.submitter_id()
        .unwrap_or_else(|| AgentId(SYSTEM_AGENT_ID_STR.to_string()));
    {
        let mut writer_w = self.rejection_writer.write()
            .map_err(|_| ApplyError::QStateLockPoisoned)?;
        writer_w.append_rejected(
            submit_id,
            q_snapshot.state_root_t,
            agent_id,
            tx.tx_kind(),
            tx_payload_cid,
            rejection_class_for(transition_err),
            raw_diagnostic_cid,
            public_summary_for(transition_err),
        );
    }
    Ok(())
}
```

```rust
// Stage 1.5 (TB-5 NEW; exhaustive per-variant): if tx is system-emitted,
// verify system_signature against pinned pubkeys for the current epoch.
// Fail-closed on mismatch — record L4.E + return ApplyError::Transition.
let system_check = system_message_for_verification(&tx);  // exhaustive match below
if let Some((message, sig, epoch)) = system_check {
    let valid = verify_system_signature(&sig, &message, epoch, &self.pinned_pubkeys);
    if !valid {
        let transition_err = TransitionError::InvalidSystemSignatureLive;
        // Route through SAME L4.E path as dispatch rejections per Q2 amendment.
        self.record_rejection(submit_id, &q_snapshot, &tx, &transition_err)?;
        return Err(ApplyError::Transition(transition_err));
    }
}
```

**Exhaustive per-variant helper** (returns `None` for agent variants which skip this stage; returns `Some((CanonicalMessage, SystemSignature, SystemEpoch))` for each system variant matched to its OWN signing-payload digest):

```rust
fn system_message_for_verification(
    tx: &TypedTx,
) -> Option<(CanonicalMessage, SystemSignature, SystemEpoch)> {
    match tx {
        TypedTx::ChallengeResolve(t) => {
            let digest = t.to_signing_payload().canonical_digest();
            Some((CanonicalMessage::ChallengeResolveSigning(digest),
                  t.system_signature, t.epoch))
        }
        TypedTx::FinalizeReward(t) => {
            let digest = t.to_signing_payload().canonical_digest();
            Some((CanonicalMessage::FinalizeRewardSigning(digest),
                  t.system_signature, t.epoch))
        }
        TypedTx::TaskExpire(t) => {
            let digest = t.to_signing_payload().canonical_digest();
            Some((CanonicalMessage::TaskExpireSigning(digest),
                  t.system_signature, t.epoch))
        }
        TypedTx::TerminalSummary(t) => {
            // TerminalSummaryTx schema lacks an `epoch` field per current src
            // (typed_tx.rs:341-351); use SystemEpoch::new(0) sentinel + verify
            // path tolerates this for now. Atom-3 implementation MAY revise
            // by reading epoch from the LedgerEntry envelope context if
            // available, but TB-5 minimum scope keeps the verification
            // call shape uniform across the four system variants.
            let digest = t.to_signing_payload().canonical_digest();
            Some((CanonicalMessage::TerminalSummarySigning(digest),
                  t.system_signature, SystemEpoch::new(0)))
        }
        // All agent-submitted variants — skip this stage.
        TypedTx::Work(_) | TypedTx::Verify(_) | TypedTx::Challenge(_)
        | TypedTx::Reuse(_) | TypedTx::TaskOpen(_) | TypedTx::EscrowLock(_) => None,
    }
}
```

The `TransitionError::InvalidSystemSignatureLive` is added to the `rejection_class_for` table at sequencer.rs:154 mapping to `L4ERejectionClass::PolicyViolation`, and to `public_summary_for` returning `Some("invalid_system_signature_live".into())`.

**Tests required (extends U28/I66 per Codex round-2 Q2 remediation)**:
- U28: `apply_one_rejects_system_variant_with_zero_signature_via_pinned_pubkey_check` — verify the L4.E row IS appended (not just Q-mutation absent).
- I66: `apply_one_rejects_zero_signature_system_variant_with_pinned_pubkey_check` — full integration: forge an envelope (test-internal queue-bypass harness) with `system_signature: SystemSignature::from_bytes([0u8; 64])` and verify L4.E row count grows by 1 + economic_state bit-identical pre/post.
- **NEW per-variant zero-signature coverage** (added per Q2 remediation): for each of `FinalizeRewardTx`, `TaskExpireTx`, `TerminalSummaryTx`, exercise the same scenario with their respective zero-signature envelopes; verify L4.E recording + class mapping. Test names:
  - `apply_one_rejects_zero_signature_finalize_reward`
  - `apply_one_rejects_zero_signature_task_expire`
  - `apply_one_rejects_zero_signature_terminal_summary`

---

## §5 ChallengeResolveTx schema (TB-5.1; Atom 4)

### §5.1 New struct in typed_tx.rs

Insert after EscrowLockTx (`src/state/typed_tx.rs:417` area):

```rust
// ────────────────────────────────────────────────────────────────────────────
// § 5c TB-5 RSP-3.0/3.1 system-emitted resolution surface — ChallengeResolveTx
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-5 charter v2 § 4.5 — system-emitted challenge resolution.
/// Cannot enter Q via agent ingress (submit_agent_tx rejects at line ~880);
/// must come through Sequencer::emit_system_tx which signs internally.
/// Released refunds challenger bond + sets ChallengeCase.status = Released.
/// UpheldDeferred sets ChallengeCase.status = UpheldDeferred; ChallengeCase
/// preserved (not removed) for future TB-6 slash routing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChallengeResolveTx {
    pub tx_id: TxId,                                //  1
    pub parent_state_root: Hash,                    //  2
    pub target_challenge_tx_id: TxId,               //  3 — keys challenge_cases_t lookup
    pub resolution: ChallengeResolution,            //  4
    pub epoch: SystemEpoch,                         //  5
    pub timestamp_logical: u64,                     //  6
    pub system_signature: SystemSignature,          //  7
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ChallengeResolution {
    Released = 0,
    UpheldDeferred = 1,
}

impl Default for ChallengeResolution { fn default() -> Self { Self::Released } }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChallengeResolveSigningPayload {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub target_challenge_tx_id: TxId,
    pub resolution: ChallengeResolution,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
}

impl ChallengeResolveSigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_SYSTEM_CHALLENGE_RESOLVE, self)
    }
}

const DOMAIN_SYSTEM_CHALLENGE_RESOLVE: &[u8] = b"turingosv4.system_sig.challenge_resolve.v1";

impl ChallengeResolveTx {
    pub fn to_signing_payload(&self) -> ChallengeResolveSigningPayload {
        ChallengeResolveSigningPayload {
            tx_id: self.tx_id.clone(),
            parent_state_root: self.parent_state_root,
            target_challenge_tx_id: self.target_challenge_tx_id.clone(),
            resolution: self.resolution,
            epoch: self.epoch,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl HasSubmitter for ChallengeResolveTx {
    fn submitter_id(&self) -> Option<AgentId> {
        None  // system-emitted; same shape as FinalizeRewardTx / TaskExpireTx
    }
}
```

### §5.2 TypedTx + TxKind cascading additions

`src/state/typed_tx.rs:739`:

```rust
pub enum TypedTx {
    Work(WorkTx),
    Verify(VerifyTx),
    Challenge(ChallengeTx),
    Reuse(ReuseTx),
    FinalizeReward(FinalizeRewardTx),
    TaskExpire(TaskExpireTx),
    TerminalSummary(TerminalSummaryTx),
    TaskOpen(TaskOpenTx),
    EscrowLock(EscrowLockTx),
    ChallengeResolve(ChallengeResolveTx),    // ← TB-5 NEW (10th variant)
}
```

`tx_kind()` arm (`:746`):
```rust
Self::ChallengeResolve(_) => TxKind::ChallengeResolve,
```

`HasSubmitter for TypedTx` arm (`:828`):
```rust
Self::ChallengeResolve(t) => t.submitter_id(),
```

`src/bottom_white/ledger/transition_ledger.rs` `TxKind` enum:
```rust
pub enum TxKind {
    Work, Verify, Challenge, Reuse, FinalizeReward, TaskExpire, TerminalSummary,
    TaskOpen, EscrowLock,
    ChallengeResolve,    // ← TB-5 NEW
}
```

Cascading exhaustive-match audits across all consumers (sequencer rejection_class_for / public_summary_for; transition_ledger replay_full_transition match; any test fixture constructing TxKind variants).

### §5.3 Golden digest constants

Add 2 new constants in `src/state/typed_tx.rs::tests`:

```rust
const EXPECTED_HEX_CHALLENGE_RESOLVE: &str = "<TB-5 fresh; computed first run>";
const EXPECTED_SIGNING_HEX_CHALLENGE_RESOLVE: &str = "<TB-5 fresh; computed first run>";
```

Plus 2 new tests `golden_challenge_resolve_tx_digest` + signing-payload digest extension.

---

## §6 ChallengeCase entry-shape additive (TB-5.1; Atom 5)

### §6.1 Current ChallengeCase

`src/state/q_state.rs:336-365` (post TB-4):

```rust
pub struct ChallengeCase {
    #[serde(default)] pub challenger: AgentId,
    #[serde(default = "MicroCoin::zero")] pub bond: MicroCoin,
    #[serde(default)] pub opened_at_round: u64,
    #[serde(default)] pub target_work_tx: TxId,
}
```

### §6.2 Atom-5 shape — AMENDED post Codex round-2 Q4 CHALLENGE

**Single-source-of-truth decision (binding per Q4 amendment)**: `ChallengeStatus` is defined ONLY in `src/state/q_state.rs` next to `ChallengeCase`. `src/state/sequencer.rs` and any tests that need it import via `use crate::state::q_state::ChallengeStatus;`. **No duplicate definition** in `typed_tx.rs` or elsewhere — preflight § 2 amended to reflect this.

```rust
// src/state/q_state.rs — single definition site for ChallengeStatus (TB-5 Q4 amendment).

/// TRACE_MATRIX TB-5 charter v2 § 4.4 + § 4.6 + § 4.7 — challenge case shape.
///
/// **TB-5 additive field**: `status: ChallengeStatus` records the resolution
/// outcome without removing the entry from challenge_cases_t. Default = Open.
/// Released zeros bond + flips status to Released (audit trail preserved).
/// UpheldDeferred preserves bond + flips status (TB-6 slash routing target).
/// Additive serde-default — pre-TB-5 serialized rows deserialize with status=Open.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChallengeCase {
    #[serde(default)] pub challenger: AgentId,
    #[serde(default = "MicroCoin::zero")] pub bond: MicroCoin,
    #[serde(default)] pub opened_at_round: u64,
    #[serde(default)] pub target_work_tx: TxId,
    #[serde(default)] pub status: ChallengeStatus,    // ← TB-5 NEW
}

/// TRACE_MATRIX TB-5 charter v2 § 4.4 — challenge resolution status.
/// Lives in q_state.rs (single source of truth per Q4 amendment).
/// Imported by sequencer.rs as `use crate::state::q_state::ChallengeStatus;`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ChallengeStatus {
    Open = 0,
    Released = 1,
    UpheldDeferred = 2,
}

impl Default for ChallengeStatus { fn default() -> Self { Self::Open } }

impl Default for ChallengeCase {
    fn default() -> Self {
        Self {
            challenger: AgentId::default(),
            bond: MicroCoin::zero(),
            opened_at_round: 0,
            target_work_tx: TxId::default(),
            status: ChallengeStatus::Open,
        }
    }
}
```

The 9-sub-field `EconomicState` invariant is preserved (no new sub-fields; only entry-shape additive, mirroring TB-4 ChallengeCase +target_work_tx pattern).

The 5-holding CTF invariant is preserved: `challenge_cases.bond` continues to be summed; on Released the entry's `bond` becomes 0 (entry stays; contributes 0 to CTF).

### §6.3 Required `monetary_invariant.rs` cascade (per Codex round-2 Q4 CHALLENGE)

Adding `TypedTx::ChallengeResolve` (10th variant) to the `TypedTx` enum **breaks** the exhaustive match in `assert_no_post_init_mint` at `src/economy/monetary_invariant.rs:214-227`. The minimum cascade required:

```rust
// src/economy/monetary_invariant.rs:214-227 — add ChallengeResolve arm.
pub fn assert_no_post_init_mint(tx: &TypedTx, q: &QState) -> Result<(), MonetaryError> {
    let is_post_init = q.state_root_t != Hash::ZERO;
    if !is_post_init {
        return Ok(());
    }
    match tx {
        TypedTx::Work(_)
        | TypedTx::Verify(_)
        | TypedTx::Challenge(_)
        | TypedTx::Reuse(_)
        | TypedTx::FinalizeReward(_)
        | TypedTx::TaskExpire(_)
        | TypedTx::TerminalSummary(_)
        | TypedTx::TaskOpen(_)
        | TypedTx::EscrowLock(_)
        | TypedTx::ChallengeResolve(_) => Ok(()),    // ← TB-5 NEW
    }
}
```

Plus update the K5-test fixture at `monetary_invariant.rs:348-356` to include `TypedTx::ChallengeResolve(ChallengeResolveTx::default())` for completeness coverage. (Note: this test already missed `TaskOpen + EscrowLock` from TB-3 — TB-5 may opportunistically fix that fixture-completeness gap, OR file as separate doc-debt; charter v2 amendment doesn't require it but it's a freebie.)

`total_supply_micro` (`monetary_invariant.rs:95-103`) is **UNCHANGED**: 5-holding count stays. ChallengeResolveTx mutations operate on existing holdings (balances + challenge_cases.bond).

---

## §7 ChallengeResolve dispatch arm (TB-5.1; Atom 5+6)

### §7.1 New state-root domain const + helper

Insert after CHALLENGE_ACCEPT_DOMAIN_V1 block (`src/state/sequencer.rs:107` area):

```rust
/// TRACE_MATRIX TB-5 charter v2 § 4.6 — ChallengeResolve-accept state-root domain.
pub(crate) const CHALLENGE_RESOLVE_DOMAIN_V1: &[u8] =
    b"turingosv4.challenge_resolve.accept.v1";

pub fn challenge_resolve_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(CHALLENGE_RESOLVE_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}
```

### §7.2 Dispatch arm body

Insert as a new arm in sequencer.rs `dispatch_transition` (after EscrowLock arm; new variant added at end of TypedTx enum so it's the last arm):

```rust
TypedTx::ChallengeResolve(resolve) => {
    // Step 1: parent-root match.
    if resolve.parent_state_root != q.state_root_t {
        return Err(TransitionError::StaleParent);
    }
    // Step 2: target ChallengeCase exists.
    let case = match q.economic_state_t.challenge_cases_t.0
        .get(&resolve.target_challenge_tx_id) {
        Some(c) => c.clone(),
        None => return Err(TransitionError::ChallengeNotFound),
    };
    // Step 3: idempotency — case must be Open at resolve time.
    if case.status != ChallengeStatus::Open {
        return Err(TransitionError::AlreadyResolved);
    }
    // Step 4: build q_next.
    let mut q_next = q.clone();
    match resolve.resolution {
        ChallengeResolution::Released => {
            // Step 4a: refund challenger.
            let cur = q.economic_state_t.balances_t.0
                .get(&case.challenger).copied()
                .unwrap_or(MicroCoin::zero());
            let new_bal = cur.micro_units() + case.bond.micro_units();
            q_next.economic_state_t.balances_t.0.insert(
                case.challenger.clone(),
                MicroCoin::from_micro_units(new_bal),
            );
            // Step 4b: zero bond + flip status (do NOT remove entry per directive § 7 Q6).
            let entry = q_next.economic_state_t.challenge_cases_t.0
                .get_mut(&resolve.target_challenge_tx_id)
                .expect("verified at step 2");
            entry.bond = MicroCoin::zero();
            entry.status = ChallengeStatus::Released;
        }
        ChallengeResolution::UpheldDeferred => {
            // Step 4c: marker only — flip status; bond preserved for TB-6 slash.
            let entry = q_next.economic_state_t.challenge_cases_t.0
                .get_mut(&resolve.target_challenge_tx_id)
                .expect("verified at step 2");
            entry.status = ChallengeStatus::UpheldDeferred;
            // bond stays > 0; challenger / opened_at_round / target_work_tx untouched
        }
    }
    // Step 5: monetary invariants.
    assert_no_post_init_mint(tx, q)
        .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
    assert_total_ctf_conserved(
        &q.economic_state_t,
        &q_next.economic_state_t,
        &[],
    )
    .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
    // Step 6: state_root advance via CHALLENGE_RESOLVE_DOMAIN_V1.
    q_next.state_root_t = challenge_resolve_accept_state_root(&q.state_root_t, tx);

    Ok((q_next, SignalBundle::default()))
}
```

### §7.3 CTF accounting check

Released path:
```text
balances_t[challenger]:        +bond
challenge_cases_t[case].bond:  -bond  (bond becomes 0)
delta sum:                     0  ← CTF round-trip closes
```

UpheldDeferred path:
```text
all 5 holdings unchanged → assert_total_ctf_conserved trivially passes
status field is NOT in any holding sum → invariant unaffected
```

### §7.4 rejection_class_for + public_summary_for extensions

`src/state/sequencer.rs:154-200`:

```rust
fn rejection_class_for(e: &TransitionError) -> L4ERejectionClass {
    use TransitionError as TE;
    use L4ERejectionClass as RC;
    match e {
        // ... existing arms ...
        TE::SystemTxForbiddenOnAgentIngress => RC::PolicyViolation,
        TE::ChallengeNotFound => RC::PolicyViolation,
        TE::AlreadyResolved => RC::PolicyViolation,
        TE::InvalidSystemSignatureLive => RC::PolicyViolation,
        // ... wildcard ...
    }
}

fn public_summary_for(e: &TransitionError) -> Option<String> {
    match e {
        // ... existing arms ...
        TransitionError::SystemTxForbiddenOnAgentIngress =>
            Some("system_tx_forbidden_on_agent_ingress".into()),
        TransitionError::ChallengeNotFound => Some("challenge_not_found".into()),
        TransitionError::AlreadyResolved => Some("already_resolved".into()),
        TransitionError::InvalidSystemSignatureLive => Some("invalid_system_signature_live".into()),
        _ => Some("policy_violation".into()),
    }
}
```

---

## §8 Test plan (per directive § 10 + charter v2 § 5.3 binding)

### §8.1 In-crate unit tests in `src/state/typed_tx.rs::tests` (T1-T5; ~5 tests)

- T1 `challenge_resolve_canonical_digest_is_deterministic`
- T2 `challenge_resolve_signing_payload_excludes_signature_field_count_6`
- T3 `golden_challenge_resolve_tx_digest`
- T4 `golden_challenge_resolve_signing_payload_digest`
- T5 `transition_error_display_covers_4_new_variants` (SystemTxForbiddenOnAgentIngress + ChallengeNotFound + AlreadyResolved + InvalidSystemSignatureLive)

### §8.2 In-crate unit tests in `src/state/sequencer.rs::tests` (U22-U33; ~12 tests)

TB-5.0 substrate (U22-U28):
- U22 `submit_agent_tx_rejects_challenge_resolve_pre_queue`
- U23 `submit_agent_tx_rejects_finalize_reward_pre_queue`
- U24 `submit_agent_tx_rejects_task_expire_pre_queue`
- U25 `submit_agent_tx_rejects_terminal_summary_pre_queue`
- U26 `submit_agent_tx_accepts_work_verify_challenge_taskopen_escrowlock_reuse`
- U27 `emit_system_tx_constructs_challenge_resolve_with_valid_system_signature`
- U28 `apply_one_rejects_system_variant_with_zero_signature_via_pinned_pubkey_check`

TB-5.1 dispatch (U29-U33):
- U29 `dispatch_challenge_resolve_released_zeros_bond_and_sets_status`
- U30 `dispatch_challenge_resolve_released_refunds_balance`
- U31 `dispatch_challenge_resolve_released_cannot_run_twice` (AlreadyResolved gate)
- U32 `dispatch_challenge_resolve_unknown_target_rejects` (ChallengeNotFound)
- U33 `dispatch_challenge_resolve_upheld_deferred_marker_only`

### §8.3 Integration tests — TB-5.0 substrate (`tests/tb_5_system_ingress_barrier.rs`; ~13 tests including per-variant zero-sig coverage)

Per directive § 10 TB-5.0 binding test list (AMENDED post Codex round-3 Q6 for full unification with charter §5.3 + preflight §10):
- I60 `agent_submit_rejects_challenge_resolve_tx`
- I61 `agent_submit_rejects_finalize_reward_tx`
- I62 `agent_submit_rejects_task_expire_tx`
- I63 `agent_submit_rejects_terminal_summary_tx`
- I64 `emit_system_tx_accepts_challenge_resolve_with_valid_signature`
- I65 `emit_system_tx_rejects_missing_signature` (forge a SystemEmitCommand path that produces an unsigned tx — should not be possible by API; defense via unit test that Sequencer::build_signed_system_tx always produces a signed result)
- I66 `apply_one_rejects_zero_signature_system_variant_with_pinned_pubkey_check` (defense-in-depth scenario: simulate a queue-bypass corrupted envelope; apply_one must reject with InvalidSystemSignatureLive AND **append L4.E row** per Q2 amendment)
  - **I66.a `apply_one_rejects_zero_signature_finalize_reward`** (per-variant zero-sig coverage per Codex round-2 Q2 remediation)
  - **I66.b `apply_one_rejects_zero_signature_task_expire`**
  - **I66.c `apply_one_rejects_zero_signature_terminal_summary`**
- I67 `legacy_submit_alias_delegates_to_submit_agent_tx_and_rejects_system_variants`
- I68 `agent_submit_then_emit_system_tx_share_queue_but_distinct_envelope_paths`
- I69 `submit_id_and_emit_id_advance_independently`

### §8.4 Integration tests — TB-5.1 resolution (`tests/tb_5_challenge_resolve_surface.rs`; ~14 tests including I88/I89 boundary)

Per directive § 10 TB-5.1 binding test list (AMENDED post Codex round-3 Q6 + round-2 Q8 optional):
- I70 `submit_challenge_resolve_released_appends_to_canonical_l4`
- I71 `released_refunds_bond` (full sequence: TaskOpen → EscrowLock → Work → Challenge → emit_system_tx ChallengeResolve{Released})
- I72 `released_conserves_ctf` (Σ holdings pre = post; bond zeroed; balances refunded)
- I73 `released_cannot_run_twice` (second resolve with same target → AlreadyResolved)
- I74 `released_unknown_challenge_rejected` (target not in challenge_cases_t → ChallengeNotFound)
- I75 `upheld_deferred_keeps_challenge_for_future_slash` (case.status=UpheldDeferred; bond preserved)
- I76 `upheld_deferred_no_balance_mutation` (economic_state_t bit-identical except status field)
- I77 `multi_challenger_resolve_independently` (two ChallengeCases; resolve one Released → other stays Open)
- I78 `released_does_not_release_solver_or_verifier_stakes` (charter § 4.8 boundary test)
- I79 `released_does_not_decrement_total_escrow` (charter § 4.8 boundary test)
- I80 `replay_invariants_hold_across_full_rsp3_1_surface` (extends TB-4 I41 to 7-tx-kind sequence)
- I81 `property_no_sequence_violates_total_ctf_conservation_with_resolve` (10-step deterministic mixed sequence including Released + UpheldDeferred + rejected admissions)
- **I88 `challenge_resolve_does_not_mutate_q_t_current_round`** (NEW per Codex round-2 Q8 optional improvement; charter § 4.10 boundary)
- **I89 `upheld_deferred_keeps_solver_verifier_stakes_byte_identical`** (NEW per Codex round-2 Q8 optional improvement; parallel to I78 boundary check for UpheldDeferred path)

### §8.5 Anti-drift tests (`tests/tb_5_anti_drift.rs`; 6 tests)

- I82 `no_slash_tx_in_src` (extends TB-4 I44 FORBIDDEN_VARIANTS with `SlashTx`)
- I83 `no_settlement_tx_in_src` (extends with `SettlementTx`)
- I84 `no_provisional_accept_tx_in_src` (extends with `ProvisionalAcceptTx`)
- I85 `no_reputation_update_tx_in_src` (extends with `ReputationUpdateTx`)
- I86 `four_anti_drift_renames_documented` (charter § 4.11 + § 6 #29-31 verify markers exist in source comments — a "philosophy preservation" test; reads charter and asserts the four renames are still in the document; soft test for documentation hygiene)
- I87 `no_p6_files_touched_in_tb5` (git diff scanner; ensures no h_vppu / MetaTape / experiments/minif2f_v4/* changes — uses `git diff main..HEAD --name-only` and asserts zero P6-pathed files)

Acceptance battery total: **~37 new TB-5 tests** (5 typed_tx unit T1-T5 + 12 sequencer in-crate U22-U33 + 17 integration I60-I89 + 3 per-variant zero-sig sub-tests under I66 [I66.a/b/c]). Target post-ship: **~608/608 cargo test green** (TB-4 baseline 571 + 37). Charter §5.3 mirrors this exactly per Codex round-2 + round-3 Q6 unification.

### §8.6 真实烟测 (per charter v2 § 5.4; non-blocking)

Same shape as TB-4 ship gate. `mathd_algebra_107` × oneshot × deepseek-chat with elevated MAX_TX. Expected `prompt_context_hash="a1f43584a17d1226"` bit-identical across **5 sessions** (TB-1/2/3/4/5).

Optional n1 SOLVED reproduction. Filed as supporting evidence; **non-blocking** per directive § 4 Q4 (Option A audit mode requires audit gate, not smoke).

---

## §9 STEP_B-protected files + line-budget per file

| File | Allowed touch | Phase-1c verification |
|---|---|---|
| `src/state/typed_tx.rs` | +ChallengeResolveTx struct + 2 new enums (ChallengeResolution + ChallengeStatus duplicate-or-import) + signing payload + DOMAIN_SYSTEM_CHALLENGE_RESOLVE + HasSubmitter + 4 new TransitionError variants + Display arms + golden rotations + 5 new tests | `git diff main..HEAD -- src/state/typed_tx.rs \| wc -l` ≤ 250 net add |
| `src/state/q_state.rs` | ChallengeCase: +status field + ChallengeStatus enum (additive serde-default) + Default impl extension | `git diff main..HEAD -- src/state/q_state.rs \| wc -l` ≤ 50 net add |
| `src/state/sequencer.rs` | +submit_agent_tx + +emit_system_tx + legacy submit alias + Sequencer.next_emit_id + Sequencer.pinned_pubkeys + new constructor signature + apply_one stage 1.5 verification + ChallengeResolve dispatch arm + state-root domain + helper + rejection_class_for / public_summary_for arms + 12 new in-crate tests + 4 new helper fns | `git diff main..HEAD -- src/state/sequencer.rs \| wc -l` ≤ 800 net add |
| `src/bottom_white/ledger/system_keypair.rs` | +CanonicalMessage::ChallengeResolveSigning variant + sign_challenge_resolve helper + canonical_digest arm | `git diff main..HEAD -- src/bottom_white/ledger/system_keypair.rs \| wc -l` ≤ 50 net add |
| `src/bottom_white/ledger/transition_ledger.rs` | TxKind +ChallengeResolve variant; cascading match audits | `git diff main..HEAD -- src/bottom_white/ledger/transition_ledger.rs \| wc -l` ≤ 30 net add |
| `src/economy/monetary_invariant.rs` | **MINIMAL CASCADE per Q4 amendment**: +ChallengeResolve arm in `assert_no_post_init_mint` exhaustive match (line 214-227) + K5-test fixture update (line 348-356). 5-holding count + total_supply_micro UNCHANGED. | `git diff main..HEAD -- src/economy/monetary_invariant.rs \| wc -l` ≤ 30 net add |
| `src/bottom_white/ledger/rejection_evidence.rs` | ZERO | empty |
| `src/kernel.rs` / `src/bus.rs` / `src/sdk/tools/wallet.rs` | ZERO | empty |
| `tests/tb_5_*.rs` | NEW files (3 — system_ingress_barrier + challenge_resolve_surface + anti_drift) | new |

---

## §10 Atom-by-atom deliverables (executable)

**AMENDED POST CODEX ROUND-2 Q7 CHALLENGE — Atoms 3 ↔ 4 swap + monetary_invariant.rs cascade**:

The original Atom 3 (emit_system_tx + apply_one stage 1.5) referenced `ChallengeResolveTx` / `ChallengeResolution` / `tx.to_signing_payload()` types that the original Atom 4 introduced. This created a compile-cycle: Atom 3 cannot land without Atom 4's types. Resolution per Codex Q7: **swap order so ABI lands first**.

Plus per Codex Q4: monetary_invariant.rs MUST cascade with the new TypedTx variant; folded into Atom 3 (the ABI-introducing atom).

| Atom | Files touched | Tests added | Commit subject |
|---|---|---|---|
| 0 (DONE @ 0b76307) | charter v2 + TB_LOG + NOTEPAD | none | TB-5 charter v2 ACTIVE |
| 1 (DONE @ 4c3414e) | preflight v1 + audit-mode supplement | none | Atom 1 — STEP_B Phase-0 preflight + Codex round-2 launch |
| 1.5 (NEW; pending round-3 narrow audit verdict) | preflight v1 → v2 amendment + charter v2 § 5.3 + § 4.11 amendments + Codex round-3 launch | none | Atom 1.5 — Round-2 CHALLENGE remediation (preflight v2 + charter §5.3/§4.11 amendments) |
| **2** (UNCHANGED) | sequencer.rs (submit_agent_tx + ingress barrier + legacy alias narrowing) + typed_tx.rs (SystemTxForbiddenOnAgentIngress + Display arm) | U22-U26, I60-I63, I67 | Atom 2 — TB-5.0 substrate ingress: submit_agent_tx + agent-ingress barrier |
| **3** (WAS Atom 4; per Q7 swap) | typed_tx.rs (ChallengeResolveTx + ChallengeResolution enum + signing payload + DOMAIN_SYSTEM_CHALLENGE_RESOLVE + HasSubmitter + TypedTx variant + tx_kind arm + canonical_hash + golden digest stubs) + q_state.rs (ChallengeCase +status field + **ChallengeStatus enum DEFINED HERE per Q4 amendment**) + transition_ledger.rs (TxKind +ChallengeResolve) + **monetary_invariant.rs (MINIMAL CASCADE per Q4: assert_no_post_init_mint exhaustive arm + K5-test fixture)** | T1-T4 | Atom 3 — TB-5 ABI: ChallengeResolveTx + ChallengeStatus + monetary_invariant cascade |
| **4** (WAS Atom 3; per Q7 swap; now depends on Atom 3 ABI) | sequencer.rs (Sequencer.pinned_pubkeys field + new constructor + emit_system_tx + apply_one stage 1.5 with **exhaustive per-variant + record_rejection helper extracted from current apply_one:945-1024** per Q2 amendment) + system_keypair.rs (CanonicalMessage::ChallengeResolveSigning variant + sign_challenge_resolve + canonical_digest arm) + typed_tx.rs (InvalidSystemSignatureLive + Display arm) | U27-U28, T5, I64-I66, I68-I69, **per-variant zero-sig coverage (FinalizeReward + TaskExpire + TerminalSummary)** | Atom 4 — TB-5.0 emit_system_tx + apply_one stage 1.5 (exhaustive per-variant; L4.E-routed) |
| 5 (UNCHANGED) | sequencer.rs (ChallengeNotFound + AlreadyResolved variants + ChallengeResolve Released dispatch arm + state-root domain + helper) + typed_tx.rs (Display arms for ChallengeNotFound + AlreadyResolved) | U29-U32, I70-I74, I78-I79 | Atom 5 — ChallengeResolve Released dispatch arm |
| 6 (UNCHANGED) | sequencer.rs (ChallengeResolve UpheldDeferred arm) | U33, I75-I77, **I88 current_round_not_mutated**, **I89 upheld_deferred_byte_identical_stakes** | Atom 6 — ChallengeResolve UpheldDeferred dispatch arm + boundary tests |
| 7 (UNCHANGED) | tests/tb_5_anti_drift.rs (4 new FORBIDDEN_VARIANTS) + tests/tb_5_challenge_resolve_surface.rs (replay + property) | I80-I87 | Atom 7 — Replay + property + anti-drift CI |
| 8 (UNCHANGED) | handover/audits/RECURSIVE_AUDIT_TB_5_2026-04-30.md + handover/evidence/tb_5_smoke_2026-04-30/ | none (audit + smoke) | Atom 8 — Codex ship audit + recursive self-audit + smoke evidence |
| Ship | (--no-ff merge) + book-keeping | none | TB-5 SHIPPED — merge experiment/tb5-rsp3-resolution-gate |

Total acceptance: **~37 new TB-5 tests** (5 typed_tx + 12 sequencer in-crate + ~17 integration including I78/I79 boundary + I82-I87 anti-drift + **I88/I89 optional boundary-tests added per Codex Q8 + per-variant zero-sig coverage for FinalizeReward/TaskExpire/TerminalSummary added per Q2/Q5 amendment**). Target post-ship: ~608 / ~608 cargo test green (TB-4 baseline 571 + 37).

**Compile-green sequencing** (post Q7 swap):
- Atom 2 introduces `submit_agent_tx` + ingress barrier — uses no ChallengeResolveTx types.
- Atom 3 introduces ChallengeResolveTx ABI + ChallengeStatus + monetary_invariant cascade — adds the variant + closes the exhaustive-match cascade in one commit.
- Atom 4 implements emit_system_tx + apply_one stage 1.5 — now the ChallengeResolveTx types it constructs ALREADY EXIST from Atom 3.
- Atoms 5-6 implement dispatch arms.
- Atoms 7-8 finalize testing + audit.

Per `feedback_phased_checkpoint`: each atom must be compile-green and adds its named tests in red→green order. Trust Root manifest rehash (R-014; non-sudo per R-018) at every state/*.rs touching atom (Atoms 3-6).

---

## §11 Resolved design questions

### §11.1 Atom-1 design questions (closed by directive 2026-04-30)

| Q | Question | Resolution |
|---|---|---|
| Q1 | Two-channel ingress: shared queue vs separate queues? | **Shared queue, distinct entry-point fns + counters**. Variant TYPE is the canonical origin signal at dispatch (no separate origin tag). § 3.6. |
| Q2 | emit_system_tx input shape: TypedTx vs higher-level command? | **SystemEmitCommand enum** — emit_system_tx constructs + signs internally; caller never carries a forged signature. § 3.4. |
| Q3 | Live signature verification call site: dispatch vs apply_one? | **apply_one stage 1.5** (between snapshot and dispatch). Defense-in-depth atop the constructive emit_system_tx guarantee. § 4.5 (amended Q2/round-2). |
| Q4 | Sequencer::submit alias: keep or remove? | **Keep, narrow + reject system variants**. Atom 2. § 3.2. |
| Q5 | submit_id and emit_id: shared counter or separate? | **Separate counters**: `next_submit_id` (existing) for agent path; `next_emit_id` (new) for system path. Both push to shared queue. § 3.6. |
| Q6 | PinnedSystemPubkeys source for tests: derive from runtime keypair? | **Yes** — tests pin `self.keypair`'s public key under `epoch`. Verification by-construction. § 4.2. |
| Q7 | ChallengeResolve dispatch: fail-closed behavior on AlreadyResolved? | **Reject with TransitionError::AlreadyResolved**. `case.status != Open` is idempotency guard. § 7.2. |

### §11.2 Codex round-2 CHALLENGE remediations (closed 2026-04-30 by `feedback_elon_mode_policy` cap-exception auto-execute on determinate-best surgical patches)

Source: `handover/audits/CODEX_TB_5_PHASE0_AUDIT_2026-04-30.md`. Conservative single-auditor verdict per audit-mode supplement.

| Codex Q | CHALLENGE substance | Remediation applied |
|---|---|---|
| Q2 (apply_one verification) | Stage 1.5 sketch only mentioned ChallengeResolveSigning + comment "or per-variant arm"; failure path direct-returned bypassing L4.E rejection-evidence writer | § 4.5 rewritten with **exhaustive `system_message_for_verification` helper** matching all 4 system variants (ChallengeResolve / FinalizeReward / TaskExpire / TerminalSummary) to their respective `CanonicalMessage` arms; **`record_rejection` helper** factored out of existing `apply_one:945-1024` and called from BOTH stage 1.5 + dispatch failure paths before `ApplyError::Transition` return |
| Q4 (monetary_invariant cascade) | Adding TypedTx::ChallengeResolve breaks exhaustive match in `assert_no_post_init_mint:214-227`; ChallengeStatus dual-defined risk | § 2 + § 6.2 + § 6.3 (NEW) + § 9: `monetary_invariant.rs` allowed minimal cascade (+1 arm + fixture update); ChallengeStatus single-defined in `q_state.rs`; sequencer.rs imports it |
| Q6 (test matrix drift) | Charter §5.3 ~30 tests vs preflight §8 ~33 tests; numbering I45-I58 vs I60-I87; name mismatches | Preflight §8 retained as canonical source-of-truth; charter v2 §5.3 amended in same commit to mirror preflight §8 exactly (including new I88/I89 boundary tests + per-variant zero-sig coverage). Total ~37 tests; target post-ship ~608/608. |
| Q7 (atom executability) | Atom 3 referenced ChallengeResolveTx types from Atom 4 → compile cycle; monetary_invariant cascade omitted | § 10 atom plan **swapped Atoms 3 ↔ 4**: new Atom 3 = ABI (ChallengeResolveTx + ChallengeStatus + monetary_invariant cascade); new Atom 4 = emit_system_tx + apply_one stage 1.5 (now compile-green because ABI exists from Atom 3) |
| Q5 (system_signature ≠ schema-only enforcement) | Carried Q2 incompleteness | Q2 remediation applies; § 4.5 + tests (per-variant zero-sig coverage) close this |
| Q8 (charter §4 → preflight) | §4.3 + §4.4 incomplete via Q2/Q4 | Q2 + Q4 remediations close these |

**Optional improvements applied** (per Codex round-2 verdict § "Optional Improvements"):
- New test I88: `challenge_resolve_does_not_mutate_q_t_current_round` — explicit assertion that `q.q_t.current_round` is bit-identical pre/post a `ChallengeResolveTx{Released}` accept (charter v2 § 4.10 boundary).
- New test I89: `upheld_deferred_keeps_solver_verifier_stakes_byte_identical` — parallel to I78's Released boundary check, but for UpheldDeferred. Asserts `stakes_t` is bit-identical pre/post.
- Charter v2 § 4.11 cite fix applied in same amendment commit.

---

## §12 Forbidden file touches (CI-verifiable)

Atom 1 commits this preflight only; Atoms 2-7 land code. Phase-1c diff audit verifies these touch budgets per § 9.

Notable enforcements (CI-tested at TB-5.2 anti-drift extension):
- `tests/tb_4_rsp2_admission_surface.rs::no_no_stake_tx_or_verifier_bond_tx_variant_in_src` (TB-4 I44) extends FORBIDDEN_VARIANTS with `SlashTx` / `SettlementTx` / `ProvisionalAcceptTx` / `ReputationUpdateTx` (per directive § 5.1).
- `tests/tb_5_anti_drift.rs::no_p6_files_touched_in_tb5` (NEW) — git diff scanner confirms no h_vppu / MetaTape / experiments/minif2f_v4/* changes in TB-5 atoms.

---

## §13 Cross-references

- Charter v2: `handover/tracer_bullets/TB-5_charter_2026-04-30.md`
- Substantive directive: `handover/directives/2026-04-30_TB5_VETO_redesign_directive.md`
- Audit-mode supplement: `handover/directives/2026-04-30_TB5_audit_mode_supplement.md`
- Round-1 merged verdict: `handover/audits/DUAL_AUDIT_TB_4_SHIP_TB_5_CHARTER_VERDICT_2026-04-30.md`
- Codex round-1 verdict: `handover/audits/CODEX_TB_4_SHIP_TB_5_CHARTER_AUDIT_2026-04-30.md`
- TB-4 preflight (template): `handover/ai-direct/TB-4_RSP2_ADMISSION_SURFACE_2026-04-30.md`
- TB-4 charter (post A1 patch; ship-record alignment with shipped code): `handover/tracer_bullets/TB-4_charter_2026-04-30.md`
- STEP_B protocol: `STEP_B_PROTOCOL.md`
- WP-vs-Roadmap reconciliation memory: `feedback_wp_vs_roadmap_reconciliation`
- 9-phase roadmap directive: `handover/directives/2026-04-29_9_phase_roadmap.md`

---

## §14 Codex audit launch history

### §14.1 Round 2 (DONE @ 2026-04-30)

- **Subjects**: charter v2 (commit `0b76307`) + preflight v1 (commit `4c3414e`)
- **Verdict**: `handover/audits/CODEX_TB_5_PHASE0_AUDIT_2026-04-30.md` — **CHALLENGE** (not VETO; substrate sound; 4 substantive amendments needed)
- **Per-Q**: Q1 PASS / Q2 CHALLENGE / Q3 PASS / Q4 CHALLENGE / Q5 CHALLENGE / Q6 CHALLENGE / Q7 CHALLENGE / Q8 CHALLENGE
- **Verification anchors**: cargo PASS=571/0, Trust Root SHA match, anti-drift grep zero, TB-3 bridge invariant 2/0 — all GREEN
- **Codex's own follow-up recommendation**: "Round-3 narrowing warranted after amendment, but narrow it to Q2, Q4, Q6, and Q7 only."

### §14.2 Round 3 (PLANNED; narrow per Codex round-2 recommendation)

After this preflight v2 amendment commits:

```text
Audit subjects:
  - handover/tracer_bullets/TB-5_charter_2026-04-30.md (charter v2 — amended §5.3 + §4.11)
  - handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md (preflight v2 — amended §2/§4.5/§6/§9/§10/§11)

Audit lens (NARROW; round 3):
  - Q2: verify §4.5 amendment is fail-closed exhaustively per-variant +
    record_rejection helper extraction is correctly scoped
  - Q4: verify §6.3 monetary_invariant cascade is minimal (just exhaustive
    arm + fixture; no 5-holding count change; no total_supply_micro change)
  - Q6: verify charter §5.3 ↔ preflight §8 test matrix is now identical
    (counts, IDs, names) and post-ship target reconciled
  - Q7: verify Atoms 3 ↔ 4 swap closes compile cycle; monetary_invariant
    cascade folded into Atom 3

Out-of-scope for round 3 (already PASS in round 2; do NOT re-audit):
  - Q1 ingress-barrier soundness
  - Q3 dispatch correctness
  - (other PASS items)

Verdict file path:
  handover/audits/CODEX_TB_5_PHASE0_R3_AUDIT_2026-04-30.md

Mandatory verdict-header caveat (per audit-mode supplement § 6):
  "Audit Mode: Codex-only single-auditor per directive supplement
   2026-04-30_TB5_audit_mode_supplement.md..."

Format:
  Per-Q (Q2/Q4/Q6/Q7 only) PASS / CHALLENGE [reason] / VETO
  If all 4 PASS → Atom 2 implementation cleared
  If any CHALLENGE → user authorization required (round-cap=2 already
    consumed; round 3 is the cap-exception per `feedback_elon_mode_policy`
    auto-execute on determinate-best surgical patches)
```
