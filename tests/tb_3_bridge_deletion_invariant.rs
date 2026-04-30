//! TB-3 Atom 7 — Bridge-resurrection CI invariant.
//!
//! Charter: `handover/tracer_bullets/TB-3_charter_2026-04-30.md` § 5 #14
//! ("No bridge resurrection") + § 4.7 last item.
//!
//! After TB-3 ships, the synthetic-TxId-from-TaskId pattern that the TB-2
//! P0-B option (a) bridge used (`TxId(work.task_id.0.clone())`) must never
//! reappear under `src/`. Per Art. I.1 ("约束转化为机器可执行的硬约束"),
//! this is enforced as a CI invariant — a Rust-native test that scans all
//! `src/**/*.rs` files for the forbidden literal and asserts zero hits.
//!
//! This is harder than ship-time grep because:
//! (a) it runs every `cargo test` invocation (not just at merge);
//! (b) it is platform-portable (no shell `grep` dependency);
//! (c) it is the constitutional contract — the rule lives in version-controlled
//!     test code, not in human review discipline.

use std::fs;
use std::path::{Path, PathBuf};

/// The forbidden literal pattern. Identifies the TB-2 P0-B option (a) bridge
/// or any morally-equivalent synthetic-TxId-from-TaskId construction in
/// admission code.
const FORBIDDEN: &str = "TxId(work.task_id.0.clone())";

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_rs_files(&path, out);
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                out.push(path);
            }
        }
    }
}

fn project_root() -> PathBuf {
    // Tests run from the project root. CARGO_MANIFEST_DIR points at the
    // workspace root for workspace tests. Use it to find src/.
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR set by cargo");
    PathBuf::from(manifest_dir)
}

/// Charter § 5 #14: bridge pattern must not resurrect anywhere under src/.
///
/// Excludes:
/// - the doc-comment in this very test file (we don't scan tests/);
/// - cases/ (constitutional case law may reference the historical bridge);
/// - handover/ (markdown docs may quote it as historical context).
#[test]
fn bridge_pattern_does_not_resurrect_in_src() {
    let root = project_root();
    let src_dir = root.join("src");
    assert!(src_dir.exists() && src_dir.is_dir(),
        "src/ directory must exist at {:?}", src_dir);

    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files);
    assert!(!files.is_empty(), "src/ must contain Rust files (sanity)");

    let mut hits: Vec<String> = Vec::new();
    for path in &files {
        let content = fs::read_to_string(path).unwrap_or_default();
        for (lineno, line) in content.lines().enumerate() {
            // Skip comment lines — only flag the pattern in CODE, not in
            // doc-comments that may legitimately reference the deleted
            // pattern as historical context (e.g., the WorkTx arm's note
            // explaining what was removed).
            let trimmed = line.trim_start();
            if trimmed.starts_with("//") {
                continue;
            }
            if line.contains(FORBIDDEN) {
                hits.push(format!("{}:{} | {}",
                    path.display(), lineno + 1, line.trim()));
            }
        }
    }

    assert!(
        hits.is_empty(),
        "TB-3 charter § 5 #14 violated — bridge pattern resurrected in src/:\n{}",
        hits.join("\n")
    );
}

/// Positive control: the scanner DOES find the forbidden literal when it
/// is present (e.g., in a temp file under tests/). This sanity-checks
/// that the scanner is actually working, not silently producing zero hits
/// because of a bug in path traversal or file reading.
#[test]
fn scanner_positive_control_finds_known_match() {
    use std::io::Write;
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let test_file = tmp.path().join("dummy.rs");
    let content = format!(
        "// known-clean test snippet\nlet x = {};\n",
        FORBIDDEN
    );
    let mut f = fs::File::create(&test_file).expect("create");
    f.write_all(content.as_bytes()).expect("write");

    let mut files = Vec::new();
    collect_rs_files(tmp.path(), &mut files);
    assert!(!files.is_empty(), "scanner finds the test file");

    let mut hits = 0;
    for path in &files {
        let content = fs::read_to_string(path).unwrap_or_default();
        for line in content.lines() {
            // Mirror production scanner: skip comment lines.
            let trimmed = line.trim_start();
            if trimmed.starts_with("//") {
                continue;
            }
            if line.contains(FORBIDDEN) {
                hits += 1;
            }
        }
    }
    assert_eq!(hits, 1, "scanner must find exactly 1 hit in the positive-control fixture");
}
