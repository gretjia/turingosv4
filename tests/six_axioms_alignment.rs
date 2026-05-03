//! TRACE_MATRIX WP § 0 — six design axioms alignment with QState.
//!
//! Per `TRACE_MATRIX_v3 § 0 设计公理`: Q_t represents axiom 1 (state monotonicity);
//! `top_white::predicates::*` represents axiom 2 (predicate-as-judge);
//! `economy::*` represents axiom 3 (economic alignment); etc. This conformance
//! test asserts that the QState surface exposes the slots WP § 0 requires.

use turingosv4::economy::money::MicroCoin;
use turingosv4::state::{AgentId, PerAgentState, QState, Reputation, TxId};

/// Axiom 1: state monotonicity — Q_t evolves only via accepted transitions.
/// Witness: `QState::default()` is a total starting state; modifying agent
/// runtime state preserves all 9 fields.
#[test]
fn axiom_1_state_monotonicity_witness() {
    let g = QState::genesis();
    let mut q = g.clone();
    q.q_t.current_round = 1;
    q.q_t.agents.insert(
        AgentId("a".into()),
        PerAgentState {
            reputation_snapshot: Reputation(0),
            last_accepted_tx: None,
            retry_counter_for_current_task: 0,
        },
    );
    // After one transition, the OTHER 8 fields remain at genesis-default
    // (only the touched sub-state changed). Inv state-cohesion.
    assert_eq!(q.head_t, g.head_t);
    assert_eq!(q.state_root_t, g.state_root_t);
    assert_eq!(q.tape_view_t, g.tape_view_t);
    assert_eq!(q.ledger_root_t, g.ledger_root_t);
    assert_eq!(q.predicate_registry_root_t, g.predicate_registry_root_t);
    assert_eq!(q.tool_registry_root_t, g.tool_registry_root_t);
    assert_eq!(q.economic_state_t, g.economic_state_t);
    assert_eq!(q.budget_state_t, g.budget_state_t);
}

/// Axiom 2: predicate-as-judge — Q_t exposes `predicate_registry_root_t` slot.
/// (Predicate registry implementation lives in `top_white::predicates`; here
/// we verify Q_t carries the canonical anchor.)
#[test]
fn axiom_2_predicate_registry_root_present() {
    let g = QState::genesis();
    let v = serde_json::to_value(&g).unwrap();
    assert!(v.as_object().unwrap().contains_key("predicate_registry_root_t"));
}

/// Axiom 3: economic alignment — Q_t exposes a 12-sub-field economic state.
/// TB-11 (architect §6.2 ruling 2026-05-02) bumped 9 → 10 with +runs_t.
/// TB-12 (architect 2026-05-03 ruling §3 + §8 Atom 1) bumped 10 → 11 with
/// +node_positions_t (flat NodePositionsIndex; canonical exposure record
/// state; NOT NodeMarketEntry which is TB-14 derived view).
/// TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling §4.3) bumped
/// 11 → 13 with +conditional_collateral_t (Coin holding per CR-13.4) +
/// conditional_share_balances_t (claims per CR-13.3).
/// TB-14 Atom 2 (2026-05-03; architect §5.1) trimmed 13 → 12 by removing the
/// legacy `price_index_t` stub; TB-14 provides `compute_price_index`
/// pure-fn derived view (charter §7 auto-resolution A: no second
/// source-of-truth).
#[test]
fn axiom_3_economic_state_present_and_complete() {
    let g = QState::genesis();
    let e = serde_json::to_value(&g.economic_state_t).unwrap();
    assert_eq!(e.as_object().unwrap().len(), 12);
}

/// Axiom 4: tool capability — Q_t exposes `tool_registry_root_t` slot.
#[test]
fn axiom_4_tool_registry_root_present() {
    let g = QState::genesis();
    let v = serde_json::to_value(&g).unwrap();
    assert!(v.as_object().unwrap().contains_key("tool_registry_root_t"));
}

/// Axiom 5: tape canonical — Q_t exposes `head_t` (chain head) and
/// `ledger_root_t` (Merkle root over accepted tx).
#[test]
fn axiom_5_tape_canonical_anchors_present() {
    let g = QState::genesis();
    let v = serde_json::to_value(&g).unwrap();
    let obj = v.as_object().unwrap();
    assert!(obj.contains_key("head_t"));
    assert!(obj.contains_key("ledger_root_t"));
}

/// Axiom 6: bounded compute — Q_t exposes `budget_state_t`.
#[test]
fn axiom_6_budget_snapshot_present() {
    let g = QState::genesis();
    let v = serde_json::to_value(&g).unwrap();
    assert!(v.as_object().unwrap().contains_key("budget_state_t"));
}

/// All six axiom anchors land in a single Q_t value.
#[test]
fn six_axioms_total_in_q_state() {
    let mut q = QState::genesis();
    // Touch every axiom slot to confirm each is a real, mutable lvalue.
    q.q_t.current_round = 1; // axiom 1
    q.predicate_registry_root_t = q.predicate_registry_root_t; // axiom 2
    q.economic_state_t.balances_t.0.insert(
        AgentId("seed".into()),
        MicroCoin::from_coin(1).unwrap(),
    ); // axiom 3
    q.tool_registry_root_t = q.tool_registry_root_t; // axiom 4
    q.head_t = q.head_t.clone(); // axiom 5
    q.budget_state_t.compute_cap_remaining = 100; // axiom 6

    let _ = serde_json::to_string(&q).unwrap();
    let _ = TxId::default(); // touch TxId so unused-import lint stays clean
}
