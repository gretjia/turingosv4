//! TB-2 Phase-1 acceptance battery — integration tests through `Sequencer::submit`.
//!
//! Charter: `handover/tracer_bullets/TB-2_charter_2026-04-30.md`.
//! Preflight v3 §5.2: 13 integration tests (I1-I13). The 3 in-crate unit
//! tests (U1-U3) live inside `src/state/sequencer.rs::tests`.
//!
//! All tests in this file go through the public `Sequencer::submit` path.
//! L4.E rows are observed via the constructor-injected
//! `Arc<RwLock<RejectionEvidenceWriter>>` clone the test retains (P0-5 r2).
//! Single-step driving uses `Sequencer::try_apply_one` (P1-3 r2; sequencer.rs
//! `pub fn try_apply_one`) since `Sequencer::run` loops until the receiver
//! closes.
//!
//! Atom 4 covers: I3 (predicate-fail), I4 (stale-parent), I5 (stakeless),
//! I6 (no-escrow), I7 (no advance), I8 (serde shield re-confirm at runtime).
//! I1, I2, I9-I13 land in Atoms 5+.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    RejectionClass as L4ERejectionClass, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{InMemoryLedgerWriter, LedgerWriter};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::{MicroCoin, StakeMicroCoin};
use turingosv4::state::q_state::{AgentId, EscrowEntry, Hash, QState, TaskId, TxId};
use turingosv4::state::sequencer::{worktx_accept_state_root, Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, EscrowLockTx, PredicateId, PredicateResultsBundle, ReadKey,
    SafetyOrCreation, TaskOpenTx, TypedTx, WorkTx, WriteKey,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ────────────────────────────────────────────────────────────────────────────
// Fixtures
// ────────────────────────────────────────────────────────────────────────────

#[derive(Clone)]
struct WorkTxFixtureOpts {
    parent_state_root: Hash,
    acceptance_passes: bool,
    settlement_passes: bool,
    stake_micro_units: i64,
    task_id: TaskId,
    agent_id: AgentId,
    tx_id_suffix: String,
}

impl Default for WorkTxFixtureOpts {
    fn default() -> Self {
        Self {
            parent_state_root: Hash::ZERO,
            acceptance_passes: true,
            settlement_passes: true,
            stake_micro_units: 1_000_000,
            task_id: TaskId("task-tb2-default".into()),
            agent_id: AgentId("alice".into()),
            tx_id_suffix: "0".into(),
        }
    }
}

fn make_worktx(opts: WorkTxFixtureOpts) -> TypedTx {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("acc1".into()),
        BoolWithProof {
            value: opts.acceptance_passes,
            proof_cid: None,
        },
    );
    let mut settlement = BTreeMap::new();
    if !opts.settlement_passes {
        settlement.insert(
            PredicateId("settle1".into()),
            BoolWithProof {
                value: false,
                proof_cid: None,
            },
        );
    }
    TypedTx::Work(WorkTx {
        tx_id: TxId(format!("worktx-tb2-{}", opts.tx_id_suffix)),
        task_id: opts.task_id,
        parent_state_root: opts.parent_state_root,
        agent_id: opts.agent_id,
        read_set: [ReadKey("k.read".into())]
            .into_iter()
            .collect::<BTreeSet<_>>(),
        write_set: [WriteKey("k.write".into())]
            .into_iter()
            .collect::<BTreeSet<_>>(),
        proposal_cid: Default::default(),
        predicate_results: PredicateResultsBundle {
            acceptance,
            settlement,
            safety_class: SafetyOrCreation::Safety,
        },
        stake: StakeMicroCoin::from_micro_units(opts.stake_micro_units),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

/// **TB-3 Atom 6 fixture migration**: the bridge that synthesized
/// `TxId(task_id.0.clone())` from a TaskId is now DELETED in the WorkTx
/// admission. The new admission requires `task_markets_t[task_id].total_escrow > 0`,
/// populated only by accepted `TaskOpenTx` + `EscrowLockTx`.
///
/// This fixture now seeds ONLY balances at the genesis level (both solver
/// "alice" and sponsor "treasury" — the latter for use by
/// `setup_funded_task_through_formal_surface` which submits `EscrowLockTx`s
/// debited from "treasury"). The `task_id` parameter is kept in the
/// signature only to preserve TB-2 call-site shape; it's no longer used
/// here (full setup happens via the formal-surface helper).
///
/// Direct balance seeding at genesis is the test-only equivalent of the
/// future RSP-0 `on_init_tx`; no parallel-ledger violation per Art 0.2
/// (balances_t IS a holding, and at genesis state_root_t == ZERO so
/// `assert_no_post_init_mint` is permissive).
///
/// I3-I8 rejection tests reject before reaching the new admission gate
/// (predicate-fail / stake-zero / stale-parent / no-escrow all fail earlier
/// or at the new gate without escrow setup). I9-I13 acceptance tests MUST
/// call `setup_funded_task_through_formal_surface` after `fresh_harness`
/// to populate `task_markets_t` via accepted TaskOpen + EscrowLock.
fn seed_q_with_escrow(_task_id: &TaskId) -> QState {
    let mut q = QState::genesis();
    q.economic_state_t
        .balances_t
        .0
        .insert(AgentId("alice".into()), MicroCoin::from_coin(10).unwrap());
    q.economic_state_t.balances_t.0.insert(
        AgentId("treasury".into()),
        MicroCoin::from_coin(100).unwrap(),
    );
    q
}

/// **TB-3 Atom 6**: drive the QState through accepted TaskOpen + EscrowLock
/// transitions via the harness's Sequencer (charter § 5.6 — fixtures use
/// accepted-tx submission, not direct EconomicState mutation).
///
/// After this helper returns, `task_markets_t[task_id]` exists and has
/// `total_escrow > 0`, satisfying the new WorkTx admission gate. The caller
/// MUST then read `h.seq.q_snapshot().state_root_t` and use it as the
/// `parent_state_root` for any subsequent WorkTx submission (the seeded state
/// has advanced two transitions past genesis).
async fn setup_funded_task_through_formal_surface(h: &mut Harness, task_id: &TaskId) {
    // Open the task.
    let pre = h.seq.q_snapshot().expect("pre snap").state_root_t;
    let open_tx = TypedTx::TaskOpen(TaskOpenTx {
        tx_id: TxId(format!("seed-open-{}", task_id.0)),
        task_id: task_id.clone(),
        parent_state_root: pre,
        sponsor_agent: AgentId("treasury".into()),
        verifier_quorum: 1,
        max_reuse_royalty_fraction_basis_points: 1000,
        settlement_rule_hash: Hash::ZERO,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 0,
    });
    h.seq.submit(open_tx).await.expect("seed TaskOpen submit");
    h.seq
        .try_apply_one(&mut h.rx)
        .expect("seed TaskOpen envelope")
        .expect("seed TaskOpen accepted");

    // Lock escrow against it.
    let parent = h.seq.q_snapshot().expect("post-open snap").state_root_t;
    let lock_tx = TypedTx::EscrowLock(EscrowLockTx {
        tx_id: TxId(format!("seed-lock-{}", task_id.0)),
        task_id: task_id.clone(),
        parent_state_root: parent,
        sponsor_agent: AgentId("treasury".into()),
        amount: MicroCoin::from_coin(50).unwrap(),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 0,
    });
    h.seq.submit(lock_tx).await.expect("seed EscrowLock submit");
    h.seq
        .try_apply_one(&mut h.rx)
        .expect("seed EscrowLock envelope")
        .expect("seed EscrowLock accepted");
}

struct Harness {
    _tmp: TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
    // Retained Arc clones for tests that need to replay (I13) or otherwise
    // observe the CAS / L4 / keypair state without going through Sequencer.
    cas: Arc<RwLock<CasStore>>,
    ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
    keypair: Arc<Ed25519Keypair>,
    epoch: SystemEpoch,
    initial_q: QState,
    predicate_registry: Arc<PredicateRegistry>,
    tool_registry: Arc<ToolRegistry>,
}

fn fresh_harness(initial_q: QState) -> Harness {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("keypair"));
    let writer: Arc<RwLock<dyn LedgerWriter>> = Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
    let preds = Arc::new(PredicateRegistry::new());
    let tools = Arc::new(ToolRegistry::new());
    let epoch = SystemEpoch::new(1);
    // TB-5 Atom 4: pin keypair pubkey under epoch (preflight § 4.2).
    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(epoch, keypair.public_key());
    let pinned_pubkeys = Arc::new(pinned);
    let (seq, rx) = Sequencer::new(
        cas.clone(),
        keypair.clone(),
        epoch,
        writer.clone(),
        rejection_writer.clone(),
        preds.clone(),
        tools.clone(),
        pinned_pubkeys,
        initial_q.clone(),
        16,
    );
    Harness {
        _tmp: tmp,
        seq,
        rx,
        rejection_writer,
        cas,
        ledger_writer: writer,
        keypair,
        epoch,
        initial_q,
        predicate_registry: preds,
        tool_registry: tools,
    }
}

fn l4e_row_count(writer: &Arc<RwLock<RejectionEvidenceWriter>>) -> usize {
    writer.read().expect("writer read").records().len()
}

// ────────────────────────────────────────────────────────────────────────────
// I3 — predicate-failed WorkTx → 1 L4.E row (PredicateFailed)
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn runtime_predicate_failed_worktx_appends_l4e() {
    let task_id = TaskId("task-i3".into());
    let mut h = fresh_harness(seed_q_with_escrow(&task_id));
    let pre_state = h.seq.q_snapshot().expect("q_snapshot").state_root_t;

    let receipt = h
        .seq
        .submit(make_worktx(WorkTxFixtureOpts {
            acceptance_passes: false,
            task_id: task_id.clone(),
            tx_id_suffix: "i3".into(),
            ..Default::default()
        }))
        .await
        .expect("submit");

    let drain = h.seq.try_apply_one(&mut h.rx).expect("envelope queued");
    assert!(drain.is_err(), "predicate-failed WorkTx must be rejected");

    assert_eq!(l4e_row_count(&h.rejection_writer), 1);
    let writer_g = h.rejection_writer.read().expect("writer read");
    let row = &writer_g.records()[0];
    assert_eq!(row.submit_id, receipt.submit_id);
    assert_eq!(row.rejection_class, L4ERejectionClass::PredicateFailed);
    assert_eq!(row.parent_state_root, pre_state);
}

// ────────────────────────────────────────────────────────────────────────────
// I4 — stale parent_state_root → 1 L4.E row (PolicyViolation; from StaleParent)
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn runtime_stale_parent_worktx_appends_l4e() {
    let task_id = TaskId("task-i4".into());
    let mut h = fresh_harness(seed_q_with_escrow(&task_id));
    let bad_parent = Hash::from_bytes([0xAB; 32]);

    let receipt = h
        .seq
        .submit(make_worktx(WorkTxFixtureOpts {
            parent_state_root: bad_parent,
            task_id: task_id.clone(),
            tx_id_suffix: "i4".into(),
            ..Default::default()
        }))
        .await
        .expect("submit");

    let drain = h.seq.try_apply_one(&mut h.rx).expect("envelope queued");
    assert!(drain.is_err());

    assert_eq!(l4e_row_count(&h.rejection_writer), 1);
    let writer_g = h.rejection_writer.read().expect("writer read");
    let row = &writer_g.records()[0];
    assert_eq!(row.submit_id, receipt.submit_id);
    assert_eq!(
        row.rejection_class,
        L4ERejectionClass::PolicyViolation,
        "TransitionError::StaleParent maps to PolicyViolation per §3.7"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// I5 — stakeless WorkTx → 1 L4.E row (PolicyViolation; from StakeInsufficient)
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn runtime_stakeless_worktx_appends_l4e() {
    let task_id = TaskId("task-i5".into());
    let mut h = fresh_harness(seed_q_with_escrow(&task_id));

    let receipt = h
        .seq
        .submit(make_worktx(WorkTxFixtureOpts {
            stake_micro_units: 0,
            task_id: task_id.clone(),
            tx_id_suffix: "i5".into(),
            ..Default::default()
        }))
        .await
        .expect("submit");

    let drain = h.seq.try_apply_one(&mut h.rx).expect("envelope queued");
    assert!(drain.is_err());

    assert_eq!(l4e_row_count(&h.rejection_writer), 1);
    let writer_g = h.rejection_writer.read().expect("writer read");
    let row = &writer_g.records()[0];
    assert_eq!(row.submit_id, receipt.submit_id);
    assert_eq!(row.rejection_class, L4ERejectionClass::PolicyViolation);
}

// ────────────────────────────────────────────────────────────────────────────
// I6 — no escrow → 1 L4.E row (EscrowMissing)
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn runtime_no_escrow_worktx_appends_l4e() {
    // QState::genesis() has no seeded escrow for task-i6.
    //
    // TB-N1-AGENT-ECONOMY Phase 2 A3 (2026-05-10): seed `alice` balance via
    // `seed_q_with_escrow` (fixture name is historical — it only seeds
    // balances, NOT escrow per its doc) so the new sequencer Step-4
    // agent-bound stake gate (`stake > balance` → StakeBalanceExceeded)
    // does NOT preempt the test's Step-5 EscrowMissing assertion. The
    // test's task_id (task-i6-no-escrow) has no `task_markets_t` entry, so
    // Step-5 still fires after the new Step-4 gate passes.
    let mut h = fresh_harness(seed_q_with_escrow(&TaskId("task-i6-no-escrow".into())));

    let receipt = h
        .seq
        .submit(make_worktx(WorkTxFixtureOpts {
            task_id: TaskId("task-i6-no-escrow".into()),
            tx_id_suffix: "i6".into(),
            ..Default::default()
        }))
        .await
        .expect("submit");

    let drain = h.seq.try_apply_one(&mut h.rx).expect("envelope queued");
    assert!(drain.is_err());

    assert_eq!(l4e_row_count(&h.rejection_writer), 1);
    let writer_g = h.rejection_writer.read().expect("writer read");
    let row = &writer_g.records()[0];
    assert_eq!(row.submit_id, receipt.submit_id);
    assert_eq!(row.rejection_class, L4ERejectionClass::EscrowMissing);
}

// ────────────────────────────────────────────────────────────────────────────
// I7 — across I3-I6, no logical_t / state_root_t / ledger_root_t advance
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn runtime_rejected_worktx_does_not_advance_logical_t_or_state_root() {
    let task_id = TaskId("task-i7".into());
    let mut h = fresh_harness(seed_q_with_escrow(&task_id));
    let q0 = h.seq.q_snapshot().expect("q0");
    let pre_state = q0.state_root_t;
    let pre_ledger = q0.ledger_root_t;
    let pre_logical = h.seq.next_logical_t_peek();

    // Submit four rejection-class WorkTxes through the same sequencer.
    let cases = vec![
        WorkTxFixtureOpts {
            acceptance_passes: false,
            task_id: task_id.clone(),
            tx_id_suffix: "i7-pred".into(),
            ..Default::default()
        },
        WorkTxFixtureOpts {
            parent_state_root: Hash::from_bytes([0xCD; 32]),
            task_id: task_id.clone(),
            tx_id_suffix: "i7-parent".into(),
            ..Default::default()
        },
        WorkTxFixtureOpts {
            stake_micro_units: 0,
            task_id: task_id.clone(),
            tx_id_suffix: "i7-stake".into(),
            ..Default::default()
        },
        WorkTxFixtureOpts {
            task_id: TaskId("task-i7-no-escrow".into()),
            tx_id_suffix: "i7-escrow".into(),
            ..Default::default()
        },
    ];

    for opts in cases {
        h.seq.submit(make_worktx(opts)).await.expect("submit");
        let drain = h.seq.try_apply_one(&mut h.rx).expect("queued");
        assert!(drain.is_err(), "expected rejection");
    }

    let q1 = h.seq.q_snapshot().expect("q1");
    assert_eq!(q1.state_root_t, pre_state, "state_root_t unchanged");
    assert_eq!(q1.ledger_root_t, pre_ledger, "ledger_root_t unchanged");
    assert_eq!(
        h.seq.next_logical_t_peek(),
        pre_logical,
        "logical_t unchanged"
    );
    assert_eq!(
        l4e_row_count(&h.rejection_writer),
        4,
        "4 L4.E rows appended"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// I8 — runtime L4.E public_view honors raw_diagnostic_cid serde shield
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn runtime_l4e_public_view_honors_serde_shield() {
    let task_id = TaskId("task-i8".into());
    let mut h = fresh_harness(seed_q_with_escrow(&task_id));

    h.seq
        .submit(make_worktx(WorkTxFixtureOpts {
            acceptance_passes: false,
            task_id: task_id.clone(),
            tx_id_suffix: "i8".into(),
            ..Default::default()
        }))
        .await
        .expect("submit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("queued");

    // Fetch the L4.E row's public view + serialize. The TB-1 P0-3 serde
    // shield says raw_diagnostic_cid is `#[serde(skip_serializing)]` on
    // RejectedSubmissionRecord (rejection_evidence.rs:108). The PublicRejectionView
    // omits raw_diagnostic_cid entirely. Both layers are checked.
    let writer_g = h.rejection_writer.read().expect("writer read");
    let row = &writer_g.records()[0];
    assert!(
        row.raw_diagnostic_cid.is_some(),
        "runtime path stores raw_diagnostic_cid (private; never serialized)"
    );

    // Serialize the raw record — raw_diagnostic_cid must NOT appear in the
    // JSON output (TB-1 P0-3 serde-skip enforced at the type level).
    let json_record = serde_json::to_string(row).expect("serialize record");
    assert!(
        !json_record.contains("raw_diagnostic_cid"),
        "RejectedSubmissionRecord serde-skip shield must hide raw_diagnostic_cid"
    );

    // Serialize the public view — additionally a structural projection that
    // never contains the raw cid.
    let public = writer_g.public_view();
    assert_eq!(public.len(), 1);
    let json_public = serde_json::to_string(&public[0]).expect("serialize public view");
    assert!(!json_public.contains("raw_diagnostic_cid"));
}

// ────────────────────────────────────────────────────────────────────────────
// I1 — submit receipt submit_id matches the L4.E row's submit_id
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn submit_returns_receipt_and_envelope_submit_id_matches() {
    // Use a rejection (no escrow) so we can read the L4.E row's submit_id.
    // Acceptance-side submit_id-matching is covered by I12 indirectly +
    // U2 in-crate (which checks submit_id materializes in L4.E).
    let mut h = fresh_harness(QState::genesis());
    let receipt = h
        .seq
        .submit(make_worktx(WorkTxFixtureOpts {
            task_id: TaskId("task-i1".into()),
            tx_id_suffix: "i1".into(),
            ..Default::default()
        }))
        .await
        .expect("submit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("queued");
    let writer_g = h.rejection_writer.read().expect("read");
    let row = &writer_g.records()[0];
    assert_eq!(
        row.submit_id, receipt.submit_id,
        "L4.E row's submit_id must match the receipt's submit_id"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// I2 — failed try_send still consumes submit_id (no ID reuse)
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn submit_queue_full_consumes_submit_id() {
    use turingosv4::state::sequencer::SubmitError;

    // Fresh sequencer with capacity=2, never drained.
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("kp"));
    let writer: Arc<RwLock<dyn LedgerWriter>> = Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
    let preds = Arc::new(PredicateRegistry::new());
    let tools = Arc::new(ToolRegistry::new());
    let epoch = SystemEpoch::new(1);
    // TB-5 Atom 4: pin keypair pubkey under epoch (preflight § 4.2).
    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(epoch, keypair.public_key());
    let pinned_pubkeys = Arc::new(pinned);
    let (seq, _rx) = Sequencer::new(
        cas,
        keypair,
        epoch,
        writer,
        rejection_writer,
        preds,
        tools,
        pinned_pubkeys,
        QState::genesis(),
        2,
    );

    let r1 = seq
        .submit(make_worktx(WorkTxFixtureOpts {
            tx_id_suffix: "i2-1".into(),
            ..Default::default()
        }))
        .await
        .expect("first submit");
    let r2 = seq
        .submit(make_worktx(WorkTxFixtureOpts {
            tx_id_suffix: "i2-2".into(),
            ..Default::default()
        }))
        .await
        .expect("second submit");
    // Queue saturated.
    let err = seq
        .submit(make_worktx(WorkTxFixtureOpts {
            tx_id_suffix: "i2-fail".into(),
            ..Default::default()
        }))
        .await
        .unwrap_err();
    assert!(matches!(err, SubmitError::QueueFull));

    // submit_id MUST have been burned even though try_send failed; the next
    // observable state of the counter is r2.submit_id + 2 (counted as: r1=1,
    // r2=2, failed=3, next would be 4). Read via next_submit_id_peek().
    assert_eq!(r1.submit_id + 1, r2.submit_id);
    assert_eq!(
        seq.next_submit_id_peek(),
        r2.submit_id + 2,
        "failed try_send must still burn its submit_id (next counter = r2 + 2)"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// I9 — accepted WorkTx advances state_root_t to WORKTX_ACCEPT_DOMAIN_V1 hash
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn runtime_accepted_worktx_advances_state_root_via_domain_v1() {
    let task_id = TaskId("task-i9".into());
    let mut h = fresh_harness(seed_q_with_escrow(&task_id));
    setup_funded_task_through_formal_surface(&mut h, &task_id).await;
    let q0 = h.seq.q_snapshot().expect("q0");
    let parent = q0.state_root_t;

    let tx = make_worktx(WorkTxFixtureOpts {
        parent_state_root: parent,
        task_id: task_id.clone(),
        tx_id_suffix: "i9".into(),
        ..Default::default()
    });
    h.seq.submit(tx.clone()).await.expect("submit");
    let drain = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("queued")
        .expect("apply_one accepted");
    let _ = drain;

    // Expected state_root_t computed via the single public TB-2 helper
    // worktx_accept_state_root. Cross-checks U3 at the integration layer.
    let expected = worktx_accept_state_root(&parent, &tx);

    let q1 = h.seq.q_snapshot().expect("q1");
    assert_eq!(
        q1.state_root_t, expected,
        "state_root_t must advance to worktx_accept_state_root(prev, tx)"
    );
    assert_ne!(q1.state_root_t, q0.state_root_t, "state_root_t advanced");
}

// ────────────────────────────────────────────────────────────────────────────
// I10 — accepted WorkTx advances ledger_root_t (canonical L4 transition_ledger)
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn runtime_accepted_worktx_advances_ledger_root() {
    let task_id = TaskId("task-i10".into());
    let mut h = fresh_harness(seed_q_with_escrow(&task_id));
    setup_funded_task_through_formal_surface(&mut h, &task_id).await;
    let q0 = h.seq.q_snapshot().expect("q0");
    let pre_ledger = q0.ledger_root_t;

    h.seq
        .submit(make_worktx(WorkTxFixtureOpts {
            parent_state_root: q0.state_root_t,
            task_id: task_id.clone(),
            tx_id_suffix: "i10".into(),
            ..Default::default()
        }))
        .await
        .expect("submit");
    h.seq
        .try_apply_one(&mut h.rx)
        .expect("queued")
        .expect("apply_one accepted");

    let post_ledger = h.seq.q_snapshot().expect("q1").ledger_root_t;
    assert_ne!(
        pre_ledger, post_ledger,
        "ledger_root_t must advance via canonical transition_ledger"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// I11 — accepted WorkTx increments accepted logical_t by 1
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn runtime_accepted_worktx_increments_logical_t() {
    let task_id = TaskId("task-i11".into());
    let mut h = fresh_harness(seed_q_with_escrow(&task_id));
    setup_funded_task_through_formal_surface(&mut h, &task_id).await;
    let parent = h.seq.q_snapshot().expect("snap").state_root_t;
    let pre = h.seq.next_logical_t_peek();

    h.seq
        .submit(make_worktx(WorkTxFixtureOpts {
            parent_state_root: parent,
            task_id: task_id.clone(),
            tx_id_suffix: "i11".into(),
            ..Default::default()
        }))
        .await
        .expect("submit");
    h.seq
        .try_apply_one(&mut h.rx)
        .expect("queued")
        .expect("apply_one accepted");

    assert_eq!(
        h.seq.next_logical_t_peek(),
        pre + 1,
        "accepted logical_t must increment by exactly 1"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// I12 — accepted WorkTx writes ZERO L4.E rows
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn runtime_accepted_worktx_does_not_append_l4e() {
    let task_id = TaskId("task-i12".into());
    let mut h = fresh_harness(seed_q_with_escrow(&task_id));
    setup_funded_task_through_formal_surface(&mut h, &task_id).await;
    let parent = h.seq.q_snapshot().expect("snap").state_root_t;

    let pre_l4e = l4e_row_count(&h.rejection_writer);
    h.seq
        .submit(make_worktx(WorkTxFixtureOpts {
            parent_state_root: parent,
            task_id: task_id.clone(),
            tx_id_suffix: "i12".into(),
            ..Default::default()
        }))
        .await
        .expect("submit");
    h.seq
        .try_apply_one(&mut h.rx)
        .expect("queued")
        .expect("apply_one accepted");

    let post_l4e = l4e_row_count(&h.rejection_writer);
    assert_eq!(
        post_l4e, pre_l4e,
        "accepted WorkTx must NOT append any L4.E row"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// I13 — replay from canonical L4 only ignores L4.E (P1:8 / Art IV Boot)
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn runtime_replay_from_l4_only_ignores_l4e() {
    use turingosv4::bottom_white::ledger::system_keypair::PinnedSystemPubkeys;
    use turingosv4::bottom_white::ledger::transition_ledger::replay_full_transition;

    let task_id = TaskId("task-i13".into());
    let mut h = fresh_harness(seed_q_with_escrow(&task_id));
    setup_funded_task_through_formal_surface(&mut h, &task_id).await;
    let parent_after_setup = h.seq.q_snapshot().expect("snap").state_root_t;

    // Submit one accepted WorkTx (predicate-passing + funded task + solvent solver).
    h.seq
        .submit(make_worktx(WorkTxFixtureOpts {
            parent_state_root: parent_after_setup,
            task_id: task_id.clone(),
            tx_id_suffix: "i13-accept".into(),
            ..Default::default()
        }))
        .await
        .expect("submit accept");
    h.seq
        .try_apply_one(&mut h.rx)
        .expect("queued")
        .expect("accept apply_one");

    // Submit one rejected WorkTx (predicate-failing). parent_state_root must
    // match the post-accept state_root; rejection happens at predicate gate
    // before any economic mutation.
    let parent_after_accept = h.seq.q_snapshot().expect("snap").state_root_t;
    h.seq
        .submit(make_worktx(WorkTxFixtureOpts {
            parent_state_root: parent_after_accept,
            acceptance_passes: false,
            task_id: task_id.clone(),
            tx_id_suffix: "i13-reject".into(),
            ..Default::default()
        }))
        .await
        .expect("submit reject");
    h.seq
        .try_apply_one(&mut h.rx)
        .expect("queued")
        .expect_err("reject apply_one returns Err");

    let post_state = h.seq.q_snapshot().expect("post").state_root_t;
    let post_ledger = h.seq.q_snapshot().expect("post").ledger_root_t;
    assert_eq!(
        l4e_row_count(&h.rejection_writer),
        1,
        "exactly 1 L4.E row from the rejected submission"
    );

    // Reconstruct QState from canonical L4 ONLY. Read entries via
    // LedgerWriter::read_at + len(), then drive replay_full_transition with
    // genesis = the same initial_q the sequencer used.
    // **TB-3 Atom 6 update**: now expects 3 L4 rows (TaskOpen + EscrowLock from
    // setup_funded_task_through_formal_surface + the accepted WorkTx). Replay
    // applies all three and reaches the same final state.
    let entries = {
        let writer_g = h.ledger_writer.read().expect("writer read");
        let n = writer_g.len();
        assert_eq!(
            n, 3,
            "3 accepted L4 rows: TaskOpen + EscrowLock (setup) + WorkTx (test)"
        );
        (0..n)
            .map(|i| writer_g.read_at(i + 1).expect("read_at"))
            .collect::<Vec<_>>()
    };

    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(h.epoch, h.keypair.public_key());

    let cas_g = h.cas.read().expect("cas read");
    let replayed_q = replay_full_transition(
        &h.initial_q,
        &entries,
        &*cas_g,
        &pinned,
        &h.predicate_registry,
        &h.tool_registry,
    )
    .expect("replay must succeed for an accepted-only L4");

    assert_eq!(
        replayed_q.state_root_t, post_state,
        "replay from canonical L4 must reach the same state_root_t as the live sequencer"
    );
    assert_eq!(
        replayed_q.ledger_root_t, post_ledger,
        "replay from canonical L4 must reach the same ledger_root_t"
    );
    // L4.E records were NOT consulted (only the 1-row L4 was). The rejected
    // submission did not influence the canonical reconstruction — Inv 7 / P1:8.
}
