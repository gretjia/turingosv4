//! No-zombie production module liveness contract.
//!
//! This gate is intentionally stricter than ordinary smoke tests. Smoke tests
//! can prove that a local seam still compiles, but they do not prove the user's
//! stop condition: every retained production code group must be necessary for
//! AGI and lit by broad real-world task evidence. The manifest below is a
//! derived inventory, not a new source of truth; constitution.md plus
//! ChainTape/CAS evidence remain authoritative.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const MANIFEST_PATH: &str = "tests/fixtures/liveness/production_module_liveness.toml";
const FINAL_CLOSURE_STATUS: &str = "OBL005_FINAL_CLOSURE_VERIFIED";

#[derive(Debug, Clone)]
struct Group {
    id: String,
    classification: String,
    status: String,
    risk_class: i64,
    allowed_as_fc_authority: bool,
    restricted_surface: bool,
    module_ids: Vec<String>,
    paths: Vec<String>,
    smoke_gates: Vec<String>,
    real_world_evidence: Vec<String>,
    evidence_requires: Vec<String>,
    closure_action: Option<String>,
}

fn manifest() -> toml::Value {
    let raw = fs::read_to_string(MANIFEST_PATH)
        .unwrap_or_else(|err| panic!("read {MANIFEST_PATH}: {err}"));
    toml::from_str(&raw).unwrap_or_else(|err| panic!("parse {MANIFEST_PATH}: {err}"))
}

fn as_str_array(table: &toml::value::Table, key: &str) -> Vec<String> {
    table
        .get(key)
        .and_then(toml::Value::as_array)
        .unwrap_or_else(|| panic!("group missing array `{key}`: {table:?}"))
        .iter()
        .map(|v| {
            v.as_str()
                .unwrap_or_else(|| panic!("array `{key}` contains non-string: {v:?}"))
                .to_string()
        })
        .collect()
}

fn as_string(table: &toml::value::Table, key: &str) -> String {
    table
        .get(key)
        .and_then(toml::Value::as_str)
        .unwrap_or_else(|| panic!("group missing string `{key}`: {table:?}"))
        .to_string()
}

fn groups() -> Vec<Group> {
    let manifest = manifest();
    manifest
        .get("group")
        .and_then(toml::Value::as_array)
        .unwrap_or_else(|| panic!("{MANIFEST_PATH} missing [[group]] rows"))
        .iter()
        .map(|row| {
            let table = row
                .as_table()
                .unwrap_or_else(|| panic!("group row is not a table: {row:?}"));
            Group {
                id: as_string(table, "id"),
                classification: as_string(table, "classification"),
                status: as_string(table, "status"),
                risk_class: table
                    .get("risk_class")
                    .and_then(toml::Value::as_integer)
                    .unwrap_or_else(|| panic!("group missing integer risk_class: {table:?}")),
                allowed_as_fc_authority: table
                    .get("allowed_as_fc_authority")
                    .and_then(toml::Value::as_bool)
                    .unwrap_or_else(|| {
                        panic!("group missing bool allowed_as_fc_authority: {table:?}")
                    }),
                restricted_surface: table
                    .get("restricted_surface")
                    .and_then(toml::Value::as_bool)
                    .unwrap_or_else(|| panic!("group missing bool restricted_surface: {table:?}")),
                module_ids: as_str_array(table, "module_ids"),
                paths: as_str_array(table, "paths"),
                smoke_gates: as_str_array(table, "smoke_gates"),
                real_world_evidence: as_str_array(table, "real_world_evidence"),
                evidence_requires: as_str_array(table, "evidence_requires"),
                closure_action: table
                    .get("closure_action")
                    .and_then(toml::Value::as_str)
                    .map(str::to_string),
            }
        })
        .collect()
}

fn module_declarations(path: &str, prefix: &str, include_private_mod: bool) -> Vec<String> {
    let raw = fs::read_to_string(path).unwrap_or_else(|err| panic!("read {path}: {err}"));
    raw.lines()
        .filter_map(|line| {
            let line = line.trim();
            let name = if let Some(rest) = line.strip_prefix("pub mod ") {
                rest.strip_suffix(';')
            } else if let Some(rest) = line.strip_prefix("pub(crate) mod ") {
                rest.strip_suffix(';')
            } else if include_private_mod {
                line.strip_prefix("mod ")?.strip_suffix(';')
            } else {
                None
            }?;
            if prefix.is_empty() {
                Some(name.to_string())
            } else {
                Some(format!("{prefix}::{name}"))
            }
        })
        .collect()
}

fn standalone_binary_inventory() -> Vec<String> {
    let mut ids = Vec::new();
    for entry in fs::read_dir("src/bin").unwrap_or_else(|err| panic!("read src/bin: {err}")) {
        let path = entry
            .unwrap_or_else(|err| panic!("read src/bin entry: {err}"))
            .path();
        if path.is_file() && path.extension().and_then(|v| v.to_str()) == Some("rs") {
            let stem = path
                .file_stem()
                .and_then(|v| v.to_str())
                .unwrap_or_else(|| panic!("invalid src/bin file name: {path:?}"));
            ids.push(format!("bin::{stem}"));
        }
    }
    ids
}

fn declared_inventory() -> BTreeSet<String> {
    let roots = [
        ("src/lib.rs", "", false),
        ("src/drivers/mod.rs", "drivers", false),
        ("src/runtime/mod.rs", "runtime", false),
        ("src/state/mod.rs", "state", false),
        ("src/economy/mod.rs", "economy", false),
        ("src/bottom_white/mod.rs", "bottom_white", false),
        ("src/bottom_white/cas/mod.rs", "bottom_white::cas", false),
        (
            "src/bottom_white/ledger/mod.rs",
            "bottom_white::ledger",
            false,
        ),
        (
            "src/bottom_white/tools/mod.rs",
            "bottom_white::tools",
            false,
        ),
        ("src/top_white/mod.rs", "top_white", false),
        (
            "src/top_white/predicates/mod.rs",
            "top_white::predicates",
            false,
        ),
        ("src/sdk/mod.rs", "sdk", false),
        ("src/sdk/tools/mod.rs", "sdk::tools", false),
        ("src/judges/mod.rs", "judges", false),
        ("src/web/mod.rs", "web", false),
        ("src/bin/turingos.rs", "bin::turingos", true),
    ];
    let mut inventory: BTreeSet<String> = roots
        .into_iter()
        .flat_map(|(path, prefix, include_private)| {
            module_declarations(path, prefix, include_private)
        })
        .collect();
    inventory.insert("main".to_string());
    inventory.extend(standalone_binary_inventory());
    inventory
}

fn manifest_module_index(groups: &[Group]) -> BTreeMap<String, String> {
    let mut index = BTreeMap::new();
    for group in groups {
        for module_id in &group.module_ids {
            if let Some(prev) = index.insert(module_id.clone(), group.id.clone()) {
                panic!(
                    "module `{module_id}` is claimed by both `{prev}` and `{}`",
                    group.id
                );
            }
        }
    }
    index
}

fn group_by_id<'a>(groups: &'a [Group], id: &str) -> &'a Group {
    groups
        .iter()
        .find(|group| group.id == id)
        .unwrap_or_else(|| panic!("missing liveness group `{id}`"))
}

fn assert_unique_group_ids(groups: &[Group]) {
    let mut seen = BTreeSet::new();
    let mut duplicates = Vec::new();
    for group in groups {
        if !seen.insert(group.id.clone()) {
            duplicates.push(group.id.clone());
        }
    }
    assert!(
        duplicates.is_empty(),
        "duplicate liveness group ids would shadow no-zombie accounting: {duplicates:?}"
    );
}

fn assert_existing_path(path: &str) {
    assert!(
        Path::new(path).exists(),
        "manifest path does not exist: {path}"
    );
}

fn assert_manifest_path_pattern_exists(pattern: &str) {
    if let Some(star_idx) = pattern.find('*') {
        let prefix = &pattern[..star_idx];
        let suffix = &pattern[star_idx + 1..];
        let parent = Path::new(prefix).parent().unwrap_or_else(|| Path::new("."));
        let has_match = fs::read_dir(parent)
            .unwrap_or_else(|err| panic!("read glob parent for `{pattern}`: {err}"))
            .any(|entry| {
                let path = entry
                    .unwrap_or_else(|err| panic!("read glob entry for `{pattern}`: {err}"))
                    .path();
                let rendered = path.to_string_lossy();
                rendered.starts_with(prefix) && rendered.ends_with(suffix)
            });
        assert!(has_match, "manifest glob pattern has no match: {pattern}");
    } else {
        assert_existing_path(pattern);
    }
}

fn assert_smoke_gate_file_exists(gate: &str) {
    let path = gate.split("::").next().unwrap_or(gate);
    assert_existing_path(path);
}

#[test]
fn liveness_manifest_policy_is_real_world_first() {
    let manifest = manifest();
    assert_eq!(
        manifest.get("schema_version").and_then(toml::Value::as_str),
        Some("turingosv4.production_module_liveness.v1")
    );
    assert_eq!(
        manifest.get("authority").and_then(toml::Value::as_str),
        Some("constitution.md + ChainTape/CAS evidence")
    );
    assert_eq!(
        manifest.get("policy").and_then(toml::Value::as_str),
        Some("real_world_required_for_final")
    );
    assert_eq!(
        manifest
            .get("smoke_is_not_final_evidence")
            .and_then(toml::Value::as_bool),
        Some(true),
        "smoke tests must never satisfy the final no-zombie claim"
    );
    assert_eq!(
        manifest
            .get("final_closure_status")
            .and_then(toml::Value::as_str),
        Some(FINAL_CLOSURE_STATUS),
        "only the explicit final closure status is allowed until full-system true runs close every retained group"
    );
}

#[test]
fn final_closure_cannot_be_claimed_while_quarantine_remains() {
    let manifest = manifest();
    let groups = groups();
    let has_quarantine = groups
        .iter()
        .any(|group| group.status == "legacy_quarantined");

    if has_quarantine {
        assert_ne!(
            manifest
                .get("final_closure_status")
                .and_then(toml::Value::as_str),
            Some("OBL005_FINAL_CLOSURE_VERIFIED"),
            "OBL-005 final closure cannot be claimed while any legacy_quarantined group remains as production blockers"
        );
    }

    for group in groups {
        if group.status == "legacy_quarantined" {
            let action = group.closure_action.as_deref().unwrap_or_default();
            assert!(
                action.contains("Delete") || action.contains("rebind") || action.contains("Rebind"),
                "legacy quarantine group `{}` must name a concrete delete-or-rebind action; got `{action}`",
                group.id
            );
        }
    }
}

#[test]
fn liveness_group_ids_are_unique() {
    assert_unique_group_ids(&groups());
}

#[test]
fn every_exported_module_has_exactly_one_liveness_group() {
    let groups = groups();
    assert_unique_group_ids(&groups);
    let index = manifest_module_index(&groups);
    let declared = declared_inventory();

    let mut missing = Vec::new();
    for module_id in &declared {
        if !index.contains_key(module_id) {
            missing.push(module_id.clone());
        }
    }
    assert!(
        missing.is_empty(),
        "exported or routed modules missing no-zombie liveness group: {missing:?}"
    );

    for module_id in index.keys() {
        assert!(
            declared.contains(module_id) || module_id == "main",
            "manifest claims module `{module_id}` that is not declared in the scanned module roots"
        );
    }
}

#[test]
fn candidate_groups_have_real_world_chaintape_or_cas_evidence() {
    for group in groups() {
        match group.status.as_str() {
            "historical_real_world_candidate" | "legacy_quarantined" | "smoke_only" => {}
            other => panic!("unknown liveness status `{other}` in `{}`", group.id),
        }
        assert!(
            !group.paths.is_empty(),
            "group `{}` must name owned paths",
            group.id
        );
        for path in &group.paths {
            assert_manifest_path_pattern_exists(path);
        }
        assert!(
            !group.smoke_gates.is_empty(),
            "group `{}` needs smoke gates for fast regression detection",
            group.id
        );
        for gate in &group.smoke_gates {
            assert_smoke_gate_file_exists(gate);
        }
        assert!(
            !group.evidence_requires.is_empty(),
            "group `{}` must name required evidence kinds",
            group.id
        );

        if group.status == "historical_real_world_candidate" {
            assert!(
                !group.real_world_evidence.is_empty(),
                "candidate group `{}` has only smoke evidence; broad real-world evidence is required",
                group.id
            );
            let real_evidence_tokens = [
                "accepted/rejected tape",
                "admission verification",
                "audit verdict",
                "backend observer",
                "bounded prompt",
                "CAS",
                "ChainTape",
                "ChainTapeLease",
                "fresh boot",
                "generated artifact",
                "integer money",
                "L4",
                "L4.E",
                "market decision trace",
                "market projection",
                "market tx",
                "predicate activation",
                "read-view",
                "real API session",
                "real command path",
                "real task attempts",
                "real user simulator",
                "registry root",
                "replay",
                "retry tape",
                "runtime repo",
                "system-only meta tx",
                "tool registry root",
            ];
            assert!(
                group.evidence_requires.iter().any(|kind| {
                    real_evidence_tokens
                        .iter()
                        .any(|token| kind.eq_ignore_ascii_case(token))
                }),
                "candidate group `{}` must require ChainTape/CAS/replay or real-run evidence, got {:?}",
                group.id,
                group.evidence_requires
            );
            for path in &group.real_world_evidence {
                assert!(
                    !path.ends_with("evaluator.stdout") && !path.ends_with("evaluator.stderr"),
                    "group `{}` cannot use stdout/stderr as final real-world evidence: {path}",
                    group.id
                );
                assert_existing_path(path);
            }
        } else {
            assert!(
                group.closure_action.as_deref().unwrap_or_default().len() >= 20,
                "non-live group `{}` must carry an explicit closure/quarantine action",
                group.id
            );
        }
    }
}

#[test]
fn product_and_legacy_rows_cannot_be_flowchart_authority() {
    for group in groups() {
        match group.classification.as_str() {
            "required_substrate" | "support_invariant" => {}
            "product_workload" | "legacy_quarantine" | "dev_only" => assert!(
                !group.allowed_as_fc_authority,
                "group `{}` is `{}` and cannot be cited as FC authority",
                group.id, group.classification
            ),
            other => panic!(
                "unknown liveness classification `{other}` in `{}`",
                group.id
            ),
        }

        if group.status == "legacy_quarantined" || group.classification == "dev_only" {
            assert!(
                group.real_world_evidence.is_empty(),
                "quarantined/dev group `{}` must not pretend to be lit by real-world AGI evidence",
                group.id
            );
        }
    }
}

#[test]
fn legacy_shadow_tape_group_is_split_from_active_tdma_ledger() {
    let groups = groups();
    assert!(
        groups
            .iter()
            .all(|group| group.id != "legacy_shadow_tape_and_tool_surfaces"),
        "the old mixed legacy group must stay split; it hid active TDMA ledger substrate inside SDK/WAL quarantine"
    );

    let tdma = group_by_id(&groups, "tdma_bounded_solver");
    assert!(
        tdma.module_ids.iter().any(|id| id == "ledger"),
        "`ledger` is active TDMA substrate and must be covered by the TDMA real-world evidence group"
    );
    assert!(
        tdma.paths.iter().any(|path| path == "src/ledger.rs"),
        "TDMA evidence group must name src/ledger.rs explicitly"
    );

    assert!(
        groups
            .iter()
            .all(|group| group.id != "legacy_wal_and_sdk_tool_surfaces"),
        "legacy WAL/SDK quarantine was closed; do not reintroduce it as an active liveness group"
    );

    let economy = group_by_id(&groups, "economy_market_settlement");
    assert!(
        economy
            .module_ids
            .iter()
            .any(|id| id == "sdk::tools::wallet"),
        "wallet projection is active economy substrate and must be covered by economy real-world evidence"
    );
}

#[test]
fn frontend_product_code_is_accounted_for_when_present() {
    if !Path::new("frontend/package.json").exists() {
        return;
    }

    let groups = groups();
    let frontend = group_by_id(&groups, "frontend_product_surface");
    assert_eq!(
        frontend.classification, "product_workload",
        "frontend is retained product code, not a constitutional authority"
    );
    assert!(
        !frontend.allowed_as_fc_authority,
        "frontend product surface cannot become flowchart authority"
    );
    assert!(
        frontend.paths.iter().any(|path| path == "frontend/src")
            && frontend.paths.iter().any(|path| path == "frontend/test"),
        "frontend group must account for source and tests"
    );
    assert!(
        frontend
            .smoke_gates
            .iter()
            .any(|gate| gate == "frontend/test/welcome.test.ts"),
        "frontend group must name a frontend-owned smoke gate"
    );
    assert!(
        !frontend.real_world_evidence.is_empty(),
        "frontend group must stay bound to real product-path evidence, not only unit tests"
    );
}

#[test]
fn design_system_code_is_accounted_for_when_present() {
    if !Path::new("design-system").exists() {
        return;
    }

    let groups = groups();
    let design_system = group_by_id(&groups, "design_system_product_surface");
    assert!(
        matches!(
            design_system.classification.as_str(),
            "product_workload" | "dev_only"
        ),
        "design-system code must be classified as retained product surface or dev-only substrate"
    );
    assert!(
        !design_system.allowed_as_fc_authority,
        "design-system code cannot become flowchart authority"
    );
    assert!(
        design_system
            .paths
            .iter()
            .any(|path| path == "design-system"),
        "design-system group must account for the root design-system path"
    );

    if design_system.classification == "product_workload" {
        assert!(
            !design_system.smoke_gates.is_empty(),
            "product design-system code must name a concrete smoke gate"
        );
        assert!(
            !design_system.real_world_evidence.is_empty(),
            "product design-system code must stay bound to real product-path evidence"
        );
    } else {
        assert!(
            design_system.real_world_evidence.is_empty(),
            "dev-only design-system code must not pretend to be lit by AGI production evidence"
        );
        assert!(
            design_system
                .closure_action
                .as_deref()
                .unwrap_or_default()
                .len()
                >= 20,
            "dev-only design-system code must carry an explicit exclusion rationale"
        );
    }
}

#[test]
fn restricted_surfaces_are_classified_high_risk() {
    for group in groups() {
        if group.restricted_surface {
            assert!(
                group.risk_class >= 3,
                "restricted group `{}` must be Class 3+; got {}",
                group.id,
                group.risk_class
            );
        }
        if group.risk_class >= 4 {
            assert!(
                group.restricted_surface || group.status == "legacy_quarantined",
                "Class 4 group `{}` must be a restricted surface or explicit quarantine",
                group.id
            );
        }
    }
}

#[test]
fn registered_real_world_suites_exist_and_are_not_smoke_labels() {
    let manifest = manifest();
    let suites = manifest
        .get("real_world_suite")
        .and_then(toml::Value::as_array)
        .unwrap_or_else(|| panic!("{MANIFEST_PATH} missing [[real_world_suite]] rows"));
    assert!(
        suites.len() >= 5,
        "broad real-world coverage requires multiple suite families, got {}",
        suites.len()
    );
    for suite in suites {
        let table = suite
            .as_table()
            .unwrap_or_else(|| panic!("real_world_suite row is not a table: {suite:?}"));
        let id = as_string(table, "id");
        let path = as_string(table, "path");
        let families = as_str_array(table, "families");
        let evidence = as_str_array(table, "evidence");
        assert_existing_path(&path);
        assert!(
            !id.to_ascii_lowercase().contains("smoke"),
            "real-world suite `{id}` must not be smoke-only"
        );
        assert!(
            !families.is_empty() && !evidence.is_empty(),
            "real-world suite `{id}` must name families and evidence kinds"
        );
    }
}
