//! A12 universal-machine witness helper.
//!
//! This file stays under `tests/support` in A12 because `src/runtime/mod.rs` is
//! trust-root pinned and A12 has no Section-8 rehash authorization. Integration
//! tests include this helper directly so the witness code is compiled and
//! exercised without changing boot/trust-root authority.

#![allow(dead_code)]

use sha2::{Digest, Sha256};

/// TRACE_MATRIX FC1-N13 + FC2-N22 + FC3-N31: witness categories for A12 tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UniversalWitnessKind {
    CounterMachine,
    BranchAndReject,
    ExternalCallReplay,
    MarketSettlement,
    AgentViewShielding,
    SelfBootstrapProposalOnly,
}

/// TRACE_MATRIX FC1-N13 + FC2-N22: replay must be offline for A12 witnesses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkPolicy {
    Off,
    Provider(String),
}

/// TRACE_MATRIX FC1-N13 + FC2-N22 + FC3-N31: expected witness outcome contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitnessExpectation {
    CounterValue {
        counter: i64,
    },
    BranchAndReject {
        accepted_count: usize,
        rejected_count: usize,
    },
    ExternalCallClosed {
        intent_count: usize,
    },
    MarketSettled,
    AgentViewShielded,
    SelfBootstrapProposalOnly,
}

/// TRACE_MATRIX FC1-N13: terminal result declared by a witness run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitnessResult {
    Passed,
    Failed(String),
}

/// TRACE_MATRIX FC1-N13: accepted L4 versus rejected L4.E witness lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TapeLane {
    Accepted,
    Rejected,
}

/// TRACE_MATRIX FC1-N13: hash-bound transition witness from a tape prefix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionWitness {
    pub transition_id: String,
    pub lane: TapeLane,
    pub payload: String,
    pub payload_hash: String,
    pub predicate_receipt_pass: bool,
}

impl TransitionWitness {
    /// TRACE_MATRIX FC1-N13: construct an accepted transition witness.
    pub fn accepted(id: impl Into<String>, payload: impl Into<String>) -> Self {
        Self::new(id, TapeLane::Accepted, payload, true)
    }

    /// TRACE_MATRIX FC1-N13: construct a rejected transition witness.
    pub fn rejected(id: impl Into<String>, payload: impl Into<String>) -> Self {
        Self::new(id, TapeLane::Rejected, payload, false)
    }

    fn new(
        id: impl Into<String>,
        lane: TapeLane,
        payload: impl Into<String>,
        predicate_receipt_pass: bool,
    ) -> Self {
        let payload = payload.into();
        Self {
            transition_id: id.into(),
            lane,
            payload_hash: hash_payload(&payload),
            payload,
            predicate_receipt_pass,
        }
    }
}

/// TRACE_MATRIX FC1-N13 + FC2-N22: hash-bound CAS object witness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CasObjectWitness {
    pub cid: String,
    pub payload: String,
    pub payload_hash: String,
}

impl CasObjectWitness {
    /// TRACE_MATRIX FC1-N13 + FC2-N22: construct a CAS witness object.
    pub fn new(cid: impl Into<String>, payload: impl Into<String>) -> Self {
        let payload = payload.into();
        Self {
            cid: cid.into(),
            payload_hash: hash_payload(&payload),
            payload,
        }
    }
}

/// TRACE_MATRIX FC1-N13 + FC2-N22: terminal state closing an external intent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalCallTerminal {
    Completed {
        terminal_id: String,
    },
    Abandoned {
        terminal_id: String,
        may_have_spent: bool,
    },
}

/// TRACE_MATRIX FC1-N13 + FC2-N22: external-call intent/terminal replay witness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalCallWitness {
    pub intent_id: String,
    pub terminal: Option<ExternalCallTerminal>,
    pub provider_called_during_replay: bool,
}

/// TRACE_MATRIX FC1-N13: market settlement witness with integer conservation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketSettlementWitness {
    pub predicate_receipt_pass: bool,
    pub before_total_micro: i128,
    pub after_total_micro: i128,
}

/// TRACE_MATRIX FC1-N5 + FC1-N6 + FC1-N13: public agent-view shielding witness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentViewWitness {
    pub public_prompt: String,
    pub private_fragments: Vec<String>,
    pub private_cids: Vec<String>,
}

/// TRACE_MATRIX FC3-N31: FC3 self-bootstrap remains proposal-only in A12.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelfBootstrapWitness {
    pub proposal_cid: String,
    pub runtime_authority_changed: bool,
    pub claims_full_fc3_closure: bool,
}

/// TRACE_MATRIX FC1-N13 + FC2-N22 + FC3-N31: complete A12 witness run packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniversalWitnessRun {
    pub witness_id: String,
    pub witness_kind: UniversalWitnessKind,
    pub input_tape_head: String,
    pub input_cas_root: Option<String>,
    pub fixture_cid: Option<String>,
    pub network_policy: NetworkPolicy,
    pub expected_outcome: WitnessExpectation,
    pub transitions: Vec<TransitionWitness>,
    pub cas_objects: Vec<CasObjectWitness>,
    pub external_calls: Vec<ExternalCallWitness>,
    pub market_settlement: Option<MarketSettlementWitness>,
    pub agent_view: Option<AgentViewWitness>,
    pub self_bootstrap: Option<SelfBootstrapWitness>,
    pub result: WitnessResult,
}

impl UniversalWitnessRun {
    /// TRACE_MATRIX FC1-N13 + FC2-N22 + FC3-N31: create an offline witness run.
    pub fn new(
        witness_id: impl Into<String>,
        witness_kind: UniversalWitnessKind,
        input_tape_head: impl Into<String>,
        expected_outcome: WitnessExpectation,
    ) -> Self {
        Self {
            witness_id: witness_id.into(),
            witness_kind,
            input_tape_head: input_tape_head.into(),
            input_cas_root: None,
            fixture_cid: None,
            network_policy: NetworkPolicy::Off,
            expected_outcome,
            transitions: Vec::new(),
            cas_objects: Vec::new(),
            external_calls: Vec::new(),
            market_settlement: None,
            agent_view: None,
            self_bootstrap: None,
            result: WitnessResult::Passed,
        }
    }
}

/// TRACE_MATRIX FC1-N13 + FC2-N22 + FC3-N31: reconstructed facts from witness verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitnessVerification {
    pub accepted_transitions: usize,
    pub rejected_transitions: usize,
    pub counter_value: Option<i64>,
    pub cas_objects_checked: usize,
    pub external_intents_closed: usize,
    pub network_used: bool,
    pub market_settlement_checked: bool,
    pub agent_view_checked: bool,
    pub self_bootstrap_proposal_only: bool,
}

/// TRACE_MATRIX FC1-N13 + FC2-N22 + FC3-N31: fail-closed A12 witness verifier errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitnessError {
    MissingTapeHead,
    NetworkPolicyNotOff,
    WitnessMarkedFailed(String),
    HashMismatch {
        object_id: String,
        expected: String,
        computed: String,
    },
    InvalidCounterPayload(String),
    ExpectationMismatch(String),
    NetworkUsedDuringReplay(String),
    ExternalIntentOpen(String),
    MissingMarketSettlement,
    PredicateReceiptNotPass,
    MoneyNotConserved {
        before_total_micro: i128,
        after_total_micro: i128,
    },
    MissingAgentView,
    PrivateViewLeak(String),
    MissingSelfBootstrap,
    MissingSelfBootstrapProposal,
    SelfBootstrapChangedRuntimeAuthority,
    SelfBootstrapClaimsFullFc3Closure,
}

/// TRACE_MATRIX FC1-N13 + FC2-N22 + FC3-N31: verify a witness using only its tape/CAS packet.
pub fn verify_universal_witness(
    run: &UniversalWitnessRun,
) -> Result<WitnessVerification, WitnessError> {
    if run.input_tape_head.trim().is_empty() {
        return Err(WitnessError::MissingTapeHead);
    }
    if run.network_policy != NetworkPolicy::Off {
        return Err(WitnessError::NetworkPolicyNotOff);
    }
    if let WitnessResult::Failed(reason) = &run.result {
        return Err(WitnessError::WitnessMarkedFailed(reason.clone()));
    }

    let mut accepted_transitions = 0usize;
    let mut rejected_transitions = 0usize;
    let mut counter_value = 0i64;
    for transition in &run.transitions {
        verify_payload_hash(
            &transition.transition_id,
            &transition.payload,
            &transition.payload_hash,
        )?;
        match transition.lane {
            TapeLane::Accepted => {
                accepted_transitions += 1;
                if let Some(delta) = parse_counter_delta(&transition.payload)? {
                    counter_value += delta;
                }
            }
            TapeLane::Rejected => rejected_transitions += 1,
        }
    }

    for object in &run.cas_objects {
        verify_payload_hash(&object.cid, &object.payload, &object.payload_hash)?;
    }

    let external_intents_closed = verify_external_calls(&run.external_calls)?;
    let market_settlement_checked = verify_market(run.market_settlement.as_ref())?;
    let agent_view_checked = verify_agent_view(run.agent_view.as_ref())?;
    let self_bootstrap_proposal_only = verify_self_bootstrap(run.self_bootstrap.as_ref())?;

    let verified = WitnessVerification {
        accepted_transitions,
        rejected_transitions,
        counter_value: (run.witness_kind == UniversalWitnessKind::CounterMachine)
            .then_some(counter_value),
        cas_objects_checked: run.cas_objects.len(),
        external_intents_closed,
        network_used: false,
        market_settlement_checked,
        agent_view_checked,
        self_bootstrap_proposal_only,
    };

    verify_expectation(run, &verified)?;
    Ok(verified)
}

fn verify_expectation(
    run: &UniversalWitnessRun,
    verified: &WitnessVerification,
) -> Result<(), WitnessError> {
    match &run.expected_outcome {
        WitnessExpectation::CounterValue { counter } => {
            if verified.counter_value != Some(*counter) {
                return Err(WitnessError::ExpectationMismatch(format!(
                    "expected counter {counter}, got {:?}",
                    verified.counter_value
                )));
            }
        }
        WitnessExpectation::BranchAndReject {
            accepted_count,
            rejected_count,
        } => {
            if verified.rejected_transitions != *rejected_count {
                return Err(WitnessError::ExpectationMismatch(format!(
                    "expected {rejected_count} rejected transitions, got {}",
                    verified.rejected_transitions
                )));
            }
            if verified.accepted_transitions != *accepted_count {
                return Err(WitnessError::ExpectationMismatch(format!(
                    "expected {accepted_count} accepted transitions, got {}",
                    verified.accepted_transitions
                )));
            }
        }
        WitnessExpectation::ExternalCallClosed { intent_count } => {
            if verified.external_intents_closed != *intent_count {
                return Err(WitnessError::ExpectationMismatch(format!(
                    "expected {intent_count} closed external intents, got {}",
                    verified.external_intents_closed
                )));
            }
        }
        WitnessExpectation::MarketSettled => {
            if !verified.market_settlement_checked {
                return Err(WitnessError::MissingMarketSettlement);
            }
        }
        WitnessExpectation::AgentViewShielded => {
            if !verified.agent_view_checked {
                return Err(WitnessError::MissingAgentView);
            }
        }
        WitnessExpectation::SelfBootstrapProposalOnly => {
            if !verified.self_bootstrap_proposal_only {
                return Err(WitnessError::MissingSelfBootstrap);
            }
        }
    }
    Ok(())
}

fn verify_external_calls(calls: &[ExternalCallWitness]) -> Result<usize, WitnessError> {
    let mut closed = 0usize;
    for call in calls {
        if call.provider_called_during_replay {
            return Err(WitnessError::NetworkUsedDuringReplay(
                call.intent_id.clone(),
            ));
        }
        if call.terminal.is_none() {
            return Err(WitnessError::ExternalIntentOpen(call.intent_id.clone()));
        }
        closed += 1;
    }
    Ok(closed)
}

fn verify_market(settlement: Option<&MarketSettlementWitness>) -> Result<bool, WitnessError> {
    let Some(settlement) = settlement else {
        return Ok(false);
    };
    if !settlement.predicate_receipt_pass {
        return Err(WitnessError::PredicateReceiptNotPass);
    }
    if settlement.before_total_micro != settlement.after_total_micro {
        return Err(WitnessError::MoneyNotConserved {
            before_total_micro: settlement.before_total_micro,
            after_total_micro: settlement.after_total_micro,
        });
    }
    Ok(true)
}

fn verify_agent_view(view: Option<&AgentViewWitness>) -> Result<bool, WitnessError> {
    let Some(view) = view else {
        return Ok(false);
    };
    for fragment in view
        .private_fragments
        .iter()
        .chain(view.private_cids.iter())
    {
        if !fragment.is_empty() && view.public_prompt.contains(fragment) {
            return Err(WitnessError::PrivateViewLeak(fragment.clone()));
        }
    }
    Ok(true)
}

fn verify_self_bootstrap(bootstrap: Option<&SelfBootstrapWitness>) -> Result<bool, WitnessError> {
    let Some(bootstrap) = bootstrap else {
        return Ok(false);
    };
    if bootstrap.proposal_cid.trim().is_empty() {
        return Err(WitnessError::MissingSelfBootstrapProposal);
    }
    if bootstrap.runtime_authority_changed {
        return Err(WitnessError::SelfBootstrapChangedRuntimeAuthority);
    }
    if bootstrap.claims_full_fc3_closure {
        return Err(WitnessError::SelfBootstrapClaimsFullFc3Closure);
    }
    Ok(true)
}

fn verify_payload_hash(id: &str, payload: &str, expected: &str) -> Result<(), WitnessError> {
    let computed = hash_payload(payload);
    if computed != expected {
        return Err(WitnessError::HashMismatch {
            object_id: id.to_string(),
            expected: expected.to_string(),
            computed,
        });
    }
    Ok(())
}

fn parse_counter_delta(payload: &str) -> Result<Option<i64>, WitnessError> {
    let Some(delta) = payload.strip_prefix("counter:") else {
        return Ok(None);
    };
    delta
        .parse::<i64>()
        .map(Some)
        .map_err(|_| WitnessError::InvalidCounterPayload(payload.to_string()))
}

fn hash_payload(payload: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(payload.as_bytes());
    let digest = hasher.finalize();
    let mut out = String::with_capacity(64);
    for byte in digest {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}
