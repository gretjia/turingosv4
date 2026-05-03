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

/// Statically-listed in-scope source files for the TB-13 forward-fence
/// Layer 1 (unconditional whole-file scan for hard-banned imports).
/// Codex round-2 CHALLENGE remediation 2026-05-03: this list is now
/// a *floor* — `discover_tb_13_files()` walks `src/` for any additional
/// file containing a TB-13 authoring marker and adds it to the
/// effective scope.
///
/// **Codex round-5 (R5) DASHBOARD-FLOOR remediation 2026-05-03**:
/// `src/bin/audit_dashboard.rs` was briefly removed from this list in
/// round-6 to dodge a Layer 2 false-positive on its negative-list test
/// fixture (string literals "price_yes" / "price_no" at line 1628-1629).
/// Codex R5 correctly pointed out that removing it from FLOOR also
/// removed it from Layer 1 hard-import scanning — but the false-positive
/// is Layer 2-specific. The right fix is two-tier scope: keep
/// `audit_dashboard.rs` in `FENCE_SCOPE_FLOOR` (Layer 1 always scans for
/// hard-banned imports), but exclude it from Layer 2's
/// effective-discovered scope until it gains TB-13 markers or type uses.
const FENCE_SCOPE_FLOOR: &[&str] = &[
    "src/state/typed_tx.rs",
    "src/state/q_state.rs",
    "src/state/sequencer.rs",
    "src/economy/monetary_invariant.rs",
    "src/bin/audit_dashboard.rs",
    "src/runtime/verify.rs",
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

/// Lines to scan for Layer 2 forbidden tokens.
///
/// **Codex round-4 RQ6 (2026-05-03)**: `tb_13_spans()` returns nothing
/// for files added to scope by `discover_by_type_use` (no marker = no
/// span), so the marker-only Layer 2 missed unmarked TB-13 contributors.
///
/// **Codex round-5 (R5) PARTIAL-MARKER (2026-05-03)**: round-6's
/// either/or rule (marker-file → spans-only; unmarked file → all
/// non-comment lines) left a hole: a marker-bearing file could hide
/// non-marker TB-13 type-use plus f64/AMM tokens outside any marker
/// span. Fix: for marker-files, scan marker-spans UNION any non-comment
/// line that contains a TB-13 type name (catches stealth TB-13 type-uses
/// outside marker spans — those lines ARE TB-13 contributions by
/// definition because they reference TB-13-introduced types).
///
/// Final rules:
/// - Marker-file: marker-spans ∪ non-comment lines containing TB-13 type names.
/// - Unmarked-discovered file: all non-comment lines (round-6 behavior).
///
/// Residual gap (acknowledged): a TB-13 helper that uses zero TB-13 type
/// names AND lives outside marker spans (e.g., a generic math helper
/// called only by TB-13 code). Without a code-marker AND without a
/// type-name signal, the fence has no way to identify it as TB-13. This
/// is a defense-in-depth limit of marker+type-name discipline; manual
/// code review remains the residual halt-trigger guard.
fn tb_13_scan_lines(source: &str) -> Vec<(usize, String)> {
    use std::collections::BTreeMap;
    let has_marker = source.lines().any(is_tb_13_authoring_marker);
    if has_marker {
        // Marker-file: marker-spans ∪ non-comment lines with TB-13 type names.
        let mut acc: BTreeMap<usize, String> = BTreeMap::new();
        for (n, l) in tb_13_spans(source) {
            acc.insert(n, l);
        }
        for (i, line) in source.lines().enumerate() {
            if is_pure_comment_line(line) {
                continue;
            }
            if TB_13_TYPE_NAMES.iter().any(|t| line.contains(t)) {
                acc.insert(i + 1, line.to_string());
            }
        }
        return acc.into_iter().collect();
    }
    // Unmarked-discovered file: all non-comment lines.
    source
        .lines()
        .enumerate()
        .filter(|(_, line)| !is_pure_comment_line(line))
        .map(|(i, line)| (i + 1, line.to_string()))
        .collect()
}

/// Read a source file relative to the workspace root, returning its
/// content as a String. Panics with a clear message if missing — fence
/// requires every file in `FENCE_SCOPE_FLOOR` to exist.
fn read_scope_file(rel_path: &str) -> String {
    let full = workspace_root().join(rel_path);
    fs::read_to_string(&full)
        .unwrap_or_else(|e| panic!("TB-13 fence: failed to read {rel_path}: {e}"))
}

/// TB-13 type names — any non-comment use of one of these in `src/`
/// marks the file as a TB-13 contributor. Codex round-3 RQ6 remediation
/// 2026-05-03: the round-2 marker-only discovery caught files whose
/// authors followed the `TRACE_MATRIX TB-13 ` / `// TB-13 ` convention,
/// but a contributor could `use crate::state::typed_tx::CompleteSetMintTx;`
/// in a fresh file without adding a marker — and the fence would miss
/// it. The type-use walk closes that gap by checking for the distinctive
/// TB-13-introduced symbol set itself; the marker discipline becomes a
/// hint for human readers, not the only line of defense.
///
/// All names below are TB-13-introduced and have no pre-existing
/// occurrence in `src/` outside `FENCE_SCOPE_FLOOR`.
const TB_13_TYPE_NAMES: &[&str] = &[
    "CompleteSetMintTx",
    "CompleteSetRedeemTx",
    "MarketSeedTx",
    "ConditionalCollateralIndex",
    "ConditionalShareBalances",
    "ShareSidePair",
    "EventNotOpen",
    "EventId",
    "OutcomeSide",
    "ShareAmount",
];

/// True iff `line` is a pure comment (line starts with `//`, `///`, or
/// `//!` after leading whitespace). Used by type-use discovery to skip
/// legacy doc-comment cross-references (e.g., TB-12 `kernel.rs` /// doc
/// strings that mention `CompleteSetMintTx` as future work).
fn is_pure_comment_line(line: &str) -> bool {
    line.trim_start().starts_with("//")
}

/// Walk `walk_root` for any `.rs` file containing a TB-13 authoring
/// marker on at least one line. Codex round-2 CHALLENGE remediation
/// 2026-05-03: makes the fence self-extending against marked but
/// not-yet-listed contributors.
fn discover_by_marker(walk_root: &std::path::Path) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    walk_rs_files(walk_root, &mut |path| {
        let rel = path
            .strip_prefix(workspace_root())
            .unwrap_or(path)
            .to_string_lossy()
            .into_owned();
        let body = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => return,
        };
        if body.lines().any(is_tb_13_authoring_marker) {
            found.push(rel);
        }
    });
    found
}

/// Walk `walk_root` for any `.rs` file that USES a TB-13 type name
/// (`TB_13_TYPE_NAMES`) on a non-comment line. Codex round-3 RQ6
/// remediation 2026-05-03: catches contributors who imported TB-13
/// types without remembering the authoring-marker convention.
fn discover_by_type_use(walk_root: &std::path::Path) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    walk_rs_files(walk_root, &mut |path| {
        let rel = path
            .strip_prefix(workspace_root())
            .unwrap_or(path)
            .to_string_lossy()
            .into_owned();
        let body = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => return,
        };
        for line in body.lines() {
            if is_pure_comment_line(line) {
                continue;
            }
            if TB_13_TYPE_NAMES.iter().any(|t| line.contains(t)) {
                found.push(rel);
                return;
            }
        }
    });
    found
}

/// Discover every TB-13-contributing file in `src/`. Union of
/// marker-walk (round-2) + type-use-walk (round-3 RQ6). Either path
/// alone would leave a loophole; together they enforce the fence even
/// when the human-followed marker convention slips.
fn discover_tb_13_files() -> Vec<String> {
    let src_root = workspace_root().join("src");
    let mut set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for f in discover_by_marker(&src_root) {
        set.insert(f);
    }
    for f in discover_by_type_use(&src_root) {
        set.insert(f);
    }
    set.into_iter().collect()
}

fn walk_rs_files(dir: &std::path::Path, visitor: &mut dyn FnMut(&std::path::Path)) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_rs_files(&path, visitor);
        } else if path.extension().map(|e| e == "rs").unwrap_or(false) {
            visitor(&path);
        }
    }
}

/// Layer 1 fence scope = FLOOR ∪ discovered. Deduplicated, sorted.
/// Used by `legacy_cpm_api_not_imported_by_complete_set` Layer 1
/// (HARD_BANNED_LEGACY_IMPORTS unconditional whole-file scan).
///
/// Layer 1 is broader than Layer 2 because legacy imports are forbidden
/// EVERYWHERE in TB-13-relevant scope, regardless of whether the file
/// carries TB-13 markers or type uses today. `audit_dashboard.rs` lives
/// here because it is TB-13-relevant scope (Atom 4 §13/§14 dashboard
/// renders TB-13 state), even though its current contributions are TB-12.
fn effective_fence_scope() -> Vec<String> {
    let mut set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for s in FENCE_SCOPE_FLOOR {
        set.insert((*s).to_string());
    }
    for s in discover_tb_13_files() {
        set.insert(s);
    }
    set.into_iter().collect()
}

/// Layer 2 fence scope = discovered only (marker OR type-use).
/// Used by Layer 2 forbidden-token scan + `no_f64_in_complete_set_or_market_seed`.
///
/// **Codex round-5 (R5) DASHBOARD-FLOOR remediation 2026-05-03**:
/// narrower than Layer 1 because Layer 2 tokens (f64 / AMM / orderbook /
/// price names) can legitimately appear in non-TB-13 files for unrelated
/// reasons (e.g., negative-list test fixtures in `audit_dashboard.rs`
/// at line 1628 that BAN those tokens — not USE them). Restricting
/// Layer 2 to discovered files (i.e., files that actually contribute
/// TB-13 code via marker OR TB-13 type use) prevents false positives on
/// non-TB-13 baseline code that happens to mention forbidden token
/// names. `audit_dashboard.rs` will auto-enter this scope when TB-14
/// ships dashboard contributions there.
fn effective_layer_2_scope() -> Vec<String> {
    let mut set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for s in discover_tb_13_files() {
        set.insert(s);
    }
    set.into_iter().collect()
}

/// Hard-banned legacy CPMM imports — these strings MUST NOT appear in
/// any FENCE_SCOPE file regardless of TB-13-marker discipline. Codex
/// round-1 Q9 CHALLENGE remediation (2026-05-03): the marker-only fence
/// could be bypassed by writing a legacy import outside a TB-13 doc-
/// comment span. These tokens are unconditionally banned (a `use
/// crate::prediction_market::BinaryMarket` anywhere in scope is an
/// architectural regression even in non-TB-13 sections).
const HARD_BANNED_LEGACY_IMPORTS: &[&str] = &[
    "use crate::prediction_market::",
    "use crate::prediction_market;",
    "crate::prediction_market::BinaryMarket",
    "crate::prediction_market::MarketError",
];

/// SG-13.0.1 — `legacy_cpm_api_not_imported_by_complete_set`.
///
/// Architect §4.2 halting trigger: HALT if NEW TB-13 code imports legacy
/// `prediction_market.rs`. Two layers of enforcement:
///
/// **Layer 1 (unconditional, Codex round-1 Q9 remediation)**: scan every
/// FENCE_SCOPE file for `HARD_BANNED_LEGACY_IMPORTS` regardless of
/// TB-13-marker discipline. Catches any new use-statement or type
/// reference that pulls legacy CPMM into a TB-13-scope module.
///
/// **Layer 2 (TB-13-marker-scoped)**: scan TB-13-marked spans for the
/// broader `FORBIDDEN_LEGACY_TOKENS` set (API names, trading/AMM
/// concepts). The marker discipline allows benign references in
/// historical doc-comments while keeping new TB-13 code clean.
#[test]
fn legacy_cpm_api_not_imported_by_complete_set() {
    let mut violations: Vec<String> = Vec::new();
    let scope = effective_fence_scope();

    // Layer 1: unconditional whole-file scan for hard-banned imports.
    for rel in &scope {
        let source = read_scope_file(rel);
        for (line_no, line) in source.lines().enumerate() {
            for token in HARD_BANNED_LEGACY_IMPORTS {
                if line.contains(token) {
                    violations.push(format!(
                        "{rel}:{}: hard-banned legacy import `{token}` — {line}",
                        line_no + 1
                    ));
                }
            }
        }
    }

    // Layer 2: scan for trading/AMM concepts. Restricted to discovered
    // (Codex round-5 DASHBOARD-FLOOR remediation 2026-05-03): Layer 2
    // tokens (f64 / AMM / orderbook / price names) can appear legitimately
    // in non-TB-13 baseline code (e.g., negative-list test fixtures); only
    // files that actually contribute TB-13 (via marker OR TB-13 type use)
    // should be Layer-2-scanned. `tb_13_scan_lines` then resolves the
    // PARTIAL-MARKER case: marker-spans ∪ non-marker TB-13-type-use lines.
    let layer_2_scope = effective_layer_2_scope();
    for rel in &layer_2_scope {
        let source = read_scope_file(rel);
        for (line_no, line) in tb_13_scan_lines(&source) {
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
                        "{rel}:{line_no}: TB-13-scope contains forbidden token `{token}` — {line}"
                    ));
                }
            }
        }
    }
    assert!(
        violations.is_empty(),
        "TB-13 SG-13.0.1 forward-fence violated (Layer 1 scope: {} files; Layer 2 scope: {} files):\n{}",
        scope.len(),
        layer_2_scope.len(),
        violations.join("\n")
    );
}

/// SG-13.0.2 — `no_f64_in_complete_set_or_market_seed`. Now uses
/// effective_layer_2_scope() (discovered-only, per Codex R5 DASHBOARD-FLOOR
/// remediation 2026-05-03 — Layer 2 tokens like f64 can appear in
/// non-TB-13 baseline code for unrelated reasons).
///
/// Architect §4.2 halting trigger: HALT if `f64` appears in NEW
/// CompleteSet / MarketSeed code. Money-path types must use integer
/// `MicroCoin` / `ShareAmount`.
#[test]
fn no_f64_in_complete_set_or_market_seed() {
    let mut violations: Vec<String> = Vec::new();
    let f64_tokens = [" f64", "f64,", "f64;", "f64)"];
    for rel in &effective_layer_2_scope() {
        let source = read_scope_file(rel);
        // tb_13_scan_lines: marker-files → spans ∪ non-marker TB-13-type-use
        // lines (Codex R5 PARTIAL-MARKER); unmarked-discovered files →
        // all non-comment lines (Codex R4 RQ6).
        for (line_no, line) in tb_13_scan_lines(&source) {
            for token in &f64_tokens {
                if line.contains(token) {
                    violations.push(format!(
                        "{rel}:{line_no}: TB-13-scope contains f64 (`{token}`) — {line}"
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

/// Round-5 RQ6 unit test: `discover_by_type_use` catches a fresh file
/// that imports a TB-13 type without an authoring marker, and the
/// pure-comment skip prevents a TB-12 doc-comment cross-reference
/// from being misclassified as a TB-13 contributor.
#[test]
fn discover_by_type_use_catches_unmarked_imports_and_skips_doc_xref() {
    use std::io::Write;
    let tmp = std::env::temp_dir().join(format!(
        "tb13_fence_discovery_test_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).expect("mkdir tmp");

    // (1) An unmarked file that USES a TB-13 type → must be discovered.
    let unmarked_path = tmp.join("unmarked_user.rs");
    {
        let mut f = fs::File::create(&unmarked_path).expect("create unmarked");
        writeln!(
            f,
            "use crate::state::typed_tx::CompleteSetMintTx;\nfn touch() -> CompleteSetMintTx {{ CompleteSetMintTx::default() }}"
        )
        .unwrap();
    }

    // (2) A file with TB-13 type names ONLY in /// doc-comment lines →
    // must NOT be discovered (TB-12 legacy doc-xref pattern).
    let docxref_path = tmp.join("doc_xref_only.rs");
    {
        let mut f = fs::File::create(&docxref_path).expect("create docxref");
        writeln!(
            f,
            "/// Replaced by TB-13 `CompleteSetMintTx` (canonical mint).\n//! see ConditionalShareBalances for the future shape.\npub struct Unrelated;"
        )
        .unwrap();
    }

    // (3) A control file with no TB-13 references → not discovered.
    let neutral_path = tmp.join("neutral.rs");
    {
        let mut f = fs::File::create(&neutral_path).expect("create neutral");
        writeln!(f, "pub fn add(a: i64, b: i64) -> i64 {{ a + b }}").unwrap();
    }

    let found = discover_by_type_use(&tmp);
    let found_set: std::collections::BTreeSet<&str> =
        found.iter().map(|s| s.as_str()).collect();

    let unmarked_str = unmarked_path.to_string_lossy().into_owned();
    let docxref_str = docxref_path.to_string_lossy().into_owned();
    let neutral_str = neutral_path.to_string_lossy().into_owned();

    assert!(
        found_set.contains(unmarked_str.as_str()),
        "RQ6: unmarked TB-13 type-use file must be discovered. Got: {found:?}"
    );
    assert!(
        !found_set.contains(docxref_str.as_str()),
        "RQ6: doc-xref-only file must NOT be discovered. Got: {found:?}"
    );
    assert!(
        !found_set.contains(neutral_str.as_str()),
        "RQ6: neutral file must NOT be discovered. Got: {found:?}"
    );

    // Also assert that the marker walk alone would have missed (1) —
    // proves type-use is the path that catches it.
    let marker_only = discover_by_marker(&tmp);
    let marker_set: std::collections::BTreeSet<&str> =
        marker_only.iter().map(|s| s.as_str()).collect();
    assert!(
        !marker_set.contains(unmarked_str.as_str()),
        "RQ6: marker walk alone should NOT have caught the unmarked file (otherwise the type-use layer is redundant). Got: {marker_only:?}"
    );

    let _ = fs::remove_dir_all(&tmp);
}

/// Round-6 R4-Codex remediation 2026-05-03: `tb_13_scan_lines` returns
/// marker-spans for marker-files (preserves the doc-xref skip) and all
/// non-comment lines for unmarked files (closes the Layer 2 gap where
/// type-use-discovered files could ship f64 / AMM tokens unscanned).
#[test]
fn tb_13_scan_lines_handles_marker_and_unmarked_files() {
    // Case A — marker-file: scan lines come from `tb_13_spans`. A
    // /// TB-12 line referencing TB-13 in passing is OUTSIDE any TB-13
    // span (because the span's first non-blank line is the TB-12 marker,
    // not a TB-13 marker), so it must be skipped.
    let marker_src = "\
//! TB-13 module header.\n\
pub fn tb13_thing() -> i32 { 42_f64 as i32 }\n\
\n\
/// TB-12 doc xref to TB-13 future work.\n\
pub fn tb12_legacy() -> i32 { 0 }\n\
";
    let scanned = tb_13_scan_lines(marker_src);
    let scanned_text: Vec<&str> =
        scanned.iter().map(|(_, l)| l.as_str()).collect();
    assert!(
        scanned_text.iter().any(|l| l.contains("tb13_thing")),
        "marker-file: TB-13 span lines must be returned"
    );
    assert!(
        scanned_text.iter().all(|l| !l.contains("tb12_legacy")),
        "marker-file: TB-12 span lines must NOT be returned (preserves doc-xref skip)"
    );

    // Case B — unmarked file: scan lines fall back to ALL non-comment
    // lines. The f64 / AMM scan must see the violating line.
    let unmarked_src = "\
use crate::state::typed_tx::CompleteSetMintTx;\n\
fn forbidden() -> f64 { 0.5_f64 }\n\
// trailing comment\n\
";
    let scanned = tb_13_scan_lines(unmarked_src);
    let scanned_text: Vec<&str> =
        scanned.iter().map(|(_, l)| l.as_str()).collect();
    assert!(
        scanned_text.iter().any(|l| l.contains("f64")),
        "unmarked-file: non-comment lines must be returned (Layer 2 must see f64)"
    );
    assert!(
        scanned_text.iter().all(|l| !l.contains("trailing comment")),
        "unmarked-file: pure-comment lines must still be filtered out"
    );
}

/// Round-7 R5-Codex PARTIAL-MARKER remediation 2026-05-03: a
/// marker-bearing file with stealth TB-13 type-use OUTSIDE any marker
/// span must still have those non-marker type-use lines scanned.
#[test]
fn tb_13_scan_lines_partial_marker_catches_stealth_type_use() {
    // Marker-file: one marker-span at top + a TB-13 type use OUTSIDE the
    // marker span (no TB-13 marker on the second function). Round-6
    // helper would have only scanned the marker span; round-7 must also
    // return the non-marker line containing `CompleteSetMintTx`.
    let src = "\
//! TB-13 module header.\n\
pub fn tb13_marked() -> i32 { 0 }\n\
\n\
fn stealth(_: CompleteSetMintTx) -> f64 { 0.0_f64 }\n\
";
    let scanned = tb_13_scan_lines(src);
    let scanned_text: Vec<&str> =
        scanned.iter().map(|(_, l)| l.as_str()).collect();
    assert!(
        scanned_text.iter().any(|l| l.contains("tb13_marked")),
        "marker-span line must be returned"
    );
    assert!(
        scanned_text
            .iter()
            .any(|l| l.contains("CompleteSetMintTx") && l.contains("f64")),
        "non-marker line containing TB-13 type name must also be returned (PARTIAL-MARKER closure)"
    );
}

/// Round-7 R5-Codex DASHBOARD-FLOOR remediation 2026-05-03: Layer 1
/// (hard-banned-imports) scope retains `audit_dashboard.rs`; Layer 2
/// (forbidden-token) scope omits it because it currently has no TB-13
/// markers and no TB-13 type uses. The split prevents Layer 2 false-
/// positives on negative-list test fixtures while preserving Layer 1
/// hard-import enforcement.
#[test]
fn audit_dashboard_in_layer_1_scope_but_not_layer_2_scope() {
    let layer_1 = effective_fence_scope();
    let layer_2 = effective_layer_2_scope();
    assert!(
        layer_1.iter().any(|s| s == "src/bin/audit_dashboard.rs"),
        "DASHBOARD-FLOOR: audit_dashboard.rs must remain in Layer 1 scope (hard-imports always banned). Got: {layer_1:?}"
    );
    assert!(
        !layer_2.iter().any(|s| s == "src/bin/audit_dashboard.rs"),
        "DASHBOARD-FLOOR: audit_dashboard.rs must NOT be in Layer 2 scope until it gains TB-13 markers / type uses (otherwise its negative-list test fixture false-positives). Got: {layer_2:?}"
    );
}
