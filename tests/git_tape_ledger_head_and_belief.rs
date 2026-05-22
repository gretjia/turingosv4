//! TRACE_MATRIX FC1a-substrate_seam + FC1b-wtool + FC3-replay:
//! Atom 22 — KILL-git-2 verified_head + KILL-git-3 BBS-via-tape tests.
//!
//! KILL-git-2: parallel of Atom 7 Gate 9 ("verified_head static under hard
//! failures") but under GitTapeLedger. After 10 unverified commits the
//! verified_head must remain at "H0" sentinel (never advanced).
//!
//! KILL-git-3: cross-impl BBS derivation equality. Commit identical
//! RetryBeliefState sequences against MemoryTapeLedger and GitTapeLedger;
//! derive_latest_belief_state_from_tape returns the same BBS payload
//! (modulo .id/.hash via cross-impl roundtrip Atom 21 already verified).
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_GENERATE_PHASE_E_DIRECTIVE_AND_§8.md

use tempfile::TempDir;

use turingosv4::git_tape_ledger::GitTapeLedger;
use turingosv4::ledger::{
    AttemptScope, CommitRequest, EvictedConstraint, EvidencePointer, FailureSignature,
    ImmutableTapeLedger, MemoryTapeLedger, NodeKind, RetryBeliefState, RetryConstraint,
};

fn s_scope() -> AttemptScope {
    AttemptScope {
        run_id: "run-1".into(),
        task_id: "task-1".into(),
        verified_parent: "H0".into(),
    }
}

fn fixture_bbs(seq: u32) -> RetryBeliefState {
    RetryBeliefState {
        schema_version: "tdma-bbs/v1".into(),
        scope: s_scope(),
        failure_signature: FailureSignature {
            reject_class: format!("rc-{}", seq),
            failed_predicate: "p".into(),
            root_cause: "rc".into(),
        },
        constraints: vec![RetryConstraint {
            id: format!("c-{}", seq),
            rule: format!("rule-{}", seq),
            priority: 128,
            source_attempt: seq,
            evidence_hash: "e".into(),
        }],
        evidence: EvidencePointer {
            evidence_node_hash: "h".into(),
            raw_stderr_sha256: "rss".into(),
            trace_view_sha256: "tvs".into(),
        },
        zero_gain_streak: 0,
        information_gain: 0.5,
        evicted: Vec::<EvictedConstraint>::new(),
    }
}

#[test]
fn kill_git_2_verified_head_static_under_ten_hard_failures() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("tdma_tape.git");
    let mut ledger = GitTapeLedger::init_bare(&path).expect("init_bare");

    assert_eq!(ledger.get_verified_head(), "H0", "initial verified_head");

    let s = s_scope();
    for i in 0..10 {
        ledger.commit(CommitRequest {
            kind: NodeKind::AgentProposal,
            verified: false,
            parent: Some("H0".into()),
            scope: Some(s.clone()),
            attempt_ordinal: Some(i),
            reject_class: Some(format!("rc-{}", i)),
            token_count: None,
            payload: serde_json::json!({"attempt": i}),
        });
        assert_eq!(
            ledger.get_verified_head(),
            "H0",
            "verified_head MUST NOT advance after unverified commit {}",
            i
        );
    }

    assert_eq!(
        ledger.count_nodes(Some(NodeKind::AgentProposal), Some(false), None, Some(&s)),
        10,
        "10 unverified AgentProposals committed under s"
    );
    assert_eq!(
        ledger.get_verified_head(),
        "H0",
        "verified_head still H0 after 10 hard failures (KILL-git-2)"
    );
}

#[test]
fn set_verified_head_persists_and_resets_on_h0() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("tdma_tape.git");
    let mut ledger = GitTapeLedger::init_bare(&path).expect("init_bare");

    let s = s_scope();
    let node = ledger.commit(CommitRequest {
        kind: NodeKind::StateAccepted,
        verified: true,
        parent: None,
        scope: Some(s.clone()),
        attempt_ordinal: None,
        reject_class: None,
        token_count: None,
        payload: serde_json::json!({}),
    });

    let oid = node.hash.clone();
    ledger.set_verified_head(oid.clone());
    assert_eq!(ledger.get_verified_head(), oid);

    // Reset to H0 deletes the ref.
    ledger.set_verified_head("H0".into());
    assert_eq!(ledger.get_verified_head(), "H0");
}

#[test]
fn kill_git_3_cross_impl_bbs_derivation_equality() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("tdma_tape.git");
    let mut git_l = GitTapeLedger::init_bare(&path).expect("init_bare");
    let mut mem_l = MemoryTapeLedger::new();

    let s = s_scope();

    // Commit 3 BBS nodes (escalating zero_gain_streak) under the same scope
    // against both impls.
    for seq in 0..3 {
        let bbs = fixture_bbs(seq);
        let payload = serde_json::to_value(&bbs).unwrap();
        let mk_req = || CommitRequest {
            kind: NodeKind::RetryBeliefState,
            verified: false,
            parent: None,
            scope: Some(s.clone()),
            attempt_ordinal: Some(seq),
            reject_class: None,
            token_count: None,
            payload: payload.clone(),
        };
        git_l.commit(mk_req());
        mem_l.commit(mk_req());
    }

    // derive_latest_belief_state_from_tape MUST return identical BBS (the
    // last-committed seq=2 fixture) under both impls — KILL-git-3.
    let g = git_l
        .derive_latest_belief_state_from_tape(&s)
        .expect("git BBS");
    let m = mem_l
        .derive_latest_belief_state_from_tape(&s)
        .expect("mem BBS");

    assert_eq!(g.schema_version, m.schema_version);
    assert_eq!(g.scope, m.scope);
    assert_eq!(g.failure_signature, m.failure_signature);
    assert_eq!(g.constraints, m.constraints);
    assert_eq!(g.evidence, m.evidence);
    assert_eq!(g.zero_gain_streak, m.zero_gain_streak);
    assert_eq!(g.evicted, m.evicted);
    // information_gain is f64 (not Eq); compare equality literally
    assert!((g.information_gain - m.information_gain).abs() < f64::EPSILON);
}

#[test]
fn derive_belief_returns_none_on_empty_scope() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("tdma_tape.git");
    let ledger = GitTapeLedger::init_bare(&path).expect("init_bare");

    let s = AttemptScope {
        run_id: "nonexistent".into(),
        task_id: "nonexistent".into(),
        verified_parent: "H0".into(),
    };
    assert!(ledger.derive_latest_belief_state_from_tape(&s).is_none());
}
