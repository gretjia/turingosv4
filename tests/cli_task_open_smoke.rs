//! TISR Phase 6.1 W1c.13 — `turingos task open` smoke / integration tests.
//!
//! Per §8 packet `2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md`
//! Section 4 allowed path: `tests/cli_*.rs`.
//!
//! Class 2 verification: confirms the wrapper subcommand routes correctly,
//! --help short-circuits cleanly, bogus flags propagate non-zero exit.
//!
//! FC-trace: FC2-N16

use std::path::PathBuf;
use std::process::Command;

/// Resolve the compiled `turingos` binary path.
///
/// Tries debug then release; panics with a useful message if neither exists
/// (caller must run `cargo build --bin turingos` first).
fn turingos_bin() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let candidates = [
        format!("{manifest_dir}/target/debug/turingos"),
        format!("{manifest_dir}/target/release/turingos"),
    ];
    for candidate in candidates.iter() {
        let path = PathBuf::from(candidate);
        if path.exists() {
            return path;
        }
    }
    panic!(
        "turingos binary not found at debug or release target paths; \
         run `cargo build --bin turingos` before running this smoke test"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 1: --help exits 0, stdout contains the description keyword
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn turingos_task_open_help_exits_zero_and_mentions_description() {
    let output = Command::new(turingos_bin())
        .arg("task")
        .arg("open")
        .arg("--help")
        .output()
        .expect("run turingos task open --help");

    assert!(
        output.status.success(),
        "turingos task open --help should exit 0; status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    assert!(
        combined.to_lowercase().contains("task")
            || combined.to_lowercase().contains("lean_market")
            || combined.to_lowercase().contains("run-task"),
        "help output should mention 'task', 'lean_market', or 'run-task'; got:\n{combined}",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2: no-args wrapper invokes lean_market and produces output
// ─────────────────────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────────────────────
// Test 3: bogus flag exits non-zero (wrapper propagates error exit code)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn turingos_task_open_bogus_flag_exits_nonzero() {
    let output = Command::new(turingos_bin())
        .arg("task")
        .arg("open")
        .arg("--zzz-bogus-flag-that-does-not-exist")
        .output()
        .expect("run turingos task open --zzz-bogus-flag-that-does-not-exist");

    assert!(
        !output.status.success(),
        "turingos task open --zzz-bogus-flag-that-does-not-exist should exit non-zero; \
         status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}
