use std::fs;
use std::path::Path;

const CLEAN_EVIDENCE: &str = "handover/evidence/real8x_market_ab_clean_20260515T141331Z";
const CONTAMINATED_EVIDENCE: &str = "handover/evidence/real8x_market_ab_20260515T134453Z";
const RATIFICATION: &str = "handover/directives/2026-05-15_REAL10_NARROW_RATIFICATION_REAL11.md";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn real10_ratification_marks_clean_evidence_as_only_conclusion_bearing() {
    let text = read(RATIFICATION);

    assert!(
        text.contains(CLEAN_EVIDENCE),
        "SG-11.0.3: ratification must name the clean canonical evidence directory"
    );
    assert!(
        text.contains("only") && text.contains("conclusion-bearing"),
        "SG-11.0.3: ratification must say the clean evidence is the only conclusion-bearing evidence"
    );
    assert!(
        text.contains(CONTAMINATED_EVIDENCE),
        "SG-11.0.2: ratification must name the contaminated evidence directory"
    );
    assert!(
        text.contains("invalid for conclusions") || text.contains("remediation-only"),
        "SG-11.0.2: contaminated evidence must be invalid/remediation-only"
    );
    assert!(
        text.contains("buy_with_coin_router=0 in all arms"),
        "ratification must preserve the REAL-10 buy_with_coin_router=0 boundary"
    );
    assert!(
        text.contains("E2") && text.contains("not achieved"),
        "ratification must preserve the E2 non-claim"
    );
}

#[test]
fn contaminated_evidence_appears_only_in_invalid_or_remediation_context() {
    let scan_scope = [
        RATIFICATION,
        "handover/reports/REAL10_DECISION_GATE_REPORT.md",
        "handover/reports/REAL10_VERIFICATION_SUMMARY.md",
        "handover/evidence/real8x_market_ab_clean_20260515T141331Z/REAL10_DECISION_GATE_REPORT.md",
        "handover/reports/REAL11_MARKET_TX_CATEGORY_REPORT.md",
        "handover/ai-direct/LATEST.md",
        "handover/tracer_bullets/TB_LOG.tsv",
    ];
    let allowed_markers = [
        "invalid",
        "contamination",
        "contaminated",
        "remediation-only",
        "excluded",
        "not conclusion-bearing",
        "cannot",
    ];

    for path in scan_scope {
        if !Path::new(path).exists() {
            continue;
        }
        let text = read(path);
        let lines: Vec<&str> = text.lines().collect();
        for (idx, line) in lines.iter().enumerate() {
            if !line.contains(CONTAMINATED_EVIDENCE) {
                continue;
            }
            let start = idx.saturating_sub(2);
            let end = usize::min(idx + 3, lines.len());
            let context = lines[start..end].join("\n").to_lowercase();
            assert!(
                allowed_markers.iter().any(|marker| context.contains(marker)),
                "contaminated evidence path in {path}:{} lacks invalid/remediation/excluded context:\n{context}",
                idx + 1
            );
        }
    }
}

#[test]
fn stale_parent_behavioral_gate_is_present() {
    let test_text = read("tests/constitution_real8_market_ab_benchmark.rs");
    assert!(
        test_text.contains("real8_task_outcome_arm_refreshes_verify_parent_behaviorally"),
        "SG-11.0.4: stale-parent behavioral test must exist or a forward OBS must be created"
    );
}
