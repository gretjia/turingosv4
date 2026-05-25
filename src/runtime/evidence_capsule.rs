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

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::{Read, Write};

use crate::bottom_white::cas::schema::Cid;
use crate::state::q_state::{AgentId, Hash, TaskId};
use crate::state::typed_tx::{CapsulePrivacyPolicy, ExhaustionReason, RunId};

const DEFAULT_MAX_EVIDENCE_LOG_UNCOMPRESSED_BYTES: u64 = 64 * 1024 * 1024;

#[cfg(test)]
thread_local! {
    static TEST_MAX_EVIDENCE_LOG_UNCOMPRESSED_BYTES: std::cell::Cell<Option<u64>> =
        const { std::cell::Cell::new(None) };
}

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

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn gzip_compress(bytes: &[u8]) -> Result<Vec<u8>, CapsuleWriteError> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(bytes)
        .map_err(|e| CapsuleWriteError::Encode(format!("gzip write: {e}")))?;
    encoder
        .finish()
        .map_err(|e| CapsuleWriteError::Encode(format!("gzip finish: {e}")))
}

fn max_evidence_log_uncompressed_bytes() -> u64 {
    #[cfg(test)]
    if let Some(max_bytes) = TEST_MAX_EVIDENCE_LOG_UNCOMPRESSED_BYTES.with(|slot| slot.get()) {
        return max_bytes;
    }

    std::env::var("TURINGOS_EVIDENCE_LOG_MAX_UNCOMPRESSED_BYTES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_MAX_EVIDENCE_LOG_UNCOMPRESSED_BYTES)
}

fn gzip_decompress_bounded(bytes: &[u8], expected_size: u64) -> Result<Vec<u8>, CapsuleWriteError> {
    let max_size = max_evidence_log_uncompressed_bytes();
    if expected_size > max_size {
        return Err(CapsuleWriteError::Encode(format!(
            "gzip evidence log manifest size {expected_size} exceeds max {max_size}"
        )));
    }
    let mut decoder = GzDecoder::new(bytes);
    let limit = expected_size
        .checked_add(1)
        .ok_or_else(|| CapsuleWriteError::Encode("gzip evidence log size overflow".into()))?;
    let mut limited = decoder.by_ref().take(limit);
    let mut out = Vec::new();
    limited
        .read_to_end(&mut out)
        .map_err(|e| CapsuleWriteError::Encode(format!("gzip decode: {e}")))?;
    if out.len() as u64 > expected_size {
        return Err(CapsuleWriteError::Encode(format!(
            "gzip evidence log exceeded manifest uncompressed size {expected_size}"
        )));
    }
    Ok(out)
}

/// Read the audit-only raw log for an EvidenceCapsule. New capsules store a
/// gzip-compressed log and verify the uncompressed sha256 from the manifest;
/// historical TB-11 `none-tb11-mvp` capsules remain readable.
///
/// TRACE_MATRIX FC3-archive + Art. 0.2: audit-only raw-log readback must
/// verify CAS/manifest compression metadata without becoming agent prompt
/// source-of-truth.
pub fn read_evidence_capsule_raw_log(
    cas: &CasStore,
    capsule: &EvidenceCapsule,
) -> Result<Vec<u8>, CapsuleWriteError> {
    let manifest_bytes = cas.get(&capsule.evidence_manifest_cid)?;
    let manifest: serde_json::Value = serde_json::from_slice(&manifest_bytes)
        .map_err(|e| CapsuleWriteError::Encode(format!("manifest decode: {e}")))?;
    if let Some(cid_hex) = manifest.get("compressed_log_cid").and_then(|v| v.as_str()) {
        if cid_hex != capsule.compressed_log_cid.hex() {
            return Err(CapsuleWriteError::Encode(format!(
                "manifest compressed_log_cid {cid_hex} does not match capsule {}",
                capsule.compressed_log_cid.hex()
            )));
        }
    }

    let stored = cas.get(&capsule.compressed_log_cid)?;
    if let Some(expected) = manifest.get("size_bytes_stored").and_then(|v| v.as_u64()) {
        if stored.len() as u64 != expected {
            return Err(CapsuleWriteError::Encode(format!(
                "stored log size mismatch: manifest={expected}, actual={}",
                stored.len()
            )));
        }
    }

    let expected_uncompressed_size = manifest
        .get("size_bytes_uncompressed")
        .and_then(|v| v.as_u64());
    let expected_uncompressed_sha = manifest.get("uncompressed_sha256").and_then(|v| v.as_str());
    let algorithm = manifest
        .get("compression_algorithm")
        .or_else(|| manifest.get("compression"))
        .and_then(|v| v.as_str())
        .unwrap_or("none-tb11-mvp");
    let raw = match algorithm {
        "gzip" => {
            let expected = expected_uncompressed_size.ok_or_else(|| {
                CapsuleWriteError::Encode(
                    "gzip evidence log manifest missing size_bytes_uncompressed".into(),
                )
            })?;
            if expected_uncompressed_sha.is_none() {
                return Err(CapsuleWriteError::Encode(
                    "gzip evidence log manifest missing uncompressed_sha256".into(),
                ));
            }
            gzip_decompress_bounded(&stored, expected)?
        }
        "none-tb11-mvp" | "none" => stored,
        other => {
            return Err(CapsuleWriteError::Encode(format!(
                "unsupported evidence log compression algorithm: {other}"
            )))
        }
    };

    if let Some(expected) = expected_uncompressed_size {
        if raw.len() as u64 != expected {
            return Err(CapsuleWriteError::Encode(format!(
                "uncompressed log size mismatch: manifest={expected}, actual={}",
                raw.len()
            )));
        }
    }
    if let Some(expected) = expected_uncompressed_sha {
        let actual = sha256_hex(&raw);
        if actual != expected {
            return Err(CapsuleWriteError::Encode(format!(
                "uncompressed log sha256 mismatch: manifest={expected}, actual={actual}"
            )));
        }
    }

    Ok(raw)
}

/// TRACE_MATRIX TB-11 Atom 3 (architect §6.1): write an EvidenceCapsule to
/// CAS. The flow:
///
/// 1. Gzip-compress raw run log → write to CAS as `CompressedRunLog`.
///    The manifest records algorithm, raw/stored sizes, and uncompressed
///    sha256; audit access still requires `privacy_policy: AuditOnly`.
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
    // Step 1: write gzip-compressed raw log to CAS.
    let raw_len = raw_log_bytes.len() as u64;
    let max_raw_len = max_evidence_log_uncompressed_bytes();
    if raw_len > max_raw_len {
        return Err(CapsuleWriteError::Encode(format!(
            "raw evidence log size {raw_len} exceeds max {max_raw_len}"
        )));
    }
    let compressed_log_bytes = gzip_compress(raw_log_bytes)?;
    let raw_log_sha256 = sha256_hex(raw_log_bytes);
    let mut cas_w = cas
        .write()
        .map_err(|_| CapsuleWriteError::InternalLockPoisoned)?;
    let compressed_log_cid = cas_w.put(
        &compressed_log_bytes,
        ObjectType::CompressedRunLog,
        creator_str,
        created_at_logical_t,
        Some("v2/evidence_capsule_raw_log.gzip".into()),
    )?;
    // Step 2: build + write manifest JSON.
    let manifest_json = serde_json::json!({
        "schema_version": "v2/evidence_manifest",
        "compressed_log_cid": compressed_log_cid.hex(),
        "compression_algorithm": "gzip",
        "compression": "gzip",
        "size_bytes_uncompressed": raw_log_bytes.len() as u64,
        "size_bytes_stored": compressed_log_bytes.len() as u64,
        "uncompressed_sha256": raw_log_sha256,
    });
    let manifest_bytes = serde_json::to_vec(&manifest_json)
        .map_err(|e| CapsuleWriteError::Encode(format!("manifest encode: {e}")))?;
    let evidence_manifest_cid = cas_w.put(
        &manifest_bytes,
        ObjectType::EvidenceManifest,
        creator_str,
        created_at_logical_t,
        Some("v2/evidence_manifest".into()),
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

    struct EvidenceLogMaxGuard {
        old: Option<u64>,
    }

    impl EvidenceLogMaxGuard {
        fn set(key: &'static str, value: &str) -> Self {
            assert_eq!(key, "TURINGOS_EVIDENCE_LOG_MAX_UNCOMPRESSED_BYTES");
            let max_bytes = value.parse::<u64>().expect("test max bytes is u64");
            let old = TEST_MAX_EVIDENCE_LOG_UNCOMPRESSED_BYTES.with(|slot| {
                let old = slot.get();
                slot.set(Some(max_bytes));
                old
            });
            Self { old }
        }
    }

    impl Drop for EvidenceLogMaxGuard {
        fn drop(&mut self) {
            TEST_MAX_EVIDENCE_LOG_UNCOMPRESSED_BYTES.with(|slot| slot.set(self.old));
        }
    }

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
            crate::state::q_state::TaskId("task:lean:heldout_49:zeta_regularization".into()),
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
        assert_eq!(
            cas_r.len(),
            3,
            "writer puts 3 CAS objects: log + manifest + capsule"
        );

        // raw log retrievable through manifest-verified decompression.
        let retrieved =
            read_evidence_capsule_raw_log(&cas_r, &capsule).expect("get/decompress raw");
        assert_eq!(retrieved, raw_log);
    }

    #[test]
    fn compressed_raw_log_round_trips_and_manifest_hash_verifies() {
        use sha2::{Digest, Sha256};
        use std::sync::{Arc, RwLock};
        use tempfile::TempDir;

        let tmp = TempDir::new().expect("tempdir");
        let cas = Arc::new(RwLock::new(
            crate::bottom_white::cas::store::CasStore::open(tmp.path()).expect("cas"),
        ));

        let counts = ExhaustionCounts {
            attempt_count: 12,
            lean_error_count: 8,
            sorry_block_count: 2,
            protocol_parse_failure_count: 1,
            partial_accept_count: 1,
        };
        let raw_log = b"attempt=1 lean_error\nattempt=2 lean_error\nattempt=3 lean_error\n\
            attempt=4 lean_error\nattempt=5 lean_error\nattempt=6 lean_error\n";

        let capsule = write_evidence_capsule(
            &cas,
            RunId("run-compression".into()),
            crate::state::q_state::TaskId("task:compression".into()),
            None,
            counts,
            (3, 15),
            ExhaustionReason::MaxTxExhausted,
            raw_log,
            CapsulePrivacyPolicy::AuditOnly,
            "evaluator-compression",
            15,
        )
        .expect("writer succeeds");

        let cas_r = cas.read().expect("cas read");
        let manifest_bytes = cas_r.get(&capsule.evidence_manifest_cid).expect("manifest");
        let manifest: serde_json::Value =
            serde_json::from_slice(&manifest_bytes).expect("manifest json");
        let manifest_meta = cas_r
            .metadata(&capsule.evidence_manifest_cid)
            .expect("manifest metadata");
        assert_eq!(manifest["compression_algorithm"], "gzip");
        assert_eq!(manifest["schema_version"], "v2/evidence_manifest");
        assert_eq!(
            manifest_meta.schema_id.as_deref(),
            Some("v2/evidence_manifest"),
            "manifest JSON schema_version and CAS metadata schema_id must match"
        );
        assert_eq!(
            manifest["size_bytes_uncompressed"].as_u64(),
            Some(raw_log.len() as u64)
        );
        assert!(
            manifest["size_bytes_stored"].as_u64().unwrap() < raw_log.len() as u64,
            "repetitive raw log should be materially compressed"
        );

        let mut h = Sha256::new();
        h.update(raw_log);
        let expected_sha = h
            .finalize()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        assert_eq!(manifest["uncompressed_sha256"], expected_sha);

        let stored_bytes = cas_r
            .get(&capsule.compressed_log_cid)
            .expect("compressed log");
        assert_ne!(
            stored_bytes, raw_log,
            "new EvidenceCapsule raw logs must be stored compressed"
        );
        let restored = read_evidence_capsule_raw_log(&cas_r, &capsule)
            .expect("decompress and verify manifest hash");
        assert_eq!(restored, raw_log);
    }

    #[test]
    fn writer_rejects_raw_log_above_default_readback_cap() {
        use std::sync::{Arc, RwLock};
        use tempfile::TempDir;

        let _guard = EvidenceLogMaxGuard::set("TURINGOS_EVIDENCE_LOG_MAX_UNCOMPRESSED_BYTES", "4");
        let tmp = TempDir::new().expect("tempdir");
        let cas = Arc::new(RwLock::new(
            crate::bottom_white::cas::store::CasStore::open(tmp.path()).expect("cas"),
        ));
        let counts = ExhaustionCounts {
            attempt_count: 1,
            lean_error_count: 1,
            sorry_block_count: 0,
            protocol_parse_failure_count: 0,
            partial_accept_count: 0,
        };
        let err = write_evidence_capsule(
            &cas,
            RunId("run-too-large".into()),
            crate::state::q_state::TaskId("task:too-large".into()),
            None,
            counts,
            (0, 1),
            ExhaustionReason::MaxTxExhausted,
            b"12345",
            CapsulePrivacyPolicy::AuditOnly,
            "evaluator",
            1,
        )
        .expect_err("writer must not create capsules that default readback rejects");
        assert!(
            err.to_string().contains("raw evidence log size"),
            "expected raw size cap error, got {err}"
        );
    }

    #[test]
    fn gzip_manifest_missing_uncompressed_size_fails_closed() {
        use std::sync::{Arc, RwLock};
        use tempfile::TempDir;

        let tmp = TempDir::new().expect("tempdir");
        let cas = Arc::new(RwLock::new(
            crate::bottom_white::cas::store::CasStore::open(tmp.path()).expect("cas"),
        ));
        let raw_log = b"bounded gzip log\nbounded gzip log\n";
        let compressed = gzip_compress(raw_log).expect("compress");
        let raw_sha = sha256_hex(raw_log);
        let mut cas_w = cas.write().expect("cas write");
        let compressed_log_cid = cas_w
            .put(
                &compressed,
                ObjectType::CompressedRunLog,
                "test",
                1,
                Some("v2/evidence_capsule_raw_log.gzip".into()),
            )
            .expect("compressed log put");
        let manifest = serde_json::json!({
            "schema_version": "v2/evidence_manifest",
            "compressed_log_cid": compressed_log_cid.hex(),
            "compression_algorithm": "gzip",
            "compression": "gzip",
            "size_bytes_stored": compressed.len() as u64,
            "uncompressed_sha256": raw_sha,
        });
        let manifest_bytes = serde_json::to_vec(&manifest).expect("manifest encode");
        let evidence_manifest_cid = cas_w
            .put(
                &manifest_bytes,
                ObjectType::EvidenceManifest,
                "test",
                1,
                Some("v2/evidence_manifest".into()),
            )
            .expect("manifest put");
        drop(cas_w);

        let capsule = EvidenceCapsule {
            evidence_manifest_cid,
            compressed_log_cid,
            ..EvidenceCapsule::default()
        };
        let cas_r = cas.read().expect("cas read");
        let err = read_evidence_capsule_raw_log(&cas_r, &capsule)
            .expect_err("gzip manifest missing size must fail closed");
        assert!(
            err.to_string().contains("size_bytes_uncompressed"),
            "expected missing size error, got {err}"
        );
    }

    #[test]
    fn gzip_manifest_understated_uncompressed_size_fails_bounded() {
        use std::sync::{Arc, RwLock};
        use tempfile::TempDir;

        let tmp = TempDir::new().expect("tempdir");
        let cas = Arc::new(RwLock::new(
            crate::bottom_white::cas::store::CasStore::open(tmp.path()).expect("cas"),
        ));
        let raw_log = b"this expands beyond the claimed size";
        let compressed = gzip_compress(raw_log).expect("compress");
        let raw_sha = sha256_hex(raw_log);
        let mut cas_w = cas.write().expect("cas write");
        let compressed_log_cid = cas_w
            .put(
                &compressed,
                ObjectType::CompressedRunLog,
                "test",
                1,
                Some("v2/evidence_capsule_raw_log.gzip".into()),
            )
            .expect("compressed log put");
        let manifest = serde_json::json!({
            "schema_version": "v2/evidence_manifest",
            "compressed_log_cid": compressed_log_cid.hex(),
            "compression_algorithm": "gzip",
            "compression": "gzip",
            "size_bytes_uncompressed": 1_u64,
            "size_bytes_stored": compressed.len() as u64,
            "uncompressed_sha256": raw_sha,
        });
        let manifest_bytes = serde_json::to_vec(&manifest).expect("manifest encode");
        let evidence_manifest_cid = cas_w
            .put(
                &manifest_bytes,
                ObjectType::EvidenceManifest,
                "test",
                1,
                Some("v2/evidence_manifest".into()),
            )
            .expect("manifest put");
        drop(cas_w);

        let capsule = EvidenceCapsule {
            evidence_manifest_cid,
            compressed_log_cid,
            ..EvidenceCapsule::default()
        };
        let cas_r = cas.read().expect("cas read");
        let err = read_evidence_capsule_raw_log(&cas_r, &capsule)
            .expect_err("gzip manifest understated size must fail closed");
        assert!(
            err.to_string()
                .contains("exceeded manifest uncompressed size"),
            "expected bounded decode error, got {err}"
        );
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
