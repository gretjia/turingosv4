//! Constitution gate — `AggregateReport` is the canonical CLAUDE.md §17
//! Report Standard conformance check for any TB-18B benchmark batch.
//!
//! Wires `wilson_ci.rs` (§B Art. I.2 PPUT statistical signal) +
//! `diversity.rs` (§C Art. II.2.1 exploration/exploitation) into a single
//! aggregate consumer per FR-18B.5 / FR-18B.6 / FR-18B.11.
//!
//! These tests are the executable face of CLAUDE.md §17:
//! ΣPPUT + Mean PPUT(solved) + Wilson 95% CI + halt_distribution + counts +
//! no-fake-accepted-nodes — every line item enforced as ship-block.
//!
//! `FC-trace: Art-I.2 + Art-II.2.1 + FC1-INV1 aggregate`.

use turingosv4::runtime::aggregate_report::{
    AggregateReport, AggregateReportError, PerRunFacts, AGGREGATE_REPORT_SCHEMA_ID,
};
use turingosv4::runtime::diversity::DiversityReport;

fn solved(problem: &str, run: &str, pput: f64) -> PerRunFacts {
    PerRunFacts {
        problem_id: problem.into(),
        run_id: run.into(),
        solved: true,
        halt_reason: "OmegaAccepted".into(),
        attempt_count: 1,
        l4_accepted: 1,
        l4e_rejected: 0,
        capsule_anchored: 0,
        pput,
    }
}

fn unsolved(problem: &str, run: &str, halt: &str, pput: f64) -> PerRunFacts {
    PerRunFacts {
        problem_id: problem.into(),
        run_id: run.into(),
        solved: false,
        halt_reason: halt.into(),
        attempt_count: 1,
        l4_accepted: 0,
        l4e_rejected: 1,
        capsule_anchored: 0,
        pput,
    }
}

/// CLAUDE.md §17 line 1-7 — well-formed batch passes all 7 conformance checks.
#[test]
fn aggregate_report_section_17_well_formed_passes() {
    let runs = vec![
        solved("p1", "r1", 1.0),
        solved("p2", "r2", 2.0),
        unsolved("p3", "r3", "MaxTxExhausted", 0.5),
    ];
    let r = AggregateReport::from_per_run(&runs, true, None);
    r.assert_claude_md_section_17()
        .expect("§17 well-formed batch passes");
    assert_eq!(r.run_count, 3);
    assert_eq!(r.solved_count, 2);
    assert_eq!(r.total_attempts, 3);
    assert_eq!(r.total_l4_accepted, 2);
    assert_eq!(r.total_l4e_rejected, 1);
    assert!(r.fc1_aggregate_attempt_equality_holds);
    assert_eq!(r.fc1_attempt_equality_pass_count, 3);
}

/// CLAUDE.md §17 line 1 — ΣPPUT must be present (and finite).
#[test]
fn aggregate_report_section_17_sigma_pput_finite() {
    let runs = vec![solved("p1", "r1", 1.5), solved("p2", "r2", 2.5)];
    let r = AggregateReport::from_per_run(&runs, true, None);
    assert!((r.sigma_pput - 4.0).abs() < 1e-9);
    assert!(r.sigma_pput.is_finite());
}

/// CLAUDE.md §17 line 2 — Mean PPUT (solved) MUST be present when solved > 0.
#[test]
fn aggregate_report_section_17_mean_pput_solved_present() {
    let runs = vec![
        solved("p1", "r1", 1.0),
        solved("p2", "r2", 3.0),
        unsolved("p3", "r3", "MaxTxExhausted", 99.0), // Excluded from mean
    ];
    let r = AggregateReport::from_per_run(&runs, true, None);
    assert!(r.mean_pput_solved.is_some());
    let mean = r.mean_pput_solved.unwrap();
    assert!((mean - 2.0).abs() < 1e-9, "mean_pput_solved={mean} != 2.0");
}

/// CLAUDE.md §17 line 3 — Wilson 95% CI MUST be present for any non-empty batch.
#[test]
fn aggregate_report_section_17_wilson_ci_present() {
    let runs = vec![solved("p1", "r1", 1.0)];
    let r = AggregateReport::from_per_run(&runs, true, None);
    assert!(r.solve_rate_wilson_95_ci.is_some());
    let ci = r.solve_rate_wilson_95_ci.as_ref().unwrap();
    assert_eq!(ci.successes, 1);
    assert_eq!(ci.trials, 1);
    assert!(ci.lower >= 0.0);
    assert!(ci.upper <= 1.0);
    assert!(ci.point >= ci.lower && ci.point <= ci.upper);
}

/// CLAUDE.md §17 line 4 — halt_reason_distribution MUST be present.
#[test]
fn aggregate_report_section_17_halt_distribution_present() {
    let runs = vec![
        solved("p1", "r1", 1.0),
        unsolved("p2", "r2", "MaxTxExhausted", 0.5),
        unsolved("p3", "r3", "ParseFailed", 0.5),
        unsolved("p4", "r4", "MaxTxExhausted", 0.5),
    ];
    let r = AggregateReport::from_per_run(&runs, true, None);
    assert_eq!(r.halt_reason_distribution.len(), 3);
    assert_eq!(*r.halt_reason_distribution.get("OmegaAccepted").unwrap(), 1);
    assert_eq!(
        *r.halt_reason_distribution.get("MaxTxExhausted").unwrap(),
        2
    );
    assert_eq!(*r.halt_reason_distribution.get("ParseFailed").unwrap(), 1);
}

/// CLAUDE.md §17 line 5-6 — proposal/attempt + accepted/rejected counts present.
#[test]
fn aggregate_report_section_17_counts_present() {
    let runs = vec![
        solved("p1", "r1", 1.0),
        solved("p2", "r2", 1.0),
        unsolved("p3", "r3", "MaxTxExhausted", 0.5),
    ];
    let r = AggregateReport::from_per_run(&runs, true, None);
    assert_eq!(r.total_attempts, 3);
    assert_eq!(r.total_l4_accepted, 2);
    assert_eq!(r.total_l4e_rejected, 1);
}

/// CLAUDE.md §17 line 7 — no_fake_accepted_nodes MUST be true; false blocks ship.
#[test]
fn aggregate_report_section_17_no_fake_accepted_required() {
    let runs = vec![solved("p1", "r1", 1.0)];
    let r_false = AggregateReport::from_per_run(&runs, false, None);
    assert_eq!(
        r_false.assert_claude_md_section_17(),
        Err(AggregateReportError::NoFakeAcceptedNotProven)
    );
    let r_true = AggregateReport::from_per_run(&runs, true, None);
    r_true
        .assert_claude_md_section_17()
        .expect("no_fake_accepted=true passes");
}

/// FR-18B.11 — FC1 aggregate invariant `total_attempts == l4 + l4e + capsule`
/// is a ship-block at gate level.
#[test]
fn aggregate_report_fc1_aggregate_invariant_is_ship_block() {
    let bad = vec![PerRunFacts {
        problem_id: "p1".into(),
        run_id: "r1".into(),
        solved: true,
        halt_reason: "OmegaAccepted".into(),
        attempt_count: 10, // Deliberately broken
        l4_accepted: 1,
        l4e_rejected: 0,
        capsule_anchored: 0,
        pput: 1.0,
    }];
    let r = AggregateReport::from_per_run(&bad, true, None);
    match r.assert_claude_md_section_17() {
        Err(AggregateReportError::Fc1AggregateInvariantBroken { attempts, rhs }) => {
            assert_eq!(attempts, 10);
            assert_eq!(rhs, 1);
        }
        other => panic!("expected Fc1AggregateInvariantBroken, got {other:?}"),
    }
}

/// Wilson CI + Diversity wire — `aggregate_report` consumes BOTH helpers.
/// Closes session #18 Wave-1/2 forward-bind items 1+2 at gate level.
#[test]
fn aggregate_report_wires_wilson_and_diversity() {
    let runs = vec![
        solved("p1", "r1", 1.0),
        solved("p2", "r2", 2.0),
        unsolved("p3", "r3", "MaxTxExhausted", 0.5),
    ];
    let parents: Vec<Option<u32>> = vec![Some(1), Some(2), Some(3)];
    let payloads: Vec<&[u8]> = vec![b"a", b"b", b"c"];
    let dr = DiversityReport::new(&parents, &payloads);
    let r = AggregateReport::from_per_run(&runs, true, Some(dr));
    // Wilson CI is present (closes Art. I.2 wire-up at consumer side)
    assert!(r.solve_rate_wilson_95_ci.is_some());
    // Diversity is present (closes Art. II.2.1 wire-up at consumer side)
    assert!(r.diversity.is_some());
    let d = r.diversity.as_ref().unwrap();
    assert_eq!(d.proposal_count, 3);
    assert!(d.parent_selection_entropy_bits > 0.0);
    assert!(d.pairwise_payload_diversity > 0.0);
    assert!(!d.below_alarm_floor);
    r.assert_claude_md_section_17()
        .expect("§17 conformant with diversity");
}

/// Schema id pin at gate level.
#[test]
fn aggregate_report_schema_id_pin_is_constitutional() {
    assert_eq!(
        AGGREGATE_REPORT_SCHEMA_ID, "turingosv4.aggregate_report.v1",
        "Constitution gate: AGGREGATE_REPORT_SCHEMA_ID drift caught at gate level."
    );
}

/// Round-trip JSON stability — aggregate report serializes/deserializes byte-stable.
#[test]
fn aggregate_report_disk_format_stable() {
    let runs = vec![solved("p1", "r1", 1.0), solved("p2", "r2", 2.0)];
    let r = AggregateReport::from_per_run(&runs, true, None);
    let body1 = r.to_json_pretty().expect("to_json");
    let r2: AggregateReport = serde_json::from_str(&body1).expect("from_json");
    let body2 = r2.to_json_pretty().expect("to_json2");
    assert_eq!(body1, body2);
}
