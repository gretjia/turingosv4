//! TRACE_MATRIX FC1-N5 + FC2-N16: Phase 7 W1+W2+W5+W6+W7 smoke tests — verifies
//! all routes are wired (Phase 5: -2 static spec routes; was 20, now 18):
//!   W1: 7 HTTP read routes
//!   W2: 1 WebSocket route
//!   W4: 1 POST /api/task/open + 1 GET /static/main.js
//!   P5: 1 driven-grill route (POST /api/spec/turn) — replaces former W5 pair
//!       (GET /api/spec/questions + POST /api/spec/submit) removed in Phase 5
//!   W5: 1 generate route (POST /api/generate) +
//!       1 artifact route (GET /api/artifact/:session_id/:name)
//!   W6: 1 HTML route (GET /build — driven-grill interview centerpiece)
//!   W7: 1 HTML route (GET /welcome — onboarding wizard) + 5 API endpoints
//!       (status + api-key + init + llm-config + agent-deploy)
//!
//! Gated on `#[cfg(feature = "web")]` so non-web builds never see this.
//! Run with: `cargo test --test cli_web_routes_smoke --features web`
//!
//! Implementation note: tests spin up a real TCP listener on a random
//! OS-assigned port (bind to 127.0.0.1:0) using tokio, send real HTTP/1.1
//! requests via tokio::net::TcpStream, and parse responses manually.
//! This avoids any dependency on `tower::ServiceExt` that is not a direct
//! Cargo.toml dependency.
#![cfg(feature = "web")]

// Mirror the same path-based module declaration used in `turingos_web.rs`
// so the test exercises the exact same module tree.
#[path = "../src/web/mod.rs"]
mod web;

use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// ---------------------------------------------------------------------------
// Helper: start the router on a random port, return the bound address.
// The server task is spawned and runs until the test process exits.
// ---------------------------------------------------------------------------

async fn start_server() -> SocketAddr {
    let router = web::router::build();
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
// Helper: send a minimal HTTP/1.1 GET and return (status_line, headers, body).
// Uses raw TCP so we have zero tower/hyper client dependency.
// ---------------------------------------------------------------------------

async fn http_get(addr: SocketAddr, path: &str) -> (u16, String, String) {
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

    // Split headers from body on first blank line
    let (head, body) = if let Some(idx) = raw.find("\r\n\r\n") {
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

    (status_code, head.to_string(), body)
}

// ---------------------------------------------------------------------------
// Gate 1: all routes exist (W1/W2/W4/P5/W5/W6). Phase 5: removed the 2 static
// spec routes (questions + submit); driven-only is enforced.
//   W1:  7 HTTP GET routes returning 200
//   W2:  1 WS route returning 101
//   W4:  /static/main.js returning 200 (POST /api/task/open not tested here)
//   P5:  POST /api/spec/turn wired (existence checked via 422 on empty body)
//   W5:  POST /api/generate wired (existence checked via 422)
//        GET /api/artifact/:session_id/:name wired (existence checked via 404)
//   W6:  GET /build returning 200 (driven-grill interview page)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn router_has_all_required_routes() {
    // W7: pre-seed an onboarded workspace so GET / serves the dashboard
    // (200) instead of redirecting (303). The redirect behavior is covered
    // by cli_web_welcome_smoke::root_redirects_to_welcome_when_not_onboarded.
    let workspace = seed_onboarded_workspace();
    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", workspace.path());

    let addr = start_server().await;

    // W7: bootstrap the AppState API key so GET / renders the dashboard
    // (instead of redirecting to /welcome). This is one request to the
    // welcome API; subsequent root-redirect logic checks AppState.
    let _ = http_post_raw(
        addr,
        "/api/welcome/api-key",
        b"{\"api_key\":\"sk-routes-smoke-fixture-XXXXXXXX\"}",
    )
    .await;
    // W1+W6: The HTTP read routes must return 200.
    let http_routes = [
        "/",
        "/agents",
        "/tasks",
        "/audit",
        "/build",
        "/welcome",
        "/api/dashboard",
        "/api/agents",
        "/api/tasks",
    ];
    for path in &http_routes {
        let (status, _, _) = http_get(addr, path).await;
        assert_eq!(status, 200u16, "expected 200 for GET {path}, got {status}");
    }

    // W4.1: /static/main.js must return 200.
    let (status_js, _, _) = http_get(addr, "/static/main.js").await;
    assert_eq!(
        status_js, 200u16,
        "GET /static/main.js must return 200, got {status_js}"
    );

    // W2: The WebSocket route must return HTTP 101 Switching Protocols.
    let (status_101, _, _) = http_get_upgrade(addr, "/ws").await;
    assert_eq!(
        status_101, 101u16,
        "GET /ws with Upgrade: websocket must return 101, got {status_101}"
    );

    // P5: POST /api/spec/turn route is wired — empty body returns 422 or 400.
    let (status_turn, _, _) = http_post_raw(addr, "/api/spec/turn", b"{}").await;
    assert!(
        status_turn == 422 || status_turn == 400,
        "POST /api/spec/turn with empty body must return 400 or 422 (route exists), got {status_turn}"
    );

    // W5: POST /api/generate route is wired — wrong content returns 422.
    let (status_gen, _, _) = http_post_raw(addr, "/api/generate", b"{}").await;
    assert!(
        status_gen == 422 || status_gen == 400,
        "POST /api/generate with empty body must return 400 or 422 (route exists), got {status_gen}"
    );

    // W5: GET /api/artifact/:session_id/:name route is wired — nonexistent returns 404.
    let (status_art, _, _) = http_get(addr, "/api/artifact/nosession-x1/index.html").await;
    assert!(
        status_art == 404 || status_art == 400,
        "GET /api/artifact/nosession/index.html must return 404 or 400 (route exists), got {status_art}"
    );

    // W7: GET /api/welcome/status must return 200 + valid JSON.
    let (status_wst, _, body_wst) = http_get(addr, "/api/welcome/status").await;
    assert_eq!(
        status_wst, 200u16,
        "GET /api/welcome/status must return 200, got {status_wst}"
    );
    let _parsed: serde_json::Value =
        serde_json::from_str(&body_wst).expect("/api/welcome/status must return valid JSON");

    // W7: the four POST endpoints must accept JSON; empty body either parses
    // (init/llm-config/agent-deploy take {} happily) or returns 400/422 for
    // api-key (needs api_key field). Each one must exist (NOT 404).
    for path in [
        "/api/welcome/init",
        "/api/welcome/llm-config",
        "/api/welcome/agent-deploy",
    ] {
        let (s, _, _) = http_post_raw(addr, path, b"{}").await;
        assert!(
            s != 404,
            "POST {path} must be wired (any status except 404); got {s}"
        );
    }
    let (s_api, _, _) = http_post_raw(addr, "/api/welcome/api-key", b"{}").await;
    assert!(
        s_api == 422 || s_api == 400,
        "POST /api/welcome/api-key with empty body must return 400 or 422 (route exists), got {s_api}"
    );

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);
}

// ---------------------------------------------------------------------------
// Helpers for the W7 onboarded-workspace fixture.
// ---------------------------------------------------------------------------

static ENV_LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();
fn env_lock() -> &'static tokio::sync::Mutex<()> {
    ENV_LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

/// Build a tempdir that looks fully onboarded so GET / renders the dashboard.
/// Has genesis_payload.toml, agent_pubkeys.json (with an entry), turingos.toml
/// with the llm.* keys, spec.md (so spec_done) and a non-empty artifacts/.
fn seed_onboarded_workspace() -> tempfile::TempDir {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let p = dir.path();
    std::fs::write(p.join("genesis_payload.toml"), "[meta]\ntemplate = \"x\"\n").unwrap();
    std::fs::write(
        p.join("agent_pubkeys.json"),
        "{\n    \"agent_001\": {\n        \"pubkey\": \"00\",\n        \"role\": \"Solver\"\n    }\n}\n",
    )
    .unwrap();
    std::fs::write(
        p.join("turingos.toml"),
        "llm.meta.model = \"x\"\nllm.blackbox.model = \"y\"\n",
    )
    .unwrap();
    std::fs::write(p.join("spec.md"), "# spec\n").unwrap();
    std::fs::create_dir_all(p.join("artifacts")).unwrap();
    std::fs::write(p.join("artifacts/index.html"), "<!doctype html>").unwrap();
    dir
}

/// Keep the old name as an alias so old test runs don't break.
/// (Delegates to the full 20-route test.)
#[tokio::test]
async fn router_has_all_eight_routes() {
    // This is the W6/W7 superseding of the W1/W2/W5 gate.
    // We re-check the 7+1 read subset here too so the test name remains meaningful.
    let workspace = seed_onboarded_workspace();
    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", workspace.path());
    let addr = start_server().await;
    let _ = http_post_raw(
        addr,
        "/api/welcome/api-key",
        b"{\"api_key\":\"sk-routes-smoke-fixture-XXXXXXXX\"}",
    )
    .await;
    let http_routes = [
        "/",
        "/agents",
        "/tasks",
        "/audit",
        "/api/dashboard",
        "/api/agents",
        "/api/tasks",
    ];
    for path in &http_routes {
        let (status, _, _) = http_get(addr, path).await;
        assert_eq!(status, 200u16, "expected 200 for GET {path}, got {status}");
    }
    let (status_101, _, _) = http_get_upgrade(addr, "/ws").await;
    assert_eq!(status_101, 101u16, "GET /ws must return 101");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);
}

// ---------------------------------------------------------------------------
// W6 Gate: /build page chrome contract.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn build_page_contains_spec_grill_mount() {
    let addr = start_server().await;
    let (status, headers, body) = http_get(addr, "/build").await;
    assert_eq!(status, 200u16, "GET /build must return 200, got {status}");
    // Content-Type must be text/html
    let ct_line = headers
        .lines()
        .find(|l| l.to_lowercase().starts_with("content-type:"))
        .unwrap_or("");
    assert!(
        ct_line.to_lowercase().contains("text/html"),
        "/build content-type must be text/html, got {ct_line}"
    );
    // The page MUST contain the spec-grill Web Component mount point.
    assert!(
        body.contains("<tos-spec-grill></tos-spec-grill>"),
        "/build HTML must mount <tos-spec-grill>"
    );
    // The page MUST mark Build as the active nav item.
    assert!(
        body.contains("aria-current=\"page\""),
        "/build HTML must mark current page in nav"
    );
    // FC3-N31 materialized-view notice must appear on the page chrome.
    assert!(
        body.contains("FC3-N31"),
        "/build HTML must include FC3-N31 footer notice"
    );
}

/// Send a minimal HTTP/1.1 POST with `application/json` Content-Type.
/// Returns (status_code, headers, body_string).
async fn http_post_raw(addr: SocketAddr, path: &str, body: &[u8]) -> (u16, String, String) {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let mut stream = tokio::net::TcpStream::connect(addr)
        .await
        .expect("connect for POST");
    let header = format!(
        "POST {path} HTTP/1.1\r\n\
         Host: 127.0.0.1\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n",
        body.len()
    );
    stream
        .write_all(header.as_bytes())
        .await
        .expect("write header");
    stream.write_all(body).await.expect("write body");

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
    (status_code, head.to_string(), resp_body)
}

/// Send an HTTP/1.1 GET with `Upgrade: websocket` headers and return the
/// status code from the response. Uses raw TCP — no external client dep.
async fn http_get_upgrade(addr: SocketAddr, path: &str) -> (u16, String, String) {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let mut stream = tokio::net::TcpStream::connect(addr)
        .await
        .expect("connect to test server for WS upgrade");

    // Minimal valid WebSocket upgrade request as per RFC 6455.
    let key = "dGhlIHNhbXBsZSBub25jZQ=="; // base64("the sample nonce")
    let request = format!(
        "GET {path} HTTP/1.1\r\n\
         Host: 127.0.0.1\r\n\
         Upgrade: websocket\r\n\
         Connection: Upgrade\r\n\
         Sec-WebSocket-Key: {key}\r\n\
         Sec-WebSocket-Version: 13\r\n\
         \r\n"
    );
    stream
        .write_all(request.as_bytes())
        .await
        .expect("write WS upgrade request");

    // Read just enough to get the status line (first ~256 bytes).
    let mut buf = vec![0u8; 256];
    let n = stream
        .read(&mut buf)
        .await
        .expect("read WS upgrade response");
    let raw = String::from_utf8_lossy(&buf[..n]).into_owned();

    let (head, body) = if let Some(idx) = raw.find("\r\n\r\n") {
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

    (status_code, head.to_string(), body)
}

// ---------------------------------------------------------------------------
// Gate 2: dashboard HTML contains required strings.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn dashboard_html_contains_required_strings() {
    let workspace = seed_onboarded_workspace();
    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", workspace.path());
    let addr = start_server().await;
    let _ = http_post_raw(
        addr,
        "/api/welcome/api-key",
        b"{\"api_key\":\"sk-routes-smoke-fixture-XXXXXXXX\"}",
    )
    .await;
    let (status, _, body) = http_get(addr, "/").await;
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);
    assert_eq!(status, 200u16, "GET / on onboarded workspace must be 200");

    // §6a Page 1: must contain "TuringOS"
    assert!(
        body.contains("TuringOS"),
        "dashboard HTML must contain \"TuringOS\""
    );

    // §6a Page 1: must contain text matching /Phase \d/
    let re_pass = (0..=9).any(|d| body.contains(&format!("Phase {d}")));
    assert!(
        re_pass,
        "dashboard HTML must contain text matching /Phase \\d/"
    );

    // §6a Page 1: DOM must contain at least one [data-block-type] element
    assert!(
        body.contains("data-block-type="),
        "dashboard HTML must contain at least one data-block-type= attribute"
    );
}

// ---------------------------------------------------------------------------
// Gate 3: /agents HTML contains data-block-type.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn agents_html_contains_data_block_type() {
    let addr = start_server().await;
    let (status, _, body) = http_get(addr, "/agents").await;
    assert_eq!(status, 200u16);
    assert!(
        body.contains("data-block-type="),
        "/agents HTML must contain at least one data-block-type= attribute"
    );
}

// ---------------------------------------------------------------------------
// Gate 4: /tasks HTML contains data-block-type.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn tasks_html_contains_data_block_type() {
    let addr = start_server().await;
    let (status, _, body) = http_get(addr, "/tasks").await;
    assert_eq!(status, 200u16);
    assert!(
        body.contains("data-block-type="),
        "/tasks HTML must contain at least one data-block-type= attribute"
    );
}

// ---------------------------------------------------------------------------
// Gate 5: /api/dashboard returns valid JSON parseable as IRRoot.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn api_dashboard_returns_valid_json() {
    let addr = start_server().await;
    let (status, headers, body) = http_get(addr, "/api/dashboard").await;
    assert_eq!(status, 200u16);

    // Content-type must be application/json
    let ct_line = headers
        .lines()
        .find(|l| l.to_lowercase().starts_with("content-type:"))
        .unwrap_or("");
    assert!(
        ct_line.to_lowercase().contains("application/json"),
        "content-type must be application/json, got {ct_line}"
    );

    // Body must parse as IRRoot
    let ir: web::ir::IRRoot = serde_json::from_str(&body).expect("body must parse as IRRoot");

    // Must be non-empty (has at least one block)
    assert!(!ir.blocks.is_empty(), "IRRoot must have at least one block");
}

// ---------------------------------------------------------------------------
// Gate 6: HTML escaping — XSS-injected content is escaped in output.
// ---------------------------------------------------------------------------

#[test]
fn html_escapes_special_chars() {
    // Build an IRRoot with a <script>alert(1)</script> text field directly
    // and exercise the renderer — no HTTP needed for this check.
    use web::ir::{Block, IRRoot, TextBlock};
    use web::render::render_page;

    let ir = IRRoot {
        id: "test:escape".to_string(),
        title: "Test <escape> & \"quotes\" 'here'".to_string(),
        blocks: vec![Block::Text(TextBlock {
            id: "blk-xss".to_string(),
            content: "<script>alert(1)</script>".to_string(),
        })],
    };

    let html = render_page(&ir, "<script>alert(2)</script>", false);

    // The raw injection strings must NOT appear verbatim in output
    assert!(
        !html.contains("<script>alert(1)</script>"),
        "raw <script>alert(1)</script> must not appear in rendered HTML"
    );
    assert!(
        !html.contains("<script>alert(2)</script>"),
        "raw <script>alert(2)</script> in title must not appear in rendered HTML"
    );

    // The escaped form MUST be present
    assert!(
        html.contains("&lt;script&gt;"),
        "rendered HTML must contain &lt;script&gt; (escaped form)"
    );
    assert!(
        html.contains("&amp;"),
        "rendered HTML must contain &amp; from title escaping"
    );
}
