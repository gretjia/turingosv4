//! TISR Phase 6.1 W1c.11 — `turingos preflight` smoke tests.
//!
//! TRACE_MATRIX FC2-N16: cli_preflight_smoke

use std::path::PathBuf;
use std::process::Command;

fn turingos_bin() -> PathBuf {
    let mut path = std::env::current_exe()
        .expect("current_exe")
        .parent()
        .expect("exe parent")
        .to_path_buf();
    // tests/ run as `target/debug/deps/cli_*-HASH` → parent is `target/debug/deps`
    path.pop(); // → target/debug
    path.push("turingos");
    if !path.exists() {
        // Try release
        path.pop();
        path.pop();
        path.push("release");
        path.push("turingos");
    }
    assert!(
        path.exists(),
        "turingos binary not found at {}",
        path.display()
    );
    path
}

/// TRACE_MATRIX FC2-N16: preflight --help short-circuits cleanly
#[test]
fn turingos_preflight_help_shows_description() {
    let output = Command::new(turingos_bin())
        .arg("preflight")
        .arg("--help")
        .output()
        .expect("run turingos");
    assert!(output.status.success(), "expected --help to succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("resume_preflight") || stdout.contains("preflight gates"),
        "help text missing expected description; got: {stdout}"
    );
}

/// TRACE_MATRIX FC2-N16: bogus flag produces non-zero exit from preflight
#[test]
fn turingos_preflight_bogus_flag_nonzero_exit() {
    let output = Command::new(turingos_bin())
        .arg("preflight")
        .arg("--this-flag-cannot-exist-zzz999")
        .output()
        .expect("run turingos");
    // Either the wrapper or resume_preflight should fail on the bogus flag.
    assert!(
        !output.status.success(),
        "expected non-zero exit on bogus flag"
    );
}
