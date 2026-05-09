//! Constitution gate — `monetary_invariant.rs` strict-equality lint.
//!
//! Authority: `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md`
//! §1.B.3 (Phase E.3) + plan `cached-noodle.md` §C.E.3.
//!
//! Codex G2 audit (2026-05-09) flagged `assert_complete_set_balanced` for
//! using `min(sum_yes, sum_no) == collateral` to enforce the CTF invariant
//! "1 Coin = 1 YES + 1 NO". `min()` is functionally correct only because
//! the symmetric case `sum_yes == sum_no` reduces it to strict equality
//! and the asymmetric case is only reachable via post-resolution partial
//! redemption. With CPMM pool reserves entering the sums (Stage C P-M4+),
//! pre-resolution asymmetry becomes possible and `min()` would silently
//! admit ghost liquidity.
//!
//! This gate enforces: any `.min(` call near sum-aggregation operations in
//! `src/economy/monetary_invariant.rs` must carry a `CTF-MIN-SAFE` audit
//! marker (line-comment) explaining why the `min()` is safe in context.
//! Unmarked `.min()` calls fail the gate.
//!
//! The Phase E.3 source refactor split `assert_complete_set_balanced` into
//! a symmetric branch (strict `sum_yes == sum_no == collateral`) and an
//! asymmetric branch (`min` guarded by `sum_yes != sum_no`). The single
//! `.min(` call in the asymmetric branch is the only intentional reduction
//! and is marked `// CTF-MIN-SAFE: ...`.

use std::path::PathBuf;

const CONSERVATION_FILE: &str = "src/economy/monetary_invariant.rs";
const SAFE_MARKER: &str = "CTF-MIN-SAFE";
/// Identifiers whose `.min()` reductions are part of conservation logic.
/// Conservative list: any `.min(` on a line that mentions one of these
/// names is a candidate; the line (or the line immediately preceding) must
/// carry the `CTF-MIN-SAFE` marker to pass.
const SUM_IDENT_TRIGGERS: &[&str] = &["sum_yes", "sum_no", "sum_winning", "sum_losing"];

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_file(rel: &str) -> String {
    let path = workspace_root().join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("E.3 lint: failed to read {}: {}", path.display(), e))
}

fn is_doc_or_comment_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("///") || trimmed.starts_with("//!")
}

/// Find unguarded `.min(` calls on lines that mention sum-aggregate idents.
/// Returns (line_number_1_indexed, line_text) for each violation.
fn scan_unguarded_min_calls(content: &str) -> Vec<(usize, String)> {
    let lines: Vec<&str> = content.lines().collect();
    let mut violations = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if is_doc_or_comment_line(line) {
            continue;
        }
        if !line.contains(".min(") {
            continue;
        }
        // Heuristic: only flag .min() calls whose line mentions a sum-aggregate
        // identifier. This excludes incidental .min() uses in unrelated code.
        if !SUM_IDENT_TRIGGERS.iter().any(|t| line.contains(t)) {
            continue;
        }
        // Check current line OR the immediately preceding non-blank line for the marker.
        let mut marked = line.contains(SAFE_MARKER);
        if !marked {
            for j in (0..i).rev() {
                let prev = lines[j].trim();
                if prev.is_empty() {
                    continue;
                }
                marked = prev.contains(SAFE_MARKER);
                break;
            }
        }
        if !marked {
            violations.push((i + 1, line.to_string()));
        }
    }
    violations
}

#[test]
fn no_unguarded_min_in_conservation_invariants() {
    let content = read_file(CONSERVATION_FILE);
    let violations = scan_unguarded_min_calls(&content);
    assert!(
        violations.is_empty(),
        "Phase E.3 strict-equality lint failed in {}: {} unguarded .min() call(s) found.\n\
         Each .min() call near sum-aggregate identifiers ({:?}) must carry a `// {}: <reason>` \
         marker (same line or immediately preceding line).\n\
         Violations:\n{}",
        CONSERVATION_FILE,
        violations.len(),
        SUM_IDENT_TRIGGERS,
        SAFE_MARKER,
        violations
            .iter()
            .map(|(ln, l)| format!("  {}:{}  {}", CONSERVATION_FILE, ln, l.trim()))
            .collect::<Vec<_>>()
            .join("\n"),
    );
}

#[test]
fn lint_self_check_unmarked_min_fails() {
    // Synthetic source mimicking pre-fix `assert_complete_set_balanced` body:
    // `.min()` on sum-aggregates with NO marker → must be flagged.
    let synthetic = r#"
pub fn assert_complete_set_balanced_synthetic() {
    let sum_yes: u128 = 100;
    let sum_no: u128 = 50;
    let coll: u128 = 50;
    let min_side = sum_yes.min(sum_no);
    if min_side != coll {
        panic!("unbalanced");
    }
}
"#;
    let violations = scan_unguarded_min_calls(synthetic);
    assert!(
        !violations.is_empty(),
        "Phase E.3 self-check: synthetic unmarked .min() near sum_yes/sum_no MUST be flagged \
         by scan_unguarded_min_calls; got 0 violations. The lint is broken.",
    );
    let (_lineno, line) = &violations[0];
    assert!(
        line.contains("sum_yes.min(sum_no)"),
        "Phase E.3 self-check: violation should be on the sum_yes.min(sum_no) line; got: {}",
        line,
    );
}

#[test]
fn lint_self_check_marked_min_passes() {
    // Synthetic source with `// CTF-MIN-SAFE: ...` marker → must NOT be flagged.
    let synthetic = r#"
pub fn assert_balanced_with_marker() {
    let sum_yes: u128 = 100;
    let sum_no: u128 = 50;
    let coll: u128 = 50;
    // CTF-MIN-SAFE: guarded by sum_yes != sum_no above; smaller side == residual collateral.
    let min_side = sum_yes.min(sum_no);
    if min_side != coll {
        panic!("unbalanced");
    }
}
"#;
    let violations = scan_unguarded_min_calls(synthetic);
    assert!(
        violations.is_empty(),
        "Phase E.3 self-check: synthetic .min() with CTF-MIN-SAFE marker must NOT be flagged; \
         got {} violations: {:?}",
        violations.len(),
        violations,
    );
}

#[test]
fn lint_ignores_min_outside_conservation_context() {
    // `.min()` on unrelated identifiers (e.g. `len.min(cap)`) must not fire.
    let synthetic = r#"
pub fn unrelated() -> usize {
    let a: usize = 10;
    let b: usize = 20;
    a.min(b)
}
"#;
    let violations = scan_unguarded_min_calls(synthetic);
    assert!(
        violations.is_empty(),
        "Phase E.3 self-check: unrelated `.min()` (no sum_yes/sum_no/sum_winning idents \
         on the line) must NOT fire; got {} violations.",
        violations.len(),
    );
}
