//! REAL-5 Atom 9 — role-based smoke gates.

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::market_decision_trace::NoTradeReason;
use turingosv4::runtime::real5_roles::{
    evaluate_real5_smoke, summarize_role_turn_traces_from_cas, write_role_turn_trace_to_cas,
    AgentRole, NoChallengeReason, NoVerifyReason, Real5SmokeInput, RoleTurnOutcome, RoleTurnTrace,
    ROLE_TURN_TRACE_SCHEMA_ID,
};
use turingosv4::state::q_state::{AgentId, TaskId};

#[test]
fn sg_r5_9_role_based_smoke_accepts_clean_negative_without_forced_trade() {
    let report = evaluate_real5_smoke(Real5SmokeInput {
        one_persistent_runtime_repo: true,
        same_cas: true,
        agent_count: 5,
        roles_assigned: vec![
            AgentRole::Solver,
            AgentRole::Trader,
            AgentRole::Verifier,
            AgentRole::Challenger,
            AgentRole::Observer,
        ],
        task_count: 4,
        market_enabled: true,
        role_views_enabled: true,
        forced_trading: false,
        solver_proof_attempts: 2,
        trader_decisions_or_no_trade: 1,
        verifier_verify_or_reason: 1,
        challenger_challenge_or_reason: 1,
        all_actions_reconstruct_from_chain_cas: true,
        price_as_truth: false,
    });
    assert!(report.minimum_green);
    assert!(report.price_signal_not_truth);
    assert!(!report.forced_trade);
    assert!(report.active_roles >= 2);
}

#[test]
fn sg_r5_9_runner_records_role_assignment_and_role_views() {
    let script = include_str!("../scripts/run_g_phase_batch.sh");
    assert!(script.contains("TURINGOS_REAL5_ROLE_ASSIGNMENT"));
    assert!(script.contains("real5_role_assignment_hash"));
    assert!(script.contains("TURINGOS_REAL5_ROLE_VIEWS"));
    assert!(script.contains("real5_role_views_enabled"));
}

#[test]
fn sg_r5_9_evaluator_writes_role_turn_reason_traces() {
    let evaluator = include_str!("../experiments/minif2f_v4/src/bin/evaluator.rs");
    assert!(evaluator.contains("write_role_turn_trace_to_cas"));
    assert!(evaluator.contains("RoleTurnOutcome::NoTrade"));
    assert!(evaluator.contains("RoleTurnOutcome::NoVerify"));
    assert!(evaluator.contains("RoleTurnOutcome::NoChallenge"));
}

#[test]
fn sg_r5_9_role_turn_reason_traces_reconstruct_from_cas() {
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("cas");
    let task_id = TaskId("task-real5-smoke".into());
    let prompt_capsule_cid = Cid([42; 32]);

    let traces = [
        RoleTurnTrace::new(
            AgentId("Agent_1".into()),
            AgentRole::Trader,
            task_id.clone(),
            prompt_capsule_cid,
            Some("append".into()),
            RoleTurnOutcome::NoTrade {
                reason: NoTradeReason::NoPerceivedEdge,
                public_summary: "trader saw role view but emitted no invest action".into(),
            },
        ),
        RoleTurnTrace::new(
            AgentId("Agent_2".into()),
            AgentRole::Verifier,
            task_id.clone(),
            prompt_capsule_cid,
            Some("append".into()),
            RoleTurnOutcome::NoVerify(NoVerifyReason {
                agent_id: AgentId("Agent_2".into()),
                reason: "no verify_peer action emitted".into(),
            }),
        ),
        RoleTurnTrace::new(
            AgentId("Agent_3".into()),
            AgentRole::Challenger,
            task_id,
            prompt_capsule_cid,
            Some("append".into()),
            RoleTurnOutcome::NoChallenge(NoChallengeReason {
                agent_id: AgentId("Agent_3".into()),
                reason: "no challenge action emitted".into(),
            }),
        ),
    ];

    for (idx, trace) in traces.iter().enumerate() {
        let cid =
            write_role_turn_trace_to_cas(&mut cas, trace, &format!("trace-{idx}"), idx as u64)
                .expect("role turn trace writes to CAS");
        let bytes = cas.get(&cid).expect("role turn trace resolves");
        assert!(!String::from_utf8_lossy(&bytes).contains("raw_prompt"));
        assert!(!String::from_utf8_lossy(&bytes).contains("chain_of_thought"));
    }

    let summary = summarize_role_turn_traces_from_cas(&cas);
    assert_eq!(summary.schema_id, ROLE_TURN_TRACE_SCHEMA_ID);
    assert_eq!(summary.total_traces, 3);
    assert_eq!(summary.no_trade_count, 1);
    assert_eq!(summary.no_verify_count, 1);
    assert_eq!(summary.no_challenge_count, 1);
    assert_eq!(summary.by_role.get(&AgentRole::Trader).copied(), Some(1));
    assert_eq!(summary.by_role.get(&AgentRole::Verifier).copied(), Some(1));
    assert_eq!(
        summary.by_role.get(&AgentRole::Challenger).copied(),
        Some(1)
    );
}
