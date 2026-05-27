//! OBL-004 repair reconciliation gate.
//!
//! This is not a substitute for the underlying constitutional gates. It
//! prevents a stale obligation ledger from reintroducing PR-B/PR-C/PR-D/PR-E
//! placeholders as blockers after current source/tests/PR receipts have been
//! reconciled.

use std::fs;
use std::path::Path;

const LEDGER: &str = "OBLIGATIONS.md";
const AUDIT: &str = "handover/audits/OBL004_REPAIR_RECONCILIATION_2026-05-27.md";
const CLEAN_AUDIT: &str =
    "handover/audits/OBL004_REPAIR_RECONCILIATION_CLEAN_CONTEXT_AUDIT_2026-05-27.md";

fn section<'a>(text: &'a str, heading: &str, next_heading: &str) -> &'a str {
    let start = text
        .find(heading)
        .unwrap_or_else(|| panic!("missing heading {heading}"));
    let tail = &text[start..];
    let end = tail
        .find(next_heading)
        .unwrap_or_else(|| panic!("missing next heading {next_heading}"));
    &tail[..end]
}

#[test]
fn obl004_ledger_is_closed_by_current_reconciliation() {
    let ledger = fs::read_to_string(LEDGER).expect("read OBLIGATIONS.md");
    let obl004 = section(&ledger, "## OBL-004:", "## OBL-005:");
    let headline = ledger
        .lines()
        .find(|line| line.starts_with("Current overall status:"))
        .expect("ledger must include current overall status");

    assert!(
        headline.contains("OBL-004 satisfied")
            || (headline.contains("COMPLETE") && headline.contains("OBL-004")),
        "ledger headline must agree with the OBL-004 section status (directly or via global COMPLETE)"
    );
    assert!(
        !headline.contains("OBL-004 in-progress"),
        "ledger headline must not keep stale OBL-004 in-progress status"
    );
    assert!(
        obl004.contains("- Status: satisfied"),
        "OBL-004 must be closed only after current reconciliation evidence lands"
    );
    assert!(
        obl004.contains(AUDIT),
        "OBL-004 must cite the current reconciliation audit"
    );
    assert!(
        obl004.contains(CLEAN_AUDIT),
        "OBL-004 must cite the clean-context reconciliation witness"
    );
    for stale in [
        "PR-B: `constitution-repair/wave1-pr-b-shielding-judge` branch + merged PR",
        "PR-C: `constitution-repair/wave1-pr-c-librarian-disjointness` branch + merged PR",
        "PR-D: `constitution-repair/wave2-pr-d-bus-cleanup-node-retire` branch + merged PR",
        "PR-E (NEW for 完全修复): `constitution-repair/wave1-pr-e-build-agent-prompt-retire` branch + merged PR",
        "CONSTITUTION_REPAIR_R1R2R3_SYNTHESIS_2026-05-24.md",
    ] {
        assert!(
            !obl004.contains(stale),
            "stale OBL-004 placeholder remains after reconciliation: {stale}"
        );
    }
    for receipt in ["PR #139", "PR #140", "PR #184", "PR #192"] {
        assert!(
            obl004.contains(receipt),
            "OBL-004 reconciliation must cite merged receipt {receipt}"
        );
    }
}

#[test]
fn obl004_reconciliation_audit_is_current_not_backfilled() {
    assert!(Path::new(AUDIT).exists(), "missing {AUDIT}");
    assert!(Path::new(CLEAN_AUDIT).exists(), "missing {CLEAN_AUDIT}");
    let audit = fs::read_to_string(AUDIT).expect("read reconciliation audit");
    let clean_audit = fs::read_to_string(CLEAN_AUDIT).expect("read clean-context audit");

    assert!(
        audit.contains("OBL004-RECONCILED-NO-UNRESOLVED-VIOLATION"),
        "audit must end with an explicit OBL-004 reconciliation verdict"
    );
    assert!(
        audit.contains("does not retroactively fabricate"),
        "audit must state that it is a current reconciliation, not a backfilled 2026-05-24 synthesis"
    );
    assert!(
        audit.contains("build_agent_prompt") && audit.contains("active retained surface"),
        "audit must explain why old PR-E deletion wording is superseded"
    );
    assert!(
        clean_audit.contains("NO-VIOLATION")
            && clean_audit.contains("OBLIGATIONS.md:9")
            && clean_audit.contains("headline consistency"),
        "clean-context audit must record the status-line remediation and final verdict"
    );
}

#[test]
fn obl005_no_longer_names_obl004_placeholders_as_blockers() {
    let ledger = fs::read_to_string(LEDGER).expect("read OBLIGATIONS.md");
    let obl005 = &ledger[ledger
        .find("## OBL-005:")
        .expect("OBL-005 section must exist")..];

    assert!(
        !obl005.contains("OBL-004 PR-B/PR-C/PR-D/PR-E ledger items must be reconciled"),
        "OBL-005 must not name stale OBL-004 placeholders as its live blocker after reconciliation"
    );
    assert!(
        obl005.contains(AUDIT),
        "OBL-005 blocker paragraph must cite the OBL-004 reconciliation audit"
    );
}
