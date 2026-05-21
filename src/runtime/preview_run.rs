/// TRACE_MATRIX FC3: Phase 7 W5 — CAS-anchored read-only preview run capsule
///
/// Each preview run produces exactly one `PreviewRunCapsule` recording
/// the artifact bundle CID, session ID, entrypoint path, and sandbox policy.
///
/// FC-trace: FC3 (CAS evidence binding)
/// Risk class: Class 2.

use std::path::Path;
use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::CasStore;
use crate::runtime::spec_capsule::{cas_path, CapsuleError};

/// TRACE_MATRIX FC3: preview run capsule schema identifier
pub const PREVIEW_RUN_CAPSULE_SCHEMA_ID: &str = "turingos-preview-run-v1";

/// TRACE_MATRIX FC3: Byte-stable sandbox policy enum.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum SandboxPolicy {
    AllowScripts = 0,
    AllowScriptsAllowSameOrigin = 1,
}

/// TRACE_MATRIX FC3: Preview run capsule containing preview attempt metadata.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct PreviewRunCapsule {
    pub schema_id: String,                       // = "turingos-preview-run-v1"
    pub artifact_bundle_cid: String,
    pub session_id: String,
    pub entrypoint_path: String,                 // matches path regex
    pub sandbox_policy: SandboxPolicy,
    pub serve_success: bool,
    pub logical_t: u64,
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

/// TRACE_MATRIX FC3: Writes the PreviewRunCapsule to CAS store.
pub fn write_preview_run(
    workspace: &Path,
    body: &PreviewRunCapsule,
) -> Result<String, CapsuleError> {
    // Validate entrypoint_path path safety
    if !is_valid_path(&body.entrypoint_path) {
        return Err(CapsuleError::Put(format!(
            "Entrypoint path traversal validation failed: {}",
            body.entrypoint_path
        )));
    }

    if body.schema_id != PREVIEW_RUN_CAPSULE_SCHEMA_ID {
        return Err(CapsuleError::Put(format!(
            "Invalid schema_id: expected '{}', got '{}'",
            PREVIEW_RUN_CAPSULE_SCHEMA_ID, body.schema_id
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
            "preview_system",
            body.logical_t,
            Some(PREVIEW_RUN_CAPSULE_SCHEMA_ID.to_string()),
        )
        .map_err(|e| CapsuleError::Put(e.to_string()))?;

    Ok(cid.hex())
}
