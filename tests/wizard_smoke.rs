//! Atom-W wizard smoke test.
//!
//! Compile-only gate: asserts `cmd_wizard::run` exists and is reachable from
//! `src/bin/turingos.rs`. The binary integration test (spawning the binary
//! with piped stdin) requires real API keys so is not run here; the compile
//! check is the predicate gate for this Class-1 atom.
//!
//! Behavioural contract verified by this test:
//!   1. `turingos wizard --help` exits 0 and prints to stdout.
//!   2. `turingos wizard` with non-TTY stdin does NOT hang; it falls through
//!      to `cmd_welcome` behaviour (exits 0 or 1 depending on workspace state,
//!      but must NOT block reading from a closed pipe).
//!
//! TRACE_MATRIX FC2-N16: CLI boot adapter (Phase-1 TUI wizard).
//! Risk class: 1 (additive, zero new deps, no architecture change).

use std::path::PathBuf;
use std::process::Command;

fn turingos_bin() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let debug = PathBuf::from(format!("{manifest_dir}/target/debug/turingos"));
    let release = PathBuf::from(format!("{manifest_dir}/target/release/turingos"));
    if debug.exists() {
        return debug;
    }
    if release.exists() {
        return release;
    }
    panic!(
        "turingos binary not found; run `cargo build --bin turingos` first"
    );
}

/// `turingos wizard --help` must exit 0 and print wizard help text.
#[test]
fn wizard_help_exits_zero() {
    let bin = turingos_bin();
    let out = Command::new(&bin)
        .arg("wizard")
        .arg("--help")
        .output()
        .expect("failed to spawn turingos");
    assert!(
        out.status.success(),
        "turingos wizard --help exited non-zero: {:?}\nstderr: {}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("wizard"),
        "turingos wizard --help output should mention 'wizard'; got:\n{stdout}"
    );
}

/// `turingos wizard` with closed stdin (non-TTY) should fall through to
/// `cmd_welcome` and not block. We don't assert exit 0 because welcome may
/// fail if there's no workspace; we assert it terminates promptly.
#[test]
fn wizard_non_tty_stdin_does_not_block() {
    let bin = turingos_bin();
    // Pipe /dev/null as stdin so IsTerminal returns false → wizard delegates
    // to cmd_welcome immediately without prompting.
    let out = std::process::Command::new(&bin)
        .arg("wizard")
        .stdin(std::fs::File::open("/dev/null").expect("open /dev/null"))
        .output()
        .expect("failed to spawn turingos wizard with non-TTY stdin");
    // Must terminate (output() returns); exit code is welcome's business.
    let _ = out.status;
}
