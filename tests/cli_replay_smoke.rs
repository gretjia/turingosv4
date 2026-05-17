//! TISR Phase 6.1 W1c.12 — `turingos replay` smoke / integration tests.
//!
//! Per §8 packet `2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md`
//! Section 4 allowed path: `tests/cli_*.rs`.
//!
//! Class 1 verification: confirms the wrapper subcommand routes correctly,
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
// Test 1: --help exits 0, stdout contains a replay/chaintape keyword
// ─────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: replay --help short-circuits cleanly and shows description
#[test]
fn turingos_replay_help_shows_description() {
    let output = Command::new(turingos_bin())
        .arg("replay")
        .arg("--help")
        .output()
        .expect("run turingos replay --help");

    assert!(
        output.status.success(),
        "turingos replay --help should exit 0; status={:?}\nstdout={}\nstderr={}",
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
        combined.to_lowercase().contains("replay") || combined.to_lowercase().contains("chaintape"),
        "help output should mention 'replay' or 'chaintape'; got:\n{combined}",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2: no-args invocation — wrapper combined output is non-empty
// ─────────────────────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────────────────────
// Test 3: bogus flag exits non-zero (wrapper propagates error exit code)
// ─────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: bogus flag produces non-zero exit from replay
#[test]
fn turingos_replay_bogus_flag_exits_nonzero() {
    let output = Command::new(turingos_bin())
        .arg("replay")
        .arg("--zzz-bogus-flag-cannot-exist-999")
        .output()
        .expect("run turingos replay --zzz-bogus-flag-cannot-exist-999");

    assert!(
        !output.status.success(),
        "turingos replay --zzz-bogus-flag-cannot-exist-999 should exit non-zero; \
         status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}
