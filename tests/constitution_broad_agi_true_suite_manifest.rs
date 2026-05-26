//! Broad AGI true-suite manifest gate.
//!
//! This is the contract layer between "we ran some old real-world tasks" and
//! the user's stronger OBL-005 stop condition. It does not claim liveness by
//! itself. It makes broad benchmark coverage falsifiable before expensive
//! DeepSeek/SiliconFlow batches run.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;

const MANIFEST: &str = "tests/fixtures/liveness/broad_agi_true_suite_manifest.toml";
const SCHEMA: &str = "turingosv4.broad_agi_true_suite_manifest.v1";
const OPEN_STATUS: &str = "OPEN_REAL_WORLD_COVERAGE_PENDING";
const REQUIRED_FAILURE_CLASSES: &[&str] = &[
    "kernel_invariant_failure",
    "model_task_failure",
    "infrastructure_failure",
    "benchmark_adapter_failure",
    "provider_policy_failure",
    "constitutional_rejection",
];
const REQUIRED_FC_BLOCKS: &[&str] = &["FC1", "FC2", "FC3"];
const REQUIRED_SUBSTRATE_GROUPS: &[&str] = &[
    "axiom_boot_trust_root",
    "canonical_tape_cas_state",
    "predicate_registry_top_white",
    "fc1_bus_read_view_bridge",
    "economy_market_settlement",
    "runtime_replay_evidence_audit",
    "agent_prompt_model_boundary",
    "fc3_runtime_meta_roles",
    "tool_registry_current",
];
const REQUIRED_FAMILIES: &[&str] = &[
    "gaia_general_assistant",
    "gpqa_science_reasoning",
    "math_formal_proof",
    "swebench_live_coding_repair",
    "webarena_web_agent",
    "mind2web_open_web",
    "osworld_computer_use",
    "toolbench_api_tool_use",
    "cybench_security_sandbox",
    "market_economy_polymarket",
    "memory_feedback_reinit",
];

#[derive(Debug)]
struct Family {
    id: String,
    source_family: String,
    public_source: String,
    risk_class: String,
    entry_boundary: String,
    fc_trace: Vec<String>,
    kernel_liveness_modules: Vec<String>,
    evidence_shape: Vec<String>,
    failure_taxonomy: Vec<String>,
    anti_contamination_guards: Vec<String>,
    final_evidence_artifacts: Vec<String>,
}

fn parse_manifest() -> toml::Value {
    let raw = fs::read_to_string(MANIFEST).unwrap_or_else(|err| panic!("read {MANIFEST}: {err}"));
    toml::from_str(&raw).unwrap_or_else(|err| panic!("parse {MANIFEST}: {err}"))
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

fn root_str_array(manifest: &toml::Value, key: &str) -> Vec<String> {
    manifest
        .get(key)
        .and_then(toml::Value::as_array)
        .unwrap_or_else(|| panic!("{MANIFEST} missing root array `{key}`"))
        .iter()
        .map(|v| {
            v.as_str()
                .unwrap_or_else(|| panic!("root array `{key}` contains non-string: {v:?}"))
                .to_string()
        })
        .collect()
}

fn families() -> Vec<Family> {
    parse_manifest()
        .get("family")
        .and_then(toml::Value::as_array)
        .unwrap_or_else(|| panic!("{MANIFEST} missing [[family]] rows"))
        .iter()
        .map(|row| {
            let table = row
                .as_table()
                .unwrap_or_else(|| panic!("family row is not a table: {row:?}"));
            Family {
                id: as_string(table, "id"),
                source_family: as_string(table, "source_family"),
                public_source: as_string(table, "public_source"),
                risk_class: as_string(table, "risk_class"),
                entry_boundary: as_string(table, "entry_boundary"),
                fc_trace: as_str_array(table, "fc_trace"),
                kernel_liveness_modules: as_str_array(table, "kernel_liveness_modules"),
                evidence_shape: as_str_array(table, "evidence_shape"),
                failure_taxonomy: as_str_array(table, "failure_taxonomy"),
                anti_contamination_guards: as_str_array(table, "anti_contamination_guards"),
                final_evidence_artifacts: as_str_array(table, "final_evidence_artifacts"),
            }
        })
        .collect()
}

fn has_url_or_internal_source(source: &str) -> bool {
    source.starts_with("https://arxiv.org/abs/")
        || source.starts_with("https://")
        || source.starts_with("internal_current_kernel_")
}

#[test]
fn broad_true_suite_manifest_is_non_closing_and_constitution_bound() {
    let manifest = parse_manifest();
    assert_eq!(
        manifest.get("schema_version").and_then(toml::Value::as_str),
        Some(SCHEMA)
    );
    assert_eq!(
        manifest.get("authority").and_then(toml::Value::as_str),
        Some("constitution.md + fresh current-kernel true-problem evidence")
    );
    assert_eq!(
        manifest.get("closure_status").and_then(toml::Value::as_str),
        Some(OPEN_STATUS),
        "benchmark manifest may define broad coverage, but cannot close OBL-005"
    );
    assert_eq!(
        manifest
            .get("old_15_is_not_sufficient")
            .and_then(toml::Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest
            .get("leaderboard_score_is_not_liveness")
            .and_then(toml::Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest
            .get("full_system_required_for_final")
            .and_then(toml::Value::as_bool),
        Some(true),
        "every final benchmark sample must run the whole constitutional machine"
    );
    assert_eq!(
        manifest
            .get("per_sample_fc_union_is_not_sufficient")
            .and_then(toml::Value::as_bool),
        Some(true),
        "FC1 from one benchmark plus FC3 from another is not full-system liveness"
    );
    assert_eq!(
        manifest
            .get("market_participation_required_for_every_sample")
            .and_then(toml::Value::as_bool),
        Some(true),
        "market/economy is constitutional substrate even for a one-agent run"
    );
    assert_eq!(
        manifest
            .get("full_system_sample_manifest")
            .and_then(toml::Value::as_str),
        Some("full_system_participation.json")
    );

    let required = root_str_array(&manifest, "required_failure_classes");
    for class in REQUIRED_FAILURE_CLASSES {
        assert!(
            required.iter().any(|item| item == class),
            "manifest root missing required failure class `{class}`"
        );
    }
    let required_fc = root_str_array(&manifest, "required_fc_blocks");
    for &fc in REQUIRED_FC_BLOCKS {
        assert!(
            required_fc.iter().any(|item| item == fc),
            "manifest root missing required FC block `{fc}`"
        );
    }
    let substrate = root_str_array(&manifest, "required_substrate_groups");
    for &group in REQUIRED_SUBSTRATE_GROUPS {
        assert!(
            substrate.iter().any(|item| item == group),
            "manifest root missing required full-system substrate group `{group}`"
        );
    }
    let market_modes = root_str_array(&manifest, "market_participation_modes");
    for mode in ["invest", "abstain_with_tape_visible_market_opportunity"] {
        assert!(
            market_modes.iter().any(|item| item == mode),
            "market participation modes must allow one-agent investment or tape-visible abstention: missing `{mode}`"
        );
    }
}

#[test]
fn benchmark_families_are_broad_and_have_machine_checkable_contracts() {
    let manifest = parse_manifest();
    let min_count = manifest
        .get("minimum_family_count")
        .and_then(toml::Value::as_integer)
        .unwrap_or(0);
    let families = families();
    assert!(
        families.len() as i64 >= min_count,
        "broad AGI suite has {} families but manifest requires {min_count}",
        families.len()
    );

    let allowed_boundaries: BTreeSet<_> = root_str_array(&manifest, "allowed_entry_boundaries")
        .into_iter()
        .collect();
    let ids: BTreeSet<_> = families.iter().map(|family| family.id.as_str()).collect();
    for required in REQUIRED_FAMILIES {
        assert!(
            ids.contains(required),
            "broad AGI suite missing required benchmark family `{required}`"
        );
    }

    let mut risk_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for family in &families {
        assert!(
            !family.source_family.trim().is_empty(),
            "family `{}` missing source_family",
            family.id
        );
        assert!(
            has_url_or_internal_source(&family.public_source),
            "family `{}` must cite a public URL or explicit internal current-kernel source",
            family.id
        );
        assert!(
            matches!(family.risk_class.as_str(), "Class 2" | "Class 3"),
            "family `{}` has unsupported risk class `{}`; Class 4 needs a separate charter",
            family.id,
            family.risk_class
        );
        *risk_counts.entry(family.risk_class.as_str()).or_default() += 1;
        assert!(
            allowed_boundaries.contains(&family.entry_boundary),
            "family `{}` uses undeclared entry boundary `{}`",
            family.id,
            family.entry_boundary
        );
        assert!(
            !family.fc_trace.is_empty()
                && !family.kernel_liveness_modules.is_empty()
                && !family.evidence_shape.is_empty()
                && !family.failure_taxonomy.is_empty()
                && !family.anti_contamination_guards.is_empty()
                && !family.final_evidence_artifacts.is_empty(),
            "family `{}` must have FC trace, liveness modules, evidence shape, failure taxonomy, guards, and final artifacts",
            family.id
        );
        for class in REQUIRED_FAILURE_CLASSES {
            assert!(
                family.failure_taxonomy.iter().any(|item| item == class),
                "family `{}` failure taxonomy missing base class `{class}`",
                family.id
            );
        }
        assert!(
            family
                .final_evidence_artifacts
                .iter()
                .all(|path| path.starts_with("handover/evidence/true_suite/<run>/")),
            "family `{}` final artifacts must be fresh true-suite paths: {:?}",
            family.id,
            family.final_evidence_artifacts
        );
        assert!(
            family
                .final_evidence_artifacts
                .iter()
                .any(|path| path.ends_with("/full_system_participation.json")),
            "family `{}` must require per-sample full-system participation evidence",
            family.id
        );
    }

    assert!(
        risk_counts.contains_key("Class 3"),
        "broad AGI suite must explicitly classify money/security/OS-like domains as Class 3"
    );
}

#[test]
fn broad_suite_lights_all_three_flowcharts_without_edge_node_drift() {
    let families = families();
    let mut fc_seen = BTreeSet::new();
    for family in &families {
        for trace in &family.fc_trace {
            assert!(
                trace.starts_with("FC1:") || trace.starts_with("FC2:") || trace.starts_with("FC3:"),
                "family `{}` has non-canonical FC trace `{trace}`",
                family.id
            );
            if let Some((fc, _)) = trace.split_once(':') {
                fc_seen.insert(fc.to_string());
            }
        }
    }
    for &fc in REQUIRED_FC_BLOCKS {
        assert!(
            fc_seen.contains(fc),
            "broad AGI manifest does not light {fc}"
        );
    }
    for family in &families {
        let family_fc: BTreeSet<_> = family
            .fc_trace
            .iter()
            .filter_map(|trace| trace.split_once(':').map(|(fc, _)| fc.to_string()))
            .collect();
        for &fc in REQUIRED_FC_BLOCKS {
            assert!(
                family_fc.contains(fc),
                "family `{}` only declares {:?}; every final sample must include {fc}, not rely on another family to light it",
                family.id,
                family_fc
            );
        }
    }
}

#[test]
fn broad_suite_forbids_old_15_and_score_only_contamination() {
    for family in families() {
        let searched_fields = family
            .final_evidence_artifacts
            .iter()
            .chain(std::iter::once(&family.public_source));
        for value in searched_fields {
            let lower = value.to_ascii_lowercase();
            for forbidden in [
                "old15",
                "old_15",
                "real8x",
                "real8_",
                "stage_phase7_real_e2e",
                "sidecar_only",
                "leaderboard",
                "score_only",
                "accuracy_only",
                "raw_prompt",
                "raw_response",
            ] {
                assert!(
                    !lower.contains(forbidden),
                    "family `{}` uses forbidden historical/sidecar source `{value}`",
                    family.id
                );
            }
        }
    }
}
