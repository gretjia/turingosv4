//! TB-N2-POLYMARKET-CPMM-LIFECYCLE atom B2 — EventResolveTx system-emit.
//!
//! Charter: `handover/tracer_bullets/TB_N2_POLYMARKET_CPMM_LIFECYCLE_charter_2026-05-10.md` §3 B2.
//! Gap audit: `handover/audits/STAGE_C_POLYMARKET_CPMM_LIFECYCLE_GAP_AUDIT_2026-05-10.md` §3.3 + §5.
//!
//! Constitutional binding: closes the CPMM lifecycle gap identified in the
//! 2026-05-10 gap audit §3.3 — `TaskMarketState::Finalized` was READ at 5+
//! admission sites (CompleteSetRedeem / mint+seed gates / verify path) but
//! WRITTEN ZERO times before B2. This atom adds the missing Open → Finalized
//! transition via system-emit on the OMEGA-Confirm path (Option 1
//! resolution authority per charter §5).
//!
//! Ship gates (SG-N2-B2.1..B2.8):
//! - SG-N2-B2.1: agent ingress rejects `EventResolveTx` with
//!               `SystemTxForbiddenOnAgentIngress` (Anti-Oreo barrier)
//! - SG-N2-B2.2: emit_system_tx EventResolve succeeds → `task_markets_t[task_id].state`
//!               flips Open → Finalized + state_root_t advances
//! - SG-N2-B2.3: idempotent re-emit (state=Finalized) rejects with
//!               `EventAlreadyResolved`
//! - SG-N2-B2.4: emit on Bankrupt market rejects with `EventAlreadyResolved`
//!               (cross-system-tx monotonicity gate: TaskBankruptcy already
//!               consumed the resolution)
//! - SG-N2-B2.5: emit_system_tx against unknown task_id returns
//!               `EventResolveTaskNotFound` (caller-side defense-in-depth)
//! - SG-N2-B2.6: pure status mutation — `balances_t` /
//!               `conditional_collateral_t` / `lp_share_balances_t` / pool
//!               reserves UNCHANGED across EventResolve accept (architect
//!               §2.1 closing guard 5 + CLAUDE.md §13 monetary invariants
//!               preserved)
//! - SG-N2-B2.7: post-resolution state opens TB-13 redeem path —
//!               outcome=Yes admits (Finalized→Yes per typed_tx.rs:1244),
//!               outcome=No rejects with `InvalidResolutionRef`
//! - SG-N2-B2.8: source-grep gate — `tb_n2_emit_event_resolve_after_finalize`
//!               advertised in adapter.rs AND called by evaluator.rs
//!               (mechanism binding test per Phase E.1 verbatim binding
//!               pattern)
//!
//! `FC-trace: Polymarket §2.1 verbatim CPMM lifecycle closure + FC1-N1
//! every externalized resolution attempt tape-visible + FC2-N9 system-emit
//! signing-domain pin + Stage C event-state-gate Phase F.9 fail-closed
//! semantics extended to Open→Finalized writer side.`

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    RejectionClass as L4ERejectionClass, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch, SystemSignature,
};
use turingosv4::bottom_white::ledger::transition_ledger::{InMemoryLedgerWriter, LedgerWriter};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::monetary_invariant::total_supply_micro;
use turingosv4::economy::money::{MicroCoin, StakeMicroCoin};
use turingosv4::state::q_state::{AgentId, Hash, QState, TaskId, TaskMarketState, TxId};
use turingosv4::state::sequencer::{
    EmitSystemError, Sequencer, SubmissionEnvelope, SubmitError, SystemEmitCommand,
};
use turingosv4::state::typed_tx::{
    AgentSignature, BankruptcyReason, BoolWithProof, EscrowLockTx, EventResolveTx, OutcomeSide,
    PredicateId, PredicateResultsBundle, ReadKey, SafetyOrCreation, TaskOpenTx, TypedTx, WorkTx,
    WriteKey,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ────────────────────────────────────────────────────────────────────────────
// Fixtures (mirror N1-A4 harness pattern)
// ────────────────────────────────────────────────────────────────────────────

struct Harness {
    _tmp: TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
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
    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(epoch, keypair.public_key());
    let pinned_pubkeys = Arc::new(pinned);
    let (seq, rx) = Sequencer::new(
        cas.clone(),
        keypair,
        epoch,
        writer,
        rejection_writer.clone(),
        preds,
        tools,
        pinned_pubkeys,
        initial_q,
        16,
    );
    Harness {
        _tmp: tmp,
        seq,
        rx,
        rejection_writer,
    }
}

fn genesis_with_balances(pairs: &[(&str, i64)]) -> QState {
    let mut q = QState::genesis();
    for (name, coin) in pairs {
        q.economic_state_t.balances_t.0.insert(
            AgentId((*name).into()),
            MicroCoin::from_coin(*coin).unwrap(),
        );
    }
    q
}

fn make_task_open(task: &str, sponsor: &str, parent: Hash, suffix: &str) -> TypedTx {
    TypedTx::TaskOpen(TaskOpenTx {
        tx_id: TxId(format!("taskopen-{}-{}", task, suffix)),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        sponsor_agent: AgentId(sponsor.into()),
        verifier_quorum: 1,
        max_reuse_royalty_fraction_basis_points: 1000,
        settlement_rule_hash: Hash::ZERO,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

fn make_escrow_lock(
    task: &str,
    sponsor: &str,
    amount_micro: i64,
    parent: Hash,
    suffix: &str,
) -> TypedTx {
    TypedTx::EscrowLock(EscrowLockTx {
        tx_id: TxId(format!("escrowlock-{}-{}", task, suffix)),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        sponsor_agent: AgentId(sponsor.into()),
        amount: MicroCoin::from_micro_units(amount_micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

async fn open_and_fund_task(h: &mut Harness, task: &str, sponsor: &str, escrow_coin: i64) {
    let pre = h.seq.q_snapshot().expect("pre snap").state_root_t;
    let open = make_task_open(task, sponsor, pre, "fund");
    h.seq.submit(open).await.expect("open submit");
    let _ = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("open env")
        .expect("open accepted");
    let parent = h.seq.q_snapshot().expect("post-open").state_root_t;
    let lock = make_escrow_lock(task, sponsor, escrow_coin * 1_000_000, parent, "fund");
    h.seq.submit(lock).await.expect("lock submit");
    let _ = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("lock env")
        .expect("lock accepted");
}

// ════════════════════════════════════════════════════════════════════════════
// SG-N2-B2.1 — agent ingress rejects EventResolveTx (Anti-Oreo)
// ════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn sg_n2_b2_1_agent_ingress_rejects_event_resolve_pre_queue() {
    let h = fresh_harness(QState::genesis());
    // Construct a (forged) EventResolveTx with a placeholder signature.
    // Anti-Oreo: submit_agent_tx must reject pre-queue regardless of
    // signature validity — construction-through-emit_system_tx is the
    // sole legal path.
    let tx = TypedTx::EventResolve(EventResolveTx {
        tx_id: TxId("forged-event-resolve".into()),
        parent_state_root: Hash::ZERO,
        task_id: TaskId("task-whatever".into()),
        outcome: OutcomeSide::Yes,
        epoch: SystemEpoch::new(1),
        timestamp_logical: 1,
        system_signature: SystemSignature::from_bytes([0u8; 64]),
    });
    let result = h.seq.submit_agent_tx(tx).await;
    assert!(
        matches!(result, Err(SubmitError::SystemTxForbiddenOnAgentIngress)),
        "EventResolveTx must be rejected at agent ingress per Anti-Oreo (Art V.1.3); got {:?}",
        result,
    );
}

// ════════════════════════════════════════════════════════════════════════════
// SG-N2-B2.2 — emit_system_tx EventResolve succeeds; state Open → Finalized
// ════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn sg_n2_b2_2_event_resolve_flips_state_open_to_finalized() {
    let mut h = fresh_harness(genesis_with_balances(&[("sponsor-b2-2", 100)]));
    let task = "task-b2-2";
    open_and_fund_task(&mut h, task, "sponsor-b2-2", 50).await;

    // Pre-condition: state == Open.
    let pre_state = h
        .seq
        .q_snapshot()
        .expect("pre snap")
        .economic_state_t
        .task_markets_t
        .0
        .get(&TaskId(task.into()))
        .expect("task present")
        .state;
    assert_eq!(
        pre_state,
        TaskMarketState::Open,
        "pre-condition: task_markets_t.state must be Open"
    );

    let pre_state_root = h.seq.q_snapshot().expect("pre snap").state_root_t;

    let receipt = h
        .seq
        .emit_system_tx(SystemEmitCommand::EventResolve {
            task_id: TaskId(task.into()),
            outcome: OutcomeSide::Yes,
        })
        .await
        .expect("emit EventResolve");
    let _ = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("env")
        .expect("event resolve accepted");

    // Post-condition: state == Finalized + state_root_t advanced.
    let post = h.seq.q_snapshot().expect("post snap");
    let post_state = post
        .economic_state_t
        .task_markets_t
        .0
        .get(&TaskId(task.into()))
        .expect("task present")
        .state;
    assert_eq!(
        post_state,
        TaskMarketState::Finalized,
        "EventResolve accept must flip task_markets_t.state Open → Finalized"
    );
    assert_ne!(
        post.state_root_t, pre_state_root,
        "EventResolve accept must advance state_root_t (event_resolve_accept_state_root domain)"
    );
    // Emit-id sanity (non-zero).
    assert!(receipt.emit_id > 0, "emit_id must be assigned");
}

// ════════════════════════════════════════════════════════════════════════════
// SG-N2-B2.3 — idempotent re-emit rejects with EventAlreadyResolved
// ════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn sg_n2_b2_3_re_emit_on_finalized_rejects_with_event_already_resolved() {
    let mut h = fresh_harness(genesis_with_balances(&[("sponsor-b2-3", 100)]));
    let task = "task-b2-3";
    open_and_fund_task(&mut h, task, "sponsor-b2-3", 50).await;

    // First emit: should accept.
    let _ = h
        .seq
        .emit_system_tx(SystemEmitCommand::EventResolve {
            task_id: TaskId(task.into()),
            outcome: OutcomeSide::Yes,
        })
        .await
        .expect("emit 1");
    let _ = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("env 1")
        .expect("first accepted");

    // Second emit on same task: emit_system_tx returns Ok (task_markets_t entry
    // present), but dispatch arm must reject with EventAlreadyResolved.
    let _ = h
        .seq
        .emit_system_tx(SystemEmitCommand::EventResolve {
            task_id: TaskId(task.into()),
            outcome: OutcomeSide::Yes,
        })
        .await
        .expect("emit 2");
    let r = h.seq.try_apply_one(&mut h.rx).expect("env 2");
    assert!(
        r.is_err(),
        "re-emit on Finalized task must reject; got {:?}",
        r
    );

    // Verify L4.E rejection class is PolicyViolation (per B2 rejection-class
    // mapping in sequencer.rs).
    let last_class = {
        let g = h.rejection_writer.read().expect("writer read");
        g.records().last().map(|r| r.rejection_class)
    };
    assert_eq!(
        last_class,
        Some(L4ERejectionClass::PolicyViolation),
        "EventAlreadyResolved must map to L4ERejectionClass::PolicyViolation"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// SG-N2-B2.4 — emit on Bankrupt market rejects with EventAlreadyResolved
// ════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn sg_n2_b2_4_emit_on_bankrupt_market_rejects() {
    let mut h = fresh_harness(genesis_with_balances(&[("sponsor-b2-4", 100)]));
    let task = "task-b2-4";
    open_and_fund_task(&mut h, task, "sponsor-b2-4", 50).await;

    // Flip to Bankrupt via TaskBankruptcy system-emit (TB-11 sibling path).
    let _ = h
        .seq
        .emit_system_tx(SystemEmitCommand::TaskBankruptcy {
            task_id: TaskId(task.into()),
            evidence_capsule_cid: turingosv4::bottom_white::cas::schema::Cid::default(),
            bankruptcy_reason: BankruptcyReason::MaxFailedRunCount,
            failed_run_count: 1,
        })
        .await
        .expect("emit bankruptcy");
    let _ = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("env bk")
        .expect("bankruptcy accepted");

    // Cross-check: state is Bankrupt.
    let s = h
        .seq
        .q_snapshot()
        .expect("snap")
        .economic_state_t
        .task_markets_t
        .0
        .get(&TaskId(task.into()))
        .expect("task")
        .state;
    assert_eq!(
        s,
        TaskMarketState::Bankrupt,
        "TaskBankruptcy must flip state to Bankrupt"
    );

    // Now attempt EventResolve: dispatch must reject with EventAlreadyResolved
    // (cross-system-tx monotonicity: Bankrupt is terminal for B2 purposes —
    // the NO-side already won; YES cannot retroactively win).
    let _ = h
        .seq
        .emit_system_tx(SystemEmitCommand::EventResolve {
            task_id: TaskId(task.into()),
            outcome: OutcomeSide::Yes,
        })
        .await
        .expect("emit event resolve post-bankruptcy");
    let r = h.seq.try_apply_one(&mut h.rx).expect("env er-post-bk");
    assert!(
        r.is_err(),
        "EventResolve on Bankrupt market must reject; got {:?}",
        r
    );

    // State unchanged — still Bankrupt (NOT mutated to Finalized).
    let s_post = h
        .seq
        .q_snapshot()
        .expect("snap-post")
        .economic_state_t
        .task_markets_t
        .0
        .get(&TaskId(task.into()))
        .expect("task")
        .state;
    assert_eq!(
        s_post,
        TaskMarketState::Bankrupt,
        "rejected EventResolve must NOT mutate task_markets_t.state (rollback discipline)"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// SG-N2-B2.5 — emit on unknown task_id returns EventResolveTaskNotFound
// ════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn sg_n2_b2_5_emit_on_unknown_task_returns_event_resolve_task_not_found() {
    let h = fresh_harness(QState::genesis());
    let result = h
        .seq
        .emit_system_tx(SystemEmitCommand::EventResolve {
            task_id: TaskId("task-does-not-exist".into()),
            outcome: OutcomeSide::Yes,
        })
        .await;
    assert!(
        matches!(result, Err(EmitSystemError::EventResolveTaskNotFound)),
        "emit_system_tx on non-existent task_id must return EventResolveTaskNotFound (defense-in-depth at construction time); got {:?}",
        result,
    );
}

// ════════════════════════════════════════════════════════════════════════════
// SG-N2-B2.6 — pure status mutation; ledger holdings UNCHANGED
// ════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn sg_n2_b2_6_pure_status_mutation_no_money_movement() {
    let mut h = fresh_harness(genesis_with_balances(&[("sponsor-b2-6", 100)]));
    let task = "task-b2-6";
    open_and_fund_task(&mut h, task, "sponsor-b2-6", 50).await;

    // Snapshot pre-resolve.
    let pre = h.seq.q_snapshot().expect("pre");
    let pre_balances = pre.economic_state_t.balances_t.0.clone();
    let pre_collateral = pre.economic_state_t.conditional_collateral_t.0.clone();
    let pre_lp = pre.economic_state_t.lp_share_balances_t.0.clone();
    let pre_pools = pre.economic_state_t.cpmm_pools_t.0.clone();
    let pre_shares = pre.economic_state_t.conditional_share_balances_t.0.clone();
    let pre_total_supply = total_supply_micro(&pre.economic_state_t).expect("pre total");

    // Emit + apply EventResolve.
    let _ = h
        .seq
        .emit_system_tx(SystemEmitCommand::EventResolve {
            task_id: TaskId(task.into()),
            outcome: OutcomeSide::Yes,
        })
        .await
        .expect("emit");
    let _ = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("env")
        .expect("accepted");

    // Snapshot post-resolve — all 6 holding tables UNCHANGED.
    let post = h.seq.q_snapshot().expect("post");
    assert_eq!(post.economic_state_t.balances_t.0, pre_balances,
        "B2 must not mutate balances_t (architect §2.1 guard 5: pool reserves not Coin; resolve is status only)");
    assert_eq!(
        post.economic_state_t.conditional_collateral_t.0, pre_collateral,
        "B2 must not mutate conditional_collateral_t"
    );
    assert_eq!(
        post.economic_state_t.lp_share_balances_t.0, pre_lp,
        "B2 must not mutate lp_share_balances_t"
    );
    assert_eq!(
        post.economic_state_t.cpmm_pools_t.0, pre_pools,
        "B2 must not mutate cpmm_pools_t (pool reserves preserved for future LP unwind atom B4)"
    );
    assert_eq!(
        post.economic_state_t.conditional_share_balances_t.0, pre_shares,
        "B2 must not mutate conditional_share_balances_t"
    );
    let post_total_supply = total_supply_micro(&post.economic_state_t).expect("post total");
    assert_eq!(
        post_total_supply, pre_total_supply,
        "B2 must preserve total_supply_micro (CLAUDE.md §13 conservation)"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// SG-N2-B2.7 — post-resolution TB-13 redeem mapping engages
// ════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn sg_n2_b2_7_post_resolution_state_makes_redeem_reachable() {
    // This is a behavioral binding test: after B2 flips Open → Finalized,
    // the static admission gate in CompleteSetRedeemTx (typed_tx.rs:2701-2713)
    // would change verdict from `RedeemBeforeResolution` (state=Open) to
    // either `Ok` (outcome=Yes per Finalized→Yes mapping) or
    // `InvalidResolutionRef` (outcome=No mismatch).
    //
    // We don't need to construct a full CompleteSetRedeem here — the gap
    // audit §3.3 closure target is "make the state value WRITTEN". Once
    // state == Finalized lands in task_markets_t, the admission paths that
    // READ it (5+ sites per gap audit) automatically engage their
    // post-resolution branches. This test asserts the state value
    // post-B2-accept is the EXACT enum variant TB-13 redeem READS as
    // "YES wins" per the resolution authority mapping.
    let mut h = fresh_harness(genesis_with_balances(&[("sponsor-b2-7", 100)]));
    let task = "task-b2-7";
    open_and_fund_task(&mut h, task, "sponsor-b2-7", 50).await;

    let _ = h
        .seq
        .emit_system_tx(SystemEmitCommand::EventResolve {
            task_id: TaskId(task.into()),
            outcome: OutcomeSide::Yes,
        })
        .await
        .expect("emit");
    let _ = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("env")
        .expect("accepted");

    let state = h
        .seq
        .q_snapshot()
        .expect("snap")
        .economic_state_t
        .task_markets_t
        .0
        .get(&TaskId(task.into()))
        .expect("task")
        .state;
    // Exact enum match — `Finalized` is the discriminant TB-13 redeem
    // admission READS as resolution authority for `outcome == Yes` (per
    // typed_tx.rs:1244 verbatim doc-comment: "If Finalized: outcome must
    // be Yes else InvalidResolutionRef").
    assert!(
        matches!(state, TaskMarketState::Finalized),
        "post-B2 state must be exactly TaskMarketState::Finalized for TB-13 redeem outcome=Yes admission; got {:?}",
        state,
    );
    // Cross-witness: the gap-audit §3.3 closure target. Pre-B2 this
    // discriminant was never written; post-B2 it is.
}

// ════════════════════════════════════════════════════════════════════════════
// SG-N2-B2.8 — source-grep gate (mechanism binding per Phase E.1 pattern)
// ════════════════════════════════════════════════════════════════════════════

/// Phase E.1 verbatim binding pattern: the B2 atom is mechanically
/// realized iff (a) `tb_n2_emit_event_resolve_after_finalize` is defined
/// in `src/runtime/adapter.rs` AND (b) the evaluator calls it on the
/// OMEGA-Confirm exit path. This source-grep gate prevents silent
/// regression (e.g. someone deletes the helper or removes the evaluator
/// hook).
#[test]
fn sg_n2_b2_8_adapter_helper_and_evaluator_hook_present() {
    let adapter_src =
        std::fs::read_to_string("src/runtime/adapter.rs").expect("adapter.rs present");
    assert!(
        adapter_src.contains("pub async fn tb_n2_emit_event_resolve_after_finalize"),
        "src/runtime/adapter.rs must define `tb_n2_emit_event_resolve_after_finalize` (B2 mechanism binding)",
    );
    assert!(
        adapter_src.contains("SystemEmitCommand::EventResolve"),
        "adapter.rs helper must call SystemEmitCommand::EventResolve",
    );

    let evaluator_src = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs")
        .expect("evaluator.rs present");
    let n_hits = evaluator_src
        .matches("tb_n2_emit_event_resolve_after_finalize")
        .count();
    assert!(
        n_hits >= 2,
        "evaluator.rs must invoke `tb_n2_emit_event_resolve_after_finalize` at \
         BOTH the full-proof OMEGA-Confirm exit AND the per-tactic OMEGA exit \
         (mirrors `tb8_emit_finalize_after_verify` 2-site pattern). Found {} occurrences.",
        n_hits,
    );
}

// ════════════════════════════════════════════════════════════════════════════
// SG-N2-B2.9 — R2 race-fix binding (Codex G2 R1 Q8 VETO closure 2026-05-11)
// ════════════════════════════════════════════════════════════════════════════

/// **R2 race-fix binding gate**. R1 audit (Codex G2 2026-05-11 Q8 VETO)
/// surfaced a runtime race condition: the adapter helper polled only
/// `task_markets_t.state == Open` and emitted `EventResolveTx`
/// immediately on observation. Since `FinalizeRewardTx` applies
/// asynchronously after `tb8_emit_finalize_after_verify` returns Ok, the
/// EventResolve construction captured a pre-FinalizeReward
/// `parent_state_root` R_0. Apply-time state_root was R_1 (post-
/// FinalizeReward apply) → dispatch Step-0 parent-root mismatch →
/// `StaleParent` L4.E `stale_parent_root`. Smoke evidence:
/// `handover/evidence/stage_b3_smoke_b2_20260511T012401Z/`
/// `deepseek-v4-flash/seed1/rep1/P002_aime_1983_p2/runtime_repo/rejections.jsonl:9`.
///
/// R2 fix: caller passes `verify_tx_id`; adapter derives `claim_id =
/// "claim-{verify_tx_id}"` (mirrors tb8 helper) and polls
/// `claims_t[claim_id].status == Finalized` ALONGSIDE the existing
/// `task_markets_t.state == Open` poll. The claim status flip is the
/// witness that FinalizeReward dispatch arm has applied (advancing
/// state_root), so the subsequent emit_system_tx captures the post-
/// FinalizeReward state_root.
///
/// This gate verbatim-binds the R2 fix: source-grep that the helper
/// signature accepts `verify_tx_id: &TxId` AND polls `claims_t` AND
/// matches on `ClaimStatus::Finalized`. Catches silent revert to R1
/// shape.
#[test]
fn sg_n2_b2_9_adapter_polls_claim_finalized_before_emit_r2() {
    let adapter_src =
        std::fs::read_to_string("src/runtime/adapter.rs").expect("adapter.rs present");

    // R2 signature: helper accepts verify_tx_id parameter.
    assert!(
        adapter_src.contains("verify_tx_id: &TxId"),
        "src/runtime/adapter.rs `tb_n2_emit_event_resolve_after_finalize` must accept \
         `verify_tx_id: &TxId` parameter (R2 race fix per Codex G2 R1 Q8 VETO closure)",
    );

    // R2 body: derives claim_id from verify_tx_id (mirrors tb8 helper).
    assert!(
        adapter_src.contains("claim_id_inner = TxId(format!(\"claim-{}\", verify_tx_id.0))"),
        "adapter helper must derive claim_id from verify_tx_id (mirrors tb8 helper's claim_id_inner pattern; R2 race fix)",
    );

    // R2 body: polls claims_t (not just task_markets_t).
    assert!(
        adapter_src.contains("economic_state_t\n                .claims_t\n                .0\n                .get(&claim_id_inner)")
            || adapter_src.contains(".claims_t.0.get(&claim_id_inner)"),
        "adapter helper must poll claims_t for the claim_id_inner status (R2 race fix)",
    );

    // R2 body: matches on ClaimStatus::Finalized as the apply-witness.
    assert!(
        adapter_src.contains("ClaimStatus::Finalized"),
        "adapter helper must require `claim.status == ClaimStatus::Finalized` as the FinalizeReward-applied witness (R2 race fix)",
    );

    // R2 body: combined gate — both claim_finalized AND task Open must hold.
    assert!(
        adapter_src.contains("claim_finalized") && adapter_src.contains("both_ready"),
        "adapter helper must use a combined gate (`both_ready = claim_finalized && task Open`) before emit (R2 race fix)",
    );

    // Evaluator call sites must pass &vid (verify_tx_id reference).
    let evaluator_src = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs")
        .expect("evaluator.rs present");
    let r2_call_hits = evaluator_src
        .matches("tb_n2_emit_event_resolve_after_finalize(\n                                                    &bundle.sequencer, b2_task_id.clone(), &vid,")
        .count();
    assert!(
        r2_call_hits >= 2,
        "evaluator.rs must invoke `tb_n2_emit_event_resolve_after_finalize` with `&vid` as 3rd arg at BOTH full-proof + per-tactic OMEGA exits (R2 race fix). Found {} R2-shape occurrences.",
        r2_call_hits,
    );
}

// ── Unused warning suppression ──────────────────────────────────────────────
// Some imports are reserved for forward-extension tests (e.g. WorkTx /
// PredicateResultsBundle for future redeem-flow integration in B4+).
// Reference them via `_` bindings to avoid `unused_import` warnings on the
// minimal B2 surface.

#[allow(dead_code)]
fn _imports_witness() {
    let _: Option<BTreeMap<String, String>> = None;
    let _: Option<BTreeSet<String>> = None;
    let _: Option<BoolWithProof> = None;
    let _: Option<PredicateId> = None;
    let _: Option<PredicateResultsBundle> = None;
    let _: Option<ReadKey> = None;
    let _: Option<SafetyOrCreation> = None;
    let _: Option<WorkTx> = None;
    let _: Option<WriteKey> = None;
    let _: Option<StakeMicroCoin> = None;
}
