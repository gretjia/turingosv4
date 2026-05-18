//! TRACE_MATRIX FC1-N5: Phase 7 W2 smoke tests — verifies the WebSocket
//! endpoint performs the HTTP 101 upgrade, pushes three initial IR envelopes,
//! and handles client-initiated close/text without crashing.
//!
//! Gated on `#[cfg(feature = "web")]` so non-web builds never see this.
//! Run with: `cargo test --test cli_web_ws_smoke --features web`
//!
//! Implementation note: tests spin up a real TCP listener on a random
//! OS-assigned port (bind to 127.0.0.1:0). WebSocket communication is done
//! over raw TCP with hand-crafted frames to avoid external stream-trait
//! dependencies. This mirrors the pattern in `cli_web_routes_smoke.rs`.
//!
//! WebSocket frame layout (RFC 6455 §5.2, client→server with masking):
//!   Byte 0:  FIN=1 | opcode
//!   Byte 1:  MASK=1 | payload_len (≤125 for our payloads)
//!   Bytes 2–5: masking key (4 bytes)
//!   Bytes 6+:  masked payload
//!
//! WebSocket frame layout (server→client, no masking):
//!   Byte 0:  FIN=1 | opcode
//!   Byte 1:  MASK=0 | payload_len (≤125 or extended)
//!   Bytes 2+: payload
#![cfg(feature = "web")]

// Mirror the same path-based module declaration used in `turingos_web.rs`
// so the test exercises the exact same module tree.
#[path = "../src/web/mod.rs"]
mod web;

use std::net::SocketAddr;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

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
// WsConn: wraps TcpStream + leftover bytes from the HTTP upgrade read.
// The upgrade reader may consume bytes past \r\n\r\n that belong to the
// first WebSocket frame; we buffer them here to avoid losing data.
// ---------------------------------------------------------------------------

struct WsConn {
    stream: TcpStream,
    /// Buffered bytes read during the upgrade but past the HTTP headers.
    leftover: Vec<u8>,
    /// Read cursor within `leftover`.
    leftover_pos: usize,
}

impl WsConn {
    /// Read exactly `n` bytes, consuming from leftover first then from stream.
    /// Panics on unexpected EOF. Use `read_byte()` first to detect clean EOF.
    async fn read_exact_buf(&mut self, n: usize) -> Vec<u8> {
        let mut out = Vec::with_capacity(n);
        // Consume from leftover first.
        let avail = self.leftover.len() - self.leftover_pos;
        if avail > 0 {
            let take = avail.min(n);
            out.extend_from_slice(&self.leftover[self.leftover_pos..self.leftover_pos + take]);
            self.leftover_pos += take;
        }
        // Read remaining from stream.
        if out.len() < n {
            let mut rest = vec![0u8; n - out.len()];
            self.stream
                .read_exact(&mut rest)
                .await
                .expect("ws read_exact from stream");
            out.extend_from_slice(&rest);
        }
        out
    }

    /// Read exactly one byte. Returns `None` if the connection closed (EOF).
    async fn read_byte(&mut self) -> Option<u8> {
        // Consume from leftover if available.
        if self.leftover_pos < self.leftover.len() {
            let b = self.leftover[self.leftover_pos];
            self.leftover_pos += 1;
            return Some(b);
        }
        // Read one byte from stream.
        let mut b = [0u8; 1];
        match self.stream.read(&mut b).await {
            Ok(0) => None, // EOF — clean close
            Ok(_) => Some(b[0]),
            Err(_) => None, // connection reset etc. — treat as close
        }
    }
}

// ---------------------------------------------------------------------------
// Helper: perform the HTTP 101 WebSocket upgrade handshake over raw TCP.
// Returns a WsConn that handles leftover bytes from the upgrade read.
// ---------------------------------------------------------------------------

const WS_KEY: &str = "dGhlIHNhbXBsZSBub25jZQ==";

async fn ws_upgrade(addr: SocketAddr) -> WsConn {
    let mut stream = TcpStream::connect(addr)
        .await
        .expect("connect to test server");

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
        .expect("write upgrade request");

    // Read byte-by-byte until we see \r\n\r\n to avoid consuming WS frame bytes.
    // This is slow but correct and only happens once per test connection.
    let mut header_buf: Vec<u8> = Vec::new();
    loop {
        let mut b = [0u8; 1];
        stream
            .read_exact(&mut b)
            .await
            .expect("read upgrade response byte");
        header_buf.push(b[0]);
        // Check for end of HTTP headers.
        if header_buf.ends_with(b"\r\n\r\n") {
            break;
        }
        assert!(
            header_buf.len() < 4096,
            "upgrade response headers too large"
        );
    }

    let response_str = String::from_utf8_lossy(&header_buf);
    let status_line = response_str.lines().next().unwrap_or("");
    let status_code: u16 = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    assert_eq!(
        status_code, 101,
        "WebSocket upgrade must return HTTP 101, got {status_code}"
    );

    WsConn {
        stream,
        leftover: Vec::new(),
        leftover_pos: 0,
    }
}

// ---------------------------------------------------------------------------
// WebSocket frame reader/writer helpers (RFC 6455, client-side perspective)
// ---------------------------------------------------------------------------

/// Opcodes relevant to our tests.
const OP_TEXT: u8 = 0x01;
#[allow(dead_code)]
const OP_BINARY: u8 = 0x02;
const OP_CLOSE: u8 = 0x08;
#[allow(dead_code)]
const OP_PING: u8 = 0x09;
#[allow(dead_code)]
const OP_PONG: u8 = 0x0a;

/// Read one complete WebSocket frame from the server (no masking expected).
/// Returns `Some((opcode, payload_bytes))` or `None` if the connection closed
/// cleanly (EOF) — which counts as an implicit Close in the close handshake.
async fn ws_recv_frame(conn: &mut WsConn) -> Option<(u8, Vec<u8>)> {
    // Read first 2 bytes: header. EOF here = server closed cleanly.
    // Read byte-by-byte using the buffered reader so we can detect EOF.
    let b0 = match conn.read_byte().await {
        Some(b) => b,
        None => return None,
    };
    let b1 = match conn.read_byte().await {
        Some(b) => b,
        None => return None,
    };

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

/// Send a masked WebSocket text frame from client to server.
async fn ws_send_text(conn: &mut WsConn, text: &str) {
    let payload = text.as_bytes();
    // Masking key — all zeros is a valid mask (XOR with 0 = unchanged payload).
    let mask: [u8; 4] = [0, 0, 0, 0];
    let mut frame = Vec::new();
    frame.push(0x80 | OP_TEXT); // FIN + text opcode
    let plen = payload.len();
    assert!(
        plen <= 125,
        "test payload must be ≤125 bytes for simplicity"
    );
    frame.push(0x80 | plen as u8); // MASK=1 + length
    frame.extend_from_slice(&mask);
    frame.extend_from_slice(payload); // mask is all-zeros so payload unchanged
    conn.stream
        .write_all(&frame)
        .await
        .expect("write ws text frame");
}

/// Send a masked WebSocket close frame from client to server.
async fn ws_send_close(conn: &mut WsConn) {
    // Close frame with status code 1000 (normal closure), masked with zero mask.
    let frame: [u8; 8] = [
        0x80 | OP_CLOSE, // FIN + close opcode
        0x80 | 2,        // MASK=1, payload len = 2
        0x00,
        0x00,
        0x00,
        0x00, // zero mask key
        0x03,
        0xe8, // 1000 in big-endian (XOR zero mask = unchanged)
    ];
    conn.stream
        .write_all(&frame)
        .await
        .expect("write ws close frame");
}

// ---------------------------------------------------------------------------
// Inline envelope type for deserialization in tests (mirrors WsEnvelope).
// ---------------------------------------------------------------------------

#[derive(Debug, serde::Deserialize)]
struct TestEnvelope {
    msg_type: String,
    view: String,
    ir: web::ir::IRRoot,
}

// ---------------------------------------------------------------------------
// Test 1: ws_handshake_succeeds
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: §6a Page 1 — "one WebSocket connection established
/// (HTTP 101 Upgrade)" criterion.
#[tokio::test]
async fn ws_handshake_succeeds() {
    let addr = start_server().await;
    // ws_upgrade asserts status 101 internally.
    let result = timeout(Duration::from_secs(2), ws_upgrade(addr)).await;
    result.expect("WebSocket handshake must complete within 2 seconds");
}

// ---------------------------------------------------------------------------
// Test 2: ws_pushes_three_initial_ir_messages
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: initial IR push — one envelope per view.
#[tokio::test]
async fn ws_pushes_three_initial_ir_messages() {
    let addr = start_server().await;
    let mut conn = ws_upgrade(addr).await;

    let mut envelopes: Vec<TestEnvelope> = Vec::new();

    let result = timeout(Duration::from_secs(1), async {
        while envelopes.len() < 3 {
            let frame = ws_recv_frame(&mut conn)
                .await
                .expect("connection must not close before 3 initial pushes");
            let (opcode, payload) = frame;
            if opcode == OP_TEXT {
                let text = String::from_utf8(payload).expect("text frame must be valid UTF-8");
                let env: TestEnvelope =
                    serde_json::from_str(&text).expect("initial push must parse as TestEnvelope");
                envelopes.push(env);
            }
            // Skip Ping/Pong/other control frames.
        }
    })
    .await;
    result.expect("must receive 3 initial IR messages within 1 second");

    assert_eq!(
        envelopes.len(),
        3,
        "must receive exactly 3 initial envelopes"
    );

    // Verify msg_type field on all.
    for env in &envelopes {
        assert_eq!(env.msg_type, "ir_update", "msg_type must be 'ir_update'");
    }

    // Verify one envelope per view (order is implementation-defined).
    let views: std::collections::HashSet<&str> =
        envelopes.iter().map(|e| e.view.as_str()).collect();
    assert!(views.contains("dashboard"), "must have dashboard envelope");
    assert!(views.contains("agents"), "must have agents envelope");
    assert!(views.contains("tasks"), "must have tasks envelope");
}

// ---------------------------------------------------------------------------
// Test 3: ws_envelope_carries_valid_ir
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: IR validity — each pushed envelope carries a
/// parseable IRRoot with at least one block.
#[tokio::test]
async fn ws_envelope_carries_valid_ir() {
    let addr = start_server().await;
    let mut conn = ws_upgrade(addr).await;

    let mut envelopes: Vec<TestEnvelope> = Vec::new();
    let result = timeout(Duration::from_secs(1), async {
        while envelopes.len() < 3 {
            let frame = ws_recv_frame(&mut conn)
                .await
                .expect("connection must not close before 3 initial pushes");
            let (opcode, payload) = frame;
            if opcode == OP_TEXT {
                let text = String::from_utf8(payload).expect("text frame must be valid UTF-8");
                let env: TestEnvelope =
                    serde_json::from_str(&text).expect("envelope must parse as TestEnvelope");
                envelopes.push(env);
            }
        }
    })
    .await;
    result.expect("must receive 3 envelopes within 1 second");

    for env in &envelopes {
        assert!(
            !env.ir.blocks.is_empty(),
            "view='{}' IR must have at least one block",
            env.view
        );
    }
}

// ---------------------------------------------------------------------------
// Test 4: ws_handles_client_close_cleanly
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: clean close — server must not timeout or panic when
/// the client sends a Close frame.
#[tokio::test]
async fn ws_handles_client_close_cleanly() {
    let addr = start_server().await;
    let mut conn = ws_upgrade(addr).await;

    // Wrap the entire test in a timeout to bound execution.
    let result = timeout(Duration::from_secs(3), async {
        // Drain the 3 initial push messages. Do NOT use a nested timeout here —
        // read_exact is not cancellation-safe; partial reads would corrupt the
        // stream state. The server always pushes exactly 3 messages, so this
        // loop will always complete within milliseconds.
        let mut count = 0usize;
        while count < 3 {
            let frame = ws_recv_frame(&mut conn)
                .await
                .expect("connection must not close during initial drain");
            if frame.0 == OP_TEXT {
                count += 1;
            }
        }

        // Send a Close frame.
        ws_send_close(&mut conn).await;

        // The server should echo a Close frame or close the TCP connection
        // (either satisfies the clean-close requirement).
        loop {
            match ws_recv_frame(&mut conn).await {
                None => break, // EOF = server closed cleanly
                Some((OP_CLOSE, _)) => break,
                Some(_) => {} // skip other frames
            }
        }
    })
    .await;
    result.expect("server must close cleanly within 3 seconds");
}

// ---------------------------------------------------------------------------
// Test 5: ws_handles_client_text_without_crashing
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: no-write guarantee — client-sent Text is ignored,
/// server must not panic, connection closes cleanly afterward.
#[tokio::test]
async fn ws_handles_client_text_without_crashing() {
    let addr = start_server().await;
    let mut conn = ws_upgrade(addr).await;

    // Wrap the entire test in a timeout to bound execution.
    let result = timeout(Duration::from_secs(3), async {
        // Drain the 3 initial push messages. Do NOT use a nested timeout —
        // read_exact is not cancellation-safe.
        let mut count = 0usize;
        while count < 3 {
            let frame = ws_recv_frame(&mut conn)
                .await
                .expect("connection must not close during initial drain");
            if frame.0 == OP_TEXT {
                count += 1;
            }
        }

        // Send junk Text — server must log and ignore, not panic.
        ws_send_text(&mut conn, r#"{"junk":"payload"}"#).await;

        // Give server a tick to process the text message before sending Close.
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Now close — server must still be alive and respond to Close.
        ws_send_close(&mut conn).await;

        // Wait for server's Close response or clean EOF.
        loop {
            match ws_recv_frame(&mut conn).await {
                None => break, // EOF = server closed cleanly
                Some((OP_CLOSE, _)) => break,
                Some(_) => {} // skip other frames
            }
        }
    })
    .await;
    result.expect("server must close cleanly after ignoring client Text");
}
