//! TB-13 Atom 0.5 — Legacy CPMM forward-fence + label ship-gate.
//!
//! TRACE_MATRIX TB-13 Atom 0.5 (architect 2026-05-03 ruling Part A §4.2;
//! SG-13.0.1 / SG-13.0.2 / SG-13.0.3).
//!
//! These three tests enforce the forward-binding fence that NEW TB-13
//! modules cannot import or reuse the legacy `src/prediction_market.rs`
//! f64 CPMM scaffolding. Architect §4.2 halting triggers:
//!
//!   HALT if new TB-13 code imports legacy prediction_market.rs.
//!   HALT if f64 appears in new CompleteSet / MarketSeed code.
//!   HALT if any AMM / CPMM router function is introduced in TB-13.
//!
//! ## What is "TB-13 code"?
//!
//! A span of Rust source belongs to TB-13 iff it is a contiguous block
//! of non-blank lines whose first non-blank line contains an authoring
//! marker that identifies TB-13 as the contributing tracer-bullet (NOT
//! a forward-reference from an earlier-TB doc-comment to TB-13's future
//! work). Authoring markers:
//!
//!   - `TRACE_MATRIX TB-13 ` (TB-12 convention used by every shipped TB).
//!   - A line that begins with `// TB-13 ` after stripping leading
//!     whitespace + comment markers.
//!   - A line that begins with `//! TB-13 ` (module-level doc).
//!   - A line that begins with `/// TB-13 ` (item-level doc).
//!
//! A span ends at the next blank line OR end-of-file. Cross-references
//! to TB-13 from inside a TB-12 (or earlier) span do NOT pull that span
//! into TB-13 scope — only the *first non-blank line* of a span is
//! checked for the authoring marker.
//!
//! ## File set in scope
//!
//! - `src/state/typed_tx.rs` — TB-13 typed-tx variant additions (Atom 1).
//! - `src/state/q_state.rs` — TB-13 EconomicState extensions (Atom 2).
//! - `src/state/sequencer.rs` — TB-13 dispatch-arm additions (Atom 2).
//! - `src/economy/monetary_invariant.rs` — TB-13 conservation extensions (Atom 3).
//! - `src/bin/audit_dashboard.rs` — TB-13 §14 dashboard rendering (Atom 4).
//!
//! At Atom 0.5 ship time, none of these files contain `TB-13` markers
//! (TB-12 is the latest contributor). The fence passes trivially. As
//! Atom 1..4 land, markers appear and the fence enforces the rule.

use std::fs;
use std::path::PathBuf;

/// In-scope source files for the TB-13 forward-fence. NEW TB-13 markers
/// appearing in any of these files are subject to the forbidden-token
/// rules below.
const FENCE_SCOPE: &[&str] = &[
    "src/state/typed_tx.rs",
    "src/state/q_state.rs",
    "src/state/sequencer.rs",
    "src/economy/monetary_invariant.rs",
    "src/bin/audit_dashboard.rs",
];

/// Tokens forbidden inside any TB-13-marker span (architect §4.2 halting
/// triggers + §4.7 forbidden list).
///
/// Each entry is a literal substring that must NOT appear in TB-13 code.
const FORBIDDEN_LEGACY_TOKENS: &[&str] = &[
    // Direct legacy CPMM imports / type names.
    "prediction_market::",
    "BinaryMarket",
    // Legacy CPMM API method names.
    ".buy_yes(",
    ".buy_no(",
    "open_bounty_market",
    "bounty_market",
    "bounty_lp_seed",
    "bounty_yes_price",
    "resolve_bounty",
    "market_ticker(",
    "market_ticker_full(",
    // f64 in money-path context (see SG-13.0.2 dedicated test for the
    // primary check; this entry catches `f64` in any TB-13-marked span).
    " f64",
    "f64,",
    "f64;",
    "f64)",
    // Trading / AMM / orderbook concepts forbidden in TB-13 (per §4.7).
    "MarketOrderTx",
    "MarketTradeTx",
    "MarketBuyTx",
    "MarketSellTx",
    "AMM",
    "CPMM",
    "DPMM",
    "orderbook",
    // Price-as-truth concepts (deferred to TB-14 per §5).
    "price_yes",
    "price_no",
    "PriceIndex",
    "yes_price",
    "no_price",
    "RationalPrice",
];

fn workspace_root() -> PathBuf {
    let manifest = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest)
}

/// Returns true if `line` is an authoring marker for TB-13 (i.e., the
/// line declares that the following block is TB-13 code, NOT a forward-
/// reference from an earlier-TB doc-comment to TB-13's future work).
fn is_tb_13_authoring_marker(line: &str) -> bool {
    if line.contains("TRACE_MATRIX TB-13 ") {
        return true;
    }
    let trimmed = line.trim_start();
    let body = trimmed
        .strip_prefix("//! ")
        .or_else(|| trimmed.strip_prefix("/// "))
        .or_else(|| trimmed.strip_prefix("// "))
        .unwrap_or("");
    body.starts_with("TB-13 ")
}

/// Extract line ranges that belong to TB-13 additions. A span is a
/// contiguous block of non-blank lines; it is in-scope iff the first
/// non-blank line is an authoring marker per `is_tb_13_authoring_marker`.
fn tb_13_spans(source: &str) -> Vec<(usize, String)> {
    let mut out: Vec<(usize, String)> = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    let n = lines.len();
    let mut i = 0;
    while i < n {
        if lines[i].trim().is_empty() {
            i += 1;
            continue;
        }
        let span_start = i;
        let mut span_end = i;
        while span_end < n && !lines[span_end].trim().is_empty() {
            span_end += 1;
        }
        let span = &lines[span_start..span_end];
        if is_tb_13_authoring_marker(span[0]) {
            for (offset, line) in span.iter().enumerate() {
                out.push((span_start + offset + 1, (*line).to_string()));
            }
        }
        i = span_end;
    }
    out
}

/// Read a source file relative to the workspace root, returning its
/// content as a String. Panics with a clear message if missing — Atom 0.5
/// ship requires every file in `FENCE_SCOPE` to exist.
fn read_scope_file(rel_path: &str) -> String {
    let full = workspace_root().join(rel_path);
    fs::read_to_string(&full)
        .unwrap_or_else(|e| panic!("TB-13 fence: failed to read {rel_path}: {e}"))
}

/// SG-13.0.1 — `legacy_cpm_api_not_imported_by_complete_set`.
///
/// Architect §4.2 halting trigger: HALT if NEW TB-13 code imports legacy
/// `prediction_market.rs`. We extend the check to all legacy CPMM API
/// names and all forbidden trading/AMM concepts.
#[test]
fn legacy_cpm_api_not_imported_by_complete_set() {
    let mut violations: Vec<String> = Vec::new();
    for rel in FENCE_SCOPE {
        let source = read_scope_file(rel);
        for (line_no, line) in tb_13_spans(&source) {
            for token in FORBIDDEN_LEGACY_TOKENS {
                // The `f64` family entries are checked in SG-13.0.2 — skip
                // them here so the failure message is unambiguous.
                if token.starts_with(" f64")
                    || token.starts_with("f64,")
                    || token.starts_with("f64;")
                    || token.starts_with("f64)")
                {
                    continue;
                }
                if line.contains(token) {
                    violations.push(format!(
                        "{rel}:{line_no}: TB-13-marked span contains forbidden token `{token}` — {line}"
                    ));
                }
            }
        }
    }
    assert!(
        violations.is_empty(),
        "TB-13 SG-13.0.1 forward-fence violated:\n{}",
        violations.join("\n")
    );
}

/// SG-13.0.2 — `no_f64_in_complete_set_or_market_seed`.
///
/// Architect §4.2 halting trigger: HALT if `f64` appears in NEW
/// CompleteSet / MarketSeed code. Money-path types must use integer
/// `MicroCoin` / `ShareAmount`.
#[test]
fn no_f64_in_complete_set_or_market_seed() {
    let mut violations: Vec<String> = Vec::new();
    let f64_tokens = [" f64", "f64,", "f64;", "f64)"];
    for rel in FENCE_SCOPE {
        let source = read_scope_file(rel);
        for (line_no, line) in tb_13_spans(&source) {
            for token in &f64_tokens {
                if line.contains(token) {
                    violations.push(format!(
                        "{rel}:{line_no}: TB-13-marked span contains f64 (`{token}`) — {line}"
                    ));
                }
            }
        }
    }
    assert!(
        violations.is_empty(),
        "TB-13 SG-13.0.2 no-f64-in-money-path violated:\n{}",
        violations.join("\n")
    );
}

/// SG-13.0.3 — `prediction_market_legacy_quarantined`.
///
/// Architect §4.2 ship gate: legacy CPMM "must be clearly labeled". We
/// enforce that `src/prediction_market.rs` carries the LEGACY module-
/// header doc-comment with the four required tokens (`legacy`,
/// `not constitutional`, `not RSP-M`, `not production market path`)
/// AND that `src/kernel.rs` market-bearing fields carry the `LEGACY`
/// label tying them to the migration path.
#[test]
fn prediction_market_legacy_quarantined() {
    let pm = read_scope_file("src/prediction_market.rs");
    let header = pm
        .lines()
        .take(60)
        .collect::<Vec<_>>()
        .join("\n");

    let required_label_tokens = [
        "LEGACY",
        "not constitutional",
        "not RSP-M",
        "not production market path",
    ];
    for token in &required_label_tokens {
        assert!(
            header.contains(token),
            "TB-13 SG-13.0.3: src/prediction_market.rs module header missing required \
             label token `{token}`. Header:\n{header}"
        );
    }

    // Architect §4.2 also requires the doc to name the migration path so
    // future maintainers don't reintroduce the legacy API.
    let migration_tokens = [
        "TB-13",
        "TB-14",
        "CompleteSetMintTx",
        "OBS_TB_12_LEGACY_CPMM_QUARANTINE",
    ];
    for token in &migration_tokens {
        assert!(
            header.contains(token),
            "TB-13 SG-13.0.3: src/prediction_market.rs module header missing migration-path \
             token `{token}`. Header:\n{header}"
        );
    }

    // Defense-in-depth: kernel.rs market-bearing fields carry LEGACY.
    let kernel = read_scope_file("src/kernel.rs");
    let kernel_struct_idx = kernel
        .find("pub struct Kernel {")
        .expect("TB-13 SG-13.0.3: cannot locate `pub struct Kernel {` in src/kernel.rs");
    let kernel_struct_end = kernel[kernel_struct_idx..]
        .find("\n}\n")
        .map(|off| kernel_struct_idx + off + 2)
        .expect("TB-13 SG-13.0.3: cannot locate end of Kernel struct");
    let kernel_struct_block = &kernel[kernel_struct_idx..kernel_struct_end];

    for field in ["markets", "bounty_market", "bounty_lp_seed"] {
        let field_marker = format!("pub {field}");
        let field_idx = kernel_struct_block
            .find(&field_marker)
            .unwrap_or_else(|| panic!("TB-13 SG-13.0.3: cannot locate field `{field}` in Kernel struct"));
        // Look for `LEGACY` in the 600 chars preceding the field
        // declaration (covers a multi-line doc-comment block).
        let doc_window_start = field_idx.saturating_sub(600);
        let doc_window = &kernel_struct_block[doc_window_start..field_idx];
        assert!(
            doc_window.contains("LEGACY"),
            "TB-13 SG-13.0.3: Kernel.{field} missing LEGACY doc-comment label. \
             Doc window:\n{doc_window}"
        );
    }
}
