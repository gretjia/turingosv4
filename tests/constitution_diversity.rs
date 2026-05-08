//! Constitution gate: Art. II.2.1 — exploration / exploitation balance.
//!
//! Closes the AMBER row "Art. II.2.1 exploration / exploitation balance" in
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §C.
//!
//! Kill condition (per matrix): `parent_selection_entropy < 0.25` OR
//! `pairwise_payload_diversity_mean < 0.25` — V3L-14 star-topology collapse.
//!
//! Production-grade Boltzmann audit at `audit_assertions::assert_e_boltzmann_parent_selection_diversity`
//! (id=43) uses the same Shannon entropy formula with a stricter ship
//! threshold (≥ 0.5). This test exercises the kill-condition floor.
//!
//! `FC-trace: Art.II.2.1 — exploration / exploitation balance`.

use turingosv4::runtime::diversity::{
    distinct_payload_fraction, parent_selection_shannon_entropy, DiversityReport,
};

#[test]
fn parent_entropy_helper_exists() {
    let parents: Vec<Option<u32>> = vec![Some(1), Some(2)];
    let h = parent_selection_shannon_entropy(&parents);
    assert!(h > 0.0);
}

#[test]
fn payload_diversity_helper_exists() {
    let payloads = vec![1u32, 2, 3];
    let d = distinct_payload_fraction(&payloads);
    assert_eq!(d, 1.0);
}

#[test]
fn parent_entropy_collapses_to_zero_on_star_topology() {
    // V3L-14 anti-pattern: 1 root + N same-non-root parents.
    let parents: Vec<Option<u32>> =
        vec![None, Some(1), Some(1), Some(1), Some(1), Some(1)];
    let h = parent_selection_shannon_entropy(&parents);
    assert_eq!(h, 0.0, "star-topology MUST collapse to 0 after None-filter");
}

#[test]
fn parent_entropy_below_alarm_floor_at_extreme_collapse() {
    // 99 same parent + 1 distinct → entropy ~0.081 bits, below 0.25 floor.
    let mut parents: Vec<Option<u32>> = (0..99).map(|_| Some(1u32)).collect();
    parents.push(Some(2));
    let h = parent_selection_shannon_entropy(&parents);
    assert!(
        h < 0.25,
        "99:1 parent collapse MUST trip Art.II.2.1 alarm (got {})",
        h
    );
}

#[test]
fn payload_diversity_below_alarm_floor_when_all_collapsed() {
    // 5 proposals all same payload → fraction = 0.2 < 0.25 floor.
    let payloads = vec![42u32; 5];
    let d = distinct_payload_fraction(&payloads);
    assert!(
        d < 0.25,
        "fully-collapsed payloads MUST trip Art.II.2.1 alarm (got {})",
        d
    );
}

#[test]
fn diversity_report_alarms_on_either_metric_collapse() {
    // Healthy parents + collapsed payloads → alarm on payload only.
    let parents: Vec<Option<u32>> =
        vec![Some(1), Some(2), Some(3), Some(4), Some(5)];
    let payloads = vec![10u32; 5];
    let r = DiversityReport::new(&parents, &payloads);
    assert!(
        r.is_below_alarm_floor(),
        "either-metric alarm MUST trip on payload collapse"
    );

    // Collapsed parents + healthy payloads → alarm on parent only.
    let mut parents_collapsed: Vec<Option<u32>> =
        (0..99).map(|_| Some(1u32)).collect();
    parents_collapsed.push(Some(2));
    let payloads_distinct: Vec<u32> = (0..100).collect();
    let r2 = DiversityReport::new(&parents_collapsed, &payloads_distinct);
    assert!(
        r2.is_below_alarm_floor(),
        "either-metric alarm MUST trip on parent collapse"
    );
}

#[test]
fn diversity_report_does_not_alarm_when_both_above_floor() {
    let parents: Vec<Option<u32>> =
        vec![Some(1), Some(1), Some(2), Some(2), Some(3)];
    let payloads = vec![10u32, 20, 30, 40, 50];
    let r = DiversityReport::new(&parents, &payloads);
    assert!(
        !r.is_below_alarm_floor(),
        "healthy distribution MUST NOT trip alarm (entropy {} diversity {})",
        r.parent_selection_entropy_bits,
        r.pairwise_payload_diversity,
    );
}
