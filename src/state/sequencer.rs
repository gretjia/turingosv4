//! L4 Sequencer + dispatch_transition (CO1.7-impl A2 + A3).
//!
//! Spec authority:
//! - `handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` § 3 (Sequencer
//!   pseudocode, K1 dual-counter, K3 head_t deferred, C3 sign API)
//! - `handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` § 8
//!   (dispatch_transition exhaustive enum match; K5 Slash dropped)
//!
//! Single-writer per (runtime_repo, run_id). Per spec § 5.2.1.
//!
//! **Stub state (this atom)**: every per-kind transition returns
//! `TransitionError::NotYetImplemented`; CO1.7.5 (downstream atom) fills the
//! bodies. The structural correctness of the apply path (snapshot → dispatch →
//! CAS put → sign → root fold → commit → Q_t mutation) is locked by the
//! impl + tests here; what's left is per-kind transition logic.
//!
//! /// TRACE_MATRIX § 5.2.1 + § 8 — L4 sequencer single-writer + dispatch.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use sha2::{Digest, Sha256};

use crate::bottom_white::cas::schema::ObjectType;
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::bottom_white::ledger::system_keypair::{
    transition_ledger_emitter, Ed25519Keypair, KeypairError, SystemEpoch,
};
use crate::bottom_white::ledger::rejection_evidence::{
    RejectionClass as L4ERejectionClass, RejectionEvidenceWriter,
};
use crate::bottom_white::ledger::transition_ledger::{
    append, canonical_encode, LedgerEntry, LedgerEntrySigningPayload, LedgerWriter,
    LedgerWriterError,
};
use crate::bottom_white::tools::registry::ToolRegistry;
use crate::economy::monetary_invariant::{
    assert_no_post_init_mint, assert_read_is_free, assert_total_ctf_conserved,
};
use crate::state::q_state::{AgentId, Hash, QState, TxId};
use crate::state::typed_tx::{HasSubmitter, SignalBundle, TransitionError, TypedTx};
use crate::top_white::predicates::registry::PredicateRegistry;

// ────────────────────────────────────────────────────────────────────────────
// TB-2 — WorkTx-accept state-root domain (preflight v3 §3.4 + P1-1 r2)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3-S3: TB-2 interim WorkTx-accept state-root domain.
///
/// Real patch semantics for `q_next.state_root_t` land in P5; until then
/// TB-2 advances the state root deterministically with this domain string
/// concatenated against `q.state_root_t` and the canonical hash of the
/// accepted WorkTx. Distinct from the TB-1 toy domain
/// `b"turingosv4.l4_state_root.v1"` used by `AcceptedLedger` at
/// `src/economy/ledger.rs:350, :357` (TB-1 RSP-0 primitive vs production
/// state-root mutator separation).
pub(crate) const WORKTX_ACCEPT_DOMAIN_V1: &[u8] = b"turingosv4.worktx.accept.v1";

/// TRACE_MATRIX FC3-S3: TB-2 canonical hash helper for a `TypedTx`.
///
/// Defined locally (not in `bottom_white::ledger::transition_ledger`) because
/// `canonical_hash(tx)` is NOT a generic existing helper there — only
/// `canonical_encode` is — and TB-2 wants a single short call site that
/// includes domain separation. Codex r2 P1-2.
pub(crate) fn worktx_canonical_hash(tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(b"turingosv4.worktx.canonical_hash.v1");
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

/// TRACE_MATRIX FC3-S3: TB-2 interim state-root mutator on WorkTx accept.
///
/// `q_next.state_root_t = sha256(WORKTX_ACCEPT_DOMAIN_V1 ‖ q.state_root_t.0
/// ‖ worktx_canonical_hash(tx).0)`. P5 replaces this with real patch
/// semantics; until then this is the deterministic monotonic mutation
/// asserted by U3 / I9.
///
/// Public single-item surface for the TB-2 accept-side state-root contract.
/// Integration tests in `tests/tb_2_runtime_boundary.rs` (e.g. I9) use this
/// helper directly to recompute the expected post-accept hash WITHOUT
/// re-implementing the WORKTX_ACCEPT_DOMAIN_V1 / worktx_canonical_hash
/// composition by hand. The composing primitives stay `pub(crate)` so the
/// public surface is a single semantic helper, not the raw building blocks
/// (Phase-1c r1 Codex P0-1 remediation).
pub fn worktx_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let work_digest = worktx_canonical_hash(tx);
    let mut h = Sha256::new();
    h.update(WORKTX_ACCEPT_DOMAIN_V1);
    h.update(prev.0);
    h.update(work_digest.0);
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

// ────────────────────────────────────────────────────────────────────────────
// TB-2 Atom 4 — rejection-path helpers (preflight v3 §3.5 + §3.7)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3-S3: TB-2 sentinel `agent_id` for rejected submissions
/// whose `HasSubmitter::submitter_id()` returns `None` (system-emitted
/// variants — none on the WorkTx arm in TB-2; reserved for future TBs).
///
/// `RejectedSubmissionRecord.agent_id: AgentId` (NOT `Option<AgentId>`) per
/// `rejection_evidence.rs:90`. The string content is internal-only and never
/// crosses the agent boundary — only `public_summary` does, per `:89-90`.
pub(crate) const SYSTEM_AGENT_ID_STR: &str = "__system__";

/// TRACE_MATRIX FC3-S3: TB-2 `TransitionError → L4ERejectionClass` mapping
/// (preflight v3 §3.7). Closed by enumeration via the documented table even
/// though the `match` uses `_` for the 19-variant tail: WorkTx-arm-reachable
/// variants are explicit; non-WorkTx-arm variants fall through to
/// `PolicyViolation` per Codex r2 P0-4 sanction.
fn rejection_class_for(e: &TransitionError) -> L4ERejectionClass {
    use TransitionError as TE;
    use L4ERejectionClass as RC;
    match e {
        TE::AcceptancePredicateFailed(_)
        | TE::VerificationPredicateFailed(_)
        | TE::SettlementPredicateFailed(_) => RC::PredicateFailed,
        TE::EscrowMissing => RC::EscrowMissing,
        TE::MonetaryInvariantViolation => RC::InvariantViolation,
        // Non-WorkTx-arm variants documented per §3.7 mapping table — should
        // not occur on the WorkTx arm; conservative sentinel preserves L4.E
        // append correctness if a future TB adds new variants.
        _ => RC::PolicyViolation,
    }
}

/// TRACE_MATRIX FC3-S3: TB-2 agent-facing summary string for an L4.E record.
///
/// Returns a small, predicate-id-stripped class label so private predicate
/// identities never leak (TB-1 §1.4 "Opaque" discipline). The wildcard arm
/// matches the §3.7 mapping policy and is the documented sentinel for
/// non-WorkTx-arm variants per Codex r2 P0-4.
fn public_summary_for(e: &TransitionError) -> Option<String> {
    match e {
        TransitionError::StaleParent => Some("stale_parent_root".into()),
        TransitionError::StakeInsufficient => Some("stake_insufficient".into()),
        TransitionError::EscrowMissing => Some("escrow_missing".into()),
        TransitionError::MonetaryInvariantViolation => Some("monetary_invariant".into()),
        TransitionError::AcceptancePredicateFailed(_)
        | TransitionError::SettlementPredicateFailed(_) => Some("predicate_failed".into()),
        _ => Some("policy_violation".into()),
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 8 dispatch_transition — exhaustive enum match (K5: NO Slash)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 8 — exhaustive dispatch over `TypedTx` variants.
///
/// **Stub state (CO1.7-impl A3)**: every variant returns
/// `TransitionError::NotYetImplemented`. CO1.7.5 fills each arm with the real
/// transition body per `STATE_TRANSITION_SPEC § 3.1-3.7`. The exhaustive match
/// itself is the contract: any future TypedTx variant addition triggers a
/// non-exhaustive-match compile error here, forcing explicit handling.
pub(crate) fn dispatch_transition(
    q: &QState,
    tx: &TypedTx,
    _predicate_registry: &PredicateRegistry,
    _tool_registry: &ToolRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {
    match tx {
        TypedTx::Work(work) => {
            // TB-2 Atom 3: WorkTx pure validation per preflight v3 §3.3.
            // No I/O, no side effects, no writer calls — apply_one is the
            // only place ledger writes happen.

            // Step 1: parent-root match (Inv 5; P1:5).
            if work.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }

            // Step 2: acceptance predicate bundle — every entry must be true.
            for (pid, bwp) in work.predicate_results.acceptance.iter() {
                if !bwp.value {
                    return Err(TransitionError::AcceptancePredicateFailed(pid.clone()));
                }
            }

            // Step 3: settlement predicate bundle (if applicable to RSP-1).
            for (pid, bwp) in work.predicate_results.settlement.iter() {
                if !bwp.value {
                    return Err(TransitionError::SettlementPredicateFailed(pid.clone()));
                }
            }

            // Step 4: YES stake gate (RSP-1 P3:3). StakeMicroCoin newtype
            // intentionally has no integer comparison; use the const accessor.
            if work.stake.micro_units() <= 0 {
                return Err(TransitionError::StakeInsufficient);
            }

            // Step 5: escrow presence gate (RSP-1 P3:5; P0-B option (a) — bridge
            // at lookup site). TB-3 introduces formal task_open_tx /
            // escrow_lock_tx / yes_stake_tx variants and DELETES this bridge.
            //
            // EscrowsIndex / TaskMarketsIndex are pub-tuple-struct newtypes
            // wrapping BTreeMap<TxId, _> at q_state.rs:159-161, 222-224 —
            // .contains_key on the wrapper would not compile; .0 reaches the
            // inner map.
            // TB-2 P0-B option (a): drop this when task_open_tx lands in TB-3
            let lookup_tx_id = TxId(work.task_id.0.clone());
            let has_escrow = q.economic_state_t.escrows_t.0.contains_key(&lookup_tx_id)
                || q
                    .economic_state_t
                    .task_markets_t
                    .0
                    .contains_key(&lookup_tx_id);
            if !has_escrow {
                return Err(TransitionError::EscrowMissing);
            }

            // Step 6: monetary invariants. q_next initially equals q (TB-2
            // does not yet move stake/escrow balances; RSP-1 lifecycle is
            // TB-3+). The conservation check therefore passes trivially in
            // TB-2 — the call is locked in shape now so future TBs that DO
            // mutate balances cannot accidentally bypass it.
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_read_is_free(tx.tx_kind(), 0)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;

            // Step 7: build q_next. state_root_t advances via the interim
            // WORKTX_ACCEPT_DOMAIN_V1 hash; economic_state_t is unchanged
            // (real RSP-1 stake/escrow movement lands in TB-3+); ledger_root_t
            // is set by apply_one stage 9 (we leave it == q.ledger_root_t here).
            let mut q_next = q.clone();
            q_next.state_root_t = worktx_accept_state_root(&q.state_root_t, tx);

            // Conservation must hold across the q → q_next transition. With
            // economic_state_t unchanged this is a no-op success in TB-2,
            // but the call MUST be present — production runtime ALWAYS passes
            // &[] (no exempt list per §8 red line 4 / charter §5).
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;

            Ok((q_next, SignalBundle::default()))
        }
        TypedTx::Verify(_) => Err(TransitionError::NotYetImplemented),
        TypedTx::Challenge(_) => Err(TransitionError::NotYetImplemented),
        TypedTx::Reuse(_) => Err(TransitionError::NotYetImplemented),
        TypedTx::FinalizeReward(_) => Err(TransitionError::NotYetImplemented),
        TypedTx::TaskExpire(_) => Err(TransitionError::NotYetImplemented),
        TypedTx::TerminalSummary(_) => Err(TransitionError::NotYetImplemented),
    }
}

// ────────────────────────────────────────────────────────────────────────────
// CO1.7-extra D2: advance_head_t — post-commit head_t close (Art 0.4)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 5 — L4 sequencer post-commit head_t wiring (Art 0.4).
///
/// Closes the G-1 carry-forward: when `writer` surfaces a commit OID hex
/// (Git2LedgerWriter), advance `q.head_t = state::q_state::NodeId(hex)`;
/// when `writer` returns None (InMemoryLedgerWriter), leave `q.head_t`
/// unchanged (no-op preservation).
///
/// Called from `apply_one` stage 9 AFTER `writer.commit` succeeds. Pure
/// function (writer is `&dyn` so behavior depends only on writer's
/// `head_commit_oid_hex` return + q's prior state).
///
/// **Visibility** (CO1.7-extra round-3 B2): `pub` (NOT `pub(crate)`) so that
/// flat integration tests under `tests/co1_7_extra_*.rs` per round-2 MF5 can
/// call this helper directly.
///
/// **Atomicity** (CO1.7-extra round-2 MF9): in apply_one, called under the
/// `q_w` write lock immediately after `writer.commit` returns Ok. For Git2
/// (Some path), this is post-commit non-failing best-effort head binding —
/// `q.head_t`, `q.ledger_root_t`, and `next_logical_t` advance atomically.
/// For InMemory (None path), this is explicit no-op preservation —
/// `q.head_t` stays at the value `*q_w = q_next` left it (which equals the
/// prior value because pure transition bodies never mutate head_t per
/// CO1.7 K3 v1.2).
pub fn advance_head_t(q: &mut QState, writer: &dyn LedgerWriter) {
    if let Some(commit_oid_hex) = writer.head_commit_oid_hex() {
        q.head_t = crate::state::q_state::NodeId(commit_oid_hex);
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Submission types — K1 dual counter
// ────────────────────────────────────────────────────────────────────────────

/// Returned by `Sequencer::submit`. Carries `submit_id` (always assigned at
/// submit time) but **NOT** `logical_t` — logical_t is only assigned post-accept
/// per K1 (see spec § 3 + CO1.7 K1 closure).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubmissionReceipt {
    pub submit_id: u64,
}

/// TRACE_MATRIX FC3-S3: L4 sequencer queue payload carrying both `submit_id`
/// and the typed tx through to `apply_one`.
///
/// Required by P1:6 / TB-2 charter §1: the L4.E rejection-evidence ledger
/// keys rejected submissions by `submit_id` (NOT `logical_t`), so the
/// sequencer driver loop must observe the same `submit_id` the caller received.
///
/// Pre-TB-2 the queue carried `TypedTx` only, stranding `submit_id` at
/// `submit()`. TB-2 preflight v3 §3.1 (P1-C r1: named struct over tuple).
#[derive(Debug)]
pub struct SubmissionEnvelope {
    pub submit_id: u64,
    pub tx: TypedTx,
}

#[derive(Debug)]
pub enum SubmitError {
    /// Bounded queue saturated (Q1/Q2 resolution: agent retries with backoff).
    QueueFull,
    /// Receiver dropped — sequencer no longer running.
    QueueClosed,
}

impl std::fmt::Display for SubmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueueFull => write!(f, "submission queue saturated"),
            Self::QueueClosed => write!(f, "submission queue closed"),
        }
    }
}
impl std::error::Error for SubmitError {}

/// Errors that can occur during `apply_one`. Spec § 3 implicitly assumes
/// `Result<_, TransitionError>` but the actual `?`-propagated error chain
/// crosses CAS, keypair, and ledger-writer boundaries — wrapper enum captures
/// all of these explicitly. **Implementation note vs. spec**: spec § 3 line
/// 307 writes the apply_one signature as `Result<LedgerEntry, TransitionError>`;
/// this implementation widens to `Result<LedgerEntry, ApplyError>` to preserve
/// distinct error provenance (TransitionError keeps its closed taxonomy +
/// additive-only invariant per CO1.1.4-pre1 § 7.2).
#[derive(Debug)]
pub enum ApplyError {
    /// Pure transition function rejected the tx.
    Transition(TransitionError),
    /// CAS payload put failed.
    Cas(CasError),
    /// System keypair sign failed.
    Keypair(KeypairError),
    /// Ledger writer commit failed.
    LedgerCommit(LedgerWriterError),
    /// Internal: canonical encoding of typed-tx payload failed (should never
    /// happen for serde-derive types; surfaced for completeness).
    PayloadEncode(String),
    /// `q.read()` / `q.write()` lock poisoned by panicking thread.
    QStateLockPoisoned,
}

impl std::fmt::Display for ApplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transition(e) => write!(f, "transition rejected: {e}"),
            Self::Cas(e) => write!(f, "cas put failed: {e}"),
            Self::Keypair(e) => write!(f, "keypair sign failed: {e:?}"),
            Self::LedgerCommit(e) => write!(f, "ledger commit failed: {e}"),
            Self::PayloadEncode(s) => write!(f, "payload encode failed: {s}"),
            Self::QStateLockPoisoned => write!(f, "q-state lock poisoned"),
        }
    }
}
impl std::error::Error for ApplyError {}

impl From<TransitionError> for ApplyError {
    fn from(e: TransitionError) -> Self {
        Self::Transition(e)
    }
}
impl From<CasError> for ApplyError {
    fn from(e: CasError) -> Self {
        Self::Cas(e)
    }
}
impl From<KeypairError> for ApplyError {
    fn from(e: KeypairError) -> Self {
        Self::Keypair(e)
    }
}
impl From<LedgerWriterError> for ApplyError {
    fn from(e: LedgerWriterError) -> Self {
        Self::LedgerCommit(e)
    }
}

#[derive(Debug)]
pub enum SequencerError {
    /// `run()` was called when the receiver had already been consumed.
    ReceiverAlreadyTaken,
}

impl std::fmt::Display for SequencerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReceiverAlreadyTaken => write!(f, "sequencer receiver already taken"),
        }
    }
}
impl std::error::Error for SequencerError {}

// ────────────────────────────────────────────────────────────────────────────
// Sequencer — single-writer per (runtime_repo, run_id)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 5.2.1 — L4 sequencer; single-writer per (runtime_repo, run_id).
///
/// **K1 dual counter**: `next_submit_id` advances at every `submit()` (used to
/// derive `SubmissionReceipt.submit_id`); `next_logical_t` advances ONLY at
/// commit time (rejected submissions never get a logical_t — preserves
/// `LedgerWriter`'s strict logical_t monotonicity invariant).
///
/// **K3 v1.2 + CO1.7-extra D2 (revised)**: the pure transition function does
/// NOT mutate `q.head_t` or `q.state_root_t`; it returns the new `QState`
/// and the sequencer accepts it as-is. `head_t` mutation now happens
/// post-commit via `advance_head_t()` (CO1.7-extra D2): when
/// `LedgerWriter::head_commit_oid_hex()` returns Some (Git2LedgerWriter),
/// the sequencer writes `q.head_t = NodeId(commit_oid_hex)`; when None
/// (InMemoryLedgerWriter), `head_t` is left unchanged (no-op preservation).
///
/// **C3 sign API**: signs through
/// `transition_ledger_emitter::sign_ledger_entry(keypair, digest_bytes)` —
/// the typed `CanonicalMessage::LedgerEntrySigning([u8;32])` extension closes
/// the C3 round-2 audit point.
/// **CO1.7-extra D3 (round-2 MF6)**: manual `Debug` impl below — `#[derive(Debug)]`
/// fails because `Arc<Ed25519Keypair>` field has no Debug derive (intentional;
/// `Ed25519Keypair` derives only `Zeroize, ZeroizeOnDrop` for secret-handling).
/// `finish_non_exhaustive()` leaks no keypair / QState / CAS contents and
/// satisfies Debug propagation through `Arc<Sequencer>` for `TuringBus.Debug`.
pub struct Sequencer {
    /// K1: assigned at submit; never appears in LedgerEntry.
    next_submit_id: AtomicU64,
    /// K1: advances ONLY on commit; first accepted entry gets logical_t=1.
    next_logical_t: AtomicU64,

    queue_tx: tokio::sync::mpsc::Sender<SubmissionEnvelope>,

    cas: Arc<RwLock<CasStore>>,
    keypair: Arc<Ed25519Keypair>,
    epoch: SystemEpoch,
    ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
    /// TB-2 Atom 4: L4.E rejection-evidence writer. Mirrors `ledger_writer`'s
    /// `Arc<RwLock<...>>` shape (P0-1 r2: `append_rejected` is `&mut self`).
    /// Constructor-injected so integration tests can retain a clone of the
    /// `Arc` for L4.E observation (P0-5 r2).
    rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,

    predicate_registry: Arc<PredicateRegistry>,
    tool_registry: Arc<ToolRegistry>,

    q: RwLock<QState>,
}

/// CO1.7-extra D3 (round-2 MF6): manual Debug impl. Uses `finish_non_exhaustive()`
/// to satisfy the Debug trait without exposing keypair / QState / CAS internals.
impl std::fmt::Debug for Sequencer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sequencer").finish_non_exhaustive()
    }
}

impl Sequencer {
    /// Construct. Returns the `Sequencer` plus the receiver half of the
    /// internal mpsc; pass the receiver to `run()` exactly once.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cas: Arc<RwLock<CasStore>>,
        keypair: Arc<Ed25519Keypair>,
        epoch: SystemEpoch,
        ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
        rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
        predicate_registry: Arc<PredicateRegistry>,
        tool_registry: Arc<ToolRegistry>,
        initial_q: QState,
        queue_capacity: usize,
    ) -> (Self, tokio::sync::mpsc::Receiver<SubmissionEnvelope>) {
        let (queue_tx, queue_rx) = tokio::sync::mpsc::channel(queue_capacity);
        let seq = Self {
            next_submit_id: AtomicU64::new(1),
            next_logical_t: AtomicU64::new(0), // first accepted commit advances to 1
            queue_tx,
            cas,
            keypair,
            epoch,
            ledger_writer,
            rejection_writer,
            predicate_registry,
            tool_registry,
            q: RwLock::new(initial_q),
        };
        (seq, queue_rx)
    }

    /// Submit a typed transition. Returns immediately with a receipt carrying
    /// `submit_id` (NOT `logical_t`). Per Q2 (back-pressure resolution): on
    /// queue saturation returns `Err(SubmitError::QueueFull)` and the agent is
    /// expected to retry with deterministic exponential backoff.
    pub async fn submit(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
        // TB-2 P1-D r1 concurrency contract: fetch_add precedes try_send, so
        // submit_id allocation order is NOT receiver arrival order under
        // multi-producer scheduling. submit_id is always burned (never reused)
        // even when try_send fails — locked by integration test I2.
        let submit_id = self.next_submit_id.fetch_add(1, Ordering::SeqCst);
        let envelope = SubmissionEnvelope { submit_id, tx };
        match self.queue_tx.try_send(envelope) {
            Ok(()) => Ok(SubmissionReceipt { submit_id }),
            Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => Err(SubmitError::QueueFull),
            Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => Err(SubmitError::QueueClosed),
        }
    }

    /// Driver loop. Drains the queue and runs `apply_one` on each tx. Errors
    /// from individual `apply_one` calls are logged and skipped (per-tx
    /// rejection does NOT halt the sequencer). Returns when the queue is
    /// closed and drained.
    pub async fn run(
        &self,
        mut queue_rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    ) -> Result<(), SequencerError> {
        while let Some(envelope) = queue_rx.recv().await {
            // Stub state: dispatch returns NotYetImplemented; apply_one
            // bubbles up. We log and continue per spec § 3 v1.2 ordering rule
            // (rejection does not consume a logical_t — see K1).
            if let Err(e) = self.apply_one(envelope) {
                log::debug!("sequencer apply_one rejected: {e}");
            }
        }
        Ok(())
    }

    /// TRACE_MATRIX FC3-S3: single-step driver companion to `run()` for tests.
    ///
    /// Drains at most one envelope from the queue and runs `apply_one` on it.
    /// Returns `None` if the queue is empty. Production code uses `run()`
    /// instead. Required by integration tests in `tests/tb_2_runtime_boundary.rs`
    /// (TB-2 Atom 4+) because `run()` loops until the receiver closes — there
    /// is no other single-poll API. TB-2 preflight v3 §3.1 (P1-3 r2).
    pub fn try_apply_one(
        &self,
        queue_rx: &mut tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    ) -> Option<Result<LedgerEntry, ApplyError>> {
        match queue_rx.try_recv() {
            Ok(envelope) => Some(self.apply_one(envelope)),
            Err(_) => None,
        }
    }

    /// TRACE_MATRIX FC3-S3: L4 sequencer per-tx critical section.
    ///
    /// Pure transition + CAS put + sign + commit + Q_t mutation. See spec § 3
    /// stages 1-9. TB-2 Atom 2 changes the input type from `TypedTx` to
    /// `SubmissionEnvelope` so `submit_id` travels in (charter §1 / P1:6);
    /// the apply pipeline itself is unchanged in Atom 2.
    ///
    /// **v1.1 C-2 closure (Codex bundle Q-B)**: `next_logical_t` advances
    /// **only on commit success** — the original spec § 3 stage-4
    /// `fetch_add(1)` happened BEFORE sign + writer.commit, so any infra
    /// failure (sign / commit) left `next_logical_t` advanced past a
    /// logical_t that was never written to the ledger. The next accepted
    /// tx would then be assigned a logical_t the writer rejects forever
    /// (writer enforces strict `len + 1`). Fixed by `load → use → store
    /// after commit succeeds`. Single-writer per spec § 5.2.1 makes the
    /// load+store atomic enough; if multi-writer ever lands the AtomicU64
    /// can be upgraded to a `compare_exchange` reservation pattern.
    pub(crate) fn apply_one(
        &self,
        envelope: SubmissionEnvelope,
    ) -> Result<LedgerEntry, ApplyError> {
        // TB-2 Atom 2: queue payload is SubmissionEnvelope so submit_id
        // travels with the tx through to apply_one. Atom 4: submit_id is
        // now actually used for the L4.E rejection-evidence path below.
        let SubmissionEnvelope { submit_id, tx } = envelope;

        // Stage 1: snapshot Q_t under read lock.
        let q_snapshot = {
            let g = self.q.read().map_err(|_| ApplyError::QStateLockPoisoned)?;
            g.clone()
        };

        // Stage 2: dispatch (pure). On reject, route to L4.E rejection-evidence
        // ledger and return early. K1: no logical_t consumed; Inv 7: no
        // state_root_t / ledger_root_t advance.
        let (q_next, _signals) = match dispatch_transition(
            &q_snapshot,
            &tx,
            &self.predicate_registry,
            &self.tool_registry,
        ) {
            Ok(ok) => ok,
            Err(transition_err) => {
                // TB-2 Atom 4 — rejection-writer path (preflight v3 §3.5).
                // CAS-put canonical-encoded tx payload + diagnostic, then
                // append_rejected to L4.E with submit_id keyed off the envelope.
                let payload_bytes = canonical_encode(&tx)
                    .map_err(|e| ApplyError::PayloadEncode(e.to_string()))?;
                let creator = format!("sequencer.rejection_path.epoch-{}", self.epoch.get());
                let rejection_logical_t = self.next_logical_t.load(Ordering::SeqCst);

                let tx_payload_cid = {
                    let mut cas_w = self
                        .cas
                        .write()
                        .map_err(|_| ApplyError::QStateLockPoisoned)?;
                    cas_w.put(
                        &payload_bytes,
                        ObjectType::ProposalPayload,
                        &creator,
                        rejection_logical_t,
                        Some("TypedTx.v1".to_string()),
                    )?
                };

                // raw_diagnostic_cid is structurally serde-shielded on
                // RejectedSubmissionRecord per TB-1 P0-3
                // (rejection_evidence.rs:108). I8 re-confirms at runtime.
                let diag_bytes = transition_err.to_string().into_bytes();
                let raw_diagnostic_cid = {
                    let mut cas_w = self
                        .cas
                        .write()
                        .map_err(|_| ApplyError::QStateLockPoisoned)?;
                    Some(cas_w.put(
                        &diag_bytes,
                        ObjectType::Generic,
                        &creator,
                        rejection_logical_t,
                        Some("TransitionError.display.v1".to_string()),
                    )?)
                };

                // P0-2 r2: HasSubmitter::submitter_id() returns Option<AgentId>.
                // RejectedSubmissionRecord.agent_id is AgentId (not Option).
                // Fall back to SYSTEM_AGENT_ID_STR for variants that return None.
                // WorkTx always returns Some so the unwrap_or_else arm is
                // theoretical for TB-2 but covers future system-emitted variants.
                let agent_id = tx
                    .submitter_id()
                    .unwrap_or_else(|| AgentId(SYSTEM_AGENT_ID_STR.to_string()));

                {
                    let mut writer_w = self
                        .rejection_writer
                        .write()
                        .map_err(|_| ApplyError::QStateLockPoisoned)?;
                    writer_w.append_rejected(
                        submit_id,
                        q_snapshot.state_root_t,
                        agent_id,
                        tx.tx_kind(),
                        tx_payload_cid,
                        rejection_class_for(&transition_err),
                        raw_diagnostic_cid,
                        public_summary_for(&transition_err),
                    );
                }

                // No logical_t advance, no state_root advance, no ledger_root
                // advance. Caller observes ApplyError::Transition.
                return Err(ApplyError::Transition(transition_err));
            }
        };

        // v1.1 C-2: TENTATIVE logical_t (do NOT fetch_add yet).
        let logical_t = self.next_logical_t.load(Ordering::SeqCst) + 1;

        // Stage 3: put payload to CAS. DIV-5 5-param put signature.
        let payload_bytes = canonical_encode(&tx)
            .map_err(|e| ApplyError::PayloadEncode(e.to_string()))?;
        let payload_cid = {
            let mut cas_w = self.cas.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
            cas_w.put(
                &payload_bytes,
                ObjectType::ProposalPayload,
                &format!("sequencer-epoch-{}", self.epoch.get()),
                logical_t,
                Some("TypedTx.v1".to_string()),
            )?
        };

        // Stage 5: build LedgerEntrySigningPayload (v1.1 — stage 4 fetch_add
        // moved to AFTER stage 9 commit success).
        let signing_payload = LedgerEntrySigningPayload {
            logical_t,
            parent_state_root: q_snapshot.state_root_t,
            parent_ledger_root: q_snapshot.ledger_root_t,
            tx_kind: tx.tx_kind(),
            tx_payload_cid: payload_cid,
            resulting_state_root: q_next.state_root_t,
            timestamp_logical: logical_t,
            epoch: self.epoch,
            extensions: std::collections::BTreeMap::new(),
        };

        // Stage 6: C3 — sign via typed CanonicalMessage::LedgerEntrySigning.
        let signing_digest = signing_payload.canonical_digest();
        let system_signature = transition_ledger_emitter::sign_ledger_entry(
            &self.keypair,
            signing_digest.0,
        )?;

        // Stage 7: pure ledger-root fold (deterministic).
        let resulting_ledger_root = append(&q_snapshot.ledger_root_t, &signing_digest);

        // Stage 8: build LedgerEntry (the stored record).
        let entry = LedgerEntry {
            logical_t: signing_payload.logical_t,
            parent_state_root: signing_payload.parent_state_root,
            parent_ledger_root: signing_payload.parent_ledger_root,
            tx_kind: signing_payload.tx_kind,
            tx_payload_cid: signing_payload.tx_payload_cid,
            resulting_state_root: signing_payload.resulting_state_root,
            resulting_ledger_root,
            timestamp_logical: signing_payload.timestamp_logical,
            epoch: signing_payload.epoch,
            extensions: signing_payload.extensions,
            system_signature,
        };

        // Stage 9: commit + mutate Q_t under write lock.
        // v1.1 C-2: next_logical_t.store(logical_t) HAPPENS ONLY AFTER
        // writer.commit succeeds — preserves K1 under infra failure.
        // CO1.7-extra D2: q.head_t = NodeId(commit_oid_hex) via advance_head_t
        // when writer surfaces a commit OID (Git2 path); no-op preservation
        // for writers that return None (InMemory path). state_root_t comes
        // from q_next as-is per K3 v1.2.
        {
            let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
            let mut writer_w = self
                .ledger_writer
                .write()
                .map_err(|_| ApplyError::QStateLockPoisoned)?;
            writer_w.commit(&entry)?; // ← may fail; if it does, fetch_add was NOT called
            // commit succeeded → safe to advance counter.
            self.next_logical_t.store(logical_t, Ordering::SeqCst);
            *q_w = q_next;
            q_w.ledger_root_t = entry.resulting_ledger_root;
            // CO1.7-extra D2: close G-1 head_t carry-forward (Art 0.4).
            advance_head_t(&mut *q_w, &*writer_w);
        }

        Ok(entry)
    }

    /// Read-only accessor (testing + CO1.7.5+ wiring).
    pub fn q_snapshot(&self) -> Result<QState, ApplyError> {
        self.q
            .read()
            .map(|g| g.clone())
            .map_err(|_| ApplyError::QStateLockPoisoned)
    }

    pub fn next_submit_id_peek(&self) -> u64 {
        self.next_submit_id.load(Ordering::SeqCst)
    }

    pub fn next_logical_t_peek(&self) -> u64 {
        self.next_logical_t.load(Ordering::SeqCst)
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Tests — stub-mode coverage (CO1.7.5 fills real-transition tests)
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bottom_white::ledger::transition_ledger::InMemoryLedgerWriter;
    use crate::state::typed_tx::{
        AgentSignature, BoolWithProof, ChallengeTx, ClaimId, FinalizeRewardTx, PredicateId,
        PredicateResultsBundle, ReadKey, ReuseTx, RunId, RunOutcome, SafetyOrCreation,
        TaskExpireTx, TaskId, TerminalSummaryTx, ToolId, VerifyTx, VerifyVerdict, WorkTx,
        WriteKey,
    };
    use crate::state::q_state::{AgentId, TxId};
    use crate::economy::money::{MicroCoin, StakeMicroCoin};
    use crate::bottom_white::cas::schema::Cid;
    use crate::bottom_white::ledger::system_keypair::SystemSignature;
    use std::collections::{BTreeMap, BTreeSet};
    use tempfile::TempDir;

    fn fresh_sequencer() -> (
        TempDir,
        Sequencer,
        tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
        Arc<RwLock<RejectionEvidenceWriter>>,
    ) {
        let tmp = TempDir::new().expect("tempdir");
        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas open")));
        let keypair = Arc::new(
            Ed25519Keypair::generate_with_secure_entropy().expect("keypair gen"),
        );
        let epoch = SystemEpoch::new(1);
        let writer: Arc<RwLock<dyn LedgerWriter>> =
            Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
        let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
        let preds = Arc::new(PredicateRegistry::new());
        let tools = Arc::new(ToolRegistry::new());
        let q = QState::genesis();
        let (seq, rx) = Sequencer::new(
            cas,
            keypair,
            epoch,
            writer,
            rejection_writer.clone(),
            preds,
            tools,
            q,
            16,
        );
        (tmp, seq, rx, rejection_writer)
    }

    fn fixture_work_tx() -> WorkTx {
        let mut acceptance = BTreeMap::new();
        acceptance.insert(
            PredicateId("acc1".into()),
            BoolWithProof {
                value: true,
                proof_cid: None,
            },
        );
        WorkTx {
            tx_id: TxId("worktx-seq-fixture".into()),
            task_id: TaskId("task-seq-fixture".into()),
            parent_state_root: Default::default(),
            agent_id: AgentId("alice".into()),
            read_set: [ReadKey("k.read.a".into())].into_iter().collect::<BTreeSet<_>>(),
            write_set: [WriteKey("k.write.a".into())].into_iter().collect::<BTreeSet<_>>(),
            proposal_cid: Default::default(),
            predicate_results: PredicateResultsBundle {
                acceptance,
                settlement: BTreeMap::new(),
                safety_class: SafetyOrCreation::Safety,
            },
            stake: StakeMicroCoin::from_micro_units(1_000_000),
            signature: AgentSignature::from_bytes([0x77u8; 64]),
            timestamp_logical: 1,
        }
    }

    // 1. dispatch_transition: every NON-WORK variant returns NotYetImplemented.
    //
    // TB-2 Atom 3 narrowed this from "all variants" to "non-Work variants" —
    // the WorkTx arm is now a real pure-validation body. Charter §4 requires
    // existing dispatch_transition_stubs to be narrowed when Work is no
    // longer a stub. WorkTx-specific behaviour is covered by U3 + I3-I12.
    #[test]
    fn dispatch_transition_stubs_non_work_variants() {
        let q = QState::genesis();
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();

        let cases: Vec<TypedTx> = vec![
            TypedTx::Verify(VerifyTx {
                tx_id: TxId("vt".into()),
                target_work_tx: TxId("wt".into()),
                verifier_agent: AgentId("v".into()),
                bond: StakeMicroCoin::from_micro_units(1),
                verdict: VerifyVerdict::Confirm,
                signature: AgentSignature::from_bytes([0; 64]),
                timestamp_logical: 1,
            }),
            TypedTx::Challenge(ChallengeTx {
                tx_id: TxId("ct".into()),
                target_work_tx: TxId("wt".into()),
                challenger_agent: AgentId("c".into()),
                stake: StakeMicroCoin::from_micro_units(1),
                counterexample_cid: Cid([0; 32]),
                signature: AgentSignature::from_bytes([0; 64]),
                timestamp_logical: 1,
            }),
            TypedTx::Reuse(ReuseTx {
                tx_id: TxId("rt".into()),
                reusing_work_tx: TxId("wt".into()),
                reused_tool_id: ToolId("tool".into()),
                reused_tool_creator: AgentId("a".into()),
                timestamp_logical: 1,
            }),
            TypedTx::FinalizeReward(FinalizeRewardTx {
                tx_id: TxId("ft".into()),
                claim_id: ClaimId::new("cl"),
                task_id: TaskId("t".into()),
                solver: AgentId("s".into()),
                reward: MicroCoin::from_micro_units(1),
                parent_state_root: Default::default(),
                epoch: SystemEpoch::new(1),
                timestamp_logical: 1,
                system_signature: SystemSignature::from_bytes([0; 64]),
            }),
            TypedTx::TaskExpire(TaskExpireTx {
                tx_id: TxId("et".into()),
                task_id: TaskId("t".into()),
                parent_state_root: Default::default(),
                bounty_refunded: MicroCoin::from_micro_units(1),
                epoch: SystemEpoch::new(1),
                timestamp_logical: 1,
                system_signature: SystemSignature::from_bytes([0; 64]),
            }),
            TypedTx::TerminalSummary(TerminalSummaryTx {
                tx_id: TxId("ts".into()),
                task_id: TaskId("t".into()),
                run_id: RunId("r".into()),
                run_outcome: RunOutcome::OmegaAccepted,
                total_attempts: 0,
                failure_class_histogram: BTreeMap::new(),
                last_logical_t: 0,
                system_signature: SystemSignature::from_bytes([0; 64]),
            }),
        ];

        for tx in cases {
            let result = dispatch_transition(&q, &tx, &preds, &tools);
            assert!(matches!(result, Err(TransitionError::NotYetImplemented)));
        }
    }

    // 2. K1 dual counter: submit advances submit_id but NOT logical_t.
    #[tokio::test]
    async fn submit_advances_submit_id_only() {
        let (_tmp, seq, _rx, _rejection_writer) = fresh_sequencer();
        assert_eq!(seq.next_submit_id_peek(), 1);
        assert_eq!(seq.next_logical_t_peek(), 0);

        let r1 = seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("submit 1");
        assert_eq!(r1.submit_id, 1);
        assert_eq!(seq.next_submit_id_peek(), 2);
        assert_eq!(seq.next_logical_t_peek(), 0, "logical_t MUST NOT advance at submit");

        let r2 = seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("submit 2");
        assert_eq!(r2.submit_id, 2);
        assert_eq!(seq.next_logical_t_peek(), 0);
    }

    // 3. apply_one rejected: returns Transition(EscrowMissing) with the default
    //    fixture (no escrow seeded for task-seq-fixture); no logical_t consumed
    //    (K1 invariant: rejected submission never advances commit counter).
    //    TB-2 Atom 3: was NotYetImplemented pre-Atom-3; now WorkTx arm runs
    //    real validation and rejects on missing escrow.
    #[test]
    fn apply_one_stub_does_not_consume_logical_t() {
        let (_tmp, seq, _rx, _rejection_writer) = fresh_sequencer();
        let pre = seq.next_logical_t_peek();
        let envelope = SubmissionEnvelope {
            submit_id: 1,
            tx: TypedTx::Work(fixture_work_tx()),
        };
        let err = seq.apply_one(envelope).unwrap_err();
        assert!(matches!(
            err,
            ApplyError::Transition(TransitionError::EscrowMissing)
        ));
        let post = seq.next_logical_t_peek();
        assert_eq!(pre, post, "logical_t MUST NOT advance on rejected apply_one");
    }

    // TB-2 Atom 4 — U2: apply_one rejected path keys L4.E by envelope.submit_id.
    //
    // Drives apply_one with a known submit_id and a WorkTx that fails the
    // EscrowMissing gate (default fixture has no seeded escrow). Asserts the
    // resulting L4.E row has the same submit_id, mapped rejection_class, and
    // q_snapshot.state_root_t carried in. Locks P1:6 contract.
    #[test]
    fn apply_one_rejected_path_uses_envelope_submit_id() {
        let (_tmp, seq, _rx, rejection_writer) = fresh_sequencer();
        let pre = seq.q_snapshot().expect("q_snapshot").state_root_t;
        let envelope = SubmissionEnvelope {
            submit_id: 42,
            tx: TypedTx::Work(fixture_work_tx()),
        };
        let err = seq.apply_one(envelope).unwrap_err();
        assert!(matches!(
            err,
            ApplyError::Transition(TransitionError::EscrowMissing)
        ));

        let writer_g = rejection_writer.read().expect("writer read");
        let records = writer_g.records();
        assert_eq!(records.len(), 1, "exactly one L4.E row appended");
        let row = &records[0];
        assert_eq!(row.submit_id, 42, "L4.E row keyed by envelope.submit_id");
        assert_eq!(
            row.rejection_class,
            L4ERejectionClass::EscrowMissing,
            "TransitionError::EscrowMissing maps to RejectionClass::EscrowMissing"
        );
        assert_eq!(
            row.parent_state_root, pre,
            "L4.E row records pre-submit state_root_t (Inv 7)"
        );
        // L4.E never advances state; sequencer's state_root_t is unchanged.
        let post = seq.q_snapshot().expect("q_snapshot").state_root_t;
        assert_eq!(pre, post, "rejected WorkTx leaves state_root_t unchanged");
        assert_eq!(seq.next_logical_t_peek(), 0, "no logical_t consumed");
    }

    // TB-2 Atom 2 — U1: apply_one consumes SubmissionEnvelope.
    //
    // Signature-level proof that the queue payload type now carries submit_id
    // through to apply_one. Charter §8 Proof 1 will further verify that the
    // submit_id materializes in an L4.E row (Atom 4); Atom 2 only locks the
    // plumbing.
    #[test]
    fn apply_one_consumes_submission_envelope() {
        let (_tmp, seq, _rx, _rejection_writer) = fresh_sequencer();
        let envelope = SubmissionEnvelope {
            submit_id: 12345,
            tx: TypedTx::Work(fixture_work_tx()),
        };
        // Compile-time: apply_one(SubmissionEnvelope) is the canonical signature.
        // Runtime (post-Atom-3): default fixture has no seeded escrow so the
        // WorkTx arm rejects with EscrowMissing.
        let result = seq.apply_one(envelope);
        assert!(matches!(
            result,
            Err(ApplyError::Transition(TransitionError::EscrowMissing))
        ));
    }

    // TB-2 Atom 2 — try_apply_one driver helper (P1-3 r2).
    //
    // Drains at most one envelope from the queue; returns None on empty.
    // Required by integration tests in tests/tb_2_runtime_boundary.rs (Atom 4+)
    // because Sequencer::run loops until close — there is no single-poll API.
    #[tokio::test]
    async fn try_apply_one_drains_one_envelope() {
        let (_tmp, seq, mut rx, _rejection_writer) = fresh_sequencer();

        // Empty queue → None.
        assert!(seq.try_apply_one(&mut rx).is_none());

        // Submit one tx through the public path; try_apply_one should drain it.
        let receipt = seq
            .submit(TypedTx::Work(fixture_work_tx()))
            .await
            .expect("submit");
        let drained = seq.try_apply_one(&mut rx).expect("envelope was queued");
        // Default fixture lacks seeded escrow so apply_one rejects with
        // EscrowMissing. The contract proven here is "envelope was drained
        // from queue and apply_one ran".
        assert!(matches!(
            drained,
            Err(ApplyError::Transition(TransitionError::EscrowMissing))
        ));
        // Receipt's submit_id is still recoverable; concurrency contract (P1-D)
        // says it MAY have been allocated as 1, 2, etc. depending on prior
        // counter state; here pre-state is fresh so it is 1.
        assert_eq!(receipt.submit_id, 1);

        // After drain, queue is empty again.
        assert!(seq.try_apply_one(&mut rx).is_none());
    }

    // TB-2 Atom 3 — U3: dispatch_transition WorkTx returns the interim
    // domain-separated state_root_t on accept.
    //
    // Drives dispatch_transition directly (not apply_one — that's the in-crate
    // pub(crate) test surface) with a predicate-passing WorkTx + stake>0 +
    // seeded escrow. Asserts q_next.state_root_t equals exactly
    // sha256(WORKTX_ACCEPT_DOMAIN_V1 || q.state_root_t.0 || worktx_canonical_hash(tx).0).
    // Locks the interim hash so any future change is loud.
    #[test]
    fn dispatch_transition_worktx_returns_state_root_via_domain_v1() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let work_tx = fixture_work_tx();
        let task_id_for_lookup = TxId(work_tx.task_id.0.clone());

        // Seed an escrow for the task so the §3.3 step-5 bridge passes.
        let mut q = QState::genesis();
        q.economic_state_t.escrows_t.0.insert(
            task_id_for_lookup,
            crate::state::q_state::EscrowEntry {
                amount: MicroCoin::from_micro_units(2_000_000),
                depositor: AgentId("treasury".into()),
            },
        );

        let tx = TypedTx::Work(work_tx);
        let (q_next, _signals) = dispatch_transition(&q, &tx, &preds, &tools)
            .expect("predicate-passing WorkTx with seeded escrow must accept");

        // Expected state_root_t per the interim domain-separated hash.
        let expected = {
            let work_digest = worktx_canonical_hash(&tx);
            let mut h = Sha256::new();
            h.update(WORKTX_ACCEPT_DOMAIN_V1);
            h.update(q.state_root_t.0);
            h.update(work_digest.0);
            let bytes: [u8; 32] = h.finalize().into();
            Hash::from_bytes(bytes)
        };

        assert_eq!(q_next.state_root_t, expected, "state_root_t must match WORKTX_ACCEPT_DOMAIN_V1 hash");
        assert_ne!(q_next.state_root_t, q.state_root_t, "state_root_t must advance on accept");
        // economic_state_t unchanged in TB-2 (real RSP-1 lifecycle is TB-3+).
        assert_eq!(q_next.economic_state_t, q.economic_state_t, "TB-2 leaves economic_state_t unchanged on WorkTx accept");
    }

    // 4. Queue saturation: submit returns QueueFull (Q1/Q2 resolution).
    #[tokio::test]
    async fn submit_returns_queue_full_on_saturation() {
        // Capacity=2; receiver never drained.
        let tmp = TempDir::new().expect("tempdir");
        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
        let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("kp"));
        let writer: Arc<RwLock<dyn LedgerWriter>> =
            Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
        let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
        let preds = Arc::new(PredicateRegistry::new());
        let tools = Arc::new(ToolRegistry::new());
        let (seq, _rx) = Sequencer::new(
            cas,
            keypair,
            SystemEpoch::new(1),
            writer,
            rejection_writer,
            preds,
            tools,
            QState::genesis(),
            2,
        );
        // Fill capacity.
        seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("1");
        seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("2");
        // Saturated.
        let err = seq.submit(TypedTx::Work(fixture_work_tx())).await.unwrap_err();
        assert!(matches!(err, SubmitError::QueueFull));
    }

    // 5. submit returns QueueClosed when receiver dropped.
    #[tokio::test]
    async fn submit_returns_queue_closed_after_rx_drop() {
        let (_tmp, seq, rx, _rejection_writer) = fresh_sequencer();
        drop(rx);
        let err = seq.submit(TypedTx::Work(fixture_work_tx())).await.unwrap_err();
        assert!(matches!(err, SubmitError::QueueClosed));
    }
}
