//! TRACE_MATRIX FC1a-substrate_seam: Art. 0.4 dual-substrate disjointness.
//!
//! Git2LedgerWriter (runtime_repo) and GitTapeLedger (tdma_tape.git) are both
//! retained substrates. This gate proves they do not silently collapse into one
//! authority by sharing ref names, default repo directories, or git object
//! pools.

use std::collections::{BTreeMap, BTreeSet};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::Cid;
use turingosv4::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
use turingosv4::bottom_white::ledger::transition_ledger::{
    Git2LedgerWriter, LedgerEntry, LedgerWriter, TxKind, CHAINTAPE_CAS_REF, CHAINTAPE_L4E_REF,
    CHAINTAPE_L4_REF,
};
use turingosv4::git_tape_ledger::{
    GitTapeLedger, GIT_LEDGER_HEAD_REF, GIT_LEDGER_LEDGER_TAIL_REF, GIT_LEDGER_REPO_DIR_DEFAULT,
    GIT_LEDGER_SCOPE_REF_PREFIX,
};
use turingosv4::ledger::{AttemptScope, CommitRequest, ImmutableTapeLedger, NodeKind};
use turingosv4::state::q_state::Hash;

#[test]
fn dual_substrate_ref_namespaces_disjoint() {
    let tdma_refs: BTreeSet<&str> = [
        GIT_LEDGER_HEAD_REF,
        GIT_LEDGER_LEDGER_TAIL_REF,
        GIT_LEDGER_SCOPE_REF_PREFIX,
    ]
    .into_iter()
    .collect();

    let runtime_refs: BTreeSet<&str> = [CHAINTAPE_L4_REF, CHAINTAPE_L4E_REF, CHAINTAPE_CAS_REF]
        .into_iter()
        .collect();

    let intersection: BTreeSet<&&str> = tdma_refs.intersection(&runtime_refs).collect();
    assert!(
        intersection.is_empty(),
        "Art. 0.4 violation: TDMA and runtime ChainTape refs overlap: {intersection:?}"
    );

    for tdma_ref in &tdma_refs {
        assert!(
            tdma_ref.starts_with("refs/tdma/"),
            "TDMA ref `{tdma_ref}` must stay under refs/tdma/"
        );
        assert!(
            !tdma_ref.starts_with("refs/chaintape/") && !tdma_ref.starts_with("refs/transitions/"),
            "TDMA ref `{tdma_ref}` must not use a runtime ChainTape namespace"
        );
    }

    for runtime_ref in &runtime_refs {
        assert!(
            runtime_ref.starts_with("refs/chaintape/"),
            "runtime ChainTape ref `{runtime_ref}` must stay under refs/chaintape/"
        );
        assert!(
            !runtime_ref.starts_with("refs/tdma/"),
            "runtime ChainTape ref `{runtime_ref}` must not use a TDMA namespace"
        );
    }
}

#[test]
fn dual_substrate_default_repo_dirs_disjoint() {
    assert_ne!(
        GIT_LEDGER_REPO_DIR_DEFAULT, "",
        "TDMA repo dir must be explicit"
    );
    assert_ne!(
        GIT_LEDGER_REPO_DIR_DEFAULT, ".",
        "TDMA repo dir must not alias the runtime workspace root"
    );
    assert!(
        GIT_LEDGER_REPO_DIR_DEFAULT.starts_with("tdma_"),
        "TDMA repo dir `{GIT_LEDGER_REPO_DIR_DEFAULT}` should express substrate segregation"
    );
}

#[test]
fn dual_substrate_git_object_pools_disjoint() {
    let tmp = TempDir::new().expect("tempdir");

    let tdma_path = tmp.path().join(GIT_LEDGER_REPO_DIR_DEFAULT);
    let mut tdma_ledger = GitTapeLedger::init_bare(&tdma_path).expect("tdma init_bare");
    let scope = AttemptScope {
        run_id: "run-disjoint".into(),
        task_id: "task-disjoint".into(),
        verified_parent: "vp-0".into(),
    };
    let tdma_node = tdma_ledger.commit(CommitRequest {
        kind: NodeKind::AgentProposal,
        verified: false,
        parent: None,
        scope: Some(scope),
        attempt_ordinal: Some(1),
        reject_class: None,
        token_count: None,
        payload: serde_json::json!({"substrate": "tdma"}),
    });

    let runtime_path = tmp.path().join("runtime_repo");
    let mut runtime_writer = Git2LedgerWriter::open(&runtime_path).expect("runtime_repo open");
    let entry = LedgerEntry {
        logical_t: 1,
        parent_state_root: Hash::ZERO,
        parent_ledger_root: Hash::ZERO,
        tx_kind: TxKind::Work,
        tx_payload_cid: Cid([1u8; 32]),
        resulting_state_root: Hash::ZERO,
        resulting_ledger_root: Hash([2u8; 32]),
        timestamp_logical: 1,
        epoch: SystemEpoch::new(1),
        extensions: BTreeMap::new(),
        system_signature: SystemSignature::from_bytes([0u8; 64]),
    };
    runtime_writer.commit(&entry).expect("runtime commit");
    let runtime_oid_hex = runtime_writer
        .head_commit_oid_hex()
        .expect("runtime head oid");

    let tdma_git_repo = git2::Repository::open_bare(&tdma_path).expect("open tdma git repo");
    let runtime_oid = git2::Oid::from_str(&runtime_oid_hex).expect("parse runtime oid");
    assert!(
        tdma_git_repo.find_object(runtime_oid, None).is_err(),
        "runtime_repo commit OID {runtime_oid_hex} must not be findable in TDMA object pool"
    );

    let runtime_git_repo = git2::Repository::open(&runtime_path).expect("open runtime git repo");
    let tdma_oid = git2::Oid::from_str(&tdma_node.hash).expect("parse tdma oid");
    assert!(
        runtime_git_repo.find_object(tdma_oid, None).is_err(),
        "TDMA commit OID {} must not be findable in runtime_repo object pool",
        tdma_node.hash
    );
}
