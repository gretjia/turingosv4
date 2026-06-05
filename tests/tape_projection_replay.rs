use std::collections::BTreeMap;

use tempfile::TempDir;

use turingosv4::bottom_white::cas::Cid;
use turingosv4::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
use turingosv4::bottom_white::ledger::transition_ledger::{
    append, Git2LedgerWriter, LedgerEntry, LedgerEntrySigningPayload, LedgerWriter, TxKind,
};
use turingosv4::runtime::projection::{derive_l4_events_from_writer, Projection, ProjectionError};
use turingosv4::runtime::tape_event::{TapeEventEnvelope, TapeEventKind};
use turingosv4::state::q_state::Hash;

fn h(byte: u8) -> Hash {
    Hash([byte; 32])
}

fn entry_at(
    logical_t: u64,
    parent_state_root: Hash,
    parent_ledger_root: Hash,
    resulting_state_root: Hash,
    kind: TxKind,
) -> LedgerEntry {
    let signing = LedgerEntrySigningPayload {
        logical_t,
        parent_state_root,
        parent_ledger_root,
        tx_kind: kind,
        tx_payload_cid: Cid::from_content(format!("payload-{logical_t}").as_bytes()),
        resulting_state_root,
        timestamp_logical: logical_t,
        epoch: SystemEpoch::new(1),
        extensions: BTreeMap::new(),
    };
    let signing_digest = signing.canonical_digest();
    let resulting_ledger_root = append(&parent_ledger_root, &signing_digest);
    LedgerEntry {
        logical_t: signing.logical_t,
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
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct KindCount {
    work: usize,
    verify: usize,
    source_head_oid: String,
}

struct KindCountProjection;

impl Projection for KindCountProjection {
    type Output = KindCount;

    fn projection_id() -> &'static str {
        "a05.kind-count"
    }

    fn derive_from_tape(events: &[TapeEventEnvelope]) -> Result<Self::Output, ProjectionError> {
        let last = events.last().ok_or(ProjectionError::EmptyTape)?;
        let mut work = 0;
        let mut verify = 0;
        for event in events {
            event.validate()?;
            if event.kind != TapeEventKind::AcceptedTransition {
                return Err(ProjectionError::UnexpectedEventKind {
                    projection_id: Self::projection_id(),
                    logical_t: event.logical_t,
                    kind: event.kind,
                });
            }
            match event.source_tx_kind {
                Some(TxKind::Work) => work += 1,
                Some(TxKind::Verify) => verify += 1,
                _ => {}
            }
        }
        Ok(KindCount {
            work,
            verify,
            source_head_oid: last.tape_ref.head_oid_hex().to_string(),
        })
    }
}

#[test]
fn projection_derives_only_from_chaintape_l4_events() {
    let tmp = TempDir::new().expect("tempdir");
    let mut writer = Git2LedgerWriter::open(tmp.path()).expect("open writer");

    let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1), TxKind::Work);
    writer.commit(&e1).expect("commit work");
    let e2 = entry_at(
        2,
        e1.resulting_state_root,
        e1.resulting_ledger_root,
        h(2),
        TxKind::Verify,
    );
    writer.commit(&e2).expect("commit verify");
    let canonical_head = writer
        .head_commit_oid_hex()
        .expect("git-backed writer exposes head oid");

    let events = derive_l4_events_from_writer(&writer).expect("derive events");
    let projected = KindCountProjection::derive_from_tape(&events).expect("project");

    assert_eq!(events.len(), 2);
    assert_eq!(projected.work, 1);
    assert_eq!(projected.verify, 1);
    assert_eq!(projected.source_head_oid, canonical_head);
}

#[test]
fn projection_rejects_manifest_only_headline_without_tape_events() {
    let err = KindCountProjection::derive_from_tape(&[]).expect_err("empty manifest is not tape");
    assert!(matches!(err, ProjectionError::EmptyTape));
}

#[test]
fn projection_rejects_event_with_stdout_headline_ref() {
    let event = TapeEventEnvelope {
        logical_t: 1,
        tape_ref: turingosv4::runtime::tape_event::TapeEventRef::DerivedView {
            source: "stdout headline: solved=100%".to_string(),
        },
        kind: TapeEventKind::AcceptedTransition,
        payload_cid: Some(Cid::from_content(b"x")),
        source_tx_kind: Some(TxKind::Work),
    };

    let err =
        KindCountProjection::derive_from_tape(&[event]).expect_err("stdout headline is not tape");
    assert!(matches!(err, ProjectionError::TapeEvent(_)));
}
