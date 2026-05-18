/// TRACE_MATRIX FC1-N5: real-time read-view push channel
///
/// WebSocket handler for TuringOS Phase 7 W2/W4.
///
/// W4 migration: `WsEnvelope` is now a tagged-union enum (serde tag = "msg_type")
/// with two variants:
///   - `IrUpdate { view, ir }`:    initial push on connect + IR change broadcasts
///   - `TaskCreated { task_id, agent_id, problem_id, bounty }`: write-path event
///
/// Design: `WsEnvelope<'a>` covers both variants for serialization. The broadcast
/// channel uses the owned `WsBroadcastMsg` type (only `TaskCreated` is broadcast;
/// `IrUpdate` is pushed inline on initial connect). The socket send path serializes
/// both `WsEnvelope` (for initial pushes) and `WsBroadcastMsg` (for broadcasts)
/// as the same JSON shape via `serde`.
///
/// `AppState` carries a `tokio::sync::broadcast::Sender<WsBroadcastMsg>` threaded
/// through axum's State extractor. The `/ws` handler subscribes on connect; the
/// POST /api/task/open handler sends on success.
///
/// W2 behaviour preserved: on upgrade, server still pushes three `IrUpdate`
/// envelopes immediately (one per view).
///
/// Receive loop policy (unchanged from W2):
///   Ping   → reply Pong
///   Text   → log only (client-driven state mutation via WS is not supported)
///   Binary → log only
///   Close  → break loop, clean shutdown
///   Err    → log and break
#[cfg(feature = "web")]
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
#[cfg(feature = "web")]
use axum::extract::State;
#[cfg(feature = "web")]
use axum::response::IntoResponse;
#[cfg(feature = "web")]
use serde::Serialize;
#[cfg(feature = "web")]
use tokio::sync::broadcast;

#[cfg(feature = "web")]
use super::fixtures;
#[cfg(feature = "web")]
use super::ir::IRRoot;
#[cfg(feature = "web")]
use super::store::TaskMemoryStore;

// ---------------------------------------------------------------------------
// Broadcast message type (owned; used for channel + TaskCreated events)
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N10 + FC1-N5 + FC2-N16: message type used in the broadcast channel.
///
/// W4: `TaskCreated` — emitted by POST /api/task/open on success.
/// W5 additions:
///   `SpecComplete`      — emitted by POST /api/spec/submit on success.
///   `GenerateStarted`   — reserved for future streaming (not yet emitted).
///   `GenerateComplete`  — emitted by POST /api/generate on success.
///
/// Using a separate owned type avoids lifetime complications with
/// `broadcast::Sender<T>` which requires `T: Clone + Send + 'static`.
///
/// Serialises with `"msg_type": "<snake_case_variant>"` to match the
/// `WsEnvelope` union shape.
#[cfg(feature = "web")]
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "msg_type", rename_all = "snake_case")]
pub(crate) enum WsBroadcastMsg {
    /// Emitted by POST /api/task/open on success.
    TaskCreated {
        task_id: String,
        agent_id: String,
        problem_id: String,
        bounty: u64,
    },
    /// TRACE_MATRIX FC2-N16: emitted by POST /api/spec/submit on success.
    ///
    /// `session_id`: the session identifier assigned to this spec run.
    /// `capsule_cid`: hex CID of the CAS EvidenceCapsule written by
    ///   `turingos spec`, or None if the CID could not be parsed from stdout.
    SpecComplete {
        session_id: String,
        capsule_cid: Option<String>,
    },
    /// TRACE_MATRIX FC2-N16: reserved for future streaming progress events.
    ///
    /// Not yet emitted by any handler. Reserved so frontends can subscribe
    /// before generate starts and show a spinner.
    GenerateStarted { session_id: String },
    /// TRACE_MATRIX FC2-N16: emitted by POST /api/generate on success.
    ///
    /// `artifacts`: list of relative paths under `<session-dir>/artifacts/`
    ///   that were written by the Blackbox LLM generation step.
    GenerateComplete {
        session_id: String,
        artifacts: Vec<String>,
    },
}

// ---------------------------------------------------------------------------
// Shared state (broadcast channel)
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5 / FC1-N10: shared axum State threaded from startup.
///
/// `broadcast_tx` is cloned into each handler that needs to publish events.
/// Each `/ws` connection calls `.subscribe()` on connect to receive all future
/// broadcasts. Channel capacity is set to 64 at startup (turingos_web.rs).
///
/// `task_store` is an `Arc`-wrapped `TaskMemoryStore` shared across all handler
/// tasks.  The POST /api/task/open handler pushes entries; the GET /api/tasks
/// and GET /tasks handlers read a snapshot and merge it with the fixture.
/// W7 adds `api_key`: an `Arc<Mutex<Option<String>>>` storing the
/// SiliconFlow API key in process memory ONLY. The value is set by
/// `POST /api/welcome/api-key`, injected into `turingos spec` / `turingos
/// generate` child processes via `Command::env`, and dropped when the
/// process exits. It is NEVER written to disk, logged, or echoed in any
/// HTTP response body. `std::sync::Mutex` is correct here (not
/// `tokio::sync::Mutex`) because the critical section is microseconds —
/// no `.await` is held while the lock is acquired.
#[cfg(feature = "web")]
#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) broadcast_tx: broadcast::Sender<WsBroadcastMsg>,
    pub(crate) task_store: std::sync::Arc<TaskMemoryStore>,
    pub(crate) api_key: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

// ---------------------------------------------------------------------------
// Envelope type — W4 tagged-union (for initial push serialization)
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: WebSocket initial-push envelope.
///
/// Used only for the three initial IR pushes on connect. Borrows `IRRoot`
/// to avoid cloning on each send.
///
/// Serialises as:
/// ```json
/// { "msg_type": "ir_update", "view": "tasks", "ir": { ... } }
/// ```
///
/// The `WsEnvelope` type is the serde counterpart to `WsBroadcastMsg` — both
/// use the same `"msg_type"` tag so the frontend can discriminate on a single field.
#[cfg(feature = "web")]
#[derive(Debug, Serialize)]
#[serde(tag = "msg_type", rename_all = "snake_case")]
pub(crate) enum WsEnvelope<'a> {
    /// Initial or incremental IR push for one view.
    IrUpdate {
        /// View name: `"dashboard"`, `"agents"`, or `"tasks"`.
        view: &'a str,
        /// Full IR for this view.
        ir: &'a IRRoot,
    },
}

// ---------------------------------------------------------------------------
// Public upgrade handler
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: real-time read-view push channel
///
/// Axum upgrade handler wired to `GET /ws`. Completes the HTTP 101 handshake
/// and hands the socket + state to `handle_socket`.
///
/// §6a Page 1 criterion: "one WebSocket OR SSE connection established
/// (if WS: HTTP 101 Upgrade)" — this handler satisfies that criterion.
#[cfg(feature = "web")]
pub(crate) async fn ws_handler(
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

// ---------------------------------------------------------------------------
// Socket lifecycle
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5 / FC1-N10: handle one WebSocket connection.
///
/// Lifecycle:
/// 1. Subscribe to the broadcast channel.
/// 2. Build and push three initial IR envelopes (one per view).
/// 3. Enter recv/broadcast loop: forward broadcast messages + handle client frames.
#[cfg(feature = "web")]
async fn handle_socket(mut socket: WebSocket, state: AppState) {
    // Subscribe to future broadcasts (must subscribe before initial push so we
    // do not miss events that arrive during the initial push window).
    let mut rx = state.broadcast_tx.subscribe();

    // Step 1: build the three views from compile-time fixtures.
    let dash_ir = fixtures::dashboard();
    let agents_ir = fixtures::agent_view();
    let tasks_ir = fixtures::task_view();

    // Step 2: push one envelope per view.
    let initial_pushes: &[(&'static str, &IRRoot)] = &[
        ("dashboard", &dash_ir),
        ("agents", &agents_ir),
        ("tasks", &tasks_ir),
    ];

    for (view, ir) in initial_pushes {
        let envelope = WsEnvelope::IrUpdate { view, ir };
        match serde_json::to_string(&envelope) {
            Ok(json) => {
                if let Err(e) = socket.send(Message::Text(json.into())).await {
                    log::warn!("ws: send initial push for view={view} failed: {e}");
                    return;
                }
            }
            Err(e) => {
                log::error!("ws: serialize WsEnvelope::IrUpdate for view={view} failed: {e}");
                return;
            }
        }
    }

    // Step 3: combined recv + broadcast loop.
    loop {
        tokio::select! {
            // Incoming client frame.
            client_msg = socket.recv() => {
                match client_msg {
                    None => {
                        log::debug!("ws: connection stream ended");
                        break;
                    }
                    Some(Err(e)) => {
                        log::warn!("ws: recv error: {e}");
                        break;
                    }
                    Some(Ok(msg)) => match msg {
                        Message::Ping(data) => {
                            if let Err(e) = socket.send(Message::Pong(data)).await {
                                log::warn!("ws: pong send failed: {e}");
                                break;
                            }
                        }
                        Message::Pong(_) => {
                            log::debug!("ws: received unsolicited Pong, ignoring");
                        }
                        Message::Text(text) => {
                            log::debug!(
                                "ws: received Text from client (ignored, len={})",
                                text.len()
                            );
                        }
                        Message::Binary(bytes) => {
                            log::debug!(
                                "ws: received Binary from client (ignored, len={})",
                                bytes.len()
                            );
                        }
                        Message::Close(_) => {
                            log::debug!("ws: received Close frame, shutting down");
                            break;
                        }
                    },
                }
            }

            // Incoming broadcast from another handler (e.g. task_open_handler).
            broadcast_result = rx.recv() => {
                match broadcast_result {
                    Ok(msg) => {
                        match serde_json::to_string(&msg) {
                            Ok(json) => {
                                if let Err(e) = socket.send(Message::Text(json.into())).await {
                                    log::warn!("ws: send broadcast failed: {e}");
                                    break;
                                }
                            }
                            Err(e) => {
                                log::error!("ws: serialize broadcast msg failed: {e}");
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        // Client is slow; we skipped n messages. Log and continue.
                        log::warn!("ws: broadcast receiver lagged {n} messages");
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        // Sender dropped — server shutting down.
                        log::debug!("ws: broadcast channel closed");
                        break;
                    }
                }
            }
        }
    }
}
