//! TB-G G2.2 — MarketDecisionTrace `## §F` summary builder.
//!
//! Charter: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
//! §1 Module G2 atom G2.2.
//!
//! Directive: `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
//! §G2 SG-G2.3 "NoTradeReason appears in dashboard and CAS".
//!
//! The walker scans CAS for `tb_n3.market_decision_trace.v1` objects,
//! tallies outcomes + per-`NoTradeReason` counts, and renders the
//! exhaustive 13-row `## §F MarketDecisionTrace summary` block consumed by
//! `audit_dashboard --run-report`.
//!
//! Lift-out from `src/bin/audit_dashboard.rs::render_tb_n3_run_report` so
//! the row-rendering contract is library-testable (binary-internal helpers
//! cannot be imported from a `tests/constitution_*.rs` integration test).
//!
//! Render contract per architect §G2 + charter §1 Module G2 atom G2.2:
//! - `total_traces: N` — total schema-versioned objects walked.
//! - `outcome[submitted|no_trade|declined] = N` — sorted by BTreeMap key.
//! - `submitted_vs_traced_ratio: <submitted>/<total>` (`n/a` if total=0).
//! - **NEW G2.2** `## §F.A NoTradeReason exhaustive breakdown` — every
//!   variant in `NoTradeReason::ALL` (13 rows in stable insertion order),
//!   including zeros, so forward audits can grep the column shape without
//!   parsing the live BTreeMap output.

use std::collections::BTreeMap;
use std::path::Path;

use crate::runtime::market_decision_trace::{
    MarketDecisionTrace, NoTradeReason, TraceOutcome,
};

/// TRACE_MATRIX FC1-N5 (TB-G G2.2 2026-05-12; charter §1 Module G2
/// atom G2.2 — `audit_dashboard --run-report` §F dashboard surface for
/// the architect §G2 NoTradeReason taxonomy).
#[derive(Debug, Clone)]
pub struct MarketDecisionTraceSummary {
    /// Total `tb_n3.market_decision_trace.v1` objects scanned.
    pub total_traces: u64,
    /// Coarse-bucket counts (submitted / no_trade / declined).
    pub outcome_counts: BTreeMap<String, u64>,
    /// Per-NoTradeReason counts. Sparse map; the renderer iterates
    /// `NoTradeReason::ALL` and fills missing entries with 0 for the
    /// exhaustive §F.A breakdown.
    pub no_trade_breakdown: BTreeMap<NoTradeReason, u64>,
    /// Submitted-outcome count (subset of `total_traces`); duplicates
    /// `outcome_counts["submitted"]` so callers can compute the
    /// submitted-vs-traced ratio without re-parsing the bucket map.
    pub submitted_count: u64,
}

impl MarketDecisionTraceSummary {
    /// Walk every CID in the CAS store and tally
    /// `tb_n3.market_decision_trace.v1` objects. Non-trace CIDs are
    /// silently skipped (we share the AttemptTelemetry ObjectType slot
    /// per the trace module's CAS write path — `schema_version` is the
    /// discriminator).
    ///
    /// TRACE_MATRIX FC1-N5 (TB-G G2.2 2026-05-12; §F walker for
    /// MarketDecisionTrace summary).
    pub fn compute_from_cas(
        cas: &crate::bottom_white::cas::store::CasStore,
    ) -> Self {
        let mut outcome_counts: BTreeMap<String, u64> = BTreeMap::new();
        let mut no_trade_breakdown: BTreeMap<NoTradeReason, u64> = BTreeMap::new();
        let mut total_traces: u64 = 0;
        let mut submitted_count: u64 = 0;
        for entry in cas.list_all_cids() {
            let bytes = match cas.get(&entry) {
                Ok(b) => b,
                Err(_) => continue,
            };
            let trace: MarketDecisionTrace = match serde_json::from_slice(&bytes) {
                Ok(t) => t,
                Err(_) => continue,
            };
            if trace.schema_version != MarketDecisionTrace::SCHEMA_VERSION {
                continue;
            }
            total_traces += 1;
            match &trace.outcome {
                TraceOutcome::Submitted { .. } => {
                    *outcome_counts.entry("submitted".into()).or_insert(0) += 1;
                    submitted_count += 1;
                }
                TraceOutcome::NoTrade { reason, .. } => {
                    *outcome_counts.entry("no_trade".into()).or_insert(0) += 1;
                    *no_trade_breakdown.entry(*reason).or_insert(0) += 1;
                }
                TraceOutcome::Declined => {
                    *outcome_counts.entry("declined".into()).or_insert(0) += 1;
                }
            }
        }
        Self {
            total_traces,
            outcome_counts,
            no_trade_breakdown,
            submitted_count,
        }
    }

    /// Convenience: open the CAS store at `cas_path` and compute the
    /// summary. Returns `Err(String)` on open failure (preserves the
    /// caller-side error-rendering shape used by `render_tb_n3_run_report`).
    ///
    /// TRACE_MATRIX FC1-N5 (TB-G G2.2 2026-05-12; path-based variant of
    /// `compute_from_cas` for fixture / replay-eval call sites).
    pub fn compute_from_path(cas_path: &Path) -> Result<Self, String> {
        let cas = crate::bottom_white::cas::store::CasStore::open(cas_path)
            .map_err(|e| format!("CAS open failed: {e}"))?;
        Ok(Self::compute_from_cas(&cas))
    }

    /// Render `submitted_vs_traced_ratio` as a stable string (charter §1
    /// Module G2.2 ship gate). `n/a` when `total_traces == 0` so the row
    /// is render-safe on empty-batch evidence.
    ///
    /// TRACE_MATRIX FC1-N5 (TB-G G2.2 2026-05-12; integer-rational
    /// percent surface for the §F ratio row).
    pub fn submitted_vs_traced_ratio_str(&self) -> String {
        if self.total_traces == 0 {
            "0/0 = n/a (no traces)".to_string()
        } else {
            // Integer-rational percent (architect §6 "price is signal, not
            // truth" + CLAUDE.md §13 no-f64-in-money-path; same discipline
            // applies to ratios surfaced as user-facing dashboard rows).
            let pct = (self.submitted_count * 100) / self.total_traces;
            format!(
                "{}/{} = {}%",
                self.submitted_count, self.total_traces, pct
            )
        }
    }

    /// Render the `## §F MarketDecisionTrace summary` block + the new
    /// `## §F.A NoTradeReason exhaustive breakdown` per architect §G2
    /// SG-G2.3. The §F.A section iterates `NoTradeReason::ALL` in stable
    /// insertion order so every variant has a fixed column position
    /// (including zero-count variants for forward grep stability).
    ///
    /// TRACE_MATRIX FC1-N5 + §17 reporting standard (TB-G G2.2 2026-05-12;
    /// `audit_dashboard --run-report` §F + §F.A row contract).
    pub fn render_section_f(&self) -> String {
        let mut out = String::new();
        out.push_str("\n## §F MarketDecisionTrace summary\n");
        out.push_str(&format!("  total_traces: {}\n", self.total_traces));
        for (k, v) in &self.outcome_counts {
            out.push_str(&format!("  outcome[{}] = {}\n", k, v));
        }
        // TB-G G2.2 SG-G2.3 — submitted_vs_traced_ratio row (charter §1
        // Module G2.2 ship gate). Always rendered, even when total=0.
        out.push_str(&format!(
            "  submitted_vs_traced_ratio: {}\n",
            self.submitted_vs_traced_ratio_str()
        ));
        if !self.no_trade_breakdown.is_empty() {
            out.push_str("  no_trade reason breakdown (observed, sorted by count):\n");
            let mut sorted: Vec<(&NoTradeReason, &u64)> =
                self.no_trade_breakdown.iter().collect();
            sorted.sort_by(|a, b| b.1.cmp(a.1));
            for (reason, n) in sorted {
                out.push_str(&format!("    {} = {}\n", reason.label(), n));
            }
        }
        // TB-G G2.2 SG-G2.3 — §F.A exhaustive 13-row stable breakdown.
        // Every variant in NoTradeReason::ALL renders a row (including
        // zero-count variants), so forward audits can grep for any
        // architect-spec variant name regardless of whether the batch
        // produced any traces for it. Closes the architect's "appears
        // in dashboard" contract for the full taxonomy.
        out.push_str("\n## §F.A NoTradeReason exhaustive breakdown\n");
        out.push_str(&format!(
            "  (architect §G2 13-variant taxonomy; stable insertion order; \
             zeros included for forward grep stability)\n"
        ));
        for &reason in NoTradeReason::ALL {
            let count = self.no_trade_breakdown.get(&reason).copied().unwrap_or(0);
            out.push_str(&format!("  {} = {}\n", reason.label(), count));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bottom_white::cas::store::CasStore;
    use crate::runtime::market_decision_trace::write_market_decision_trace_to_cas;
    use crate::state::q_state::{AgentId, TxId};
    use crate::state::typed_tx::BuyDirection;

    fn make_cas() -> (tempfile::TempDir, CasStore) {
        let dir = tempfile::tempdir().expect("tempdir");
        let cas = CasStore::open(dir.path()).expect("cas open");
        (dir, cas)
    }

    fn put_no_trade(cas: &mut CasStore, agent: &str, reason: NoTradeReason, t: u64) {
        let trace = MarketDecisionTrace::no_trade(
            AgentId(agent.into()),
            None,
            None,
            None,
            reason,
            format!("fixture {}", reason.label()),
        );
        write_market_decision_trace_to_cas(cas, &trace, &format!("{}-{}", agent, t), t)
            .expect("cas put");
    }

    fn put_submitted(cas: &mut CasStore, agent: &str, t: u64) {
        let trace = MarketDecisionTrace::submitted(
            AgentId(agent.into()),
            TxId(format!("worktx-{agent}-{t}")),
            BuyDirection::BuyYes,
            500_000,
            TxId(format!("router-{agent}-{t}")),
            "fixture submitted",
        );
        write_market_decision_trace_to_cas(cas, &trace, &format!("{}-{}", agent, t), t)
            .expect("cas put");
    }

    /// G2.2 U1 — empty CAS renders a deterministic header with zero
    /// total_traces + `n/a` ratio + all 13 variants at zero.
    #[test]
    fn empty_cas_renders_thirteen_zero_rows() {
        let (_dir, cas) = make_cas();
        let summary = MarketDecisionTraceSummary::compute_from_cas(&cas);
        assert_eq!(summary.total_traces, 0);
        assert_eq!(summary.submitted_count, 0);
        let out = summary.render_section_f();
        assert!(out.contains("total_traces: 0"));
        assert!(out.contains("submitted_vs_traced_ratio: 0/0 = n/a (no traces)"));
        assert!(out.contains("## §F.A NoTradeReason exhaustive breakdown"));
        for &reason in NoTradeReason::ALL {
            assert!(
                out.contains(&format!("  {} = 0", reason.label())),
                "G2.2 U1: zero row for {} missing",
                reason.label()
            );
        }
    }

    /// G2.2 U2 — mixed batch (some no-trade variants + some submitted)
    /// renders correct counts, ratio row, and exhaustive 13-row block.
    #[test]
    fn mixed_batch_renders_counts_and_ratio() {
        let (_dir, mut cas) = make_cas();
        put_no_trade(&mut cas, "Agent_0", NoTradeReason::NoPool, 1);
        put_no_trade(&mut cas, "Agent_1", NoTradeReason::NoPool, 2);
        put_no_trade(&mut cas, "Agent_2", NoTradeReason::NoPerceivedEdge, 3);
        put_no_trade(&mut cas, "Agent_3", NoTradeReason::PromptBudgetExceeded, 4);
        put_no_trade(&mut cas, "Agent_4", NoTradeReason::RouterRejected, 5);
        put_submitted(&mut cas, "Agent_5", 6);
        put_submitted(&mut cas, "Agent_6", 7);
        let summary = MarketDecisionTraceSummary::compute_from_cas(&cas);
        assert_eq!(summary.total_traces, 7);
        assert_eq!(summary.submitted_count, 2);
        assert_eq!(summary.no_trade_breakdown.get(&NoTradeReason::NoPool), Some(&2));
        assert_eq!(
            summary.no_trade_breakdown.get(&NoTradeReason::NoPerceivedEdge),
            Some(&1)
        );
        assert_eq!(
            summary.no_trade_breakdown.get(&NoTradeReason::PromptBudgetExceeded),
            Some(&1)
        );
        let out = summary.render_section_f();
        assert!(out.contains("total_traces: 7"));
        assert!(out.contains("outcome[submitted] = 2"));
        assert!(out.contains("outcome[no_trade] = 5"));
        // 2/7 = 28%
        assert!(
            out.contains("submitted_vs_traced_ratio: 2/7 = 28%"),
            "render output missing expected ratio row: {out}"
        );
        // Exhaustive §F.A rows: all 13 variants present; observed counts
        // match; un-observed variants render as 0.
        assert!(out.contains("  no_pool = 2"));
        assert!(out.contains("  no_perceived_edge = 1"));
        assert!(out.contains("  prompt_budget_exceeded = 1"));
        assert!(out.contains("  router_rejected = 1"));
        assert!(out.contains("  agent_declined = 0"));
        assert!(out.contains("  too_fast_solve = 0"));
        assert!(out.contains("  slippage_out_zero = 0"));
        assert!(out.contains("  unknown = 0"));
    }

    /// G2.2 U3 — `compute_from_path` opens CAS and matches direct walk.
    #[test]
    fn compute_from_path_round_trip() {
        let (dir, mut cas) = make_cas();
        put_no_trade(&mut cas, "Agent_X", NoTradeReason::AmountExceedsBalance, 1);
        drop(cas);
        let summary =
            MarketDecisionTraceSummary::compute_from_path(dir.path()).expect("ok");
        assert_eq!(summary.total_traces, 1);
        assert_eq!(
            summary.no_trade_breakdown.get(&NoTradeReason::AmountExceedsBalance),
            Some(&1)
        );
    }

    /// G2.2 U4 — submitted_vs_traced_ratio is `n/a` on empty + integer-
    /// rational percent on non-empty (no f64).
    #[test]
    fn submitted_vs_traced_ratio_string_shape() {
        let (_dir, mut cas) = make_cas();
        let s0 = MarketDecisionTraceSummary::compute_from_cas(&cas);
        assert_eq!(s0.submitted_vs_traced_ratio_str(), "0/0 = n/a (no traces)");
        // 1 submitted / 0 no_trade
        put_submitted(&mut cas, "Agent_R", 1);
        let s1 = MarketDecisionTraceSummary::compute_from_cas(&cas);
        assert_eq!(s1.submitted_vs_traced_ratio_str(), "1/1 = 100%");
        // 1 submitted / 1 no_trade
        put_no_trade(&mut cas, "Agent_R2", NoTradeReason::NoPool, 2);
        let s2 = MarketDecisionTraceSummary::compute_from_cas(&cas);
        assert_eq!(s2.submitted_vs_traced_ratio_str(), "1/2 = 50%");
    }
}
