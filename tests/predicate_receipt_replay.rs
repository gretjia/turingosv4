//! A08 PredicateReceipt replay contract tests.

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::transition_ledger::TxKind;
use turingosv4::runtime::predicate_receipt::{
    derive_predicate_receipt, read_from_cas, write_to_cas, PredicateReceiptError,
    PREDICATE_RECEIPT_SCHEMA_ID,
};
use turingosv4::runtime::tape_event::{TapeEventEnvelope, TapeEventKind, TapeEventRef};
use turingosv4::state::q_state::{Hash, TxId};
use turingosv4::state::typed_tx::PredicateId;

const HEAD: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn accepted_event(logical_t: u64, verdict_cid: Cid) -> TapeEventEnvelope {
    TapeEventEnvelope {
        logical_t,
        tape_ref: TapeEventRef::L4Accepted {
            head_oid: HEAD.to_string(),
        },
        kind: TapeEventKind::AcceptedTransition,
        payload_cid: Some(verdict_cid),
        source_tx_kind: Some(TxKind::Work),
    }
}

#[test]
fn predicate_receipt_is_generic_cas_schema_and_round_trips() {
    let tmp = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(tmp.path()).expect("cas");
    let input_cid = Cid::from_content(b"predicate input view");
    let verdict_cid = Cid::from_content(b"predicate verdict bytes");
    let event = accepted_event(3, verdict_cid);

    let receipt = derive_predicate_receipt(
        &event,
        PredicateId("lean_artifact_v1".into()),
        TxId("work-42".into()),
        input_cid,
        verdict_cid,
        Hash([9; 32]),
        true,
    )
    .expect("canonical tape event derives receipt");

    let receipt_cid = write_to_cas(&mut cas, &receipt, "a08-test").expect("write receipt");
    assert_eq!(
        cas.list_cids_by_schema_id(PREDICATE_RECEIPT_SCHEMA_ID),
        vec![receipt_cid],
        "PredicateReceipt must use Generic CAS + schema_id, not a new ObjectType"
    );
    let decoded = read_from_cas(&cas, &receipt_cid).expect("read receipt");
    assert_eq!(decoded, receipt);
}

#[test]
fn predicate_receipt_rejects_manifest_or_derived_view_sources() {
    let verdict_cid = Cid::from_content(b"verdict");
    let event = TapeEventEnvelope {
        logical_t: 1,
        tape_ref: TapeEventRef::DerivedView {
            source: "dashboard".into(),
        },
        kind: TapeEventKind::AcceptedTransition,
        payload_cid: Some(verdict_cid),
        source_tx_kind: Some(TxKind::Work),
    };

    let err = derive_predicate_receipt(
        &event,
        PredicateId("lean_artifact_v1".into()),
        TxId("work-42".into()),
        Cid::from_content(b"input"),
        verdict_cid,
        Hash([1; 32]),
        false,
    )
    .expect_err("derived views cannot become predicate receipts");

    assert!(matches!(err, PredicateReceiptError::TapeEvent(_)));
}
