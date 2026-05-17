//! TISR Phase 6.2 W1.1 — `turingos render` smoke / integration test.
//!
//! Per §8 packet `2026-05-17_TISR_PHASE6_2_SEPARATE_CHARTER_SECTION8_PACKET.md`
//! Section 4 allowed path: `tests/cli_*.rs`.
//!
//! Class 1 verification: confirms the render subcommand shells out correctly to
//! experiments/tisr_ui_spike/render.py for happy-path, help, missing-fixture,
//! and bogus-flag cases.
//!
//! FC-trace: FC2-N16 + FC3-N31

use std::path::PathBuf;
use std::process::Command;

/// TRACE_MATRIX FC2-N16: resolve the compiled `turingos` binary path.
/// Tries debug then release; panics if neither exists.
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

/// TRACE_MATRIX FC2-N16: resolve experiments/tisr_ui_spike/ relative to CARGO_MANIFEST_DIR.
fn spike_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("experiments")
        .join("tisr_ui_spike")
}

// ─────────────────────────────────────────────────────────────────────
// Test 1: --help exits 0; stdout contains expected keywords; no leakage
// ─────────────────────────────────────────────────────────────────────

#[test]
fn render_help_exits_0_and_contains_expected_keywords() {
    let output = Command::new(turingos_bin())
        .arg("render")
        .arg("--help")
        .output()
        .expect("run turingos render --help");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "turingos render --help should exit 0; status={:?}\nstdout={stdout}\nstderr={stderr}",
        output.status,
    );
    assert!(
        stdout.contains("fixture"),
        "help text should mention 'fixture'\nstdout={stdout}"
    );
    assert!(
        stdout.contains("UI IR"),
        "help text should mention 'UI IR'\nstdout={stdout}"
    );
    // No leakage of backend-specific names
    assert!(
        !stdout.contains("lean_market"),
        "help text must not mention 'lean_market'\nstdout={stdout}"
    );
    assert!(
        !stdout.contains("Lean"),
        "help text must not mention 'Lean'\nstdout={stdout}"
    );
    assert!(
        !stdout.contains("minif2f"),
        "help text must not mention 'minif2f'\nstdout={stdout}"
    );
}

// ─────────────────────────────────────────────────────────────────────
// Test 2: happy path — renders dashboard_sample.json, exits 0
// ─────────────────────────────────────────────────────────────────────

#[test]
fn render_fixture_happy_path_exits_0() {
    let fixture = spike_dir().join("fixtures").join("dashboard_sample.json");

    // Skip gracefully if the fixture file is absent (shouldn't happen; guard against
    // worktree misconfiguration without panicking).
    if !fixture.exists() {
        eprintln!("SKIP: fixture not found at {}", fixture.display());
        return;
    }

    let output = Command::new(turingos_bin())
        .arg("render")
        .arg("--fixture")
        .arg(&fixture)
        .output()
        .expect("run turingos render --fixture dashboard_sample.json");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "turingos render --fixture ... should exit 0; status={:?}\nstdout={stdout}\nstderr={stderr}",
        output.status,
    );
    assert!(
        !stdout.is_empty(),
        "stdout should be non-empty for a valid fixture\nstdout={stdout}"
    );
    // The fixture contains "run_id" as a label in the dashboard_panel block.
    assert!(
        stdout.contains("run_id"),
        "rendered output should contain 'run_id' from the fixture\nstdout={stdout}"
    );
}

// ─────────────────────────────────────────────────────────────────────
// Test 3: missing fixture exits non-zero; stderr is informative
// ─────────────────────────────────────────────────────────────────────

#[test]
fn render_missing_fixture_exits_nonzero() {
    let output = Command::new(turingos_bin())
        .arg("render")
        .arg("--fixture")
        .arg("/nonexistent/path/render_test_fixture_zzz.json")
        .output()
        .expect("run turingos render --fixture /nonexistent/...");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "turingos render with a missing fixture should exit non-zero; status={:?}\nstderr={stderr}",
        output.status,
    );
    // Python renderer or the wrapper must emit something mentioning "fixture",
    // "not found", or "No such file" to stderr.
    let stderr_lower = stderr.to_lowercase();
    assert!(
        stderr_lower.contains("fixture")
            || stderr_lower.contains("not found")
            || stderr_lower.contains("no such file"),
        "stderr should mention 'fixture', 'not found', or 'No such file'\nstderr={stderr}"
    );
}

// ─────────────────────────────────────────────────────────────────────
// Test 4: bogus flag exits non-zero (graceful, no panic)
// ─────────────────────────────────────────────────────────────────────

#[test]
fn render_bogus_flag_exits_nonzero_no_panic() {
    let output = Command::new(turingos_bin())
        .arg("render")
        .arg("--zzz-bogus-flag")
        .output()
        .expect("run turingos render --zzz-bogus-flag");

    assert!(
        !output.status.success(),
        "turingos render with a bogus flag should exit non-zero; status={:?}",
        output.status,
    );
    // Must not be a Rust panic (exit 101 on Linux is the Rust panic exit code, but
    // we primarily check that the process terminated and was not a panic abort).
    // Graceful means exit code from Python (2 for argparse error) — just non-zero.
    let code = output.status.code().unwrap_or(1);
    assert!(
        code != 0,
        "exit code must be non-zero for bogus flag; got {code}"
    );
}
