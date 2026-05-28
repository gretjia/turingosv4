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
    response::Response,
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
    let workspace = resolve_workspace();
    let ws_path = std::path::Path::new(&workspace);
    let bundle_file = match turingosv4::runtime::artifact_bundle::read_artifact_bundle_file(
        ws_path,
        &artifact_bundle_cid_hex,
        req_path,
    ) {
        Ok(file) => file,
        Err(turingosv4::runtime::artifact_bundle::ArtifactBundleReadError::InvalidPath(_)) => {
            return Err((
                StatusCode::BAD_REQUEST,
                "path traversal attempt or invalid path detected".to_string(),
            ));
        }
        Err(turingosv4::runtime::artifact_bundle::ArtifactBundleReadError::InvalidBundleCid(_)) => {
            return Err((
                StatusCode::BAD_REQUEST,
                "invalid artifact_bundle_cid format".to_string(),
            ));
        }
        Err(turingosv4::runtime::artifact_bundle::ArtifactBundleReadError::Open(e)) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to open CAS store: {e}"),
            ));
        }
        Err(turingosv4::runtime::artifact_bundle::ArtifactBundleReadError::ManifestRead(e)) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to retrieve bundle manifest: {e}"),
            ));
        }
        Err(turingosv4::runtime::artifact_bundle::ArtifactBundleReadError::ManifestDecode(e)) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to parse bundle manifest JSON: {e}"),
            ));
        }
        Err(turingosv4::runtime::artifact_bundle::ArtifactBundleReadError::InvalidFileCid(e)) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("invalid file CID hex in bundle manifest: {e}"),
            ));
        }
        Err(turingosv4::runtime::artifact_bundle::ArtifactBundleReadError::BundleNotFound(_)) => {
            return Err((
                StatusCode::NOT_FOUND,
                "artifact bundle not found in CAS index".to_string(),
            ));
        }
        Err(turingosv4::runtime::artifact_bundle::ArtifactBundleReadError::SchemaMismatch(_)) => {
            return Err((
                StatusCode::NOT_FOUND,
                "artifact bundle schema mismatch or not an artifact bundle".to_string(),
            ));
        }
        Err(turingosv4::runtime::artifact_bundle::ArtifactBundleReadError::FilePathNotFound(_)) => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("file path '{req_path}' not found in artifact bundle manifest"),
            ));
        }
        Err(turingosv4::runtime::artifact_bundle::ArtifactBundleReadError::FileRead(e)) => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("file content not found in CAS: {e}"),
            ));
        }
    };

    // Return bytes with Content-Type = files[].mime.
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, bundle_file.mime)
        .body(Body::from(bundle_file.bytes))
        .expect("response builder infallible"))
}

#[cfg(feature = "web")]
fn resolve_workspace() -> String {
    let raw = std::env::var("TURINGOS_WEB_WORKSPACE")
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "tmp/phase7_active".to_string());
    let p = std::path::PathBuf::from(&raw);
    if p.is_absolute() {
        return raw;
    }
    match std::env::current_dir() {
        Ok(cwd) => cwd.join(p).to_string_lossy().into_owned(),
        Err(_) => raw,
    }
}
