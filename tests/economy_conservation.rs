use turingosv4::bottom_white::cas::{CasStore, Cid, ObjectType};
use turingosv4::bottom_white::ledger::transition_ledger::canonical_encode;
use turingosv4::economy::conservation::{
    assert_projection_conserved, conservation_report_from_projection,
};
use turingosv4::economy::money::{MicroCoin, StakeMicroCoin};
use turingosv4::economy::projections::derive_economy_projection;
use turingosv4::runtime::tape_event::{TapeEventEnvelope, TapeEventKind, TapeEventRef};
use turingosv4::state::q_state::{AgentId, Hash, TaskId, TxId};
use turingosv4::state::typed_tx::{
    AgentSignature, ChallengeTx, PredicateResultsBundle, ReadKey, TypedTx, WorkTx, WriteKey,
};

const HEAD_1: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const HEAD_2: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const HEAD_3: &str = "cccccccccccccccccccccccccccccccccccccccc";

fn fresh_cas() -> (tempfile::TempDir, CasStore) {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let cas = CasStore::open(tmp.path()).expect("open cas");
    (tmp, cas)
}

fn put_typed_tx(cas: &mut CasStore, tx: &TypedTx, logical_t: u64) -> Cid {
    let bytes = canonical_encode(tx).expect("encode typed tx");
    cas.put(
        &bytes,
        ObjectType::Generic,
        "economy-conservation-test",
        logical_t,
        Some("turingosv4.typed_tx.v1".to_string()),
    )
    .expect("put typed tx")
}

fn accepted_event(
    logical_t: u64,
    head_oid: &str,
    tx: &TypedTx,
    payload_cid: Cid,
) -> TapeEventEnvelope {
    TapeEventEnvelope {
        logical_t,
        tape_ref: TapeEventRef::L4Accepted {
            head_oid: head_oid.to_string(),
        },
        kind: TapeEventKind::AcceptedTransition,
        payload_cid: Some(payload_cid),
        source_tx_kind: Some(tx.tx_kind()),
    }
}

fn open_task_tx() -> TypedTx {
    TypedTx::TaskOpen(turingosv4::state::typed_tx::TaskOpenTx {
        tx_id: TxId("open-beta".into()),
        task_id: TaskId("task-beta".into()),
        parent_state_root: Hash::ZERO,
        sponsor_agent: AgentId("sponsor".into()),
        verifier_quorum: 1,
        max_reuse_royalty_fraction_basis_points: 100,
        settlement_rule_hash: Hash::ZERO,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

fn work_tx() -> TypedTx {
    TypedTx::Work(WorkTx {
        tx_id: TxId("work-beta".into()),
        task_id: TaskId("task-beta".into()),
        parent_state_root: Hash::ZERO,
        agent_id: AgentId("solver".into()),
        read_set: [ReadKey("input".into())].into_iter().collect(),
        write_set: [WriteKey("proof".into())].into_iter().collect(),
        proposal_cid: Cid::from_content(b"work-payload"),
        predicate_results: PredicateResultsBundle::default(),
        stake: StakeMicroCoin::from_micro_units(150_000),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 2,
    })
}

fn challenge_tx() -> TypedTx {
    TypedTx::Challenge(ChallengeTx {
        tx_id: TxId("challenge-beta".into()),
        parent_state_root: Hash::ZERO,
        target_work_tx: TxId("work-beta".into()),
        challenger_agent: AgentId("challenger".into()),
        stake: StakeMicroCoin::from_micro_units(50_000),
        counterexample_cid: Cid::from_content(b"counterexample"),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 3,
    })
}

#[test]
fn conservation_sums_wallet_deltas_and_position_holdings_from_same_tape_prefix() {
    let (_tmp, mut cas) = fresh_cas();
    let txs = [open_task_tx(), work_tx(), challenge_tx()];
    let heads = [HEAD_1, HEAD_2, HEAD_3];

    let events: Vec<TapeEventEnvelope> = txs
        .iter()
        .enumerate()
        .map(|(idx, tx)| {
            let logical_t = (idx + 1) as u64;
            let cid = put_typed_tx(&mut cas, tx, logical_t);
            accepted_event(logical_t, heads[idx], tx, cid)
        })
        .collect();

    let projection = derive_economy_projection(&cas, &events).expect("derive projection");
    let report = conservation_report_from_projection(&projection).expect("conservation report");

    assert_eq!(projection.derived_from_tape_head, HEAD_3);
    assert_eq!(
        projection.wallet_balances.get(&AgentId("solver".into())),
        Some(&MicroCoin::from_micro_units(-150_000))
    );
    assert_eq!(
        projection
            .wallet_balances
            .get(&AgentId("challenger".into())),
        Some(&MicroCoin::from_micro_units(-50_000))
    );
    assert_eq!(
        report.wallet_delta_micro + report.escrow_micro + report.open_position_micro,
        report.minted_total_micro,
        "projection conservation must be derived from one tape prefix"
    );
    assert_eq!(report.minted_total_micro, 0);
    assert_eq!(report.open_position_micro, 200_000);
    assert_eq!(report.total_supply_delta_micro, 0);
    assert_eq!(projection.conservation_root, report.conservation_root);

    assert_projection_conserved(&projection).expect("projection is conserved");
}
