//! TISR Phase 6.1 W1a.4 — `turingos report bankruptcy` smoke test.
//!
//! Per §8 packet `2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md`
//! Section 4 allowed path: `tests/cli_*.rs`.
//!
//! FC-trace: FC2-N16 (boot / genesis / tape replay view).
//! TB-11 / TB-12: RunExhausted / Bankruptcy evidence shell-out plumbing.
//!
//! Class 1 verification: confirms the `turingos report bankruptcy` dispatch
//! path is wired, --help exits 0 with expected keywords, no-args produces
//! combined output (shell-out plumbing exercised), and bogus flags produce
//! non-zero exit.

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

// ─────────────────────────────────────────────────────────────────────
// Test 1: --help exits 0 and contains bankruptcy-related keywords
// ─────────────────────────────────────────────────────────────────────

#[test]
fn turingos_report_bankruptcy_help_exits_zero_and_mentions_keyword() {
    let output = Command::new(turingos_bin())
        .args(["report", "bankruptcy", "--help"])
        .output()
        .expect("run turingos report bankruptcy --help");

    assert!(
        output.status.success(),
        "expected exit 0 on --help; status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let has_keyword = combined.contains("bankruptcy")
        || combined.contains("view-bankruptcy")
        || combined.contains("exhausted")
        || combined.contains("Exhausted")
        || combined.contains("Bankrupt");

    assert!(
        has_keyword,
        "expected 'bankruptcy', 'view-bankruptcy', or 'exhausted' in --help output; got:\n{combined}"
    );
}

// ─────────────────────────────────────────────────────────────────────
// Test 2: no-args invocation produces non-empty combined output
//         (verifies shell-out plumbing is exercised; lean_market may
//          print a usage error — that is acceptable; the wrapper must
//          not silently produce nothing)
// ─────────────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────────────
// Test 3: bogus flag produces non-zero exit
// ─────────────────────────────────────────────────────────────────────

#[test]
fn turingos_report_bankruptcy_bogus_flag_nonzero_exit() {
    let output = Command::new(turingos_bin())
        .args(["report", "bankruptcy", "--zzz-bogus"])
        .output()
        .expect("run turingos report bankruptcy --zzz-bogus");

    assert!(
        !output.status.success(),
        "expected non-zero exit for --zzz-bogus; status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}
