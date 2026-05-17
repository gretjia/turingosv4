//! TISR Phase 6.1 W1b.8 — `turingos audit dashboard` smoke tests.

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

#[test]
fn turingos_audit_dashboard_help_shows_description() {
    let output = Command::new(turingos_bin())
        .arg("audit")
        .arg("dashboard")
        .arg("--help")
        .output()
        .expect("run turingos");
    assert!(output.status.success(), "expected --help to succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("audit dashboard") || stdout.contains("audit_dashboard"),
        "help text missing expected description; got: {stdout}"
    );
}

#[test]
fn turingos_audit_dashboard_no_args_output_nonempty() {
    // Invoke with no args — audit_dashboard will print its own usage + non-zero
    // exit or an error. We assert the wrapper's stdout/stderr is non-empty to
    // verify that the shell-out plumbing actually fired.
    let output = Command::new(turingos_bin())
        .arg("audit")
        .arg("dashboard")
        .output()
        .expect("run turingos");
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !combined.is_empty(),
        "wrapper produced no output — shell-out may have failed silently"
    );
}

#[test]
fn turingos_audit_dashboard_bogus_flag_nonzero_exit() {
    let output = Command::new(turingos_bin())
        .arg("audit")
        .arg("dashboard")
        .arg("--this-flag-cannot-exist-zzz999")
        .output()
        .expect("run turingos");
    assert!(
        !output.status.success(),
        "expected non-zero exit on bogus flag"
    );
}
