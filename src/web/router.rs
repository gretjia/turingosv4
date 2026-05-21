/// TRACE_MATRIX FC1-N5 / FC2-N16: read view materialization + write-path (W4/W5/W7)
///
/// axum 0.7 router for TuringOS Phase 7 Web MVP. 20 routes total.
///
/// W1 adds seven read-only HTTP routes backed by compile-time fixture data.
/// W2 adds one WebSocket route (HTTP 101 Upgrade) for real-time IR push.
/// W4 adds one write route: POST /api/task/open (shells out to `turingos task open`).
/// W5 adds four routes: spec questions + spec submit + generate + artifact serve.
/// W7 adds six routes: welcome HTML + status + 4 onboarding POSTs.
///
/// HTML routes (return `text/html`):
///   GET /          → dashboard fixture rendered to HTML
///   GET /agents    → agent-view fixture rendered to HTML
///   GET /tasks     → task-view fixture rendered to HTML (includes <tos-task-open-form>)
///   GET /audit     → dashboard fixture rendered to HTML
///   GET /build     → spec interview page (chrome + <tos-spec-grill> mount; W6)
///
/// JSON routes (return `application/json`):
///   GET /api/dashboard → dashboard fixture as JSON
///   GET /api/agents    → agent-view fixture as JSON
///   GET /api/tasks     → task-view fixture as JSON
///
/// WebSocket route (HTTP 101 Upgrade):
///   GET /ws        → WebSocket upgrade; pushes 3 initial IR messages on connect;
///                    subscribes to broadcast channel for task/spec/generate events
///
/// Write route (W4):
///   POST /api/task/open → validates body, shells out to `turingos task open`,
///                         broadcasts task_created, returns { task_id, status: "created" }
///
/// Static asset (W4.1):
///   GET /static/main.js → embedded esbuild frontend bundle
///
/// Spec interview routes (W5):
///   GET  /api/spec/questions → returns the 8 canonical Zh interview questions
///   POST /api/spec/submit    → accepts 8 answers, shells out to `turingos spec`,
///                              returns { session_id, spec_md, capsule_cid }
///
/// Generate route (W5):
///   POST /api/generate → accepts session_id, shells out to `turingos generate`,
///                        returns { session_id, artifacts: [{ path, size_bytes, content_type }] }
///
/// Artifact serve route (W5):
///   GET /api/artifact/:session_id/:name → serves one artifact file with Content-Type
///
/// All HTTP routes return HTTP 200 on the happy path.
/// All items are `pub(crate)`.
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use tokio::sync::broadcast;

use super::artifact::artifact_get_handler;
use super::artifact_bundle::artifact_bundle_get_handler;
use super::preview::preview_get_handler;
use super::build_session::build_session_handler;
use super::fixtures;
use super::generate::generate_handler;
use super::ir::{Block, IRRoot, TaskCardBlock};
use super::render::{render_build_page, render_page_with_view, render_welcome_page, ViewKind};
use super::spec::{spec_questions_handler, spec_submit_handler, spec_turn_handler};
use super::store::TaskMemoryStore;
use super::welcome::{
    welcome_agent_deploy_handler, welcome_init_handler, welcome_llm_config_handler,
    welcome_set_api_key_handler, welcome_status_handler, NextStep,
};
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

/// TRACE_MATRIX FC1-N5 / FC1-N10 / FC2-N16: read view materialization + write path
///
/// Build the axum router with all Phase 7 routes wired and `AppState` attached.
/// Total: 14 routes
///   4 HTML  (W0/W1): /, /agents, /tasks, /audit
///   1 HTML  (W6): /build (spec-grill interview centerpiece)
///   3 JSON  (W1): /api/dashboard, /api/agents, /api/tasks
///   1 WS    (W2): /ws
///   1 POST  (W4): /api/task/open
///   1 GET   (W4.1): /static/main.js
///   2 spec  (W5): GET /api/spec/questions, POST /api/spec/submit
///   1 gen   (W5): POST /api/generate
///   1 art   (W5): GET /api/artifact/:session_id/:name
///
/// `broadcast_capacity` sets the tokio broadcast channel buffer. At startup
/// (turingos_web.rs) this is called with capacity = 64.
pub(crate) fn build_with_state(broadcast_capacity: usize) -> Router {
    let (tx, _) = broadcast::channel(broadcast_capacity);
    let state = AppState {
        broadcast_tx: tx,
        task_store: std::sync::Arc::new(TaskMemoryStore::new()),
        api_key: std::sync::Arc::new(std::sync::Mutex::new(None)),
        sessions: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
    };

    Router::new()
        // HTML routes (W0/W1)
        // W7: `/` redirects to /welcome whenever onboarding is incomplete.
        .route("/", get(handle_root_redirect))
        .route("/agents", get(handle_agents))
        .route("/tasks", get(handle_tasks))
        .route("/audit", get(handle_audit))
        // W6: spec interview centerpiece (chrome + <tos-spec-grill> mount)
        .route("/build", get(handle_build))
        // W7: welcome wizard page chrome + <tos-welcome> mount.
        .route("/welcome", get(handle_welcome_page))
        // JSON routes (W1)
        .route("/api/dashboard", get(handle_api_dashboard))
        .route("/api/agents", get(handle_api_agents))
        .route("/api/tasks", get(handle_api_tasks))
        // WebSocket route (W2/W4): HTTP 101 Upgrade → real-time IR push + broadcasts
        .route("/ws", get(ws_handler))
        // Write route (W4): POST /api/task/open → CLI shellout → WS broadcast
        .route("/api/task/open", post(task_open_handler))
        // Static asset (W4.1): frontend bundle embedded at compile time
        .route("/static/main.js", get(serve_main_js))
        // Spec interview routes (W5): GET questions + POST submit
        .route("/api/spec/questions", get(spec_questions_handler))
        .route("/api/spec/submit", post(spec_submit_handler))
        // Phase 6.3.x driven-mode grill turn route (W7)
        .route("/api/spec/turn", post(spec_turn_handler))
        // Generate route (W5): POST → CLI shellout → artifacts list + WS broadcast
        .route("/api/generate", post(generate_handler))
        // Artifact serve route (W5): GET one artifact file with Content-Type
        .route("/api/artifact/:session_id/:name", get(artifact_get_handler))
        // CAS-backed bundle file serve route (C5): GET one artifact file from CAS
        .route("/api/bundle/:artifact_bundle_cid/file", get(artifact_bundle_get_handler))
        // CAS-backed preview run file serve route (C6): GET one preview file
        .route("/api/preview/:artifact_bundle_cid/file", get(preview_get_handler))
        // C7: GET build session view
        .route("/api/build/session/:session_id", get(build_session_handler))
        // W7: welcome onboarding API surface (5 endpoints; in-memory API key)
        .route("/api/welcome/status", get(welcome_status_handler))
        .route("/api/welcome/api-key", post(welcome_set_api_key_handler))
        .route("/api/welcome/init", post(welcome_init_handler))
        .route("/api/welcome/llm-config", post(welcome_llm_config_handler))
        .route(
            "/api/welcome/agent-deploy",
            post(welcome_agent_deploy_handler),
        )
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
///
/// TRACE_MATRIX FC2-N16: Phase 7 web — build axum Router (production entry).
pub(crate) fn build() -> Router {
    build_with_state(64)
}

/// Compatibility alias: W0 tests call `build_router()`; keep it working.
///
/// TRACE_MATRIX FC2-N16: Phase 7 web — build_router alias for W0-era tests.
pub(crate) fn build_router() -> Router {
    build()
}

// ---------------------------------------------------------------------------
// HTML handlers
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC2-N16: W7 — GET / dispatches to /welcome when the user has
/// not finished onboarding, else renders the dashboard. We compute next_step
/// using the same inspection logic that backs `/api/welcome/status` so the
/// redirect decision can't drift from the wizard's own state machine.
async fn handle_root_redirect(State(state): State<AppState>) -> axum::response::Response {
    let workspace = super::welcome::resolve_workspace_path();
    let api_key_set = state
        .api_key
        .lock()
        .ok()
        .and_then(|g| g.as_ref().map(|s| !s.is_empty()))
        .unwrap_or(false);
    let next = super::welcome::next_step_for(&workspace, api_key_set);
    if !matches!(next, NextStep::Done) {
        return Redirect::to("/welcome").into_response();
    }
    let ir = fixtures::dashboard();
    let html = render_page_with_view(&ir, &ir.title.clone(), false, ViewKind::Dashboard);
    (StatusCode::OK, Html(html)).into_response()
}

/// W7: server-rendered welcome page chrome + <tos-welcome> mount.
async fn handle_welcome_page() -> Html<String> {
    Html(render_welcome_page())
}

async fn handle_agents() -> Html<String> {
    let ir = fixtures::agent_view();
    Html(render_page_with_view(
        &ir,
        &ir.title.clone(),
        false,
        ViewKind::Agents,
    ))
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
    Html(render_page_with_view(
        &ir,
        &ir.title.clone(),
        true,
        ViewKind::Tasks,
    ))
}

/// GET /audit — reuses the dashboard fixture until W4 produces a distinct
/// audit view. §6a Page 5 requires `/audit` route to exist and contain
/// the task_id from page 4; fixture data satisfies the route-existence check.
async fn handle_audit() -> Html<String> {
    let ir = fixtures::dashboard();
    Html(render_page_with_view(
        &ir,
        &ir.title.clone(),
        false,
        ViewKind::Audit,
    ))
}

/// TRACE_MATRIX FC1-N5 + FC1-N10: Phase 7 W6 — GET /build handler.
///
/// Renders the chrome + `<tos-spec-grill>` placeholder. The Web Component
/// fetches `/api/spec/questions`, walks the user through 8 questions, posts
/// `/api/spec/submit`, then `/api/generate`, then previews artifacts via the
/// sibling `<tos-artifact-viewer>` component. No IR is materialised on the
/// server for this page; the interview is fully client-orchestrated.
async fn handle_build() -> Html<String> {
    Html(render_build_page())
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
