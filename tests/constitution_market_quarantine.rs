//! TuringOS Constitution Gate — §5.2 legacy CPMM quarantine + no-f64
//! in market modules (architect 2026-05-07 ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_
//! POLYMARKET_MANUAL §5.2 verbatim).
//!
//! # Scope
//!
//! Architect §5.2 verbatim:
//!   "在任何新 market 代码前，必须确保：
//!    `src/prediction_market.rs` legacy f64 CPMM 不被新代码 import。"
//!
//!   测试：
//!     legacy_cpm_api_not_imported_by_new_market
//!     no_f64_in_market_modules
//!
//! # Constitution-gate surface (vs TB-13 SG-13.0)
//!
//! TB-13's `tb_13_legacy_cpmm_forward_fence.rs` enforces the same intent
//! at the TB-13 ship-gate surface using elaborate marker-discipline
//! (Layer 1 unconditional + Layer 2 marker-scoped + tb_13_scan_lines
//! span discovery). That complexity is appropriate for TB-13's
//! hand-off to TB-14+ — files acquire markers, scope grows.
//!
//! This file is the *constitution-gate* surface. Per
//! `feedback_no_workarounds_strict_constitution` ("我不要凑活"), it uses
//! a simpler, explicit allow-list of current market-substrate files and
//! scans line-by-line for hard-banned tokens. Stage C P-M0+ TBs MUST
//! extend `MARKET_SUBSTRATE_ALLOW_LIST` when they add new market
//! modules — the architect §5.2 expectation is that ANY new market
//! module is f64-free and legacy-import-free, not just TB-13's surfaces.
//!
//! Architect §5.2 verbatim names are mapped to test functions below.

use std::fs;
use std::path::Path;

/// Files in scope for §5.2 quarantine gate at Stage A3 SHIPPED FINAL.
///
/// **Update rule (Stage C P-M0+ TBs)**: every TB that adds a new
/// market-substrate module MUST extend this list in the same commit
/// that lands the module. Skipping this is a constitution-gate gap.
///
/// Stage A3 baseline: only TB-13 surfaces (CompleteSet + MarketSeed)
/// participate in the market-substrate. Polymarket P-M0+ atoms will add
/// `src/state/polymarket/*.rs`, `src/economy/cpmm/*.rs`, etc. — those
/// must be appended here as they land.
const MARKET_SUBSTRATE_ALLOW_LIST: &[&str] = &[
    "src/state/typed_tx.rs",
    "src/state/sequencer.rs",
    "src/state/q_state.rs",
    "src/economy/monetary_invariant.rs",
];

/// Hard-banned tokens that must NOT appear in any market-substrate file
/// (in non-comment lines). Architect §5.2 + §4.7 forbidden list, narrowed
/// 2026-05-09 (Stage C P-M4 + P-M5) to the truly-forbidden surface:
///
///   - `prediction_market::` — direct path-import of legacy f64 module
///   - `BinaryMarket` — legacy f64 CPMM type
///   - `bounty_market` / `bounty_lp_seed` / `bounty_yes_price` /
///     `resolve_bounty` / `open_bounty_market` — legacy bounty surface
///   - ` DPMM` — architect explicit forbidden list ("no DPMM / pro-rata
///     payout inside CTF track"; charter §6 + architect manual §6 / §8)
///   - `orderbook` — orderbook trading deferred indefinitely
///   - `RationalPrice` — legacy f64 price substitute type
///   - `.buy_yes(` / `.buy_no(` — legacy method-call shape (architect
///     §7.7 router uses different method names: `buy_yes_with_coin_router`
///     etc.)
///
/// **Tokens REMOVED from ban list at Stage C P-M4+P-M5 landing
/// (2026-05-09)**:
///   - ` CPMM`, ` AMM` — architect manual §7.5 / §7.6 explicitly defines
///     `CpmmPool` (integer math) + share-only swap (AMM-shaped). f64
///     defense is provided independently by `no_f64_in_market_modules` +
///     `swap_uses_integer_math_no_f64` tests.
///   - `PriceIndex`, `yes_price`, `no_price`, `price_yes`, `price_no` —
///     architect manual §7.8 defines `PriceIndex` as signal (not truth).
///     Price-as-truth defense is provided by
///     `tests/constitution_predicate_gate.rs::price_never_overrides_predicate`
///     + P-M7 verbatim `price_signal_not_predicate`.
const HARD_BANNED_LEGACY_TOKENS: &[&str] = &[
    "prediction_market::",
    "BinaryMarket",
    "open_bounty_market",
    "bounty_market",
    "bounty_lp_seed",
    "bounty_yes_price",
    "resolve_bounty",
    " DPMM",
    "orderbook",
    "RationalPrice",
    ".buy_yes(",
    ".buy_no(",
];

/// Tokens that catch f64 in money-path positions. Conservative pattern
/// set: catches type annotations (`: f64`), function signatures
/// (`f64,`, `f64;`, `f64)`), and standalone uses (` f64`, `\tf64`).
const F64_PATTERNS: &[&str] = &[
    " f64",
    "\tf64",
    "f64,",
    "f64;",
    "f64)",
    ": f64",
];

fn read_file(rel_path: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path);
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("§5.2 quarantine gate: cannot read {rel_path}: {e}"))
}

/// Returns true if `line` (after trimming leading whitespace) is a
/// pure comment line — `//`, `///`, `//!`, or block-comment continuation.
/// Doc-comments are allowed to mention forbidden tokens (e.g.,
/// "this module forbids prediction_market::") because they describe the
/// quarantine, not violate it.
fn is_comment_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("//")
        || trimmed.starts_with("/*")
        || trimmed.starts_with("*")
}

/// Returns true if `line` is inside a string literal that's clearly a
/// negative-list / quarantine-name fixture. Conservatively checks for
/// the line being entirely a `&str` literal or array-of-str entry.
/// Stage A3 baseline: no fixture exemptions in market-substrate files.
fn is_negative_list_fixture(_line: &str) -> bool {
    false
}

// ════════════════════════════════════════════════════════════════════════════
// §5.2 verbatim — `legacy_cpm_api_not_imported_by_new_market`
// ════════════════════════════════════════════════════════════════════════════

/// Architect §5.2 halting trigger: legacy CPMM (`src/prediction_market.rs`)
/// MUST NOT be imported by any new market-substrate code. Legacy file
/// is deleted (per `tb_13_legacy_cpmm_forward_fence::prediction_market_legacy_quarantined`)
/// — this gate enforces the absence at substrate-file scan level so
/// re-introducing it would fire from two independent surfaces.
#[test]
fn legacy_cpm_api_not_imported_by_new_market() {
    // Assertion 0: src/prediction_market.rs MUST NOT exist.
    let pm_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/prediction_market.rs");
    assert!(
        !pm_path.exists(),
        "§5.2 quarantine: src/prediction_market.rs MUST NOT exist \
         (legacy f64 CPMM excised in TB-14 Atom 6). Reintroducing it \
         would resurrect f64 trading semantics + automatic CPMM \
         liquidity that architect 2026-05-03 + 2026-05-07 directives \
         forbid."
    );

    // Assertion 1: src/lib.rs does not declare the legacy module.
    let lib = read_file("src/lib.rs");
    for forbidden in ["pub mod prediction_market", "mod prediction_market"] {
        assert!(
            !lib.contains(forbidden),
            "§5.2 quarantine: src/lib.rs MUST NOT contain `{forbidden}`"
        );
    }

    // Assertion 2: scan market-substrate allow-list for hard-banned tokens.
    let mut violations: Vec<String> = Vec::new();
    for rel in MARKET_SUBSTRATE_ALLOW_LIST {
        let source = read_file(rel);
        for (line_no, line) in source.lines().enumerate() {
            if is_comment_line(line) || is_negative_list_fixture(line) {
                continue;
            }
            for token in HARD_BANNED_LEGACY_TOKENS {
                if line.contains(token) {
                    violations.push(format!(
                        "{rel}:{}: market-substrate contains hard-banned \
                         token `{token}` — {}",
                        line_no + 1,
                        line.trim()
                    ));
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "§5.2 verbatim `legacy_cpm_api_not_imported_by_new_market` violated \
         in {} files:\n{}\n\nUpdate rule: any new market module Stage C \
         P-M0+ adds MUST be appended to MARKET_SUBSTRATE_ALLOW_LIST and \
         remain free of HARD_BANNED_LEGACY_TOKENS.",
        MARKET_SUBSTRATE_ALLOW_LIST.len(),
        violations.join("\n")
    );
}

// ════════════════════════════════════════════════════════════════════════════
// §5.2 verbatim — `no_f64_in_market_modules`
// ════════════════════════════════════════════════════════════════════════════

/// Architect §5.2 halting trigger: NO f64 in market-substrate code.
/// All money-path arithmetic MUST use integer types (`MicroCoin` /
/// `ShareAmount` / `LpShareAmount`). f64 in market modules is the
/// classical CPMM bug-class architect §5.2 explicitly forbids ("integer /
/// rational math only" per §4.5).
#[test]
fn no_f64_in_market_modules() {
    let mut violations: Vec<String> = Vec::new();
    for rel in MARKET_SUBSTRATE_ALLOW_LIST {
        let source = read_file(rel);
        for (line_no, line) in source.lines().enumerate() {
            if is_comment_line(line) || is_negative_list_fixture(line) {
                continue;
            }
            for pattern in F64_PATTERNS {
                if line.contains(pattern) {
                    violations.push(format!(
                        "{rel}:{}: market-substrate contains f64 token \
                         `{pattern}` — {}",
                        line_no + 1,
                        line.trim()
                    ));
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "§5.2 verbatim `no_f64_in_market_modules` violated in {} files:\n{}\n\
         \nMoney-path types MUST be integer (MicroCoin / ShareAmount).",
        MARKET_SUBSTRATE_ALLOW_LIST.len(),
        violations.join("\n")
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Self-tests: ensure the gate scanner can detect violations on
// synthetic input (closure-3 "every test can fail" per
// `tests/constitution_closure_3_no_trivial_asserts.rs`).
// ════════════════════════════════════════════════════════════════════════════

/// Self-test: confirm `is_comment_line` recognizes the standard
/// comment forms. If this regresses, doc-comment exemption could
/// silently swallow real code lines.
#[test]
fn self_test_comment_recognition() {
    assert!(is_comment_line("// regular line comment"));
    assert!(is_comment_line("    //! module doc"));
    assert!(is_comment_line("\t/// item doc"));
    assert!(is_comment_line("    /* block start"));
    assert!(is_comment_line("     * block continuation"));
    assert!(!is_comment_line("let x: f64 = 1.0;"));
    assert!(!is_comment_line("use crate::prediction_market::Foo;"));
    assert!(!is_comment_line(""));
}

/// Self-test: confirm token-scan detects HARD_BANNED_LEGACY_TOKENS in
/// a synthetic non-comment line. Without this, `legacy_cpm_api_not_imported_by_new_market`
/// could pass vacuously if the comment-skip logic accidentally always
/// returned true.
#[test]
fn self_test_token_scan_detects_violation() {
    let synthetic = "use crate::prediction_market::BinaryMarket;";
    assert!(!is_comment_line(synthetic), "synthetic line must not be comment-skipped");
    let mut found = false;
    for token in HARD_BANNED_LEGACY_TOKENS {
        if synthetic.contains(token) {
            found = true;
            break;
        }
    }
    assert!(
        found,
        "scanner failed to detect hard-banned token in synthetic violation \
         — main test would pass vacuously"
    );
}

/// Self-test: confirm f64 token-scan detects `let x: f64 = 1.0;`.
#[test]
fn self_test_f64_scan_detects_violation() {
    let synthetic = "let x: f64 = 1.0;";
    assert!(!is_comment_line(synthetic));
    let mut found = false;
    for pattern in F64_PATTERNS {
        if synthetic.contains(pattern) {
            found = true;
            break;
        }
    }
    assert!(found, "f64 scanner failed on synthetic `: f64` declaration");
}
