//! TRACE_MATRIX FC1a-substrate_seam + FC3-replay: KILL-git-1 roundtrip tests
//! for the Atom 21 GitTapeLedger implementation.
//!
//! For each of the 6 NodeKind variants we commit a fully-populated TapeNode
//! and verify that retrieval via `latest_node` returns byte-identical fields
//! for every Option<*> field. We then cross-check semantic equality against
//! MemoryTapeLedger committing the same sequence (excluding `.hash` which is
//! OID-derived).
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_GENERATE_PHASE_E_DIRECTIVE_AND_§8.md

use tempfile::TempDir;

use turingosv4::git_tape_ledger::GitTapeLedger;
use turingosv4::ledger::{
    AttemptScope, CommitRequest, ImmutableTapeLedger, MemoryTapeLedger, NodeKind,
};

fn scope(suffix: &str) -> AttemptScope {
    AttemptScope {
        run_id: format!("run-{suffix}"),
        task_id: format!("task-{suffix}"),
        verified_parent: format!("vp-{suffix}"),
    }
}

fn req_with_kind(kind: NodeKind, suffix: &str) -> CommitRequest {
    CommitRequest {
        kind,
        verified: false,
        parent: None,
        scope: Some(scope(suffix)),
        attempt_ordinal: Some(7),
        reject_class: Some(format!("rc-{suffix}")),
        token_count: Some(42),
        payload: serde_json::json!({
            "k": "v",
            "n": 17,
            "nested": { "a": 1, "b": [1, 2, 3] }
        }),
    }
}

fn assert_node_fields_equal_modulo_id_hash(a: &turingosv4::ledger::TapeNode, b: &turingosv4::ledger::TapeNode) {
    assert_eq!(a.kind, b.kind, "kind");
    assert_eq!(a.verified, b.verified, "verified");
    assert_eq!(a.parent, b.parent, "parent");
    assert_eq!(a.scope, b.scope, "scope");
    assert_eq!(a.attempt_ordinal, b.attempt_ordinal, "attempt_ordinal");
    assert_eq!(a.reject_class, b.reject_class, "reject_class");
    assert_eq!(a.token_count, b.token_count, "token_count");
    assert_eq!(a.payload, b.payload, "payload");
    assert_eq!(a.created_at_unix_ms, b.created_at_unix_ms, "created_at_unix_ms");
}

fn roundtrip_one_kind(kind: NodeKind, suffix: &str) {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("tdma_tape.git");
    let mut ledger = GitTapeLedger::init_bare(&path).expect("init_bare");

    let s = scope(suffix);
    let req = req_with_kind(kind.clone(), suffix);
    let in_node = ledger.commit(req.clone());

    let out_node = ledger
        .latest_node(kind.clone(), &s)
        .unwrap_or_else(|| panic!("latest_node returned None for kind {:?}", kind));

    assert_eq!(in_node.id, out_node.id, "id");
    assert_eq!(in_node.hash, out_node.hash, "hash");
    assert_node_fields_equal_modulo_id_hash(&in_node, &out_node);
}

#[test]
fn roundtrip_state_accepted() {
    roundtrip_one_kind(NodeKind::StateAccepted, "sa");
}

#[test]
fn roundtrip_agent_proposal() {
    roundtrip_one_kind(NodeKind::AgentProposal, "ap");
}

#[test]
fn roundtrip_retry_belief_state() {
    roundtrip_one_kind(NodeKind::RetryBeliefState, "bs");
}

#[test]
fn roundtrip_charter_core() {
    roundtrip_one_kind(NodeKind::CharterCore, "cc");
}

#[test]
fn roundtrip_prompt_assembly() {
    roundtrip_one_kind(NodeKind::PromptAssembly, "pa");
}

#[test]
fn roundtrip_escalation() {
    roundtrip_one_kind(NodeKind::Escalation, "es");
}

#[test]
fn cross_impl_semantic_equality() {
    // Commit the same sequence against both impls. Assert all fields equal
    // EXCEPT .hash (OID-derived; MemoryTapeLedger uses content hash, GitTapeLedger
    // uses commit OID).
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("tdma_tape.git");
    let mut git_ledger = GitTapeLedger::init_bare(&path).expect("init_bare");
    let mut mem_ledger = MemoryTapeLedger::new();

    // Commit a 3-node sequence under the SAME scope (each is a different kind).
    let s = scope("xx");
    let kinds: Vec<NodeKind> = vec![
        NodeKind::AgentProposal,
        NodeKind::RetryBeliefState,
        NodeKind::StateAccepted,
    ];

    for (_i, k) in kinds.iter().enumerate() {
        // Build the request manually so both impls see the same scope (`s`)
        // and identical payload — req_with_kind would re-derive a different
        // scope from the suffix.
        let mk_req = || CommitRequest {
            kind: k.clone(),
            verified: false,
            parent: None,
            scope: Some(s.clone()),
            attempt_ordinal: Some(7),
            reject_class: Some("rc-xx".into()),
            token_count: Some(42),
            payload: serde_json::json!({ "k": "v", "n": 17 }),
        };
        let g = git_ledger.commit(mk_req());
        let m = mem_ledger.commit(mk_req());
        assert_eq!(g.id, m.id, "id should match (both monotonic)");
        // hash will differ (OID vs content hash)
        assert_node_fields_equal_modulo_id_hash(&g, &m);
    }

    // Cross-impl: latest_node by kind/scope returns semantically-equal nodes.
    for k in kinds.iter() {
        let g = git_ledger.latest_node(k.clone(), &s).expect("git latest");
        let m = mem_ledger.latest_node(k.clone(), &s).expect("mem latest");
        assert_eq!(g.id, m.id, "latest id");
        assert_node_fields_equal_modulo_id_hash(&g, &m);
    }
}

#[test]
fn count_nodes_filters_by_scope_kind_verified() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("tdma_tape.git");
    let mut ledger = GitTapeLedger::init_bare(&path).expect("init_bare");

    let s1 = scope("a");
    let s2 = scope("b");

    // 3 AgentProposal verified=false under s1
    for i in 0..3 {
        ledger.commit(CommitRequest {
            kind: NodeKind::AgentProposal,
            verified: false,
            parent: None,
            scope: Some(s1.clone()),
            attempt_ordinal: Some(i),
            reject_class: None,
            token_count: None,
            payload: serde_json::json!({"i": i}),
        });
    }
    // 1 StateAccepted verified=true under s1
    ledger.commit(CommitRequest {
        kind: NodeKind::StateAccepted,
        verified: true,
        parent: None,
        scope: Some(s1.clone()),
        attempt_ordinal: None,
        reject_class: None,
        token_count: None,
        payload: serde_json::json!({}),
    });
    // 2 AgentProposal verified=false under s2
    for _ in 0..2 {
        ledger.commit(CommitRequest {
            kind: NodeKind::AgentProposal,
            verified: false,
            parent: None,
            scope: Some(s2.clone()),
            attempt_ordinal: None,
            reject_class: None,
            token_count: None,
            payload: serde_json::json!({}),
        });
    }

    assert_eq!(
        ledger.count_nodes(Some(NodeKind::AgentProposal), Some(false), None, Some(&s1)),
        3,
        "3 AgentProposal verified=false under s1"
    );
    assert_eq!(
        ledger.count_nodes(Some(NodeKind::StateAccepted), Some(true), None, Some(&s1)),
        1,
        "1 StateAccepted verified=true under s1"
    );
    assert_eq!(
        ledger.count_nodes(Some(NodeKind::AgentProposal), Some(false), None, Some(&s2)),
        2,
        "2 AgentProposal verified=false under s2"
    );
}

#[test]
fn dump_all_nodes_yields_committed_count() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("tdma_tape.git");
    let mut ledger = GitTapeLedger::init_bare(&path).expect("init_bare");

    let s = scope("dump");
    for i in 0..5 {
        ledger.commit(CommitRequest {
            kind: NodeKind::AgentProposal,
            verified: false,
            parent: None,
            scope: Some(s.clone()),
            attempt_ordinal: Some(i),
            reject_class: None,
            token_count: None,
            payload: serde_json::json!({"i": i}),
        });
    }

    let dump = ledger.dump_all_nodes();
    assert_eq!(dump.len(), 5);
    // Every entry has hash == commit OID == node.hash
    for (h, n) in &dump {
        assert_eq!(h, &n.hash);
        assert_eq!(n.kind, NodeKind::AgentProposal);
    }
}
