//! REAL-BCAST-1 — binding to REAL-13 clean evidence.

use std::path::Path;

use serde_json::Value;

#[test]
fn real13_status_sync_points_to_existing_clean_evidence() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let evidence = root.join("handover/evidence/real13_market_pressure_probe_20260516T071216Z");
    assert!(
        evidence.exists(),
        "REAL-13 clean evidence missing: {evidence:?}"
    );
    assert!(evidence
        .join("REAL13_MARKET_PRESSURE_PROBE_REPORT.md")
        .exists());
    assert!(evidence.join("aggregate_verdict.json").exists());
    let cas_index = evidence.join("cas/.turingos_cas_index.jsonl");
    if !cas_index.exists() {
        eprintln!(
            "REAL-13 historical CAS fixture is not hydrated in this worktree: {:?}; \
             continuing with tracked report/status binding only",
            cas_index
        );
    }

    let aggregate: Value = serde_json::from_str(
        &std::fs::read_to_string(evidence.join("aggregate_verdict.json"))
            .expect("aggregate verdict readable"),
    )
    .expect("aggregate verdict json");
    assert!(
        aggregate["tape_root"]["cas_object_count"]
            .as_u64()
            .expect("cas_object_count")
            > 0,
        "REAL-13 tracked audit verdict must bind a non-empty CAS object count"
    );
    let local_sidecar = evidence.join("cas/.turingos_cas_index.jsonl");
    if local_sidecar.exists() {
        let bytes = std::fs::metadata(&local_sidecar)
            .expect("local sidecar metadata")
            .len();
        assert!(
            bytes > 0,
            "local REAL-13 sidecar, when present, must be non-empty"
        );
    }

    let report = std::fs::read_to_string(evidence.join("REAL13_MARKET_PRESSURE_PROBE_REPORT.md"))
        .expect("report readable");
    assert!(report.contains("runtime_repo:"));
    assert!(report.contains("CAS path:"));
    assert!(report.contains("ev_decision_trace_total_cas | 10"));
    assert!(report.contains("market_review_summary_cas_count | 10"));
    assert!(report.contains("live_non_scripted_router_tx_count | 0"));
    assert!(report.contains("E2 NOT ACHIEVED"));

    let status = std::fs::read_to_string(
        root.join("handover/directives/2026-05-16_REAL13_STATUS_SYNC_FOR_ARCHITECT.md"),
    )
    .expect("status sync readable");
    assert!(status.contains("architect"));
    assert!(status.contains("E2 NOT ACHIEVED"));
    assert!(status.contains("real13_market_pressure_probe_20260516T071216Z"));
}

#[test]
fn dashboard_has_librarian_materialized_view_section() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dashboard = std::fs::read_to_string(root.join("src/bin/audit_dashboard.rs"))
        .expect("audit_dashboard source readable");
    assert!(dashboard.contains("§REAL-BCAST Librarian Broadcast"));
    assert!(dashboard.contains("librarian_digest_cas_count"));
    assert!(dashboard.contains("librarian_shielding_verdict"));
    assert!(dashboard.contains("dashboard is not truth"));
}
