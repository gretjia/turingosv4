//! CAS store backed by git2-rs blob layer.
//!
//! Each runtime_repo (per spec § 5.2.2 cell isolation) has its own CasStore.
//! Objects are content-addressed by `Cid` (sha256 of content); git's sha-1
//! OID is recorded but not canonical.
//!
//! **CO1.4-extra (this atom)** adds index persistence: the `Cid → metadata`
//! map is durably persisted to a sidecar JSONL file at
//! `<repo_path>/.turingos_cas_index.jsonl`. On `CasStore::open()` the sidecar
//! is replayed into an in-memory BTreeMap; on `CasStore::put()` (new entries
//! only) one JSONL line is appended + flushed. This closes the Art 0.2
//! tape-canonicality cold-replay gate that CO1.7 spec § 0 + CO1.1.4-pre1
//! v1.1 § 0.1 declared a hard prerequisite for `replay_full_transition`
//! (CO1.7-impl A4).
//!
//! **Design choice (sidecar JSONL)**: chosen over (b) git-tag manifest /
//! (c) bincode index + WAL because (a) is the simplest deterministic
//! append-only artifact, replayable from scratch, easy to audit by reading.
//! Per "压缩即智能" — pick simplest correct shape; upgrade later if profiling
//! shows O(N)-on-restart cost is real.
//!
//! /// TRACE_MATRIX WP-arch-§5.L3 + spec-§5.2.2 (cell isolation): CAS store
//! /// TRACE_MATRIX CO1.7 spec § 0 + CO1.1.4-pre1 § 0.1 cross-atom ordering:
//! /// CAS index persistence — required by `replay_full_transition` cold-restart.

use git2::Repository;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::{remove_file, rename, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use super::schema::{CasObjectMetadata, Cid, ObjectType};

const CAS_INDEX_FILENAME: &str = ".turingos_cas_index.jsonl";
const CAS_CHAIN_LOCK_FILENAME: &str = ".turingos_cas_chain.lock";

#[derive(Debug)]
pub enum CasError {
    /// git2-rs underlying error.
    Git2(git2::Error),
    /// Cid not found in this CasStore's metadata index.
    CidNotFound(Cid),
    /// Content stored at git OID but Cid metadata absent (corrupted index).
    MetadataMissing(Cid),
    /// Content's sha256 doesn't match the asserted Cid (corruption).
    CidMismatch { expected: Cid, computed: Cid },
    /// I/O error reading or writing the CO1.4-extra sidecar index file.
    IoError(io::Error),
    /// JSON-deserialization error on a sidecar index line. Includes 1-based
    /// line number for diagnostics.
    IndexParse { line: usize, error: String },
    /// TB-16.x.1: backend corruption detected by a defense-in-depth check
    /// (size bound, libgit2 zlib timeout, etc). Carries a human-readable
    /// detail string for the audit trail.
    BackendCorruption(String),
}

impl std::fmt::Display for CasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Git2(e) => write!(f, "git2 backend error: {e}"),
            Self::CidNotFound(c) => write!(f, "{c} not found in CAS index"),
            Self::MetadataMissing(c) => write!(f, "{c} metadata missing (index corrupted)"),
            Self::CidMismatch { expected, computed } => write!(
                f,
                "CAS content corruption: expected {expected}, computed {computed}"
            ),
            Self::IoError(e) => write!(f, "cas index I/O error: {e}"),
            Self::IndexParse { line, error } => {
                write!(f, "cas index parse error at line {line}: {error}")
            }
            Self::BackendCorruption(detail) => write!(f, "backend corruption: {detail}"),
        }
    }
}

impl std::error::Error for CasError {}

impl From<git2::Error> for CasError {
    fn from(e: git2::Error) -> Self {
        Self::Git2(e)
    }
}

impl From<io::Error> for CasError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

fn cas_index_path(repo_path: &Path) -> PathBuf {
    repo_path.join(CAS_INDEX_FILENAME)
}

fn cas_chain_lock_path(repo_path: &Path) -> PathBuf {
    repo_path.join(CAS_CHAIN_LOCK_FILENAME)
}

struct CasChainLock {
    path: PathBuf,
}

impl Drop for CasChainLock {
    fn drop(&mut self) {
        let _ = remove_file(&self.path);
    }
}

fn acquire_cas_chain_lock(repo_path: &Path) -> Result<CasChainLock, CasError> {
    let path = cas_chain_lock_path(repo_path);
    let timeout_secs = std::env::var("TURINGOS_CAS_CHAIN_LOCK_TIMEOUT_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(120);
    let deadline = Instant::now() + Duration::from_secs(timeout_secs);
    loop {
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(mut file) => {
                let pid_line = format!("pid={}\n", std::process::id());
                file.write_all(pid_line.as_bytes())?;
                file.sync_data()?;
                return Ok(CasChainLock { path });
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists && Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                return Err(CasError::BackendCorruption(format!(
                    "CAS chain lock timed out after {timeout_secs}s at {}",
                    path.display(),
                )));
            }
            Err(e) => return Err(CasError::IoError(e)),
        }
    }
}

/// CO1.4-extra: read the sidecar JSONL into an in-memory index.
/// Strict mode — any malformed line aborts the load (per Art 0.2: a
/// corrupted index means the tape is non-canonical; abort + diagnose
/// is more honest than skip-and-warn).
fn load_index_from_sidecar(repo_path: &Path) -> Result<BTreeMap<Cid, CasObjectMetadata>, CasError> {
    let path = cas_index_path(repo_path);
    let mut index = BTreeMap::new();
    if !path.exists() {
        return Ok(index);
    }
    let content = std::fs::read_to_string(&path)?;
    for (i, line) in content.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let meta: CasObjectMetadata =
            serde_json::from_str(line).map_err(|e| CasError::IndexParse {
                line: i + 1,
                error: e.to_string(),
            })?;
        index.insert(meta.cid, meta);
    }
    Ok(index)
}

fn load_index_for_repo_unlocked(
    repo_path: &Path,
) -> Result<BTreeMap<Cid, CasObjectMetadata>, CasError> {
    let sidecar_path = cas_index_path(repo_path);
    let sidecar_exists = sidecar_path.exists();
    let sidecar = load_index_from_sidecar(repo_path)?;
    let chain = super::git_chain::reconstruct_index_from_chain_with_cache(
        repo_path,
        sidecar_exists.then_some(&sidecar),
    )?;

    match chain {
        Some(chain_index) => {
            if sidecar_exists && sidecar != chain_index {
                return Err(CasError::BackendCorruption(format!(
                    "CAS sidecar cache mismatch with CAS commit-chain at {}",
                    sidecar_path.display()
                )));
            }
            Ok(chain_index)
        }
        None => Ok(sidecar),
    }
}

fn load_index_for_repo(repo_path: &Path) -> Result<BTreeMap<Cid, CasObjectMetadata>, CasError> {
    let _lock = acquire_cas_chain_lock(repo_path)?;
    load_index_for_repo_unlocked(repo_path)
}

/// CO1.4-extra: append a single JSONL line for a newly-created CAS object.
/// Followed by `sync_data` for durability.
///
/// **TB-7.6 fix (2026-05-01)**: write the JSON line + trailing newline
/// in ONE `write_all` call instead of two. POSIX `O_APPEND` guarantees
/// atomicity for individual writes ≤ PIPE_BUF (4096 bytes typical;
/// CasObjectMetadata serializes to ~300-400 bytes). Pre-fix used two
/// separate `write_all` calls (`serialized` then `b"\n"`), which could
/// interleave with another concurrent writer's append, producing
/// corrupted lines like `{...}{...}` (no separator). Discovered during
/// TB-7 real-LLM smoke runs 2 + 5 (mathd_algebra_171 + mathd_numbertheory_5)
/// where evaluator opens multiple CasStore handles concurrently for
/// per-tx writes (Atom 1.5 ProposalTelemetry CAS + Atom 5
/// agent_audit_trail synthetic seed + Atoms 2/3 evaluator hot-path
/// telemetry writes). See
/// `handover/evidence/tb_7_real_smoke_5_problems_2026-05-01/README.md` §3.
fn append_to_sidecar(repo_path: &Path, meta: &CasObjectMetadata) -> Result<(), CasError> {
    let path = cas_index_path(repo_path);
    let serialized = serde_json::to_string(meta).map_err(|e| CasError::IndexParse {
        line: 0,
        error: format!("serialize: {e}"),
    })?;
    let mut f = OpenOptions::new().create(true).append(true).open(&path)?;
    // Atomic single-write append: serialize + newline in one buffer.
    let mut line = serialized.into_bytes();
    line.push(b'\n');
    f.write_all(&line)?;
    f.sync_data()?;
    Ok(())
}

fn rewrite_sidecar_cache(
    repo_path: &Path,
    index: &BTreeMap<Cid, CasObjectMetadata>,
) -> Result<(), CasError> {
    let path = cas_index_path(repo_path);
    let tmp_path = repo_path.join(format!("{CAS_INDEX_FILENAME}.tmp.{}", std::process::id()));
    let _ = remove_file(&tmp_path);
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&tmp_path)?;
    for meta in index.values() {
        let serialized = serde_json::to_string(meta).map_err(|e| CasError::IndexParse {
            line: 0,
            error: format!("serialize: {e}"),
        })?;
        f.write_all(serialized.as_bytes())?;
        f.write_all(b"\n")?;
    }
    f.sync_data()?;
    drop(f);
    rename(&tmp_path, &path)?;
    Ok(())
}

/// Content-addressable store backed by git's blob object database.
#[derive(Debug)]
pub struct CasStore {
    repo_path: PathBuf,
    /// Cid → metadata index. BTreeMap per spec § 2 I-BTREE.
    index: BTreeMap<Cid, CasObjectMetadata>,
}

impl CasStore {
    /// Open or initialize a CAS store at the given runtime_repo path.
    /// Creates the git repo if it doesn't exist. **CO1.4-extra**: replays
    /// the sidecar `.turingos_cas_index.jsonl` (if any) into the in-memory
    /// index, restoring all metadata that was durably appended in prior
    /// sessions.
    pub fn open(repo_path: &Path) -> Result<Self, CasError> {
        let repo_path = repo_path.to_path_buf();
        let _repo = match Repository::open(&repo_path) {
            Ok(r) => r,
            Err(_) => Repository::init(&repo_path)?,
        };
        let index = load_index_for_repo(&repo_path)?;
        Ok(Self { repo_path, index })
    }

    fn open_repo(&self) -> Result<Repository, CasError> {
        Repository::open(&self.repo_path).map_err(CasError::from)
    }

    /// TB-18R R3.fix (preflight `handover/ai-direct/TB-18R_R3FIX_STEP_B_cas_reload.md`):
    /// re-read the on-disk sidecar `.turingos_cas_index.jsonl` and merge new
    /// entries into `self.index`. Existing entries are preserved (idempotent on
    /// already-loaded CIDs; see `load_index_from_sidecar` strict-mode contract).
    ///
    /// Use case: a long-lived `CasStore` handle (e.g. `Sequencer.cas`) was
    /// opened at process start; another short-lived `CasStore` handle opened
    /// later on the same disk path wrote new objects (e.g. evaluator's per-path
    /// AttemptTelemetry write); the long-lived handle's in-memory index does
    /// NOT see those entries until this method is called. The L0 smoke
    /// 2026-05-06 surfaced this split-brain: sequencer's R3 admission helper
    /// `refine_rejection_class_via_attempt_telemetry` was reading via the
    /// stale index, getting `CidNotFound`, and falling back to PredicateFailed
    /// instead of refining to LeanFailed/SorryBlocked/ParseFailed/LlmError.
    ///
    /// Idempotent: replays the sidecar in full; entries already present in
    /// `self.index` are overwritten with the same metadata; new entries are
    /// inserted. No on-disk side effect.
    ///
    /// **Strict mode**: any malformed sidecar line returns
    /// `CasError::IndexParse` and `self.index` is left UNCHANGED (the
    /// freshly-loaded BTreeMap is computed first, then swapped in only on
    /// success).
    ///
    /// TRACE_MATRIX FC1-N41 (TB-18R R3.fix; consumed by sequencer.rs
    /// `refine_rejection_class_via_attempt_telemetry` retry path).
    pub fn reload_index_from_sidecar(&mut self) -> Result<(), CasError> {
        let fresh = load_index_for_repo(&self.repo_path)?;
        // Replace, do not merge. The loader prefers the CAS commit-chain when
        // present and validates any sidecar cache against it, so stale hot
        // entries must not survive a reload.
        self.index = fresh;
        Ok(())
    }

    /// TB-18R R4 (preflight `handover/ai-direct/TB-18R_R4_STEP_B_invariant.md`):
    /// count CAS objects whose metadata `object_type` matches `ty`.
    ///
    /// Used by `chain_derived_run_facts::compute_run_facts_from_chain_with_invariant`
    /// to populate `attempt_aborted_count` (FR-18R.4 v2 — count of
    /// `TerminalAbortRecord` CAS objects per run). Pure function over the
    /// in-memory index; no disk I/O. Caller is responsible for ensuring the
    /// index reflects the durable sidecar (open the store post-drain or call
    /// `reload_index_from_sidecar` first).
    ///
    /// TRACE_MATRIX FC1-N43 (TB-18R R4 invariant ship-gate equation
    /// numerator).
    pub fn count_by_object_type(&self, ty: ObjectType) -> u64 {
        self.index.values().filter(|m| m.object_type == ty).count() as u64
    }

    /// TB-18R R5 (preflight `handover/ai-direct/TB-18R_R5_preflight_audit_extension.md`):
    /// list CIDs of all CAS objects whose metadata `object_type` matches `ty`.
    ///
    /// Used by `audit_assertions::assert_44_attempt_telemetry_retrievable_from_cas`
    /// + `assert_45_lean_result_retrievable_from_cas` to walk all
    /// AttemptTelemetry / LeanResult CAS objects per FR-18R.7. Pure
    /// function over the in-memory index; no disk I/O.
    ///
    /// TRACE_MATRIX FC2-N34 (TB-18R R5 audit-tape sampler reaches
    /// mathematical content).
    pub fn list_cids_by_object_type(&self, ty: ObjectType) -> Vec<Cid> {
        let mut matches: Vec<&CasObjectMetadata> = self
            .index
            .values()
            .filter(|m| m.object_type == ty)
            .collect();
        matches.sort_by_key(|m| (m.created_at_logical_t, m.cid));
        matches.into_iter().map(|m| m.cid).collect()
    }

    /// TRACE_MATRIX FC1-N34 + FC1-N35: bytes-content immutability witness
    /// (Art. 0.3 immutability fence). Sibling helper to
    /// `list_cids_by_object_type` (FC2-N34) and `count_by_object_type`.
    ///
    /// **TB-C0 strict-audit FC1-INV6 fix (2026-05-07; Finding D)**:
    /// list every Cid in the index regardless of object_type. Used by
    /// `audit_assertions::assert_50_cas_bytes_match_cids` to walk all CAS
    /// objects and re-hash them against their stored CIDs (closes the
    /// `flip_cas_byte` detection hole revealed on P05's tape — see
    /// `STRICT_AUDIT_TBC0_TAPE_2026-05-07.md` §1 Finding D). Pure-additive
    /// helper over the in-memory index; no disk I/O; no change to put/get/
    /// open semantics. Class 3 surface (cas/store.rs is NOT in CLAUDE.md
    /// STEP_B restricted list — only cas/schema.rs is).
    pub fn list_all_cids(&self) -> Vec<Cid> {
        self.index.values().map(|m| m.cid).collect()
    }

    /// TRACE_MATRIX FC1-N41 + FC2-N34: tape-derived CAS lookup by schema id
    /// without exposing raw evidence bytes to ordinary agent read views.
    pub fn list_cids_by_schema_id(&self, schema_id: &str) -> Vec<Cid> {
        let mut matches: Vec<&CasObjectMetadata> = self
            .index
            .values()
            .filter(|m| m.schema_id.as_deref() == Some(schema_id))
            .collect();
        matches.sort_by_key(|m| (m.created_at_logical_t, m.cid));
        matches.into_iter().map(|m| m.cid).collect()
    }

    /// TRACE_MATRIX FC1-N41 + FC2-N34: tape-derived CAS lookup by creator/run
    /// authority for audit samplers and replay diagnostics.
    pub fn list_cids_by_creator(&self, creator: &str) -> Vec<Cid> {
        let mut matches: Vec<&CasObjectMetadata> = self
            .index
            .values()
            .filter(|m| m.creator == creator)
            .collect();
        matches.sort_by_key(|m| (m.created_at_logical_t, m.cid));
        matches.into_iter().map(|m| m.cid).collect()
    }

    /// TRACE_MATRIX FC1-N41 + FC2-N34: tape-derived CAS lookup by logical time
    /// for reconstructable audit and replay views.
    pub fn list_cids_by_logical_t(&self, logical_t: u64) -> Vec<Cid> {
        let mut matches: Vec<&CasObjectMetadata> = self
            .index
            .values()
            .filter(|m| m.created_at_logical_t == logical_t)
            .collect();
        matches.sort_by_key(|m| m.cid);
        matches.into_iter().map(|m| m.cid).collect()
    }

    /// Store content; returns its Cid. Idempotent — same content → same Cid.
    pub fn put(
        &mut self,
        content: &[u8],
        object_type: ObjectType,
        creator: &str,
        created_at_logical_t: u64,
        schema_id: Option<String>,
    ) -> Result<Cid, CasError> {
        let cid = Cid::from_content(content);
        let _lock = acquire_cas_chain_lock(&self.repo_path)?;
        let sidecar_was_present = cas_index_path(&self.repo_path).exists();
        let had_cas_chain = super::git_chain::has_cas_commit_chain(&self.repo_path)?;
        self.index = load_index_for_repo_unlocked(&self.repo_path)?;

        // If already in index, idempotent: just return Cid (content addressing
        // guarantees same content → same Cid → already present)
        if self.index.contains_key(&cid) {
            return Ok(cid);
        }

        let repo = self.open_repo()?;
        let git_oid = repo.blob(content)?;
        let metadata = CasObjectMetadata {
            cid,
            backend_oid_hex: git_oid.to_string(),
            object_type,
            creator: creator.to_string(),
            created_at_logical_t,
            schema_id,
            size_bytes: content.len() as u64,
        };

        let previous_cas_root = if self.index.is_empty() {
            None
        } else {
            Some(super::git_chain::merkle_root_for_index(&self.index))
        };
        let legacy_prefix_metadata: Vec<CasObjectMetadata> = if had_cas_chain {
            Vec::new()
        } else {
            self.index.values().cloned().collect()
        };
        let mut prospective = self.index.clone();
        prospective.insert(cid, metadata.clone());
        let resulting_cas_root = super::git_chain::merkle_root_for_index(&prospective);

        // The CAS commit-chain is now canonical. If the commit/ref update
        // fails, put fails before the sidecar cache or hot index can accept
        // the object.
        super::git_chain::append_cas_commit(
            &self.repo_path,
            &metadata,
            previous_cas_root,
            resulting_cas_root,
            legacy_prefix_metadata,
        )?;

        // Sidecar is a cache. Keep it warm on the hot path, but if cache write
        // fails after the canonical chain advanced, remove the stale cache so
        // the next open rebuilds from ChainTape/CAS instead of seeing mismatch.
        let cache_write = if sidecar_was_present {
            append_to_sidecar(&self.repo_path, &metadata)
        } else {
            rewrite_sidecar_cache(&self.repo_path, &prospective)
        };
        if let Err(e) = cache_write {
            let path = cas_index_path(&self.repo_path);
            let _ = remove_file(&path);
            if path.exists() {
                return Err(e);
            }
        }
        self.index = prospective;

        Ok(cid)
    }

    /// Retrieve content by Cid. Verifies content sha256 matches Cid (corruption check).
    ///
    /// **TB-16.x.1 defense-in-depth (Class 2)**: wraps the libgit2 read in a
    /// worker thread + `recv_timeout` so adversarial CAS bytes (e.g. a loose
    /// object whose back half is zeroed by a tamper harness) cannot hang the
    /// audit pipeline. Empirically (2026-05-04), libgit2's zlib decompression
    /// of certain corrupted MarkovEvidenceCapsule loose objects pegs a single
    /// CPU core indefinitely; this wrapper converts that hang into a bounded
    /// `BackendCorruption` error so audit termination is guaranteed.
    ///
    /// Knobs:
    /// * `TURINGOS_CAS_GET_TIMEOUT_SECS` — override the default 10s timeout.
    pub fn get(&self, cid: &Cid) -> Result<Vec<u8>, CasError> {
        let metadata = self.index.get(cid).ok_or(CasError::CidNotFound(*cid))?;
        let repo_path = self.repo_path.clone();
        let oid_hex = metadata.backend_oid_hex.clone();
        let expected_size = metadata.size_bytes;
        let cid_copy = *cid;

        let timeout_secs: u64 = std::env::var("TURINGOS_CAS_GET_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10);

        let (tx, rx) = std::sync::mpsc::channel::<Result<Vec<u8>, CasError>>();
        std::thread::Builder::new()
            .name("cas-get".to_string())
            .spawn(move || {
                let result: Result<Vec<u8>, CasError> = (|| {
                    let repo = Repository::open(&repo_path).map_err(CasError::from)?;
                    let git_oid = git2::Oid::from_str(&oid_hex).map_err(CasError::Git2)?;
                    let blob = repo.find_blob(git_oid)?;
                    let content = blob.content().to_vec();

                    // Defense-in-depth size bound: a blob whose decompressed
                    // length exceeds the recorded `size_bytes` (plus tiny
                    // header slack) signals adversarial expansion. Reject
                    // before sha256 to bound any downstream alloc.
                    if content.len() as u64 > expected_size.saturating_add(256) {
                        return Err(CasError::BackendCorruption(format!(
                            "blob content {} bytes exceeds expected size {} for {}",
                            content.len(),
                            expected_size,
                            cid_copy
                        )));
                    }

                    // Verify content sha256 matches Cid (corruption check).
                    let mut h = Sha256::new();
                    h.update(&content);
                    let computed = Cid(h.finalize().into());
                    if computed != cid_copy {
                        return Err(CasError::CidMismatch {
                            expected: cid_copy,
                            computed,
                        });
                    }
                    Ok(content)
                })();
                let _ = tx.send(result);
            })
            .map_err(|e| {
                CasError::IoError(io::Error::new(
                    io::ErrorKind::Other,
                    format!("cas-get worker spawn: {e}"),
                ))
            })?;

        match rx.recv_timeout(std::time::Duration::from_secs(timeout_secs)) {
            Ok(r) => r,
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                Err(CasError::BackendCorruption(format!(
                    "cas.get timed out after {timeout_secs}s for {cid_copy} \
                     — adversarial bytes hang libgit2 zlib (TB-16.x.1 \
                     defense-in-depth; set TURINGOS_CAS_GET_TIMEOUT_SECS to \
                     override)"
                )))
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                Err(CasError::BackendCorruption(
                    "cas.get worker thread disconnected unexpectedly".into(),
                ))
            }
        }
    }

    /// Get metadata only (no content fetch).
    pub fn metadata(&self, cid: &Cid) -> Option<&CasObjectMetadata> {
        self.index.get(cid)
    }

    /// TRACE_MATRIX FC3-N4: list all objects currently present in the CAS index.
    pub fn list(&self) -> Result<Vec<CasObjectMetadata>, CasError> {
        Ok(self.index.values().cloned().collect())
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// Merkle root over all CAS object metadata; deterministic per BTreeMap order.
    pub fn merkle_root(&self) -> [u8; 32] {
        super::git_chain::merkle_root_for_index(&self.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn fresh_store() -> (TempDir, CasStore) {
        let tmp = TempDir::new().unwrap();
        let store = CasStore::open(tmp.path()).unwrap();
        (tmp, store)
    }

    #[test]
    fn put_get_round_trip_small() {
        let (_tmp, mut s) = fresh_store();
        let cid = s
            .put(
                b"hello world",
                ObjectType::ProposalPayload,
                "alice",
                100,
                None,
            )
            .unwrap();
        let content = s.get(&cid).unwrap();
        assert_eq!(content, b"hello world");
    }

    #[test]
    fn put_get_round_trip_large() {
        let (_tmp, mut s) = fresh_store();
        let big = vec![0xab; 65536];
        let cid = s
            .put(
                &big,
                ObjectType::PredicateBytecode,
                "system",
                0,
                Some("wasm".into()),
            )
            .unwrap();
        let content = s.get(&cid).unwrap();
        assert_eq!(content, big);
    }

    #[test]
    fn put_idempotent_same_content() {
        let (_tmp, mut s) = fresh_store();
        let cid_a = s.put(b"x", ObjectType::Generic, "alice", 1, None).unwrap();
        let cid_b = s.put(b"x", ObjectType::Generic, "bob", 2, None).unwrap();
        assert_eq!(cid_a, cid_b, "same content → same Cid");
        // Index size = 1 (idempotent)
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn cid_is_content_address() {
        let (_tmp, mut s) = fresh_store();
        let cid = s
            .put(b"specific content", ObjectType::Generic, "system", 0, None)
            .unwrap();
        // Cid is sha256 of content; verifiable independently
        let expected = Cid::from_content(b"specific content");
        assert_eq!(cid, expected);
    }

    #[test]
    fn get_nonexistent_returns_error() {
        let (_tmp, s) = fresh_store();
        let bogus = Cid([0u8; 32]);
        match s.get(&bogus) {
            Err(CasError::CidNotFound(c)) => assert_eq!(c, bogus),
            other => panic!("expected CidNotFound, got {other:?}"),
        }
    }

    #[test]
    fn metadata_recorded() {
        let (_tmp, mut s) = fresh_store();
        let cid = s
            .put(
                b"meta test",
                ObjectType::CounterexamplePayload,
                "carol",
                250,
                Some("v1".into()),
            )
            .unwrap();
        let meta = s.metadata(&cid).unwrap();
        assert_eq!(meta.cid, cid);
        assert_eq!(meta.object_type, ObjectType::CounterexamplePayload);
        assert_eq!(meta.creator, "carol");
        assert_eq!(meta.created_at_logical_t, 250);
        assert_eq!(meta.schema_id.as_deref(), Some("v1"));
        assert_eq!(meta.size_bytes, 9);
    }

    #[test]
    fn merkle_root_deterministic_two_runs() {
        let (_tmp1, mut s1) = fresh_store();
        let (_tmp2, mut s2) = fresh_store();
        for content in [b"a".as_slice(), b"b".as_slice(), b"c".as_slice()] {
            s1.put(content, ObjectType::Generic, "system", 0, None)
                .unwrap();
        }
        // Different insertion order
        for content in [b"c".as_slice(), b"b".as_slice(), b"a".as_slice()] {
            s2.put(content, ObjectType::Generic, "system", 0, None)
                .unwrap();
        }
        assert_eq!(
            s1.merkle_root(),
            s2.merkle_root(),
            "BTreeMap-ordered: insertion order independent (I-DET)"
        );
    }

    #[test]
    fn empty_store_root() {
        let (_tmp, s) = fresh_store();
        let r = s.merkle_root();
        let expected: [u8; 32] = Sha256::new().finalize().into();
        assert_eq!(r, expected, "empty store root = sha256(empty)");
    }

    #[test]
    fn cell_isolation_disjoint_cas() {
        // Per spec § 5.2.2 cross-cell isolation: separate runtime_repo paths
        // → completely disjoint CasStore instances.
        let (_tmp_a, mut store_a) = fresh_store();
        let (_tmp_b, mut store_b) = fresh_store();

        let cid_a = store_a
            .put(b"only in a", ObjectType::Generic, "agent_a", 100, None)
            .unwrap();
        let cid_b = store_b
            .put(b"only in b", ObjectType::Generic, "agent_b", 100, None)
            .unwrap();

        // Each store has its own object only
        assert!(store_a.get(&cid_a).is_ok(), "store_a has cid_a");
        assert!(
            store_a.get(&cid_b).is_err(),
            "store_a lacks cid_b (isolated)"
        );
        assert!(store_b.get(&cid_b).is_ok(), "store_b has cid_b");
        assert!(
            store_b.get(&cid_a).is_err(),
            "store_b lacks cid_a (isolated)"
        );
    }

    #[test]
    fn put_many_then_iterate_count() {
        let (_tmp, mut s) = fresh_store();
        for i in 0..50 {
            s.put(
                format!("content {i}").as_bytes(),
                ObjectType::ProposalPayload,
                "system",
                i as u64,
                None,
            )
            .unwrap();
        }
        assert_eq!(s.len(), 50);
        assert!(!s.is_empty());
    }

    /// TB-7.6 regression — CAS index concurrent-write race
    ///
    /// **Bug discovered during TB-7 real-LLM smoke runs 2 + 5**
    /// (commit a981317): when the production binary opens multiple
    /// `CasStore` handles against the same on-disk repo path and writes
    /// concurrently, the pre-TB-7.6 `append_to_sidecar` performed two
    /// separate `write_all` calls (serialized JSON, then `b"\n"`) which
    /// could interleave with another writer's append, producing a
    /// corrupted index line like `{"cid":...}{"cid":...}` (no separator).
    /// The fix combines the JSON + newline into ONE `write_all` call,
    /// relying on POSIX `O_APPEND` atomicity for writes ≤ PIPE_BUF.
    ///
    /// This test fires N concurrent threads, each performing 20 puts
    /// against a SHARED repo path via independent `CasStore` instances,
    /// then reopens the store and verifies the on-disk index parses
    /// cleanly (no trailing-characters error).
    #[test]
    fn concurrent_writers_share_index_without_race() {
        use std::sync::Arc;
        use std::thread;
        let tmp = TempDir::new().expect("tempdir");
        let repo_path: Arc<PathBuf> = Arc::new(tmp.path().to_path_buf());
        // Initialize once to set up the git repo.
        {
            let _s = CasStore::open(&repo_path).expect("init");
        }

        let n_threads = 4;
        let writes_per_thread = 20;
        let mut handles = Vec::new();
        for t in 0..n_threads {
            let path = Arc::clone(&repo_path);
            handles.push(thread::spawn(move || {
                let mut store = CasStore::open(&path).expect("thread open");
                for i in 0..writes_per_thread {
                    let content = format!("thread-{t}-write-{i}");
                    store
                        .put(
                            content.as_bytes(),
                            ObjectType::Generic,
                            &format!("agent-{t}"),
                            (t * writes_per_thread + i) as u64,
                            Some(format!("schema-{t}")),
                        )
                        .expect("put");
                }
            }));
        }
        for h in handles {
            h.join().expect("thread join");
        }

        // Reopen — this internally calls `load_index_from_sidecar` which
        // is strict (any malformed line aborts). Pre-TB-7.6 this would
        // intermittently fail with `IndexParse { line: 1, error: "trailing
        // characters at line 1 column N" }`.
        let final_store = CasStore::open(&repo_path).expect(
            "reopen after concurrent writes must succeed (TB-7.6 fix verifies \
             O_APPEND atomicity prevents interleaved writes)",
        );
        assert!(
            final_store.len() >= (n_threads * writes_per_thread) as usize,
            "expected at least {} entries, got {}",
            n_threads * writes_per_thread,
            final_store.len()
        );
    }

    // ── CO1.4-extra: sidecar JSONL persistence tests ─────────────────────────

    /// Cold-restart: reopen recovers all metadata; get() works post-reopen
    /// (closes the Art 0.2 cold-replay gate that CO1.7-impl A4 needs).
    #[test]
    fn reopen_recovers_index_and_get_works() {
        let tmp = TempDir::new().expect("tempdir");
        let cid_a;
        let cid_b;
        {
            let mut s = CasStore::open(tmp.path()).expect("open");
            cid_a = s
                .put(b"alpha", ObjectType::ProposalPayload, "alice", 1, None)
                .unwrap();
            cid_b = s
                .put(
                    b"beta",
                    ObjectType::CounterexamplePayload,
                    "bob",
                    2,
                    Some("s.v1".into()),
                )
                .unwrap();
        }
        // Reopen: in-memory store is fresh; sidecar replay is the ONLY way
        // metadata survives.
        let s2 = CasStore::open(tmp.path()).expect("reopen");
        assert_eq!(s2.len(), 2);
        assert_eq!(s2.get(&cid_a).expect("get a"), b"alpha");
        assert_eq!(s2.get(&cid_b).expect("get b"), b"beta");

        let meta_b = s2.metadata(&cid_b).expect("metadata b");
        assert_eq!(meta_b.creator, "bob");
        assert_eq!(meta_b.created_at_logical_t, 2);
        assert_eq!(meta_b.schema_id.as_deref(), Some("s.v1"));
        assert_eq!(meta_b.object_type, ObjectType::CounterexamplePayload);
    }

    /// Idempotent put: same content twice → same Cid → only ONE sidecar line.
    #[test]
    fn idempotent_put_does_not_duplicate_sidecar_line() {
        let tmp = TempDir::new().expect("tempdir");
        let mut s = CasStore::open(tmp.path()).expect("open");
        let _ = s
            .put(b"content", ObjectType::Generic, "alice", 1, None)
            .unwrap();
        let _ = s
            .put(b"content", ObjectType::Generic, "alice", 1, None)
            .unwrap();
        let path = cas_index_path(tmp.path());
        let lines: Vec<&str> = std::fs::read_to_string(&path)
            .unwrap()
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                // own the str via leak — cheap for test
                Box::leak(l.to_string().into_boxed_str()) as &str
            })
            .collect();
        assert_eq!(
            lines.len(),
            1,
            "idempotent put should produce 1 sidecar line, got {}",
            lines.len()
        );
    }

    /// Append-only: each NEW put adds exactly ONE line.
    #[test]
    fn each_new_put_appends_one_line() {
        let tmp = TempDir::new().expect("tempdir");
        let mut s = CasStore::open(tmp.path()).expect("open");
        for i in 0..5 {
            s.put(
                format!("c{i}").as_bytes(),
                ObjectType::Generic,
                "system",
                i,
                None,
            )
            .unwrap();
        }
        let path = cas_index_path(tmp.path());
        let line_count = std::fs::read_to_string(&path)
            .unwrap()
            .lines()
            .filter(|l| !l.is_empty())
            .count();
        assert_eq!(line_count, 5);
    }

    /// Corrupted JSONL → strict parse error with line number (not silent skip).
    #[test]
    fn corrupted_sidecar_line_returns_parse_error() {
        let tmp = TempDir::new().expect("tempdir");
        // Init repo + ONE valid put to get a known-good first line.
        {
            let mut s = CasStore::open(tmp.path()).expect("open");
            s.put(b"hello", ObjectType::Generic, "alice", 1, None)
                .unwrap();
        }
        // Corrupt: append a malformed line.
        let path = cas_index_path(tmp.path());
        let mut f = OpenOptions::new().append(true).open(&path).unwrap();
        f.write_all(b"this is not valid json\n").unwrap();
        f.sync_data().unwrap();

        // Reopen MUST fail with a typed IndexParse error citing the line number.
        let err = CasStore::open(tmp.path()).unwrap_err();
        match err {
            CasError::IndexParse { line, .. } => {
                assert_eq!(line, 2, "expected line 2 to be flagged");
            }
            other => panic!("expected IndexParse, got {other:?}"),
        }
    }

    /// Empty / non-existent sidecar → opens fresh with empty index.
    #[test]
    fn missing_sidecar_opens_fresh() {
        let tmp = TempDir::new().expect("tempdir");
        let s = CasStore::open(tmp.path()).expect("open");
        assert_eq!(s.len(), 0);
        assert!(s.is_empty());
    }

    #[test]
    fn cas_ref_points_to_commit_object_not_blob_after_put() {
        let tmp = TempDir::new().expect("tempdir");
        let mut s = CasStore::open(tmp.path()).expect("open");
        s.put(b"cas-chain-object", ObjectType::Generic, "alice", 1, None)
            .expect("put");

        let repo = Repository::open(tmp.path()).expect("repo");
        let oid =
            crate::bottom_white::ledger::transition_ledger::Git2LedgerWriter::head_chaintape_cas(
                tmp.path(),
            )
            .expect("read cas ref")
            .expect("cas ref present");
        let object = repo.find_object(oid, None).expect("cas head object");
        assert_eq!(
            object.kind(),
            Some(git2::ObjectType::Commit),
            "refs/chaintape/cas must point at the CAS commit-chain head, not at a raw blob"
        );
    }

    #[test]
    fn invalid_blob_cas_ref_fails_open_closed() {
        let tmp = TempDir::new().expect("tempdir");
        let _s = CasStore::open(tmp.path()).expect("init repo");
        let repo = Repository::open(tmp.path()).expect("repo");
        let blob_oid = repo.blob(b"not-a-cas-chain-commit").expect("blob");
        repo.reference(
            crate::bottom_white::ledger::transition_ledger::CHAINTAPE_CAS_REF,
            blob_oid,
            true,
            "test invalid cas ref",
        )
        .expect("write invalid cas ref");

        let err = CasStore::open(tmp.path())
            .expect_err("present but non-commit refs/chaintape/cas must fail closed");
        assert!(
            err.to_string().contains("not a CAS commit-chain head"),
            "expected invalid CAS ref diagnostic, got {err}"
        );
    }

    #[test]
    fn legacy_blob_cas_ref_with_sidecar_opens_and_upgrades_on_next_put() {
        let tmp = TempDir::new().expect("tempdir");
        let _s = CasStore::open(tmp.path()).expect("init repo");
        let repo = Repository::open(tmp.path()).expect("repo");
        let legacy_bytes = b"legacy-object-with-blob-ref";
        let legacy_oid = repo.blob(legacy_bytes).expect("legacy blob");
        let legacy_meta = CasObjectMetadata {
            cid: Cid::from_content(legacy_bytes),
            backend_oid_hex: legacy_oid.to_string(),
            object_type: ObjectType::Generic,
            creator: "legacy-writer".to_string(),
            created_at_logical_t: 1,
            schema_id: Some("legacy/schema.v1".to_string()),
            size_bytes: legacy_bytes.len() as u64,
        };
        append_to_sidecar(tmp.path(), &legacy_meta).expect("legacy sidecar");
        repo.reference(
            crate::bottom_white::ledger::transition_ledger::CHAINTAPE_CAS_REF,
            legacy_oid,
            true,
            "legacy latest-blob CAS ref",
        )
        .expect("write legacy blob ref");

        let mut s = CasStore::open(tmp.path()).expect("legacy blob ref should open via sidecar");
        assert_eq!(s.get(&legacy_meta.cid).expect("legacy get"), legacy_bytes);
        let new_cid = s
            .put(
                b"new-forward-object",
                ObjectType::Generic,
                "runner",
                2,
                None,
            )
            .expect("forward put upgrades legacy blob ref");

        let head =
            crate::bottom_white::ledger::transition_ledger::Git2LedgerWriter::head_chaintape_cas(
                tmp.path(),
            )
            .expect("read cas head")
            .expect("cas head");
        let object = repo.find_object(head, None).expect("cas head object");
        assert_eq!(
            object.kind(),
            Some(git2::ObjectType::Commit),
            "first forward put must upgrade legacy blob ref to CAS commit-chain head"
        );
        assert_eq!(s.get(&legacy_meta.cid).expect("legacy get"), legacy_bytes);
        assert_eq!(s.get(&new_cid).expect("new get"), b"new-forward-object");
    }

    #[test]
    fn merge_shaped_cas_chain_fails_validation() {
        let tmp = TempDir::new().expect("tempdir");
        {
            let mut s = CasStore::open(tmp.path()).expect("open");
            s.put(b"alpha", ObjectType::Generic, "alice", 1, None)
                .expect("put alpha");
            s.put(b"beta", ObjectType::Generic, "alice", 2, None)
                .expect("put beta");
        }

        let repo = Repository::open(tmp.path()).expect("repo");
        let head_oid =
            crate::bottom_white::ledger::transition_ledger::Git2LedgerWriter::head_chaintape_cas(
                tmp.path(),
            )
            .expect("read cas head")
            .expect("cas head");
        let head_commit = repo.find_commit(head_oid).expect("head commit");
        let parent_commit = head_commit.parent(0).expect("parent commit");
        let tree = head_commit.tree().expect("tree");
        let sig = git2::Signature::now("test", "test@local").expect("sig");
        let merge_oid = repo
            .commit(
                None,
                &sig,
                &sig,
                "invalid merge-shaped cas chain",
                &tree,
                &[&head_commit, &parent_commit],
            )
            .expect("merge commit");
        repo.reference(
            crate::bottom_white::ledger::transition_ledger::CHAINTAPE_CAS_REF,
            merge_oid,
            true,
            "point CAS ref at merge-shaped chain",
        )
        .expect("update cas ref");

        let err = CasStore::open(tmp.path())
            .expect_err("merge-shaped CAS histories must fail validation");
        assert!(
            err.to_string().contains("merge-shaped CAS histories"),
            "expected merge-shaped diagnostic, got {err}"
        );
    }

    #[test]
    fn cas_put_advances_strict_commit_chain_roots() {
        let tmp = TempDir::new().expect("tempdir");
        let mut s = CasStore::open(tmp.path()).expect("open");
        s.put(b"alpha", ObjectType::Generic, "alice", 1, None)
            .expect("put alpha");
        s.put(b"beta", ObjectType::Generic, "alice", 2, None)
            .expect("put beta");

        let records = crate::bottom_white::cas::git_chain::load_chain_records(tmp.path())
            .expect("chain records");
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].previous_cas_root, None);
        assert_eq!(
            records[1].previous_cas_root,
            Some(records[0].resulting_cas_root),
            "second CAS commit must chain to the first resulting CAS root"
        );
        assert_ne!(
            records[0].resulting_cas_root, records[1].resulting_cas_root,
            "each new CAS object must advance the CAS Merkle root"
        );
    }

    #[test]
    fn cas_chain_reconstructs_exact_metadata_index() {
        let tmp = TempDir::new().expect("tempdir");
        let mut s = CasStore::open(tmp.path()).expect("open");
        let cid_a = s
            .put(
                b"alpha",
                ObjectType::ProposalPayload,
                "alice",
                10,
                Some("schema/proposal.v1".into()),
            )
            .expect("put alpha");
        let cid_b = s
            .put(
                b"beta",
                ObjectType::LeanResult,
                "lean-runner",
                11,
                Some("schema/lean-result.v1".into()),
            )
            .expect("put beta");

        let from_chain =
            crate::bottom_white::cas::git_chain::reconstruct_index_from_chain(tmp.path())
                .expect("reconstruct from chain")
                .expect("cas commit-chain present");
        assert_eq!(from_chain.len(), 2);
        assert_eq!(
            from_chain.get(&cid_a),
            s.metadata(&cid_a),
            "chain-derived metadata must match hot index exactly"
        );
        assert_eq!(
            from_chain.get(&cid_b),
            s.metadata(&cid_b),
            "chain-derived metadata must match hot index exactly"
        );
    }

    #[test]
    fn missing_sidecar_rebuilds_from_cas_commit_chain() {
        let tmp = TempDir::new().expect("tempdir");
        let cid;
        {
            let mut s = CasStore::open(tmp.path()).expect("open");
            cid = s
                .put(
                    b"rebuild-me",
                    ObjectType::AttemptTelemetry,
                    "evaluator",
                    42,
                    Some("schema/attempt.v1".into()),
                )
                .expect("put");
        }
        std::fs::remove_file(cas_index_path(tmp.path())).expect("remove sidecar cache");

        let reopened = CasStore::open(tmp.path()).expect("reopen via cas chain");
        assert_eq!(reopened.len(), 1);
        assert_eq!(reopened.get(&cid).expect("get rebuilt cid"), b"rebuild-me");
        let meta = reopened.metadata(&cid).expect("rebuilt metadata");
        assert_eq!(meta.object_type, ObjectType::AttemptTelemetry);
        assert_eq!(meta.creator, "evaluator");
        assert_eq!(meta.created_at_logical_t, 42);
        assert_eq!(meta.schema_id.as_deref(), Some("schema/attempt.v1"));
    }

    #[test]
    fn missing_sidecar_rebuild_then_put_writes_complete_cache() {
        let tmp = TempDir::new().expect("tempdir");
        let (cid_a, cid_b, cid_c);
        {
            let mut s = CasStore::open(tmp.path()).expect("open");
            cid_a = s
                .put(b"alpha", ObjectType::Generic, "alice", 1, None)
                .expect("put alpha");
            cid_b = s
                .put(b"beta", ObjectType::Generic, "bob", 2, None)
                .expect("put beta");
        }
        std::fs::remove_file(cas_index_path(tmp.path())).expect("remove sidecar cache");

        {
            let mut rebuilt = CasStore::open(tmp.path()).expect("rebuild from chain");
            assert_eq!(rebuilt.len(), 2);
            cid_c = rebuilt
                .put(b"gamma", ObjectType::Generic, "carol", 3, None)
                .expect("put after rebuild");
        }

        let reopened = CasStore::open(tmp.path())
            .expect("cache recreated after chain rebuild must match the full chain");
        assert_eq!(reopened.len(), 3);
        assert_eq!(reopened.get(&cid_a).expect("get alpha"), b"alpha");
        assert_eq!(reopened.get(&cid_b).expect("get beta"), b"beta");
        assert_eq!(reopened.get(&cid_c).expect("get gamma"), b"gamma");
        let line_count = std::fs::read_to_string(cas_index_path(tmp.path()))
            .expect("sidecar cache")
            .lines()
            .filter(|line| !line.is_empty())
            .count();
        assert_eq!(
            line_count, 3,
            "recreated sidecar cache must contain the complete chain-derived index"
        );
    }

    #[test]
    fn legacy_sidecar_is_anchored_not_rewritten_into_historical_commits() {
        let tmp = TempDir::new().expect("tempdir");
        let _s = CasStore::open(tmp.path()).expect("init repo");
        let repo = Repository::open(tmp.path()).expect("repo");
        let legacy_oid = repo.blob(b"legacy-object").expect("legacy blob");
        let legacy_meta = CasObjectMetadata {
            cid: Cid::from_content(b"legacy-object"),
            backend_oid_hex: legacy_oid.to_string(),
            object_type: ObjectType::Generic,
            creator: "legacy-writer".to_string(),
            created_at_logical_t: 1,
            schema_id: Some("legacy/schema.v1".to_string()),
            size_bytes: b"legacy-object".len() as u64,
        };
        append_to_sidecar(tmp.path(), &legacy_meta).expect("legacy sidecar");
        let mut legacy_index = BTreeMap::new();
        legacy_index.insert(legacy_meta.cid, legacy_meta.clone());
        let legacy_root = crate::bottom_white::cas::git_chain::merkle_root_for_index(&legacy_index);

        let mut s = CasStore::open(tmp.path()).expect("open legacy sidecar");
        let new_cid = s
            .put(
                b"new-forward-object",
                ObjectType::LeanResult,
                "runner",
                2,
                None,
            )
            .expect("new put");

        let cache = load_index_from_sidecar(tmp.path()).expect("sidecar after put");
        let records =
            crate::bottom_white::cas::git_chain::load_chain_records_with_cache(tmp.path(), &cache)
                .expect("records");
        assert_eq!(
            records.len(),
            1,
            "legacy sidecar entries must not be retro-committed"
        );
        assert_eq!(records[0].cid, new_cid);
        assert_eq!(
            records[0].legacy_prefix_metadata,
            vec![legacy_meta.clone()],
            "the first forward CAS commit carries a legacy metadata snapshot so the sidecar remains cache"
        );
        assert_eq!(
            records[0].previous_cas_root,
            Some(legacy_root),
            "first forward CAS commit should anchor the pre-chain legacy root"
        );
        assert_eq!(
            s.get(&legacy_meta.cid).expect("legacy get"),
            b"legacy-object"
        );
        assert_eq!(s.get(&new_cid).expect("new get"), b"new-forward-object");
    }

    #[test]
    fn missing_legacy_sidecar_with_forward_snapshot_rebuilds_successfully() {
        let tmp = TempDir::new().expect("tempdir");
        let _s = CasStore::open(tmp.path()).expect("init repo");
        let repo = Repository::open(tmp.path()).expect("repo");
        let legacy_oid = repo.blob(b"legacy-object").expect("legacy blob");
        let legacy_meta = CasObjectMetadata {
            cid: Cid::from_content(b"legacy-object"),
            backend_oid_hex: legacy_oid.to_string(),
            object_type: ObjectType::Generic,
            creator: "legacy-writer".to_string(),
            created_at_logical_t: 1,
            schema_id: None,
            size_bytes: b"legacy-object".len() as u64,
        };
        append_to_sidecar(tmp.path(), &legacy_meta).expect("legacy sidecar");
        {
            let mut s = CasStore::open(tmp.path()).expect("open legacy sidecar");
            s.put(
                b"new-forward-object",
                ObjectType::Generic,
                "runner",
                2,
                None,
            )
            .expect("new put");
        }
        std::fs::remove_file(cas_index_path(tmp.path())).expect("remove legacy cache");

        let reopened = CasStore::open(tmp.path())
            .expect("legacy prefix snapshot in the CAS chain should rebuild missing sidecar");
        assert_eq!(
            reopened.get(&legacy_meta.cid).expect("legacy get"),
            b"legacy-object"
        );
    }

    #[test]
    fn large_legacy_sidecar_prefix_rebuilds_from_chunked_chain() {
        let tmp = TempDir::new().expect("tempdir");
        let _s = CasStore::open(tmp.path()).expect("init repo");
        let repo = Repository::open(tmp.path()).expect("repo");
        let mut legacy_lines = String::new();
        let mut legacy_cids = Vec::new();
        let legacy_count = 512usize;

        for i in 0..legacy_count {
            let content = format!("legacy-object-{i:04}-{}", "x".repeat(96));
            let oid = repo.blob(content.as_bytes()).expect("legacy blob");
            let meta = CasObjectMetadata {
                cid: Cid::from_content(content.as_bytes()),
                backend_oid_hex: oid.to_string(),
                object_type: ObjectType::Generic,
                creator: format!("legacy-writer-{i:04}"),
                created_at_logical_t: i as u64,
                schema_id: Some(format!("legacy/schema/{i:04}")),
                size_bytes: content.len() as u64,
            };
            legacy_cids.push(meta.cid);
            legacy_lines.push_str(&serde_json::to_string(&meta).expect("legacy json"));
            legacy_lines.push('\n');
        }
        std::fs::write(cas_index_path(tmp.path()), legacy_lines).expect("write legacy sidecar");

        {
            let mut s = CasStore::open(tmp.path()).expect("open legacy sidecar");
            s.put(
                b"new-forward-object-after-large-legacy-prefix",
                ObjectType::Generic,
                "runner",
                legacy_count as u64 + 1,
                None,
            )
            .expect("new put");
        }
        std::fs::remove_file(cas_index_path(tmp.path())).expect("remove legacy cache");

        let reopened = CasStore::open(tmp.path())
            .expect("large legacy prefix chunks in the CAS chain should rebuild missing sidecar");
        assert_eq!(
            reopened.len(),
            legacy_count + 1,
            "all legacy entries plus the forward object must reconstruct"
        );
        assert!(
            reopened.metadata(&legacy_cids[0]).is_some(),
            "first legacy entry must be reconstructable"
        );
        assert!(
            reopened.metadata(&legacy_cids[legacy_count - 1]).is_some(),
            "last legacy entry must be reconstructable"
        );

        let records = crate::bottom_white::cas::git_chain::load_chain_records(tmp.path())
            .expect("chain records");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].legacy_prefix_metadata.len(), legacy_count);
        assert_eq!(
            records[0].legacy_prefix_chunk_count as usize, legacy_count,
            "large legacy prefixes must be stored as bounded tree blobs, not one oversized record"
        );
    }

    #[test]
    fn tampered_sidecar_mismatch_fails_closed_when_chain_exists() {
        let tmp = TempDir::new().expect("tempdir");
        {
            let mut s = CasStore::open(tmp.path()).expect("open");
            s.put(b"guarded", ObjectType::Generic, "alice", 1, None)
                .expect("put");
        }

        let path = cas_index_path(tmp.path());
        let tampered = std::fs::read_to_string(&path)
            .expect("read sidecar")
            .replace("\"creator\":\"alice\"", "\"creator\":\"mallory\"");
        std::fs::write(&path, tampered).expect("tamper sidecar");

        let err = CasStore::open(tmp.path()).expect_err("tampered cache must fail closed");
        assert!(
            err.to_string()
                .contains("CAS sidecar cache mismatch with CAS commit-chain"),
            "error should diagnose sidecar/chain mismatch, got {err}"
        );
    }

    #[test]
    fn open_waits_for_inflight_cas_chain_cache_refresh() {
        let tmp = TempDir::new().expect("tempdir");
        let mut s = CasStore::open(tmp.path()).expect("open");
        s.put(b"first", ObjectType::Generic, "alice", 1, None)
            .expect("first put");

        let _lock = acquire_cas_chain_lock(tmp.path()).expect("hold cas chain lock");
        let repo = Repository::open(tmp.path()).expect("repo");
        let content = b"second";
        let cid = Cid::from_content(content);
        let git_oid = repo.blob(content).expect("blob");
        let metadata = CasObjectMetadata {
            cid,
            backend_oid_hex: git_oid.to_string(),
            object_type: ObjectType::Generic,
            creator: "bob".to_string(),
            created_at_logical_t: 2,
            schema_id: None,
            size_bytes: content.len() as u64,
        };
        let mut prospective = load_index_from_sidecar(tmp.path()).expect("sidecar index");
        let previous_root = Some(crate::bottom_white::cas::git_chain::merkle_root_for_index(
            &prospective,
        ));
        prospective.insert(cid, metadata.clone());
        let resulting_root =
            crate::bottom_white::cas::git_chain::merkle_root_for_index(&prospective);
        crate::bottom_white::cas::git_chain::append_cas_commit(
            tmp.path(),
            &metadata,
            previous_root,
            resulting_root,
            Vec::new(),
        )
        .expect("advance cas chain");

        let reader_path = tmp.path().to_path_buf();
        let reader = std::thread::spawn(move || CasStore::open(&reader_path));
        std::thread::sleep(Duration::from_millis(50));
        assert!(
            !reader.is_finished(),
            "reader open must wait for the CAS chain lock while sidecar cache is stale"
        );

        append_to_sidecar(tmp.path(), &metadata).expect("finish sidecar cache refresh");
        drop(_lock);

        let reopened = reader
            .join()
            .expect("reader thread")
            .expect("reader must not misclassify in-flight cache refresh as corruption");
        assert_eq!(reopened.len(), 2);
        assert!(reopened.metadata(&cid).is_some());
    }

    #[test]
    fn forced_cas_ref_update_failure_fails_put_closed() {
        let tmp = TempDir::new().expect("tempdir");
        let mut s = CasStore::open(tmp.path()).expect("open");

        crate::bottom_white::cas::git_chain::set_force_ref_update_failure_for_test(true);
        let err = s
            .put(b"must-not-commit", ObjectType::Generic, "alice", 1, None)
            .expect_err("forced CAS ref failure must fail put");
        crate::bottom_white::cas::git_chain::set_force_ref_update_failure_for_test(false);

        assert!(err
            .to_string()
            .contains("forced test CAS ref update failure"));
        assert_eq!(s.len(), 0, "hot index must not accept failed CAS put");
        assert!(
            !cas_index_path(tmp.path()).exists(),
            "sidecar cache must not be written before the canonical CAS ref advances"
        );
        let head =
            crate::bottom_white::ledger::transition_ledger::Git2LedgerWriter::head_chaintape_cas(
                tmp.path(),
            )
            .expect("read cas ref");
        assert!(head.is_none(), "failed put must not leave a CAS head ref");
    }

    #[test]
    fn oversized_cas_chain_record_fails_put_before_ref_or_cache() {
        let tmp = TempDir::new().expect("tempdir");
        let mut s = CasStore::open(tmp.path()).expect("open");
        let oversized_creator = "creator".repeat(12_000);

        let err = s
            .put(
                b"oversized-record",
                ObjectType::Generic,
                &oversized_creator,
                1,
                Some("schema.v1".to_string()),
            )
            .expect_err("writer must reject records its reader cannot replay");

        assert!(
            err.to_string()
                .contains("CAS chain record exceeds bounded read limit"),
            "expected bounded record diagnostic, got {err}"
        );
        assert_eq!(s.len(), 0, "hot index must not accept failed CAS put");
        assert!(
            !cas_index_path(tmp.path()).exists(),
            "sidecar cache must not be written when canonical CAS commit is rejected"
        );
        let head =
            crate::bottom_white::ledger::transition_ledger::Git2LedgerWriter::head_chaintape_cas(
                tmp.path(),
            )
            .expect("read cas ref");
        assert!(head.is_none(), "failed put must not leave a CAS head ref");
        let reopened = CasStore::open(tmp.path()).expect("repo remains openable");
        assert_eq!(reopened.len(), 0);
    }

    #[test]
    fn tape_derived_lookup_helpers_return_exact_expected_cids() {
        let tmp = TempDir::new().expect("tempdir");
        let mut s = CasStore::open(tmp.path()).expect("open");
        let cid_a = s
            .put(
                b"proposal-a",
                ObjectType::ProposalPayload,
                "alice",
                7,
                Some("schema/shared.v1".into()),
            )
            .expect("put a");
        let cid_b = s
            .put(
                b"proposal-b",
                ObjectType::ProposalPayload,
                "bob",
                8,
                Some("schema/shared.v1".into()),
            )
            .expect("put b");
        let cid_c = s
            .put(
                b"lean-result",
                ObjectType::LeanResult,
                "alice",
                9,
                Some("schema/lean.v1".into()),
            )
            .expect("put c");

        assert_eq!(
            s.list_cids_by_schema_id("schema/shared.v1"),
            vec![cid_a, cid_b]
        );
        assert_eq!(s.list_cids_by_creator("alice"), vec![cid_a, cid_c]);
        assert_eq!(s.list_cids_by_logical_t(8), vec![cid_b]);
        assert_eq!(
            s.list_cids_by_object_type(ObjectType::ProposalPayload),
            vec![cid_a, cid_b]
        );
    }

    #[test]
    fn cas_chain_rejects_backend_blob_cid_mismatch() {
        let tmp = TempDir::new().expect("tempdir");
        let _s = CasStore::open(tmp.path()).expect("init repo");
        let repo = Repository::open(tmp.path()).expect("repo");
        let wrong_oid = repo.blob(b"xxxxx").expect("wrong blob");
        let metadata = CasObjectMetadata {
            cid: Cid::from_content(b"abcde"),
            backend_oid_hex: wrong_oid.to_string(),
            object_type: ObjectType::Generic,
            creator: "test".to_string(),
            created_at_logical_t: 1,
            schema_id: None,
            size_bytes: 5,
        };
        let mut index = BTreeMap::new();
        index.insert(metadata.cid, metadata.clone());
        let root = crate::bottom_white::cas::git_chain::merkle_root_for_index(&index);

        let err = crate::bottom_white::cas::git_chain::append_cas_commit(
            tmp.path(),
            &metadata,
            None,
            root,
            Vec::new(),
        )
        .expect_err("mismatched backend blob must not enter CAS chain");
        assert!(
            err.to_string().contains("backend blob cid mismatch"),
            "expected backend cid mismatch, got {err}"
        );
    }

    #[test]
    fn cas_chain_rejects_backend_blob_above_hard_validation_cap() {
        struct EnvGuard {
            key: &'static str,
            prev: Option<String>,
        }
        impl Drop for EnvGuard {
            fn drop(&mut self) {
                match &self.prev {
                    Some(value) => std::env::set_var(self.key, value),
                    None => std::env::remove_var(self.key),
                }
            }
        }

        let key = "TURINGOS_CAS_CHAIN_MAX_BACKEND_BLOB_BYTES";
        let _guard = EnvGuard {
            key,
            prev: std::env::var(key).ok(),
        };
        std::env::set_var(key, "4");

        let tmp = TempDir::new().expect("tempdir");
        let _s = CasStore::open(tmp.path()).expect("init repo");
        let repo = Repository::open(tmp.path()).expect("repo");
        let content = b"abcde";
        let oid = repo.blob(content).expect("blob");
        let metadata = CasObjectMetadata {
            cid: Cid::from_content(content),
            backend_oid_hex: oid.to_string(),
            object_type: ObjectType::Generic,
            creator: "test".to_string(),
            created_at_logical_t: 1,
            schema_id: None,
            size_bytes: content.len() as u64,
        };
        let mut index = BTreeMap::new();
        index.insert(metadata.cid, metadata.clone());
        let root = crate::bottom_white::cas::git_chain::merkle_root_for_index(&index);

        let err = crate::bottom_white::cas::git_chain::append_cas_commit(
            tmp.path(),
            &metadata,
            None,
            root,
            Vec::new(),
        )
        .expect_err("backend blob above hard cap must not enter CAS chain");
        assert!(
            err.to_string()
                .contains("exceeds bounded read limit before content read"),
            "expected hard-cap diagnostic, got {err}"
        );
    }

    #[test]
    fn forced_backend_validation_timeout_fails_put_closed() {
        let tmp = TempDir::new().expect("tempdir");
        let mut s = CasStore::open(tmp.path()).expect("open");

        crate::bottom_white::cas::git_chain::set_force_backend_validate_timeout_for_test(true);
        let err = s
            .put(b"timeout-me", ObjectType::Generic, "alice", 1, None)
            .expect_err("forced backend validation timeout must fail put");
        crate::bottom_white::cas::git_chain::set_force_backend_validate_timeout_for_test(false);

        assert!(
            err.to_string()
                .contains("CAS chain backend validation timed out"),
            "expected backend validation timeout, got {err}"
        );
        assert_eq!(s.len(), 0);
        assert!(!cas_index_path(tmp.path()).exists());
    }
}
