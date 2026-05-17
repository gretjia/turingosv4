//! TISR Phase 6.1 W1b.6 — `turingos verify chaintape` smoke tests.

use std::path::PathBuf;
use std::process::Command;

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

#[test]
fn turingos_verify_chaintape_help_exits_zero() {
    let output = Command::new(turingos_bin())
        .arg("verify")
        .arg("chaintape")
        .arg("--help")
        .output()
        .expect("run turingos");
    assert!(output.status.success(), "expected --help to exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("chaintape") || stdout.contains("verify") || stdout.contains("integrity"),
        "help text missing expected keyword; got: {stdout}"
    );
}

#[test]
fn turingos_verify_chaintape_bogus_flag_nonzero_exit() {
    let output = Command::new(turingos_bin())
        .arg("verify")
        .arg("chaintape")
        .arg("--zzz-bogus")
        .output()
        .expect("run turingos");
    assert!(
        !output.status.success(),
        "expected non-zero exit on bogus flag --zzz-bogus"
    );
}
