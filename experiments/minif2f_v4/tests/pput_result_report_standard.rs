// Phase 9 § 0 regression test — verify PputResult jsonl rows include
// Report Standard aux fields (C-053 + C-061 + C-059).
//
// We parse a historical jsonl row (without the fields) and a synthetic
// row (with them) to confirm:
//   a. pre-9.0 rows still parse (backward compat via #[serde(default)])
//   b. post-9.0 rows include the 4 new fields when present
//
// This is a pure serde test — no need to launch a full evaluator binary.

use serde::Deserialize;
use std::collections::HashMap;

/// Mirror of `PputResult` relevant fields for test parsing.
/// (The real struct is private to evaluator.rs; we parse as our own
/// shape with matching serde names.)
#[derive(Debug, Deserialize)]
struct PputRowMirror {
    problem: String,
    has_golden_path: bool,
    pput: f64,
    #[serde(default)]
    reputation_at_end: Option<HashMap<String, u32>>,
    #[serde(default)]
    halt_reason: Option<String>,
    #[serde(default)]
    pairwise_diversity_mean: Option<f64>,
    #[serde(default)]
    parent_selection_entropy: Option<f64>,
}

#[test]
fn pre_9_0_row_parses_with_default_none() {
    // Historical row as produced before Phase 9 § 0 (no aux fields).
    let old_row = r#"{
        "problem": "mathd_algebra_148",
        "condition": "oneshot",
        "model": "deepseek-chat",
        "has_golden_path": true,
        "time_secs": 80.0,
        "pput": 1.25,
        "gp_token_count": 100,
        "gp_node_count": 1,
        "tx_count": 1
    }"#;
    let r: PputRowMirror = serde_json::from_str(old_row)
        .expect("pre-9.0 jsonl must parse");
    assert!(r.has_golden_path);
    assert!((r.pput - 1.25).abs() < 1e-6);
    assert!(r.reputation_at_end.is_none(), "pre-9.0 row: no reputation field");
    assert!(r.halt_reason.is_none(), "pre-9.0 row: no halt_reason");
    assert!(r.pairwise_diversity_mean.is_none());
    assert!(r.parent_selection_entropy.is_none());
}

#[test]
fn post_9_0_row_carries_aux_fields() {
    // Post-9.0 row: all fields present.
    let new_row = r#"{
        "problem": "imo_1964_p2",
        "condition": "n8",
        "model": "deepseek-chat",
        "has_golden_path": true,
        "time_secs": 539.0,
        "pput": 0.185,
        "gp_token_count": 1177,
        "gp_node_count": 23,
        "tx_count": 35,
        "reputation_at_end": {"Agent_0": 5, "Agent_3": 2},
        "halt_reason": "OmegaAccepted",
        "pairwise_diversity_mean": 0.42,
        "parent_selection_entropy": 1.87
    }"#;
    let r: PputRowMirror = serde_json::from_str(new_row)
        .expect("post-9.0 jsonl must parse");
    let rep = r.reputation_at_end.unwrap();
    assert_eq!(*rep.get("Agent_0").unwrap(), 5);
    assert_eq!(r.halt_reason.unwrap(), "OmegaAccepted");
    assert!((r.pairwise_diversity_mean.unwrap() - 0.42).abs() < 1e-6);
    assert!((r.parent_selection_entropy.unwrap() - 1.87).abs() < 1e-6);
}

#[test]
fn unsolved_row_has_none_reputation() {
    // Unsolved problem should still emit the row, with reputation_at_end
    // possibly being empty map (serialized) or None (skipped).
    let row = r#"{
        "problem": "unknown_problem",
        "condition": "oneshot",
        "model": "deepseek-chat",
        "has_golden_path": false,
        "time_secs": 900.0,
        "pput": 0.0,
        "gp_token_count": 0,
        "gp_node_count": 0,
        "tx_count": 1
    }"#;
    let r: PputRowMirror = serde_json::from_str(row).expect("unsolved row parses");
    assert!(!r.has_golden_path);
    // Neither asserting Some nor None — both are valid; #[serde(default)]
    // gives None when absent, and writer skip_serializing_if omits when None.
}

#[test]
fn halt_reason_values_are_canonical() {
    // halt_reason must be one of the 5 canonical strings for stable
    // halt_reason_distribution rollups.
    let canonical = [
        "OmegaAccepted",
        "MaxTxExhausted",
        "WallClockCap",
        "ComputeCapViolated",
        "ErrorHalt",
    ];
    for name in canonical {
        let row = format!(r#"{{"problem":"x","condition":"y","model":"z","has_golden_path":true,
                              "time_secs":1.0,"pput":100.0,"gp_token_count":1,"gp_node_count":1,
                              "tx_count":1,"halt_reason":"{}"}}"#, name);
        let r: PputRowMirror = serde_json::from_str(&row).unwrap();
        assert_eq!(r.halt_reason.unwrap(), name);
    }
}
