//! W4.5 atom — cmd_llm triage action tests (subprocess-based).
//!
//! These tests run `target/debug/turingos llm triage ...` as a subprocess.
//! Real LLM calls are NOT made; instead we either (a) test args/help only,
//! or (b) point TURINGOS_SILICONFLOW_ENDPOINT at a local mock TCP server.
//!
//! TRACE_MATRIX FC2-N16 W4.5: CLI surface contract tests for `turingos llm triage`.

use std::process::Command;

fn bin_path() -> std::path::PathBuf {
    let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("target/debug/turingos");
    p
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
