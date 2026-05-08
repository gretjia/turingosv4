//! Diversity metrics for selective broadcast / exploration-exploitation balance.
//!
//! Closes Art. II.2.1 AMBER row in `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
//! §C. Kill condition (per matrix): "`parent_selection_entropy < 0.25` OR
//! `pairwise_payload_diversity_mean < 0.25`" — i.e. exploration collapses to a
//! single parent or proposals collapse to a single payload (V3L-14 star-topology
//! anti-pattern).
//!
//! These are pure helper functions over already-decoded data. The production
//! Boltzmann audit (id=43 `assert_e_boltzmann_parent_selection_diversity` in
//! `audit_assertions.rs`) uses the same Shannon entropy formula with a stricter
//! ship threshold (≥ 0.5 bits over non-None subset). This module exposes the
//! computation as a reusable helper for aggregate-report wiring.
//!
//! `FC-trace: Art.II.2.1 — exploration / exploitation balance`.

use std::collections::HashMap;
use std::hash::Hash;

/// Shannon entropy (in bits) of a discrete distribution given by occurrence
/// counts. Returns 0.0 for empty input or single-category distributions.
///
/// `H = -Σ p_i * log2(p_i)`
fn shannon_entropy_from_counts(counts: &[usize]) -> f64 {
    let total: usize = counts.iter().sum();
    if total == 0 {
        return 0.0;
    }
    let n = total as f64;
    let mut h = 0.0;
    for &c in counts {
        if c == 0 {
            continue;
        }
        let p = c as f64 / n;
        h -= p * p.log2();
    }
    h
}

/// TRACE_MATRIX § 3 orphan (Constitution Landing 2026-05-08; Art. II.2.1): Shannon entropy of parent selection over a set of proposal parent references. `None` entries (root proposals) are FILTERED OUT before computation per the V3L-14 anti-pattern fix in `audit_assertions.rs::assert_e_boltzmann_parent_selection_diversity` id=43 (a star topology with one root + many same-non-root parents was masking genuine collapse).
///
/// Returns 0.0 when the non-None subset has fewer than 2 distinct parents.
pub fn parent_selection_shannon_entropy<P: Eq + Hash + Clone>(
    parents: &[Option<P>],
) -> f64 {
    let non_none: Vec<P> = parents.iter().filter_map(|p| p.clone()).collect();
    if non_none.len() < 2 {
        return 0.0;
    }
    let mut counts: HashMap<P, usize> = HashMap::new();
    for p in &non_none {
        *counts.entry(p.clone()).or_insert(0) += 1;
    }
    let counts_vec: Vec<usize> = counts.into_values().collect();
    shannon_entropy_from_counts(&counts_vec)
}

/// TRACE_MATRIX § 3 orphan (Constitution Landing 2026-05-08; Art. II.2.1): pairwise payload diversity. Fraction of distinct payload identifiers in the input set. Returns 0.0 for empty input, 1.0 for all-distinct, 1/n for all-identical (n payloads collapsed to one).
///
/// This is the simplest mechanical proxy for diversity. Richer metrics
/// (Jaccard over tokens, Hamming over normalized AST) are forward.
pub fn distinct_payload_fraction<P: Eq + Hash + Clone>(payloads: &[P]) -> f64 {
    if payloads.is_empty() {
        return 0.0;
    }
    let mut seen: HashMap<P, ()> = HashMap::new();
    for p in payloads {
        seen.insert(p.clone(), ());
    }
    seen.len() as f64 / payloads.len() as f64
}

/// TRACE_MATRIX § 3 orphan (Constitution Landing 2026-05-08; Art. II.2.1): diversity report for one task — parent-selection entropy + payload diversity. Both metrics MUST be ≥ 0.25 per Art. II.2.1 kill condition. The Boltzmann production audit uses ≥ 0.5 entropy (stricter ship threshold).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DiversityReport {
    pub proposal_count: usize,
    pub parent_selection_entropy_bits: f64,
    pub pairwise_payload_diversity: f64,
}

impl DiversityReport {
    /// TRACE_MATRIX § 3 orphan (Art. II.2.1): construct from parent + payload slices.
    pub fn new<P, C>(parents: &[Option<P>], payloads: &[C]) -> Self
    where
        P: Eq + Hash + Clone,
        C: Eq + Hash + Clone,
    {
        Self {
            proposal_count: parents.len(),
            parent_selection_entropy_bits: parent_selection_shannon_entropy(
                parents,
            ),
            pairwise_payload_diversity: distinct_payload_fraction(payloads),
        }
    }

    /// TRACE_MATRIX § 3 orphan (Art. II.2.1): alarm threshold check at 0.25 floor on either metric.
    pub fn is_below_alarm_floor(&self) -> bool {
        self.parent_selection_entropy_bits < 0.25
            || self.pairwise_payload_diversity < 0.25
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shannon_entropy_uniform_two_categories_is_one_bit() {
        let h = shannon_entropy_from_counts(&[5, 5]);
        assert!((h - 1.0).abs() < 1e-9);
    }

    #[test]
    fn shannon_entropy_single_category_is_zero() {
        let h = shannon_entropy_from_counts(&[10]);
        assert_eq!(h, 0.0);
    }

    #[test]
    fn shannon_entropy_empty_is_zero() {
        let h = shannon_entropy_from_counts(&[]);
        assert_eq!(h, 0.0);
    }

    #[test]
    fn parent_entropy_filters_none_root_proposals() {
        // 1 root + 3 same-non-root → all non-None proposals cite same parent
        // → non-None entropy = 0 (V3L-14 star topology).
        let parents: Vec<Option<u32>> = vec![None, Some(1), Some(1), Some(1)];
        let h = parent_selection_shannon_entropy(&parents);
        assert_eq!(
            h, 0.0,
            "star-topology should collapse to 0 entropy after None-filter"
        );
    }

    #[test]
    fn parent_entropy_balanced_two_parents_is_one_bit() {
        let parents: Vec<Option<u32>> =
            vec![Some(1), Some(1), Some(2), Some(2)];
        let h = parent_selection_shannon_entropy(&parents);
        assert!((h - 1.0).abs() < 1e-9);
    }

    #[test]
    fn parent_entropy_below_alarm_floor_when_collapsed() {
        // 9 of 10 same parent + 1 distinct → entropy ~0.469 bits
        let parents: Vec<Option<u32>> = vec![
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(2),
        ];
        let h = parent_selection_shannon_entropy(&parents);
        assert!(
            h > 0.25,
            "9:1 split should be above alarm floor (got {})",
            h
        );
        // But 99:1 split should fall below 0.25:
        let mut parents99: Vec<Option<u32>> =
            (0..99).map(|_| Some(1u32)).collect();
        parents99.push(Some(2));
        let h99 = parent_selection_shannon_entropy(&parents99);
        assert!(
            h99 < 0.25,
            "99:1 split should be below alarm floor (got {})",
            h99
        );
    }

    #[test]
    fn distinct_payload_fraction_all_distinct_is_one() {
        let payloads = vec![1u32, 2, 3, 4, 5];
        let d = distinct_payload_fraction(&payloads);
        assert_eq!(d, 1.0);
    }

    #[test]
    fn distinct_payload_fraction_all_same_is_one_over_n() {
        let payloads = vec![1u32; 10];
        let d = distinct_payload_fraction(&payloads);
        assert!((d - 0.1).abs() < 1e-9);
    }

    #[test]
    fn distinct_payload_fraction_empty_is_zero() {
        let payloads: Vec<u32> = vec![];
        let d = distinct_payload_fraction(&payloads);
        assert_eq!(d, 0.0);
    }

    #[test]
    fn diversity_report_combines_both_metrics() {
        let parents: Vec<Option<u32>> =
            vec![Some(1), Some(1), Some(2), Some(2)];
        let payloads = vec![10u32, 20, 30, 40];
        let r = DiversityReport::new(&parents, &payloads);
        assert_eq!(r.proposal_count, 4);
        assert!((r.parent_selection_entropy_bits - 1.0).abs() < 1e-9);
        assert_eq!(r.pairwise_payload_diversity, 1.0);
        assert!(!r.is_below_alarm_floor());
    }

    #[test]
    fn diversity_report_alarms_on_collapsed_parents() {
        // 99:1 parent split + all-distinct payloads → entropy below floor.
        let mut parents: Vec<Option<u32>> =
            (0..99).map(|_| Some(1u32)).collect();
        parents.push(Some(2));
        let payloads: Vec<u32> = (0..100).collect();
        let r = DiversityReport::new(&parents, &payloads);
        assert!(
            r.is_below_alarm_floor(),
            "99:1 parent collapse must trip alarm"
        );
    }

    #[test]
    fn diversity_report_alarms_on_collapsed_payloads() {
        let parents: Vec<Option<u32>> =
            vec![Some(1), Some(1), Some(2), Some(2)];
        // 4 proposals, only 1 distinct payload → 0.25 exactly is the floor;
        // need 5 collapsed to fall STRICTLY below 0.25.
        let parents5: Vec<Option<u32>> = vec![
            Some(1),
            Some(1),
            Some(2),
            Some(2),
            Some(3),
        ];
        let payloads5 = vec![10u32; 5];
        let r = DiversityReport::new(&parents5, &payloads5);
        assert!(r.pairwise_payload_diversity < 0.25);
        assert!(
            r.is_below_alarm_floor(),
            "fully-collapsed payloads must trip alarm"
        );
        let _ = parents;
    }
}
