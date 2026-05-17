/// TRACE_MATRIX FC1-N5 + FC2-N16: Phase 7 W5 — artifact file serving endpoint
///
/// Route exposed:
///   GET /api/artifact/:session_id/:name → serve one artifact file from
///     `<workspace>/sessions/<session_id>/artifacts/<name>` with the correct
///     Content-Type header.
///
/// FC-trace: FC1-N5 (read-view; no state mutation; path-traversal hygiene
///   critical at trust boundary).
/// Risk class: Class 2.
///
/// # Path-traversal hygiene (CRITICAL)
///
/// The handler uses `.join()` + `canonicalize()` + prefix check to ensure
/// the resolved path stays under `<workspace>/sessions/<session_id>/artifacts/`.
/// Any request whose canonicalized path escapes this root returns 400.
///
/// Specifically blocked:
/// - `..` components in either session_id or name
/// - URL-encoded traversal (`%2F`, `%2E%2E`) — decoded by axum before routing
/// - Absolute paths (caught by is_safe_* guards)
/// - Non-ASCII session IDs / file names that could confuse the VFS
///
/// # Content-Type
///
/// Sniffed by file extension using the same table as `generate.rs`.
/// Defaults to `application/octet-stream` for unknown extensions.
#[cfg(feature = "web")]
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};

#[cfg(feature = "web")]
use super::ws::AppState;

// ---------------------------------------------------------------------------
// GET /api/artifact/:session_id/:name handler
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: GET /api/artifact/:session_id/:name handler.
///
/// Validates both path segments, resolves the file path, canonicalizes to
/// prevent traversal, reads the file, and returns it with the correct
/// Content-Type.
#[cfg(feature = "web")]
pub(crate) async fn artifact_get_handler(
    State(_state): State<AppState>,
    Path((session_id, name)): Path<(String, String)>,
) -> Result<Response, (StatusCode, String)> {
    // Step 1: validate session_id — safe filesystem component chars only.
    if !is_safe_path_component(&session_id) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "session_id {:?} contains invalid characters; must match ^[a-zA-Z0-9_-]{{1,128}}$",
                session_id
            ),
        ));
    }

    // Step 2: validate name — safe filesystem component chars only;
    // allow dots for extensions (e.g. "index.html") but not ".." sequences.
    if !is_safe_artifact_name(&name) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "artifact name {:?} contains invalid or unsafe characters",
                name
            ),
        ));
    }

    // Step 3: resolve workspace and build the expected root.
    let workspace = resolve_workspace();
    let artifacts_root = std::path::PathBuf::from(&workspace)
        .join("sessions")
        .join(&session_id)
        .join("artifacts");

    // Step 4: canonicalize the root FIRST (so we have an absolute resolved base).
    // If the root doesn't exist, 404.
    let canonical_root = match artifacts_root.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("session {:?} artifacts directory not found", session_id),
            ));
        }
    };

    // Step 5: join the name (already validated) and canonicalize the full path.
    let full_path = canonical_root.join(&name);
    let canonical_full = match full_path.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            // canonicalize fails if the file does not exist.
            return Err((
                StatusCode::NOT_FOUND,
                format!("artifact {:?} not found in session {:?}", name, session_id),
            ));
        }
    };

    // Step 6: CRITICAL — verify the canonicalized path starts with canonical_root.
    // This catches any remaining traversal attempts.
    if !canonical_full.starts_with(&canonical_root) {
        return Err((
            StatusCode::BAD_REQUEST,
            "path traversal attempt detected".to_string(),
        ));
    }

    // Step 7: read file bytes (std::fs in spawn_blocking; tokio::fs not available).
    let full_for_read = canonical_full.clone();
    let bytes = tokio::task::spawn_blocking(move || std::fs::read(&full_for_read))
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("spawn_blocking error: {e}"),
            )
        })?
        .map_err(|e| {
            (
                StatusCode::NOT_FOUND,
                format!("failed to read artifact: {e}"),
            )
        })?;

    // Step 8: determine Content-Type by extension.
    let content_type = mime_by_extension(&canonical_full);

    // Step 9: build response.
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .body(Body::from(bytes))
        .expect("response builder infallible"))
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

/// Returns `true` if `s` is a safe session ID: `^[a-zA-Z0-9_-]{1,128}$`.
///
/// No dots, no slashes, no percent signs — these are decoded by axum's router
/// before reaching the handler, so a raw `..` or URL-encoded form would decode
/// to a component-separator or traversal that we must still catch.
#[cfg(feature = "web")]
fn is_safe_path_component(s: &str) -> bool {
    if s.is_empty() || s.len() > 128 {
        return false;
    }
    s.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

/// Returns `true` if `name` is a safe artifact file name.
///
/// Rules:
/// - Non-empty, max 255 chars (filesystem limit).
/// - Allowed chars: alphanumeric + `_`, `-`, `.` (for extensions).
/// - Must NOT be `..` or start with `.` (hidden files / traversal).
/// - Must NOT contain `/` or `\` (directory separators).
///
/// Note: axum decodes percent-encoding before routing, so `%2F` would arrive
/// here as `/`, which is rejected by the character whitelist.
#[cfg(feature = "web")]
fn is_safe_artifact_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 255 {
        return false;
    }
    // Reject pure-dot names and names starting with dot.
    if name == "." || name == ".." || name.starts_with('.') {
        return false;
    }
    // Allowed: alphanumeric, underscore, dash, dot.
    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
}

// ---------------------------------------------------------------------------
// MIME sniff by extension
// ---------------------------------------------------------------------------

/// Sniff MIME type by file extension (same table as generate.rs).
#[cfg(feature = "web")]
fn mime_by_extension(path: &std::path::Path) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "html" | "htm" => "text/html; charset=utf-8",
        "js" | "mjs" => "application/javascript; charset=utf-8",
        "py" => "text/x-python; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "txt" | "md" => "text/plain; charset=utf-8",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "svg" => "image/svg+xml",
        "ts" | "tsx" => "text/typescript; charset=utf-8",
        _ => "application/octet-stream",
    }
}

// ---------------------------------------------------------------------------
// Workspace resolution
// ---------------------------------------------------------------------------

/// Resolve the TuringOS workspace directory.
/// Resolution order: TURINGOS_WEB_WORKSPACE → current_dir.
#[cfg(feature = "web")]
fn resolve_workspace() -> String {
    if let Ok(v) = std::env::var("TURINGOS_WEB_WORKSPACE") {
        if !v.is_empty() {
            return v;
        }
    }
    std::env::current_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| ".".to_string())
}

// ---------------------------------------------------------------------------
// Unit tests (no I/O)
// ---------------------------------------------------------------------------

#[cfg(all(feature = "web", test))]
mod tests {
    use super::*;

    #[test]
    fn safe_path_component_accepts_valid() {
        assert!(is_safe_path_component("session-01"));
        assert!(is_safe_path_component("1716000000_3f8a1b2c"));
        assert!(is_safe_path_component(&"a".repeat(128)));
    }

    #[test]
    fn safe_path_component_rejects_dot() {
        assert!(!is_safe_path_component("."));
        assert!(!is_safe_path_component(".."));
        assert!(!is_safe_path_component("a/b"));
        assert!(!is_safe_path_component("a.b"));
        assert!(!is_safe_path_component(""));
        assert!(!is_safe_path_component(&"a".repeat(129)));
    }

    #[test]
    fn safe_artifact_name_accepts_valid() {
        assert!(is_safe_artifact_name("index.html"));
        assert!(is_safe_artifact_name("main.py"));
        assert!(is_safe_artifact_name("app.js"));
        assert!(is_safe_artifact_name("style.css"));
        assert!(is_safe_artifact_name("file-name_v2.txt"));
    }

    #[test]
    fn safe_artifact_name_rejects_traversal() {
        assert!(!is_safe_artifact_name(".."));
        assert!(!is_safe_artifact_name(".hidden"));
        assert!(!is_safe_artifact_name("a/b"));
        assert!(!is_safe_artifact_name("a\\b"));
        assert!(!is_safe_artifact_name(""));
        assert!(!is_safe_artifact_name(&"a".repeat(256)));
    }

    #[test]
    fn safe_artifact_name_rejects_slash_in_name() {
        // After URL decode, `%2F` becomes `/` — must be rejected.
        assert!(!is_safe_artifact_name("a/b.html"));
        assert!(!is_safe_artifact_name("../../etc/passwd"));
    }

    #[test]
    fn mime_html() {
        assert_eq!(
            mime_by_extension(std::path::Path::new("index.html")),
            "text/html; charset=utf-8"
        );
    }

    #[test]
    fn mime_unknown() {
        assert_eq!(
            mime_by_extension(std::path::Path::new("foo.bin")),
            "application/octet-stream"
        );
    }
}
