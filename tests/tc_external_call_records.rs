use turingosv4::drivers::llm_http::{
    recorded_generate_with_transport, DriverError, GenerateRequest, GenerateResponse, Message,
    MockLlmTransport, RecordedLlmClient,
};
use turingosv4::runtime::external_call::{
    ExternalCallCrashState, ExternalCallIntent, ExternalCallLedger, ExternalCallOutbox,
    ExternalCallTerminal, Usage,
};
use turingosv4::runtime::tc_tape_canonical::{TapeAnchor, TapeCanonicalError};

fn intent(id: &str) -> ExternalCallIntent {
    ExternalCallIntent {
        intent_id: id.to_string(),
        logical_call_id: format!("logical-{id}"),
        call_site: "lean_market_agent:proof".to_string(),
        run_id: "run-tc".to_string(),
        request_hash: format!("request-hash-{id}"),
        provider: "mock-provider".to_string(),
        model: Some("mock-model".to_string()),
        redacted_request_cid: format!("cid-{id}"),
        idempotency_key: format!("idem-{id}"),
        timeout_ms: 30_000,
        logical_t: 7,
    }
}

fn anchor() -> TapeAnchor {
    TapeAnchor {
        run_id: "run-tc".to_string(),
        logical_t: Some(7),
        submit_id: None,
        head_ref: "refs/chaintape/l4".to_string(),
        head_oid: "0123456789abcdef0123456789abcdef01234567".to_string(),
    }
}

#[test]
fn terminal_count_invariant_holds_for_clean_ledger() {
    let mut ledger = ExternalCallLedger::default();
    ledger.record_intent(intent("a")).unwrap();
    ledger
        .record_terminal(
            "a",
            ExternalCallTerminal::Result {
                result_hash: "result-hash-a".to_string(),
                usage: Usage {
                    prompt_tokens: 11,
                    completion_tokens: 5,
                    total_tokens: 16,
                },
                status: 200,
                provider_request_id: Some("provider-a".to_string()),
            },
        )
        .unwrap();

    ledger.record_intent(intent("b")).unwrap();
    ledger
        .record_terminal(
            "b",
            ExternalCallTerminal::Failure {
                class: "http_429".to_string(),
                retryable: true,
                public_summary: "rate limited".to_string(),
            },
        )
        .unwrap();

    ledger.record_intent(intent("c")).unwrap();
    ledger
        .record_terminal(
            "c",
            ExternalCallTerminal::Abandoned {
                reason: "provider_2xx_client_write_broke".to_string(),
                may_have_spent: true,
            },
        )
        .unwrap();

    let summary = ledger.summary();
    assert_eq!(summary.intent_count, 3);
    assert_eq!(summary.result_count, 1);
    assert_eq!(summary.failure_count, 1);
    assert_eq!(summary.abandoned_count, 1);
    assert_eq!(summary.pending_count, 0);
    assert!(summary.clean_claim_allowed);
}

#[test]
fn pending_or_duplicate_terminal_blocks_clean_claim() {
    let mut ledger = ExternalCallLedger::default();
    ledger.record_intent(intent("a")).unwrap();
    ledger.record_intent(intent("b")).unwrap();
    ledger
        .record_terminal(
            "a",
            ExternalCallTerminal::Failure {
                class: "parse_fail".to_string(),
                retryable: false,
                public_summary: "invalid JSON".to_string(),
            },
        )
        .unwrap();

    let summary = ledger.summary();
    assert_eq!(summary.intent_count, 2);
    assert_eq!(summary.pending_count, 1);
    assert!(!summary.clean_claim_allowed);
    assert!(ledger.assert_clean_halt().is_err());

    let err = ledger
        .record_terminal(
            "a",
            ExternalCallTerminal::Abandoned {
                reason: "second terminal".to_string(),
                may_have_spent: false,
            },
        )
        .unwrap_err();
    assert!(err.contains("already has terminal"));
}

#[test]
fn outbox_reopens_and_preserves_pending_records() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("external_calls.jsonl");

    let mut outbox = ExternalCallOutbox::open(&path).expect("new outbox opens");
    outbox.append_intent(intent("pending")).unwrap();

    let reopened = ExternalCallOutbox::open(&path).expect("outbox reopens");
    let summary = reopened.ledger().summary();
    assert_eq!(summary.intent_count, 1);
    assert_eq!(summary.pending_count, 1);
    assert!(!summary.clean_claim_allowed);

    let mut reopened = reopened;
    let duplicate = reopened.append_intent(intent("pending")).unwrap_err();
    assert!(duplicate.contains("already exists"));

    std::fs::write(&path, "{not-json}\n").unwrap();
    let malformed = ExternalCallOutbox::open(&path).unwrap_err();
    assert!(malformed.contains("malformed external-call JSONL"));
}

#[test]
fn crash_states_map_to_deterministic_terminals() {
    assert_eq!(
        ExternalCallTerminal::from_crash_state(ExternalCallCrashState::IntentBeforeSend),
        ExternalCallTerminal::Abandoned {
            reason: "intent_durable_before_send".to_string(),
            may_have_spent: false
        }
    );
    assert_eq!(
        ExternalCallTerminal::from_crash_state(ExternalCallCrashState::SentNoTerminal),
        ExternalCallTerminal::Abandoned {
            reason: "send_marker_without_terminal".to_string(),
            may_have_spent: true
        }
    );

    let timeout = ExternalCallTerminal::from_crash_state(ExternalCallCrashState::HttpTimeout);
    assert!(matches!(
        timeout,
        ExternalCallTerminal::Failure {
            retryable: true,
            ..
        }
    ));

    let parse_fail =
        ExternalCallTerminal::from_crash_state(ExternalCallCrashState::ParseFailAfterResponse);
    assert!(matches!(
        parse_fail,
        ExternalCallTerminal::Failure {
            retryable: false,
            ..
        }
    ));

    let success = ExternalCallTerminal::from_crash_state(ExternalCallCrashState::ParsedSuccess {
        result_hash: "result-hash".to_string(),
        usage: Usage {
            prompt_tokens: 1,
            completion_tokens: 2,
            total_tokens: 3,
        },
        status: 200,
        provider_request_id: Some("provider-ok".to_string()),
    });
    assert!(matches!(
        success,
        ExternalCallTerminal::Result { status: 200, .. }
    ));
}

#[test]
fn llm_call_fact_has_redacted_request_cid_and_no_raw_prompt() {
    let record = turingosv4::runtime::external_call::ExternalCallRecord {
        intent: intent("llm"),
        terminal: Some(ExternalCallTerminal::Result {
            result_hash: "result-hash-llm".to_string(),
            usage: Usage {
                prompt_tokens: 13,
                completion_tokens: 8,
                total_tokens: 21,
            },
            status: 200,
            provider_request_id: Some("provider-req-1".to_string()),
        }),
    };

    let fact = record
        .llm_call_fact(anchor(), "llm call result recorded")
        .expect("result terminal can produce public LLM fact");

    assert_eq!(fact.fact.kind, "llm_call");
    assert_eq!(fact.fact.anchor.run_id, "run-tc");
    assert_eq!(fact.request_hash, "request-hash-llm");
    assert_eq!(fact.redacted_request_cid, "cid-llm");
    assert_eq!(fact.result_hash, "result-hash-llm");
    assert_eq!(fact.usage.total_tokens, 21);

    let serialized = serde_json::to_string(&fact).expect("serializes");
    assert!(serialized.contains("redacted_request_cid"));
    assert!(serialized.contains("request_hash"));
    assert!(serialized.contains("result_hash"));
    assert!(!serialized.contains("private prompt body"));
    assert!(!serialized.contains("provider response body"));

    let mut leaky = record.clone();
    leaky.intent.provider = ["Authori", "zation"].concat();
    let err = leaky
        .llm_call_fact(anchor(), "llm call result recorded")
        .expect_err("credential marker must not serialize as public fact");
    assert_eq!(err, TapeCanonicalError::UnshieldedPublicSummary);

    let mut prompt_leak = record;
    prompt_leak.intent.redacted_request_cid = "raw prompt body".to_string();
    let err = prompt_leak
        .llm_call_fact(anchor(), "llm call result recorded")
        .expect_err("raw prompt marker must not serialize as public fact");
    assert_eq!(err, TapeCanonicalError::UnshieldedPublicSummary);
}

#[test]
fn recorded_llm_client_writes_intent_before_mock_send_and_terminal_after() {
    let dir = tempfile::tempdir().expect("tempdir");
    let outbox_path = dir.path().join("external-call.jsonl");
    let mut outbox = ExternalCallOutbox::open(&outbox_path).expect("open outbox");
    let request = GenerateRequest {
        model: "mock-model".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: "private prompt body must be hashed".to_string(),
        }],
        temperature: Some(0.0),
        max_tokens: Some(16),
    };
    let mut transport = MockLlmTransport::new(GenerateResponse {
        content: "ok".to_string(),
        completion_tokens: 3,
        prompt_tokens: 5,
        model: "mock-model".to_string(),
    });

    let record = recorded_generate_with_transport(
        &mut outbox,
        &mut transport,
        "run-tc",
        "proof-step",
        "mock-provider",
        &request,
    )
    .expect("mock transport call records");

    assert!(transport.send_seen_intent);
    assert_eq!(transport.send_count, 1);
    assert_eq!(outbox.ledger().summary().intent_count, 1);
    assert_eq!(outbox.ledger().summary().result_count, 1);
    assert_eq!(outbox.ledger().summary().pending_count, 0);
    assert!(outbox.ledger().summary().clean_claim_allowed);

    let reopened = ExternalCallOutbox::open(&outbox_path).expect("reopen durable outbox");
    assert_eq!(reopened.ledger().summary().intent_count, 1);
    assert_eq!(reopened.ledger().summary().result_count, 1);
    assert_eq!(reopened.ledger().summary().pending_count, 0);
    assert!(reopened.ledger().summary().clean_claim_allowed);
    assert_eq!(record.intent.model.as_deref(), Some("mock-model"));
    assert_eq!(record.intent.provider, "mock-provider");
    assert_ne!(
        record.intent.request_hash,
        "private prompt body must be hashed"
    );
    assert!(record
        .intent
        .redacted_request_cid
        .starts_with("redacted-request:"));
    assert!(matches!(
        record.terminal,
        Some(ExternalCallTerminal::Result { status: 200, .. })
    ));
}

#[test]
fn recorded_llm_client_writes_failure_terminal_on_transport_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    let outbox_path = dir.path().join("external-call.jsonl");
    let mut outbox = ExternalCallOutbox::open(&outbox_path).expect("open outbox");
    let request = GenerateRequest {
        model: "mock-model".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: "private prompt body must be hashed".to_string(),
        }],
        temperature: Some(0.0),
        max_tokens: Some(16),
    };
    let mut transport = MockLlmTransport::failing(DriverError::NetworkError(
        "simulated transport drop".to_string(),
    ));

    let err = recorded_generate_with_transport(
        &mut outbox,
        &mut transport,
        "run-tc",
        "proof-step",
        "mock-provider",
        &request,
    )
    .expect_err("transport error returns driver error");

    assert!(matches!(err, DriverError::NetworkError(_)));
    assert!(transport.send_seen_intent);
    assert_eq!(transport.send_count, 1);
    assert_eq!(outbox.ledger().summary().intent_count, 1);
    assert_eq!(outbox.ledger().summary().failure_count, 1);
    assert_eq!(outbox.ledger().summary().pending_count, 0);
    assert!(outbox.ledger().summary().clean_claim_allowed);

    let reopened = ExternalCallOutbox::open(&outbox_path).expect("reopen durable outbox");
    assert_eq!(reopened.ledger().summary().intent_count, 1);
    assert_eq!(reopened.ledger().summary().failure_count, 1);
    assert_eq!(reopened.ledger().summary().pending_count, 0);
    assert!(reopened.ledger().summary().clean_claim_allowed);
}

#[tokio::test]
async fn recorded_production_client_writes_failure_terminal_on_transport_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    let outbox_path = dir.path().join("external-call.jsonl");
    let request = GenerateRequest {
        model: "mock-model".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: "private prompt body must be hashed".to_string(),
        }],
        temperature: Some(0.0),
        max_tokens: Some(16),
    };
    let client = RecordedLlmClient::with_outbox(
        "http://127.0.0.1:1",
        1,
        0,
        &outbox_path,
        "run-tc",
        "local-proxy",
    );

    let err = client
        .generate_recorded_at("proof-step", &request)
        .await
        .expect_err("closed local port must fail");

    assert!(matches!(
        err,
        DriverError::NetworkError(_) | DriverError::Timeout
    ));
    let reopened = ExternalCallOutbox::open(&outbox_path).expect("reopen durable outbox");
    let summary = reopened.ledger().summary();
    assert_eq!(summary.intent_count, 1);
    assert_eq!(summary.failure_count, 1);
    assert_eq!(summary.pending_count, 0);
    assert!(summary.clean_claim_allowed);
}

#[test]
fn production_bins_use_recorded_llm_client_not_direct_generate() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/bin");
    let mut offenders = Vec::new();
    for entry in std::fs::read_dir(&root).expect("read src/bin") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|x| x.to_str()) != Some("rs") {
            continue;
        }
        let body = std::fs::read_to_string(&path).expect("read bin");
        if body.contains(".generate(") || body.contains("ResilientLLMClient::new(") {
            offenders.push(path.display().to_string());
        }
    }
    assert!(
        offenders.is_empty(),
        "production LLM bins must use RecordedLlmClient::generate_recorded: {offenders:?}"
    );
}
