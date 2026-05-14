//! TB-G G5 — public activity role classifier.
//!
//! Derived only from public ChainTape/CAS activity counts. No prompt body,
//! completion, or private chain-of-thought is an input.

/// TRACE_MATRIX FC3-N43 + Art.III shielding: public G5 role label derived from
/// accepted ChainTape activity counts only, never raw prompt/completion/CoT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AgentRole {
    Solver,
    Verifier,
    Challenger,
    Trader,
    Observer,
}

impl AgentRole {
    /// TRACE_MATRIX FC3-N43 + Art.III shielding: stable display label for the
    /// materialized activity report; not a model-ranking or emergence claim.
    pub fn label(self) -> &'static str {
        match self {
            Self::Solver => "Solver",
            Self::Verifier => "Verifier",
            Self::Challenger => "Challenger",
            Self::Trader => "Trader",
            Self::Observer => "Observer",
        }
    }
}

/// TRACE_MATRIX FC3-N43 + Art.III shielding: public activity counters consumed
/// by the role classifier; all fields are ChainTape-visible aggregates.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RoleActivity {
    pub work_tx_accepted: u64,
    pub verify_tx_accepted: u64,
    pub challenge_tx_accepted: u64,
    pub invest_tx_accepted: u64,
}

/// TRACE_MATRIX FC3-N43 + Art.III shielding: classify a role from public
/// accepted-tx counts only; no private agent reasoning or hidden telemetry.
pub fn classify_agent_role(activity: &RoleActivity) -> AgentRole {
    let candidates = [
        (activity.challenge_tx_accepted, AgentRole::Challenger),
        (activity.verify_tx_accepted, AgentRole::Verifier),
        (activity.invest_tx_accepted, AgentRole::Trader),
        (activity.work_tx_accepted, AgentRole::Solver),
    ];
    candidates
        .into_iter()
        .max_by_key(|(count, _)| *count)
        .filter(|(count, _)| *count > 0)
        .map(|(_, role)| role)
        .unwrap_or(AgentRole::Observer)
}

/// TRACE_MATRIX FC3-N43: render §I as a materialized dashboard view with a
/// mechanism-bottleneck explanation when public activity does not yet show
/// differentiated roles.
pub fn render_role_activity_section(rows: &[(String, RoleActivity)]) -> String {
    use std::collections::BTreeSet;

    let mut out = String::new();
    out.push_str("\n## §I Role activity classifier\n");
    out.push_str("  source: public ChainTape/CAS activity counts only\n");
    let mut roles = BTreeSet::new();
    for (agent, activity) in rows {
        let role = classify_agent_role(activity);
        roles.insert(role);
        out.push_str(&format!(
            "  {agent}: role={} work={} verify={} challenge={} invest={}\n",
            role.label(),
            activity.work_tx_accepted,
            activity.verify_tx_accepted,
            activity.challenge_tx_accepted,
            activity.invest_tx_accepted
        ));
    }
    if roles.len() < 2 {
        out.push_str("  MECHANISM BOTTLENECK: fewer than two distinct roles observed.\n");
        out.push_str(
            "  - agents may have solved or halted before market/verify opportunities appeared\n",
        );
        out.push_str(
            "  - prompt context may not have exposed actionable peer-review or market edges\n",
        );
        out.push_str(
            "  - incentives or scheduler ordering may not yet create differentiated behavior\n",
        );
    }
    out
}
