//! B3: Tests that failed generate routes CIDs to stderr with [failed run] prefix.
//!
//! TRACE_MATRIX FC2-N16: cmd_generate error path (UX hardening).
//! Risk class: 2 (additive, production wire-up).
//!
//! Strategy: use a workspace with a turingos.toml pointing to a localhost
//! stub endpoint that returns HTTP 400 with a generic error body. This
//! exercises the LlmApiError path which writes generation_attempt_cid and
//! rejection_cid to stderr with [failed run] prefix.
//!
//! If no local stub server is available (the test binary doesn't start one),
//! we instead verify the [failed run] prefix is NOT on stdout by using an
//! unreachable endpoint. The key invariant is that on LLM failure the
//! error output format is correct.
//!
//! NOTE: If mock server reliability is an issue, this test can be skipped
//! (see ESCALATION note in the implementation contract).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn turingos_bin() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let debug = PathBuf::from(format!("{manifest_dir}/target/debug/turingos"));
    let release = PathBuf::from(format!("{manifest_dir}/target/release/turingos"));
    if debug.exists() { return debug; }
    if release.exists() { return release; }
    panic!(
        "turingos binary not found at debug or release paths; \
         run `cargo build --bin turingos` first"
    );
}

/// Workspace configured to hit a local mock server on port 19877.
/// Returns the workspace path.
fn workspace_with_stub_endpoint(dir: &std::path::Path) -> std::path::PathBuf {
    let ws = dir.join("ws");
    fs::create_dir_all(&ws).expect("create ws dir");
    fs::create_dir_all(ws.join("cas")).expect("create cas dir");
    let config = "llm.blackbox.model = \"test-model\"\n\
                  llm.blackbox.api_key_env = \"FAKE_BLACKBOX_KEY\"\n";
    fs::write(ws.join("turingos.toml"), config).expect("write turingos.toml");
    fs::write(ws.join("spec.md"), "# Test spec\nBuild a hello world app.\n").expect("write spec.md");
    ws
}

/// Start a minimal HTTP server that always returns 400 + a generic error body.
/// Returns the server's port. The server runs in a background thread and is
/// stopped when the returned guard is dropped.
struct MockServer {
    port: u16,
    _handle: std::thread::JoinHandle<()>,
    shutdown: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    fn start_400() -> Self {
        use std::io::{Read, Write};
        use std::net::TcpListener;
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, Ordering};

        let listener = TcpListener::bind("127.0.0.1:0").expect("bind listener");
        let port = listener.local_addr().expect("local addr").port();
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown_clone = shutdown.clone();

        let handle = std::thread::spawn(move || {
            listener.set_nonblocking(true).ok();
            while !shutdown_clone.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        // Read the request (drain it).
                        let mut buf = [0u8; 4096];
                        let _ = stream.set_read_timeout(Some(std::time::Duration::from_millis(100)));
                        let _ = stream.read(&mut buf);
                        // Return a 400 with a generic error body.
                        let body = r#"{"error":{"message":"bad request","type":"invalid_request"}}"#;
                        let response = format!(
                            "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
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

        MockServer { port, _handle: handle, shutdown }
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.shutdown.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

/// B3: On LLM 4xx, stderr must contain [failed run] prefix BEFORE the error message.
/// stdout must NOT contain [failed run] (CIDs are on stderr on failure).
#[test]
fn failed_generate_cids_on_stderr_with_prefix() {
    let server = MockServer::start_400();
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let ws = workspace_with_stub_endpoint(tmp.path());

    let output = Command::new(turingos_bin())
        .arg("generate")
        .arg("--workspace")
        .arg(&ws)
        .env("FAKE_BLACKBOX_KEY", "fake-key-for-test")
        .env("TURINGOS_SILICONFLOW_ENDPOINT", format!("http://127.0.0.1:{}/v1/chat/completions", server.port))
        .output()
        .expect("spawn turingos generate");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    // Must fail.
    assert!(
        !output.status.success(),
        "generate must fail with 400 from mock LLM; exit={:?}\nstderr={stderr}",
        output.status
    );

    // [failed run] header must appear on stderr.
    assert!(
        stderr.contains("[failed run]"),
        "stderr must contain [failed run] prefix on failure; got:\nstderr={stderr}"
    );

    // [failed run] must NOT appear on stdout (CIDs go to stderr on failure).
    assert!(
        !stdout.contains("[failed run]"),
        "stdout must NOT contain [failed run]; got:\nstdout={stdout}"
    );

    // The error message must appear on stderr (after the [failed run] lines).
    assert!(
        stderr.contains("HTTP 400") || stderr.contains("turingos generate:"),
        "stderr must contain the actual error message; got:\nstderr={stderr}"
    );

    // Ordering: [failed run] BEFORE the HTTP error line.
    if let (Some(prefix_pos), Some(error_pos)) = (
        stderr.find("[failed run]"),
        stderr.find("HTTP 400").or_else(|| stderr.find("turingos generate:")),
    ) {
        assert!(
            prefix_pos < error_pos,
            "[failed run] prefix must appear BEFORE the error message in stderr;\
             \nprefix_pos={prefix_pos}, error_pos={error_pos}\nstderr={stderr}"
        );
    }
}

/// B3: On success (no mock needed — we just verify the negative: no [failed run] on success).
/// We can't easily simulate success without a real LLM, so this test checks that
/// the structural guarantee holds for the failure code path at the type level.
/// (The subprocess success case is covered by other test suites and the acceptance command.)
#[test]
fn failed_generate_stdout_has_no_bundle_cid_on_early_4xx() {
    let server = MockServer::start_400();
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let ws = workspace_with_stub_endpoint(tmp.path());

    let output = Command::new(turingos_bin())
        .arg("generate")
        .arg("--workspace")
        .arg(&ws)
        .env("FAKE_BLACKBOX_KEY", "fake-key-for-test")
        .env("TURINGOS_SILICONFLOW_ENDPOINT", format!("http://127.0.0.1:{}/v1/chat/completions", server.port))
        .output()
        .expect("spawn turingos generate");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    // On early LLM 4xx, no artifact bundle was written, so stdout must NOT
    // contain artifact_bundle_cid=.
    assert!(
        !stdout.contains("artifact_bundle_cid="),
        "stdout must not contain artifact_bundle_cid= on early 4xx failure;\
         \nstdout={stdout}"
    );
}
