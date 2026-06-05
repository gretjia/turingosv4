use std::fs;

const LADDER_MANIFEST: &str = "handover/directives/tc_ladder_2026-06-04/L0_L5_MANIFEST.yaml";

fn ladder_manifest() -> String {
    fs::read_to_string(LADDER_MANIFEST).expect("TC-019A L0-L5 manifest exists")
}

fn section<'a>(body: &'a str, header: &str, next_headers: &[&str]) -> &'a str {
    let start = body.find(header).expect("section header exists");
    let after = &body[start..];
    let end = next_headers
        .iter()
        .filter_map(|next| {
            after[header.len()..]
                .find(next)
                .map(|idx| header.len() + idx)
        })
        .min()
        .unwrap_or(after.len());
    &after[..end]
}

#[test]
fn ladder_manifest_has_l0_through_l5() {
    let body = ladder_manifest();

    assert!(body.contains("manifest_id: tc_l0_l5_task_bank_v1"));
    assert!(body.contains("freeze_state: frozen"));
    assert!(body.contains("levels:"));

    for level in ["L0", "L1", "L2", "L3", "L4", "L5"] {
        let marker = format!("- id: {level}");
        assert!(body.contains(&marker), "missing level marker {marker}");
    }

    assert_eq!(body.matches("- id: L").count(), 6);
    assert!(!body.contains("TBD"));
    assert!(!body.contains("TODO"));
}

#[test]
fn l5_entry_requires_l0_l4_green_receipts() {
    let body = ladder_manifest();
    let gate = section(&body, "l5_entry_gate:", &["hard_claim_c_gate:"]);

    assert!(gate.contains("gate_id: L5_ENTRY_LADDER_GATE"));
    assert!(gate.contains("mode: fail_closed"));
    assert!(gate.contains("required_green_receipts:"));
    for level in ["L0", "L1", "L2", "L3", "L4"] {
        let marker = format!("- {level}");
        assert!(
            gate.contains(&marker),
            "missing green receipt requirement {marker}"
        );
    }
    assert!(!gate.contains("- L5"));
}

#[test]
fn hard_claim_c_cannot_start_without_ladder_gate() {
    let body = ladder_manifest();
    let gate = section(&body, "hard_claim_c_gate:", &["claim_language:"]);

    assert!(gate.contains("claim_id: Claim C"));
    assert!(gate.contains("start_requires_gate: L5_ENTRY_LADDER_GATE"));
    assert!(gate.contains("when_gate_missing: blocked"));
    assert!(gate.contains("allowed_report_mode: descriptive_only"));
    assert!(gate.contains("claim_headline: forbidden"));
}
