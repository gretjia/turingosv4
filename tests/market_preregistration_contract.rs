use turingosv4::workloads::market_research::{
    MarketPreregistration, MarketPreregistrationError, MarketTrack,
};

fn prereg(track: MarketTrack) -> MarketPreregistration {
    MarketPreregistration {
        track,
        hypothesis:
            "Market routing improves verifier-backed task pass rate versus equal-budget control."
                .to_string(),
        mde: "absolute +5 verifier-backed TASK-PASS points".to_string(),
        sample_size: 120,
        budget_equalization: "same tx budget, same model budget, same hidden verifier".to_string(),
        ablations: vec![
            "no-price router".to_string(),
            "shuffled-price router".to_string(),
            "single-agent control".to_string(),
        ],
        hidden_verifier_shielding: "hidden verifier and oracle data stay outside AgentView"
            .to_string(),
        route_decision_tape_policy:
            "every route decision writes a SchedulerDecision tape event before execution"
                .to_string(),
        replay_command: "turingos os replay --run-dir <run>".to_string(),
        headline_claim_allowed: false,
        clean_context_audit_required: true,
    }
}

#[test]
fn all_market_tracks_validate_when_preregistered() {
    for track in [
        MarketTrack::A,
        MarketTrack::B,
        MarketTrack::C,
        MarketTrack::D,
    ] {
        prereg(track).validate().expect("track prereg validates");
    }
}

#[test]
fn sample_size_is_required_before_market_claims() {
    let mut p = prereg(MarketTrack::A);
    p.sample_size = 0;
    let err = p.validate().expect_err("sample size must be declared");
    assert_eq!(err, MarketPreregistrationError::ZeroSampleSize);
}

#[test]
fn ablations_are_required_before_market_claims() {
    let mut p = prereg(MarketTrack::B);
    p.ablations.clear();
    let err = p.validate().expect_err("ablations must be declared");
    assert_eq!(err, MarketPreregistrationError::MissingAblations);
}

#[test]
fn route_decision_policy_must_be_tape_visible() {
    let mut p = prereg(MarketTrack::C);
    p.route_decision_tape_policy = "router keeps decisions in memory".to_string();
    let err = p
        .validate()
        .expect_err("route decisions must be tape visible");
    assert_eq!(err, MarketPreregistrationError::RouteDecisionNotTapeVisible);
}

#[test]
fn headline_claim_requires_clean_context_audit() {
    let mut p = prereg(MarketTrack::D);
    p.headline_claim_allowed = true;
    p.clean_context_audit_required = false;
    let err = p
        .validate()
        .expect_err("headline claims require clean context audit");
    assert_eq!(err, MarketPreregistrationError::HeadlineWithoutAudit);
}
