//! TB-5.0 substrate ingress-barrier integration tests.
//!
//! Charter: `handover/tracer_bullets/TB-5_charter_2026-04-30.md` v2 § 4.2 + § 4.9 + § 5.3.
//! Preflight: `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md` v2 § 3.2 + § 8.3.
//!
//! Per charter § 5.3 + preflight § 8.3, this file holds the integration tests
//! that verify the agent-ingress barrier rejects system-emitted variants
//! BEFORE queue insertion. Atom 2 covers I61-I63 + I67. The remaining tests
//! in §8.3 (I60 ChallengeResolve rejection; I64-I69 emit_system_tx tests)
//! are deferred to Atom 3 (ABI) and Atom 4 (emit_system_tx + apply_one
//! stage 1.5) respectively, since they require types / API not yet landed
//! at TB-5 Atom 2 HEAD.
//!
//! Constitutional anchor (Anti-Oreo per WP architecture § 12.4 + Constitution
//! Art V.1.3): system-emitted variants — FinalizeReward / TaskExpire /
//! TerminalSummary (and ChallengeResolve once Atom 3 lands) — must NOT
//! reach the queue through `Sequencer::submit` or `Sequencer::submit_agent_tx`.
//! Agent ≠ direct state writer. Live enforcement at TB-5.0 retires the
//! TB-3 + TB-4 documented-norm-without-enforcement debt.

use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, SystemEpoch, SystemSignature,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    InMemoryLedgerWriter, LedgerWriter,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::MicroCoin;
use turingosv4::state::q_state::{AgentId, Hash, QState, TaskId, TxId};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope, SubmitError};
use turingosv4::state::typed_tx::{
    ClaimId, FinalizeRewardTx, RunId, RunOutcome, TaskExpireTx, TerminalSummaryTx, TypedTx,
};

// ────────────────────────────────────────────────────────────────────────────
// Harness
// ────────────────────────────────────────────────────────────────────────────

struct Harness {
    _tmp: TempDir,
    seq: Sequencer,
    _rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    _ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
    _rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
}

fn fresh_harness() -> Harness {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(
        Ed25519Keypair::generate_with_secure_entropy().expect("keypair"),
    );
    let writer: Arc<RwLock<dyn LedgerWriter>> =
        Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
    let preds = Arc::new(turingosv4::top_white::predicates::registry::PredicateRegistry::new());
    let tools = Arc::new(ToolRegistry::new());
    let epoch = SystemEpoch::new(1);
    let (seq, rx) = Sequencer::new(
        cas,
        keypair,
        epoch,
        writer.clone(),
        rejection_writer.clone(),
        preds,
        tools,
        QState::genesis(),
        16,
    );
    Harness {
        _tmp: tmp,
        seq,
        _rx: rx,
        _ledger_writer: writer,
        _rejection_writer: rejection_writer,
    }
}

// ────────────────────────────────────────────────────────────────────────────
// I61 — agent-ingress rejects FinalizeRewardTx via Sequencer::submit_agent_tx
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn agent_submit_rejects_finalize_reward_tx() {
    let h = fresh_harness();
    let pre_submit_id = h.seq.next_submit_id_peek();

    let tx = TypedTx::FinalizeReward(FinalizeRewardTx {
        tx_id: TxId("ft-i61".into()),
        claim_id: ClaimId::new("cl-i61"),
        task_id: TaskId("t-i61".into()),
        solver: AgentId("solver".into()),
        reward: MicroCoin::from_micro_units(100),
        parent_state_root: Hash::ZERO,
        epoch: SystemEpoch::new(1),
        timestamp_logical: 1,
        system_signature: SystemSignature::from_bytes([0u8; 64]),
    });

    let err = h.seq.submit_agent_tx(tx).await.unwrap_err();
    assert!(
        matches!(err, SubmitError::SystemTxForbiddenOnAgentIngress),
        "expected SystemTxForbiddenOnAgentIngress, got {err:?}"
    );

    // submit_id NOT advanced — rejection is pre-queue, before fetch_add.
    // Anti-Oreo guarantee: agent-side ingress wastes no system resources
    // on forbidden variants.
    assert_eq!(h.seq.next_submit_id_peek(), pre_submit_id,
        "submit_id must not advance on system-tx ingress rejection");
}

// ────────────────────────────────────────────────────────────────────────────
// I62 — agent-ingress rejects TaskExpireTx via Sequencer::submit_agent_tx
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn agent_submit_rejects_task_expire_tx() {
    let h = fresh_harness();
    let pre_submit_id = h.seq.next_submit_id_peek();

    let tx = TypedTx::TaskExpire(TaskExpireTx {
        tx_id: TxId("et-i62".into()),
        task_id: TaskId("t-i62".into()),
        parent_state_root: Hash::ZERO,
        bounty_refunded: MicroCoin::from_micro_units(50),
        epoch: SystemEpoch::new(1),
        timestamp_logical: 1,
        system_signature: SystemSignature::from_bytes([0u8; 64]),
    });

    let err = h.seq.submit_agent_tx(tx).await.unwrap_err();
    assert!(
        matches!(err, SubmitError::SystemTxForbiddenOnAgentIngress),
        "expected SystemTxForbiddenOnAgentIngress, got {err:?}"
    );
    assert_eq!(h.seq.next_submit_id_peek(), pre_submit_id);
}

// ────────────────────────────────────────────────────────────────────────────
// I63 — agent-ingress rejects TerminalSummaryTx via Sequencer::submit_agent_tx
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn agent_submit_rejects_terminal_summary_tx() {
    let h = fresh_harness();
    let pre_submit_id = h.seq.next_submit_id_peek();

    let tx = TypedTx::TerminalSummary(TerminalSummaryTx {
        tx_id: TxId("ts-i63".into()),
        task_id: TaskId("t-i63".into()),
        run_id: RunId("r-i63".into()),
        run_outcome: RunOutcome::OmegaAccepted,
        total_attempts: 0,
        failure_class_histogram: BTreeMap::new(),
        last_logical_t: 0,
        system_signature: SystemSignature::from_bytes([0u8; 64]),
    });

    let err = h.seq.submit_agent_tx(tx).await.unwrap_err();
    assert!(
        matches!(err, SubmitError::SystemTxForbiddenOnAgentIngress),
        "expected SystemTxForbiddenOnAgentIngress, got {err:?}"
    );
    assert_eq!(h.seq.next_submit_id_peek(), pre_submit_id);
}

// ────────────────────────────────────────────────────────────────────────────
// I67 — legacy `Sequencer::submit` alias delegates to `submit_agent_tx`
// and inherits the system-variant rejection. Charter v2 § 4.2 binding.
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn legacy_submit_alias_delegates_to_submit_agent_tx_and_rejects_system_variants() {
    let h = fresh_harness();
    let pre_submit_id = h.seq.next_submit_id_peek();

    // Try all 3 system variants through legacy `submit()` alias.
    let cases: Vec<TypedTx> = vec![
        TypedTx::FinalizeReward(FinalizeRewardTx {
            tx_id: TxId("ft-i67".into()),
            claim_id: ClaimId::new("cl"),
            task_id: TaskId("t".into()),
            solver: AgentId("s".into()),
            reward: MicroCoin::from_micro_units(1),
            parent_state_root: Hash::ZERO,
            epoch: SystemEpoch::new(1),
            timestamp_logical: 1,
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        }),
        TypedTx::TaskExpire(TaskExpireTx {
            tx_id: TxId("et-i67".into()),
            task_id: TaskId("t".into()),
            parent_state_root: Hash::ZERO,
            bounty_refunded: MicroCoin::from_micro_units(1),
            epoch: SystemEpoch::new(1),
            timestamp_logical: 1,
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        }),
        TypedTx::TerminalSummary(TerminalSummaryTx {
            tx_id: TxId("ts-i67".into()),
            task_id: TaskId("t".into()),
            run_id: RunId("r".into()),
            run_outcome: RunOutcome::OmegaAccepted,
            total_attempts: 0,
            failure_class_histogram: BTreeMap::new(),
            last_logical_t: 0,
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        }),
    ];

    for tx in cases {
        let err = h.seq.submit(tx).await.unwrap_err();
        assert!(
            matches!(err, SubmitError::SystemTxForbiddenOnAgentIngress),
            "legacy submit() must inherit submit_agent_tx rejection; got {err:?}"
        );
    }

    // After 3 rejections through the legacy alias, submit_id is still
    // unchanged — the legacy alias correctly delegates pre-queue.
    assert_eq!(h.seq.next_submit_id_peek(), pre_submit_id,
        "legacy submit() must reject pre-queue (no submit_id burn) just like submit_agent_tx");
}
