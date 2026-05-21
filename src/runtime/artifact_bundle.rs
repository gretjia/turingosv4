use std::path::Path;
use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::CasStore;
use crate::runtime::spec_capsule::{cas_path, CapsuleError};

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
    pub path: String,        // must match ^(?!/)(?!.*(?:^|/)\.\.(?:/|$)).+
    pub cid: String,         // hex cid of file bytes in CAS
    pub mime: String,
    pub sha256: String,
    pub size_bytes: u64,
    pub role: ArtifactFileRole,
}

/// TRACE_MATRIX FC1 + FC3-N4: Bundle manifest containing metadata for all generated artifacts.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ArtifactBundleManifest {
    pub schema_id: String,                       // = "turingos-artifact-bundle-v1"
    pub session_id: String,
    pub spec_capsule_cid: Option<String>,
    pub generation_attempt_cid: String,          // references C2 capsule
    pub previous_bundle_cid: Option<String>,     // provenance chain across regenerations
    pub files: Vec<ArtifactFileEntry>,
    pub entrypoint: String,                      // MUST equal one of files[].path
    pub bundle_size_bytes_total: u64,            // sum of files[].size_bytes
    pub created_at_logical_t: u64,
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
