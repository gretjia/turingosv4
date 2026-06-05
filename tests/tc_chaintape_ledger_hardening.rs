use std::collections::BTreeMap;
use std::process::Command;

use tempfile::TempDir;

use turingosv4::bottom_white::cas::Cid;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    RejectionClass, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
use turingosv4::bottom_white::ledger::transition_ledger::{
    append, Git2LedgerWriter, LedgerEntry, LedgerEntrySigningPayload, LedgerWriter,
    LedgerWriterError, TxKind,
};
use turingosv4::state::q_state::{AgentId, Hash};

struct EnvGuard {
    key: &'static str,
    old: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: &std::path::Path) -> Self {
        let old = std::env::var(key).ok();
        std::env::set_var(key, value);
        Self { key, old }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.old {
            Some(value) => std::env::set_var(self.key, value),
            None => std::env::remove_var(self.key),
        }
    }
}

fn h(byte: u8) -> Hash {
    Hash([byte; 32])
}

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
        extensions: BTreeMap::new(),
    };
    let signing_digest = signing.canonical_digest();
    let resulting_ledger_root = append(&parent_ledger_root, &signing_digest);
    LedgerEntry {
        logical_t: signing.logical_t,
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

fn assert_git_fsck_clean(repo_path: &std::path::Path) {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("fsck")
        .arg("--full")
        .output()
        .expect("run git fsck --full");
    assert!(
        output.status.success(),
        "git fsck --full failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn l4_commit_writes_canonical_head_alias_and_fsck_clean() {
    let tmp = TempDir::new().expect("tempdir");
    let mut writer = Git2LedgerWriter::open(tmp.path()).expect("open writer");
    let entry = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));

    writer.commit(&entry).expect("commit entry");

    let l4_head = Git2LedgerWriter::head_chaintape_l4(tmp.path())
        .expect("read l4 head")
        .expect("l4 head after commit");
    assert_eq!(
        writer.head_commit_oid(),
        Some(l4_head),
        "writer head must equal canonical refs/chaintape/l4"
    );

    let repo = git2::Repository::open(tmp.path()).expect("open generated repo");
    let alias_head = repo
        .find_reference("refs/transitions/main")
        .expect("C1 alias ref")
        .target()
        .expect("C1 alias target");
    assert_eq!(
        alias_head, l4_head,
        "refs/transitions/main must mirror canonical refs/chaintape/l4"
    );
    assert_eq!(writer.read_at(1).expect("read entry"), entry);
    assert_git_fsck_clean(tmp.path());
}

#[test]
fn divergent_transition_alias_repairs_from_canonical_l4() {
    let tmp = TempDir::new().expect("tempdir");
    let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
    let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
    let (oid_1, oid_2) = {
        let mut writer = Git2LedgerWriter::open(tmp.path()).expect("open writer");
        writer.commit(&e1).expect("commit 1");
        let oid_1 = writer.head_commit_oid().expect("head 1");
        writer.commit(&e2).expect("commit 2");
        let oid_2 = writer.head_commit_oid().expect("head 2");
        (oid_1, oid_2)
    };

    let repo = git2::Repository::open(tmp.path()).expect("open generated repo");
    repo.reference(
        "refs/transitions/main",
        oid_1,
        true,
        "A04 test: force stale C1 alias",
    )
    .expect("force stale alias");

    let reopened = Git2LedgerWriter::open(tmp.path()).expect("reopen repairs alias");
    assert_eq!(reopened.len(), 2);
    assert_eq!(reopened.head_commit_oid(), Some(oid_2));

    let repaired_alias = repo
        .find_reference("refs/transitions/main")
        .expect("repaired alias")
        .target()
        .expect("repaired alias target");
    assert_eq!(
        repaired_alias, oid_2,
        "C1 alias must be repaired from canonical C2 head"
    );
}

#[test]
fn logical_t_gap_fails_without_advancing_canonical_head() {
    let tmp = TempDir::new().expect("tempdir");
    let mut writer = Git2LedgerWriter::open(tmp.path()).expect("open writer");
    let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
    writer.commit(&e1).expect("commit 1");
    let pre_writer_head = writer.head_commit_oid();
    let pre_l4_head = Git2LedgerWriter::head_chaintape_l4(tmp.path()).expect("pre l4 head");

    let gap_entry = entry_at(3, e1.resulting_state_root, e1.resulting_ledger_root, h(3));
    let err = writer.commit(&gap_entry).expect_err("logical_t gap");
    assert!(matches!(
        err,
        LedgerWriterError::LogicalTGap {
            expected: 2,
            got: 3
        }
    ));
    assert_eq!(writer.len(), 1);
    assert_eq!(writer.head_commit_oid(), pre_writer_head);
    assert_eq!(
        Git2LedgerWriter::head_chaintape_l4(tmp.path()).expect("post l4 head"),
        pre_l4_head
    );
}

#[test]
fn l4e_ref_update_failure_does_not_accept_rejection_record() {
    let tmp = TempDir::new().expect("tempdir");
    let jsonl_path = tmp.path().join("rejections.jsonl");
    let non_git_repo = tmp.path().join("not-a-git-repo");
    std::fs::create_dir_all(&non_git_repo).expect("non git dir");
    let _env = EnvGuard::set("TURINGOS_CHAINTAPE_PATH", &non_git_repo);

    let mut writer = RejectionEvidenceWriter::open_jsonl(jsonl_path.clone()).expect("open jsonl");
    let returned_hash = writer.append_rejected(
        1,
        Hash::ZERO,
        AgentId("a04-agent".into()),
        TxKind::Work,
        Cid([0xA4; 32]),
        RejectionClass::PolicyViolation,
        None,
        Some("A04 L4.E ref update must fail closed".into()),
    );

    assert_eq!(
        returned_hash,
        Hash::ZERO,
        "append_rejected must return the previous hash when L4.E ref movement fails"
    );
    assert_eq!(
        writer.len(),
        0,
        "L4.E ref movement failure must not be accepted into the in-memory rejection chain"
    );

    let jsonl_bytes = std::fs::read(&jsonl_path).expect("read jsonl after failed append");
    assert!(
        jsonl_bytes.is_empty(),
        "L4.E ref movement failure must not leave a replayable JSONL record"
    );
    let replayed =
        RejectionEvidenceWriter::open_jsonl(jsonl_path).expect("reopen jsonl after failed append");
    assert_eq!(
        replayed.len(),
        0,
        "failed canonical L4.E append must not resurrect through JSONL replay"
    );
}
