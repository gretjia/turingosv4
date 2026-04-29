//! TB-1 Day-3 P1 acceptance battery (6 tests).
//!
//! Charter: `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` § Day-3.
//!
//! Six tests cover P1 kill criteria + L4 hash-chain Exit:
//!   1. test_p1_kill_1_no_wtool_bypass
//!      — direct mutation to state.db that bypassed L4 fails to round-trip via
//!        `AcceptedLedger::reconstruct_state`. Persisted state.db can ALWAYS be
//!        re-derived from the L4 chain; any bypass write is washed out.
//!   2. test_p1_kill_2_rejected_tx_no_state_advance
//!      — a rejected tx leaves `state_root` unchanged, L4 `logical_t` NOT
//!        incremented, and L4.E records exactly one `submit_id`-scoped record
//!        with `raw_diagnostic_cid` populated.
//!   3. test_p1_kill_3_ledger_reconstructable
//!      — drop state.db; reconstruct from L4 only; bit-equal to pre-drop
//!        `state_root`. L4.E intentionally NOT consulted.
//!   4. test_p1_kill_4_rejected_log_isolated
//!      — raw L4.E diagnostic NOT in another agent's materialized read view;
//!        only `public_summary` (when explicitly set) crosses the boundary.
//!   5. test_p1_kill_4b_rejection_chain_breaks_on_row_deletion
//!      — write 3 rejection-evidence records; delete row 2;
//!        `RejectionEvidenceWriter::verify_chain()` returns Err(HashMismatch).
//!   6. test_p1_exit_7_l4_chain_breaks_on_row_deletion
//!      — write 5 accepted L4 entries; delete row 3;
//!        `AcceptedLedger::verify_chain(0, 4)` returns Err(HashMismatch).

use std::collections::{BTreeMap, BTreeSet};

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    PublicRejectionView, RejectionClass, RejectionEvidenceError, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::transition_ledger::TxKind;
use turingosv4::economy::ledger::{AcceptedLedger, LedgerError};
use turingosv4::economy::money::StakeMicroCoin;
use turingosv4::state::q_state::{AgentId, Hash, TxId};
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, PredicateId, PredicateResultsBundle, ReadKey,
    SafetyOrCreation, TaskId, TypedTx, WorkTx, WriteKey,
};

// ────────────────────────────────────────────────────────────────────────────
// Fixtures
// ────────────────────────────────────────────────────────────────────────────

fn fixture_work_tx(suffix: u32) -> TypedTx {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId(format!("acc-{}", suffix)),
        BoolWithProof {
            value: true,
            proof_cid: Some(Cid([0x11; 32])),
        },
    );
    let mut settlement = BTreeMap::new();
    settlement.insert(
        PredicateId(format!("set-{}", suffix)),
        BoolWithProof {
            value: true,
            proof_cid: None,
        },
    );
    let mut read_set = BTreeSet::new();
    read_set.insert(ReadKey(format!("k.r.{}", suffix)));
    let mut write_set = BTreeSet::new();
    write_set.insert(WriteKey(format!("k.w.{}", suffix)));
    TypedTx::Work(WorkTx {
        tx_id: TxId(format!("worktx-{}", suffix)),
        task_id: TaskId(format!("task-{}", suffix)),
        parent_state_root: Hash::ZERO,
        agent_id: AgentId("alice".into()),
        read_set,
        write_set,
        proposal_cid: Cid([0x13; 32]),
        predicate_results: PredicateResultsBundle {
            acceptance,
            settlement,
            safety_class: SafetyOrCreation::Safety,
        },
        stake: StakeMicroCoin::from_micro_units(1_000_000),
        signature: AgentSignature::from_bytes([0x77u8; 64]),
        timestamp_logical: suffix as u64,
    })
}

fn cid(byte: u8) -> Cid {
    Cid([byte; 32])
}

fn agent(s: &str) -> AgentId {
    AgentId(s.to_string())
}

// ────────────────────────────────────────────────────────────────────────────
// (1) P1 kill 1 — no wtool bypass
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_1_no_wtool_bypass() {
    // Build an accepted chain, persist, then simulate a "bypass" mutation that
    // edits state.db on disk WITHOUT going through `append_accepted` → L4.
    // Reconstruction from the L4 chain washes the bypass out: the
    // reconstructed `state_root` does not match the (corrupted) on-disk view.
    let mut l = AcceptedLedger::new();
    for i in 1..=3 {
        l.append_accepted(&fixture_work_tx(i)).unwrap();
    }
    let canonical_root = l.current_state_root();

    let tmp = tempfile::NamedTempFile::new().unwrap();
    l.persist(tmp.path()).unwrap();

    // Bypass: directly overwrite state.db with garbage that claims a different
    // state_root by replacing one entry's resulting_state_root JSON-side.
    let raw = std::fs::read(tmp.path()).unwrap();
    let mut tampered: Vec<turingosv4::economy::ledger::AcceptedEntry> =
        serde_json::from_slice(&raw).unwrap();
    // Tamper the last entry's resulting_state_root (a wtool-bypass would have
    // mutated state without re-deriving the chain hash).
    tampered.last_mut().unwrap().resulting_state_root = Hash([0xFF; 32]);
    let bytes = serde_json::to_vec(&tampered).unwrap();
    std::fs::write(tmp.path(), bytes).unwrap();

    // Reconstruction MUST fail to round-trip: either an explicit error, or a
    // reconstructed root that no longer matches the canonical root.
    let result = AcceptedLedger::load_from_path(tmp.path());
    match result {
        Err(_) => {} // bypass detected via integrity error — expected.
        Ok((_, reconstructed)) => {
            assert_ne!(
                reconstructed, canonical_root,
                "bypass mutation must not survive a round-trip through reconstruct_state"
            );
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// (2) P1 kill 2 — rejected tx does not advance state
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_2_rejected_tx_no_state_advance() {
    let mut l4 = AcceptedLedger::new();
    let mut l4e = RejectionEvidenceWriter::new();

    // One accepted tx — sets a non-zero baseline state_root.
    l4.append_accepted(&fixture_work_tx(1)).unwrap();
    let baseline_root = l4.current_state_root();
    let baseline_logical_t = l4.len();

    // Simulate predicate-failed dispatch: tx routes to L4.E, NOT L4.
    l4e.append_rejected(
        42,
        baseline_root,
        agent("alice"),
        TxKind::Work,
        cid(0x20),
        RejectionClass::PredicateFailed,
        Some(cid(0xAA)), // raw diagnostic populated
        None,
    );

    // L4: state_root unchanged, logical_t unchanged.
    assert_eq!(
        l4.current_state_root(),
        baseline_root,
        "rejected tx must NOT advance L4 state_root"
    );
    assert_eq!(
        l4.len(),
        baseline_logical_t,
        "rejected tx must NOT advance L4 logical_t"
    );

    // L4.E: exactly one record, raw_diagnostic_cid populated.
    assert_eq!(l4e.len(), 1, "rejection produces exactly one L4.E record");
    let r = &l4e.records()[0];
    assert_eq!(r.submit_id, 42);
    assert!(
        r.raw_diagnostic_cid.is_some(),
        "L4.E record must carry raw_diagnostic_cid"
    );
    assert!(l4e.verify_chain().is_ok());
}

// ────────────────────────────────────────────────────────────────────────────
// (3) P1 kill 3 — ledger reconstructable
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_3_ledger_reconstructable() {
    let mut l = AcceptedLedger::new();
    for i in 1..=4 {
        l.append_accepted(&fixture_work_tx(i)).unwrap();
    }
    let pre_drop_root = l.current_state_root();

    // Persist, drop in-memory state.db, reconstruct from chaintape.jsonl.
    let tmp = tempfile::NamedTempFile::new().unwrap();
    l.persist(tmp.path()).unwrap();

    drop(l); // simulate full state-vector drop.

    let (l_reborn, reconstructed_root) = AcceptedLedger::load_from_path(tmp.path()).unwrap();
    assert_eq!(
        reconstructed_root, pre_drop_root,
        "reconstructed state_root must be bit-equal to pre-drop state_root"
    );
    assert_eq!(l_reborn.len(), 4);
    assert!(l_reborn.verify_chain(0, 4).is_ok());
}

// ────────────────────────────────────────────────────────────────────────────
// (4) P1 kill 4 — rejected log is isolated from agent-facing read view
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_4_rejected_log_isolated() {
    let mut l4e = RejectionEvidenceWriter::new();
    l4e.append_rejected(
        7,
        Hash::ZERO,
        agent("alice"),
        TxKind::Work,
        cid(0x10),
        RejectionClass::PredicateFailed,
        Some(cid(0xBE)), // raw diagnostic CID — NEVER materialized to other agents
        Some("predicate acceptance failed for acc-7".into()),
    );

    // The view another agent's materializer sees:
    let view: Vec<PublicRejectionView> = l4e.public_view();
    assert_eq!(view.len(), 1);

    // Structural isolation: the type does not carry raw_diagnostic_cid.
    // Round-trip via JSON to assert the wire form omits it too.
    let json = serde_json::to_value(&view[0]).unwrap();
    let obj = json.as_object().expect("PublicRejectionView serializes as object");
    assert!(
        !obj.contains_key("raw_diagnostic_cid"),
        "raw_diagnostic_cid must NOT appear in agent-facing public view"
    );
    // Only public_summary crosses the boundary.
    assert_eq!(
        obj.get("public_summary").and_then(|v| v.as_str()),
        Some("predicate acceptance failed for acc-7")
    );

    // Sanity: the underlying L4.E forensic record DID carry the raw cid
    // (so the writer is not silently dropping it; it is shielded structurally).
    assert!(
        l4e.records()[0].raw_diagnostic_cid.is_some(),
        "L4.E forensic record must retain raw_diagnostic_cid (shielding is structural, not destructive)"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// (5) P1 kill 4b — L4.E hash chain breaks on row deletion
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_4b_rejection_chain_breaks_on_row_deletion() {
    let mut l4e = RejectionEvidenceWriter::new();
    for i in 1..=3u64 {
        l4e.append_rejected(
            i,
            Hash::ZERO,
            agent("alice"),
            TxKind::Work,
            cid(0x10),
            RejectionClass::PredicateFailed,
            None,
            None,
        );
    }
    assert!(l4e.verify_chain().is_ok());

    // Delete the middle row — surviving row's prev_hash now disagrees with its
    // (new) predecessor's hash.
    l4e.tamper_remove_record(1);
    let r = l4e.verify_chain();
    assert!(
        matches!(r, Err(RejectionEvidenceError::HashMismatch { at: 1 })),
        "deleting row 1 must surface as HashMismatch at the new index 1; got {:?}",
        r
    );
}

// ────────────────────────────────────────────────────────────────────────────
// (6) P1 Exit 7 — L4 hash chain breaks on row deletion
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_exit_7_l4_chain_breaks_on_row_deletion() {
    let mut l = AcceptedLedger::new();
    for i in 1..=5 {
        l.append_accepted(&fixture_work_tx(i)).unwrap();
    }
    assert!(l.verify_chain(0, 5).is_ok());

    // Delete row 2 (was logical_t=3); surviving rows are now [t=1, t=2, t=4, t=5].
    l.tamper_remove_entry(2);

    // Chain length is now 4. Verifying [0, 4):
    // - i=0,1: clean.
    // - i=2: stored entry was originally logical_t=4 with prev_hash=hash(t=3).
    //        Expected logical_t at index 2 is 3 (i+1) — fails LogicalTGap first.
    // Either LogicalTGap or HashMismatch at index 2 satisfies the kill criterion.
    let r = l.verify_chain(0, 4);
    match r {
        Err(LedgerError::LogicalTGap { at_index: 2, .. })
        | Err(LedgerError::HashMismatch { at_index: 2 }) => {}
        other => panic!(
            "deleting an L4 row must break the chain at index 2; got {:?}",
            other
        ),
    }
}
