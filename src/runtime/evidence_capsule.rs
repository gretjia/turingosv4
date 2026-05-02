//! TB-11 Atom 1 — `EvidenceCapsule` schema (architect §6.1, ruling
//! 2026-05-02).
//!
//! O(1) chain cost, O(N) auditability. The chain anchors a single
//! `evidence_capsule_cid: Cid` on `TerminalSummaryTx` (architect's
//! RunExhaustedTx) or `TaskBankruptcyTx`; the capsule itself, plus its
//! manifest and compressed run log, live in CAS. Privacy default
//! `CapsulePrivacyPolicy::AuditOnly` — only `public_summary` surfaces
//! to non-audit views per architect §6.1 屏蔽规则.
//!
//! The writer (Atom 3) lives in this module too, so this file is the
//! complete surface for capsule production.
//!
//! TRACE_MATRIX FC3-N1 + Art. 0.2 (Tape Canonical: capsule canonical bytes
//! are themselves the CAS object referenced by `capsule_id`).
//!
//! /// TRACE_MATRIX architect §6.1 ruling 2026-05-02: EvidenceCapsule schema.

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::Cid;
use crate::state::q_state::{AgentId, Hash, TaskId};
use crate::state::typed_tx::{CapsulePrivacyPolicy, ExhaustionReason, RunId};

/// TRACE_MATRIX TB-11 (architect §6.1 ruling 2026-05-02) — CAS-resident
/// evidence rollup for a failed evaluator run.
///
/// The struct is canonical-encoded into CAS; `capsule_id` is the Cid of
/// those bytes and is set by the writer (Atom 3). For Atom 1, only the
/// schema + Default fixture exist.
///
/// **Privacy** (architect §6.1 屏蔽规则):
/// - `public_summary`: low-information string surface; can enter dashboard /
///   broadcast.
/// - `evidence_manifest_cid`: JSON manifest enumerating sub-CAS objects.
/// - `compressed_log_cid`: gzipped raw run log; access requires the
///   capsule's `privacy_policy` to permit the requesting role
///   (`AuditOnly` blocks default Agent reads).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceCapsule {
    /// CAS Cid of the canonical-encoded EvidenceCapsule itself. Set by the
    /// writer post-encode; before-set value is `Cid::default()` (32 zero
    /// bytes) — the writer canonical-encodes the struct with this field
    /// zeroed, takes the sha256, and returns a fresh struct with the
    /// resulting Cid filled in. (Future TB may make this a non-stored
    /// derivative, but for TB-11 we keep it as a stored field for ease
    /// of replay.)
    pub capsule_id: Cid,

    /// Backref to the run.
    pub run_id: RunId,
    /// Backref to the task.
    pub task_id: TaskId,
    /// Owner of the failed run, if any (None when no solver was assigned).
    pub solver_agent: Option<AgentId>,

    // ── Architect §6.1 mandated counts ───────────────────────────────────
    pub attempt_count: u64,
    pub lean_error_count: u64,
    pub sorry_block_count: u64,
    pub protocol_parse_failure_count: u64,
    pub partial_accept_count: u64,

    /// First logical_t observed in the run.
    pub started_at_round: u64,
    /// Last logical_t observed.
    pub ended_at_round: u64,
    /// Architect §6.1: terminal failure mode.
    pub terminal_reason: ExhaustionReason,

    // ── Architect §6.1 mandated content ──────────────────────────────────
    /// Low-pollution one-line summary surfaced to dashboard / broadcast.
    pub public_summary: String,
    /// JSON manifest enumerating sub-CAS objects (compressed log Cid +
    /// size + sha256). Stored separately so the capsule itself stays small.
    pub evidence_manifest_cid: Cid,
    /// CAS Cid of the gzipped raw run log. Access requires
    /// `privacy_policy` to permit the requesting role.
    pub compressed_log_cid: Cid,

    /// Architect §6.1 屏蔽规则 — privacy default `AuditOnly`.
    pub privacy_policy: CapsulePrivacyPolicy,

    /// SHA-256 of the canonical-encoded capsule bytes (with `capsule_id`
    /// zeroed during the hash). Defense-in-depth duplicate of `capsule_id`.
    pub sha256: Hash,
}

impl Default for EvidenceCapsule {
    fn default() -> Self {
        Self {
            capsule_id: Cid::default(),
            run_id: RunId::default(),
            task_id: TaskId::default(),
            solver_agent: None,
            attempt_count: 0,
            lean_error_count: 0,
            sorry_block_count: 0,
            protocol_parse_failure_count: 0,
            partial_accept_count: 0,
            started_at_round: 0,
            ended_at_round: 0,
            terminal_reason: ExhaustionReason::default(),
            public_summary: String::new(),
            evidence_manifest_cid: Cid::default(),
            compressed_log_cid: Cid::default(),
            privacy_policy: CapsulePrivacyPolicy::default(),
            sha256: Hash::ZERO,
        }
    }
}

/// TRACE_MATRIX TB-11 — counts surface for the writer API. The writer
/// (Atom 3) takes this struct + raw log bytes and produces an
/// `EvidenceCapsule` written to CAS.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExhaustionCounts {
    pub attempt_count: u64,
    pub lean_error_count: u64,
    pub sorry_block_count: u64,
    pub protocol_parse_failure_count: u64,
    pub partial_accept_count: u64,
}

impl EvidenceCapsule {
    /// TRACE_MATRIX architect §6.1 — formats the architect-mandated
    /// counts into a public_summary string. Used by the writer to fill
    /// `public_summary` in a deterministic, low-pollution shape.
    ///
    /// Example:
    /// ```text
    /// "132 attempts; 73 lean errors; 14 sorry-blocks; 26 parse failures; 32 partial accepts; reason=MaxTxExhausted; no accepted proof"
    /// ```
    pub fn format_public_summary(
        counts: &ExhaustionCounts,
        terminal_reason: ExhaustionReason,
    ) -> String {
        format!(
            "{} attempts; {} lean errors; {} sorry-blocks; {} parse failures; \
             {} partial accepts; reason={:?}; no accepted proof",
            counts.attempt_count,
            counts.lean_error_count,
            counts.sorry_block_count,
            counts.protocol_parse_failure_count,
            counts.partial_accept_count,
            terminal_reason,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TB-11 U1: EvidenceCapsule default round-trips through canonical bytes.
    #[test]
    fn evidence_capsule_default_round_trip() {
        use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
        let c = EvidenceCapsule::default();
        let bytes = canonical_encode(&c).expect("encode");
        let back: EvidenceCapsule = canonical_decode(&bytes).expect("decode");
        assert_eq!(c, back);
    }

    /// TB-11 U2: format_public_summary embeds all 5 architect-mandated counts.
    #[test]
    fn format_public_summary_contains_all_counts() {
        let counts = ExhaustionCounts {
            attempt_count: 132,
            lean_error_count: 73,
            sorry_block_count: 14,
            protocol_parse_failure_count: 26,
            partial_accept_count: 32,
        };
        let s = EvidenceCapsule::format_public_summary(&counts, ExhaustionReason::MaxTxExhausted);
        assert!(s.contains("132"));
        assert!(s.contains("73"));
        assert!(s.contains("14"));
        assert!(s.contains("26"));
        assert!(s.contains("32"));
        assert!(s.contains("MaxTxExhausted"));
    }

    /// TB-11 U3: privacy_policy default is AuditOnly per architect §6.1
    /// 屏蔽规则.
    #[test]
    fn privacy_policy_default_is_audit_only() {
        let c = EvidenceCapsule::default();
        assert_eq!(c.privacy_policy, CapsulePrivacyPolicy::AuditOnly);
    }
}
