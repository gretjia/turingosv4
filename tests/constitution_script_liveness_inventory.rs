//! Script liveness inventory for OBL-005 no-zombie accounting.
//!
//! The Rust module/bin inventory is not enough: retained scripts can be
//! production entrypoints, evidence packaging tools, CI gates, or historical
//! smoke helpers. This gate makes that accounting explicit.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const MANIFEST_PATH: &str = "tests/fixtures/liveness/script_liveness_inventory.toml";
const REALWORLD_MANIFEST: &str = "tests/fixtures/liveness/realworld_liveness_coverage.toml";
const BROAD_MANIFEST: &str = "tests/fixtures/liveness/broad_agi_true_suite_manifest.toml";
const AUTOMATION_ROOTS: &[&str] = &[
    "scripts",
    "tools",
    "rules",
    ".claude/hooks",
    ".github/workflows",
];

#[derive(Debug)]
struct ScriptGroup {
    id: String,
    classification: String,
    status: String,
    paths: Vec<String>,
    covered_by: Vec<String>,
    realworld_task_ids: Vec<String>,
    broad_family_ids: Vec<String>,
    counts_for_obl005_script_closure: bool,
}

fn parse_toml(path: &str) -> toml::Value {
    let raw = fs::read_to_string(path).unwrap_or_else(|err| panic!("read {path}: {err}"));
    toml::from_str(&raw).unwrap_or_else(|err| panic!("parse {path}: {err}"))
}

fn as_string(table: &toml::value::Table, key: &str) -> String {
    table
        .get(key)
        .and_then(toml::Value::as_str)
        .unwrap_or_else(|| panic!("script group missing string `{key}`: {table:?}"))
        .to_string()
}

fn as_bool(table: &toml::value::Table, key: &str) -> bool {
    table
        .get(key)
        .and_then(toml::Value::as_bool)
        .unwrap_or_else(|| panic!("script group missing bool `{key}`: {table:?}"))
}

fn str_array(table: &toml::value::Table, key: &str) -> Vec<String> {
    table
        .get(key)
        .and_then(toml::Value::as_array)
        .unwrap_or_else(|| panic!("script group missing array `{key}`: {table:?}"))
        .iter()
        .map(|item| {
            item.as_str()
                .unwrap_or_else(|| panic!("array `{key}` contains non-string: {item:?}"))
                .to_string()
        })
        .collect()
}

fn optional_str_array(table: &toml::value::Table, key: &str) -> Vec<String> {
    table
        .get(key)
        .and_then(toml::Value::as_array)
        .map(|items| {
            items
                .iter()
                .map(|item| {
                    item.as_str()
                        .unwrap_or_else(|| panic!("array `{key}` contains non-string: {item:?}"))
                        .to_string()
                })
                .collect()
        })
        .unwrap_or_default()
}

fn groups() -> Vec<ScriptGroup> {
    parse_toml(MANIFEST_PATH)
        .get("script_group")
        .and_then(toml::Value::as_array)
        .unwrap_or_else(|| panic!("{MANIFEST_PATH} missing [[script_group]] rows"))
        .iter()
        .map(|row| {
            let table = row
                .as_table()
                .unwrap_or_else(|| panic!("script_group row is not a table: {row:?}"));
            ScriptGroup {
                id: as_string(table, "id"),
                classification: as_string(table, "classification"),
                status: as_string(table, "status"),
                paths: str_array(table, "paths"),
                covered_by: str_array(table, "covered_by"),
                realworld_task_ids: optional_str_array(table, "realworld_task_ids"),
                broad_family_ids: optional_str_array(table, "broad_family_ids"),
                counts_for_obl005_script_closure: as_bool(
                    table,
                    "counts_for_obl005_script_closure",
                ),
            }
        })
        .collect()
}

fn collect_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(path) = stack.pop() {
        // Skip Python bytecode caches: a `__pycache__/*.pyc` is a compiled build artifact, not
        // a retained automation file (forensic 2026-06-01 — an untracked .pyc must never make the
        // no-zombie inventory un-closeable; this corrects the scan's scope, it does not weaken it).
        if path.file_name().map(|n| n == "__pycache__").unwrap_or(false) {
            continue;
        }
        if path.extension().map(|e| e == "pyc").unwrap_or(false) {
            continue;
        }
        if path.is_dir() {
            for entry in
                fs::read_dir(&path).unwrap_or_else(|err| panic!("read dir {path:?}: {err}"))
            {
                stack.push(
                    entry
                        .unwrap_or_else(|err| panic!("read dir entry {path:?}: {err}"))
                        .path(),
                );
            }
        } else if path.is_file() {
            out.push(path);
        }
    }
    out.sort();
    out
}

fn normalize(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn contract_ids(path: &str, key: &str) -> BTreeSet<String> {
    parse_toml(path)
        .get(key)
        .and_then(toml::Value::as_array)
        .unwrap_or_else(|| panic!("{path} missing [[{key}]] rows"))
        .iter()
        .map(|row| {
            row.as_table()
                .and_then(|table| table.get("id"))
                .and_then(toml::Value::as_str)
                .unwrap_or_else(|| panic!("{path} [[{key}]] row missing id: {row:?}"))
                .to_string()
        })
        .collect()
}

fn claimed_script_paths(groups: &[ScriptGroup]) -> BTreeMap<String, String> {
    let mut claimed = BTreeMap::new();
    for group in groups {
        for path in &group.paths {
            let path = Path::new(path);
            assert!(
                AUTOMATION_ROOTS.iter().any(|root| path.starts_with(root)),
                "script group `{}` claims path outside retained automation roots: {}",
                group.id,
                path.display()
            );
            assert!(
                path.exists(),
                "script group `{}` claims missing path: {}",
                group.id,
                path.display()
            );
            for file in collect_files(path) {
                let key = normalize(&file);
                if let Some(prev) = claimed.insert(key.clone(), group.id.clone()) {
                    panic!(
                        "script `{key}` is claimed by both `{prev}` and `{}`",
                        group.id
                    );
                }
            }
        }
    }
    claimed
}

#[test]
fn script_inventory_policy_is_final_closure_witness() {
    let manifest = parse_toml(MANIFEST_PATH);
    assert_eq!(
        manifest.get("schema_version").and_then(toml::Value::as_str),
        Some("turingosv4.script_liveness_inventory.v1")
    );
    assert_eq!(
        manifest.get("authority").and_then(toml::Value::as_str),
        Some("constitution.md + script inventory + ChainTape/CAS evidence")
    );
    assert_eq!(
        manifest
            .get("final_closure_status")
            .and_then(toml::Value::as_str),
        Some("OBL005_FINAL_CLOSURE_VERIFIED"),
        "final closure has been claimed by the witness"
    );
}

#[test]
fn every_retained_script_file_has_exactly_one_liveness_group() {
    let groups = groups();
    let claimed = claimed_script_paths(&groups);
    let mut discovered = BTreeSet::new();
    for root in AUTOMATION_ROOTS {
        discovered.extend(
            collect_files(Path::new(root))
                .into_iter()
                .map(|path| normalize(&path)),
        );
    }
    let claimed_set: BTreeSet<_> = claimed.keys().cloned().collect();
    assert_eq!(
        claimed_set, discovered,
        "every retained automation file must be explicitly classified"
    );
}

#[test]
fn script_groups_have_valid_closure_semantics() {
    let allowed_classifications = BTreeSet::from([
        "constitution_gate",
        "dev_harness",
        "evidence_packaging",
        "git_hygiene",
        "historical_smoke",
        "local_probe",
        "production_entrypoint",
        "stress_harness",
        "true_suite_orchestrator",
    ]);
    let allowed_status = BTreeSet::from([
        "active_replay_bound",
        "active_support_gate",
        "dev_only",
        "historical_smoke",
    ]);

    for group in groups() {
        assert!(
            allowed_classifications.contains(group.classification.as_str()),
            "script group `{}` has unknown classification `{}`",
            group.id,
            group.classification
        );
        assert!(
            allowed_status.contains(group.status.as_str()),
            "script group `{}` has unknown status `{}`",
            group.id,
            group.status
        );
        for evidence_path in &group.covered_by {
            assert!(
                Path::new(evidence_path).exists(),
                "script group `{}` references missing coverage path: {evidence_path}",
                group.id
            );
        }
        if group.status == "historical_smoke"
            || group.status == "dev_only"
            || matches!(
                group.classification.as_str(),
                "historical_smoke" | "local_probe"
            )
        {
            assert!(
                !group.counts_for_obl005_script_closure,
                "historical/dev script group `{}` cannot count toward OBL-005 final closure",
                group.id
            );
        }
        if group.counts_for_obl005_script_closure {
            assert!(
                matches!(
                    group.status.as_str(),
                    "active_replay_bound" | "active_support_gate"
                ),
                "closure-counted script group `{}` must be active, not `{}`",
                group.id,
                group.status
            );
        }
    }
}

#[test]
fn production_true_suite_scripts_bind_to_realworld_or_broad_contracts() {
    let realworld_ids = contract_ids(REALWORLD_MANIFEST, "task");
    let broad_ids = contract_ids(BROAD_MANIFEST, "family");

    for group in groups() {
        if group.classification != "production_entrypoint" {
            continue;
        }
        assert!(
            group.counts_for_obl005_script_closure,
            "production entrypoint group `{}` must count toward script closure",
            group.id
        );
        assert!(
            !group.realworld_task_ids.is_empty() || !group.broad_family_ids.is_empty(),
            "production entrypoint group `{}` must bind to realworld or broad contracts",
            group.id
        );
        for task_id in &group.realworld_task_ids {
            assert!(
                realworld_ids.contains(task_id),
                "script group `{}` references unknown realworld task `{task_id}`",
                group.id
            );
        }
        for family_id in &group.broad_family_ids {
            assert!(
                broad_ids.contains(family_id),
                "script group `{}` references unknown broad family `{family_id}`",
                group.id
            );
        }
    }
}
