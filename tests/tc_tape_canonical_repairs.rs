use tempfile::TempDir;

use turingosv4::bottom_white::cas::{CasStore, ObjectType};
use turingosv4::runtime::external_call::{
    ExternalCallError, ExternalCallRecorder, ExternalCallRequest, ExternalCallState,
};
use turingosv4::runtime::tape_event::{TapeEventEnvelope, TapeEventKind, TapeEventRef};

fn head_oid() -> String {
    "abcdef0123456789abcdef0123456789abcdef01".to_string()
}

fn cas() -> (TempDir, CasStore) {
    let dir = TempDir::new().expect("tempdir");
    let store = CasStore::open(dir.path()).expect("cas open");
    (dir, store)
}

fn request(cas: &mut CasStore, call_id: &str) -> ExternalCallRequest {
    let request_cid = cas
        .put(
            b"request bytes",
            ObjectType::Generic,
            "a06-canonical-request",
            1,
            Some("turingos.external_call.request.fixture.v1".to_string()),
        )
        .expect("request cas put");
    ExternalCallRequest {
        call_id: call_id.to_string(),
        provider: "siliconflow".to_string(),
        operation: "chat.completions".to_string(),
        request_cid,
        idempotency_key: format!("idem-{call_id}"),
        provider_supports_idempotency: true,
    }
}

#[test]
fn lone_intent_blocks_clean_halt_until_terminal_lands() {
    let (_dir, mut cas) = cas();
    let mut recorder = ExternalCallRecorder::new(head_oid());
    let req = request(&mut cas, "pending-call");

    recorder
        .record_intent(&mut cas, &req, "a06-test")
        .expect("record intent");
    let state = ExternalCallState::derive_from_tape(recorder.records()).expect("derive state");

    assert!(!state.clean_halt_allowed());
    assert_eq!(
        state.require_clean_halt(),
        Err(ExternalCallError::PendingIntents)
    );
}

#[test]
fn terminal_event_without_matching_intent_is_rejected() {
    let (_dir, mut cas) = cas();
    let mut recorder = ExternalCallRecorder::new(head_oid());
    let req = request(&mut cas, "terminal-only");
    let intent = recorder
        .record_intent(&mut cas, &req, "a06-test")
        .expect("record intent")
        .intent;
    recorder
        .record_abandoned_terminal(&mut cas, &intent, "OS_CRASH_RECOVERY", true, "a06-test")
        .expect("record terminal");

    let terminal_only = vec![recorder.records()[1].clone()];
    let err = ExternalCallState::derive_from_tape(&terminal_only).expect_err("must reject");

    assert_eq!(
        err,
        ExternalCallError::TerminalWithoutIntent {
            call_id: "terminal-only".to_string()
        }
    );
}

#[test]
fn derived_view_refs_cannot_impersonate_external_call_tape_events() {
    let event = TapeEventEnvelope {
        logical_t: 1,
        tape_ref: TapeEventRef::DerivedView {
            source: "manifest-only".to_string(),
        },
        kind: TapeEventKind::PendingIntent,
        payload_cid: None,
        source_tx_kind: None,
    };

    assert!(event.validate().is_err());
}
