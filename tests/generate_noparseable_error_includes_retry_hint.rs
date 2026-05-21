//! R4-1: Subprocess test — verifies that when the LLM returns an empty 200
//! response (no parseable files), the error message includes a retry hint,
//! and the hint appears BEFORE the [failed run] CID lines.
//!
//! Strategy: start a minimal TCP server that returns an OpenAI-compat 200
//! with empty `choices[0].message.content`. Run `turingos generate`. Capture
//! stderr. Assert all three conditions:
//!   1. exit code is non-zero
//!   2. stderr contains "Blackbox LLM emitted no parseable files"
//!   3. stderr contains "Try running" (the retry hint)
//!   4. the retry hint line appears BEFORE any "[failed run]" CID line
//!
//! TRACE_MATRIX FC2-N16: cmd_generate NoFilesParsed UX (R4-1).
//! Risk class: 1 (additive test; no production code change).

use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

fn turingos_bin() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let debug = PathBuf::from(format!("{manifest_dir}/target/debug/turingos"));
    let release = PathBuf::from(format!("{manifest_dir}/target/release/turingos"));
    if debug.exists() {
        return debug;
    }
    if release.exists() {
        return release;
    }
    panic!(
        "turingos binary not found at debug or release paths; \
         run `cargo build --bin turingos` first"
    );
}

/// Mock server that returns a valid OpenAI-compat 200 with empty content —
/// simulates a transient DeepSeek API hiccup (empty body, no parseable files).
struct MockServerEmptyContent {
    port: u16,
    shutdown: Arc<AtomicBool>,
    _handle: std::thread::JoinHandle<()>,
}

impl MockServerEmptyContent {
    fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind listener");
        let port = listener.local_addr().expect("local addr").port();
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown_clone = shutdown.clone();

        let handle = std::thread::spawn(move || {
            listener.set_nonblocking(true).ok();
            while !shutdown_clone.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let mut buf = [0u8; 4096];
                        let _ = stream
                            .set_read_timeout(Some(std::time::Duration::from_millis(200)));
                        let _ = stream.read(&mut buf);
                        // Valid OpenAI-compat 200 with empty content — triggers NoFilesParsed.
                        let body = r#"{"choices":[{"message":{"role":"assistant","content":""},"finish_reason":"stop"}],"usage":{"prompt_tokens":10,"completion_tokens":0,"total_tokens":10}}"#;
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            body.len(),
                            body
                        );
                        let _ = stream.write_all(response.as_bytes());
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                    Err(_) => break,
                }
            }
        });

        MockServerEmptyContent { port, shutdown, _handle: handle }
    }
}

impl Drop for MockServerEmptyContent {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
    }
}

fn workspace_with_valid_key(dir: &std::path::Path) -> PathBuf {
    let ws = dir.join("ws");
    fs::create_dir_all(&ws).expect("create ws dir");
    fs::create_dir_all(ws.join("cas")).expect("create cas dir");
    let config = "llm.blackbox.model = \"test-model\"\n\
                  llm.blackbox.api_key_env = \"MOCK_API_KEY\"\n";
    fs::write(ws.join("turingos.toml"), config).expect("write turingos.toml");
    fs::write(ws.join("spec.md"), "# Test spec\nBuild a hello world app.\n")
        .expect("write spec.md");
    ws
}

/// R4-1: NoFilesParsed error must include retry hint, and hint must precede CID lines.
#[test]
fn noparseable_error_includes_retry_hint() {
    let server = MockServerEmptyContent::start();
    let endpoint = format!("http://127.0.0.1:{}/v1/chat/completions", server.port);
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let ws = workspace_with_valid_key(tmp.path());

    let output = Command::new(turingos_bin())
        .arg("generate")
        .arg("--workspace")
        .arg(&ws)
        .env("MOCK_API_KEY", "test-key-r4-1")
        .env("TURINGOS_SILICONFLOW_ENDPOINT", &endpoint)
        .output()
        .expect("spawn turingos generate");

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    // 1. Must fail.
    assert!(
        !output.status.success(),
        "generate must fail with NoFilesParsed; exit={:?}\nstderr={stderr}",
        output.status
    );

    // 2. Must contain the base error message.
    assert!(
        stderr.contains("Blackbox LLM emitted no parseable files"),
        "stderr must contain base NoFilesParsed message; got:\nstderr={stderr}"
    );

    // 3. Must contain the retry hint.
    assert!(
        stderr.contains("Try running"),
        "stderr must contain retry hint 'Try running'; got:\nstderr={stderr}"
    );

    // 4. Retry hint must appear BEFORE any [failed run] CID lines.
    let stderr_lines: Vec<&str> = stderr.lines().collect();
    let hint_idx = stderr_lines.iter().position(|l| l.contains("Try running"));
    let cid_idx = stderr_lines.iter().position(|l| l.contains("[failed run]"));

    if let (Some(h), Some(c)) = (hint_idx, cid_idx) {
        assert!(
            h < c,
            "R4-1: retry hint (line {h}) must appear BEFORE [failed run] CID lines (line {c}).\n\
             stderr (full):\n{stderr}"
        );
    }
    // If no CID lines present (CAS write failed), ordering is trivially satisfied.
}
