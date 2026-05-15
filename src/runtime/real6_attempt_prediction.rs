//! REAL-6B — AttemptPredictionMarket sealed-oracle scripted fixture.
//!
//! Current-stage limit: design + scripted fixture only. This module does not
//! add live LLM scheduling, typed transaction discriminants, sequencer
//! admission, or oracle settlement semantics. It records the deterministic
//! shape that a future Class-4 `SubmitCandidateTx -> MarketCloseTx ->
//! OracleResolveTx` atom must preserve.

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::economy::money::MicroCoin;
use crate::runtime::real5_roles::AgentRole;
use crate::state::q_state::{AgentId, TaskId, TxId};
use crate::state::typed_tx::EventId;

pub const REAL6B_SCHEMA_ID: &str = "real6b.attempt_prediction_fixture.v1";
pub const REAL6B_STAGE_LIMIT: &str =
    "REAL-6B = design + scripted fixture only. No live real-LLM ship until explicit Class-4 ratification.";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeanOracleResult {
    Verified,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttemptPredictionStepKind {
    SubmitCandidate,
    AttemptPredictionMarketOpen,
    TraderAction,
    VerifierAction,
    ChallengerAction,
    MarketClose,
    OracleResolve,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttemptPredictionStep {
    pub logical_t: u64,
    pub kind: AttemptPredictionStepKind,
    pub agent_id: Option<AgentId>,
    pub role: Option<AgentRole>,
    pub chain_tape_visible: bool,
    pub uses_wall_clock_sleep: bool,
    pub observed_price: Option<String>,
    pub reserved_micro: MicroCoin,
    pub oracle_is_absolute_truth: bool,
}

impl AttemptPredictionStep {
    pub const fn is_role_window_tick(&self) -> bool {
        matches!(
            self.kind,
            AttemptPredictionStepKind::TraderAction
                | AttemptPredictionStepKind::VerifierAction
                | AttemptPredictionStepKind::ChallengerAction
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttemptPredictionFixture {
    pub schema_id: String,
    pub stage_limit: String,
    pub task_id: TaskId,
    pub candidate_cid: Cid,
    pub submit_candidate_tx_id: TxId,
    pub event_id: EventId,
    pub opened_at_logical_t: u64,
    pub k_logical_ticks: u64,
    pub seed_liquidity_micro: MicroCoin,
    pub yes_liquidity_micro: MicroCoin,
    pub no_liquidity_micro: MicroCoin,
    pub lean_oracle_result: LeanOracleResult,
    pub price_affects_verification: bool,
    pub live_llm_ship_enabled: bool,
    pub steps: Vec<AttemptPredictionStep>,
}

impl AttemptPredictionFixture {
    pub fn first_logical_t(&self, kind: AttemptPredictionStepKind) -> Option<u64> {
        self.steps
            .iter()
            .find(|step| step.kind == kind)
            .map(|step| step.logical_t)
    }
}

pub fn attempt_prediction_event_id(task_id: &TaskId, candidate_cid: Cid) -> EventId {
    EventId(TaskId(format!(
        "attempt_prediction:{}:{}",
        task_id.0,
        candidate_cid.hex()
    )))
}

pub fn build_scripted_attempt_prediction_fixture(
    task_id: TaskId,
    candidate_cid: Cid,
    submit_candidate_tx_id: TxId,
    role_window: Vec<(AgentId, AgentRole)>,
    opened_at_logical_t: u64,
    k_logical_ticks: u64,
    seed_liquidity_micro: MicroCoin,
    lean_oracle_result: LeanOracleResult,
) -> Result<AttemptPredictionFixture, String> {
    if k_logical_ticks == 0 {
        return Err("REAL-6B K logical ticks must be positive".into());
    }
    if role_window.len() as u64 != k_logical_ticks {
        return Err(format!(
            "REAL-6B scripted fixture requires exactly K role ticks: roles={} K={}",
            role_window.len(),
            k_logical_ticks
        ));
    }
    if !seed_liquidity_micro.is_positive() {
        return Err("REAL-6B seed liquidity must be positive".into());
    }
    if opened_at_logical_t == 0 {
        return Err(
            "REAL-6B SubmitCandidate must be strictly before AttemptPredictionMarketOpen".into(),
        );
    }

    let event_id = attempt_prediction_event_id(&task_id, candidate_cid);
    let mut steps = Vec::with_capacity(role_window.len() + 4);
    steps.push(AttemptPredictionStep {
        logical_t: opened_at_logical_t - 1,
        kind: AttemptPredictionStepKind::SubmitCandidate,
        agent_id: None,
        role: None,
        chain_tape_visible: true,
        uses_wall_clock_sleep: false,
        observed_price: None,
        reserved_micro: MicroCoin::zero(),
        oracle_is_absolute_truth: false,
    });
    steps.push(AttemptPredictionStep {
        logical_t: opened_at_logical_t,
        kind: AttemptPredictionStepKind::AttemptPredictionMarketOpen,
        agent_id: None,
        role: None,
        chain_tape_visible: true,
        uses_wall_clock_sleep: false,
        observed_price: Some("1/2".into()),
        reserved_micro: MicroCoin::zero(),
        oracle_is_absolute_truth: false,
    });

    for (idx, (agent_id, role)) in role_window.into_iter().enumerate() {
        let kind = match role {
            AgentRole::Trader => AttemptPredictionStepKind::TraderAction,
            AgentRole::Verifier => AttemptPredictionStepKind::VerifierAction,
            AgentRole::Challenger => AttemptPredictionStepKind::ChallengerAction,
            other => {
                return Err(format!(
                    "REAL-6B scripted window role must be Trader/Verifier/Challenger, got {}",
                    other.label()
                ));
            }
        };
        let reserved_micro = match role {
            AgentRole::Trader => {
                MicroCoin::from_micro_units(seed_liquidity_micro.micro_units() / 10)
            }
            _ => MicroCoin::zero(),
        };
        steps.push(AttemptPredictionStep {
            logical_t: opened_at_logical_t + 1 + idx as u64,
            kind,
            agent_id: Some(agent_id),
            role: Some(role),
            chain_tape_visible: true,
            uses_wall_clock_sleep: false,
            observed_price: Some("1/2".into()),
            reserved_micro,
            oracle_is_absolute_truth: false,
        });
    }

    let close_t = opened_at_logical_t + k_logical_ticks + 1;
    steps.push(AttemptPredictionStep {
        logical_t: close_t,
        kind: AttemptPredictionStepKind::MarketClose,
        agent_id: None,
        role: None,
        chain_tape_visible: true,
        uses_wall_clock_sleep: false,
        observed_price: Some("1/2".into()),
        reserved_micro: MicroCoin::zero(),
        oracle_is_absolute_truth: false,
    });
    steps.push(AttemptPredictionStep {
        logical_t: close_t + 1,
        kind: AttemptPredictionStepKind::OracleResolve,
        agent_id: None,
        role: None,
        chain_tape_visible: true,
        uses_wall_clock_sleep: false,
        observed_price: None,
        reserved_micro: MicroCoin::zero(),
        oracle_is_absolute_truth: true,
    });

    let fixture = AttemptPredictionFixture {
        schema_id: REAL6B_SCHEMA_ID.into(),
        stage_limit: REAL6B_STAGE_LIMIT.into(),
        task_id,
        candidate_cid,
        submit_candidate_tx_id,
        event_id,
        opened_at_logical_t,
        k_logical_ticks,
        seed_liquidity_micro,
        yes_liquidity_micro: seed_liquidity_micro,
        no_liquidity_micro: seed_liquidity_micro,
        lean_oracle_result,
        price_affects_verification: false,
        live_llm_ship_enabled: false,
        steps,
    };
    validate_attempt_prediction_fixture(&fixture)?;
    Ok(fixture)
}

pub fn validate_attempt_prediction_fixture(
    fixture: &AttemptPredictionFixture,
) -> Result<(), String> {
    if fixture.schema_id != REAL6B_SCHEMA_ID {
        return Err("REAL-6B fixture schema mismatch".into());
    }
    if !fixture
        .stage_limit
        .contains("design + scripted fixture only")
        || !fixture.stage_limit.contains("No live real-LLM ship")
        || fixture.live_llm_ship_enabled
    {
        return Err("REAL-6B fixture violates current-stage scripted-only limit".into());
    }
    if fixture.steps.iter().any(|step| step.uses_wall_clock_sleep) {
        return Err("SG-6B.1 violation: wall-clock sleep in scripted fixture".into());
    }

    let open_t = fixture
        .first_logical_t(AttemptPredictionStepKind::AttemptPredictionMarketOpen)
        .ok_or_else(|| "missing AttemptPredictionMarket open step".to_string())?;
    let submit_t = fixture
        .first_logical_t(AttemptPredictionStepKind::SubmitCandidate)
        .ok_or_else(|| "missing SubmitCandidate step".to_string())?;
    let close_t = fixture
        .first_logical_t(AttemptPredictionStepKind::MarketClose)
        .ok_or_else(|| "missing MarketClose step".to_string())?;
    let oracle_t = fixture
        .first_logical_t(AttemptPredictionStepKind::OracleResolve)
        .ok_or_else(|| "missing OracleResolve step".to_string())?;
    if submit_t >= open_t {
        return Err(
            "SG-6B sealed-oracle order violation: SubmitCandidate must be strictly before AttemptPredictionMarketOpen"
                .into(),
        );
    }
    if close_t >= oracle_t {
        return Err("SG-6B.4 violation: MarketClose must precede OracleResolve".into());
    }
    if close_t != open_t + fixture.k_logical_ticks + 1 {
        return Err("SG-6B.2 violation: MarketClose not derived from K logical ticks".into());
    }

    let role_steps: Vec<&AttemptPredictionStep> = fixture
        .steps
        .iter()
        .filter(|step| step.is_role_window_tick())
        .collect();
    if role_steps.len() as u64 != fixture.k_logical_ticks {
        return Err("SG-6B.2 violation: role-window tick count != K".into());
    }
    for (idx, step) in role_steps.iter().enumerate() {
        if step.logical_t != open_t + 1 + idx as u64 {
            return Err("SG-6B.2 violation: non-contiguous logical tick window".into());
        }
        if !step.chain_tape_visible {
            return Err("SG-6B.5 violation: role action not ChainTape-visible".into());
        }
    }

    if fixture.price_affects_verification {
        return Err("SG-6B.6 violation: price affects verification".into());
    }
    let oracle = fixture
        .steps
        .iter()
        .find(|step| step.kind == AttemptPredictionStepKind::OracleResolve)
        .ok_or_else(|| "missing oracle step".to_string())?;
    if !oracle.oracle_is_absolute_truth {
        return Err("SG-6B.3 violation: Lean oracle not absolute".into());
    }

    if fixture.seed_liquidity_micro != fixture.yes_liquidity_micro
        || fixture.seed_liquidity_micro != fixture.no_liquidity_micro
        || !fixture.seed_liquidity_micro.is_positive()
    {
        return Err("SG-6B.7 violation: unbalanced or empty liquidity".into());
    }
    if fixture
        .steps
        .iter()
        .any(|step| step.reserved_micro.micro_units() > fixture.seed_liquidity_micro.micro_units())
    {
        return Err("SG-6B.7 violation: reserved amount exceeds seeded liquidity".into());
    }

    Ok(())
}

pub fn write_attempt_prediction_fixture_to_cas(
    cas: &mut CasStore,
    fixture: &AttemptPredictionFixture,
    suffix: &str,
    logical_t: u64,
) -> Result<Cid, CasError> {
    validate_attempt_prediction_fixture(fixture)
        .map_err(|e| CasError::BackendCorruption(format!("attempt prediction fixture: {e}")))?;
    let bytes = serde_json::to_vec(fixture).map_err(|e| {
        CasError::BackendCorruption(format!("attempt prediction fixture encode: {e}"))
    })?;
    cas.put(
        &bytes,
        ObjectType::Generic,
        &format!("real6b-attempt-prediction-fixture-{suffix}"),
        logical_t,
        Some(REAL6B_SCHEMA_ID.to_string()),
    )
}

pub fn attempt_prediction_fixture_cids(cas: &CasStore) -> Vec<Cid> {
    cas.list_all_cids()
        .into_iter()
        .filter(|cid| {
            cas.metadata(cid).and_then(|meta| meta.schema_id.as_deref()) == Some(REAL6B_SCHEMA_ID)
        })
        .collect()
}
