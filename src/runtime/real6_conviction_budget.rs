//! REAL-6C — ConvictionBudget / PnL feedback.
//!
//! Lawful pressure rule: free cognition, paid conviction. This module is a
//! pure materialized view over canonical `QState` economic indices and the G3
//! PnL view. It does not mutate balances, admission, predicates, or sequencer
//! rules.

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::Cid;
use crate::bottom_white::cas::store::CasStore;
use crate::bottom_white::ledger::rejection_evidence::RejectionClass;
use crate::economy::money::MicroCoin;
use crate::runtime::agent_pnl::{
    bankruptcy_risk_cap_micro, compute_agent_pnl, initial_balance_micro_from_default_preseed,
    OpenPosition,
};
use crate::runtime::autopsy_capsule::{
    write_autopsy_capsule, AgentAutopsyCapsule, AutopsyWriteError, LossReasonClass,
};
use crate::runtime::real5_roles::{route_role_action, AgentRole, RoleAction, RoleActionRoute};
use crate::state::q_state::{AgentId, QState, TaskId};
use crate::state::typed_tx::{CapsulePrivacyPolicy, EventId, RiskRuleId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConvictionBudget {
    pub agent_id: AgentId,
    pub available_micro: i64,
    pub reserved_micro: i64,
    pub realized_pnl: i64,
    pub unrealized_pnl: i64,
    pub risk_cap: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConvictionAction {
    Observe,
    Read,
    Abstain,
    Solve,
    Verify,
    HighRiskMarket,
    HighRiskChallenge,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConvictionActionAvailability {
    pub allowed: bool,
    pub reason: Option<String>,
}

pub fn derive_conviction_budget(q: &QState, agent_id: &AgentId) -> ConvictionBudget {
    let initial = initial_balance_micro_from_default_preseed(agent_id);
    let view = compute_agent_pnl(q, agent_id, initial);
    ConvictionBudget {
        agent_id: agent_id.clone(),
        available_micro: view.balance,
        reserved_micro: reserved_micro_from_positions(&view.open_positions),
        realized_pnl: view.realized_pnl,
        unrealized_pnl: view.unrealized_pnl,
        risk_cap: bankruptcy_risk_cap_micro(agent_id, q),
    }
}

fn reserved_micro_from_positions(positions: &[OpenPosition]) -> i64 {
    let mut total: i64 = 0;
    for position in positions {
        let amount = match position {
            OpenPosition::Stake { amount_micro, .. } => *amount_micro,
            OpenPosition::Claim { .. } => 0,
            OpenPosition::ConditionalShare { units, .. } => u128_to_i64_saturating(*units / 2),
            OpenPosition::LpShare { units, .. } => u128_to_i64_saturating(*units),
            OpenPosition::NodePosition { amount_micro, .. } => *amount_micro,
        };
        total = total.saturating_add(amount.max(0));
    }
    total
}

fn u128_to_i64_saturating(value: u128) -> i64 {
    if value > i64::MAX as u128 {
        i64::MAX
    } else {
        value as i64
    }
}

pub fn conviction_action_allowed(
    budget: &ConvictionBudget,
    role: AgentRole,
    action: ConvictionAction,
) -> ConvictionActionAvailability {
    let high_risk_blocked = budget.available_micro < budget.risk_cap
        && matches!(
            (role, action),
            (AgentRole::Trader, ConvictionAction::HighRiskMarket)
                | (AgentRole::MarketMaker, ConvictionAction::HighRiskMarket)
                | (AgentRole::Challenger, ConvictionAction::HighRiskChallenge)
        );

    if high_risk_blocked {
        return ConvictionActionAvailability {
            allowed: false,
            reason: Some(format!(
                "below_conviction_risk_cap: available_micro={} risk_cap={}",
                budget.available_micro, budget.risk_cap
            )),
        };
    }

    ConvictionActionAvailability {
        allowed: true,
        reason: None,
    }
}

pub fn route_role_action_with_conviction_budget(
    role: AgentRole,
    action: &RoleAction,
    budget: Option<&ConvictionBudget>,
) -> RoleActionRoute {
    let route = route_role_action(role, action);
    if !matches!(route, RoleActionRoute::L4 { .. }) {
        return route;
    }

    let Some(conviction_action) = high_risk_conviction_action(action) else {
        return route;
    };

    let Some(budget) = budget else {
        return RoleActionRoute::L4E {
            rejection_class: RejectionClass::PolicyViolation,
            public_summary: "conviction_budget_unavailable_for_high_risk_action".into(),
        };
    };

    let availability = conviction_action_allowed(budget, role, conviction_action);
    if availability.allowed {
        route
    } else {
        RoleActionRoute::L4E {
            rejection_class: RejectionClass::PolicyViolation,
            public_summary: availability
                .reason
                .unwrap_or_else(|| "below_conviction_risk_cap_for_high_risk_action".to_string()),
        }
    }
}

fn high_risk_conviction_action(action: &RoleAction) -> Option<ConvictionAction> {
    match action {
        RoleAction::Invest(_) | RoleAction::ProvideLiquidity(_) => {
            Some(ConvictionAction::HighRiskMarket)
        }
        RoleAction::ChallengeNode(_) => Some(ConvictionAction::HighRiskChallenge),
        _ => None,
    }
}

pub fn render_scoped_conviction_budget_summary(q: &QState, agent_id: &AgentId) -> String {
    let budget = derive_conviction_budget(q, agent_id);
    format!(
        "=== Conviction Budget ===\n\
         source=ChainTape/CAS-derived QState fold\n\
         agent_id={agent}\n\
         available_micro={available}\n\
         reserved_micro={reserved}\n\
         realized_pnl={realized}\n\
         unrealized_pnl={unrealized}\n\
         risk_cap={risk_cap}\n\
         free_cognition=true\n\
         paid_conviction=true\n",
        agent = budget.agent_id.0,
        available = budget.available_micro,
        reserved = budget.reserved_micro,
        realized = budget.realized_pnl,
        unrealized = budget.unrealized_pnl,
        risk_cap = budget.risk_cap,
    )
}

pub fn write_significant_loss_autopsy_to_cas(
    cas: &std::sync::Arc<std::sync::RwLock<CasStore>>,
    budget: &ConvictionBudget,
    task_id: TaskId,
    threshold: MicroCoin,
    evidence_cids: Vec<Cid>,
    creator_str: &str,
    created_at_logical_t: u64,
    created_at_round: u64,
) -> Result<Option<AgentAutopsyCapsule>, AutopsyWriteError> {
    let total_pnl = budget.realized_pnl.saturating_add(budget.unrealized_pnl);
    if total_pnl >= 0 {
        return Ok(None);
    }
    let loss_micro = total_pnl.saturating_abs();
    if loss_micro < threshold.micro_units() {
        return Ok(None);
    }

    let event_id = EventId(task_id);
    let loss_amount = MicroCoin::from_micro_units(loss_micro);
    let private_detail = format!(
        "real6c_conviction_loss agent={} total_pnl={} available_micro={} reserved_micro={} risk_cap={}",
        budget.agent_id.0,
        total_pnl,
        budget.available_micro,
        budget.reserved_micro,
        budget.risk_cap
    );

    write_autopsy_capsule(
        cas,
        budget.agent_id.clone(),
        event_id,
        loss_amount,
        LossReasonClass::Overleverage,
        Some(RiskRuleId("real6c.conviction_budget".into())),
        None,
        evidence_cids,
        private_detail.as_bytes(),
        CapsulePrivacyPolicy::AuditOnly,
        creator_str,
        created_at_logical_t,
        created_at_round,
    )
    .map(Some)
}
