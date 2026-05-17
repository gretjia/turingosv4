//! TISR Phase 6.1 — generic shell-out-wrapper plumbing tests.
//!
//! Per §8 packet `2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md`
//! Section 4 allowed path: `tests/cli_*.rs`.
//!
//! ## What this file replaces
//!
//! Phase 6.1 originally had ~15 per-wrapper `*_no_args_produces_output`
//! tests, each shelling out to the real backend (`lean_market`) and asserting
//! its output was non-empty. That design (a) coupled the test suite to a
//! specific Lean-era backend — a violation of TuringOS's general-agent-OS
//! posture — and (b) forced a multi-minute rebuild of `lean_market` to
//! satisfy a "shell-out plumbing works" assertion.
//!
//! This file consolidates that proof into ONE generic suite that
//! `PATH`-injects POSIX system binaries as backend stubs. No real backend
//! is built; no fake Cargo binary is compiled. The wrapper contract under
//! test is:
//!
//!   1. Subcommand token is prepended before the user's args
//!      (e.g. `turingos report wallet --x y` → backend sees `view-wallet --x y`).
//!   2. The wrapper preserves the child process's exit code
//!      (success and failure paths).
//!   3. The wrapper exits 2 with a clear stderr message when the backend
//!      cannot be invoked at all (binary missing).
//!
//! `TURINGOS_BIN_DIR` is the operator/test override knob; setting it
//! points the wrapper at a tempdir containing a symlinked stub. See
//! `src/bin/turingos/common.rs::run_external` for the resolution order.
//!
//! FC-trace: FC2-N16. Class 1 verification (no sequencer call, no CAS write).

#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::Command;

/// Resolve the compiled `turingos` binary path.
fn turingos_bin() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    for candidate in &[
        format!("{manifest_dir}/target/debug/turingos"),
        format!("{manifest_dir}/target/release/turingos"),
    ] {
        let p = PathBuf::from(candidate);
        if p.exists() {
            return p;
        }
    }
    panic!(
        "turingos binary not found at target/{{debug,release}}/turingos; \
         run `cargo build --bin turingos` first"
    );
}

/// Create a temp directory with a symlink named `stub_name` pointing at
/// `target_path` (a POSIX system binary such as `/bin/echo`). Returns the
/// directory handle (drops to clean up on test exit).
fn make_stub(stub_name: &str, target_path: &str) -> tempfile::TempDir {
    let dir = tempfile::TempDir::new().expect("create tempdir");
    let stub_path = dir.path().join(stub_name);
    std::os::unix::fs::symlink(target_path, &stub_path).expect("symlink stub");
    dir
}

/// Pick the first existing POSIX path from a candidate list, or skip the
/// test gracefully (returning None) if none exist. Different distros place
/// system binaries in /bin vs /usr/bin.
fn first_existing(candidates: &[&str]) -> Option<String> {
    candidates
        .iter()
        .find(|p| Path::new(p).exists())
        .map(|s| s.to_string())
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 1: subcommand token is prepended + user args forwarded verbatim
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn wrapper_prepends_subcommand_and_forwards_args_via_echo_stub() {
    // `/bin/echo` echoes its argv to stdout, terminated by `\n`. By
    // symlinking it as `lean_market` in a tempdir and pointing
    // `TURINGOS_BIN_DIR` at that dir, we can inspect exactly what argv
    // the wrapper passes to the backend — proving both that shell-out
    // plumbing works and that the subcommand token was prepended.
    let echo =
        first_existing(&["/bin/echo", "/usr/bin/echo"]).expect("no /bin/echo on this system");
    let stub_dir = make_stub("lean_market", &echo);

    let output = Command::new(turingos_bin())
        .args(["report", "wallet", "--chaintape", "/tmp/x"])
        .env("TURINGOS_BIN_DIR", stub_dir.path())
        .output()
        .expect("spawn turingos");

    assert!(
        output.status.success(),
        "echo stub should exit 0; status={:?}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    // `lean_market view-wallet --chaintape /tmp/x` is what the wrapper
    // should have invoked. echo echoes args after argv[0] separated by
    // spaces, so stdout begins with "view-wallet --chaintape /tmp/x".
    assert!(
        stdout.contains("view-wallet"),
        "subcommand token not prepended; echo stdout: {stdout}",
    );
    assert!(
        stdout.contains("--chaintape") && stdout.contains("/tmp/x"),
        "user args not forwarded; echo stdout: {stdout}",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2: child exit code 0 propagates to wrapper exit 0
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn wrapper_preserves_child_exit_code_zero_via_true_stub() {
    let true_bin =
        first_existing(&["/bin/true", "/usr/bin/true"]).expect("no /bin/true on this system");
    let stub_dir = make_stub("lean_market", &true_bin);

    let output = Command::new(turingos_bin())
        .args(["task", "tick"])
        .env("TURINGOS_BIN_DIR", stub_dir.path())
        .output()
        .expect("spawn turingos");

    assert_eq!(
        output.status.code(),
        Some(0),
        "wrapper should propagate child exit 0; status={:?}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3: child exit code non-zero propagates to wrapper exit non-zero
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn wrapper_preserves_child_exit_code_nonzero_via_false_stub() {
    let false_bin =
        first_existing(&["/bin/false", "/usr/bin/false"]).expect("no /bin/false on this system");
    let stub_dir = make_stub("lean_market", &false_bin);

    let output = Command::new(turingos_bin())
        .args(["report", "wallet"])
        .env("TURINGOS_BIN_DIR", stub_dir.path())
        .output()
        .expect("spawn turingos");

    assert_eq!(
        output.status.code(),
        Some(1),
        "wrapper should propagate /bin/false's exit 1; status={:?}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4: missing backend → wrapper exits 2 with a clear stderr message
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn wrapper_exit_2_with_clear_stderr_when_backend_missing() {
    // Empty stub dir + empty PATH → wrapper cannot resolve the backend
    // anywhere, must exit 2 and emit "failed to invoke" on stderr (no
    // panic, no silent fallback).
    let empty = tempfile::TempDir::new().expect("tempdir");

    let output = Command::new(turingos_bin())
        .args(["report", "wallet"])
        .env("TURINGOS_BIN_DIR", empty.path())
        .env("PATH", empty.path())
        .output()
        .expect("spawn turingos");

    assert_eq!(
        output.status.code(),
        Some(2),
        "wrapper should exit 2 when backend missing; status={:?}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("failed to invoke"),
        "wrapper should report failure clearly; stderr was:\n{stderr}",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5: spot-check that each of the 7 task-runner wrappers prepends the
// correct subcommand token. One test, parameterised over the wrapper name +
// expected token. Catches off-by-one renames (e.g. `view-wallet` vs `wallet`).
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn each_task_runner_wrapper_prepends_correct_subcommand_token() {
    let echo =
        first_existing(&["/bin/echo", "/usr/bin/echo"]).expect("no /bin/echo on this system");
    let stub_dir = make_stub("lean_market", &echo);

    // (user-facing turingos subcommand, expected prepended token to the backend)
    let cases: &[(&[&str], &str)] = &[
        (&["report", "wallet"], "view-wallet"),
        (&["report", "positions"], "view-positions"),
        (&["report", "bankruptcy"], "view-bankruptcy"),
        (&["replay"], "view-replay"),
        (&["task", "open"], "run-task"),
        (&["task", "view"], "view-task"),
        (&["task", "tick"], "tick"),
    ];

    for (user_args, expected_token) in cases {
        let output = Command::new(turingos_bin())
            .args(*user_args)
            .env("TURINGOS_BIN_DIR", stub_dir.path())
            .output()
            .unwrap_or_else(|e| panic!("spawn turingos {user_args:?}: {e}"));

        assert!(
            output.status.success(),
            "wrapper {user_args:?} should succeed under echo stub; \
             status={:?}\nstderr={}",
            output.status,
            String::from_utf8_lossy(&output.stderr),
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains(expected_token),
            "wrapper {user_args:?} did not prepend `{expected_token}`; \
             echo stdout: {stdout}",
        );
    }
}
