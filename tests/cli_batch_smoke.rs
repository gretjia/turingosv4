//! TISR Phase 6.0/6.1 alpha — `turingos batch` smoke / integration test.
//!
//! TRACE_MATRIX FC2-N16: batch scaffold smoke tests.
//!
//! Per §8 packet `2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md`
//! Section 4 allowed path: `tests/cli_*.rs`.
//!
//! Class 1 verification: confirms the batch subcommand produces the expected
//! scaffold directories, rejects duplicates, and rejects invalid batch names.
//!
//! [R-022-skip: see handover/alignment/OBS_R022_TISR_PHASE6_1_CLI_DISPATCH.md]

use std::path::PathBuf;
use std::process::Command;

/// Resolve the compiled `turingos` binary path.
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

/// Test 1: `turingos batch --help` exits 0 + stdout contains "batch" + "new/list/view".
#[test]
fn turingos_batch_help_exits_zero_and_contains_key_terms() {
    let output = Command::new(turingos_bin())
        .arg("batch")
        .arg("--help")
        .output()
        .expect("run turingos batch --help");

    assert!(
        output.status.success(),
        "turingos batch --help should exit 0; status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("batch"),
        "help output should mention 'batch'; got: {stdout}",
    );
    assert!(
        stdout.contains("new") && stdout.contains("list") && stdout.contains("view"),
        "help output should describe new/list/view actions; got: {stdout}",
    );
}

/// Test 2: `turingos batch new --name first --workspace <tmp>` succeeds +
/// batches/first/manifest.toml created.
#[test]
fn turingos_batch_new_creates_manifest() {
    let tmp = tempfile::TempDir::new().expect("create tempdir");
    let workspace = tmp.path().to_str().expect("tempdir path to str");

    let output = Command::new(turingos_bin())
        .arg("batch")
        .arg("new")
        .arg("--name")
        .arg("first")
        .arg("--workspace")
        .arg(workspace)
        .output()
        .expect("run turingos batch new");

    assert!(
        output.status.success(),
        "turingos batch new should succeed; status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let manifest_path = tmp
        .path()
        .join("batches")
        .join("first")
        .join("manifest.toml");
    assert!(
        manifest_path.is_file(),
        "batches/first/manifest.toml should exist; path={manifest_path:?}",
    );

    let content = std::fs::read_to_string(&manifest_path).expect("read manifest.toml");
    assert!(
        content.contains("name = \"first\""),
        "manifest should contain name = \"first\"; got:\n{content}",
    );
    assert!(
        content.contains("status = \"scaffold\""),
        "manifest should contain status = \"scaffold\"; got:\n{content}",
    );
}

/// Test 3: `turingos batch list --workspace <tmp>` lists "first" after creation.
#[test]
fn turingos_batch_list_shows_created_batch() {
    let tmp = tempfile::TempDir::new().expect("create tempdir");
    let workspace = tmp.path().to_str().expect("tempdir path to str");

    // Setup: create a batch first.
    let new_out = Command::new(turingos_bin())
        .arg("batch")
        .arg("new")
        .arg("--name")
        .arg("first")
        .arg("--workspace")
        .arg(workspace)
        .output()
        .expect("run turingos batch new");
    assert!(
        new_out.status.success(),
        "setup new failed: {:?}",
        String::from_utf8_lossy(&new_out.stderr)
    );

    // Now list.
    let output = Command::new(turingos_bin())
        .arg("batch")
        .arg("list")
        .arg("--workspace")
        .arg(workspace)
        .output()
        .expect("run turingos batch list");

    assert!(
        output.status.success(),
        "turingos batch list should succeed; status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("first"),
        "list output should mention 'first'; got: {stdout}",
    );
}

/// Test 4: `turingos batch view --name first --workspace <tmp>` shows manifest content.
#[test]
fn turingos_batch_view_shows_manifest_content() {
    let tmp = tempfile::TempDir::new().expect("create tempdir");
    let workspace = tmp.path().to_str().expect("tempdir path to str");

    // Setup: create a batch first.
    let new_out = Command::new(turingos_bin())
        .arg("batch")
        .arg("new")
        .arg("--name")
        .arg("first")
        .arg("--workspace")
        .arg(workspace)
        .output()
        .expect("run turingos batch new");
    assert!(
        new_out.status.success(),
        "setup new failed: {:?}",
        String::from_utf8_lossy(&new_out.stderr)
    );

    // View the manifest.
    let output = Command::new(turingos_bin())
        .arg("batch")
        .arg("view")
        .arg("--name")
        .arg("first")
        .arg("--workspace")
        .arg(workspace)
        .output()
        .expect("run turingos batch view");

    assert!(
        output.status.success(),
        "turingos batch view should succeed; status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("name = \"first\""),
        "view output should contain manifest content; got: {stdout}",
    );
    assert!(
        stdout.contains("status = \"scaffold\""),
        "view output should show scaffold status; got: {stdout}",
    );
}

/// Test 5: `turingos batch new --name first --workspace <tmp>` returns non-zero exit
/// when the batch already exists.
#[test]
fn turingos_batch_new_rejects_duplicate_name() {
    let tmp = tempfile::TempDir::new().expect("create tempdir");
    let workspace = tmp.path().to_str().expect("tempdir path to str");

    // First creation — should succeed.
    let first = Command::new(turingos_bin())
        .arg("batch")
        .arg("new")
        .arg("--name")
        .arg("first")
        .arg("--workspace")
        .arg(workspace)
        .output()
        .expect("run turingos batch new (first)");
    assert!(
        first.status.success(),
        "first creation should succeed; stderr={}",
        String::from_utf8_lossy(&first.stderr)
    );

    // Second creation — should fail.
    let second = Command::new(turingos_bin())
        .arg("batch")
        .arg("new")
        .arg("--name")
        .arg("first")
        .arg("--workspace")
        .arg(workspace)
        .output()
        .expect("run turingos batch new (second)");

    assert!(
        !second.status.success(),
        "second creation should fail (already exists); status={:?}",
        second.status,
    );
}

/// Test 6: `turingos batch new --name ../bad --workspace <tmp>` returns non-zero exit
/// (invalid name with path traversal).
#[test]
fn turingos_batch_new_rejects_invalid_name() {
    let tmp = tempfile::TempDir::new().expect("create tempdir");
    let workspace = tmp.path().to_str().expect("tempdir path to str");

    let output = Command::new(turingos_bin())
        .arg("batch")
        .arg("new")
        .arg("--name")
        .arg("../bad")
        .arg("--workspace")
        .arg(workspace)
        .output()
        .expect("run turingos batch new --name ../bad");

    assert!(
        !output.status.success(),
        "batch new with invalid name '../bad' should fail; status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}
