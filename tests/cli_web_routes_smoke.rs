//! TRACE_MATRIX FC1-N5: Phase 7 W1 smoke tests — verifies all seven read routes
//! are wired and return correct content types and body substrings.
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
// Gate 1: all seven routes return 200.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn router_has_all_seven_routes() {
    let addr = start_server().await;
    let routes = [
        "/",
        "/agents",
        "/tasks",
        "/audit",
        "/api/dashboard",
        "/api/agents",
        "/api/tasks",
    ];
    for path in &routes {
        let (status, _, _) = http_get(addr, path).await;
        assert_eq!(status, 200u16, "expected 200 for GET {path}, got {status}");
    }
}

// ---------------------------------------------------------------------------
// Gate 2: dashboard HTML contains required strings.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn dashboard_html_contains_required_strings() {
    let addr = start_server().await;
    let (status, _, body) = http_get(addr, "/").await;
    assert_eq!(status, 200u16);

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

    let html = render_page(&ir, "<script>alert(2)</script>");

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
