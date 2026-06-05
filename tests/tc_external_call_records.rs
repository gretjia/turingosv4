use tempfile::TempDir;

use turingosv4::bottom_white::cas::{CasStore, ObjectType};
use turingosv4::runtime::external_call::{
    execute_external_call, retry_request_from_intent, ExternalCallProvider,
    ExternalCallProviderContext, ExternalCallProviderResult, ExternalCallRecorder,
    ExternalCallRequest, ExternalCallState, ExternalCallStatus, EXTERNAL_CALL_INTENT_SCHEMA_ID,
    EXTERNAL_CALL_TERMINAL_SCHEMA_ID,
};
use turingosv4::runtime::tape_event::TapeEventKind;

fn head_oid() -> String {
    "0123456789abcdef0123456789abcdef01234567".to_string()
}

fn cas() -> (TempDir, CasStore) {
    let dir = TempDir::new().expect("tempdir");
    let store = CasStore::open(dir.path()).expect("cas open");
    (dir, store)
}

fn request_cid(cas: &mut CasStore, logical_t: u64) -> turingosv4::bottom_white::cas::Cid {
    cas.put(
        br#"{"messages":[{"role":"user","content":"hello"}]}"#,
        ObjectType::Generic,
        "a06-test-request",
        logical_t,
        Some("turingos.external_call.request.fixture.v1".to_string()),
    )
    .expect("request cas put")
}

fn request(
    cas: &mut CasStore,
    call_id: &str,
    idempotency_key: &str,
    logical_t: u64,
) -> ExternalCallRequest {
    ExternalCallRequest {
        call_id: call_id.to_string(),
        provider: "siliconflow".to_string(),
        operation: "chat.completions".to_string(),
        request_cid: request_cid(cas, logical_t),
        idempotency_key: idempotency_key.to_string(),
        provider_supports_idempotency: true,
    }
}

#[derive(Debug)]
struct InspectingProvider {
    result: ExternalCallProviderResult,
    calls: usize,
    observed_pending_event_before_call: bool,
}

impl ExternalCallProvider for InspectingProvider {
    fn call(
        &mut self,
        intent: &turingosv4::runtime::external_call::ExternalCallIntent,
        context: &ExternalCallProviderContext,
    ) -> ExternalCallProviderResult {
        self.calls += 1;
        assert_eq!(intent.call_id, "call-success");
        assert_eq!(context.record_count_before_provider, 1);
        assert_eq!(context.intent_event.kind, TapeEventKind::PendingIntent);
        assert_eq!(context.intent_event.payload_cid, Some(context.intent_cid));
        self.observed_pending_event_before_call = true;
        self.result.clone()
    }
}

#[test]
fn successful_provider_call_records_intent_before_terminal() {
    let (_dir, mut cas) = cas();
    let mut recorder = ExternalCallRecorder::new(head_oid());
    let req = request(&mut cas, "call-success", "idem-success", 1);
    let mut provider = InspectingProvider {
        result: ExternalCallProviderResult::Success {
            response_bytes: b"{\"ok\":true}".to_vec(),
        },
        calls: 0,
        observed_pending_event_before_call: false,
    };

    let execution = execute_external_call(&mut cas, &mut recorder, &req, "a06-test", &mut provider)
        .expect("execute external call");
    let state = ExternalCallState::derive_from_tape(recorder.records()).expect("derive state");
    let terminal = state.terminal_for("call-success").expect("terminal exists");

    assert_eq!(provider.calls, 1);
    assert!(provider.observed_pending_event_before_call);
    assert_eq!(recorder.records().len(), 2);
    assert!(state.clean_halt_allowed());
    assert_eq!(terminal.status, ExternalCallStatus::Succeeded);
    assert_eq!(terminal.response_cid, execution.response_cid);
    assert_eq!(
        cas.metadata(&execution.intent_cid)
            .and_then(|m| m.schema_id.as_deref()),
        Some(EXTERNAL_CALL_INTENT_SCHEMA_ID)
    );
    assert_eq!(
        cas.metadata(&execution.terminal_cid)
            .and_then(|m| m.schema_id.as_deref()),
        Some(EXTERNAL_CALL_TERMINAL_SCHEMA_ID)
    );
}

#[test]
fn provider_errors_and_timeouts_still_close_with_terminal_events() {
    let cases = [
        (
            ExternalCallProviderResult::Failed {
                error_class: "provider_500".to_string(),
                may_have_spent: false,
            },
            ExternalCallStatus::Failed,
            "provider_500",
        ),
        (
            ExternalCallProviderResult::TimedOut {
                error_class: "timeout".to_string(),
                may_have_spent: true,
            },
            ExternalCallStatus::TimedOut,
            "timeout",
        ),
    ];

    for (result, expected_status, expected_error) in cases {
        let (_dir, mut cas) = cas();
        let mut recorder = ExternalCallRecorder::new(head_oid());
        let req = request(&mut cas, "call-success", "idem-error", 1);
        let mut provider = InspectingProvider {
            result,
            calls: 0,
            observed_pending_event_before_call: false,
        };

        execute_external_call(&mut cas, &mut recorder, &req, "a06-test", &mut provider)
            .expect("execute external call");
        let state = ExternalCallState::derive_from_tape(recorder.records()).expect("derive state");
        let terminal = state.terminal_for("call-success").expect("terminal exists");

        assert_eq!(provider.calls, 1);
        assert!(state.clean_halt_allowed());
        assert_eq!(terminal.status, expected_status);
        assert_eq!(terminal.error_class.as_deref(), Some(expected_error));
    }
}

#[test]
fn idempotent_retry_reuses_logical_call_id_and_idempotency_key() {
    let (_dir, mut cas) = cas();
    let mut recorder = ExternalCallRecorder::new(head_oid());
    let req = request(&mut cas, "retry-call", "idem-retry-1", 1);
    let recorded = recorder
        .record_intent(&mut cas, &req, "a06-test")
        .expect("record intent");

    let retry = retry_request_from_intent(&recorded.intent).expect("retry request");

    assert_eq!(retry.call_id, recorded.intent.call_id);
    assert_eq!(retry.idempotency_key, recorded.intent.idempotency_key);
    assert_eq!(retry.request_cid, recorded.intent.request_cid);
}
