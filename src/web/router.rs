/// TRACE_MATRIX FC1-N5: read view materialization
///
/// axum 0.7 router for TuringOS Phase 7 Web MVP.
///
/// W1 adds seven read-only HTTP routes backed by compile-time fixture data.
/// W2 adds one WebSocket route (HTTP 101 Upgrade) for real-time IR push.
///
/// HTML routes (return `text/html`):
///   GET /          → dashboard fixture rendered to HTML
///   GET /agents    → agent-view fixture rendered to HTML
///   GET /tasks     → task-view fixture rendered to HTML
///   GET /audit     → dashboard fixture rendered to HTML (W4 will add distinct view)
///
/// JSON routes (return `application/json`):
///   GET /api/dashboard → dashboard fixture as JSON
///   GET /api/agents    → agent-view fixture as JSON
///   GET /api/tasks     → task-view fixture as JSON
///
/// WebSocket route (HTTP 101 Upgrade):
///   GET /ws        → WebSocket upgrade; pushes 3 initial IR messages on connect
///
/// All HTTP routes return HTTP 200 on the happy path.
/// All items are `pub(crate)`.
use axum::{response::Html, routing::get, Json, Router};

use super::fixtures;
use super::ir::IRRoot;
use super::render::render_page;
use super::ws::ws_handler;

/// TRACE_MATRIX FC1-N5: read view materialization + real-time push channel
///
/// Build the axum router with all Phase 7 W1 read routes and W2 WebSocket
/// route wired. Total: 8 routes (4 HTML + 3 JSON + 1 WS).
pub(crate) fn build() -> Router {
    Router::new()
        // HTML routes
        .route("/", get(handle_dashboard))
        .route("/agents", get(handle_agents))
        .route("/tasks", get(handle_tasks))
        .route("/audit", get(handle_audit))
        // JSON routes
        .route("/api/dashboard", get(handle_api_dashboard))
        .route("/api/agents", get(handle_api_agents))
        .route("/api/tasks", get(handle_api_tasks))
        // WebSocket route (W2): HTTP 101 Upgrade → real-time IR push
        .route("/ws", get(ws_handler))
}

/// Compatibility alias: W0 tests call `build_router()`; keep it working.
pub(crate) fn build_router() -> Router {
    build()
}

// ---------------------------------------------------------------------------
// HTML handlers
// ---------------------------------------------------------------------------

async fn handle_dashboard() -> Html<String> {
    let ir = fixtures::dashboard();
    Html(render_page(&ir, &ir.title.clone()))
}

async fn handle_agents() -> Html<String> {
    let ir = fixtures::agent_view();
    Html(render_page(&ir, &ir.title.clone()))
}

async fn handle_tasks() -> Html<String> {
    let ir = fixtures::task_view();
    Html(render_page(&ir, &ir.title.clone()))
}

/// GET /audit — reuses the dashboard fixture until W4 produces a distinct
/// audit view. §6a Page 5 requires `/audit` route to exist and contain
/// the task_id from page 4; fixture data satisfies the route-existence check.
async fn handle_audit() -> Html<String> {
    let ir = fixtures::dashboard();
    Html(render_page(&ir, &ir.title.clone()))
}

// ---------------------------------------------------------------------------
// JSON handlers
// ---------------------------------------------------------------------------

async fn handle_api_dashboard() -> Json<IRRoot> {
    Json(fixtures::dashboard())
}

async fn handle_api_agents() -> Json<IRRoot> {
    Json(fixtures::agent_view())
}

async fn handle_api_tasks() -> Json<IRRoot> {
    Json(fixtures::task_view())
}
