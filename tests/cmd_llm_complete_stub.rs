//! W4 atom — cmd_llm complete action tests (subprocess-based).
//!
//! These tests run `target/debug/turingos llm complete ...` as a subprocess.
//! Real LLM calls are NOT made; instead we either (a) test args/help only,
//! or (b) point TURINGOS_SILICONFLOW_ENDPOINT at a local mock TCP server.
//!
//! TRACE_MATRIX FC2-N16 W4: CLI surface contract tests for `turingos llm complete`.

use std::process::Command;

fn bin_path() -> std::path::PathBuf {
    let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("target/debug/turingos");
    p
}

#[test]
fn help_lists_complete_action() {
    // Some commands print help to stderr; check both.
    let output = Command::new(bin_path())
        .arg("llm")
        .output()
        .expect("failed to spawn");
    let combined = String::from_utf8_lossy(&output.stdout).to_string()
        + &String::from_utf8_lossy(&output.stderr);
    assert!(
        combined.contains("complete"),
        "help should mention 'complete' action; got: {}",
        combined
    );
}

#[test]
fn complete_without_workspace_fails_args() {
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("complete")
        .output()
        .expect("failed to spawn");
    assert!(
        !output.status.success(),
        "complete without --workspace should fail"
    );
    // Verify it prints JSON with ok=false
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"ok\":false") || stdout.contains("\"ok\": false"),
        "stdout should be error JSON; got: {}",
        stdout
    );
}

#[test]
fn complete_with_missing_prompt_file_fails_io() {
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("complete")
        .arg("--workspace")
        .arg("/tmp")
        .arg("--prompt-file")
        .arg("/nonexistent/path/to/nothing.json")
        .output()
        .expect("failed to spawn");
    assert!(
        !output.status.success(),
        "complete with missing prompt file should fail"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"ok\":false") || stdout.contains("\"ok\": false"),
        "stdout should be error JSON; got: {}",
        stdout
    );
}

#[test]
fn complete_without_prompt_file_fails_args() {
    // --workspace is given but --prompt-file is omitted; should exit non-zero.
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("complete")
        .arg("--workspace")
        .arg("/tmp")
        .output()
        .expect("failed to spawn");
    assert!(
        !output.status.success(),
        "complete without --prompt-file should fail"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"ok\":false") || stdout.contains("\"ok\": false"),
        "stdout should be error JSON; got: {}",
        stdout
    );
}

#[test]
fn complete_error_json_contains_kind_field() {
    // Verify that error output is valid JSON with an "error.kind" field.
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("complete")
        .arg("--workspace")
        .arg("/tmp")
        .arg("--prompt-file")
        .arg("/nonexistent/path/to/nothing.json")
        .output()
        .expect("failed to spawn");
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Must be parseable JSON.
    let v: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("stdout must be valid JSON");
    assert_eq!(v["ok"], serde_json::json!(false));
    assert!(
        v["error"]["kind"].is_string(),
        "error.kind must be a string; got: {}",
        v
    );
    assert!(
        v["error"]["detail"].is_string(),
        "error.detail must be a string; got: {}",
        v
    );
}

#[test]
fn complete_capsule_dir_without_turn_id_fails() {
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("complete")
        .arg("--workspace")
        .arg("/tmp")
        .arg("--capsule-dir")
        .arg("/tmp/capsules")
        // --turn-id deliberately omitted
        .arg("--prompt-file")
        .arg("/nonexistent/nothing.json")
        .output()
        .expect("failed to spawn");
    // Should fail with args error (exit 5) before even trying to read the file.
    assert!(
        !output.status.success(),
        "--capsule-dir without --turn-id should fail"
    );
}

#[test]
fn complete_unknown_flag_fails_args() {
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("complete")
        .arg("--workspace")
        .arg("/tmp")
        .arg("--this-flag-does-not-exist")
        .output()
        .expect("failed to spawn");
    assert!(
        !output.status.success(),
        "unknown flag should cause failure"
    );
}

#[test]
fn complete_invalid_role_fails_args() {
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("complete")
        .arg("--workspace")
        .arg("/tmp")
        .arg("--role")
        .arg("invalid_role")
        .output()
        .expect("failed to spawn");
    assert!(
        !output.status.success(),
        "invalid --role should cause failure"
    );
}

#[test]
fn complete_invalid_lang_fails_args() {
    let output = Command::new(bin_path())
        .arg("llm")
        .arg("complete")
        .arg("--workspace")
        .arg("/tmp")
        .arg("--lang")
        .arg("fr")
        .output()
        .expect("failed to spawn");
    assert!(
        !output.status.success(),
        "unsupported --lang should cause failure"
    );
}

#[test]
#[ignore = "real-LLM-stub test; needs mock HTTP server infrastructure; deferred to W9"]
fn complete_stub_returns_valid_json() {
    // TODO: spin up TcpListener on 127.0.0.1:0, accept one connection,
    // respond with canned chat-completions JSON. Set
    // TURINGOS_SILICONFLOW_ENDPOINT to mock URL. Verify stdout JSON ok=true.
}
