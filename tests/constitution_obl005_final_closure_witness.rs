//! OBL-005 liveness re-audit witness.
//!
//! The 2026-05-27 final-closure witness remains an immutable historical receipt.
//! Current OBL-005 status is reopened while production/script liveness accounting
//! is re-audited against retained diagnostic bins that lack ChainTape/CAS final
//! evidence.

use std::fs;

const OBLIGATIONS_PATH: &str = "OBLIGATIONS.md";
const LATEST_PATH: &str = "handover/ai-direct/LATEST.md";
const WITNESS_PATH: &str = "handover/audits/OBL005_FINAL_CLOSURE_WITNESS_2026-05-27.md";
const CLOSURE_SCOPE_PACKET: &str =
    "handover/directives/2026-06-05_OBL005_CLOSURE_SCOPE_DECISION_PACKET.md";
const RECONCILIATION_MANIFEST: &str =
    "tests/fixtures/liveness/true_suite_evidence_reconciliation.toml";
const PRODUCTION_MANIFEST: &str = "tests/fixtures/liveness/production_module_liveness.toml";
const SCRIPT_MANIFEST: &str = "tests/fixtures/liveness/script_liveness_inventory.toml";
const REALWORLD_MANIFEST: &str = "tests/fixtures/liveness/realworld_liveness_coverage.toml";
const BROAD_MANIFEST: &str = "tests/fixtures/liveness/broad_agi_true_suite_manifest.toml";
const EXECUTION_MATRIX: &str = "handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md";
const REAUDIT_STATUS: &str = "OBL005_REAUDIT_IN_PROGRESS";

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
fn obl005_is_reopened_for_liveness_reaudit() {
    let text = read_text(OBLIGATIONS_PATH);

    let headline: String = text.lines().take(15).collect::<Vec<_>>().join("\n");
    assert!(
        headline.contains("REOPENED / REAUDIT IN PROGRESS"),
        "OBLIGATIONS.md headline must state the current re-audit status; headline:\n{headline}"
    );
    assert!(
        !headline.contains("**COMPLETE**"),
        "OBLIGATIONS.md headline must not claim global all-closed completion while OBL-005 is reopened; headline:\n{headline}"
    );

    let obl005_block = extract_obl_block(&text, "OBL-005");
    assert!(
        obl005_block.contains("Status: in_progress") && obl005_block.contains("reopened"),
        "OBL-005 must be reopened/in_progress in OBLIGATIONS.md; found block:\n{obl005_block}"
    );
    assert!(
        obl005_block.contains("OBL005_REAUDIT_IN_PROGRESS")
            && obl005_block.contains("historical, not current closure authority"),
        "OBL-005 block must state why the historical final witness is not current closure authority; found block:\n{obl005_block}"
    );

    let obl001_block = extract_obl_block(&text, "OBL-001");
    assert!(
        obl001_block.contains("metrics.json"),
        "OBL-001 evidence must reference final metrics; found block:\n{obl001_block}"
    );
    assert!(
        obl001_block.contains("redaction_audit.json"),
        "OBL-001 evidence must reference redaction audit; found block:\n{obl001_block}"
    );
    assert!(
        obl001_block.contains("CLEAN_CONTEXT_AUDIT"),
        "OBL-001 evidence must reference clean-context audit; found block:\n{obl001_block}"
    );
}

#[test]
fn historical_witness_file_exists_but_is_not_current_authority() {
    let text = read_text(WITNESS_PATH);

    assert!(
        text.contains("VERDICT: OBL005-FINAL-CLOSURE-VERIFIED"),
        "witness file must contain 'VERDICT: OBL005-FINAL-CLOSURE-VERIFIED'"
    );

    let ledger = read_text(OBLIGATIONS_PATH);
    let obl005_block = extract_obl_block(&ledger, "OBL-005");
    assert!(
        obl005_block.contains("Historical final closure witness")
            && obl005_block.contains("not current closure authority"),
        "ledger must preserve the historical witness while denying current closure authority; found block:\n{obl005_block}"
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
fn closure_scope_packet_is_required_before_fresh_final_witness() {
    let packet = read_text(CLOSURE_SCOPE_PACKET);
    for required in [
        "APPROVED-OBL005-NO-ZOMBIE-SCOPE",
        "Before any future PR changes `final_closure_claimed`",
        "One-word messages",
        "benchmark/domain failures as capability-pending facts",
        "multi-node priced-DAG reward settlement",
        "Class 4 M2/M3 settlement redesign",
        "Do not edit old ChainTape/CAS evidence",
    ] {
        assert!(
            packet.contains(required),
            "closure-scope packet must preserve ratification/fake-closure boundary text: {required}"
        );
    }

    let ledger = read_text(OBLIGATIONS_PATH);
    let obl005_block = extract_obl_block(&ledger, "OBL-005");
    assert!(
        obl005_block.contains(CLOSURE_SCOPE_PACKET)
            && obl005_block.contains("pending explicit ratification")
            && obl005_block.contains("No final closure is claimed"),
        "OBL-005 ledger must bind the scope packet as a current closure precondition; found block:\n{obl005_block}"
    );

    let latest = read_text(LATEST_PATH);
    assert!(
        latest.contains(CLOSURE_SCOPE_PACKET)
            && latest.contains("No final closure is claimed")
            && latest.contains("Last synchronized base"),
        "LATEST.md must describe the scope packet as derived handover state without claiming current closure"
    );
}

#[test]
fn closure_scope_ratification_guard_is_current_handover_state() {
    let ledger = read_text(OBLIGATIONS_PATH);
    let obl005_block = extract_obl_block(&ledger, "OBL-005");
    for required in [
        "Closure-scope ratification guard",
        "PR #277",
        "tests/constitution_obl005_final_closure_witness.rs::closure_scope_packet_is_required_before_fresh_final_witness",
        "does not ratify the scope",
        "does not close OBL-005",
    ] {
        assert!(
            obl005_block.contains(required),
            "OBL-005 ledger must record the executable scope-ratification guard: {required}"
        );
    }

    let latest = read_text(LATEST_PATH);
    for required in [
        "PR #278",
        "dbfbe52c",
        "executable closure-scope ratification guard",
        "Current-binary boundary",
        "418d8a7d",
        "7b12e9f1",
        "guard only",
        "does not close OBL-005",
    ] {
        assert!(
            latest.contains(required),
            "LATEST.md must be synchronized to the current guard/boundary state: {required}"
        );
    }
}

#[test]
fn execution_matrix_does_not_claim_current_obl005_final_closure() {
    let matrix = read_text(EXECUTION_MATRIX);
    assert!(
        matrix.contains(REAUDIT_STATUS),
        "execution matrix must mirror current OBL-005 re-audit status"
    );
    assert!(
        !matrix.contains("OBL-005 final closure verified"),
        "execution matrix is a derived view and must not preserve stale final-closure status while OBL-005 is reopened"
    );
    assert!(
        !matrix.contains("2026-05-27 final closure witness verified"),
        "execution matrix must treat the 2026-05-27 witness as historical, not current closure authority"
    );
}

#[test]
fn all_current_liveness_manifests_reopen_final_closure() {
    let reconciliation = parse_toml(RECONCILIATION_MANIFEST);
    assert_eq!(
        reconciliation
            .get("reconciliation_status")
            .and_then(toml::Value::as_str),
        Some(REAUDIT_STATUS),
        "current reconciliation_status must reopen to {REAUDIT_STATUS}; the historical witness file remains immutable separately"
    );

    let production = parse_toml(PRODUCTION_MANIFEST);
    assert_eq!(
        production
            .get("final_closure_status")
            .and_then(toml::Value::as_str),
        Some(REAUDIT_STATUS),
        "production final_closure_status must reopen to {REAUDIT_STATUS}"
    );

    let script = parse_toml(SCRIPT_MANIFEST);
    assert_eq!(
        script
            .get("final_closure_status")
            .and_then(toml::Value::as_str),
        Some(REAUDIT_STATUS),
        "script final_closure_status must reopen to {REAUDIT_STATUS}"
    );

    let realworld = parse_toml(REALWORLD_MANIFEST);
    assert_eq!(
        realworld
            .get("final_closure_status")
            .and_then(toml::Value::as_str),
        Some(REAUDIT_STATUS),
        "realworld manifest must not out-rank production/script liveness while OBL-005 is reopened"
    );

    let broad = parse_toml(BROAD_MANIFEST);
    assert_eq!(
        broad.get("closure_status").and_then(toml::Value::as_str),
        Some(REAUDIT_STATUS),
        "broad manifest must not out-rank production/script liveness while OBL-005 is reopened"
    );
}

#[test]
fn historical_reconciliation_claim_cannot_override_reaudit_status() {
    let manifest = parse_toml(RECONCILIATION_MANIFEST);
    assert_eq!(
        manifest
            .get("final_closure_claimed")
            .and_then(toml::Value::as_bool),
        Some(false),
        "final_closure_claimed must be false while OBL-005 is reopened"
    );
    assert_eq!(
        manifest
            .get("rewrites_historical_evidence")
            .and_then(toml::Value::as_bool),
        Some(false),
        "rewrites_historical_evidence must be false: no old evidence may be rewritten for closure"
    );

    let production = parse_toml(PRODUCTION_MANIFEST);
    let script = parse_toml(SCRIPT_MANIFEST);
    assert_eq!(
        production
            .get("final_closure_status")
            .and_then(toml::Value::as_str),
        Some(REAUDIT_STATUS),
        "current production liveness status outranks historical reconciliation closure"
    );
    assert_eq!(
        script
            .get("final_closure_status")
            .and_then(toml::Value::as_str),
        Some(REAUDIT_STATUS),
        "current script liveness status outranks historical reconciliation closure"
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
            status, "legacy_quarantined",
            "legacy_quarantined groups remain incompatible with OBL-005 closure"
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
fn global_obligation_completion_is_not_currently_claimed() {
    let text = read_text(OBLIGATIONS_PATH);
    let headline: String = text.lines().take(15).collect::<Vec<_>>().join("\n");

    assert!(
        headline.contains("REOPENED / REAUDIT IN PROGRESS"),
        "OBLIGATIONS.md headline must state current re-audit, not all-closed completion; headline:\n{headline}"
    );
    assert!(
        !headline.contains("OBL-ALL-CLOSED"),
        "headline:\n{headline}"
    );

    let witness_text = read_text(WITNESS_PATH);
    let lower = witness_text.to_lowercase();
    assert!(
        !lower.contains("obl-001 satisfied") && !lower.contains("obl-001: satisfied"),
        "OBL-005 witness must not itself claim OBL-001 is satisfied (scoped to OBL-005)"
    );
}
