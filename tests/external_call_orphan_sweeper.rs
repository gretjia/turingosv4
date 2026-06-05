use tempfile::TempDir;

use turingosv4::bottom_white::cas::{CasStore, ObjectType};
use turingosv4::runtime::external_call::{
    ExternalCallRecorder, ExternalCallRequest, ExternalCallState, ExternalCallStatus,
};
use turingosv4::runtime::orphan_intent_sweeper::{
    sweep_orphan_external_call_intents, OrphanSweepConfig, OS_CRASH_RECOVERY,
};

fn head_oid() -> String {
    "89abcdef0123456789abcdef0123456789abcdef".to_string()
}

fn cas() -> (TempDir, CasStore) {
    let dir = TempDir::new().expect("tempdir");
    let store = CasStore::open(dir.path()).expect("cas open");
    (dir, store)
}

fn request(cas: &mut CasStore) -> ExternalCallRequest {
    let request_cid = cas
        .put(
            b"request bytes",
            ObjectType::Generic,
            "a06-orphan-request",
            1,
            Some("turingos.external_call.request.fixture.v1".to_string()),
        )
        .expect("request cas put");
    ExternalCallRequest {
        call_id: "orphan-call".to_string(),
        provider: "siliconflow".to_string(),
        operation: "chat.completions".to_string(),
        request_cid,
        idempotency_key: "idem-orphan".to_string(),
        provider_supports_idempotency: true,
    }
}

#[test]
fn stale_unclosed_intent_appends_abandoned_terminal_without_reissuing_provider_call() {
    let (_dir, mut cas) = cas();
    let mut recorder = ExternalCallRecorder::new(head_oid());
    let req = request(&mut cas);
    recorder
        .record_intent(&mut cas, &req, "a06-test")
        .expect("record orphan intent");
    let provider_calls_that_already_happened = 1usize;

    let report = sweep_orphan_external_call_intents(
        &mut cas,
        &mut recorder,
        OrphanSweepConfig {
            stale_at_or_before_logical_t: 1,
            may_have_spent: true,
            creator: "boot-orphan-sweeper".to_string(),
        },
    )
    .expect("sweep orphans");
    let state = ExternalCallState::derive_from_tape(recorder.records()).expect("derive state");
    let terminal = state.terminal_for("orphan-call").expect("terminal exists");

    assert_eq!(provider_calls_that_already_happened, 1);
    assert_eq!(report.abandoned_call_ids, vec!["orphan-call".to_string()]);
    assert!(state.clean_halt_allowed());
    assert_eq!(terminal.status, ExternalCallStatus::Abandoned);
    assert_eq!(terminal.error_class.as_deref(), Some(OS_CRASH_RECOVERY));
    assert!(terminal.may_have_spent);
}

#[test]
fn memory_only_cleanup_does_not_close_pending_intent() {
    let (_dir, mut cas) = cas();
    let mut recorder = ExternalCallRecorder::new(head_oid());
    let req = request(&mut cas);
    recorder
        .record_intent(&mut cas, &req, "a06-test")
        .expect("record orphan intent");

    let state = ExternalCallState::derive_from_tape(recorder.records()).expect("derive state");

    assert!(!state.clean_halt_allowed());
    assert_eq!(state.pending_call_ids(), vec!["orphan-call".to_string()]);
}
