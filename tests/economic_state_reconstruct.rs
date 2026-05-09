//! TRACE_MATRIX WP § 2 economic — EconomicState 9 sub-fields reconstructibility.
//!
//! Atom CO1.2.2: each sub-index round-trips and is insertion-order independent.

use turingosv4::economy::money::MicroCoin;
use turingosv4::state::{
    AgentId, BalancesIndex, ChallengeCase, ChallengeCasesIndex, ClaimEntry, ClaimsIndex,
    EconomicState, EscrowEntry, EscrowsIndex, Reputation, ReputationsIndex,
    RoyaltyEdge, RoyaltyGraph, StakeEntry, StakesIndex, TaskId, TaskMarketEntry, TaskMarketsIndex,
    TxId,
};

#[test]
fn fifteen_sub_fields_present() {
    // TB-12: was ten (TB-11 +runs_t); +node_positions_t (architect
    // 2026-05-03 §3). TB-13 Atom 2 (architect 2026-05-03 post-TB-12
    // ruling Part A §4.3): 11 → 13 (+conditional_collateral_t Coin
    // holding + conditional_share_balances_t claims).
    // TB-14 Atom 2 (2026-05-03; architect §5.1): 13 → 12 (-price_index_t;
    // TB-14 derives the price view via `compute_price_index` pure fn,
    // not stored as canonical state — "price is signal, not truth";
    // charter §7 auto-resolution A: no second source-of-truth).
    // TB-15 Atom 3 (2026-05-03; architect §6.2 ruling): 12 → 13
    // (+agent_autopsies_t — `AutopsyIndex` = `BTreeMap<EventId, Vec<Cid>>`;
    // sequencer-side per-event Cid index for AgentAutopsyCapsule emission;
    // capsule bytes live in CAS; NOT projected to AgentVisibleProjection
    // per CR-15.1 + halt-trigger #1).
    // Stage C P-M4 / Phase F.3 (architect manual §7.5 + remediation
    // directive 2026-05-09 §1.C row 3): 13 → 15 (+cpmm_pools_t
    // CpmmPoolsIndex one-pool-per-event LiquidityPool state, NOT Coin per
    // architect §7.5 rule 2; +lp_share_balances_t LpShareBalancesIndex
    // per-(agent, event) LP token balance, NOT Coin per rule 3).
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
        "runs_t",                          // TB-11 (architect §6.2 ruling 2026-05-02)
        "node_positions_t",                // TB-12 (architect 2026-05-03 ruling §3 + §8 Atom 1)
        "conditional_collateral_t",        // TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling §4.3)
        "conditional_share_balances_t",    // TB-13 Atom 2
        "agent_autopsies_t",               // TB-15 Atom 3 (architect §6.2 ruling 2026-05-02 + 2026-05-03)
        "cpmm_pools_t",                    // Stage C P-M4 / Phase F.3 (architect §7.5)
        "lp_share_balances_t",             // Stage C P-M4 / Phase F.3 (architect §7.5 rule 3)
    ];
    assert_eq!(obj.len(), 15);
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
    // TB-14 Atom 2 (2026-05-03): legacy `price_index_t` field removed —
    // TB-14 derives the price view via `compute_price_index` pure fn over
    // `node_positions_t` + `conditional_share_balances_t`, not stored as
    // canonical state.

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
    // TB-14 Atom 2 (2026-05-03): legacy `PriceIndex` struct removed.
    // The TB-14 derived view is `compute_price_index(econ)` returning a
    // `BTreeMap<TxId, NodeMarketEntry>` — its empty serialization is
    // covered by the inline `empty_state_yields_empty_index` test in
    // `src/state/price_index.rs`.
}
