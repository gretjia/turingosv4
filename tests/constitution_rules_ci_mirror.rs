//! K-3.1' CI redundancy mirror of `rules/active/R-*.yaml` (subset).
//!
//! ## Why "mirror" not "replacement"
//!
//! Adversarial review (A1) found that `rules/engine.py` is invoked
//! synchronously by `.claude/hooks/judge.sh` as a **PreToolUse** hook.
//! `judge.sh` exits 2 to block writes BEFORE they touch disk. `cargo test`
//! is post-write (after file is on disk) and takes 5-30s, so it cannot
//! replace `engine.py`'s sub-second pre-edit blocking.
//!
//! K-3.1' (this file) adds a **CI redundancy mirror**: for the subset of
//! YAML rules whose check is pure-grep on file content, we add a Rust
//! test that re-runs the grep at CI time. This is defense-in-depth:
//!
//! - `rules/engine.py` blocks edits at write-time (primary enforcement)
//! - `tests/constitution_rules_ci_mirror.rs` (this file) catches violations
//!   at CI time if engine.py was bypassed (e.g., direct git commit, hook
//!   disabled, etc.)
//!
//! Both layers must stay in sync. If a YAML rule changes, this file's
//! corresponding test must be updated. See `rules/active/R-*.yaml` for
//! the source-of-truth patterns.
//!
//! ## Non-mirrored rules (need engine state or non-content checks)
//!
//! The following YAML rules are NOT mirrored here because their check
//! requires runtime state beyond file content grep:
//!
//! - R-003 wal deletion (matches `rm` commands, not file content)
//! - R-006 kernel modification (matches any edit, needs change-event context)
//! - R-007 bus lifecycle (lifecycle-aware)
//! - R-009 payload limits (numeric config, needs parse)
//! - R-013 format contract (needs schema validation)
//! - R-014 trust root manifest drift (needs hash recomputation)
//! - R-018 constitution amendment sudo (needs sudo flag check)
//! - R-022 trace matrix pub symbol block (state-aware)
//!
//! For these, `rules/engine.py` remains the only enforcement layer.

use std::fs;
use std::path::Path;

/// Recursively collect all `.rs` files under given root, excluding target/.
fn collect_rs_files(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    collect_rs_recursive(root, &mut out);
    out
}

fn collect_rs_recursive(dir: &Path, out: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "target" || name == ".git" || name.starts_with(".") {
                continue;
            }
            if path.is_dir() {
                collect_rs_recursive(&path, out);
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                if let Some(s) = path.to_str() {
                    out.push(s.to_string());
                }
            }
        }
    }
}

/// R-001 kernel purity (block): src/kernel.rs must not contain domain terms.
#[test]
fn r001_kernel_purity_no_domain_terms() {
    let kernel = fs::read_to_string("src/kernel.rs").expect("src/kernel.rs must exist");
    // Pattern from rules/active/R-001_kernel_purity.yaml
    let forbidden = ["Lean", "theorem", "proof", "sorry", "simp", "lemma", "tactic", "induction", "[OMEGA]"];
    let mut found = Vec::new();
    for term in &forbidden {
        if kernel.contains(term) {
            found.push(*term);
        }
    }
    assert!(
        found.is_empty(),
        "R-001 kernel purity violation: src/kernel.rs contains domain terms {:?}. \
         Kernel must remain pure topology (Law 1).",
        found
    );
}

/// R-002 no coin minting (block): no .rs file may contain coin-minting fn names.
#[test]
fn r002_no_coin_minting_in_repo() {
    // Pattern from R-002: fund_agent|mint_coins|add_balance|new_coins|rebirth.*balance|print_money
    // Allow these in tests/ (mock harness), economy code's authorized impl,
    // but flag any new occurrence in src/ unrelated paths.
    let forbidden = ["mint_coins", "print_money"];
    let files = collect_rs_files(Path::new("src"));
    let mut violations = Vec::new();
    for f in &files {
        // Skip authorized economy/money path (where minting impl lives in on_init)
        if f.contains("economy/money.rs") || f.contains("state/sequencer.rs") || f.contains("state/q_state.rs") {
            continue;
        }
        let content = fs::read_to_string(f).unwrap_or_default();
        for term in &forbidden {
            if content.contains(term) {
                violations.push(format!("{} contains '{}'", f, term));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "R-002 no-coin-minting violation: {:?}. Only `on_init` may mint base coins (Economy Law 4).",
        violations
    );
}

/// R-005 forced investment (block): no .rs file may impose forced investment.
///
/// Allowlist: files that document the no-forced-investment guarantee using
/// `forced_live_investment` as a guard field name (their purpose IS to enforce
/// the rule). These files are read by engine.py as guards, not violations.
#[test]
fn r005_no_forced_investment_in_repo() {
    // Pattern from R-005: forced.*invest|mandatory.*stake
    let forbidden_pairs = [("forced", "invest"), ("mandatory", "stake")];
    // Files that ENFORCE the rule via guard fields (their content is supposed
    // to mention these terms by design):
    let allowlist = [
        "src/runtime/g7_structural_smoke.rs",
        "src/bin/audit_dashboard.rs",
    ];
    let files = collect_rs_files(Path::new("src"));
    let mut violations = Vec::new();
    for f in &files {
        if allowlist.iter().any(|a| f.contains(a)) {
            continue;
        }
        let content = fs::read_to_string(f).unwrap_or_default();
        for line in content.lines() {
            // Skip comments
            let trimmed = line.trim();
            if trimmed.starts_with("//") || trimmed.starts_with("///") || trimmed.starts_with("//!") {
                continue;
            }
            for (a, b) in &forbidden_pairs {
                if line.contains(a) && line.contains(b) {
                    violations.push(format!("{}: {}", f, line.trim()));
                }
            }
        }
    }
    assert!(
        violations.is_empty(),
        "R-005 forced-investment violation: {:?}. Economy Law: information is free, only investment costs money — but never forced.",
        violations
    );
}

/// R-015 trace_matrix_pub_symbol (warn): every newly-pub symbol in src/ ideally
/// has a TRACE_MATRIX FCx-Nyy backlink in its doc comment. This is a WARN rule
/// (informational), so this test just sanity-checks the matrix file exists.
#[test]
fn r015_trace_matrix_file_exists() {
    let matrix_path = "handover/alignment/TRACE_FLOWCHART_MATRIX.md";
    assert!(
        Path::new(matrix_path).exists(),
        "R-015 mirror: TRACE_FLOWCHART_MATRIX.md must exist to receive pub-symbol backlinks."
    );
}

/// R-018 constitution amendment sudo (block): constitution.md must not be
/// modified without explicit sudo marker. CI mirror only checks the file
/// exists and is non-empty (engine.py handles the actual sudo flow).
#[test]
fn r018_constitution_md_exists() {
    let content = fs::read_to_string("constitution.md")
        .expect("R-018 mirror: constitution.md must exist (engine.py enforces sudo for edits)");
    assert!(
        !content.is_empty(),
        "R-018 mirror: constitution.md must be non-empty"
    );
}

/// Sanity: all 15 YAML rules in rules/active/ are still present (deletion of
/// the source-of-truth YAML is itself a violation — engine.py wouldn't see it).
#[test]
fn all_15_active_yaml_rules_present() {
    let dir = "rules/active";
    let entries: Vec<_> = fs::read_dir(dir)
        .expect("rules/active must exist")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.starts_with("R-") && s.ends_with(".yaml"))
                .unwrap_or(false)
        })
        .collect();
    assert_eq!(
        entries.len(),
        15,
        "rules/active/ must contain exactly 15 R-*.yaml files (engine.py source-of-truth). Found: {}",
        entries.len()
    );
}

/// Sanity: engine.py and judge.sh still present (deletion would break primary
/// synchronous enforcement layer; K-3.1' is supplementary CI mirror only).
#[test]
fn engine_py_and_hook_still_present() {
    assert!(
        Path::new("rules/engine.py").exists(),
        "rules/engine.py must remain (PreToolUse synchronous block; K-3.1' is CI mirror only)"
    );
    assert!(
        Path::new(".claude/hooks/judge.sh").exists(),
        ".claude/hooks/judge.sh must remain (entry point for engine.py)"
    );
}
