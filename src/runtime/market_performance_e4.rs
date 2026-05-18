//! REAL-16 candidate-only E4 market-performance verifier.
//!
//! This module is a report/verifier surface. It does not influence sequencer
//! admission, Lean predicates, market prices, wallets, or economic state.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::store::CasStore;
use crate::bottom_white::ledger::transition_ledger::{
    canonical_decode, Git2LedgerWriter, LedgerWriter, TxKind,
};
use crate::runtime::chain_derived_run_facts::compute_run_facts_from_chain;
use crate::runtime::market_e2_candidate_verifier::{
    E2CandidateVerifierReport, E2CandidateVerifierVerdict,
};
use crate::runtime::real5_roles::{read_role_turn_trace_from_cas, role_turn_trace_cids};
use crate::runtime::wilson_ci::WilsonCi;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX FC3: REAL-16 metric source boundary keeps E4 evidence tied to ChainTape/CAS verifiers.
pub enum E4MetricSource {
    ChainTapeCasVerifier,
    ChainTapeCasPlusRuntimePputResult,
    DashboardOnly,
    StdoutOnly,
}

impl E4MetricSource {
    fn is_candidate_source(&self) -> bool {
        matches!(self, E4MetricSource::ChainTapeCasVerifier)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX FC3: REAL-16 arm evidence config pins hashes before candidate-only E4 comparison.
pub struct E4ArmEvidenceConfig {
    pub arm_id: String,
    pub evidence_dir: String,
    pub problem_set_hash: String,
    pub model_assignment_hash: String,
    pub budget_hash: String,
    pub prompt_template_hash: String,
    pub runtime_config_hash: String,
    pub market_pressure_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX FC3: REAL-16 arm input carries only verifier-derived candidate metrics.
pub struct E4ArmInput {
    pub arm_id: String,
    pub evidence_dir: String,
    pub problem_set_hash: String,
    pub model_assignment_hash: String,
    pub budget_hash: String,
    pub prompt_template_hash: String,
    pub runtime_config_hash: String,
    pub audit_tape_verdict: String,
    pub e2_verifier_verdict: String,
    pub e2_verifier_failure_reasons: Vec<String>,
    pub metric_source: E4MetricSource,
    pub market_pressure_enabled: bool,
    pub task_count: u32,
    pub solved_count: u32,
    pub verified_pput_micro: u64,
    pub wasted_attempt_count: u64,
    pub verification_latency_ms_total: u64,
    pub failed_branch_count: u64,
    pub cost_per_solved_proof_tokens: Option<u64>,
    pub exact_join_count: u64,
    pub ev_to_action_conversion_bps: u32,
    pub role_diversity_bps: u32,
    pub market_tx_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX FC3: REAL-16 verifier verdict bounds candidate-only E4 reporting.
pub enum E4Verdict {
    Proceed,
    CleanNegative,
    Veto,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX FC3: REAL-16 Wilson interval summary supports bounded A/B evidence.
pub struct E4WilsonCiBps {
    pub successes: u32,
    pub trials: u32,
    pub point_bps: u32,
    pub lower_bps: u32,
    pub upper_bps: u32,
}

impl E4WilsonCiBps {
    fn from_counts(successes: u32, trials: u32) -> Option<Self> {
        let ci = WilsonCi::new_95(successes, trials)?;
        Some(Self {
            successes,
            trials,
            point_bps: to_bps(ci.point),
            lower_bps: to_bps(ci.lower),
            upper_bps: to_bps(ci.upper),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX FC3: REAL-16 arm report row materializes verifier-derived metrics without becoming truth.
pub struct E4ArmReportRow {
    pub arm_id: String,
    pub evidence_dir: String,
    pub audit_tape_verdict: String,
    pub e2_verifier_verdict: String,
    pub e2_verifier_failure_reasons: Vec<String>,
    pub metric_source: E4MetricSource,
    pub market_pressure_enabled: bool,
    pub task_count: u32,
    pub solved_count: u32,
    pub verified_pput_micro: u64,
    pub wasted_attempt_count: u64,
    pub verification_latency_ms_total: u64,
    pub failed_branch_count: u64,
    pub cost_per_solved_proof_tokens: Option<u64>,
    pub exact_join_count: u64,
    pub ev_to_action_conversion_bps: u32,
    pub role_diversity_bps: u32,
    pub market_tx_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX FC3: REAL-16 performance report preserves candidate-only E4 claim boundaries.
pub struct E4PerformanceReport {
    pub schema_version: String,
    pub claim_boundary: String,
    pub source_boundary: String,
    pub arm_count: usize,
    pub baseline_arm_id: Option<String>,
    pub best_arm_id: Option<String>,
    pub e4_candidate: bool,
    pub verdict: E4Verdict,
    pub improved_metrics: Vec<String>,
    pub failure_reasons: Vec<String>,
    pub solve_rate_wilson_95_ci_by_arm: BTreeMap<String, E4WilsonCiBps>,
    pub arms: Vec<E4ArmReportRow>,
}

impl E4PerformanceReport {
    /// TRACE_MATRIX FC3: REAL-16 markdown rendering is a materialized view, not a source of truth.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# REAL-16 Market Performance Report\n\n");
        out.push_str(&format!("claim_boundary: `{}`\n", self.claim_boundary));
        out.push_str("claim_note: candidate-only; not E4 achieved\n\n");
        out.push_str(&format!("verdict: `{:?}`\n", self.verdict));
        out.push_str(&format!("e4_candidate: `{}`\n\n", self.e4_candidate));
        out.push_str("## Source Boundary\n\n");
        out.push_str(&self.source_boundary);
        out.push_str("\n\n");
        out.push_str("## Arms\n\n");
        out.push_str("| arm | tasks | solved | exact_join | verified_pput_micro | wasted | latency_ms_total | role_diversity_bps | market_tx_count |\n");
        out.push_str("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n");
        for row in &self.arms {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                row.arm_id,
                row.task_count,
                row.solved_count,
                row.exact_join_count,
                row.verified_pput_micro,
                row.wasted_attempt_count,
                row.verification_latency_ms_total,
                row.role_diversity_bps,
                row.market_tx_count
            ));
        }
        out.push_str("\n## Improved Metrics\n\n");
        if self.improved_metrics.is_empty() {
            out.push_str("none\n");
        } else {
            for metric in &self.improved_metrics {
                out.push_str(&format!("- `{metric}`\n"));
            }
        }
        out.push_str("\n## Failure Reasons\n\n");
        if self.failure_reasons.is_empty() {
            out.push_str("none\n");
        } else {
            for reason in &self.failure_reasons {
                out.push_str(&format!("- `{reason}`\n"));
            }
        }
        out.push_str("\n## Forbidden Claim Boundary\n\n");
        out.push_str("This report supports candidate-only language. It does not ship a mechanism or authorize achieved-status wording.\n");
        out
    }
}

/// TRACE_MATRIX FC3: REAL-16 E4 evaluation compares pinned verifier-derived arms only.
pub fn evaluate_market_performance_e4(arms: &[E4ArmInput]) -> E4PerformanceReport {
    let arms = normalize_control_arm_e2_claims(arms);
    let mut failure_reasons = Vec::new();
    let mut verdict = E4Verdict::CleanNegative;

    if arms.len() < 2 {
        failure_reasons.push("fewer_than_two_ab_arms".to_string());
        verdict = E4Verdict::Veto;
    }

    if arms
        .iter()
        .any(|a| a.metric_source == E4MetricSource::ChainTapeCasPlusRuntimePputResult)
    {
        failure_reasons.push("runtime_pput_sidecar_not_claim_bearing".to_string());
        verdict = E4Verdict::Veto;
    }
    if arms
        .iter()
        .any(|a| a.metric_source == E4MetricSource::DashboardOnly)
    {
        failure_reasons.push("dashboard_only_metric_source".to_string());
        verdict = E4Verdict::Veto;
    }
    if arms
        .iter()
        .any(|a| a.metric_source == E4MetricSource::StdoutOnly)
    {
        failure_reasons.push("stdout_only_metric_source".to_string());
        verdict = E4Verdict::Veto;
    }
    if arms.iter().any(|a| !a.metric_source.is_candidate_source()) {
        failure_reasons.push("unsupported_metric_source".to_string());
        verdict = E4Verdict::Veto;
    }
    if arms.iter().any(|a| a.audit_tape_verdict != "PROCEED") {
        failure_reasons.push("audit_tape_not_proceed".to_string());
        verdict = E4Verdict::Veto;
    }
    if arms
        .iter()
        .any(|a| a.market_pressure_enabled && a.e2_verifier_verdict != "PROCEED")
    {
        failure_reasons.push("e2_verifier_not_proceed".to_string());
        verdict = E4Verdict::Veto;
    }
    if arms
        .iter()
        .any(|a| a.market_pressure_enabled && !a.e2_verifier_failure_reasons.is_empty())
    {
        failure_reasons.push("e2_verifier_failure_reasons_present".to_string());
        verdict = E4Verdict::Veto;
    }
    if arms.iter().any(|a| a.task_count == 0) {
        failure_reasons.push("zero_task_arm".to_string());
        verdict = E4Verdict::Veto;
    }
    if !same_hashes(&arms) {
        failure_reasons.push("non_pinned_ab_hashes".to_string());
        verdict = E4Verdict::Veto;
    }

    let baseline = arms
        .iter()
        .find(|a| a.arm_id == "A")
        .or_else(|| arms.first());
    let market_arms: Vec<&E4ArmInput> = arms.iter().filter(|a| a.market_pressure_enabled).collect();
    if market_arms.is_empty() && verdict != E4Verdict::Veto {
        failure_reasons.push("no_market_pressure_arm".to_string());
        verdict = E4Verdict::CleanNegative;
    }

    let mut improved_metrics = Vec::new();
    let mut best_arm_id = None;
    if let Some(base) = baseline {
        let best = market_arms
            .iter()
            .max_by_key(|a| improvement_score(base, a))
            .copied();
        if let Some(best) = best {
            best_arm_id = Some(best.arm_id.clone());
            improved_metrics = improved_metric_names(base, best);
            let behavior_metrics = behavior_metric_names(base, best);
            if best.exact_join_count == 0 && verdict != E4Verdict::Veto {
                failure_reasons.push("no_live_market_action_in_best_arm".to_string());
                verdict = E4Verdict::CleanNegative;
            }
            if behavior_metrics.is_empty() && verdict != E4Verdict::Veto {
                failure_reasons.push("market_tx_count_only_not_e4".to_string());
                verdict = E4Verdict::CleanNegative;
            }
        }
    }

    if verdict != E4Verdict::Veto
        && baseline
            .and_then(|base| {
                arms.iter()
                    .find(|a| Some(&a.arm_id) == best_arm_id.as_ref())
                    .map(|best| !behavior_metric_names(base, best).is_empty())
            })
            .unwrap_or(false)
        && best_arm_id.is_some()
        && arms
            .iter()
            .find(|a| Some(&a.arm_id) == best_arm_id.as_ref())
            .map(|a| a.exact_join_count > 0)
            .unwrap_or(false)
    {
        verdict = E4Verdict::Proceed;
    }

    let mut solve_rate_wilson_95_ci_by_arm = BTreeMap::new();
    for arm in &arms {
        if let Some(ci) = E4WilsonCiBps::from_counts(arm.solved_count, arm.task_count) {
            solve_rate_wilson_95_ci_by_arm.insert(arm.arm_id.clone(), ci);
        }
    }

    let e4_candidate = verdict == E4Verdict::Proceed;
    let arms: Vec<E4ArmReportRow> = arms
        .iter()
        .map(|a| E4ArmReportRow {
            arm_id: a.arm_id.clone(),
            evidence_dir: a.evidence_dir.clone(),
            audit_tape_verdict: a.audit_tape_verdict.clone(),
            e2_verifier_verdict: a.e2_verifier_verdict.clone(),
            e2_verifier_failure_reasons: a.e2_verifier_failure_reasons.clone(),
            metric_source: a.metric_source.clone(),
            market_pressure_enabled: a.market_pressure_enabled,
            task_count: a.task_count,
            solved_count: a.solved_count,
            verified_pput_micro: a.verified_pput_micro,
            wasted_attempt_count: a.wasted_attempt_count,
            verification_latency_ms_total: a.verification_latency_ms_total,
            failed_branch_count: a.failed_branch_count,
            cost_per_solved_proof_tokens: a.cost_per_solved_proof_tokens,
            exact_join_count: a.exact_join_count,
            ev_to_action_conversion_bps: a.ev_to_action_conversion_bps,
            role_diversity_bps: a.role_diversity_bps,
            market_tx_count: a.market_tx_count,
        })
        .collect();

    E4PerformanceReport {
        schema_version: "real16.market_performance_e4.v1".to_string(),
        claim_boundary: if e4_candidate {
            "E4 candidate pending audit".to_string()
        } else {
            "clean-negative; no E4 candidate".to_string()
        },
        source_boundary: "ChainTape/CAS/exact-join verifier-derived metrics only; dashboard text and evaluator stdout are not source of truth".to_string(),
        arm_count: arms.len(),
        baseline_arm_id: baseline.map(|a| a.arm_id.clone()),
        best_arm_id,
        e4_candidate,
        verdict,
        improved_metrics,
        failure_reasons: dedup(failure_reasons),
        solve_rate_wilson_95_ci_by_arm,
        arms,
    }
}

fn normalize_control_arm_e2_claims(arms: &[E4ArmInput]) -> Vec<E4ArmInput> {
    arms.iter()
        .map(|arm| {
            let mut normalized = arm.clone();
            if !normalized.market_pressure_enabled && normalized.e2_verifier_verdict != "PROCEED" {
                normalized.exact_join_count = 0;
                normalized.ev_to_action_conversion_bps = 0;
            }
            normalized
        })
        .collect()
}

/// TRACE_MATRIX FC3: REAL-16 evidence adapter derives arm inputs from ChainTape/CAS and exact-join reports.
pub fn derive_arm_input_from_evidence(
    config: E4ArmEvidenceConfig,
    e2_report: E2CandidateVerifierReport,
) -> Result<E4ArmInput, String> {
    let evidence_dir = Path::new(&config.evidence_dir);
    let repo_path = evidence_dir.join("runtime_repo");
    let cas_path = evidence_dir.join("cas");
    let audit_tape_verdict = read_audit_tape_verdict(evidence_dir)?;
    let facts = compute_run_facts_from_chain(&repo_path, &cas_path)
        .map_err(|e| format!("chain-derived facts failed: {e}"))?;
    let l4 = scan_l4_terminal_and_market(&repo_path, &cas_path)?;
    let role_diversity_bps = role_diversity_bps_from_cas(&cas_path)?;
    let solved_count = l4.omega_terminal_summary_count;
    let cost_per_solved_proof_tokens = if solved_count == 0 {
        None
    } else {
        Some(facts.golden_path_token_count / u64::from(solved_count))
    };

    Ok(E4ArmInput {
        arm_id: config.arm_id,
        evidence_dir: config.evidence_dir,
        problem_set_hash: config.problem_set_hash,
        model_assignment_hash: config.model_assignment_hash,
        budget_hash: config.budget_hash,
        prompt_template_hash: config.prompt_template_hash,
        runtime_config_hash: config.runtime_config_hash,
        audit_tape_verdict,
        e2_verifier_verdict: e2_verdict_label(e2_report.verdict).to_string(),
        e2_verifier_failure_reasons: e2_report.failure_reasons,
        metric_source: E4MetricSource::ChainTapeCasVerifier,
        market_pressure_enabled: config.market_pressure_enabled,
        task_count: l4.terminal_summary_count,
        solved_count,
        verified_pput_micro: 0,
        wasted_attempt_count: facts.proposal_count.saturating_sub(u64::from(solved_count)),
        verification_latency_ms_total: 0,
        failed_branch_count: facts.failed_branch_count,
        cost_per_solved_proof_tokens,
        exact_join_count: e2_report.exact_join_count,
        ev_to_action_conversion_bps: ratio_bps(
            e2_report.exact_join_count,
            l4.terminal_summary_count,
        ),
        role_diversity_bps,
        market_tx_count: l4.market_tx_count,
    })
}

struct L4ArmScan {
    terminal_summary_count: u32,
    omega_terminal_summary_count: u32,
    market_tx_count: u64,
}

fn scan_l4_terminal_and_market(repo_path: &Path, cas_path: &Path) -> Result<L4ArmScan, String> {
    let writer = Git2LedgerWriter::open(repo_path).map_err(|e| format!("open L4 ledger: {e:?}"))?;
    let cas = CasStore::open(cas_path).map_err(|e| format!("open CAS: {e}"))?;
    let mut terminal_summary_count = 0_u32;
    let mut omega_terminal_summary_count = 0_u32;
    let mut market_tx_count = 0_u64;
    for logical_t in 1..=writer.len() {
        let entry = writer
            .read_at(logical_t)
            .map_err(|e| format!("read L4 logical_t={logical_t}: {e:?}"))?;
        if matches!(
            entry.tx_kind,
            TxKind::MarketSeed | TxKind::CpmmPool | TxKind::CpmmSwap | TxKind::BuyWithCoinRouter
        ) {
            market_tx_count = market_tx_count.saturating_add(1);
        }
        if entry.tx_kind != TxKind::TerminalSummary {
            continue;
        }
        let payload = cas
            .get(&entry.tx_payload_cid)
            .map_err(|e| format!("read TerminalSummary payload at t={logical_t}: {e}"))?;
        let typed_tx = canonical_decode(&payload)
            .map_err(|e| format!("decode TerminalSummary payload at t={logical_t}: {e}"))?;
        let crate::state::typed_tx::TypedTx::TerminalSummary(summary) = typed_tx else {
            return Err(format!(
                "L4 t={logical_t} has TerminalSummary kind but non-TerminalSummary payload"
            ));
        };
        terminal_summary_count = terminal_summary_count.saturating_add(1);
        if summary.run_outcome == crate::state::typed_tx::RunOutcome::OmegaAccepted {
            omega_terminal_summary_count = omega_terminal_summary_count.saturating_add(1);
        }
    }
    Ok(L4ArmScan {
        terminal_summary_count,
        omega_terminal_summary_count,
        market_tx_count,
    })
}

fn role_diversity_bps_from_cas(cas_path: &Path) -> Result<u32, String> {
    let cas = CasStore::open(cas_path).map_err(|e| format!("open CAS: {e}"))?;
    let mut roles = BTreeSet::new();
    for cid in role_turn_trace_cids(&cas) {
        let trace = read_role_turn_trace_from_cas(&cas, &cid)
            .map_err(|e| format!("read RoleTurnTrace {cid}: {e}"))?;
        roles.insert(trace.role.label().to_string());
    }
    Ok((roles.len() as u32).saturating_mul(2_000).min(10_000))
}

fn read_audit_tape_verdict(evidence_dir: &Path) -> Result<String, String> {
    let path = evidence_dir.join("aggregate_verdict.json");
    let body =
        std::fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let value: serde_json::Value =
        serde_json::from_str(&body).map_err(|e| format!("parse {}: {e}", path.display()))?;
    value
        .get("verdict")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or_else(|| format!("{} missing string verdict", path.display()))
}

fn e2_verdict_label(verdict: E2CandidateVerifierVerdict) -> &'static str {
    match verdict {
        E2CandidateVerifierVerdict::Proceed => "PROCEED",
        E2CandidateVerifierVerdict::Veto => "VETO",
    }
}

fn ratio_bps(numerator: u64, denominator: u32) -> u32 {
    if denominator == 0 {
        0
    } else {
        numerator
            .saturating_mul(10_000)
            .checked_div(u64::from(denominator))
            .unwrap_or(0)
            .min(10_000) as u32
    }
}

fn same_hashes(arms: &[E4ArmInput]) -> bool {
    if let Some(first) = arms.first() {
        arms.iter().all(|a| {
            a.problem_set_hash == first.problem_set_hash
                && a.model_assignment_hash == first.model_assignment_hash
                && a.budget_hash == first.budget_hash
                && a.prompt_template_hash == first.prompt_template_hash
                && a.runtime_config_hash == first.runtime_config_hash
        })
    } else {
        false
    }
}

fn improved_metric_names(base: &E4ArmInput, candidate: &E4ArmInput) -> Vec<String> {
    let mut out = Vec::new();
    out.extend(behavior_metric_names(base, candidate));
    if candidate.ev_to_action_conversion_bps > base.ev_to_action_conversion_bps {
        out.push("ev_to_action_conversion".to_string());
    }
    if candidate.role_diversity_bps > base.role_diversity_bps {
        out.push("role_diversity".to_string());
    }
    out
}

fn behavior_metric_names(base: &E4ArmInput, candidate: &E4ArmInput) -> Vec<String> {
    let mut out = Vec::new();
    if ratio_gt(
        candidate.solved_count as u128,
        candidate.task_count as u128,
        base.solved_count as u128,
        base.task_count as u128,
    ) {
        out.push("solve_rate".to_string());
    }
    if candidate.verified_pput_micro > base.verified_pput_micro {
        out.push("verified_pput".to_string());
    }
    if candidate.wasted_attempt_count < base.wasted_attempt_count {
        out.push("wasted_attempts".to_string());
    }
    if ratio_lt(
        candidate.verification_latency_ms_total as u128,
        candidate.task_count as u128,
        base.verification_latency_ms_total as u128,
        base.task_count as u128,
    ) {
        out.push("verification_latency".to_string());
    }
    if candidate.failed_branch_count < base.failed_branch_count {
        out.push("failed_branch_count".to_string());
    }
    match (
        candidate.cost_per_solved_proof_tokens,
        base.cost_per_solved_proof_tokens,
    ) {
        (Some(c), Some(b)) if c < b => out.push("cost_per_solved_proof".to_string()),
        (Some(_), None) => out.push("cost_per_solved_proof".to_string()),
        _ => {}
    }
    out
}

fn improvement_score(base: &E4ArmInput, candidate: &E4ArmInput) -> usize {
    improved_metric_names(base, candidate).len()
}

fn ratio_gt(a_num: u128, a_den: u128, b_num: u128, b_den: u128) -> bool {
    a_den != 0 && b_den != 0 && a_num.saturating_mul(b_den) > b_num.saturating_mul(a_den)
}

fn ratio_lt(a_num: u128, a_den: u128, b_num: u128, b_den: u128) -> bool {
    a_den != 0 && b_den != 0 && a_num.saturating_mul(b_den) < b_num.saturating_mul(a_den)
}

fn to_bps(v: f64) -> u32 {
    if !v.is_finite() || v <= 0.0 {
        0
    } else if v >= 1.0 {
        10_000
    } else {
        (v * 10_000.0).round() as u32
    }
}

fn dedup(values: Vec<String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for value in values {
        if seen.insert(value.clone()) {
            out.push(value);
        }
    }
    out
}
