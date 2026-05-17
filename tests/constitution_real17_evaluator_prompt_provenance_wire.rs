//! REAL-17 evaluator wiring gate for direct submitted MarketDecision provenance.
//!
//! This source-level gate is intentionally narrow: it guards that the live
//! evaluator writes a CAS sidecar from the submitted MarketDecisionTrace CID
//! and the same-turn PromptCapsule CID, without changing typed tx admission.

#[test]
fn evaluator_submitted_invest_branch_writes_market_decision_prompt_provenance_link() {
    let evaluator =
        std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").expect("evaluator");

    assert!(
        evaluator.contains(
            "let market_decision_trace_cid = write_market_decision_trace_to_cas_or_exit("
        ),
        "submitted branch must keep the MarketDecisionTrace CID instead of discarding it"
    );
    assert!(
        evaluator.contains("write_market_decision_provenance_link_to_cas_or_exit("),
        "submitted branch must write a MarketDecisionProvenanceLink sidecar"
    );
    assert!(
        evaluator.contains("real5_prompt_capsule_cid_for_turn"),
        "sidecar must use the same-turn PromptCapsule CID"
    );
    assert!(
        evaluator.contains("MarketDecisionProvenanceLink"),
        "evaluator must construct the typed REAL-17 provenance sidecar"
    );
    assert!(
        evaluator.contains("if let Some(prompt_capsule_cid) = real5_prompt_capsule_cid_for_turn"),
        "submitted direct provenance must be conditional on real PromptCapsule availability"
    );
}
