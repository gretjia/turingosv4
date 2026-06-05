//! A07 generic AgentView projection.
//!
//! This module derives an ordinary agent read view from A05 `TapeEventEnvelope`
//! inputs. It is a read-only projection: it does not read dashboards, move tape
//! refs, mutate prompt schemas, or authorize sequencer admission.

use std::collections::BTreeSet;

use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::Cid;
use crate::runtime::tape_event::{TapeEventEnvelope, TapeEventError};
use crate::state::q_state::AgentId;

/// TRACE_MATRIX FC1-N5 + FC1-N6 + FC1-N7 + FC3-N31: request for one prefix-bound ordinary agent read view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentViewRequest {
    pub agent_id: AgentId,
    pub role_label: Option<String>,
    pub view_policy_id: String,
    pub allowed_tape_prefix_head: String,
}

/// TRACE_MATRIX FC1-N5 + FC1-N6 + FC1-N7 + FC3-N31: in-memory shielding policy used while deriving AgentView.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AgentViewPolicy {
    pub redacted_fields: Vec<String>,
    pub denied_cids: Vec<Cid>,
}

/// TRACE_MATRIX FC1-N5 + FC1-N6 + FC1-N7 + FC3-N31: serialized ordinary agent read view derived from a granted tape prefix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentView {
    pub agent_id: AgentId,
    pub role_label: Option<String>,
    pub view_policy_id: String,
    pub allowed_tape_prefix_head: String,
    #[serde(serialize_with = "serialize_cid_as_hex")]
    pub visible_context_cid: Cid,
    #[serde(serialize_with = "serialize_cid_as_hex")]
    pub visible_context_hash: Cid,
    #[serde(serialize_with = "serialize_cids_as_hex")]
    pub visible_event_cids: Vec<Cid>,
    pub visible_context_bytes: Vec<u8>,
    #[serde(skip)]
    pub redacted_fields: Vec<String>,
}

#[derive(Serialize)]
struct CanonicalAgentViewContext<'a> {
    agent_id: &'a AgentId,
    role_label: &'a Option<String>,
    view_policy_id: &'a str,
    allowed_tape_prefix_head: &'a str,
    visible_event_cids: &'a [Cid],
}

/// TRACE_MATRIX FC1-N5 + FC1-N6 + FC1-N7 + FC3-N31: derive AgentView from canonical tape events and fail closed on derived-view refs.
pub fn derive_agent_view(
    request: AgentViewRequest,
    policy: AgentViewPolicy,
    events: &[TapeEventEnvelope],
) -> Result<AgentView, AgentViewError> {
    let denied: BTreeSet<Cid> = policy.denied_cids.iter().copied().collect();
    let mut visible_event_cids = Vec::new();

    for event in events {
        event.validate()?;
        if event.tape_ref.head_oid_hex() != request.allowed_tape_prefix_head {
            continue;
        }
        if let Some(cid) = event.payload_cid {
            if !denied.contains(&cid) {
                visible_event_cids.push(cid);
            }
        }
    }

    let visible_context_bytes = serde_json::to_vec(&CanonicalAgentViewContext {
        agent_id: &request.agent_id,
        role_label: &request.role_label,
        view_policy_id: &request.view_policy_id,
        allowed_tape_prefix_head: &request.allowed_tape_prefix_head,
        visible_event_cids: &visible_event_cids,
    })
    .map_err(|e| AgentViewError::Encode(e.to_string()))?;
    let visible_context_cid = Cid::from_content(&visible_context_bytes);

    Ok(AgentView {
        agent_id: request.agent_id,
        role_label: request.role_label,
        view_policy_id: request.view_policy_id,
        allowed_tape_prefix_head: request.allowed_tape_prefix_head,
        visible_context_cid,
        visible_context_hash: visible_context_cid,
        visible_event_cids,
        visible_context_bytes,
        redacted_fields: policy.redacted_fields,
    })
}

/// TRACE_MATRIX FC1-N5 + FC1-N6 + FC1-N7 + FC3-N31: typed AgentView derivation errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentViewError {
    TapeEvent(TapeEventError),
    Encode(String),
}

impl From<TapeEventError> for AgentViewError {
    fn from(value: TapeEventError) -> Self {
        Self::TapeEvent(value)
    }
}

impl std::fmt::Display for AgentViewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TapeEvent(e) => write!(f, "invalid tape event for AgentView: {e}"),
            Self::Encode(e) => write!(f, "encode AgentView context: {e}"),
        }
    }
}

impl std::error::Error for AgentViewError {}

fn serialize_cid_as_hex<S>(cid: &Cid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&cid.hex())
}

fn serialize_cids_as_hex<S>(cids: &[Cid], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut seq = serializer.serialize_seq(Some(cids.len()))?;
    for cid in cids {
        seq.serialize_element(&cid.hex())?;
    }
    seq.end()
}
