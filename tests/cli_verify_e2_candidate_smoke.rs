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
fn turingos_verify_e2_candidate_invokes_target_binary() {
    // Invoke with no args — real14_e2_candidate_verifier will print its own
    // usage or error. We assert the wrapper's combined output is non-empty
    // (i.e., shell-out plumbing works, not just dispatch).
    let output = Command::new(turingos_bin())
        .arg("verify")
        .arg("e2-candidate")
        .output()
        .expect("run turingos");
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    // real14_e2_candidate_verifier's no-arg behavior: prints usage or error.
    // Accept either stdout (its --help) or stderr (its arg-error).
    assert!(
        !combined.is_empty(),
        "wrapper produced no output — shell-out may have failed silently"
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
