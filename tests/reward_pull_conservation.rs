// Phase 2 integration test: reward-pull conservation + payout correctness.
//
// Confirms:
//   1. Under TAPE_ECONOMY_V2=1, appending a node grants the author γ·lp YES
//      shares in their wallet portfolio.
//   2. At halt_and_settle with the new node on the golden path, the author's
//      wallet is credited with γ·lp Coins.
//   3. The same node off the golden path yields 0 credit and zero portfolio
//      residue (entry cleared).
//   4. Conservation: total Coin flow per market ≤ LP; no mint.
//
// These assertions operationalize Law 2 under the reward-pull extension.
// Running TAPE_ECONOMY_V2=0 (the baseline) must leave balances unchanged.

use turingosv4::bus::{BusConfig, BusResult, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::sdk::tools::wallet::WalletTool;

use std::sync::{Mutex, MutexGuard};

// All 5 tests in this file mutate the process-global TAPE_ECONOMY_V2 env var.
// Cargo runs tests in parallel by default → without serialization, a test that
// expects the flag OFF can race with a peer's `with_env(...,"1",...)` window.
// Acquire ENV_LOCK at the top of every test that touches the var.
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn env_lock() -> MutexGuard<'static, ()> {
    ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner())
}

fn make_config() -> BusConfig {
    BusConfig {
        max_payload_chars: 1200,
        max_payload_lines: 18,
        system_lp_amount: 200.0,
        forbidden_patterns: vec![],
    }
}

fn setup_bus() -> TuringBus {
    let mut bus = TuringBus::new(Kernel::new(), make_config());
    bus.mount_tool(Box::new(WalletTool::new(10000.0)));
    bus.init(&["A0".into(), "A1".into()]);
    bus
}

fn with_env<F: FnOnce() -> R, R>(k: &str, v: &str, f: F) -> R {
    std::env::set_var(k, v);
    let out = f();
    std::env::remove_var(k);
    out
}

fn wallet_balance(bus: &TuringBus, agent: &str) -> f64 {
    bus.tools.iter()
        .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
        .map(|w| w.balance(agent))
        .unwrap_or(0.0)
}

fn wallet_yes(bus: &TuringBus, agent: &str, node: &str) -> f64 {
    bus.tools.iter()
        .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
        .and_then(|w| w.portfolios.get(agent))
        .and_then(|p| p.get(node))
        .map(|(y, _, _)| *y)
        .unwrap_or(0.0)
}

#[test]
fn phase2_founder_grant_credits_yes_on_append() {
    let _g = env_lock();
    with_env("TAPE_ECONOMY_V2", "1", || {
        let mut bus = setup_bus();
        let r = bus.append("A0", "step 1", None).unwrap();
        if let BusResult::Appended { node_id } = r {
            let yes = wallet_yes(&bus, "A0", &node_id);
            // default γ=0.05, lp=200 → 10.0 YES shares
            assert!((yes - 10.0).abs() < 1e-9,
                    "founder grant should be 10.0 YES shares, got {}", yes);
        } else {
            panic!("expected Appended, got {:?}", r);
        }
    });
}

#[test]
fn phase2_no_grant_when_flag_off() {
    let _g = env_lock();
    // Ensure the env var is off for this test (conservative; other tests set it).
    std::env::remove_var("TAPE_ECONOMY_V2");
    let mut bus = setup_bus();
    let r = bus.append("A0", "step 1", None).unwrap();
    if let BusResult::Appended { node_id } = r {
        let yes = wallet_yes(&bus, "A0", &node_id);
        assert_eq!(yes, 0.0, "flag off → no founder grant");
    } else {
        panic!("expected Appended");
    }
}

#[test]
fn phase2_settle_pays_out_on_golden_path() {
    let _g = env_lock();
    with_env("TAPE_ECONOMY_V2", "1", || {
        let mut bus = setup_bus();
        let initial = wallet_balance(&bus, "A0");
        let node_id = match bus.append("A0", "step 1", None).unwrap() {
            BusResult::Appended { node_id } => node_id,
            _ => panic!("expected Appended"),
        };

        // Halt with the appended node on the golden path → YES wins.
        bus.halt_and_settle(&[node_id.clone()]).unwrap();

        let after = wallet_balance(&bus, "A0");
        let expected = initial + 10.0; // founder grant redeemed at 1:1
        assert!((after - expected).abs() < 1e-9,
                "wallet should gain 10.0 on GP win (was {} -> {}, expected {})",
                initial, after, expected);

        // Settlement is idempotent (no double redeem).
        bus.halt_and_settle(&[]).unwrap(); // already resolved, no-op internally
        let after2 = wallet_balance(&bus, "A0");
        assert!((after2 - after).abs() < 1e-9, "idempotent settle");
    });
}

#[test]
fn phase2_settle_zero_on_losing_side() {
    let _g = env_lock();
    with_env("TAPE_ECONOMY_V2", "1", || {
        let mut bus = setup_bus();
        let initial = wallet_balance(&bus, "A0");
        match bus.append("A0", "doomed lemma", None).unwrap() {
            BusResult::Appended { node_id: _ } => {}
            _ => panic!("expected Appended"),
        };
        // Empty golden path → this node resolves NO.
        bus.halt_and_settle(&[]).unwrap();
        let after = wallet_balance(&bus, "A0");
        assert!((after - initial).abs() < 1e-9,
                "no payout on NO-resolved (was {}, after {})", initial, after);
    });
}

#[test]
fn phase2_conservation_total_coins_bounded() {
    // Global conservation check: across one append + resolve, total Coins
    // across all wallets + unmovable LP ghost liquidity is bounded by
    // (initial wallet total) + system_lp_amount per created market.
    let _g = env_lock();
    with_env("TAPE_ECONOMY_V2", "1", || {
        let mut bus = setup_bus();
        let initial: f64 = bus.tools.iter()
            .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
            .map(|w| w.balances.values().sum::<f64>())
            .unwrap();
        // 5 appends → 5 markets × 200 LP = 1000 Coin ghost budget
        let mut nodes = Vec::new();
        for i in 0..5 {
            if let BusResult::Appended { node_id } = bus.append("A0", &format!("s{}", i), None).unwrap() {
                nodes.push(node_id);
            }
        }
        // Halt with all 5 on GP → 5 × γ·lp = 50 Coin payouts to A0
        bus.halt_and_settle(&nodes).unwrap();

        let final_total: f64 = bus.tools.iter()
            .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
            .map(|w| w.balances.values().sum::<f64>())
            .unwrap();
        let pay = final_total - initial;
        // expect +50 (5 markets × 10 Coin founder grant on win)
        assert!((pay - 50.0).abs() < 1e-9,
                "expected 50 Coin payout (5·γ·lp); got {}", pay);
        // upper bound sanity: cannot exceed total ghost LP
        assert!(pay <= 5.0 * 200.0, "payout must not exceed total LP");
    });
}
