//! REAL-17P21 — voluntary market order ticket sidecar.
//!
//! `MarketOrderTicket` externalizes each Bull/Bear market-review turn as a
//! structured, CAS-backed choice. It may require a ticket, but it does not
//! require a non-zero trade: `amount_micro=0` remains the constitutional
//! representation of a voluntary abstain.

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::runtime::ev_decision_trace::{
    read_ev_decision_trace_from_cas, EVAction, EVDecisionTrace,
};
use crate::runtime::prompt_capsule::read_prompt_capsule_v2_from_cas;
use crate::runtime::real5_roles::{AgentRole, MarketSide};
use crate::state::q_state::{AgentId, TxId};
use crate::state::typed_tx::EventId;

/// TRACE_MATRIX FC1/FC3: schema id for REAL-17P21 market-order ticket
/// sidecars. This is a Generic CAS record, not a TypedTx or sequencer schema.
pub const MARKET_ORDER_TICKET_SCHEMA_ID: &str = "real17p21.market_order_ticket.v1";

/// TRACE_MATRIX FC1: the explicit market-review choice captured by a ticket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketOrderTicketChoice {
    BuyYes,
    BuyNo,
    Abstain,
}

/// TRACE_MATRIX FC1/FC3: CAS evidence for a Bull/Bear trader turn. Quote
/// fields are public previews and cannot be used as predicate truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketOrderTicket {
    pub schema_version: String,
    pub review_window_id: TxId,
    pub review_response_id: String,
    pub agent_id: AgentId,
    pub role: AgentRole,
    pub event_id: EventId,
    pub role_allowed_action: String,
    pub side: MarketSide,
    pub candidate_amount_micro: i64,
    pub final_amount_micro: i64,
    pub choice: MarketOrderTicketChoice,
    pub quote_direction: String,
    pub quoted_out_shares_micro: Option<i64>,
    pub quoted_get_shares_micro: Option<i64>,
    pub quoted_effective_price_num: Option<i64>,
    pub quoted_effective_price_den: Option<i64>,
    pub router_liquidity_warning: String,
    pub balance_ok: bool,
    pub risk_ok: bool,
    pub liquidity_ok: bool,
    pub slippage_ok: Option<bool>,
    pub edge_bps: Option<i64>,
    pub expected_value_micro: Option<i128>,
    pub blocking_constraints: Vec<String>,
    pub prompt_capsule_cid: Cid,
    pub ev_decision_trace_cid: Cid,
    pub submitted_router_tx_id: Option<TxId>,
    pub forced_nonzero_trade: bool,
    pub public_summary: String,
}

impl MarketOrderTicket {
    /// TRACE_MATRIX FC1/FC3: canonical schema tag for REAL-17P21 tickets.
    pub const SCHEMA_VERSION: &'static str = MARKET_ORDER_TICKET_SCHEMA_ID;
}

/// TRACE_MATRIX FC3: validate a ticket against linked CAS evidence. This
/// prevents report/dashboard text from manufacturing a market-review choice.
pub fn validate_market_order_ticket(
    cas: &CasStore,
    ticket: &MarketOrderTicket,
) -> Result<(), String> {
    if ticket.schema_version != MARKET_ORDER_TICKET_SCHEMA_ID {
        return Err(format!(
            "unexpected MarketOrderTicket schema_version={}",
            ticket.schema_version
        ));
    }
    if ticket.review_window_id.0.trim().is_empty() {
        return Err("MarketOrderTicket review_window_id must be non-empty".into());
    }
    if ticket.review_response_id.trim().is_empty() {
        return Err("MarketOrderTicket review_response_id must be non-empty".into());
    }
    if ticket.agent_id.0.trim().is_empty() {
        return Err("MarketOrderTicket agent_id must be non-empty".into());
    }
    if ticket.candidate_amount_micro < 0 || ticket.final_amount_micro < 0 {
        return Err("MarketOrderTicket amounts must be non-negative".into());
    }
    if ticket.forced_nonzero_trade {
        return Err("MarketOrderTicket forced_nonzero_trade is forbidden".into());
    }
    if ticket.public_summary.trim().is_empty() {
        return Err("MarketOrderTicket public_summary must be non-empty".into());
    }
    if contains_forbidden_private_material(&ticket.public_summary) {
        return Err("MarketOrderTicket public_summary contains private/raw material".into());
    }

    match ticket.role {
        AgentRole::BullTrader => {
            if ticket.side != MarketSide::Yes
                || ticket.role_allowed_action != "buy_yes"
                || ticket.quote_direction != "buy_yes"
                || ticket.choice == MarketOrderTicketChoice::BuyNo
            {
                return Err("BullTrader MarketOrderTicket must stay on YES side".into());
            }
        }
        AgentRole::BearTrader => {
            if ticket.side != MarketSide::No
                || ticket.role_allowed_action != "buy_no"
                || ticket.quote_direction != "buy_no"
                || ticket.choice == MarketOrderTicketChoice::BuyYes
            {
                return Err("BearTrader MarketOrderTicket must stay on NO side".into());
            }
        }
        other => {
            return Err(format!(
                "MarketOrderTicket requires BullTrader/BearTrader, got {}",
                other.label()
            ));
        }
    }

    match ticket.choice {
        MarketOrderTicketChoice::Abstain => {
            if ticket.final_amount_micro != 0 {
                return Err("Abstain MarketOrderTicket final_amount_micro must be 0".into());
            }
            if ticket.submitted_router_tx_id.is_some() {
                return Err("Abstain MarketOrderTicket cannot carry submitted_router_tx_id".into());
            }
        }
        MarketOrderTicketChoice::BuyYes | MarketOrderTicketChoice::BuyNo => {
            if ticket.final_amount_micro <= 0 {
                return Err("Buy MarketOrderTicket final_amount_micro must be positive".into());
            }
        }
    }

    let prompt_capsule = read_prompt_capsule_v2_from_cas(cas, &ticket.prompt_capsule_cid)
        .map_err(|e| format!("PromptCapsuleV2 read failed: {e}"))?;
    if prompt_capsule.agent_id != ticket.agent_id || prompt_capsule.role != ticket.role {
        return Err(format!(
            "MarketOrderTicket prompt mismatch: ticket={}/{} capsule={}/{}",
            ticket.agent_id.0,
            ticket.role.label(),
            prompt_capsule.agent_id.0,
            prompt_capsule.role.label()
        ));
    }

    let ev_trace = read_ev_decision_trace_from_cas(cas, &ticket.ev_decision_trace_cid)
        .map_err(|e| format!("EVDecisionTrace read failed: {e}"))?;
    validate_ticket_matches_ev(ticket, &ev_trace)?;
    Ok(())
}

fn validate_ticket_matches_ev(
    ticket: &MarketOrderTicket,
    ev_trace: &EVDecisionTrace,
) -> Result<(), String> {
    if ev_trace.agent_id != ticket.agent_id
        || ev_trace.role != ticket.role
        || ev_trace.event_id != ticket.event_id
        || ev_trace.side != ticket.side
        || ev_trace.prompt_capsule_cid != ticket.prompt_capsule_cid
    {
        return Err("MarketOrderTicket does not match linked EVDecisionTrace".into());
    }

    let ev_choice = match ev_trace.action {
        EVAction::BuyYes => MarketOrderTicketChoice::BuyYes,
        EVAction::BuyNo => MarketOrderTicketChoice::BuyNo,
        EVAction::Abstain => MarketOrderTicketChoice::Abstain,
    };
    if ticket.choice != ev_choice {
        return Err("MarketOrderTicket choice does not match linked EVDecisionTrace".into());
    }
    if ticket.edge_bps != ev_trace.edge_bps {
        return Err("MarketOrderTicket edge_bps does not match linked EVDecisionTrace".into());
    }
    if ticket.expected_value_micro != ev_trace.expected_value_micro {
        return Err(
            "MarketOrderTicket expected_value_micro does not match linked EVDecisionTrace".into(),
        );
    }
    Ok(())
}

/// TRACE_MATRIX FC1/FC3: write a validated ticket to CAS as a Generic
/// schema-tagged sidecar.
pub fn write_market_order_ticket_to_cas(
    cas: &mut CasStore,
    ticket: &MarketOrderTicket,
    suffix: &str,
    logical_t: u64,
) -> Result<Cid, CasError> {
    validate_market_order_ticket(cas, ticket)
        .map_err(|e| CasError::BackendCorruption(format!("MarketOrderTicket invalid: {e}")))?;
    let bytes = serde_json::to_vec(ticket)
        .map_err(|e| CasError::BackendCorruption(format!("MarketOrderTicket encode: {e}")))?;
    cas.put(
        &bytes,
        ObjectType::Generic,
        &format!("real17p21-market-order-ticket-{suffix}"),
        logical_t,
        Some(MARKET_ORDER_TICKET_SCHEMA_ID.to_string()),
    )
}

/// TRACE_MATRIX FC3: read and validate one ticket from CAS.
pub fn read_market_order_ticket_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<MarketOrderTicket, CasError> {
    let bytes = cas.get(cid)?;
    let ticket: MarketOrderTicket = serde_json::from_slice(&bytes)
        .map_err(|e| CasError::BackendCorruption(format!("MarketOrderTicket decode: {e}")))?;
    validate_market_order_ticket(cas, &ticket)
        .map_err(|e| CasError::BackendCorruption(format!("MarketOrderTicket invalid: {e}")))?;
    Ok(ticket)
}

/// TRACE_MATRIX FC3: enumerate ticket CIDs from CAS metadata.
pub fn market_order_ticket_cids(cas: &CasStore) -> Vec<Cid> {
    cas.list_cids_by_schema_id(MARKET_ORDER_TICKET_SCHEMA_ID)
}

/// TRACE_MATRIX FC3: read all ticket sidecars in CAS metadata order.
pub fn list_market_order_tickets_from_cas(
    cas: &CasStore,
) -> Result<Vec<(Cid, MarketOrderTicket)>, CasError> {
    market_order_ticket_cids(cas)
        .into_iter()
        .map(|cid| {
            let ticket = read_market_order_ticket_from_cas(cas, &cid)?;
            Ok((cid, ticket))
        })
        .collect()
}

fn contains_forbidden_private_material(summary: &str) -> bool {
    let lower = summary.to_ascii_lowercase();
    [
        "raw_prompt",
        "raw prompt",
        "raw_completion",
        "raw completion",
        "private cot",
        "chain of thought",
        "raw_log",
        "raw log",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}
