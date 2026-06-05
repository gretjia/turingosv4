use std::process::Command;

use tempfile::TempDir;

use turingosv4::git_tape_ledger::{GitTapeLedger, TcHeadRefs};
use turingosv4::ledger::{AttemptScope, CommitRequest, ImmutableTapeLedger, NodeKind};

fn scope() -> AttemptScope {
    AttemptScope {
        run_id: "run-tc".to_string(),
        task_id: "task-tc".to_string(),
        verified_parent: "H0".to_string(),
    }
}

fn req(i: u32) -> CommitRequest {
    CommitRequest {
        kind: NodeKind::AgentProposal,
        verified: false,
        parent: Some("H0".to_string()),
        scope: Some(scope()),
        attempt_ordinal: Some(i),
        reject_class: Some("tc-red".to_string()),
        token_count: Some(10),
        payload: serde_json::json!({ "i": i }),
    }
}

#[test]
fn tc_head_refs_match_locked_contract() {
    let refs = TcHeadRefs::default();
    assert_eq!(refs.accepted_l4, "refs/chaintape/l4");
    assert_eq!(refs.rejected_l4e, "refs/chaintape/l4e");
    assert_eq!(refs.cas_root, "refs/chaintape/cas");
    assert_eq!(refs.tdma_verified, "refs/tdma/verified_head");
    assert_eq!(refs.tdma_tail, "refs/tdma/ledger_tail");
}

#[test]
fn reopen_append_continues_monotonic_tape_ids() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("tdma_tape.git");

    {
        let mut ledger = GitTapeLedger::init_bare(&path).unwrap();
        assert_eq!(ledger.commit(req(0)).id, "tn-1");
        assert_eq!(ledger.commit(req(1)).id, "tn-2");
    }

    {
        let mut reopened = GitTapeLedger::open(&path).unwrap();
        let node = reopened.commit(req(2));
        assert_eq!(node.id, "tn-3");
        assert_eq!(
            reopened.count_nodes(
                Some(NodeKind::AgentProposal),
                Some(false),
                None,
                Some(&scope())
            ),
            3
        );
    }

    let fsck = Command::new("git")
        .arg("--git-dir")
        .arg(&path)
        .arg("fsck")
        .arg("--full")
        .output()
        .unwrap();
    assert!(
        fsck.status.success(),
        "git fsck failed:\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&fsck.stdout),
        String::from_utf8_lossy(&fsck.stderr)
    );
}

#[test]
#[should_panic(expected = "invalid tdma verified_head oid")]
fn invalid_verified_head_panics_instead_of_silent_authority_skip() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("tdma_tape.git");
    let mut ledger = GitTapeLedger::init_bare(&path).unwrap();

    ledger.set_verified_head("not-a-git-oid".to_string());
}
