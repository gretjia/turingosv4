//! Constitution-only flowchart source alignment gate.
//!
//! This test intentionally treats `constitution.md` as the only source for
//! FC1/FC2/FC3 topology. Historical extracts and trace matrices are derived
//! views; they may document history, but they must not define current FC truth.

use std::fs;

const CONSTITUTION: &str = "constitution.md";
const TRACE_MATRIX: &str = "handover/alignment/TRACE_FLOWCHART_MATRIX.md";
const EXEC_MATRIX: &str = "handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md";
const LIVENESS_INVENTORY: &str = "handover/audits/FLOWCHART_LIVENESS_INVENTORY_2026-05-25.md";
const ARCHITECTURE_MAP: &str = "handover/audits/TURINGOSV4_ARCHITECTURE_LIVENESS_MAP_2026-05-25.md";
const FC_ALIGNMENT: &str = "tests/fc_alignment_conformance.rs";
const FC3_META: &str = "tests/constitution_fc3_meta.rs";
const FC3_EVIDENCE: &str = "tests/constitution_fc3_evidence_binding.rs";
const REALWORLD_TDMA_JUDGE: &str = "tests/realworld_tdma_judge_ai_step_proof.rs";

const ACTIVE_FILES: &[&str] = &[
    TRACE_MATRIX,
    EXEC_MATRIX,
    FC_ALIGNMENT,
    FC3_META,
    FC3_EVIDENCE,
    REALWORLD_TDMA_JUDGE,
];

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"))
}

fn block_after<'a>(src: &'a str, marker: &str, fence: &str) -> &'a str {
    let start = src
        .find(marker)
        .unwrap_or_else(|| panic!("constitution flowchart marker missing: {marker}"));
    let tail = &src[start..];
    let end = tail
        .find(fence)
        .unwrap_or_else(|| panic!("constitution flowchart fence missing after: {marker}"));
    &tail[..end]
}

fn assert_no_active_occurrence(path: &str, body: &str, needle: &str) {
    assert!(
        !body.contains(needle),
        "{path} contains stale authoritative flowchart text `{needle}`"
    );
}

fn assert_line_contains_all(path: &str, body: &str, anchor: &str, required: &[&str]) {
    let line = body
        .lines()
        .find(|line| line.contains(anchor))
        .unwrap_or_else(|| panic!("{path} missing required active row `{anchor}`"));
    for token in required {
        assert!(
            line.contains(token),
            "{path} row `{anchor}` must contain `{token}`, got: {line}"
        );
    }
}

fn stale_flowchart_needles() -> Vec<String> {
    vec![
        ["FC", "_ELEMENTS"].concat(),
        ["Judge", "AI"].concat(),
        ["judge", "AI"].concat(),
        ["FC2-", "N24"].concat(),
        ["FC2-", "N25"].concat(),
        ["FC2-", "N26"].concat(),
        ["FC2-", "N27"].concat(),
        ["FC3-", "N40"].concat(),
        "fc2_n24".to_string(),
        "fc2_n25".to_string(),
        "fc2_n26".to_string(),
        "fc2_n27".to_string(),
        "fc3_n40".to_string(),
        "deep-history override".to_string(),
    ]
}

#[test]
fn constitution_flowchart_blocks_are_directly_parseable() {
    let constitution = read(CONSTITUTION);
    let fc1 = block_after(&constitution, "```mermaid\ngraph TD", "\n```");
    let fc2 = block_after(&constitution, "    flowchart TD", "\n    ```");
    let fc3 = block_after(&constitution, "    graph TB", "\n    ```");

    for id in [
        "q0", "HEAD0", "tape0", "q1", "HEAD1", "tape1", "delta", "qi", "si", "qo", "ao", "p", "r",
        "w",
    ] {
        assert!(fc1.contains(id), "FC1 constitution block missing `{id}`");
    }

    for id in [
        "human",
        "law",
        "initAI",
        "halt",
        "q0",
        "HEAD0",
        "tape0",
        "q1",
        "HEAD1",
        "tape1",
        "r",
        "qi",
        "si",
        "delta",
        "qo",
        "ao",
        "predicates",
        "p",
        "mr",
        "clock",
        "w",
        "tools",
    ] {
        assert!(fc2.contains(id), "FC2 constitution block missing `{id}`");
    }
    assert!(fc2.contains("clock --> mr"));
    assert!(fc2.contains("mr ==>|map| tape0"));
    assert!(fc2.contains("mr ==>|reduce| tape1"));

    for id in [
        "boot",
        "human",
        "constitution",
        "logs",
        "vetoAI",
        "architectAI",
        "top",
        "agents",
        "tools",
        "tape",
        "log",
        "error",
    ] {
        assert!(fc3.contains(id), "FC3 constitution block missing `{id}`");
    }
    assert!(fc3.contains("logs -->|feedback| architectAI"));
    assert!(fc3.contains("init ==> error ==========>|re-init| boot"));
}

#[test]
fn active_views_do_not_promote_derived_extracts_as_authority() {
    for path in [TRACE_MATRIX, EXEC_MATRIX, FC_ALIGNMENT] {
        let body = read(path);
        let needles = [
            "Raw flowchart node enumeration".to_string(),
            "Existing symbol-level mapping".to_string(),
            "Existing FC element extract".to_string(),
            "Existing FC trace".to_string(),
            "Source of mappings:".to_string(),
            ["Update `FC", "_ELEMENTS"].concat(),
            ["TRACE", "_MATRIX_v0_"].concat(),
            ["TRACE", "_MATRIX_v1_"].concat(),
            ["TRACE", "_MATRIX_v2_"].concat(),
            ["TRACE", "_MATRIX_v3_"].concat(),
        ];
        for needle in needles {
            assert_no_active_occurrence(path, &body, &needle);
        }
    }
}

#[test]
fn active_views_use_veto_ai_current_audit_doctrine() {
    for path in ACTIVE_FILES {
        let body = read(path);
        let needles = [
            ["Judge", "AI"].concat(),
            ["judge", "AI"].concat(),
            ["Codex/Gemi", "ni"].concat(),
            ["Codex + Gemi", "ni"].concat(),
            ["Gemi", "ni"].concat(),
        ];
        for needle in needles {
            assert_no_active_occurrence(path, &body, &needle);
        }
    }
}

#[test]
fn active_views_do_not_use_retired_fc_handles_as_canonical_coverage() {
    for path in [TRACE_MATRIX, EXEC_MATRIX, FC_ALIGNMENT] {
        let body = read(path);
        let needles = [
            ["FC2-", "N24"].concat(),
            ["FC2-", "N25"].concat(),
            ["FC2-", "N26"].concat(),
            ["FC2-", "N27"].concat(),
            ["FC3-", "N40"].concat(),
            "fc2_n24".to_string(),
            "fc2_n25".to_string(),
            "fc2_n26".to_string(),
            "fc2_n27".to_string(),
            "fc3_n40".to_string(),
            ["FC1-", "N32"].concat(),
            ["FC1-", "N33"].concat(),
            ["FC2-", "N29"].concat(),
            ["FC2-", "N30"].concat(),
            ["FC3-", "N42"].concat(),
            ["FC3-", "N43"].concat(),
            ["fc1_", "n32"].concat(),
            ["fc1_", "n33"].concat(),
            ["fc2_", "n29"].concat(),
            ["fc2_", "n30"].concat(),
            ["fc3_", "n42"].concat(),
            ["fc3_", "n43"].concat(),
            ["FC1-", "E18"].concat(),
            ["FC3-", "E14"].concat(),
            ["FC3-", "S3"].concat(),
        ];
        for needle in needles {
            assert_no_active_occurrence(path, &body, &needle);
        }
    }
}

#[test]
fn ignored_runtime_stubs_are_not_counted_as_green_coverage() {
    let trace = read(TRACE_MATRIX);
    let exec = read(EXEC_MATRIX);
    for line in trace.lines().chain(exec.lines()) {
        if line.contains('✅') {
            assert!(
                !line.contains("runtime not implemented")
                    && !line.contains("external-only")
                    && !line.contains("external ("),
                "green row counts non-runtime or ignored coverage: {line}"
            );
        }
    }
}

#[test]
fn flowchart_liveness_status_matches_class4_closures() {
    let trace = read(TRACE_MATRIX);
    let exec = read(EXEC_MATRIX);
    let inventory = read(LIVENESS_INVENTORY);
    let architecture = read(ARCHITECTURE_MAP);

    assert_line_contains_all(TRACE_MATRIX, &trace, "FC2:mr", &["✅"]);
    assert_line_contains_all(EXEC_MATRIX, &exec, "Art. IV.tick", &["GREEN"]);
    assert_line_contains_all(
        EXEC_MATRIX,
        &exec,
        "map-reduce tick tape-visible",
        &["GREEN"],
    );
    assert_line_contains_all(
        TRACE_MATRIX,
        &trace,
        "FC3 edge `logs -> feedback -> architectAI`",
        &["LIVE"],
    );
    assert_line_contains_all(
        TRACE_MATRIX,
        &trace,
        "FC3 edge `init -> error -> re-init -> boot`",
        &["LIVE"],
    );
    assert_line_contains_all(TRACE_MATRIX, &trace, "FC3:vetoAI", &["RUNTIME"]);
    assert_line_contains_all(TRACE_MATRIX, &trace, "FC3:architectAI", &["RUNTIME"]);
    assert_line_contains_all(EXEC_MATRIX, &exec, "Veto-AI veto-only", &["RUNTIME"]);

    assert_line_contains_all(
        &LIVENESS_INVENTORY,
        &inventory,
        "`rtool -> input`",
        &["LIVE"],
    );
    assert_line_contains_all(
        &LIVENESS_INVENTORY,
        &inventory,
        "map-reduce tick",
        &["LIVE"],
    );
    assert_line_contains_all(
        &LIVENESS_INVENTORY,
        &inventory,
        "logs feedback to ArchitectAI",
        &["LIVE"],
    );
    assert_line_contains_all(
        &LIVENESS_INVENTORY,
        &inventory,
        "error to re-init semantics",
        &["LIVE"],
    );
    assert_line_contains_all(&LIVENESS_INVENTORY, &inventory, "Veto-AI role", &["LIVE"]);
    assert_line_contains_all(
        &LIVENESS_INVENTORY,
        &inventory,
        "ArchitectAI role",
        &["LIVE"],
    );

    assert_line_contains_all(
        &ARCHITECTURE_MAP,
        &architecture,
        "`rtool -> input`",
        &["LIVE"],
    );
    assert_line_contains_all(
        &ARCHITECTURE_MAP,
        &architecture,
        "map-reduce tick",
        &["LIVE"],
    );
    assert_line_contains_all(
        &ARCHITECTURE_MAP,
        &architecture,
        "logs feedback to ArchitectAI",
        &["LIVE"],
    );
    assert_line_contains_all(
        &ARCHITECTURE_MAP,
        &architecture,
        "error re-init loop",
        &["LIVE"],
    );
    assert_line_contains_all(
        &ARCHITECTURE_MAP,
        &architecture,
        "ArchitectAI / Veto-AI",
        &["LIVE"],
    );
}

#[test]
fn archived_alignment_files_with_stale_handles_are_marked_non_authoritative() {
    for entry in fs::read_dir("handover/alignment").expect("read handover/alignment") {
        let entry = entry.expect("read handover/alignment entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        let path_str = path.to_string_lossy();
        if ACTIVE_FILES.contains(&path_str.as_ref()) {
            continue;
        }

        let body = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read archived alignment file {path_str}: {e}"));
        let has_stale_handle = stale_flowchart_needles()
            .iter()
            .any(|needle| body.contains(needle));
        if has_stale_handle {
            assert!(
                body.contains("ARCHIVAL DERIVED VIEW") && body.contains("not current authority"),
                "{path_str} contains stale flowchart vocabulary but lacks a non-authoritative archive banner"
            );
        }
    }
}
