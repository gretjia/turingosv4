//! REAL-10 Atom 1 — TRACE_MATRIX / R-022 backlink cleanup gates.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

fn normalize_skip_symbol(symbol: &str) -> String {
    if symbol == "pub_const_fn" {
        return "fn".into();
    }
    for prefix in [
        "pub_fn_",
        "pub_struct_",
        "pub_enum_",
        "pub_trait_",
        "pub_const_",
        "pub_mod_",
        "pub_type_",
        "pub_static_",
    ] {
        if let Some(rest) = symbol.strip_prefix(prefix) {
            return rest.into();
        }
    }
    symbol.into()
}

fn real5s_real9_skip_pairs() -> BTreeSet<(String, String)> {
    let log = fs::read_to_string("rules/enforcement.log").expect("enforcement log");
    let mut pairs = BTreeSet::new();
    for line in log.lines() {
        if !line.contains("R-022-SKIP")
            || !line.contains("OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md")
        {
            continue;
        }
        let file = line
            .split(" file=")
            .nth(1)
            .and_then(|s| s.split_whitespace().next())
            .expect("skip line has file");
        let symbol = line
            .split(" symbol=")
            .nth(1)
            .and_then(|s| s.split_whitespace().next())
            .expect("skip line has symbol");
        if symbol == "trace_removal" {
            continue;
        }
        pairs.insert((file.into(), normalize_skip_symbol(symbol)));
    }
    pairs
}

fn trace_matrix_j_pairs() -> BTreeSet<(String, String)> {
    let matrix = fs::read_to_string("handover/alignment/TRACE_MATRIX_v3_2026-04-27.md")
        .expect("TRACE_MATRIX");
    let mut pairs = BTreeSet::new();
    let mut in_j2 = false;
    for line in matrix.lines() {
        if line.starts_with("### § J.2 Open orphan rows") {
            in_j2 = true;
            continue;
        }
        if in_j2 && line.starts_with("### § J.3") {
            break;
        }
        if !in_j2 || !line.starts_with('|') || line.contains("---") {
            continue;
        }
        let cols: Vec<_> = line.split('|').map(str::trim).collect();
        if cols.len() < 4 || cols[1] == "File path" || cols[1].starts_with("_(") {
            continue;
        }
        let file = cols[1].trim_matches('`').to_string();
        let symbol = cols[2].trim_matches('`').to_string();
        pairs.insert((file, symbol));
    }
    pairs
}

#[test]
fn real10_r022_skipped_surfaces_have_trace_matrix_j_rows() {
    let skips = real5s_real9_skip_pairs();
    assert!(
        !skips.is_empty(),
        "REAL-10 cleanup must find REAL-5S->REAL-9 R-022 skip entries"
    );
    let registered = trace_matrix_j_pairs();
    let missing: Vec<_> = skips.difference(&registered).cloned().collect();
    assert!(
        missing.is_empty(),
        "R-022 skipped surfaces missing TRACE_MATRIX §J rows: {missing:#?}"
    );
}

#[test]
fn real10_trace_cleanup_report_records_one_time_exception_not_waiver() {
    let report =
        fs::read_to_string("handover/alignment/TRACE_MATRIX_BACKLINK_CLEANUP_REAL5S_REAL9.md")
            .expect("cleanup report");
    for expected in [
        "one-time bulk-ship exception closure, not policy relaxation",
        "R-022 not treated as waiver",
        "docs-first §J registration",
        "skipped public surfaces",
    ] {
        assert!(
            report.contains(expected),
            "cleanup report must preserve architect boundary: {expected}"
        );
    }
}

#[test]
fn real10_trace_cleanup_justification_file_exists() {
    assert!(
        Path::new("handover/alignment/OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md").exists(),
        "§J rows must cite an existing OBS_R022 justification file"
    );
}

#[test]
fn real10_trace_removals_are_closed_audit_trail_not_open_surfaces() {
    let matrix = fs::read_to_string("handover/alignment/TRACE_MATRIX_v3_2026-04-27.md")
        .expect("TRACE_MATRIX");
    let registered = trace_matrix_j_pairs();
    assert!(
        !registered.contains(&(
            "src/runtime/attempt_telemetry.rs".to_string(),
            "trace_removal".to_string()
        )),
        "trace_removal must not be represented as an open §J.2 public surface"
    );
    assert!(
        !registered.contains(&(
            "src/state/typed_tx.rs".to_string(),
            "trace_removal".to_string()
        )),
        "trace_removal must not be represented as an open §J.2 public surface"
    );
    for expected in [
        "`src/runtime/attempt_telemetry.rs` | `trace_removal` | Legacy TRACE_MATRIX backlink removal",
        "`src/state/typed_tx.rs` | `trace_removal` | Legacy TRACE_MATRIX backlink removal",
        "audit-trail row only, not an open public surface",
    ] {
        assert!(
            matrix.contains(expected),
            "closed §J.3 audit trail must preserve trace_removal context: {expected}"
        );
    }
}

#[test]
fn real10_trace_cleanup_rows_use_checker_compatible_symbols() {
    let registered = trace_matrix_j_pairs();
    for (_, symbol) in registered {
        for raw_prefix in [
            "pub_fn_",
            "pub_struct_",
            "pub_enum_",
            "pub_trait_",
            "pub_const_",
            "pub_mod_",
            "pub_type_",
            "pub_static_",
        ] {
            assert!(
                !symbol.starts_with(raw_prefix),
                "§J.2 symbols must use checker-compatible normalized names, not raw skip token {symbol}"
            );
        }
    }
}

#[test]
fn real10_pub_const_fn_parser_ambiguity_is_explicit() {
    let matrix = fs::read_to_string("handover/alignment/TRACE_MATRIX_v3_2026-04-27.md")
        .expect("TRACE_MATRIX");
    for expected in [
        "parser_ambiguity=pub_const_fn",
        "actual functions: `label` at line 52 and `kind` at line 421",
        "actual function: `is_role_window_tick` at line 53",
        "checker-compatible key `fn`",
    ] {
        assert!(
            matrix.contains(expected),
            "pub_const_fn ambiguity must be explicit in TRACE_MATRIX: {expected}"
        );
    }
}
