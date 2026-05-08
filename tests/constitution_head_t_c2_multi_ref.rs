//! Stage A3 — HEAD_t C2 multi-ref ChainTape ship gates.
//!
//! Per `STAGE_A3_HEAD_T_C2_charter_2026-05-07.md` §4 Ship Gates:
//!
//! ```text
//! SG-A3-HEAD-T-C2.1  L4 head ref advances on accepted transition
//! SG-A3-HEAD-T-C2.2  L4.E head ref advances on rejected evidence
//! SG-A3-HEAD-T-C2.3  CAS root ref advances when CAS evidence added
//! SG-A3-HEAD-T-C2.4  Replay reconstructs HEAD_t (six-field byte equality)
//! SG-A3-HEAD-T-C2.5  No hidden filesystem pointer
//! ```
//!
//! These tests are the executable face of FR-A3-HEAD-T-C2.1..7 +
//! CR-A3-HEAD-T-C2.5 (the three named Git refs ARE the canonical pointer).
//! The C1 baseline `refs/transitions/main` is preserved as a backward-compat
//! alias; every accepted transition dual-writes to `refs/chaintape/l4`.
//!
//! `FC-trace: FC2-INV1 + Art-0.4 + G-009 Path C C2`.

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use git2::Repository;
use turingosv4::bottom_white::ledger::transition_ledger::{
    append, Git2LedgerWriter, LedgerEntry, LedgerEntrySigningPayload, LedgerWriter, TxKind,
    CHAINTAPE_CAS_REF, CHAINTAPE_L4E_REF, CHAINTAPE_L4_REF,
};
use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
use turingosv4::state::head_t_witness::HeadTWitness;
use turingosv4::state::q_state::Hash;

static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn fresh_repo() -> (tempfile::TempDir, PathBuf, Git2LedgerWriter) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let path = tmp.path().to_path_buf();
    let writer = Git2LedgerWriter::open(&path).expect("open");
    (tmp, path, writer)
}

/// Build a minimally-valid LedgerEntry at logical_t with deterministic fields.
fn entry_at(
    logical_t: u64,
    parent_state_root: Hash,
    parent_ledger_root: Hash,
    resulting_state_root: Hash,
) -> LedgerEntry {
    let signing = LedgerEntrySigningPayload {
        logical_t,
        parent_state_root,
        parent_ledger_root,
        tx_kind: TxKind::Work,
        tx_payload_cid: Cid([0u8; 32]),
        resulting_state_root,
        timestamp_logical: logical_t,
        epoch: SystemEpoch::new(1),
        extensions: std::collections::BTreeMap::new(),
    };
    let signing_digest = signing.canonical_digest();
    let resulting_ledger_root = append(&parent_ledger_root, &signing_digest);
    LedgerEntry {
        logical_t,
        parent_state_root: signing.parent_state_root,
        parent_ledger_root: signing.parent_ledger_root,
        tx_kind: signing.tx_kind,
        tx_payload_cid: signing.tx_payload_cid,
        resulting_state_root: signing.resulting_state_root,
        resulting_ledger_root,
        timestamp_logical: signing.timestamp_logical,
        epoch: signing.epoch,
        extensions: signing.extensions,
        system_signature: SystemSignature::from_bytes([0u8; 64]),
    }
}

/// Helper — synthesize a Git commit OID by writing a simple blob+tree+commit.
/// Used for SG-A3.2/3 to advance the L4.E and CAS refs to deterministic OIDs
/// without requiring the full L4.E or CAS subsystems to participate.
fn synth_commit_oid(repo_path: &Path, marker: &str) -> git2::Oid {
    let repo = Repository::open(repo_path).expect("open repo");
    let counter = TEST_COUNTER.fetch_add(1, Ordering::SeqCst) as i64;
    let blob = repo.blob(marker.as_bytes()).expect("blob");
    let mut tb = repo.treebuilder(None).expect("treebuilder");
    tb.insert("payload", blob, 0o100644).expect("tree insert");
    let tree_oid = tb.write().expect("tree write");
    let tree = repo.find_tree(tree_oid).expect("find_tree");
    // Use deterministic time so OIDs are reproducible.
    let time = git2::Time::new(counter, 0);
    let sig = git2::Signature::new("test", "test@local", &time).expect("sig");
    let commit_oid = repo
        .commit(None, &sig, &sig, marker, &tree, &[])
        .expect("commit");
    commit_oid
}

/// SG-A3-HEAD-T-C2.1 — accepting one L4 transition advances `refs/chaintape/l4`
/// and the new ref OID equals the C1 alias OID (dual-write semantics).
#[test]
fn sg_a3_l4_head_ref_advances_on_accepted_transition() {
    let (_tmp, path, mut writer) = fresh_repo();

    // Pre-state: no chaintape/l4 ref.
    let pre = Git2LedgerWriter::head_chaintape_l4(&path).expect("read pre");
    assert!(pre.is_none(), "fresh repo must have no chaintape/l4 ref");

    // Append one entry; C1 + C2 dual-write fires.
    let e = entry_at(1, Hash::ZERO, Hash::ZERO, Hash([0x11; 32]));
    let _ = writer.commit(&e).expect("commit");

    // Post-state: chaintape/l4 exists and equals C1 head.
    let l4_post = Git2LedgerWriter::head_chaintape_l4(&path)
        .expect("read l4")
        .expect("l4 ref present after commit");
    let c1_post = writer.head_commit_oid().expect("c1 head");
    assert_eq!(
        l4_post, c1_post,
        "dual-write: refs/chaintape/l4 OID must equal refs/transitions/main OID"
    );

    // A second commit advances the ref.
    let e2 = entry_at(2, Hash([0x11; 32]), e.resulting_ledger_root, Hash([0x22; 32]));
    let _ = writer.commit(&e2).expect("commit-2");
    let l4_post_2 = Git2LedgerWriter::head_chaintape_l4(&path)
        .expect("read l4-2")
        .expect("l4 ref present after commit-2");
    assert_ne!(
        l4_post, l4_post_2,
        "refs/chaintape/l4 must advance on each accepted transition"
    );
    assert_eq!(
        l4_post_2,
        writer.head_commit_oid().expect("c1 head 2"),
        "dual-write semantics must hold after 2nd commit"
    );
}

/// SG-A3-HEAD-T-C2.2 — `advance_chaintape_l4e_to` advances `refs/chaintape/l4e`
/// to the supplied OID, idempotently overwriting any prior value.
#[test]
fn sg_a3_l4e_head_ref_advances_on_rejected_evidence() {
    let (_tmp, path, _writer) = fresh_repo();

    // Pre: no l4e ref.
    let pre = Git2LedgerWriter::head_chaintape_l4e(&path).expect("read pre");
    assert!(pre.is_none(), "fresh repo must have no chaintape/l4e ref");

    // Synthesize a deterministic L4.E commit OID, then advance the ref.
    let oid_a = synth_commit_oid(&path, "l4e_record_1");
    Git2LedgerWriter::advance_chaintape_l4e_to(&path, oid_a, "L4.E append #1")
        .expect("advance l4e");
    let post_a = Git2LedgerWriter::head_chaintape_l4e(&path)
        .expect("read l4e")
        .expect("l4e ref present after advance");
    assert_eq!(post_a, oid_a);

    // A second advance moves the ref atomically.
    let oid_b = synth_commit_oid(&path, "l4e_record_2");
    Git2LedgerWriter::advance_chaintape_l4e_to(&path, oid_b, "L4.E append #2")
        .expect("advance l4e #2");
    let post_b = Git2LedgerWriter::head_chaintape_l4e(&path)
        .expect("read l4e #2")
        .expect("l4e ref present after second advance");
    assert_eq!(post_b, oid_b);
    assert_ne!(post_a, post_b, "l4e ref must advance on each rejected evidence record");
}

/// SG-A3-HEAD-T-C2.3 — `advance_chaintape_cas_to` advances `refs/chaintape/cas`
/// to the supplied OID per CAS write batch.
#[test]
fn sg_a3_cas_root_ref_advances_on_cas_write() {
    let (_tmp, path, _writer) = fresh_repo();

    let pre = Git2LedgerWriter::head_chaintape_cas(&path).expect("read pre");
    assert!(pre.is_none(), "fresh repo must have no chaintape/cas ref");

    let oid_a = synth_commit_oid(&path, "cas_batch_1");
    Git2LedgerWriter::advance_chaintape_cas_to(&path, oid_a, "CAS write #1")
        .expect("advance cas");
    let post_a = Git2LedgerWriter::head_chaintape_cas(&path)
        .expect("read cas")
        .expect("cas ref present after advance");
    assert_eq!(post_a, oid_a);

    let oid_b = synth_commit_oid(&path, "cas_batch_2");
    Git2LedgerWriter::advance_chaintape_cas_to(&path, oid_b, "CAS write #2")
        .expect("advance cas #2");
    let post_b = Git2LedgerWriter::head_chaintape_cas(&path)
        .expect("read cas #2")
        .expect("cas ref present after second advance");
    assert_eq!(post_b, oid_b);
    assert_ne!(post_a, post_b);
}

/// SG-A3-HEAD-T-C2.4 — Replay reconstructs HEAD_t from refs alone (six-field
/// byte equality) without requiring `&QState`. The L4 head, L4.E head, and
/// CAS root come purely from `refs/chaintape/{l4,l4e,cas}`.
#[test]
fn sg_a3_replay_reconstructs_head_t_from_refs() {
    let (_tmp, path, mut writer) = fresh_repo();

    // Build a small history: 1 L4 commit + 1 L4.E advance + 1 CAS advance.
    let e = entry_at(1, Hash::ZERO, Hash::ZERO, Hash([0x11; 32]));
    let _ = writer.commit(&e).expect("commit");
    let l4e_oid = synth_commit_oid(&path, "l4e_replay_test");
    Git2LedgerWriter::advance_chaintape_l4e_to(&path, l4e_oid, "L4.E for replay")
        .expect("advance l4e");
    let cas_oid = synth_commit_oid(&path, "cas_replay_test");
    Git2LedgerWriter::advance_chaintape_cas_to(&path, cas_oid, "CAS for replay")
        .expect("advance cas");

    // Reconstruct using a fixed state_root + economic_state_root + run_id.
    // (These come from QState replay in production; we synthesize them here
    // to test the ref-side reconstruction in isolation.)
    let state_root = Hash([0x33; 32]);
    let econ_root = Hash([0x44; 32]);
    let w1 = HeadTWitness::reconstruct_from_chaintape_refs(
        &path,
        "test-run-1",
        state_root,
        econ_root,
    )
    .expect("reconstruct ok")
    .expect("non-empty witness");

    // Six canonical fields populated from refs.
    assert_eq!(w1.state_root, state_root);
    assert_eq!(
        w1.l4_head.0,
        writer.head_commit_oid().expect("c1 head").to_string(),
        "l4_head must equal refs/chaintape/l4 OID"
    );
    assert!(w1.l4e_head.is_some(), "l4e_head must reflect refs/chaintape/l4e");
    assert_eq!(
        w1.l4e_head.as_ref().unwrap().0,
        l4e_oid.to_string(),
        "l4e_head must match the advanced ref"
    );
    assert!(w1.cas_root.is_some(), "cas_root must reflect refs/chaintape/cas");
    assert_eq!(w1.economic_state_root, econ_root);
    assert_eq!(w1.run_id, "test-run-1");

    // Byte-equality on canonical hash for two reconstructions with identical refs.
    let w2 = HeadTWitness::reconstruct_from_chaintape_refs(
        &path,
        "test-run-1",
        state_root,
        econ_root,
    )
    .expect("reconstruct ok")
    .expect("non-empty witness");
    assert_eq!(w1, w2, "reconstruction must be deterministic from refs");
    assert_eq!(
        w1.canonical_hash(),
        w2.canonical_hash(),
        "FR-A3-HEAD-T-C2.4 byte-equality witness"
    );

    // Pre-genesis (no l4 ref) returns None.
    let (_tmp_empty, empty_path, _w_empty) = fresh_repo();
    let w_empty = HeadTWitness::reconstruct_from_chaintape_refs(
        &empty_path,
        "test-run-empty",
        state_root,
        econ_root,
    )
    .expect("reconstruct on empty ok");
    assert!(w_empty.is_none(), "pre-genesis reconstruction must return None");
}

/// SG-A3-HEAD-T-C2.5 — No hidden filesystem pointer. The three named Git
/// refs ARE the canonical pointer (per CR-A3-HEAD-T-C2.5). This test grep-
/// checks the source tree for forbidden filesystem-pointer file names.
#[test]
fn sg_a3_no_hidden_filesystem_pointer() {
    let forbidden = [
        "LATEST_HEAD_T.txt",
        "CURRENT_RUN.json",
        "GLOBAL_HEAD_POINTER",
        "RUN_LATEST_POINTER",
        "head_t_pointer.txt",
    ];
    // Walk src/ and check no .rs file references a forbidden pointer file.
    let src = Path::new("src");
    walk_and_check(src, &forbidden);
    // Also ensure the matrix does not declare a forbidden pointer as canonical.
    let matrix_path = "handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md";
    if Path::new(matrix_path).exists() {
        let body = fs::read_to_string(matrix_path).expect("matrix");
        for f in &forbidden {
            assert!(
                !body.contains(f),
                "CR-A3-HEAD-T-C2.5 violation: matrix references forbidden pointer `{f}`"
            );
        }
    }
}

fn walk_and_check(dir: &Path, forbidden: &[&str]) {
    if !dir.exists() {
        return;
    }
    let entries = fs::read_dir(dir).expect("read_dir");
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_and_check(&path, forbidden);
        } else if path.extension().map(|e| e == "rs").unwrap_or(false) {
            let body = match fs::read_to_string(&path) {
                Ok(s) => s,
                Err(_) => continue,
            };
            for f in forbidden {
                // Ignore comments that explicitly mention forbidden names as
                // banned (so this very file's forbidden-list doesn't trip).
                let is_test_file = path
                    .file_name()
                    .map(|n| n.to_string_lossy().contains("constitution_head_t_c2"))
                    .unwrap_or(false);
                if is_test_file {
                    continue;
                }
                assert!(
                    !body.contains(f),
                    "CR-A3-HEAD-T-C2.5 violation: {} contains forbidden pointer name `{f}`",
                    path.display()
                );
            }
        }
    }
}

/// Regression — REF constants pinned at canonical names per FR-A3-HEAD-T-C2.1.
/// A future rename would silently break replay-from-refs; this fires the gate.
#[test]
fn sg_a3_ref_name_constants_pinned() {
    assert_eq!(CHAINTAPE_L4_REF, "refs/chaintape/l4");
    assert_eq!(CHAINTAPE_L4E_REF, "refs/chaintape/l4e");
    assert_eq!(CHAINTAPE_CAS_REF, "refs/chaintape/cas");
}
