//! TB-17 SG-17.18 enforcement — MiniF2F scale ≠ real-world readiness.
//!
//! Per 2026-05-05 architect verdict §B.7 SG-17.18 + Q6.4 verbatim:
//!
//!     "Large-scale MiniF2F testing is NOT real-world readiness;
//!      not a substitute for TB-17 readiness gate."
//!
//! This test grep-checks that no source file under `src/` or
//! `experiments/minif2f_v4/src/` claims MiniF2F results constitute
//! real-world readiness (or P7 readiness, or production-pilot
//! qualification, etc.).
//!
//! Forbidden phrase patterns (case-insensitive):
//!   - "minif2f" + "real_world"  (any whitespace / punctuation between)
//!   - "minif2f" + "p7_ready"
//!   - "minif2f" + "production"   ... + "ready"
//!   - "mini_f2f" same combinations
//!
//! Allowed contexts:
//!   - inside comment blocks that explicitly disclaim the conflation
//!     (e.g., a `// FORBIDDEN: ...` or `// MUST NOT ...` warning)
//!   - test files like THIS one
//!
//! TRACE_MATRIX SG-17.18.

use std::path::{Path, PathBuf};

fn collect_rust_sources(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let walker = match std::fs::read_dir(root) {
        Ok(it) => it,
        Err(_) => return out,
    };
    let mut stack: Vec<PathBuf> = Vec::new();
    for ent in walker.flatten() {
        stack.push(ent.path());
    }
    while let Some(path) = stack.pop() {
        if path.is_dir() {
            // Skip non-Rust artifact directories.
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "target"
                || name == ".git"
                || name == "node_modules"
                || name == ".lake"
            {
                continue;
            }
            if let Ok(it) = std::fs::read_dir(&path) {
                for ent in it.flatten() {
                    stack.push(ent.path());
                }
            }
        } else if path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s == "rs")
            .unwrap_or(false)
        {
            out.push(path);
        }
    }
    out
}

fn line_is_disclaimer(line: &str) -> bool {
    let l = line.trim_start();
    if !l.starts_with("//") && !l.starts_with("*") && !l.starts_with("///") {
        return false;
    }
    let lower = l.to_lowercase();
    lower.contains("forbidden")
        || lower.contains("must not")
        || lower.contains("must-not")
        || lower.contains("anti-example")
        || lower.contains("not real-world")
        || lower.contains("not real world")
        || lower.contains("not p7")
        || lower.contains("≠ real")
        || lower.contains("!= real")
        || lower.contains("disclaimer")
}

fn forbidden_phrase_violations(path: &Path) -> Vec<(usize, String)> {
    let body = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };
    let mut violations = Vec::new();
    for (idx, line) in body.lines().enumerate() {
        let lower = line.to_lowercase();
        let has_minif2f = lower.contains("minif2f") || lower.contains("mini_f2f");
        if !has_minif2f {
            continue;
        }
        let conflates = (lower.contains("real_world") && lower.contains("ready"))
            || (lower.contains("real-world") && lower.contains("ready"))
            || lower.contains("p7_ready")
            || lower.contains("p7-ready")
            || (lower.contains("production") && lower.contains("ready"))
            || (lower.contains("production") && lower.contains("pilot"));
        if !conflates {
            continue;
        }
        if line_is_disclaimer(line) {
            continue;
        }
        violations.push((idx + 1, line.to_string()));
    }
    violations
}

#[test]
fn sg_17_18_no_minif2f_real_world_conflation_in_src() {
    let mut all_violations: Vec<(PathBuf, usize, String)> = Vec::new();
    for root in &[Path::new("src"), Path::new("experiments")] {
        for path in collect_rust_sources(root) {
            // Skip the test file itself.
            let p = path.to_string_lossy();
            if p.contains("tb_17_minif2f_scale_separation") {
                continue;
            }
            for (line_no, line) in forbidden_phrase_violations(&path) {
                all_violations.push((path.clone(), line_no, line));
            }
        }
    }
    if !all_violations.is_empty() {
        let mut msg = String::from(
            "SG-17.18 violation: source files conflate MiniF2F with real-world / P7 readiness:\n",
        );
        for (path, line_no, line) in &all_violations {
            msg.push_str(&format!("  {}:{}: {}\n", path.display(), line_no, line));
        }
        msg.push_str(
            "  (Allowed: phrase inside disclaimer comment lines containing \
             'forbidden' / 'must not' / 'anti-example' / 'disclaimer' / \
             'not real-world')\n",
        );
        panic!("{}", msg);
    }
}

#[test]
fn sg_17_18_minif2f_policy_doc_exists() {
    let path = Path::new("handover/whitepapers/REAL_WORLD_READINESS_REPORT.md");
    assert!(
        path.exists(),
        "REAL_WORLD_READINESS_REPORT.md must exist for SG-17.18 cite"
    );
    let body = std::fs::read_to_string(path).unwrap();
    let lower = body.to_lowercase();
    assert!(
        lower.contains("minif2f") || lower.contains("mini_f2f") || lower.contains("formal benchmark"),
        "report must cite MiniF2F or formal-benchmark distinction"
    );
}
