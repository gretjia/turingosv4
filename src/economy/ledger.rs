//! L4 accepted-only ledger wrapper — TB-1 Day-3 P1.
//!
//! Charter authority:
//! - `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` Day-3.
//! - ROADMAP P1 Exit 5 (state_root advances on accept), Exit 6 (state_root
//!   unchanged on reject), Exit 7 (ledger hash chain), Exit 8 (state.db
//!   reconstructable from chaintape).
//! - `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`:
//!   accepted transitions ONLY land here; rejected submissions go to L4.E
//!   (`bottom_white::ledger::rejection_evidence`).
//!
//! Constitutional authority:
//! - WP § 5.L4 — ChainTape Layer 4 spine; one entry per accepted transition.
//! - Art IV (Boot) — every Q_t field MUST be reconstructible by replaying L4.
//! - Inv 7 (no rejection on the accepted spine) — rejections never advance
//!   `state_root_t` / `ledger_root_t`.
//!
//! Scope (RSP-0 minimum-viable wrapper):
//! - Self-contained accepted-only hash chain over `TypedTx` canonical bytes.
//! - `append_accepted` advances `logical_t` and chains `prev_hash`.
//! - `verify_chain(start, end)` walks the hash chain over `[start, end)`.
//! - `reconstruct_state` replays L4 only and returns the canonical
//!   `state_root_t` (L4.E is intentionally NOT consulted).
//! - Persistence helpers (`persist` / `load_from_path`) provide the
//!   "drop state.db; reconstruct from L4" round-trip used by P1 kill
//!   acceptance tests.
//!
//! Out of scope (deferred to CO1.7.5+):
//! - `SystemSignature` attachment (full signing payload + epoch binding).
//! - `dispatch_transition` re-run (state_root mutation requires CO1.8).
//! - Real `Git2LedgerWriter` commit chain — that's the production backend
//!   over `refs/transitions/main`; this RSP-0 wrapper uses an in-memory Vec.
//!
//! /// TRACE_MATRIX WP § 5.L4 + Art IV + ROADMAP P1:5/P1:6/P1:7/P1:8: L4 accepted-only ledger.

use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::bottom_white::ledger::transition_ledger::{canonical_encode, TxKind};
use crate::state::q_state::Hash;
use crate::state::typed_tx::TypedTx;

// ────────────────────────────────────────────────────────────────────────────
// AcceptedEntry — one row on the L4 accepted-only chain
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:5 — one accepted-only L4 row.
///
/// All seven fields enter the hash; tampering any single field breaks
/// `verify_chain` at the affected index. The `tx_payload_hash` is the
/// SHA-256 over the bincode-canonical encoding of the source `TypedTx`,
/// re-using the lower-level `canonical_encode` from `transition_ledger`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcceptedEntry {
    /// 1-based monotonic counter; advances ONLY on accept (not on reject —
    /// rejections take a `submit_id` on L4.E instead, per the L4/L4.E split).
    pub logical_t: u64,
    /// Hash of the immediately-preceding entry; `Hash::ZERO` for the first row.
    pub prev_hash: Hash,
    /// Discriminator over the source `TypedTx` variant.
    pub tx_kind: TxKind,
    /// SHA-256 of `canonical_encode(tx)` — content-address of the payload.
    pub tx_payload_hash: Hash,
    /// State-root before this entry was applied.
    pub parent_state_root: Hash,
    /// State-root after this entry was applied. Computed by `next_state_root`
    /// (the RSP-0 toy mutator); a real `dispatch_transition` lands in CO1.7.5.
    pub resulting_state_root: Hash,
    /// SHA-256 over the six fields above plus a domain-separation prefix.
    pub hash: Hash,
}

impl AcceptedEntry {
    fn compute_hash(
        logical_t: u64,
        prev_hash: &Hash,
        tx_kind: TxKind,
        tx_payload_hash: &Hash,
        parent_state_root: &Hash,
        resulting_state_root: &Hash,
    ) -> Hash {
        let mut h = Sha256::new();
        h.update(b"turingosv4.l4_accepted.v1");
        h.update(logical_t.to_be_bytes());
        h.update(prev_hash.0);
        h.update((tx_kind as u8).to_be_bytes());
        h.update(tx_payload_hash.0);
        h.update(parent_state_root.0);
        h.update(resulting_state_root.0);
        Hash(h.finalize().into())
    }
}

// ────────────────────────────────────────────────────────────────────────────
// LedgerError — shared error taxonomy for append / verify / reconstruct
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:5/P1:6/P1:7/P1:8 — error taxonomy for the L4 wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    /// `verify_chain` walked off the end of `entries`.
    OutOfBounds { len: usize, requested_end: usize },
    /// Hash mismatch at the given chain index (prev_hash break OR entry hash break).
    HashMismatch { at_index: usize },
    /// `logical_t` is not the expected `index + 1` value.
    LogicalTGap { at_index: usize, expected: u64, got: u64 },
    /// `parent_state_root` doesn't match the running replay state.
    ParentStateMismatch { at_index: usize },
    /// `canonical_encode` of the source `TypedTx` failed.
    Encode(String),
    /// File system or JSON serialization error during persist / load.
    Io(String),
}

impl std::fmt::Display for LedgerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfBounds { len, requested_end } => {
                write!(f, "verify_chain end={} exceeds chain len={}", requested_end, len)
            }
            Self::HashMismatch { at_index } => {
                write!(f, "L4 hash chain break at index {}", at_index)
            }
            Self::LogicalTGap { at_index, expected, got } => write!(
                f,
                "logical_t gap at index {}: expected {}, got {}",
                at_index, expected, got
            ),
            Self::ParentStateMismatch { at_index } => {
                write!(f, "parent_state_root mismatch at index {}", at_index)
            }
            Self::Encode(e) => write!(f, "canonical_encode failed: {}", e),
            Self::Io(e) => write!(f, "persistence I/O failed: {}", e),
        }
    }
}

impl std::error::Error for LedgerError {}

/// TRACE_MATRIX P1:7 — `verify_chain` failure alias; kept distinct from
/// `ReconstructError` so callers can pattern-match on chain-walk vs replay.
pub type ChainError = LedgerError;
/// TRACE_MATRIX P1:8 — `reconstruct_state` / `load_from_path` failure alias;
/// distinct from `ChainError` so replay errors are syntactically separable.
pub type ReconstructError = LedgerError;

// ────────────────────────────────────────────────────────────────────────────
// AcceptedLedger — the wrapper itself
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:5/P1:6/P1:7/P1:8 — accepted-only L4 hash chain (RSP-0).
///
/// Single source of truth for the accepted spine. Rejected transitions
/// MUST NOT touch this struct; they take a `submit_id` on L4.E
/// (`bottom_white::ledger::rejection_evidence`).
#[derive(Debug, Clone, Default)]
pub struct AcceptedLedger {
    entries: Vec<AcceptedEntry>,
    current_state_root: Hash,
}

impl AcceptedLedger {
    /// TRACE_MATRIX Art IV Boot — empty L4 (genesis state_root is `Hash::ZERO`).
    pub fn new() -> Self {
        Self::default()
    }

    /// TRACE_MATRIX P1:5 — count of accepted rows.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// TRACE_MATRIX P1:5 — empty predicate.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// TRACE_MATRIX P1:5 — current canonical `state_root_t`.
    pub fn current_state_root(&self) -> Hash {
        self.current_state_root
    }

    /// TRACE_MATRIX P1:5 — append-accepted entry; advances `logical_t` by 1.
    ///
    /// Advances `current_state_root` via the toy mutator `next_state_root`.
    /// Returns the freshly-built `AcceptedEntry` (clone of what was pushed).
    pub fn append_accepted(&mut self, tx: &TypedTx) -> Result<AcceptedEntry, LedgerError> {
        let bytes = canonical_encode(tx).map_err(|e| LedgerError::Encode(e.to_string()))?;
        let tx_payload_hash = sha256_of(&bytes);
        let prev_hash = self.entries.last().map(|e| e.hash).unwrap_or(Hash::ZERO);
        let logical_t = (self.entries.len() as u64) + 1;
        let parent_state_root = self.current_state_root;
        let tx_kind = tx.tx_kind();
        let resulting_state_root = next_state_root(&parent_state_root, &tx_payload_hash);
        let hash = AcceptedEntry::compute_hash(
            logical_t,
            &prev_hash,
            tx_kind,
            &tx_payload_hash,
            &parent_state_root,
            &resulting_state_root,
        );
        let entry = AcceptedEntry {
            logical_t,
            prev_hash,
            tx_kind,
            tx_payload_hash,
            parent_state_root,
            resulting_state_root,
            hash,
        };
        self.entries.push(entry.clone());
        self.current_state_root = resulting_state_root;
        Ok(entry)
    }

    /// TRACE_MATRIX P1:7 — verify hash-chain integrity over `[start, end)`.
    ///
    /// Returns `Err(HashMismatch)` if any single field (logical_t, prev_hash,
    /// tx_payload_hash, parent_state_root, resulting_state_root, tx_kind, or
    /// the entry hash itself) was tampered.
    pub fn verify_chain(&self, start: usize, end: usize) -> Result<(), ChainError> {
        if end > self.entries.len() {
            return Err(LedgerError::OutOfBounds {
                len: self.entries.len(),
                requested_end: end,
            });
        }
        if start > end {
            return Err(LedgerError::OutOfBounds {
                len: self.entries.len(),
                requested_end: start,
            });
        }
        let mut prev = if start == 0 {
            Hash::ZERO
        } else {
            self.entries[start - 1].hash
        };
        for i in start..end {
            let e = &self.entries[i];
            let expected_logical_t = (i as u64) + 1;
            if e.logical_t != expected_logical_t {
                return Err(LedgerError::LogicalTGap {
                    at_index: i,
                    expected: expected_logical_t,
                    got: e.logical_t,
                });
            }
            if e.prev_hash != prev {
                return Err(LedgerError::HashMismatch { at_index: i });
            }
            let recomputed = AcceptedEntry::compute_hash(
                e.logical_t,
                &e.prev_hash,
                e.tx_kind,
                &e.tx_payload_hash,
                &e.parent_state_root,
                &e.resulting_state_root,
            );
            if recomputed != e.hash {
                return Err(LedgerError::HashMismatch { at_index: i });
            }
            prev = e.hash;
        }
        Ok(())
    }

    /// TRACE_MATRIX P1:8 — replay L4 only; recompute the canonical `state_root_t`.
    ///
    /// L4.E is intentionally NOT consulted: rejected submissions never affect
    /// `state_root_t` (Inv 7).
    pub fn reconstruct_state(&self) -> Result<Hash, ReconstructError> {
        let mut s = Hash::ZERO;
        for (i, e) in self.entries.iter().enumerate() {
            if e.parent_state_root != s {
                return Err(LedgerError::ParentStateMismatch { at_index: i });
            }
            let expected = next_state_root(&s, &e.tx_payload_hash);
            if e.resulting_state_root != expected {
                return Err(LedgerError::HashMismatch { at_index: i });
            }
            s = e.resulting_state_root;
        }
        Ok(s)
    }

    /// TRACE_MATRIX P1:8 — persist entries to `state_path` for cold restart.
    pub fn persist(&self, state_path: &Path) -> Result<(), LedgerError> {
        let bytes = serde_json::to_vec(&self.entries).map_err(|e| LedgerError::Io(e.to_string()))?;
        std::fs::write(state_path, bytes).map_err(|e| LedgerError::Io(e.to_string()))?;
        Ok(())
    }

    /// TRACE_MATRIX P1:8 — load entries from `state_path` and recompute the
    /// canonical `state_root_t`. Used by the "drop state.db; reconstruct from L4"
    /// kill test: any direct mutation that bypassed the L4 path is washed out.
    pub fn load_from_path(state_path: &Path) -> Result<(Self, Hash), ReconstructError> {
        let bytes = std::fs::read(state_path).map_err(|e| LedgerError::Io(e.to_string()))?;
        let entries: Vec<AcceptedEntry> =
            serde_json::from_slice(&bytes).map_err(|e| LedgerError::Io(e.to_string()))?;
        let mut l = Self {
            entries,
            current_state_root: Hash::ZERO,
        };
        let s = l.reconstruct_state()?;
        l.current_state_root = s;
        Ok((l, s))
    }

    /// TRACE_MATRIX P1:7 — read-only entry slice (for replay / debug / external
    /// tooling that wants to inspect the chain without mutating it).
    pub fn entries(&self) -> &[AcceptedEntry] {
        &self.entries
    }

    /// TRACE_MATRIX P1:7 — TAMPER-ONLY hook used by kill-criteria integration
    /// tests to simulate adversarial row deletion. The `tamper_` prefix and
    /// `#[doc(hidden)]` mark this as not part of the supported API; production
    /// callers MUST NOT use it. Kept `pub` (rather than `cfg(test)`) only so
    /// integration tests in `tests/` can reach it; integration tests link
    /// against the lib without `cfg(test)` enabled.
    #[doc(hidden)]
    pub fn tamper_remove_entry(&mut self, idx: usize) {
        self.entries.remove(idx);
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Helpers
// ────────────────────────────────────────────────────────────────────────────

fn sha256_of(bytes: &[u8]) -> Hash {
    let mut h = Sha256::new();
    h.update(bytes);
    Hash(h.finalize().into())
}

/// RSP-0 toy state mutator: `next = SHA-256(domain || prev_state_root || tx_payload_hash)`.
///
/// This is a minimum-viable demonstration of the state-root-advances-on-accept
/// invariant. The real `dispatch_transition`-driven state_root mutation lands
/// in CO1.7.5 / CO1.8 (proper economic + agent-swarm state evolution).
fn next_state_root(prev: &Hash, tx_payload_hash: &Hash) -> Hash {
    let mut h = Sha256::new();
    h.update(b"turingosv4.l4_state_root.v1");
    h.update(prev.0);
    h.update(tx_payload_hash.0);
    Hash(h.finalize().into())
}

// ────────────────────────────────────────────────────────────────────────────
// Inline correctness tests (round-trip + tamper detection on every field).
// Cross-cutting P1 kill acceptance tests live in `tests/tb_1_p1_acceptance.rs`.
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::q_state::{AgentId, TxId};
    use crate::state::typed_tx::{
        AgentSignature, BoolWithProof, PredicateId, PredicateResultsBundle, ReadKey,
        SafetyOrCreation, TaskId, TypedTx, WorkTx, WriteKey,
    };
    use crate::bottom_white::cas::schema::Cid;
    use crate::economy::money::StakeMicroCoin;
    use std::collections::{BTreeMap, BTreeSet};

    fn fixture_work_tx(suffix: u32) -> TypedTx {
        let mut acceptance = BTreeMap::new();
        acceptance.insert(
            PredicateId(format!("acc-{}", suffix)),
            BoolWithProof { value: true, proof_cid: Some(Cid([0x11; 32])) },
        );
        let mut settlement = BTreeMap::new();
        settlement.insert(
            PredicateId(format!("set-{}", suffix)),
            BoolWithProof { value: true, proof_cid: None },
        );
        let mut read_set = BTreeSet::new();
        read_set.insert(ReadKey(format!("k.r.{}", suffix)));
        let mut write_set = BTreeSet::new();
        write_set.insert(WriteKey(format!("k.w.{}", suffix)));
        TypedTx::Work(WorkTx {
            tx_id: TxId(format!("worktx-{}", suffix)),
            task_id: TaskId(format!("task-{}", suffix)),
            parent_state_root: Hash::ZERO,
            agent_id: AgentId("alice".into()),
            read_set,
            write_set,
            proposal_cid: Cid([0x13; 32]),
            predicate_results: PredicateResultsBundle {
                acceptance,
                settlement,
                safety_class: SafetyOrCreation::Safety,
            },
            stake: StakeMicroCoin::from_micro_units(1_000_000),
            signature: AgentSignature::from_bytes([0x77u8; 64]),
            timestamp_logical: suffix as u64,
        })
    }

    #[test]
    fn append_advances_logical_t_and_state_root() {
        let mut l = AcceptedLedger::new();
        assert_eq!(l.len(), 0);
        assert_eq!(l.current_state_root(), Hash::ZERO);

        let e1 = l.append_accepted(&fixture_work_tx(1)).unwrap();
        assert_eq!(e1.logical_t, 1);
        assert_eq!(e1.prev_hash, Hash::ZERO);
        assert_eq!(e1.parent_state_root, Hash::ZERO);
        assert_ne!(e1.resulting_state_root, Hash::ZERO);
        assert_eq!(l.current_state_root(), e1.resulting_state_root);

        let e2 = l.append_accepted(&fixture_work_tx(2)).unwrap();
        assert_eq!(e2.logical_t, 2);
        assert_eq!(e2.prev_hash, e1.hash);
        assert_eq!(e2.parent_state_root, e1.resulting_state_root);
    }

    #[test]
    fn verify_chain_passes_on_clean_chain() {
        let mut l = AcceptedLedger::new();
        for i in 1..=5 {
            l.append_accepted(&fixture_work_tx(i)).unwrap();
        }
        assert!(l.verify_chain(0, 5).is_ok());
        assert!(l.verify_chain(0, 0).is_ok());
        assert!(l.verify_chain(2, 4).is_ok());
    }

    #[test]
    fn verify_chain_out_of_bounds_rejected() {
        let mut l = AcceptedLedger::new();
        l.append_accepted(&fixture_work_tx(1)).unwrap();
        let r = l.verify_chain(0, 5);
        assert!(matches!(r, Err(LedgerError::OutOfBounds { .. })));
    }

    #[test]
    fn reconstruct_state_round_trip() {
        let mut l = AcceptedLedger::new();
        for i in 1..=4 {
            l.append_accepted(&fixture_work_tx(i)).unwrap();
        }
        let pre = l.current_state_root();
        let reconstructed = l.reconstruct_state().unwrap();
        assert_eq!(pre, reconstructed);
    }

    #[test]
    fn persist_and_load_round_trip() {
        let mut l = AcceptedLedger::new();
        for i in 1..=3 {
            l.append_accepted(&fixture_work_tx(i)).unwrap();
        }
        let pre = l.current_state_root();

        let tmp = tempfile::NamedTempFile::new().unwrap();
        l.persist(tmp.path()).unwrap();
        let (l2, post) = AcceptedLedger::load_from_path(tmp.path()).unwrap();
        assert_eq!(pre, post);
        assert_eq!(l2.len(), 3);
    }
}
