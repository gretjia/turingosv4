use std::fs;

fn repo_path(relative: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

#[test]
fn clean_checkout_replay_script_requires_no_network() {
    let script =
        fs::read_to_string(repo_path("scripts/tc_clean_checkout_replay.sh")).expect("read script");
    let report = fs::read_to_string(repo_path(
        "handover/reports/TC_CLEAN_CHECKOUT_REPLAY_2026-06-04.md",
    ))
    .expect("read report");

    assert!(script.contains("TURINGOS_TC_REPLAY_NO_NETWORK"));
    assert!(script.contains("network must be disabled"));
    assert!(script.contains("git -C \"$REPO\" worktree add --detach"));
    assert!(report.contains("network_policy: disabled"));
    assert!(report.contains("llm_replay_policy: disabled"));
    assert!(!script.contains("curl "));
}

#[test]
fn clean_checkout_replay_compares_exported_hashes() {
    let script =
        fs::read_to_string(repo_path("scripts/tc_clean_checkout_replay.sh")).expect("read script");
    let report = fs::read_to_string(repo_path(
        "handover/reports/TC_CLEAN_CHECKOUT_REPLAY_2026-06-04.md",
    ))
    .expect("read report");

    assert!(script.contains("hash_file"));
    assert!(script.contains("packet_report_hash"));
    assert!(script.contains("replay_report_hash"));
    assert!(script.contains("--test tc_crash_matrix"));
    assert!(script.contains("--test tc_universal_witnesses"));
    assert!(script.contains("--test tc_g0_completeness"));
    assert!(report.contains("hash_compare: required"));
}

#[test]
fn clean_checkout_replay_requires_obl_all_closed() {
    let script =
        fs::read_to_string(repo_path("scripts/tc_clean_checkout_replay.sh")).expect("read script");
    let report = fs::read_to_string(repo_path(
        "handover/reports/TC_CLEAN_CHECKOUT_REPLAY_2026-06-04.md",
    ))
    .expect("read report");

    assert!(script.contains("OBL-014 is not marked satisfied"));
    assert!(script.contains("obligation_witness_verdict: OBL-ALL-CLOSED"));
    assert!(script.contains("final_obligation_witness_verdict: OBL-ALL-CLOSED"));
    assert!(report.contains("obligation_witness_required: OBL-ALL-CLOSED"));
}
