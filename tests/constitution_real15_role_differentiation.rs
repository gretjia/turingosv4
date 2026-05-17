//! REAL-15 -- role differentiation / E3 candidate gates.

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::market_e2_candidate_verifier::{
    BcastShieldingReport, E2CandidateVerifierReport, E2CandidateVerifierVerdict,
    MatchedTxProvenanceReport,
};
use turingosv4::runtime::real5_roles::{
    write_role_turn_trace_to_cas, AgentRole, RoleTurnOutcome, RoleTurnTrace,
};
use turingosv4::runtime::role_differentiation::{
    summarize_role_differentiation_from_runs, RoleDifferentiationRunInput,
    RoleDifferentiationVerdict,
};
use turingosv4::state::q_state::{AgentId, TaskId};

#[test]
fn role_differentiation_summary_derives_from_cas_and_exact_join_not_dashboard_text() {
    let (cas_a, e2_a) = fixture_run_with_bull_and_solver("run-a", true);
    let (cas_b, e2_b) = fixture_run_with_bull_and_solver("run-b", true);

    let report = summarize_role_differentiation_from_runs(&[
        RoleDifferentiationRunInput::new("run-a", &cas_a, &e2_a, true),
        RoleDifferentiationRunInput::new("run-b", &cas_b, &e2_b, true),
    ])
    .expect("role differentiation summary should derive from CAS + exact verifier");

    assert_eq!(report.verdict, RoleDifferentiationVerdict::Proceed);
    assert_eq!(report.run_count, 2);
    assert_eq!(report.audit_tape_proceed_count, 2);
    assert!(report.source_boundary.contains("ChainTape/CAS"));
    assert!(!report.source_boundary.contains("dashboard"));
    assert_eq!(
        report
            .by_role
            .get("BullTrader")
            .expect("BullTrader zero row exists")
            .exact_join_market_action_count,
        2
    );
    assert_eq!(
        report
            .by_role
            .get("Solver")
            .expect("Solver zero row exists")
            .work_count,
        2
    );
    assert!(
        report.e3_candidate,
        "BullTrader and Solver have persistent distinct action distributions"
    );
}

#[test]
fn role_differentiation_separates_bull_and_bear_side_distributions() {
    let (cas_a, e2_a) = fixture_run_with_bull_bear_and_solver("run-a");
    let (cas_b, e2_b) = fixture_run_with_bull_bear_and_solver("run-b");

    let report = summarize_role_differentiation_from_runs(&[
        RoleDifferentiationRunInput::new("run-a", &cas_a, &e2_a, true),
        RoleDifferentiationRunInput::new("run-b", &cas_b, &e2_b, true),
    ])
    .expect("summary");

    let bull = report.by_role.get("BullTrader").expect("BullTrader row");
    let bear = report.by_role.get("BearTrader").expect("BearTrader row");
    assert_eq!(bull.buy_yes_count, 2);
    assert_eq!(bull.buy_no_count, 0);
    assert_eq!(bear.buy_yes_count, 0);
    assert_eq!(bear.buy_no_count, 2);
    assert_ne!(bull.action_signature, bear.action_signature);
}

#[test]
fn prompt_labels_without_chain_cas_activity_do_not_satisfy_e3() {
    let dir = TempDir::new().expect("tempdir");
    let cas = CasStore::open(dir.path()).expect("cas");
    let e2 = empty_e2_report();

    let report = summarize_role_differentiation_from_runs(&[
        RoleDifferentiationRunInput::new("run-a", &cas, &e2, true),
        RoleDifferentiationRunInput::new("run-b", &cas, &e2, true),
    ])
    .expect("empty evidence should be a clean-negative, not an error");

    assert_eq!(report.verdict, RoleDifferentiationVerdict::CleanNegative);
    assert!(!report.e3_candidate);
    assert!(report
        .failure_reasons
        .iter()
        .any(|reason| reason.contains("fewer than two persistent active roles")));
}

#[test]
fn role_differentiation_excludes_scripted_or_policy_verifier_rows() {
    let (cas_a, mut e2_a) = fixture_run_with_bull_and_solver("run-a", true);
    e2_a.scripted_fixture_tx_count = 1;
    let (cas_b, e2_b) = fixture_run_with_bull_and_solver("run-b", true);

    let report = summarize_role_differentiation_from_runs(&[
        RoleDifferentiationRunInput::new("run-a", &cas_a, &e2_a, true),
        RoleDifferentiationRunInput::new("run-b", &cas_b, &e2_b, true),
    ])
    .expect("report still renders failure reasons");

    assert_eq!(report.verdict, RoleDifferentiationVerdict::Veto);
    assert!(!report.e3_candidate);
    assert!(report
        .failure_reasons
        .iter()
        .any(|reason| reason.contains("scripted_fixture_tx_count")));
}

#[test]
fn role_differentiation_markdown_forbids_achieved_and_ship_claims() {
    let (cas_a, e2_a) = fixture_run_with_bull_bear_and_solver("run-a");
    let (cas_b, e2_b) = fixture_run_with_bull_bear_and_solver("run-b");
    let report = summarize_role_differentiation_from_runs(&[
        RoleDifferentiationRunInput::new("run-a", &cas_a, &e2_a, true),
        RoleDifferentiationRunInput::new("run-b", &cas_b, &e2_b, true),
    ])
    .expect("summary");

    let md = report.render_markdown();
    assert!(md.contains("claim_boundary: E3 candidate pending audit"));
    assert!(md.contains("not E3 achieved"));
    assert!(!md.contains("market emergence proven"));
    assert!(!md.contains("market mechanism shipped"));
}

#[test]
fn real15_role_differentiation_cli_is_source_separated() {
    let cli = include_str!("../src/bin/real15_role_differentiation_verifier.rs");
    assert!(cli.contains("RoleDifferentiationRunInput"));
    assert!(cli.contains("--e2-report"));
    assert!(cli.contains("--cas"));
    assert!(cli.contains("--json-out"));
    assert!(cli.contains("--md-out"));
    assert!(!cli.contains("audit_dashboard_run_report"));
}

fn fixture_run_with_bull_and_solver(
    run_id: &str,
    include_market: bool,
) -> (CasStore, E2CandidateVerifierReport) {
    let dir = TempDir::new().expect("tempdir").keep();
    let mut cas = CasStore::open(&dir).expect("cas");
    write_role_turn(
        &mut cas,
        "Agent_0",
        AgentRole::BullTrader,
        "task-shared",
        RoleTurnOutcome::MarketDecision {
            tx_kind: "BuyWithCoinRouter".into(),
            public_summary: "public market decision".into(),
        },
        0,
    );
    write_role_turn(
        &mut cas,
        "Agent_2",
        AgentRole::Solver,
        "task-shared",
        RoleTurnOutcome::SubmitProof {
            tx_kind: "WorkTx".into(),
        },
        1,
    );
    let e2 = if include_market {
        e2_report(vec![matched("bull-tx", "Agent_0", "BuyYes", "BullTrader")])
    } else {
        empty_e2_report()
    };
    let _ = run_id;
    (cas, e2)
}

fn fixture_run_with_bull_bear_and_solver(run_id: &str) -> (CasStore, E2CandidateVerifierReport) {
    let (mut cas, _) = fixture_run_with_bull_and_solver(run_id, false);
    write_role_turn(
        &mut cas,
        "Agent_1",
        AgentRole::BearTrader,
        "task-shared",
        RoleTurnOutcome::MarketDecision {
            tx_kind: "BuyWithCoinRouter".into(),
            public_summary: "public no-side market decision".into(),
        },
        2,
    );
    let e2 = e2_report(vec![
        matched("bull-tx", "Agent_0", "BuyYes", "BullTrader"),
        matched("bear-tx", "Agent_1", "BuyNo", "BearTrader"),
    ]);
    (cas, e2)
}

fn write_role_turn(
    cas: &mut CasStore,
    agent_id: &str,
    role: AgentRole,
    task_id: &str,
    outcome: RoleTurnOutcome,
    logical_t: u64,
) {
    let trace = RoleTurnTrace::new(
        AgentId(agent_id.into()),
        role,
        TaskId(task_id.into()),
        Cid([logical_t as u8 + 1; 32]),
        Some("action".into()),
        outcome,
    );
    write_role_turn_trace_to_cas(cas, &trace, &format!("{agent_id}-{logical_t}"), logical_t)
        .expect("role turn trace writes");
}

fn e2_report(rows: Vec<MatchedTxProvenanceReport>) -> E2CandidateVerifierReport {
    E2CandidateVerifierReport {
        schema_version: E2CandidateVerifierReport::SCHEMA_VERSION.into(),
        l4_router_tx_count: rows.len() as u64,
        submitted_trace_tx_count: rows.len() as u64,
        exact_join_count: rows.len() as u64,
        matched_tx_ids: rows.iter().map(|row| row.tx_id.clone()).collect(),
        unmatched_l4_router_tx_ids: vec![],
        unmatched_submitted_trace_tx_ids: vec![],
        duplicate_l4_router_tx_id_count: 0,
        duplicate_submitted_trace_tx_id_count: 0,
        scripted_fixture_tx_count: 0,
        policy_counts_for_e2: false,
        direct_prompt_capsule_provenance_count: rows
            .iter()
            .filter(|row| {
                row.prompt_capsule_linkage == "direct_via_market_decision_provenance_link"
            })
            .count() as u64,
        indirect_prompt_capsule_provenance_count: rows
            .iter()
            .filter(|row| row.prompt_capsule_linkage == "indirect_via_ev_decision_trace")
            .count() as u64,
        missing_direct_prompt_capsule_provenance_count: rows
            .iter()
            .filter(|row| {
                row.prompt_capsule_linkage != "direct_via_market_decision_provenance_link"
            })
            .count() as u64,
        matched_tx_provenance: rows,
        bcast_shielding: BcastShieldingReport {
            librarian_digest_count: 1,
            librarian_role_crop_count: 1,
            visible_context_count: 1,
            verdict: "PASS".into(),
            failure_reasons: vec![],
        },
        verdict: E2CandidateVerifierVerdict::Proceed,
        failure_reasons: vec![],
    }
}

fn empty_e2_report() -> E2CandidateVerifierReport {
    e2_report(vec![])
}

fn matched(tx_id: &str, actor: &str, direction: &str, role: &str) -> MatchedTxProvenanceReport {
    MatchedTxProvenanceReport {
        tx_id: tx_id.into(),
        l4_buyer: actor.into(),
        l4_event_id: "event-task-shared".into(),
        l4_direction: direction.into(),
        l4_pay_coin_micro: 1000,
        l4_min_out_shares: 1,
        market_decision_trace_count: 1,
        market_decision_trace_cids: vec!["cid:market".into()],
        market_decision_provenance_link_cids: vec![],
        market_decision_agent_id: Some(actor.into()),
        market_decision_chosen_node_id: Some("node".into()),
        market_decision_direction: Some(direction.into()),
        market_decision_amount_micro: Some(1000),
        ev_decision_trace_count: 1,
        ev_decision_trace_cids: vec!["cid:ev".into()],
        ev_actions: vec![direction.into()],
        ev_reasons: vec!["PositiveEV".into()],
        market_opportunity_trace_count: 1,
        market_opportunity_trace_cids: vec!["cid:opportunity".into()],
        prompt_capsule_linkage: "indirect_via_ev_decision_trace".into(),
        prompt_capsule_cids: vec!["cid:prompt".into()],
        role_turn_trace_count: 1,
        role_turn_trace_cids: vec!["cid:role-turn".into()],
        live_agent_role: Some(role.into()),
        actor_is_policy_trader: false,
        actor_is_live_agent_role: true,
        residual_risks: vec![],
    }
}
