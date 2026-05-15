//! TB-18R R1 Integration Test — `AttemptTelemetry` canonical-encode +
//! CAS round-trip.
//!
//! Verifies that an `AttemptTelemetry` written via
//! `write_attempt_telemetry_to_cas` is byte-identical when read back via
//! `read_attempt_telemetry_from_cas`. Mirrors the TB-7 `tb_7_proposal_telemetry`
//! integration pattern.
//!
//! Maps to TB-18R charter v2 SG-18R.1 (== SG-TAPE-1) ship gate.
//!
//! TRACE_MATRIX FC1-N41 (TB-18R R1 NEW witness).

use sha2::{Digest, Sha256};
use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::attempt_telemetry::{
    read_attempt_telemetry_from_cas, read_attempt_telemetry_shared_slot_from_cas,
    write_attempt_telemetry_to_cas, AttemptKind, AttemptOutcome, AttemptTelemetry,
    ATTEMPT_TELEMETRY_SCHEMA_VERSION,
};
use turingosv4::runtime::market_decision_trace::{
    write_market_decision_trace_to_cas, MarketDecisionTrace, NoTradeReason,
};
use turingosv4::runtime::proposal_telemetry::TokenCounts;
use turingosv4::state::q_state::{AgentId, Hash, TxId};

fn hash_for(domain: &str) -> Hash {
    let mut h = Sha256::new();
    h.update(domain.as_bytes());
    Hash(h.finalize().into())
}

#[test]
fn fc1_n41_attempt_telemetry_round_trip_via_cas() {
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("open cas");

    let original = AttemptTelemetry::new_root(
        TxId("att-001".into()),
        "test-run-001".into(),
        "minif2f.algebra_001".into(),
        AgentId("agent_0".into()),
        "n1.b0".into(),
        hash_for("prompt-context-001"),
        Cid::from_content(b"parsed candidate: rfl"),
        AttemptKind::ExternalizedLlmCycle,
        AttemptOutcome::LeanPass,
        TokenCounts {
            prompt_tokens: 100,
            completion_tokens: 30,
            tool_tokens: 0,
        },
        "omega_wtool".into(),
    );

    let cid =
        write_attempt_telemetry_to_cas(&mut cas, &original, "evaluator", 42).expect("write to cas");

    let recovered = read_attempt_telemetry_from_cas(&cas, &cid).expect("read from cas");

    assert_eq!(original, recovered, "round-trip must preserve all fields");
    assert_eq!(recovered.schema_version, ATTEMPT_TELEMETRY_SCHEMA_VERSION);
    assert_eq!(recovered.attempt_kind, AttemptKind::ExternalizedLlmCycle);
    assert_eq!(recovered.outcome, AttemptOutcome::LeanPass);
    assert_eq!(
        recovered.attempt_chain_root, None,
        "intermediate / non-terminal attempt must have attempt_chain_root = None"
    );
}

#[test]
fn fc1_n41_attempt_telemetry_failure_path_outcomes_round_trip() {
    // Per Codex Q5 ratified: AttemptOutcome and AttemptKind are separate
    // enums. A failure-path outcome (LeanFail / ParseFail / SorryBlock /
    // LlmErr / Aborted) must round-trip with the same kind
    // (ExternalizedLlmCycle for TB-18R) — no collapsing.
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("open cas");

    for outcome in [
        AttemptOutcome::LeanFail,
        AttemptOutcome::ParseFail,
        AttemptOutcome::SorryBlock,
        AttemptOutcome::LlmErr,
        AttemptOutcome::Aborted,
    ] {
        let mut original = AttemptTelemetry::new_root(
            TxId(format!("att-{:?}", outcome)),
            "test-run-002".into(),
            "minif2f.algebra_002".into(),
            AgentId("agent_1".into()),
            "n1.b0".into(),
            hash_for("prompt-context-002"),
            Cid::from_content(format!("{:?}", outcome).as_bytes()),
            AttemptKind::ExternalizedLlmCycle,
            AttemptOutcome::LeanPass,
            TokenCounts::default(),
            "step".into(),
        );
        original.outcome = outcome;
        let cid =
            write_attempt_telemetry_to_cas(&mut cas, &original, "evaluator", 100).expect("write");
        let recovered = read_attempt_telemetry_from_cas(&cas, &cid).expect("read");
        assert_eq!(recovered.outcome, outcome);
        assert_eq!(recovered.attempt_kind, AttemptKind::ExternalizedLlmCycle);
    }
}

#[test]
fn fc1_n41_idempotent_cid_for_byte_identical_attempts() {
    // CAS is content-addressed: the same canonical-encoded record must
    // produce the same CID across calls. This is the TB-18R analogue of
    // the TB-7 `proposal_cid` byte-stability property.
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("open cas");

    let attempt = AttemptTelemetry::new_root(
        TxId("att-idem".into()),
        "test-run-003".into(),
        "minif2f.algebra_003".into(),
        AgentId("agent_0".into()),
        "n1.b0".into(),
        hash_for("idem-prompt"),
        Cid::from_content(b"idem-candidate"),
        AttemptKind::ExternalizedLlmCycle,
        AttemptOutcome::LeanPass,
        TokenCounts::default(),
        "omega_wtool".into(),
    );

    let cid1 =
        write_attempt_telemetry_to_cas(&mut cas, &attempt, "evaluator", 1).expect("first write");
    let cid2 =
        write_attempt_telemetry_to_cas(&mut cas, &attempt, "evaluator", 2).expect("second write");

    assert_eq!(
        cid1, cid2,
        "byte-identical attempts must produce the same CID regardless of logical_t"
    );
}

#[test]
fn fc1_n41_shared_attempt_slot_classifies_market_trace_json_and_unknown_json() {
    // REAL-6A / REAL-2 hardening: MarketDecisionTrace currently shares the
    // AttemptTelemetry CAS slot as transitional compatibility. Bulk
    // AttemptTelemetry walkers must explicitly classify the known JSON
    // schema and skip it, while unknown JSON fails closed.
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("open cas");

    let trace = MarketDecisionTrace::no_trade(
        AgentId("agent_trader".into()),
        Some(TxId("task-outcome:task-1".into())),
        Some(turingosv4::state::typed_tx::BuyDirection::BuyYes),
        Some(100),
        NoTradeReason::NoPerceivedEdge,
        "saw active TaskOutcomeMarket but no edge",
    );
    let trace_cid = write_market_decision_trace_to_cas(&mut cas, &trace, "shared-slot-test", 7)
        .expect("write trace");

    let classified = read_attempt_telemetry_shared_slot_from_cas(&cas, &trace_cid)
        .expect("known MarketDecisionTrace JSON is classified");
    assert!(
        classified.is_none(),
        "recognized MarketDecisionTrace JSON must not be decoded as AttemptTelemetry"
    );

    let unknown_json_cid = cas
        .put(
            br#"{"schema_version":"unknown.shared.slot.v1","payload":true}"#,
            ObjectType::AttemptTelemetry,
            "unknown-json-test",
            8,
            None,
        )
        .expect("write unknown json");
    let err = read_attempt_telemetry_shared_slot_from_cas(&cas, &unknown_json_cid)
        .expect_err("unknown JSON in AttemptTelemetry slot must fail closed");
    assert!(
        err.to_string().contains("unknown JSON"),
        "error should explain fail-closed unknown JSON, got: {err}"
    );
}
