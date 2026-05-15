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

use std::io::Write;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::bottom_white::cas::schema::Cid;
use crate::bottom_white::ledger::transition_ledger::TxKind;
use crate::runtime::attempt_telemetry::LeanErrorClass;
use crate::state::q_state::{AgentId, Hash};

/// TB-6 Atom 1.2 — JSONL-backend shadow struct.
///
/// Mirrors `RejectedSubmissionRecord` 1-to-1 but does NOT carry the
/// `#[serde(skip_serializing, default)]` attribute on `raw_diagnostic_cid`
/// (TB-1 P0-3 shield). The skip attribute is correct for `PublicRejectionView`
/// projection (Inv 10) but inappropriate for the L4.E persistent backend —
/// the JSONL file is a forensic ledger, NOT an agent-facing view, and MUST
/// preserve every field that contributes to `compute_hash` so reload + chain
/// verification round-trip is bit-deterministic.
#[derive(Serialize, Deserialize)]
struct JsonlRecord {
    submit_id: u64,
    parent_state_root: Hash,
    agent_id: AgentId,
    tx_kind: TxKind,
    tx_payload_cid: Cid,
    rejection_class: RejectionClass,
    raw_diagnostic_cid: Option<Cid>,
    public_summary: Option<String>,
    prev_hash: Hash,
    hash: Hash,
}

impl From<&RejectedSubmissionRecord> for JsonlRecord {
    fn from(r: &RejectedSubmissionRecord) -> Self {
        Self {
            submit_id: r.submit_id,
            parent_state_root: r.parent_state_root,
            agent_id: r.agent_id.clone(),
            tx_kind: r.tx_kind,
            tx_payload_cid: r.tx_payload_cid.clone(),
            rejection_class: r.rejection_class,
            raw_diagnostic_cid: r.raw_diagnostic_cid.clone(),
            public_summary: r.public_summary.clone(),
            prev_hash: r.prev_hash,
            hash: r.hash,
        }
    }
}

impl From<JsonlRecord> for RejectedSubmissionRecord {
    fn from(j: JsonlRecord) -> Self {
        Self {
            submit_id: j.submit_id,
            parent_state_root: j.parent_state_root,
            agent_id: j.agent_id,
            tx_kind: j.tx_kind,
            tx_payload_cid: j.tx_payload_cid,
            rejection_class: j.rejection_class,
            raw_diagnostic_cid: j.raw_diagnostic_cid,
            public_summary: j.public_summary,
            prev_hash: j.prev_hash,
            hash: j.hash,
        }
    }
}

/// TB-6 Atom 1.2 — single-record JSONL flush helper.
///
/// Serializes `record` (via `JsonlRecord` shadow — preserves `raw_diagnostic_cid`)
/// as one JSON line, opens `path` in append mode (creating if missing), writes
/// the line + newline, and `flush()` then `sync_data()` before returning.
/// Errors are wrapped into `RejectionEvidenceError::Io`.
fn flush_jsonl_record(
    path: &std::path::Path,
    record: &RejectedSubmissionRecord,
) -> Result<(), RejectionEvidenceError> {
    let shadow = JsonlRecord::from(record);
    let line = serde_json::to_string(&shadow)
        .map_err(|e| RejectionEvidenceError::Io(format!("serialize record: {e}")))?;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| RejectionEvidenceError::Io(format!("open append {path:?}: {e}")))?;
    file.write_all(line.as_bytes())
        .map_err(|e| RejectionEvidenceError::Io(format!("write: {e}")))?;
    file.write_all(b"\n")
        .map_err(|e| RejectionEvidenceError::Io(format!("write newline: {e}")))?;
    file.flush()
        .map_err(|e| RejectionEvidenceError::Io(format!("flush: {e}")))?;
    file.sync_data()
        .map_err(|e| RejectionEvidenceError::Io(format!("sync_data: {e}")))?;

    // Stage A3 / HEAD_t C2 R3.5 — advance refs/chaintape/l4e on each L4.E
    // append, when env var `TURINGOS_CHAINTAPE_PATH` points at the same
    // runtime_repo. Per CR-A3-HEAD-T-C2.5 the ref IS the canonical L4.E
    // pointer; the JSONL file remains the public-summary backing store
    // for backward compatibility. Best-effort: a ref-update failure does
    // NOT roll back the durable JSONL append. Per FR-A3-HEAD-T-C2.6 the
    // pre-Stage-A3 evidence (JSONL-only) remains replayable via existing
    // tooling.
    if let Ok(repo_path) = std::env::var("TURINGOS_CHAINTAPE_PATH") {
        let repo_path = std::path::PathBuf::from(&repo_path);
        // Synthesize a deterministic commit OID that anchors the L4.E
        // record's hash chain. tree blob = canonical JSONL bytes for the
        // record; commit message references submit_id; author/committer
        // time = submit_id (deterministic, no wall-clock leakage).
        let blob_bytes = line.as_bytes();
        let _ = advance_l4e_ref_for_record(&repo_path, record, blob_bytes);
    }

    Ok(())
}

/// TRACE_MATRIX § 3 orphan (Stage A3 / HEAD_t C2 R3.5; SG-A3.2 under-load): advance `refs/chaintape/l4e` to a new commit anchoring the just-flushed L4.E record. Best-effort; failures are logged but do not propagate. The L4.E commit chain is a parallel attestation to the JSONL backing store. Constitutional Justification: STAGE_A3_HEAD_T_C2_charter_2026-05-07.md FR-A3-HEAD-T-C2.2 + CR-A3-HEAD-T-C2.5.
fn advance_l4e_ref_for_record(
    repo_path: &std::path::Path,
    record: &RejectedSubmissionRecord,
    blob_bytes: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    use git2::Repository;
    let repo = Repository::open(repo_path)?;
    let blob_oid = repo.blob(blob_bytes)?;
    let mut tb = repo.treebuilder(None)?;
    tb.insert("rejection_record", blob_oid, 0o100644)?;
    let tree_oid = tb.write()?;
    let tree = repo.find_tree(tree_oid)?;
    // Deterministic time = submit_id (no wall clock; matches Git2LedgerWriter convention).
    let time = git2::Time::new(record.submit_id as i64, 0);
    let sig = git2::Signature::new("turingosv4 sequencer", "system@turingos", &time)?;
    // Walk parent chain by querying current refs/chaintape/l4e.
    let parents: Vec<git2::Commit<'_>> = match repo
        .find_reference(crate::bottom_white::ledger::transition_ledger::CHAINTAPE_L4E_REF)
    {
        Ok(r) => r
            .target()
            .and_then(|oid| repo.find_commit(oid).ok())
            .map(|c| vec![c])
            .unwrap_or_default(),
        Err(_) => Vec::new(),
    };
    let parent_refs: Vec<&git2::Commit<'_>> = parents.iter().collect();
    let message = format!("L4.E record submit_id={}", record.submit_id);
    let _new_oid = repo.commit(
        Some(crate::bottom_white::ledger::transition_ledger::CHAINTAPE_L4E_REF),
        &sig,
        &sig,
        &message,
        &tree,
        &parent_refs,
    )?;
    Ok(())
}

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
    /// **TB-3 charter § 4.5**: sponsor or solver lacked balance for the
    /// requested debit (`EscrowLockTx` `amount` > `balances_t[sponsor]`,
    /// or accepted `WorkTx` `stake` > `balances_t[agent]`). Distinct from
    /// `PolicyViolation` so P4 Information Loom can cluster "余额不足" as
    /// its own failure class. Stable repr-u8 = 5; tail-append, no
    /// renumbering of existing variants.
    InsufficientBalance = 5,
    /// **TB-18R R3** (`feedback_chaintape_externalized_proposal` + charter
    /// §0.A Q8 remediation): Lean tactic returned a failure verdict
    /// (type error / unification failure / undefined symbol / etc.) on the
    /// runtime evaluator hot path. Mirrors `LeanErrorClass::LeanFailed = 6`
    /// from R1 `attempt_telemetry.rs`. Stable repr-u8 = 6; tail-append, no
    /// renumbering of existing variants 0..5.
    LeanFailed = 6,
    /// **TB-18R R3**: evaluator could not parse a candidate from the LLM
    /// output (no recognizable lean code block, malformed wrapper). Mirrors
    /// `LeanErrorClass::ParseFailed = 7`. Stable repr-u8 = 7; tail-append.
    ParseFailed = 7,
    /// **TB-18R R3**: candidate uses `sorry` or another forbidden incomplete
    /// proof token. Mirrors `LeanErrorClass::SorryBlocked = 8`. Stable repr-u8
    /// = 8; tail-append.
    SorryBlocked = 8,
    /// **TB-18R R3**: LLM API itself errored (HTTP non-200, timeout,
    /// rate-limit, JSON parse fail on the LLM client side). Mirrors
    /// `LeanErrorClass::LlmError = 9`. Stable repr-u8 = 9; tail-append.
    LlmError = 9,
}

/// TB-18R R3 (preflight `handover/ai-direct/TB-18R_R3_STEP_B_admission.md` §3.3):
/// transcode the evaluator-side `LeanErrorClass` (from `attempt_telemetry.rs`,
/// shipped in R1) into the sequencer-side `RejectionClass`. The discriminator
/// values match (6/7/8/9 on both sides) so this is a no-op-byte transcode that
/// preserves repr-u8.
impl From<LeanErrorClass> for RejectionClass {
    fn from(lec: LeanErrorClass) -> Self {
        match lec {
            LeanErrorClass::LeanFailed => RejectionClass::LeanFailed,
            LeanErrorClass::ParseFailed => RejectionClass::ParseFailed,
            LeanErrorClass::SorryBlocked => RejectionClass::SorryBlocked,
            LeanErrorClass::LlmError => RejectionClass::LlmError,
        }
    }
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

/// TRACE_MATRIX P1:6 — error returned by `RejectionEvidenceWriter::verify_chain` + JSONL persistence ops (TB-6 Atom 1.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejectionEvidenceError {
    /// `prev_hash` chain or per-record hash diverged at the given index
    /// (covers row deletion, field tampering, and reordering).
    HashMismatch { at: usize },
    /// TB-6 Atom 1.2 — JSONL persistence I/O failure (open / read / write / fsync).
    /// `String`-wrapped to keep `Clone + PartialEq + Eq` derives.
    Io(String),
    /// TB-6 Atom 1.2 — JSONL deserialization failed during open_jsonl replay
    /// (line is corrupted / not valid JSON / shape mismatch).
    JsonlParse { line: usize, reason: String },
}

impl std::fmt::Display for RejectionEvidenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HashMismatch { at } => {
                write!(f, "rejection-evidence chain break at index {}", at)
            }
            Self::Io(e) => write!(f, "rejection-evidence I/O error: {e}"),
            Self::JsonlParse { line, reason } => write!(
                f,
                "rejection-evidence JSONL parse failure at line {line}: {reason}"
            ),
        }
    }
}

impl std::error::Error for RejectionEvidenceError {}

// ────────────────────────────────────────────────────────────────────────────
// RejectionEvidenceWriter — append + verify + project-to-public
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:6/P1:9 — rejection-evidence writer with optional persistent backend.
///
/// One `Vec<RejectedSubmissionRecord>`; `prev_hash` chained; `submit_id`
/// monotonicity is the caller's responsibility (the writer trusts the
/// `Sequencer::next_submit_id` issuer). No `logical_t` field — accepted
/// spine and rejection-evidence are intentionally disjoint per the L4 / L4.E
/// split (`DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`).
///
/// **TB-6 Atom 1.2 extension**: optional `Backend::JsonlAppend(path)` causes
/// every `append_rejected` call to also serialize the new record as a single
/// JSONL line + append + fsync to `path`. `open_jsonl(path)` constructs the
/// writer from an existing JSONL file, replaying records into the in-memory
/// chain. The "或等价结构" form of architect § 3.5 `refs/rejections/main` —
/// JSONL with embedded `prev_hash` + `hash` chain.
#[derive(Debug, Clone, Default)]
pub struct RejectionEvidenceWriter {
    records: Vec<RejectedSubmissionRecord>,
    /// TB-6 Atom 1.2 — backend for persistence; default = `InMemory` (no I/O).
    backend: Backend,
}

/// TB-6 Atom 1.2 — internal backend for `RejectionEvidenceWriter`.
/// NOT pub — backend selection is via `RejectionEvidenceWriter::open_jsonl`.
#[derive(Debug, Clone, Default)]
enum Backend {
    /// Pure in-memory `Vec<RejectedSubmissionRecord>`; existing TB-1 behavior.
    #[default]
    InMemory,
    /// TB-6 Atom 1.2 — JSONL append-only persistent file. `append_rejected`
    /// flushes a single JSONL line per record + fsync. `open_jsonl` reads
    /// existing file on construction + replays into `records`.
    JsonlAppend { path: std::path::PathBuf },
}

impl RejectionEvidenceWriter {
    /// TRACE_MATRIX P1:6 — empty writer with `InMemory` backend.
    pub fn new() -> Self {
        Self::default()
    }

    /// TRACE_MATRIX FC3-N1 + P1:6 (TB-6 Atom 1.2) — open or create a JSONL-backed writer.
    ///
    /// On open:
    /// - If `path` exists, read each line as a `RejectedSubmissionRecord`,
    ///   append to `records`, and `verify_chain()` over the loaded records to
    ///   reject tampering at load time.
    /// - If `path` does not exist, create the file (and parent dirs) empty.
    /// - Either way, set backend to `JsonlAppend(path)` so subsequent
    ///   `append_rejected` calls flush to that path.
    ///
    /// Architect § 3.5 "或等价结构": JSONL chain-hash equivalent to git
    /// `refs/rejections/main`. Each line embeds `prev_hash` + `hash` so
    /// tampering with any line breaks the chain at that line.
    pub fn open_jsonl(path: std::path::PathBuf) -> Result<Self, RejectionEvidenceError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                RejectionEvidenceError::Io(format!("create parent dir for {path:?}: {e}"))
            })?;
        }
        let mut records: Vec<RejectedSubmissionRecord> = Vec::new();
        if path.exists() {
            let contents = std::fs::read_to_string(&path)
                .map_err(|e| RejectionEvidenceError::Io(format!("read {path:?}: {e}")))?;
            for (idx, line) in contents.lines().enumerate() {
                if line.trim().is_empty() {
                    continue;
                }
                let shadow: JsonlRecord =
                    serde_json::from_str(line).map_err(|e| RejectionEvidenceError::JsonlParse {
                        line: idx,
                        reason: e.to_string(),
                    })?;
                records.push(shadow.into());
            }
        } else {
            // Create empty file with parent dirs already ensured above.
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
                .map_err(|e| RejectionEvidenceError::Io(format!("create {path:?}: {e}")))?;
        }
        let writer = Self {
            records,
            backend: Backend::JsonlAppend { path },
        };
        // Validate chain integrity on load — tampering with any record
        // breaks here.
        writer.verify_chain()?;
        Ok(writer)
    }

    /// TRACE_MATRIX FC3-N1 (TB-6 Atom 1.2) — convenience: is this writer JSONL-backed?
    pub fn is_jsonl_backed(&self) -> bool {
        matches!(self.backend, Backend::JsonlAppend { .. })
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
    ///
    /// **TB-6 Atom 1.2**: when the writer is `JsonlAppend`-backed, the new
    /// record is also serialized as one JSONL line and appended (with fsync)
    /// to the configured path. JSONL flush failures are silently logged and
    /// the in-memory record is still pushed — `Sequencer::apply_one` (the
    /// caller) does not have an error return path for this writer; tests
    /// validate persistence via `RejectionEvidenceWriter::open_jsonl`'s
    /// reload + `verify_chain` round-trip rather than checking append return.
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
        // TB-6 Atom 1.2: flush to JSONL backend if configured. We do this
        // BEFORE pushing to records so a write failure does not leave the
        // in-memory chain ahead of the on-disk chain (best-effort
        // consistency).
        if let Backend::JsonlAppend { path } = &self.backend {
            if let Err(e) = flush_jsonl_record(path, &record) {
                log::error!(
                    "rejection-evidence JSONL flush failed (record dropped from in-memory chain to preserve persistence consistency): {e}"
                );
                // Caller's chain-hash invariant is preserved because we did
                // NOT push the record. The next append_rejected will compute
                // prev_hash from the unchanged last_hash. The dropped record
                // is logged but lost — this is the failure mode for an I/O
                // error mid-run; recovery is to investigate the JSONL path.
                return prev_hash;
            }
        }
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

/// TRACE_MATRIX FC1-N34 (session #34 L4.E body integrity landing;
/// `assert_51_l4e_git_attestation_matches_jsonl`): parse a single
/// JSONL-encoded rejection record from raw bytes (no trailing newline
/// required — the `\n` is trimmed if present) and verify the embedded
/// `hash` field equals the SHA-256 over its 9 content fields. Catches
/// any field-level body tampering inside the bytes.
///
/// Used by the audit-side L4.E git-attestation walker: each commit on
/// `refs/chaintape/l4e` carries a `rejection_record` blob whose bytes are
/// the canonical JSONL line for that record. The walker passes those
/// bytes here; a `Err(HashMismatch)` proves the git-side blob diverged
/// from the canonical record (any field flip / re-serialization with
/// non-matching `hash` field).
pub fn parse_and_verify_jsonl_record_bytes(
    bytes: &[u8],
) -> Result<RejectedSubmissionRecord, RejectionEvidenceError> {
    let s = std::str::from_utf8(bytes).map_err(|e| RejectionEvidenceError::JsonlParse {
        line: 0,
        reason: format!("utf8: {e}"),
    })?;
    let trimmed = s.trim_end_matches(|c: char| c == '\n' || c == '\r');
    let shadow: JsonlRecord =
        serde_json::from_str(trimmed).map_err(|e| RejectionEvidenceError::JsonlParse {
            line: 0,
            reason: e.to_string(),
        })?;
    let record: RejectedSubmissionRecord = shadow.into();
    let recomputed = RejectedSubmissionRecord::compute_hash(
        record.submit_id,
        &record.parent_state_root,
        &record.agent_id,
        record.tx_kind,
        &record.tx_payload_cid,
        record.rejection_class,
        &record.raw_diagnostic_cid,
        &record.public_summary,
        &record.prev_hash,
    );
    if recomputed != record.hash {
        return Err(RejectionEvidenceError::HashMismatch { at: 0 });
    }
    Ok(record)
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
        assert!(matches!(
            r,
            Err(RejectionEvidenceError::HashMismatch { at: 0 })
        ));
    }
}
