//! F9 regression test (2026-05-19): a transient `llm complete` failure
//! followed by a client retry of the same `(session_id, user_answer)`
//! must NOT duplicate the user turn in the next prompt JSON.
//!
//! ## Defect (D-NEW-2)
//!
//! Surfaced by Π4.2 P5 code-switch (`handover/evidence/
//! phase6_3_x_universality_1779111375/pi4/p5_codeswitch/verdict.json`):
//!
//! When `turingos llm complete` returns `ok=false` (D8 transient
//! SiliconFlow flake — happens 5-43% of the time), the on-disk
//! turn-prompt JSON captured the failed turn's messages array. On
//! retry, the user turn was APPENDED AGAIN, resulting in duplicated
//! user turns in `messages[]`. The next Meta call saw the corrupted
//! history and emitted an empty envelope → F6 silent-zero handling →
//! session terminated.
//!
//! ## Root cause
//!
//! Pre-F9: `spec_turn_handler` pushed `(prev_question, user_answer)`
//! into `last_3_turns` (and `all_user_answers`) BEFORE shelling out
//! to `llm complete`. If that call returned `ok=false`, the handler
//! returned HTTP 500 without rolling back the push. The client's
//! natural retry pushed the same pair AGAIN.
//!
//! ## Fix
//!
//! F9 defers the push to `last_3_turns` / `all_user_answers` until
//! AFTER `llm complete` succeeds (Step 11 of the handler). The
//! current turn's (q, a) is still included in the prompt-build
//! snapshot via a local clone so the Meta LLM sees the latest
//! exchange — it just isn't persisted to the session state until
//! the turn truly succeeds. This mirrors the CLI driven path
//! (cmd_spec.rs:1288-1289 — CLI loop pushes only after a
//! successful payload parse).
//!
//! ## Test scenario
//!
//! 1. Start a session via a null-user_answer POST (uses a stub binary
//!    whose `llm complete` always succeeds for the bootstrap call).
//! 2. POST a user_answer; the stub's `llm complete` returns
//!    `{ok:false, content:"transient flake"}` → handler returns 500.
//! 3. POST the SAME user_answer again; the stub's `llm complete` now
//!    returns a valid envelope → handler returns 200.
//! 4. Inspect the persisted `turn-2-prompt.json` on disk and assert
//!    that the user_answer text appears EXACTLY ONCE in the JSON.
//!    Pre-F9 it would appear twice; post-F9 it appears once.
//!
//! ## Stub behaviour
//!
//! The stub binary is a `/bin/sh` script that:
//! - On `llm triage` arg: always returns `{"ok":true,"class":"relevant"}`.
//! - On `llm complete` arg: increments a counter file and switches
//!   between failure and success based on the counter value:
//!     - counter == 1 → bootstrap (turn 1): succeed with a valid envelope.
//!     - counter == 2 → turn 2 first try: fail with ok=false.
//!     - counter >= 3 → turn 2 retry: succeed with a valid envelope.
//!
//! Run:
//! ```bash
//! cargo test --features web --test web_spec_retry_no_transcript_duplication
//! ```
#![cfg(feature = "web")]

#[path = "../src/web/mod.rs"]
mod web;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

// Process-global async lock for env mutation. Same pattern as
// `tests/cli_web_spec_smoke.rs` — `TURINGOS_BACKEND_OVERRIDE` and
// `TURINGOS_WEB_WORKSPACE` are process-global, so only one test
// touching them may run at a time.
static ENV_LOCK: std::sync::OnceLock<Mutex<()>> = std::sync::OnceLock::new();

fn env_lock() -> &'static Mutex<()> {
    ENV_LOCK.get_or_init(|| Mutex::new(()))
}

// ---------------------------------------------------------------------------
// HTTP + server helpers (copied from `tests/web_spec_turn_endpoint.rs`)
// ---------------------------------------------------------------------------

async fn start_server() -> std::net::SocketAddr {
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

async fn http_post_json(addr: std::net::SocketAddr, path: &str, body: &str) -> (u16, String) {
    let mut stream = TcpStream::connect(addr)
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
// Stub binary that responds differently per invocation
// ---------------------------------------------------------------------------

/// Write a `/bin/sh` stub at `dir/turingos`. The stub:
///   - Detects `triage` vs `complete` subcommand by scanning argv.
///   - For `triage`: always emits `{"ok":true,"class":"relevant"}`.
///   - For `complete`: increments `counter_file` and:
///       - if counter is in `fail_on_counts`: emits `{"ok":false,"content":"transient flake"}`
///       - otherwise: emits the canned success envelope `success_envelope`.
fn write_retry_stub(
    dir: &tempfile::TempDir,
    counter_file: &str,
    fail_on_counts: &[u32],
    success_envelope_json: &str,
) -> String {
    let script_path = dir.path().join("turingos");
    let fail_on_set: String = fail_on_counts
        .iter()
        .map(|n| format!("{n}"))
        .collect::<Vec<_>>()
        .join(" ");
    // The stub script uses `read` to load the counter, increments it,
    // and writes it back. The script then matches the counter against
    // `fail_on_counts` and emits the appropriate JSON.
    let script_content = format!(
        r#"#!/bin/sh
# Detect subcommand
subcmd=""
for arg in "$@"; do
    if [ "$arg" = "triage" ]; then
        subcmd="triage"
        break
    fi
    if [ "$arg" = "complete" ]; then
        subcmd="complete"
        break
    fi
done

if [ "$subcmd" = "triage" ]; then
    # Always classify as relevant.
    printf '%s\n' '{{"ok":true,"class":"relevant","class_confidence":0.95}}'
    exit 0
fi

if [ "$subcmd" = "complete" ]; then
    # Increment counter file.
    counter_file='{counter_file}'
    if [ -f "$counter_file" ]; then
        counter=$(cat "$counter_file")
    else
        counter=0
    fi
    counter=$((counter + 1))
    printf '%s' "$counter" > "$counter_file"

    # Check if this call should fail.
    fail_set='{fail_on_set}'
    for f in $fail_set; do
        if [ "$counter" = "$f" ]; then
            # Emit ok=false envelope (transient flake simulation).
            printf '%s\n' '{{"ok":false,"content":"transient flake (counter='"$counter"')","parsed_envelope":null}}'
            exit 0
        fi
    done

    # Success case: emit canned envelope.
    printf '%s\n' '{success_envelope}'
    exit 0
fi

# Unknown subcommand — exit non-zero so test fails loudly.
printf '%s\n' "stub: unknown subcommand; argv=$*" >&2
exit 2
"#,
        counter_file = shell_quote(counter_file),
        fail_on_set = fail_on_set,
        success_envelope = success_envelope_json,
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

/// Write a minimal `assets/prompts/grill_meta_v1.md` into the workspace.
/// The handler reads this file unconditionally (FIX F4) before shelling out
/// for `llm complete`, so the workspace must contain a readable file.
fn write_meta_prompt(workspace: &std::path::Path) {
    let assets_dir = workspace.join("assets/prompts");
    std::fs::create_dir_all(&assets_dir).expect("create assets/prompts");
    std::fs::write(
        assets_dir.join("grill_meta_v1.md"),
        "# Stub meta prompt (test).\nReturn an envelope JSON.\n",
    )
    .expect("write stub meta prompt");
}

// ---------------------------------------------------------------------------
// Test: F9 regression
// ---------------------------------------------------------------------------

/// F9 regression: a transient `llm complete` ok=false on turn 2 followed
/// by a client retry must NOT duplicate the user_answer in the persisted
/// turn-2-prompt.json. Pre-F9 the user_answer appeared twice; post-F9
/// it appears exactly once.
#[tokio::test]
async fn retry_after_llm_complete_flake_does_not_duplicate_user_turn() {
    let _guard = env_lock().lock().await;

    // ── Setup workspace ────────────────────────────────────────────────
    let workspace_dir = tempfile::tempdir().expect("create temp workspace");
    let workspace_path = workspace_dir.path().to_path_buf();
    write_meta_prompt(&workspace_path);

    // ── Setup stub binary ──────────────────────────────────────────────
    let stub_dir = tempfile::tempdir().expect("create temp stub dir");
    let counter_path = stub_dir.path().join("complete_counter");
    let counter_path_str = counter_path.to_string_lossy().into_owned();

    // The bootstrap call (turn 1) emits this envelope. It establishes
    // `last_question_emitted = "What is your favourite colour?"` so the
    // subsequent turn-2 call has a real `prev_question` for triage.
    //
    // Counter sequence:
    //   - counter == 1 → turn 1 bootstrap: success with Q1.
    //   - counter == 2 → turn 2 first try: ok=false (transient flake).
    //   - counter >= 3 → turn 2 retry: success with Q2.
    //
    // Both Q1 and Q2 envelopes are minimal but valid for the parser.
    let success_envelope = r#"{"ok":true,"content":"{\"turn\":99,\"question\":\"What is your favourite colour?\",\"covered_slots\":[\"job\"],\"open_slots\":[\"anchor\",\"memory\",\"first_run\",\"robustness\",\"scope\",\"acceptance\"],\"confidence\":0.3,\"done\":false,\"rationale\":\"stub\"}","parsed_envelope":{"turn":99,"question":"What is your favourite colour?","covered_slots":["job"],"open_slots":["anchor","memory","first_run","robustness","scope","acceptance"],"confidence":0.3,"done":false,"rationale":"stub"}}"#;

    let stub_path = write_retry_stub(&stub_dir, &counter_path_str, &[2], success_envelope);

    // ── Set env vars and start server ──────────────────────────────────
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &stub_path);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", workspace_path.to_string_lossy().as_ref());

    let addr = start_server().await;

    let session_id = "f9_retry_test_001";

    // ── Step 1: bootstrap (null user_answer) ───────────────────────────
    let body = format!(r#"{{"session_id":"{session_id}","user_answer":null,"lang":"en"}}"#);
    let (status, resp) = http_post_json(addr, "/api/spec/turn", &body).await;
    assert_eq!(
        status, 200,
        "bootstrap call must return 200; resp={resp}"
    );

    // ── Step 2: turn-2 first attempt (will fail with ok=false) ─────────
    let user_answer = "I want a tracker for my grocery list";
    let body = format!(
        r#"{{"session_id":"{session_id}","user_answer":"{user_answer}","lang":"en"}}"#
    );
    let (status, resp) = http_post_json(addr, "/api/spec/turn", &body).await;
    assert_eq!(
        status, 500,
        "turn-2 first attempt must return 500 (stub emits ok=false); resp={resp}"
    );
    assert!(
        resp.contains("shellout_failed") || resp.contains("ok=false"),
        "500 body must indicate llm complete failure; resp={resp}"
    );

    // ── Step 3: turn-2 retry (same user_answer) — must succeed ─────────
    let body = format!(
        r#"{{"session_id":"{session_id}","user_answer":"{user_answer}","lang":"en"}}"#
    );
    let (status, resp) = http_post_json(addr, "/api/spec/turn", &body).await;
    assert_eq!(
        status, 200,
        "turn-2 retry must return 200 (stub now succeeds); resp={resp}"
    );

    // ── Step 4: inspect persisted turn-2 prompt JSON ───────────────────
    let prompt_path = workspace_path
        .join("sessions")
        .join(session_id)
        .join("turn-2-prompt.json");
    assert!(
        prompt_path.exists(),
        "turn-2-prompt.json must exist at {}",
        prompt_path.display()
    );
    let prompt_json_str =
        std::fs::read_to_string(&prompt_path).expect("read turn-2-prompt.json");
    let prompt_json: serde_json::Value =
        serde_json::from_str(&prompt_json_str).expect("parse turn-2-prompt.json");
    let messages = prompt_json
        .get("messages")
        .and_then(|m| m.as_array())
        .expect("turn-2-prompt.json must have messages array");

    // Count how many messages have content == user_answer. The F9
    // regression test: this count must be exactly 1. Pre-F9 it would be
    // 2 (the original push + the retry push).
    let user_answer_occurrences = messages
        .iter()
        .filter(|m| {
            m.get("role")
                .and_then(|r| r.as_str())
                .map(|r| r == "user")
                .unwrap_or(false)
                && m.get("content")
                    .and_then(|c| c.as_str())
                    .map(|c| c == user_answer)
                    .unwrap_or(false)
        })
        .count();

    assert_eq!(
        user_answer_occurrences, 1,
        "user_answer must appear EXACTLY ONCE in turn-2-prompt.json messages[] \
         (pre-F9 it duplicated on retry). Got {user_answer_occurrences} occurrences. \
         Full messages array:\n{}",
        serde_json::to_string_pretty(messages).unwrap()
    );

    // Additionally, count assistant messages (the question echoes). Pre-F9
    // each duplicated user pair was preceded by a duplicated assistant
    // message (the same prev_question), so we should also see exactly ONE
    // assistant message echoing the bootstrap question "What is your
    // favourite colour?".
    let assistant_echo_count = messages
        .iter()
        .filter(|m| {
            m.get("role")
                .and_then(|r| r.as_str())
                .map(|r| r == "assistant")
                .unwrap_or(false)
                && m.get("content")
                    .and_then(|c| c.as_str())
                    .map(|c| c == "What is your favourite colour?")
                    .unwrap_or(false)
        })
        .count();
    assert_eq!(
        assistant_echo_count, 1,
        "assistant question echo must appear EXACTLY ONCE in turn-2-prompt.json \
         (pre-F9 it duplicated). Got {assistant_echo_count} occurrences."
    );

    // ── Cleanup env ────────────────────────────────────────────────────
    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
}
