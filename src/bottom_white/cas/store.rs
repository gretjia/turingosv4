//! CAS store backed by git2-rs blob layer.
//!
//! Each runtime_repo (per spec § 5.2.2 cell isolation) has its own CasStore.
//! Objects are content-addressed by `Cid` (sha256 of content); git's sha-1
//! OID is recorded but not canonical.
//!
//! /// TRACE_MATRIX WP-arch-§5.L3 + spec-§5.2.2 (cell isolation): CAS store

use git2::{ObjectType as Git2ObjectType, Repository};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::schema::{CasObjectMetadata, Cid, ObjectType};

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
        }
    }
}

impl std::error::Error for CasError {}

impl From<git2::Error> for CasError {
    fn from(e: git2::Error) -> Self {
        Self::Git2(e)
    }
}

/// Content-addressable store backed by git's blob object database.
pub struct CasStore {
    repo_path: PathBuf,
    /// Cid → metadata index. BTreeMap per spec § 2 I-BTREE.
    index: BTreeMap<Cid, CasObjectMetadata>,
}

impl CasStore {
    /// Open or initialize a CAS store at the given runtime_repo path.
    /// Creates the git repo if it doesn't exist.
    pub fn open(repo_path: &Path) -> Result<Self, CasError> {
        let repo_path = repo_path.to_path_buf();
        // Init or open
        let _repo = match Repository::open(&repo_path) {
            Ok(r) => r,
            Err(_) => Repository::init(&repo_path)?,
        };
        Ok(Self {
            repo_path,
            index: BTreeMap::new(),
        })
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
}
