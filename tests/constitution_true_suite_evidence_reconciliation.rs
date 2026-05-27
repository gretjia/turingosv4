//! Cross-run true-suite evidence reconciliation for OBL-005.
//!
//! Individual runner PRs intentionally commit immutable evidence roots. This
//! gate proves the final accounting layer can reconcile those separate roots
//! without rewriting old evidence or treating a single batch directory as the
//! only possible closure shape.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

const RECONCILIATION_MANIFEST: &str =
    "tests/fixtures/liveness/true_suite_evidence_reconciliation.toml";
const REALWORLD_MANIFEST: &str = "tests/fixtures/liveness/realworld_liveness_coverage.toml";
const BROAD_MANIFEST: &str = "tests/fixtures/liveness/broad_agi_true_suite_manifest.toml";
const TRUE_SUITE_ROOT: &str = "handover/evidence/true_suite";
const FULL_SYSTEM_SCHEMA: &str = "turingosv4.true_suite.full_system_participation.v1";
const RECONCILIATION_STATUS: &str = "FULL_SYSTEM_RECONCILIATION_CANDIDATE";
const OPEN_STATUS: &str = "OPEN_REAL_WORLD_COVERAGE_PENDING";

#[derive(Debug)]
struct ContractRow {
    id: String,
    final_evidence_artifacts: Vec<String>,
}

#[derive(Debug)]
struct EvidenceBinding {
    id: String,
    evidence_run: String,
    evidence_subdir: String,
}

fn parse_toml(path: &str) -> toml::Value {
    let raw = fs::read_to_string(path).unwrap_or_else(|err| panic!("read {path}: {err}"));
    toml::from_str(&raw).unwrap_or_else(|err| panic!("parse {path}: {err}"))
}

fn as_string(table: &toml::value::Table, key: &str) -> String {
    table
        .get(key)
        .and_then(toml::Value::as_str)
        .unwrap_or_else(|| panic!("row missing string `{key}`: {table:?}"))
        .to_string()
}

fn as_str_array(table: &toml::value::Table, key: &str) -> Vec<String> {
    table
        .get(key)
        .and_then(toml::Value::as_array)
        .unwrap_or_else(|| panic!("row missing array `{key}`: {table:?}"))
        .iter()
        .map(|item| {
            item.as_str()
                .unwrap_or_else(|| panic!("array `{key}` contains non-string: {item:?}"))
                .to_string()
        })
        .collect()
}

fn contract_rows(path: &str, key: &str) -> BTreeMap<String, ContractRow> {
    parse_toml(path)
        .get(key)
        .and_then(toml::Value::as_array)
        .unwrap_or_else(|| panic!("{path} missing [[{key}]] rows"))
        .iter()
        .map(|row| {
            let table = row
                .as_table()
                .unwrap_or_else(|| panic!("{key} row is not a table: {row:?}"));
            let id = as_string(table, "id");
            let final_evidence_artifacts = as_str_array(table, "final_evidence_artifacts");
            (
                id.clone(),
                ContractRow {
                    id,
                    final_evidence_artifacts,
                },
            )
        })
        .collect()
}

fn reconciliation_rows(key: &str) -> Vec<EvidenceBinding> {
    parse_toml(RECONCILIATION_MANIFEST)
        .get(key)
        .and_then(toml::Value::as_array)
        .unwrap_or_else(|| panic!("{RECONCILIATION_MANIFEST} missing [[{key}]] rows"))
        .iter()
        .map(|row| {
            let table = row
                .as_table()
                .unwrap_or_else(|| panic!("{key} row is not a table: {row:?}"));
            EvidenceBinding {
                id: as_string(table, "id"),
                evidence_run: as_string(table, "evidence_run"),
                evidence_subdir: as_string(table, "evidence_subdir"),
            }
        })
        .collect()
}

fn materialize(template: &str, run_id: &str) -> PathBuf {
    PathBuf::from(template.replace(
        "handover/evidence/true_suite/<run>",
        &format!("{TRUE_SUITE_ROOT}/{run_id}"),
    ))
}

fn packaged_git_store_for(path: &Path) -> Option<PathBuf> {
    let name = path.file_name()?.to_str()?;
    let parent = path.parent()?;
    match name {
        "runtime_repo" => Some(parent.join("runtime_repo.dotgit.tar.gz")),
        "cas" => Some(parent.join("cas.dotgit.tar.gz")),
        _ => None,
    }
}

fn assert_artifact_reconstructable(binding: &EvidenceBinding, template: &str) {
    let lower = template.to_ascii_lowercase();
    assert!(
        !lower.contains("raw_prompt")
            && !lower.contains("raw_response")
            && !lower.contains("leaderboard")
            && !lower.contains("old15")
            && !lower.contains("old_15"),
        "binding `{}` uses non-final or contamination-prone artifact path: {template}",
        binding.id
    );

    let path = materialize(template, &binding.evidence_run);
    if let Some(packaged) = packaged_git_store_for(&path) {
        assert!(
            packaged.exists(),
            "binding `{}` raw git-store placeholder `{}` must be represented by packaged tarball `{}`",
            binding.id,
            path.display(),
            packaged.display()
        );
        return;
    }

    assert!(
        path.exists(),
        "binding `{}` declared artifact does not exist after run substitution: {}",
        binding.id,
        path.display()
    );
    if path.is_dir() {
        assert!(
            fs::read_dir(&path)
                .unwrap_or_else(|err| panic!("read dir {}: {err}", path.display()))
                .next()
                .is_some(),
            "binding `{}` declared artifact directory is empty: {}",
            binding.id,
            path.display()
        );
    }
}

fn nested_bool(report: &Value, keys: &[&str]) -> bool {
    let mut cur = report;
    for key in keys {
        cur = match cur.get(*key) {
            Some(value) => value,
            None => return false,
        };
    }
    cur.as_bool() == Some(true)
}

fn nested_u64(report: &Value, keys: &[&str]) -> u64 {
    let mut cur = report;
    for key in keys {
        cur = match cur.get(*key) {
            Some(value) => value,
            None => return 0,
        };
    }
    cur.as_u64().unwrap_or(0)
}

fn market_choice_lit(report: &Value) -> bool {
    nested_bool(report, &["market", "present"])
        && (nested_u64(report, &["market", "agent_market_action_txs"]) > 0
            || nested_u64(report, &["market", "market_decision_submitted_count"]) > 0
            || nested_u64(report, &["market", "market_decision_no_trade_count"]) > 0
            || nested_u64(report, &["market", "market_decision_declined_count"]) > 0)
}

fn assert_full_system_lit(binding: &EvidenceBinding, row: &ContractRow) {
    assert!(
        !binding.evidence_run.contains('/') && !binding.evidence_run.contains(".."),
        "binding `{}` evidence_run must be a single evidence root name",
        binding.id
    );
    assert!(
        !binding.evidence_subdir.starts_with('/') && !binding.evidence_subdir.contains(".."),
        "binding `{}` evidence_subdir must be relative inside its run root",
        binding.id
    );

    let run_root = Path::new(TRUE_SUITE_ROOT).join(&binding.evidence_run);
    let subdir = run_root.join(&binding.evidence_subdir);
    assert!(
        run_root.exists(),
        "binding `{}` missing evidence run root: {}",
        binding.id,
        run_root.display()
    );
    assert!(
        subdir.exists(),
        "binding `{}` missing evidence subdir: {}",
        binding.id,
        subdir.display()
    );

    for template in &row.final_evidence_artifacts {
        assert_artifact_reconstructable(binding, template);
    }

    let full_system_template = row
        .final_evidence_artifacts
        .iter()
        .find(|path| path.ends_with("/full_system_participation.json"))
        .unwrap_or_else(|| {
            panic!(
                "contract row `{}` has no full_system_participation.json",
                row.id
            )
        });
    let full_system_path = materialize(full_system_template, &binding.evidence_run);
    assert!(
        full_system_path.starts_with(&subdir),
        "binding `{}` full-system report must live under declared subdir `{}`: {}",
        binding.id,
        subdir.display(),
        full_system_path.display()
    );
    let report: Value = serde_json::from_str(
        &fs::read_to_string(&full_system_path)
            .unwrap_or_else(|err| panic!("read {}: {err}", full_system_path.display())),
    )
    .unwrap_or_else(|err| panic!("parse {}: {err}", full_system_path.display()));

    assert_eq!(
        report.get("schema_version").and_then(Value::as_str),
        Some(FULL_SYSTEM_SCHEMA),
        "binding `{}` has wrong full-system schema",
        binding.id
    );
    assert_eq!(
        report.get("run_id").and_then(Value::as_str),
        Some(binding.evidence_run.as_str()),
        "binding `{}` full-system report run_id must match evidence_run",
        binding.id
    );
    assert_eq!(
        report
            .pointer("/verdict/full_system_participation")
            .and_then(Value::as_bool),
        Some(true),
        "binding `{}` is not full-system participation",
        binding.id
    );
    assert_eq!(
        report
            .pointer("/verdict/full_system_verdict")
            .and_then(Value::as_str),
        Some("FULL_SYSTEM_LIT"),
        "binding `{}` is not FULL_SYSTEM_LIT",
        binding.id
    );
    assert_eq!(
        report
            .pointer("/verdict/missing")
            .and_then(Value::as_array)
            .map(Vec::len),
        Some(0),
        "binding `{}` has missing full-system rows",
        binding.id
    );

    let required_rows = [
        ("FC1", nested_bool(&report, &["fc1", "present"])),
        ("FC2", nested_bool(&report, &["fc2", "present"])),
        (
            "FC3_typed_meta_roles",
            nested_bool(&report, &["fc3", "typed_meta_roles_present"]),
        ),
        (
            "FC3_reinit_semantics",
            nested_bool(&report, &["fc3", "reinit_semantics_present"]),
        ),
        ("market_choice", market_choice_lit(&report)),
        (
            "replay_all_indicators_pass",
            nested_bool(&report, &["replay", "all_indicators_pass"]),
        ),
    ];
    let missing: Vec<_> = required_rows
        .iter()
        .filter_map(|(name, lit)| (!*lit).then_some(*name))
        .collect();
    assert!(
        missing.is_empty(),
        "binding `{}` lacks required full-system rows: {missing:?}",
        binding.id
    );
}

fn assert_bindings_cover_contract(
    binding_key: &str,
    contract_path: &str,
    contract_key: &str,
) -> usize {
    let contracts = contract_rows(contract_path, contract_key);
    let bindings = reconciliation_rows(binding_key);
    let contract_ids: BTreeSet<_> = contracts.keys().cloned().collect();
    let binding_ids: BTreeSet<_> = bindings.iter().map(|binding| binding.id.clone()).collect();
    assert_eq!(
        binding_ids, contract_ids,
        "{binding_key} must exactly cover {contract_path} [[{contract_key}]] rows"
    );

    for binding in &bindings {
        let row = contracts
            .get(&binding.id)
            .unwrap_or_else(|| panic!("binding `{}` missing contract row", binding.id));
        assert_full_system_lit(binding, row);
    }
    bindings.len()
}

#[test]
fn reconciliation_manifest_is_candidate_not_evidence_rewrite() {
    let manifest = parse_toml(RECONCILIATION_MANIFEST);
    assert_eq!(
        manifest.get("schema_version").and_then(toml::Value::as_str),
        Some("turingosv4.true_suite.evidence_reconciliation.v1")
    );
    assert_eq!(
        manifest
            .get("reconciliation_status")
            .and_then(toml::Value::as_str),
        Some(RECONCILIATION_STATUS)
    );
    assert_eq!(
        manifest
            .get("final_closure_claimed")
            .and_then(toml::Value::as_bool),
        Some(false),
        "this gate may compute a candidate; OBL closure still requires audit/witness"
    );
    assert_eq!(
        manifest
            .get("rewrites_historical_evidence")
            .and_then(toml::Value::as_bool),
        Some(false)
    );
    for (path, key) in [
        (REALWORLD_MANIFEST, "final_closure_status"),
        (BROAD_MANIFEST, "closure_status"),
    ] {
        assert_eq!(
            parse_toml(path).get(key).and_then(toml::Value::as_str),
            Some(OPEN_STATUS),
            "{path} must remain non-closing while reconciliation is candidate-only"
        );
    }
}

#[test]
fn reconciliation_covers_every_fresh_domain_and_broad_family_with_lit_evidence() {
    let realworld_count =
        assert_bindings_cover_contract("coverage_task", REALWORLD_MANIFEST, "task");
    let broad_count = assert_bindings_cover_contract("broad_family", BROAD_MANIFEST, "family");
    assert_eq!(realworld_count, 10, "unexpected real-world task count");
    assert_eq!(broad_count, 11, "unexpected broad-family count");
}
