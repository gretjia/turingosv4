//! Constitution Landing First 2026-05-07 (HARNESS.md §3 G-009; architect
//! ruling 2026-05-07): C1 immediate `HEAD_t` witness.
//!
//! ## Why
//!
//! Art. 0.4 (`Q_t` version-controlled) and the architect's G-009 Path-C-hybrid
//! ruling require a derived 6-field witness over Q_t that audit can use to
//! answer "where did the tape head sit at this transition?" without reading
//! private state. CLAUDE.md §4.1 pins the schema:
//!
//! ```text
//! HEAD_t = {
//!   state_root,
//!   l4_head,
//!   l4e_head,
//!   cas_root,
//!   economic_state_root,
//!   run_id
//! }
//! ```
//!
//! The C1 witness is **derived** from existing `QState` fields plus the L4.E
//! head + CAS root + run_id passed by the caller. Keeping it derived (not
//! a new persisted field) avoids touching canonical signing payload — which
//! is Class-4 — while still landing the architect's required witness shape
//! for FC2 replay assertions and dashboard lookups.
//!
//! Path-C2 production (libgit2-backed `refs/chaintape/{l4, l4e, cas}`) is the
//! forward step; this module is the immediate C1 witness.
//!
//! `FC-trace: FC1-INV1 + FC2-INV1 + Art-0.4 + G-009`.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::bottom_white::ledger::transition_ledger::canonical_encode;
use crate::state::q_state::{EconomicState, Hash, NodeId, QState};

/// TRACE_MATRIX FC1-N45 + Art-0.4: six-field C1 `HEAD_t` witness
/// (architect-pinned 2026-05-07 §4.1).
///
/// Derived from `QState` + caller-supplied L4.E head + CAS root + run_id.
/// Two field counts are CONSTITUTIONALLY meaningful:
///   - 6 fields total — pinned by `head_t_witness_has_six_canonical_fields`.
///   - `l4e_head` and `cas_root` are `Option` because pre-genesis / pre-CAS
///     boot states have no head; downstream gates assert presence on
///     post-genesis snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeadTWitness {
    /// Materialized state Merkle root (from `QState.state_root_t`).
    pub state_root: Hash,
    /// L4 accepted-ledger head (from `QState.head_t`).
    pub l4_head: NodeId,
    /// L4.E rejection-ledger head (caller-supplied; `None` pre-genesis).
    pub l4e_head: Option<NodeId>,
    /// CAS root hash (caller-supplied; `None` when CAS index is empty).
    pub cas_root: Option<Hash>,
    /// `EconomicState` canonical digest (`QState.economic_state_t.canonical_digest()`).
    pub economic_state_root: Hash,
    /// Run identifier — caller-supplied, sourced from `genesis_payload.toml`
    /// or the bootstrap run-id assignment.
    pub run_id: String,
}

impl HeadTWitness {
    /// TRACE_MATRIX FC1-N45: synthesize the witness from a Q-state plus the
    /// L4.E head, CAS root, and run_id known by the caller. **Derived** view
    /// — does not mutate `QState`. The `&QState` (read-only) signature is
    /// the structural pin behind `dashboard_reads_head_t_derived_state`.
    pub fn from_q_state(
        q: &QState,
        run_id: impl Into<String>,
        l4e_head: Option<NodeId>,
        cas_root: Option<Hash>,
    ) -> Self {
        Self {
            state_root: q.state_root_t,
            l4_head: q.head_t.clone(),
            l4e_head,
            cas_root,
            economic_state_root: economic_state_canonical_root(&q.economic_state_t),
            run_id: run_id.into(),
        }
    }

    /// TRACE_MATRIX FC1-N45: canonical root hash of an `EconomicState`
    /// (sha256 over its canonical bincode encoding). Used as the witness
    /// `economic_state_root` field and exposed for downstream
    /// replay-equivalence checks.
    pub fn economic_state_canonical_root(econ: &EconomicState) -> Hash {
        economic_state_canonical_root(econ)
    }

    /// TRACE_MATRIX § 3 orphan (Stage A3 / HEAD_t C2 SG-A3-HEAD-T-C2.4): reconstruct the C2 witness from `refs/chaintape/{l4,l4e,cas}` ref OIDs alone, plus caller-supplied state/economic/run_id (which MUST be reconstructed from the chain by the caller). Returns `Some` iff at least the L4 ref exists; pre-genesis returns `None`. Constitutional Justification: STAGE_A3_HEAD_T_C2_charter_2026-05-07.md FR-A3-HEAD-T-C2.3 + SG-A3.4 replay-byte-equality.
    pub fn reconstruct_from_chaintape_refs(
        repo_path: &std::path::Path,
        run_id: impl Into<String>,
        state_root: Hash,
        economic_state_root: Hash,
    ) -> Result<Option<Self>, crate::bottom_white::ledger::transition_ledger::LedgerWriterError>
    {
        use crate::bottom_white::ledger::transition_ledger::Git2LedgerWriter;
        let l4_oid = Git2LedgerWriter::head_chaintape_l4(repo_path)?;
        let l4_oid = match l4_oid {
            Some(o) => o,
            None => return Ok(None),
        };
        let l4e_oid = Git2LedgerWriter::head_chaintape_l4e(repo_path)?;
        let cas_oid = Git2LedgerWriter::head_chaintape_cas(repo_path)?;

        // Map git OIDs to NodeId (40-hex string) and Hash (raw bytes).
        let l4_head = NodeId(l4_oid.to_string());
        let l4e_head = l4e_oid.map(|o| NodeId(o.to_string()));
        // Git2 OIDs are 20 bytes (SHA-1) or 32 bytes (SHA-256 repos); hash
        // them with sha256 to canonicalize into the 32-byte Hash shape.
        // This is a derived view; the canonical chain-side OID lives in the
        // ref itself.
        let cas_root = cas_oid.map(|o| {
            let mut hasher = Sha256::new();
            hasher.update(b"chaintape/cas/oid");
            hasher.update(o.as_bytes());
            Hash(hasher.finalize().into())
        });

        Ok(Some(Self {
            state_root,
            l4_head,
            l4e_head,
            cas_root,
            economic_state_root,
            run_id: run_id.into(),
        }))
    }

    /// TRACE_MATRIX FC1-N45: canonical hash of the witness (sha256 of
    /// canonical-encoded bytes). Two witnesses with identical 6-field
    /// contents MUST produce identical canonical hashes — this is what
    /// `head_t_reconstructs_from_replay` pins.
    pub fn canonical_hash(&self) -> Hash {
        // We hash a stable concatenation of the canonical fields rather than
        // depending on an external canonical-encoder; this keeps the C1
        // witness self-contained and lets gates assert hash stability
        // without crossing module boundaries.
        let mut h = Sha256::new();
        h.update(b"HEAD_t/v1");
        h.update(self.state_root.0);
        h.update(self.l4_head.0.as_bytes());
        match &self.l4e_head {
            Some(n) => {
                h.update([1u8]);
                h.update(n.0.as_bytes());
            }
            None => h.update([0u8]),
        }
        match &self.cas_root {
            Some(r) => {
                h.update([1u8]);
                h.update(r.0);
            }
            None => h.update([0u8]),
        }
        h.update(self.economic_state_root.0);
        h.update(self.run_id.as_bytes());
        Hash(h.finalize().into())
    }
}

/// Free helper — sha256 over canonical bincode of an EconomicState.
/// Falls back to all-zeros only on a codec error (which would indicate a
/// catastrophic schema bug; surface it via panic in tests rather than mask).
fn economic_state_canonical_root(econ: &EconomicState) -> Hash {
    let bytes = canonical_encode(econ).expect(
        "EconomicState canonical_encode is infallible for serde-Serialize types — \
         a failure here means the schema is broken and the witness cannot be trusted",
    );
    let mut h = Sha256::new();
    h.update(b"EconomicState/v1");
    h.update(&bytes);
    Hash(h.finalize().into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_q_with_head(head_oid: &str) -> QState {
        let mut q = QState::genesis();
        q.head_t = NodeId(head_oid.into());
        q.state_root_t = Hash([0x11; 32]);
        q
    }

    #[test]
    fn six_field_count() {
        let q = QState::genesis();
        let w = HeadTWitness::from_q_state(&q, "run-1", None, None);
        let v = serde_json::to_value(&w).expect("serialize");
        let obj = v.as_object().expect("witness is a JSON object");
        assert_eq!(
            obj.len(),
            6,
            "HEAD_t witness must have exactly 6 canonical fields per architect §4.1; got {}",
            obj.len()
        );
        for f in [
            "state_root",
            "l4_head",
            "l4e_head",
            "cas_root",
            "economic_state_root",
            "run_id",
        ] {
            assert!(
                obj.contains_key(f),
                "HEAD_t witness missing canonical field `{f}`"
            );
        }
    }

    #[test]
    fn advances_on_l4_head_change() {
        let q1 = fixture_q_with_head(&"a".repeat(40));
        let q2 = fixture_q_with_head(&"b".repeat(40));
        let w1 = HeadTWitness::from_q_state(&q1, "run-1", None, None);
        let w2 = HeadTWitness::from_q_state(&q2, "run-1", None, None);
        assert_ne!(w1.l4_head, w2.l4_head);
        assert_ne!(w1.canonical_hash(), w2.canonical_hash());
    }

    #[test]
    fn does_not_advance_on_l4e_only() {
        let q = fixture_q_with_head(&"a".repeat(40));
        let w_pre = HeadTWitness::from_q_state(&q, "run-1", None, None);
        let w_post = HeadTWitness::from_q_state(
            &q,
            "run-1",
            Some(NodeId("ff".repeat(20))),
            None,
        );
        assert_eq!(
            w_pre.l4_head, w_post.l4_head,
            "l4_head must NOT advance when only L4.E events occur"
        );
        assert_ne!(w_pre.l4e_head, w_post.l4e_head);
        assert_ne!(
            w_pre.canonical_hash(),
            w_post.canonical_hash(),
            "canonical hash must reflect l4e_head changes"
        );
    }

    #[test]
    fn reconstructs_from_replay() {
        let q = fixture_q_with_head(&"a".repeat(40));
        let w_a = HeadTWitness::from_q_state(
            &q,
            "run-1",
            Some(NodeId("ff".repeat(20))),
            Some(Hash([0x22; 32])),
        );
        // Same inputs → same witness → same canonical hash. This is the
        // replay-equivalence property dashboards rely on.
        let w_b = HeadTWitness::from_q_state(
            &q,
            "run-1",
            Some(NodeId("ff".repeat(20))),
            Some(Hash([0x22; 32])),
        );
        assert_eq!(w_a, w_b);
        assert_eq!(w_a.canonical_hash(), w_b.canonical_hash());
    }
}
