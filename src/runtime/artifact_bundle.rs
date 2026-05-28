use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::CasStore;
use crate::runtime::cid_hex::cid_from_hex_str;
use crate::runtime::spec_capsule::{cas_path, CapsuleError};
use std::path::Path;

/// TRACE_MATRIX FC1: artifact bundle schema identifier
pub const ARTIFACT_BUNDLE_SCHEMA_ID: &str = "turingos-artifact-bundle-v1";

/// TRACE_MATRIX FC1: v5-derived typed role enum for files in the bundle.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ArtifactFileRole {
    Entrypoint,
    Source,
    Asset,
    Manifest,
    Test,
    Other,
}

/// TRACE_MATRIX FC1: Single file entry within the artifact bundle.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ArtifactFileEntry {
    pub path: String, // must match ^(?!/)(?!.*(?:^|/)\.\.(?:/|$)).+
    pub cid: String,  // hex cid of file bytes in CAS
    pub mime: String,
    pub sha256: String,
    pub size_bytes: u64,
    pub role: ArtifactFileRole,
}

/// TRACE_MATRIX FC1 + FC3-N4: Bundle manifest containing metadata for all generated artifacts.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ArtifactBundleManifest {
    pub schema_id: String, // = "turingos-artifact-bundle-v1"
    pub session_id: String,
    pub spec_capsule_cid: Option<String>,
    pub generation_attempt_cid: String, // references C2 capsule
    pub previous_bundle_cid: Option<String>, // provenance chain across regenerations
    pub files: Vec<ArtifactFileEntry>,
    pub entrypoint: String,           // MUST equal one of files[].path
    pub bundle_size_bytes_total: u64, // sum of files[].size_bytes
    pub created_at_logical_t: u64,
}

/// TRACE_MATRIX FC1 + FC3: runtime artifact-bundle read error for shared CLI/Web CAS reads.
#[derive(Debug)]
pub enum ArtifactBundleReadError {
    InvalidPath(String),
    InvalidBundleCid(String),
    Open(String),
    BundleNotFound(String),
    SchemaMismatch(String),
    ManifestRead(String),
    ManifestDecode(String),
    FilePathNotFound(String),
    InvalidFileCid(String),
    FileRead(String),
}

impl std::fmt::Display for ArtifactBundleReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPath(e) => write!(f, "invalid artifact path: {e}"),
            Self::InvalidBundleCid(e) => write!(f, "invalid artifact bundle cid: {e}"),
            Self::Open(e) => write!(f, "artifact bundle CAS open: {e}"),
            Self::BundleNotFound(e) => write!(f, "artifact bundle not found: {e}"),
            Self::SchemaMismatch(e) => write!(f, "artifact bundle schema mismatch: {e}"),
            Self::ManifestRead(e) => write!(f, "artifact bundle manifest read: {e}"),
            Self::ManifestDecode(e) => write!(f, "artifact bundle manifest decode: {e}"),
            Self::FilePathNotFound(e) => write!(f, "artifact bundle file path not found: {e}"),
            Self::InvalidFileCid(e) => write!(f, "invalid artifact file cid: {e}"),
            Self::FileRead(e) => write!(f, "artifact bundle file read: {e}"),
        }
    }
}

impl std::error::Error for ArtifactBundleReadError {}

/// TRACE_MATRIX FC1 + FC3: runtime artifact-bundle file bytes returned to transport layers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactBundleFileBytes {
    pub bytes: Vec<u8>,
    pub mime: String,
}

/// Helper validation for paths matching ^(?!/)(?!.*(?:^|/)\.\.(?:/|$)).+
fn is_valid_path(path: &str) -> bool {
    if path.is_empty() || path.starts_with('/') {
        return false;
    }
    for seg in path.split('/') {
        if seg == ".." {
            return false;
        }
    }
    true
}

/// TRACE_MATRIX FC3-N4: Writes the ArtifactBundleManifest to CAS store.
pub fn write_artifact_bundle(
    workspace: &Path,
    body: &ArtifactBundleManifest,
) -> Result<String, CapsuleError> {
    // 1. Path-traversal validation: path must match ^(?!/)(?!.*(?:^|/)\.\.(?:/|$)).+
    for file in &body.files {
        if !is_valid_path(&file.path) {
            return Err(CapsuleError::Put(format!(
                "Path traversal validation failed for file path: {}",
                file.path
            )));
        }
    }

    // 2. Entrypoint validation: entrypoint must be present in files[].path
    if !body.files.iter().any(|f| f.path == body.entrypoint) {
        return Err(CapsuleError::Put(format!(
            "Entrypoint '{}' not found in files list",
            body.entrypoint
        )));
    }

    let cas_dir = cas_path(workspace);
    std::fs::create_dir_all(&cas_dir)
        .map_err(|e| CapsuleError::Open(format!("create cas dir: {e}")))?;

    let mut store = CasStore::open(&cas_dir).map_err(|e| CapsuleError::Open(e.to_string()))?;

    let body_bytes =
        serde_json::to_vec(body).map_err(|e| CapsuleError::Put(format!("serialize body: {e}")))?;

    let cid = store
        .put(
            &body_bytes,
            ObjectType::EvidenceCapsule,
            "generate_system",
            body.created_at_logical_t,
            Some(ARTIFACT_BUNDLE_SCHEMA_ID.to_string()),
        )
        .map_err(|e| CapsuleError::Put(e.to_string()))?;

    Ok(cid.hex())
}

/// TRACE_MATRIX FC1 + FC3: Reads the latest ArtifactBundleManifest CID for a given session.
pub fn latest_artifact_bundle_cid_for_session(
    workspace: &Path,
    session_id: &str,
) -> Result<Option<String>, CapsuleError> {
    let cas_dir = cas_path(workspace);
    if !cas_dir.exists() {
        return Ok(None);
    }
    let store = match CasStore::open(&cas_dir) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };
    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
    let mut best: Option<(u64, Cid)> = None;
    for cid in cids {
        if let Some(meta) = store.metadata(&cid) {
            if meta.schema_id.as_deref() == Some(ARTIFACT_BUNDLE_SCHEMA_ID) {
                if let Ok(bytes) = store.get(&cid) {
                    if let Ok(manifest) = serde_json::from_slice::<ArtifactBundleManifest>(&bytes) {
                        if manifest.session_id == session_id {
                            match best {
                                Some((t, _)) if t >= meta.created_at_logical_t => {}
                                _ => best = Some((meta.created_at_logical_t, cid)),
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(best.map(|(_, cid)| cid.hex()))
}

/// TRACE_MATRIX FC1 + FC3: Reads one bundle manifest by CID from the runtime CAS.
pub fn read_artifact_bundle_manifest_by_cid(
    workspace: &Path,
    artifact_bundle_cid_hex: &str,
) -> Result<ArtifactBundleManifest, ArtifactBundleReadError> {
    let bundle_cid = cid_from_hex_str(artifact_bundle_cid_hex)
        .map_err(ArtifactBundleReadError::InvalidBundleCid)?;
    let cas_dir = cas_path(workspace);
    let store =
        CasStore::open(&cas_dir).map_err(|e| ArtifactBundleReadError::Open(e.to_string()))?;

    let metadata = store.metadata(&bundle_cid).ok_or_else(|| {
        ArtifactBundleReadError::BundleNotFound(artifact_bundle_cid_hex.to_string())
    })?;
    if metadata.schema_id.as_deref() != Some(ARTIFACT_BUNDLE_SCHEMA_ID) {
        return Err(ArtifactBundleReadError::SchemaMismatch(
            artifact_bundle_cid_hex.to_string(),
        ));
    }

    let manifest_bytes = store
        .get(&bundle_cid)
        .map_err(|e| ArtifactBundleReadError::ManifestRead(e.to_string()))?;
    serde_json::from_slice::<ArtifactBundleManifest>(&manifest_bytes)
        .map_err(|e| ArtifactBundleReadError::ManifestDecode(e.to_string()))
}

/// TRACE_MATRIX FC1 + FC3: Reads one file from an ArtifactBundleManifest.
pub fn read_artifact_bundle_file(
    workspace: &Path,
    artifact_bundle_cid_hex: &str,
    relative_path: &str,
) -> Result<ArtifactBundleFileBytes, ArtifactBundleReadError> {
    if !is_valid_path(relative_path) {
        return Err(ArtifactBundleReadError::InvalidPath(
            relative_path.to_string(),
        ));
    }

    let manifest = read_artifact_bundle_manifest_by_cid(workspace, artifact_bundle_cid_hex)?;
    let file_entry = manifest
        .files
        .iter()
        .find(|file| file.path == relative_path)
        .ok_or_else(|| ArtifactBundleReadError::FilePathNotFound(relative_path.to_string()))?;
    let file_cid =
        cid_from_hex_str(&file_entry.cid).map_err(ArtifactBundleReadError::InvalidFileCid)?;

    let cas_dir = cas_path(workspace);
    let store =
        CasStore::open(&cas_dir).map_err(|e| ArtifactBundleReadError::Open(e.to_string()))?;
    let bytes = store
        .get(&file_cid)
        .map_err(|e| ArtifactBundleReadError::FileRead(e.to_string()))?;
    Ok(ArtifactBundleFileBytes {
        bytes,
        mime: file_entry.mime.clone(),
    })
}
