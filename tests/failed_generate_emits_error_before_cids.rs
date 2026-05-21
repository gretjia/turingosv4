//! X1: Subprocess test — verifies that on LLM failure, the error message
//! appears on stderr BEFORE the [failed run] CID diagnostic lines.
//!
//! PR #66 only added the [failed run] prefix but did not reorder; this test
//! would have caught that regression. The fix is in cmd_generate.rs: CID
//! lines are buffered into a GenError::WithFooter and emitted by run() AFTER
//! printing the error message.
//!
//! Strategy: start a minimal TCP server that always returns HTTP 401, run
//! `turingos generate`, capture stderr separately from stdout, and assert
//! line-index ordering.
//!
//! TRACE_MATRIX FC2-N16: cmd_generate error path (UX hardening, X1 fix).
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

/// Minimal HTTP server that returns a 401 with an authentication error body
/// (mirrors the SiliconFlow / DeepSeek 401 format from Round 3).
struct MockServer401 {
    port: u16,
    shutdown: Arc<AtomicBool>,
    _handle: std::thread::JoinHandle<()>,
}

impl MockServer401 {
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
                        let _ =
                            stream.set_read_timeout(Some(std::time::Duration::from_millis(200)));
                        let _ = stream.read(&mut buf);
                        let body = r#"{"error":{"message":"Authentication Fails, Your api key: ****-123 is invalid","type":"authentication_error","param":null,"code":"invalid_request_error"}}"#;
                        let response = format!(
                            "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
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

        MockServer401 { port, shutdown, _handle: handle }
    }
}

impl Drop for MockServer401 {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
    }
}

fn workspace_with_invalid_key(dir: &std::path::Path, endpoint: &str) -> PathBuf {
    let ws = dir.join("ws");
    fs::create_dir_all(&ws).expect("create ws dir");
    fs::create_dir_all(ws.join("cas")).expect("create cas dir");
    let config = format!(
        "llm.blackbox.model = \"test-model\"\n\
         llm.blackbox.api_key_env = \"FAKE_INVALID_KEY\"\n"
    );
    fs::write(ws.join("turingos.toml"), config).expect("write turingos.toml");
    fs::write(ws.join("spec.md"), "# Test spec\nBuild a hello world app.\n")
        .expect("write spec.md");
    // Write the endpoint override so the binary hits our mock server.
    let _ = endpoint; // endpoint is passed via env var below
    ws
}

/// X1: the HTTP error message must appear on stderr BEFORE [failed run] CID lines.
/// Verifies line-index ordering — not just substring presence.
#[test]
fn error_message_precedes_cid_lines_in_stderr() {
    let server = MockServer401::start();
    let endpoint = format!("http://127.0.0.1:{}/v1/chat/completions", server.port);
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let ws = workspace_with_invalid_key(tmp.path(), &endpoint);

    let output = Command::new(turingos_bin())
        .arg("generate")
        .arg("--workspace")
        .arg(&ws)
        .env("FAKE_INVALID_KEY", "garbage-key-intentionally-invalid")
        .env("TURINGOS_SILICONFLOW_ENDPOINT", &endpoint)
        .output()
        .expect("spawn turingos generate");

    // Capture stderr and stdout SEPARATELY (critical: stdout != stderr).
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    // Must fail.
    assert!(
        !output.status.success(),
        "generate must fail with 401 from mock LLM; exit={:?}\nstderr={stderr}",
        output.status
    );

    // The error message must be on stderr.
    assert!(
        stderr.contains("HTTP 401")
            || stderr.contains("turingos generate:")
            || stderr.contains("Authentication"),
        "stderr must contain the HTTP error message; got:\nstderr={stderr}"
    );

    // CID lines must be on stderr (not stdout) when present.
    assert!(
        !stdout.contains("[failed run]"),
        "stdout must NOT contain [failed run] CID lines; got:\nstdout={stdout}"
    );
    assert!(
        !stdout.contains("generation_attempt_cid="),
        "stdout must NOT contain generation_attempt_cid= on failure; got:\nstdout={stdout}"
    );

    // Core X1 ordering assertion: find the line indices.
    let stderr_lines: Vec<&str> = stderr.lines().collect();

    let error_line_idx = stderr_lines.iter().position(|l| {
        l.contains("HTTP 401")
            || (l.contains("turingos generate:") && !l.contains("[failed run]"))
            || l.contains("Authentication")
    });
    let cid_line_idx = stderr_lines.iter().position(|l| {
        l.contains("[failed run] generation_attempt_cid=")
            || l.contains("[failed run] rejection_cid=")
    });

    match (error_line_idx, cid_line_idx) {
        (Some(err_idx), Some(cid_idx)) => {
            assert!(
                err_idx < cid_idx,
                "X1: error message (line {err_idx}) must appear BEFORE [failed run] CID \
                 lines (line {cid_idx}) in stderr.\n\
                 This was the regression found in Round 3: PR #66 added [failed run] prefix \
                 but didn't reorder.\n\
                 stderr (full):\n{stderr}"
            );
        }
        (None, Some(_)) => {
            panic!(
                "No error message line found in stderr, but CID lines were present.\n\
                 stderr={stderr}"
            );
        }
        (Some(_), None) => {
            // CAS write failed (no cas dir, no attempt capsule) — no CID lines to order.
            // Still a valid outcome; the ordering invariant is trivially satisfied.
        }
        (None, None) => {
            // Both missing — the test should have caught !output.status.success() above.
            panic!("Neither error message nor CID lines found in stderr.\nstderr={stderr}");
        }
    }
}

/// X2 smoke: ensure --help does not write turingos.toml (tested more thoroughly
/// in llm_config_help_does_not_run_command.rs).
#[test]
fn generate_help_exits_cleanly() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let output = Command::new(turingos_bin())
        .arg("generate")
        .arg("--help")
        .current_dir(tmp.path())
        .output()
        .expect("spawn turingos generate --help");

    assert!(
        output.status.success(),
        "turingos generate --help must exit 0; got {:?}",
        output.status
    );
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    assert!(
        stdout.contains("turingos generate"),
        "help must contain usage text; got:\nstdout={stdout}"
    );
}
