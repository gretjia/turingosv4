//! AggregateReport — TB-18B FR-18B.5 / FR-18B.6 / FR-18B.11 — emits a
//! batch-aggregate report consuming Wilson 95% CI + DiversityReport.
//!
//! Per CLAUDE.md §17 Report Standard for formal proof benchmark reports:
//!
//! ```text
//! - ΣPPUT
//! - Mean PPUT on solved
//! - 95% CI if reporting aggregate
//! - halt_reason_distribution
//! - proposal / attempt counts
//! - accepted / rejected counts
//! - no fake accepted nodes status
//! ```
//!
//! This module is the single consumer wiring `wilson_ci.rs` +
//! `diversity.rs` into a CLAUDE.md §17 conformant aggregate. Per FR-18B.6:
//! "Aggregate report does NOT depend on evaluator stdout for core facts" —
//! all inputs are typed Rust structs derived from per-run chain-resident
//! evidence (`chain_invariant.json` + `run_summary.json` + telemetry CAS
//! objects), not stdout parsing.
//!
//! `FC-trace: Art-I.2 Statistical signal + Art-II.2.1 exploration/exploitation`.
//! `Phase-tag: P3 Lean Proof Task Market — formal benchmark scale-up`.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::runtime::diversity::DiversityReport;
use crate::runtime::wilson_ci::WilsonCi;

/// TRACE_MATRIX § 3 orphan (Constitution Landing 2026-05-08; Stage B3 / TB-18B): per-run facts that the AggregateReport consumes. Constructed from per-problem evidence files.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerRunFacts {
    /// Problem id (e.g. "mathd_algebra_107").
    pub problem_id: String,
    /// Run id (concrete instance label; cross-references chain_invariant.json).
    pub run_id: String,
    /// Solved verdict on this run.
    pub solved: bool,
    /// Halt reason class (e.g. "OmegaAccepted", "MaxTxExhausted").
    pub halt_reason: String,
    /// Total proposal/attempt count for the run.
    pub attempt_count: u64,
    /// L4-accepted attempt count for the run.
    pub l4_accepted: u64,
    /// L4.E-rejected attempt count for the run.
    pub l4e_rejected: u64,
    /// Capsule-anchored attempt count for the run.
    pub capsule_anchored: u64,
    /// Run PPUT (proposals per unit task / proofs per unit time / etc.; the
    /// metric name is fixed by upstream measurement).
    pub pput: f64,
}

impl PerRunFacts {
    /// TRACE_MATRIX § 3 orphan (Stage B3 / TB-18B FR-18B.11): chain-resident attempt-equality on this run. Constitutional Justification: CLAUDE.md §6 Externalized Attempt Rule canonical LHS scope.
    pub fn fc1_attempt_equality_holds(&self) -> bool {
        self.attempt_count == self.l4_accepted + self.l4e_rejected + self.capsule_anchored
    }
}

/// TRACE_MATRIX § 3 orphan: aggregate report shape conforming to CLAUDE.md §17 Report Standard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AggregateReport {
    /// Total runs aggregated.
    pub run_count: u64,
    /// Solved-run count.
    pub solved_count: u64,
    /// ΣPPUT — sum of pput over all runs (CLAUDE.md §17 line 1).
    pub sigma_pput: f64,
    /// Mean PPUT on solved runs only (CLAUDE.md §17 line 2). None if 0 solved.
    pub mean_pput_solved: Option<f64>,
    /// Wilson 95% CI for solve_rate = solved_count / run_count (CLAUDE.md §17 line 3).
    /// None when run_count == 0.
    pub solve_rate_wilson_95_ci: Option<WilsonCiJson>,
    /// halt_reason_distribution (CLAUDE.md §17 line 4).
    pub halt_reason_distribution: BTreeMap<String, u64>,
    /// Total proposal/attempt count (CLAUDE.md §17 line 5).
    pub total_attempts: u64,
    /// Accepted count (CLAUDE.md §17 line 6).
    pub total_l4_accepted: u64,
    /// Rejected count (CLAUDE.md §17 line 6).
    pub total_l4e_rejected: u64,
    /// Capsule-anchored count.
    pub total_capsule_anchored: u64,
    /// no_fake_accepted_nodes status (CLAUDE.md §17 line 7).
    pub no_fake_accepted_nodes: bool,
    /// Per-run FC1-INV1 attempt-equality count (FR-18B.11).
    pub fc1_attempt_equality_pass_count: u64,
    /// Aggregate FC1-INV1 attempt-equality status across the batch.
    pub fc1_aggregate_attempt_equality_holds: bool,
    /// Optional diversity report aggregate (closes Art. II.2.1 wire-up).
    pub diversity: Option<DiversityReportJson>,
    /// Schema id pin.
    pub schema_id: String,
}

/// TRACE_MATRIX § 3 orphan (Stage B3 / TB-18B): schema id pin for AggregateReport. Constitutional Justification: TB-18B charter §3 + `feedback_benchmark_manifest_required` schema-stability discipline.
pub const AGGREGATE_REPORT_SCHEMA_ID: &str = "turingosv4.aggregate_report.v1";

/// TRACE_MATRIX § 3 orphan (Stage B3 / TB-18B; Art. I.2 PPUT report side): JSON-friendly shape of WilsonCi (the WilsonCi struct itself is `Copy`-only; this avoids surface-area changes there). Constitutional Justification: CLAUDE.md §17 Report Standard "95% CI if reporting aggregate".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WilsonCiJson {
    pub successes: u32,
    pub trials: u32,
    pub point: f64,
    pub lower: f64,
    pub upper: f64,
}

impl From<WilsonCi> for WilsonCiJson {
    fn from(w: WilsonCi) -> Self {
        Self {
            successes: w.successes,
            trials: w.trials,
            point: w.point,
            lower: w.lower,
            upper: w.upper,
        }
    }
}

/// TRACE_MATRIX § 3 orphan (Stage B3 / TB-18B; Art. II.2.1 exploration/exploitation report side): JSON-friendly shape of DiversityReport (matches diversity.rs::DiversityReport surface). Constitutional Justification: CLAUDE.md §17 Report Standard parent-selection-entropy + payload-diversity surfacing for aggregate audit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiversityReportJson {
    pub proposal_count: usize,
    pub parent_selection_entropy_bits: f64,
    pub pairwise_payload_diversity: f64,
    pub below_alarm_floor: bool,
}

impl From<DiversityReport> for DiversityReportJson {
    fn from(d: DiversityReport) -> Self {
        Self {
            proposal_count: d.proposal_count,
            parent_selection_entropy_bits: d.parent_selection_entropy_bits,
            pairwise_payload_diversity: d.pairwise_payload_diversity,
            below_alarm_floor: d.is_below_alarm_floor(),
        }
    }
}

impl AggregateReport {
    /// TRACE_MATRIX § 3 orphan — construct an AggregateReport from per-run facts.
    /// `diversity` is optional because not every batch wires diversity in this
    /// version (M1 ships without; M2 wires it once parent_tx logging is stable
    /// at scale). `no_fake_accepted_nodes` is caller-supplied; it should come
    /// from `audit_tape` cross-witness (per CLAUDE.md §17).
    pub fn from_per_run(
        runs: &[PerRunFacts],
        no_fake_accepted_nodes: bool,
        diversity: Option<DiversityReport>,
    ) -> Self {
        let run_count = runs.len() as u64;
        let solved_count = runs.iter().filter(|r| r.solved).count() as u64;

        let sigma_pput: f64 = runs.iter().map(|r| r.pput).sum();
        let mean_pput_solved: Option<f64> = if solved_count == 0 {
            None
        } else {
            let sum_solved: f64 = runs.iter().filter(|r| r.solved).map(|r| r.pput).sum();
            Some(sum_solved / solved_count as f64)
        };

        let solve_rate_wilson_95_ci: Option<WilsonCiJson> = if run_count == 0 {
            None
        } else {
            WilsonCi::new_95(solved_count as u32, run_count as u32).map(WilsonCiJson::from)
        };

        let mut halt_reason_distribution: BTreeMap<String, u64> = BTreeMap::new();
        let mut total_attempts: u64 = 0;
        let mut total_l4_accepted: u64 = 0;
        let mut total_l4e_rejected: u64 = 0;
        let mut total_capsule_anchored: u64 = 0;
        let mut fc1_pass_count: u64 = 0;
        for r in runs {
            *halt_reason_distribution
                .entry(r.halt_reason.clone())
                .or_insert(0) += 1;
            total_attempts += r.attempt_count;
            total_l4_accepted += r.l4_accepted;
            total_l4e_rejected += r.l4e_rejected;
            total_capsule_anchored += r.capsule_anchored;
            if r.fc1_attempt_equality_holds() {
                fc1_pass_count += 1;
            }
        }
        let fc1_aggregate_attempt_equality_holds =
            total_attempts == total_l4_accepted + total_l4e_rejected + total_capsule_anchored;

        Self {
            run_count,
            solved_count,
            sigma_pput,
            mean_pput_solved,
            solve_rate_wilson_95_ci,
            halt_reason_distribution,
            total_attempts,
            total_l4_accepted,
            total_l4e_rejected,
            total_capsule_anchored,
            no_fake_accepted_nodes,
            fc1_attempt_equality_pass_count: fc1_pass_count,
            fc1_aggregate_attempt_equality_holds,
            diversity: diversity.map(DiversityReportJson::from),
            schema_id: AGGREGATE_REPORT_SCHEMA_ID.to_string(),
        }
    }

    /// TRACE_MATRIX § 3 orphan — CLAUDE.md §17 Report Standard conformance check.
    /// Returns `Ok(())` if all required fields are populated and self-consistent;
    /// otherwise returns the FIRST missing requirement.
    pub fn assert_claude_md_section_17(&self) -> Result<(), AggregateReportError> {
        use AggregateReportError::*;
        if self.schema_id != AGGREGATE_REPORT_SCHEMA_ID {
            return Err(SchemaIdMismatch(self.schema_id.clone()));
        }
        if self.run_count == 0 {
            return Err(EmptyBatch);
        }
        // ΣPPUT is always present (f64); a NaN/inf would indicate corrupt input.
        if !self.sigma_pput.is_finite() {
            return Err(NonFiniteSigmaPput(self.sigma_pput));
        }
        // Mean PPUT (solved) optional only when solved_count == 0.
        if self.solved_count > 0 && self.mean_pput_solved.is_none() {
            return Err(MeanPputSolvedMissing);
        }
        if self.solve_rate_wilson_95_ci.is_none() {
            return Err(WilsonCiMissing);
        }
        if self.halt_reason_distribution.is_empty() {
            return Err(HaltDistributionMissing);
        }
        if !self.fc1_aggregate_attempt_equality_holds {
            return Err(Fc1AggregateInvariantBroken {
                attempts: self.total_attempts,
                rhs: self.total_l4_accepted + self.total_l4e_rejected + self.total_capsule_anchored,
            });
        }
        if !self.no_fake_accepted_nodes {
            return Err(NoFakeAcceptedNotProven);
        }
        Ok(())
    }

    /// TRACE_MATRIX § 3 orphan — pretty JSON for evidence packaging.
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// TRACE_MATRIX § 3 orphan (Stage B3 / TB-18B): AggregateReport conformance error class. One variant per CLAUDE.md §17 Report Standard ship-block clause. Constitutional Justification: CLAUDE.md §17 + TB-18B charter §3 ship-gate semantics.
#[derive(Debug, Clone, PartialEq)]
pub enum AggregateReportError {
    SchemaIdMismatch(String),
    EmptyBatch,
    NonFiniteSigmaPput(f64),
    MeanPputSolvedMissing,
    WilsonCiMissing,
    HaltDistributionMissing,
    Fc1AggregateInvariantBroken { attempts: u64, rhs: u64 },
    NoFakeAcceptedNotProven,
}

impl std::fmt::Display for AggregateReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AggregateReportError::*;
        match self {
            SchemaIdMismatch(s) => write!(
                f,
                "AggregateReport: schema_id {s:?} != {AGGREGATE_REPORT_SCHEMA_ID}"
            ),
            EmptyBatch => write!(f, "AggregateReport: run_count == 0"),
            NonFiniteSigmaPput(v) => {
                write!(f, "AggregateReport: sigma_pput={v} is not finite")
            }
            MeanPputSolvedMissing => write!(
                f,
                "AggregateReport: solved_count > 0 but mean_pput_solved is None (CLAUDE.md §17 line 2 missing)"
            ),
            WilsonCiMissing => write!(
                f,
                "AggregateReport: solve_rate_wilson_95_ci is None (CLAUDE.md §17 line 3 missing)"
            ),
            HaltDistributionMissing => write!(
                f,
                "AggregateReport: halt_reason_distribution empty (CLAUDE.md §17 line 4 missing)"
            ),
            Fc1AggregateInvariantBroken { attempts, rhs } => write!(
                f,
                "AggregateReport: FC1-INV1 aggregate broken — total_attempts={attempts} != l4 + l4e + capsule = {rhs}"
            ),
            NoFakeAcceptedNotProven => write!(
                f,
                "AggregateReport: no_fake_accepted_nodes=false (CLAUDE.md §17 line 7 not affirmed)"
            ),
        }
    }
}

impl std::error::Error for AggregateReportError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::diversity::DiversityReport;

    fn run(problem_id: &str, run_id: &str, solved: bool, halt: &str, pput: f64) -> PerRunFacts {
        // Internally consistent attempt counts: 1 L4 if solved, else 1 L4.E
        let (l4, l4e, cap) = if solved { (1, 0, 0) } else { (0, 1, 0) };
        PerRunFacts {
            problem_id: problem_id.into(),
            run_id: run_id.into(),
            solved,
            halt_reason: halt.into(),
            attempt_count: l4 + l4e + cap,
            l4_accepted: l4,
            l4e_rejected: l4e,
            capsule_anchored: cap,
            pput,
        }
    }

    #[test]
    fn empty_batch_fails_section_17() {
        let r = AggregateReport::from_per_run(&[], true, None);
        assert_eq!(
            r.assert_claude_md_section_17(),
            Err(AggregateReportError::EmptyBatch)
        );
    }

    #[test]
    fn solved_3_of_5_produces_complete_section_17() {
        let runs = vec![
            run("p1", "r1", true, "OmegaAccepted", 1.0),
            run("p2", "r2", true, "OmegaAccepted", 2.0),
            run("p3", "r3", true, "OmegaAccepted", 3.0),
            run("p4", "r4", false, "MaxTxExhausted", 0.5),
            run("p5", "r5", false, "ParseFailed", 0.25),
        ];
        let r = AggregateReport::from_per_run(&runs, true, None);
        assert_eq!(r.run_count, 5);
        assert_eq!(r.solved_count, 3);
        assert!((r.sigma_pput - 6.75).abs() < 1e-9);
        assert!((r.mean_pput_solved.unwrap() - 2.0).abs() < 1e-9);
        assert!(r.solve_rate_wilson_95_ci.is_some());
        let ci = r.solve_rate_wilson_95_ci.as_ref().unwrap();
        assert_eq!(ci.successes, 3);
        assert_eq!(ci.trials, 5);
        assert!(ci.point > 0.59 && ci.point < 0.61);
        assert!(ci.lower > 0.0 && ci.upper < 1.0);
        assert_eq!(r.halt_reason_distribution.len(), 3);
        assert_eq!(*r.halt_reason_distribution.get("OmegaAccepted").unwrap(), 3);
        assert!(r.fc1_aggregate_attempt_equality_holds);
        assert_eq!(r.fc1_attempt_equality_pass_count, 5);
        r.assert_claude_md_section_17().expect("§17 conformant");
    }

    #[test]
    fn no_fake_accepted_false_blocks_section_17() {
        let runs = vec![run("p1", "r1", true, "OmegaAccepted", 1.0)];
        let r = AggregateReport::from_per_run(&runs, false, None);
        assert_eq!(
            r.assert_claude_md_section_17(),
            Err(AggregateReportError::NoFakeAcceptedNotProven)
        );
    }

    #[test]
    fn fc1_aggregate_break_blocks_section_17() {
        let runs = vec![PerRunFacts {
            problem_id: "p1".into(),
            run_id: "r1".into(),
            solved: true,
            halt_reason: "OmegaAccepted".into(),
            // Deliberate FC1 break: attempts > sum
            attempt_count: 5,
            l4_accepted: 1,
            l4e_rejected: 0,
            capsule_anchored: 0,
            pput: 1.0,
        }];
        let r = AggregateReport::from_per_run(&runs, true, None);
        match r.assert_claude_md_section_17() {
            Err(AggregateReportError::Fc1AggregateInvariantBroken { attempts, rhs }) => {
                assert_eq!(attempts, 5);
                assert_eq!(rhs, 1);
            }
            other => panic!("expected Fc1AggregateInvariantBroken, got {other:?}"),
        }
    }

    #[test]
    fn diversity_wire_through() {
        let runs = vec![
            run("p1", "r1", true, "OmegaAccepted", 1.0),
            run("p2", "r2", true, "OmegaAccepted", 1.0),
        ];
        // Synthesize a DiversityReport — entropy 1.0 (perfectly diverse) on 2 parents.
        let parent_picks: Vec<Option<u32>> = vec![Some(1), Some(2)];
        let payloads: Vec<&[u8]> = vec![b"a", b"b"];
        let dr = DiversityReport::new(&parent_picks, &payloads);
        let r = AggregateReport::from_per_run(&runs, true, Some(dr));
        assert!(r.diversity.is_some());
        let d = r.diversity.as_ref().unwrap();
        assert_eq!(d.proposal_count, 2);
        assert!(d.parent_selection_entropy_bits > 0.0);
        assert!(d.pairwise_payload_diversity > 0.0);
        assert!(!d.below_alarm_floor);
        r.assert_claude_md_section_17().expect("§17 conformant");
    }

    #[test]
    fn schema_id_mismatch_blocks() {
        let mut r = AggregateReport::from_per_run(
            &[run("p1", "r1", true, "OmegaAccepted", 1.0)],
            true,
            None,
        );
        r.schema_id = "turingosv4.aggregate_report.v0".into();
        match r.assert_claude_md_section_17() {
            Err(AggregateReportError::SchemaIdMismatch(s)) => {
                assert_eq!(s, "turingosv4.aggregate_report.v0")
            }
            other => panic!("expected SchemaIdMismatch, got {other:?}"),
        }
    }

    #[test]
    fn round_trip_json() {
        let runs = vec![run("p1", "r1", true, "OmegaAccepted", 1.0)];
        let r = AggregateReport::from_per_run(&runs, true, None);
        let body = r.to_json_pretty().expect("to_json");
        let r2: AggregateReport = serde_json::from_str(&body).expect("from_json");
        assert_eq!(r, r2);
    }

    #[test]
    fn schema_id_constant_pinned() {
        assert_eq!(AGGREGATE_REPORT_SCHEMA_ID, "turingosv4.aggregate_report.v1");
    }
}
