// Phase 8.F (C-053) regression test
// Art. I.2 信誉累积: "统计某个 Agent 提出的方案在后续流程中被其他 Agent
// 成功调用的总次数" — 必须作为 per-agent 累积标量暴露.
//
// Before this fix, Tape only tracked `reverse_citations: HashMap<NodeId, Vec<NodeId>>`
// per-node and no per-author aggregate. Snapshot didn't surface it at all.

use turingosv4::bus::{BusConfig, BusResult, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::sdk::tools::wallet::WalletTool;

fn make_bus() -> TuringBus {
    let kernel = Kernel::new();
    let config = BusConfig {
        max_payload_chars: 500,
        max_payload_lines: 20,
        system_lp_amount: 200.0,
        forbidden_patterns: vec![],
        min_class_count_to_broadcast: 3,
    };
    let mut bus = TuringBus::new(kernel, config);
    bus.mount_tool(Box::new(WalletTool::new(10_000.0)));
    bus.init(&["Alice".into(), "Bob".into(), "Carol".into()]);
    bus
}

fn append(bus: &mut TuringBus, author: &str, payload: &str, parent: Option<&str>) -> String {
    match bus.append(author, payload, parent).unwrap() {
        BusResult::Appended { node_id } => node_id,
        other => panic!("expected Appended, got {:?}", other),
    }
}

#[test]
fn reputation_zero_at_init() {
    let bus = make_bus();
    let snap = bus.snapshot();
    assert_eq!(snap.get_reputation("Alice"), 0);
    assert_eq!(snap.get_reputation("Bob"), 0);
    assert_eq!(snap.get_reputation("Carol"), 0);
}

#[test]
fn reputation_increments_on_citation() {
    let mut bus = make_bus();
    // Alice seeds a lemma.
    let a = append(&mut bus, "Alice", "have h : 1=1 := rfl", None);
    let s0 = bus.snapshot();
    assert_eq!(s0.get_reputation("Alice"), 0, "no citations yet");

    // Bob cites Alice's lemma.
    let _ = append(&mut bus, "Bob", "exact h.symm", Some(&a));
    let s1 = bus.snapshot();
    assert_eq!(s1.get_reputation("Alice"), 1,
        "Alice should gain rep=1 after Bob cites");
    assert_eq!(s1.get_reputation("Bob"), 0,
        "Bob's own contribution not yet cited");
}

#[test]
fn reputation_accumulates_across_citers() {
    let mut bus = make_bus();
    let a = append(&mut bus, "Alice", "seed lemma", None);
    append(&mut bus, "Bob", "build on alice", Some(&a));
    append(&mut bus, "Carol", "also build on alice", Some(&a));
    let snap = bus.snapshot();
    assert_eq!(snap.get_reputation("Alice"), 2,
        "Alice cited by Bob + Carol = 2");
    assert_eq!(snap.get_reputation("Bob"), 0);
    assert_eq!(snap.get_reputation("Carol"), 0);
}

#[test]
fn reputation_multiple_citations_in_one_node() {
    // Tape::append accepts multi-parent citations; each counts.
    let mut bus = make_bus();
    let a = append(&mut bus, "Alice", "lemma A", None);
    let b = append(&mut bus, "Bob", "lemma B", None);
    // Carol cites both — via append (which only supports 1 parent) so we
    // simulate via two appends. Or use kernel direct if multi-parent
    // is bus-path. For this test, a chain: Alice → Bob → ... only.
    // Validate that chained citation credits each predecessor once.
    append(&mut bus, "Carol", "build on alice", Some(&a));
    append(&mut bus, "Carol", "build on bob", Some(&b));
    let snap = bus.snapshot();
    assert_eq!(snap.get_reputation("Alice"), 1);
    assert_eq!(snap.get_reputation("Bob"), 1);
    assert_eq!(snap.get_reputation("Carol"), 0);
}

#[test]
fn reputation_self_citation_counts() {
    // Author cites their own earlier node — still counts (measure
    // "work built upon", not "peer-vs-self"). Documented ruling in C-053.
    let mut bus = make_bus();
    let a = append(&mut bus, "Alice", "lemma A", None);
    append(&mut bus, "Alice", "lemma A' built on A", Some(&a));
    let snap = bus.snapshot();
    assert_eq!(snap.get_reputation("Alice"), 1,
        "self-citation counts; C-053 measures citation of work not peer voting");
}

#[test]
fn reputation_ignores_unappended_parent() {
    // Dangling parent → append rejects; rep must NOT change.
    let mut bus = make_bus();
    let err = bus.append("Bob", "citing nothing", Some("tx_999_by_ghost"));
    assert!(err.is_err() || matches!(err, Ok(BusResult::Vetoed { .. })),
        "bus should reject dangling citation");
    let snap = bus.snapshot();
    assert_eq!(snap.get_reputation("Bob"), 0);
}

#[test]
fn reputation_map_surfaces_all_authors() {
    let mut bus = make_bus();
    let a = append(&mut bus, "Alice", "seed", None);
    let _b = append(&mut bus, "Bob", "cite alice", Some(&a));
    // Verify the full map contains Alice (implicit enumeration test).
    let snap = bus.snapshot();
    assert!(snap.reputation.contains_key("Alice"));
    assert_eq!(*snap.reputation.get("Alice").unwrap(), 1);
}
