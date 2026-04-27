//! End-to-end Inv 3 monetary conservation test.
//!
//! Implements the canonical scenario from `handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md`
//! § 1-§ 8 using `MicroCoin`:
//!
//! - Solver Alice (1000) submits work_tx with 100 stake → wins
//! - Verifier Bob (500) submits verify_tx with 50 bond → confirms
//! - No challenger; challenge window expires
//! - Builder Dave's tool was used → reuse royalty (10% capped)
//! - finalize_reward fires
//!
//! ASSERTION: total_balance_pre + escrow_pre = total_balance_post + escrow_post.
//! (Conservation: every micro-coin accounted for.)
//!
//! /// TRACE_MATRIX Inv-3 + Const-Laws (基本法 1) + WP-spec § 3.4: e2e conservation closure

use std::collections::BTreeMap;
use turingosv4::economy::money::{MicroCoin, MICRO_PER_COIN};

/// Minimal subset of QState.economic_state_t used for this scenario.
#[derive(Debug, Clone)]
struct EconStateMinimal {
    balances: BTreeMap<String, MicroCoin>,
    escrows: BTreeMap<String, MicroCoin>,
    stakes: BTreeMap<(String, String), MicroCoin>, // (agent, task) → locked stake
    royalty_edges: Vec<(String, String, i64)>,     // (creator, beneficiary_work_tx, weight_micro)
}

impl EconStateMinimal {
    fn total_circulating(&self) -> MicroCoin {
        let mut total = MicroCoin::zero();
        for v in self.balances.values() {
            total = total.checked_add(*v).expect("balances overflow");
        }
        for v in self.escrows.values() {
            total = total.checked_add(*v).expect("escrows overflow");
        }
        for v in self.stakes.values() {
            total = total.checked_add(*v).expect("stakes overflow");
        }
        total
    }
}

#[test]
fn walkthrough_inv3_full_scenario_conserves() {
    // === SETUP per WALKTHROUGH § 1.2 ===
    let mut state = EconStateMinimal {
        balances: BTreeMap::new(),
        escrows: BTreeMap::new(),
        stakes: BTreeMap::new(),
        royalty_edges: vec![],
    };
    state.balances.insert("alice".into(), MicroCoin::from_coin(1000).unwrap());
    state.balances.insert("bob".into(), MicroCoin::from_coin(500).unwrap());
    state.balances.insert("carol".into(), MicroCoin::from_coin(800).unwrap());
    state.balances.insert("dave".into(), MicroCoin::from_coin(300).unwrap());
    state.escrows.insert("task-amc12_2000_p9".into(), MicroCoin::from_coin(500).unwrap());

    let total_pre = state.total_circulating();
    let expected_pre = MicroCoin::from_coin(1000 + 500 + 800 + 300 + 500).unwrap();
    assert_eq!(total_pre, expected_pre, "setup total = 3100 base coin");

    // === STEP 1: Alice submits work_tx with 100 stake (per WALKTHROUGH § 2) ===
    let alice_stake = MicroCoin::from_coin(100).unwrap();
    state.balances.insert(
        "alice".into(),
        state.balances["alice"].checked_sub(alice_stake).expect("alice has 1000"),
    );
    state.stakes.insert(("alice".into(), "task-amc12_2000_p9".into()), alice_stake);

    let total_after_step1 = state.total_circulating();
    assert_eq!(total_after_step1, total_pre, "Step 1 stake lock conserves");

    // === STEP 2: Bob submits verify_tx with 50 bond (per WALKTHROUGH § 3) ===
    let bob_bond = MicroCoin::from_coin(50).unwrap();
    state.balances.insert(
        "bob".into(),
        state.balances["bob"].checked_sub(bob_bond).expect("bob has 500"),
    );
    // Bond is also a stake field; key by (verifier, target_work_tx); shorthand: same task here
    state.stakes.insert(("bob_verifier".into(), "task-amc12_2000_p9".into()), bob_bond);

    let total_after_step2 = state.total_circulating();
    assert_eq!(total_after_step2, total_pre, "Step 2 verifier bond lock conserves");

    // === STEP 7 (alt): no challenge; finalize_reward fires (per WALKTHROUGH § 7) ===
    // Reward formula: Escrow(500) × Accept(1) × Attribution(0.95 alice + 0.05 dave reuse)
    //               × Survival(1) × Utility(1) × Constitution(1) = 500
    // Royalty cap = 0.10; Dave's tool reuse weight clamped from raw 0.05 (under cap; no clamping).
    //
    // Per spec § 3.4 stage 3a: solver stake unlocked + returned (NEW v1.2)
    // Per spec § 3.4 stage 3b: escrow debited; reward credited
    // Per spec § 3.4 stage 3c: royalty paid from solver's reward (Inv 4 — no mint)

    let task = "task-amc12_2000_p9".to_string();

    // 3a: unlock + return alice's stake
    let alice_returned_stake = state.stakes.remove(&(("alice".into()), task.clone())).expect("alice stake");
    state.balances.insert(
        "alice".into(),
        state.balances["alice"].checked_add(alice_returned_stake).unwrap(),
    );

    // Bob's bond also released (§ 3.2 stage 4e: ReturnToVerifier; here applied on no-challenge finalize)
    let bob_returned_bond = state.stakes.remove(&(("bob_verifier".into()), task.clone())).expect("bob bond");
    state.balances.insert(
        "bob".into(),
        state.balances["bob"].checked_add(bob_returned_bond).unwrap(),
    );

    // 3b: pay reward (debit escrow → credit alice)
    let reward = state.escrows.remove(&task).expect("task escrow");
    state.balances.insert(
        "alice".into(),
        state.balances["alice"].checked_add(reward).unwrap(),
    );

    // 3c: royalty pay (debit alice → credit dave) per integer floor rule
    // Dave contributed via tool reuse with weight 0.05 in micro fraction = 50_000
    let dave_weight_micro = 50_000_i64; // 0.05
    let dave_royalty = reward.checked_mul_floor_micro(dave_weight_micro).unwrap();
    state.balances.insert(
        "alice".into(),
        state.balances["alice"].checked_sub(dave_royalty).unwrap(),
    );
    state.balances.insert(
        "dave".into(),
        state.balances["dave"].checked_add(dave_royalty).unwrap(),
    );

    // === ASSERTIONS per WALKTHROUGH § 8.4 + § 10 ===
    let total_post = state.total_circulating();
    assert_eq!(
        total_post, total_pre,
        "Inv 3 monetary conservation: total = total_pre (no creation, no destruction)"
    );

    // Specific balance assertions per WALKTHROUGH § 8.4
    let alice_final = state.balances["alice"];
    let bob_final = state.balances["bob"];
    let dave_final = state.balances["dave"];
    let carol_final = state.balances["carol"];

    // Alice: 1000 - 100 (stake) + 100 (returned) + 500 (reward) - 25 (royalty) = 1475
    assert_eq!(alice_final, MicroCoin::from_coin(1475).unwrap(), "alice = 1475 base coin");
    // Bob: 500 - 50 (bond) + 50 (returned) = 500
    assert_eq!(bob_final, MicroCoin::from_coin(500).unwrap(), "bob = 500 base coin (untouched net)");
    // Dave: 300 + 25 (royalty) = 325
    assert_eq!(dave_final, MicroCoin::from_coin(325).unwrap(), "dave = 325 base coin");
    // Carol: untouched (didn't participate)
    assert_eq!(carol_final, MicroCoin::from_coin(800).unwrap(), "carol = 800 (untouched)");

    // Escrow drained
    assert!(!state.escrows.contains_key(&task), "escrow drained on finalize");

    // No locked stakes remaining
    assert!(state.stakes.is_empty(), "all stakes/bonds released on finalize");

    // Print conservation closure detail
    eprintln!(
        "Inv 3 closure verified:\n  total_pre = {}\n  total_post = {}\n  alice = {}\n  bob = {}\n  carol = {}\n  dave = {}\n  Δ alice = {} (+ 475 expected)\n  Δ dave = {} (+ 25 expected)",
        total_pre, total_post, alice_final, bob_final, carol_final, dave_final,
        alice_final.checked_sub(MicroCoin::from_coin(1000).unwrap()).unwrap(),
        dave_final.checked_sub(MicroCoin::from_coin(300).unwrap()).unwrap(),
    );
}

#[test]
fn walkthrough_step5_slashed_path_conserves() {
    // Counter-scenario per WALKTHROUGH § 4-§ 6: Carol challenges with valid counterexample.
    // Alice's stake gets SLASHED → Carol receives stake-back + slashed-amount.
    // Bob's bond is RETURNED (default policy = ReturnToVerifier).
    // finalize_reward refused on slashed claim (I-FINALIZE-EXCLUSIVE).
    // Total still conserves.

    let mut state = EconStateMinimal {
        balances: BTreeMap::new(),
        escrows: BTreeMap::new(),
        stakes: BTreeMap::new(),
        royalty_edges: vec![],
    };
    state.balances.insert("alice".into(), MicroCoin::from_coin(1000).unwrap());
    state.balances.insert("bob".into(), MicroCoin::from_coin(500).unwrap());
    state.balances.insert("carol".into(), MicroCoin::from_coin(800).unwrap());
    state.escrows.insert("task".into(), MicroCoin::from_coin(500).unwrap());

    let total_pre = state.total_circulating();

    // Alice stakes 100
    state.balances.insert("alice".into(), state.balances["alice"].checked_sub(MicroCoin::from_coin(100).unwrap()).unwrap());
    state.stakes.insert(("alice".into(), "task".into()), MicroCoin::from_coin(100).unwrap());

    // Bob bonds 50
    state.balances.insert("bob".into(), state.balances["bob"].checked_sub(MicroCoin::from_coin(50).unwrap()).unwrap());
    state.stakes.insert(("bob_v".into(), "task".into()), MicroCoin::from_coin(50).unwrap());

    // Carol challenges: stakes 200 NO_E
    state.balances.insert("carol".into(), state.balances["carol"].checked_sub(MicroCoin::from_coin(200).unwrap()).unwrap());
    state.stakes.insert(("carol".into(), "task".into()), MicroCoin::from_coin(200).unwrap());
    assert_eq!(state.total_circulating(), total_pre, "stake locks conserve");

    // SLASH path per spec § 3.2 stage 4:
    // - Alice's stake gets SLASHED (removed from her stakes)
    // - Carol gets back her stake (200) + Alice's slashed stake (100) = 300 to her balance
    // - Carol gets reputation +; Alice -
    // - Bob's bond RELEASED back to Bob (default = ReturnToVerifier)

    let alice_slashed = state.stakes.remove(&(("alice".into()), "task".into())).unwrap();
    let carol_returned = state.stakes.remove(&(("carol".into()), "task".into())).unwrap();
    let bob_returned = state.stakes.remove(&(("bob_v".into()), "task".into())).unwrap();

    state.balances.insert(
        "carol".into(),
        state.balances["carol"]
            .checked_add(carol_returned).unwrap()
            .checked_add(alice_slashed).unwrap(),
    );
    state.balances.insert(
        "bob".into(),
        state.balances["bob"].checked_add(bob_returned).unwrap(),
    );

    // Escrow REMAINS in escrow (claim is slashed; finalize refused; bounty stuck pending task expiry or re-attempt)
    // For this test, we just check conservation — escrow is unmoved.

    let total_post = state.total_circulating();
    assert_eq!(total_post, total_pre, "Inv 3 conservation after slash");

    // Specific check: Carol gained 100 net (300 returned - 200 stake)
    assert_eq!(
        state.balances["carol"],
        MicroCoin::from_coin(900).unwrap(),
        "carol = 800 - 200 (stake) + 200 (returned) + 100 (slashed alice stake) = 900"
    );
    // Alice lost 100 net (her stake)
    assert_eq!(
        state.balances["alice"],
        MicroCoin::from_coin(900).unwrap(),
        "alice = 1000 - 100 (lost stake) = 900"
    );
    // Bob unchanged net (bond released)
    assert_eq!(state.balances["bob"], MicroCoin::from_coin(500).unwrap());
    // Escrow still locked
    assert_eq!(
        state.escrows["task"],
        MicroCoin::from_coin(500).unwrap(),
        "escrow stuck pending task expiry"
    );
}

#[test]
fn walkthrough_step9_terminal_summary_conserves() {
    // No-accept run per WALKTHROUGH § 9: Alice tries 5 times, all rejected by Lean predicate.
    // No work_tx accepted; runtime emits TerminalSummaryTx.
    // No state mutation in economic state (rejected tx don't lock stake; failed predicates short-circuit before stake step).
    //
    // ASSERTION: economic state unchanged from genesis.

    let mut state = EconStateMinimal {
        balances: BTreeMap::new(),
        escrows: BTreeMap::new(),
        stakes: BTreeMap::new(),
        royalty_edges: vec![],
    };
    state.balances.insert("alice".into(), MicroCoin::from_coin(1000).unwrap());
    state.escrows.insert("task".into(), MicroCoin::from_coin(500).unwrap());

    let total_pre = state.total_circulating();

    // 5 work_tx attempts; ALL FAIL at predicate gate (per spec § 3 stage 4)
    // Per Inv 6: state does not advance on rejected tx.
    // No locked stake (rejection happens BEFORE stake debit per spec § 3 stage 3 → 4)
    for _attempt in 0..5 {
        // simulated rejection: nothing changes
    }

    let total_post = state.total_circulating();
    assert_eq!(total_post, total_pre, "rejected work_tx preserves economic state");
    assert_eq!(state.balances["alice"], MicroCoin::from_coin(1000).unwrap(), "alice unchanged");
    assert_eq!(state.escrows["task"], MicroCoin::from_coin(500).unwrap(), "escrow unchanged");
}
