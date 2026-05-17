//! TISR Phase 6.1 W2.4 — `turingos export evidence` smoke / integration tests.
//!
//! FC-trace: FC2-N16 (boot/genesis gate — evidence bundle export).
//! Class 1 verification: filesystem read+write; no sequencer; no typed_tx.
//!
//! 5 tests:
//!   1. --help exits 0 + output contains "export" and "evidence"
//!   2. Happy path: copies source files to --out, reports counts
//!   3. --source nonexistent → non-zero exit
//!   4. --source is a file (not dir) → non-zero exit
//!   5. --out already exists → non-zero exit

use std::fs;
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

// ─────────────────────────────────────────────────────────────────────
// Test 1: --help exits 0, output contains "export" and "evidence"
// ─────────────────────────────────────────────────────────────────────

#[test]
fn turingos_export_evidence_help_exits_zero_and_contains_keywords() {
    let output = Command::new(turingos_bin())
        .arg("export")
        .arg("evidence")
        .arg("--help")
        .output()
        .expect("run turingos");

    assert!(
        output.status.success(),
        "turingos export evidence --help returned non-zero: status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("export"),
        "help output missing 'export': {stdout}"
    );
    assert!(
        stdout.contains("evidence"),
        "help output missing 'evidence': {stdout}"
    );
}

// ─────────────────────────────────────────────────────────────────────
// Test 2: Happy path — copies files, reports correct counts
// ─────────────────────────────────────────────────────────────────────

#[test]
fn turingos_export_evidence_copies_files_and_reports_counts() {
    let tmp = tempfile::TempDir::new().expect("create tempdir");

    // Set up source directory with 3 files.
    let src = tmp.path().join("evidence_src");
    fs::create_dir_all(&src).expect("create src dir");
    fs::write(
        src.join("genesis_report.json"),
        r#"{"run_id":"test-run-001"}"#,
    )
    .expect("write genesis_report.json");
    fs::write(src.join("chaintape.jsonl"), "{\"tx_id\":\"tx_0001\"}\n")
        .expect("write chaintape.jsonl");
    let cas_dir = src.join("cas");
    fs::create_dir_all(&cas_dir).expect("create cas subdir");
    fs::write(cas_dir.join("object_001.json"), r#"{"cid":"abc123"}"#).expect("write cas object");

    let out = tmp.path().join("evidence_bundle");

    let output = Command::new(turingos_bin())
        .arg("export")
        .arg("evidence")
        .arg("--source")
        .arg(&src)
        .arg("--out")
        .arg(&out)
        .output()
        .expect("run turingos");

    assert!(
        output.status.success(),
        "turingos export evidence failed: status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    // Output directory must exist.
    assert!(out.exists(), "output directory not created");
    assert!(out.is_dir(), "output path is not a directory");

    // All 3 source files must be present in the output.
    assert!(
        out.join("genesis_report.json").is_file(),
        "genesis_report.json missing from bundle"
    );
    assert!(
        out.join("chaintape.jsonl").is_file(),
        "chaintape.jsonl missing from bundle"
    );
    assert!(
        out.join("cas").join("object_001.json").is_file(),
        "cas/object_001.json missing from bundle"
    );

    // Stdout must report file count and bytes.
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("files"),
        "stdout missing 'files' count: {stdout}"
    );
    assert!(
        stdout.contains("bytes"),
        "stdout missing 'bytes' count: {stdout}"
    );
    // Must mention both source and destination paths.
    assert!(
        stdout.contains("exported"),
        "stdout missing 'exported' keyword: {stdout}"
    );
}

// ─────────────────────────────────────────────────────────────────────
// Test 3: --source nonexistent → non-zero exit
// ─────────────────────────────────────────────────────────────────────

#[test]
fn turingos_export_evidence_nonexistent_source_fails() {
    let tmp = tempfile::TempDir::new().expect("create tempdir");
    let out = tmp.path().join("should_not_be_created");

    let output = Command::new(turingos_bin())
        .arg("export")
        .arg("evidence")
        .arg("--source")
        .arg("/nonexistent/path/that/does/not/exist")
        .arg("--out")
        .arg(&out)
        .output()
        .expect("run turingos");

    assert!(
        !output.status.success(),
        "export should fail when --source does not exist; got exit 0"
    );

    // Output directory must NOT have been created.
    assert!(
        !out.exists(),
        "output dir was created even though source was missing"
    );
}

// ─────────────────────────────────────────────────────────────────────
// Test 4: --source is a file (not dir) → non-zero exit
// ─────────────────────────────────────────────────────────────────────

#[test]
fn turingos_export_evidence_source_is_file_fails() {
    let tmp = tempfile::TempDir::new().expect("create tempdir");

    // Create a regular file at the source path.
    let src_file = tmp.path().join("not_a_dir.txt");
    fs::write(&src_file, "this is a file, not a directory").expect("write src file");

    let out = tmp.path().join("bundle_out");

    let output = Command::new(turingos_bin())
        .arg("export")
        .arg("evidence")
        .arg("--source")
        .arg(&src_file)
        .arg("--out")
        .arg(&out)
        .output()
        .expect("run turingos");

    assert!(
        !output.status.success(),
        "export should fail when --source is a regular file; got exit 0"
    );

    assert!(
        !out.exists(),
        "output dir was created even though source was a file"
    );
}

// ─────────────────────────────────────────────────────────────────────
// Test 5: --out already exists → non-zero exit
// ─────────────────────────────────────────────────────────────────────

#[test]
fn turingos_export_evidence_existing_out_fails() {
    let tmp = tempfile::TempDir::new().expect("create tempdir");

    // Set up source dir.
    let src = tmp.path().join("evidence_src2");
    fs::create_dir_all(&src).expect("create src dir");
    fs::write(src.join("run_summary.json"), r#"{"status":"ok"}"#).expect("write run_summary");

    // Pre-create the output directory.
    let out = tmp.path().join("already_exists");
    fs::create_dir_all(&out).expect("pre-create out dir");

    let output = Command::new(turingos_bin())
        .arg("export")
        .arg("evidence")
        .arg("--source")
        .arg(&src)
        .arg("--out")
        .arg(&out)
        .output()
        .expect("run turingos");

    assert!(
        !output.status.success(),
        "export should fail when --out already exists; got exit 0"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("already exists") || stderr.contains("force"),
        "stderr should mention 'already exists' or '--force': {stderr}"
    );
}
