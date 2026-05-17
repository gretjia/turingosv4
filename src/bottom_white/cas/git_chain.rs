//! Git-backed CAS commit-chain.
//!
//! The CAS `Cid` remains `sha256(content)`. This module only upgrades
//! `refs/chaintape/cas` from a latest-blob pointer into a strict Git commit
//! chain whose commit payload records the CAS metadata and resulting Merkle
//! root.

use git2::{Repository, Signature as GitSignature};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::Path;
use std::time::Duration;

use super::schema::{CasObjectMetadata, Cid, ObjectType};
use super::store::CasError;
use crate::bottom_white::ledger::transition_ledger::CHAINTAPE_CAS_REF;

const CAS_CHAIN_RECORD_BLOB: &str = "cas_chain_record.json";
const CAS_CHAIN_METADATA_BLOB: &str = "metadata.json";
const CAS_CHAIN_LEGACY_PREFIX_BLOB_PREFIX: &str = "legacy_prefix_metadata_";
const CAS_CHAIN_LEGACY_PREFIX_BLOB_SUFFIX: &str = ".json";
const CAS_CHAIN_RECORD_MAX_BYTES: u64 = 64 * 1024;
const CAS_CHAIN_BACKEND_BLOB_MAX_BYTES: u64 = 64 * 1024 * 1024;

/// TRACE_MATRIX WP-arch-§5.L3 + Art. 0.4: one Git commit payload in the
/// canonical CAS ref chain, binding CID metadata to previous/resulting roots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CasChainRecord {
    pub schema_version: String,
    pub cid: Cid,
    pub backend_oid_hex: String,
    pub object_type: ObjectType,
    pub schema_id: Option<String>,
    pub creator: String,
    pub created_at_logical_t: u64,
    pub size_bytes: u64,
    pub previous_cas_root: Option<[u8; 32]>,
    pub resulting_cas_root: [u8; 32],
    pub metadata: CasObjectMetadata,
    #[serde(default)]
    pub legacy_prefix_metadata: Vec<CasObjectMetadata>,
    #[serde(default)]
    pub legacy_prefix_chunk_count: u32,
}

impl CasChainRecord {
    fn new(
        metadata: CasObjectMetadata,
        previous_cas_root: Option<[u8; 32]>,
        resulting_cas_root: [u8; 32],
        legacy_prefix_metadata: Vec<CasObjectMetadata>,
    ) -> Self {
        Self {
            schema_version: "v1/cas_git_chain_record".to_string(),
            cid: metadata.cid,
            backend_oid_hex: metadata.backend_oid_hex.clone(),
            object_type: metadata.object_type,
            schema_id: metadata.schema_id.clone(),
            creator: metadata.creator.clone(),
            created_at_logical_t: metadata.created_at_logical_t,
            size_bytes: metadata.size_bytes,
            previous_cas_root,
            resulting_cas_root,
            metadata,
            legacy_prefix_metadata,
            legacy_prefix_chunk_count: 0,
        }
    }

    fn validate_mirror_fields(&self) -> Result<(), CasError> {
        if self.schema_version != "v1/cas_git_chain_record" {
            return Err(chain_error(format!(
                "unsupported CAS chain record schema_version {}",
                self.schema_version
            )));
        }
        if self.cid != self.metadata.cid
            || self.backend_oid_hex != self.metadata.backend_oid_hex
            || self.object_type != self.metadata.object_type
            || self.schema_id != self.metadata.schema_id
            || self.creator != self.metadata.creator
            || self.created_at_logical_t != self.metadata.created_at_logical_t
            || self.size_bytes != self.metadata.size_bytes
        {
            return Err(chain_error(format!(
                "CAS chain record mirror fields disagree with metadata for {}",
                self.cid
            )));
        }
        Ok(())
    }
}

/// TRACE_MATRIX FC2-N34 + Art. 0.2: deterministic CAS metadata root used for
/// replay validation and `previous_cas_root`/`resulting_cas_root` chaining.
pub fn merkle_root_for_index(index: &BTreeMap<Cid, CasObjectMetadata>) -> [u8; 32] {
    let mut h = Sha256::new();
    for meta in index.values() {
        h.update(meta.canonical_hash());
    }
    h.finalize().into()
}

/// TRACE_MATRIX FC1-wtool->Q_{t+1} + WP-arch-§5.L3: append one canonical CAS
/// commit and advance `refs/chaintape/cas` fail-closed.
pub fn append_cas_commit(
    repo_path: &Path,
    metadata: &CasObjectMetadata,
    previous_cas_root: Option<[u8; 32]>,
    resulting_cas_root: [u8; 32],
    legacy_prefix_metadata: Vec<CasObjectMetadata>,
) -> Result<git2::Oid, CasError> {
    #[cfg(test)]
    if FORCE_REF_UPDATE_FAILURE.load(std::sync::atomic::Ordering::SeqCst) {
        return Err(chain_error("forced test CAS ref update failure"));
    }

    let repo = Repository::open(repo_path).map_err(CasError::from)?;
    for prefix_metadata in &legacy_prefix_metadata {
        validate_metadata_backend_blob(repo_path, prefix_metadata)?;
    }
    validate_metadata_backend_blob(repo_path, metadata)?;
    let mut record = CasChainRecord::new(
        metadata.clone(),
        previous_cas_root,
        resulting_cas_root,
        Vec::new(),
    );
    record.legacy_prefix_chunk_count = legacy_prefix_metadata.len().try_into().map_err(|_| {
        chain_error(format!(
            "CAS legacy prefix metadata has too many entries: {}",
            legacy_prefix_metadata.len()
        ))
    })?;
    let record_json = serde_json::to_vec_pretty(&record)
        .map_err(|e| chain_error(format!("serialize CAS chain record: {e}")))?;
    let metadata_json = serde_json::to_vec(metadata)
        .map_err(|e| chain_error(format!("serialize CAS metadata: {e}")))?;
    if record_json.len() as u64 > CAS_CHAIN_RECORD_MAX_BYTES {
        return Err(chain_error(format!(
            "CAS chain record exceeds bounded read limit before commit: {} > {}",
            record_json.len(),
            CAS_CHAIN_RECORD_MAX_BYTES
        )));
    }
    if metadata_json.len() as u64 > CAS_CHAIN_RECORD_MAX_BYTES {
        return Err(chain_error(format!(
            "CAS chain metadata blob exceeds bounded read limit before commit: {} > {}",
            metadata_json.len(),
            CAS_CHAIN_RECORD_MAX_BYTES
        )));
    }

    let mut tb = repo
        .treebuilder(None)
        .map_err(|e| chain_error(format!("treebuilder: {e}")))?;
    let record_blob = repo
        .blob(&record_json)
        .map_err(|e| chain_error(format!("record blob: {e}")))?;
    tb.insert(CAS_CHAIN_RECORD_BLOB, record_blob, 0o100644)
        .map_err(|e| chain_error(format!("tree insert record: {e}")))?;
    let metadata_blob = repo
        .blob(&metadata_json)
        .map_err(|e| chain_error(format!("metadata blob: {e}")))?;
    tb.insert(CAS_CHAIN_METADATA_BLOB, metadata_blob, 0o100644)
        .map_err(|e| chain_error(format!("tree insert metadata: {e}")))?;
    for (idx, prefix_metadata) in legacy_prefix_metadata.iter().enumerate() {
        let chunk_json = serde_json::to_vec(prefix_metadata).map_err(|e| {
            chain_error(format!(
                "serialize CAS legacy prefix metadata chunk {idx}: {e}"
            ))
        })?;
        if chunk_json.len() as u64 > CAS_CHAIN_RECORD_MAX_BYTES {
            return Err(chain_error(format!(
                "CAS legacy prefix metadata chunk {idx} exceeds bounded read limit: {} > {}",
                chunk_json.len(),
                CAS_CHAIN_RECORD_MAX_BYTES
            )));
        }
        let chunk_blob = repo
            .blob(&chunk_json)
            .map_err(|e| chain_error(format!("legacy prefix chunk blob {idx}: {e}")))?;
        tb.insert(&legacy_prefix_blob_name(idx), chunk_blob, 0o100644)
            .map_err(|e| chain_error(format!("tree insert legacy prefix chunk {idx}: {e}")))?;
    }
    let tree_oid = tb
        .write()
        .map_err(|e| chain_error(format!("tree write: {e}")))?;
    let tree = repo
        .find_tree(tree_oid)
        .map_err(|e| chain_error(format!("find tree: {e}")))?;

    let (parent, legacy_ref_upgrade) =
        cas_head_commit_for_append(&repo_path, &repo, &legacy_prefix_metadata)?;
    let parents: Vec<git2::Commit<'_>> = match parent {
        Some(oid) => vec![find_commit_with_refresh(&repo, oid, "parent commit")?],
        None => Vec::new(),
    };
    let parent_refs: Vec<&git2::Commit<'_>> = parents.iter().collect();

    let time = git2::Time::new(metadata.created_at_logical_t as i64, 0);
    let author = GitSignature::new("turingosv4 cas", "cas@turingos", &time)
        .map_err(|e| chain_error(format!("git sig: {e}")))?;
    let message = format!(
        "cas put cid={} logical_t={}\n",
        metadata.cid, metadata.created_at_logical_t
    );
    let update_ref = if legacy_ref_upgrade {
        None
    } else {
        Some(CHAINTAPE_CAS_REF)
    };
    let commit_oid = repo
        .commit(update_ref, &author, &author, &message, &tree, &parent_refs)
        .map_err(|e| chain_error(format!("chaintape cas commit/ref update: {e}")))?;
    if legacy_ref_upgrade {
        validate_cas_chain_head_oid(repo_path, commit_oid)?;
        repo.reference(
            CHAINTAPE_CAS_REF,
            commit_oid,
            true,
            "upgrade legacy blob CAS ref to commit-chain head",
        )
        .map_err(|e| chain_error(format!("chaintape cas legacy ref upgrade: {e}")))?;
    }
    Ok(commit_oid)
}

/// TRACE_MATRIX FC2-boot/replay + Art. 0.4: distinguish absent CAS ref from a
/// present, validated CAS commit-chain head.
pub fn has_cas_commit_chain(repo_path: &Path) -> Result<bool, CasError> {
    let repo = Repository::open(repo_path).map_err(CasError::from)?;
    match cas_ref_target(&repo)? {
        CasRefTarget::Missing => Ok(false),
        CasRefTarget::Commit(_) => Ok(true),
        CasRefTarget::LegacyNonCommit(oid) => {
            if repo_contains_cas_chain_commit(repo_path, &repo)? {
                return Err(non_commit_cas_ref_error(oid));
            }
            Ok(false)
        }
    }
}

/// TRACE_MATRIX FC2-boot/replay: load validated CAS-chain records for audit
/// and missing-sidecar reconstruction.
pub fn load_chain_records(repo_path: &Path) -> Result<Vec<CasChainRecord>, CasError> {
    let repo = Repository::open(repo_path).map_err(CasError::from)?;
    let Some(head) = cas_head_commit(&repo)? else {
        return Ok(Vec::new());
    };
    let records = load_raw_records_from_head(repo_path, &repo, head)?;
    let _ = validate_records(repo_path, &records, &BTreeMap::new())?;
    Ok(records)
}

/// TRACE_MATRIX FC2-boot/replay + Art. 0.2: load CAS-chain records and verify
/// an existing sidecar is only a byte-equivalent cache.
pub fn load_chain_records_with_cache(
    repo_path: &Path,
    cache: &BTreeMap<Cid, CasObjectMetadata>,
) -> Result<Vec<CasChainRecord>, CasError> {
    let repo = Repository::open(repo_path).map_err(CasError::from)?;
    let Some(head) = cas_head_commit(&repo)? else {
        return Ok(Vec::new());
    };
    let records = load_raw_records_from_head(repo_path, &repo, head)?;
    let chain_index = validate_records(repo_path, &records, &BTreeMap::new())?;
    if cache != &chain_index {
        return Err(chain_error(
            "CAS sidecar cache mismatch with CAS commit-chain",
        ));
    }
    Ok(records)
}

/// TRACE_MATRIX Art. 0.4 + SG-A3-HEAD-T-C2.3: validate a proposed CAS ref
/// target as a replayable CAS commit-chain head.
pub fn validate_cas_chain_head_oid(repo_path: &Path, oid: git2::Oid) -> Result<(), CasError> {
    let repo = Repository::open(repo_path).map_err(CasError::from)?;
    let object = repo
        .find_object(oid, None)
        .map_err(|e| chain_error(format!("read CAS ref target: {e}")))?;
    if object.kind() != Some(git2::ObjectType::Commit) {
        return Err(chain_error(format!(
            "refs/chaintape/cas target {oid} is not a commit object"
        )));
    }
    let records = load_raw_records_from_head(repo_path, &repo, oid)?;
    validate_records(repo_path, &records, &BTreeMap::new()).map(|_| ())
}

/// TRACE_MATRIX Art. 0.4 + SG-A3-HEAD-T-C2.3: validate CAS ref advancement
/// without allowing rewinds or forks away from the current chain head.
pub fn validate_cas_chain_head_update(repo_path: &Path, oid: git2::Oid) -> Result<(), CasError> {
    validate_cas_chain_head_oid(repo_path, oid)?;
    let repo = Repository::open(repo_path).map_err(CasError::from)?;
    if let Some(current) = cas_head_commit(&repo)? {
        let is_descendant = repo
            .graph_descendant_of(oid, current)
            .map_err(|e| chain_error(format!("CAS chain descendant check: {e}")))?;
        if oid != current && !is_descendant {
            return Err(chain_error(format!(
                "CAS ref update target {oid} is not a descendant of current CAS head {current}"
            )));
        }
    }
    Ok(())
}

fn load_raw_records_from_head(
    repo_path: &Path,
    repo: &Repository,
    head: git2::Oid,
) -> Result<Vec<CasChainRecord>, CasError> {
    let mut newest_first = Vec::new();
    let mut cursor = Some(head);
    while let Some(oid) = cursor {
        let commit = find_commit_with_refresh(repo, oid, "walk CAS parent")?;
        if commit.parent_count() > 1 {
            return Err(chain_error(format!(
                "CAS commit-chain head {oid} has {} parents; merge-shaped CAS histories are forbidden",
                commit.parent_count()
            )));
        }
        newest_first.push(read_record_from_commit(repo_path, repo, &commit)?);
        cursor = if commit.parent_count() == 1 {
            Some(
                commit
                    .parent_id(0)
                    .map_err(|e| chain_error(format!("walk CAS parent: {e}")))?,
            )
        } else {
            None
        };
    }
    newest_first.reverse();
    Ok(newest_first)
}

/// TRACE_MATRIX FC2-boot/replay + Art. 0.2: reconstruct the CAS metadata index
/// solely from the canonical Git commit-chain.
pub fn reconstruct_index_from_chain(
    repo_path: &Path,
) -> Result<Option<BTreeMap<Cid, CasObjectMetadata>>, CasError> {
    reconstruct_index_from_chain_with_cache(repo_path, None)
}

/// TRACE_MATRIX FC2-boot/replay + Art. 0.2: reconstruct from chain while
/// fail-closing if a supplied sidecar cache disagrees.
pub fn reconstruct_index_from_chain_with_cache(
    repo_path: &Path,
    cache: Option<&BTreeMap<Cid, CasObjectMetadata>>,
) -> Result<Option<BTreeMap<Cid, CasObjectMetadata>>, CasError> {
    let repo = Repository::open(repo_path).map_err(CasError::from)?;
    let head = match cas_ref_target(&repo)? {
        CasRefTarget::Missing => return Ok(None),
        CasRefTarget::Commit(head) => head,
        CasRefTarget::LegacyNonCommit(oid) => {
            let Some(cache_index) = cache else {
                return Err(non_commit_cas_ref_error(oid));
            };
            if cache_index.is_empty() || !legacy_ref_target_matches_cache(oid, cache_index) {
                return Err(non_commit_cas_ref_error(oid));
            }
            if repo_contains_cas_chain_commit(repo_path, &repo)? {
                return Err(non_commit_cas_ref_error(oid));
            }
            return Ok(None);
        }
    };
    let records = load_raw_records_from_head(repo_path, &repo, head)?;
    let chain_index = validate_records(repo_path, &records, &BTreeMap::new())?;

    match cache {
        None => Ok(Some(chain_index)),
        Some(cache_index) if cache_index == &chain_index => Ok(Some(chain_index)),
        Some(_) => Err(chain_error(
            "CAS sidecar cache mismatch with CAS commit-chain",
        )),
    }
}

enum CasRefTarget {
    Missing,
    Commit(git2::Oid),
    LegacyNonCommit(git2::Oid),
}

fn cas_ref_target(repo: &Repository) -> Result<CasRefTarget, CasError> {
    let reference = match repo.find_reference(CHAINTAPE_CAS_REF) {
        Ok(reference) => reference,
        Err(e) if e.code() == git2::ErrorCode::NotFound => return Ok(CasRefTarget::Missing),
        Err(e) => return Err(chain_error(format!("read CAS ref: {e}"))),
    };
    let oid = reference.target().ok_or_else(|| {
        chain_error(format!(
            "{CHAINTAPE_CAS_REF} exists but is not a direct CAS commit-chain ref"
        ))
    })?;
    let object = repo
        .find_object(oid, None)
        .map_err(|e| chain_error(format!("read CAS ref object: {e}")))?;
    if object.kind() != Some(git2::ObjectType::Commit) {
        return Ok(CasRefTarget::LegacyNonCommit(oid));
    }
    Ok(CasRefTarget::Commit(oid))
}

fn cas_head_commit(repo: &Repository) -> Result<Option<git2::Oid>, CasError> {
    match cas_ref_target(repo)? {
        CasRefTarget::Missing => Ok(None),
        CasRefTarget::Commit(oid) => Ok(Some(oid)),
        CasRefTarget::LegacyNonCommit(oid) => Err(non_commit_cas_ref_error(oid)),
    }
}

fn cas_head_commit_for_append(
    repo_path: &Path,
    repo: &Repository,
    legacy_prefix_metadata: &[CasObjectMetadata],
) -> Result<(Option<git2::Oid>, bool), CasError> {
    match cas_ref_target(repo)? {
        CasRefTarget::Missing => Ok((None, false)),
        CasRefTarget::Commit(oid) => Ok((Some(oid), false)),
        CasRefTarget::LegacyNonCommit(oid) => {
            if legacy_prefix_metadata.is_empty()
                || !legacy_prefix_metadata
                    .iter()
                    .any(|meta| meta.backend_oid_hex == oid.to_string())
                || repo_contains_cas_chain_commit(repo_path, repo)?
            {
                return Err(non_commit_cas_ref_error(oid));
            }
            Ok((None, true))
        }
    }
}

fn legacy_ref_target_matches_cache(
    oid: git2::Oid,
    cache: &BTreeMap<Cid, CasObjectMetadata>,
) -> bool {
    let oid_hex = oid.to_string();
    cache.values().any(|meta| meta.backend_oid_hex == oid_hex)
}

fn repo_contains_cas_chain_commit(repo_path: &Path, repo: &Repository) -> Result<bool, CasError> {
    let odb = repo
        .odb()
        .map_err(|e| chain_error(format!("open object database: {e}")))?;
    let mut found = false;
    odb.foreach(|oid| {
        if found {
            return false;
        }
        if let Ok(commit) = repo.find_commit(*oid) {
            if let Ok(tree) = commit.tree() {
                if tree
                    .get_name(CAS_CHAIN_RECORD_BLOB)
                    .is_some_and(|entry| entry.kind() == Some(git2::ObjectType::Blob))
                {
                    found = true;
                    return false;
                }
            }
        }
        true
    })
    .map_err(|e| {
        chain_error(format!(
            "scan CAS commit-chain objects under {}: {e}",
            repo_path.display()
        ))
    })?;
    Ok(found)
}

fn non_commit_cas_ref_error(oid: git2::Oid) -> CasError {
    chain_error(format!(
        "refs/chaintape/cas exists but target {oid} is not a CAS commit-chain head"
    ))
}

fn read_record_from_commit(
    repo_path: &Path,
    repo: &Repository,
    commit: &git2::Commit<'_>,
) -> Result<CasChainRecord, CasError> {
    let tree = commit_tree_with_refresh(repo, commit)?;
    let entry = tree
        .get_name(CAS_CHAIN_RECORD_BLOB)
        .ok_or_else(|| chain_error("CAS commit missing cas_chain_record.json"))?;
    if entry.kind() != Some(git2::ObjectType::Blob) {
        return Err(chain_error("CAS chain record is not a blob"));
    }
    let record_bytes = read_blob_bounded(
        repo_path,
        entry.id(),
        CAS_CHAIN_RECORD_MAX_BYTES,
        "CAS chain record",
    )?;
    let mut record: CasChainRecord = serde_json::from_slice(&record_bytes)
        .map_err(|e| chain_error(format!("parse CAS chain record: {e}")))?;
    record.validate_mirror_fields()?;
    if record.legacy_prefix_chunk_count > 0 {
        if !record.legacy_prefix_metadata.is_empty() {
            return Err(chain_error(
                "CAS chain record mixes inline and chunked legacy prefix metadata",
            ));
        }
        for idx in 0..record.legacy_prefix_chunk_count as usize {
            let name = legacy_prefix_blob_name(idx);
            let entry = tree
                .get_name(&name)
                .ok_or_else(|| chain_error(format!("CAS commit missing {name}")))?;
            if entry.kind() != Some(git2::ObjectType::Blob) {
                return Err(chain_error(format!("{name} is not a blob")));
            }
            let chunk_bytes = read_blob_bounded(
                repo_path,
                entry.id(),
                CAS_CHAIN_RECORD_MAX_BYTES,
                name.clone(),
            )?;
            let metadata: CasObjectMetadata =
                serde_json::from_slice(&chunk_bytes).map_err(|e| {
                    chain_error(format!("parse CAS legacy prefix metadata chunk {idx}: {e}"))
                })?;
            record.legacy_prefix_metadata.push(metadata);
        }
    }
    Ok(record)
}

fn find_commit_with_refresh<'repo>(
    repo: &'repo Repository,
    oid: git2::Oid,
    context: &str,
) -> Result<git2::Commit<'repo>, CasError> {
    match repo.find_commit(oid) {
        Ok(commit) => Ok(commit),
        Err(first) => {
            let _ = repo.odb().and_then(|odb| odb.refresh());
            repo.find_commit(oid).map_err(|second| {
                chain_error(format!("{context}: {second}; first attempt: {first}"))
            })
        }
    }
}

fn commit_tree_with_refresh<'repo>(
    repo: &'repo Repository,
    commit: &git2::Commit<'repo>,
) -> Result<git2::Tree<'repo>, CasError> {
    match commit.tree() {
        Ok(tree) => Ok(tree),
        Err(first) => {
            let _ = repo.odb().and_then(|odb| odb.refresh());
            commit.tree().map_err(|second| {
                chain_error(format!("CAS commit tree: {second}; first attempt: {first}"))
            })
        }
    }
}

fn validate_records(
    repo_path: &Path,
    records: &[CasChainRecord],
    prefix_index: &BTreeMap<Cid, CasObjectMetadata>,
) -> Result<BTreeMap<Cid, CasObjectMetadata>, CasError> {
    let mut index = prefix_index.clone();
    let mut previous_root = if index.is_empty() {
        None
    } else {
        Some(merkle_root_for_index(&index))
    };
    for (idx, record) in records.iter().enumerate() {
        record.validate_mirror_fields()?;
        if !record.legacy_prefix_metadata.is_empty() {
            if idx != 0 || !index.is_empty() {
                return Err(chain_error(
                    "CAS legacy prefix metadata is only allowed on the first chain record",
                ));
            }
            for prefix_metadata in &record.legacy_prefix_metadata {
                validate_metadata_backend_blob(repo_path, prefix_metadata)?;
                if index
                    .insert(prefix_metadata.cid, prefix_metadata.clone())
                    .is_some()
                {
                    return Err(chain_error(format!(
                        "duplicate legacy prefix CAS CID {}",
                        prefix_metadata.cid
                    )));
                }
            }
            previous_root = Some(merkle_root_for_index(&index));
        }
        validate_metadata_backend_blob(repo_path, &record.metadata)?;
        if record.previous_cas_root != previous_root {
            return Err(chain_error(format!(
                "CAS commit-chain root mismatch for {}: expected previous {:?}, got {:?}",
                record.cid, previous_root, record.previous_cas_root
            )));
        }
        if index.insert(record.cid, record.metadata.clone()).is_some() {
            return Err(chain_error(format!(
                "duplicate CAS commit-chain CID {}",
                record.cid
            )));
        }
        let computed = merkle_root_for_index(&index);
        if computed != record.resulting_cas_root {
            return Err(chain_error(format!(
                "CAS commit-chain resulting root mismatch for {}",
                record.cid
            )));
        }
        previous_root = Some(record.resulting_cas_root);
    }
    Ok(index)
}

fn validate_metadata_backend_blob(
    repo_path: &Path,
    metadata: &CasObjectMetadata,
) -> Result<(), CasError> {
    #[cfg(test)]
    if FORCE_BACKEND_VALIDATE_TIMEOUT.load(std::sync::atomic::Ordering::SeqCst) {
        return Err(chain_error(format!(
            "CAS chain backend validation timed out after 0s for {} (forced test hook)",
            metadata.cid
        )));
    }

    let oid = git2::Oid::from_str(&metadata.backend_oid_hex)
        .map_err(|e| chain_error(format!("parse backend oid for {}: {e}", metadata.cid)))?;
    let bytes = read_blob_bounded(
        repo_path,
        oid,
        cas_chain_backend_blob_max_bytes(),
        "CAS chain backend blob",
    )?;
    if bytes.len() as u64 != metadata.size_bytes {
        return Err(chain_error(format!(
            "backend blob size mismatch for {}: metadata={}, actual={}",
            metadata.cid,
            metadata.size_bytes,
            bytes.len()
        )));
    }
    let computed = Cid::from_content(&bytes);
    if computed != metadata.cid {
        return Err(chain_error(format!(
            "backend blob cid mismatch: metadata={}, computed={}",
            metadata.cid, computed
        )));
    }
    Ok(())
}

fn cas_chain_validate_timeout_secs() -> u64 {
    std::env::var("TURINGOS_CAS_CHAIN_VALIDATE_TIMEOUT_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .or_else(|| {
            std::env::var("TURINGOS_CAS_GET_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
        })
        .unwrap_or(10)
}

fn cas_chain_backend_blob_max_bytes() -> u64 {
    std::env::var("TURINGOS_CAS_CHAIN_MAX_BACKEND_BLOB_BYTES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(CAS_CHAIN_BACKEND_BLOB_MAX_BYTES)
}

fn read_blob_bounded(
    repo_path: &Path,
    oid: git2::Oid,
    max_bytes: u64,
    label: impl Into<String>,
) -> Result<Vec<u8>, CasError> {
    let repo_path = repo_path.to_path_buf();
    let label = label.into();
    let worker_label = label.clone();
    let timeout_secs = cas_chain_validate_timeout_secs();
    let (tx, rx) = std::sync::mpsc::channel::<Result<Vec<u8>, CasError>>();
    std::thread::Builder::new()
        .name("cas-chain-read-blob".to_string())
        .spawn(move || {
            let result: Result<Vec<u8>, CasError> = (|| {
                let repo = Repository::open(&repo_path).map_err(CasError::from)?;
                let odb = repo
                    .odb()
                    .map_err(|e| chain_error(format!("open object database: {e}")))?;
                let (size, kind) = odb.read_header(oid).map_err(|e| {
                    chain_error(format!("{worker_label} header missing for oid {oid}: {e}"))
                })?;
                if kind != git2::ObjectType::Blob {
                    return Err(chain_error(format!(
                        "{worker_label} oid {oid} is not a blob object"
                    )));
                }
                if size as u64 > max_bytes {
                    return Err(chain_error(format!(
                        "{worker_label} exceeds bounded read limit before content read: oid={oid} limit={max_bytes} actual={size}"
                    )));
                }
                let object = odb
                    .read(oid)
                    .map_err(|e| chain_error(format!("{worker_label} read failed for oid {oid}: {e}")))?;
                if object.kind() != git2::ObjectType::Blob {
                    return Err(chain_error(format!(
                        "{worker_label} oid {oid} changed kind during read"
                    )));
                }
                if object.len() != size {
                    return Err(chain_error(format!(
                        "{worker_label} oid {oid} size changed during read: header={size} read={}",
                        object.len()
                    )));
                }
                Ok(object.data().to_vec())
            })();
            let _ = tx.send(result);
        })
        .map_err(|e| chain_error(format!("{label} worker spawn: {e}")))?;

    match rx.recv_timeout(Duration::from_secs(timeout_secs)) {
        Ok(result) => result,
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => Err(chain_error(format!(
            "{label} read timed out after {timeout_secs}s for oid {oid}"
        ))),
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => Err(chain_error(format!(
            "{label} worker disconnected unexpectedly"
        ))),
    }
}

fn chain_error(detail: impl Into<String>) -> CasError {
    CasError::BackendCorruption(detail.into())
}

fn legacy_prefix_blob_name(idx: usize) -> String {
    format!("{CAS_CHAIN_LEGACY_PREFIX_BLOB_PREFIX}{idx:06}{CAS_CHAIN_LEGACY_PREFIX_BLOB_SUFFIX}")
}

#[cfg(test)]
static FORCE_REF_UPDATE_FAILURE: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

#[cfg(test)]
static FORCE_BACKEND_VALIDATE_TIMEOUT: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

#[cfg(test)]
pub fn set_force_ref_update_failure_for_test(value: bool) {
    FORCE_REF_UPDATE_FAILURE.store(value, std::sync::atomic::Ordering::SeqCst);
}

#[cfg(test)]
pub fn set_force_backend_validate_timeout_for_test(value: bool) {
    FORCE_BACKEND_VALIDATE_TIMEOUT.store(value, std::sync::atomic::Ordering::SeqCst);
}
