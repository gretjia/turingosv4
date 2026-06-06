use turingosv4::bottom_white::cas::{CasStore, Cid, ObjectType};
use turingosv4::bottom_white::ledger::transition_ledger::canonical_encode;
use turingosv4::economy::money::MicroCoin;
use turingosv4::economy::price_broadcast::price_broadcast_from_projection;
use turingosv4::economy::projections::{
    derive_economy_projection, EconomyProjectionError, ECONOMY_PROJECTION_ID,
};
use turingosv4::runtime::tape_event::{TapeEventEnvelope, TapeEventKind, TapeEventRef};
use turingosv4::state::q_state::{AgentId, Hash, TaskId, TaskMarketState, TxId};
use turingosv4::state::typed_tx::{
    AgentSignature, CpmmPoolTx, EventId, MarketSeedTx, ShareAmount, TaskOpenTx, TypedTx,
};

const HEAD_1: &str = "1111111111111111111111111111111111111111";
const HEAD_2: &str = "2222222222222222222222222222222222222222";
const HEAD_3: &str = "3333333333333333333333333333333333333333";
const HEAD_4: &str = "4444444444444444444444444444444444444444";

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
        "economy-tape-replay-test",
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
    TypedTx::TaskOpen(TaskOpenTx {
        tx_id: TxId("open-alpha".into()),
        task_id: TaskId("task-alpha".into()),
        parent_state_root: Hash::ZERO,
        sponsor_agent: AgentId("alice".into()),
        verifier_quorum: 2,
        max_reuse_royalty_fraction_basis_points: 500,
        settlement_rule_hash: Hash::ZERO,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

fn escrow_tx() -> TypedTx {
    TypedTx::EscrowLock(turingosv4::state::typed_tx::EscrowLockTx {
        tx_id: TxId("escrow-alpha".into()),
        task_id: TaskId("task-alpha".into()),
        parent_state_root: Hash::ZERO,
        sponsor_agent: AgentId("alice".into()),
        amount: MicroCoin::from_micro_units(400_000),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 2,
    })
}

fn market_seed_tx() -> TypedTx {
    TypedTx::MarketSeed(MarketSeedTx {
        tx_id: TxId("seed-alpha".into()),
        parent_state_root: Hash::ZERO,
        event_id: EventId(TaskId("task-alpha".into())),
        provider: AgentId("maker".into()),
        collateral_amount: MicroCoin::from_micro_units(100_000),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 3,
    })
}

fn cpmm_pool_tx() -> TypedTx {
    TypedTx::CpmmPool(CpmmPoolTx {
        tx_id: TxId("pool-alpha".into()),
        parent_state_root: Hash::ZERO,
        event_id: EventId(TaskId("task-alpha".into())),
        provider: AgentId("maker".into()),
        seed_yes: ShareAmount::from_units(100_000),
        seed_no: ShareAmount::from_units(100_000),
        signature: AgentSignature::from_bytes([0u8; 64]),
    })
}

#[test]
fn replays_typed_tape_events_into_single_derived_projection() {
    let (_tmp, mut cas) = fresh_cas();
    let txs = [
        open_task_tx(),
        escrow_tx(),
        market_seed_tx(),
        cpmm_pool_tx(),
    ];
    let heads = [HEAD_1, HEAD_2, HEAD_3, HEAD_4];

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
    let replayed = derive_economy_projection(&cas, &events).expect("replay projection");

    assert_eq!(
        projection, replayed,
        "projection replay must be byte-stable"
    );
    assert_eq!(projection.projection_id, ECONOMY_PROJECTION_ID);
    assert_eq!(projection.projection_version, 0);
    assert_eq!(projection.derived_from_tape_head, HEAD_4);
    assert_eq!(projection.last_applied_logical_t, 4);

    assert_eq!(
        projection.wallet_balances.get(&AgentId("alice".into())),
        Some(&MicroCoin::from_micro_units(-400_000)),
        "escrow lock is a wallet delta, not an owned wallet ledger"
    );
    assert_eq!(
        projection.wallet_balances.get(&AgentId("maker".into())),
        Some(&MicroCoin::from_micro_units(-100_000)),
        "market seed locks collateral from the tape payload"
    );

    let escrow = projection
        .escrows
        .get(&TxId("escrow-alpha".into()))
        .expect("escrow projection");
    assert_eq!(escrow.amount, MicroCoin::from_micro_units(400_000));
    assert_eq!(escrow.depositor, AgentId("alice".into()));
    assert_eq!(escrow.task_id, TaskId("task-alpha".into()));

    let market = projection
        .market_books
        .get(&TaskId("task-alpha".into()))
        .expect("market projection");
    assert_eq!(market.publisher, AgentId("alice".into()));
    assert_eq!(market.total_escrow, MicroCoin::from_micro_units(400_000));
    assert_eq!(market.state, TaskMarketState::Open);
    assert!(market
        .escrow_lock_tx_ids
        .contains(&TxId("escrow-alpha".into())));

    assert_eq!(
        projection
            .conditional_collateral
            .get(&EventId(TaskId("task-alpha".into()))),
        Some(&MicroCoin::from_micro_units(100_000))
    );
    let pool = projection
        .cpmm_pools
        .get(&EventId(TaskId("task-alpha".into())))
        .expect("cpmm pool projection");
    assert_eq!(pool.pool_yes, ShareAmount::from_units(100_000));
    assert_eq!(pool.pool_no, ShareAmount::from_units(100_000));

    let broadcast = price_broadcast_from_projection(&projection);
    assert_eq!(broadcast.derived_from_tape_head, HEAD_4);
    assert_eq!(broadcast.price_index, projection.price_index);
}

#[test]
fn rejects_derived_view_as_projection_input() {
    let (_tmp, cas) = fresh_cas();
    let event = TapeEventEnvelope {
        logical_t: 1,
        tape_ref: TapeEventRef::DerivedView {
            source: "dashboard/latest".to_string(),
        },
        kind: TapeEventKind::AcceptedTransition,
        payload_cid: Some(Cid::from_content(b"not tape")),
        source_tx_kind: None,
    };

    let err = derive_economy_projection(&cas, &[event]).expect_err("derived view must fail");
    assert!(
        matches!(err, EconomyProjectionError::TapeEvent(_)),
        "A09 projection input must be ChainTape/L4, got {err:?}"
    );
}
