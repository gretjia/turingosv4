//! REAL-15 -- role differentiation verifier helpers.
//!
//! This module turns ChainTape/CAS-derived role traces plus the independent
//! exact-join market verifier output into a candidate-only E3 materialized
//! report. It is not a sequencer rule and does not authorize E3 achieved
//! claims.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::store::CasStore;
use crate::runtime::market_e2_candidate_verifier::{
    E2CandidateVerifierReport, E2CandidateVerifierVerdict,
};
use crate::runtime::real5_roles::{
    read_role_turn_trace_from_cas, role_turn_trace_cids, AgentRole, RoleTurnOutcome,
};

/// TRACE_MATRIX FC3: REAL-15 verifier verdict for candidate-only reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RoleDifferentiationVerdict {
    Proceed,
    CleanNegative,
    Veto,
}

/// TRACE_MATRIX FC3: one immutable run input. The CAS store carries role-turn
/// evidence; the exact-join verifier report carries live market action
/// evidence. Dashboard text is intentionally not an input.
pub struct RoleDifferentiationRunInput<'a> {
    pub run_id: String,
    pub cas: &'a CasStore,
    pub e2_report: &'a E2CandidateVerifierReport,
    pub audit_tape_proceed: bool,
}

impl<'a> RoleDifferentiationRunInput<'a> {
    /// TRACE_MATRIX FC3: REAL-15 run input constructor binds CAS role traces to exact-join evidence.
    pub fn new(
        run_id: impl Into<String>,
        cas: &'a CasStore,
        e2_report: &'a E2CandidateVerifierReport,
        audit_tape_proceed: bool,
    ) -> Self {
        Self {
            run_id: run_id.into(),
            cas,
            e2_report,
            audit_tape_proceed,
        }
    }
}

/// TRACE_MATRIX FC1/FC3: per-role action vector used to decide whether a role
/// distribution is distinct. All counters are public ChainTape/CAS/verifier
/// facts.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleDifferentiationRoleRow {
    pub role: String,
    pub turn_count: u64,
    pub active_run_count: u64,
    pub task_count: u64,
    pub work_count: u64,
    pub verify_count: u64,
    pub challenge_count: u64,
    pub market_decision_turn_count: u64,
    pub exact_join_market_action_count: u64,
    pub buy_yes_count: u64,
    pub buy_no_count: u64,
    pub no_trade_count: u64,
    pub no_verify_count: u64,
    pub no_challenge_count: u64,
    pub action_signature: String,
}

impl RoleDifferentiationRoleRow {
    fn new(role: &str) -> Self {
        Self {
            role: role.to_string(),
            ..Self::default()
        }
    }

    fn active_action_count(&self) -> u64 {
        self.work_count
            + self.verify_count
            + self.challenge_count
            + self.exact_join_market_action_count
    }

    fn refresh_signature(&mut self) {
        self.action_signature = format!(
            "work:{}|verify:{}|challenge:{}|buy_yes:{}|buy_no:{}",
            self.work_count,
            self.verify_count,
            self.challenge_count,
            self.buy_yes_count,
            self.buy_no_count
        );
    }
}

/// TRACE_MATRIX FC3: candidate-only REAL-15 report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleDifferentiationReport {
    pub schema_version: String,
    pub source_boundary: String,
    pub run_count: u64,
    pub audit_tape_proceed_count: u64,
    pub by_role: BTreeMap<String, RoleDifferentiationRoleRow>,
    pub persistent_active_role_count: u64,
    pub distinct_action_signature_count: u64,
    pub e3_candidate: bool,
    pub verdict: RoleDifferentiationVerdict,
    pub failure_reasons: Vec<String>,
    pub residual_risks: Vec<String>,
}

impl RoleDifferentiationReport {
    /// TRACE_MATRIX FC3: REAL-15 report schema version pins candidate-only role differentiation output.
    pub const SCHEMA_VERSION: &'static str = "real15.role_differentiation.v1";

    /// TRACE_MATRIX FC3: REAL-15 markdown rendering is a materialized view, not canonical evidence.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# REAL-15 Role Differentiation Verifier Report\n\n");
        let boundary = if self.e3_candidate && self.verdict == RoleDifferentiationVerdict::Proceed {
            "E3 candidate pending audit"
        } else if self.verdict == RoleDifferentiationVerdict::Veto {
            "not an E3 candidate"
        } else {
            "clean-negative: no E3 candidate in this comparison"
        };
        out.push_str(&format!("claim_boundary: {boundary}\n"));
        out.push_str("claim_note: candidate-only; not E3 achieved\n\n");
        out.push_str(&format!("verdict: {:?}\n", self.verdict));
        out.push_str(&format!("run_count: {}\n", self.run_count));
        out.push_str(&format!(
            "audit_tape_proceed_count: {}\n",
            self.audit_tape_proceed_count
        ));
        out.push_str(&format!(
            "persistent_active_role_count: {}\n",
            self.persistent_active_role_count
        ));
        out.push_str(&format!(
            "distinct_action_signature_count: {}\n",
            self.distinct_action_signature_count
        ));
        out.push_str("\nrole_activity_distribution:\n");
        for row in self.by_role.values() {
            out.push_str(&format!(
                "- role={} turns={} active_runs={} tasks={} work={} verify={} challenge={} buy_yes={} buy_no={} exact_market={} signature={}\n",
                row.role,
                row.turn_count,
                row.active_run_count,
                row.task_count,
                row.work_count,
                row.verify_count,
                row.challenge_count,
                row.buy_yes_count,
                row.buy_no_count,
                row.exact_join_market_action_count,
                row.action_signature
            ));
        }
        if !self.failure_reasons.is_empty() {
            out.push_str("\nfailure_reasons:\n");
            for reason in &self.failure_reasons {
                out.push_str(&format!("- {reason}\n"));
            }
        }
        if !self.residual_risks.is_empty() {
            out.push_str("\nresidual_risks:\n");
            for risk in &self.residual_risks {
                out.push_str(&format!("- {risk}\n"));
            }
        }
        out
    }
}

#[derive(Debug, Clone, Default)]
struct RoleAccumulator {
    row: RoleDifferentiationRoleRow,
    active_runs: BTreeSet<String>,
    tasks: BTreeSet<String>,
}

/// TRACE_MATRIX FC1/FC3: summarize role differentiation from public role-turn
/// CAS traces plus exact-join market verifier reports. This helper never reads
/// dashboard text and fails closed on unreadable role-turn CAS objects.
pub fn summarize_role_differentiation_from_runs(
    runs: &[RoleDifferentiationRunInput<'_>],
) -> Result<RoleDifferentiationReport, String> {
    let mut accumulators = expected_role_accumulators();
    let mut failure_reasons = Vec::new();
    let mut residual_risks = Vec::new();
    let mut audit_tape_proceed_count = 0_u64;

    for run in runs {
        if run.audit_tape_proceed {
            audit_tape_proceed_count += 1;
        } else {
            failure_reasons.push(format!("run {} audit_tape was not PROCEED", run.run_id));
        }
        validate_e2_report_boundary(&run.run_id, run.e2_report, &mut failure_reasons);
        ingest_role_turns(run, &mut accumulators)?;
        ingest_exact_join_market_actions(run, &mut accumulators, &mut residual_risks);
    }

    let required_active_runs = if runs.len() >= 2 { 2 } else { 1 };
    let mut persistent_signatures = BTreeSet::new();
    let mut persistent_active_role_count = 0_u64;
    let mut any_active_action = false;

    let mut by_role = BTreeMap::new();
    for (role, mut acc) in accumulators {
        acc.row.active_run_count = acc.active_runs.len() as u64;
        acc.row.task_count = acc.tasks.len() as u64;
        acc.row.refresh_signature();
        if acc.row.active_action_count() > 0 {
            any_active_action = true;
        }
        if acc.row.active_run_count as usize >= required_active_runs
            && acc.row.active_action_count() > 0
        {
            persistent_active_role_count += 1;
            persistent_signatures.insert(acc.row.action_signature.clone());
        }
        by_role.insert(role, acc.row);
    }

    let distinct_action_signature_count = persistent_signatures.len() as u64;
    if persistent_active_role_count < 2 {
        failure_reasons
            .push("fewer than two persistent active roles across the compared evidence".into());
    }
    if distinct_action_signature_count < 2 {
        failure_reasons.push("persistent roles do not show distinct action distributions".into());
    }
    if !any_active_action {
        failure_reasons.push("no market/verify/challenge/solver action evidence found".into());
    }
    if audit_tape_proceed_count != runs.len() as u64 {
        failure_reasons.push("not all runs have audit_tape PROCEED".into());
    }

    let has_veto = failure_reasons.iter().any(|reason| {
        reason.contains("scripted_fixture_tx_count")
            || reason.contains("policy_counts_for_e2")
            || reason.contains("verifier verdict VETO")
    });
    let e3_candidate = failure_reasons.is_empty()
        && persistent_active_role_count >= 2
        && distinct_action_signature_count >= 2
        && audit_tape_proceed_count == runs.len() as u64;
    let verdict = if has_veto {
        RoleDifferentiationVerdict::Veto
    } else if e3_candidate {
        RoleDifferentiationVerdict::Proceed
    } else {
        RoleDifferentiationVerdict::CleanNegative
    };

    Ok(RoleDifferentiationReport {
        schema_version: RoleDifferentiationReport::SCHEMA_VERSION.to_string(),
        source_boundary: "ChainTape/CAS role traces plus independent exact-join verifier output"
            .into(),
        run_count: runs.len() as u64,
        audit_tape_proceed_count,
        by_role,
        persistent_active_role_count,
        distinct_action_signature_count,
        e3_candidate,
        verdict,
        failure_reasons,
        residual_risks,
    })
}

fn expected_role_accumulators() -> BTreeMap<String, RoleAccumulator> {
    let mut rows = BTreeMap::new();
    for role in [
        AgentRole::BullTrader,
        AgentRole::BearTrader,
        AgentRole::Solver,
        AgentRole::Verifier,
        AgentRole::Challenger,
        AgentRole::Trader,
        AgentRole::MarketMaker,
        AgentRole::Observer,
    ] {
        rows.insert(
            role.label().to_string(),
            RoleAccumulator {
                row: RoleDifferentiationRoleRow::new(role.label()),
                ..RoleAccumulator::default()
            },
        );
    }
    rows
}

fn validate_e2_report_boundary(
    run_id: &str,
    report: &E2CandidateVerifierReport,
    failure_reasons: &mut Vec<String>,
) {
    if report.verdict == E2CandidateVerifierVerdict::Veto {
        failure_reasons.push(format!("run {run_id} verifier verdict VETO"));
    }
    if report.scripted_fixture_tx_count > 0 {
        failure_reasons.push(format!(
            "run {run_id} scripted_fixture_tx_count={} cannot support E3",
            report.scripted_fixture_tx_count
        ));
    }
    if report.policy_counts_for_e2 {
        failure_reasons.push(format!(
            "run {run_id} policy_counts_for_e2=true cannot support E3"
        ));
    }
    if report.duplicate_l4_router_tx_id_count > 0
        || report.duplicate_submitted_trace_tx_id_count > 0
    {
        failure_reasons.push(format!(
            "run {run_id} duplicate exact-join tx ids cannot support E3"
        ));
    }
}

fn ingest_role_turns(
    run: &RoleDifferentiationRunInput<'_>,
    accumulators: &mut BTreeMap<String, RoleAccumulator>,
) -> Result<(), String> {
    for cid in role_turn_trace_cids(run.cas) {
        let trace = read_role_turn_trace_from_cas(run.cas, &cid).map_err(|e| {
            format!(
                "run {} failed to decode role-turn trace {}: {e}",
                run.run_id, cid
            )
        })?;
        let role = trace.role.label().to_string();
        let acc = accumulators
            .entry(role.clone())
            .or_insert_with(|| RoleAccumulator {
                row: RoleDifferentiationRoleRow::new(&role),
                ..RoleAccumulator::default()
            });
        acc.row.turn_count += 1;
        acc.tasks.insert(trace.task_id.0.clone());
        match trace.outcome {
            RoleTurnOutcome::SubmitProof { .. } => {
                acc.row.work_count += 1;
                acc.active_runs.insert(run.run_id.clone());
            }
            RoleTurnOutcome::VerifyTx { .. } => {
                acc.row.verify_count += 1;
                acc.active_runs.insert(run.run_id.clone());
            }
            RoleTurnOutcome::ChallengeTx { .. } => {
                acc.row.challenge_count += 1;
                acc.active_runs.insert(run.run_id.clone());
            }
            RoleTurnOutcome::MarketDecision { .. } => {
                acc.row.market_decision_turn_count += 1;
            }
            RoleTurnOutcome::NoTrade { .. } => acc.row.no_trade_count += 1,
            RoleTurnOutcome::NoVerify(_) => acc.row.no_verify_count += 1,
            RoleTurnOutcome::NoChallenge(_) => acc.row.no_challenge_count += 1,
            RoleTurnOutcome::ToolProposal { .. }
            | RoleTurnOutcome::VetoDecision { .. }
            | RoleTurnOutcome::Abstain { .. }
            | RoleTurnOutcome::ParseFailed { .. }
            | RoleTurnOutcome::PolicyRejected { .. } => {}
        }
    }
    Ok(())
}

fn ingest_exact_join_market_actions(
    run: &RoleDifferentiationRunInput<'_>,
    accumulators: &mut BTreeMap<String, RoleAccumulator>,
    residual_risks: &mut Vec<String>,
) {
    for row in &run.e2_report.matched_tx_provenance {
        let Some(role) = row.live_agent_role.as_deref() else {
            residual_risks.push(format!(
                "run {} tx {} has no live_agent_role",
                run.run_id, row.tx_id
            ));
            continue;
        };
        if row.actor_is_policy_trader || !row.actor_is_live_agent_role {
            residual_risks.push(format!(
                "run {} tx {} excluded from role differentiation actor flags",
                run.run_id, row.tx_id
            ));
            continue;
        }
        let acc = accumulators
            .entry(role.to_string())
            .or_insert_with(|| RoleAccumulator {
                row: RoleDifferentiationRoleRow::new(role),
                ..RoleAccumulator::default()
            });
        acc.row.exact_join_market_action_count += 1;
        match row.l4_direction.as_str() {
            "BuyYes" => acc.row.buy_yes_count += 1,
            "BuyNo" => acc.row.buy_no_count += 1,
            other => residual_risks.push(format!(
                "run {} tx {} has unknown l4_direction={other}",
                run.run_id, row.tx_id
            )),
        }
        acc.tasks.insert(row.l4_event_id.clone());
        acc.active_runs.insert(run.run_id.clone());
        if !row.residual_risks.is_empty() {
            residual_risks.push(format!(
                "run {} tx {} verifier residual risks: {}",
                run.run_id,
                row.tx_id,
                row.residual_risks.len()
            ));
        }
    }
}
