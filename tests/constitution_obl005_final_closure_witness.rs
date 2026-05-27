//! OBL-005 final closure witness.
//!
//! Verifies the complete final closure state of OBL-005 (三 Flowchart 全链路覆盖测试集设计与落地).
//! This gate is the authoritative closure witness for OBL-005. It does not claim OBL-001
//! is satisfied or that the global project is complete.

use std::fs;

const OBLIGATIONS_PATH: &str = "OBLIGATIONS.md";
const WITNESS_PATH: &str = "handover/audits/OBL005_FINAL_CLOSURE_WITNESS_2026-05-27.md";
const RECONCILIATION_MANIFEST: &str =
    "tests/fixtures/liveness/true_suite_evidence_reconciliation.toml";
const PRODUCTION_MANIFEST: &str = "tests/fixtures/liveness/production_module_liveness.toml";
const SCRIPT_MANIFEST: &str = "tests/fixtures/liveness/script_liveness_inventory.toml";
const REALWORLD_MANIFEST: &str = "tests/fixtures/liveness/realworld_liveness_coverage.toml";
const BROAD_MANIFEST: &str = "tests/fixtures/liveness/broad_agi_true_suite_manifest.toml";
const FINAL_CLOSURE_STATUS: &str = "OBL005_FINAL_CLOSURE_VERIFIED";

fn read_text(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn parse_toml(path: &str) -> toml::Value {
    let raw = read_text(path);
    toml::from_str(&raw).unwrap_or_else(|err| panic!("parse {path}: {err}"))
}

fn extract_obl_block(text: &str, obl_id: &str) -> String {
    let mut in_block = false;
    let mut lines = Vec::new();
    for line in text.lines() {
        if line.starts_with("## ") && line.contains(obl_id) {
            in_block = true;
        } else if in_block && line.starts_with("## ") {
            break;
        }
        if in_block {
            lines.push(line);
        }
    }
    lines.join("\n")
}

#[test]
fn obl005_is_satisfied_and_obl001_remains_open() {
    let text = read_text(OBLIGATIONS_PATH);

    let obl005_block = extract_obl_block(&text, "OBL-005");
    assert!(
        obl005_block.contains("Status: **satisfied**")
            || obl005_block.contains("Status: satisfied"),
        "OBL-005 must be Status: satisfied in OBLIGATIONS.md; found block:\n{obl005_block}"
    );

    let obl001_block = extract_obl_block(&text, "OBL-001");
    assert!(
        obl001_block.contains("Status: open") || obl001_block.contains("Status: **open**"),
        "OBL-001 must remain Status: open; found block:\n{obl001_block}"
    );
}

#[test]
fn witness_file_exists_and_contains_required_verdict() {
    let text = read_text(WITNESS_PATH);

    assert!(
        text.contains("VERDICT: OBL005-FINAL-CLOSURE-VERIFIED"),
        "witness file must contain 'VERDICT: OBL005-FINAL-CLOSURE-VERIFIED'"
    );

    let lower = text.to_lowercase();
    assert!(
        lower.contains("does not close obl-001") || lower.contains("not close obl-001"),
        "witness must state it does not close OBL-001; witness text (lowercase):\n{lower}"
    );
    assert!(
        lower.contains("full project completion") || lower.contains("project completion"),
        "witness must address global project completion scope"
    );
    assert!(
        lower.contains("no historical") || lower.contains("not rewritten"),
        "witness must affirm no historical evidence was rewritten"
    );
    assert!(
        lower.contains("src/"),
        "witness must state no runtime source under src/ was touched"
    );
}

#[test]
fn all_five_manifest_fields_are_final_closure_verified() {
    let reconciliation = parse_toml(RECONCILIATION_MANIFEST);
    assert_eq!(
        reconciliation
            .get("reconciliation_status")
            .and_then(toml::Value::as_str),
        Some(FINAL_CLOSURE_STATUS),
        "reconciliation_status must be {FINAL_CLOSURE_STATUS}"
    );

    let production = parse_toml(PRODUCTION_MANIFEST);
    assert_eq!(
        production
            .get("final_closure_status")
            .and_then(toml::Value::as_str),
        Some(FINAL_CLOSURE_STATUS),
        "production final_closure_status must be {FINAL_CLOSURE_STATUS}"
    );

    let script = parse_toml(SCRIPT_MANIFEST);
    assert_eq!(
        script
            .get("final_closure_status")
            .and_then(toml::Value::as_str),
        Some(FINAL_CLOSURE_STATUS),
        "script final_closure_status must be {FINAL_CLOSURE_STATUS}"
    );

    let realworld = parse_toml(REALWORLD_MANIFEST);
    assert_eq!(
        realworld
            .get("final_closure_status")
            .and_then(toml::Value::as_str),
        Some(FINAL_CLOSURE_STATUS),
        "realworld final_closure_status must be {FINAL_CLOSURE_STATUS}"
    );

    let broad = parse_toml(BROAD_MANIFEST);
    assert_eq!(
        broad.get("closure_status").and_then(toml::Value::as_str),
        Some(FINAL_CLOSURE_STATUS),
        "broad closure_status must be {FINAL_CLOSURE_STATUS}"
    );
}

#[test]
fn reconciliation_manifest_has_final_closure_claimed_and_no_evidence_rewrite() {
    let manifest = parse_toml(RECONCILIATION_MANIFEST);
    assert_eq!(
        manifest
            .get("final_closure_claimed")
            .and_then(toml::Value::as_bool),
        Some(true),
        "final_closure_claimed must be true in reconciliation manifest"
    );
    assert_eq!(
        manifest
            .get("rewrites_historical_evidence")
            .and_then(toml::Value::as_bool),
        Some(false),
        "rewrites_historical_evidence must be false: no old evidence may be rewritten for closure"
    );
}

#[test]
fn no_legacy_quarantined_group_may_coexist_with_final_closure() {
    let manifest = parse_toml(PRODUCTION_MANIFEST);
    let groups = manifest
        .get("group")
        .and_then(toml::Value::as_array)
        .expect("production_module_liveness.toml must have [[group]] rows");
    for group in groups {
        let table = group.as_table().expect("group entry must be a table");
        let status = table
            .get("status")
            .and_then(toml::Value::as_str)
            .unwrap_or("");
        assert_ne!(
            status,
            "legacy_quarantined",
            "no group may have status legacy_quarantined when final_closure_status is {FINAL_CLOSURE_STATUS}"
        );
    }
}

#[test]
fn dev_only_and_historical_script_groups_do_not_count_for_closure() {
    let manifest = parse_toml(SCRIPT_MANIFEST);
    let groups = manifest
        .get("script_group")
        .and_then(toml::Value::as_array)
        .expect("script_liveness_inventory.toml must have [[script_group]] rows");
    for group in groups {
        let table = group
            .as_table()
            .expect("script_group entry must be a table");
        let status = table
            .get("status")
            .and_then(toml::Value::as_str)
            .unwrap_or("");
        let classification = table
            .get("classification")
            .and_then(toml::Value::as_str)
            .unwrap_or("");
        let counts = table
            .get("counts_for_obl005_script_closure")
            .and_then(toml::Value::as_bool)
            .unwrap_or(false);
        if matches!(status, "dev_only" | "historical_smoke") || classification == "local_probe" {
            assert!(
                !counts,
                "script group with status '{status}' / classification '{classification}' \
                 must not count for OBL-005 script closure"
            );
        }
    }
}

#[test]
fn global_project_closure_is_not_claimed() {
    let text = read_text(OBLIGATIONS_PATH);
    let headline: String = text.lines().take(15).collect::<Vec<_>>().join("\n");

    assert!(
        headline.contains("OBL-001 open"),
        "OBLIGATIONS.md headline must explicitly state OBL-001 is open; headline:\n{headline}"
    );
    assert!(
        !headline
            .to_lowercase()
            .contains("all obligations satisfied")
            && !headline.to_lowercase().contains("project complete"),
        "OBLIGATIONS.md headline must not claim global project completion"
    );

    let witness_text = read_text(WITNESS_PATH);
    let lower = witness_text.to_lowercase();
    assert!(
        !lower.contains("obl-001 satisfied") && !lower.contains("obl-001: satisfied"),
        "witness must not claim OBL-001 is satisfied"
    );
}
