// Phase 8.A regression test (C-049)
// Constitutional basis: Art. II.2 (economic signal must be visible to agents)
//
// Before this fix, `bus.snapshot()` hardcoded empty balances/portfolios,
// so every agent prompt displayed `Balance: 0 Coins` regardless of wallet
// state. That invalidated every TAPE_ECONOMY / Hayek bounty experiment
// (F-2026-04-18-02, TAPE_ECONOMY_v1/v2 results cannot be trusted).
//
// These tests lock the fix in place. We reach into `bus.tools` the same way
// evaluator.rs does in production (invest path), since `bus.invest()` is not
// a first-class API — invest is a ToolSignal emitted during append.

use turingosv4::bus::{BusConfig, BusResult, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::sdk::tools::wallet::WalletTool;

fn make_bus(genesis: f64) -> TuringBus {
    let kernel = Kernel::new();
    let config = BusConfig {
        max_payload_chars: 500,
        max_payload_lines: 20,
        system_lp_amount: 200.0,
        forbidden_patterns: vec![],
    };
    let mut bus = TuringBus::new(kernel, config);
    bus.mount_tool(Box::new(WalletTool::new(genesis)));
    bus.init(&["Agent_0".into(), "Agent_1".into()]);
    bus
}

/// Helper mirroring evaluator.rs:688-697 (invest handler): find wallet,
/// deduct, buy shares, record.
fn do_invest(bus: &mut TuringBus, agent: &str, node_id: &str, buy_yes: bool, amount: f64) {
    let wallet = bus.tools.iter_mut()
        .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>())
        .expect("wallet mounted");
    wallet.deduct(agent, amount).expect("sufficient balance");
    let shares = if buy_yes {
        bus.kernel.buy_yes(node_id, amount).expect("market exists, buy YES")
    } else {
        bus.kernel.buy_no(node_id, amount).expect("market exists, buy NO")
    };
    let wallet = bus.tools.iter_mut()
        .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>())
        .expect("wallet still mounted");
    if buy_yes {
        wallet.record_shares(agent, node_id, shares, 0.0, 0.0);
    } else {
        wallet.record_shares(agent, node_id, 0.0, shares, 0.0);
    }
}

#[test]
fn snapshot_balances_nonempty_after_genesis() {
    let bus = make_bus(10_000.0);
    let snap = bus.snapshot();
    assert!(!snap.balances.is_empty(),
        "C-049: balances empty after genesis — regression of Phase 8.A fix");
    assert_eq!(snap.get_balance("Agent_0"), 10_000.0,
        "Agent_0 should see genesis grant, not 0 (Art. II.2 signal)");
    assert_eq!(snap.get_balance("Agent_1"), 10_000.0,
        "Agent_1 should see genesis grant, not 0");
}

#[test]
fn snapshot_portfolio_appears_after_invest() {
    let mut bus = make_bus(10_000.0);
    let node_id = match bus.append("Agent_0", "seed proof step", None).unwrap() {
        BusResult::Appended { node_id } => node_id,
        other => panic!("expected Appended, got {:?}", other),
    };
    let balance_before = bus.snapshot().get_balance("Agent_1");
    do_invest(&mut bus, "Agent_1", &node_id, /*buy_yes=*/true, 100.0);

    let snap = bus.snapshot();
    let p = snap.get_portfolio("Agent_1")
        .expect("Agent_1 portfolio should exist after invest");
    let pos = p.get(&node_id).expect("position on invested node");
    assert!(pos.0 > 0.0,
        "C-049: YES shares must appear in snapshot.portfolios; got {:?}", pos);
    let balance_after = snap.get_balance("Agent_1");
    assert!(balance_after < balance_before,
        "Agent_1 balance must decrease post-invest; before={} after={}",
        balance_before, balance_after);
    assert!((balance_before - balance_after - 100.0).abs() < 1e-6,
        "Debit amount should equal invest amount; Δ={}",
        balance_before - balance_after);
}

#[test]
fn snapshot_reflects_wallet_mutations() {
    // After genesis + direct manipulation, snapshot must track.
    let mut bus = make_bus(10_000.0);
    let s0 = bus.snapshot();
    assert_eq!(s0.get_balance("Agent_0"), 10_000.0);

    // Deduct via the tool path (evaluator-style)
    let wallet = bus.tools.iter_mut()
        .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>())
        .unwrap();
    wallet.deduct("Agent_0", 500.0).unwrap();

    let s1 = bus.snapshot();
    assert!((s1.get_balance("Agent_0") - 9_500.0).abs() < 1e-6,
        "Snapshot must reflect direct wallet deduct; got {}", s1.get_balance("Agent_0"));
}

#[test]
fn snapshot_tracks_multiple_agents_distinct_portfolios() {
    // Codex CHALLENGE-B: cover simultaneous distinct portfolios across agents.
    let mut bus = make_bus(10_000.0);
    let node_a = match bus.append("Agent_0", "node A seed", None).unwrap() {
        BusResult::Appended { node_id } => node_id,
        other => panic!("expected Appended, got {:?}", other),
    };
    let node_b = match bus.append("Agent_1", "node B seed", None).unwrap() {
        BusResult::Appended { node_id } => node_id,
        other => panic!("expected Appended, got {:?}", other),
    };
    // Agent_0 bets YES on its own node; Agent_1 bets NO on its own node.
    do_invest(&mut bus, "Agent_0", &node_a, /*buy_yes=*/true, 50.0);
    do_invest(&mut bus, "Agent_1", &node_b, /*buy_yes=*/false, 75.0);

    let snap = bus.snapshot();

    // Distinct portfolios must appear for both agents.
    let p0 = snap.get_portfolio("Agent_0").expect("Agent_0 portfolio");
    let p1 = snap.get_portfolio("Agent_1").expect("Agent_1 portfolio");
    let pos_a = p0.get(&node_a).expect("Agent_0 position on node A");
    let pos_b = p1.get(&node_b).expect("Agent_1 position on node B");
    assert!(pos_a.0 > 0.0, "Agent_0 should have YES shares on node A; got {:?}", pos_a);
    assert!(pos_b.1 > 0.0, "Agent_1 should have NO shares on node B; got {:?}", pos_b);
    // Agent_0 should NOT appear in node B's position list for their portfolio.
    assert!(p0.get(&node_b).is_none(),
        "Agent_0 must not show position on node B (cross-contamination)");
    assert!(p1.get(&node_a).is_none(),
        "Agent_1 must not show position on node A (cross-contamination)");
    // Distinct balances reflect distinct debits.
    assert!((snap.get_balance("Agent_0") - 9_950.0).abs() < 1e-6,
        "Agent_0 debited 50; got {}", snap.get_balance("Agent_0"));
    assert!((snap.get_balance("Agent_1") - 9_925.0).abs() < 1e-6,
        "Agent_1 debited 75; got {}", snap.get_balance("Agent_1"));
}

/// RAII guard: set env var in constructor, remove on drop — even if test
/// panics. Prevents cross-test contamination under cargo test parallelism.
struct EnvGuard(&'static str);
impl EnvGuard {
    fn set(key: &'static str, val: &str) -> Self {
        std::env::set_var(key, val);
        EnvGuard(key)
    }
}
impl Drop for EnvGuard {
    fn drop(&mut self) {
        std::env::remove_var(self.0);
    }
}

#[test]
fn snapshot_reflects_halt_and_settle_payout() {
    // Codex CHALLENGE-B: cover snapshot() state after halt_and_settle().
    // Flow: init → append node → invest YES → halt_and_settle with golden
    // path → snapshot balance reflects payout (YES holders credited).
    //
    // Settlement of per-node portfolios is gated on TAPE_ECONOMY_V2=1
    // (see bus.rs:halt_and_settle). EnvGuard ensures cleanup even on panic.
    let _guard = EnvGuard::set("TAPE_ECONOMY_V2", "1");

    let mut bus = make_bus(10_000.0);
    let node_id = match bus.append("Agent_0", "golden path step", None).unwrap() {
        BusResult::Appended { node_id } => node_id,
        other => panic!("expected Appended, got {:?}", other),
    };
    do_invest(&mut bus, "Agent_1", &node_id, /*buy_yes=*/true, 100.0);
    let balance_before_settle = bus.snapshot().get_balance("Agent_1");

    bus.halt_and_settle(&[node_id.clone()])
        .expect("halt_and_settle ok on valid golden path");

    let snap_after = bus.snapshot();
    let balance_after = snap_after.get_balance("Agent_1");
    assert!(balance_after > balance_before_settle,
        "C-049 CHALLENGE-B: Agent_1 YES-holder balance must increase after settle; \
         before={} after={}", balance_before_settle, balance_after);
    let market = snap_after.markets.get(&node_id).expect("market in snapshot");
    assert_eq!(market.resolved, Some(true),
        "Golden path node market should be resolved YES after settle");
}

#[test]
fn snapshot_empty_when_no_wallet_mounted() {
    // Regression guard: tool-less bus must not panic, just return empty maps.
    let kernel = Kernel::new();
    let config = BusConfig {
        max_payload_chars: 100,
        max_payload_lines: 5,
        system_lp_amount: 100.0,
        forbidden_patterns: vec![],
    };
    let bus = TuringBus::new(kernel, config);
    let snap = bus.snapshot();
    assert!(snap.balances.is_empty(),
        "No wallet mounted → balances empty (not panic)");
    assert!(snap.portfolios.is_empty(),
        "No wallet mounted → portfolios empty (not panic)");
}
