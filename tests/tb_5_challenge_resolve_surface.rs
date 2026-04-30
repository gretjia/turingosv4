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
    AgentId, ChallengeCase, ChallengeStatus, QState, TxId,
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
