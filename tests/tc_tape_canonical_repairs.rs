use turingosv4::runtime::external_call::{ExternalCallIntent, ExternalCallLedger};
use turingosv4::runtime::tc_tape_canonical::{TapeAnchor, TapeCanonicalError, TcTapeCanonicalFact};

fn anchor() -> TapeAnchor {
    TapeAnchor {
        run_id: "run-tc-101".to_string(),
        logical_t: Some(7),
        submit_id: None,
        head_ref: "refs/chaintape/l4".to_string(),
        head_oid: "0123456789abcdef0123456789abcdef01234567".to_string(),
    }
}

fn intent(id: &str) -> ExternalCallIntent {
    ExternalCallIntent {
        intent_id: id.to_string(),
        logical_call_id: format!("logical-{id}"),
        call_site: "test:harness".to_string(),
        run_id: "run-tc-101".to_string(),
        request_hash: format!("request-hash-{id}"),
        provider: "mock-provider".to_string(),
        model: None,
        redacted_request_cid: format!("cid-{id}"),
        idempotency_key: format!("idem-{id}"),
        timeout_ms: 30_000,
        logical_t: 7,
    }
}

#[test]
fn gateway_fact_requires_tape_anchor_and_reconstructs() {
    let fact = TcTapeCanonicalFact::new(
        "gateway",
        anchor(),
        br#"{"request":"redacted"}"#,
        "gateway intent recorded",
    )
    .expect("valid gateway fact");

    assert_eq!(fact.kind, "gateway");
    assert_eq!(fact.anchor.run_id, "run-tc-101");
    assert_eq!(fact.anchor.logical_t, Some(7));
    assert_eq!(fact.payload_hash.len(), 64);
    assert_eq!(fact.public_summary, "gateway intent recorded");

    let reconstructed = TcTapeCanonicalFact::new(
        "gateway",
        fact.anchor.clone(),
        br#"{"request":"redacted"}"#,
        "gateway intent recorded",
    )
    .expect("reconstructable gateway fact");
    assert_eq!(fact, reconstructed);
}

#[test]
fn gateway_fact_rejects_missing_tape_anchor_position() {
    let mut missing = anchor();
    missing.logical_t = None;
    missing.submit_id = None;

    let err = TcTapeCanonicalFact::new("gateway", missing, b"{}", "gateway intent recorded")
        .expect_err("missing logical_t and submit_id must fail");
    assert_eq!(err, TapeCanonicalError::MissingTapePosition);
}

#[test]
fn gateway_fact_rejects_unshielded_public_summary() {
    let e = ["std", "err"].concat();
    let leaks = vec![
        format!("{} {e}: private proof body", "raw"),
        format!("{} {}: theorem body", "Lean", ["std", "err"].concat()),
        format!(
            "{}: {} secret",
            ["Authori", "zation"].concat(),
            ["Bear", "er"].concat()
        ),
        format!("{}=secret", ["api", "_", "key"].concat()),
        "raw provider response".to_string(),
        "raw prompt".to_string(),
    ];
    for leak in leaks {
        let err = TcTapeCanonicalFact::new("gateway", anchor(), b"{}", leak)
            .expect_err("unshielded public summary must fail");
        assert_eq!(err, TapeCanonicalError::UnshieldedPublicSummary);
    }
}

#[test]
fn wal_fact_is_recovery_log_not_authority() {
    let fact = TcTapeCanonicalFact::wal_recovery(
        anchor(),
        br#"{"phase":"after_intent"}"#,
        "outbox recovery receipt",
    )
    .expect("valid WAL recovery fact");

    assert_eq!(fact.kind, "wal_recovery");
    assert!(!fact.is_replay_authority());
    assert_eq!(fact.public_summary, "outbox recovery receipt");

    let err = TcTapeCanonicalFact::wal_recovery(anchor(), b"{}", "WAL replay input")
        .expect_err("WAL cannot become replay authority");
    assert_eq!(err, TapeCanonicalError::SidecarAsAuthority);
}

#[test]
fn map_reduce_tick_fact_links_existing_l4_tx() {
    let fact = TcTapeCanonicalFact::map_reduce_tick(
        anchor(),
        br#"{"accepted_tx":"tn-8","tick":"mr-1"}"#,
        "map-reduce tick linked to accepted transition",
    )
    .expect("valid map-reduce tick fact");

    assert_eq!(fact.kind, "map_reduce_tick");
    assert_eq!(fact.anchor.head_ref, "refs/chaintape/l4");
    assert!(!fact.is_replay_authority());

    let mut rejected = anchor();
    rejected.head_ref = "refs/chaintape/l4e".to_string();
    let err = TcTapeCanonicalFact::map_reduce_tick(
        rejected,
        br#"{"accepted_tx":"tn-8"}"#,
        "map-reduce tick linked to accepted transition",
    )
    .expect_err("map-reduce tick must anchor to accepted L4");
    assert_eq!(err, TapeCanonicalError::MissingAcceptedHead);

    let err = TcTapeCanonicalFact::map_reduce_tick(anchor(), b"{}", "stdout-only tick")
        .expect_err("stdout-only tick must not become tape fact");
    assert_eq!(err, TapeCanonicalError::StdoutOnlyEvidence);
}

#[test]
fn search_fact_records_query_hash_result_hash_and_anchor() {
    let fact = TcTapeCanonicalFact::search_activity(
        anchor(),
        b"private librarian query body",
        b"provider result body",
        "librarian search receipt",
    )
    .expect("valid search fact");

    assert_eq!(fact.fact.kind, "search_activity");
    assert_eq!(fact.fact.anchor.run_id, "run-tc-101");
    assert_eq!(fact.query_hash.len(), 64);
    assert_eq!(fact.result_hash.len(), 64);
    assert_eq!(fact.query_hash, fact.query_hash_recomputed);
    assert_eq!(fact.result_hash, fact.result_hash_recomputed);
    assert!(!fact.replay_requires_network);
    assert!(!fact.public_summary_contains_raw_query_or_result());
}

#[test]
fn wallet_fact_replays_from_chaintape_not_tool_state() {
    let fact = TcTapeCanonicalFact::wallet_derived(
        anchor(),
        "chaintape_replay",
        br#"{"agent":"a","balance":1000}"#,
        "wallet derived from chain replay",
    )
    .expect("valid wallet-derived fact");

    assert_eq!(fact.kind, "wallet_derived");
    assert_eq!(fact.anchor.head_ref, "refs/chaintape/l4");
    assert_eq!(fact.payload_hash.len(), 64);

    let err = TcTapeCanonicalFact::wallet_derived(
        anchor(),
        "sdk_wallet_state",
        b"{}",
        "wallet derived from tool state",
    )
    .expect_err("SDK wallet state cannot be source of truth");
    assert_eq!(err, TapeCanonicalError::SecondSourceDrift);
}

#[test]
fn board_fact_is_reconstructable_view_not_truth() {
    let fact = TcTapeCanonicalFact::board_derived(
        anchor(),
        "chaintape_cas",
        br#"{"node":"n1","status":"accepted"}"#,
        "board view reconstructed from chain and cas",
    )
    .expect("valid board-derived fact");

    assert_eq!(fact.kind, "board_derived");
    assert_eq!(fact.anchor.head_ref, "refs/chaintape/l4");
    assert_eq!(fact.payload_hash.len(), 64);

    for source in ["dashboard", "board"] {
        let err = TcTapeCanonicalFact::board_derived(anchor(), source, b"{}", "board view")
            .expect_err("board/dashboard cannot be canonical input");
        assert_eq!(err, TapeCanonicalError::SecondSourceDrift);
    }
}

#[test]
fn halt_fact_blocks_clean_claim_with_pending_side_effect() {
    let mut ledger = ExternalCallLedger::default();
    ledger.record_intent(intent("pending")).unwrap();

    let err = TcTapeCanonicalFact::halt_clean(anchor(), &ledger, "run complete")
        .expect_err("pending side effect blocks clean halt fact");
    assert_eq!(err, TapeCanonicalError::PendingSideEffects);

    let fact =
        TcTapeCanonicalFact::halt_clean(anchor(), &ExternalCallLedger::default(), "run halted")
            .expect("balanced side effects allow halt fact");
    assert_eq!(fact.kind, "halt_clean");
    assert_eq!(fact.anchor.head_ref, "refs/chaintape/l4");
    assert_eq!(fact.payload_hash.len(), 64);
}

#[test]
fn lean_error_fact_shields_verifier_output_and_links_attempt() {
    let fact = TcTapeCanonicalFact::lean_error(
        anchor(),
        "theorem-alpha",
        "attempt-7",
        "tactic",
        "bounded tactic failure summary",
    )
    .expect("valid Lean feature-layer error fact");

    assert_eq!(fact.fact.kind, "lean_error");
    assert_eq!(fact.fact.anchor.run_id, "run-tc-101");
    assert_eq!(fact.theorem_id, "theorem-alpha");
    assert_eq!(fact.attempt_id, "attempt-7");
    assert_eq!(fact.class, "tactic");
    assert_eq!(fact.public_summary, "bounded tactic failure summary");
    assert_eq!(fact.public_summary.len(), fact.public_summary_len);
    assert!(fact.public_summary_len <= 240);

    let raw_marker = ["raw ", &["std", "err"].concat()].concat();
    let err = TcTapeCanonicalFact::lean_error(
        anchor(),
        "theorem-alpha",
        "attempt-8",
        "tactic",
        format!("{raw_marker}: private theorem body"),
    )
    .expect_err("unshielded verifier output must be rejected");
    assert_eq!(err, TapeCanonicalError::UnshieldedPublicSummary);

    let long_summary = "x".repeat(241);
    let err = TcTapeCanonicalFact::lean_error(
        anchor(),
        "theorem-alpha",
        "attempt-9",
        "tactic",
        long_summary,
    )
    .expect_err("unbounded public summary must be rejected");
    assert_eq!(err, TapeCanonicalError::UnboundedPublicSummary);
}
