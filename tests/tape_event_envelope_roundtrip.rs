use turingosv4::bottom_white::cas::Cid;
use turingosv4::bottom_white::ledger::transition_ledger::{
    canonical_decode, canonical_encode, TxKind,
};
use turingosv4::runtime::tape_event::{
    TapeEventEnvelope, TapeEventError, TapeEventKind, TapeEventRef,
};

#[test]
fn tape_event_envelope_canonical_roundtrip_is_stable() {
    let event = TapeEventEnvelope {
        logical_t: 7,
        tape_ref: TapeEventRef::L4Accepted {
            head_oid: "0123456789abcdef0123456789abcdef01234567".to_string(),
        },
        kind: TapeEventKind::AcceptedTransition,
        payload_cid: Some(Cid::from_content(b"opaque work payload")),
        source_tx_kind: Some(TxKind::Work),
    };

    let bytes = canonical_encode(&event).expect("canonical encode event");
    let decoded: TapeEventEnvelope = canonical_decode(&bytes).expect("canonical decode event");

    assert_eq!(decoded, event);
    assert_eq!(
        bytes,
        canonical_encode(&decoded).expect("re-encode decoded event"),
        "canonical roundtrip must be byte-stable"
    );
}

#[test]
fn tape_event_kind_is_generic_not_market_specific() {
    let kinds = TapeEventKind::all();

    assert!(kinds.contains(&TapeEventKind::AcceptedTransition));
    assert!(kinds.contains(&TapeEventKind::RejectedTransition));
    assert!(kinds.contains(&TapeEventKind::PendingIntent));
    assert!(kinds.contains(&TapeEventKind::TerminalExternalCall));

    let serialized = serde_json::to_string(&kinds).expect("serialize kinds");
    for forbidden in [
        "market",
        "wallet",
        "price",
        "softmax",
        "router",
        "scheduler",
    ] {
        assert!(
            !serialized.to_ascii_lowercase().contains(forbidden),
            "generic TapeEventKind must not smuggle {forbidden} policy"
        );
    }
}

#[test]
fn tape_event_rejects_non_canonical_head_oid() {
    let event = TapeEventEnvelope {
        logical_t: 1,
        tape_ref: TapeEventRef::L4Accepted {
            head_oid: "not-a-git-oid".to_string(),
        },
        kind: TapeEventKind::AcceptedTransition,
        payload_cid: Some(Cid::from_content(b"x")),
        source_tx_kind: Some(TxKind::Work),
    };

    let err = event.validate().expect_err("bad oid must fail closed");
    assert!(matches!(err, TapeEventError::InvalidGitOid { .. }));
}

#[test]
fn accepted_transition_requires_payload_and_tx_kind() {
    let event = TapeEventEnvelope {
        logical_t: 1,
        tape_ref: TapeEventRef::L4Accepted {
            head_oid: "0123456789abcdef0123456789abcdef01234567".to_string(),
        },
        kind: TapeEventKind::AcceptedTransition,
        payload_cid: None,
        source_tx_kind: None,
    };

    let err = event
        .validate()
        .expect_err("accepted L4 transition must keep opaque payload link");
    assert!(matches!(err, TapeEventError::MissingAcceptedPayload { .. }));
}
