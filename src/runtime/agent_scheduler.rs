//! TB-G G5 — observe-only opportunity scheduler helper.
//!
//! This module is intentionally pure. It records which agent would be selected
//! under a scheduler mode; it does not mutate QState or replace sequencer
//! admission.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::bottom_white::ledger::transition_ledger::{
    canonical_decode, canonical_encode, CanonicalCodecError,
};

// S3 — Boltzmann scheduler observe-only selection trace. Nested here as a
// `#[path]` submodule because `agent_scheduler.rs` is an UNPINNED file already
// declared in the trust-root-pinned `runtime/mod.rs`; nesting avoids editing
// the pinned `mod.rs`. Access path:
// `crate::runtime::agent_scheduler::boltzmann_selection_trace::*`.
/// TRACE_MATRIX FC2-N29 + FC1-N7: S3 observe-only Boltzmann selection trace
/// (live `boltzmann_select_parent_v2` over canonical `EconomicState`, CAS-anchored).
#[path = "boltzmann_selection_trace.rs"]
pub mod boltzmann_selection_trace;

// LIVE-FC1 — tape-driven FC-liveness OBSERVER. Nested here as a `#[path]`
// submodule for the same reason as `boltzmann_selection_trace`: keep the
// trust-root-pinned `runtime/mod.rs` byte-identical. Access path:
// `crate::runtime::agent_scheduler::fc_liveness_observer::*`.
/// TRACE_MATRIX FC1-N34 + FC2-N31 + FC3-N33: LIVE-FC1 observe-only FC-liveness
/// witness (reconstructs FC1/FC2/FC3 node liveness + no-zombie inventory from
/// L4 + L4.E + CAS; CAS-anchored, never a source of truth).
#[path = "fc_liveness_observer.rs"]
pub mod fc_liveness_observer;

// LIVE-FC1 — tape-canonical Verified-PPUT (VPPUT) reconstruction. Nested here as
// a `#[path]` submodule for the same reason as the two observers above: keep the
// trust-root-pinned `runtime/mod.rs` byte-identical (this file, `agent_scheduler.rs`,
// is genesis-pinned-count 0). Access path:
// `crate::runtime::agent_scheduler::vpput_reconstruction::*`.
/// TRACE_MATRIX FC1-N34 + Art.I.1: LIVE-FC1 tape-canonical VPPUT reconstruction
/// (per-task verified-PPUT + held-out H-VPPUT, reconstructed from L4 + L4.E + CAS;
/// integer-only micro-units, ground-truth gated, shielded — never an agent read view).
#[path = "vpput_reconstruction.rs"]
pub mod vpput_reconstruction;

// LIVE-FC1 Phase 5 — BUDGET HARD-CEILING admission (the Turing fuel = FC2-HALT).
// Nested here as a `#[path]` submodule for the SAME reason as the three modules
// above (`boltzmann_selection_trace` / `fc_liveness_observer` /
// `vpput_reconstruction`): keep the trust-root-pinned `runtime/mod.rs`
// byte-identical (this parent file, `agent_scheduler.rs`, is genesis-pinned-count
// 0). It REUSES the Phase-2 VPPUT `C_i` reconstruction in `vpput_reconstruction`
// (declared immediately above) for the tape-derived integer spend, the PINNED
// read-only `BudgetSnapshot.cost_ceiling_microcoin` field for the ceiling, and
// the PINNED `RejectionClass::BudgetExceeded` discriminant for the reject label —
// ZERO new pinned discriminants, ZERO genesis-pinned-file edits. Access path:
// `crate::runtime::agent_scheduler::budget_ceiling::*`.
/// TRACE_MATRIX FC1a-predicates + FC2-HALT: §8 token
/// APPROVE-BUDGET-HARD-CEILING-FROM-MANIFEST — signed-manifest integer cost
/// ceiling + tape-derived integer spend (reuses VPPUT C_i) + pure pre-admission
/// hard-ceiling check (the external Turing fuel that forces termination).
#[path = "budget_ceiling.rs"]
pub mod budget_ceiling;

// LIVE-FC1 Phase 6 — brand-GENERIC provider identity on the canonical CAS.
// Nested here as a `#[path]` submodule for the SAME reason as the four modules
// above (`boltzmann_selection_trace` / `fc_liveness_observer` /
// `vpput_reconstruction` / `budget_ceiling`): keep the trust-root-pinned
// `runtime/mod.rs` byte-identical (this parent file, `agent_scheduler.rs`, is
// genesis-pinned-count 0). It REUSES the PINNED-FREE `crate::sdk::id_handle`
// (#328) for the generic sha256 provider handle and mirrors the
// `boltzmann_selection_trace` observe-only CAS-capsule pattern (ObjectType::Generic
// + free schema_id + R3 self-addressing + restore/read/cids, no fs-write). The
// brand→handle mapping stays in an EXTERNAL sidecar, NEVER on the canonical tape.
// Access path: `crate::runtime::agent_scheduler::provider_handle_capsule::*`.
/// TRACE_MATRIX FC2-N31 + FC1-N7: LIVE-FC1 Phase 6 brand-generic provider handle
/// capsule (per-agent generic sha256 model handle on canonical CAS; heterogeneity
/// provable WITHOUT brands; brand→handle mapping external sidecar only).
#[path = "provider_handle_capsule.rs"]
pub mod provider_handle_capsule;

// LIVE-FC1 Phase 6 — from-genesis replay-diff acceptance helper. Nested here as
// a `#[path]` submodule for the SAME reason as the modules above: keep the
// trust-root-pinned `runtime/mod.rs` byte-identical (this parent file is
// genesis-pinned-count 0). It REUSES the PINNED `crate::runtime::verify::verify_chaintape`
// (TB-6 Atom 4) replay verifier — it does NOT reimplement replay — and reduces
// the ReplayReport to the from-genesis state_root/ledger_root equality boolean.
// Access path: `crate::runtime::agent_scheduler::replay_diff_acceptance::*`.
/// TRACE_MATRIX FC3-N1 + FC1-N34: LIVE-FC1 Phase 6 from-genesis replay-diff
/// acceptance (reuses the pinned verify_chaintape; asserts reconstructed roots
/// equal recorded roots).
#[path = "replay_diff_acceptance.rs"]
pub mod replay_diff_acceptance;

use crate::runtime::real5_roles::{AgentRole, HeadT, PriceSignal};
use crate::state::price_index::RationalPrice;
use crate::state::q_state::{AgentId, Hash, TxId};

pub const SCHEDULER_DECISION_TRACE_SCHEMA_ID: &str = "real6.scheduler_decision_trace.v1";

/// TRACE_MATRIX FC1-N7 + FC3-N43: G5 closeout scheduler mode is a
/// materialized runtime/reporting helper only; it does not mutate QState or
/// sequencer admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerMode {
    RoundRobin,
    ObserveOnly,
}

/// TRACE_MATRIX FC1-N7 + FC3-N43: public schedule decision witness used by
/// tests and reports to prove observe-only scheduling without hidden market
/// authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentScheduleDecision {
    pub agent_id: Option<AgentId>,
    pub mode: SchedulerMode,
    pub observe_only: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchedulerPnlSignal {
    pub agent_id: AgentId,
    pub realized_pnl: i64,
    pub unrealized_pnl: i64,
    pub available_micro: i64,
    pub risk_cap_micro: i64,
}

/// TRACE_MATRIX FC1-N7 + FC3-N43: REAL-6D observe-only recommendation
/// record. It may carry price/PnL signals into reporting, but it never
/// changes admission, predicates, or task verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchedulerDecisionTrace {
    pub head_t: HeadT,
    pub visible_agents: Vec<AgentId>,
    pub visible_nodes: Vec<TxId>,
    pub price_signals: Vec<PriceSignal>,
    pub pnl_signals: Vec<SchedulerPnlSignal>,
    pub recommended_agent: Option<AgentId>,
    pub recommended_role: Option<AgentRole>,
    pub recommended_action: Option<String>,
    pub observe_only: bool,
}

pub fn write_scheduler_decision_trace_to_cas(
    cas: &mut CasStore,
    trace: &SchedulerDecisionTrace,
    suffix: &str,
    logical_t: u64,
) -> Result<Cid, CasError> {
    let bytes = serde_json::to_vec(trace)
        .map_err(|e| CasError::BackendCorruption(format!("scheduler trace encode: {e}")))?;
    cas.put(
        &bytes,
        ObjectType::Generic,
        &format!("real6-scheduler-decision-trace-{suffix}"),
        logical_t,
        Some(SCHEDULER_DECISION_TRACE_SCHEMA_ID.to_string()),
    )
}

pub fn read_scheduler_decision_trace_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<SchedulerDecisionTrace, CasError> {
    let bytes = cas.get(cid)?;
    serde_json::from_slice(&bytes)
        .map_err(|e| CasError::BackendCorruption(format!("scheduler trace decode: {e}")))
}

pub fn scheduler_decision_trace_cids(cas: &CasStore) -> Vec<Cid> {
    cas.list_all_cids()
        .into_iter()
        .filter(|cid| {
            cas.metadata(cid).and_then(|meta| meta.schema_id.as_deref())
                == Some(SCHEDULER_DECISION_TRACE_SCHEMA_ID)
        })
        .collect()
}

impl AgentScheduleDecision {
    /// TRACE_MATRIX FC1-N7 + FC3-N43: explicit abstain witness for empty or
    /// non-actionable agent sets; no ChainTape mutation is performed here.
    pub fn abstain(reason: impl Into<String>) -> Self {
        Self {
            agent_id: None,
            mode: SchedulerMode::RoundRobin,
            observe_only: true,
            reason: Some(reason.into()),
        }
    }
}

/// TRACE_MATRIX FC1-N7 + FC3-N43: deterministic G5 scheduler helper preserving
/// round-robin back-compat while exposing observe-only mode as reportable
/// evidence.
pub fn schedule_next_agent(
    agents: &[AgentId],
    turn_index: usize,
    mode: SchedulerMode,
) -> AgentScheduleDecision {
    if agents.is_empty() {
        return AgentScheduleDecision::abstain("no_agents_available");
    }
    let idx = turn_index % agents.len();
    AgentScheduleDecision {
        agent_id: Some(agents[idx].clone()),
        mode,
        observe_only: matches!(mode, SchedulerMode::ObserveOnly),
        reason: None,
    }
}

pub fn build_observe_only_scheduler_trace(
    head_t: HeadT,
    visible_agents: Vec<AgentId>,
    visible_nodes: Vec<TxId>,
    price_signals: Vec<PriceSignal>,
    pnl_signals: Vec<SchedulerPnlSignal>,
    recommended_agent: Option<AgentId>,
    recommended_role: Option<AgentRole>,
    recommended_action: Option<String>,
) -> SchedulerDecisionTrace {
    SchedulerDecisionTrace {
        head_t,
        visible_agents,
        visible_nodes,
        price_signals,
        pnl_signals,
        recommended_agent,
        recommended_role,
        recommended_action,
        observe_only: true,
    }
}

pub fn render_scheduler_trace_section(trace: &SchedulerDecisionTrace) -> String {
    let mut out = String::new();
    out.push_str("\n## §J.1 Opportunity Scheduler recommendation (observe-only)\n");
    out.push_str("  interpretation: non-binding materialized view; price is signal, not truth\n");
    out.push_str("  recommendation does not change sequencer admission or L4/L4.E predicates\n");
    out.push_str(&format!("  head_t: {}\n", trace.head_t));
    out.push_str(&format!("  observe_only: {}\n", trace.observe_only));
    out.push_str(&format!(
        "  visible_agents: {}\n",
        trace.visible_agents.len()
    ));
    out.push_str(&format!("  visible_nodes: {}\n", trace.visible_nodes.len()));
    out.push_str(&format!("  price_signals: {}\n", trace.price_signals.len()));
    out.push_str(&format!("  pnl_signals: {}\n", trace.pnl_signals.len()));
    out.push_str(&format!(
        "  recommended_agent: {}\n",
        trace
            .recommended_agent
            .as_ref()
            .map(|a| a.0.as_str())
            .unwrap_or("None")
    ));
    out.push_str(&format!(
        "  recommended_role: {}\n",
        trace.recommended_role.map(|r| r.label()).unwrap_or("None")
    ));
    out.push_str(&format!(
        "  recommended_action: {}\n",
        trace.recommended_action.as_deref().unwrap_or("None")
    ));
    if !trace.price_signals.is_empty() {
        out.push_str("  price_signal_sample:\n");
        for signal in trace.price_signals.iter().take(3) {
            out.push_str(&format!(
                "    - event={} price={} depth_micro={}\n",
                signal.event_id,
                signal.price,
                signal
                    .depth
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "None".into())
            ));
        }
    }
    if !trace.pnl_signals.is_empty() {
        out.push_str("  pnl_signal_sample:\n");
        for signal in trace.pnl_signals.iter().take(3) {
            out.push_str(&format!(
                "    - agent={} realized_pnl={}μC unrealized_pnl={}μC available={}μC risk_cap={}μC\n",
                signal.agent_id.0,
                signal.realized_pnl,
                signal.unrealized_pnl,
                signal.available_micro,
                signal.risk_cap_micro
            ));
        }
    }
    out
}

/// TRACE_MATRIX FC1-N7 + FC1-N13: CAS schema id for replayable scheduler candidates.
pub const A11_SCHEDULER_CANDIDATE_SET_SCHEMA_ID: &str = "scheduler.candidate_set.v1";
/// TRACE_MATRIX FC1-N7 + FC1-N13: CAS schema id for scheduler policy inputs.
pub const A11_SCHEDULER_POLICY_INPUT_SCHEMA_ID: &str = "scheduler.policy_input_bundle.v1";
/// TRACE_MATRIX FC1-N7 + FC1-N13: CAS schema id for the public scheduler view.
pub const A11_SCHEDULER_VIEW_SCHEMA_ID: &str = "scheduler.view.v1";
/// TRACE_MATRIX FC1-N7 + FC1-N13: CAS schema id for observe-only decisions.
pub const A11_SCHEDULER_DECISION_EVENT_SCHEMA_ID: &str = "scheduler.decision_event.v1";

/// TRACE_MATRIX FC1-N7: policy candidate with optional price signal only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchedulerCandidate {
    pub id: String,
    pub price: Option<RationalPrice>,
    pub public_context_cid: Option<Cid>,
}

/// TRACE_MATRIX FC1-N7 + FC1-N13: replay input set bound to one tape head.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchedulerCandidateSet {
    pub input_tape_head: String,
    pub candidates: Vec<SchedulerCandidate>,
}

/// TRACE_MATRIX FC1-N7 + FC1-N13: hash-bound scheduler policy input receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchedulerPolicyInputBundle {
    pub policy_name: String,
    pub policy_version: u32,
    pub input_tape_head: String,
    pub price_projection_head: Option<String>,
    pub scoped_agent_view_head: Option<String>,
    pub candidate_set_hash: Hash,
}

impl SchedulerPolicyInputBundle {
    /// TRACE_MATRIX FC1-N7 + FC1-N13: derive replay input receipt from a decision event.
    pub fn from_decision_event(event: &SchedulerDecisionEvent) -> Self {
        Self {
            policy_name: event.policy_name.clone(),
            policy_version: 1,
            input_tape_head: event.input_tape_head.clone(),
            price_projection_head: event.price_projection_head.clone(),
            scoped_agent_view_head: event.scoped_agent_view_head.clone(),
            candidate_set_hash: event.candidate_set_hash,
        }
    }
}

/// TRACE_MATRIX FC1-N7: explains deterministic or seeded observe-only choice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionReason {
    Seeded { seed: u64 },
    Deterministic { reason: String },
}

/// TRACE_MATRIX FC1-N7 + FC1-N13: observe-only scheduler decision trace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchedulerDecisionEvent {
    pub decision_id: String,
    pub input_tape_head: String,
    pub price_projection_head: Option<String>,
    pub scheduler_view_cid: Cid,
    pub candidate_set_cid: Cid,
    pub candidate_set_hash: Hash,
    pub policy_input_bundle_hash: Hash,
    pub scoped_agent_view_head: Option<String>,
    pub policy_name: String,
    pub selected_agent_or_task: String,
    pub random_seed_or_deterministic_reason: DecisionReason,
}

/// TRACE_MATRIX FC1-N13: public scheduler projection reconstructed from CAS.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchedulerView {
    pub input_tape_head: String,
    pub candidate_set_cid: Cid,
    pub candidate_set_hash: Hash,
    pub policy_input_bundle_hash: Hash,
    pub policy_name: String,
    pub selected_agent_or_task: String,
}

/// TRACE_MATRIX FC1-N7 + FC1-N13: fail-closed scheduler policy construction errors.
#[derive(Debug)]
pub enum SchedulerPolicyError {
    Codec(String),
    Cas(CasError),
    CandidateSetHeadMismatch {
        event_head: String,
        candidate_set_head: String,
    },
    SelectedCandidateMissing(String),
}

impl fmt::Display for SchedulerPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Codec(e) => write!(f, "scheduler_codec_error: {e}"),
            Self::Cas(e) => write!(f, "scheduler_cas_error: {e}"),
            Self::CandidateSetHeadMismatch {
                event_head,
                candidate_set_head,
            } => write!(
                f,
                "candidate_set_head_mismatch: event={event_head} candidate_set={candidate_set_head}"
            ),
            Self::SelectedCandidateMissing(id) => {
                write!(f, "selected_candidate_missing: {id}")
            }
        }
    }
}

impl std::error::Error for SchedulerPolicyError {}

impl From<CasError> for SchedulerPolicyError {
    fn from(value: CasError) -> Self {
        Self::Cas(value)
    }
}

impl From<CanonicalCodecError> for SchedulerPolicyError {
    fn from(value: CanonicalCodecError) -> Self {
        Self::Codec(value.to_string())
    }
}

/// TRACE_MATRIX FC1-N13: fail-closed replay reconstruction errors.
#[derive(Debug)]
pub enum SchedulerReplayError {
    CandidateSetCidMissing(Cid),
    CandidateSetHashMismatch { expected: Hash, computed: Hash },
    Codec(String),
    Cas(CasError),
}

impl fmt::Display for SchedulerReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CandidateSetCidMissing(cid) => {
                write!(f, "candidate_set_cid_missing: {cid}")
            }
            Self::CandidateSetHashMismatch { expected, computed } => write!(
                f,
                "candidate_set_hash_mismatch: expected={} computed={}",
                hash_hex(expected),
                hash_hex(computed)
            ),
            Self::Codec(e) => write!(f, "candidate_set_decode_error: {e}"),
            Self::Cas(e) => write!(f, "candidate_set_cas_error: {e}"),
        }
    }
}

impl std::error::Error for SchedulerReplayError {}

impl From<CanonicalCodecError> for SchedulerReplayError {
    fn from(value: CanonicalCodecError) -> Self {
        Self::Codec(value.to_string())
    }
}

/// TRACE_MATRIX FC1-N7 + FC1-N13: build an observe-only scheduler decision
/// witness from replayable CAS-backed inputs. The event is not a transaction
/// and does not participate in sequencer admission.
pub fn build_scheduler_decision_event(
    cas: &mut CasStore,
    decision_id: String,
    input_tape_head: String,
    price_projection_head: Option<String>,
    scoped_agent_view_head: Option<String>,
    policy_name: String,
    candidate_set: SchedulerCandidateSet,
    selected_agent_or_task: String,
    reason: DecisionReason,
    logical_t: u64,
) -> Result<SchedulerDecisionEvent, SchedulerPolicyError> {
    if candidate_set.input_tape_head != input_tape_head {
        return Err(SchedulerPolicyError::CandidateSetHeadMismatch {
            event_head: input_tape_head,
            candidate_set_head: candidate_set.input_tape_head,
        });
    }
    if !candidate_set
        .candidates
        .iter()
        .any(|candidate| candidate.id == selected_agent_or_task)
    {
        return Err(SchedulerPolicyError::SelectedCandidateMissing(
            selected_agent_or_task,
        ));
    }

    let candidate_set_hash = canonical_scheduler_hash(&candidate_set)?;
    let candidate_set_cid = put_scheduler_canonical_object(
        cas,
        &candidate_set,
        "scheduler-candidate-set",
        logical_t,
        A11_SCHEDULER_CANDIDATE_SET_SCHEMA_ID,
    )?;

    let bundle = SchedulerPolicyInputBundle {
        policy_name: policy_name.clone(),
        policy_version: 1,
        input_tape_head: input_tape_head.clone(),
        price_projection_head: price_projection_head.clone(),
        scoped_agent_view_head: scoped_agent_view_head.clone(),
        candidate_set_hash,
    };
    let policy_input_bundle_hash = canonical_scheduler_hash(&bundle)?;
    put_scheduler_canonical_object(
        cas,
        &bundle,
        "scheduler-policy-input-bundle",
        logical_t,
        A11_SCHEDULER_POLICY_INPUT_SCHEMA_ID,
    )?;

    let view = SchedulerView {
        input_tape_head: input_tape_head.clone(),
        candidate_set_cid,
        candidate_set_hash,
        policy_input_bundle_hash,
        policy_name: policy_name.clone(),
        selected_agent_or_task: selected_agent_or_task.clone(),
    };
    let scheduler_view_cid = put_scheduler_canonical_object(
        cas,
        &view,
        "scheduler-view",
        logical_t,
        A11_SCHEDULER_VIEW_SCHEMA_ID,
    )?;

    let event = SchedulerDecisionEvent {
        decision_id,
        input_tape_head,
        price_projection_head,
        scheduler_view_cid,
        candidate_set_cid,
        candidate_set_hash,
        policy_input_bundle_hash,
        scoped_agent_view_head,
        policy_name,
        selected_agent_or_task,
        random_seed_or_deterministic_reason: reason,
    };

    put_scheduler_canonical_object(
        cas,
        &event,
        "scheduler-decision-event",
        logical_t,
        A11_SCHEDULER_DECISION_EVENT_SCHEMA_ID,
    )?;

    Ok(event)
}

/// TRACE_MATRIX FC1-N13: reconstruct the candidate set from CAS and verify its hash.
pub fn reconstruct_candidate_set_from_event(
    cas: &CasStore,
    event: &SchedulerDecisionEvent,
) -> Result<SchedulerCandidateSet, SchedulerReplayError> {
    let bytes = match cas.get(&event.candidate_set_cid) {
        Ok(bytes) => bytes,
        Err(CasError::CidNotFound(_)) => {
            return Err(SchedulerReplayError::CandidateSetCidMissing(
                event.candidate_set_cid,
            ));
        }
        Err(e) => return Err(SchedulerReplayError::Cas(e)),
    };

    let computed = hash_bytes(&bytes);
    if computed != event.candidate_set_hash {
        return Err(SchedulerReplayError::CandidateSetHashMismatch {
            expected: event.candidate_set_hash,
            computed,
        });
    }

    canonical_decode::<SchedulerCandidateSet>(&bytes).map_err(SchedulerReplayError::from)
}

/// TRACE_MATRIX FC1-N13: verify an input bundle against the decision hash.
pub fn verify_policy_input_bundle(
    event: &SchedulerDecisionEvent,
    bundle: &SchedulerPolicyInputBundle,
) -> Result<bool, SchedulerPolicyError> {
    Ok(canonical_scheduler_hash(bundle)? == event.policy_input_bundle_hash)
}

/// TRACE_MATRIX FC1-N13: canonical hash helper for scheduler receipts.
pub fn canonical_scheduler_hash<T: Serialize>(value: &T) -> Result<Hash, SchedulerPolicyError> {
    hash_canonical(value).map_err(SchedulerPolicyError::from)
}

/// TRACE_MATRIX FC1-N7: stable candidate ordering helper for policy tests.
pub fn stable_candidate_ids(candidate_set: &SchedulerCandidateSet) -> Vec<String> {
    let mut ids: Vec<String> = candidate_set
        .candidates
        .iter()
        .map(|candidate| candidate.id.clone())
        .collect();
    ids.sort();
    ids
}

/// TRACE_MATRIX FC1-N13: write scheduler read-model receipts to CAS only.
pub fn put_scheduler_canonical_object<T: Serialize>(
    cas: &mut CasStore,
    value: &T,
    creator: &str,
    logical_t: u64,
    schema_id: &str,
) -> Result<Cid, SchedulerPolicyError> {
    let bytes = canonical_encode(value)?;
    cas.put(
        &bytes,
        ObjectType::Generic,
        creator,
        logical_t,
        Some(schema_id.to_string()),
    )
    .map_err(SchedulerPolicyError::from)
}

fn hash_canonical<T: Serialize>(value: &T) -> Result<Hash, CanonicalCodecError> {
    let bytes = canonical_encode(value)?;
    Ok(hash_bytes(&bytes))
}

fn hash_bytes(bytes: &[u8]) -> Hash {
    let mut h = Sha256::new();
    h.update(bytes);
    Hash(h.finalize().into())
}

fn hash_hex(hash: &Hash) -> String {
    let mut out = String::with_capacity(64);
    for byte in hash.0 {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

/// TRACE_MATRIX FC1-N7: bounded integer-only softmax configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SoftmaxConfig {
    pub temperature_milli: u64,
}

/// TRACE_MATRIX FC1-N7: fail-closed softmax policy errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SoftmaxError {
    NoCandidates,
    InvalidTemperature,
    InvalidPrice(String),
}

impl fmt::Display for SoftmaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoCandidates => write!(f, "no_candidates"),
            Self::InvalidTemperature => write!(f, "invalid_temperature"),
            Self::InvalidPrice(id) => write!(f, "invalid_price: {id}"),
        }
    }
}

impl std::error::Error for SoftmaxError {}

/// TRACE_MATRIX FC1-N7: deterministic seeded distributional selection using
/// integer-only weights. Price is an optional signal, never predicate truth.
pub fn softmax_select_candidate(
    candidates: &[SchedulerCandidate],
    config: SoftmaxConfig,
    seed: u64,
) -> Result<SchedulerCandidate, SoftmaxError> {
    if candidates.is_empty() {
        return Err(SoftmaxError::NoCandidates);
    }
    if config.temperature_milli == 0 {
        return Err(SoftmaxError::InvalidTemperature);
    }

    let mut weights = Vec::with_capacity(candidates.len());
    let mut total = 0u128;
    for candidate in candidates {
        let weight = softmax_weight(candidate, config)?;
        total = total.saturating_add(weight);
        weights.push(weight);
    }

    let draw = splitmix64(seed) as u128 % total.max(1);
    let mut acc = 0u128;
    for (candidate, weight) in candidates.iter().zip(weights) {
        acc = acc.saturating_add(weight);
        if draw < acc {
            return Ok(candidate.clone());
        }
    }

    Ok(candidates[candidates.len() - 1].clone())
}

/// TRACE_MATRIX FC1-N7: positive-control policy that should collapse distribution.
pub fn argmax_candidate_for_positive_control(
    candidates: &[SchedulerCandidate],
) -> Result<SchedulerCandidate, SoftmaxError> {
    if candidates.is_empty() {
        return Err(SoftmaxError::NoCandidates);
    }

    let mut best = candidates[0].clone();
    let mut best_weight = softmax_weight(
        &best,
        SoftmaxConfig {
            temperature_milli: 1_000,
        },
    )?;
    for candidate in &candidates[1..] {
        let weight = softmax_weight(
            candidate,
            SoftmaxConfig {
                temperature_milli: 1_000,
            },
        )?;
        if weight > best_weight {
            best = candidate.clone();
            best_weight = weight;
        }
    }
    Ok(best)
}

fn softmax_weight(
    candidate: &SchedulerCandidate,
    config: SoftmaxConfig,
) -> Result<u128, SoftmaxError> {
    let Some(price) = candidate.price else {
        return Ok(1_000_000);
    };
    if price.denominator == 0 {
        return Err(SoftmaxError::InvalidPrice(candidate.id.clone()));
    }

    let price_milli = price.numerator.saturating_mul(1_000) / price.denominator;
    let x_milli = price_milli.saturating_mul(1_000) / config.temperature_milli as u128;

    Ok(1_000_000u128
        .saturating_add(x_milli.saturating_mul(1_000))
        .saturating_add(x_milli.saturating_mul(x_milli) / 2)
        .saturating_add(x_milli.saturating_mul(x_milli).saturating_mul(x_milli) / 6_000))
}

fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

/// TRACE_MATRIX FC1-N7 + FC1-N13: private lane input before public shielding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParallelLaneInput {
    pub lane_id: String,
    pub public_tape_head: String,
    pub public_candidate_set_cid: Cid,
    pub private_error_context: Option<String>,
}

/// TRACE_MATRIX FC1-N13: public-only parallel lane view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParallelLaneView {
    pub lane_id: String,
    pub public_tape_head: String,
    pub public_candidate_set_cid: Cid,
}

/// TRACE_MATRIX FC1-N13: fail-closed public parallel-lane isolation errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParallelLaneError {
    DuplicateLaneId(String),
    PublicTapeHeadMismatch,
    PublicCandidateSetMismatch,
}

impl fmt::Display for ParallelLaneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateLaneId(id) => write!(f, "duplicate_lane_id: {id}"),
            Self::PublicTapeHeadMismatch => write!(f, "public_tape_head_mismatch"),
            Self::PublicCandidateSetMismatch => write!(f, "public_candidate_set_mismatch"),
        }
    }
}

impl std::error::Error for ParallelLaneError {}

/// TRACE_MATRIX FC1-N13: build public lane views without exposing private context.
pub fn build_parallel_lane_views(
    inputs: &[ParallelLaneInput],
) -> Result<Vec<ParallelLaneView>, ParallelLaneError> {
    let mut seen = BTreeSet::new();
    let first_head = inputs.first().map(|input| input.public_tape_head.as_str());
    let first_candidate_set = inputs.first().map(|input| input.public_candidate_set_cid);
    let mut views = Vec::with_capacity(inputs.len());

    for input in inputs {
        if !seen.insert(input.lane_id.clone()) {
            return Err(ParallelLaneError::DuplicateLaneId(input.lane_id.clone()));
        }
        if Some(input.public_tape_head.as_str()) != first_head {
            return Err(ParallelLaneError::PublicTapeHeadMismatch);
        }
        if Some(input.public_candidate_set_cid) != first_candidate_set {
            return Err(ParallelLaneError::PublicCandidateSetMismatch);
        }

        views.push(ParallelLaneView {
            lane_id: input.lane_id.clone(),
            public_tape_head: input.public_tape_head.clone(),
            public_candidate_set_cid: input.public_candidate_set_cid,
        });
    }

    Ok(views)
}

/// TRACE_MATRIX FC1-N7: hard loop caps for bounded scheduler/search execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForcedLoopBounds {
    pub max_iterations: u64,
    pub max_tokens: u64,
    pub max_wall_clock_ms: u64,
}

/// TRACE_MATRIX FC1-N7: current loop counters checked against hard caps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForcedLoopState {
    pub iterations: u64,
    pub tokens_used: u64,
    pub wall_clock_ms: u64,
}

/// TRACE_MATRIX FC1-N7: deterministic stop reason for a bounded loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForcedLoopStop {
    MaxIterations,
    MaxTokens,
    MaxWallClock,
}

/// TRACE_MATRIX FC1-N7: fail-closed invalid loop-bound configuration errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForcedLoopBoundsError {
    MaxIterationsMustBePositive,
    MaxTokensMustBePositive,
    MaxWallClockMsMustBePositive,
}

impl fmt::Display for ForcedLoopBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MaxIterationsMustBePositive => write!(f, "max_iterations_must_be_positive"),
            Self::MaxTokensMustBePositive => write!(f, "max_tokens_must_be_positive"),
            Self::MaxWallClockMsMustBePositive => write!(f, "max_wall_clock_ms_must_be_positive"),
        }
    }
}

impl std::error::Error for ForcedLoopBoundsError {}

impl ForcedLoopBounds {
    /// TRACE_MATRIX FC1-N7: reject unbounded zero-valued caps.
    pub fn validate(&self) -> Result<(), ForcedLoopBoundsError> {
        if self.max_iterations == 0 {
            return Err(ForcedLoopBoundsError::MaxIterationsMustBePositive);
        }
        if self.max_tokens == 0 {
            return Err(ForcedLoopBoundsError::MaxTokensMustBePositive);
        }
        if self.max_wall_clock_ms == 0 {
            return Err(ForcedLoopBoundsError::MaxWallClockMsMustBePositive);
        }
        Ok(())
    }
}

/// TRACE_MATRIX FC1-N7: compute whether the forced loop must halt.
pub fn forced_loop_stop_reason(
    bounds: &ForcedLoopBounds,
    state: &ForcedLoopState,
) -> Option<ForcedLoopStop> {
    if state.iterations >= bounds.max_iterations {
        return Some(ForcedLoopStop::MaxIterations);
    }
    if state.tokens_used >= bounds.max_tokens {
        return Some(ForcedLoopStop::MaxTokens);
    }
    if state.wall_clock_ms >= bounds.max_wall_clock_ms {
        return Some(ForcedLoopStop::MaxWallClock);
    }
    None
}

// ── S3 Boltzmann observe-only LIVE-TICK (LIVE-FC1 forward-wiring) ──────────────
//
// `boltzmann_selection_trace::record_boltzmann_selection_over_econ` was
// proven-but-not-production-driven: its only caller lived in the
// `experiments/minif2f_v4/src/bin/lean_market.rs` view-positions command. This
// `src/`-side helper is the production-path LIVE-TICK seam: any runtime/report
// loop that holds a live canonical `&EconomicState` can call it once per tick to
// anchor the observe-only Boltzmann recommendation on CAS AND render a bounded
// report section — without touching the trust-root-pinned `src/bus.rs` (the real
// scheduler, pinned) or the pinned `runtime/mod.rs` (this file is pin-count 0).
//
// HONEST SCOPE: this is an OBSERVE-ONLY recommendation tick, not a binding
// scheduler step. "Price is signal, not truth." It borrows `&EconomicState`,
// mutates no `QState`, advances no head, and changes no sequencer admission or
// L4/L4.E predicate. The CAS write IS the L4 anchor; there is no filesystem
// side-store. The float `boltzmann_softmax_select_parent` is never called —
// integer-rational only.

/// TRACE_MATRIX FC2-N29 + FC1-N7: a production-path LIVE-TICK that records the
/// observe-only Boltzmann selection trace over a LIVE canonical `&EconomicState`
/// and returns the self-addressed CAS `Cid` of the anchored trace. Deterministic:
/// identical `(econ, edges, policy, rng_seed, logical_t)` → identical `Cid`
/// (Art.0.2 replay-stable). The `rng_seed` SHOULD be bound to a tape-derived
/// value (e.g. the replay head / `logical_t`) by the caller so the recommendation
/// is replay-reproducible.
///
/// Observe-only: `econ` is borrowed `&`; nothing is mutated and no head advances.
pub fn tick_boltzmann_selection_over_live_econ(
    cas: &mut CasStore,
    econ: &crate::state::q_state::EconomicState,
    edges: &crate::state::price_index::CanonicalNodeGraph,
    policy: &crate::state::price_index::BoltzmannMaskPolicy,
    rng_seed: u64,
    logical_t: u64,
) -> Result<Cid, CasError> {
    boltzmann_selection_trace::record_boltzmann_selection_over_econ(
        cas, econ, edges, policy, rng_seed, logical_t,
    )
}

/// TRACE_MATRIX FC2-N29 + FC1-N7: render a bounded, shielded report section for a
/// Boltzmann live-tick. Emits ONLY integer counts + ids (no raw Lean stderr /
/// autopsy / float). Reconstructs the just-anchored trace from CAS to prove the
/// section is a derived view of tape-canonical bytes (Art.0.2), never a source of
/// truth. The "observe-only / price is signal, not truth" interpretation is
/// stamped into the section so a reader cannot mistake it for a binding step.
pub fn render_boltzmann_tick_section(cas: &CasStore, trace_cid: &Cid) -> String {
    let mut out = String::new();
    out.push_str("\n## §S3.1 Boltzmann scheduler live-tick (observe-only)\n");
    out.push_str("  interpretation: non-binding recommendation; price is signal, not truth\n");
    out.push_str("  does NOT change sequencer admission or L4/L4.E predicates\n");
    out.push_str(&format!("  trace_cid: {trace_cid}\n"));
    match boltzmann_selection_trace::read_boltzmann_selection_from_cas(cas, trace_cid) {
        Ok(trace) => {
            out.push_str(&format!("  observe_only: {}\n", trace.observe_only));
            out.push_str(&format!(
                "  candidate_nodes: {}\n",
                trace.candidate_nodes.len()
            ));
            out.push_str(&format!("  masked_nodes: {}\n", trace.masked_nodes.len()));
            out.push_str(&format!(
                "  selection_branch: {}\n",
                match trace.selection_branch {
                    boltzmann_selection_trace::SelectionBranch::EmptyCandidates =>
                        "EmptyCandidates",
                    boltzmann_selection_trace::SelectionBranch::EpsilonExploration =>
                        "EpsilonExploration",
                    boltzmann_selection_trace::SelectionBranch::ArgmaxExploit => "ArgmaxExploit",
                }
            ));
            out.push_str(&format!(
                "  selected_parent: {}\n",
                trace
                    .selected_parent
                    .as_ref()
                    .map(|p| p.0.as_str())
                    .unwrap_or("None")
            ));
        }
        Err(e) => {
            out.push_str(&format!("  (trace unreadable from CAS: {e})\n"));
        }
    }
    out
}

#[cfg(test)]
mod boltzmann_tick_tests {
    use super::*;
    use crate::economy::money::MicroCoin;
    use crate::state::price_index::{BoltzmannMaskPolicy, CanonicalNodeGraph};
    use crate::state::q_state::{AgentId, EconomicState, TaskId};
    use crate::state::typed_tx::{NodePosition, PositionKind, PositionSide};
    use tempfile::TempDir;

    fn econ_with_two_nodes() -> EconomicState {
        let mut econ = EconomicState::default();
        for (pid, node, owner, amt) in [
            ("p1", "low", "a1", 300_000_i64),
            ("p2", "high", "a2", 900_000_i64),
        ] {
            let p = NodePosition {
                position_id: TxId(pid.into()),
                node_id: TxId(node.into()),
                task_id: TaskId("t1".into()),
                owner: AgentId(owner.into()),
                side: PositionSide::Long,
                kind: PositionKind::FirstLong,
                amount: MicroCoin::from_micro_units(amt),
                source_tx: TxId(pid.into()),
                opened_at_round: 1,
            };
            econ.node_positions_t.0.insert(p.position_id.clone(), p);
        }
        econ
    }

    /// The live-tick records a CAS-anchored trace over a borrowed econ and the
    /// render section is a derived view of the anchored bytes. Observe-only:
    /// econ is byte-identical before/after.
    #[test]
    fn live_tick_records_and_renders_observe_only() {
        let tmp = TempDir::new().unwrap();
        let mut cas = CasStore::open(tmp.path()).unwrap();
        let econ = econ_with_two_nodes();
        let before = econ.clone();
        let policy = BoltzmannMaskPolicy {
            epsilon_exploration_num: 0,
            epsilon_exploration_den: 1,
            ..BoltzmannMaskPolicy::default()
        };
        let edges = CanonicalNodeGraph::default();
        let cid = tick_boltzmann_selection_over_live_econ(&mut cas, &econ, &edges, &policy, 7, 7)
            .unwrap();
        // Observe-only: borrowing econ mutated nothing.
        assert_eq!(econ, before, "live-tick must not mutate the borrowed econ");
        let section = render_boltzmann_tick_section(&cas, &cid);
        assert!(section.contains("observe_only: true"));
        assert!(section.contains("candidate_nodes: 2"));
        assert!(section.contains("price is signal, not truth"));
        // Shielded + integer-only: the rendered numeric fields are integer counts
        // and ids; no decimal-pointed float ever appears in the section.
        assert!(
            !section.contains("0.") && !section.contains("1."),
            "live-tick section must be integer-only (no float): {section}"
        );
    }

    /// Replay-stable: identical inputs+seed → identical anchored trace Cid.
    #[test]
    fn live_tick_is_replay_stable() {
        let econ = econ_with_two_nodes();
        let policy = BoltzmannMaskPolicy::default();
        let edges = CanonicalNodeGraph::default();
        let run = || {
            let tmp = TempDir::new().unwrap();
            let mut cas = CasStore::open(tmp.path()).unwrap();
            tick_boltzmann_selection_over_live_econ(&mut cas, &econ, &edges, &policy, 99, 3)
                .unwrap()
        };
        assert_eq!(run(), run(), "identical inputs+seed → identical trace Cid");
    }
}
