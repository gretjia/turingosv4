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

use git2::{ObjectType as Git2ObjectType, Repository};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use super::schema::{CasObjectMetadata, Cid, ObjectType};

const CAS_INDEX_FILENAME: &str = ".turingos_cas_index.jsonl";

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
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    // Atomic single-write append: serialize + newline in one buffer.
    let mut line = serialized.into_bytes();
    line.push(b'\n');
    f.write_all(&line)?;
    f.sync_data()?;
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
        let index = load_index_from_sidecar(&repo_path)?;
        Ok(Self { repo_path, index })
    }

    fn open_repo(&self) -> Result<Repository, CasError> {
        Repository::open(&self.repo_path).map_err(CasError::from)
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
        let repo = self.open_repo()?;
        let git_oid = repo.blob(content)?;

        // If already in index, idempotent: just return Cid (content addressing
        // guarantees same content → same Cid → already present)
        if self.index.contains_key(&cid) {
            return Ok(cid);
        }

        let metadata = CasObjectMetadata {
            cid,
            backend_oid_hex: git_oid.to_string(),
            object_type,
            creator: creator.to_string(),
            created_at_logical_t,
            schema_id,
            size_bytes: content.len() as u64,
        };
        // CO1.4-extra: durably append BEFORE inserting into in-memory index
        // (so a crash mid-write leaves the runtime in a consistent state —
        // either the entry is durably recorded AND in-memory, or neither).
        append_to_sidecar(&self.repo_path, &metadata)?;
        self.index.insert(cid, metadata);
        Ok(cid)
    }

    /// Retrieve content by Cid. Verifies content sha256 matches Cid (corruption check).
    pub fn get(&self, cid: &Cid) -> Result<Vec<u8>, CasError> {
        let metadata = self
            .index
            .get(cid)
            .ok_or(CasError::CidNotFound(*cid))?;
        let repo = self.open_repo()?;
        let git_oid = git2::Oid::from_str(&metadata.backend_oid_hex)
            .map_err(CasError::Git2)?;
        let blob = repo.find_blob(git_oid)?;
        let content = blob.content().to_vec();

        // Verify content sha256 matches Cid (defense against corruption).
        let mut h = Sha256::new();
        h.update(&content);
        let computed = Cid(h.finalize().into());
        if &computed != cid {
            return Err(CasError::CidMismatch {
                expected: *cid,
                computed,
            });
        }

        Ok(content)
    }

    /// Get metadata only (no content fetch).
    pub fn metadata(&self, cid: &Cid) -> Option<&CasObjectMetadata> {
        self.index.get(cid)
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// Merkle root over all CAS object metadata; deterministic per BTreeMap order.
    pub fn merkle_root(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        for (_cid, meta) in &self.index {
            h.update(meta.canonical_hash());
        }
        h.finalize().into()
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
        let cid = s.put(b"hello world", ObjectType::ProposalPayload, "alice", 100, None).unwrap();
        let content = s.get(&cid).unwrap();
        assert_eq!(content, b"hello world");
    }

    #[test]
    fn put_get_round_trip_large() {
        let (_tmp, mut s) = fresh_store();
        let big = vec![0xab; 65536];
        let cid = s.put(&big, ObjectType::PredicateBytecode, "system", 0, Some("wasm".into())).unwrap();
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
        let cid = s.put(b"specific content", ObjectType::Generic, "system", 0, None).unwrap();
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
        let cid = s.put(b"meta test", ObjectType::CounterexamplePayload, "carol", 250, Some("v1".into())).unwrap();
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
            s1.put(content, ObjectType::Generic, "system", 0, None).unwrap();
        }
        // Different insertion order
        for content in [b"c".as_slice(), b"b".as_slice(), b"a".as_slice()] {
            s2.put(content, ObjectType::Generic, "system", 0, None).unwrap();
        }
        assert_eq!(s1.merkle_root(), s2.merkle_root(),
            "BTreeMap-ordered: insertion order independent (I-DET)");
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

        let cid_a = store_a.put(b"only in a", ObjectType::Generic, "agent_a", 100, None).unwrap();
        let cid_b = store_b.put(b"only in b", ObjectType::Generic, "agent_b", 100, None).unwrap();

        // Each store has its own object only
        assert!(store_a.get(&cid_a).is_ok(), "store_a has cid_a");
        assert!(store_a.get(&cid_b).is_err(), "store_a lacks cid_b (isolated)");
        assert!(store_b.get(&cid_b).is_ok(), "store_b has cid_b");
        assert!(store_b.get(&cid_a).is_err(), "store_b lacks cid_a (isolated)");
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
                .put(b"beta", ObjectType::CounterexamplePayload, "bob", 2, Some("s.v1".into()))
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
        assert_eq!(lines.len(), 1, "idempotent put should produce 1 sidecar line, got {}", lines.len());
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
            s.put(b"hello", ObjectType::Generic, "alice", 1, None).unwrap();
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
}
