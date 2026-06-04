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
const REAUDIT_STATUS: &str = "OBL005_REAUDIT_IN_PROGRESS";

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
    blockers: Vec<String>,
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
                blockers: as_str_array(table, "blockers"),
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

fn is_raw_observation_template(template: &str) -> bool {
    let lower = template.to_ascii_lowercase();
    lower.contains("/browser_traces")
        || lower.contains("/dom_log")
        || lower.contains("/dom_logs")
        || lower.contains("/screenshots")
        || lower.contains("screenshot")
        || lower.ends_with(".html")
        || lower.ends_with(".htm")
        || lower.ends_with(".png")
        || lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
}

fn has_nonempty_string_at(value: &Value, pointer: &str) -> bool {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .map(|text| !text.trim().is_empty())
        .unwrap_or(false)
}

fn is_git_commit_hex(text: &str) -> bool {
    let text = text.trim();
    text.len() == 40 && text.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn source_tree_fingerprint_present(report: &Value) -> bool {
    [
        "/source_tree/commit",
        "/source_tree/head_commit",
        "/source_tree/git_commit",
        "/source_tree/source_commit",
        "/source/source_commit",
        "/source/turingos_commit",
        "/source_commit",
        "/turingos_commit",
        "/workspace/source_commit",
        "/workspace/git_commit",
    ]
    .iter()
    .any(|pointer| {
        report
            .pointer(pointer)
            .and_then(Value::as_str)
            .map(is_git_commit_hex)
            .unwrap_or(false)
    })
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

fn is_replay_report_path(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.to_ascii_lowercase().ends_with("replay_report.json"))
        .unwrap_or(false)
}

fn assert_replay_report_green(binding: &EvidenceBinding, path: &Path) {
    let report: Value = serde_json::from_str(
        &fs::read_to_string(path).unwrap_or_else(|err| panic!("read {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse {}: {err}", path.display()));

    if report.get("schema_version").and_then(Value::as_str)
        == Some("turingosv4.true_suite.tdma_replay_report.v1")
    {
        assert_eq!(
            report.get("ok").and_then(Value::as_bool),
            Some(true),
            "binding `{}` TDMA replay report is not ok: {}",
            binding.id,
            path.display()
        );
        let checks = report
            .get("checks")
            .and_then(Value::as_object)
            .unwrap_or_else(|| {
                panic!(
                    "binding `{}` TDMA replay report has no checks object: {}",
                    binding.id,
                    path.display()
                )
            });
        assert!(
            !checks.is_empty(),
            "binding `{}` TDMA replay report has empty checks object: {}",
            binding.id,
            path.display()
        );
        for (name, value) in checks {
            assert_eq!(
                value.as_bool(),
                Some(true),
                "binding `{}` TDMA replay check `{name}` is not true: {}",
                binding.id,
                path.display()
            );
        }
        assert_eq!(
            report.get("stages_completed").and_then(Value::as_u64),
            report.get("stages_total").and_then(Value::as_u64),
            "binding `{}` TDMA replay did not complete all stages: {}",
            binding.id,
            path.display()
        );
        return;
    }

    for key in [
        "ledger_root_verified",
        "system_signatures_verified",
        "state_reconstructed",
        "economic_state_reconstructed",
        "cas_payloads_retrievable",
        "agent_signatures_verified",
        "proposal_telemetry_cas_retrievable",
    ] {
        assert!(
            nested_bool(&report, &[key]),
            "binding `{}` replay report `{key}` is not true: {}",
            binding.id,
            path.display()
        );
    }
    assert!(
        report
            .get("replay_failure")
            .map(Value::is_null)
            .unwrap_or(true),
        "binding `{}` replay report has replay_failure: {}",
        binding.id,
        path.display()
    );
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
    if is_replay_report_path(&path) {
        assert_replay_report_green(binding, &path);
    }
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

fn assert_raw_observation_templates_are_cas_bound(
    binding: &EvidenceBinding,
    row: &ContractRow,
    report: &Value,
) {
    let raw_templates: Vec<_> = row
        .final_evidence_artifacts
        .iter()
        .filter(|template| is_raw_observation_template(template))
        .collect();
    if raw_templates.is_empty() {
        return;
    }

    assert!(
        nested_bool(report, &["replay", "cas_payloads_retrievable"]),
        "binding `{}` cites raw observation artifacts but replay does not prove CAS payload retrievability: {:?}",
        binding.id,
        raw_templates
    );
    assert!(
        has_nonempty_string_at(report, "/evidence_paths/cas")
            || has_nonempty_string_at(report, "/domain_manifest/cas"),
        "binding `{}` cites raw observation artifacts but the full-system receipt does not bind a CAS evidence path: {:?}",
        binding.id,
        raw_templates
    );

    let cid_pointers = [
        "/domain_manifest/observation_capsule_cid",
        "/domain_manifest/browser_action_trace_cid",
        "/domain_manifest/snapshot_capsule_cid",
        "/domain_manifest/sandbox_trace_cid",
    ];
    assert!(
        cid_pointers
            .iter()
            .any(|pointer| has_nonempty_string_at(report, pointer)),
        "binding `{}` cites raw observation artifacts but the domain manifest has no observation/trace CAS CID: {:?}",
        binding.id,
        raw_templates
    );
}

fn full_system_template(row: &ContractRow) -> &str {
    row.final_evidence_artifacts
        .iter()
        .find(|path| path.ends_with("/full_system_participation.json"))
        .map(String::as_str)
        .unwrap_or_else(|| {
            panic!(
                "contract row `{}` has no full_system_participation.json",
                row.id
            )
        })
}

fn full_system_report_path(binding: &EvidenceBinding, row: &ContractRow) -> PathBuf {
    materialize(full_system_template(row), &binding.evidence_run)
}

fn read_full_system_report(binding: &EvidenceBinding, row: &ContractRow) -> Value {
    let full_system_path = full_system_report_path(binding, row);
    serde_json::from_str(
        &fs::read_to_string(&full_system_path)
            .unwrap_or_else(|err| panic!("read {}: {err}", full_system_path.display())),
    )
    .unwrap_or_else(|err| panic!("parse {}: {err}", full_system_path.display()))
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

fn direction_is_no_or_short(text: &str) -> bool {
    matches!(
        text.to_ascii_lowercase().as_str(),
        "no" | "buy_no" | "buyno" | "long_no" | "short"
    )
}

fn value_has_no_or_short_market_side(value: &Value) -> bool {
    match value {
        Value::Object(map) => map.iter().any(|(key, child)| {
            let key = key.to_ascii_lowercase();
            let direction_key = matches!(
                key.as_str(),
                "direction"
                    | "side"
                    | "buy_direction"
                    | "l4_direction"
                    | "market_decision_direction"
            );
            if direction_key
                && child
                    .as_str()
                    .map(direction_is_no_or_short)
                    .unwrap_or(false)
            {
                return true;
            }
            if matches!(key.as_str(), "buy_no_count" | "no_side_market_action_txs")
                && child.as_u64().unwrap_or(0) > 0
            {
                return true;
            }
            if key.contains("short")
                && (child.as_u64().unwrap_or(0) > 0 || child.as_bool() == Some(true))
            {
                return true;
            }
            value_has_no_or_short_market_side(child)
        }),
        Value::Array(items) => items.iter().any(value_has_no_or_short_market_side),
        _ => false,
    }
}

fn binding_has_no_or_short_market_side(
    binding: &EvidenceBinding,
    row: &ContractRow,
    report: &Value,
) -> bool {
    if value_has_no_or_short_market_side(report) {
        return true;
    }

    row.final_evidence_artifacts.iter().any(|template| {
        let path = materialize(template, &binding.evidence_run);
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") || !path.exists() {
            return false;
        }
        let Ok(raw) = fs::read_to_string(&path) else {
            return false;
        };
        let Ok(value) = serde_json::from_str::<Value>(&raw) else {
            return false;
        };
        value_has_no_or_short_market_side(&value)
    })
}

fn is_market_binding(binding: &EvidenceBinding, row: &ContractRow) -> bool {
    binding.id.contains("market")
        || row
            .final_evidence_artifacts
            .iter()
            .any(|template| template.contains("/market"))
}

fn value_has_benchmark_failure(value: &Value) -> bool {
    match value {
        Value::Object(map) => map.iter().any(|(key, child)| {
            let key = key.to_ascii_lowercase();
            if matches!(
                key.as_str(),
                "answer_correct"
                    | "exact_match"
                    | "action_match"
                    | "safe_action_match"
                    | "selected_candidate_available"
            ) && child.as_bool() == Some(false)
            {
                return true;
            }
            if key == "benchmark_verdict" {
                let verdict = child.as_str().unwrap_or_default().to_ascii_lowercase();
                if verdict.contains("mismatch")
                    || verdict.contains("incorrect")
                    || verdict.contains("plausible")
                {
                    return true;
                }
            }
            value_has_benchmark_failure(child)
        }),
        Value::Array(items) => items.iter().any(value_has_benchmark_failure),
        _ => false,
    }
}

fn allowed_blocker_classes() -> BTreeSet<&'static str> {
    BTreeSet::from([
        "source_receipt_final_closure_false",
        "domain_receipt_final_closure_false",
        "domain_receipt_final_closure_missing",
        "benchmark_capability_not_solved",
        "market_no_or_short_side_missing",
        "source_tree_fingerprint_missing",
        "fresh_final_closure_witness_missing",
    ])
}

fn assert_blocker_classes_are_receipt_derived(
    binding_key: &str,
    binding: &EvidenceBinding,
    row: &ContractRow,
    report: &Value,
    closure_witness_missing: bool,
) {
    let allowed = allowed_blocker_classes();
    let blockers: BTreeSet<_> = binding.blockers.iter().map(String::as_str).collect();
    assert_eq!(
        blockers.len(),
        binding.blockers.len(),
        "{binding_key}:{} has duplicate closure blocker classes",
        binding.id
    );
    for blocker in &blockers {
        assert!(
            allowed.contains(blocker),
            "{binding_key}:{} declares unknown closure blocker `{blocker}`",
            binding.id
        );
    }

    let source_final_false = report
        .pointer("/verdict/final_closure_possible")
        .and_then(Value::as_bool)
        == Some(false);
    let domain_final_false = report
        .pointer("/domain_manifest/final_closure_possible")
        .and_then(Value::as_bool)
        == Some(false);
    let domain_final_missing = report
        .get("domain_manifest")
        .and_then(Value::as_object)
        .is_some()
        && report
            .pointer("/domain_manifest/final_closure_possible")
            .and_then(Value::as_bool)
            .is_none();
    let market_no_or_short_missing = is_market_binding(binding, row)
        && !binding_has_no_or_short_market_side(binding, row, report);
    let benchmark_capability_failed = value_has_benchmark_failure(report);
    let source_tree_fingerprint_missing = !source_tree_fingerprint_present(report);

    if blockers.contains("source_receipt_final_closure_false") {
        assert!(
            source_final_false,
            "{binding_key}:{} claims source receipt is non-closing, but report is not false",
            binding.id
        );
    }
    if blockers.contains("domain_receipt_final_closure_false") {
        assert!(
            domain_final_false,
            "{binding_key}:{} claims domain manifest is non-closing, but report does not prove that",
            binding.id
        );
    }
    if blockers.contains("domain_receipt_final_closure_missing") {
        assert!(
            domain_final_missing,
            "{binding_key}:{} claims domain manifest closure status is missing, but the receipt does not prove that",
            binding.id
        );
    }
    if blockers.contains("market_no_or_short_side_missing") {
        assert!(
            market_no_or_short_missing,
            "{binding_key}:{} claims missing NO/short market side, but evidence already contains one or is not a market binding",
            binding.id
        );
    }
    if blockers.contains("benchmark_capability_not_solved") {
        assert!(
            benchmark_capability_failed,
            "{binding_key}:{} claims benchmark capability failure, but receipt has no failing capability marker",
            binding.id
        );
    }
    if blockers.contains("source_tree_fingerprint_missing") {
        assert!(
            source_tree_fingerprint_missing,
            "{binding_key}:{} claims missing source-tree fingerprint, but the receipt already carries one",
            binding.id
        );
    }
    if blockers.contains("fresh_final_closure_witness_missing") {
        assert!(
            closure_witness_missing,
            "{binding_key}:{} claims fresh final closure witness is missing after closure was claimed",
            binding.id
        );
    }

    if source_final_false {
        assert!(
            blockers.contains("source_receipt_final_closure_false"),
            "{binding_key}:{} is non-closing but does not declare source_receipt_final_closure_false",
            binding.id
        );
    }
    if domain_final_false {
        assert!(
            blockers.contains("domain_receipt_final_closure_false"),
            "{binding_key}:{} has a non-closing domain manifest but does not declare it",
            binding.id
        );
    }
    if domain_final_missing {
        assert!(
            blockers.contains("domain_receipt_final_closure_missing"),
            "{binding_key}:{} has a domain manifest without final_closure_possible but does not declare it",
            binding.id
        );
    }
    if market_no_or_short_missing {
        assert!(
            blockers.contains("market_no_or_short_side_missing"),
            "{binding_key}:{} lacks NO/short-side market evidence but does not declare it",
            binding.id
        );
    }
    if benchmark_capability_failed {
        assert!(
            blockers.contains("benchmark_capability_not_solved"),
            "{binding_key}:{} has benchmark capability failure markers but does not declare them",
            binding.id
        );
    }
    if source_tree_fingerprint_missing {
        assert!(
            blockers.contains("source_tree_fingerprint_missing"),
            "{binding_key}:{} has no current source-tree fingerprint but does not declare it",
            binding.id
        );
    }
    if closure_witness_missing && source_final_false {
        assert!(
            blockers.contains("fresh_final_closure_witness_missing"),
            "{binding_key}:{} is non-closing during reaudit but does not declare missing final witness",
            binding.id
        );
    }
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

    let full_system_path = full_system_report_path(binding, row);
    assert!(
        full_system_path.starts_with(&subdir),
        "binding `{}` full-system report must live under declared subdir `{}`: {}",
        binding.id,
        subdir.display(),
        full_system_path.display()
    );
    let report = read_full_system_report(binding, row);

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
    assert_raw_observation_templates_are_cas_bound(binding, row, &report);
}

#[test]
fn no_or_short_market_side_detector_is_structural() {
    let buy_no = serde_json::json!({
        "market": {"present": true, "buy_no_count": 1},
        "domain_manifest": {"direction": "yes"}
    });
    let short_alias = serde_json::json!({
        "domain_manifest": {"market_decision_direction": "short"}
    });
    let yes_only = serde_json::json!({
        "market": {
            "present": true,
            "agent_market_action_txs": 2,
            "market_decision_no_trade_count": 0
        },
        "domain_manifest": {"direction": "yes"}
    });

    assert!(value_has_no_or_short_market_side(&buy_no));
    assert!(value_has_no_or_short_market_side(&short_alias));
    assert!(
        !value_has_no_or_short_market_side(&yes_only),
        "YES-only market activity must not satisfy the NO/short closure detector"
    );
}

#[test]
fn replay_report_artifacts_must_be_machine_green_not_just_present() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let path = tmp.path().join("restore_replay_report.json");
    fs::write(
        &path,
        serde_json::json!({
            "ledger_root_verified": false,
            "system_signatures_verified": true,
            "state_reconstructed": true,
            "economic_state_reconstructed": true,
            "cas_payloads_retrievable": true,
            "agent_signatures_verified": true,
            "proposal_telemetry_cas_retrievable": true,
            "replay_failure": null
        })
        .to_string(),
    )
    .expect("write bad replay report");

    let binding = EvidenceBinding {
        id: "synthetic_bad_replay".into(),
        evidence_run: "synthetic".into(),
        evidence_subdir: "synthetic".into(),
        blockers: Vec::new(),
    };
    let result = std::panic::catch_unwind(|| {
        assert_artifact_reconstructable(&binding, path.to_str().expect("utf8 path"));
    });
    assert!(
        result.is_err(),
        "replay/restore artifacts with false verifier booleans must not pass by mere file existence"
    );
}

#[test]
fn domain_manifest_missing_closure_status_must_be_explicit_blocker() {
    let binding = EvidenceBinding {
        id: "synthetic_missing_domain_closure".into(),
        evidence_run: "synthetic".into(),
        evidence_subdir: "synthetic".into(),
        blockers: vec!["source_receipt_final_closure_false".into()],
    };
    let row = ContractRow {
        id: binding.id.clone(),
        final_evidence_artifacts: Vec::new(),
    };
    let report = serde_json::json!({
        "verdict": {"final_closure_possible": false},
        "domain_manifest": {"benchmark_verdict": "correct_with_rationale"}
    });

    let result = std::panic::catch_unwind(|| {
        assert_blocker_classes_are_receipt_derived("synthetic", &binding, &row, &report, false);
    });
    assert!(
        result.is_err(),
        "domain manifests without final_closure_possible must not pass blocker reconciliation silently"
    );
}

#[test]
fn missing_source_tree_fingerprint_must_be_explicit_blocker() {
    let binding = EvidenceBinding {
        id: "synthetic_missing_source_tree_fingerprint".into(),
        evidence_run: "synthetic".into(),
        evidence_subdir: "synthetic".into(),
        blockers: vec!["source_receipt_final_closure_false".into()],
    };
    let row = ContractRow {
        id: binding.id.clone(),
        final_evidence_artifacts: Vec::new(),
    };
    let report = serde_json::json!({
        "verdict": {"final_closure_possible": false},
        "replay": {"head_commit_oid_hex": "1111111111111111111111111111111111111111"}
    });

    let result = std::panic::catch_unwind(|| {
        assert_blocker_classes_are_receipt_derived("synthetic", &binding, &row, &report, false);
    });
    assert!(
        result.is_err(),
        "full-system receipts without a source-tree fingerprint must not pass blocker reconciliation silently"
    );
}

#[test]
fn source_tree_fingerprint_detector_rejects_replay_head_as_source_proof() {
    let replay_only = serde_json::json!({
        "replay": {"head_commit_oid_hex": "1111111111111111111111111111111111111111"}
    });
    let source_commit = serde_json::json!({
        "source_tree": {"commit": "2222222222222222222222222222222222222222"}
    });

    assert!(
        !source_tree_fingerprint_present(&replay_only),
        "runtime replay HEAD is not a source-tree fingerprint"
    );
    assert!(source_tree_fingerprint_present(&source_commit));
}

#[test]
fn final_closure_claim_requires_all_bound_receipts_to_be_closing_receipts() {
    let manifest = parse_toml(RECONCILIATION_MANIFEST);
    let final_closure_claimed = manifest
        .get("final_closure_claimed")
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);

    let mut non_closing_receipts = Vec::new();
    let mut closure_receipts_without_source_tree = Vec::new();
    for (binding_key, contract_path, contract_key) in [
        ("coverage_task", REALWORLD_MANIFEST, "task"),
        ("broad_family", BROAD_MANIFEST, "family"),
    ] {
        let contracts = contract_rows(contract_path, contract_key);
        for binding in reconciliation_rows(binding_key) {
            let row = contracts
                .get(&binding.id)
                .unwrap_or_else(|| panic!("binding `{}` missing contract row", binding.id));
            let report = read_full_system_report(&binding, row);
            let closure_possible = report
                .pointer("/verdict/final_closure_possible")
                .and_then(Value::as_bool)
                == Some(true);
            if !closure_possible {
                non_closing_receipts.push(format!(
                    "{binding_key}:{}:{}",
                    binding.id,
                    full_system_report_path(&binding, row).display()
                ));
            } else if !source_tree_fingerprint_present(&report) {
                closure_receipts_without_source_tree.push(format!(
                    "{binding_key}:{}:{}",
                    binding.id,
                    full_system_report_path(&binding, row).display()
                ));
            }
        }
    }

    if final_closure_claimed {
        assert!(
            non_closing_receipts.is_empty(),
            "final_closure_claimed=true cannot cite non-closing receipts: {non_closing_receipts:?}"
        );
        assert!(
            closure_receipts_without_source_tree.is_empty(),
            "final_closure_claimed=true cannot cite receipts without source-tree fingerprints: {closure_receipts_without_source_tree:?}"
        );
    } else {
        assert!(
            !non_closing_receipts.is_empty(),
            "REAUDIT manifests must keep final_closure_claimed=false until closure-capable receipts replace the non-closing bindings"
        );
    }
}

#[test]
fn non_closing_bound_receipts_have_receipt_derived_blockers() {
    let manifest = parse_toml(RECONCILIATION_MANIFEST);
    let closure_witness_missing = manifest
        .get("reconciliation_status")
        .and_then(toml::Value::as_str)
        == Some(REAUDIT_STATUS)
        && manifest
            .get("final_closure_claimed")
            .and_then(toml::Value::as_bool)
            == Some(false);

    let mut non_closing_bound_receipts = 0usize;
    for (binding_key, contract_path, contract_key) in [
        ("coverage_task", REALWORLD_MANIFEST, "task"),
        ("broad_family", BROAD_MANIFEST, "family"),
    ] {
        let contracts = contract_rows(contract_path, contract_key);
        for binding in reconciliation_rows(binding_key) {
            let row = contracts
                .get(&binding.id)
                .unwrap_or_else(|| panic!("binding `{}` missing contract row", binding.id));
            let report = read_full_system_report(&binding, row);
            let final_closure_possible = report
                .pointer("/verdict/final_closure_possible")
                .and_then(Value::as_bool)
                == Some(true);
            if final_closure_possible {
                assert!(
                    binding.blockers.is_empty(),
                    "{binding_key}:{} is closing but still carries blockers: {:?}",
                    binding.id,
                    binding.blockers
                );
                continue;
            }
            non_closing_bound_receipts += 1;
            assert!(
                !binding.blockers.is_empty(),
                "{binding_key}:{} is non-closing but has no blocker inventory",
                binding.id
            );
            assert_blocker_classes_are_receipt_derived(
                binding_key,
                &binding,
                row,
                &report,
                closure_witness_missing,
            );
        }
    }

    assert!(
        non_closing_bound_receipts > 0,
        "this reaudit gate must be able to observe at least one non-closing receipt"
    );
}

#[test]
fn final_closure_claim_requires_market_no_or_short_side_evidence() {
    let manifest = parse_toml(RECONCILIATION_MANIFEST);
    let final_closure_claimed = manifest
        .get("final_closure_claimed")
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);

    let mut market_bindings = Vec::new();
    let mut missing_no_or_short = Vec::new();
    for (binding_key, contract_path, contract_key) in [
        ("coverage_task", REALWORLD_MANIFEST, "task"),
        ("broad_family", BROAD_MANIFEST, "family"),
    ] {
        let contracts = contract_rows(contract_path, contract_key);
        for binding in reconciliation_rows(binding_key) {
            let row = contracts
                .get(&binding.id)
                .unwrap_or_else(|| panic!("binding `{}` missing contract row", binding.id));
            if !is_market_binding(&binding, row) {
                continue;
            }
            market_bindings.push(format!("{binding_key}:{}", binding.id));
            let report = read_full_system_report(&binding, row);
            if !binding_has_no_or_short_market_side(&binding, row, &report) {
                missing_no_or_short.push(format!(
                    "{binding_key}:{}:{}",
                    binding.id,
                    full_system_report_path(&binding, row).display()
                ));
            }
        }
    }

    assert!(
        !market_bindings.is_empty(),
        "the reconciliation guard must inspect at least one market/economy binding"
    );
    if final_closure_claimed {
        assert!(
            missing_no_or_short.is_empty(),
            "final market/economy closure cannot cite YES-only market receipts: {missing_no_or_short:?}"
        );
    }
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
fn reconciliation_manifest_is_reaudit_candidate_no_evidence_rewrite() {
    let manifest = parse_toml(RECONCILIATION_MANIFEST);
    assert_eq!(
        manifest.get("schema_version").and_then(toml::Value::as_str),
        Some("turingosv4.true_suite.evidence_reconciliation.v1")
    );
    assert_eq!(
        manifest
            .get("reconciliation_status")
            .and_then(toml::Value::as_str),
        Some(REAUDIT_STATUS)
    );
    assert_eq!(
        manifest
            .get("final_closure_claimed")
            .and_then(toml::Value::as_bool),
        Some(false),
        "current OBL-005 final closure must not be claimed while production/script liveness inventories remain in reaudit"
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
            Some(REAUDIT_STATUS),
            "{path} must stay in OBL005_REAUDIT_IN_PROGRESS until a fresh current-tree final closure witness exists"
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
