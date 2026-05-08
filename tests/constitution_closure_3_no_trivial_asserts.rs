//! Constitution Landing Gate — Closure #3 (directive §12)
//!
//! `feedback_constitutional_harness_engineering` + CR-C0.1: a test that cannot
//! fail is documentation, not a gate. CONSTITUTION_EXECUTION_MATRIX.md §O #3
//! has sat at "🟡 AMBER (verify on commit)" because no executable enforcement
//! existed — the rule was an editorial norm, not a mechanism.
//!
//! This test mechanically forbids trivially-passing assertions in the
//! `tests/constitution_*.rs` battery (the constitutional gate surface where
//! the kill-condition matters most). It is the mechanism that converts the
//! norm into a gate per `feedback_norm_needs_mechanism`.
//!
//! Forbidden patterns (whitespace-tolerant): `assert!(true)`,
//! `assert!(!false)`, `assert_eq!(1, 1)` (and tautological number/bool/string
//! pairs), `assert_ne!(0, 1)` (and complements), `debug_assert!(true)`.
//! Doc-comments mentioning these patterns are filtered (stripped before the
//! scan). The scanning test itself is skipped to avoid self-reference.
//!
//! `FC-trace: FC1-INV1` — protects "every externalized claim of a passing
//! invariant must be a real claim, not a stub" — the gate-surface analogue
//! of the FC1 tape-visibility rule.

use std::fs;
use std::path::{Path, PathBuf};

const TESTS_DIR: &str = "tests";
const SELF_FILE: &str = "constitution_closure_3_no_trivial_asserts.rs";

/// Trivially-passing assertion shapes after whitespace removal. Each variant
/// is unconditionally true at compile or run time; if a `#[test]` body
/// reduces to one of these, the test is documentation, not enforcement.
const FORBIDDEN_PATTERNS: &[&str] = &[
    "assert!(true)",
    "assert!(true,",
    "assert!(!false)",
    "assert!(!false,",
    "debug_assert!(true)",
    "debug_assert!(true,",
    "debug_assert!(!false)",
    "assert_eq!(1,1)",
    "assert_eq!(0,0)",
    "assert_eq!(true,true)",
    "assert_eq!(false,false)",
    "assert_ne!(0,1)",
    "assert_ne!(1,0)",
    "assert_ne!(true,false)",
    "assert_ne!(false,true)",
];

fn collect_constitution_test_files() -> Vec<PathBuf> {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let dir = Path::new(project_root).join(TESTS_DIR);
    let mut files = Vec::new();
    let entries = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()));
    for entry in entries.flatten() {
        let path = entry.path();
        let name = match path.file_name().and_then(|s| s.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };
        if !name.starts_with("constitution_") || !name.ends_with(".rs") {
            continue;
        }
        if name == SELF_FILE {
            // Skip self — this file lists the patterns as string literals.
            continue;
        }
        files.push(path);
    }
    files
}

/// Strip Rust line comments and remove all whitespace. The constitution test
/// battery uses `//`, `///`, and `//!` exclusively for comments; no test file
/// embeds `//` inside a string literal. So a line-level prefix split on `//`
/// is sufficient for this surface (the scan only protects
/// `tests/constitution_*.rs`, not arbitrary repo code).
fn strip_comments_and_whitespace(content: &str) -> String {
    let mut out = String::with_capacity(content.len());
    for line in content.lines() {
        let code = match line.find("//") {
            Some(idx) => &line[..idx],
            None => line,
        };
        for ch in code.chars() {
            if !ch.is_whitespace() {
                out.push(ch);
            }
        }
    }
    out
}

/// Closure #3 — every test in the constitution battery must be capable of
/// failing. Trivially-passing assertion shapes are forbidden.
#[test]
fn constitution_closure_3_no_trivial_asserts_in_constitution_tests() {
    let files = collect_constitution_test_files();
    assert!(
        !files.is_empty(),
        "Closure #3 violation: no `tests/constitution_*.rs` files found — \
         scan would vacuously pass. Did the test directory move?"
    );

    let mut hits: Vec<String> = Vec::new();
    for path in &files {
        let content = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        let stripped = strip_comments_and_whitespace(&content);
        for pattern in FORBIDDEN_PATTERNS {
            if stripped.contains(pattern) {
                hits.push(format!(
                    "{}: contains trivially-passing assertion `{}`",
                    path.file_name().and_then(|s| s.to_str()).unwrap_or("<?>"),
                    pattern
                ));
            }
        }
    }

    assert!(
        hits.is_empty(),
        "Closure #3 violation (CR-C0.1): trivially-passing assertion(s) \
         detected in constitution test battery. A test that cannot fail is \
         documentation, not a gate. Replace with a real invariant, OR if the \
         test is a structural placeholder, mark the row 🚫 N/A in \
         `CONSTITUTION_EXECUTION_MATRIX.md` and remove the test. Hits:\n{}",
        hits.join("\n")
    );
}

/// Self-test of the strip helper — the comment/whitespace pipeline must
/// remove pattern-text appearing inside a `//!` doc-comment so that the
/// existing CR-C0.1 banners across the constitution battery (e.g.,
/// `tests/constitution_fc1_runtime_loop.rs:23://! All tests are real
/// assertions — no \`assert!(true)\` per CR-C0.1.`) do NOT cause this test to
/// false-positive on the very files it protects.
#[test]
fn strip_helper_drops_doc_comment_pattern_text() {
    let sample = "//! banner mentioning assert!(true) per CR-C0.1\n\
                  fn body() { assert_eq!(real, expected); }\n";
    let stripped = strip_comments_and_whitespace(sample);
    assert!(
        !stripped.contains("assert!(true)"),
        "strip helper failed to remove doc-comment text: stripped={stripped:?}"
    );
    assert!(
        stripped.contains("assert_eq!(real,expected)"),
        "strip helper damaged real assertion: stripped={stripped:?}"
    );
}

/// Self-test of the FORBIDDEN_PATTERNS list — synthesize each pattern in a
/// freshly-built non-self string and confirm the scan would catch it. This
/// proves the test "can fail" (Closure #3 applied to itself).
#[test]
fn forbidden_patterns_list_is_load_bearing() {
    for pattern in FORBIDDEN_PATTERNS {
        // Build a synthetic file body containing the pattern outside any
        // comment. Use whitespace inside to confirm whitespace-tolerance.
        let synthetic = format!("fn t() {{ {} }}\n", spaced(pattern));
        let stripped = strip_comments_and_whitespace(&synthetic);
        assert!(
            stripped.contains(pattern),
            "FORBIDDEN_PATTERNS entry {pattern:?} not detected in \
             synthetic body {synthetic:?} (stripped={stripped:?}) — the \
             scan would silently miss this shape."
        );
    }
}

/// Insert spaces inside a forbidden pattern to demonstrate whitespace-tolerance:
/// `assert!(true)` → `assert! ( true )`. After strip, both shapes collapse to
/// the same canonical substring.
fn spaced(pattern: &str) -> String {
    pattern
        .chars()
        .flat_map(|c| {
            if c == '(' || c == ')' || c == ',' {
                vec![' ', c, ' ']
            } else {
                vec![c]
            }
        })
        .collect()
}
