//! B8: subprocess test — `turingos init --provider deepseek` stdout contains
//! all three required export hints for the DeepSeek dual-key path.
//!
//! TRACE_MATRIX FC2-N16: init subcommand (provider flag, next-steps output).
//! Risk class: 2 (production wire-up, additive output).

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
        "turingos binary not found at debug or release paths; \
         run `cargo build --bin turingos` first"
    );
}

/// B8: `turingos init --provider deepseek` stdout must include all 3 export
/// hints so a non-developer user can copy-paste them directly.
#[test]
fn init_deepseek_next_steps_contains_all_export_hints() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let ws = tmp.path().join("ws_ds_hints");

    let output = Command::new(turingos_bin())
        .arg("init")
        .arg("--project")
        .arg(&ws)
        .arg("--provider")
        .arg("deepseek")
        .output()
        .expect("spawn turingos init --provider deepseek");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    assert!(
        output.status.success(),
        "init --provider deepseek must succeed;\nexit={:?}\nstdout={stdout}\nstderr={stderr}",
        output.status
    );

    // All three export hints must appear in stdout.
    assert!(
        stdout.contains("DEEPSEEK_API_KEY="),
        "stdout must contain DEEPSEEK_API_KEY= export hint;\nstdout={stdout}"
    );
    assert!(
        stdout.contains("DEEPSEEK_API_KEY_WORKER="),
        "stdout must contain DEEPSEEK_API_KEY_WORKER= export hint;\nstdout={stdout}"
    );
    assert!(
        stdout.contains("TURINGOS_SILICONFLOW_ENDPOINT="),
        "stdout must contain TURINGOS_SILICONFLOW_ENDPOINT= export hint;\nstdout={stdout}"
    );
}

/// Regression: default (siliconflow) provider must NOT print DeepSeek env hints.
#[test]
fn init_default_provider_does_not_print_deepseek_hints() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let ws = tmp.path().join("ws_sf_hints");

    let output = Command::new(turingos_bin())
        .arg("init")
        .arg("--project")
        .arg(&ws)
        .output()
        .expect("spawn turingos init (default provider)");

    assert!(
        output.status.success(),
        "default init must succeed; exit={:?}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    assert!(
        !stdout.contains("DEEPSEEK_API_KEY_WORKER"),
        "siliconflow (default) stdout must not mention DEEPSEEK_API_KEY_WORKER;\nstdout={stdout}"
    );
    assert!(
        stdout.contains("SILICONFLOW_API_KEY"),
        "siliconflow stdout must mention SILICONFLOW_API_KEY;\nstdout={stdout}"
    );
}
