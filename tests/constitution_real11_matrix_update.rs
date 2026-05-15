use std::fs;

#[test]
fn real11_decision_gate_records_b_c_diagnostic_clean_negative() {
    let report = fs::read_to_string("handover/reports/REAL11_DECISION_GATE_REPORT.md")
        .expect("REAL-11 decision gate report exists");

    for required in [
        "REAL-11 takes a B/C diagnostic branch",
        "No live non-scripted router tx was observed.",
        "The router substrate is not the immediate blocker.",
        "scripted TaskOutcome buys forbidden by runner = PASS",
        "Patched no-scripted-contamination runs did not schedule an actionable Trader turn.",
        "Supplemental diagnostic evidence shows actionable_markets > 0",
        "actionable_markets > 0",
        "NoTradeReason is no_perceived_edge, not no_pool.",
        "Live Trader activation / objective-routing redesign",
        "Do not jump directly to live REAL-6B",
        "E2 spontaneous market action",
        "not used as REAL-11 Atom 5 sentinels",
    ] {
        assert!(
            report.contains(required),
            "REAL-11 decision report missing required finding: {required}"
        );
    }
}

#[test]
fn real11_traceability_update_maps_fc_and_kill_conditions() {
    let update = fs::read_to_string("handover/alignment/REAL11_TRACE_MATRIX_UPDATE.md")
        .expect("REAL-11 traceability update exists");

    for required in [
        "FC1: externalized role/economic action loop",
        "FC2: Trust Root / replay authority",
        "FC3: dashboard/report as materialized view",
        "src/runtime/market_tx_category.rs",
        "src/runtime/market_opportunity_trace.rs",
        "scripts/run_real11_e2_micro_probe.sh",
        "scripted fixture is counted as E2",
        "live REAL-6B is enabled without separate Class-4 ratification",
    ] {
        assert!(
            update.contains(required),
            "REAL-11 traceability update missing required binding: {required}"
        );
    }
}

#[test]
fn constitution_execution_matrix_tracks_real11_boundary() {
    let matrix = fs::read_to_string("handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md")
        .expect("Constitution Execution Matrix exists");

    for required in [
        "**REAL-11** Agent Economic Action Activation",
        "patched E2 micro-probe `handover/evidence/real11_e2_micro_probe_20260515T172707Z_r2_max10/`",
        "supplemental actionable-opportunity diagnostic",
        "patched probe has no live buy or scripted contamination",
        "live Trader activation / objective-routing redesign",
        "E2 not achieved",
        "scripted fixture counted as E2",
        "live REAL-6B enabled without separate Class-4 packet",
    ] {
        assert!(
            matrix.contains(required),
            "Constitution Execution Matrix missing REAL-11 boundary: {required}"
        );
    }
}
