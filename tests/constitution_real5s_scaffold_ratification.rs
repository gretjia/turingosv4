//! REAL-5S — scaffold ratification / clean-negative closure gates.
//!
//! These tests bind the architect's 2026-05-15 REAL-5S route:
//! REAL-5 proves role scaffolding; REAL-5 does not prove market emergence.

use std::fs;
use std::path::PathBuf;

fn repo_file(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path)
}

fn read(path: &str) -> String {
    fs::read_to_string(repo_file(path)).unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn sg_5s_scaffold_ratification_report_narrows_real5_claim() {
    let report =
        read("handover/evidence/real5_overnight_20260514/REAL5_SCAFFOLD_RATIFICATION_REPORT.md");

    for required in [
        "role gateway blocks Trader proof-style leakage",
        "Verifier behavior observed",
        "Trader buy=0",
        "NoPool dominates",
        "No E2/E3 claim",
        "REAL-5 proves role scaffolding.",
        "REAL-5 does not prove market emergence.",
    ] {
        assert!(
            report.contains(required),
            "REAL-5S scaffold report must contain architect-required text: {required}"
        );
    }

    assert!(
        report.contains("g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z"),
        "report must cite post-VETO trader-first role-gateway evidence"
    );
    assert!(
        report.contains("CODEX_REAL5_IMPLEMENTATION_REVIEW_R3.md"),
        "report must cite final clean-context Codex R3 PROCEED audit"
    );
    assert!(
        !report.contains("REAL-5 proves market emergence"),
        "REAL-5S must not overclaim market emergence"
    );
}

#[test]
fn sg_5s_clean_negative_report_records_no_trade_diagnosis() {
    let report =
        read("handover/evidence/real5_overnight_20260514/REAL5_CLEAN_NEGATIVE_NO_TRADE_REPORT.md");

    for required in [
        "Why no trade?",
        "NoPool dominates.",
        "Post-accept node market timing too late.",
        "Prompt-only exhausted.",
        "Trader buy=0",
        "No E2/E3 claim",
    ] {
        assert!(
            report.contains(required),
            "REAL-5S clean-negative report must contain architect-required text: {required}"
        );
    }

    assert!(
        report.contains("REAL-6"),
        "clean-negative report must point to REAL-6 event timing rather than more prompt variants"
    );
    assert!(
        !report.contains("forced trade") && !report.contains("must trade"),
        "clean-negative report must not recommend forced trading"
    );
}
