//! REAL-6B — AttemptPredictionMarket sealed-oracle scripted fixture.
//!
//! Architect gates:
//! - SG-6B.1 No sleep-based artificial blocking.
//! - SG-6B.2 K logical tape ticks are deterministic and replayable.
//! - SG-6B.3 Lean oracle remains absolute truth.
//! - SG-6B.4 MarketCloseTx happens before OracleResolveTx.
//! - SG-6B.5 Trader actions during window are ChainTape-visible.
//! - SG-6B.6 Price does not affect verification.
//! - SG-6B.7 No ghost liquidity.

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::real5_roles::AgentRole;

#[path = "support/real6_attempt_prediction_fixture.rs"]
mod real6_attempt_prediction_fixture;

use real6_attempt_prediction_fixture::{
    attempt_prediction_fixture_cids, build_scripted_attempt_prediction_fixture,
    validate_attempt_prediction_fixture, write_attempt_prediction_fixture_to_cas,
    AttemptPredictionStepKind, LeanOracleResult, REAL6B_SCHEMA_ID,
};
use turingosv4::state::q_state::{AgentId, TaskId, TxId};

fn fixture() -> real6_attempt_prediction_fixture::AttemptPredictionFixture {
    build_scripted_attempt_prediction_fixture(
        TaskId("task-real6b".into()),
        Cid::from_content(b"candidate-proof-artifact"),
        TxId("submit-candidate-001".into()),
        vec![
            (AgentId("Agent_trader".into()), AgentRole::Trader),
            (AgentId("Agent_verifier".into()), AgentRole::Verifier),
            (AgentId("Agent_challenger".into()), AgentRole::Challenger),
        ],
        10,
        3,
        MicroCoin::from_micro_units(100_000),
        LeanOracleResult::Verified,
    )
    .expect("fixture builds")
}

#[test]
fn sg_6b_fixture_is_scripted_schema_not_live_llm_ship() {
    let fx = fixture();
    assert_eq!(fx.schema_id, REAL6B_SCHEMA_ID);
    assert!(
        fx.stage_limit.contains("design + scripted fixture only")
            && fx.stage_limit.contains("No live real-LLM ship"),
        "REAL-6B current stage must preserve architect limit"
    );
}

#[test]
fn sg_6b_no_sleep_and_k_logical_ticks_are_deterministic() {
    let a = fixture();
    let b = fixture();
    assert_eq!(a, b, "fixture must replay deterministically from inputs");
    assert_eq!(a.k_logical_ticks, 3);
    assert!(
        a.steps.iter().all(|step| !step.uses_wall_clock_sleep),
        "SG-6B.1: no sleep-based artificial blocking"
    );
    let window_ticks: Vec<u64> = a
        .steps
        .iter()
        .filter(|step| step.is_role_window_tick())
        .map(|step| step.logical_t)
        .collect();
    assert_eq!(window_ticks, vec![11, 12, 13]);
}

#[test]
fn sg_6b_market_close_precedes_oracle_resolve() {
    let fx = fixture();
    validate_attempt_prediction_fixture(&fx).expect("fixture validates");
    let close_t = fx
        .first_logical_t(AttemptPredictionStepKind::MarketClose)
        .expect("MarketClose exists");
    let oracle_t = fx
        .first_logical_t(AttemptPredictionStepKind::OracleResolve)
        .expect("OracleResolve exists");
    assert!(
        close_t < oracle_t,
        "SG-6B.4: MarketCloseTx must happen before OracleResolveTx"
    );
}

#[test]
fn sg_6b_submit_candidate_strictly_precedes_market_open() {
    let err = build_scripted_attempt_prediction_fixture(
        TaskId("task-real6b-zero-open".into()),
        Cid::from_content(b"candidate-proof-artifact-zero-open"),
        TxId("submit-candidate-zero-open".into()),
        vec![
            (AgentId("Agent_trader".into()), AgentRole::Trader),
            (AgentId("Agent_verifier".into()), AgentRole::Verifier),
            (AgentId("Agent_challenger".into()), AgentRole::Challenger),
        ],
        0,
        3,
        MicroCoin::from_micro_units(100_000),
        LeanOracleResult::Verified,
    )
    .expect_err("open_t=0 would collapse SubmitCandidate and MarketOpen logical_t");
    assert!(
        err.contains("SubmitCandidate")
            && err.contains("strictly before AttemptPredictionMarketOpen"),
        "unexpected error: {err}"
    );
}

#[test]
fn sg_6b_role_actions_during_window_are_tape_visible() {
    let fx = fixture();
    let role_steps: Vec<_> = fx
        .steps
        .iter()
        .filter(|step| step.is_role_window_tick())
        .collect();
    assert_eq!(role_steps.len(), 3);
    assert!(
        role_steps.iter().all(|step| step.chain_tape_visible),
        "SG-6B.5: Trader/Verifier/Challenger actions must be ChainTape-visible"
    );
    assert!(
        role_steps
            .iter()
            .any(|step| step.role == Some(AgentRole::Trader)),
        "scripted window must contain Trader action"
    );
}

#[test]
fn sg_6b_oracle_is_absolute_and_price_not_truth() {
    let fx = fixture();
    assert_eq!(fx.lean_oracle_result, LeanOracleResult::Verified);
    assert!(
        !fx.price_affects_verification,
        "SG-6B.6: price must not affect verification"
    );
    assert!(
        fx.steps
            .iter()
            .find(|step| step.kind == AttemptPredictionStepKind::OracleResolve)
            .expect("oracle step")
            .oracle_is_absolute_truth,
        "SG-6B.3: Lean oracle remains absolute truth"
    );
}

#[test]
fn sg_6b_no_ghost_liquidity() {
    let fx = fixture();
    assert_eq!(
        fx.seed_liquidity_micro.micro_units(),
        fx.yes_liquidity_micro.micro_units()
    );
    assert_eq!(
        fx.seed_liquidity_micro.micro_units(),
        fx.no_liquidity_micro.micro_units()
    );
    assert!(
        fx.steps
            .iter()
            .filter(|step| step.kind == AttemptPredictionStepKind::TraderAction)
            .all(|step| step.reserved_micro.micro_units() <= fx.seed_liquidity_micro.micro_units()),
        "SG-6B.7: scripted market action cannot create ghost liquidity"
    );
}

#[test]
fn sg_6b_scripted_fixture_can_be_chain_backed_cas_evidence() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("cas");
    let fx = fixture();
    let cid = write_attempt_prediction_fixture_to_cas(&mut cas, &fx, "sg-6b", 77)
        .expect("fixture writes to CAS");
    let meta = cas.metadata(&cid).expect("fixture metadata");
    assert_eq!(meta.schema_id.as_deref(), Some(REAL6B_SCHEMA_ID));
    assert_eq!(attempt_prediction_fixture_cids(&cas), vec![cid]);
    let bytes = cas.get(&cid).expect("fixture resolves");
    let decoded: real6_attempt_prediction_fixture::AttemptPredictionFixture =
        serde_json::from_slice(&bytes).expect("json fixture decodes");
    assert_eq!(decoded, fx);
}
