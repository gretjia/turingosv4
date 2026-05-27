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
    "swebench_coding_repair",
    "osworld_computer_use",
    "tdma_proof",
    "fc3_governance_reinit",
    "replay_cas",
    "boot_cli",
];
const REQUIRED_FC_BLOCKS: &[&str] = &["FC1", "FC2", "FC3"];
const REQUIRED_PER_SAMPLE_GROUPS: &[&str] = &[
    "canonical_tape_cas_state",
    "predicate_registry_top_white",
    "fc1_bus_read_view_bridge",
    "economy_market_settlement",
    "runtime_replay_evidence_audit",
    "fc3_runtime_meta_roles",
    "role_economic_learning_sidecars",
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
    tape_kind: String,
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

fn root_str_array(manifest: &toml::Value, key: &str) -> Vec<String> {
    manifest
        .get(key)
        .and_then(toml::Value::as_array)
        .unwrap_or_else(|| panic!("{COVERAGE_MANIFEST} missing root array `{key}`"))
        .iter()
        .map(|v| {
            v.as_str()
                .unwrap_or_else(|| panic!("root array `{key}` contains non-string: {v:?}"))
                .to_string()
        })
        .collect()
}

fn fc_blocks(paths: &[String]) -> BTreeSet<String> {
    paths
        .iter()
        .filter_map(|path| path.split_once(':').map(|(fc, _)| fc.to_string()))
        .collect()
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
                tape_kind: as_string(table, "tape_kind"),
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

fn assert_unique_task_ids(tasks: &[CoverageTask]) {
    let mut seen = BTreeSet::new();
    let mut duplicates = Vec::new();
    for task in tasks {
        if !seen.insert(task.id.clone()) {
            duplicates.push(task.id.clone());
        }
    }
    assert!(
        duplicates.is_empty(),
        "duplicate real-world coverage task ids would shadow liveness accounting: {duplicates:?}"
    );
}

fn assert_path_exists(path: &str) {
    assert!(Path::new(path).exists(), "path does not exist: {path}");
}

fn has_canonical_l4_or_cas_artifact(paths: &[String]) -> bool {
    paths.iter().any(|path| {
        let lower = path.to_ascii_lowercase();
        lower.contains("/cas/")
            || lower.ends_with("/cas")
            || lower.ends_with("cas.dotgit.tar.gz")
            || lower.ends_with("runtime_repo.dotgit.tar.gz")
            || lower.contains("chaintape")
            || lower.contains("/l4/")
            || lower.ends_with("/l4.jsonl")
    })
}

fn has_tdma_domain_artifact(paths: &[String]) -> bool {
    paths
        .iter()
        .any(|path| path.contains("/tdma_tape.git/") || path.ends_with("/tdma_tape.git.tar.gz"))
        && paths
            .iter()
            .any(|path| path.contains("/per_attempt_probes.jsonl"))
        && paths
            .iter()
            .any(|path| path.contains("/tdma_run_manifest.json"))
}

fn has_raw_provider_or_score_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.ends_with(".stdout")
        || lower.ends_with(".stderr")
        || lower.ends_with("_output.txt")
        || lower.contains("raw_prompt")
        || lower.contains("raw_response")
        || lower.contains("leaderboard")
        || lower.contains("accuracy")
        || lower.contains("score_only")
        || lower.contains("old15")
        || lower.contains("old_15")
        || lower.contains("real8x")
        || lower.contains("stage_phase7_real_e2e")
        || lower.contains("tdma_zero_gain_demo")
}

fn assert_allowed_tape_kind(task: &CoverageTask) {
    assert!(
        matches!(task.tape_kind.as_str(), "canonical_l4" | "tdma_domain"),
        "task `{}` has unsupported tape_kind `{}`",
        task.id,
        task.tape_kind
    );
    if task.tape_kind == "tdma_domain" {
        assert!(
            task.anti_contamination_guards
                .iter()
                .any(|guard| guard.contains("must not be mislabeled as bottom-white L4 ChainTape")),
            "tdma_domain task `{}` must explicitly guard against canonical ChainTape conflation",
            task.id
        );
    }
}

fn assert_no_smoke_or_stdout(task_id: &str, path: &str) {
    let lower = path.to_ascii_lowercase();
    assert!(
        !lower.contains("smoke"),
        "real-world coverage task `{task_id}` uses smoke label in artifact path `{path}`"
    );
    assert!(
        !has_raw_provider_or_score_path(path),
        "task `{task_id}` cannot use raw provider output, old/historical candidate evidence, score-only evidence, or raw stdout/stderr as liveness evidence: {path}"
    );
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
    assert_eq!(
        manifest
            .get("full_system_required_for_final")
            .and_then(toml::Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest
            .get("per_sample_fc_union_is_not_sufficient")
            .and_then(toml::Value::as_bool),
        Some(true),
        "different tests may not be unioned to fake one full constitutional run"
    );
    assert_eq!(
        manifest
            .get("market_participation_required_for_every_sample")
            .and_then(toml::Value::as_bool),
        Some(true),
        "market/economy participates even in one-agent runs via invest or tape-visible abstention"
    );
    assert_eq!(
        manifest
            .get("full_system_sample_manifest")
            .and_then(toml::Value::as_str),
        Some("full_system_participation.json")
    );
    let root_fc = root_str_array(&manifest, "required_per_sample_fc_blocks");
    for &fc in REQUIRED_FC_BLOCKS {
        assert!(
            root_fc.iter().any(|item| item == fc),
            "coverage manifest missing required per-sample FC block `{fc}`"
        );
    }
    let root_groups = root_str_array(&manifest, "required_per_sample_module_groups");
    for &group in REQUIRED_PER_SAMPLE_GROUPS {
        assert!(
            root_groups.iter().any(|item| item == group),
            "coverage manifest missing required per-sample module group `{group}`"
        );
    }
    let market_modes = root_str_array(&manifest, "market_participation_modes");
    for mode in ["invest", "abstain_with_tape_visible_market_opportunity"] {
        assert!(
            market_modes.iter().any(|item| item == mode),
            "market participation must support one-agent investment or tape-visible abstention; missing `{mode}`"
        );
    }
}

#[test]
fn realworld_tasks_cover_required_domains_without_smoke_labels() {
    let tasks = coverage_tasks();
    assert_unique_task_ids(&tasks);
    assert!(
        tasks.len() >= 6,
        "final liveness needs broad true-problem tasks, got {}",
        tasks.len()
    );

    let mut domains = BTreeSet::new();
    let mut fresh_domains = BTreeSet::new();
    for task in &tasks {
        assert_allowed_tape_kind(task);
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
        if task.status == "fresh_required" {
            let task_fc = fc_blocks(&task.constitutional_paths);
            for &fc in REQUIRED_FC_BLOCKS {
                assert!(
                    task_fc.contains(fc),
                    "fresh task `{}` must declare {fc} participation for the full-system final contract, got {:?}",
                    task.id,
                    task_fc
                );
            }
            for &group in REQUIRED_PER_SAMPLE_GROUPS {
                assert!(
                    task.module_groups.iter().any(|item| item == group),
                    "fresh task `{}` must declare required per-sample module group `{group}`",
                    task.id
                );
            }
            assert!(
                task.final_evidence_artifacts
                    .iter()
                    .any(|path| path.ends_with("/full_system_participation.json")),
                "fresh task `{}` must require full_system_participation.json before final closure",
                task.id
            );
        }
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
                    task.evidence_artifacts.is_empty(),
                    "fresh-required task `{}` must not attach historical candidate artifacts to the final coverage contract",
                    task.id
                );
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
    assert_unique_task_ids(&tasks);
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
        let has_expected_tape = match task.tape_kind.as_str() {
            "canonical_l4" => has_canonical_l4_or_cas_artifact(&task.final_evidence_artifacts),
            "tdma_domain" => has_tdma_domain_artifact(&task.final_evidence_artifacts),
            other => panic!("unknown tape_kind `{other}` in `{}`", task.id),
        };
        let has_replay_or_verifier =
            has_replay_or_verifier_artifact(&task.final_evidence_artifacts);
        assert!(
            has_expected_tape && has_replay_or_verifier,
            "task `{}` final evidence must include the declared tape_kind `{}` plus replay/verifier output; runtime_repo alone is supplemental, got {:?}",
            task.id,
            task.tape_kind,
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
