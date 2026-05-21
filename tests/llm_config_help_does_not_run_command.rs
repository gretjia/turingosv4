//! X2: `turingos llm config --help` must print FULL_HELP (including
//! DEEPSEEK/ANTHROPIC/OPENAI examples) and exit 0 WITHOUT writing turingos.toml.
//!
//! Before this fix, `turingos llm config --help` would silently run the config
//! command, writing turingos.toml to the current working directory. That is a
//! side effect the user did not ask for and cannot see (no output was printed).
//!
//! TRACE_MATRIX FC2-N16: cmd_llm --help guard (UX hardening, X2 fix).
//! Risk class: 1 (additive test; no production code change beyond the guard).

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

/// X2: `turingos llm config --help` exits 0, prints provider examples, does NOT
/// write turingos.toml.
#[test]
fn llm_config_help_exits_zero_and_shows_provider_examples() {
    let tmp = tempfile::TempDir::new().expect("tempdir");

    let output = Command::new(turingos_bin())
        .arg("llm")
        .arg("config")
        .arg("--help")
        .current_dir(tmp.path())
        .output()
        .expect("spawn turingos llm config --help");

    // Must exit 0.
    assert!(
        output.status.success(),
        "turingos llm config --help must exit 0; got {:?}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    // Must contain all three provider examples added by PR #70.
    assert!(
        stdout.contains("DEEPSEEK DUAL-KEY EXAMPLE"),
        "stdout must contain DEEPSEEK DUAL-KEY EXAMPLE; got:\n{stdout}"
    );
    assert!(
        stdout.contains("ANTHROPIC EXAMPLE"),
        "stdout must contain ANTHROPIC EXAMPLE; got:\n{stdout}"
    );
    assert!(
        stdout.contains("OPENAI EXAMPLE"),
        "stdout must contain OPENAI EXAMPLE; got:\n{stdout}"
    );

    // Critical: turingos.toml must NOT have been written to the tempdir.
    let toml_path = tmp.path().join("turingos.toml");
    assert!(
        !toml_path.exists(),
        "turingos llm config --help must NOT write turingos.toml; \
         but found it at {toml_path:?}"
    );
}

/// X2: `turingos llm show --help` also exits cleanly without side effects.
#[test]
fn llm_show_help_exits_zero() {
    let tmp = tempfile::TempDir::new().expect("tempdir");

    let output = Command::new(turingos_bin())
        .arg("llm")
        .arg("show")
        .arg("--help")
        .current_dir(tmp.path())
        .output()
        .expect("spawn turingos llm show --help");

    assert!(
        output.status.success(),
        "turingos llm show --help must exit 0; got {:?}",
        output.status
    );
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    assert!(
        stdout.contains("turingos llm"),
        "help must contain usage text; got:\n{stdout}"
    );
}

/// X2: `turingos llm complete --help` exits cleanly.
#[test]
fn llm_complete_help_exits_zero() {
    let tmp = tempfile::TempDir::new().expect("tempdir");

    // complete has its own --help handler but let's verify it doesn't crash.
    let output = Command::new(turingos_bin())
        .arg("llm")
        .arg("complete")
        .arg("--help")
        .current_dir(tmp.path())
        .output()
        .expect("spawn turingos llm complete --help");

    assert!(
        output.status.success(),
        "turingos llm complete --help must exit 0; got {:?}",
        output.status
    );
}
