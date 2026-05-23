//! W4.5 atom — cmd_llm triage action tests (subprocess-based).
//!
//! These tests run `target/debug/turingos llm triage ...` as a subprocess.
//! Real LLM calls are NOT made; instead we either (a) test args/help only,
//! or (b) point TURINGOS_SILICONFLOW_ENDPOINT at a local mock TCP server.
//!
//! TRACE_MATRIX FC2-N16 W4.5: CLI surface contract tests for `turingos llm triage`.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::Command;
use std::sync::{mpsc, Mutex, OnceLock};
use std::time::Duration;

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> &'static Mutex<()> {
    ENV_LOCK.get_or_init(|| Mutex::new(()))
}

fn bin_path() -> std::path::PathBuf {
    let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("target/debug/turingos");
    p
}

fn write_triage_workspace(dir: &tempfile::TempDir) -> std::path::PathBuf {
    let workspace = dir.path().join("ws");
    let prompt_dir = workspace.join("assets/prompts");
    std::fs::create_dir_all(&prompt_dir).expect("prompt dir");
    std::fs::write(
        prompt_dir.join("grill_triage_blackbox_v1.md"),
        "## System prompt (verbatim)\n```text\nReturn only JSON.\n```\n",
    )
    .expect("triage prompt");
    std::fs::write(
        workspace.join("turingos.toml"),
        r#"
llm.blackbox.model = "mock-blackbox"
llm.blackbox.api_key_env = "MOCK_TRIAGE_API_KEY"
llm.blackbox.thinking = "on"
"#,
    )
    .expect("turingos.toml");
    workspace
}

fn start_mock_chat_server() -> (String, mpsc::Receiver<serde_json::Value>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock server");
    let addr = listener.local_addr().expect("mock addr");
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept mock request");
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .expect("set timeout");
        let mut buf = Vec::new();
        let mut tmp = [0u8; 1024];
        loop {
            let n = stream.read(&mut tmp).expect("read request");
            assert!(n > 0, "mock client closed before request body");
            buf.extend_from_slice(&tmp[..n]);
            let Some(header_end) = buf.windows(4).position(|w| w == b"\r\n\r\n") else {
                continue;
            };
            let header = String::from_utf8_lossy(&buf[..header_end]).to_string();
            let content_len = header
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse::<usize>().ok())?
                })
                .unwrap_or(0);
            let body_start = header_end + 4;
            if buf.len() < body_start + content_len {
                continue;
            }
            let body = &buf[body_start..body_start + content_len];
            let payload: serde_json::Value =
                serde_json::from_slice(body).expect("request body must be JSON");
            tx.send(payload).expect("send captured payload");
            let response_body = br#"{"choices":[{"message":{"role":"assistant","content":"{\"class\":\"relevant\",\"confidence\":0.91}"}}],"usage":{"prompt_tokens":1,"completion_tokens":1,"total_tokens":2}}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n",
                response_body.len()
            );
            stream.write_all(response.as_bytes()).expect("write head");
            stream.write_all(response_body).expect("write body");
            return;
        }
    });
    (format!("http://{addr}/v1/chat/completions"), rx)
}

#[test]
fn help_lists_triage_action() {
    let output = Command::new(bin_path())
        .arg("llm")
        .output()
        .expect("failed to spawn");
    let combined = String::from_utf8_lossy(&output.stdout).to_string()
        + &String::from_utf8_lossy(&output.stderr);
    assert!(
        combined.contains("triage"),
        "help should mention 'triage' action; got: {}",
        combined
    );
}

#[test]
fn triage_without_workspace_fails_args() {
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("triage")
        .output()
        .expect("failed to spawn");
    assert!(
        !output.status.success(),
        "triage without --workspace should fail"
    );
}

#[test]
fn triage_without_user_answer_fails_args() {
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("triage")
        .arg("--workspace")
        .arg("/tmp")
        .output()
        .expect("failed to spawn");
    assert!(
        !output.status.success(),
        "triage without --user-answer should fail"
    );
}

#[test]
fn triage_with_capsule_dir_without_turn_id_fails() {
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("triage")
        .arg("--workspace")
        .arg("/tmp")
        .arg("--user-answer")
        .arg("test answer")
        .arg("--capsule-dir")
        .arg("/tmp/caps")
        .output()
        .expect("failed to spawn");
    assert!(
        !output.status.success(),
        "triage with --capsule-dir but no --turn-id should fail"
    );
}

#[test]
fn triage_stub_uses_large_budget_and_forces_thinking_off() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = write_triage_workspace(&dir);
    let (endpoint, rx) = start_mock_chat_server();

    let _guard = env_lock().lock().expect("env lock");
    std::env::set_var("MOCK_TRIAGE_API_KEY", "sk-mock-triage-key-0000000000");
    std::env::set_var("TURINGOS_SILICONFLOW_ENDPOINT", &endpoint);

    let output = Command::new(bin_path())
        .arg("llm")
        .arg("triage")
        .arg("--workspace")
        .arg(&workspace)
        .arg("--user-answer")
        .arg("我要做一个井字棋游戏")
        .arg("--question")
        .arg("你想做什么？")
        .output()
        .expect("spawn triage");

    std::env::remove_var("MOCK_TRIAGE_API_KEY");
    std::env::remove_var("TURINGOS_SILICONFLOW_ENDPOINT");
    drop(_guard);

    assert!(
        output.status.success(),
        "triage should accept mock JSON; stdout={}\nstderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let payload = rx
        .recv_timeout(Duration::from_secs(5))
        .expect("captured payload");
    assert_eq!(
        payload["max_tokens"].as_u64(),
        Some(512),
        "triage must reserve enough tokens for JSON even if provider would emit reasoning; payload={payload}"
    );
    assert!(
        payload.get("thinking").is_none(),
        "triage must force thinking off even when llm.blackbox.thinking=on; payload={payload}"
    );
}

#[test]
#[ignore = "real-Blackbox-stub test; needs mock HTTP server; deferred to W9"]
fn triage_stub_returns_valid_classification() {
    // TODO: spin up TcpListener mock returning {"class":"relevant","confidence":0.9}
    // Set TURINGOS_SILICONFLOW_ENDPOINT to mock URL. Verify stdout JSON.
}

#[test]
#[ignore = "real-Blackbox-stub test; needs mock HTTP server"]
fn triage_stub_handles_abusive_class() {
    // TODO: mock returns {"class":"abusive","confidence":0.95}
}

#[test]
#[ignore = "real-Blackbox-stub test"]
fn triage_stub_handles_malformed_output_exits_3() {
    // TODO: mock returns invalid JSON; verify exit code 3 and error.kind = "parse_failed"
}
