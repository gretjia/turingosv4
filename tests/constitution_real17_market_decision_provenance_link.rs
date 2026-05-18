//! REAL-17 — MarketDecisionTrace direct PromptCapsule provenance link gates.
//!
//! These gates keep prompt provenance additive and CAS-derived: the sidecar
//! links a submitted MarketDecisionTrace CID to the PromptCapsule CID without
//! changing typed transaction, sequencer, signing, or CAS ObjectType schemas.

use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::market_decision_provenance_link::{
    read_market_decision_provenance_link_from_cas, write_market_decision_provenance_link_to_cas,
    MarketDecisionProvenanceLink, MARKET_DECISION_PROVENANCE_LINK_SCHEMA_ID,
};
use turingosv4::runtime::market_decision_trace::{
    write_market_decision_trace_to_cas, MarketDecisionTrace,
};
use turingosv4::runtime::prompt_capsule::{write_prompt_capsule_v2_to_cas, PromptCapsuleV2};
use turingosv4::runtime::real5_roles::AgentRole;
use turingosv4::state::q_state::{AgentId, Hash, TxId};
use turingosv4::state::typed_tx::BuyDirection;

#[test]
fn market_decision_provenance_link_round_trips_with_generic_schema_metadata() {
    let cas_dir = tempfile::tempdir().expect("cas dir");
    let mut cas = CasStore::open(cas_dir.path()).expect("cas open");
    let visible_context_cid = cas
        .put(
            br#"{"visible":"context"}"#,
            ObjectType::Generic,
            "real17-test",
            1,
            Some("real17-test.visible_context.v1".to_string()),
        )
        .expect("put visible context");
    let prompt_capsule = PromptCapsuleV2 {
        prompt_context_hash: Hash([0x17; 32]),
        agent_id: AgentId("Agent_0".to_string()),
        role: AgentRole::BullTrader,
        view_policy_id: "real17-test-policy".to_string(),
        visible_context_cid,
        read_set: vec![visible_context_cid],
        hidden_fields_redacted: vec!["private_diagnostics".to_string()],
        system_prompt_template_hash: Hash([0x23; 32]),
        model_assignment_cid: None,
    };
    let prompt_capsule_cid =
        write_prompt_capsule_v2_to_cas(&mut cas, &prompt_capsule, "real17-test", 2)
            .expect("put prompt capsule");
    let market_decision = MarketDecisionTrace::submitted(
        AgentId("Agent_0".to_string()),
        TxId("event-real17".to_string()),
        BuyDirection::BuyYes,
        1_000,
        TxId("router-real17".to_string()),
        "submitted by fixture",
    );
    let market_decision_trace_cid =
        write_market_decision_trace_to_cas(&mut cas, &market_decision, "real17-test", 3)
            .expect("put market decision");

    let link = MarketDecisionProvenanceLink {
        schema_version: MarketDecisionProvenanceLink::SCHEMA_VERSION.to_string(),
        market_decision_trace_cid,
        submitted_router_tx_id: TxId("router-real17".to_string()),
        agent_id: AgentId("Agent_0".to_string()),
        prompt_capsule_cid,
        ev_decision_trace_cid: None,
        market_opportunity_trace_cid: None,
        created_at_logical_t: 4,
        public_summary: "direct prompt provenance for submitted market decision".to_string(),
    };
    let link_cid = write_market_decision_provenance_link_to_cas(&mut cas, &link, "real17-test", 4)
        .expect("put provenance link");
    let metadata = cas.metadata(&link_cid).expect("link metadata");

    assert_eq!(metadata.object_type, ObjectType::Generic);
    assert_eq!(
        metadata.schema_id.as_deref(),
        Some(MARKET_DECISION_PROVENANCE_LINK_SCHEMA_ID)
    );
    assert_eq!(
        read_market_decision_provenance_link_from_cas(&cas, &link_cid).expect("read link"),
        link
    );

    let encoded = serde_json::to_string(&link).expect("encode link");
    for forbidden in [
        "raw prompt",
        "raw_prompt",
        "raw completion",
        "raw_completion",
        "private cot",
        "chain of thought",
        "raw log",
        "raw_log",
    ] {
        assert!(
            !encoded.to_ascii_lowercase().contains(forbidden),
            "sidecar must not contain forbidden raw material phrase: {forbidden}"
        );
    }
}

#[test]
fn market_decision_provenance_link_rejects_non_submitted_market_decision_trace() {
    let cas_dir = tempfile::tempdir().expect("cas dir");
    let mut cas = CasStore::open(cas_dir.path()).expect("cas open");
    let visible_context_cid = cas
        .put(
            br#"{"visible":"context"}"#,
            ObjectType::Generic,
            "real17-test",
            1,
            Some("real17-test.visible_context.v1".to_string()),
        )
        .expect("put visible context");
    let prompt_capsule = PromptCapsuleV2 {
        prompt_context_hash: Hash([0x17; 32]),
        agent_id: AgentId("Agent_0".to_string()),
        role: AgentRole::BullTrader,
        view_policy_id: "real17-test-policy".to_string(),
        visible_context_cid,
        read_set: vec![visible_context_cid],
        hidden_fields_redacted: vec!["private_diagnostics".to_string()],
        system_prompt_template_hash: Hash([0x23; 32]),
        model_assignment_cid: None,
    };
    let prompt_capsule_cid =
        write_prompt_capsule_v2_to_cas(&mut cas, &prompt_capsule, "real17-test", 2)
            .expect("put prompt capsule");
    let no_trade = MarketDecisionTrace::no_trade(
        AgentId("Agent_0".to_string()),
        Some(TxId("event-real17".to_string())),
        Some(BuyDirection::BuyYes),
        Some(1_000),
        turingosv4::runtime::market_decision_trace::NoTradeReason::NoPerceivedEdge,
        "no trade",
    );
    let market_decision_trace_cid =
        write_market_decision_trace_to_cas(&mut cas, &no_trade, "real17-test", 3)
            .expect("put no-trade decision");
    let link = MarketDecisionProvenanceLink {
        schema_version: MarketDecisionProvenanceLink::SCHEMA_VERSION.to_string(),
        market_decision_trace_cid,
        submitted_router_tx_id: TxId("router-real17".to_string()),
        agent_id: AgentId("Agent_0".to_string()),
        prompt_capsule_cid,
        ev_decision_trace_cid: None,
        market_opportunity_trace_cid: None,
        created_at_logical_t: 4,
        public_summary: "direct prompt provenance for submitted market decision".to_string(),
    };

    assert!(
        write_market_decision_provenance_link_to_cas(&mut cas, &link, "real17-test", 4).is_err(),
        "direct submitted provenance must fail closed for non-submitted MarketDecisionTrace"
    );
}
