//! REAL-17 — direct submitted MarketDecisionTrace provenance sidecar.
//!
//! This additive CAS evidence links a submitted `MarketDecisionTrace` CID to
//! the same-turn `PromptCapsuleV2` CID without changing TypedTx, sequencer
//! admission, canonical signing payloads, or CAS ObjectType schema.

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::runtime::market_decision_trace::{MarketDecisionTrace, TraceOutcome};
use crate::runtime::prompt_capsule::read_prompt_capsule_v2_from_cas;
use crate::state::q_state::{AgentId, TxId};

/// TRACE_MATRIX FC1/FC3: schema id for REAL-17 direct prompt provenance links.
pub const MARKET_DECISION_PROVENANCE_LINK_SCHEMA_ID: &str =
    "real17.market_decision_provenance_link.v1";

/// TRACE_MATRIX FC1/FC3: CAS sidecar binding a submitted market decision to
/// its prompt capsule. Optional EV/opportunity CIDs may be populated by future
/// wiring, but the prompt link is mandatory for direct provenance coverage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketDecisionProvenanceLink {
    pub schema_version: String,
    pub market_decision_trace_cid: Cid,
    pub submitted_router_tx_id: TxId,
    pub agent_id: AgentId,
    pub prompt_capsule_cid: Cid,
    pub ev_decision_trace_cid: Option<Cid>,
    pub market_opportunity_trace_cid: Option<Cid>,
    pub created_at_logical_t: u64,
    pub public_summary: String,
}

impl MarketDecisionProvenanceLink {
    /// TRACE_MATRIX FC1/FC3: canonical schema version for REAL-17 prompt
    /// provenance sidecars.
    pub const SCHEMA_VERSION: &'static str = MARKET_DECISION_PROVENANCE_LINK_SCHEMA_ID;
}

/// TRACE_MATRIX FC3: validate sidecar links against CAS contents so dashboard
/// or verifier views cannot manufacture prompt provenance from unanchored JSON.
pub fn validate_market_decision_provenance_link(
    cas: &CasStore,
    link: &MarketDecisionProvenanceLink,
) -> Result<(), String> {
    if link.schema_version != MARKET_DECISION_PROVENANCE_LINK_SCHEMA_ID {
        return Err(format!(
            "unexpected MarketDecisionProvenanceLink schema_version={}",
            link.schema_version
        ));
    }
    if link.submitted_router_tx_id.0.trim().is_empty() {
        return Err("MarketDecisionProvenanceLink submitted_router_tx_id must be non-empty".into());
    }
    if link.agent_id.0.trim().is_empty() {
        return Err("MarketDecisionProvenanceLink agent_id must be non-empty".into());
    }
    if link.public_summary.trim().is_empty() {
        return Err("MarketDecisionProvenanceLink public_summary must be non-empty".into());
    }
    if contains_forbidden_private_material(&link.public_summary) {
        return Err(
            "MarketDecisionProvenanceLink public_summary contains forbidden raw material marker"
                .into(),
        );
    }

    let decision_bytes = cas
        .get(&link.market_decision_trace_cid)
        .map_err(|e| format!("MarketDecisionTrace read failed: {e}"))?;
    let decision: MarketDecisionTrace = serde_json::from_slice(&decision_bytes)
        .map_err(|e| format!("MarketDecisionTrace decode failed: {e}"))?;
    if decision.schema_version != MarketDecisionTrace::SCHEMA_VERSION {
        return Err(format!(
            "linked MarketDecisionTrace schema_version={} is not {}",
            decision.schema_version,
            MarketDecisionTrace::SCHEMA_VERSION
        ));
    }
    if decision.agent_id != link.agent_id {
        return Err(format!(
            "MarketDecisionProvenanceLink agent mismatch: link={} trace={}",
            link.agent_id.0, decision.agent_id.0
        ));
    }
    match &decision.outcome {
        TraceOutcome::Submitted { tx_id } if tx_id == &link.submitted_router_tx_id => {}
        TraceOutcome::Submitted { tx_id } => {
            return Err(format!(
                "MarketDecisionProvenanceLink router tx mismatch: link={} trace={}",
                link.submitted_router_tx_id.0, tx_id.0
            ));
        }
        _ => {
            return Err(
                "MarketDecisionProvenanceLink must point to a submitted MarketDecisionTrace".into(),
            );
        }
    }

    let prompt_capsule = read_prompt_capsule_v2_from_cas(cas, &link.prompt_capsule_cid)
        .map_err(|e| format!("PromptCapsuleV2 read failed: {e}"))?;
    if prompt_capsule.agent_id != link.agent_id {
        return Err(format!(
            "MarketDecisionProvenanceLink prompt agent mismatch: link={} capsule={}",
            link.agent_id.0, prompt_capsule.agent_id.0
        ));
    }
    Ok(())
}

/// TRACE_MATRIX FC1/FC3: write a validated direct prompt-provenance link to
/// CAS as a Generic schema-tagged sidecar.
pub fn write_market_decision_provenance_link_to_cas(
    cas: &mut CasStore,
    link: &MarketDecisionProvenanceLink,
    suffix: &str,
    logical_t: u64,
) -> Result<Cid, CasError> {
    validate_market_decision_provenance_link(cas, link).map_err(|e| {
        CasError::BackendCorruption(format!("MarketDecisionProvenanceLink invalid: {e}"))
    })?;
    let bytes = serde_json::to_vec(link).map_err(|e| {
        CasError::BackendCorruption(format!("MarketDecisionProvenanceLink encode: {e}"))
    })?;
    cas.put(
        &bytes,
        ObjectType::Generic,
        &format!("real17-market-decision-provenance-link-{suffix}"),
        logical_t,
        Some(MARKET_DECISION_PROVENANCE_LINK_SCHEMA_ID.to_string()),
    )
}

/// TRACE_MATRIX FC3: read and validate one sidecar from CAS.
pub fn read_market_decision_provenance_link_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<MarketDecisionProvenanceLink, CasError> {
    let bytes = cas.get(cid)?;
    let link: MarketDecisionProvenanceLink = serde_json::from_slice(&bytes).map_err(|e| {
        CasError::BackendCorruption(format!("MarketDecisionProvenanceLink decode: {e}"))
    })?;
    validate_market_decision_provenance_link(cas, &link).map_err(|e| {
        CasError::BackendCorruption(format!("MarketDecisionProvenanceLink invalid: {e}"))
    })?;
    Ok(link)
}

/// TRACE_MATRIX FC3: enumerate REAL-17 direct provenance sidecars by metadata
/// schema id, not by dashboard text.
pub fn market_decision_provenance_link_cids(cas: &CasStore) -> Vec<Cid> {
    cas.list_cids_by_schema_id(MARKET_DECISION_PROVENANCE_LINK_SCHEMA_ID)
}

/// TRACE_MATRIX FC3: read all sidecars in CAS metadata order.
pub fn list_market_decision_provenance_links_from_cas(
    cas: &CasStore,
) -> Result<Vec<(Cid, MarketDecisionProvenanceLink)>, CasError> {
    market_decision_provenance_link_cids(cas)
        .into_iter()
        .map(|cid| {
            let link = read_market_decision_provenance_link_from_cas(cas, &cid)?;
            Ok((cid, link))
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
