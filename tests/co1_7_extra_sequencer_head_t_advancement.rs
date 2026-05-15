//! CO1.7-extra D4 (substrate-independent): verify advance_head_t correctly
//! advances q.head_t when writer surfaces a commit OID, and preserves
//! q.head_t when writer returns None.
//!
//! Substrate-independent: uses only LedgerWriter trait + QState + a mock
//! writer. Closes round-2 MF2 (the actual D2 code path was untested in
//! v1; this test validates the advance_head_t helper semantics in
//! isolation, without requiring CO1.7.5 transition bodies).

use std::sync::Mutex;
use turingosv4::bottom_white::ledger::transition_ledger::{
    LedgerEntry, LedgerWriter, LedgerWriterError,
};
use turingosv4::state::q_state::{Hash, NodeId, QState};
use turingosv4::state::sequencer::advance_head_t;

/// Mock LedgerWriter — returns a configurable head_commit_oid_hex value.
/// Stubs commit() to always succeed (returns dummy Hash). Required trait
/// method head_commit_oid_hex is explicitly declared (no default to
/// inherit per round-2 MF3 / v1.2 trait change).
struct MockLedgerWriter {
    head_oid: Mutex<Option<String>>,
    len: Mutex<u64>,
}

impl MockLedgerWriter {
    fn new(head_oid: Option<String>) -> Self {
        Self {
            head_oid: Mutex::new(head_oid),
            len: Mutex::new(0),
        }
    }
}

impl LedgerWriter for MockLedgerWriter {
    fn commit(&mut self, _entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
        let mut len_guard = self.len.lock().expect("len lock");
        *len_guard += 1;
        Ok(Hash([0xAB; 32]))
    }

    fn read_at(&self, _logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
        unimplemented!(
            "MockLedgerWriter does not support read_at — not needed for advance_head_t test"
        )
    }

    fn len(&self) -> u64 {
        *self.len.lock().expect("len lock")
    }

    fn head_commit_oid_hex(&self) -> Option<String> {
        self.head_oid.lock().expect("head_oid lock").clone()
    }
}

#[test]
fn advance_head_t_writes_node_id_when_writer_returns_some() {
    let writer = MockLedgerWriter::new(Some("a".repeat(40))); // 40-char hex literal
    let mut q = QState::genesis();
    let q_initial_head = q.head_t.clone();

    advance_head_t(&mut q, &writer);

    // Post-condition: q.head_t = NodeId("aaaa...aaaa")
    assert_eq!(q.head_t.0, "a".repeat(40));
    assert_ne!(
        q.head_t, q_initial_head,
        "advance_head_t must advance head_t when writer returns Some"
    );
}

#[test]
fn advance_head_t_preserves_node_id_when_writer_returns_none() {
    let writer = MockLedgerWriter::new(None);
    let mut q = QState::genesis();
    let q_initial_head = q.head_t.clone();

    advance_head_t(&mut q, &writer);

    // Post-condition: q.head_t unchanged (no-op preservation per § 1.1).
    assert_eq!(
        q.head_t, q_initial_head,
        "advance_head_t must preserve head_t when writer returns None"
    );
}

#[test]
fn advance_head_t_overwrites_prior_node_id_on_subsequent_some() {
    // Simulates two successive commits: first commit advances to OID-A,
    // second commit advances to OID-B; q.head_t must reflect the latest.
    let writer = MockLedgerWriter::new(Some("a".repeat(40)));
    let mut q = QState::genesis();

    advance_head_t(&mut q, &writer);
    assert_eq!(q.head_t, NodeId("a".repeat(40)));

    // Update writer's head_commit_oid_hex to OID-B (simulates next commit).
    *writer.head_oid.lock().expect("lock") = Some("b".repeat(40));
    advance_head_t(&mut q, &writer);
    assert_eq!(q.head_t, NodeId("b".repeat(40)));
}
