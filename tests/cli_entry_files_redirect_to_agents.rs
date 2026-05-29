//! Phase 3 (harness platform-agnostic unification, 2026-05-29) — cross-CLI
//! contract-conformance gate.
//!
//! Enforces the **two-layer model** stated in `AGENTS.md`:
//!   - **Layer 1 — universal contract:** the governance rules every platform
//!     obeys identically live ONLY in `AGENTS.md` (the sole home of a rule).
//!   - **Layer 2 — platform-native enhancement:** each thin CLI entry file
//!     (`GEMINI.md`, `.cursorrules`, …) is a *pointer* to that contract plus
//!     optional platform-native config. Layer 2 may ADD; it may NOT
//!     restate-and-narrow a Layer-1 rule.
//!
//! Two checks per thin file:
//!   1. **Canonical-redirect sentinel** — the file names `AGENTS.md` and states
//!      it is canonical / wins on conflict (so a reader knows where truth lives).
//!   2. **Restricted-surface drift deny-list** — the file must not name any
//!      `src/…`- or `rules/…`-shaped path token that is NOT one of `AGENTS.md`
//!      §6's bulleted restricted surfaces. Listing an invented surface (e.g.
//!      `rules/engine.py`, absent from §6) reads as "only these are restricted":
//!      a Layer-1 *narrowing/contradiction*, not a Layer-2 addition. The fix is
//!      to point at §6, never to re-list it.
//!
//! This complements (does not duplicate) the K-HARDEN-8 tests in
//! `constitution_subagent_pr_hygiene.rs`, which assert each file exists, names
//! `AGENTS.md`, and carries a PR-only marker.
//!
//! Per `AGENTS.md` §7 ("gate tests must be able to fail"), two synthetic
//! controls below prove the deny-list flags real drift (`rules/engine.py`) yet
//! passes a benign platform-native glob (`**/*.rs`).

use std::collections::BTreeSet;
use std::fs;

/// Thin CLI entry files (Layer 2). `.aider.conf.yml` is pure YAML config with
/// no prose sentinel — special-cased in the sentinel check below.
const THIN_FILES: &[&str] = &[
    "GEMINI.md",
    "CONVENTIONS.md",
    ".aider.conf.yml",
    ".cursorrules",
    ".cursor/rules/000-agents-alignment.mdc",
    ".windsurfrules",
    ".github/copilot-instructions.md",
    "WARP.md",
];

/// Parse `AGENTS.md` §6 "Restricted Surfaces" and return the set of
/// backtick-wrapped `src/…` paths it declares. These are the only `src/`-shaped
/// path tokens a thin file may legitimately name.
fn agents_md_restricted_src_paths() -> BTreeSet<String> {
    let agents = fs::read_to_string("AGENTS.md").expect("AGENTS.md readable");
    let section = slice_section(&agents, "## 6. Restricted Surfaces");
    let allowed: BTreeSet<String> = backtick_tokens(&section)
        .into_iter()
        .filter(|t| t.starts_with("src/"))
        .collect();
    assert!(
        allowed.len() >= 6,
        "expected >=6 src/ restricted surfaces parsed from AGENTS.md §6, got {}: {:?}",
        allowed.len(),
        allowed
    );
    allowed
}

/// Return the body of a markdown section: the lines after the `## ` header line
/// equal to `start_header`, up to (but not including) the next `## ` header.
fn slice_section(text: &str, start_header: &str) -> String {
    let mut out = String::new();
    let mut in_section = false;
    for line in text.lines() {
        if line.trim_end() == start_header {
            in_section = true;
            continue;
        }
        if in_section {
            // "## " starts a new top-level section; "### " (space after ##?) does
            // not — "### ".starts_with("## ") is false, so subsections are kept.
            if line.starts_with("## ") {
                break;
            }
            out.push_str(line);
            out.push('\n');
        }
    }
    assert!(in_section, "section header not found in AGENTS.md: {}", start_header);
    out
}

/// Extract backtick-delimited inline-code tokens from markdown text.
fn backtick_tokens(text: &str) -> Vec<String> {
    let mut toks = Vec::new();
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '`' {
            let mut t = String::new();
            for c2 in chars.by_ref() {
                if c2 == '`' {
                    break;
                }
                t.push(c2);
            }
            if !t.is_empty() {
                toks.push(t);
            }
        }
    }
    toks
}

/// Extract maximal path-like tokens (runs of path chars that contain at least
/// one `/`) from arbitrary text — finds surface references regardless of how
/// they are delimited (backticks, prose, list bullets). Trailing dots (prose
/// sentence periods) are trimmed so `src/kernel.rs.` matches `src/kernel.rs`.
fn path_like_tokens(text: &str) -> Vec<String> {
    let is_path_char =
        |c: char| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '/' | '*' | '-');
    let mut toks = Vec::new();
    let mut cur = String::new();
    for c in text.chars() {
        if is_path_char(c) {
            cur.push(c);
        } else {
            if cur.contains('/') {
                let trimmed = cur.trim_end_matches('.').to_string();
                if !trimmed.is_empty() {
                    toks.push(trimmed);
                }
            }
            cur.clear();
        }
    }
    if cur.contains('/') {
        let trimmed = cur.trim_end_matches('.').to_string();
        if !trimmed.is_empty() {
            toks.push(trimmed);
        }
    }
    toks
}

/// The deny-list core: path tokens that are `src/`- or `rules/`-shaped but are
/// NOT in the AGENTS.md §6 allow-set. Pure (operates on a string) so it can be
/// exercised by both the file tests and the synthetic controls.
fn surface_violations(content: &str, allowed: &BTreeSet<String>) -> Vec<String> {
    path_like_tokens(content)
        .into_iter()
        .filter(|t| (t.starts_with("src/") || t.starts_with("rules/")) && !allowed.contains(t))
        .collect()
}

#[test]
fn thin_files_carry_canonical_redirect_sentinel() {
    for path in THIN_FILES {
        let content = fs::read_to_string(path).unwrap_or_else(|_| panic!("readable: {}", path));
        assert!(
            content.contains("AGENTS.md"),
            "{}: must reference AGENTS.md as the canonical contract (Layer-1 redirect)",
            path
        );
        if path.ends_with(".yml") {
            // Pure config: AGENTS.md in the read-list IS the redirect; no prose.
            continue;
        }
        assert!(
            content.contains("wins") || content.contains("canonical"),
            "{}: must state AGENTS.md is canonical / wins on conflict (redirect sentinel)",
            path
        );
    }
}

#[test]
fn thin_files_do_not_narrow_restricted_surfaces() {
    let allowed = agents_md_restricted_src_paths();
    let mut violations: Vec<String> = Vec::new();
    for path in THIN_FILES {
        let content = fs::read_to_string(path).unwrap_or_else(|_| panic!("readable: {}", path));
        for tok in surface_violations(&content, &allowed) {
            violations.push(format!(
                "{}: '{}' is not an AGENTS.md §6 restricted surface — Layer 2 must point to §6, \
                 never restate/narrow it",
                path, tok
            ));
        }
    }
    assert!(
        violations.is_empty(),
        "thin CLI files name restricted surfaces absent from AGENTS.md §6:\n{}",
        violations.join("\n")
    );
}

// ── Synthetic controls (gate must be able to fail, and must not over-fire) ──

#[test]
fn denylist_flags_invented_rules_surface() {
    let allowed = agents_md_restricted_src_paths();
    let drift = "- Restricted surfaces: `rules/engine.py`, `rules/active/*.yaml` need §8.\n";
    let mut v = surface_violations(drift, &allowed);
    v.sort();
    assert_eq!(
        v,
        vec!["rules/active/*.yaml".to_string(), "rules/engine.py".to_string()],
        "deny-list must flag invented rules/ surfaces (proves the gate can fail)"
    );
}

#[test]
fn denylist_passes_benign_platform_native_glob() {
    let allowed = agents_md_restricted_src_paths();
    // A Cursor-style glob and a §6 pointer: neither names a non-§6 src//rules/
    // surface, so a platform-native addition is never punished.
    let benign = "globs: [\"**/*.rs\"]\nRestricted surfaces: see `AGENTS.md` §6.\n";
    assert!(
        surface_violations(benign, &allowed).is_empty(),
        "benign platform-native glob must not be flagged"
    );
}

#[test]
fn denylist_allows_real_six_surfaces() {
    // A thin file is permitted to cite a real §6 surface verbatim (membership,
    // not mere shape, is what the allow-set checks).
    let allowed = agents_md_restricted_src_paths();
    let ok = "Touching `src/kernel.rs` or `src/state/sequencer.rs` needs §8.\n";
    assert!(
        surface_violations(ok, &allowed).is_empty(),
        "real §6 surfaces must pass the deny-list"
    );
}
