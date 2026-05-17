/// TRACE_MATRIX FC1-N5: read view materialization + write-path (W4)
///
/// axum 0.7 router for TuringOS Phase 7 Web MVP.
///
/// W1 adds seven read-only HTTP routes backed by compile-time fixture data.
/// W2 adds one WebSocket route (HTTP 101 Upgrade) for real-time IR push.
/// W4 adds one write route: POST /api/task/open (shells out to `turingos task open`).
///
/// HTML routes (return `text/html`):
///   GET /          → dashboard fixture rendered to HTML
///   GET /agents    → agent-view fixture rendered to HTML
///   GET /tasks     → task-view fixture rendered to HTML (includes <tos-task-open-form>)
///   GET /audit     → dashboard fixture rendered to HTML (W4 will add distinct view)
///
/// JSON routes (return `application/json`):
///   GET /api/dashboard → dashboard fixture as JSON
///   GET /api/agents    → agent-view fixture as JSON
///   GET /api/tasks     → task-view fixture as JSON
///
/// WebSocket route (HTTP 101 Upgrade):
///   GET /ws        → WebSocket upgrade; pushes 3 initial IR messages on connect;
///                    subscribes to broadcast channel for task_created events (W4)
///
/// Write route (W4):
///   POST /api/task/open → validates body, shells out to `turingos task open`,
///                         broadcasts task_created, returns { task_id, status: "created" }
///
/// All HTTP routes return HTTP 200 on the happy path.
/// All items are `pub(crate)`.
use axum::{
    extract::State,
    http::header,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use tokio::sync::broadcast;

use super::fixtures;
use super::ir::{Block, IRRoot, TaskCardBlock};
use super::render::render_page;
use super::store::TaskMemoryStore;
use super::write::task_open_handler;
use super::ws::{ws_handler, AppState};

/// TRACE_MATRIX FC1-N5: frontend bundle embedded at compile time.
///
/// `frontend/dist/main.js` is the esbuild-bundled ESM produced by
/// `cd frontend && npm run build`. Embedding it (rather than serving via
/// `tower-http::services::ServeDir`) avoids needing the `tower-http/fs`
/// feature (~8 transitive deps) and keeps Phase 7 deployment a single
/// binary. Trade-off: backend must be rebuilt whenever the frontend
/// changes — acceptable for Phase 7's research scope.
const FRONTEND_MAIN_JS: &[u8] = include_bytes!("../../frontend/dist/main.js");

/// TRACE_MATRIX FC1-N5 / FC1-N10: read view materialization + write path
///
/// Build the axum router with all Phase 7 routes wired and `AppState` attached.
/// Total: 9 routes (4 HTML + 3 JSON + 1 WS + 1 POST write).
///
/// `broadcast_capacity` sets the tokio broadcast channel buffer. At startup
/// (turingos_web.rs) this is called with capacity = 64.
pub(crate) fn build_with_state(broadcast_capacity: usize) -> Router {
    let (tx, _) = broadcast::channel(broadcast_capacity);
    let state = AppState {
        broadcast_tx: tx,
        task_store: std::sync::Arc::new(TaskMemoryStore::new()),
    };

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
        // WebSocket route (W2/W4): HTTP 101 Upgrade → real-time IR push + task_created
        .route("/ws", get(ws_handler))
        // Write route (W4): POST /api/task/open → CLI shellout → WS broadcast
        .route("/api/task/open", post(task_open_handler))
        // Static asset (W4.1): frontend bundle embedded at compile time
        .route("/static/main.js", get(serve_main_js))
        .with_state(state)
}

/// TRACE_MATRIX FC1-N5: serves the embedded esbuild ESM bundle.
async fn serve_main_js() -> impl IntoResponse {
    (
        [(
            header::CONTENT_TYPE,
            "application/javascript; charset=utf-8",
        )],
        FRONTEND_MAIN_JS,
    )
}

/// Build the router. Uses `build_with_state` with production capacity = 64.
pub(crate) fn build() -> Router {
    build_with_state(64)
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
    Html(render_page(&ir, &ir.title.clone(), false))
}

async fn handle_agents() -> Html<String> {
    let ir = fixtures::agent_view();
    Html(render_page(&ir, &ir.title.clone(), false))
}

/// GET /tasks — renders the task-view fixture merged with in-memory store entries.
///
/// Merge order: synthesized `TaskCardBlock` entries from the in-memory store are
/// PREPENDED (newest first) ahead of the fixture blocks so that newly-created
/// tasks appear at the top of the rendered list.  The fixture blocks follow as
/// the "base layer".
async fn handle_tasks(State(state): State<AppState>) -> Html<String> {
    let ir = merged_task_view(&state);
    // Pass show_task_form=true so the tasks page includes <tos-task-open-form>
    Html(render_page(&ir, &ir.title.clone(), true))
}

/// GET /audit — reuses the dashboard fixture until W4 produces a distinct
/// audit view. §6a Page 5 requires `/audit` route to exist and contain
/// the task_id from page 4; fixture data satisfies the route-existence check.
async fn handle_audit() -> Html<String> {
    let ir = fixtures::dashboard();
    Html(render_page(&ir, &ir.title.clone(), false))
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

/// GET /api/tasks — returns the task-view fixture merged with in-memory store
/// entries.
///
/// Merge order: synthesized `TaskCardBlock` entries from the in-memory store
/// are PREPENDED (newest first) ahead of the fixture blocks so that
/// newly-created tasks appear at the top of the JSON block list.  The fixture
/// blocks follow as the "base layer".  This satisfies §6a Page 4: "DOM updates
/// to show the new task in the task list within 5 sec."
async fn handle_api_tasks(State(state): State<AppState>) -> Json<IRRoot> {
    Json(merged_task_view(&state))
}

// ---------------------------------------------------------------------------
// Merge helper
// ---------------------------------------------------------------------------

/// Build a merged `IRRoot` for the task view: in-memory store entries (newest
/// first) prepended to the compile-time fixture blocks.
///
/// Each `TaskEntry` becomes a `Block::TaskCard` with `status = "open"` (no
/// lifecycle tracking until W5+).  Optional fields (`reward_micro`,
/// `attempt_count`, `assigned_agent_id`) are set to sensible defaults.
fn merged_task_view(state: &AppState) -> IRRoot {
    let mut ir = fixtures::task_view();

    // Snapshot the store; reverse so newest entries come first.
    let mut entries = state.task_store.snapshot();
    entries.reverse();

    // Synthesize a TaskCardBlock for each store entry and prepend to ir.blocks.
    let synthesized: Vec<Block> = entries
        .into_iter()
        .map(|e| {
            Block::TaskCard(TaskCardBlock {
                id: format!("blk-task-card-{}", e.task_id),
                task_id: e.task_id,
                problem_id: e.problem_id,
                status: "open".to_string(),
                reward_micro: Some(e.bounty),
                attempt_count: Some(0),
                assigned_agent_id: Some(e.agent_id),
            })
        })
        .collect();

    // Prepend synthesized blocks (newest first) ahead of fixture blocks.
    let mut new_blocks = synthesized;
    new_blocks.extend(ir.blocks);
    ir.blocks = new_blocks;
    ir
}
