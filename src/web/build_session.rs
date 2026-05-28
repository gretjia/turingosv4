/// TRACE_MATRIX FC2-N16: GET /api/build/session/:session_id
///
/// Route exposed:
///   GET /api/build/session/:session_id
///
/// FC-trace: FC2 (derived view)
/// Risk class: Class 2.

#[cfg(feature = "web")]
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[cfg(feature = "web")]
use super::ws::AppState;
#[cfg(feature = "web")]
use turingosv4::runtime::build_session_view::derive_build_session_view;

/// TRACE_MATRIX FC2-N16: GET /api/build/session/:session_id route handler.
#[cfg(feature = "web")]
pub(crate) async fn build_session_handler(
    State(_state): State<AppState>,
    Path(session_id): Path<String>,
) -> Result<Response, (StatusCode, String)> {
    // Validate session_id — safe filesystem characters only
    if !is_safe_path_component(&session_id) {
        return Err((
            StatusCode::BAD_REQUEST,
            "invalid session_id format".to_string(),
        ));
    }

    let workspace = resolve_workspace();
    let ws_path = std::path::Path::new(&workspace);

    match derive_build_session_view(ws_path, &session_id) {
        Ok(view) => Ok(Json(view).into_response()),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to derive build session view: {e}"),
        )),
    }
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
