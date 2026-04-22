// Phase 8.G (C-055) regression test
// Art. II.1 "多个 Agent 都在同一个地方跌倒" requires frequency threshold
// before broadcasting a class as a "typical error". Prior to this fix,
// TopKClasses broadcast any class with count ≥ 1 — a single agent slip
// was amplified to whole swarm.

use turingosv4::bus::{BusConfig, RejectionScope, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::sdk::tools::wallet::WalletTool;

// No env guard needed — threshold is a BusConfig field, not process env.

fn make_bus_with_threshold(threshold: u32) -> TuringBus {
    let kernel = Kernel::new();
    let config = BusConfig {
        max_payload_chars: 500,
        max_payload_lines: 20,
        system_lp_amount: 200.0,
        forbidden_patterns: vec![],
        min_class_count_to_broadcast: threshold,
    };
    let mut bus = TuringBus::new(kernel, config);
    bus.mount_tool(Box::new(WalletTool::new(10_000.0)));
    bus.init(&["Agent_0".into(), "Agent_1".into(), "Agent_2".into()]);
    bus
}

fn make_bus() -> TuringBus {
    make_bus_with_threshold(3)
}

#[test]
fn threshold_blocks_single_instance_classes() {
    let mut bus = make_bus();
    // Single rejection of a class → should NOT be broadcast.
    bus.record_rejection("Agent_0", "err:tactic_linarith");
    let out = bus.recent_rejections_scoped(
        "Agent_1", 10, RejectionScope::TopKClasses(5),
    );
    assert!(out.is_empty(),
        "C-055: single-instance class must not broadcast; got {:?}", out);
}

#[test]
fn threshold_allows_typical_classes() {
    let mut bus = make_bus();
    // Same class hit 3 times → reaches threshold → should broadcast.
    bus.record_rejection("Agent_0", "err:tactic_simp_noprog");
    bus.record_rejection("Agent_1", "err:tactic_simp_noprog");
    bus.record_rejection("Agent_2", "err:tactic_simp_noprog");
    let out = bus.recent_rejections_scoped(
        "Agent_0", 10, RejectionScope::TopKClasses(5),
    );
    assert_eq!(out.len(), 1, "typical class (count=3) should broadcast");
    assert!(out[0].contains("err:tactic_simp_noprog") && out[0].contains("(3)"),
        "expected 'err:tactic_simp_noprog(3)', got {:?}", out);
}

#[test]
fn threshold_mixed_broadcast_filters_below() {
    let mut bus = make_bus();
    // One class hits 3+ times (broadcast), another hits only 1 (filter).
    for _ in 0..4 { bus.record_rejection("Agent_0", "err:tactic_ring"); }
    bus.record_rejection("Agent_1", "err:tactic_norm_num");
    let out = bus.recent_rejections_scoped(
        "Agent_0", 10, RejectionScope::TopKClasses(5),
    );
    assert_eq!(out.len(), 1, "only err:tactic_ring should pass threshold");
    assert!(out[0].contains("err:tactic_ring") && out[0].contains("(4)"),
        "expected 'err:tactic_ring(4)', got {:?}", out);
}

#[test]
fn threshold_default_is_three() {
    // Default BusConfig uses 3; verify via explicit helper.
    let mut bus = make_bus_with_threshold(3);
    bus.record_rejection("Agent_0", "err:unknown_const");
    bus.record_rejection("Agent_1", "err:unknown_const");  // count=2 < default 3
    let out = bus.recent_rejections_scoped(
        "Agent_0", 10, RejectionScope::TopKClasses(5),
    );
    assert!(out.is_empty(),
        "count=2 with default threshold=3 should not broadcast; got {:?}", out);
}

#[test]
fn threshold_one_allows_every_class() {
    let mut bus = make_bus_with_threshold(1);
    bus.record_rejection("Agent_0", "err:unsolved_goals");
    let out = bus.recent_rejections_scoped(
        "Agent_1", 10, RejectionScope::TopKClasses(5),
    );
    assert_eq!(out.len(), 1, "threshold=1 restores pre-fix behaviour");
}
