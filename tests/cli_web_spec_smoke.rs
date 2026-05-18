//! TRACE_MATRIX FC1-N5 + FC2-N16: Phase 7 W5 smoke tests — verifies the spec
//! interview endpoints:
//!   GET  /api/spec/questions → 200 + 8 questions
//!   POST /api/spec/submit    → input validation, CLI shellout (via stub),
//!                              spec.md read, WS broadcast of SpecComplete.
//!
//! Gated on `#![cfg(feature = "web")]` so non-web builds never see this.
//! Run with: `cargo test --test cli_web_spec_smoke --features web`
//!
//! ## Stub pattern
//!
//! Tests that exercise the shellout use `TURINGOS_BACKEND_OVERRIDE` to point
//! the handler at a small shell script written to a tempdir. The script
//! records argv (to a file in the tempdir) and emits configurable
//! exit code + stdout + stderr. This mirrors the pattern in
//! `cli_web_write_smoke.rs`.
//!
//! ## Env-var isolation
//!
//! Tests that mutate `TURINGOS_BACKEND_OVERRIDE` / `TURINGOS_WEB_WORKSPACE`
//! acquire `ENV_LOCK` (a process-global tokio Mutex) before setting and
//! release it after clearing.
#![cfg(feature = "web")]

#[path = "../src/web/mod.rs"]
mod web;

use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

// Process-global async lock for env mutation.
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

async fn http_get(addr: SocketAddr, path: &str) -> (u16, String) {
    let mut stream = tokio::net::TcpStream::connect(addr)
        .await
        .expect("connect to test server");
    let request = format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n");
    stream
        .write_all(request.as_bytes())
        .await
        .expect("write request");
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await.expect("read response");
    let raw = String::from_utf8_lossy(&buf).into_owned();
    let (head, body) = if let Some(idx) = raw.find("\r\n\r\n") {
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
    (status_code, body)
}

async fn http_post_json(addr: SocketAddr, path: &str, body: &str) -> (u16, String) {
    let mut stream = tokio::net::TcpStream::connect(addr)
        .await
        .expect("connect to test server");
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
    stream
        .write_all(request.as_bytes())
        .await
        .expect("write request");
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await.expect("read response");
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
// Stub script helpers (identical pattern to cli_web_write_smoke.rs)
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
    std::fs::write(&script_path, &script_content).expect("write stub script");
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(&script_path)
        .expect("stat stub")
        .permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&script_path, perms).expect("chmod stub");
    script_path.to_string_lossy().into_owned()
}

fn shell_quote(s: &str) -> String {
    let escaped = s.replace('\'', "'\\''");
    format!("'{escaped}'")
}

// ---------------------------------------------------------------------------
// WebSocket helpers (RFC 6455 raw TCP — same pattern as cli_web_write_smoke.rs)
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
        stream.read_exact(&mut b).await.expect("read upgrade byte");
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
// Helper: build a valid 8-answer JSON array body
// ---------------------------------------------------------------------------

fn valid_answers_body() -> String {
    let answers: Vec<String> = (0..8)
        .map(|i| format!("This is a valid answer for question {i}"))
        .collect();
    let json_answers: Vec<String> = answers.iter().map(|a| format!("\"{a}\"")).collect();
    format!("{{\"answers\":[{}]}}", json_answers.join(","))
}

// ---------------------------------------------------------------------------
// Test 1: spec_questions_returns_8_strings
// ---------------------------------------------------------------------------

/// GET /api/spec/questions must return 200 + JSON object with "questions": [8 strings].
#[tokio::test]
async fn spec_questions_returns_8_strings() {
    let addr = start_server().await;
    let (status, body) = http_get(addr, "/api/spec/questions").await;
    assert_eq!(status, 200, "must return 200; body={body}");

    // Parse the response body as JSON.
    let parsed: serde_json::Value =
        serde_json::from_str(&body).expect("response must be valid JSON");
    let questions = parsed["questions"]
        .as_array()
        .expect("response must have 'questions' array");
    assert_eq!(
        questions.len(),
        8,
        "must return exactly 8 questions; got {}; body={body}",
        questions.len()
    );
    for (i, q) in questions.iter().enumerate() {
        assert!(q.is_string(), "question {i} must be a string; got {q:?}");
        assert!(
            !q.as_str().unwrap_or("").is_empty(),
            "question {i} must not be empty"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 2: spec_submit_rejects_empty_answer
// ---------------------------------------------------------------------------

/// POST /api/spec/submit with one empty answer must return 400 + invalid_input.
#[tokio::test]
async fn spec_submit_rejects_empty_answer() {
    let addr = start_server().await;
    // Build 8 answers where answer[2] is empty.
    let mut answers: Vec<&str> = (0..8).map(|_| "valid answer text").collect();
    answers[2] = "";
    let json_arr: Vec<String> = answers.iter().map(|a| format!("\"{a}\"")).collect();
    let body = format!("{{\"answers\":[{}]}}", json_arr.join(","));

    let (status, resp_body) = http_post_json(addr, "/api/spec/submit", &body).await;
    assert_eq!(
        status, 400,
        "empty answer must return 400; body={resp_body}"
    );
    assert!(
        resp_body.contains("invalid_input"),
        "must contain kind=invalid_input; body={resp_body}"
    );
}

// ---------------------------------------------------------------------------
// Test 3: spec_submit_rejects_oversized_answer
// ---------------------------------------------------------------------------

/// POST /api/spec/submit with a 4097-char answer must return 400 + invalid_input.
#[tokio::test]
async fn spec_submit_rejects_oversized_answer() {
    let addr = start_server().await;
    // Build 8 answers where answer[0] is 4097 chars.
    let big_answer = "x".repeat(4097);
    let mut json_parts: Vec<String> = (0..8).map(|_| format!("\"valid answer\"")).collect();
    json_parts[0] = format!("\"{big_answer}\"");
    let body = format!("{{\"answers\":[{}]}}", json_parts.join(","));

    let (status, resp_body) = http_post_json(addr, "/api/spec/submit", &body).await;
    assert_eq!(
        status, 400,
        "oversized answer must return 400; body={resp_body}"
    );
    assert!(
        resp_body.contains("invalid_input"),
        "must contain kind=invalid_input; body={resp_body}"
    );
}

// ---------------------------------------------------------------------------
// Test 4: spec_submit_invokes_shellout_with_correct_args_via_override
// ---------------------------------------------------------------------------

/// Stub the shellout; POST 8 valid answers; assert stub recorded
/// `spec --workspace <session-dir> --answers-file <path> --lang zh`;
/// assert response 200 with session_id + non-empty spec_md.
#[tokio::test]
async fn spec_submit_invokes_shellout_with_correct_args_via_override() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let args_file = dir.path().join("recorded_args.txt");
    let args_path = args_file.to_string_lossy().into_owned();
    let workspace = dir.path().to_string_lossy().into_owned();

    // The stub needs to write spec.md so the handler can read it.
    // We pre-write spec.md to the location the handler will look:
    // <workspace>/sessions/<session_id>/spec.md — but we don't know
    // the session_id yet. Instead, we make the stub write a spec.md
    // by parsing the workspace arg from its argv.
    // Simpler: write a stub that creates spec.md at the path passed as --workspace.
    let script_path = dir.path().join("turingos");
    let spec_content = "# Stub spec.md\n\nThis is a stub spec.\n\n<!-- TURINGOS_SPEC_END -->\n";
    let stdout_content = format!(
        "Spec interview complete.\n  spec.md            -> /tmp/x/spec.md\n  CAS capsule CID    -> deadbeef0001\n"
    );
    // The stub: record args, create spec.md at --workspace value, write stdout.
    let script_body = format!(
        r#"#!/bin/sh
# Record all args
printf '%s\n' "$@" > {args_path_q}
# Parse --workspace value (it's the arg after --workspace)
ws=""
while [ $# -gt 0 ]; do
    if [ "$1" = "--workspace" ]; then
        ws="$2"
        shift 2
    else
        shift
    fi
done
# Write spec.md at the workspace location
if [ -n "$ws" ]; then
    printf '%s' {spec_q} > "$ws/spec.md"
fi
printf '%s' {stdout_q}
exit 0
"#,
        args_path_q = shell_quote(&args_path),
        spec_q = shell_quote(spec_content),
        stdout_q = shell_quote(&stdout_content),
    );
    std::fs::write(&script_path, &script_body).expect("write stub script");
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(&script_path)
        .expect("stat stub")
        .permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&script_path, perms).expect("chmod stub");
    let script_path_str = script_path.to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &script_path_str);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace);

    let addr = start_server().await;
    let body = valid_answers_body();
    let (status, resp_body) = http_post_json(addr, "/api/spec/submit", &body).await;

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(status, 200, "stub exit 0 must return 200; body={resp_body}");

    // Parse response.
    let parsed: serde_json::Value =
        serde_json::from_str(&resp_body).expect("response must be valid JSON");
    assert!(
        parsed["session_id"].is_string(),
        "response must have session_id; body={resp_body}"
    );
    assert!(
        !parsed["spec_md"].as_str().unwrap_or("").is_empty(),
        "response must have non-empty spec_md; body={resp_body}"
    );
    assert_eq!(
        parsed["capsule_cid"].as_str(),
        Some("deadbeef0001"),
        "capsule_cid must be parsed from stdout; body={resp_body}"
    );

    // Verify stub received correct args.
    let recorded = std::fs::read_to_string(&args_file).expect("stub should have written args file");
    let args: Vec<&str> = recorded.lines().collect();
    assert!(
        args.contains(&"spec"),
        "stub must receive 'spec' subcommand; recorded={recorded:?}"
    );
    assert!(
        args.contains(&"--workspace"),
        "stub must receive --workspace; recorded={recorded:?}"
    );
    assert!(
        args.contains(&"--answers-file"),
        "stub must receive --answers-file; recorded={recorded:?}"
    );
    assert!(
        args.contains(&"--lang"),
        "stub must receive --lang; recorded={recorded:?}"
    );
    assert!(
        args.contains(&"zh"),
        "stub must receive lang=zh; recorded={recorded:?}"
    );
    // answers.json path should contain workspace path.
    assert!(
        args.iter().any(|a| a.contains("answers.json")),
        "stub must receive answers.json path; recorded={recorded:?}"
    );
}

// ---------------------------------------------------------------------------
// Test 5: spec_submit_broadcasts_spec_complete_to_ws
// ---------------------------------------------------------------------------

/// Open WS, POST spec submit via stub, assert receives SpecComplete envelope.
#[tokio::test]
async fn spec_submit_broadcasts_spec_complete_to_ws() {
    use std::time::Duration;
    use tokio::time::timeout;

    let dir = tempfile::TempDir::new().expect("tempdir");
    let args_file = dir.path().join("recorded_args.txt");
    let args_path = args_file.to_string_lossy().into_owned();
    let workspace = dir.path().to_string_lossy().into_owned();

    // Script: record args, write spec.md, print CID in stdout.
    let script_path = dir.path().join("turingos");
    let spec_content = "# Stub spec.md for WS test\n\n<!-- TURINGOS_SPEC_END -->\n";
    let stdout_content = "Spec interview complete.\n  CAS capsule CID    -> aabbcc001122\n";
    let script_body = format!(
        r#"#!/bin/sh
printf '%s\n' "$@" > {args_path_q}
ws=""
while [ $# -gt 0 ]; do
    if [ "$1" = "--workspace" ]; then ws="$2"; shift 2; else shift; fi
done
if [ -n "$ws" ]; then
    printf '%s' {spec_q} > "$ws/spec.md"
fi
printf '%s' {stdout_q}
exit 0
"#,
        args_path_q = shell_quote(&args_path),
        spec_q = shell_quote(spec_content),
        stdout_q = shell_quote(stdout_content),
    );
    std::fs::write(&script_path, &script_body).expect("write stub");
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(&script_path).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&script_path, perms).unwrap();

    let _guard = env_lock().lock().await;
    std::env::set_var(
        "TURINGOS_BACKEND_OVERRIDE",
        script_path.to_string_lossy().as_ref(),
    );
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace);

    let addr = start_server().await;

    // Open WS and drain the 3 initial ir_update messages.
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
    .expect("drain initial messages within 2s");

    // POST in a separate task.
    let post_handle = tokio::spawn(async move {
        let body = valid_answers_body();
        http_post_json(addr, "/api/spec/submit", &body).await
    });

    // Wait for spec_complete on WS.
    let ws_result = timeout(Duration::from_secs(3), async {
        loop {
            match ws_recv_frame(&mut ws).await {
                Some((op, payload)) if op == OP_TEXT => {
                    let text = String::from_utf8(payload).unwrap_or_default();
                    if text.contains("spec_complete") {
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
    .expect("must receive spec_complete within 3s");

    let ws_msg = ws_result.expect("WS must receive spec_complete message");

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    let (post_status, _) = post_handle.await.expect("post completed");
    assert_eq!(post_status, 200, "POST must return 200");

    assert!(
        ws_msg.contains("\"msg_type\":\"spec_complete\""),
        "WS message must have msg_type=spec_complete; msg={ws_msg}"
    );
    assert!(
        ws_msg.contains("session_id"),
        "WS message must contain session_id; msg={ws_msg}"
    );
}

// ---------------------------------------------------------------------------
// Test 6: spec_submit_returns_500_on_shellout_failure
// ---------------------------------------------------------------------------

/// Stub exits 1; POST → 500 + shellout_failed.
#[tokio::test]
async fn spec_submit_returns_500_on_shellout_failure() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let args_file = dir.path().join("recorded_args.txt");
    let args_path = args_file.to_string_lossy().into_owned();
    let script_path = write_stub_script(&dir, 1, "", "fake spec failure: LLM error", &args_path);

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &script_path);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", dir.path().to_str().unwrap_or("."));

    let addr = start_server().await;
    let body = valid_answers_body();
    let (status, resp_body) = http_post_json(addr, "/api/spec/submit", &body).await;

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(status, 500, "stub exit 1 must return 500; body={resp_body}");
    assert!(
        resp_body.contains("shellout_failed"),
        "must contain kind=shellout_failed; body={resp_body}"
    );
    assert!(
        resp_body.contains("fake spec failure"),
        "reason must contain stderr content; body={resp_body}"
    );
}
