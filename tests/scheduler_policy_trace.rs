use std::path::Path;

use turingosv4::bottom_white::cas::{CasStore, Cid};
use turingosv4::runtime::agent_scheduler::{
    build_scheduler_decision_event, reconstruct_candidate_set_from_event,
    verify_policy_input_bundle, DecisionReason, SchedulerCandidate, SchedulerCandidateSet,
    SchedulerDecisionEvent, SchedulerPolicyInputBundle,
};
use turingosv4::state::q_state::Hash;

fn hash(seed: u8) -> Hash {
    Hash([seed; 32])
}

fn candidate(id: &str) -> SchedulerCandidate {
    SchedulerCandidate {
        id: id.to_string(),
        price: None,
        public_context_cid: None,
    }
}

#[test]
fn scheduler_decision_event_binds_candidate_set_and_policy_input_to_cas() {
    let tmp = tempfile::tempdir().expect("tmp");
    let mut cas = CasStore::open(tmp.path()).expect("open cas");
    let candidate_set = SchedulerCandidateSet {
        input_tape_head: "git-head-a".to_string(),
        candidates: vec![
            candidate("task-a"),
            candidate("task-b"),
            candidate("task-c"),
        ],
    };

    let event = build_scheduler_decision_event(
        &mut cas,
        "scheduler-decision-1".to_string(),
        "git-head-a".to_string(),
        Some("git-head-price-a".to_string()),
        Some("scoped-agent-view-head-a".to_string()),
        "softmax.v1".to_string(),
        candidate_set.clone(),
        "task-b".to_string(),
        DecisionReason::Seeded { seed: 7 },
        17,
    )
    .expect("build decision event");

    assert_eq!(event.input_tape_head, "git-head-a");
    assert_eq!(
        event.price_projection_head.as_deref(),
        Some("git-head-price-a")
    );
    assert_eq!(
        event.scoped_agent_view_head.as_deref(),
        Some("scoped-agent-view-head-a")
    );
    assert_eq!(event.policy_name, "softmax.v1");
    assert_eq!(event.selected_agent_or_task, "task-b");
    assert_ne!(event.scheduler_view_cid, Cid::default());
    assert_ne!(event.candidate_set_cid, Cid::default());
    assert_ne!(event.candidate_set_hash, Hash::ZERO);
    assert_ne!(event.policy_input_bundle_hash, Hash::ZERO);

    let restored =
        reconstruct_candidate_set_from_event(&cas, &event).expect("candidate set reconstructs");
    assert_eq!(
        restored, candidate_set,
        "replay must recover candidates from CAS, not memory-only scheduler state"
    );

    let bundle = SchedulerPolicyInputBundle::from_decision_event(&event);
    assert!(
        verify_policy_input_bundle(&event, &bundle).expect("bundle hash verifies"),
        "policy input bundle hash must bind policy id, heads, and candidate hash"
    );

    let mut memory_only_drift = bundle.clone();
    memory_only_drift.input_tape_head = "memory-only-head".to_string();
    assert!(
        !verify_policy_input_bundle(&event, &memory_only_drift).expect("bundle hash compares"),
        "memory-only policy input drift must not replay as the taped decision"
    );
}

#[test]
fn scheduler_replay_rejects_missing_or_tampered_candidate_set() {
    let tmp = tempfile::tempdir().expect("tmp");
    let mut cas = CasStore::open(tmp.path()).expect("open cas");
    let candidate_set = SchedulerCandidateSet {
        input_tape_head: "git-head-b".to_string(),
        candidates: vec![candidate("task-a"), candidate("task-b")],
    };

    let mut event = build_scheduler_decision_event(
        &mut cas,
        "scheduler-decision-2".to_string(),
        "git-head-b".to_string(),
        None,
        None,
        "round_robin.v1".to_string(),
        candidate_set,
        "task-a".to_string(),
        DecisionReason::Deterministic {
            reason: "single runnable lane".to_string(),
        },
        18,
    )
    .expect("build decision event");

    event.candidate_set_hash = hash(9);
    let err = reconstruct_candidate_set_from_event(&cas, &event).expect_err("hash mismatch");
    assert!(
        err.to_string().contains("candidate_set_hash_mismatch"),
        "tampered candidate hash must fail closed: {err}"
    );

    let missing = SchedulerDecisionEvent {
        candidate_set_cid: Cid::from_content(b"not-written-to-this-cas"),
        candidate_set_hash: hash(1),
        ..event
    };
    let err = reconstruct_candidate_set_from_event(&cas, &missing).expect_err("missing CAS object");
    assert!(
        err.to_string().contains("candidate_set_cid_missing"),
        "memory-only candidate set must not be replayable without CAS: {err}"
    );
}

#[test]
fn scheduler_event_does_not_enter_sequencer_or_typed_tx_authority_surface() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for rel in ["src/state/sequencer.rs", "src/state/typed_tx.rs"] {
        let text = std::fs::read_to_string(root.join(rel)).expect("read restricted source");
        assert!(
            !text.contains("SchedulerDecisionEvent"),
            "{rel} must not gain scheduler authority in A11"
        );
        assert!(
            !text.contains("scheduler::policy"),
            "{rel} must not import scheduler policy code"
        );
    }
}
