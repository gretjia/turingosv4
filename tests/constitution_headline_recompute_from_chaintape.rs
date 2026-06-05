use std::collections::BTreeMap;

use tempfile::TempDir;

use turingosv4::bottom_white::cas::Cid;
use turingosv4::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
use turingosv4::bottom_white::ledger::transition_ledger::{
    append, Git2LedgerWriter, LedgerEntry, LedgerEntrySigningPayload, LedgerWriter, TxKind,
};
use turingosv4::runtime::projection::{
    derive_l4_events_from_writer, Projection, ProjectionError, ProjectionOutput,
};
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
        tx_payload_cid: Cid::from_content(format!("headline-payload-{logical_t}").as_bytes()),
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
struct HeadlineProjection {
    accepted_transition_count: usize,
    source_head_oid: String,
}

struct AcceptedTransitionHeadline;

impl Projection for AcceptedTransitionHeadline {
    type Output = HeadlineProjection;

    fn projection_id() -> &'static str {
        "a05.accepted-transition-headline"
    }

    fn derive_from_tape(events: &[TapeEventEnvelope]) -> Result<Self::Output, ProjectionError> {
        let last = events.last().ok_or(ProjectionError::EmptyTape)?;
        let mut accepted_transition_count = 0;
        for event in events {
            event.validate()?;
            if event.kind == TapeEventKind::AcceptedTransition {
                accepted_transition_count += 1;
            }
        }
        Ok(HeadlineProjection {
            accepted_transition_count,
            source_head_oid: last.tape_ref.head_oid_hex().to_string(),
        })
    }
}

fn render_headline(output: &ProjectionOutput<HeadlineProjection>) -> String {
    format!(
        "{} accepted_transitions={} source_head_oid={}",
        output.projection_id, output.value.accepted_transition_count, output.source_head_oid
    )
}

#[test]
fn headline_recomputes_from_chaintape_events_not_manifest_text() {
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

    let events = derive_l4_events_from_writer(&writer).expect("derive L4 events");
    let value = AcceptedTransitionHeadline::derive_from_tape(&events).expect("derive headline");
    let output = ProjectionOutput {
        projection_id: AcceptedTransitionHeadline::projection_id().to_string(),
        source_head_oid: value.source_head_oid.clone(),
        value,
    };
    let headline = render_headline(&output);

    assert!(headline.contains("accepted_transitions=2"));
    assert!(
        headline.contains(
            &writer
                .head_commit_oid_hex()
                .expect("git-backed writer exposes head oid")
        ),
        "headline must carry the ChainTape head it was recomputed from"
    );

    let forged_manifest_headline =
        "a05.accepted-transition-headline accepted_transitions=999 source_head_oid=dashboard";
    assert_ne!(
        headline, forged_manifest_headline,
        "manifest/dashboard text cannot override ChainTape-derived headline"
    );
}

#[test]
fn headline_projection_rejects_manifest_only_input() {
    let err = AcceptedTransitionHeadline::derive_from_tape(&[])
        .expect_err("manifest-only proof is empty tape");
    assert!(matches!(err, ProjectionError::EmptyTape));
}
