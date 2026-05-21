/// TRACE_MATRIX FC3: Phase 7 W5 — CAS-backed preview run endpoint
///
/// Route exposed:
///   GET /api/preview/:artifact_bundle_cid/file?path=<relative-path>&session_id=<session_id>&sandbox_policy=<sandbox_policy>
///
/// FC-trace: FC3 (CAS evidence binding)
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
use turingosv4::runtime::preview_run::{
    write_preview_run, PreviewRunCapsule, SandboxPolicy, PREVIEW_RUN_CAPSULE_SCHEMA_ID,
};

#[cfg(feature = "web")]
#[derive(Deserialize)]
/// TRACE_MATRIX FC3: query parameters for preview serving
pub(crate) struct PreviewQuery {
    path: String,
    session_id: String,
    sandbox_policy: String,
}

/// TRACE_MATRIX FC3: GET /api/preview/:artifact_bundle_cid/file handler
#[cfg(feature = "web")]
pub(crate) async fn preview_get_handler(
    State(_state): State<AppState>,
    Path(artifact_bundle_cid_hex): Path<String>,
    Query(query): Query<PreviewQuery>,
) -> Result<Response, (StatusCode, String)> {
    // 1. Path-traversal validation: path must match ^(?!/)(?!.*(?:^|/)\.\.(?:/|$)).+
    let req_path = &query.path;
    if !is_valid_path(req_path) {
        return Err((
            StatusCode::BAD_REQUEST,
            "path traversal attempt or invalid path detected".to_string(),
        ));
    }

    // 2. Validate session_id — safe filesystem characters only
    if !is_safe_path_component(&query.session_id) {
        return Err((
            StatusCode::BAD_REQUEST,
            "invalid session_id format".to_string(),
        ));
    }

    // 3. Map sandbox_policy string to SandboxPolicy enum
    let sandbox_policy = match query.sandbox_policy.to_lowercase().as_str() {
        "allowscripts" => SandboxPolicy::AllowScripts,
        "allowscriptsallowsameorigin" => SandboxPolicy::AllowScriptsAllowSameOrigin,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                "invalid sandbox_policy value; expected 'allowscripts' or 'allowscriptsallowsameorigin'"
                    .to_string(),
            ));
        }
    };

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

    // Open the CAS store to verify file existence and read manifest.
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

    // Check if the bundle exists and has correct schema
    let mut serve_success = false;
    let mut file_bytes = Vec::new();
    let mut mime_type = "application/octet-stream".to_string();

    if let Some(metadata) = store.metadata(&bundle_cid) {
        if metadata.schema_id.as_deref() == Some(turingosv4::runtime::artifact_bundle::ARTIFACT_BUNDLE_SCHEMA_ID) {
            if let Ok(manifest_bytes) = store.get(&bundle_cid) {
                if let Ok(manifest) = serde_json::from_slice::<turingosv4::runtime::artifact_bundle::ArtifactBundleManifest>(&manifest_bytes) {
                    if let Some(file_entry) = manifest.files.iter().find(|f| f.path == *req_path) {
                        if let Some(file_cid) = cid_from_hex(&file_entry.cid) {
                            if let Ok(bytes) = store.get(&file_cid) {
                                file_bytes = bytes;
                                mime_type = file_entry.mime.clone();
                                serve_success = true;
                            }
                        }
                    }
                }
            }
        }
    }

    // Get logical timestamp
    use std::time::{SystemTime, UNIX_EPOCH};
    let logical_t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    // Construct PreviewRunCapsule
    let capsule = PreviewRunCapsule {
        schema_id: PREVIEW_RUN_CAPSULE_SCHEMA_ID.to_string(),
        artifact_bundle_cid: artifact_bundle_cid_hex,
        session_id: query.session_id.clone(),
        entrypoint_path: query.path.clone(),
        sandbox_policy,
        serve_success,
        logical_t,
    };

    // Write the capsule to CAS
    if let Err(e) = write_preview_run(ws_path, &capsule) {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to write preview run capsule to CAS: {e}"),
        ));
    }

    if !serve_success {
        return Err((
            StatusCode::NOT_FOUND,
            format!("file path '{req_path}' not found in artifact bundle manifest"),
        ));
    }

    // Return bytes with Content-Type from manifest.
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime_type)
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
fn is_safe_path_component(s: &str) -> bool {
    if s.is_empty() || s.len() > 128 {
        return false;
    }
    s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
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
