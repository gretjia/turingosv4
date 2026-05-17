//! TISR Phase 6.1 W1b.10 — `turingos audit tamper` smoke tests.
//!
//! TRACE_MATRIX FC2-N16: cli_audit_tamper_smoke

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

/// TRACE_MATRIX FC2-N16: audit tamper --help short-circuits cleanly
#[test]
fn turingos_audit_tamper_help_shows_description() {
    let output = Command::new(turingos_bin())
        .arg("audit")
        .arg("tamper")
        .arg("--help")
        .output()
        .expect("run turingos");
    assert!(output.status.success(), "expected --help to succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("audit_tape_tamper") || stdout.contains("tamper-resistance"),
        "help text missing expected description; got: {stdout}"
    );
}

/// TRACE_MATRIX FC2-N16: bogus flag produces non-zero exit from audit tamper
#[test]
fn turingos_audit_tamper_intentionally_bad_args_nonzero() {
    let output = Command::new(turingos_bin())
        .arg("audit")
        .arg("tamper")
        .arg("--this-flag-cannot-exist-zzz999")
        .output()
        .expect("run turingos");
    // Either the wrapper or audit_tape_tamper should fail on the bogus flag.
    assert!(
        !output.status.success(),
        "expected non-zero exit on bogus flag"
    );
}
