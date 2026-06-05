use std::fs;
use std::process::Command;

fn repo_path(relative: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

#[test]
fn audit_packet_manifest_lists_required_artifacts() {
    let report = fs::read_to_string(repo_path(
        "handover/reports/TC_FULL_AUDIT_PACKET_MANIFEST_2026-06-04.md",
    ))
    .expect("read report");

    for marker in [
        "source_sha",
        "worktree_status",
        "constitution_hash",
        "boot_manifest_hash",
        "path_b_ref_schema",
        "replay_commands",
        "crash_matrix_results",
        "universal_witnesses",
        "g0_manifest",
        "scheduler_traces",
        "parity_schema",
        "clean_context_audits",
        "obligation_witness",
        "dirty_tree_preservation",
    ] {
        assert!(report.contains(marker), "missing marker {marker}");
    }
}

#[test]
fn audit_packet_reliability_excludes_metadata_artifacts() {
    let report = fs::read_to_string(repo_path(
        "handover/reports/TC_FULL_AUDIT_PACKET_MANIFEST_2026-06-04.md",
    ))
    .expect("read report");

    for excluded in ["RUN_STATUS.json", "STAGE_A_POWER_GATE.json", "prereg.json"] {
        assert!(
            report.contains(excluded),
            "exclusion must be explicit for {excluded}"
        );
        assert!(
            !report.contains(&format!("reliability_input: {excluded}")),
            "{excluded} must not be a reliability input"
        );
    }
}

#[test]
fn audit_packet_export_check_passes_contract() {
    let status = Command::new("bash")
        .arg(repo_path("scripts/export_tc_audit_packet.sh"))
        .arg("--check")
        .status()
        .expect("run export check");
    assert!(status.success(), "export check must pass");
}
