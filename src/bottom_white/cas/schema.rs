//! CAS object schema per WP architecture § 5.L3.
//!
//! /// TRACE_MATRIX WP-arch-§5.L3: CAS object schema

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Content-addressed identifier — sha256 of payload bytes.
///
/// Distinct from git's SHA-1 OID (which is an internal storage detail of
/// the git2-rs backend). `Cid` is the v4-canonical identifier; spec § 1.2
/// `WorkTx.proposal_cid: Cid` references this.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct Cid(pub [u8; 32]);

impl Cid {
    /// Compute Cid from content bytes.
    pub fn from_content(content: &[u8]) -> Self {
        let mut h = Sha256::new();
        h.update(content);
        Self(h.finalize().into())
    }

    /// Hex-encoded representation (lowercase; 64 chars).
    pub fn hex(&self) -> String {
        let mut s = String::with_capacity(64);
        for b in self.0 {
            s.push_str(&format!("{:02x}", b));
        }
        s
    }
}

impl std::fmt::Display for Cid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cid:{}", self.hex())
    }
}

/// Type tag for CAS objects (replaces inline string-typed kind).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectType {
    /// Agent's work_tx proposal payload (Lean proof, code patch, etc.).
    ProposalPayload,
    /// Challenger's counterexample for a slashed claim.
    CounterexamplePayload,
    /// Predicate bytecode (Lean tactic, WASM module, Rust source bytes).
    PredicateBytecode,
    /// Tool bytecode.
    ToolBytecode,
    /// Constitution diff (for amendment proposals).
    AmendmentDiff,
    /// Reversibility plan attached to a meta_tx.
    ReversibilityPlan,
    /// TB-11 (architect §6.1): canonical-encoded `EvidenceCapsule` bytes.
    /// Referenced by `TerminalSummaryTx.evidence_capsule_cid` /
    /// `TaskBankruptcyTx.evidence_capsule_cid` for O(N) audit access while
    /// keeping L4 chain cost O(1).
    EvidenceCapsule,
    /// TB-11 (architect §6.1): JSON manifest enumerating sub-CAS objects
    /// of an EvidenceCapsule (e.g. compressed log Cid + size + sha256).
    EvidenceManifest,
    /// TB-11 (architect §6.1): gzipped raw run log bytes (audit-only access).
    /// Privacy default `CapsulePrivacyPolicy::AuditOnly` — never enters
    /// Agent read view.
    CompressedRunLog,
    /// TB-15 (architect §6.2): canonical-encoded `AgentAutopsyCapsule`
    /// bytes. Per-agent, per-event loss capsule derived from ChainTape
    /// evidence (NEVER from agent self-narration). Anchored from
    /// `EconomicState.agent_autopsies_t[event_id]: Vec<Cid>`.
    /// Privacy default `CapsulePrivacyPolicy::AuditOnly`; only
    /// `public_summary` text may broadcast on N≥3 typical-error cluster.
    AgentAutopsyCapsule,
    /// TB-15 (architect §6.2): private-detail JSON for an
    /// `AgentAutopsyCapsule`. Referenced by
    /// `AgentAutopsyCapsule.private_detail_cid`. Audit-only by default;
    /// MUST NOT enter `AgentVisibleProjection`.
    AutopsyPrivateDetail,
    /// TB-15 (architect §6.2): canonical-encoded `MarkovEvidenceCapsule`
    /// bytes. End-of-TB rollup binding constitution_hash + L4 root +
    /// L4.E root + CAS root + previous_capsule_cid + typical_errors +
    /// unresolved_obs + next_session_context_cid. Default next-session
    /// bootstrap source; deeper history requires `TURINGOS_MARKOV_OVERRIDE=1`.
    MarkovEvidenceCapsule,
    /// TB-15 (architect §6.2 FR-15.4): JSON blob describing the next
    /// session's default boot context (`{constitution_hash,
    /// latest_markov_cid, boot_seq[]}`). Referenced by
    /// `MarkovEvidenceCapsule.next_session_context_cid`.
    NextSessionContext,
    /// TB-18R R1 (charter v2 §1 + Codex Gate 1 ratified 2026-05-06):
    /// canonical-encoded `AttemptTelemetry` bytes for one externalized
    /// LLM-Lean cycle. Privacy invariant: `candidate_payload_cid` points
    /// at parsed external candidate bytes, NEVER raw LLM response (per
    /// CR-18R.4 v2). See `src/runtime/attempt_telemetry.rs`.
    AttemptTelemetry,
    /// TB-18R R1 (charter v2 §1 + Codex Gate 1 ratified 2026-05-06):
    /// canonical-encoded `LeanResult` bytes — Lean's verdict on one
    /// externalized candidate (exit_code + verified + stderr_cid +
    /// stdout_cid + proof_artifact_cid + error_class). Raw stderr /
    /// stdout stay shielded behind their own AuditOnly CAS objects.
    LeanResult,
    /// TB-18R R1 (charter v2 FR-18R.3 v2 + Codex Q4 remediation):
    /// canonical-encoded `TerminalAbortRecord` bytes for one aborted
    /// attempt (externally killed / per-call budget halt / WallClockCap
    /// during Lean / etc.). Per FR-18R.3 v2: aborted attempts are
    /// excluded from `evaluator_reported_completed_llm_calls` and counted
    /// in `attempt_aborted_count`; this CAS object provides the explicit
    /// per-aborted-attempt evidence so the chain-derived equation holds
    /// exactly after the sequencer drain barrier.
    TerminalAbortRecord,
    /// Constitution Landing First 2026-05-07 (HARNESS.md §3 G-016/G-019/
    /// G-021/G-028; architect ruling): canonical-encoded `PromptCapsule`
    /// bytes — tape-resident proof that the agent's prompt context was
    /// derivable from a fixed read-set + redaction policy + system prompt
    /// template hash. Carries `prompt_context_hash`, `read_set`,
    /// `policy_version`, `hidden_fields_redacted`, `visible_context_cid`,
    /// `system_prompt_template_hash`, `agent_view_manifest_cid`. Verbatim
    /// prompt bytes are NEVER stored here by default; verbatim is a
    /// separate Class-4 audit-only artifact requiring explicit
    /// ratification. See `src/runtime/prompt_capsule.rs`.
    PromptCapsule,
    /// Generic / unclassified blob.
    Generic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CasObjectMetadata {
    /// Content-addressed identifier (sha256 of content).
    pub cid: Cid,
    /// Backend-specific OID (git sha-1 for git2-rs backend); informational only.
    /// Different backends may have different OID schemes; Cid is canonical.
    pub backend_oid_hex: String,
    pub object_type: ObjectType,
    /// Submitter / author. Use "system" for runtime-emitted objects.
    pub creator: String,
    /// Logical time at insertion (assigned by sequencer; not wall clock).
    pub created_at_logical_t: u64,
    /// Optional schema identifier (JSON Schema URI, type tag, etc.).
    pub schema_id: Option<String>,
    /// Size of content in bytes (informational; not part of canonical hash).
    pub size_bytes: u64,
}

impl CasObjectMetadata {
    /// Canonical hash of metadata for Merkle tree inclusion.
    pub fn canonical_hash(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(self.cid.0);
        h.update(self.backend_oid_hex.as_bytes());
        h.update(serde_json::to_vec(&self.object_type).expect("object_type serialize"));
        h.update(self.creator.as_bytes());
        h.update(self.created_at_logical_t.to_be_bytes());
        if let Some(s) = &self.schema_id {
            h.update(b"\x01");
            h.update(s.as_bytes());
        } else {
            h.update(b"\x00");
        }
        h.update(self.size_bytes.to_be_bytes());
        h.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cid_from_empty_content() {
        let cid = Cid::from_content(b"");
        // SHA-256 of empty input = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        assert_eq!(
            cid.hex(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn cid_deterministic() {
        let cid_a = Cid::from_content(b"hello");
        let cid_b = Cid::from_content(b"hello");
        assert_eq!(cid_a, cid_b);
    }

    #[test]
    fn cid_differs_on_content() {
        let cid_a = Cid::from_content(b"hello");
        let cid_b = Cid::from_content(b"world");
        assert_ne!(cid_a, cid_b);
    }

    #[test]
    fn cid_display_format() {
        let cid = Cid::from_content(b"x");
        let s = cid.to_string();
        assert!(s.starts_with("cid:"));
        assert_eq!(s.len(), 4 + 64);
    }

    #[test]
    fn metadata_canonical_hash_deterministic() {
        let m = CasObjectMetadata {
            cid: Cid::from_content(b"x"),
            backend_oid_hex: "abc123".to_string(),
            object_type: ObjectType::ProposalPayload,
            creator: "alice".to_string(),
            created_at_logical_t: 100,
            schema_id: Some("v1/proposal".to_string()),
            size_bytes: 1,
        };
        assert_eq!(m.canonical_hash(), m.canonical_hash());
    }

    #[test]
    fn metadata_canonical_hash_differs_on_object_type() {
        let base = CasObjectMetadata {
            cid: Cid::from_content(b"x"),
            backend_oid_hex: "abc".to_string(),
            object_type: ObjectType::ProposalPayload,
            creator: "alice".to_string(),
            created_at_logical_t: 100,
            schema_id: None,
            size_bytes: 1,
        };
        let mut variant = base.clone();
        variant.object_type = ObjectType::CounterexamplePayload;
        assert_ne!(base.canonical_hash(), variant.canonical_hash());
    }

    // TB-18R R1: per preflight §7 — 4 ObjectType variant tests verifying
    // (a) the new variants serialize to distinct canonical-hash bytes from
    // each other and from existing variants, and (b) all 13 pre-existing
    // variants still serialize byte-identical to their TB-15 baseline (so
    // pre-TB-18R chain entries replay byte-identical per FR-18R.10 +
    // `feedback_no_retroactive_evidence_rewrite`).

    #[test]
    fn object_type_attempt_telemetry_canonical_hash_distinct() {
        // The new AttemptTelemetry variant must produce a canonical hash
        // distinct from every pre-existing variant.
        let mk = |t: ObjectType| CasObjectMetadata {
            cid: Cid::from_content(b"x"),
            backend_oid_hex: "abc".to_string(),
            object_type: t,
            creator: "evaluator".to_string(),
            created_at_logical_t: 100,
            schema_id: None,
            size_bytes: 1,
        };
        let attempt = mk(ObjectType::AttemptTelemetry);
        for other in [
            ObjectType::ProposalPayload,
            ObjectType::CounterexamplePayload,
            ObjectType::PredicateBytecode,
            ObjectType::ToolBytecode,
            ObjectType::AmendmentDiff,
            ObjectType::ReversibilityPlan,
            ObjectType::EvidenceCapsule,
            ObjectType::EvidenceManifest,
            ObjectType::CompressedRunLog,
            ObjectType::AgentAutopsyCapsule,
            ObjectType::AutopsyPrivateDetail,
            ObjectType::MarkovEvidenceCapsule,
            ObjectType::NextSessionContext,
            ObjectType::Generic,
        ] {
            assert_ne!(
                attempt.canonical_hash(),
                mk(other).canonical_hash(),
                "AttemptTelemetry canonical hash must differ from {:?}",
                other,
            );
        }
    }

    #[test]
    fn object_type_lean_result_canonical_hash_distinct() {
        let mk = |t: ObjectType| CasObjectMetadata {
            cid: Cid::from_content(b"x"),
            backend_oid_hex: "abc".to_string(),
            object_type: t,
            creator: "evaluator".to_string(),
            created_at_logical_t: 100,
            schema_id: None,
            size_bytes: 1,
        };
        let lean = mk(ObjectType::LeanResult);
        for other in [
            ObjectType::AttemptTelemetry,
            ObjectType::TerminalAbortRecord,
            ObjectType::ProposalPayload,
            ObjectType::Generic,
        ] {
            assert_ne!(
                lean.canonical_hash(),
                mk(other).canonical_hash(),
                "LeanResult canonical hash must differ from {:?}",
                other,
            );
        }
    }

    #[test]
    fn object_type_terminal_abort_record_canonical_hash_distinct() {
        let mk = |t: ObjectType| CasObjectMetadata {
            cid: Cid::from_content(b"x"),
            backend_oid_hex: "abc".to_string(),
            object_type: t,
            creator: "sequencer".to_string(),
            created_at_logical_t: 100,
            schema_id: None,
            size_bytes: 1,
        };
        let abort = mk(ObjectType::TerminalAbortRecord);
        for other in [
            ObjectType::AttemptTelemetry,
            ObjectType::LeanResult,
            ObjectType::EvidenceCapsule,
            ObjectType::Generic,
        ] {
            assert_ne!(
                abort.canonical_hash(),
                mk(other).canonical_hash(),
                "TerminalAbortRecord canonical hash must differ from {:?}",
                other,
            );
        }
    }

    #[test]
    fn object_type_pre_tb_18r_variants_unchanged() {
        // Per FR-18R.10 + `feedback_no_retroactive_evidence_rewrite`:
        // the 14 pre-TB-18R ObjectType variants must serialize to the same
        // canonical bytes after the TB-18R tail-append. This test pins their
        // serialized form so a future renumbering / reorder is caught.
        //
        // serde-json encoding: ObjectType uses derived externally-tagged
        // representation, so each variant serializes to its name string.
        // Tail-appending new variants does NOT affect the existing ones.
        let pre_tb_18r = [
            (ObjectType::ProposalPayload, "\"ProposalPayload\""),
            (ObjectType::CounterexamplePayload, "\"CounterexamplePayload\""),
            (ObjectType::PredicateBytecode, "\"PredicateBytecode\""),
            (ObjectType::ToolBytecode, "\"ToolBytecode\""),
            (ObjectType::AmendmentDiff, "\"AmendmentDiff\""),
            (ObjectType::ReversibilityPlan, "\"ReversibilityPlan\""),
            (ObjectType::EvidenceCapsule, "\"EvidenceCapsule\""),
            (ObjectType::EvidenceManifest, "\"EvidenceManifest\""),
            (ObjectType::CompressedRunLog, "\"CompressedRunLog\""),
            (ObjectType::AgentAutopsyCapsule, "\"AgentAutopsyCapsule\""),
            (ObjectType::AutopsyPrivateDetail, "\"AutopsyPrivateDetail\""),
            (ObjectType::MarkovEvidenceCapsule, "\"MarkovEvidenceCapsule\""),
            (ObjectType::NextSessionContext, "\"NextSessionContext\""),
            (ObjectType::Generic, "\"Generic\""),
        ];
        for (variant, expected) in pre_tb_18r {
            let actual = serde_json::to_string(&variant).expect("serialize");
            assert_eq!(
                actual, expected,
                "pre-TB-18R variant {:?} must serialize to {}",
                variant, expected,
            );
        }
        // Sanity: the new TB-18R variants serialize to their expected names.
        assert_eq!(
            serde_json::to_string(&ObjectType::AttemptTelemetry).expect("serialize"),
            "\"AttemptTelemetry\"",
        );
        assert_eq!(
            serde_json::to_string(&ObjectType::LeanResult).expect("serialize"),
            "\"LeanResult\"",
        );
        assert_eq!(
            serde_json::to_string(&ObjectType::TerminalAbortRecord).expect("serialize"),
            "\"TerminalAbortRecord\"",
        );
    }
}
