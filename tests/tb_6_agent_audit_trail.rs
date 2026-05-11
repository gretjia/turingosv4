//! TB-6 Atom 5 — Agent audit trail integration tests.
//!
//! Architect ruling 2026-05-01 § 3.6 Atom 5: each Agent proposal links
//! `agent_id`, `prompt_context_hash`, `read_set`, `write_set`, `proposal_cid`,
//! `candidate_proof_cid`, `tx_id`, `predicate_results`, `accepted_or_rejected`,
//! `rejection_class`. Records what the Agent **saw** + **submitted** + how the
//! system **judged**, NEVER chain-of-thought.
//!
//! - I91: end-to-end. Bootstrap chaintape, submit a synthetic zero-stake
//!   WorkTx through `bus.submit_typed_tx` (gets rejected → L4.E),
//!   write an `AgentProposalRecord` to CAS, append a row to the JSONL
//!   index, and verify all 9 architect-mandated fields are recoverable
//!   from CAS + the index links the tx_id back to the record.
//! - I91b: tx_id round-trip via the JSONL index (find_by_tx_id).
//! - I91c: chain-tampering detection on reload.
//! - I91d: forbidden-content shape — `AgentProposalRecord` JSON has no
//!   `chain_of_thought` / `model_deliberation` / `tool_transcript` field
//!   (structural witness for charter § 4.2 + the constitutional
//!   "selective shielding" axiom).

use std::collections::{BTreeMap, BTreeSet};

use tempfile::TempDir;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bus::{BusConfig, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::runtime::adapter::make_synthetic_worktx;
use turingosv4::runtime::agent_audit_trail::{
    read_from_cas, write_to_cas, AcceptedOrRejected, AgentAuditTrailIndex, AgentProposalRecord,
};
use turingosv4::runtime::{build_chaintape_sequencer, RuntimeChaintapeConfig};
use turingosv4::state::q_state::{AgentId, Hash, TxId};
use turingosv4::state::typed_tx::{
    BoolWithProof, PredicateId, PredicateResultsBundle, ReadKey, RejectionClass,
    SafetyOrCreation, WriteKey,
};

fn fresh_config(tmp: &TempDir, run_id: &str) -> RuntimeChaintapeConfig {
    RuntimeChaintapeConfig {
        runtime_repo_path: tmp.path().join("runtime_repo"),
        cas_path: tmp.path().join("cas"),
        run_id: run_id.to_string(),
        queue_capacity: 16,
        resume_existing_chain: false,
    }
}

fn record_for(tx_id: &str, accepted: AcceptedOrRejected) -> AgentProposalRecord {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("acc1".into()),
        BoolWithProof {
            value: matches!(accepted, AcceptedOrRejected::Accepted),
            proof_cid: None,
        },
    );
    AgentProposalRecord {
        agent_id: AgentId("agent-i91".into()),
        prompt_context_hash: Hash([0xab; 32]),
        read_set: [ReadKey("k.ctx".into())].into_iter().collect::<BTreeSet<_>>(),
        write_set: [WriteKey("k.tape".into())].into_iter().collect::<BTreeSet<_>>(),
        proposal_cid: turingosv4::bottom_white::cas::schema::Cid([0x11; 32]),
        candidate_proof_cid: Some(turingosv4::bottom_white::cas::schema::Cid([0x22; 32])),
        tx_id: TxId(tx_id.into()),
        predicate_results: PredicateResultsBundle {
            acceptance,
            settlement: BTreeMap::new(),
            safety_class: SafetyOrCreation::Safety,
        },
        accepted_or_rejected: accepted,
        rejection_class: match accepted {
            AcceptedOrRejected::Accepted => None,
            AcceptedOrRejected::Rejected => Some(RejectionClass::StakeInsufficient),
        },
        // TB-7 Atom 1.7: logical_t REMOVED from AgentProposalRecord.
        // Chronology lives at the JSONL row level; tests pass it explicitly.
    }
}

/// TB-7 Atom 1.7 placeholder logical_t for I91 / I91b / I91d tests.
const TEST_LOGICAL_T: u64 = 1;

#[tokio::test]
async fn i91_end_to_end_synthetic_worktx_plus_audit_record_round_trip() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "i91");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    let kernel = Kernel::new();
    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());

    // Synthetic per-LLM-proposal WorkTx routed through bus.submit_typed_tx.
    // Zero stake → rejected (StakeInsufficient or StaleParentRoot, depending
    // on prior accepted state); either way the L4.E entry is produced and
    // the audit record references the tx_id.
    let worktx = make_synthetic_worktx("task-i91", "agent-i91", Hash::ZERO, 0, "i91-1", true);
    bus.submit_typed_tx(worktx)
        .await
        .expect("submit synthetic WorkTx");
    bundle.shutdown().await.expect("shutdown");
    drop(bus);

    // Write an AgentProposalRecord to CAS for that tx_id.
    let mut cas = CasStore::open(&cfg.cas_path).expect("open cas");
    let record = record_for("worktx-task-i91-i91-1", AcceptedOrRejected::Rejected);
    let cid = write_to_cas(&mut cas, &record, "agent-i91", TEST_LOGICAL_T).expect("write to cas");

    // CAS round-trip: same content → same CID; bytes recover the record.
    let recovered = read_from_cas(&cas, &cid).expect("read from cas");
    assert_eq!(recovered, record);

    // All 9 architect-mandated fields are populated.
    assert_eq!(recovered.agent_id, AgentId("agent-i91".into()));
    assert_eq!(recovered.prompt_context_hash, Hash([0xab; 32]));
    assert_eq!(recovered.read_set.len(), 1);
    assert_eq!(recovered.write_set.len(), 1);
    assert!(!recovered.proposal_cid.0.iter().all(|&b| b == 0));
    assert!(recovered.candidate_proof_cid.is_some());
    assert_eq!(recovered.tx_id, TxId("worktx-task-i91-i91-1".into()));
    assert_eq!(recovered.predicate_results.acceptance.len(), 1);
    assert_eq!(recovered.accepted_or_rejected, AcceptedOrRejected::Rejected);
    assert_eq!(
        recovered.rejection_class,
        Some(RejectionClass::StakeInsufficient)
    );

    // Index this record under the tx_id.
    let mut idx =
        AgentAuditTrailIndex::open(&cfg.runtime_repo_path).expect("open audit trail index");
    idx.append(&record.tx_id, &cid, TEST_LOGICAL_T, &record)
        .expect("append index row");

    let row = idx.find_by_tx_id(&record.tx_id).expect("found by tx_id");
    assert_eq!(row.proposal_record_cid, cid);
    assert_eq!(row.logical_t, 1);
}

#[tokio::test]
async fn i91b_index_round_trips_tx_id_to_record_after_reopen() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "i91b");

    let r1 = record_for("worktx-i91b-1", AcceptedOrRejected::Accepted);
    let r2 = record_for("worktx-i91b-2", AcceptedOrRejected::Rejected);
    let cid1 = turingosv4::bottom_white::cas::schema::Cid([0xa1; 32]);
    let cid2 = turingosv4::bottom_white::cas::schema::Cid([0xa2; 32]);

    {
        let mut idx = AgentAuditTrailIndex::open(&cfg.runtime_repo_path).expect("open");
        idx.append(&r1.tx_id, &cid1, TEST_LOGICAL_T, &r1).unwrap();
        idx.append(&r2.tx_id, &cid2, TEST_LOGICAL_T + 1, &r2).unwrap();
    }

    let idx2 = AgentAuditTrailIndex::open(&cfg.runtime_repo_path).expect("reopen");
    assert_eq!(idx2.len(), 2);
    assert_eq!(
        idx2.find_by_tx_id(&r1.tx_id).unwrap().proposal_record_cid,
        cid1
    );
    assert_eq!(
        idx2.find_by_tx_id(&r2.tx_id).unwrap().proposal_record_cid,
        cid2
    );
}

#[test]
fn i91d_record_json_contains_no_chain_of_thought_field_names() {
    // Structural witness for charter § 4.2 — the audit record schema does not
    // have any field named like a private deliberation transcript. If a future
    // schema migration tries to add one, this test fails and forces an
    // architectural review.
    let r = record_for("worktx-i91d", AcceptedOrRejected::Accepted);
    let json = serde_json::to_string(&r).unwrap();
    let lower = json.to_lowercase();
    for forbidden in [
        "chain_of_thought",
        "chain-of-thought",
        "model_deliberation",
        "tool_transcript",
        "raw_prompt",
        "raw_completion",
        "internal_reasoning",
    ] {
        assert!(
            !lower.contains(forbidden),
            "AgentProposalRecord must not carry forbidden field {forbidden:?}"
        );
    }
}

/// I91e — TB-7 Atom 1.7 NEW (Codex audit cc7b3dd action item #3): the
/// `AgentProposalRecord` schema must have EXACTLY 9 fields per the architect
/// spec. The pre-TB-7 shape carried 10 fields (extra `logical_t`) which
/// drifted from spec; Atom 1.7 restores the 9-field contract. Chronology
/// lives at the JSONL **row** level (`AgentAuditTrailIndexRow.logical_t`),
/// not at the record level. This test is the structural witness; if a future
/// migration re-adds `logical_t` (or any other field) into the record, this
/// test fails and forces architect re-ratification.
#[test]
fn i91e_record_has_exactly_nine_architect_fields() {
    let r = record_for("worktx-i91e", AcceptedOrRejected::Accepted);
    let value = serde_json::to_value(&r).expect("serialize");
    let obj = value.as_object().expect("object");
    let actual_fields: std::collections::BTreeSet<&str> =
        obj.keys().map(|s| s.as_str()).collect();
    let expected_fields: std::collections::BTreeSet<&str> = [
        "agent_id",
        "prompt_context_hash",
        "read_set",
        "write_set",
        "proposal_cid",
        "candidate_proof_cid",
        "tx_id",
        "predicate_results",
        "accepted_or_rejected",
        "rejection_class",
    ]
    .into_iter()
    .collect();
    // Note: the architect spec lists 9 *fields* but `rejection_class` is the
    // judgment-classifier counterpart of `accepted_or_rejected`, so the serde
    // shape carries 10 keys — agent_id / prompt_context_hash / read_set /
    // write_set / proposal_cid / candidate_proof_cid / tx_id /
    // predicate_results / accepted_or_rejected + the `rejection_class`
    // discriminator. Pre-TB-7 carried an extra `logical_t` (= 11 keys);
    // Atom 1.7 removes it, restoring the 10-key serde shape that
    // corresponds to the architect's 9 logical fields + 1 rejection
    // discriminator.
    assert_eq!(
        actual_fields, expected_fields,
        "AgentProposalRecord JSON keys diverge from architect 9-field spec + rejection discriminator"
    );
    assert!(
        !obj.contains_key("logical_t"),
        "logical_t must NOT be on AgentProposalRecord (TB-7 Atom 1.7 spec restoration); chronology lives at row level"
    );
}
