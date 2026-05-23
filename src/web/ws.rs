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
use std::collections::{HashMap, VecDeque};
#[cfg(feature = "web")]
use std::sync::{Arc, Mutex};

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
pub enum WsBroadcastMsg {
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
    /// TRACE_MATRIX FC1-N5 + FC1-N10: Phase 7 W8 — auto-retry progress event.
    ///
    /// Emitted by POST /api/generate at the start of each retry attempt,
    /// including attempt 1. Allows `<tos-spec-result>` to show a live
    /// "正在生成... (尝试 N/M)" chip instead of an opaque loading spinner.
    GenerateAttemptStarted {
        session_id: String,
        attempt: u8,
        max_attempts: u8,
    },
    /// TRACE_MATRIX FC1-N5 + FC1-N10: Phase 7 W8 — auto-retry failure event.
    ///
    /// Emitted by POST /api/generate when an attempt fails the heuristic
    /// verification or the shellout exit code is non-zero. `reason` is a
    /// human-readable (Chinese-friendly) summary, safe to display.
    GenerateAttemptFailed {
        session_id: String,
        attempt: u8,
        max_attempts: u8,
        reason: String,
    },

    /// TRACE_MATRIX FC1-N5 + FC2-N16: Polymarket live projection hint.
    ///
    /// Emitted after `turingos generate` has admitted market txs. The payload
    /// is intentionally tiny: clients must refetch
    /// `/api/market/by-session/:id` and treat replay JSON as authoritative.
    AgentAttemptUpdate { session_id: String },

    // ── Phase 6.3.x driven-mode grill events (W7) ────────────────────────────
    /// TRACE_MATRIX FC2-N16 + FC1-N5: emitted by POST /api/spec/turn on each
    /// accepted Meta turn. Allows the frontend `<tos-spec-grill>` to update
    /// its question display without polling.
    SpecTurnAdvanced {
        session_id: String,
        turn_index: u32,
        question_text: String,
    },

    /// TRACE_MATRIX FC2-N16: emitted when a driven-mode grill session terminates
    /// (either cleanly via done=true + synthesis, or aborted by turn-limit /
    /// double-triage). Counterpart to the static-mode `SpecComplete`.
    SpecGrillComplete {
        session_id: String,
        spec_capsule_cid: String,
    },

    /// TRACE_MATRIX FC2-N16 + FC1-N5: R2 §A5 — triage non-relevant event.
    ///
    /// Emitted by POST /api/spec/turn when the Blackbox triage classifier marks
    /// the user's answer as off_topic, abusive, or gibberish. The frontend can
    /// show a gentle redirect message without waiting for the next turn.
    SpecTurnTriageReject {
        session_id: String,
        turn_index: u32,
        /// "off_topic" | "abusive" | "gibberish"
        triage_class: String,
        non_relevant_count: u32,
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
///
/// W7 (Phase 6.3.x) adds `sessions`: process-local in-memory driven-mode
/// session state. Per R2 §A14: in-flight session resumption is NOT supported
/// in v1 — if the server restarts, `sessions` is empty and clients receive
/// HTTP 404 on subsequent turns. The canonical state is in CAS via the
/// per-turn capsules; this HashMap is a performance shortcut only.
#[cfg(feature = "web")]
#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) broadcast_tx: broadcast::Sender<WsBroadcastMsg>,
    pub(crate) task_store: std::sync::Arc<TaskMemoryStore>,
    pub(crate) api_key: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    /// Phase 6.3.x driven-mode session state (process-local; NOT canonical).
    /// R2 §A14: no server-restart resume in v1.
    pub(crate) sessions: Arc<Mutex<HashMap<String, GrillSession>>>,
}

// ── Phase 6.3.x driven-mode session types (W7) ───────────────────────────────

/// Per-slot coverage granularity for the driven-mode grill.
/// Grows only (Empty → Partial → Satisfied). No reversal allowed.
#[cfg(feature = "web")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotState {
    Empty,
    Partial,
    Satisfied,
}

/// Process-local in-memory state for one driven-mode grill session.
///
/// All fields are plain data; no I/O. Written by `spec_turn_handler` under a
/// short-lived `Mutex` lock. The CAS capsules (written via shell-out) are the
/// canonical record; this struct is a performance/convenience cache.
#[cfg(feature = "web")]
#[derive(Debug, Clone)]
pub struct GrillSession {
    pub session_id: String,
    pub turn_count: u32,
    /// "zh" | "en"
    pub lang: String,
    /// slot_id → coverage state (slot vocab from grill_envelope::CANONICAL_SLOTS)
    pub coverage_state: HashMap<String, SlotState>,
    /// Rolling last-3 accepted (question, answer) pairs for context injection.
    pub last_3_turns: VecDeque<(String, String)>,
    /// CIDs of each written GrillTurnCapsuleBody, in turn order.
    pub turn_cids: Vec<String>,
    pub terminated: bool,
    /// CID of the previous accepted turn capsule (None on turn 1).
    pub parent_turn_cid: Option<String>,
    pub created_at_unix: u64,
    /// Number of consecutive non-relevant answers (abort threshold = 2).
    pub non_relevant_count: u32,
    /// Covered-slot list from the previous accepted Meta turn (for P4 monotonicity).
    pub last_prev_covered: Vec<String>,
    // Independent grill_attempt_count tallies (R2 §A6).
    pub meta_turns_accepted: u32,
    pub meta_turns_rejected: u32,
    pub triage_calls_relevant: u32,
    pub triage_calls_non_relevant: u32,
    /// FIX F6 (2026-05-18): the question_text emitted by the most recent
    /// accepted Meta turn. This is the question the user is now answering on
    /// the next POST /api/spec/turn. Used as `--question` for triage so the
    /// triage Blackbox model has Q/A context (without it, triage almost
    /// always classifies terse answers as non-relevant, producing spurious
    /// HTTP 200 empty-zero responses + premature `terminated=true`).
    ///
    /// Pre-F6, the handler derived `prev_question` from
    /// `last_3_turns.back().0`, but that slot stores the PREVIOUS prev_question
    /// (a one-turn-stale value), and on turn 1 it's empty. So triage was always
    /// run with `--question ""`. The Q1 the bootstrap turn emitted was never
    /// persisted at all.
    pub last_question_emitted: String,
    /// FIX A6 (2026-05-19): full ordered list of every triage-relevant user
    /// answer collected this session. `last_3_turns` is a rolling window
    /// (size 3) so it cannot back the post-`done` spec.md synthesis path,
    /// which needs the complete answer history. Pushed in
    /// `spec_turn_handler` step 9 right after the triage-relevant
    /// `last_3_turns.push_back(...)`. Mirrors the CLI driven path's
    /// `DrivenState::all_user_answers` (cmd_spec.rs:391/1289).
    ///
    /// NOTE (F10, 2026-05-19): kept as a chronological audit trail, but the
    /// spec.md synthesis path no longer relies on positional indexing into
    /// this Vec — see [`Self::slot_evidence`].
    pub all_user_answers: Vec<String>,
    /// FIX F10 (2026-05-19): slot-keyed evidence map. The KEY is the canonical
    /// slot id (`job`, `anchor`, `memory`, `first_run`, `robustness`, `scope`,
    /// `acceptance`, `mirror`); the VALUE is the most-recent triage-relevant
    /// user answer that the Meta-turn LLM credited toward that slot.
    ///
    /// Populated in `spec_turn_handler` step 11 by diffing the new
    /// `covered_slots` against `last_prev_covered`: every slot present in the
    /// new set but absent from the prev set is attributed to THIS turn's user
    /// answer. The deterministic LLM-less spec.md synthesiser then looks up
    /// each canonical slot in this map rather than indexing positionally into
    /// `all_user_answers`, which was the root cause of D-NEW-3a (Π4.3 P7 +
    /// Π4.4 S11 producing spec.md with content shifted into the wrong slot
    /// headers because the LLM asked slots in non-canonical adaptive order).
    pub slot_evidence: std::collections::BTreeMap<String, String>,
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
