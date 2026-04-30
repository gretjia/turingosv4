//! L4.E rejection-evidence ledger — TB-1 Day-3 P1.
//!
//! Charter authority:
//! - `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` Day-3.
//! - `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`
//!   (architectural commitment to L4 / L4.E split, post external audit
//!   2026-04-29 CF-1).
//! - ROADMAP P1 Exit 6 (rejected tx ≠ state_root advance), Exit 9
//!   (rejected log not visible in another agent's read view).
//!
//! Constitutional authority:
//! - Inv 7 — accepted spine and rejection-evidence are disjoint ledgers;
//!   rejections never mutate `state_root_t` / `ledger_root_t`.
//! - Inv 10 (Goodhart shield) — raw rejection diagnostics are isolated
//!   from agent-facing materialized views; only `public_summary` is
//!   permitted to cross the agent boundary.
//! - Art. III.4 (selective shielding) — rejection raw content is shielded
//!   by default; explicit opt-in via `public_summary`.
//!
//! Scope (RSP-0 minimum-viable):
//! - In-memory `Vec<RejectedSubmissionRecord>` chained via `prev_hash`.
//! - `submit_id` (NOT `logical_t`) keys each record per the L4 / L4.E split:
//!   accepted spine takes the canonical counter; rejection-evidence carries
//!   an independent submit-side counter from `Sequencer::next_submit_id`.
//! - `raw_diagnostic_cid` is a CAS handle to the raw error bytes; the
//!   `PublicRejectionView` projection (used to materialize agent-facing
//!   read views) DOES NOT carry that field — structural shielding rather
//!   than runtime access-control.
//!
//! Out of scope (deferred):
//! - Persistence backend (Git2 commit chain on `refs/rejections/main` —
//!   future RSP / TB).
//! - SystemSignature attestation per record (CO1.7.5+ when system_keypair
//!   gets a `CanonicalMessage::RejectionEvidence` variant).
//! - Cross-agent visibility policy machinery (CO P2.7).
//!
//! /// TRACE_MATRIX Inv 7 + Inv 10 + ROADMAP P1:6/P1:9: L4.E rejection-evidence ledger.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::bottom_white::cas::schema::Cid;
use crate::bottom_white::ledger::transition_ledger::TxKind;
use crate::state::q_state::{AgentId, Hash};

// ────────────────────────────────────────────────────────────────────────────
// RejectionClass — taxonomy of why a submission was rejected
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:6 — coarse rejection-class discriminator.
///
/// Stable byte-encoding via `#[repr(u8)]` so the discriminator can ride into
/// the canonical hash deterministically across compiler versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum RejectionClass {
    /// A `top_white::predicates` acceptance gate returned `false`.
    PredicateFailed = 0,
    /// A higher-level policy gate (visibility / quorum / quota) said no.
    PolicyViolation = 1,
    /// `Inv 3` escrow-lock missing for a write-side mutation.
    EscrowMissing = 2,
    /// `monetary_invariant` (Inv 4 / 基本法 1) flagged a conservation break.
    InvariantViolation = 3,
    /// `canonical_decode` of the submitted bytes failed.
    MalformedPayload = 4,
}

// ────────────────────────────────────────────────────────────────────────────
// RejectedSubmissionRecord — one row on the L4.E chain
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:6/P1:9 — one rejection-evidence row.
///
/// Distinguished from `LedgerEntry` (the L4 accepted spine):
/// - keyed by `submit_id` (not `logical_t`);
/// - records `parent_state_root` for the snapshot-at-submit but never a
///   `resulting_state_root` (rejection MUST NOT advance state);
/// - `raw_diagnostic_cid` holds the raw error content shielded behind a CAS
///   handle (not exposed in agent-facing views);
/// - `public_summary` is the ONLY field permitted to cross the agent boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RejectedSubmissionRecord {
    /// Independent submit-side counter from `Sequencer::next_submit_id`.
    pub submit_id: u64,
    /// State-root snapshot at submit time — recorded for forensics; NEVER
    /// advanced by rejection (Inv 7).
    pub parent_state_root: Hash,
    /// Submitter agent (opaque string).
    pub agent_id: AgentId,
    /// Discriminator over the submitted (now-rejected) `TypedTx` variant.
    pub tx_kind: TxKind,
    /// CAS handle to the canonical-encoded source `TypedTx`.
    pub tx_payload_cid: Cid,
    /// Coarse why-class (one of `RejectionClass`).
    pub rejection_class: RejectionClass,
    /// CAS handle to the raw diagnostic bytes (e.g. predicate counter-example).
    /// `None` when no raw payload is captured. NEVER exposed via `PublicRejectionView`.
    ///
    /// **TB-1 P0-3 type shield** (Codex audit 2026-04-29): `#[serde(skip_serializing,
    /// default)]` ensures that EVEN IF a future caller bypasses
    /// `PublicRejectionView` and serializes a raw `RejectedSubmissionRecord`, the
    /// raw cid is structurally absent from the output. Forensic in-memory access
    /// continues via `RejectionEvidenceWriter::records()`. A capability-gated
    /// audit-only API replaces this skip in a later TB; until then, the persisted
    /// form is INTENTIONALLY incomplete (rehydration recovers `None` and the
    /// chain hash will not re-verify — RSP-0 is in-memory only).
    #[serde(skip_serializing, default)]
    pub raw_diagnostic_cid: Option<Cid>,
    /// Agent-facing summary string. `None` when no public summary is permitted
    /// (raw-diagnostic-only mode). The ONLY field that crosses the agent boundary.
    pub public_summary: Option<String>,
    /// Hash of the immediately-preceding rejection record; `Hash::ZERO` for the first.
    pub prev_hash: Hash,
    /// SHA-256 over the nine fields above plus a domain-separation prefix.
    pub hash: Hash,
}

impl RejectedSubmissionRecord {
    fn compute_hash(
        submit_id: u64,
        parent_state_root: &Hash,
        agent_id: &AgentId,
        tx_kind: TxKind,
        tx_payload_cid: &Cid,
        rejection_class: RejectionClass,
        raw_diagnostic_cid: &Option<Cid>,
        public_summary: &Option<String>,
        prev_hash: &Hash,
    ) -> Hash {
        let mut h = Sha256::new();
        h.update(b"turingosv4.l4e_rejection_evidence.v1");
        h.update(submit_id.to_be_bytes());
        h.update(parent_state_root.0);
        h.update((agent_id.0.len() as u64).to_be_bytes());
        h.update(agent_id.0.as_bytes());
        h.update((tx_kind as u8).to_be_bytes());
        h.update(tx_payload_cid.0);
        h.update((rejection_class as u8).to_be_bytes());
        match raw_diagnostic_cid {
            Some(c) => {
                h.update([1u8]);
                h.update(c.0);
            }
            None => h.update([0u8]),
        }
        match public_summary {
            Some(s) => {
                h.update([1u8]);
                h.update((s.len() as u64).to_be_bytes());
                h.update(s.as_bytes());
            }
            None => h.update([0u8]),
        }
        h.update(prev_hash.0);
        Hash(h.finalize().into())
    }
}

// ────────────────────────────────────────────────────────────────────────────
// PublicRejectionView — agent-facing projection (Inv 10 Goodhart shield)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX Inv 10 + ROADMAP P1:9 — agent-facing projection.
///
/// **Structural** isolation: the type itself does not carry
/// `raw_diagnostic_cid`. Materializing this view from a
/// `RejectedSubmissionRecord` cannot accidentally leak the raw diagnostic
/// because there is no field to write it into.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicRejectionView {
    pub submit_id: u64,
    pub parent_state_root: Hash,
    pub agent_id: AgentId,
    pub tx_kind: TxKind,
    pub rejection_class: RejectionClass,
    pub public_summary: Option<String>,
}

impl From<&RejectedSubmissionRecord> for PublicRejectionView {
    fn from(r: &RejectedSubmissionRecord) -> Self {
        Self {
            submit_id: r.submit_id,
            parent_state_root: r.parent_state_root,
            agent_id: r.agent_id.clone(),
            tx_kind: r.tx_kind,
            rejection_class: r.rejection_class,
            public_summary: r.public_summary.clone(),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// RejectionEvidenceError — chain-walk failure taxonomy
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:6 — error returned by `RejectionEvidenceWriter::verify_chain`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejectionEvidenceError {
    /// `prev_hash` chain or per-record hash diverged at the given index
    /// (covers row deletion, field tampering, and reordering).
    HashMismatch { at: usize },
}

impl std::fmt::Display for RejectionEvidenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HashMismatch { at } => write!(f, "rejection-evidence chain break at index {}", at),
        }
    }
}

impl std::error::Error for RejectionEvidenceError {}

// ────────────────────────────────────────────────────────────────────────────
// RejectionEvidenceWriter — append + verify + project-to-public
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:6/P1:9 — RSP-0 in-memory rejection-evidence writer.
///
/// One `Vec<RejectedSubmissionRecord>`; `prev_hash` chained; `submit_id`
/// monotonicity is the caller's responsibility (the writer trusts the
/// `Sequencer::next_submit_id` issuer). No `logical_t` field — accepted
/// spine and rejection-evidence are intentionally disjoint per the L4 / L4.E
/// split (`DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`).
#[derive(Debug, Clone, Default)]
pub struct RejectionEvidenceWriter {
    records: Vec<RejectedSubmissionRecord>,
}

impl RejectionEvidenceWriter {
    /// TRACE_MATRIX P1:6 — empty writer.
    pub fn new() -> Self {
        Self::default()
    }

    /// TRACE_MATRIX P1:6 — count of recorded rejections.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// TRACE_MATRIX P1:6 — empty predicate.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// TRACE_MATRIX P1:6 — last record's hash, or `Hash::ZERO` for empty chain.
    pub fn last_hash(&self) -> Hash {
        self.records.last().map(|r| r.hash).unwrap_or(Hash::ZERO)
    }

    /// TRACE_MATRIX P1:6/P1:9 — append a rejection record; returns the new chain hash.
    ///
    /// CRITICAL: this method MUST NOT be called from the L4 (accepted) write
    /// path — Inv 7 forbids state-root advance on rejection. The caller's
    /// dispatch logic decides which ledger receives the record.
    #[allow(clippy::too_many_arguments)]
    pub fn append_rejected(
        &mut self,
        submit_id: u64,
        parent_state_root: Hash,
        agent_id: AgentId,
        tx_kind: TxKind,
        tx_payload_cid: Cid,
        rejection_class: RejectionClass,
        raw_diagnostic_cid: Option<Cid>,
        public_summary: Option<String>,
    ) -> Hash {
        let prev_hash = self.last_hash();
        let hash = RejectedSubmissionRecord::compute_hash(
            submit_id,
            &parent_state_root,
            &agent_id,
            tx_kind,
            &tx_payload_cid,
            rejection_class,
            &raw_diagnostic_cid,
            &public_summary,
            &prev_hash,
        );
        let record = RejectedSubmissionRecord {
            submit_id,
            parent_state_root,
            agent_id,
            tx_kind,
            tx_payload_cid,
            rejection_class,
            raw_diagnostic_cid,
            public_summary,
            prev_hash,
            hash,
        };
        self.records.push(record);
        hash
    }

    /// TRACE_MATRIX P1:6 — verify the rejection-evidence chain end-to-end.
    ///
    /// Returns `Err(HashMismatch)` if any single field of any record was
    /// tampered, or if a row was deleted (the surviving row's `prev_hash`
    /// no longer matches its predecessor's `hash`).
    pub fn verify_chain(&self) -> Result<(), RejectionEvidenceError> {
        let mut prev = Hash::ZERO;
        for (i, r) in self.records.iter().enumerate() {
            if r.prev_hash != prev {
                return Err(RejectionEvidenceError::HashMismatch { at: i });
            }
            let recomputed = RejectedSubmissionRecord::compute_hash(
                r.submit_id,
                &r.parent_state_root,
                &r.agent_id,
                r.tx_kind,
                &r.tx_payload_cid,
                r.rejection_class,
                &r.raw_diagnostic_cid,
                &r.public_summary,
                &r.prev_hash,
            );
            if recomputed != r.hash {
                return Err(RejectionEvidenceError::HashMismatch { at: i });
            }
            prev = r.hash;
        }
        Ok(())
    }

    /// TRACE_MATRIX P1:9 — read-only record slice (for L4.E forensics; full
    /// records carry `raw_diagnostic_cid` and MUST NOT be exposed across the
    /// agent boundary; use `public_view` for that).
    pub fn records(&self) -> &[RejectedSubmissionRecord] {
        &self.records
    }

    /// TRACE_MATRIX Inv 10 + P1:9 — agent-facing projection.
    ///
    /// `PublicRejectionView` does not carry `raw_diagnostic_cid` by type
    /// construction; this method's output is safe to materialize into another
    /// agent's read view.
    pub fn public_view(&self) -> Vec<PublicRejectionView> {
        self.records.iter().map(PublicRejectionView::from).collect()
    }

    /// TRACE_MATRIX P1:6 — TAMPER-ONLY hook used by kill-criteria integration
    /// tests (`test_p1_kill_4b_rejection_chain_breaks_on_row_deletion`).
    /// `#[doc(hidden)]` + `tamper_` prefix flags any production use as a
    /// reviewable violation; kept `pub` only so integration tests in `tests/`
    /// can reach it (they link against the lib without `cfg(test)` enabled).
    #[doc(hidden)]
    pub fn tamper_remove_record(&mut self, idx: usize) {
        self.records.remove(idx);
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Inline correctness tests; cross-cutting P1 kill acceptance tests live in
// `tests/tb_1_p1_acceptance.rs`.
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn cid(byte: u8) -> Cid {
        Cid([byte; 32])
    }
    fn agent(s: &str) -> AgentId {
        AgentId(s.to_string())
    }

    #[test]
    fn append_records_and_chains() {
        let mut w = RejectionEvidenceWriter::new();
        let h1 = w.append_rejected(
            1,
            Hash::ZERO,
            agent("alice"),
            TxKind::Work,
            cid(0x10),
            RejectionClass::PredicateFailed,
            Some(cid(0xAA)),
            Some("predicate acc1 returned false".into()),
        );
        let h2 = w.append_rejected(
            2,
            Hash::ZERO,
            agent("bob"),
            TxKind::Verify,
            cid(0x11),
            RejectionClass::PolicyViolation,
            None,
            None,
        );
        assert_eq!(w.len(), 2);
        assert_ne!(h1, Hash::ZERO);
        assert_ne!(h2, Hash::ZERO);
        assert_eq!(w.records()[1].prev_hash, h1);
        assert_eq!(w.last_hash(), h2);
        assert!(w.verify_chain().is_ok());
    }

    #[test]
    fn public_view_omits_raw_diagnostic_cid() {
        let mut w = RejectionEvidenceWriter::new();
        w.append_rejected(
            1,
            Hash::ZERO,
            agent("alice"),
            TxKind::Work,
            cid(0x10),
            RejectionClass::PredicateFailed,
            Some(cid(0xAA)), // raw diagnostic bytes
            Some("acc1 false".into()),
        );
        let view = w.public_view();
        assert_eq!(view.len(), 1);
        // Structural isolation: `PublicRejectionView` doesn't have a
        // `raw_diagnostic_cid` field. Round-trip via JSON to assert the
        // serialized form also omits it.
        let json = serde_json::to_value(&view[0]).unwrap();
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("raw_diagnostic_cid"));
        assert_eq!(obj.get("public_summary").unwrap(), "acc1 false");
    }

    #[test]
    fn raw_diagnostic_cid_skipped_in_record_serialization() {
        // TB-1 P0-3 type shield (Codex audit 2026-04-29): even if a caller
        // bypasses PublicRejectionView and serializes a raw
        // RejectedSubmissionRecord, raw_diagnostic_cid must NOT appear in the
        // serialized form. Forensic in-memory access still works.
        let mut w = RejectionEvidenceWriter::new();
        w.append_rejected(
            1,
            Hash::ZERO,
            agent("alice"),
            TxKind::Work,
            cid(0x10),
            RejectionClass::PredicateFailed,
            Some(cid(0xAA)), // raw diagnostic present in-memory
            Some("acc1 false".into()),
        );
        let record = &w.records()[0];

        // Forensic access: in-memory field is populated.
        assert!(
            record.raw_diagnostic_cid.is_some(),
            "in-memory forensic access must still see the raw cid"
        );

        // Serialization: field MUST be structurally absent.
        let json = serde_json::to_value(record).unwrap();
        let obj = json.as_object().expect("record serializes as object");
        assert!(
            !obj.contains_key("raw_diagnostic_cid"),
            "raw_diagnostic_cid must not serialize on RejectedSubmissionRecord"
        );

        // The other shielded-but-public fields stay present.
        assert!(obj.contains_key("submit_id"));
        assert!(obj.contains_key("public_summary"));
    }

    #[test]
    fn verify_detects_field_tamper() {
        let mut w = RejectionEvidenceWriter::new();
        w.append_rejected(
            1,
            Hash::ZERO,
            agent("alice"),
            TxKind::Work,
            cid(0x10),
            RejectionClass::PredicateFailed,
            None,
            Some("ok".into()),
        );
        w.append_rejected(
            2,
            Hash::ZERO,
            agent("bob"),
            TxKind::Verify,
            cid(0x11),
            RejectionClass::PolicyViolation,
            None,
            None,
        );
        // Tamper public_summary on record 0; per-record hash should now
        // disagree with its computed value.
        w.records[0].public_summary = Some("tampered".into());
        let r = w.verify_chain();
        assert!(matches!(r, Err(RejectionEvidenceError::HashMismatch { at: 0 })));
    }
}
