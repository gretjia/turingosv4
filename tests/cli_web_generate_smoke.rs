//! TRACE_MATRIX FC1-N5 + FC2-N16: Phase 7 W5 smoke tests — verifies the
//! generate endpoint:
//!   POST /api/generate → session validation, spec.md check, CLI shellout
//!                        (via stub), artifact list, WS broadcast of
//!                        GenerateComplete.
//!
//! Also verifies the artifact serve endpoint:
//!   GET /api/artifact/:session_id/:name → file bytes + Content-Type
//!   GET /api/artifact/:session_id/..%2Fetc/passwd → 400 (traversal blocked)
//!
//! Gated on `#![cfg(feature = "web")]` so non-web builds never see this.
//! Run with: `cargo test --test cli_web_generate_smoke --features web`
#![cfg(feature = "web")]

#[path = "../src/web/mod.rs"]
mod web;

use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

static ENV_LOCK: std::sync::OnceLock<Mutex<()>> = std::sync::OnceLock::new();

fn env_lock() -> &'static Mutex<()> {
    ENV_LOCK.get_or_init(|| Mutex::new(()))
}

// ---------------------------------------------------------------------------
// Server / HTTP helpers
// ---------------------------------------------------------------------------

async fn start_server() -> SocketAddr {
    let router = web::router::build_with_state(64);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind random port");
    let addr = listener.local_addr().expect("local addr");
    tokio::spawn(async move {
        axum::serve(listener, router)
            .await
            .expect("axum serve error in test");
    });
    addr
}

async fn http_get(addr: SocketAddr, path: &str) -> (u16, Vec<u8>, String) {
    let mut stream = tokio::net::TcpStream::connect(addr).await.expect("connect");
    let request = format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).await.expect("write");
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await.expect("read");
    let raw = String::from_utf8_lossy(&buf).into_owned();
    let (head, _) = if let Some(idx) = raw.find("\r\n\r\n") {
        (&raw[..idx], &raw[idx + 4..])
    } else {
        (raw.as_str(), "")
    };
    let status_code: u16 = head
        .lines()
        .next()
        .unwrap_or("")
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    // Find body start in raw bytes.
    let header_end = buf
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .map(|p| p + 4)
        .unwrap_or(buf.len());
    let body_bytes = buf[header_end..].to_vec();
    let headers = head.to_string();
    (status_code, body_bytes, headers)
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
    let status_code: u16 = head
        .lines()
        .next()
        .unwrap_or("")
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    (status_code, resp_body)
}

// ---------------------------------------------------------------------------
// Stub helpers
// ---------------------------------------------------------------------------

fn write_stub_script(
    dir: &tempfile::TempDir,
    exit_code: i32,
    stdout_content: &str,
    stderr_content: &str,
    record_args_file: &str,
) -> String {
    let script_path = dir.path().join("turingos");
    let script_content = format!(
        r#"#!/bin/sh
printf '%s\n' "$@" > {record_args_file}
printf '%s' {stdout_q}
printf '%s' {stderr_q} >&2
exit {exit_code}
"#,
        record_args_file = shell_quote(record_args_file),
        stdout_q = shell_quote(stdout_content),
        stderr_q = shell_quote(stderr_content),
        exit_code = exit_code,
    );
    std::fs::write(&script_path, &script_content).expect("write stub");
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(&script_path).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&script_path, perms).unwrap();
    script_path.to_string_lossy().into_owned()
}

fn write_env_recording_generate_stub(
    dir: &tempfile::TempDir,
    record_args_file: &str,
    record_env_file: &str,
) -> String {
    let script_path = dir.path().join("turingos");
    let script_content = format!(
        r#"#!/bin/sh
printf '%s\n' "$@" > {record_args_file}
{{
  printf 'SILICONFLOW_API_KEY=%s\n' "${{SILICONFLOW_API_KEY:-<unset>}}"
  printf 'DEEPSEEK_API_KEY=%s\n' "${{DEEPSEEK_API_KEY:-<unset>}}"
  printf 'DEEPSEEK_API_KEY_WORKER=%s\n' "${{DEEPSEEK_API_KEY_WORKER:-<unset>}}"
}} > {record_env_file}
WORKSPACE="$3"
mkdir -p "$WORKSPACE/artifacts"
cat > "$WORKSPACE/artifacts/index.html" <<'HTMLEOF'
{good_html}
HTMLEOF
exit 0
"#,
        record_args_file = shell_quote(record_args_file),
        record_env_file = shell_quote(record_env_file),
        good_html = good_html_payload(),
    );
    std::fs::write(&script_path, &script_content).expect("write env stub");
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(&script_path).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&script_path, perms).unwrap();
    script_path.to_string_lossy().into_owned()
}

fn shell_quote(s: &str) -> String {
    let escaped = s.replace('\'', "'\\''");
    format!("'{escaped}'")
}

// ---------------------------------------------------------------------------
// Setup helpers: create a session dir + spec.md
// ---------------------------------------------------------------------------

fn setup_session(workspace: &std::path::Path, session_id: &str) -> std::path::PathBuf {
    let session_dir = workspace.join("sessions").join(session_id);
    std::fs::create_dir_all(&session_dir).expect("create session dir");
    session_dir
}

fn write_spec_md(session_dir: &std::path::Path) {
    let spec_content =
        "# Stub Spec\n\n## One-line Goal\n\nTest spec.\n\n<!-- TURINGOS_SPEC_END -->\n";
    std::fs::write(session_dir.join("spec.md"), spec_content).expect("write spec.md");
}

/// Write a known-good index.html that passes W8 heuristic verification:
/// has <canvas>, document.addEventListener('keydown'), requestAnimationFrame,
/// balanced braces/script tags, size > 2 KB, no external resources, no
/// inverted nullish guard. Plus a main.py to exercise multi-file handling.
fn write_stub_artifacts(session_dir: &std::path::Path) {
    let artifacts_dir = session_dir.join("artifacts");
    std::fs::create_dir_all(&artifacts_dir).expect("create artifacts dir");
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html><head><title>Hello</title></head><body>");
    html.push_str("<canvas id=\"c\" width=\"200\" height=\"400\"></canvas>");
    html.push_str("<script>\n");
    html.push_str("let ctx = document.getElementById('c').getContext('2d');\n");
    html.push_str("let player = { x: 0 };\n");
    html.push_str(
        "function tick() { ctx.fillRect(player.x, 0, 10, 10); requestAnimationFrame(tick); }\n",
    );
    html.push_str("document.addEventListener('keydown', function(e) {\n");
    html.push_str("  if (e.code === 'ArrowLeft') { player.x = player.x - 1; }\n");
    html.push_str("  if (e.code === 'ArrowRight') { player.x = player.x + 1; }\n");
    html.push_str("});\n");
    for _ in 0..40 {
        html.push_str(
            "// padding comment line to clear minimum size heuristic threshold ok ok ok ok\n",
        );
    }
    html.push_str("tick();\n");
    html.push_str("</script></body></html>\n");
    std::fs::write(artifacts_dir.join("index.html"), &html).expect("write index.html");
    std::fs::write(
        artifacts_dir.join("main.py"),
        "# stub python\nprint('hello')\n",
    )
    .expect("write main.py");
}

// ---------------------------------------------------------------------------
// WebSocket helpers
// ---------------------------------------------------------------------------

struct WsConn {
    stream: tokio::net::TcpStream,
    leftover: Vec<u8>,
    leftover_pos: usize,
}

impl WsConn {
    async fn read_byte(&mut self) -> Option<u8> {
        if self.leftover_pos < self.leftover.len() {
            let b = self.leftover[self.leftover_pos];
            self.leftover_pos += 1;
            return Some(b);
        }
        let mut b = [0u8; 1];
        match self.stream.read(&mut b).await {
            Ok(0) | Err(_) => None,
            Ok(_) => Some(b[0]),
        }
    }

    async fn read_exact_buf(&mut self, n: usize) -> Vec<u8> {
        let mut out = Vec::with_capacity(n);
        let avail = self.leftover.len() - self.leftover_pos;
        if avail > 0 {
            let take = avail.min(n);
            out.extend_from_slice(&self.leftover[self.leftover_pos..self.leftover_pos + take]);
            self.leftover_pos += take;
        }
        if out.len() < n {
            let mut rest = vec![0u8; n - out.len()];
            self.stream
                .read_exact(&mut rest)
                .await
                .expect("ws read_exact");
            out.extend_from_slice(&rest);
        }
        out
    }
}

const WS_KEY: &str = "dGhlIHNhbXBsZSBub25jZQ==";
const OP_TEXT: u8 = 0x01;
const OP_CLOSE: u8 = 0x08;

async fn ws_upgrade(addr: SocketAddr) -> WsConn {
    let mut stream = tokio::net::TcpStream::connect(addr)
        .await
        .expect("connect ws");
    let request = format!(
        "GET /ws HTTP/1.1\r\n\
         Host: 127.0.0.1\r\n\
         Upgrade: websocket\r\n\
         Connection: Upgrade\r\n\
         Sec-WebSocket-Key: {WS_KEY}\r\n\
         Sec-WebSocket-Version: 13\r\n\
         \r\n"
    );
    stream
        .write_all(request.as_bytes())
        .await
        .expect("write ws upgrade");
    let mut header_buf: Vec<u8> = Vec::new();
    loop {
        let mut b = [0u8; 1];
        stream.read_exact(&mut b).await.expect("read byte");
        header_buf.push(b[0]);
        if header_buf.ends_with(b"\r\n\r\n") {
            break;
        }
        assert!(header_buf.len() < 4096, "upgrade response too large");
    }
    let response = String::from_utf8_lossy(&header_buf);
    let status: u16 = response
        .lines()
        .next()
        .unwrap_or("")
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    assert_eq!(status, 101, "WS upgrade must return 101");
    WsConn {
        stream,
        leftover: Vec::new(),
        leftover_pos: 0,
    }
}

async fn ws_recv_frame(conn: &mut WsConn) -> Option<(u8, Vec<u8>)> {
    let b0 = conn.read_byte().await?;
    let b1 = conn.read_byte().await?;
    let opcode = b0 & 0x0f;
    let masked = (b1 & 0x80) != 0;
    let len_byte = (b1 & 0x7f) as usize;
    let payload_len = if len_byte < 126 {
        len_byte
    } else if len_byte == 126 {
        let ext = conn.read_exact_buf(2).await;
        u16::from_be_bytes([ext[0], ext[1]]) as usize
    } else {
        let ext = conn.read_exact_buf(8).await;
        let mut arr = [0u8; 8];
        arr.copy_from_slice(&ext);
        u64::from_be_bytes(arr) as usize
    };
    let mask_key = if masked {
        let k = conn.read_exact_buf(4).await;
        Some([k[0], k[1], k[2], k[3]])
    } else {
        None
    };
    let mut payload = if payload_len > 0 {
        conn.read_exact_buf(payload_len).await
    } else {
        Vec::new()
    };
    if let Some(key) = mask_key {
        for (i, byte) in payload.iter_mut().enumerate() {
            *byte ^= key[i % 4];
        }
    }
    Some((opcode, payload))
}

// ---------------------------------------------------------------------------
// Test 1: generate_rejects_missing_session
// ---------------------------------------------------------------------------

/// POST /api/generate with a nonexistent session_id → 400.
#[tokio::test]
async fn generate_rejects_missing_session() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace);

    let addr = start_server().await;
    let body = r#"{"session_id":"nonexistent-session-xyz","from_capsule":false}"#;
    let (status, resp_body) = http_post_json(addr, "/api/generate", body).await;

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(
        status, 400,
        "missing session must return 400; body={resp_body}"
    );
    assert!(
        resp_body.contains("invalid_input") || resp_body.contains("not found"),
        "must indicate missing session; body={resp_body}"
    );
}

// ---------------------------------------------------------------------------
// Test 2: generate_rejects_missing_spec_md
// ---------------------------------------------------------------------------

/// Session dir exists but no spec.md → 400 kind=spec_md_missing.
#[tokio::test]
async fn generate_rejects_missing_spec_md() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let session_id = "test-no-spec-01";
    // Create session dir but do NOT write spec.md.
    setup_session(&workspace, session_id);
    let workspace_str = workspace.to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let addr = start_server().await;
    let body = format!(r#"{{"session_id":"{session_id}","from_capsule":false}}"#);
    let (status, resp_body) = http_post_json(addr, "/api/generate", &body).await;

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(
        status, 400,
        "missing spec.md must return 400; body={resp_body}"
    );
    assert!(
        resp_body.contains("spec_md_missing"),
        "must contain kind=spec_md_missing; body={resp_body}"
    );
}

// ---------------------------------------------------------------------------
// Test 3: generate_invokes_shellout_with_correct_args_via_override
// ---------------------------------------------------------------------------

/// Setup a fake session with spec.md; POST generate; stub records argv;
/// assert correct CLI args including --workspace <session-dir>.
#[tokio::test]
async fn generate_invokes_shellout_with_correct_args_via_override() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let session_id = "test-gen-args-01";
    let session_dir = setup_session(&workspace, session_id);
    write_spec_md(&session_dir);

    let args_file = dir.path().join("recorded_args.txt");
    let args_path = args_file.to_string_lossy().into_owned();
    // Stub exits 0, writes nothing (no artifacts dir → handler returns empty list).
    let script_path = write_stub_script(
        &dir,
        0,
        "Generated 0 file(s) under /x/artifacts/\n",
        "",
        &args_path,
    );
    let workspace_str = workspace.to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &script_path);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let addr = start_server().await;
    let body = format!(r#"{{"session_id":"{session_id}","from_capsule":false}}"#);
    let (status, resp_body) = http_post_json(addr, "/api/generate", &body).await;

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(status, 200, "stub exit 0 must return 200; body={resp_body}");

    let recorded = std::fs::read_to_string(&args_file).expect("stub should have written args file");
    let args: Vec<&str> = recorded.lines().collect();

    assert!(
        args.contains(&"generate"),
        "stub must receive 'generate' subcommand; recorded={recorded:?}"
    );
    assert!(
        args.contains(&"--workspace"),
        "stub must receive --workspace; recorded={recorded:?}"
    );
    // The --workspace value must be the session_dir path.
    let session_dir_str = session_dir.to_string_lossy().into_owned();
    assert!(
        args.iter().any(|a| a.contains(&session_dir_str[..])),
        "stub must receive session_dir as workspace value; recorded={recorded:?}"
    );
    // --from-capsule should NOT be present (from_capsule=false).
    assert!(
        !args.contains(&"--from-capsule"),
        "stub must NOT receive --from-capsule when from_capsule=false; recorded={recorded:?}"
    );
    assert!(
        args.windows(2)
            .any(|w| matches!(w, ["--n-parallel-workers", "3"])),
        "web default must request N=3 Polymarket workers; recorded={recorded:?}"
    );
}

// ---------------------------------------------------------------------------
// Test 4: generate_returns_artifact_list_on_success
// ---------------------------------------------------------------------------

/// Setup session with spec.md + pre-populated artifacts/; POST generate (stub
/// returns 0); response lists the artifacts with correct content-types.
#[tokio::test]
async fn generate_returns_artifact_list_on_success() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let session_id = "test-gen-artifacts-01";
    let session_dir = setup_session(&workspace, session_id);
    write_spec_md(&session_dir);
    write_stub_artifacts(&session_dir);

    let args_file = dir.path().join("recorded_args.txt");
    let args_path = args_file.to_string_lossy().into_owned();
    let script_path = write_stub_script(
        &dir,
        0,
        "Generated 2 file(s) under artifacts/\n",
        "",
        &args_path,
    );
    let workspace_str = workspace.to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &script_path);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let addr = start_server().await;
    let body = format!(r#"{{"session_id":"{session_id}","from_capsule":false}}"#);
    let (status, resp_body) = http_post_json(addr, "/api/generate", &body).await;

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(status, 200, "must return 200; body={resp_body}");

    let parsed: serde_json::Value =
        serde_json::from_str(&resp_body).expect("response must be valid JSON");
    let artifacts = parsed["artifacts"]
        .as_array()
        .expect("response must have 'artifacts' array");
    assert_eq!(
        artifacts.len(),
        2,
        "must return 2 artifacts; got {}; body={resp_body}",
        artifacts.len()
    );

    // Find index.html entry.
    let html_entry = artifacts
        .iter()
        .find(|e| e["path"].as_str().unwrap_or("").contains("index.html"))
        .expect("must include index.html artifact");
    assert_eq!(
        html_entry["content_type"].as_str().unwrap_or(""),
        "text/html",
        "index.html must have content_type=text/html"
    );
    assert!(
        html_entry["size_bytes"].as_u64().unwrap_or(0) > 0,
        "index.html size_bytes must be > 0"
    );

    // Find main.py entry.
    let py_entry = artifacts
        .iter()
        .find(|e| e["path"].as_str().unwrap_or("").contains("main.py"))
        .expect("must include main.py artifact");
    assert_eq!(
        py_entry["content_type"].as_str().unwrap_or(""),
        "text/x-python",
        "main.py must have content_type=text/x-python"
    );
}

// ---------------------------------------------------------------------------
// Test 5: generate_broadcasts_generate_complete_to_ws
// ---------------------------------------------------------------------------

/// WS subscribe, POST generate, assert GenerateComplete envelope arrives.
#[tokio::test]
async fn generate_broadcasts_generate_complete_to_ws() {
    use std::time::Duration;
    use tokio::time::timeout;

    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let session_id = "test-gen-ws-01";
    let session_dir = setup_session(&workspace, session_id);
    write_spec_md(&session_dir);
    write_stub_artifacts(&session_dir);

    let args_file = dir.path().join("recorded_args.txt");
    let args_path = args_file.to_string_lossy().into_owned();
    let script_path = write_stub_script(&dir, 0, "Generated 2 file(s)\n", "", &args_path);
    let workspace_str = workspace.to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &script_path);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let addr = start_server().await;

    // Open WS, drain 3 initial IR messages.
    let mut ws = ws_upgrade(addr).await;
    timeout(Duration::from_secs(2), async {
        let mut count = 0usize;
        while count < 3 {
            match ws_recv_frame(&mut ws).await {
                Some((op, _)) if op == OP_TEXT => count += 1,
                Some((op, _)) if op == OP_CLOSE => break,
                None => break,
                _ => {}
            }
        }
    })
    .await
    .expect("drain initial messages");

    // POST generate in a separate task.
    let post_handle = tokio::spawn(async move {
        let body = format!(r#"{{"session_id":"{session_id}","from_capsule":false}}"#);
        http_post_json(addr, "/api/generate", &body).await
    });

    // Wait for generate_complete WS message.
    let ws_result = timeout(Duration::from_secs(3), async {
        loop {
            match ws_recv_frame(&mut ws).await {
                Some((op, payload)) if op == OP_TEXT => {
                    let text = String::from_utf8(payload).unwrap_or_default();
                    if text.contains("generate_complete") {
                        return Some(text);
                    }
                }
                Some((op, _)) if op == OP_CLOSE => return None,
                None => return None,
                _ => {}
            }
        }
    })
    .await
    .expect("must receive generate_complete within 3s");

    let ws_msg = ws_result.expect("WS must receive generate_complete");

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    let (post_status, _) = post_handle.await.expect("post completed");
    assert_eq!(post_status, 200, "POST must return 200");

    assert!(
        ws_msg.contains("\"msg_type\":\"generate_complete\""),
        "WS message must have msg_type=generate_complete; msg={ws_msg}"
    );
    assert!(
        ws_msg.contains("session_id"),
        "WS message must contain session_id; msg={ws_msg}"
    );
    assert!(
        ws_msg.contains("artifacts"),
        "WS message must contain artifacts; msg={ws_msg}"
    );
}

// ---------------------------------------------------------------------------
// Test 6: artifact_get_serves_html_with_correct_content_type
// ---------------------------------------------------------------------------

/// Setup artifacts/index.html; GET /api/artifact/<sid>/index.html →
/// 200 + Content-Type: text/html + body matches.
#[tokio::test]
async fn artifact_get_serves_html_with_correct_content_type() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let session_id = "test-art-get-01";
    let session_dir = setup_session(&workspace, session_id);
    write_stub_artifacts(&session_dir);
    let workspace_str = workspace.to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let addr = start_server().await;
    let path = format!("/api/artifact/{session_id}/index.html");
    let (status, body_bytes, headers) = http_get(addr, &path).await;

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(status, 200, "must return 200 for existing artifact");

    // Content-Type must be text/html.
    let ct_line = headers
        .lines()
        .find(|l| l.to_lowercase().starts_with("content-type:"))
        .unwrap_or("");
    assert!(
        ct_line.to_lowercase().contains("text/html"),
        "content-type must be text/html; got {ct_line}"
    );

    // Body must contain the expected HTML.
    let body_str = String::from_utf8_lossy(&body_bytes);
    assert!(
        body_str.contains("Hello"),
        "body must contain expected HTML content; body={body_str}"
    );
}

// ---------------------------------------------------------------------------
// Test 7: artifact_get_rejects_path_traversal
// ---------------------------------------------------------------------------

/// GET /api/artifact/<sid>/..%2Fetc/passwd → 400 (path traversal blocked).
///
/// Note: axum decodes percent-encoding before routing, so `..%2F` becomes
/// `../`, which our `is_safe_artifact_name` rejects because it contains `/`.
/// After decoding by the OS-level URL stack, `..%2F` should arrive as a
/// single path segment `..` which our guard also rejects.
/// We test both raw `..` and the encoded form.
#[tokio::test]
async fn artifact_get_rejects_path_traversal() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let session_id = "test-art-traversal-01";
    setup_session(&workspace, session_id);
    let workspace_str = workspace.to_string_lossy().into_owned();

    // Write a sentinel file one level up from the session artifacts to
    // confirm it's inaccessible.
    std::fs::write(dir.path().join("sensitive.txt"), "DO NOT SERVE").unwrap();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let addr = start_server().await;

    // Attempt 1: literal `..` in name field (single segment).
    // axum routes /:session_id/:name where :name is a single segment.
    // `..` would be the name field — rejected by is_safe_artifact_name.
    let path_dotdot = format!("/api/artifact/{session_id}/..");
    let (status_dotdot, _, _) = http_get(addr, &path_dotdot).await;

    // Attempt 2: URL-encoded form `..%2Fetc%2Fpasswd`.
    // Axum decodes this before routing; the `:name` segment might become
    // `../etc/passwd` which our guard rejects.
    let path_encoded = format!("/api/artifact/{session_id}/..%2Fetc%2Fpasswd");
    let (status_encoded, _, _) = http_get(addr, &path_encoded).await;

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    // Both traversal attempts must NOT return 200.
    assert_ne!(
        status_dotdot, 200,
        "traversal with '..' must not return 200; got {status_dotdot}"
    );
    assert_ne!(
        status_encoded, 200,
        "URL-encoded traversal must not return 200; got {status_encoded}"
    );
}

// ---------------------------------------------------------------------------
// Test (regression W6.1): generate_accepts_bare_session_id_payload
//
// §6a v2 verifier (handover/evidence/stage_phase7_web_v2_*) caught a
// frontend↔backend contract gap: `<tos-spec-result>` POSTs `{ session_id }`
// to /api/generate (only the session_id field), but `GenerateRequest`
// originally required `from_capsule: bool` without `#[serde(default)]`,
// causing serde to reject the bare payload with HTTP 422. Fix added
// #[serde(default)] to both `from_capsule` and `max_files`. This test
// locks in that contract.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn generate_accepts_bare_session_id_payload_regression_w6_1() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let session_id = "test-bare-payload-01";
    let session_dir = setup_session(&workspace, session_id);
    write_spec_md(&session_dir);

    let args_file = dir.path().join("recorded_args.txt");
    let args_path = args_file.to_string_lossy().into_owned();
    let script_path = write_stub_script(
        &dir,
        0,
        "Generated 0 file(s) under /x/artifacts/\n",
        "",
        &args_path,
    );
    let workspace_str = workspace.to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &script_path);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let addr = start_server().await;
    // BARE payload — only session_id, no from_capsule, no max_files.
    // This is what the W6 frontend <tos-spec-result> sends on the
    // "生成代码" click. Must accept (deserialize with defaults).
    let body = format!(r#"{{"session_id":"{session_id}"}}"#);
    let (status, resp_body) = http_post_json(addr, "/api/generate", &body).await;

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(
        status, 200,
        "bare {{session_id}} payload must return 200 (not 422); got {status} body={resp_body}"
    );

    // Confirm defaults applied: from_capsule=false → no --from-capsule flag.
    let recorded = std::fs::read_to_string(&args_file).unwrap_or_default();
    assert!(
        !recorded.contains("--from-capsule"),
        "default from_capsule=false must NOT emit --from-capsule flag; recorded={recorded:?}"
    );
}

#[tokio::test]
async fn generate_injects_welcome_api_key_into_all_provider_aliases() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let session_id = "test-env-aliases";
    let session_dir = setup_session(&workspace, session_id);
    write_spec_md(&session_dir);

    let args_file = dir
        .path()
        .join("recorded_args.txt")
        .to_string_lossy()
        .into_owned();
    let env_file = dir
        .path()
        .join("recorded_env.txt")
        .to_string_lossy()
        .into_owned();
    let script_path = write_env_recording_generate_stub(&dir, &args_file, &env_file);
    let workspace_str = workspace.to_string_lossy().into_owned();
    let api_key = "sk-session-generate-alias-test-0000000000";

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &script_path);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);
    std::env::remove_var("SILICONFLOW_API_KEY");
    std::env::remove_var("DEEPSEEK_API_KEY");
    std::env::remove_var("DEEPSEEK_API_KEY_WORKER");
    let addr = start_server().await;

    let (key_status, key_body) = http_post_json(
        addr,
        "/api/welcome/api-key",
        &format!(r#"{{"api_key":"{api_key}"}}"#),
    )
    .await;
    assert_eq!(key_status, 200, "api-key set failed; body={key_body}");

    let body = format!(r#"{{"session_id":"{session_id}"}}"#);
    let (status, resp_body) = http_post_json(addr, "/api/generate", &body).await;

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(status, 200, "generate must succeed; body={resp_body}");
    let recorded_env = std::fs::read_to_string(&env_file).expect("env recorded");
    assert!(
        recorded_env.contains(&format!("SILICONFLOW_API_KEY={api_key}")),
        "SiliconFlow alias missing from child env; env={recorded_env}"
    );
    assert!(
        recorded_env.contains(&format!("DEEPSEEK_API_KEY={api_key}")),
        "DeepSeek meta alias missing from child env; env={recorded_env}"
    );
    assert!(
        recorded_env.contains(&format!("DEEPSEEK_API_KEY_WORKER={api_key}")),
        "DeepSeek worker alias missing from child env; env={recorded_env}"
    );
}

// ---------------------------------------------------------------------------
// Test (W8): generate_retries_on_heuristic_failure_via_stub
//
// Drives the auto-retry loop in `generate_handler`. The stub script writes
// a deliberately broken artifact (500 bytes, no canvas, no keydown, etc.)
// on its first call and a heuristic-passing artifact on the second call.
//
// Assertions:
//   1. HTTP response is 200 (succeeded on retry).
//   2. JSON `total_attempts` is 2 (one retry).
//   3. WS broadcast log contains two `generate_attempt_started` and one
//      `generate_attempt_failed` for attempt=1, plus the final
//      `generate_complete`.
// ---------------------------------------------------------------------------

/// Write a per-call switching stub: attempt 1 emits a tiny broken file,
/// attempt 2 emits a heuristic-passing file. The call count is tracked on
/// disk via an integer counter file so each invocation increments it.
fn write_retry_stub_script(dir: &tempfile::TempDir, counter_path: &str) -> String {
    let script_path = dir.path().join("turingos");
    // The script reads $counter_path, increments, writes back, and based on
    // the previous value writes either a broken or a good index.html into
    // <workspace>/artifacts/. $1 is "generate", $2 is "--workspace", $3 is
    // the workspace value.
    let good_html = good_html_payload();
    let script = format!(
        r#"#!/bin/sh
set -e
N=0
if [ -f {counter_q} ]; then
  N=$(cat {counter_q})
fi
NEXT=$((N + 1))
printf '%s' "$NEXT" > {counter_q}
# Locate workspace from positional args ($1=generate $2=--workspace $3=value).
WORKSPACE="$3"
mkdir -p "$WORKSPACE/artifacts"
if [ "$N" = "0" ]; then
  # Attempt 1 → broken (tiny, no canvas, no keydown).
  printf '%s' '<html><body>broken</body></html>' > "$WORKSPACE/artifacts/index.html"
else
  # Attempt >= 2 → good heuristic-passing artifact.
  cat > "$WORKSPACE/artifacts/index.html" <<'HTMLEOF'
{good_html}
HTMLEOF
fi
exit 0
"#,
        counter_q = shell_quote(counter_path),
        good_html = good_html,
    );
    std::fs::write(&script_path, &script).expect("write retry stub");
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(&script_path).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&script_path, perms).unwrap();
    script_path.to_string_lossy().into_owned()
}

/// A heuristic-passing HTML: > 2 KB, has <canvas>, document.addEventListener('keydown'),
/// requestAnimationFrame, balanced braces and script tags, no external resources,
/// no inverted nullish guard.
fn good_html_payload() -> String {
    let mut buf = String::new();
    buf.push_str("<!DOCTYPE html><html><head><title>g</title></head><body>");
    buf.push_str("<canvas id=\"c\" width=\"200\" height=\"400\"></canvas>");
    buf.push_str("<script>\n");
    buf.push_str("let ctx = document.getElementById('c').getContext('2d');\n");
    buf.push_str("let player = { x: 0 };\n");
    buf.push_str("function tick() { ctx.fillStyle='#fff'; ctx.fillRect(player.x, 0, 10, 10); requestAnimationFrame(tick); }\n");
    buf.push_str("document.addEventListener('keydown', function(e) {\n");
    buf.push_str("  if (e.code === 'ArrowLeft') { player.x = player.x - 1; }\n");
    buf.push_str("  if (e.code === 'ArrowRight') { player.x = player.x + 1; }\n");
    buf.push_str("});\n");
    for _ in 0..40 {
        buf.push_str(
            "// padding comment line to clear minimum size heuristic threshold ok ok ok ok\n",
        );
    }
    buf.push_str("tick();\n");
    buf.push_str("</script></body></html>\n");
    buf
}

#[tokio::test]
async fn generate_retries_on_heuristic_failure_via_stub() {
    use std::time::Duration;
    use tokio::time::timeout;

    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let session_id = "test-retry-01";
    let session_dir = setup_session(&workspace, session_id);
    write_spec_md(&session_dir);

    // Counter file lives outside the session dir so it isn't wiped between
    // attempts by the handler's clear-artifacts step.
    let counter_file = dir.path().join("call_counter.txt");
    let counter_path = counter_file.to_string_lossy().into_owned();
    let script_path = write_retry_stub_script(&dir, &counter_path);

    let workspace_str = workspace.to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &script_path);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let addr = start_server().await;

    // Open WS first so we capture all broadcasts after the initial 3 IR.
    let mut ws = ws_upgrade(addr).await;
    timeout(Duration::from_secs(2), async {
        let mut count = 0usize;
        while count < 3 {
            match ws_recv_frame(&mut ws).await {
                Some((op, _)) if op == OP_TEXT => count += 1,
                Some((op, _)) if op == OP_CLOSE => break,
                None => break,
                _ => {}
            }
        }
    })
    .await
    .expect("drain initial messages");

    // POST generate in a separate task while we collect WS events.
    let post_handle = tokio::spawn(async move {
        let body = format!(r#"{{"session_id":"{session_id}"}}"#);
        http_post_json(addr, "/api/generate", &body).await
    });

    // Collect WS messages until generate_complete arrives.
    let mut messages: Vec<String> = Vec::new();
    let collect_result = timeout(Duration::from_secs(6), async {
        loop {
            match ws_recv_frame(&mut ws).await {
                Some((op, payload)) if op == OP_TEXT => {
                    let text = String::from_utf8(payload).unwrap_or_default();
                    let done = text.contains("generate_complete");
                    messages.push(text);
                    if done {
                        return;
                    }
                }
                Some((op, _)) if op == OP_CLOSE => return,
                None => return,
                _ => {}
            }
        }
    })
    .await;
    collect_result.expect("must observe generate_complete within 6s");

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    let (post_status, post_body) = post_handle.await.expect("post completed");
    assert_eq!(
        post_status, 200,
        "POST must succeed on retry; body={post_body}"
    );

    let parsed: serde_json::Value =
        serde_json::from_str(&post_body).expect("response must be JSON");
    let total_attempts = parsed["total_attempts"].as_u64().unwrap_or(0);
    assert_eq!(
        total_attempts, 2,
        "must report total_attempts=2 (succeeded on retry); body={post_body}"
    );

    // Count broadcast types.
    let started_count = messages
        .iter()
        .filter(|m| m.contains("\"msg_type\":\"generate_attempt_started\""))
        .count();
    let failed_count = messages
        .iter()
        .filter(|m| m.contains("\"msg_type\":\"generate_attempt_failed\""))
        .count();
    let complete_count = messages
        .iter()
        .filter(|m| m.contains("\"msg_type\":\"generate_complete\""))
        .count();
    let attempt_update_count = messages
        .iter()
        .filter(|m| m.contains("\"msg_type\":\"agent_attempt_update\""))
        .count();
    assert_eq!(
        started_count, 2,
        "must broadcast 2 generate_attempt_started events; got {started_count}; messages={messages:?}"
    );
    assert_eq!(
        failed_count, 1,
        "must broadcast 1 generate_attempt_failed event; got {failed_count}; messages={messages:?}"
    );
    assert_eq!(
        complete_count, 1,
        "must broadcast 1 generate_complete event; got {complete_count}; messages={messages:?}"
    );
    assert_eq!(
        attempt_update_count, 1,
        "must broadcast 1 agent_attempt_update before generate_complete so the panel refetches replay-derived market view; got {attempt_update_count}; messages={messages:?}"
    );
    let attempt_update_pos = messages
        .iter()
        .position(|m| m.contains("\"msg_type\":\"agent_attempt_update\""))
        .expect("agent_attempt_update must be present");
    let complete_pos = messages
        .iter()
        .position(|m| m.contains("\"msg_type\":\"generate_complete\""))
        .expect("generate_complete must be present");
    assert!(
        attempt_update_pos < complete_pos,
        "agent_attempt_update must arrive before generate_complete; messages={messages:?}"
    );
}
