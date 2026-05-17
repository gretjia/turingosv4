//! TISR Phase 6.1 W1b.9 — `turingos audit tape` smoke / integration tests.
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
// Test 1: --help exits 0, stdout contains an audit/tape keyword
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn turingos_audit_tape_help_exits_zero_and_mentions_audit() {
    let output = Command::new(turingos_bin())
        .arg("audit")
        .arg("tape")
        .arg("--help")
        .output()
        .expect("run turingos audit tape --help");

    assert!(
        output.status.success(),
        "turingos audit tape --help should exit 0; status={:?}\nstdout={}\nstderr={}",
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
        combined.to_lowercase().contains("audit") || combined.to_lowercase().contains("tape"),
        "help output should mention 'audit' or 'tape'; got:\n{combined}",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2: no-args invocation — wrapper combined output is non-empty
// ─────────────────────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────────────────────
// Test 3: bogus flag exits non-zero (wrapper propagates error exit code)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn turingos_audit_tape_bogus_flag_exits_nonzero() {
    let output = Command::new(turingos_bin())
        .arg("audit")
        .arg("tape")
        .arg("--zzz-bogus")
        .output()
        .expect("run turingos audit tape --zzz-bogus");

    assert!(
        !output.status.success(),
        "turingos audit tape --zzz-bogus should exit non-zero; \
         status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}
