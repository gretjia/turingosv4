//! TRACE_MATRIX FC2-N16: Phase 7 W7 smoke tests — onboarding wizard backend.
//!
//! Covers `/api/welcome/*` endpoints:
//!   - GET  /api/welcome/status        → 200 + OnboardingStatus snapshot
//!   - POST /api/welcome/api-key       → in-memory only, never written to disk
//!   - POST /api/welcome/init          → shells out to `turingos init`
//!   - POST /api/welcome/llm-config    → shells out to `turingos llm config`
//!   - POST /api/welcome/agent-deploy  → shells out to `turingos agent deploy`
//!
//! Plus: GET / redirects to /welcome on a fresh workspace; spec.rs and
//! generate.rs inject SILICONFLOW_API_KEY from AppState into child env.
//!
//! Gated on `#![cfg(feature = "web")]`. Run with:
//!   `cargo test --test cli_web_welcome_smoke --features web`
#![cfg(feature = "web")]

#[path = "../src/web/mod.rs"]
mod web;

use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

// Process-global lock for env mutation (TURINGOS_BACKEND_OVERRIDE /
// TURINGOS_WEB_WORKSPACE).
static ENV_LOCK: std::sync::OnceLock<Mutex<()>> = std::sync::OnceLock::new();
fn env_lock() -> &'static Mutex<()> {
    ENV_LOCK.get_or_init(|| Mutex::new(()))
}

// ---------------------------------------------------------------------------
// HTTP helpers (raw TCP, no tower/hyper client dep)
// ---------------------------------------------------------------------------

async fn start_server() -> SocketAddr {
    let router = web::router::build_with_state(64);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind random port");
    let addr = listener.local_addr().expect("local addr");
    tokio::spawn(async move {
        axum::serve(listener, router).await.expect("axum serve");
    });
    addr
}

async fn http_get(addr: SocketAddr, path: &str) -> (u16, String, String) {
    let mut stream = tokio::net::TcpStream::connect(addr).await.expect("connect");
    let request = format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).await.expect("write");
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await.expect("read");
    let raw = String::from_utf8_lossy(&buf).into_owned();
    let (head, body) = if let Some(idx) = raw.find("\r\n\r\n") {
        (&raw[..idx], raw[idx + 4..].to_string())
    } else {
        (raw.as_str(), String::new())
    };
    let status: u16 = head
        .lines()
        .next()
        .unwrap_or("")
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    (status, head.to_string(), body)
}

async fn http_post_json(addr: SocketAddr, path: &str, body: &str) -> (u16, String) {
    let mut stream = tokio::net::TcpStream::connect(addr).await.expect("connect");
    let request = format!(
        "POST {path} HTTP/1.1\r\n\
         Host: 127.0.0.1\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {body}",
        body.len()
    );
    stream.write_all(request.as_bytes()).await.expect("write");
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await.expect("read");
    let raw = String::from_utf8_lossy(&buf).into_owned();
    let (head, resp_body) = if let Some(idx) = raw.find("\r\n\r\n") {
        (&raw[..idx], raw[idx + 4..].to_string())
    } else {
        (raw.as_str(), String::new())
    };
    let status: u16 = head
        .lines()
        .next()
        .unwrap_or("")
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    (status, resp_body)
}

// ---------------------------------------------------------------------------
// Stub script writer — records argv + env to a file then exits.
// ---------------------------------------------------------------------------

fn write_recording_stub(
    dir: &tempfile::TempDir,
    args_path: &str,
    env_path: &str,
    side_effect: &str,
) -> String {
    let script_path = dir.path().join("turingos");
    let body = format!(
        r#"#!/bin/sh
# Record argv (one per line) and selected env vars
printf '%s\n' "$@" > {args_q}
{{
  printf 'SILICONFLOW_API_KEY=%s\n' "${{SILICONFLOW_API_KEY:-<unset>}}"
}} > {env_q}
{side}
exit 0
"#,
        args_q = shell_quote(args_path),
        env_q = shell_quote(env_path),
        side = side_effect,
    );
    std::fs::write(&script_path, body).expect("write stub");
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(&script_path).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&script_path, perms).unwrap();
    script_path.to_string_lossy().into_owned()
}

fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

/// Make a stub that *also* writes the four files that `inspect_workspace`
/// looks for so subsequent welcome/status calls progress to the next step.
fn make_init_stub(dir: &tempfile::TempDir, args_path: &str, env_path: &str) -> String {
    // Parse --project, write genesis_payload.toml + agent_pubkeys.json there.
    let side = r#"
proj=""
while [ $# -gt 0 ]; do
    if [ "$1" = "--project" ]; then proj="$2"; shift 2; else shift; fi
done
if [ -n "$proj" ]; then
    mkdir -p "$proj"
    printf '# stub genesis\n[meta]\ntemplate = "multi-agent"\n' > "$proj/genesis_payload.toml"
    printf '{}\n' > "$proj/agent_pubkeys.json"
fi
"#;
    write_recording_stub(dir, args_path, env_path, side)
}

fn make_init_stub_with_toml(dir: &tempfile::TempDir, args_path: &str, env_path: &str) -> String {
    let side = r#"
proj=""
provider="siliconflow"
while [ $# -gt 0 ]; do
    case "$1" in
        --project) proj="$2"; shift 2 ;;
        --provider) provider="$2"; shift 2 ;;
        *) shift ;;
    esac
done
if [ -n "$proj" ]; then
    mkdir -p "$proj"
    printf '# stub genesis\n[meta]\ntemplate = "multi-agent"\n' > "$proj/genesis_payload.toml"
    printf '{}\n' > "$proj/agent_pubkeys.json"
    if [ "$provider" = "deepseek" ]; then
        printf 'llm.provider = "deepseek"\nllm.meta.model = "deepseek-v4-pro"\nllm.blackbox.model = "deepseek-v4-flash"\n' > "$proj/turingos.toml"
    else
        printf 'llm.provider = "siliconflow"\nllm.meta.model = "deepseek-ai/DeepSeek-V3.2"\nllm.blackbox.model = "Qwen/Qwen3-Coder-30B-A3B-Instruct"\n' > "$proj/turingos.toml"
    fi
fi
"#;
    write_recording_stub(dir, args_path, env_path, side)
}

fn make_agent_deploy_stub(dir: &tempfile::TempDir, args_path: &str, env_path: &str) -> String {
    let side = r#"
ws=""
id=""
while [ $# -gt 0 ]; do
    case "$1" in
        --workspace) ws="$2"; shift 2 ;;
        --id) id="$2"; shift 2 ;;
        *) shift ;;
    esac
done
if [ -n "$ws" ]; then
    # Emit an agent_pubkeys.json entry that inspect_workspace recognises
    # (looks for lines starting with " and ending with {).
    printf '{\n    "%s": {\n        "pubkey": "00",\n        "role": "Solver"\n    }\n}\n' "$id" > "$ws/agent_pubkeys.json"
fi
"#;
    write_recording_stub(dir, args_path, env_path, side)
}

fn make_combo_stub(dir: &tempfile::TempDir, args_path: &str, env_path: &str) -> String {
    // A single stub that dispatches on $1 so we can run init/llm-config/
    // agent-deploy/spec from the same TURINGOS_BACKEND_OVERRIDE.
    let script_path = dir.path().join("turingos");
    let body = format!(
        r#"#!/bin/sh
sub="$1"
shift
printf '%s\n' "$sub" > {args_q}
printf '%s\n' "$@" >> {args_q}
{{
  printf 'SILICONFLOW_API_KEY=%s\n' "${{SILICONFLOW_API_KEY:-<unset>}}"
}} >> {env_q}
case "$sub" in
    init)
        proj=""
        while [ $# -gt 0 ]; do
            if [ "$1" = "--project" ]; then proj="$2"; shift 2; else shift; fi
        done
        if [ -n "$proj" ]; then
            mkdir -p "$proj"
            printf '# stub genesis\n[meta]\ntemplate = "multi-agent"\n' > "$proj/genesis_payload.toml"
            printf '{{}}\n' > "$proj/agent_pubkeys.json"
        fi
        ;;
    llm)
        ws=""
        while [ $# -gt 0 ]; do
            if [ "$1" = "--workspace" ]; then ws="$2"; shift 2; else shift; fi
        done
        if [ -n "$ws" ]; then
            printf 'llm.provider = "siliconflow"\nllm.meta.model = "x"\nllm.meta.api_key_env = "SILICONFLOW_API_KEY"\nllm.blackbox.model = "y"\nllm.blackbox.api_key_env = "SILICONFLOW_API_KEY"\n' > "$ws/turingos.toml"
        fi
        ;;
    agent)
        ws=""
        id=""
        while [ $# -gt 0 ]; do
            case "$1" in
                --workspace) ws="$2"; shift 2 ;;
                --id) id="$2"; shift 2 ;;
                *) shift ;;
            esac
        done
        if [ -n "$ws" ]; then
            printf '{{\n    "%s": {{\n        "pubkey": "00",\n        "role": "Solver"\n    }}\n}}\n' "$id" > "$ws/agent_pubkeys.json"
        fi
        ;;
    spec)
        ws=""
        while [ $# -gt 0 ]; do
            if [ "$1" = "--workspace" ]; then ws="$2"; shift 2; else shift; fi
        done
        if [ -n "$ws" ]; then
            mkdir -p "$ws"
            printf '# stub spec\n' > "$ws/spec.md"
        fi
        printf 'Spec interview complete.\n  CAS capsule CID    -> deadbeef0001\n'
        ;;
esac
exit 0
"#,
        args_q = shell_quote(args_path),
        env_q = shell_quote(env_path),
    );
    std::fs::write(&script_path, body).expect("write combo stub");
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(&script_path).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&script_path, perms).unwrap();
    script_path.to_string_lossy().into_owned()
}

// Helper that returns an 8-answer JSON body for spec/submit.
fn valid_answers_body() -> String {
    let answers: Vec<String> = (0..8).map(|i| format!("Answer text {i}")).collect();
    let parts: Vec<String> = answers.iter().map(|a| format!("\"{a}\"")).collect();
    format!("{{\"answers\":[{}]}}", parts.join(","))
}

// ---------------------------------------------------------------------------
// Test 1: GET /api/welcome/status on a fresh workspace returns next_step=Init.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn welcome_status_fresh_workspace() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().join("fresh-ws").to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace);

    let addr = start_server().await;
    let (status, _, body) = http_get(addr, "/api/welcome/status").await;

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(
        status, 200,
        "GET /api/welcome/status must return 200; body={body}"
    );
    let parsed: serde_json::Value =
        serde_json::from_str(&body).expect("response must be valid JSON");
    assert_eq!(
        parsed["init_done"].as_bool(),
        Some(false),
        "init_done must be false"
    );
    assert_eq!(parsed["llm_config_done"].as_bool(), Some(false));
    assert_eq!(parsed["api_key_set"].as_bool(), Some(false));
    assert_eq!(parsed["agents_count"].as_u64(), Some(0));
    assert_eq!(parsed["spec_done"].as_bool(), Some(false));
    assert_eq!(parsed["artifacts_done"].as_bool(), Some(false));
    assert_eq!(parsed["next_step"].as_str(), Some("Init"));
}

#[tokio::test]
async fn welcome_page_first_paint_never_skips_missing_api_key() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().join("ws");
    std::fs::create_dir_all(&workspace).expect("mkdir workspace");
    std::fs::write(
        workspace.join("genesis_payload.toml"),
        "[meta]\ntemplate = \"multi-agent\"\n",
    )
    .expect("write genesis");
    std::fs::write(workspace.join("agent_pubkeys.json"), "{}\n").expect("write pubkeys");
    std::fs::write(
        workspace.join("turingos.toml"),
        "llm.meta.model = \"deepseek-v4-pro\"\nllm.blackbox.model = \"deepseek-v4-flash\"\n",
    )
    .expect("write llm config");
    let workspace = workspace.to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace);
    let addr = start_server().await;

    let (status, _, body) = http_get(addr, "/welcome").await;

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(status, 200, "GET /welcome must return 200; body={body}");
    assert!(
        body.contains("API 密钥") || body.contains("把 API 密钥交给我"),
        "server first paint must expose the API-key step; body={body}"
    );
    assert!(
        !body.contains("你的工作站已就绪"),
        "server first paint must not show Ready while api_key_set=false; body={body}"
    );
}

// ---------------------------------------------------------------------------
// Test 2: POST /api/welcome/api-key rejects malformed keys with 400.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn welcome_set_api_key_rejects_invalid_format() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace);
    let addr = start_server().await;

    // Missing sk- prefix
    let (status, body) = http_post_json(
        addr,
        "/api/welcome/api-key",
        "{\"api_key\":\"plain-junk-no-prefix\"}",
    )
    .await;
    assert_eq!(
        status, 400,
        "malformed api key must return 400; body={body}"
    );
    assert!(
        body.contains("invalid_input"),
        "body must mention invalid_input; got {body}"
    );

    // Too short
    let (status2, body2) =
        http_post_json(addr, "/api/welcome/api-key", "{\"api_key\":\"sk-tiny\"}").await;
    assert_eq!(status2, 400, "short key must return 400; body={body2}");

    // Empty
    let (status3, _) = http_post_json(addr, "/api/welcome/api-key", "{\"api_key\":\"\"}").await;
    assert_eq!(status3, 400, "empty key must return 400");

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);
}

// ---------------------------------------------------------------------------
// Test 3: POST /api/welcome/api-key stores in memory; status flips api_key_set.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn welcome_set_api_key_stores_in_memory() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace);
    let addr = start_server().await;

    let api_key = "sk-stub-test-key-for-welcome-smoke-XXXXXX";
    let body = format!("{{\"api_key\":\"{api_key}\"}}");
    let (status, resp_body) = http_post_json(addr, "/api/welcome/api-key", &body).await;
    assert_eq!(status, 200, "valid key must return 200; body={resp_body}");

    // The response body MUST NOT echo the key.
    assert!(
        !resp_body.contains(api_key),
        "response body must NOT contain the API key value; body={resp_body}"
    );

    // Subsequent GET must report api_key_set: true.
    let (s2, _, body2) = http_get(addr, "/api/welcome/status").await;
    assert_eq!(s2, 200);
    let parsed: serde_json::Value = serde_json::from_str(&body2).expect("valid json");
    assert_eq!(parsed["api_key_set"].as_bool(), Some(true));
    // And again — the GET response must not contain the key value.
    assert!(
        !body2.contains(api_key),
        "/api/welcome/status response must NOT contain the API key value; body={body2}"
    );

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);
}

// ---------------------------------------------------------------------------
// Test 4: setting an API key must NEVER write the key value to disk.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn welcome_set_api_key_never_writes_to_disk() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().join("ws").to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace);
    let addr = start_server().await;

    // Pre-seed a workspace + write a turingos.toml that records the env-var
    // NAME (Phase 6.3 invariant: NAME goes to disk, VALUE never does).
    std::fs::create_dir_all(&workspace).expect("mkdir ws");
    std::fs::write(
        std::path::Path::new(&workspace).join("turingos.toml"),
        "llm.meta.api_key_env = \"SILICONFLOW_API_KEY\"\n",
    )
    .unwrap();

    let api_key = "sk-NEVER-LET-ME-HIT-DISK-xxxxxxxxxxxxxxxxxxxx";
    let body = format!("{{\"api_key\":\"{api_key}\"}}");
    let (status, _) = http_post_json(addr, "/api/welcome/api-key", &body).await;
    assert_eq!(status, 200);

    // Walk the workspace directory; assert no file's contents contain the key.
    fn walk_check(dir: &std::path::Path, needle: &str) -> bool {
        let rd = match std::fs::read_dir(dir) {
            Ok(rd) => rd,
            Err(_) => return false,
        };
        for entry in rd.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if walk_check(&path, needle) {
                    return true;
                }
            } else if let Ok(content) = std::fs::read_to_string(&path) {
                if content.contains(needle) {
                    return true;
                }
            }
        }
        false
    }
    let found = walk_check(std::path::Path::new(&workspace), api_key);
    assert!(
        !found,
        "API key value MUST NOT appear in any file under the workspace"
    );

    // The turingos.toml file must still contain the env-var NAME (Phase 6.3 contract).
    let toml_content =
        std::fs::read_to_string(std::path::Path::new(&workspace).join("turingos.toml")).unwrap();
    assert!(
        toml_content.contains("SILICONFLOW_API_KEY"),
        "turingos.toml must keep the env-var NAME, not the value"
    );
    assert!(
        !toml_content.contains(api_key),
        "turingos.toml must NOT contain the API key value"
    );

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);
}

// ---------------------------------------------------------------------------
// Test 5: POST /api/welcome/init invokes the correct shellout argv.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn welcome_init_invokes_correct_shellout() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().join("ws").to_string_lossy().into_owned();
    let args_path = dir.path().join("args.txt").to_string_lossy().into_owned();
    let env_path = dir.path().join("env.txt").to_string_lossy().into_owned();
    let stub = make_init_stub(&dir, &args_path, &env_path);

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &stub);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace);
    let addr = start_server().await;

    let (status, body) = http_post_json(addr, "/api/welcome/init", "{}").await;

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(status, 200, "init must return 200; body={body}");
    let recorded = std::fs::read_to_string(&args_path).expect("stub must record args");
    let args: Vec<&str> = recorded.lines().collect();
    assert!(args.contains(&"init"), "must invoke 'init'; args={args:?}");
    assert!(
        args.contains(&"--project"),
        "must pass --project; args={args:?}"
    );
    assert!(
        args.contains(&"--template"),
        "must pass --template; args={args:?}"
    );
    assert!(
        args.contains(&"multi-agent"),
        "must pass template=multi-agent; args={args:?}"
    );
    assert!(
        args.iter().any(|a| *a == workspace),
        "must pass workspace path"
    );

    // Now status should report init_done.
    let _guard2 = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace);
    let (s2, _, body2) = http_get(addr, "/api/welcome/status").await;
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard2);
    assert_eq!(s2, 200);
    let parsed: serde_json::Value = serde_json::from_str(&body2).unwrap();
    assert_eq!(parsed["init_done"].as_bool(), Some(true));
    assert_eq!(parsed["next_step"].as_str(), Some("LlmConfig"));
}

#[tokio::test]
async fn welcome_init_uses_deepseek_provider_when_endpoint_is_deepseek_direct() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().join("ws").to_string_lossy().into_owned();
    let args_path = dir.path().join("args.txt").to_string_lossy().into_owned();
    let env_path = dir.path().join("env.txt").to_string_lossy().into_owned();
    let stub = make_init_stub_with_toml(&dir, &args_path, &env_path);

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &stub);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace);
    std::env::set_var(
        "TURINGOS_SILICONFLOW_ENDPOINT",
        "https://api.deepseek.com/v1/chat/completions",
    );
    let addr = start_server().await;

    let (status, body) = http_post_json(addr, "/api/welcome/init", "{}").await;

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    std::env::remove_var("TURINGOS_SILICONFLOW_ENDPOINT");
    drop(_guard);

    assert_eq!(status, 200, "init must return 200; body={body}");
    let recorded = std::fs::read_to_string(&args_path).expect("stub must record args");
    let args: Vec<&str> = recorded.lines().collect();
    assert!(
        args.windows(2)
            .any(|pair| pair == ["--provider", "deepseek"]),
        "DeepSeek direct endpoint must initialize DeepSeek model namespace; args={args:?}"
    );
    let toml = std::fs::read_to_string(std::path::Path::new(&workspace).join("turingos.toml"))
        .expect("stub writes turingos.toml");
    assert!(
        toml.contains("deepseek-v4-pro") && toml.contains("deepseek-v4-flash"),
        "fresh welcome init must leave DeepSeek direct model IDs when endpoint is DeepSeek; toml={toml}"
    );
}

// ---------------------------------------------------------------------------
// Test 6: POST /api/welcome/init is idempotent — second call still returns 200.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn welcome_init_idempotent() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().join("ws").to_string_lossy().into_owned();
    // Pre-seed the workspace so init_done is already true.
    std::fs::create_dir_all(&workspace).unwrap();
    std::fs::write(
        std::path::Path::new(&workspace).join("genesis_payload.toml"),
        "[meta]\ntemplate = \"multi-agent\"\n",
    )
    .unwrap();
    std::fs::write(
        std::path::Path::new(&workspace).join("agent_pubkeys.json"),
        "{}\n",
    )
    .unwrap();

    let args_path = dir.path().join("args.txt").to_string_lossy().into_owned();
    let env_path = dir.path().join("env.txt").to_string_lossy().into_owned();
    let stub = make_init_stub(&dir, &args_path, &env_path);

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &stub);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace);
    let addr = start_server().await;

    let (status, _) = http_post_json(addr, "/api/welcome/init", "{}").await;
    let (status2, _) = http_post_json(addr, "/api/welcome/init", "{}").await;

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(status, 200);
    assert_eq!(
        status2, 200,
        "second init must still return 200 (idempotent)"
    );

    // The stub should never have been invoked at all because of the idempotent
    // fast path. If args.txt exists with content, the stub ran; it shouldn't have.
    let args_exists = std::fs::read_to_string(&args_path).is_ok();
    assert!(
        !args_exists,
        "stub should not be invoked when workspace is already init-done"
    );
}

// ---------------------------------------------------------------------------
// Test 7: POST /api/welcome/llm-config requires init_done; returns 409 otherwise.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn welcome_llm_config_requires_init() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().join("uninit-ws").to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace);
    let addr = start_server().await;

    let (status, body) = http_post_json(addr, "/api/welcome/llm-config", "{}").await;

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(status, 409, "uninit workspace must return 409; body={body}");
    assert!(
        body.contains("prerequisite_missing"),
        "body must include kind=prerequisite_missing; got {body}"
    );
}

// ---------------------------------------------------------------------------
// Test 8: agent deploy uses a 64-hex synthetic pubkey.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn welcome_agent_deploy_uses_synthetic_pubkey() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().join("ws").to_string_lossy().into_owned();
    // Pre-seed init + llm config so agent deploy passes its prerequisite gate.
    std::fs::create_dir_all(&workspace).unwrap();
    let p = std::path::Path::new(&workspace);
    std::fs::write(p.join("genesis_payload.toml"), "[meta]\n").unwrap();
    std::fs::write(p.join("agent_pubkeys.json"), "{}\n").unwrap();
    std::fs::write(
        p.join("turingos.toml"),
        "llm.meta.model = \"x\"\nllm.blackbox.model = \"y\"\n",
    )
    .unwrap();

    let args_path = dir.path().join("args.txt").to_string_lossy().into_owned();
    let env_path = dir.path().join("env.txt").to_string_lossy().into_owned();
    let stub = make_agent_deploy_stub(&dir, &args_path, &env_path);

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &stub);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace);
    let addr = start_server().await;

    let (status, body) = http_post_json(addr, "/api/welcome/agent-deploy", "{}").await;

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(status, 200, "agent deploy must return 200; body={body}");
    let recorded = std::fs::read_to_string(&args_path).expect("args recorded");
    let args: Vec<&str> = recorded.lines().collect();
    assert!(args.contains(&"agent"));
    assert!(args.contains(&"deploy"));
    assert!(args.contains(&"--id"));
    assert!(args.contains(&"agent_001"));
    assert!(args.contains(&"--role"));
    assert!(args.contains(&"Solver"));

    // Pubkey: the arg right after --pubkey must be 64 hex chars.
    let pk_pos = args
        .iter()
        .position(|a| *a == "--pubkey")
        .expect("must pass --pubkey");
    let pk = args
        .get(pk_pos + 1)
        .expect("must have value after --pubkey");
    assert_eq!(
        pk.len(),
        64,
        "pubkey must be 64 chars; got {} for {pk}",
        pk.len()
    );
    assert!(
        pk.chars().all(|c| c.is_ascii_hexdigit()),
        "pubkey must be hex; got {pk}"
    );
}

// ---------------------------------------------------------------------------
// Test 9: full flow — sequential POSTs progress next_step correctly.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn welcome_full_flow_progresses_next_step() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().join("ws").to_string_lossy().into_owned();
    let args_path = dir.path().join("args.txt").to_string_lossy().into_owned();
    let env_path = dir.path().join("env.txt").to_string_lossy().into_owned();
    let stub = make_combo_stub(&dir, &args_path, &env_path);

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &stub);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace);
    let addr = start_server().await;

    // status: Init
    let (_, _, b0) = http_get(addr, "/api/welcome/status").await;
    let p0: serde_json::Value = serde_json::from_str(&b0).unwrap();
    assert_eq!(p0["next_step"].as_str(), Some("Init"));

    // init → LlmConfig
    let (s, b) = http_post_json(addr, "/api/welcome/init", "{}").await;
    assert_eq!(s, 200);
    let p: serde_json::Value = serde_json::from_str(&b).unwrap();
    assert_eq!(p["next_step"].as_str(), Some("LlmConfig"));

    // llm-config → ApiKey
    let (s, b) = http_post_json(addr, "/api/welcome/llm-config", "{}").await;
    assert_eq!(s, 200, "body={b}");
    let p: serde_json::Value = serde_json::from_str(&b).unwrap();
    assert_eq!(p["next_step"].as_str(), Some("ApiKey"));

    // api-key → AgentDeploy
    let key = "sk-stub-test-key-1234567890abcdef";
    let (s, b) = http_post_json(
        addr,
        "/api/welcome/api-key",
        &format!("{{\"api_key\":\"{key}\"}}"),
    )
    .await;
    assert_eq!(s, 200, "body={b}");
    let p: serde_json::Value = serde_json::from_str(&b).unwrap();
    assert_eq!(p["next_step"].as_str(), Some("AgentDeploy"));

    // agent-deploy → Spec
    let (s, b) = http_post_json(addr, "/api/welcome/agent-deploy", "{}").await;
    assert_eq!(s, 200, "body={b}");
    let p: serde_json::Value = serde_json::from_str(&b).unwrap();
    assert_eq!(p["next_step"].as_str(), Some("Spec"));

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);
}

// ---------------------------------------------------------------------------
// Bonus: GET / redirects to /welcome on a fresh workspace (W7 cold-open UX).
// ---------------------------------------------------------------------------

#[tokio::test]
async fn root_redirects_to_welcome_when_not_onboarded() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().join("fresh-ws").to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace);
    let addr = start_server().await;
    let (status, headers, _) = http_get(addr, "/").await;
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert!(
        status == 302 || status == 303 || status == 307,
        "GET / on fresh workspace must redirect; got {status}"
    );
    let location_line = headers
        .lines()
        .find(|l| l.to_lowercase().starts_with("location:"))
        .unwrap_or("");
    assert!(
        location_line.contains("/welcome"),
        "Location header must point at /welcome; got {location_line}"
    );
}
