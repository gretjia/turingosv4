//! TB-18R R5 — markov failure cluster source-eligibility (FR-18R.6 /
//! SG-18R.6).
//!
//! Asserts that AttemptTelemetry CAS objects with failure outcomes
//! constitute a type-safe input for markov failure-cluster derivation.
//! Full re-wire of `markov_capsule::generate` to read AttemptTelemetry
//! outcome distribution as a cluster source is forward-bound; this test
//! verifies the type-system path exists per R5 preflight §2.4.
//!
//! See `handover/ai-direct/TB-18R_R5_preflight_audit_extension.md` §2.4.

use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::attempt_telemetry::{
    read_attempt_telemetry_from_cas, write_attempt_telemetry_to_cas, AttemptKind, AttemptOutcome,
    AttemptTelemetry,
};
use turingosv4::runtime::proposal_telemetry::TokenCounts;
use turingosv4::state::q_state::{AgentId, Hash, TxId};

fn write_failure_attempt(cas: &mut CasStore, outcome: AttemptOutcome, tag: &str) -> Cid {
    let attempt = AttemptTelemetry::new_root(
        TxId(format!("att-{tag}")),
        "tb18r-r5-markov".into(),
        "task-markov".into(),
        AgentId(format!("agent_{tag}")),
        format!("n0.b{tag}"),
        Hash([0x33; 32]),
        Cid::from_content(tag.as_bytes()),
        AttemptKind::ExternalizedLlmCycle,
        outcome,
        TokenCounts::default(),
        tag.into(),
    );
    let mut cas_w = cas;
    write_attempt_telemetry_to_cas(&mut cas_w, &attempt, "test", 0).expect("write")
}

/// FR-18R.6: AttemptTelemetry outcome enum covers the failure-cluster
/// space: {LeanFail, ParseFail, SorryBlock, LlmErr}. Each is a valid
/// markov cluster source discriminator.
#[test]
fn attempt_outcome_failure_set_covers_cluster_source_discriminators() {
    let dir = TempDir::new().expect("tempdir");
    let cas_path = dir.path().join("cas");
    std::fs::create_dir_all(&cas_path).expect("mkdir");
    let mut cas = CasStore::open(&cas_path).expect("open cas");

    write_failure_attempt(&mut cas, AttemptOutcome::LeanFail, "lf");
    write_failure_attempt(&mut cas, AttemptOutcome::ParseFail, "pf");
    write_failure_attempt(&mut cas, AttemptOutcome::SorryBlock, "sb");
    write_failure_attempt(&mut cas, AttemptOutcome::LlmErr, "le");

    let cids = cas.list_cids_by_object_type(ObjectType::AttemptTelemetry);
    assert_eq!(cids.len(), 4, "4 failure-outcome AttemptTelemetry on chain");

    let mut has_lean_fail = false;
    let mut has_parse_fail = false;
    let mut has_sorry_block = false;
    let mut has_llm_err = false;
    for cid in &cids {
        let att = read_attempt_telemetry_from_cas(&cas, cid).expect("read");
        match att.outcome {
            AttemptOutcome::LeanFail => has_lean_fail = true,
            AttemptOutcome::ParseFail => has_parse_fail = true,
            AttemptOutcome::SorryBlock => has_sorry_block = true,
            AttemptOutcome::LlmErr => has_llm_err = true,
            _ => {}
        }
    }
    assert!(has_lean_fail);
    assert!(has_parse_fail);
    assert!(has_sorry_block);
    assert!(has_llm_err);
}

/// CR-18R.6: tool_dist remains as a materialized-view metric; the
/// AttemptTelemetry CAS path is the SOURCE-OF-TRUTH for failure facts.
/// This test verifies the type-system dichotomy: AttemptTelemetry has
/// the failure outcome enum, NOT tool_dist.
#[test]
fn attempt_telemetry_is_failure_source_of_truth_not_tool_dist() {
    let dir = TempDir::new().expect("tempdir");
    let cas_path = dir.path().join("cas");
    std::fs::create_dir_all(&cas_path).expect("mkdir");
    let mut cas = CasStore::open(&cas_path).expect("open cas");

    write_failure_attempt(&mut cas, AttemptOutcome::LeanFail, "src1");
    write_failure_attempt(&mut cas, AttemptOutcome::SorryBlock, "src2");

    // The AttemptTelemetry CAS path is queryable. tool_dist is NOT
    // present in this view (it's an evaluator-side stdout-tier metric).
    // Markov cluster source = AttemptTelemetry.outcome iteration.
    let cids = cas.list_cids_by_object_type(ObjectType::AttemptTelemetry);
    assert_eq!(cids.len(), 2);
    for cid in &cids {
        let att = read_attempt_telemetry_from_cas(&cas, cid).expect("read");
        // Outcome discriminator drives clustering — type-system witness.
        let _: AttemptOutcome = att.outcome;
    }
}
