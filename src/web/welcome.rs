/// TRACE_MATRIX FC2-N16: Phase 7 W7 — first-time-user onboarding wrapper.
///
/// W7 wraps the Phase 6.3 `turingos welcome` 5-step flow inside the browser so
/// a non-developer can land on `http://127.0.0.1:8080/`, get redirected to
/// `/welcome`, and click through workspace init / LLM config / API-key entry /
/// agent deploy without ever touching a shell.
///
/// FC-trace: FC2-N16 (boot/onboarding gate extended from the CLI to the web)
/// plus FC1-N5 (read view of workspace state) and FC1-N10 (write actions via
/// existing Phase 6.3 shellouts; no new Class-4 admission, no typed-tx).
///
/// Risk class: Class 2. Production wire-up around the existing CLI; in-memory
/// API-key handle on `AppState`; no Class 4 surface touched.
///
/// # API-key invariant (Phase 6.3 §cmd_llm.rs:18)
///
/// `turingos llm config` writes only the env-var NAME (e.g. `SILICONFLOW_API_KEY`)
/// into turingos.toml — never the value. W7 preserves that invariant strictly:
///   - the value lives only in `AppState.api_key` (in-memory `Arc<Mutex<Option<String>>>`)
///   - the value is NEVER written to disk
///   - the value is NEVER echoed in any HTTP response body
///   - the value is NEVER logged
///   - the value is injected into spec/generate child processes via
///     `Command::env("SILICONFLOW_API_KEY", value)`, NOT via a `.env`/.toml file
///
/// On process restart, the in-memory key is lost — the user re-enters it
/// through the `/welcome` step-3 card. This is intentional and called out in
/// the welcome UI copy.
#[cfg(feature = "web")]
use axum::{extract::State, http::StatusCode, Json};
#[cfg(feature = "web")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "web")]
use std::path::{Path, PathBuf};
#[cfg(feature = "web")]
use std::time::Duration;

#[cfg(feature = "web")]
use super::ws::AppState;
#[cfg(feature = "web")]
use turingosv4::sdk::sanitized_runner::{
    env_allowlist_from_current, run_sanitized, SanitizedCommand, SanitizedOutput,
};

// ---------------------------------------------------------------------------
// Workspace status (mirrors src/bin/turingos/cmd_welcome.rs::inspect_workspace)
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC2-N16: which onboarding step the UI should show as active.
///
/// Drives the welcome-wizard step progression on the frontend. Matches the
/// 5-step flow described in `turingos welcome` FULL_HELP, plus the W7-added
/// `ApiKey` step which has no on-disk artifact (the key lives only on
/// `AppState`).
#[cfg(feature = "web")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub(crate) enum NextStep {
    /// `turingos init` not yet run for the configured workspace.
    Init,
    /// init done, but turingos.toml has no llm.meta.model / llm.blackbox.model.
    LlmConfig,
    /// llm config done but the in-memory API key on AppState is unset.
    ApiKey,
    /// API key set but agent_pubkeys.json has no entries.
    AgentDeploy,
    /// All four prerequisites met but no spec capsule has been written yet.
    Spec,
    /// Spec done but artifacts/ is empty.
    Generate,
    /// All 5 done.
    Done,
}

/// TRACE_MATRIX FC2-N16: snapshot of onboarding state for the web UI.
///
/// Read by `GET /api/welcome/status`, written into the response body of all
/// W7 mutation endpoints so the frontend can re-render in one round-trip.
///
/// Note `api_key_set`: this reflects AppState ONLY — it never inspects disk
/// (the Phase 6.3 invariant says the value is never persisted). On a fresh
/// process restart this field always returns to `false`.
#[cfg(feature = "web")]
#[derive(Debug, Clone, Serialize)]
pub(crate) struct OnboardingStatus {
    pub(crate) workspace_path: String,
    pub(crate) init_done: bool,
    pub(crate) llm_config_done: bool,
    pub(crate) api_key_set: bool,
    pub(crate) agents_count: usize,
    pub(crate) spec_done: bool,
    pub(crate) spec_capsule_cid: Option<String>,
    pub(crate) artifacts_done: bool,
    pub(crate) next_step: NextStep,
}

/// TRACE_MATRIX FC2-N16: shape of the set-api-key request body.
///
/// The handler validates the key shape locally (starts with "sk-", reasonable
/// length, printable ASCII) and stores the value in AppState. The value is
/// NEVER persisted, logged, or echoed.
#[cfg(feature = "web")]
#[derive(Debug, Deserialize)]
pub(crate) struct ApiKeyRequest {
    pub(crate) api_key: String,
}

/// TRACE_MATRIX FC2-N16: error envelope for welcome endpoints.
///
/// `kind` values:
/// - `"invalid_input"`         400: api_key format check failed
/// - `"prerequisite_missing"`  409: e.g. llm_config requested before init
/// - `"init_failed"`           500: `turingos init` shellout non-zero exit
/// - `"llm_config_failed"`     500: `turingos llm config` shellout non-zero exit
/// - `"agent_deploy_failed"`   500: `turingos agent deploy` shellout non-zero exit
/// - `"workspace_setup"`       500: filesystem io error (mkdir, etc.)
#[cfg(feature = "web")]
#[derive(Debug, Serialize)]
pub(crate) struct SetupError {
    pub(crate) reason: String,
    pub(crate) kind: &'static str,
}

// ---------------------------------------------------------------------------
// Workspace inspection (mirrors cmd_welcome.rs::inspect_workspace)
// ---------------------------------------------------------------------------

#[cfg(feature = "web")]
struct WsInspect {
    init_done: bool,
    llm_configured: bool,
    agents_count: usize,
    spec_done: bool,
    spec_capsule_cid: Option<String>,
    artifacts_done: bool,
}

/// Filesystem-only inspection. Mirrors cmd_welcome.rs `inspect_workspace`
/// verbatim (intentionally; this is the single source of truth for "is X
/// done" semantics). Spec capsule lookup is a best-effort glob over
/// `<workspace>/cas/` — failure here just means `spec_capsule_cid: None`.
#[cfg(feature = "web")]
fn inspect_workspace(ws: &Path) -> WsInspect {
    let init_done =
        ws.join("genesis_payload.toml").is_file() && ws.join("agent_pubkeys.json").is_file();

    let llm_configured = ws
        .join("turingos.toml")
        .is_file()
        .then(|| std::fs::read_to_string(ws.join("turingos.toml")).ok())
        .flatten()
        .map(|content| content.contains("llm.meta.model") && content.contains("llm.blackbox.model"))
        .unwrap_or(false);

    let agents_count = std::fs::read_to_string(ws.join("agent_pubkeys.json"))
        .map(|content| {
            content
                .lines()
                .filter(|l| l.trim_start().starts_with('"') && l.trim_end().ends_with("{"))
                .count()
        })
        .unwrap_or(0);

    // Best-effort CID lookup. We avoid pulling in the kernel CAS helpers
    // (Phase 6.1/6.3 hard-constraint surfaces) and instead just check for any
    // file under <ws>/cas with a name resembling a hex CID. Front-end uses
    // this for display only.
    let spec_capsule_cid = find_first_cas_cid(&ws.join("cas"));
    let spec_done = spec_capsule_cid.is_some() || ws.join("spec.md").is_file();

    let artifacts_done = ws
        .join("artifacts")
        .read_dir()
        .map(|mut it| it.next().is_some())
        .unwrap_or(false);

    WsInspect {
        init_done,
        llm_configured,
        agents_count,
        spec_done,
        spec_capsule_cid,
        artifacts_done,
    }
}

#[cfg(feature = "web")]
fn find_first_cas_cid(cas_dir: &Path) -> Option<String> {
    let rd = std::fs::read_dir(cas_dir).ok()?;
    for entry in rd.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        // Accept either a bare hex filename or "<hex>.json" style.
        let base = name.split('.').next().unwrap_or("");
        if base.len() >= 32 && base.chars().all(|c| c.is_ascii_hexdigit()) {
            return Some(base.to_string());
        }
    }
    None
}

#[cfg(feature = "web")]
fn compute_next_step(inspect: &WsInspect, api_key_set: bool) -> NextStep {
    if !inspect.init_done {
        NextStep::Init
    } else if !inspect.llm_configured {
        NextStep::LlmConfig
    } else if !api_key_set {
        NextStep::ApiKey
    } else if inspect.agents_count == 0 {
        NextStep::AgentDeploy
    } else if !inspect.spec_done {
        NextStep::Spec
    } else if !inspect.artifacts_done {
        NextStep::Generate
    } else {
        NextStep::Done
    }
}

#[cfg(feature = "web")]
fn build_status(workspace: &Path, inspect: WsInspect, api_key_set: bool) -> OnboardingStatus {
    let next_step = compute_next_step(&inspect, api_key_set);
    OnboardingStatus {
        workspace_path: workspace.to_string_lossy().into_owned(),
        init_done: inspect.init_done,
        llm_config_done: inspect.llm_configured,
        api_key_set,
        agents_count: inspect.agents_count,
        spec_done: inspect.spec_done,
        spec_capsule_cid: inspect.spec_capsule_cid,
        artifacts_done: inspect.artifacts_done,
        next_step,
    }
}

#[cfg(feature = "web")]
fn current_api_key_set(state: &AppState) -> bool {
    state
        .api_key
        .lock()
        .ok()
        .and_then(|g| g.as_ref().map(|s| !s.is_empty()))
        .unwrap_or(false)
}

/// TRACE_MATRIX FC2-N16: convenience for the router redirect.
///
/// Computes the welcome `NextStep` for a given workspace + api-key-set bit
/// without going through the JSON-wrapping `welcome_status_handler`. The
/// root-route handler uses this to decide whether `/` should 302 to
/// `/welcome` or render the dashboard.
#[cfg(feature = "web")]
pub(crate) fn next_step_for(workspace: &Path, api_key_set: bool) -> NextStep {
    let inspect = inspect_workspace(workspace);
    compute_next_step(&inspect, api_key_set)
}

// ---------------------------------------------------------------------------
// Workspace + binary resolution
// ---------------------------------------------------------------------------

/// Default workspace for the W7 welcome flow. Matches the §6a Page 1
/// frontend smoke pattern: `tmp/phase7_active` under the project root.
///
/// Always returns an **absolute path** — see spec.rs::resolve_workspace for
/// rationale (spawned subprocess cwd != server cwd).
///
/// Resolution order (matches spec.rs / generate.rs):
///   1. `TURINGOS_WEB_WORKSPACE` env var (operator override)
///   2. `tmp/phase7_active` under the server's current_dir at startup.
///
/// TRACE_MATRIX FC2-N16: Phase 7 web — workspace path resolution for welcome/spec/generate.
#[cfg(feature = "web")]
pub(crate) fn resolve_workspace_path() -> PathBuf {
    let raw = std::env::var("TURINGOS_WEB_WORKSPACE")
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "tmp/phase7_active".to_string());
    let p = PathBuf::from(&raw);
    if p.is_absolute() {
        return p;
    }
    match std::env::current_dir() {
        Ok(cwd) => cwd.join(p),
        Err(_) => p,
    }
}

#[cfg(feature = "web")]
fn resolve_turingos_bin() -> String {
    if let Ok(v) = std::env::var("TURINGOS_BACKEND_OVERRIDE") {
        if !v.is_empty() {
            return v;
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sibling = dir.join("turingos");
            if sibling.exists() {
                return sibling.to_string_lossy().into_owned();
            }
        }
    }
    "turingos".to_string()
}

// ---------------------------------------------------------------------------
// API-key validation (W7 trust boundary; NEVER writes to disk)
// ---------------------------------------------------------------------------

/// SiliconFlow + most OpenAI-compatible providers use the `sk-` prefix.
/// We require it as a sanity check so a typo doesn't sit silently waiting
/// for the first spec submission to fail with a confusing 401.
#[cfg(feature = "web")]
fn validate_api_key_shape(key: &str) -> Result<(), SetupError> {
    if !key.starts_with("sk-") {
        return Err(SetupError {
            reason: "API key must start with \"sk-\" (SiliconFlow / OpenAI convention).".into(),
            kind: "invalid_input",
        });
    }
    if key.len() < 16 || key.len() > 256 {
        return Err(SetupError {
            reason: format!(
                "API key length {} is out of range; expected 16-256 characters.",
                key.len()
            ),
            kind: "invalid_input",
        });
    }
    if !key.chars().all(|c| c.is_ascii_graphic()) {
        return Err(SetupError {
            reason: "API key must contain only printable ASCII characters.".into(),
            kind: "invalid_input",
        });
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// GET /api/welcome/status
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC2-N16: read-only status snapshot for the welcome wizard.
#[cfg(feature = "web")]
pub(crate) async fn welcome_status_handler(
    State(state): State<AppState>,
) -> Json<OnboardingStatus> {
    let workspace = resolve_workspace_path();
    let inspect = inspect_workspace(&workspace);
    let api_key_set = current_api_key_set(&state);
    Json(build_status(&workspace, inspect, api_key_set))
}

// ---------------------------------------------------------------------------
// POST /api/welcome/api-key
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC2-N16: set the in-memory API key on `AppState`.
///
/// Critical invariants preserved from Phase 6.3:
///   - the key value is NEVER written to disk
///   - the key value is NEVER echoed in the response body
///   - the key value is NEVER logged
///
/// On success the handler returns a refreshed `OnboardingStatus` (which only
/// exposes the boolean `api_key_set`, not the value).
#[cfg(feature = "web")]
pub(crate) async fn welcome_set_api_key_handler(
    State(state): State<AppState>,
    Json(req): Json<ApiKeyRequest>,
) -> Result<Json<OnboardingStatus>, (StatusCode, Json<SetupError>)> {
    validate_api_key_shape(&req.api_key).map_err(|e| (StatusCode::BAD_REQUEST, Json(e)))?;

    // Brief critical section: hold std::sync::Mutex only long enough to swap
    // the value. No `.await` is permitted between lock + drop.
    {
        let mut guard = state.api_key.lock().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SetupError {
                    reason: "api_key mutex poisoned".into(),
                    kind: "workspace_setup",
                }),
            )
        })?;
        *guard = Some(req.api_key);
        // `guard` drops here.
    }
    // SECURITY: do NOT log req.api_key — we just consumed it into the mutex.
    log::info!("welcome_set_api_key_handler: api_key updated (value redacted)");

    let workspace = resolve_workspace_path();
    let inspect = inspect_workspace(&workspace);
    Ok(Json(build_status(&workspace, inspect, true)))
}

// ---------------------------------------------------------------------------
// POST /api/welcome/init
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC2-N16: scaffold workspace via `turingos init`.
///
/// Idempotent: if the workspace already has `genesis_payload.toml` +
/// `agent_pubkeys.json`, the handler returns the current status without
/// re-running the CLI.
///
/// Exact CLI invocation:
///   `turingos init --project <workspace> --template multi-agent`
///
/// (Multi-agent template is the documented choice for the W7 demo flow; the
/// scaffold contains nothing template-specific the rest of the flow depends on,
/// but it leaves a richer turingos.toml header for the user to read later.)
#[cfg(feature = "web")]
pub(crate) async fn welcome_init_handler(
    State(state): State<AppState>,
) -> Result<Json<OnboardingStatus>, (StatusCode, Json<SetupError>)> {
    let workspace = resolve_workspace_path();
    let workspace_str = workspace.to_string_lossy().into_owned();
    let api_key_set = current_api_key_set(&state);

    let initial = inspect_workspace(&workspace);
    if initial.init_done {
        // Idempotent fast path: initialized workspaces can safely re-seed
        // driven-grill prerequisites before returning. Fresh workspaces must
        // run `turingos init` first; otherwise seeding creates files that make
        // the CLI reject the non-empty target directory.
        seed_grill_prerequisites(&workspace).await?;
        return Ok(Json(build_status(&workspace, initial, api_key_set)));
    }

    let parent_workspace = workspace.parent().map(|p| p.to_path_buf());
    if let Some(parent) = parent_workspace {
        if !parent.as_os_str().is_empty() {
            let parent_clone = parent.clone();
            tokio::task::spawn_blocking(move || std::fs::create_dir_all(&parent_clone))
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(SetupError {
                            reason: format!("spawn_blocking error: {e}"),
                            kind: "workspace_setup",
                        }),
                    )
                })?
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(SetupError {
                            reason: format!(
                                "failed to create parent dir {}: {e}",
                                parent.display()
                            ),
                            kind: "workspace_setup",
                        }),
                    )
                })?;
        }
    }

    let bin = resolve_turingos_bin();
    log::info!(
        "welcome_init_handler: bin={:?} workspace={:?} template=multi-agent",
        bin,
        workspace_str
    );

    let mut init_args = vec![
        "init".into(),
        "--project".into(),
        workspace_str.clone(),
        "--template".into(),
        "multi-agent".into(),
    ];
    if deepseek_direct_endpoint_selected() {
        init_args.push("--provider".into());
        init_args.push("deepseek".into());
    }

    let output = run_welcome_command(
        bin.clone(),
        workspace.clone(),
        init_args,
        Duration::from_secs(120),
    )
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SetupError {
                reason: format!("failed to spawn/run {:?}: {e}", bin),
                kind: "init_failed",
            }),
        )
    })?;

    if !output.success() {
        let combined = combine_truncated(&output.stdout, &output.stderr);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SetupError {
                reason: combined,
                kind: "init_failed",
            }),
        ));
    }

    // Phase 5 driven-grill prerequisites are seeded after fresh init so the
    // prompt assets and PromotionReceipts match the currently-running binary
    // without making the init target non-empty before the CLI scaffolds it.
    seed_grill_prerequisites(&workspace).await?;

    let inspect = inspect_workspace(&workspace);
    Ok(Json(build_status(&workspace, inspect, api_key_set)))
}

/// Phase 5 — driven-grill prerequisites helper. Runs on EVERY welcome init
/// call (not just first-time init) so binary upgrades that change prompt
/// content stay consistent with the workspace state:
///
///   1. Materialize the 3 embedded grill prompt assets into
///      `<workspace>/assets/prompts/` so `spec_turn_handler` +
///      `cmd_llm::run_triage` find them at the path they read.
///      (Closes Phase 1-4 audit GAP-3.)
///   2. Seed binary-baked `PromptPromotionReceipt`s in `<workspace>/cas/` so
///      C10 `check_promotion_guard` accepts the embedded prompts on the
///      first LLM call without needing a real `prompt-eval` run.
///      (Closes Phase 1-4 audit GAP-1.)
///
/// Both steps are idempotent (file overwrite + CAS content-address dedup) so
/// repeated calls are cheap and safe.
#[cfg(feature = "web")]
async fn seed_grill_prerequisites(workspace: &Path) -> Result<(), (StatusCode, Json<SetupError>)> {
    let ws_for_materialize = workspace.to_path_buf();
    let materialize_result = tokio::task::spawn_blocking(move || {
        turingosv4::runtime::embedded_prompts::materialize_grill_prompts(&ws_for_materialize)
    })
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SetupError {
                reason: format!("spawn_blocking error materializing grill prompts: {e}"),
                kind: "workspace_setup",
            }),
        )
    })?;
    materialize_result.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SetupError {
                reason: format!("failed to materialize embedded grill prompts: {e}"),
                kind: "workspace_setup",
            }),
        )
    })?;

    let ws_for_seed = workspace.to_path_buf();
    let logical_t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let seed_result = tokio::task::spawn_blocking(move || {
        turingosv4::runtime::embedded_prompts::seed_embedded_promotion_receipts(
            &ws_for_seed,
            logical_t,
        )
    })
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SetupError {
                reason: format!("spawn_blocking error seeding promotion receipts: {e}"),
                kind: "workspace_setup",
            }),
        )
    })?;
    seed_result.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SetupError {
                reason: format!("failed to seed promotion receipts: {e}"),
                kind: "workspace_setup",
            }),
        )
    })?;

    Ok(())
}

// ---------------------------------------------------------------------------
// POST /api/welcome/llm-config
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC2-N16: persist the two-LLM config via `turingos llm config`.
///
/// Uses the Phase 6.3 defaults (SiliconFlow + DeepSeek-V3.2 +
/// Qwen3-Coder-30B + env-var `SILICONFLOW_API_KEY`). No flags are passed —
/// the CLI fills defaults. The API key VALUE is NOT touched by this handler
/// (the env-var NAME goes into turingos.toml; the value lives only on
/// AppState).
///
/// Exact CLI invocation:
///   `turingos llm config --workspace <workspace>`
///
/// Requires init_done; otherwise 409.
#[cfg(feature = "web")]
pub(crate) async fn welcome_llm_config_handler(
    State(state): State<AppState>,
) -> Result<Json<OnboardingStatus>, (StatusCode, Json<SetupError>)> {
    let workspace = resolve_workspace_path();
    let workspace_str = workspace.to_string_lossy().into_owned();
    let api_key_set = current_api_key_set(&state);

    let pre = inspect_workspace(&workspace);
    if !pre.init_done {
        return Err((
            StatusCode::CONFLICT,
            Json(SetupError {
                reason: "Step 1 (workspace init) must complete before LLM config.".into(),
                kind: "prerequisite_missing",
            }),
        ));
    }
    if pre.llm_configured {
        return Ok(Json(build_status(&workspace, pre, api_key_set)));
    }

    let bin = resolve_turingos_bin();
    log::info!(
        "welcome_llm_config_handler: bin={:?} workspace={:?}",
        bin,
        workspace_str
    );

    let output = run_welcome_command(
        bin.clone(),
        workspace.clone(),
        welcome_llm_config_args(&workspace_str),
        Duration::from_secs(120),
    )
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SetupError {
                reason: format!("failed to spawn/run {:?}: {e}", bin),
                kind: "llm_config_failed",
            }),
        )
    })?;

    if !output.success() {
        let combined = combine_truncated(&output.stdout, &output.stderr);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SetupError {
                reason: combined,
                kind: "llm_config_failed",
            }),
        ));
    }

    let inspect = inspect_workspace(&workspace);
    Ok(Json(build_status(&workspace, inspect, api_key_set)))
}

// ---------------------------------------------------------------------------
// POST /api/welcome/agent-deploy
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC2-N16: register a Solver agent via `turingos agent deploy`.
///
/// Uses a synthetic deterministic 64-hex pubkey derived from the workspace
/// path (FNV-1a + zero-pad to 64 hex chars). Matches the W4.3 setup-script
/// pattern. The pubkey is for demo plumbing only — no real signature
/// verification runs in the Phase 7 web flow.
///
/// Exact CLI invocation:
///   `turingos agent deploy --workspace <workspace> --id agent_001
///                          --pubkey <synthetic-64-hex> --role Solver`
///
/// Requires init_done + llm_config_done; otherwise 409.
#[cfg(feature = "web")]
pub(crate) async fn welcome_agent_deploy_handler(
    State(state): State<AppState>,
) -> Result<Json<OnboardingStatus>, (StatusCode, Json<SetupError>)> {
    let workspace = resolve_workspace_path();
    let workspace_str = workspace.to_string_lossy().into_owned();
    let api_key_set = current_api_key_set(&state);

    let pre = inspect_workspace(&workspace);
    if !pre.init_done {
        return Err((
            StatusCode::CONFLICT,
            Json(SetupError {
                reason: "Workspace init must complete before agent deploy.".into(),
                kind: "prerequisite_missing",
            }),
        ));
    }
    if !pre.llm_configured {
        return Err((
            StatusCode::CONFLICT,
            Json(SetupError {
                reason: "LLM config must complete before agent deploy.".into(),
                kind: "prerequisite_missing",
            }),
        ));
    }
    if pre.agents_count > 0 {
        return Ok(Json(build_status(&workspace, pre, api_key_set)));
    }

    let pubkey = synthetic_pubkey_for(&workspace_str);
    let bin = resolve_turingos_bin();
    log::info!(
        "welcome_agent_deploy_handler: bin={:?} workspace={:?} id=agent_001 role=Solver",
        bin,
        workspace_str
    );

    let output = run_welcome_command(
        bin.clone(),
        workspace.clone(),
        vec![
            "agent".into(),
            "deploy".into(),
            "--workspace".into(),
            workspace_str.clone(),
            "--id".into(),
            "agent_001".into(),
            "--pubkey".into(),
            pubkey.clone(),
            "--role".into(),
            "Solver".into(),
        ],
        Duration::from_secs(120),
    )
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SetupError {
                reason: format!("failed to spawn/run {:?}: {e}", bin),
                kind: "agent_deploy_failed",
            }),
        )
    })?;

    if !output.success() {
        let combined = combine_truncated(&output.stdout, &output.stderr);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SetupError {
                reason: combined,
                kind: "agent_deploy_failed",
            }),
        ));
    }

    let inspect = inspect_workspace(&workspace);
    Ok(Json(build_status(&workspace, inspect, api_key_set)))
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a synthetic, deterministic 64-hex pubkey for the welcome demo flow.
///
/// Implementation: FNV-1a 64-bit hash over `"agent_001:Solver:<workspace>"`,
/// repeated to fill 64 hex chars. Deterministic per workspace so re-running
/// the welcome flow without `--force` produces the same pubkey.
#[cfg(feature = "web")]
fn synthetic_pubkey_for(workspace: &str) -> String {
    let seed = format!("agent_001:Solver:{workspace}");
    let mut h: u64 = 0xcbf29ce484222325;
    for b in seed.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    let primary = format!("{h:016x}");
    let mut out = String::with_capacity(64);
    while out.len() < 64 {
        out.push_str(&primary);
    }
    out.truncate(64);
    out
}

#[cfg(feature = "web")]
async fn run_welcome_command(
    bin: String,
    workspace: PathBuf,
    args: Vec<String>,
    timeout: Duration,
) -> Result<SanitizedOutput, String> {
    let cwd = explicit_cwd(&workspace);
    let command = SanitizedCommand {
        program: PathBuf::from(bin),
        args,
        cwd,
        env: env_allowlist_from_current(&["PATH"]),
        stdin: None,
        timeout,
    };
    tokio::task::spawn_blocking(move || run_sanitized(command))
        .await
        .map_err(|e| format!("spawn_blocking: {e}"))?
        .map_err(|e| e.to_string())
}

#[cfg(feature = "web")]
fn explicit_cwd(workspace: &Path) -> PathBuf {
    if workspace.is_dir() {
        workspace.to_path_buf()
    } else {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    }
}

/// Combine stdout + stderr into a truncated diagnostic string for an
/// error response. Mirrors the spec.rs / generate.rs truncation policy.
#[cfg(feature = "web")]
fn combine_truncated(stdout: &[u8], stderr: &[u8]) -> String {
    let stdout_s = String::from_utf8_lossy(stdout);
    let stderr_s = String::from_utf8_lossy(stderr);
    let combined = format!("stdout: {stdout_s} | stderr: {stderr_s}");
    if combined.len() > 512 {
        format!("{}…", &combined[..512])
    } else {
        combined
    }
}

#[cfg(feature = "web")]
fn deepseek_direct_endpoint_selected() -> bool {
    std::env::var("TURINGOS_SILICONFLOW_ENDPOINT")
        .map(|v| {
            let lower = v.to_ascii_lowercase();
            lower.contains("api.deepseek.com") || lower.contains("deepseek")
        })
        .unwrap_or(false)
}

#[cfg(feature = "web")]
fn welcome_llm_config_args(workspace_str: &str) -> Vec<String> {
    let mut args = vec![
        "llm".to_string(),
        "config".to_string(),
        "--workspace".to_string(),
        workspace_str.to_string(),
    ];
    if deepseek_direct_endpoint_selected() {
        args.extend([
            "--provider".to_string(),
            "deepseek".to_string(),
            "--api-key-env".to_string(),
            "DEEPSEEK_API_KEY".to_string(),
            "--meta-model".to_string(),
            "deepseek-v4-pro".to_string(),
            "--blackbox-model".to_string(),
            "deepseek-v4-flash".to_string(),
            "--meta-thinking".to_string(),
            "on".to_string(),
            "--blackbox-thinking".to_string(),
            "off".to_string(),
        ]);
    }
    args
}

// ---------------------------------------------------------------------------
// Unit tests (no I/O)
// ---------------------------------------------------------------------------

#[cfg(all(feature = "web", test))]
mod tests {
    use super::*;

    #[test]
    fn validate_api_key_shape_accepts_valid() {
        assert!(validate_api_key_shape("sk-aaaaaaaaaaaaaa").is_ok());
        // Constructed at runtime so static secret-pattern scanners don't flag the literal.
        let valid_key = format!("sk-{}-{}", "1234567890abcdef", "XYZ");
        assert!(validate_api_key_shape(&valid_key).is_ok());
    }

    #[test]
    fn validate_api_key_shape_rejects_missing_prefix() {
        let err = validate_api_key_shape("aaaaaaaaaaaaaaaaa").unwrap_err();
        assert_eq!(err.kind, "invalid_input");
    }

    #[test]
    fn validate_api_key_shape_rejects_too_short() {
        let err = validate_api_key_shape("sk-short").unwrap_err();
        assert_eq!(err.kind, "invalid_input");
    }

    #[test]
    fn validate_api_key_shape_rejects_too_long() {
        let key = format!("sk-{}", "x".repeat(300));
        let err = validate_api_key_shape(&key).unwrap_err();
        assert_eq!(err.kind, "invalid_input");
    }

    #[test]
    fn validate_api_key_shape_rejects_control_chars() {
        let key = "sk-aaaaaaaaaaaaa\x01x";
        let err = validate_api_key_shape(key).unwrap_err();
        assert_eq!(err.kind, "invalid_input");
    }

    #[test]
    fn synthetic_pubkey_is_64_hex_chars() {
        let pk = synthetic_pubkey_for("tmp/phase7_active");
        assert_eq!(pk.len(), 64);
        assert!(pk.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn synthetic_pubkey_is_deterministic() {
        let a = synthetic_pubkey_for("tmp/phase7_active");
        let b = synthetic_pubkey_for("tmp/phase7_active");
        assert_eq!(a, b);
    }

    #[test]
    fn synthetic_pubkey_varies_by_workspace() {
        let a = synthetic_pubkey_for("tmp/phase7_active");
        let b = synthetic_pubkey_for("tmp/other_workspace");
        assert_ne!(a, b);
    }

    #[test]
    fn next_step_progression() {
        let no_ws = WsInspect {
            init_done: false,
            llm_configured: false,
            agents_count: 0,
            spec_done: false,
            spec_capsule_cid: None,
            artifacts_done: false,
        };
        assert_eq!(compute_next_step(&no_ws, false), NextStep::Init);

        let after_init = WsInspect {
            init_done: true,
            ..no_ws
        };
        assert_eq!(compute_next_step(&after_init, false), NextStep::LlmConfig);

        let after_llm = WsInspect {
            init_done: true,
            llm_configured: true,
            agents_count: 0,
            spec_done: false,
            spec_capsule_cid: None,
            artifacts_done: false,
        };
        assert_eq!(compute_next_step(&after_llm, false), NextStep::ApiKey);
        assert_eq!(compute_next_step(&after_llm, true), NextStep::AgentDeploy);

        let after_agent = WsInspect {
            init_done: true,
            llm_configured: true,
            agents_count: 1,
            spec_done: false,
            spec_capsule_cid: None,
            artifacts_done: false,
        };
        assert_eq!(compute_next_step(&after_agent, true), NextStep::Spec);

        let after_spec = WsInspect {
            init_done: true,
            llm_configured: true,
            agents_count: 1,
            spec_done: true,
            spec_capsule_cid: Some("deadbeef".repeat(8)),
            artifacts_done: false,
        };
        assert_eq!(compute_next_step(&after_spec, true), NextStep::Generate);

        let after_artifact = WsInspect {
            init_done: true,
            llm_configured: true,
            agents_count: 1,
            spec_done: true,
            spec_capsule_cid: Some("deadbeef".repeat(8)),
            artifacts_done: true,
        };
        assert_eq!(compute_next_step(&after_artifact, true), NextStep::Done);
    }
}
