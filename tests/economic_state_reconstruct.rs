//! TRACE_MATRIX WP § 2 economic — EconomicState 9 sub-fields reconstructibility.
//!
//! Atom CO1.2.2: each sub-index round-trips and is insertion-order independent.

use turingosv4::economy::money::MicroCoin;
use turingosv4::state::{
    AgentId, BalancesIndex, ChallengeCase, ChallengeCasesIndex, ClaimEntry, ClaimsIndex,
    EconomicState, EscrowEntry, EscrowsIndex, PriceIndex, Reputation, ReputationsIndex,
    RoyaltyEdge, RoyaltyGraph, StakeEntry, StakesIndex, TaskId, TaskMarketEntry, TaskMarketsIndex,
    TxId,
};

#[test]
fn ten_sub_fields_present() {  // TB-11: was nine_sub_fields_present (architect §6.2 +runs_t)
    let e = EconomicState::default();
    let v = serde_json::to_value(&e).unwrap();
    let obj = v.as_object().unwrap();
    let names = [
        "balances_t",
        "escrows_t",
        "stakes_t",
        "claims_t",
        "reputations_t",
        "task_markets_t",
        "royalty_graph_t",
        "challenge_cases_t",
        "price_index_t",
        "runs_t", // TB-11 (architect §6.2 ruling 2026-05-02)
    ];
    assert_eq!(obj.len(), 10);
    for n in names.iter() {
        assert!(obj.contains_key(*n), "missing sub-field {}", n);
    }
}

#[test]
fn populated_economic_state_round_trip() {
    let mut e = EconomicState::default();
    e.balances_t.0.insert(AgentId("a".into()), MicroCoin::from_coin(10).unwrap());
    e.escrows_t.0.insert(
        TxId("t1".into()),
        EscrowEntry {
            amount: MicroCoin::from_coin(5).unwrap(),
            depositor: AgentId("a".into()),
            task_id: TaskId("t4".into()),
        },
    );
    e.stakes_t.0.insert(
        TxId("t2".into()),
        StakeEntry {
            amount: MicroCoin::from_coin(3).unwrap(),
            staker: AgentId("b".into()),
            task_id: TaskId("t4".into()),
        },
    );
    e.claims_t.0.insert(
        TxId("t3".into()),
        ClaimEntry {
            amount: MicroCoin::from_coin(7).unwrap(),
            claimant: AgentId("c".into()),
            ..Default::default()
        },
    );
    e.reputations_t.0.insert(AgentId("a".into()), Reputation(100));
    // **TB-3 fixture migration**: TaskMarketEntry no longer has `bounty`;
    // money has migrated to `escrows_t.amount`. `total_escrow` is the derived
    // cache (matches the escrow above for round-trip determinism).
    let mut market = TaskMarketEntry::default();
    market.publisher = AgentId("p".into());
    market.total_escrow = MicroCoin::from_coin(5).unwrap();
    market.escrow_lock_tx_ids.insert(TxId("t1".into()));
    market.verifier_quorum = 1;
    market.max_reuse_royalty_fraction_basis_points = 1000;
    e.task_markets_t.0.insert(TaskId("t4".into()), market);
    e.royalty_graph_t.0.insert(
        TxId("t5".into()),
        vec![RoyaltyEdge { ancestor: TxId("t4".into()), fraction_basis_points: 500 }],
    );
    e.challenge_cases_t.0.insert(
        TxId("t6".into()),
        ChallengeCase {
            challenger: AgentId("ch".into()),
            bond: MicroCoin::from_coin(2).unwrap(),
            opened_at_round: 5,
            target_work_tx: TxId("target_wt".into()), // TB-4 additive backref
            status: turingosv4::state::q_state::ChallengeStatus::Open, // TB-5 additive
        },
    );
    e.price_index_t.0.insert(TxId("t7".into()), MicroCoin::from_coin(9).unwrap());

    let s = serde_json::to_string(&e).unwrap();
    let back: EconomicState = serde_json::from_str(&s).unwrap();
    assert_eq!(e, back);
}

#[test]
fn balances_insertion_order_independence() {
    let mut a = BalancesIndex::default();
    let mut b = BalancesIndex::default();
    let names = ["zeta", "alpha", "mu", "beta", "gamma"];
    for (i, n) in names.iter().enumerate() {
        a.0.insert(AgentId(n.to_string()), MicroCoin::from_coin(i as i64).unwrap());
    }
    for n in names.iter().rev() {
        let i = names.iter().position(|x| x == n).unwrap();
        b.0.insert(AgentId(n.to_string()), MicroCoin::from_coin(i as i64).unwrap());
    }
    assert_eq!(serde_json::to_string(&a).unwrap(), serde_json::to_string(&b).unwrap());
}

#[test]
fn empty_indices_serialize_to_empty_objects() {
    assert_eq!(serde_json::to_string(&BalancesIndex::default()).unwrap(), "{}");
    assert_eq!(serde_json::to_string(&EscrowsIndex::default()).unwrap(), "{}");
    assert_eq!(serde_json::to_string(&StakesIndex::default()).unwrap(), "{}");
    assert_eq!(serde_json::to_string(&ClaimsIndex::default()).unwrap(), "{}");
    assert_eq!(serde_json::to_string(&ReputationsIndex::default()).unwrap(), "{}");
    assert_eq!(serde_json::to_string(&TaskMarketsIndex::default()).unwrap(), "{}");
    assert_eq!(serde_json::to_string(&RoyaltyGraph::default()).unwrap(), "{}");
    assert_eq!(serde_json::to_string(&ChallengeCasesIndex::default()).unwrap(), "{}");
    assert_eq!(serde_json::to_string(&PriceIndex::default()).unwrap(), "{}");
}
