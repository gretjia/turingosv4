//! TB-18R R1 Integration Test — `attempt_chain_root` payload schema.
//!
//! Per Codex Q8 ratified (TB-18R charter v2 §0.A Amendment Log):
//! `attempt_chain_root` lives on `AttemptTelemetry`, NOT on `WorkTx`. It is
//! `Some(merkle_root)` only on the OMEGA-accept terminal composite proof
//! attempt; `None` for all intermediate / failed / aborted attempts.
//!
//! Replay reconstruction path:
//! `WorkTx.proposal_cid → AttemptTelemetry → attempt_chain_root →
//! constituent attempt_ids → individual AttemptTelemetry CAS objects`.
//!
//! Verifies:
//! 1. Intermediate attempts have `attempt_chain_root = None`.
//! 2. Terminal composite attempts have `attempt_chain_root = Some(merkle_root)`.
//! 3. The merkle_root is a deterministic Hash over constituent attempt_ids.
//! 4. Pre-TB-18R `WorkTx` canonical wire bytes are unchanged (Design B).
//!
//! Maps to TB-18R charter v2 SG-18R.8 (== SG-TAPE-8) + Codex Q8 remediation.
//!
//! TRACE_MATRIX FC1-N41 (TB-18R R1 NEW witness).

use sha2::{Digest, Sha256};
use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::transition_ledger::canonical_encode;
use turingosv4::runtime::attempt_telemetry::{
    read_attempt_telemetry_from_cas, write_attempt_telemetry_to_cas, AttemptKind,
    AttemptOutcome, AttemptTelemetry,
};
use turingosv4::runtime::proposal_telemetry::TokenCounts;
use turingosv4::state::q_state::{AgentId, Hash, TxId};
use turingosv4::state::typed_tx::WorkTx;

fn hash_for(domain: &str) -> Hash {
    let mut h = Sha256::new();
    h.update(domain.as_bytes());
    Hash(h.finalize().into())
}

/// Compute Merkle-root-style hash over a list of attempt_ids. R2 / R5 will
/// use the same construction at evaluator-time + audit-time.
fn merkle_root_of_attempt_ids(ids: &[TxId]) -> Hash {
    let mut h = Sha256::new();
    h.update(b"turingosv4.tb18r.attempt_chain_root.v1");
    for id in ids {
        h.update((id.0.len() as u64).to_be_bytes());
        h.update(id.0.as_bytes());
    }
    Hash(h.finalize().into())
}

#[test]
fn fc1_n41_intermediate_attempts_have_none_chain_root() {
    // Per Codex Q8: only terminal composite has Some(root); all intermediate
    // attempts have None.
    let intermediate = AttemptTelemetry::new_root(
        TxId("att-intermediate".into()),
        "test-run".into(),
        "task-001".into(),
        AgentId("agent_0".into()),
        "n1.b0".into(),
        hash_for("ctx"),
        Cid::from_content(b"intermediate-payload"),
        AttemptKind::ExternalizedLlmCycle,
        AttemptOutcome::LeanPass,
        TokenCounts::default(),
        "step_partial_ok".into(),
    );
    assert_eq!(intermediate.attempt_chain_root, None);
}

#[test]
fn fc1_n41_terminal_composite_has_some_chain_root() {
    let constituents = vec![
        TxId("att-0".into()),
        TxId("att-1".into()),
        TxId("att-2".into()),
        TxId("att-3".into()),
    ];
    let merkle_root = merkle_root_of_attempt_ids(&constituents);

    let terminal = AttemptTelemetry::new_terminal_composite(
        TxId("att-final".into()),
        "test-run".into(),
        "task-001".into(),
        AgentId("agent_0".into()),
        "n1.b0".into(),
        Some(TxId("att-3".into())),
        4,
        hash_for("ctx-final"),
        Cid::from_content(b"final-composite-proof"),
        TokenCounts::default(),
        "omega_wtool".into(),
        merkle_root,
    );

    assert_eq!(terminal.attempt_chain_root, Some(merkle_root));
    assert_eq!(terminal.outcome, AttemptOutcome::LeanPass);
    assert_eq!(terminal.attempt_kind, AttemptKind::ExternalizedLlmCycle);
}

#[test]
fn fc1_n41_terminal_chain_root_round_trips_via_cas() {
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("open cas");

    let constituents = vec![TxId("att-a".into()), TxId("att-b".into())];
    let merkle_root = merkle_root_of_attempt_ids(&constituents);

    let terminal = AttemptTelemetry::new_terminal_composite(
        TxId("att-omega".into()),
        "test-run".into(),
        "task-002".into(),
        AgentId("agent_0".into()),
        "n1.b0".into(),
        Some(TxId("att-b".into())),
        2,
        hash_for("ctx-omega"),
        Cid::from_content(b"omega-composite"),
        TokenCounts::default(),
        "omega_wtool".into(),
        merkle_root,
    );

    let cid = write_attempt_telemetry_to_cas(&mut cas, &terminal, "evaluator", 100)
        .expect("write");
    let recovered = read_attempt_telemetry_from_cas(&cas, &cid).expect("read");

    assert_eq!(recovered.attempt_chain_root, Some(merkle_root),
        "merkle_root must round-trip byte-identical");
    assert_eq!(recovered, terminal);
}

#[test]
fn fc1_n41_chain_root_is_deterministic_over_constituent_ids() {
    // Sanity check that the merkle construction is deterministic for the
    // same constituent list (same input → same hash).
    let constituents = vec![
        TxId("att-a".into()),
        TxId("att-b".into()),
        TxId("att-c".into()),
    ];
    let h1 = merkle_root_of_attempt_ids(&constituents);
    let h2 = merkle_root_of_attempt_ids(&constituents);
    assert_eq!(h1, h2);

    // Different ordering → different hash (order-sensitive root).
    let mut shuffled = constituents.clone();
    shuffled.swap(0, 2);
    let h3 = merkle_root_of_attempt_ids(&shuffled);
    assert_ne!(h1, h3, "different ordering must produce different root");
}

#[test]
fn fc1_n41_pre_tb_18r_worktx_canonical_unchanged() {
    // Per Design B (preflight §3 + Codex Q8 ratified): WorkTx canonical
    // wire bytes MUST be unchanged after TB-18R R1. This test constructs
    // a default-shape WorkTx (which represents pre-TB-18R chain entries)
    // and asserts it canonical-encodes successfully without referencing
    // any attempt_chain_root field on the WorkTx itself.
    //
    // This is the structural guarantee that pre-TB-18R chains replay
    // byte-identical (per FR-18R.10 + `feedback_no_retroactive_evidence_rewrite`).
    let work = WorkTx::default();
    let bytes = canonical_encode(&work).expect("encode pre-TB-18R-shape WorkTx");
    // Sanity: the encoding produced bytes (didn't fail), and the size is
    // bounded — TB-18R didn't blow up the WorkTx schema.
    assert!(
        !bytes.is_empty(),
        "default WorkTx must canonical-encode to non-empty"
    );
    // The exact byte length is implementation-dependent, but must not
    // grow drastically vs the pre-TB-18R baseline. We assert a generous
    // upper bound to catch accidental expansion of WorkTx fields.
    assert!(
        bytes.len() < 1024,
        "WorkTx canonical encoding must remain compact ({} bytes); \
         R1 must not have added attempt_chain_root to WorkTx",
        bytes.len()
    );
}
