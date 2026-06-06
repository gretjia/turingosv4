use turingosv4::bottom_white::cas::{CasStore, Cid, ObjectType};
use turingosv4::bottom_white::ledger::transition_ledger::canonical_encode;
use turingosv4::economy::money::{MicroCoin, StakeMicroCoin};
use turingosv4::economy::projections::{
    derive_economy_projection, derive_economy_projection_with_cache, economy_projection_cache_key,
    EconomyProjectionCache, EconomyProjectionCacheReplayReason, EconomyProjectionCacheStatus,
};
use turingosv4::runtime::tape_event::{TapeEventEnvelope, TapeEventKind, TapeEventRef};
use turingosv4::state::q_state::{AgentId, Hash, TaskId, TxId};
use turingosv4::state::typed_tx::{
    AgentSignature, ChallengeTx, PredicateResultsBundle, ReadKey, TaskOpenTx, TypedTx, WorkTx,
    WriteKey,
};

const HEAD_1: &str = "1010101010101010101010101010101010101010";
const HEAD_2: &str = "2020202020202020202020202020202020202020";
const HEAD_3: &str = "3030303030303030303030303030303030303030";
const ORPHAN_HEAD: &str = "9090909090909090909090909090909090909090";

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
        "economy-projection-cache-test",
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
        tx_id: TxId("open-cache".into()),
        task_id: TaskId("task-cache".into()),
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
        tx_id: TxId("work-cache".into()),
        task_id: TaskId("task-cache".into()),
        parent_state_root: Hash::ZERO,
        agent_id: AgentId("solver".into()),
        read_set: [ReadKey("input".into())].into_iter().collect(),
        write_set: [WriteKey("proof".into())].into_iter().collect(),
        proposal_cid: Cid::from_content(b"cache-work"),
        predicate_results: PredicateResultsBundle::default(),
        stake: StakeMicroCoin::from_micro_units(150_000),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 2,
    })
}

fn challenge_tx() -> TypedTx {
    TypedTx::Challenge(ChallengeTx {
        tx_id: TxId("challenge-cache".into()),
        parent_state_root: Hash::ZERO,
        target_work_tx: TxId("work-cache".into()),
        challenger_agent: AgentId("challenger".into()),
        stake: StakeMicroCoin::from_micro_units(50_000),
        counterexample_cid: Cid::from_content(b"cache-counterexample"),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 3,
    })
}

fn tape(cas: &mut CasStore) -> Vec<TapeEventEnvelope> {
    let txs = [open_task_tx(), work_tx(), challenge_tx()];
    let heads = [HEAD_1, HEAD_2, HEAD_3];
    txs.iter()
        .enumerate()
        .map(|(idx, tx)| {
            let logical_t = (idx + 1) as u64;
            let cid = put_typed_tx(cas, tx, logical_t);
            accepted_event(logical_t, heads[idx], tx, cid)
        })
        .collect()
}

#[test]
fn cache_hit_requires_current_git_oid_watermark() {
    let (_tmp, mut cas) = fresh_cas();
    let events = tape(&mut cas);
    let full = derive_economy_projection(&cas, &events).expect("full replay");
    let mut cache = EconomyProjectionCache::default();

    let first =
        derive_economy_projection_with_cache(&cas, &events, &mut cache).expect("cached replay");
    assert_eq!(first.projection, full);
    assert!(matches!(
        first.status,
        EconomyProjectionCacheStatus::FullReplay {
            reason: EconomyProjectionCacheReplayReason::EmptyCache
        }
    ));

    let second =
        derive_economy_projection_with_cache(&cas, &events, &mut cache).expect("cache hit");
    assert_eq!(second.projection, full);
    assert!(matches!(
        second.status,
        EconomyProjectionCacheStatus::CacheHit { ref head_oid } if head_oid == HEAD_3
    ));

    let mut tampered = second.projection.clone();
    tampered.derived_from_tape_head = HEAD_2.to_string();
    tampered.wallet_balances.insert(
        AgentId("attacker".into()),
        MicroCoin::from_micro_units(9_999_999),
    );
    cache.insert(economy_projection_cache_key(HEAD_3), tampered);

    let repaired =
        derive_economy_projection_with_cache(&cas, &events, &mut cache).expect("repair by replay");
    assert_eq!(
        repaired.projection, full,
        "tampered current-head cache must not replace tape replay"
    );
    assert!(matches!(
        repaired.status,
        EconomyProjectionCacheStatus::FullReplay {
            reason: EconomyProjectionCacheReplayReason::TamperedCurrentEntry
        }
    ));
}

#[test]
fn stale_cache_delta_applies_only_with_ancestry_proof() {
    let (_tmp, mut cas) = fresh_cas();
    let events = tape(&mut cas);
    let prefix = &events[..2];
    let prefix_projection = derive_economy_projection(&cas, prefix).expect("prefix replay");
    let full = derive_economy_projection(&cas, &events).expect("full replay");
    let mut cache = EconomyProjectionCache::default();

    cache.insert(economy_projection_cache_key(HEAD_2), prefix_projection);
    let delta =
        derive_economy_projection_with_cache(&cas, &events, &mut cache).expect("delta apply");
    assert_eq!(delta.projection, full);
    assert!(matches!(
        delta.status,
        EconomyProjectionCacheStatus::DeltaApplied {
            ref from_head_oid,
            ref to_head_oid,
            delta_event_count: 1,
        } if from_head_oid == HEAD_2 && to_head_oid == HEAD_3
    ));

    let mut orphan_cache = EconomyProjectionCache::default();
    let mut orphan_projection = full.clone();
    orphan_projection.derived_from_tape_head = ORPHAN_HEAD.to_string();
    orphan_projection.last_applied_logical_t = 99;
    orphan_cache.insert(economy_projection_cache_key(ORPHAN_HEAD), orphan_projection);

    let repaired = derive_economy_projection_with_cache(&cas, &events, &mut orphan_cache)
        .expect("orphan cache repaired by full replay");
    assert_eq!(repaired.projection, full);
    assert!(matches!(
        repaired.status,
        EconomyProjectionCacheStatus::FullReplay {
            reason: EconomyProjectionCacheReplayReason::NoAncestorForStaleCache
        }
    ));
}
