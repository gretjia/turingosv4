//! TISR Phase 6.1 W1a.2 — `turingos report wallet` smoke tests.

use std::path::PathBuf;
use std::process::Command;

fn turingos_bin() -> PathBuf {
    let mut path = std::env::current_exe()
        .expect("current_exe")
        .parent()
        .expect("exe parent")
        .to_path_buf();
    path.pop();
    path.push("turingos");
    if !path.exists() {
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
fn turingos_report_wallet_help_shows_description() {
    let output = Command::new(turingos_bin())
        .arg("report")
        .arg("wallet")
        .arg("--help")
        .output()
        .expect("run turingos");
    assert!(output.status.success(), "expected --help success");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("wallet") || stdout.contains("lean_market"),
        "help text missing expected description; got: {stdout}"
    );
}

#[test]
fn turingos_report_wallet_invokes_target_binary() {
    // Invoke with no args — lean_market view-wallet will print usage / error.
    // We assert wrapper output is non-empty (i.e., reached lean_market).
    let output = Command::new(turingos_bin())
        .arg("report")
        .arg("wallet")
        .output()
        .expect("run turingos");
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !combined.is_empty(),
        "wrapper produced no output — shell-out may have failed"
    );
}

#[test]
fn turingos_report_wallet_intentionally_bad_args_nonzero() {
    let output = Command::new(turingos_bin())
        .arg("report")
        .arg("wallet")
        .arg("--zzz-this-flag-does-not-exist")
        .output()
        .expect("run turingos");
    assert!(
        !output.status.success(),
        "expected non-zero exit on bogus flag"
    );
}
