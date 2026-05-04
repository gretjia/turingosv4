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

// ────────────────────────────────────────────────────────────────────────────
// TB-11 Atom 3 — EvidenceCapsule writer (architect §6.1)
// ────────────────────────────────────────────────────────────────────────────

use crate::bottom_white::cas::schema::ObjectType;
use crate::bottom_white::cas::store::CasStore;
use crate::bottom_white::ledger::transition_ledger::canonical_encode;
// TaskId already imported via the schema-section `use` statement above.

/// TRACE_MATRIX TB-11 Atom 3 (architect §6.1 ruling 2026-05-02): error
/// taxonomy for the EvidenceCapsule writer.
#[derive(Debug)]
pub enum CapsuleWriteError {
    Cas(crate::bottom_white::cas::store::CasError),
    Encode(String),
    InternalLockPoisoned,
}

impl std::fmt::Display for CapsuleWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cas(e) => write!(f, "cas write failed: {e}"),
            Self::Encode(s) => write!(f, "encode failed: {s}"),
            Self::InternalLockPoisoned => write!(f, "internal lock poisoned"),
        }
    }
}
impl std::error::Error for CapsuleWriteError {}

impl From<crate::bottom_white::cas::store::CasError> for CapsuleWriteError {
    fn from(e: crate::bottom_white::cas::store::CasError) -> Self {
        Self::Cas(e)
    }
}

/// TRACE_MATRIX TB-11 Atom 3 (architect §6.1): write an EvidenceCapsule to
/// CAS. The flow:
///
/// 1. Compute sha256 of raw run log → write to CAS as `CompressedRunLog`.
///    (TB-11 MVP stores **uncompressed** raw bytes; gzip wrapping is
///    forward-compat in TB-15 Markov Loom. The Cid is still unique;
///    audit access still requires `privacy_policy: AuditOnly`.)
/// 2. Build minimal JSON manifest enumerating compressed_log_cid +
///    size_bytes + sha256 → write to CAS as `EvidenceManifest`.
/// 3. Build the `EvidenceCapsule` struct with `capsule_id =
///    Cid::default()` (placeholder). Canonical-encode + sha256 → that's
///    the eventual `capsule_id`.
/// 4. Re-create the struct with `capsule_id` filled in + write to CAS as
///    `EvidenceCapsule`.
///
/// Returns the populated `EvidenceCapsule` (with `capsule_id` set).
///
/// **Privacy** (architect §6.1 屏蔽规则): the capsule struct itself
/// includes `public_summary` (broadcast-eligible) + `compressed_log_cid`
/// (the audit-only handle). Caller controls `privacy_policy` at the call
/// site; `AuditOnly` is the recommended default and is enforced
/// elsewhere (dashboard, agent read view).
pub fn write_evidence_capsule(
    cas: &std::sync::Arc<std::sync::RwLock<CasStore>>,
    run_id: RunId,
    task_id: TaskId,
    solver_agent: Option<crate::state::q_state::AgentId>,
    counts: ExhaustionCounts,
    rounds: (u64, u64),
    terminal_reason: ExhaustionReason,
    raw_log_bytes: &[u8],
    privacy: CapsulePrivacyPolicy,
    creator_str: &str,
    created_at_logical_t: u64,
) -> Result<EvidenceCapsule, CapsuleWriteError> {
    // Step 1: write raw log to CAS (uncompressed for TB-11 MVP).
    let mut cas_w = cas
        .write()
        .map_err(|_| CapsuleWriteError::InternalLockPoisoned)?;
    let compressed_log_cid = cas_w.put(
        raw_log_bytes,
        ObjectType::CompressedRunLog,
        creator_str,
        created_at_logical_t,
        Some("v1/evidence_capsule_raw_log".into()),
    )?;
    // Step 2: build + write manifest JSON.
    let manifest_json = serde_json::json!({
        "schema_version": "v1/evidence_manifest",
        "compressed_log_cid": compressed_log_cid.hex(),
        "size_bytes_uncompressed": raw_log_bytes.len() as u64,
        "size_bytes_stored": raw_log_bytes.len() as u64,
        "compression": "none-tb11-mvp",
    });
    let manifest_bytes = serde_json::to_vec(&manifest_json)
        .map_err(|e| CapsuleWriteError::Encode(format!("manifest encode: {e}")))?;
    let evidence_manifest_cid = cas_w.put(
        &manifest_bytes,
        ObjectType::EvidenceManifest,
        creator_str,
        created_at_logical_t,
        Some("v1/evidence_manifest".into()),
    )?;

    // Step 3: build capsule with sha256 = 0 + capsule_id = 0; canonical
    // encode; sha256 of that is the eventual capsule_id.
    let public_summary = EvidenceCapsule::format_public_summary(&counts, terminal_reason);
    let mut capsule = EvidenceCapsule {
        capsule_id: Cid::default(),
        run_id: run_id.clone(),
        task_id: task_id.clone(),
        solver_agent: solver_agent.clone(),
        attempt_count: counts.attempt_count,
        lean_error_count: counts.lean_error_count,
        sorry_block_count: counts.sorry_block_count,
        protocol_parse_failure_count: counts.protocol_parse_failure_count,
        partial_accept_count: counts.partial_accept_count,
        started_at_round: rounds.0,
        ended_at_round: rounds.1,
        terminal_reason,
        public_summary,
        evidence_manifest_cid,
        compressed_log_cid,
        privacy_policy: privacy,
        sha256: crate::state::q_state::Hash::ZERO,
    };
    // TB-16 Atom 7 R1 Step 3 (architect §7.5 SG-16.6 + Codex TB-15 R2
    // writer-pattern fix carry-forward): capsule_id MUST equal
    // sha256(stored_bytes). Previous code stored DIFFERENT bytes (with
    // capsule_id + sha256 fields populated) than those whose sha256 was
    // capsule_id, breaking cas.get(capsule.capsule_id) — discovered by
    // TB-16 Atom 7 R1 Step 4 arena run5_exhaust (TB-11 latent bug; the
    // TB-15 R2 fix patched AgentAutopsyCapsule + MarkovEvidenceCapsule
    // writers but missed this older EvidenceCapsule writer).
    //
    // Fix: store the IDENTITY-ZEROED bytes in CAS (capsule_id +
    // sha256 = ZERO). The in-memory struct returned to the caller has
    // capsule_id + sha256 populated; readers use
    // restore_evidence_capsule_from_cas_bytes to reconstruct identity
    // post-fetch. Mirrors TB-15 R2 writer pattern.
    let stored_bytes = canonical_encode(&capsule)
        .map_err(|e| CapsuleWriteError::Encode(format!("capsule stored-bytes encode: {e:?}")))?;
    let capsule_cid = Cid::from_content(&stored_bytes);
    let _ = cas_w.put(
        &stored_bytes,
        ObjectType::EvidenceCapsule,
        creator_str,
        created_at_logical_t,
        Some("v1/evidence_capsule".into()),
    )?;
    // Populate identity fields on the returned struct (the on-disk bytes
    // remain the zeroed-identity form; capsule.capsule_id is the Cid of
    // those stored bytes).
    capsule.capsule_id = capsule_cid;
    capsule.sha256 = crate::state::q_state::Hash(capsule_cid.0);

    Ok(capsule)
}

/// TRACE_MATRIX TB-16 Atom 7 R1 Step 3 (architect §7.5 SG-16.6 carry-
/// forward of TB-15 R2 writer-pattern fix): reconstruct an
/// `EvidenceCapsule` from CAS-stored bytes (which have capsule_id +
/// sha256 = ZERO). Caller supplies the Cid that was returned by
/// `write_evidence_capsule`; this helper reads CAS, decodes, and
/// re-populates capsule_id + sha256 from the Cid.
pub fn restore_evidence_capsule_from_cas_bytes(
    bytes: &[u8],
) -> Result<EvidenceCapsule, CapsuleWriteError> {
    use crate::bottom_white::ledger::transition_ledger::canonical_decode;
    let mut capsule: EvidenceCapsule = canonical_decode(bytes)
        .map_err(|e| CapsuleWriteError::Encode(format!("capsule decode: {e:?}")))?;
    let cid = Cid::from_content(bytes);
    capsule.capsule_id = cid;
    capsule.sha256 = crate::state::q_state::Hash(cid.0);
    Ok(capsule)
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

    /// TB-11 Atom 3 — Writer: writes raw log + manifest + capsule to CAS;
    /// returned capsule has populated capsule_id (Cid of canonical bytes).
    #[test]
    fn write_evidence_capsule_to_cas_round_trip() {
        use std::sync::{Arc, RwLock};
        use tempfile::TempDir;

        let tmp = TempDir::new().expect("tempdir");
        let cas = Arc::new(RwLock::new(
            crate::bottom_white::cas::store::CasStore::open(tmp.path()).expect("cas"),
        ));

        let counts = ExhaustionCounts {
            attempt_count: 132,
            lean_error_count: 73,
            sorry_block_count: 14,
            protocol_parse_failure_count: 26,
            partial_accept_count: 32,
        };
        let raw_log = b"FAKE_RUN_LOG\n[attempt 1]: lean error\n[attempt 132]: max-tx exhausted\n";

        let capsule = write_evidence_capsule(
            &cas,
            RunId("run-zeta-001".into()),
            crate::state::q_state::TaskId(
                "task:lean:heldout_49:zeta_regularization".into(),
            ),
            Some(crate::state::q_state::AgentId("Agent_solver_0".into())),
            counts,
            (0, 1300),
            ExhaustionReason::MaxTxExhausted,
            raw_log,
            CapsulePrivacyPolicy::AuditOnly,
            "evaluator-tb11",
            1,
        )
        .expect("writer succeeds");

        // capsule_id populated and matches sha256.
        assert_ne!(capsule.capsule_id, Cid::default());
        assert_eq!(capsule.capsule_id.0, capsule.sha256.0);

        // Counts faithfully recorded.
        assert_eq!(capsule.attempt_count, 132);
        assert_eq!(capsule.lean_error_count, 73);
        assert_eq!(capsule.sorry_block_count, 14);
        assert_eq!(capsule.protocol_parse_failure_count, 26);
        assert_eq!(capsule.partial_accept_count, 32);
        assert_eq!(capsule.terminal_reason, ExhaustionReason::MaxTxExhausted);

        // public_summary contains all 5 counts + reason.
        assert!(capsule.public_summary.contains("132 attempts"));
        assert!(capsule.public_summary.contains("73 lean errors"));
        assert!(capsule.public_summary.contains("MaxTxExhausted"));

        // CAS contains 3 objects: raw log + manifest + capsule itself.
        let cas_r = cas.read().expect("cas read");
        assert_eq!(cas_r.len(), 3, "writer puts 3 CAS objects: log + manifest + capsule");

        // raw log retrievable by compressed_log_cid.
        let retrieved = cas_r.get(&capsule.compressed_log_cid).expect("get raw");
        assert_eq!(retrieved, raw_log);
    }

    /// TB-11 Atom 3 — Writer: same inputs → same capsule_id (deterministic).
    #[test]
    fn write_evidence_capsule_deterministic_capsule_id() {
        use std::sync::{Arc, RwLock};
        use tempfile::TempDir;

        let counts = ExhaustionCounts {
            attempt_count: 5,
            lean_error_count: 3,
            sorry_block_count: 1,
            protocol_parse_failure_count: 1,
            partial_accept_count: 0,
        };
        let raw_log = b"deterministic test";

        let cap_a = {
            let tmp_a = TempDir::new().unwrap();
            let cas_a = Arc::new(RwLock::new(
                crate::bottom_white::cas::store::CasStore::open(tmp_a.path()).unwrap(),
            ));
            write_evidence_capsule(
                &cas_a,
                RunId("run-A".into()),
                crate::state::q_state::TaskId("t-A".into()),
                None,
                counts,
                (10, 20),
                ExhaustionReason::MaxTxExhausted,
                raw_log,
                CapsulePrivacyPolicy::AuditOnly,
                "writer",
                1,
            )
            .expect("writer A")
        };
        let cap_b = {
            let tmp_b = TempDir::new().unwrap();
            let cas_b = Arc::new(RwLock::new(
                crate::bottom_white::cas::store::CasStore::open(tmp_b.path()).unwrap(),
            ));
            write_evidence_capsule(
                &cas_b,
                RunId("run-A".into()),
                crate::state::q_state::TaskId("t-A".into()),
                None,
                counts,
                (10, 20),
                ExhaustionReason::MaxTxExhausted,
                raw_log,
                CapsulePrivacyPolicy::AuditOnly,
                "writer",
                1,
            )
            .expect("writer B")
        };
        assert_eq!(cap_a.capsule_id, cap_b.capsule_id);
        assert_eq!(cap_a.compressed_log_cid, cap_b.compressed_log_cid);
        assert_eq!(cap_a.evidence_manifest_cid, cap_b.evidence_manifest_cid);
    }
}
