//! TRACE_MATRIX FC1-N10: Phase 7 W4 smoke tests — verifies the write path:
//! POST /api/task/open → input validation → CLI shellout → WS broadcast.
//!
//! Gated on `#![cfg(feature = "web")]` so non-web builds never see this.
//! Run with: `cargo test --test cli_web_write_smoke --features web`
//!
//! ## Stub pattern
//!
//! Tests that exercise the shell-out use `TURINGOS_BACKEND_OVERRIDE` to point
//! the handler at a small shell script written to a tempdir. The script records
//! the argv it receives (to a file in the same tempdir) and then exits with a
//! configured exit code + stdout/stderr. This mirrors the `TURINGOS_BIN_DIR`
//! stub pattern in `tests/cli_wrapper_plumbing.rs`.
//!
//! No real `turingos` workspace is needed; no real CLI binary is invoked.
//!
//! ## WS broadcast test
//!
//! Test 6 opens a WebSocket connection BEFORE posting the request, then sends
//! the POST, and asserts that a `task_created` message arrives on the WS
//! within 2 seconds. Uses the raw-TCP RFC 6455 frame reader from
//! `cli_web_ws_smoke.rs` (inlined here to avoid cross-test module deps).
//!
//! ## Env-var isolation
//!
//! Tests that mutate `TURINGOS_BACKEND_OVERRIDE` and `TURINGOS_WEB_WORKSPACE`
//! acquire `ENV_LOCK` (a process-global Mutex) before setting and release it
//! after clearing. This prevents test parallelism from corrupting each other's
//! env state. Validation-only tests (tests 1–3) do not need the lock because
//! they never reach the shell-out path.
#![cfg(feature = "web")]

#[path = "../src/web/mod.rs"]
mod web;

use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

/// Process-global async lock for tests that mutate TURINGOS_BACKEND_OVERRIDE /
/// TURINGOS_WEB_WORKSPACE. Using tokio::sync::Mutex so the guard can be held
/// across .await points safely.
static ENV_LOCK: std::sync::OnceLock<Mutex<()>> = std::sync::OnceLock::new();

fn env_lock() -> &'static Mutex<()> {
    ENV_LOCK.get_or_init(|| Mutex::new(()))
}

// ---------------------------------------------------------------------------
// Server startup helpers
// ---------------------------------------------------------------------------

/// Start the router (with AppState) on a random port.
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

// ---------------------------------------------------------------------------
// HTTP POST helper
// ---------------------------------------------------------------------------

/// Send a POST with JSON body and return (status_code, body_string).
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

    let status_line = head.lines().next().unwrap_or("").to_string();
    let status_code: u16 = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    (status_code, resp_body)
}

// ---------------------------------------------------------------------------
// Stub script helpers
// ---------------------------------------------------------------------------

/// Write a POSIX shell stub script to a tempdir.
/// `exit_code`: 0 for success, non-zero for failure.
/// `stdout_content`: written to stdout.
/// `stderr_content`: written to stderr.
/// `record_args_file`: path where the stub writes its received argv (one arg per line).
/// Returns the path to the script.
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
    // Make executable
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(&script_path)
        .expect("stat stub script")
        .permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&script_path, perms).expect("chmod stub script");
    script_path.to_string_lossy().into_owned()
}

/// Very simple shell quoting: wrap in single quotes, escape embedded single quotes.
fn shell_quote(s: &str) -> String {
    let escaped = s.replace('\'', "'\\''");
    format!("'{escaped}'")
}

// ---------------------------------------------------------------------------
// WebSocket helpers (inlined from cli_web_ws_smoke.rs pattern)
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
// Test 1: task_open_rejects_invalid_problem_id
// ---------------------------------------------------------------------------

/// FC1-N5 shielding: path-traversal problem_id must return 400 invalid_input.
#[tokio::test]
async fn task_open_rejects_invalid_problem_id() {
    let addr = start_server().await;
    let body = r#"{"problem_id":"../../etc/passwd","bounty":1000,"agent_id":"agent_0"}"#;
    let (status, resp_body) = http_post_json(addr, "/api/task/open", body).await;
    assert_eq!(status, 400, "must reject invalid problem_id with 400");
    assert!(
        resp_body.contains("invalid_input"),
        "response must contain kind=invalid_input; body={resp_body}"
    );
}

// ---------------------------------------------------------------------------
// Test 2: task_open_rejects_zero_bounty
// ---------------------------------------------------------------------------

#[tokio::test]
async fn task_open_rejects_zero_bounty() {
    let addr = start_server().await;
    let body = r#"{"problem_id":"prob_001","bounty":0,"agent_id":"agent_0"}"#;
    let (status, resp_body) = http_post_json(addr, "/api/task/open", body).await;
    assert_eq!(status, 400, "must reject bounty=0 with 400");
    assert!(
        resp_body.contains("invalid_input"),
        "response must contain kind=invalid_input; body={resp_body}"
    );
}

// ---------------------------------------------------------------------------
// Test 3: task_open_rejects_oversized_agent_id
// ---------------------------------------------------------------------------

#[tokio::test]
async fn task_open_rejects_oversized_agent_id() {
    let addr = start_server().await;
    // 65-char agent_id
    let agent_id = "a".repeat(65);
    let body = format!(r#"{{"problem_id":"prob_001","bounty":1000,"agent_id":"{agent_id}"}}"#);
    let (status, resp_body) = http_post_json(addr, "/api/task/open", &body).await;
    assert_eq!(status, 400, "must reject 65-char agent_id with 400");
    assert!(
        resp_body.contains("invalid_input"),
        "response must contain kind=invalid_input; body={resp_body}"
    );
}

// ---------------------------------------------------------------------------
// Test 4: task_open_invokes_shellout_with_correct_args_via_override
// ---------------------------------------------------------------------------

/// Verifies the full success path: stub records argv; response has task_id.
#[tokio::test]
async fn task_open_invokes_shellout_with_correct_args_via_override() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let args_file = dir.path().join("recorded_args.txt");
    let args_path = args_file.to_string_lossy().into_owned();
    let script_path = write_stub_script(&dir, 0, "task_id: t_abc123\n", "", &args_path);

    // Use a known workspace dir so we can assert --chaintape.
    let workspace = dir.path().to_string_lossy().into_owned();

    // Hold ENV_LOCK across env mutation + server start + request.
    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &script_path);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace);

    let addr = start_server().await;
    let body = r#"{"problem_id":"prob-001","bounty":5000,"agent_id":"agent_0"}"#;
    let (status, resp_body) = http_post_json(addr, "/api/task/open", body).await;

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(status, 200, "stub exit 0 must return 200; body={resp_body}");
    assert!(
        resp_body.contains("t_abc123"),
        "response must contain task_id=t_abc123; body={resp_body}"
    );
    assert!(
        resp_body.contains("\"status\":\"created\""),
        "response must contain status=created; body={resp_body}"
    );

    // Verify the stub recorded the expected args.
    let recorded = std::fs::read_to_string(&args_file).expect("stub should have written args file");
    let args: Vec<&str> = recorded.lines().collect();

    // Expected: task open --problem prob-001 --bounty 5000 --agent-id agent_0 --chaintape <workspace>
    assert!(
        args.contains(&"task"),
        "stub must receive 'task' arg; recorded={recorded:?}"
    );
    assert!(
        args.contains(&"open"),
        "stub must receive 'open' arg; recorded={recorded:?}"
    );
    assert!(
        args.contains(&"--problem"),
        "stub must receive --problem flag; recorded={recorded:?}"
    );
    assert!(
        args.contains(&"prob-001"),
        "stub must receive problem_id value; recorded={recorded:?}"
    );
    assert!(
        args.contains(&"--bounty"),
        "stub must receive --bounty flag; recorded={recorded:?}"
    );
    assert!(
        args.contains(&"5000"),
        "stub must receive bounty value; recorded={recorded:?}"
    );
    assert!(
        args.contains(&"--agent-id"),
        "stub must receive --agent-id flag; recorded={recorded:?}"
    );
    assert!(
        args.contains(&"agent_0"),
        "stub must receive agent_id value; recorded={recorded:?}"
    );
    assert!(
        args.contains(&"--chaintape"),
        "stub must receive --chaintape flag; recorded={recorded:?}"
    );
    assert!(
        args.iter().any(|a| a.contains(&workspace[..])),
        "stub must receive workspace path; recorded={recorded:?}"
    );
}

// ---------------------------------------------------------------------------
// Test 5: task_open_returns_500_on_shellout_failure
// ---------------------------------------------------------------------------

#[tokio::test]
async fn task_open_returns_500_on_shellout_failure() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let args_file = dir.path().join("recorded_args.txt");
    let args_path = args_file.to_string_lossy().into_owned();
    let script_path =
        write_stub_script(&dir, 1, "", "fake failure: workspace not found", &args_path);

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &script_path);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", dir.path().to_str().unwrap_or("."));

    let addr = start_server().await;
    let body = r#"{"problem_id":"prob-001","bounty":5000,"agent_id":"agent_0"}"#;
    let (status, resp_body) = http_post_json(addr, "/api/task/open", body).await;

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(status, 500, "stub exit 1 must return 500; body={resp_body}");
    assert!(
        resp_body.contains("shellout_failed"),
        "response must contain kind=shellout_failed; body={resp_body}"
    );
    assert!(
        resp_body.contains("fake failure"),
        "response reason must contain stderr content; body={resp_body}"
    );
}

// ---------------------------------------------------------------------------
// Test 6: task_open_broadcasts_to_ws_on_success
// ---------------------------------------------------------------------------

/// Open a WS connection, POST a successful task-open, assert that `task_created`
/// message arrives on the WS connection within 2 seconds.
#[tokio::test]
async fn task_open_broadcasts_to_ws_on_success() {
    use std::time::Duration;
    use tokio::time::timeout;

    let dir = tempfile::TempDir::new().expect("tempdir");
    let args_file = dir.path().join("recorded_args.txt");
    let args_path = args_file.to_string_lossy().into_owned();
    let script_path = write_stub_script(&dir, 0, "task_id: t_broadcast_test\n", "", &args_path);

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &script_path);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", dir.path().to_str().unwrap_or("."));

    // Start server once (shared for both WS and HTTP).
    let addr = start_server().await;

    // Open WS connection and drain the 3 initial ir_update messages.
    let mut ws = ws_upgrade(addr).await;
    {
        let drain = timeout(Duration::from_secs(2), async {
            let mut count = 0usize;
            while count < 3 {
                match ws_recv_frame(&mut ws).await {
                    Some((op, _)) if op == OP_TEXT => count += 1,
                    Some((op, _)) if op == OP_CLOSE => break,
                    None => break,
                    _ => {}
                }
            }
        });
        drain
            .await
            .expect("should drain 3 initial messages within 2s");
    }

    // POST the task open (in a separate task so we can drive both sides).
    let post_handle = tokio::spawn(async move {
        let body = r#"{"problem_id":"prob-bcast","bounty":100,"agent_id":"agent_1"}"#;
        http_post_json(addr, "/api/task/open", body).await
    });

    // Wait for the task_created message on the WS (within 2 sec).
    let ws_result = timeout(Duration::from_secs(2), async {
        loop {
            match ws_recv_frame(&mut ws).await {
                Some((op, payload)) if op == OP_TEXT => {
                    let text = String::from_utf8(payload).unwrap_or_default();
                    if text.contains("task_created") {
                        return Some(text);
                    }
                    // Skip ir_update or other frames.
                }
                Some((op, _)) if op == OP_CLOSE => return None,
                None => return None,
                _ => {}
            }
        }
    })
    .await
    .expect("must receive task_created within 2s");

    let ws_msg = ws_result.expect("WS must receive a task_created message");

    // Clean up env before dropping guard.
    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    // Verify POST succeeded.
    let (status, _) = post_handle.await.expect("post task completed");
    assert_eq!(status, 200, "POST /api/task/open must return 200");

    // Verify the WS message shape.
    assert!(
        ws_msg.contains("\"msg_type\":\"task_created\""),
        "WS message must have msg_type=task_created; msg={ws_msg}"
    );
    assert!(
        ws_msg.contains("t_broadcast_test"),
        "WS message must contain task_id; msg={ws_msg}"
    );
    assert!(
        ws_msg.contains("prob-bcast"),
        "WS message must contain problem_id; msg={ws_msg}"
    );
    assert!(
        ws_msg.contains("agent_1"),
        "WS message must contain agent_id; msg={ws_msg}"
    );
}

// ---------------------------------------------------------------------------
// Test 7: task_open_visible_in_api_tasks_after_post
// ---------------------------------------------------------------------------

/// W4.2 §6a Page 4 criterion: after POST /api/task/open succeeds, GET
/// /api/tasks must return a TaskCardBlock containing the new task_id.
///
/// Uses the stub pattern to avoid a real CLI invocation.
#[tokio::test]
async fn task_open_visible_in_api_tasks_after_post() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let args_file = dir.path().join("recorded_args.txt");
    let args_path = args_file.to_string_lossy().into_owned();
    let script_path = write_stub_script(&dir, 0, "task_id: t_visibility_test\n", "", &args_path);

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &script_path);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", dir.path().to_str().unwrap_or("."));

    let addr = start_server().await;

    // POST the task open.
    let body = r#"{"problem_id":"prob-vis","bounty":2000,"agent_id":"agent_vis"}"#;
    let (post_status, _) = http_post_json(addr, "/api/task/open", body).await;

    // GET /api/tasks immediately after.
    let mut stream = tokio::net::TcpStream::connect(addr)
        .await
        .expect("connect for GET /api/tasks");
    let get_request = "GET /api/tasks HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n";
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    stream
        .write_all(get_request.as_bytes())
        .await
        .expect("write GET");
    let mut buf = Vec::new();
    stream
        .read_to_end(&mut buf)
        .await
        .expect("read GET response");
    let response = String::from_utf8_lossy(&buf).into_owned();

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(post_status, 200, "POST must return 200");
    assert!(
        response.contains("t_visibility_test"),
        "/api/tasks response must contain the new task_id; response={response}"
    );
    // The synthesized entry has status=open.
    assert!(
        response.contains("\"open\""),
        "/api/tasks response must contain status=open for new entry; response={response}"
    );
}

// ---------------------------------------------------------------------------
// Test 8: task_open_visible_in_tasks_html_after_post
// ---------------------------------------------------------------------------

/// W4.2 §6a Page 4 criterion: after POST /api/task/open succeeds, GET /tasks
/// (HTML render) must contain the new task_id as a substring in the body.
#[tokio::test]
async fn task_open_visible_in_tasks_html_after_post() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let args_file = dir.path().join("recorded_args.txt");
    let args_path = args_file.to_string_lossy().into_owned();
    let script_path = write_stub_script(&dir, 0, "task_id: t_html_visible\n", "", &args_path);

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &script_path);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", dir.path().to_str().unwrap_or("."));

    let addr = start_server().await;

    // POST the task open.
    let body = r#"{"problem_id":"prob-html","bounty":3000,"agent_id":"agent_html"}"#;
    let (post_status, _) = http_post_json(addr, "/api/task/open", body).await;

    // GET /tasks (HTML).
    let mut stream = tokio::net::TcpStream::connect(addr)
        .await
        .expect("connect for GET /tasks");
    let get_request = "GET /tasks HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n";
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    stream
        .write_all(get_request.as_bytes())
        .await
        .expect("write GET /tasks");
    let mut buf = Vec::new();
    stream
        .read_to_end(&mut buf)
        .await
        .expect("read HTML response");
    let response = String::from_utf8_lossy(&buf).into_owned();

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(post_status, 200, "POST must return 200");
    assert!(
        response.contains("t_html_visible"),
        "/tasks HTML must contain the new task_id; response snippet={}",
        &response[..response.len().min(1024)]
    );
}

// ---------------------------------------------------------------------------
// Test 9: task_store_cap_enforced_at_1001
// ---------------------------------------------------------------------------

/// Directly push 1001 entries to TaskMemoryStore and verify the cap (len ≤ 1000).
#[test]
fn task_store_cap_enforced_at_1001() {
    let store = web::store::TaskMemoryStore::new();
    for i in 0..1001u32 {
        store.push(web::store::TaskEntry {
            task_id: format!("t_cap_{i:04}"),
            agent_id: "agent_cap".to_string(),
            problem_id: "prob_cap".to_string(),
            bounty: 1,
            created_at_unix: u64::from(i),
        });
    }
    assert!(
        store.len() <= 1000,
        "store must cap at ≤1000 entries; got {}",
        store.len()
    );
}

// ---------------------------------------------------------------------------
// Test 10: TB-SOFTWARE-3-0 Atom S1 — CLI exit 0 with unparseable stdout returns
// 502 BAD_GATEWAY (NOT a synthesized t_hash_* canonical id, NOT 200-with-warning).
// ---------------------------------------------------------------------------

#[tokio::test]
async fn task_open_returns_502_on_unparseable_stdout() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let args_file = dir.path().join("recorded_args.txt");
    let args_path = args_file.to_string_lossy().into_owned();
    // Stub exits 0 but emits stdout the parser CANNOT recognize as a task id.
    // Pre-S1: synthesized `t_hash_xxxxxxxx` + 200 OK + write to store + broadcast.
    // Post-S1: 502 BAD_GATEWAY + kind="task_id_parse_failed" + no store write.
    let script_path = write_stub_script(
        &dir,
        0,
        "opening task...\nsome arbitrary CLI output\nno task identifier here\n",
        "",
        &args_path,
    );

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &script_path);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", dir.path().to_str().unwrap_or("."));

    let addr = start_server().await;
    let body = r#"{"problem_id":"prob-001","bounty":5000,"agent_id":"agent_0"}"#;
    let (status, resp_body) = http_post_json(addr, "/api/task/open", body).await;

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(
        status, 502,
        "unparseable stdout MUST return 502 BAD_GATEWAY; body={resp_body}"
    );
    assert!(
        resp_body.contains("task_id_parse_failed"),
        "response must contain kind=task_id_parse_failed; body={resp_body}"
    );
    assert!(
        !resp_body.contains("t_hash_"),
        "S1 KILL: response must NOT contain synthesized t_hash_* id; body={resp_body}"
    );
    assert!(
        !resp_body.contains("\"status\":\"created\""),
        "response must NOT claim status=created on parse failure; body={resp_body}"
    );
}
