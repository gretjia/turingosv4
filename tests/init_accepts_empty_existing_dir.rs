//! B2: Subprocess test — `turingos init --project .` accepts an empty
//! existing directory without --force.
//!
//! TRACE_MATRIX FC2-N16: init subcommand (UX hardening).
//! Risk class: 2 (additive, production wire-up).

use std::fs;
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

/// B2: An empty existing directory should be accepted by `turingos init`
/// without `--force`.
#[test]
fn init_accepts_empty_existing_dir_without_force() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    // Create an empty subdirectory inside tmp.
    let empty_dir = tmp.path().join("empty_ws");
    fs::create_dir_all(&empty_dir).expect("create empty dir");

    // Confirm it is indeed empty.
    let count = fs::read_dir(&empty_dir).expect("read dir").count();
    assert_eq!(count, 0, "precondition: directory must be empty");

    let output = Command::new(turingos_bin())
        .arg("init")
        .arg("--project")
        .arg(&empty_dir)
        .output()
        .expect("spawn turingos init");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "init should succeed for an empty existing dir (no --force needed);\
         \nexit={:?}\nstdout={stdout}\nstderr={stderr}",
        output.status
    );

    // genesis_payload.toml must exist after init.
    let genesis = empty_dir.join("genesis_payload.toml");
    assert!(
        genesis.exists(),
        "genesis_payload.toml must be created after init; dir={}", empty_dir.display()
    );
}

/// B2 regression: a NON-empty existing dir must still require --force.
#[test]
fn init_rejects_non_empty_existing_dir_without_force() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let nonempty_dir = tmp.path().join("nonempty_ws");
    fs::create_dir_all(&nonempty_dir).expect("create dir");
    // Plant a file so the directory is non-empty.
    fs::write(nonempty_dir.join("existing_file.txt"), "content").expect("write file");

    let output = Command::new(turingos_bin())
        .arg("init")
        .arg("--project")
        .arg(&nonempty_dir)
        .output()
        .expect("spawn turingos init");

    assert!(
        !output.status.success(),
        "init must fail for non-empty dir without --force; exit={:?}",
        output.status
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("not empty") || stderr.contains("--force"),
        "error message should mention 'not empty' or '--force'; got:\n{stderr}"
    );
}

/// B2 regression: non-empty dir SUCCEEDS with --force.
#[test]
fn init_with_force_overwrites_non_empty_dir() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let ws = tmp.path().join("force_ws");
    fs::create_dir_all(&ws).expect("create dir");
    fs::write(ws.join("existing_file.txt"), "old content").expect("write file");

    let output = Command::new(turingos_bin())
        .arg("init")
        .arg("--project")
        .arg(&ws)
        .arg("--force")
        .output()
        .expect("spawn turingos init --force");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "init --force should succeed for non-empty dir;\
         \nexit={:?}\nstdout={stdout}\nstderr={stderr}",
        output.status
    );

    assert!(
        ws.join("genesis_payload.toml").exists(),
        "genesis_payload.toml must exist after --force init"
    );
    // The pre-existing file must still be there (not deleted).
    assert!(
        ws.join("existing_file.txt").exists(),
        "pre-existing files must be preserved with --force"
    );
}
