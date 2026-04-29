//! CO1.13.2 — Rust orchestrator for the R-022 shell integration suite.
//!
//! Provides `cargo test` discoverability for `tests/integration/co1_13/run_all.sh`
//! per spec § 3.10. The shell suite exercises real `git init` temp repos and
//! drives the pre-commit hook; this Rust harness simply invokes it and asserts
//! exit-status zero. Skipped on platforms where `bash` is unavailable.

use std::env;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn r_022_shell_integration_suite_passes() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let runner = manifest_dir.join("tests/integration/co1_13/run_all.sh");
    assert!(
        runner.exists(),
        "missing run_all.sh at {}",
        runner.display()
    );

    let status = Command::new("bash")
        .arg(&runner)
        .current_dir(&manifest_dir)
        .status()
        .expect("failed to spawn bash");
    assert!(
        status.success(),
        "R-022 shell integration suite failed (exit {:?}); see run_all.sh output above",
        status.code()
    );
}
