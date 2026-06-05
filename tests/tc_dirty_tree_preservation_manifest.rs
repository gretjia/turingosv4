const MANIFEST: &str =
    include_str!("../handover/reports/TC_Q_DIRTY_TREE_PRESERVATION_2026-06-04.yaml");

#[test]
fn dirty_tree_manifest_locks_clean_base_and_snapshot_hashes() {
    for needle in [
        "snapshot_id: tc-q-20260603T232218Z",
        "source_branch: claude/p1-realvalue",
        "source_head: 1da3f6674aa7036d76cc9a49b273853b10e13d3e",
        "origin_main: \"39233aa7c868f0e9b37a7a29eb426279f41cf032\"",
        "tracked_patch_sha256: 3c7b3036360fd2b86253faf23482a35301959fc2d5cfe3264fd8ec391b224036",
        "ahead_patch_sha256: 4333566bde6e313559000fd10aa34c15b0353b975838794828a213f7d485f4d1",
        "untracked_tgz_sha256: 8ba17efbb60fa32edaf881cbf74c99a479e5525a12f7337b3e15b10880924de3",
        "status_txt_sha256: bbac726a8bac635451b50b67250700b2575e2c50b606f4ed82e1042eaa6c7d0f",
    ] {
        assert!(MANIFEST.contains(needle), "manifest missing {needle}");
    }
}

#[test]
fn dirty_tree_manifest_classifies_conflict_artifacts_as_drop_junk() {
    assert!(MANIFEST.contains("PORT_NOW: []"));
    assert!(MANIFEST.contains("rules/enforcement.log"));
    assert!(MANIFEST.contains("No dirty-file payload is imported wholesale"));
}
