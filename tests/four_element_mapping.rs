//! TRACE_MATRIX Art 0.1 — 四要素映射 (Tape / Input-Tape / Q / State).
//!
//! Constitution Art 0.1 names four canonical elements; QState § 1.1 gives the
//! white-box runtime mapping. This conformance test verifies the mapping is
//! complete and accessible.

use turingosv4::state::{AgentSwarmState, AgentVisibleProjection, Hash, NodeId, QState};

#[test]
fn tape_element_maps_to_head_t() {
    // "Tape" = the L4 chain head pointer (canonical history).
    let q = QState::genesis();
    let _: &NodeId = &q.head_t;
    assert_eq!(q.head_t, NodeId::default());
}

#[test]
fn input_tape_element_maps_to_tape_view_t() {
    // "Input-Tape" = the projection an agent reads (filtered tape view).
    let q = QState::genesis();
    let _: &AgentVisibleProjection = &q.tape_view_t;
    assert!(q.tape_view_t.views.is_empty());
}

#[test]
fn q_control_element_maps_to_q_t() {
    // "Q" = control sub-state (agent swarm + per-agent runtime state).
    let q = QState::genesis();
    let _: &AgentSwarmState = &q.q_t;
    assert_eq!(q.q_t.current_round, 0);
    assert!(q.q_t.agents.is_empty());
}

#[test]
fn state_element_maps_to_state_root_t() {
    // "State" = the materialized state Merkle root.
    let q = QState::genesis();
    let _: &Hash = &q.state_root_t;
    assert_eq!(q.state_root_t, Hash::ZERO);
}

#[test]
fn four_element_mapping_total() {
    // All four canonical elements have a typed slot in QState (no missing element).
    let q = QState::genesis();
    let _ = (&q.q_t, &q.head_t, &q.state_root_t, &q.tape_view_t);
    assert_eq!(q, QState::default());
}
