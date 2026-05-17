//! TISR Phase 6.1 W1b.7 — `turingos verify e2-candidate` smoke tests.

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
fn turingos_verify_e2_candidate_help_shows_description() {
    let output = Command::new(turingos_bin())
        .arg("verify")
        .arg("e2-candidate")
        .arg("--help")
        .output()
        .expect("run turingos");
    assert!(output.status.success(), "expected --help to succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("E2")
            || stdout.contains("e2")
            || stdout.contains("candidate")
            || stdout.contains("real14"),
        "help text missing expected description; got: {stdout}"
    );
}

#[test]
fn turingos_verify_e2_candidate_bogus_flag_nonzero() {
    let output = Command::new(turingos_bin())
        .arg("verify")
        .arg("e2-candidate")
        .arg("--zzz-bogus")
        .output()
        .expect("run turingos");
    // Either the wrapper or real14_e2_candidate_verifier should fail on the
    // bogus flag. We don't pin the exit code; just non-zero.
    assert!(
        !output.status.success(),
        "expected non-zero exit on bogus flag"
    );
}
