//! TB-G G5 — Role Classifier gates.

use turingosv4::runtime::agent_role_classifier::{
    classify_agent_role, render_role_activity_section, AgentRole, RoleActivity,
};

#[test]
fn sg_g5_4_role_classifier_derives_roles_from_public_activity_counts() {
    assert_eq!(
        classify_agent_role(&RoleActivity {
            work_tx_accepted: 3,
            ..RoleActivity::default()
        }),
        AgentRole::Solver
    );
    assert_eq!(
        classify_agent_role(&RoleActivity {
            verify_tx_accepted: 2,
            ..RoleActivity::default()
        }),
        AgentRole::Verifier
    );
    assert_eq!(
        classify_agent_role(&RoleActivity {
            challenge_tx_accepted: 1,
            ..RoleActivity::default()
        }),
        AgentRole::Challenger
    );
    assert_eq!(
        classify_agent_role(&RoleActivity {
            invest_tx_accepted: 4,
            ..RoleActivity::default()
        }),
        AgentRole::Trader
    );
}

#[test]
fn sg_g5_5_single_role_report_includes_mechanism_bottleneck() {
    let rows = vec![
        ("Agent_0".to_string(), RoleActivity::default()),
        ("Agent_1".to_string(), RoleActivity::default()),
    ];
    let out = render_role_activity_section(&rows);
    assert!(out.contains("## §I Role activity classifier"));
    assert!(out.contains("MECHANISM BOTTLENECK"));
    let causes = out
        .lines()
        .filter(|line| line.trim_start().starts_with("- "))
        .count();
    assert!(
        causes >= 3,
        "single-role clean-negative must list at least 3 candidate causes: {out}"
    );
}
