/// TRACE_MATRIX FC1-N5: real-time read-view push channel
///
/// WebSocket handler for TuringOS Phase 7 W2.
///
/// On upgrade, immediately pushes three `WsEnvelope` messages — one per
/// view (dashboard, agents, tasks) — serialized as JSON text frames. After
/// the initial push, the server enters a receive loop:
///
///   Ping   → reply Pong (explicit; axum 0.7 auto-handles heartbeat too)
///   Text   → log only (client-driven state mutation is W4; ignored here)
///   Binary → log only (same policy as Text)
///   Close  → break loop, clean shutdown
///   Err    → log and break (do not panic)
///
/// No write paths: client messages are never parsed into state mutations.
/// Reconnect is a client-side concern; the inline JS does NOT auto-reconnect
/// (W3 components may add their own strategy if needed).
#[cfg(feature = "web")]
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
#[cfg(feature = "web")]
use axum::response::IntoResponse;
#[cfg(feature = "web")]
use serde::Serialize;

#[cfg(feature = "web")]
use super::fixtures;
#[cfg(feature = "web")]
use super::ir::IRRoot;

// ---------------------------------------------------------------------------
// Envelope type
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: real-time read-view push channel
///
/// JSON envelope sent over the WebSocket on initial connect and on future
/// IR-update pushes.
///
/// Fields:
/// - `msg_type`: always `"ir_update"` for this message class
/// - `view`:     one of `"dashboard"`, `"agents"`, `"tasks"`
/// - `ir`:       the full IR for the named view
///
/// W3 components listen for `CustomEvent('turingos:ir_update', { detail: parsed })`
/// dispatched by the inline JS; `parsed` is the deserialized form of this struct.
#[derive(Debug, Serialize)]
pub(crate) struct WsEnvelope<'a> {
    /// Message type discriminant. Always `"ir_update"` for Phase 7 W2.
    pub(crate) msg_type: &'static str,
    /// View name: `"dashboard"`, `"agents"`, or `"tasks"`.
    pub(crate) view: &'static str,
    /// Borrow of the full IR for this view. Avoids cloning the IR on push.
    pub(crate) ir: &'a IRRoot,
}

// ---------------------------------------------------------------------------
// Public upgrade handler
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: real-time read-view push channel
///
/// Axum upgrade handler wired to `GET /ws`. Completes the HTTP 101 handshake
/// and hands the socket to `handle_socket`.
///
/// §6a Page 1 criterion: "one WebSocket OR SSE connection established
/// (if WS: HTTP 101 Upgrade)" — this handler satisfies that criterion.
#[cfg(feature = "web")]
pub(crate) async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

// ---------------------------------------------------------------------------
// Socket lifecycle
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: real-time read-view push channel
///
/// Handle one WebSocket connection from initial push through clean close.
///
/// Lifecycle:
/// 1. Build the three IR payloads from compile-time fixtures.
/// 2. Serialize each to a `WsEnvelope` JSON string and send as `Message::Text`.
/// 3. Enter recv loop; dispatch by message variant.
#[cfg(feature = "web")]
async fn handle_socket(mut socket: WebSocket) {
    // Step 1: build the three views from compile-time fixtures.
    let dash_ir = fixtures::dashboard();
    let agents_ir = fixtures::agent_view();
    let tasks_ir = fixtures::task_view();

    // Step 2: push one envelope per view.
    let initial_pushes: [(&'static str, &IRRoot); 3] = [
        ("dashboard", &dash_ir),
        ("agents", &agents_ir),
        ("tasks", &tasks_ir),
    ];

    for (view, ir) in &initial_pushes {
        let envelope = WsEnvelope {
            msg_type: "ir_update",
            view,
            ir,
        };
        match serde_json::to_string(&envelope) {
            Ok(json) => {
                if let Err(e) = socket.send(Message::Text(json.into())).await {
                    // Client disconnected before initial push completed — not an error.
                    log::warn!("ws: send initial push for view={view} failed: {e}");
                    return;
                }
            }
            Err(e) => {
                // Fixture serialization should never fail; log and abort.
                log::error!("ws: serialize WsEnvelope for view={view} failed: {e}");
                return;
            }
        }
    }

    // Step 3: recv loop — handle client messages.
    loop {
        match socket.recv().await {
            None => {
                // Stream ended (client closed without sending Close frame).
                log::debug!("ws: connection stream ended");
                break;
            }
            Some(Err(e)) => {
                log::warn!("ws: recv error: {e}");
                break;
            }
            Some(Ok(msg)) => match msg {
                Message::Ping(data) => {
                    // axum 0.7 auto-responds to Ping, but we also reply explicitly
                    // to make the intent clear in the implementation.
                    if let Err(e) = socket.send(Message::Pong(data)).await {
                        log::warn!("ws: pong send failed: {e}");
                        break;
                    }
                }
                Message::Pong(_) => {
                    // Unsolicited Pong from client — ignore.
                    log::debug!("ws: received unsolicited Pong, ignoring");
                }
                Message::Text(text) => {
                    // Client-driven Text is logged but NOT parsed into state.
                    // State mutation via browser is W4.
                    log::debug!(
                        "ws: received Text from client (ignored, len={})",
                        text.len()
                    );
                }
                Message::Binary(bytes) => {
                    // Client-driven Binary is logged but NOT parsed into state.
                    log::debug!(
                        "ws: received Binary from client (ignored, len={})",
                        bytes.len()
                    );
                }
                Message::Close(_) => {
                    // Client requested close — exit the loop cleanly.
                    log::debug!("ws: received Close frame, shutting down");
                    break;
                }
            },
        }
    }
}
