use std::fs;
use std::path::Path;

#[test]
fn real11_launch_and_beta_forbidden_claims_are_absent_or_explicitly_forbidden() {
    let scan_scope = [
        "handover/directives/2026-05-15_REAL10_NARROW_RATIFICATION_REAL11.md",
        "handover/alignment/DECISION_REAL11_MARKET_TX_CATEGORY.md",
        "handover/reports/REAL11_MARKET_TX_CATEGORY_REPORT.md",
    ];
    let forbidden_claims = [
        "autonomous prediction market",
        "emergent agent economy",
        "market-proven performance improvement",
        "real-world readiness",
        "agent economy beta",
        "emergent market beta",
        "launch-ready",
        "public launch",
    ];
    let allowed_context = [
        "forbidden",
        "not allowed",
        "not achieved",
        "does not ratify",
        "requires e2",
        "requires e3",
        "without e2",
        "without e3",
        "no ",
    ];

    for path in scan_scope {
        if !Path::new(path).exists() {
            continue;
        }
        let text = fs::read_to_string(path).expect("scan target readable");
        let lower_lines: Vec<String> = text.lines().map(|line| line.to_lowercase()).collect();
        for (idx, line) in lower_lines.iter().enumerate() {
            for claim in forbidden_claims {
                if !line.contains(claim) {
                    continue;
                }
                let start = idx.saturating_sub(2);
                let end = usize::min(idx + 3, lower_lines.len());
                let context = lower_lines[start..end].join("\n");
                assert!(
                    allowed_context.iter().any(|marker| context.contains(marker)),
                    "{path}:{} uses forbidden launch/beta claim `{claim}` outside explicit forbidden/threshold context:\n{context}",
                    idx + 1
                );
            }
        }
    }
}
