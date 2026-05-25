//! Real-world no-zombie coverage contract.
//!
//! `constitution_production_module_liveness` proves every production module
//! group is accounted for. This gate is the next layer: every retained group
//! must map to a true-problem suite, while historical evidence remains only a
//! candidate until a fresh ChainTape/CAS run lights the current kernel.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const PRODUCTION_MANIFEST: &str = "tests/fixtures/liveness/production_module_liveness.toml";
const COVERAGE_MANIFEST: &str = "tests/fixtures/liveness/realworld_liveness_coverage.toml";
const OPEN_CLOSURE_STATUS: &str = "OPEN_REAL_WORLD_COVERAGE_PENDING";
const REQUIRED_DOMAINS: &[&str] = &[
    "market_economy",
    "generate_artifact",
    "tdma_proof",
    "fc3_governance_reinit",
    "replay_cas",
    "boot_cli",
];

#[derive(Debug)]
struct ProductionGroup {
    id: String,
    classification: String,
    status: String,
}

#[derive(Debug)]
struct CoverageTask {
    id: String,
    problem_type: String,
    status: String,
    entrypoint: String,
    module_groups: Vec<String>,
    constitutional_paths: Vec<String>,
    anti_contamination_guards: Vec<String>,
    evidence_artifacts: Vec<String>,
    final_evidence_artifacts: Vec<String>,
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
        .map(|v| {
            v.as_str()
                .unwrap_or_else(|| panic!("array `{key}` contains non-string: {v:?}"))
                .to_string()
        })
        .collect()
}

fn production_groups() -> Vec<ProductionGroup> {
    parse_toml(PRODUCTION_MANIFEST)
        .get("group")
        .and_then(toml::Value::as_array)
        .unwrap_or_else(|| panic!("{PRODUCTION_MANIFEST} missing [[group]] rows"))
        .iter()
        .map(|row| {
            let table = row
                .as_table()
                .unwrap_or_else(|| panic!("group row is not a table: {row:?}"));
            ProductionGroup {
                id: as_string(table, "id"),
                classification: as_string(table, "classification"),
                status: as_string(table, "status"),
            }
        })
        .collect()
}

fn coverage_tasks() -> Vec<CoverageTask> {
    parse_toml(COVERAGE_MANIFEST)
        .get("task")
        .and_then(toml::Value::as_array)
        .unwrap_or_else(|| panic!("{COVERAGE_MANIFEST} missing [[task]] rows"))
        .iter()
        .map(|row| {
            let table = row
                .as_table()
                .unwrap_or_else(|| panic!("task row is not a table: {row:?}"));
            CoverageTask {
                id: as_string(table, "id"),
                problem_type: as_string(table, "problem_type"),
                status: as_string(table, "status"),
                entrypoint: as_string(table, "entrypoint"),
                module_groups: as_str_array(table, "module_groups"),
                constitutional_paths: as_str_array(table, "constitutional_paths"),
                anti_contamination_guards: as_str_array(table, "anti_contamination_guards"),
                evidence_artifacts: as_str_array(table, "evidence_artifacts"),
                final_evidence_artifacts: as_str_array(table, "final_evidence_artifacts"),
            }
        })
        .collect()
}

fn assert_path_exists(path: &str) {
    assert!(Path::new(path).exists(), "path does not exist: {path}");
}

fn assert_no_smoke_or_stdout(task_id: &str, path: &str) {
    let lower = path.to_ascii_lowercase();
    assert!(
        !lower.contains("smoke"),
        "real-world coverage task `{task_id}` uses smoke label in artifact path `{path}`"
    );
    assert!(
        !lower.ends_with(".stdout") && !lower.ends_with(".stderr"),
        "task `{task_id}` cannot use raw stdout/stderr as liveness evidence: {path}"
    );
}

fn has_tape_artifact(paths: &[String]) -> bool {
    paths.iter().any(|path| {
        let lower = path.to_ascii_lowercase();
        lower.contains("chaintape") || lower.contains("/l4/") || lower.ends_with("/l4.jsonl")
    })
}

fn has_cas_artifact(paths: &[String]) -> bool {
    paths.iter().any(|path| {
        let lower = path.to_ascii_lowercase();
        lower.contains("/cas/") || lower.ends_with("/cas")
    })
}

fn has_replay_or_verifier_artifact(paths: &[String]) -> bool {
    paths.iter().any(|path| {
        let lower = path.to_ascii_lowercase();
        lower.contains("replay")
            || lower.contains("verify")
            || lower.contains("verifier")
            || lower.contains("aggregate_verdict")
            || lower.contains("tamper_report")
    })
}

#[test]
fn realworld_coverage_policy_requires_fresh_current_evidence() {
    let manifest = parse_toml(COVERAGE_MANIFEST);
    assert_eq!(
        manifest.get("schema_version").and_then(toml::Value::as_str),
        Some("turingosv4.realworld_liveness_coverage.v1")
    );
    assert_eq!(
        manifest.get("authority").and_then(toml::Value::as_str),
        Some("constitution.md + fresh ChainTape/CAS real-world evidence")
    );
    assert_eq!(
        manifest
            .get("historical_evidence_is_final")
            .and_then(toml::Value::as_bool),
        Some(false),
        "historical pre-closure runs cannot close the final no-zombie claim"
    );
    assert_eq!(
        manifest
            .get("final_closure_status")
            .and_then(toml::Value::as_str),
        Some(OPEN_CLOSURE_STATUS),
        "the suite contract may define work, but must remain explicitly open until fresh real-world evidence closes every domain"
    );
    assert_eq!(
        manifest
            .get("smoke_is_not_final_evidence")
            .and_then(toml::Value::as_bool),
        Some(true)
    );
}

#[test]
fn realworld_tasks_cover_required_domains_without_smoke_labels() {
    let tasks = coverage_tasks();
    assert!(
        tasks.len() >= 6,
        "final liveness needs broad true-problem tasks, got {}",
        tasks.len()
    );

    let mut domains = BTreeSet::new();
    let mut fresh_domains = BTreeSet::new();
    for task in &tasks {
        for field in [&task.id, &task.problem_type, &task.entrypoint] {
            assert!(
                !field.to_ascii_lowercase().contains("smoke"),
                "real-world coverage task `{}` uses smoke label in `{field}`",
                task.id
            );
        }
        for path in task
            .evidence_artifacts
            .iter()
            .chain(task.final_evidence_artifacts.iter())
        {
            assert_no_smoke_or_stdout(&task.id, path);
        }
        assert_path_exists(&task.entrypoint);
        assert!(
            !task.module_groups.is_empty()
                && !task.constitutional_paths.is_empty()
                && !task.anti_contamination_guards.is_empty(),
            "task `{}` must bind groups, FC paths, and anti-contamination guards",
            task.id
        );
        match task.status.as_str() {
            "historical_candidate" => {
                assert!(
                    !task.evidence_artifacts.is_empty(),
                    "historical task `{}` needs existing evidence artifacts",
                    task.id
                );
                for path in &task.evidence_artifacts {
                    assert_path_exists(path);
                }
            }
            "fresh_required" => {
                fresh_domains.insert(task.problem_type.as_str());
                assert!(
                    !task.final_evidence_artifacts.is_empty(),
                    "fresh-required task `{}` must declare final artifacts",
                    task.id
                );
                for path in &task.evidence_artifacts {
                    assert_path_exists(path);
                }
            }
            other => panic!("unknown task status `{other}` in `{}`", task.id),
        }
        domains.insert(task.problem_type.as_str());
    }

    for required in REQUIRED_DOMAINS {
        assert!(
            domains.contains(required),
            "missing required real-world problem domain `{required}`"
        );
        assert!(
            fresh_domains.contains(required),
            "domain `{required}` must have fresh-required current-kernel coverage, not only historical candidate evidence"
        );
    }
}

#[test]
fn every_retained_candidate_group_maps_to_realworld_task() {
    let production = production_groups();
    let tasks = coverage_tasks();
    let mut coverage: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut fresh_coverage: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for task in &tasks {
        for group in &task.module_groups {
            coverage
                .entry(group.clone())
                .or_default()
                .push(task.id.clone());
            if task.status == "fresh_required" {
                fresh_coverage
                    .entry(group.clone())
                    .or_default()
                    .push(task.id.clone());
            }
        }
    }

    let mut missing = Vec::new();
    let mut missing_fresh = Vec::new();
    for group in &production {
        if group.status == "historical_real_world_candidate" {
            if !coverage.contains_key(&group.id) {
                missing.push(group.id.clone());
            }
            if !fresh_coverage.contains_key(&group.id) {
                missing_fresh.push(group.id.clone());
            }
        }
        if group.classification == "legacy_quarantine" || group.classification == "dev_only" {
            assert!(
                !coverage.contains_key(&group.id),
                "quarantined/dev group `{}` must not be treated as AGI production coverage",
                group.id
            );
        }
    }
    assert!(
        missing.is_empty(),
        "retained candidate groups missing real-world task coverage: {missing:?}"
    );
    assert!(
        missing_fresh.is_empty(),
        "retained candidate groups missing fresh-required current-kernel coverage: {missing_fresh:?}"
    );

    let known_groups: BTreeSet<_> = production.iter().map(|g| g.id.as_str()).collect();
    for group_id in coverage.keys() {
        assert!(
            known_groups.contains(group_id.as_str()),
            "coverage manifest references unknown production group `{group_id}`"
        );
    }
}

#[test]
fn final_evidence_shape_is_tape_cas_or_quarantine_not_stdout() {
    for task in coverage_tasks() {
        let has_tape_or_cas = has_tape_artifact(&task.final_evidence_artifacts)
            || has_cas_artifact(&task.final_evidence_artifacts);
        let has_replay_or_verifier =
            has_replay_or_verifier_artifact(&task.final_evidence_artifacts);
        assert!(
            has_tape_or_cas && has_replay_or_verifier,
            "task `{}` final evidence must include ChainTape or CAS plus replay/verifier output; runtime_repo alone is supplemental, got {:?}",
            task.id,
            task.final_evidence_artifacts
        );
        for path in task
            .evidence_artifacts
            .iter()
            .chain(task.final_evidence_artifacts.iter())
        {
            assert!(
                !path.ends_with(".stdout") && !path.ends_with(".stderr"),
                "task `{}` cannot use raw stdout/stderr as final liveness evidence: {path}",
                task.id
            );
        }
    }
}
