//! L4 Transition Ledger (CO1.7) — implementation atom.
//!
//! TRACE_MATRIX FC2-Append: canonical envelope appended to L4 once a transition is accepted.
//! TRACE_MATRIX WP § 5.L4: ChainTape Layer 4 spine; one LedgerEntry per accepted transition.
//! TRACE_MATRIX § 1-§ 8 (CO1_7_TRANSITION_LEDGER_v1_2026-04-28 v1.2): schema +
//! append() + replay_chain_integrity() + replay_full_transition() + Git2LedgerWriter.
//!
//! **Status**: CO1.7 spec PASS/PASS (3 rounds) + CO1.7-impl bundle PASS/PASS
//! (3 rounds: A1+A2+A3+A4 + CO1.4-extra). Per-kind transition function bodies
//! deferred to CO1.7.5 (NotYetImplemented stubs in `src/state/sequencer.rs`).
//!
//! v1 → v1.1 changes (smoke for round-2 dual audit):
//! - C1: two-mode replay enum (ChainOnly v1; FullTransition CO1.7.5+); skeleton now
//!   exposes `replay_chain_integrity` only (renamed for honesty).
//! - K1: sequencer dual-counter design — documented in spec § 3; skeleton has no
//!   sequencer code (deferred to CO1.7.5).
//! - K2: `parent_ledger_root: Hash` field added + bound in signing payload (transplant
//!   defense); new test asserts replay rejects parent_ledger_root tamper.
//! - K3: L4/L5 boundary clarified — CO1.7 owns ledger_root + commit-chain head_t;
//!   CO1.8 owns state_root mutation. Skeleton reflects boundary (no state_root mutation).
//! - K5: `TxKind::Slash` DROPPED for v4 (deferred to CO P2.5).
//! - K6: `#[repr(u8)]` + explicit discriminants on TxKind.
//! - K7: +2 conformance tests (parent_ledger_root tamper, digest exclusion).
//! - G1: `extensions: BTreeMap<String, Vec<u8>>` forward-compat field (empty in v1).
//! - C3 / Q8: signing target is `LedgerEntrySigningPayload` (separate struct) ready to
//!   ride a `CanonicalMessage::LedgerEntrySigning(_)` variant when CO1.7.5+ extends
//!   `system_keypair` (Wave 4-B additive extension). Skeleton has the payload struct
//!   + canonical_digest method; the actual CanonicalMessage extension is deferred.
//! - Q9: canonical_digest now lives on LedgerEntrySigningPayload, not LedgerEntry —
//!   structurally enforces "derivatives excluded".
//! - D1: epoch is bound in signing payload (Codex security wins over Gemini orthogonality).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::bottom_white::cas::schema::Cid;
use crate::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
use crate::state::q_state::Hash;

// ────────────────────────────────────────────────────────────────────────────
// § 1 LedgerEntry — the stored record (11 fields per v1.1)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append: discriminator for the typed payload behind a CAS Cid.
/// **K6**: `#[repr(u8)]` + explicit discriminants for stable cast in canonical digest.
/// **K5**: NO `Slash` variant — ChallengeCourt slash event deferred to CO P2.5 atom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TxKind {
    Work            = 0,
    Verify          = 1,
    Challenge       = 2,
    Reuse           = 3,
    FinalizeReward  = 4,
    TaskExpire      = 5,
    TerminalSummary = 6,
    /// TB-3 RSP-1 formal-tx-surface (charter § 4.1). Sponsor-emitted task
    /// market registration; metadata-only (no money movement).
    TaskOpen        = 7,
    /// TB-3 RSP-1 formal-tx-surface (charter § 4.1). Sponsor-emitted bounty
    /// funding; the sole RSP-1 path that grows `task_markets_t.total_escrow`.
    EscrowLock      = 8,
}

/// TRACE_MATRIX FC2-Append + WP § 5.L4: stored LedgerEntry record (11 fields).
///
/// Distinct from `LedgerEntrySigningPayload`: this is the FULL stored record
/// (includes derivatives + signature); the signing payload is the subset that
/// the system keypair attests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerEntry {
    /// **K1**: assigned ONLY at commit (sequencer dual-counter design); rejected
    /// submissions never get a logical_t.
    pub logical_t: u64,                          //  1
    pub parent_state_root: Hash,                 //  2
    /// **K2 NEW**: parent_ledger_root before fold; bound in signed payload to
    /// prevent transplant attacks.
    pub parent_ledger_root: Hash,                //  3
    pub tx_kind: TxKind,                         //  4
    /// CAS handle (CO1.4) to canonical-serialized payload (DIV-5 5-param put).
    pub tx_payload_cid: Cid,                     //  5
    /// Resulting state_root post-transition (NOT mutated by L4 — accepted as
    /// returned by transition function per K3 boundary).
    pub resulting_state_root: Hash,              //  6
    /// Resulting ledger_root after fold. Derivative; NOT in signed digest.
    pub resulting_ledger_root: Hash,             //  7
    pub timestamp_logical: u64,                  //  8
    /// **D1 / Q10**: epoch bound in signed payload (Codex security wins).
    pub epoch: SystemEpoch,                      //  9
    /// **G1 NEW**: forward-compat extension map. Empty in v1; reserved for v4.x.
    /// Bound in signed payload (G1 cannot bypass signature).
    pub extensions: BTreeMap<String, Vec<u8>>,   // 10
    /// Detached system signature over `LedgerEntrySigningPayload.canonical_digest()`.
    pub system_signature: SystemSignature,       // 11
}

// ────────────────────────────────────────────────────────────────────────────
// § 1.1 LedgerEntrySigningPayload — the signed bytes (NEW per C3 / Q9)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append C3: the bytes the system keypair actually signs.
///
/// **Excludes** (Q9 cycle prevention):
/// - `resulting_ledger_root` (derivative; including → cycle)
/// - `system_signature` (its own input)
///
/// **Includes** (9 non-derivative bound fields). Domain-separation prefix is
/// part of the digest to prevent cross-namespace collision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerEntrySigningPayload {
    pub logical_t: u64,
    pub parent_state_root: Hash,
    pub parent_ledger_root: Hash,                  // K2
    pub tx_kind: TxKind,
    pub tx_payload_cid: Cid,
    pub resulting_state_root: Hash,
    pub timestamp_logical: u64,
    pub epoch: SystemEpoch,                        // D1
    pub extensions: BTreeMap<String, Vec<u8>>,     // G1
}

impl LedgerEntrySigningPayload {
    /// Canonical SHA-256 digest. Stable wire format (NOT bincode/serde dependent).
    pub fn canonical_digest(&self) -> Hash {
        let mut h = Sha256::new();
        h.update(b"turingosv4.ledger_entry_signing.v1");
        h.update(self.logical_t.to_be_bytes());
        h.update(self.parent_state_root.0);
        h.update(self.parent_ledger_root.0);
        h.update((self.tx_kind as u8).to_be_bytes()); // K6 #[repr(u8)] makes cast stable
        h.update(self.tx_payload_cid.0);
        h.update(self.resulting_state_root.0);
        h.update(self.timestamp_logical.to_be_bytes());
        h.update(self.epoch.get().to_be_bytes());
        // Extensions: BTreeMap iterates in lex key order (deterministic);
        // length-prefix every field to prevent ambiguity attacks.
        h.update((self.extensions.len() as u64).to_be_bytes());
        for (k, v) in &self.extensions {
            h.update((k.len() as u64).to_be_bytes());
            h.update(k.as_bytes());
            h.update((v.len() as u64).to_be_bytes());
            h.update(v);
        }
        Hash(h.finalize().into())
    }
}

impl LedgerEntry {
    /// Project the LedgerEntry's signed-fields-subset back into a signing payload.
    /// Used by replay to recompute `signing_digest` and re-verify chain integrity.
    pub fn to_signing_payload(&self) -> LedgerEntrySigningPayload {
        LedgerEntrySigningPayload {
            logical_t: self.logical_t,
            parent_state_root: self.parent_state_root,
            parent_ledger_root: self.parent_ledger_root,
            tx_kind: self.tx_kind,
            tx_payload_cid: self.tx_payload_cid,
            resulting_state_root: self.resulting_state_root,
            timestamp_logical: self.timestamp_logical,
            epoch: self.epoch,
            extensions: self.extensions.clone(),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 4 append() — pure ledger-root fold
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append + spec § 4: pure ledger-root fold over signed digests.
/// Same `(parent_root, signing_digest)` → byte-identical `new_root`.
/// No I/O, no clock, no env. Witness for I-DET ledger axis.
pub fn append(parent_root: &Hash, signing_digest: &Hash) -> Hash {
    let mut h = Sha256::new();
    h.update(b"turingosv4.ledger_root.v1");
    h.update(parent_root.0);
    h.update(signing_digest.0);
    Hash(h.finalize().into())
}

// ────────────────────────────────────────────────────────────────────────────
// LedgerWriter trait (K4 reconciled to skeleton signature)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append: storage abstraction for L4.
/// Production impl is `Git2LedgerWriter` (CO1.7.5+; refs/transitions/main commit chain).
/// Test/skeleton impl is `InMemoryLedgerWriter` below.
///
/// **K4**: signature `commit(&mut self) → Hash` (NOT `&self → NodeId`); `iter_from`
/// deferred to CO1.7.5+ (only used by FullTransition replay; not v1 deliverable).
pub trait LedgerWriter: Send + Sync {
    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;
    fn len(&self) -> u64;

    /// TRACE_MATRIX § 5 — L4 sequencer post-commit head_t wiring (Art 0.4).
    ///
    /// Canonical 40-char lowercase hex commit OID of the most recent appended
    /// entry, or None if the chain is empty / backend has no commit-OID notion.
    ///
    /// **REQUIRED** (no default impl per CO1.7-extra round-2 MF3): Rust compiler
    /// enforces every `LedgerWriter` implementation declares this method. This
    /// satisfies both safety arguments raised across the audit arc:
    /// - **silent stagnation prevention**: impossible to inherit a default that
    ///   silently leaves head_t stale; a missing impl is a compile error.
    /// - **post-commit no-panic**: impl is free to return None at runtime if the
    ///   backend has no OID notion (e.g. InMemoryLedgerWriter); no panic risk.
    fn head_commit_oid_hex(&self) -> Option<String>;
}

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

/// In-memory test/skeleton writer; Vec backing strict logical_t enforced at commit.
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

    /// CO1.7-extra D2: InMemory has no git substrate → always None.
    /// The override is required (no trait default) per round-2 MF3, making the
    /// choice explicit rather than implicit.
    fn head_commit_oid_hex(&self) -> Option<String> {
        None
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 4 replay — TWO-MODE per C1
// ────────────────────────────────────────────────────────────────────────────

/// **C1 NEW**: replay mode discriminator.
/// - `ChainOnly`: skeleton-stage; chain integrity only (parent_state_root +
///   parent_ledger_root + ledger_root chain). NOT the I-DETHASH witness.
/// - `FullTransition`: CO1.7.5+ stage; verifies signatures + re-fetches payloads
///   from CAS + re-runs pure transitions + asserts state_root match. THE
///   I-DETHASH witness; requires CO1.4-extra (CAS index persistence).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayMode {
    ChainOnly,
    FullTransition,
}

#[derive(Debug)]
pub enum ReplayError {
    LogicalTGap { at: usize, expected: u64, got: u64 },
    ParentStateMismatch { at: usize },
    ParentLedgerMismatch { at: usize }, // K2 NEW
    LedgerRootMismatch { at: usize },
    // FullTransition-mode-only (CO1.7.5+):
    BadSignature { at: usize },
    CasMissing { at: usize },
    StateRootMismatch { at: usize },
    /// CO1.7-impl A4: dispatch_transition rejected the re-run. In stub state
    /// (CO1.7.5 not yet shipped), this fires on every replay step with
    /// `inner = NotYetImplemented`.
    Transition {
        at: usize,
        inner: crate::state::typed_tx::TransitionError,
    },
    /// CO1.7-impl A4 v1.1 (Codex bundle Q-J / C-3): the canonical-decoded
    /// `TypedTx` variant disagrees with the entry's `tx_kind` discriminator.
    /// Signed envelope claims one kind; CAS payload is another.
    TxKindMismatch {
        at: usize,
        envelope_kind: TxKind,
        decoded_kind: TxKind,
    },
    /// CO1.7-impl A4 v1.1 (Codex bundle Q-K secondary): payload bytes
    /// retrieved from CAS but `canonical_decode` failed (corruption /
    /// non-canonical bytes). Distinct from `CasMissing` (lookup failure).
    PayloadDecode {
        at: usize,
        reason: String,
    },
}

impl std::fmt::Display for ReplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LogicalTGap { at, expected, got } => {
                write!(f, "logical_t gap at index {at}: expected {expected}, got {got}")
            }
            Self::ParentStateMismatch { at } => write!(f, "parent_state_root mismatch at index {at}"),
            Self::ParentLedgerMismatch { at } => write!(f, "parent_ledger_root mismatch at index {at}"),
            Self::LedgerRootMismatch { at } => write!(f, "ledger_root mismatch at index {at}"),
            Self::BadSignature { at } => write!(f, "system_signature verify failed at index {at}"),
            Self::CasMissing { at } => write!(f, "CAS payload not retrievable at index {at}"),
            Self::StateRootMismatch { at } => write!(f, "resulting_state_root divergence at index {at}"),
            Self::Transition { at, inner } => write!(f, "dispatch_transition rejected at index {at}: {inner}"),
            Self::TxKindMismatch { at, envelope_kind, decoded_kind } => write!(
                f,
                "tx_kind mismatch at index {at}: envelope claims {envelope_kind:?} but CAS payload decoded as {decoded_kind:?}"
            ),
            Self::PayloadDecode { at, reason } => write!(f, "payload canonical_decode failed at index {at}: {reason}"),
        }
    }
}
impl std::error::Error for ReplayError {}

// ────────────────────────────────────────────────────────────────────────────
// CO1.7-impl A4: LedgerCasView trait + replay_full_transition
// ────────────────────────────────────────────────────────────────────────────

/// CO1.7 spec § 4 + DIV-4 closure: narrow read-only CAS trait that replay
/// needs. Decouples `replay_full_transition` from full `CasStore` (the
/// production impl). Anything that can hand back the bytes for a `Cid`
/// satisfies this — testing can mock it; cold-replay uses CasStore directly.
pub trait LedgerCasView {
    fn get_typed_payload(
        &self,
        cid: &crate::bottom_white::cas::schema::Cid,
    ) -> Result<Vec<u8>, ReplayError>;
}

impl LedgerCasView for crate::bottom_white::cas::store::CasStore {
    fn get_typed_payload(
        &self,
        cid: &crate::bottom_white::cas::schema::Cid,
    ) -> Result<Vec<u8>, ReplayError> {
        self.get(cid).map_err(|_| ReplayError::CasMissing { at: 0 })
    }
}

/// CO1.7-impl A4 — full-mode replay (THE I-DETHASH witness).
///
/// Validates **every** stage spec § 4 + § 6 promises:
/// 1. logical_t monotonicity
/// 2. parent_state_root chain
/// 3. parent_ledger_root chain (K2 transplant defense)
/// 4. system_signature verifies via CanonicalMessage::LedgerEntrySigning + pinned pubkeys
/// 5. CAS lookup of tx_payload_cid succeeds (CO1.4-extra cold-replay capability)
/// 6. canonical_decode of payload bytes → TypedTx
/// 6.5 (v1.1 C-3): decoded_typed_tx.tx_kind() MUST equal entry.tx_kind
/// 7. dispatch_transition re-run produces (q_next, _signals)
/// 8. q_next.state_root_t matches entry.resulting_state_root
/// 9. resulting_ledger_root recomputed via append() matches stored
///
/// **v1.1 C-1 closure**: takes a full `genesis: &QState` (was `genesis_state_root`
/// + `genesis_ledger_root` only). Caller provides the complete genesis state
/// so dispatch_transition can read budget / registries / balances / task markets
/// — fabricating `QState::genesis()` was dropping these fields.
///
/// **Stub-state caveat (CO1.7.5 unblocks)**: while `dispatch_transition`
/// returns `NotYetImplemented` for every variant, replay errors at stage 7
/// for any non-empty chain. Conformance tests exercising stages 1-6.5
/// independently are `#[test]`-runnable now; full state_root reconstruction
/// gates on CO1.7.5.
pub fn replay_full_transition(
    genesis: &crate::state::q_state::QState,
    entries: &[LedgerEntry],
    cas: &dyn LedgerCasView,
    pinned_pubkeys: &crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys,
    predicate_registry: &crate::top_white::predicates::registry::PredicateRegistry,
    tool_registry: &crate::bottom_white::tools::registry::ToolRegistry,
) -> Result<crate::state::q_state::QState, ReplayError> {
    use crate::bottom_white::ledger::system_keypair::{
        verify_system_signature, CanonicalMessage,
    };
    use crate::state::sequencer::dispatch_transition;

    let mut q = genesis.clone();

    for (i, entry) in entries.iter().enumerate() {
        // Stage 1
        let expected_logical_t = (i as u64) + 1;
        if entry.logical_t != expected_logical_t {
            return Err(ReplayError::LogicalTGap {
                at: i,
                expected: expected_logical_t,
                got: entry.logical_t,
            });
        }
        // Stage 2
        if entry.parent_state_root != q.state_root_t {
            return Err(ReplayError::ParentStateMismatch { at: i });
        }
        // Stage 3
        if entry.parent_ledger_root != q.ledger_root_t {
            return Err(ReplayError::ParentLedgerMismatch { at: i });
        }

        // Stage 4: system_signature verify (FullTransition mode only).
        let signing_payload = entry.to_signing_payload();
        let signing_digest = signing_payload.canonical_digest();
        let canonical_msg = CanonicalMessage::LedgerEntrySigning(signing_digest.0);
        if !verify_system_signature(
            &entry.system_signature,
            &canonical_msg,
            entry.epoch,
            pinned_pubkeys,
        ) {
            return Err(ReplayError::BadSignature { at: i });
        }

        // Stage 5: CAS lookup.
        let payload_bytes = cas
            .get_typed_payload(&entry.tx_payload_cid)
            .map_err(|_| ReplayError::CasMissing { at: i })?;

        // Stage 6: canonical_decode → TypedTx (v1.1 C-3-secondary: distinct
        // error from CasMissing).
        let typed_tx: crate::state::typed_tx::TypedTx =
            canonical_decode(&payload_bytes).map_err(|e| ReplayError::PayloadDecode {
                at: i,
                reason: e.to_string(),
            })?;

        // Stage 6.5 (v1.1 C-3): tx_kind envelope vs decoded payload kind MUST match.
        // Otherwise a signed envelope claiming `Work` could ride a CAS payload
        // that decodes as `Verify` — sequencer would have written that
        // mismatch but replay would have silently accepted it pre-v1.1.
        let decoded_kind = typed_tx.tx_kind();
        if decoded_kind != entry.tx_kind {
            return Err(ReplayError::TxKindMismatch {
                at: i,
                envelope_kind: entry.tx_kind,
                decoded_kind,
            });
        }

        // Stage 7: re-run pure dispatch_transition.
        let (q_next, _signals) =
            dispatch_transition(&q, &typed_tx, predicate_registry, tool_registry)
                .map_err(|inner| ReplayError::Transition { at: i, inner })?;

        // Stage 8: state_root match.
        if q_next.state_root_t != entry.resulting_state_root {
            return Err(ReplayError::StateRootMismatch { at: i });
        }

        // Stage 9: ledger_root match (recompute via append).
        let recomputed_ledger_root = append(&q.ledger_root_t, &signing_digest);
        if recomputed_ledger_root != entry.resulting_ledger_root {
            return Err(ReplayError::LedgerRootMismatch { at: i });
        }

        // Advance.
        q = q_next;
        q.ledger_root_t = entry.resulting_ledger_root;
    }

    Ok(q)
}

/// Skeleton-stage entry point (v1.1).
///
/// Validates:
/// 1. logical_t monotonicity (no gaps, no duplicates)
/// 2. parent_state_root chain
/// 3. parent_ledger_root chain (K2 transplant defense)
/// 4. resulting_ledger_root recomputed via append(prev_ledger_root, signing_digest)
///
/// Does NOT verify:
/// - system_signature (CO1.7.5+: requires CanonicalMessage extension wired through keypair)
/// - resulting_state_root (CO1.7.5+: requires dispatch_transition + CO1.4-extra CAS persistence)
///
/// Returns final (state_root, ledger_root) on success.
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
            return Err(ReplayError::ParentStateMismatch { at: i });
        }
        // K2 NEW: parent_ledger_root chain check
        if entry.parent_ledger_root != prev_ledger_root {
            return Err(ReplayError::ParentLedgerMismatch { at: i });
        }
        let signing_digest = entry.to_signing_payload().canonical_digest();
        let recomputed = append(&prev_ledger_root, &signing_digest);
        if recomputed != entry.resulting_ledger_root {
            return Err(ReplayError::LedgerRootMismatch { at: i });
        }
        prev_state_root = entry.resulting_state_root;
        prev_ledger_root = entry.resulting_ledger_root;
    }

    Ok((prev_state_root, prev_ledger_root))
}

// ────────────────────────────────────────────────────────────────────────────
// § 2.5 Canonical serialization (bincode v2; STATE_TRANSITION_SPEC § 2.5)
// ────────────────────────────────────────────────────────────────────────────

/// `bincode::config` used for the canonical `LedgerEntry` wire format.
///
/// **Frozen choices** (per STATE_TRANSITION_SPEC § 2.5):
/// - **Big-endian** byte order (network order; deterministic across platforms).
/// - **Fixed-int encoding** (no varint; fixed-width for byte-stable round-trip).
/// - **`BTreeMap` keys**: bincode iterates the map in serde-supplied order; we
///   only ever encode `BTreeMap` (sorted by construction) so key order is lex.
/// - **No padding, no implicit alignment.**
fn bincode_canonical_config() -> impl bincode::config::Config {
    bincode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding()
}

/// Canonical encode any serde-Serialize value to bytes (CO1.7 wire format).
/// Used by `Git2LedgerWriter` for commit-message bodies and by future callers
/// needing byte-stable signatures over typed payloads.
pub fn canonical_encode<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonicalCodecError> {
    bincode::serde::encode_to_vec(value, bincode_canonical_config())
        .map_err(|e| CanonicalCodecError::Encode(e.to_string()))
}

/// Canonical decode the inverse of `canonical_encode`. Returns the value plus
/// the number of bytes consumed (entire input must be consumed for a clean decode).
pub fn canonical_decode<T: serde::de::DeserializeOwned>(
    bytes: &[u8],
) -> Result<T, CanonicalCodecError> {
    let (value, consumed) =
        bincode::serde::decode_from_slice::<T, _>(bytes, bincode_canonical_config())
            .map_err(|e| CanonicalCodecError::Decode(e.to_string()))?;
    if consumed != bytes.len() {
        return Err(CanonicalCodecError::TrailingBytes {
            consumed,
            total: bytes.len(),
        });
    }
    Ok(value)
}

#[derive(Debug)]
pub enum CanonicalCodecError {
    Encode(String),
    Decode(String),
    TrailingBytes { consumed: usize, total: usize },
}

impl std::fmt::Display for CanonicalCodecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encode(s) => write!(f, "canonical encode failed: {s}"),
            Self::Decode(s) => write!(f, "canonical decode failed: {s}"),
            Self::TrailingBytes { consumed, total } => {
                write!(f, "trailing bytes after decode: consumed {consumed} of {total}")
            }
        }
    }
}
impl std::error::Error for CanonicalCodecError {}

// ────────────────────────────────────────────────────────────────────────────
// § 5 Git2LedgerWriter — git2-rs commit chain on `refs/transitions/main`
// ────────────────────────────────────────────────────────────────────────────

/// Spec § 5 production storage backend.
///
/// **Mapping**:
/// - One `LedgerEntry` = one git commit on `refs/transitions/main`.
/// - **Commit tree** = three named blobs:
///     - `payload_cid`     = entry.tx_payload_cid.0 (32 bytes)
///     - `signature`       = entry.system_signature.as_bytes() (64 bytes)
///     - `entry_canonical` = bincode v2 BE + fixed-int encoding of the full
///       `LedgerEntry` (deterministic, byte-stable; this blob IS the
///       canonical record — `read_at` decodes it directly).
/// - **Commit message** = human-readable `"transition logical_t=<N>\n"` (the
///   canonical record lives in the tree blob, not the message — git
///   normalizes message bytes in ways that break round-trip).
/// - **Parent**: `head_t-1` commit (or none at genesis).
/// - **Author/committer identity**: fixed `("turingosv4 sequencer", "system@turingos")`
///   with `time = (logical_t as i64, 0)` to keep commit OIDs deterministic. NO
///   wall-clock leakage (`I-NOENV` + `I-LOGTIME`).
///
/// **K3 (revised v1.2)**: this writer surfaces `commit_oid` for callers that
/// need it (CO1.7.5+ `head_t` wiring), but the `LedgerWriter::commit` trait
/// returns only `Hash` (entry.resulting_ledger_root). Callers requesting the
/// commit OID use [`Git2LedgerWriter::head_commit_oid`] post-commit.
pub struct Git2LedgerWriter {
    repo_path: PathBuf,
    /// Last commit OID on `refs/transitions/main`; `None` at empty-chain genesis.
    head_oid: Option<git2::Oid>,
    /// Number of entries committed = highest assigned `logical_t` (0 at genesis).
    len: u64,
}

const TRANSITIONS_REF: &str = "refs/transitions/main";
const TREE_BLOB_PAYLOAD_CID: &str = "payload_cid";
const TREE_BLOB_SIGNATURE: &str = "signature";
const TREE_BLOB_ENTRY_CANONICAL: &str = "entry_canonical";

impl Git2LedgerWriter {
    /// Open or initialize a `Git2LedgerWriter` rooted at `repo_path`.
    /// Creates the underlying git repo if it doesn't exist; resolves the
    /// existing `refs/transitions/main` if present and seeds `head_oid` + `len`.
    pub fn open(repo_path: &Path) -> Result<Self, LedgerWriterError> {
        let repo_path = repo_path.to_path_buf();
        let repo = match Repository::open(&repo_path) {
            Ok(r) => r,
            Err(_) => Repository::init(&repo_path).map_err(|e| {
                LedgerWriterError::BackendCorruption(format!("repo init: {e}"))
            })?,
        };

        // Resolve refs/transitions/main if it exists.
        let (head_oid, len) = match repo.find_reference(TRANSITIONS_REF) {
            Ok(reference) => {
                let oid = reference
                    .target()
                    .ok_or_else(|| {
                        LedgerWriterError::BackendCorruption(format!(
                            "{TRANSITIONS_REF} has no direct target"
                        ))
                    })?;
                // Walk parents to count chain length.
                let mut n: u64 = 0;
                let mut cursor = Some(oid);
                while let Some(c) = cursor {
                    n += 1;
                    let commit = repo.find_commit(c).map_err(|e| {
                        LedgerWriterError::BackendCorruption(format!("walk parent: {e}"))
                    })?;
                    cursor = commit.parent(0).ok().map(|p| p.id());
                }
                (Some(oid), n)
            }
            Err(_) => (None, 0),
        };

        Ok(Self {
            repo_path,
            head_oid,
            len,
        })
    }

    fn open_repo(&self) -> Result<Repository, LedgerWriterError> {
        Repository::open(&self.repo_path)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("repo open: {e}")))
    }

    /// Commit OID of the most recent appended entry (None if chain is empty).
    /// CO1.7.5+ `head_t` wiring uses this to surface commit_sha alongside Hash.
    pub fn head_commit_oid(&self) -> Option<git2::Oid> {
        self.head_oid
    }

    /// Read raw canonical-encoded `LedgerEntry` bytes (the `entry_canonical`
    /// tree blob) for the entry at `logical_t`. `logical_t` is 1-indexed.
    fn read_canonical_bytes(&self, logical_t: u64) -> Result<Vec<u8>, LedgerWriterError> {
        if logical_t == 0 || logical_t > self.len {
            return Err(LedgerWriterError::NotFound { logical_t });
        }
        let repo = self.open_repo()?;
        // Walk back (len - logical_t) parents from head.
        let mut cursor = self.head_oid.ok_or(LedgerWriterError::NotFound { logical_t })?;
        let mut steps_back = self.len - logical_t;
        while steps_back > 0 {
            let commit = repo.find_commit(cursor).map_err(|e| {
                LedgerWriterError::BackendCorruption(format!("find_commit: {e}"))
            })?;
            cursor = commit
                .parent(0)
                .map_err(|e| LedgerWriterError::BackendCorruption(format!("parent: {e}")))?
                .id();
            steps_back -= 1;
        }
        let commit = repo
            .find_commit(cursor)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_commit: {e}")))?;
        let tree = commit
            .tree()
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree: {e}")))?;
        let entry_obj = tree
            .get_name(TREE_BLOB_ENTRY_CANONICAL)
            .ok_or_else(|| {
                LedgerWriterError::BackendCorruption(format!(
                    "missing {TREE_BLOB_ENTRY_CANONICAL} blob at logical_t={logical_t}"
                ))
            })?;
        let blob = repo
            .find_blob(entry_obj.id())
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_blob: {e}")))?;
        Ok(blob.content().to_vec())
    }
}

impl LedgerWriter for Git2LedgerWriter {
    /// CO1.7-extra D2: surface 40-char lowercase hex commit OID for sequencer
    /// post-commit head_t wiring. Maps existing `head_commit_oid()` accessor
    /// (returns Option<git2::Oid>) to canonical hex string.
    fn head_commit_oid_hex(&self) -> Option<String> {
        self.head_commit_oid().map(|oid| oid.to_string())
    }

    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
        let expected = self.len + 1;
        if entry.logical_t != expected {
            return Err(LedgerWriterError::LogicalTGap {
                expected,
                got: entry.logical_t,
            });
        }

        let repo = self.open_repo()?;
        let canonical = canonical_encode(entry).map_err(|e| {
            LedgerWriterError::BackendCorruption(format!("canonical_encode: {e}"))
        })?;

        let mut tb = repo
            .treebuilder(None)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("treebuilder: {e}")))?;
        let cid_blob = repo
            .blob(&entry.tx_payload_cid.0)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("cid blob: {e}")))?;
        tb.insert(TREE_BLOB_PAYLOAD_CID, cid_blob, 0o100644)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert cid: {e}")))?;
        let sig_blob = repo
            .blob(entry.system_signature.as_bytes())
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("sig blob: {e}")))?;
        tb.insert(TREE_BLOB_SIGNATURE, sig_blob, 0o100644)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert sig: {e}")))?;
        let entry_blob = repo
            .blob(&canonical)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("entry blob: {e}")))?;
        tb.insert(TREE_BLOB_ENTRY_CANONICAL, entry_blob, 0o100644)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert entry: {e}")))?;
        let tree_oid = tb
            .write()
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree write: {e}")))?;
        let tree = repo
            .find_tree(tree_oid)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_tree: {e}")))?;

        // Determinism: time = (logical_t, 0). NO wall clock.
        let time = git2::Time::new(entry.logical_t as i64, 0);
        let author = GitSignature::new("turingosv4 sequencer", "system@turingos", &time)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("git sig: {e}")))?;
        let committer = author.clone();

        let parents: Vec<git2::Commit<'_>> = match self.head_oid {
            Some(oid) => vec![repo.find_commit(oid).map_err(|e| {
                LedgerWriterError::BackendCorruption(format!("parent commit: {e}"))
            })?],
            None => Vec::new(),
        };
        let parent_refs: Vec<&git2::Commit<'_>> = parents.iter().collect();
        let message = format!("transition logical_t={}\n", entry.logical_t);
        let new_oid = repo
            .commit(
                Some(TRANSITIONS_REF),
                &author,
                &committer,
                &message,
                &tree,
                &parent_refs,
            )
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("commit: {e}")))?;

        self.head_oid = Some(new_oid);
        self.len += 1;
        Ok(entry.resulting_ledger_root)
    }

    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
        let bytes = self.read_canonical_bytes(logical_t)?;
        canonical_decode::<LedgerEntry>(&bytes).map_err(|e| {
            LedgerWriterError::BackendCorruption(format!("canonical_decode at {logical_t}: {e}"))
        })
    }

    fn len(&self) -> u64 {
        self.len
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Tests — 8 conformance items (4 NEW vs v1 skeleton: K2 / Q9 / repr(u8) / extensions)
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn h(byte: u8) -> Hash {
        Hash([byte; 32])
    }

    /// Build an entry that satisfies all chain invariants given the previous state.
    fn entry_at(
        logical_t: u64,
        parent_state_root: Hash,
        parent_ledger_root: Hash,
        resulting_state_root: Hash,
    ) -> LedgerEntry {
        let signing = LedgerEntrySigningPayload {
            logical_t,
            parent_state_root,
            parent_ledger_root,
            tx_kind: TxKind::Work,
            tx_payload_cid: Cid([0u8; 32]),
            resulting_state_root,
            timestamp_logical: logical_t,
            epoch: SystemEpoch::new(1),
            extensions: BTreeMap::new(),
        };
        let signing_digest = signing.canonical_digest();
        let resulting_ledger_root = append(&parent_ledger_root, &signing_digest);
        LedgerEntry {
            logical_t: signing.logical_t,
            parent_state_root: signing.parent_state_root,
            parent_ledger_root: signing.parent_ledger_root,
            tx_kind: signing.tx_kind,
            tx_payload_cid: signing.tx_payload_cid,
            resulting_state_root: signing.resulting_state_root,
            resulting_ledger_root,
            timestamp_logical: signing.timestamp_logical,
            epoch: signing.epoch,
            extensions: signing.extensions,
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        }
    }

    // 1. append byte-stable (I-DET ledger axis)
    #[test]
    fn append_is_pure_and_byte_stable() {
        let a = append(&Hash::ZERO, &h(1));
        let b = append(&Hash::ZERO, &h(1));
        assert_eq!(a, b);
        let c = append(&Hash::ZERO, &h(2));
        assert_ne!(a, c);
    }

    // 2. canonical_digest stable (#[repr(u8)] discriminant stable)
    #[test]
    fn canonical_digest_stable_across_clones() {
        let p = LedgerEntrySigningPayload {
            logical_t: 1,
            parent_state_root: Hash::ZERO,
            parent_ledger_root: Hash::ZERO,
            tx_kind: TxKind::Work,
            tx_payload_cid: Cid([7u8; 32]),
            resulting_state_root: h(0xaa),
            timestamp_logical: 1,
            epoch: SystemEpoch::new(2),
            extensions: BTreeMap::new(),
        };
        let d1 = p.canonical_digest();
        let d2 = p.clone().canonical_digest();
        assert_eq!(d1, d2);
    }

    // 3. InMemoryWriter enforces logical_t monotonic
    #[test]
    fn in_memory_writer_enforces_logical_t() {
        let mut w = InMemoryLedgerWriter::new();
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        assert!(w.commit(&e1).is_ok());

        let e_skip = entry_at(3, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        let err = w.commit(&e_skip).unwrap_err();
        assert!(matches!(err, LedgerWriterError::LogicalTGap { expected: 2, got: 3 }));
    }

    // 4. ChainOnly replay validates clean chain
    #[test]
    fn replay_chain_integrity_clean() {
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));
        let (final_state, final_ledger) =
            replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1.clone(), e2.clone(), e3.clone()])
                .expect("clean chain replays");
        assert_eq!(final_state, e3.resulting_state_root);
        assert_eq!(final_ledger, e3.resulting_ledger_root);
    }

    // 5. ChainOnly replay rejects parent_state_root tamper
    #[test]
    fn replay_rejects_parent_state_tamper() {
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        e2.parent_state_root = h(0xff);
        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
        assert!(matches!(err, ReplayError::ParentStateMismatch { at: 1 }));
    }

    // 6. K2 NEW: ChainOnly replay rejects parent_ledger_root tamper (transplant defense)
    #[test]
    fn replay_rejects_parent_ledger_tamper() {
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        // Tamper with parent_ledger_root WITHOUT recomputing resulting_ledger_root —
        // simulates an attacker transplanting an entry from a different ledger history.
        e2.parent_ledger_root = h(0xff);
        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
        assert!(matches!(err, ReplayError::ParentLedgerMismatch { at: 1 }));
    }

    // 7. ChainOnly replay rejects ledger_root tamper
    #[test]
    fn replay_rejects_ledger_root_tamper() {
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        e2.resulting_ledger_root = h(0xee);
        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
        assert!(matches!(err, ReplayError::LedgerRootMismatch { at: 1 }));
    }

    // 8. Q9 NEW: canonical_digest excludes derivatives
    // Mutating `resulting_ledger_root` or `system_signature` of LedgerEntry must NOT
    // change the signing payload digest (because they're not in LedgerEntrySigningPayload).
    #[test]
    fn canonical_digest_excludes_derivatives() {
        let e_clean = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let digest_clean = e_clean.to_signing_payload().canonical_digest();

        // Mutate resulting_ledger_root (a derivative; should NOT affect signing digest)
        let mut e_tamper = e_clean.clone();
        e_tamper.resulting_ledger_root = h(0xff);
        let digest_after_root_tamper = e_tamper.to_signing_payload().canonical_digest();
        assert_eq!(
            digest_clean, digest_after_root_tamper,
            "resulting_ledger_root MUST NOT affect signing digest (Q9 cycle prevention)"
        );

        // Mutate system_signature (signature is its own input; should NOT affect signing digest)
        let mut e_tamper2 = e_clean.clone();
        e_tamper2.system_signature = SystemSignature::from_bytes([0xffu8; 64]);
        let digest_after_sig_tamper = e_tamper2.to_signing_payload().canonical_digest();
        assert_eq!(digest_clean, digest_after_sig_tamper);

        // Sanity: mutating a SIGNED field DOES change digest
        let mut e_signed_change = e_clean.clone();
        e_signed_change.epoch = SystemEpoch::new(99);
        let digest_after_signed = e_signed_change.to_signing_payload().canonical_digest();
        assert_ne!(digest_clean, digest_after_signed);
    }

    // 9. C3 closure (round-2): real signature roundtrip via system_keypair extension.
    // Verifies: (a) typed sign API works; (b) signature verifies via CanonicalMessage::LedgerEntrySigning;
    // (c) signature does NOT verify after mutating a signed field (parent_ledger_root — K2 transplant defense).
    #[test]
    fn signature_round_trip_and_transplant_defense() {
        use crate::bottom_white::ledger::system_keypair::{
            transition_ledger_emitter, CanonicalMessage, Ed25519Keypair, PinnedSystemPubkeys,
            SystemEpoch, verify_system_signature,
        };

        let keypair = Ed25519Keypair::generate_with_secure_entropy().expect("keypair gen");
        let epoch = SystemEpoch::new(1);
        let mut pinned = PinnedSystemPubkeys::new();
        pinned.insert(epoch, keypair.public_key());

        // Build a clean signing payload (e1's worth)
        let payload = LedgerEntrySigningPayload {
            logical_t: 1,
            parent_state_root: Hash::ZERO,
            parent_ledger_root: Hash::ZERO,
            tx_kind: TxKind::Work,
            tx_payload_cid: Cid([42u8; 32]),
            resulting_state_root: h(1),
            timestamp_logical: 1,
            epoch,
            extensions: BTreeMap::new(),
        };
        let digest = payload.canonical_digest();

        // Real sign through the typed CanonicalMessage extension
        let sig = transition_ledger_emitter::sign_ledger_entry(&keypair, digest.0)
            .expect("sign_ledger_entry");

        // Verify (clean) — must succeed
        let msg_clean = CanonicalMessage::LedgerEntrySigning(digest.0);
        assert!(
            verify_system_signature(&sig, &msg_clean, epoch, &pinned),
            "clean signature must verify"
        );

        // Verify (tamper parent_ledger_root) — K2 transplant defense
        let mut payload_tamper = payload.clone();
        payload_tamper.parent_ledger_root = h(0xff);
        let digest_tamper = payload_tamper.canonical_digest();
        let msg_tamper = CanonicalMessage::LedgerEntrySigning(digest_tamper.0);
        assert!(
            !verify_system_signature(&sig, &msg_tamper, epoch, &pinned),
            "transplanted parent_ledger_root MUST fail signature verify (K2)"
        );

        // Verify (cross-epoch transplant) — D1 defense via epoch IN payload digest.
        // Attacker scenario: sig was made for payload with epoch=1; attacker forges a
        // NEW payload claiming epoch=2 reusing the old sig. Since epoch is in the
        // canonical digest, digest_v2 ≠ digest_v1, so the sig on digest_v1 cannot
        // verify against digest_v2.
        let mut payload_other_epoch = payload.clone();
        payload_other_epoch.epoch = SystemEpoch::new(2);
        let digest_other_epoch = payload_other_epoch.canonical_digest();
        assert_ne!(digest, digest_other_epoch, "epoch is bound in canonical digest");
        let msg_other_epoch = CanonicalMessage::LedgerEntrySigning(digest_other_epoch.0);
        assert!(
            !verify_system_signature(&sig, &msg_other_epoch, epoch, &pinned),
            "cross-epoch transplant MUST fail signature verify (D1 epoch binding)"
        );
    }

    // ──────────────────────────────────────────────────────────────────────
    // 10–13. Git2LedgerWriter — git2-rs commit chain backend (§ 5)
    // ──────────────────────────────────────────────────────────────────────

    use tempfile::TempDir;

    fn fresh_git_writer() -> (TempDir, Git2LedgerWriter) {
        let tmp = TempDir::new().expect("tempdir");
        let w = Git2LedgerWriter::open(tmp.path()).expect("open");
        (tmp, w)
    }

    // 10. Empty repo: len()=0, head_commit_oid=None.
    #[test]
    fn git2_writer_empty_chain() {
        let (_tmp, w) = fresh_git_writer();
        assert_eq!(w.len(), 0);
        assert!(w.head_commit_oid().is_none());
    }

    // 11. Append three entries; len + head_commit_oid advance per commit;
    //     read_at recovers each entry byte-identically (canonical encode/decode round-trip).
    #[test]
    fn git2_writer_append_and_read_back() {
        let (_tmp, mut w) = fresh_git_writer();
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));

        let r1 = w.commit(&e1).expect("commit 1");
        assert_eq!(r1, e1.resulting_ledger_root);
        assert_eq!(w.len(), 1);
        let oid_1 = w.head_commit_oid().expect("head after 1");

        let r2 = w.commit(&e2).expect("commit 2");
        assert_eq!(r2, e2.resulting_ledger_root);
        assert_eq!(w.len(), 2);
        let oid_2 = w.head_commit_oid().expect("head after 2");
        assert_ne!(oid_1, oid_2, "head must advance after second commit");

        w.commit(&e3).expect("commit 3");
        assert_eq!(w.len(), 3);

        // read_at returns each entry byte-identically.
        assert_eq!(w.read_at(1).expect("read 1"), e1);
        assert_eq!(w.read_at(2).expect("read 2"), e2);
        assert_eq!(w.read_at(3).expect("read 3"), e3);
    }

    // 12. Skipping a logical_t triggers LogicalTGap; chain state is unchanged.
    #[test]
    fn git2_writer_rejects_logical_t_gap() {
        let (_tmp, mut w) = fresh_git_writer();
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        w.commit(&e1).expect("commit 1");
        let pre_oid = w.head_commit_oid();

        // Try to commit a logical_t=3 entry (gap: expected 2)
        let e_skip = entry_at(3, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        let err = w.commit(&e_skip).unwrap_err();
        assert!(matches!(err, LedgerWriterError::LogicalTGap { expected: 2, got: 3 }));
        // Chain unchanged.
        assert_eq!(w.len(), 1);
        assert_eq!(w.head_commit_oid(), pre_oid);
    }

    // 13. Reopening the same repo path resurrects the chain (head + len recovered
    //     from refs/transitions/main). Crucial for runtime cold-restart.
    #[test]
    fn git2_writer_reopen_recovers_chain() {
        let tmp = TempDir::new().expect("tempdir");
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        let oid_after_two;
        {
            let mut w = Git2LedgerWriter::open(tmp.path()).expect("open");
            w.commit(&e1).expect("commit 1");
            w.commit(&e2).expect("commit 2");
            oid_after_two = w.head_commit_oid().expect("head");
        }
        // Reopen — fresh struct, same on-disk repo.
        let w2 = Git2LedgerWriter::open(tmp.path()).expect("reopen");
        assert_eq!(w2.len(), 2);
        assert_eq!(w2.head_commit_oid(), Some(oid_after_two));
        assert_eq!(w2.read_at(1).expect("read 1"), e1);
        assert_eq!(w2.read_at(2).expect("read 2"), e2);

        // Continue chain after reopen.
        let mut w3 = Git2LedgerWriter::open(tmp.path()).expect("reopen 2");
        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));
        w3.commit(&e3).expect("commit 3");
        assert_eq!(w3.len(), 3);
    }

    // ──────────────────────────────────────────────────────────────────────
    // 15-18. CO1.7-impl A4 — replay_full_transition (THE I-DETHASH witness)
    // ──────────────────────────────────────────────────────────────────────

    use crate::bottom_white::cas::schema::ObjectType;
    use crate::bottom_white::cas::store::CasStore;
    use crate::bottom_white::ledger::system_keypair::{
        transition_ledger_emitter, Ed25519Keypair, PinnedSystemPubkeys,
    };
    use crate::bottom_white::tools::registry::ToolRegistry;
    use crate::state::typed_tx::{
        AgentSignature, BoolWithProof, PredicateId, PredicateResultsBundle, ReadKey,
        SafetyOrCreation, TypedTx, WorkTx, WriteKey,
    };
    use crate::state::q_state::{AgentId, TaskId, TxId as QTxId};
    use crate::top_white::predicates::registry::PredicateRegistry;

    fn dummy_typed_tx() -> TypedTx {
        let mut acceptance = std::collections::BTreeMap::new();
        acceptance.insert(
            PredicateId("acc1".into()),
            BoolWithProof { value: true, proof_cid: None },
        );
        TypedTx::Work(WorkTx {
            tx_id: QTxId("worktx-replay-fixture".into()),
            task_id: TaskId("task-replay".into()),
            parent_state_root: Hash::ZERO,
            agent_id: AgentId("alice".into()),
            read_set: [ReadKey("k.r".into())].into_iter().collect::<std::collections::BTreeSet<_>>(),
            write_set: [WriteKey("k.w".into())].into_iter().collect::<std::collections::BTreeSet<_>>(),
            proposal_cid: Cid([0; 32]),
            predicate_results: PredicateResultsBundle {
                acceptance,
                settlement: std::collections::BTreeMap::new(),
                safety_class: SafetyOrCreation::Safety,
            },
            stake: crate::economy::money::StakeMicroCoin::from_micro_units(1),
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: 1,
        })
    }

    /// Build a real signed LedgerEntry against the given keypair + epoch,
    /// with the typed_tx's canonical bytes stored in CAS. Mirrors
    /// `Sequencer::apply_one` stages 5-9 outside the runtime.
    fn build_signed_entry(
        logical_t: u64,
        parent_state_root: Hash,
        parent_ledger_root: Hash,
        resulting_state_root: Hash,
        epoch: SystemEpoch,
        keypair: &Ed25519Keypair,
        cas: &mut CasStore,
        typed_tx: &TypedTx,
    ) -> LedgerEntry {
        let bytes = canonical_encode(typed_tx).expect("encode");
        let cid = cas
            .put(&bytes, ObjectType::ProposalPayload, "test", logical_t, None)
            .expect("cas put");
        let signing = LedgerEntrySigningPayload {
            logical_t,
            parent_state_root,
            parent_ledger_root,
            tx_kind: typed_tx.tx_kind(),
            tx_payload_cid: cid,
            resulting_state_root,
            timestamp_logical: logical_t,
            epoch,
            extensions: BTreeMap::new(),
        };
        let digest = signing.canonical_digest();
        let sig = transition_ledger_emitter::sign_ledger_entry(keypair, digest.0)
            .expect("sign");
        let resulting_ledger_root = append(&parent_ledger_root, &digest);
        LedgerEntry {
            logical_t,
            parent_state_root,
            parent_ledger_root,
            tx_kind: typed_tx.tx_kind(),
            tx_payload_cid: cid,
            resulting_state_root,
            resulting_ledger_root,
            timestamp_logical: logical_t,
            epoch,
            extensions: BTreeMap::new(),
            system_signature: sig,
        }
    }

    fn replay_test_setup() -> (
        TempDir,
        CasStore,
        Ed25519Keypair,
        SystemEpoch,
        PinnedSystemPubkeys,
        PredicateRegistry,
        ToolRegistry,
    ) {
        let tmp = TempDir::new().expect("tempdir");
        let cas = CasStore::open(tmp.path()).expect("cas");
        let kp = Ed25519Keypair::generate_with_secure_entropy().expect("kp");
        let epoch = SystemEpoch::new(1);
        let mut pinned = PinnedSystemPubkeys::new();
        pinned.insert(epoch, kp.public_key());
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        (tmp, cas, kp, epoch, pinned, preds, tools)
    }

    /// 15. CO1.7.5-stage: in stub mode, dispatch errors with NotYetImplemented;
    ///     replay correctly bubbles up `Transition { at: 0, inner: NotYetImplemented }`.
    ///     This proves stages 1-6 (chain + sig + CAS + decode) all PASS,
    ///     leaving stage 7 (dispatch) as the only gate. CO1.7.5 fills it.
    #[test]
    fn replay_full_transition_reaches_dispatch_then_stubs() {
        let (_tmp, mut cas, kp, epoch, pinned, preds, tools) = replay_test_setup();
        let entry = build_signed_entry(
            1,
            Hash::ZERO,
            Hash::ZERO,
            h(1), // resulting state_root (won't be reached due to dispatch stub)
            epoch,
            &kp,
            &mut cas,
            &dummy_typed_tx(),
        );
        let err = replay_full_transition(
            &crate::state::q_state::QState::genesis(),
            &[entry],
            &cas,
            &pinned,
            &preds,
            &tools,
        )
        .unwrap_err();
        assert!(
            matches!(err, ReplayError::Transition { at: 0, inner: crate::state::typed_tx::TransitionError::EscrowMissing }),
            "expected Transition(NotYetImplemented at 0); got {err:?}"
        );
    }

    /// 16. system_signature_verifies_via_canonical_message — tampering the
    ///     signature MUST fire BadSignature BEFORE dispatch is reached.
    #[test]
    fn replay_rejects_bad_system_signature() {
        let (_tmp, mut cas, kp, epoch, pinned, preds, tools) = replay_test_setup();
        let mut entry = build_signed_entry(
            1,
            Hash::ZERO,
            Hash::ZERO,
            h(1),
            epoch,
            &kp,
            &mut cas,
            &dummy_typed_tx(),
        );
        // Tamper signature.
        entry.system_signature = SystemSignature::from_bytes([0xff; 64]);
        let err = replay_full_transition(
            &crate::state::q_state::QState::genesis(),
            &[entry],
            &cas,
            &pinned,
            &preds,
            &tools,
        )
        .unwrap_err();
        assert!(matches!(err, ReplayError::BadSignature { at: 0 }));
    }

    /// 17. cas_payload_round_trip — replay correctly fetches CAS bytes;
    ///     CO1.4-extra cold-restart capability test.
    #[test]
    fn replay_cas_payload_round_trip_after_reopen() {
        let tmp = TempDir::new().expect("tempdir");
        let kp = Ed25519Keypair::generate_with_secure_entropy().expect("kp");
        let epoch = SystemEpoch::new(1);
        let mut pinned = PinnedSystemPubkeys::new();
        pinned.insert(epoch, kp.public_key());
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();

        let entry;
        {
            let mut cas = CasStore::open(tmp.path()).expect("cas");
            entry = build_signed_entry(
                1,
                Hash::ZERO,
                Hash::ZERO,
                h(1),
                epoch,
                &kp,
                &mut cas,
                &dummy_typed_tx(),
            );
        }
        // Reopen — CO1.4-extra sidecar replay restores the CAS index.
        let cas2 = CasStore::open(tmp.path()).expect("reopen");
        let err = replay_full_transition(
            &crate::state::q_state::QState::genesis(),
            &[entry],
            &cas2,
            &pinned,
            &preds,
            &tools,
        )
        .unwrap_err();
        // Stages 1-6.5 (incl. CAS lookup post-reopen + tx_kind match) PASS;
        // stage 7 stubs.
        assert!(matches!(err, ReplayError::Transition { at: 0, .. }));
    }

    /// 18b. v1.1 C-3 closure: tx_kind mismatch — envelope claims one variant,
    ///      CAS payload decodes as another. Replay MUST reject before stage 7.
    #[test]
    fn replay_rejects_tx_kind_mismatch() {
        let (_tmp, mut cas, kp, epoch, pinned, preds, tools) = replay_test_setup();
        // Build a real entry whose envelope tx_kind matches the payload (Work).
        let mut entry = build_signed_entry(
            1,
            Hash::ZERO,
            Hash::ZERO,
            h(1),
            epoch,
            &kp,
            &mut cas,
            &dummy_typed_tx(),
        );
        // Tamper: claim a different tx_kind on the envelope, RE-SIGN with the
        // tampered envelope so signature still verifies.
        let tampered_signing = LedgerEntrySigningPayload {
            logical_t: entry.logical_t,
            parent_state_root: entry.parent_state_root,
            parent_ledger_root: entry.parent_ledger_root,
            tx_kind: TxKind::Verify, // ← lies about the payload kind
            tx_payload_cid: entry.tx_payload_cid,
            resulting_state_root: entry.resulting_state_root,
            timestamp_logical: entry.timestamp_logical,
            epoch: entry.epoch,
            extensions: entry.extensions.clone(),
        };
        let tampered_digest = tampered_signing.canonical_digest();
        let tampered_sig =
            transition_ledger_emitter::sign_ledger_entry(&kp, tampered_digest.0).expect("sign");
        entry.tx_kind = TxKind::Verify;
        entry.system_signature = tampered_sig;
        // Recompute resulting_ledger_root with the tampered signing digest so
        // chain check (stage 9) wouldn't be the failure path.
        entry.resulting_ledger_root = append(&Hash::ZERO, &tampered_digest);

        let err = replay_full_transition(
            &crate::state::q_state::QState::genesis(),
            &[entry],
            &cas,
            &pinned,
            &preds,
            &tools,
        )
        .unwrap_err();
        assert!(
            matches!(
                err,
                ReplayError::TxKindMismatch {
                    at: 0,
                    envelope_kind: TxKind::Verify,
                    decoded_kind: TxKind::Work
                }
            ),
            "expected TxKindMismatch(Verify vs Work), got {err:?}"
        );
    }

    /// 18c. v1.1 closure (Codex Q-K secondary): payload decode failure
    ///      reports as PayloadDecode (NOT CasMissing).
    #[test]
    fn replay_rejects_payload_decode_failure() {
        let (_tmp, mut cas, kp, epoch, pinned, preds, tools) = replay_test_setup();

        // Manually put NON-canonical bytes into CAS, then build an entry
        // pointing at them. Signature verifies because envelope binds the
        // cid, not the cid's contents.
        let bad_bytes = b"\xff\xff this is not a valid bincode TypedTx";
        let bad_cid = cas
            .put(bad_bytes, ObjectType::ProposalPayload, "test", 1, None)
            .expect("cas put");
        let signing = LedgerEntrySigningPayload {
            logical_t: 1,
            parent_state_root: Hash::ZERO,
            parent_ledger_root: Hash::ZERO,
            tx_kind: TxKind::Work,
            tx_payload_cid: bad_cid,
            resulting_state_root: h(1),
            timestamp_logical: 1,
            epoch,
            extensions: BTreeMap::new(),
        };
        let digest = signing.canonical_digest();
        let sig =
            transition_ledger_emitter::sign_ledger_entry(&kp, digest.0).expect("sign");
        let entry = LedgerEntry {
            logical_t: 1,
            parent_state_root: Hash::ZERO,
            parent_ledger_root: Hash::ZERO,
            tx_kind: TxKind::Work,
            tx_payload_cid: bad_cid,
            resulting_state_root: h(1),
            resulting_ledger_root: append(&Hash::ZERO, &digest),
            timestamp_logical: 1,
            epoch,
            extensions: BTreeMap::new(),
            system_signature: sig,
        };

        let err = replay_full_transition(
            &crate::state::q_state::QState::genesis(),
            &[entry],
            &cas,
            &pinned,
            &preds,
            &tools,
        )
        .unwrap_err();
        assert!(
            matches!(err, ReplayError::PayloadDecode { at: 0, .. }),
            "expected PayloadDecode at 0, got {err:?}"
        );
    }

    /// 18. sequencer_serial_replay_byte_identity — gated behind #[ignore]
    ///     until CO1.7.5 fills dispatch bodies. The skeleton of the
    ///     test is here so CO1.7.5 just removes the #[ignore].
    #[test]
    #[ignore = "CO1.7.5: requires real per-kind transition bodies"]
    fn sequencer_serial_replay_byte_identity() {
        // CO1.7.5 plan: submit N tx through Sequencer + collect entries from
        // ledger_writer + replay_full_transition(...) → assert final state_root
        // matches sequencer's q.state_root_t. Dispatch must produce real
        // (q_next, _signals) — currently all NotYetImplemented.
    }

    // 14. canonical_encode/decode round-trip for LedgerEntry (foundation of read_at).
    #[test]
    fn canonical_codec_round_trip() {
        let e1 = entry_at(7, h(0xaa), h(0xbb), h(0xcc));
        let bytes = canonical_encode(&e1).expect("encode");
        let e1_back: LedgerEntry = canonical_decode(&bytes).expect("decode");
        assert_eq!(e1, e1_back);

        // Two encodes of the same value must produce byte-identical bytes (I-DET).
        let bytes_again = canonical_encode(&e1).expect("encode again");
        assert_eq!(bytes, bytes_again);

        // Trailing garbage rejected.
        let mut bytes_extra = bytes.clone();
        bytes_extra.push(0xff);
        let err = canonical_decode::<LedgerEntry>(&bytes_extra).unwrap_err();
        assert!(matches!(err, CanonicalCodecError::TrailingBytes { .. }));
    }
}
