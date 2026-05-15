//! CO1.7-extra D4 (substrate-independent): Git2LedgerWriter
//! head_commit_oid_hex returns Some after commit.
//!
//! Defensive against silent head_t stagnation per round-2 MF3 + Gemini Q8
//! (round-1) concern: if Git2LedgerWriter ever inherits a default
//! behavior (now impossible given v1.2 trait method is REQUIRED — no
//! default impl), this test catches it. Belt-and-suspenders for the
//! constitutional anchor (head_t per Art 0.4).
//!
//! Round-2 MF7: `entry_at` helper at transition_ledger.rs:813 is private
//! to module tests; integration tests construct LedgerEntry inline using
//! public surfaces.

use std::collections::BTreeMap;
use turingosv4::bottom_white::cas::Cid;
use turingosv4::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
use turingosv4::bottom_white::ledger::transition_ledger::{
    Git2LedgerWriter, LedgerEntry, LedgerWriter, TxKind,
};
use turingosv4::state::q_state::Hash;

#[test]
fn git2_writer_returns_some_after_commit() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let mut writer = Git2LedgerWriter::open(tmp.path()).expect("open");

    // Inline LedgerEntry construction (round-2 MF7) — uses only public
    // CO1.7-impl surfaces. Resulting_ledger_root is non-zero so the writer
    // commit produces a meaningful tree blob.
    let entry = LedgerEntry {
        logical_t: 1,
        parent_state_root: Hash::ZERO,
        parent_ledger_root: Hash::ZERO,
        tx_kind: TxKind::Work,
        tx_payload_cid: Cid([0u8; 32]),
        resulting_state_root: Hash::ZERO,
        resulting_ledger_root: Hash([1u8; 32]),
        timestamp_logical: 1,
        epoch: SystemEpoch::new(1),
        extensions: BTreeMap::new(),
        system_signature: SystemSignature::from_bytes([0u8; 64]),
    };

    writer.commit(&entry).expect("commit");

    // Constitutional anchor invariant: Git2LedgerWriter MUST return Some
    // after a successful commit. If this ever returns None, q.head_t
    // would silently stagnate at genesis — a constitutional Art 0.4
    // violation.
    assert!(
        writer.head_commit_oid_hex().is_some(),
        "Git2LedgerWriter MUST return Some after commit; constitutional anchor violation otherwise"
    );

    // Sanity: hex length is 40 (canonical git OID length).
    let hex = writer.head_commit_oid_hex().unwrap();
    assert_eq!(
        hex.len(),
        40,
        "git OID hex must be 40 chars; got {}",
        hex.len()
    );
    assert!(
        hex.chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()),
        "git OID hex must be lowercase hexadecimal; got {hex}"
    );
}
