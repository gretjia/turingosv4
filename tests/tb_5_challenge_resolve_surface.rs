//! TB-5.1 ChallengeResolve dispatch-surface integration tests.
//!
//! Charter: `handover/tracer_bullets/TB-5_charter_2026-04-30.md` v2 § 4.6 + § 5.3.
//! Preflight: `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md` v2 § 7 + § 8.4.
//!
//! Atom 5 covers Released-path tests (I70 / I73 / I74). Atom 6 extends with
//! UpheldDeferred-path tests (I75-I77 + I88/I89 boundary). Atom 7 adds
//! replay/property/anti-drift coverage.
//!
//! Constitutional anchor: ChallengeResolve is a system-emitted variant
//! (Anti-Oreo per WP § 12.4). Tests exercise the full pipeline through
//! `Sequencer::emit_system_tx` (NOT `submit` — that path is barred by
//! the agent-ingress barrier landed in TB-5.0 Atom 2). Apply via
//! `try_apply_one` after seeding `challenge_cases_t`.

use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    RejectionClass as L4ERejectionClass, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    InMemoryLedgerWriter, LedgerWriter, TxKind,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::MicroCoin;
use turingosv4::state::q_state::{
    AgentId, ChallengeCase, ChallengeStatus, EscrowEntry, QState, StakeEntry, TaskId,
    TaskMarketEntry, TxId,
};
use turingosv4::state::sequencer::{
    Sequencer, SubmissionEnvelope, SystemEmitCommand,
};
use turingosv4::state::typed_tx::ChallengeResolution;
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ────────────────────────────────────────────────────────────────────────────
// Harness — keeps Sequencer, queue rx, writers, and the seed challenger info
// so tests can assert pre/post invariants.
// ────────────────────────────────────────────────────────────────────────────

struct Harness {
    _tmp: TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
    ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
}

/// Build a sequencer with `initial_q` seeded — caller pre-fills
/// `challenge_cases_t` with the cases under test.
fn fresh_harness_with(initial_q: QState) -> Harness {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(
        Ed25519Keypair::generate_with_secure_entropy().expect("keypair"),
    );
    let writer: Arc<RwLock<dyn LedgerWriter>> =
        Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
    let preds = Arc::new(PredicateRegistry::new());
    let tools = Arc::new(ToolRegistry::new());
    let epoch = SystemEpoch::new(1);
    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(epoch, keypair.public_key());
    let pinned_pubkeys = Arc::new(pinned);
    let (seq, rx) = Sequencer::new(
        cas,
        keypair,
        epoch,
        writer.clone(),
        rejection_writer.clone(),
        preds,
        tools,
        pinned_pubkeys,
        initial_q,
        16,
    );
    Harness { _tmp: tmp, seq, rx, rejection_writer, ledger_writer: writer }
}

/// Seed Q with one Open ChallengeCase, returning challenger AgentId + bond
/// for post-condition assertions.
fn q_with_one_open_case(
    challenger: &str,
    starting_balance_micro: i64,
    challenge_tx_id: &str,
    bond_micro: i64,
    target_work_tx_id: &str,
) -> (QState, TxId, AgentId, MicroCoin) {
    let mut q = QState::genesis();
    let challenger_id = AgentId(challenger.into());
    if starting_balance_micro > 0 {
        q.economic_state_t.balances_t.0.insert(
            challenger_id.clone(),
            MicroCoin::from_micro_units(starting_balance_micro),
        );
    }
    let challenge_id = TxId(challenge_tx_id.into());
    let bond = MicroCoin::from_micro_units(bond_micro);
    q.economic_state_t.challenge_cases_t.0.insert(
        challenge_id.clone(),
        ChallengeCase {
            challenger: challenger_id.clone(),
            bond,
            opened_at_round: 7,
            target_work_tx: TxId(target_work_tx_id.into()),
            status: ChallengeStatus::Open,
        },
    );
    (q, challenge_id, challenger_id, bond)
}

// ────────────────────────────────────────────────────────────────────────────
// I70 — emit_system_tx ChallengeResolve{Released} → apply_one accepts →
//       canonical L4 has 1 ChallengeResolve row.
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn submit_challenge_resolve_released_appends_to_canonical_l4() {
    let (q, target, _challenger, _bond) = q_with_one_open_case(
        "challenger-i70", 96, "ct-i70", 4, "wt-i70");
    let mut h = fresh_harness_with(q);

    let _receipt = h
        .seq
        .emit_system_tx(SystemEmitCommand::ChallengeResolve {
            target_challenge_tx_id: target.clone(),
            resolution: ChallengeResolution::Released,
        })
        .await
        .expect("emit ok");

    // Drain queue + apply.
    let res = h.seq.try_apply_one(&mut h.rx).expect("envelope present");
    let entry = res.expect("Released path with seeded Open case must accept");
    assert_eq!(entry.tx_kind, TxKind::ChallengeResolve);

    // Canonical L4 has exactly one row of TxKind::ChallengeResolve.
    let writer_guard = h.ledger_writer.read().expect("read");
    let len = writer_guard.len();
    assert_eq!(len, 1, "exactly one L4 entry after Released apply");
}

// ────────────────────────────────────────────────────────────────────────────
// I71 — Released refunds bond (post-condition on balances_t).
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn released_refunds_bond() {
    let (q, target, challenger, bond) = q_with_one_open_case(
        "challenger-i71", 96, "ct-i71", 4, "wt-i71");
    let pre_balance = q
        .economic_state_t
        .balances_t
        .0
        .get(&challenger)
        .copied()
        .unwrap();
    let mut h = fresh_harness_with(q);

    let _ = h
        .seq
        .emit_system_tx(SystemEmitCommand::ChallengeResolve {
            target_challenge_tx_id: target.clone(),
            resolution: ChallengeResolution::Released,
        })
        .await
        .expect("emit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("envelope").expect("accept");

    let q_post = h.seq.q_snapshot().expect("q snapshot");
    let post_balance = q_post
        .economic_state_t
        .balances_t
        .0
        .get(&challenger)
        .copied()
        .unwrap();
    assert_eq!(
        post_balance.micro_units(),
        pre_balance.micro_units() + bond.micro_units(),
        "Released refunds bond to challenger balance"
    );
    let entry = q_post
        .economic_state_t
        .challenge_cases_t
        .0
        .get(&target)
        .expect("entry preserved");
    assert_eq!(entry.bond.micro_units(), 0, "bond zeroed");
    assert_eq!(entry.status, ChallengeStatus::Released);
}

// ────────────────────────────────────────────────────────────────────────────
// I73 — second Released on same target → AlreadyResolved + L4.E row;
//       no L4 advance (canonical len stays 1).
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn released_cannot_run_twice() {
    let (q, target, _challenger, _bond) = q_with_one_open_case(
        "challenger-i73", 96, "ct-i73", 4, "wt-i73");
    let mut h = fresh_harness_with(q);

    // First resolve succeeds.
    h.seq
        .emit_system_tx(SystemEmitCommand::ChallengeResolve {
            target_challenge_tx_id: target.clone(),
            resolution: ChallengeResolution::Released,
        })
        .await
        .expect("emit 1");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("accept");
    let canonical_len_after_first = h.ledger_writer.read().unwrap().len();
    assert_eq!(canonical_len_after_first, 1);

    // Second resolve on same target — case is now Released.
    h.seq
        .emit_system_tx(SystemEmitCommand::ChallengeResolve {
            target_challenge_tx_id: target.clone(),
            resolution: ChallengeResolution::Released,
        })
        .await
        .expect("emit 2");
    let res = h.seq.try_apply_one(&mut h.rx).expect("env");
    let err = res.expect_err("second resolve must reject");
    let s = format!("{err}");
    assert!(
        s.contains("already resolved") || s.contains("AlreadyResolved"),
        "expected AlreadyResolved-class error, got: {s}"
    );

    // Canonical L4 length unchanged (rejected resolve does NOT advance L4).
    assert_eq!(
        h.ledger_writer.read().unwrap().len(),
        1,
        "K1: rejection does not consume logical_t / advance L4"
    );

    // L4.E gained one row.
    let l4e_records = h.rejection_writer.read().unwrap().records().len();
    assert_eq!(l4e_records, 1, "1 L4.E row for AlreadyResolved");
    let last = h.rejection_writer.read().unwrap().records().last().cloned().expect("row");
    assert_eq!(last.rejection_class, L4ERejectionClass::PolicyViolation);
}

// ────────────────────────────────────────────────────────────────────────────
// I74 — unknown target → ChallengeNotFound + L4.E row.
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn released_unknown_challenge_rejected() {
    // Seed Q with NO challenge_cases_t.
    let q = QState::genesis();
    let mut h = fresh_harness_with(q);

    h.seq
        .emit_system_tx(SystemEmitCommand::ChallengeResolve {
            target_challenge_tx_id: TxId("ct-i74-nonexistent".into()),
            resolution: ChallengeResolution::Released,
        })
        .await
        .expect("emit");
    let res = h.seq.try_apply_one(&mut h.rx).expect("env");
    let err = res.expect_err("unknown target must reject");
    let s = format!("{err}");
    assert!(
        s.contains("ChallengeResolveTx target_challenge_tx_id not present")
            || s.contains("ChallengeNotFound"),
        "expected ChallengeNotFound-class error, got: {s}"
    );

    // L4.E row written.
    let l4e_records = h.rejection_writer.read().unwrap().records().len();
    assert_eq!(l4e_records, 1, "1 L4.E row for ChallengeNotFound");
    let last = h.rejection_writer.read().unwrap().records().last().cloned().expect("row");
    assert_eq!(last.rejection_class, L4ERejectionClass::PolicyViolation);
    assert_eq!(last.tx_kind, TxKind::ChallengeResolve);

    // Canonical L4 unchanged (rejection does not advance L4).
    assert_eq!(h.ledger_writer.read().unwrap().len(), 0);
}

// ────────────────────────────────────────────────────────────────────────────
// Atom 6 — UpheldDeferred path tests + boundary tests
//
// I75: upheld_deferred_keeps_challenge_for_future_slash
// I76: upheld_deferred_no_balance_mutation
// I77: multi_challenger_resolve_independently
// I78: released_does_not_release_solver_or_verifier_stakes (boundary)
// I79: released_does_not_decrement_total_escrow (boundary)
// I88: challenge_resolve_does_not_mutate_q_t_current_round (boundary)
// I89: upheld_deferred_keeps_solver_verifier_stakes_byte_identical (boundary)
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn upheld_deferred_keeps_challenge_for_future_slash() {
    // I75: case.status flips to UpheldDeferred; case.bond is preserved
    // (TB-6 RSP-3.2 slash routing target).
    let (q, target, _challenger, bond) = q_with_one_open_case(
        "challenger-i75", 96, "ct-i75", 4, "wt-i75");
    let mut h = fresh_harness_with(q);

    h.seq
        .emit_system_tx(SystemEmitCommand::ChallengeResolve {
            target_challenge_tx_id: target.clone(),
            resolution: ChallengeResolution::UpheldDeferred,
        })
        .await
        .expect("emit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("accept");

    let q_post = h.seq.q_snapshot().expect("snapshot");
    let entry = q_post
        .economic_state_t
        .challenge_cases_t
        .0
        .get(&target)
        .expect("entry preserved");
    assert_eq!(entry.status, ChallengeStatus::UpheldDeferred);
    assert_eq!(
        entry.bond.micro_units(),
        bond.micro_units(),
        "UpheldDeferred MUST preserve bond"
    );
    // TB-6 RSP-3.2 slash routing target — challenger / target_work_tx /
    // opened_at_round MUST also be preserved.
    assert_eq!(entry.challenger, AgentId("challenger-i75".into()));
    assert_eq!(entry.target_work_tx, TxId("wt-i75".into()));
    assert_eq!(entry.opened_at_round, 7);
}

#[tokio::test]
async fn upheld_deferred_no_balance_mutation() {
    // I76: economic_state_t.balances_t bit-identical pre/post UpheldDeferred.
    let (q, target, challenger, _bond) = q_with_one_open_case(
        "challenger-i76", 96, "ct-i76", 4, "wt-i76");
    let pre_balances = q.economic_state_t.balances_t.clone();
    let mut h = fresh_harness_with(q);

    h.seq
        .emit_system_tx(SystemEmitCommand::ChallengeResolve {
            target_challenge_tx_id: target.clone(),
            resolution: ChallengeResolution::UpheldDeferred,
        })
        .await
        .expect("emit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("accept");

    let q_post = h.seq.q_snapshot().expect("snapshot");
    assert_eq!(
        q_post.economic_state_t.balances_t, pre_balances,
        "UpheldDeferred must not mutate balances_t (no money movement)"
    );
    // Challenger balance specifically untouched.
    let post_chal_bal = q_post
        .economic_state_t
        .balances_t
        .0
        .get(&challenger)
        .copied()
        .expect("challenger balance");
    assert_eq!(post_chal_bal.micro_units(), 96);
}

#[tokio::test]
async fn multi_challenger_resolve_independently() {
    // I77: Two ChallengeCases (different challengers / targets); resolve one
    // Released → other stays Open. Tests that dispatch operates on the
    // targeted case only and does not bleed into siblings.
    let mut q = QState::genesis();
    let chal_a = AgentId("challenger-a-i77".into());
    let chal_b = AgentId("challenger-b-i77".into());
    q.economic_state_t.balances_t.0.insert(chal_a.clone(), MicroCoin::from_micro_units(96));
    q.economic_state_t.balances_t.0.insert(chal_b.clone(), MicroCoin::from_micro_units(96));
    q.economic_state_t.challenge_cases_t.0.insert(
        TxId("ct-i77-a".into()),
        ChallengeCase {
            challenger: chal_a.clone(),
            bond: MicroCoin::from_micro_units(4),
            opened_at_round: 7,
            target_work_tx: TxId("wt-i77-a".into()),
            status: ChallengeStatus::Open,
        },
    );
    q.economic_state_t.challenge_cases_t.0.insert(
        TxId("ct-i77-b".into()),
        ChallengeCase {
            challenger: chal_b.clone(),
            bond: MicroCoin::from_micro_units(5),
            opened_at_round: 7,
            target_work_tx: TxId("wt-i77-b".into()),
            status: ChallengeStatus::Open,
        },
    );
    let mut h = fresh_harness_with(q);

    // Resolve only A.
    h.seq
        .emit_system_tx(SystemEmitCommand::ChallengeResolve {
            target_challenge_tx_id: TxId("ct-i77-a".into()),
            resolution: ChallengeResolution::Released,
        })
        .await
        .expect("emit a");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("accept a");

    let q_post = h.seq.q_snapshot().expect("snap");
    // A: Released, bond zeroed, challenger refunded.
    let entry_a = q_post.economic_state_t.challenge_cases_t.0.get(&TxId("ct-i77-a".into())).unwrap();
    assert_eq!(entry_a.status, ChallengeStatus::Released);
    assert_eq!(entry_a.bond.micro_units(), 0);
    let bal_a = q_post.economic_state_t.balances_t.0.get(&chal_a).copied().unwrap();
    assert_eq!(bal_a.micro_units(), 100, "challenger A refunded");
    // B: still Open, bond intact, challenger balance untouched.
    let entry_b = q_post.economic_state_t.challenge_cases_t.0.get(&TxId("ct-i77-b".into())).unwrap();
    assert_eq!(entry_b.status, ChallengeStatus::Open);
    assert_eq!(entry_b.bond.micro_units(), 5);
    let bal_b = q_post.economic_state_t.balances_t.0.get(&chal_b).copied().unwrap();
    assert_eq!(bal_b.micro_units(), 96, "challenger B unaffected");
}

// ────────────────────────────────────────────────────────────────────────────
// I78 / I79 / I89 — boundary tests asserting Released does NOT touch
// solver/verifier stakes_t entries or task_markets_t.total_escrow.
// Charter v2 § 4.8 — explicit boundary: ChallengeResolve.Released ONLY
// affects challenger bond + ChallengeCase.status; everything else is OUT
// OF SCOPE (TB-6 RSP-3.2 territory or TB-3/TB-4 admission territory).
// ────────────────────────────────────────────────────────────────────────────

fn q_with_full_economy(challenger: &str, bond_micro: i64) -> (QState, TxId, AgentId) {
    let mut q = QState::genesis();
    let challenger_id = AgentId(challenger.into());
    let solver_id = AgentId("solver-x".into());
    let verifier_id = AgentId("verifier-y".into());
    let task_id = TaskId("task-i78".into());
    let work_tx_id = TxId("wt-i78".into());
    let verify_tx_id = TxId("vt-i78".into());
    let challenge_tx_id = TxId(format!("ct-{challenger}"));
    let escrow_tx_id = TxId("et-i78".into());

    // Balances: challenger pre-debit (already 4 less than nominal).
    q.economic_state_t.balances_t.0.insert(challenger_id.clone(), MicroCoin::from_micro_units(96));
    q.economic_state_t.balances_t.0.insert(solver_id.clone(), MicroCoin::from_micro_units(50));
    q.economic_state_t.balances_t.0.insert(verifier_id.clone(), MicroCoin::from_micro_units(70));

    // Solver stake (TB-3 lock-on-accept) + verifier bond (TB-4) — both must
    // be byte-identical pre/post Released.
    q.economic_state_t.stakes_t.0.insert(
        work_tx_id.clone(),
        StakeEntry { amount: MicroCoin::from_micro_units(10), staker: solver_id, task_id: task_id.clone() },
    );
    q.economic_state_t.stakes_t.0.insert(
        verify_tx_id.clone(),
        StakeEntry { amount: MicroCoin::from_micro_units(7), staker: verifier_id, task_id: task_id.clone() },
    );

    // Task market with pinned total_escrow.
    let mut escrow_lock_tx_ids = std::collections::BTreeSet::new();
    escrow_lock_tx_ids.insert(escrow_tx_id.clone());
    q.economic_state_t.task_markets_t.0.insert(
        task_id.clone(),
        TaskMarketEntry {
            publisher: AgentId("publisher".into()),
            total_escrow: MicroCoin::from_micro_units(20),
            escrow_lock_tx_ids,
            verifier_quorum: 1,
            max_reuse_royalty_fraction_basis_points: 1000,
            settlement_rule_hash: turingosv4::state::q_state::Hash::ZERO,
        },
    );
    q.economic_state_t.escrows_t.0.insert(
        escrow_tx_id,
        EscrowEntry {
            amount: MicroCoin::from_micro_units(20),
            depositor: AgentId("publisher".into()),
            task_id: task_id.clone(),
        },
    );

    // The ChallengeCase under test.
    q.economic_state_t.challenge_cases_t.0.insert(
        challenge_tx_id.clone(),
        ChallengeCase {
            challenger: challenger_id.clone(),
            bond: MicroCoin::from_micro_units(bond_micro),
            opened_at_round: 7,
            target_work_tx: work_tx_id,
            status: ChallengeStatus::Open,
        },
    );

    // q.q_t.current_round non-zero so I88 can verify it doesn't move.
    q.q_t.current_round = 42;

    (q, challenge_tx_id, challenger_id)
}

#[tokio::test]
async fn released_does_not_release_solver_or_verifier_stakes() {
    // I78: stakes_t entries untouched after Released apply (TB-6 territory).
    let (q, target, _challenger) = q_with_full_economy("challenger-i78", 4);
    let pre_stakes = q.economic_state_t.stakes_t.clone();
    let mut h = fresh_harness_with(q);

    h.seq
        .emit_system_tx(SystemEmitCommand::ChallengeResolve {
            target_challenge_tx_id: target,
            resolution: ChallengeResolution::Released,
        })
        .await
        .expect("emit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("accept");

    let q_post = h.seq.q_snapshot().expect("snap");
    assert_eq!(
        q_post.economic_state_t.stakes_t, pre_stakes,
        "Released MUST NOT touch solver/verifier stakes_t (charter § 4.8 boundary)"
    );
}

#[tokio::test]
async fn released_does_not_decrement_total_escrow() {
    // I79: task_markets_t.total_escrow + escrows_t bit-identical post-Released.
    let (q, target, _challenger) = q_with_full_economy("challenger-i79", 4);
    let pre_markets = q.economic_state_t.task_markets_t.clone();
    let pre_escrows = q.economic_state_t.escrows_t.clone();
    let mut h = fresh_harness_with(q);

    h.seq
        .emit_system_tx(SystemEmitCommand::ChallengeResolve {
            target_challenge_tx_id: target,
            resolution: ChallengeResolution::Released,
        })
        .await
        .expect("emit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("accept");

    let q_post = h.seq.q_snapshot().expect("snap");
    assert_eq!(
        q_post.economic_state_t.task_markets_t, pre_markets,
        "Released MUST NOT decrement total_escrow (charter § 4.8 boundary)"
    );
    assert_eq!(
        q_post.economic_state_t.escrows_t, pre_escrows,
        "Released MUST NOT touch escrows_t"
    );
}

#[tokio::test]
async fn challenge_resolve_does_not_mutate_q_t_current_round() {
    // I88: q.q_t.current_round preserved across Released + UpheldDeferred.
    let (q, target, _challenger) = q_with_full_economy("challenger-i88", 4);
    let pre_round = q.q_t.current_round;
    assert_eq!(pre_round, 42);
    let mut h = fresh_harness_with(q);

    h.seq
        .emit_system_tx(SystemEmitCommand::ChallengeResolve {
            target_challenge_tx_id: target,
            resolution: ChallengeResolution::Released,
        })
        .await
        .expect("emit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("accept");

    let q_post = h.seq.q_snapshot().expect("snap");
    assert_eq!(
        q_post.q_t.current_round, pre_round,
        "ChallengeResolve dispatch MUST NOT mutate q.q_t.current_round (charter § 4.10)"
    );
}

#[tokio::test]
async fn upheld_deferred_keeps_solver_verifier_stakes_byte_identical() {
    // I89: parallel boundary check for UpheldDeferred — neither stakes_t nor
    // task_markets_t / escrows_t move. The UpheldDeferred path is marker-
    // only; only ChallengeCase.status changes.
    let (q, target, _challenger) = q_with_full_economy("challenger-i89", 4);
    let pre_stakes = q.economic_state_t.stakes_t.clone();
    let pre_markets = q.economic_state_t.task_markets_t.clone();
    let pre_escrows = q.economic_state_t.escrows_t.clone();
    let pre_balances = q.economic_state_t.balances_t.clone();
    let mut h = fresh_harness_with(q);

    h.seq
        .emit_system_tx(SystemEmitCommand::ChallengeResolve {
            target_challenge_tx_id: target,
            resolution: ChallengeResolution::UpheldDeferred,
        })
        .await
        .expect("emit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("accept");

    let q_post = h.seq.q_snapshot().expect("snap");
    assert_eq!(q_post.economic_state_t.stakes_t, pre_stakes);
    assert_eq!(q_post.economic_state_t.task_markets_t, pre_markets);
    assert_eq!(q_post.economic_state_t.escrows_t, pre_escrows);
    assert_eq!(q_post.economic_state_t.balances_t, pre_balances,
        "UpheldDeferred must NOT touch balances either");
}
