//! TB-18R R5 — final composite proof references attempt_chain_root
//! (FR-18R.8 / SG-18R.8).
//!
//! Asserts the R1 schema invariant `AttemptTelemetry.attempt_chain_root:
//! Option<Hash>` round-trips through canonical encoding/CAS storage.
//! Per R3 §3.5 amended: omega-path WorkTx.proposal_cid stays as
//! ProposalTelemetry CID — actual attempt_chain_root population is
//! forward-binding when omega cutover lifts the TB-7 audit chain
//! constraint.
//!
//! See `handover/ai-direct/TB-18R_R5_preflight_audit_extension.md` §2.3.

use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::attempt_telemetry::{
    read_attempt_telemetry_from_cas, write_attempt_telemetry_to_cas, AttemptKind, AttemptOutcome,
    AttemptTelemetry,
};
use turingosv4::runtime::proposal_telemetry::TokenCounts;
use turingosv4::state::q_state::{AgentId, Hash, TxId};

/// SG-18R.8: AttemptTelemetry::new_terminal_composite carries
/// `attempt_chain_root: Some(merkle_root)`; canonical encode/decode
/// round-trips byte-identical.
#[test]
fn terminal_composite_attempt_chain_root_round_trips() {
    let dir = TempDir::new().expect("tempdir");
    let cas_path = dir.path().join("cas");
    std::fs::create_dir_all(&cas_path).expect("mkdir");
    let mut cas = CasStore::open(&cas_path).expect("open cas");

    let merkle_root = Hash([0xab; 32]);
    let candidate_cid = Cid([0xcd; 32]);
    let composite = AttemptTelemetry::new_terminal_composite(
        TxId("att-terminal".into()),
        "tb18r-r5".into(),
        "task-terminal".into(),
        AgentId("agent_0".into()),
        "n0.b0".into(),
        Some(TxId("att-parent".into())),
        3,
        Hash([0x11; 32]),
        candidate_cid,
        TokenCounts::default(),
        "rfl".into(),
        merkle_root,
    );
    assert_eq!(composite.attempt_chain_root, Some(merkle_root));

    let cid = write_attempt_telemetry_to_cas(&mut cas, &composite, "test", 0).expect("write");
    let decoded = read_attempt_telemetry_from_cas(&cas, &cid).expect("read");
    assert_eq!(
        decoded.attempt_chain_root,
        Some(merkle_root),
        "attempt_chain_root must round-trip byte-identical"
    );
    assert_eq!(decoded.outcome, AttemptOutcome::LeanPass);
    assert_eq!(decoded.attempt_kind, AttemptKind::ExternalizedLlmCycle);
}

/// SG-18R.8: intermediate AttemptTelemetry (non-terminal) has
/// `attempt_chain_root: None`. R3 §3.5 amended: omega-path WorkTx
/// currently uses ProposalTelemetry CID; AttemptTelemetry on chain
/// is intermediate-only.
#[test]
fn intermediate_attempt_chain_root_is_none() {
    let dir = TempDir::new().expect("tempdir");
    let cas_path = dir.path().join("cas");
    std::fs::create_dir_all(&cas_path).expect("mkdir");
    let mut cas = CasStore::open(&cas_path).expect("open cas");

    let candidate_cid = Cid([0xee; 32]);
    let intermediate = AttemptTelemetry::new_root(
        TxId("att-mid".into()),
        "tb18r-r5".into(),
        "task-mid".into(),
        AgentId("agent_0".into()),
        "n0.b0".into(),
        Hash([0x22; 32]),
        candidate_cid,
        AttemptKind::ExternalizedLlmCycle,
        AttemptOutcome::LeanFail,
        TokenCounts::default(),
        "nlinarith".into(),
    );
    assert!(intermediate.attempt_chain_root.is_none());

    let cid = write_attempt_telemetry_to_cas(&mut cas, &intermediate, "test", 0).expect("write");
    let decoded = read_attempt_telemetry_from_cas(&cas, &cid).expect("read");
    assert!(
        decoded.attempt_chain_root.is_none(),
        "intermediate attempt_chain_root must be None"
    );
    let _ = ObjectType::AttemptTelemetry; // type-system witness
}
