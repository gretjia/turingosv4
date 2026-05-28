//! TRACE_MATRIX FC1-N5: Phase 7 web — GET /api/progress/by-session/:session_id
//!
//! Read-only projection over the per-session DERIVED-EVIDENCE progress stream
//! `<workspace>/sessions/<session_id>/generate_progress.jsonl`, appended by
//! `turingos generate` at each stage boundary (worker_start / worker_done /
//! market_settled).
//!
//! **Truth boundary (Karpathy / Art. 0.4)**: this stream is NOT canonical. It
//! carries wall-clock timestamps and is one-directional UI evidence — the
//! authoritative node tree comes from `/api/market/by-session` (ChainTape
//! replay). This handler NEVER feeds economic / replay / market-state logic;
//! the committed tree replaces these markers once a stage settles. An absent
//! file is `200` with empty `events` (generation has not started writing yet),
//! not an error.

use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

use super::ws::AppState;

const GENERATE_PROGRESS_JSONL: &str = "generate_progress.jsonl";

/// One progress marker, mirroring the JSON shape written by
/// `cmd_generate::append_generate_progress`. Derived evidence only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ProgressEvent {
    pub session_id: String,
    /// "worker_start" | "worker_done" | "market_settled"
    pub stage: String,
    /// e.g. "worker-alpha" | "worker-beta" | "worker-gamma" | "market"
    pub agent: String,
    #[serde(default)]
    pub artifact_cid: Option<String>,
    #[serde(default)]
    pub t_unix_ms: u64,
}

fn is_safe_session_id(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 128
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// GET /api/progress/by-session/:session_id — derived-evidence progress feed.
pub async fn progress_view_handler(
    Path(session_id): Path<String>,
    State(_state): State<AppState>,
) -> Response {
    if !is_safe_session_id(&session_id) {
        return (
            StatusCode::BAD_REQUEST,
            [(header::CONTENT_TYPE, "application/json")],
            r#"{"error":"invalid session_id"}"#.to_string(),
        )
            .into_response();
    }
    let workspace = super::welcome::resolve_workspace_path();
    let path = workspace
        .join("sessions")
        .join(&session_id)
        .join(GENERATE_PROGRESS_JSONL);

    // Defensive line-by-line parse: malformed lines are skipped, never fatal.
    let events: Vec<ProgressEvent> = match std::fs::read_to_string(&path) {
        Ok(text) => text
            .lines()
            .filter(|l| !l.trim().is_empty())
            .filter_map(|l| serde_json::from_str::<ProgressEvent>(l).ok())
            .collect(),
        Err(_) => Vec::new(),
    };

    let body = serde_json::json!({
        "session_id": session_id,
        "events": events,
    });
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        body.to_string(),
    )
        .into_response()
}
