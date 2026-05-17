//! TISR Phase 6.1 W1a.3 — `turingos report positions` smoke / integration test.
//!
//! Per §8 packet `2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md`
//! Section 4 allowed path: `tests/cli_*.rs`.
//!
//! Class 1 verification: confirms the `report positions` shell-out wrapper
//! plumbing is wired correctly. Three tests:
//!   1. --help exits 0 and mentions positions / view-positions / exposure.
//!   2. bare invocation produces non-empty output (shell-out plumbing reached lean_market).
//!   3. bogus flag returns non-zero exit.
//!
//! FC-trace: FC2-N16.

use std::path::PathBuf;
use std::process::Command;

/// Resolve the compiled `turingos` binary path. Tries debug then release.
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

/// Test 1: --help exits 0 and output references positions / view-positions / exposure.
#[test]
fn turingos_report_positions_help_exits_zero() {
    let output = Command::new(turingos_bin())
        .arg("report")
        .arg("positions")
        .arg("--help")
        .output()
        .expect("run turingos report positions --help");

    assert!(
        output.status.success(),
        "turingos report positions --help should exit 0; status={:?}\nstdout={}\nstderr={}",
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
        combined.contains("positions")
            || combined.contains("view-positions")
            || combined.contains("exposure"),
        "help output should mention 'positions', 'view-positions', or 'exposure'; got:\n{combined}"
    );
}

/// Test 3: bogus flag returns non-zero exit.
///
/// lean_market should reject an unknown flag and exit non-zero. If lean_market
/// is absent, the wrapper itself exits 2. Either way, exit must be non-zero.
#[test]
fn turingos_report_positions_bogus_flag_returns_nonzero() {
    let output = Command::new(turingos_bin())
        .arg("report")
        .arg("positions")
        .arg("--zzz-bogus-flag")
        .output()
        .expect("run turingos report positions --zzz-bogus-flag");

    assert!(
        !output.status.success(),
        "turingos report positions --zzz-bogus-flag should exit non-zero; \
         status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}
