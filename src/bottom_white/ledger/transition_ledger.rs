//! L4 Transition Ledger (CO1.7) — type skeleton + pure helpers.
//!
//! TRACE_MATRIX FC2-Append: canonical envelope appended to L4 once a transition is accepted.
//! TRACE_MATRIX WP § 5.L4: ChainTape Layer 4 spine; one LedgerEntry per accepted transition.
//! TRACE_MATRIX § 1 (CO1_7_TRANSITION_LEDGER_v1_2026-04-28): schema + append() + replay() pseudocode.
//!
//! **Status**: type skeleton ahead of round-1 dual external audit.
//! All bodies that depend on yet-to-implement transition functions are
//! `unimplemented!()` and gated by `cfg(any())` to keep `cargo check` clean.
//!
//! **Spec ↔ code divergences flagged for round-1 audit** (NOT silently absorbed):
//! - **DIV-1** `LedgerEntry` cannot ride existing `CanonicalMessage` enum
//!   (3 variants: RejectedAttemptSummary / TerminalSummaryTx / EpochRotationProof).
//!   Either extend the enum OR introduce a sibling sign primitive. v1 spec
//!   chose neither — round-1 audit Q8 (NEW).
//! - **DIV-2** `Sequencer` integration with `Q_t.economic_state_t.balances_t`
//!   etc. requires those indices to expose mutation API; current `q_state.rs`
//!   only stores the BTreeMap shells. Mutation API arrives at CO P2.x economy
//!   atoms; skeleton uses `unimplemented!()` for state mutation paths.
//! - **DIV-3** Spec § 1 missed `epoch: SystemEpoch` field. Added here. Without
//!   it, signature verification cannot resolve which pinned pubkey to use.
//! - **DIV-4** Spec § 4 used a `CasReader` trait; actual code uses concrete
//!   `CasStore` struct. Skeleton uses a narrower trait `LedgerCasView` that
//!   `CasStore` will impl in CO1.7.5+; keeps test seams open.
//! - **DIV-5** Spec § 1 `tx_payload_cid: Cid`. CAS `put` requires
//!   `(content, object_type, creator, created_at_logical_t, schema_id)` —
//!   five fields, not just bytes. Sequencer must build full metadata when
//!   storing; skeleton documents this requirement.

use sha2::{Digest, Sha256};

use crate::bottom_white::cas::schema::Cid;
use crate::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
use crate::state::q_state::Hash;

// ────────────────────────────────────────────────────────────────────────────
// § 1 LedgerEntry schema (skeleton; round-1-audit-pending)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append: discriminator for the typed payload behind a CAS Cid.
/// **Skeleton note**: serde derives deferred — bincode v2 canonical shape is
/// round-1 audit Q5 / spec § 2.5; premature derive would lock the shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TxKind {
    /// WorkTx (STATE_TRANSITION_SPEC § 1.2; 12 fields).
    Work,
    /// VerifyTx (§ 1.3).
    Verify,
    /// ChallengeTx (§ 1.3).
    Challenge,
    /// ReuseTx (§ 1.3).
    Reuse,
    /// Claim window expired clean → reward + stake return (§ 3.4).
    FinalizeReward,
    /// Task deadline reached unsolved → bounty refund (§ 3.6).
    TaskExpire,
    /// Run end without acceptance (§ 1.5 + § 3.7).
    TerminalSummary,
    /// Post-CO P2.5 ChallengeCourt slashing event.
    Slash,
}

/// TRACE_MATRIX FC2-Append + WP § 5.L4 (12-field WorkTx envelope wrapper):
/// canonical envelope stamped by the L4 sequencer once `dispatch_transition` succeeds.
///
/// **DIV-3** vs spec v1: added `epoch: SystemEpoch` to bind signature verification
/// to a specific pinned pubkey (per `system_keypair::verify_system_signature`).
///
/// **Skeleton note**: serde derives deferred per round-1 audit Q5 (canonical shape
/// for cross-impl byte parity is a real spec choice, not a default).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerEntry {
    /// Monotonic counter from sequencer; starts at 1 per genesis.
    pub logical_t: u64,

    /// Parent state_root before this transition. Equals `prev.resulting_state_root`
    /// (or `Hash::ZERO` at logical_t=1).
    pub parent_state_root: Hash,

    /// Discriminator; payload schema depends on this.
    pub tx_kind: TxKind,

    /// CAS handle (CO1.4) to canonically-serialized payload.
    /// Sequencer is responsible for building full CAS metadata
    /// (object_type / creator / created_at_logical_t / schema_id) per **DIV-5**.
    pub tx_payload_cid: Cid,

    /// Resulting state_root after `dispatch_transition` applied.
    /// Used by I-DETHASH replay test.
    pub resulting_state_root: Hash,

    /// Resulting ledger_root after this entry is folded in.
    /// Convention: `ledger_root_{t+1} = sha256(ledger_root_t || canonical_digest_unsigned(LedgerEntry_t))`
    pub resulting_ledger_root: Hash,

    /// Wall-clock-free timestamp (= `logical_t`).
    /// Runtime layer does NOT mutate this field after sequencer commit.
    pub timestamp_logical: u64,

    /// **DIV-3**: which pinned epoch pubkey signed this entry.
    pub epoch: SystemEpoch,

    /// System runtime keypair signature over `canonical_digest_unsigned`.
    /// Distinct from the `agent_signature` inside payload (agent self-sign).
    /// System signature attests "sequencer accepted this entry at this logical_t".
    ///
    /// **DIV-1**: how this is computed is round-1 audit Q8. Skeleton stores it
    /// but does not yet derive it through `CanonicalMessage` (which currently has
    /// 3 fixed variants and does NOT include LedgerEntry).
    pub system_signature: SystemSignature,
}

impl LedgerEntry {
    /// TRACE_MATRIX FC2-Append: canonical digest of the 7 fields the system
    /// signature attests. **Excludes** `resulting_ledger_root` AND `system_signature`:
    /// - `system_signature` (8) is excluded because the digest is its input.
    /// - `resulting_ledger_root` (6) is excluded because it is *derived* via
    ///   `append(prev_ledger_root, digest)` — including it would create a
    ///   circular dependency (ledger_root ⊃ digest ⊃ ledger_root).
    ///
    /// **Spec finding**: this exclusion was NOT explicit in spec v1 § 1.
    /// Skeleton smoke caught the cycle immediately on first replay-test run.
    /// To be sedimented into spec v1.1 round-1 audit Q9 (NEW).
    ///
    /// **DIV-1**: this digest is what the system_signature must sign once the
    /// `CanonicalMessage` integration question is resolved at round-1.
    pub fn canonical_digest_unsigned(&self) -> Hash {
        let mut h = Sha256::new();
        h.update(b"turingosv4.ledger_entry.v1");
        h.update(self.logical_t.to_be_bytes());
        h.update(self.parent_state_root.0);
        h.update((self.tx_kind as u8).to_be_bytes());
        h.update(self.tx_payload_cid.0);
        h.update(self.resulting_state_root.0);
        // EXCLUDED: self.resulting_ledger_root.0 — derivative of this digest.
        h.update(self.timestamp_logical.to_be_bytes());
        h.update(self.epoch.get().to_be_bytes());
        Hash(h.finalize().into())
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 4 append() — pure ledger-root fold
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append + spec § 4: pure ledger-root fold.
/// Same `(parent_root, entry_digest)` → byte-identical `new_root`.
/// No I/O, no clock, no env. Witness for I-DET / I-DETHASH ledger axis.
pub fn append(parent_root: &Hash, entry_digest: &Hash) -> Hash {
    let mut h = Sha256::new();
    h.update(b"turingosv4.ledger_root.v1");
    h.update(parent_root.0);
    h.update(entry_digest.0);
    Hash(h.finalize().into())
}

// ────────────────────────────────────────────────────────────────────────────
// LedgerWriter trait + in-memory test impl
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append: storage abstraction for L4.
/// Production impl is `Git2LedgerWriter` (CO1.7.5; refs/transitions/main commit chain).
/// Test/skeleton impl is `InMemoryLedgerWriter` below.
pub trait LedgerWriter: Send + Sync {
    /// Commit a signed entry. Atomic: either the entry lands at the next logical_t
    /// or no state change. Returns the entry's `resulting_ledger_root` on success.
    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;

    /// Read the entry at a specific 1-indexed `logical_t`.
    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;

    /// Total number of accepted entries (highest assigned `logical_t`; 0 at genesis).
    fn len(&self) -> u64;
}

/// TRACE_MATRIX FC2-Append: error surface for storage layer.
#[derive(Debug)]
pub enum LedgerWriterError {
    LogicalTGap { expected: u64, got: u64 },
    NotFound { logical_t: u64 },
    BackendCorruption(String),
}

impl std::fmt::Display for LedgerWriterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LogicalTGap { expected, got } => {
                write!(f, "logical_t gap: expected {expected}, got {got}")
            }
            Self::NotFound { logical_t } => write!(f, "no entry at logical_t={logical_t}"),
            Self::BackendCorruption(msg) => write!(f, "backend corruption: {msg}"),
        }
    }
}
impl std::error::Error for LedgerWriterError {}

/// TRACE_MATRIX FC2-Append: in-memory test/skeleton writer.
/// Vec backing → strict logical_t ordering enforced at commit.
#[derive(Debug, Default)]
pub struct InMemoryLedgerWriter {
    entries: Vec<LedgerEntry>,
}

impl InMemoryLedgerWriter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl LedgerWriter for InMemoryLedgerWriter {
    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
        let expected = (self.entries.len() as u64) + 1;
        if entry.logical_t != expected {
            return Err(LedgerWriterError::LogicalTGap {
                expected,
                got: entry.logical_t,
            });
        }
        let root = entry.resulting_ledger_root;
        self.entries.push(entry.clone());
        Ok(root)
    }

    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
        if logical_t == 0 || logical_t > self.entries.len() as u64 {
            return Err(LedgerWriterError::NotFound { logical_t });
        }
        Ok(self.entries[(logical_t - 1) as usize].clone())
    }

    fn len(&self) -> u64 {
        self.entries.len() as u64
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 4 replay() — chain-integrity skeleton (full transition dispatch deferred)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX I-DETHASH (chain-integrity axis only at this iteration).
/// **Skeleton**: validates parent_state_root + ledger_root chain only.
/// Full transition dispatch (re-running each tx pure-function-side) lands when
/// CO1.7.5+ implements the actual transition function bodies.
#[derive(Debug)]
pub enum ReplayError {
    LogicalTGap { at: usize, expected: u64, got: u64 },
    ParentMismatch { at: usize },
    LedgerRootMismatch { at: usize },
}

impl std::fmt::Display for ReplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LogicalTGap { at, expected, got } => {
                write!(f, "logical_t gap at index {at}: expected {expected}, got {got}")
            }
            Self::ParentMismatch { at } => write!(f, "parent_state_root mismatch at index {at}"),
            Self::LedgerRootMismatch { at } => write!(f, "ledger_root mismatch at index {at}"),
        }
    }
}
impl std::error::Error for ReplayError {}

/// Replay chain integrity. Returns final `(state_root, ledger_root)` after replaying
/// `entries` from a given (genesis_state_root, genesis_ledger_root) start.
///
/// Per **DIV-2**: this skeleton does NOT yet re-run pure transition functions to
/// independently verify `entry.resulting_state_root`. That step lands in CO1.7.5
/// once `dispatch_transition` is implementable.
pub fn replay_chain_integrity(
    genesis_state_root: Hash,
    genesis_ledger_root: Hash,
    entries: &[LedgerEntry],
) -> Result<(Hash, Hash), ReplayError> {
    let mut prev_state_root = genesis_state_root;
    let mut prev_ledger_root = genesis_ledger_root;

    for (i, entry) in entries.iter().enumerate() {
        let expected_logical_t = (i as u64) + 1;
        if entry.logical_t != expected_logical_t {
            return Err(ReplayError::LogicalTGap {
                at: i,
                expected: expected_logical_t,
                got: entry.logical_t,
            });
        }
        if entry.parent_state_root != prev_state_root {
            return Err(ReplayError::ParentMismatch { at: i });
        }
        let recomputed = append(&prev_ledger_root, &entry.canonical_digest_unsigned());
        if recomputed != entry.resulting_ledger_root {
            return Err(ReplayError::LedgerRootMismatch { at: i });
        }
        prev_state_root = entry.resulting_state_root;
        prev_ledger_root = entry.resulting_ledger_root;
    }

    Ok((prev_state_root, prev_ledger_root))
}

// ────────────────────────────────────────────────────────────────────────────
// Tests — only the genuinely-pure paths are exercised at skeleton stage
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn h(byte: u8) -> Hash {
        Hash([byte; 32])
    }

    fn entry_at(
        logical_t: u64,
        parent_state_root: Hash,
        resulting_state_root: Hash,
        prev_ledger_root: Hash,
    ) -> LedgerEntry {
        let mut entry = LedgerEntry {
            logical_t,
            parent_state_root,
            tx_kind: TxKind::Work,
            tx_payload_cid: Cid([0u8; 32]),
            resulting_state_root,
            resulting_ledger_root: Hash::ZERO, // patched below
            timestamp_logical: logical_t,
            epoch: SystemEpoch::new(1),
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        };
        entry.resulting_ledger_root = append(&prev_ledger_root, &entry.canonical_digest_unsigned());
        entry
    }

    #[test]
    fn append_is_pure_and_byte_stable() {
        let a = append(&Hash::ZERO, &h(1));
        let b = append(&Hash::ZERO, &h(1));
        assert_eq!(a, b, "I-DET witness on append()");
        let c = append(&Hash::ZERO, &h(2));
        assert_ne!(a, c, "different entry digests must produce different roots");
    }

    #[test]
    fn canonical_digest_byte_stable_across_clones() {
        let e = entry_at(1, Hash::ZERO, h(0xaa), Hash::ZERO);
        let d1 = e.canonical_digest_unsigned();
        let e2 = e.clone();
        let d2 = e2.canonical_digest_unsigned();
        assert_eq!(d1, d2);
    }

    #[test]
    fn in_memory_writer_enforces_logical_t() {
        let mut w = InMemoryLedgerWriter::new();
        let e1 = entry_at(1, Hash::ZERO, h(1), Hash::ZERO);
        assert!(w.commit(&e1).is_ok());
        assert_eq!(w.len(), 1);

        let e_skip = entry_at(3, e1.resulting_state_root, h(2), e1.resulting_ledger_root);
        let err = w.commit(&e_skip).unwrap_err();
        assert!(matches!(err, LedgerWriterError::LogicalTGap { expected: 2, got: 3 }));
    }

    #[test]
    fn replay_validates_parent_chain() {
        let e1 = entry_at(1, Hash::ZERO, h(1), Hash::ZERO);
        let e2 = entry_at(2, e1.resulting_state_root, h(2), e1.resulting_ledger_root);
        let e3 = entry_at(3, e2.resulting_state_root, h(3), e2.resulting_ledger_root);
        let (final_state, final_ledger) =
            replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1.clone(), e2.clone(), e3.clone()])
                .expect("clean chain replays");
        assert_eq!(final_state, e3.resulting_state_root);
        assert_eq!(final_ledger, e3.resulting_ledger_root);
    }

    #[test]
    fn replay_rejects_parent_mismatch() {
        let e1 = entry_at(1, Hash::ZERO, h(1), Hash::ZERO);
        // e2 lies about parent_state_root
        let mut e2 = entry_at(2, e1.resulting_state_root, h(2), e1.resulting_ledger_root);
        e2.parent_state_root = h(0xff); // tamper
        e2.resulting_ledger_root = append(&e1.resulting_ledger_root, &e2.canonical_digest_unsigned());

        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
        assert!(matches!(err, ReplayError::ParentMismatch { at: 1 }));
    }

    #[test]
    fn replay_rejects_ledger_root_tamper() {
        let e1 = entry_at(1, Hash::ZERO, h(1), Hash::ZERO);
        let mut e2 = entry_at(2, e1.resulting_state_root, h(2), e1.resulting_ledger_root);
        // Don't recompute resulting_ledger_root — tamper directly
        e2.resulting_ledger_root = h(0xee);

        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
        assert!(matches!(err, ReplayError::LedgerRootMismatch { at: 1 }));
    }
}
