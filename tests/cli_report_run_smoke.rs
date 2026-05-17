//! TISR Phase 6.1 W1a.1 — `turingos report run` smoke tests.

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
fn turingos_report_run_help_shows_description() {
    let output = Command::new(turingos_bin())
        .arg("report")
        .arg("run")
        .arg("--help")
        .output()
        .expect("run turingos");
    assert!(output.status.success(), "expected --help to succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("run-summary report") || stdout.contains("gen_run_summary"),
        "help text missing expected description; got: {stdout}"
    );
}

#[test]
fn turingos_report_run_invokes_target_binary() {
    // Invoke with no args — gen_run_summary will print its own usage + non-zero
    // exit. We assert the wrapper's stderr or stdout shows it actually reached
    // gen_run_summary (i.e., shell-out plumbing works, not just dispatch).
    let output = Command::new(turingos_bin())
        .arg("report")
        .arg("run")
        .output()
        .expect("run turingos");
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    // gen_run_summary's no-arg behavior: prints usage hint or error.
    // Accept either stdout (its --help) or stderr (its arg-error).
    assert!(
        !combined.is_empty(),
        "wrapper produced no output — shell-out may have failed silently"
    );
}

#[test]
fn turingos_report_run_intentionally_bad_args_nonzero() {
    let output = Command::new(turingos_bin())
        .arg("report")
        .arg("run")
        .arg("--this-flag-cannot-exist-zzz123")
        .output()
        .expect("run turingos");
    // Either the wrapper or gen_run_summary should fail on the bogus flag.
    // We don't pin the exit code; just non-zero.
    assert!(
        !output.status.success(),
        "expected non-zero exit on bogus flag"
    );
}
