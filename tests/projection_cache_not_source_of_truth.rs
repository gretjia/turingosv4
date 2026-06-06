use std::path::PathBuf;

use turingosv4::bottom_white::cas::{CasStore, Cid, ObjectType};
use turingosv4::bottom_white::ledger::transition_ledger::canonical_encode;
use turingosv4::economy::money::MicroCoin;
use turingosv4::economy::projections::{
    derive_economy_projection, derive_economy_projection_with_cache, economy_projection_cache_key,
    EconomyProjectionCache, EconomyProjectionCacheReplayReason, EconomyProjectionCacheStatus,
};
use turingosv4::runtime::tape_event::{TapeEventEnvelope, TapeEventKind, TapeEventRef};
use turingosv4::state::q_state::{AgentId, Hash, TaskId, TxId};
use turingosv4::state::typed_tx::{AgentSignature, TaskOpenTx, TypedTx};

const HEAD: &str = "abababababababababababababababababababab";

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
        "projection-cache-not-truth-test",
        logical_t,
        Some("turingosv4.typed_tx.v1".to_string()),
    )
    .expect("put typed tx")
}

fn task_open_tx() -> TypedTx {
    TypedTx::TaskOpen(TaskOpenTx {
        tx_id: TxId("open-cache-not-truth".into()),
        task_id: TaskId("task-cache-not-truth".into()),
        parent_state_root: Hash::ZERO,
        sponsor_agent: AgentId("sponsor".into()),
        verifier_quorum: 1,
        max_reuse_royalty_fraction_basis_points: 0,
        settlement_rule_hash: Hash::ZERO,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

fn one_event_tape(cas: &mut CasStore) -> Vec<TapeEventEnvelope> {
    let tx = task_open_tx();
    let cid = put_typed_tx(cas, &tx, 1);
    vec![TapeEventEnvelope {
        logical_t: 1,
        tape_ref: TapeEventRef::L4Accepted {
            head_oid: HEAD.to_string(),
        },
        kind: TapeEventKind::AcceptedTransition,
        payload_cid: Some(cid),
        source_tx_kind: Some(tx.tx_kind()),
    }]
}

#[test]
fn dropping_or_tampering_cache_does_not_change_replay_result() {
    let (_tmp, mut cas) = fresh_cas();
    let events = one_event_tape(&mut cas);
    let direct = derive_economy_projection(&cas, &events).expect("direct replay");
    let mut cache = EconomyProjectionCache::default();

    let cached =
        derive_economy_projection_with_cache(&cas, &events, &mut cache).expect("cached replay");
    assert_eq!(cached.projection, direct);

    let mut dropped_cache = EconomyProjectionCache::default();
    let without_cache = derive_economy_projection_with_cache(&cas, &events, &mut dropped_cache)
        .expect("cache can be deleted");
    assert_eq!(without_cache.projection, direct);

    let mut poisoned = direct.clone();
    poisoned.wallet_balances.insert(
        AgentId("poison".into()),
        MicroCoin::from_micro_units(1_000_000),
    );
    poisoned.derived_from_tape_head = "not-the-current-head".to_string();
    cache.insert(economy_projection_cache_key(HEAD), poisoned);

    let repaired =
        derive_economy_projection_with_cache(&cas, &events, &mut cache).expect("repair replay");
    assert_eq!(repaired.projection, direct);
    assert!(matches!(
        repaired.status,
        EconomyProjectionCacheStatus::FullReplay {
            reason: EconomyProjectionCacheReplayReason::TamperedCurrentEntry
        }
    ));
}

#[test]
fn projection_cache_is_not_predicate_or_settlement_authority() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for rel in [
        "src/runtime/predicate_receipt.rs",
        "src/economy/settlement.rs",
        "src/state/sequencer.rs",
    ] {
        let text = std::fs::read_to_string(workspace.join(rel)).expect("read authority source");
        assert!(
            !text.contains("ProjectionCache"),
            "{rel} must not accept projection cache as predicate/settlement/admission authority"
        );
        assert!(
            !text.contains("derive_economy_projection_with_cache"),
            "{rel} must not derive predicate/settlement/admission truth from the economy cache"
        );
    }
}
