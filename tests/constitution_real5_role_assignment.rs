//! REAL-5 Atom 0/1 — charter + AgentRoleAssignment gates.

use tempfile::TempDir;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::genesis_report::GenesisReport;
use turingosv4::runtime::real5_roles::{
    detect_hidden_role_switch, read_role_assignment_manifest_from_cas,
    render_role_assignment_dashboard, role_assignment_from_csv, sorted_role_assignment,
    write_role_assignment_manifest_to_cas, AgentRole, AgentRoleAssignment, RoleAssignmentManifest,
};
use turingosv4::state::q_state::AgentId;

#[test]
fn sg_r5_0_charter_cites_fc_and_forbidden_list() {
    let charter = include_str!(
        "../handover/tracer_bullets/REAL-5_role_based_generative_scaffolding_charter.md"
    );
    assert!(charter.contains("FC1"));
    assert!(charter.contains("FC2"));
    assert!(charter.contains("FC3"));
    for required in [
        "no price-as-truth",
        "no forced trade",
        "no private CoT recording",
        "no raw-log broadcast",
        "no ghost liquidity",
    ] {
        assert!(
            charter.contains(required),
            "missing forbidden item: {required}"
        );
    }
}

#[test]
fn sg_r5_1_role_assignment_persists_replays_and_has_no_hidden_switch() {
    let unsorted = vec![
        role("Agent_2", AgentRole::Trader),
        role("Agent_0", AgentRole::Solver),
        role("Agent_1", AgentRole::Verifier),
    ];
    let sorted = sorted_role_assignment(unsorted);
    assert_eq!(sorted[0].agent_id.0, "Agent_0");
    assert_eq!(sorted[1].agent_id.0, "Agent_1");
    assert_eq!(sorted[2].agent_id.0, "Agent_2");

    let report = GenesisReport {
        constitution_hash: None,
        runtime_repo: "/tmp/runtime".into(),
        cas_path: "/tmp/runtime/cas".into(),
        system_pubkey_hash: None,
        agent_pubkeys_path: "agent_pubkeys.json".into(),
        initial_balances: vec![],
        task_id: None,
        task_open_tx: None,
        escrow_lock_tx: None,
        agent_model_assignment: vec![],
        model_assignment_manifest_cid: None,
        agent_role_assignment: sorted.clone(),
        role_assignment_manifest_cid: None,
    };
    assert_eq!(report.agent_role_assignment.len(), 3);

    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let manifest = RoleAssignmentManifest {
        batch_id: "real5-fixture".into(),
        agent_role_assignment: sorted.clone(),
        source: "test-fixture".into(),
        created_at_head_t: "HEAD0".into(),
    };
    let cid = write_role_assignment_manifest_to_cas(&manifest, &mut cas, "test", 0).unwrap();
    let decoded = read_role_assignment_manifest_from_cas(&cas, &cid).unwrap();
    assert_eq!(decoded, manifest);
    let report_with_cid = GenesisReport {
        role_assignment_manifest_cid: Some(cid.to_string()),
        ..report.clone()
    };
    let json = serde_json::to_string(&report_with_cid).unwrap();
    let decoded_report: GenesisReport = serde_json::from_str(&json).unwrap();
    assert_eq!(
        decoded_report.role_assignment_manifest_cid,
        Some(cid.to_string())
    );
    assert_eq!(decoded_report.agent_role_assignment, sorted);

    detect_hidden_role_switch(&sorted, &[(AgentId("Agent_2".into()), AgentRole::Trader)])
        .expect("matching role is not a hidden switch");
    let err = detect_hidden_role_switch(&sorted, &[(AgentId("Agent_2".into()), AgentRole::Solver)])
        .expect_err("actual role mismatch must be blocked");
    assert!(err.contains("hidden role switch"));

    let dashboard = render_role_assignment_dashboard(&sorted);
    assert!(dashboard.contains("## REAL-5 role assignment"));
    assert!(dashboard.contains("Agent_0"));
    assert!(dashboard.contains("Trader"));
}

#[test]
fn sg_r5_1_role_assignment_env_csv_is_strict_and_deterministic() {
    let agents = vec![
        AgentId("Agent_0".into()),
        AgentId("Agent_1".into()),
        AgentId("Agent_2".into()),
    ];
    let parsed = role_assignment_from_csv(&agents, "Solver,Trader,Verifier")
        .expect("valid REAL-5 assignment csv parses");
    assert_eq!(parsed.len(), 3);
    assert_eq!(parsed[0].agent_id.0, "Agent_0");
    assert_eq!(parsed[0].role, AgentRole::Solver);
    assert_eq!(parsed[1].role, AgentRole::Trader);
    assert!(parsed[1].allowed_tools.contains(&"invest".to_string()));
    assert_eq!(
        parsed[1].view_policy_id, "real5/trader_view/v1",
        "view policy is deterministic and role-scoped"
    );
    assert!(
        parsed[1].role_objective_cid != parsed[2].role_objective_cid,
        "role objective CIDs must be role/agent-specific facts"
    );

    let mismatch = role_assignment_from_csv(&agents, "Solver,Trader")
        .expect_err("partial assignment must fail closed");
    assert!(mismatch.contains("role assignment count mismatch"));
    let unknown = role_assignment_from_csv(&agents, "Solver,Trader,Oracle")
        .expect_err("unknown role must fail closed");
    assert!(unknown.contains("unknown REAL-5 role"));
}

fn role(agent_id: &str, role: AgentRole) -> AgentRoleAssignment {
    AgentRoleAssignment {
        agent_id: AgentId(agent_id.into()),
        role,
        role_objective_cid: Default::default(),
        allowed_tools: vec!["abstain".into()],
        risk_budget_micro: MicroCoin::from_micro_units(100),
        view_policy_id: "policy.real5.v1".into(),
    }
}
