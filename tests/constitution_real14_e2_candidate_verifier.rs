//! REAL-14 — independent E2 candidate verifier gates.
//!
//! These tests bind the R16 audit follow-up requirement that live agent
//! economic action must be recomputed from ChainTape/CAS, not copied from
//! dashboard text.

use std::collections::BTreeMap;

use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
use turingosv4::bottom_white::ledger::transition_ledger::{
    append, canonical_encode, Git2LedgerWriter, LedgerEntry, LedgerEntrySigningPayload,
    LedgerWriter, TxKind,
};
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::market_decision_trace::{
    write_market_decision_trace_to_cas, MarketDecisionTrace,
};
use turingosv4::runtime::market_e2_candidate_verifier::{
    verify_market_e2_candidate, E2CandidateVerifierOptions, E2CandidateVerifierVerdict,
};
use turingosv4::state::q_state::{AgentId, Hash, TaskId, TxId};
use turingosv4::state::typed_tx::{
    AgentSignature, BuyDirection, BuyWithCoinRouterTx, EventId, ShareAmount, TypedTx,
};

fn router_tx(tx_id: &str, buyer: &str) -> TypedTx {
    TypedTx::BuyWithCoinRouter(BuyWithCoinRouterTx {
        tx_id: TxId(tx_id.to_string()),
        parent_state_root: Hash([0u8; 32]),
        event_id: EventId(TaskId("event-real14".to_string())),
        buyer: AgentId(buyer.to_string()),
        direction: BuyDirection::BuyYes,
        pay_coin: MicroCoin::from_micro_units(1_000),
        min_out_shares: ShareAmount { units: 1 },
        signature: AgentSignature::from_bytes([0u8; 64]),
    })
}

fn tx_kind(tx: &TypedTx) -> TxKind {
    match tx {
        TypedTx::BuyWithCoinRouter(_) => TxKind::BuyWithCoinRouter,
        other => other.tx_kind(),
    }
}

fn append_typed_tx(
    writer: &mut Git2LedgerWriter,
    cas: &mut CasStore,
    logical_t: u64,
    tx: &TypedTx,
    parent_ledger_root: &mut Hash,
) {
    let payload = canonical_encode(tx).expect("canonical typed tx");
    let cid = cas
        .put(
            &payload,
            ObjectType::ProposalPayload,
            "real14-test",
            logical_t,
            Some("TypedTx.v1".to_string()),
        )
        .expect("put typed tx");
    let signing = LedgerEntrySigningPayload {
        logical_t,
        parent_state_root: Hash([0u8; 32]),
        parent_ledger_root: *parent_ledger_root,
        tx_kind: tx_kind(tx),
        tx_payload_cid: cid,
        resulting_state_root: Hash([logical_t as u8; 32]),
        timestamp_logical: logical_t,
        epoch: SystemEpoch::new(1),
        extensions: BTreeMap::new(),
    };
    let resulting_ledger_root = append(parent_ledger_root, &signing.canonical_digest());
    let entry = LedgerEntry {
        logical_t,
        parent_state_root: signing.parent_state_root,
        parent_ledger_root: signing.parent_ledger_root,
        tx_kind: signing.tx_kind,
        tx_payload_cid: signing.tx_payload_cid,
        resulting_state_root: signing.resulting_state_root,
        resulting_ledger_root,
        timestamp_logical: signing.timestamp_logical,
        epoch: signing.epoch,
        extensions: signing.extensions,
        system_signature: SystemSignature::from_bytes([0u8; 64]),
    };
    writer.commit(&entry).expect("commit l4 entry");
    *parent_ledger_root = resulting_ledger_root;
}

fn put_submitted_trace(cas: &mut CasStore, agent: &str, tx_id: &str, logical_t: u64) {
    let trace = MarketDecisionTrace::submitted(
        AgentId(agent.to_string()),
        TxId(format!("node-{logical_t}")),
        BuyDirection::BuyYes,
        1_000,
        TxId(tx_id.to_string()),
        "submitted by fixture",
    );
    write_market_decision_trace_to_cas(cas, &trace, tx_id, logical_t).expect("put trace");
}

#[test]
fn exact_join_verifier_counts_only_l4_and_submitted_trace_intersection() {
    let repo_dir = tempfile::tempdir().expect("repo dir");
    let cas_dir = tempfile::tempdir().expect("cas dir");
    let mut cas = CasStore::open(cas_dir.path()).expect("cas open");
    let mut writer = Git2LedgerWriter::open(repo_dir.path()).expect("writer open");
    let mut parent_ledger_root = Hash([0u8; 32]);

    append_typed_tx(
        &mut writer,
        &mut cas,
        1,
        &router_tx("router-matched", "Agent_0"),
        &mut parent_ledger_root,
    );
    append_typed_tx(
        &mut writer,
        &mut cas,
        2,
        &router_tx("router-unmatched-l4", "Agent_1"),
        &mut parent_ledger_root,
    );
    put_submitted_trace(&mut cas, "Agent_0", "router-matched", 3);
    put_submitted_trace(&mut cas, "Agent_2", "router-unmatched-trace", 4);

    let report = verify_market_e2_candidate(
        repo_dir.path(),
        cas_dir.path(),
        E2CandidateVerifierOptions {
            expected_exact_join_count: Some(1),
            require_matched_tx_provenance: false,
        },
    )
    .expect("verify");

    assert_eq!(report.l4_router_tx_count, 2);
    assert_eq!(report.submitted_trace_tx_count, 2);
    assert_eq!(report.exact_join_count, 1);
    assert_eq!(report.matched_tx_ids, vec!["router-matched".to_string()]);
    assert_eq!(
        report.unmatched_l4_router_tx_ids,
        vec!["router-unmatched-l4".to_string()]
    );
    assert_eq!(
        report.unmatched_submitted_trace_tx_ids,
        vec!["router-unmatched-trace".to_string()]
    );
    assert_eq!(report.verdict, E2CandidateVerifierVerdict::Proceed);
}

#[test]
fn verifier_markdown_does_not_label_zero_join_runs_as_candidates() {
    let repo_dir = tempfile::tempdir().expect("repo dir");
    let cas_dir = tempfile::tempdir().expect("cas dir");
    let _cas = CasStore::open(cas_dir.path()).expect("cas open");
    let _writer = Git2LedgerWriter::open(repo_dir.path()).expect("writer open");

    let report = verify_market_e2_candidate(
        repo_dir.path(),
        cas_dir.path(),
        E2CandidateVerifierOptions::default(),
    )
    .expect("verify");
    let markdown = report.render_markdown();

    assert_eq!(report.exact_join_count, 0);
    assert_eq!(report.verdict, E2CandidateVerifierVerdict::Proceed);
    assert!(markdown.contains("claim_boundary: clean-negative: no E2 candidate in this run"));
    assert!(!markdown.contains("claim_boundary: E2 candidate pending audit"));
}

#[test]
fn exact_join_verifier_requires_matched_tx_provenance_by_default() {
    let repo_dir = tempfile::tempdir().expect("repo dir");
    let cas_dir = tempfile::tempdir().expect("cas dir");
    let mut cas = CasStore::open(cas_dir.path()).expect("cas open");
    let mut writer = Git2LedgerWriter::open(repo_dir.path()).expect("writer open");
    let mut parent_ledger_root = Hash([0u8; 32]);

    append_typed_tx(
        &mut writer,
        &mut cas,
        1,
        &router_tx("router-matched-no-provenance", "Agent_0"),
        &mut parent_ledger_root,
    );
    put_submitted_trace(&mut cas, "Agent_0", "router-matched-no-provenance", 2);

    let report = verify_market_e2_candidate(
        repo_dir.path(),
        cas_dir.path(),
        E2CandidateVerifierOptions {
            expected_exact_join_count: Some(1),
            ..E2CandidateVerifierOptions::default()
        },
    )
    .expect("verify");

    assert_eq!(report.exact_join_count, 1);
    assert_eq!(report.verdict, E2CandidateVerifierVerdict::Veto);
    assert!(report
        .failure_reasons
        .iter()
        .any(|reason| reason.contains("no EVDecisionTrace")));
    assert!(report
        .failure_reasons
        .iter()
        .any(|reason| reason.contains("no MarketOpportunityTrace")));
    assert!(report
        .failure_reasons
        .iter()
        .any(|reason| reason.contains("no PromptCapsule")));
}

#[test]
fn exact_join_verifier_fails_closed_on_duplicate_l4_router_tx_id() {
    let repo_dir = tempfile::tempdir().expect("repo dir");
    let cas_dir = tempfile::tempdir().expect("cas dir");
    let mut cas = CasStore::open(cas_dir.path()).expect("cas open");
    let mut writer = Git2LedgerWriter::open(repo_dir.path()).expect("writer open");
    let mut parent_ledger_root = Hash([0u8; 32]);

    let tx = router_tx("router-dup", "Agent_0");
    append_typed_tx(&mut writer, &mut cas, 1, &tx, &mut parent_ledger_root);
    append_typed_tx(&mut writer, &mut cas, 2, &tx, &mut parent_ledger_root);
    put_submitted_trace(&mut cas, "Agent_0", "router-dup", 3);

    let report = verify_market_e2_candidate(
        repo_dir.path(),
        cas_dir.path(),
        E2CandidateVerifierOptions {
            expected_exact_join_count: Some(1),
            require_matched_tx_provenance: false,
        },
    )
    .expect("verify");

    assert_eq!(report.duplicate_l4_router_tx_id_count, 1);
    assert_eq!(report.verdict, E2CandidateVerifierVerdict::Veto);
    assert!(report
        .failure_reasons
        .iter()
        .any(|reason| reason.contains("duplicate L4 router tx_id")));
}

#[test]
fn exact_join_verifier_fails_closed_on_unknown_market_decision_schema() {
    let repo_dir = tempfile::tempdir().expect("repo dir");
    let cas_dir = tempfile::tempdir().expect("cas dir");
    let mut cas = CasStore::open(cas_dir.path()).expect("cas open");
    let _writer = Git2LedgerWriter::open(repo_dir.path()).expect("writer open");
    let unknown = serde_json::json!({
        "schema_version": "tb_n3.market_decision_trace.v0",
        "outcome": {"Submitted": {"tx_id": "router-unknown"}},
    });
    cas.put(
        serde_json::to_vec(&unknown).unwrap().as_slice(),
        ObjectType::AttemptTelemetry,
        "real14-test",
        1,
        None,
    )
    .expect("put unknown trace");

    let report = verify_market_e2_candidate(
        repo_dir.path(),
        cas_dir.path(),
        E2CandidateVerifierOptions::default(),
    )
    .expect("verify");

    assert_eq!(report.verdict, E2CandidateVerifierVerdict::Veto);
    assert!(report
        .failure_reasons
        .iter()
        .any(|reason| reason.contains("unknown MarketDecisionTrace schema")));
}

#[test]
fn real14_verifier_cli_and_dashboard_binding_are_source_separated() {
    let dashboard = std::fs::read_to_string("src/bin/audit_dashboard.rs").expect("dashboard");
    let cli = std::fs::read_to_string("src/bin/real14_e2_candidate_verifier.rs")
        .expect("REAL-14 verifier CLI must exist");
    let helper =
        std::fs::read_to_string("src/runtime/market_e2_candidate_verifier.rs").expect("helper");

    assert!(
        dashboard.contains("verify_market_e2_candidate("),
        "dashboard should consume the independent verifier helper"
    );
    assert!(
        cli.contains("verify_market_e2_candidate("),
        "CLI must consume the independent verifier helper"
    );
    assert!(
        !helper.contains("audit_dashboard_run_report"),
        "independent verifier must not parse dashboard text as truth"
    );
}
