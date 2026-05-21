/// TRACE_MATRIX FC1 + FC3-N4: Phase 7 W5 — CAS-backed artifact bundle serving endpoint
///
/// Route exposed:
///   GET /api/bundle/:artifact_bundle_cid/file?path=<relative-path>
///
/// FC-trace: FC1 (external read), FC3 (CAS evidence)
/// Risk class: Class 2.

#[cfg(feature = "web")]
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};

#[cfg(feature = "web")]
use serde::Deserialize;

#[cfg(feature = "web")]
use super::ws::AppState;

#[cfg(feature = "web")]
#[derive(Deserialize)]
/// TRACE_MATRIX FC1: query parameters for bundle serving
pub(crate) struct ServeQuery {
    path: String,
}

/// TRACE_MATRIX FC1: GET /api/bundle/:artifact_bundle_cid/file?path=<relative-path> handler
#[cfg(feature = "web")]
pub(crate) async fn artifact_bundle_get_handler(
    State(_state): State<AppState>,
    Path(artifact_bundle_cid_hex): Path<String>,
    Query(query): Query<ServeQuery>,
) -> Result<Response, (StatusCode, String)> {
    // 1. Path-traversal validation: path must match ^(?!/)(?!.*(?:^|/)\.\.(?:/|$)).+
    let req_path = &query.path;
    if !is_valid_path(req_path) {
        return Err((
            StatusCode::BAD_REQUEST,
            "path traversal attempt or invalid path detected".to_string(),
        ));
    }

    // Parse the bundle CID from hex.
    let bundle_cid = match cid_from_hex(&artifact_bundle_cid_hex) {
        Some(cid) => cid,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                "invalid artifact_bundle_cid format".to_string(),
            ));
        }
    };

    // Open the CAS store.
    let workspace = resolve_workspace();
    let ws_path = std::path::Path::new(&workspace);
    let cas_dir = turingosv4::runtime::spec_capsule::cas_path(ws_path);
    let store = match turingosv4::bottom_white::cas::store::CasStore::open(&cas_dir) {
        Ok(s) => s,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to open CAS store: {e}"),
            ));
        }
    };

    // 1. Look up :artifact_bundle_cid in CAS index.
    let metadata = match store.metadata(&bundle_cid) {
        Some(meta) => meta,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                "artifact bundle not found in CAS index".to_string(),
            ));
        }
    };

    // 2. ABORT with 404 if metadata.schema_id != "turingos-artifact-bundle-v1".
    if metadata.schema_id.as_deref() != Some(turingosv4::runtime::artifact_bundle::ARTIFACT_BUNDLE_SCHEMA_ID) {
        return Err((
            StatusCode::NOT_FOUND,
            "artifact bundle schema mismatch or not an artifact bundle".to_string(),
        ));
    }

    // 3. Parse manifest body.
    let manifest_bytes = match store.get(&bundle_cid) {
        Ok(bytes) => bytes,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to retrieve bundle manifest: {e}"),
            ));
        }
    };

    let manifest = match serde_json::from_slice::<turingosv4::runtime::artifact_bundle::ArtifactBundleManifest>(&manifest_bytes) {
        Ok(m) => m,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to parse bundle manifest JSON: {e}"),
            ));
        }
    };

    // 4. Find `path` in manifest.files[].path (byte-equal match, no resolution).
    let file_entry = match manifest.files.iter().find(|f| f.path == *req_path) {
        Some(f) => f,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("file path '{req_path}' not found in artifact bundle manifest"),
            ));
        }
    };

    // Parse the file CID.
    let file_cid = match cid_from_hex(&file_entry.cid) {
        Some(cid) => cid,
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("invalid file CID hex '{}' in bundle manifest", file_entry.cid),
            ));
        }
    };

    // 6. Read file CID bytes from CAS.
    let file_bytes = match store.get(&file_cid) {
        Ok(bytes) => bytes,
        Err(e) => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("file content not found in CAS: {e}"),
            ));
        }
    };

    // 7. Return bytes with Content-Type = files[].mime.
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, file_entry.mime.clone())
        .body(Body::from(file_bytes))
        .expect("response builder infallible"))
}

#[cfg(feature = "web")]
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

#[cfg(feature = "web")]
fn resolve_workspace() -> String {
    if let Ok(val) = std::env::var("TURINGOS_WEB_WORKSPACE") {
        if !val.is_empty() {
            return val;
        }
    }
    "tmp/phase7_active".to_string()
}

#[cfg(feature = "web")]
fn cid_from_hex(s: &str) -> Option<turingosv4::bottom_white::cas::schema::Cid> {
    if s.len() != 64 {
        return None;
    }
    let mut bytes = [0u8; 32];
    for i in 0..32 {
        let hex_byte = &s[i * 2..i * 2 + 2];
        bytes[i] = u8::from_str_radix(hex_byte, 16).ok()?;
    }
    Some(turingosv4::bottom_white::cas::schema::Cid(bytes))
}
