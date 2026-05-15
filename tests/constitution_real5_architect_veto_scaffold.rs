//! REAL-5 Atom 8 — ArchitectAI / VetoAI Scaffold gates.

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::runtime::real5_roles::{
    proposal_activation_status, MetricEstimate, ToolProposal, VetoDecision, VetoReasonClass,
    VetoVerdict,
};
use turingosv4::state::q_state::TxId;

#[test]
fn sg_r5_8_proposal_requires_veto_and_never_mutates_production_directly() {
    let proposal = ToolProposal {
        proposal_id: TxId("proposal-1".into()),
        evidence_capsule_cid: Cid([1; 32]),
        proposed_tool_patch_cid: Cid([2; 32]),
        expected_error_reduction: Some(MetricEstimate {
            metric: "parse_fail_rate".into(),
            numerator_delta: -1,
            denominator: 10,
        }),
    };
    assert_eq!(
        proposal_activation_status(&proposal, None),
        "blocked:no_veto_decision"
    );

    let rejected = VetoDecision {
        proposal_id: proposal.proposal_id.clone(),
        verdict: VetoVerdict::Reject,
        reason_class: VetoReasonClass::UnsafeMutation,
        public_summary: "patch mutates production directly".into(),
    };
    assert_eq!(
        proposal_activation_status(&proposal, Some(&rejected)),
        "evidence:persist_rejected"
    );

    let accepted = VetoDecision {
        proposal_id: proposal.proposal_id.clone(),
        verdict: VetoVerdict::Accept,
        reason_class: VetoReasonClass::CanaryEligible,
        public_summary: "sandbox only".into(),
    };
    assert_eq!(
        proposal_activation_status(&proposal, Some(&accepted)),
        "sandbox:canary_only"
    );
}
