//! TRACE_MATRIX Art 0.4 / WP § 4 — Q_t reconstructibility.
//!
//! Q_t MUST be reconstructible from the L4 transition ledger replay; as a
//! prerequisite, Q_t MUST round-trip through the canonical serialization
//! rule (per spec § 2.5: bincode v2 big-endian + BTreeMap lex; serde_json
//! used here as a deterministic-for-BTreeMap interim until CO1.1.4-pre1 land
//! the bincode fixture corpus).

use turingosv4::economy::money::MicroCoin;
use turingosv4::state::{
    AgentId, BalancesIndex, EconomicState, Hash, NodeId, PerAgentState, QState, Reputation, TxId,
};

#[test]
fn genesis_round_trips_via_serde_json() {
    let g = QState::genesis();
    let s = serde_json::to_string(&g).unwrap();
    let back: QState = serde_json::from_str(&s).unwrap();
    assert_eq!(g, back);
}

#[test]
fn populated_q_state_round_trips() {
    let mut q = QState::genesis();
    q.q_t.current_round = 7;
    q.q_t.agents.insert(
        AgentId("alice".into()),
        PerAgentState {
            reputation_snapshot: Reputation(42),
            last_accepted_tx: Some(TxId("tx-001".into())),
            retry_counter_for_current_task: 3,
        },
    );
    q.head_t = NodeId("deadbeefcafe".into());
    q.state_root_t = Hash::from_bytes([0xAB; 32]);
    q.economic_state_t.balances_t.0.insert(
        AgentId("alice".into()),
        MicroCoin::from_coin(100).unwrap(),
    );

    let s = serde_json::to_string(&q).unwrap();
    let back: QState = serde_json::from_str(&s).unwrap();
    assert_eq!(q, back, "QState must be reconstruct-equivalent after serde round-trip");
}

#[test]
fn balances_index_insertion_order_independence() {
    // Inv determinism: BTreeMap-backed indices serialize identically regardless
    // of insertion order (Codex round-2 flagged HashMap nondeterminism).
    let mut a = BalancesIndex::default();
    a.0.insert(AgentId("a".into()), MicroCoin::from_coin(1).unwrap());
    a.0.insert(AgentId("b".into()), MicroCoin::from_coin(2).unwrap());
    a.0.insert(AgentId("c".into()), MicroCoin::from_coin(3).unwrap());

    let mut b = BalancesIndex::default();
    b.0.insert(AgentId("c".into()), MicroCoin::from_coin(3).unwrap());
    b.0.insert(AgentId("a".into()), MicroCoin::from_coin(1).unwrap());
    b.0.insert(AgentId("b".into()), MicroCoin::from_coin(2).unwrap());

    assert_eq!(serde_json::to_string(&a).unwrap(), serde_json::to_string(&b).unwrap());
}

#[test]
fn nine_top_level_fields() {
    let v = serde_json::to_value(QState::genesis()).unwrap();
    let obj = v.as_object().unwrap();
    let expected = [
        "q_t",
        "head_t",
        "state_root_t",
        "tape_view_t",
        "ledger_root_t",
        "predicate_registry_root_t",
        "tool_registry_root_t",
        "economic_state_t",
        "budget_state_t",
    ];
    assert_eq!(obj.len(), 9, "WP § 4 mandates exactly 9 fields");
    for k in expected.iter() {
        assert!(obj.contains_key(*k), "missing field {}", k);
    }
}

#[test]
fn empty_economic_state_serializes_to_twelve_sub_fields() {
    // TB-11 (architect §6.2 ruling 2026-05-02): 9 → 10 (+runs_t).
    // TB-12 (architect 2026-05-03 ruling §3 + §8 Atom 1): 10 → 11 (+node_positions_t).
    // TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling §4.3):
    //   11 → 13 (+conditional_collateral_t +conditional_share_balances_t).
    // TB-14 Atom 2 (2026-05-03; architect §5.1 + charter §7 auto-resolution A):
    //   13 → 12 (-price_index_t legacy stub; TB-14 derives the price view via
    //   `compute_price_index` pure fn, not stored as canonical state — "price
    //   is signal, not truth"; no second source-of-truth).
    let e = EconomicState::default();
    let v = serde_json::to_value(&e).unwrap();
    let obj = v.as_object().unwrap();
    assert_eq!(obj.len(), 12);
}
