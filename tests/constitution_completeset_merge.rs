//! Constitution gate — Stage C P-M2 / Phase F.1 CompleteSetMergeTx
//! verbatim semantics per architect manual §7.3.
//!
//! Authority: `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md`
//! §7.3 (verbatim 6-field struct + 6-line semantics block + 5 mandatory tests).
//! Companion: `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md`
//! §1.C row 1 (rebuild post Stage C VETO; strict 6-field — NO `timestamp_logical`
//! drift; mechanism gates Phase E.1 + E.2 + E.3 in place to catch defect-class
//! recurrence).
//!
//! Test names mirror the architect §7.3 verbatim list:
//!   1. merge_yes_no_returns_coin
//!   2. merge_requires_both_sides
//!   3. merge_conserves_total_coin
//!   4. merge_reduces_collateral
//!   5. merge_unavailable_after_final_redeem_if_shares_exhausted
//!
//! Each test exercises the actual sequencer accept arm via `submit_and_apply`
//! against a real `Sequencer` + `InMemoryLedgerWriter`. No fixture forgery —
//! the post-state under test is reached by submitting CompleteSetMintTx /
//! CompleteSetMergeTx / CompleteSetRedeemTx through the live ingress path.
//! Per `feedback_tape_first_real_tests`: no tape activity = not a TuringOS
//! test; these tests advance state_root_t through real transitions.

use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    InMemoryLedgerWriter, LedgerWriter,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::MicroCoin;
use turingosv4::economy::monetary_invariant::{
    assert_complete_set_balanced, assert_total_ctf_conserved,
};
use turingosv4::state::q_state::{
    AgentId, QState, TaskId, TaskMarketEntry, TaskMarketState, TxId,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, CompleteSetMergeTx, CompleteSetMintTx, CompleteSetRedeemTx,
    EventId, OutcomeSide, ShareAmount, TypedTx,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ── Harness (mirrors tb_13_complete_set.rs pattern) ─────────────────────────

struct Harness {
    _tmp: TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    _ledger: Arc<RwLock<dyn LedgerWriter>>,
}

fn fresh_harness(initial_q: QState) -> Harness {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("kp"));
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
        cas, keypair, epoch, writer.clone(), rejection_writer, preds, tools,
        pinned_pubkeys, initial_q, 16,
    );
    Harness { _tmp: tmp, seq, rx, _ledger: writer }
}

fn genesis_with_balances_and_open_task(
    pairs: &[(&str, i64)],
    task: &str,
) -> QState {
    let mut q = QState::genesis();
    for (name, coin) in pairs {
        q.economic_state_t.balances_t.0.insert(
            AgentId((*name).into()),
            MicroCoin::from_coin(*coin).unwrap(),
        );
    }
    let mut entry = TaskMarketEntry::default();
    entry.state = TaskMarketState::Open;
    q.economic_state_t
        .task_markets_t
        .0
        .insert(TaskId(task.into()), entry);
    q
}

async fn submit_and_apply(h: &mut Harness, tx: TypedTx) -> Result<(), String> {
    h.seq
        .submit_agent_tx(tx)
        .await
        .map_err(|e| format!("submit error: {e:?}"))?;
    let outcome = h
        .seq
        .try_apply_one(&mut h.rx)
        .ok_or_else(|| "no envelope drained".to_string())?;
    outcome
        .map(|_ledger_entry| ())
        .map_err(|e| format!("apply error: {e:?}"))
}

fn build_mint(
    parent: turingosv4::state::q_state::Hash,
    owner: &str,
    task: &str,
    micro: i64,
    seq_no: u64,
) -> TypedTx {
    TypedTx::CompleteSetMint(CompleteSetMintTx {
        tx_id: TxId(format!("mint-{owner}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        owner: AgentId(owner.into()),
        amount: MicroCoin::from_micro_units(micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1000 + seq_no,
    })
}

fn build_merge(
    parent: turingosv4::state::q_state::Hash,
    owner: &str,
    task: &str,
    units: u128,
    seq_no: u64,
) -> TypedTx {
    TypedTx::CompleteSetMerge(CompleteSetMergeTx {
        tx_id: TxId(format!("merge-{owner}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        owner: AgentId(owner.into()),
        amount: ShareAmount::from_units(units),
        signature: AgentSignature::from_bytes([0u8; 64]),
    })
}

fn build_redeem(
    parent: turingosv4::state::q_state::Hash,
    owner: &str,
    task: &str,
    outcome: OutcomeSide,
    units: u128,
    seq_no: u64,
) -> TypedTx {
    TypedTx::CompleteSetRedeem(CompleteSetRedeemTx {
        tx_id: TxId(format!("redeem-{owner}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        owner: AgentId(owner.into()),
        outcome,
        share_amount: ShareAmount::from_units(units),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 2000 + seq_no,
    })
}

// ── Architect §7.3 verbatim test 1 ──────────────────────────────────────────

/// merge_yes_no_returns_coin — architect §7.3 verbatim: 1 YES + 1 NO -> 1 Coin.
/// Owner mints conditional shares (Coin → YES + NO), then merges them back.
/// Post-merge: balance restored, collateral debited, both share sides debited.
#[tokio::test]
async fn merge_yes_no_returns_coin() {
    let q0 = genesis_with_balances_and_open_task(&[("alice", 10)], "evt-1");
    let mut h = fresh_harness(q0);

    let parent_after_mint = {
        let parent = h.seq.q_snapshot().unwrap().state_root_t;
        submit_and_apply(&mut h, build_mint(parent, "alice", "evt-1", 5_000_000, 1))
            .await
            .expect("mint accepted");
        h.seq.q_snapshot().unwrap().state_root_t
    };

    // Pre-merge sanity: YES + NO each at 5_000_000 units; collateral at 5_000_000;
    // alice balance at 5_000_000 (10 Coin = 10_000_000 micro − 5_000_000 mint).
    {
        let q = h.seq.q_snapshot().unwrap();
        let alice_bal = q
            .economic_state_t
            .balances_t
            .0
            .get(&AgentId("alice".into()))
            .copied()
            .unwrap();
        assert_eq!(alice_bal.micro_units(), 5_000_000);
        let pair = q
            .economic_state_t
            .conditional_share_balances_t
            .0
            .get(&AgentId("alice".into()))
            .and_then(|m| m.get(&EventId(TaskId("evt-1".into()))))
            .copied()
            .unwrap();
        assert_eq!(pair.yes.units, 5_000_000_u128);
        assert_eq!(pair.no.units, 5_000_000_u128);
    }

    // Merge 3_000_000 YES + 3_000_000 NO -> 3_000_000 Coin.
    submit_and_apply(
        &mut h,
        build_merge(parent_after_mint, "alice", "evt-1", 3_000_000, 1),
    )
    .await
    .expect("merge accepted");

    let q = h.seq.q_snapshot().unwrap();
    let alice_bal = q
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("alice".into()))
        .copied()
        .unwrap();
    // Post-merge balance: 5_000_000 (post-mint residual) + 3_000_000 merge-credit = 8_000_000.
    assert_eq!(
        alice_bal.micro_units(),
        8_000_000,
        "merge must credit owner balance 1:1 with amount"
    );

    let pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("alice".into()))
        .and_then(|m| m.get(&EventId(TaskId("evt-1".into()))))
        .copied()
        .unwrap();
    assert_eq!(pair.yes.units, 2_000_000_u128, "YES shares debited by amount");
    assert_eq!(pair.no.units, 2_000_000_u128, "NO shares debited by amount");
}

// ── Architect §7.3 verbatim test 2 ──────────────────────────────────────────

/// merge_requires_both_sides — architect §7.3 verbatim: BOTH YES and NO must
/// be present in matching `amount`. Restructured per Codex R1 CHALLENGE Q2
/// remediation 2026-05-09 to be FULLY LIVE (no fixture-side state flip + no
/// harness reseat): mint live, merge-all live consumes both sides, then
/// attempt-merge live fails on share-balance preconditions.
#[tokio::test]
async fn merge_requires_both_sides() {
    let q0 = genesis_with_balances_and_open_task(&[("bob", 10)], "evt-2");
    let mut h = fresh_harness(q0);

    // Step 1: mint live 4_000_000 (Open state) → bob holds 4M YES + 4M NO.
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(parent, "bob", "evt-2", 4_000_000, 1))
        .await
        .expect("mint accepted");

    // Step 2: merge-all live 4_000_000 — consumes both sides through the
    // live sequencer accept arm; post-merge bob holds 0 YES + 0 NO.
    let parent_after_mint = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_merge(parent_after_mint, "bob", "evt-2", 4_000_000, 1))
        .await
        .expect("merge-all accepted");

    // Sanity: confirm both sides at zero post-merge.
    {
        let q = h.seq.q_snapshot().unwrap();
        let pair = q
            .economic_state_t
            .conditional_share_balances_t
            .0
            .get(&AgentId("bob".into()))
            .and_then(|m| m.get(&EventId(TaskId("evt-2".into()))))
            .copied()
            .unwrap_or_default();
        assert_eq!(pair.yes.units, 0);
        assert_eq!(pair.no.units, 0);
    }

    // Step 3: attempt merge live 1_000_000 — must fail because BOTH sides
    // are now < amount (both at 0). Live rejection through the sequencer
    // accept arm; not a fixture-forge.
    let parent_after_merge = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h,
        build_merge(parent_after_merge, "bob", "evt-2", 1_000_000, 1),
    )
    .await
    .expect_err("merge must fail when both sides are exhausted");
    assert!(
        err.contains("InsufficientSharesForMerge"),
        "expected InsufficientSharesForMerge, got: {err}"
    );
}

// ── Architect §7.3 verbatim test 3 ──────────────────────────────────────────

/// merge_conserves_total_coin — architect §7.3 verbatim: merge is the
/// bit-for-bit inverse of CompleteSetMint. The 6-holding CTF total is
/// preserved across mint→merge round-trips (post-merge state is identical
/// in total to pre-mint state for any held position).
#[tokio::test]
async fn merge_conserves_total_coin() {
    let q0 = genesis_with_balances_and_open_task(&[("carol", 7)], "evt-3");
    let q_initial_econ = q0.economic_state_t.clone();
    let mut h = fresh_harness(q0);

    // Round-trip: mint 2_000_000 then merge 2_000_000.
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(parent, "carol", "evt-3", 2_000_000, 1))
        .await
        .expect("mint accepted");
    let parent2 = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_merge(parent2, "carol", "evt-3", 2_000_000, 1))
        .await
        .expect("merge accepted");

    let q_final = h.seq.q_snapshot().unwrap();
    // The 6-holding CTF total must equal genesis total. assert_total_ctf_conserved
    // checks `pre.total == post.total`; if either mint or merge double-counted
    // shares as Coin, this would fail.
    assert_total_ctf_conserved(&q_initial_econ, &q_final.economic_state_t, &[])
        .expect("total Coin conserved across mint→merge round-trip");
    // Complete-set balance must hold (no orphan shares + no orphan collateral).
    assert_complete_set_balanced(&q_final.economic_state_t)
        .expect("complete-set balanced post-round-trip");
}

// ── Architect §7.3 verbatim test 4 ──────────────────────────────────────────

/// merge_reduces_collateral — architect §7.3 verbatim:
/// "conditional_collateral_t[event] -= amount". Mint credits collateral by
/// amount; merge debits it by amount. Net-zero round-trip.
#[tokio::test]
async fn merge_reduces_collateral() {
    let q0 = genesis_with_balances_and_open_task(&[("dave", 9)], "evt-4");
    let mut h = fresh_harness(q0);

    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(parent, "dave", "evt-4", 6_000_000, 1))
        .await
        .expect("mint accepted");

    // Mid-state: collateral at 6_000_000.
    {
        let q = h.seq.q_snapshot().unwrap();
        let collateral = q
            .economic_state_t
            .conditional_collateral_t
            .0
            .get(&EventId(TaskId("evt-4".into())))
            .copied()
            .unwrap();
        assert_eq!(collateral.micro_units(), 6_000_000);
    }

    let parent2 = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_merge(parent2, "dave", "evt-4", 4_000_000, 1))
        .await
        .expect("merge accepted");

    let q = h.seq.q_snapshot().unwrap();
    let collateral = q
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("evt-4".into())))
        .copied()
        .unwrap();
    // 6_000_000 (post-mint) − 4_000_000 (merge debit) = 2_000_000.
    assert_eq!(
        collateral.micro_units(),
        2_000_000,
        "merge must debit conditional_collateral_t[event] by amount"
    );
}

// ── Architect §7.3 verbatim test 5 ──────────────────────────────────────────

/// merge_unavailable_after_final_redeem_if_shares_exhausted — architect §7.3
/// verbatim test name. After resolution + winning-side redeem, the winning
/// side's share inventory is zero; any remaining merge attempt fails because
/// one side (the winning one, fully redeemed) is < amount.
///
/// Per Codex R1 CHALLENGE Q2 remediation 2026-05-09: this test uses a
/// FIXTURE-side `task_markets_t.state = Finalized` flip (via harness reseat)
/// to establish the resolution pre-condition, but the LIVE merge rejection
/// path is fully exercised through `submit_and_apply` → `dispatch_transition`
/// → `TransitionError::InsufficientSharesForMerge`. The fixture covers
/// pre-condition only; live FinalizeRewardTx / TaskBankruptcyTx system-emit
/// witness is provided separately by `tests/tb_8_minimal_payout.rs`
/// (FinalizeReward → state=Finalized) and `tests/tb_11_*.rs` (TaskBankruptcy
/// → state=Bankrupt). Splitting the witness this way keeps the P-M2 atom
/// scope on the merge accept arm rather than re-exercising TB-8/TB-11
/// resolution flows that already have their own gates.
#[tokio::test]
async fn merge_unavailable_after_final_redeem_if_shares_exhausted() {
    let q0 = genesis_with_balances_and_open_task(&[("eve", 10)], "evt-5");
    let mut h = fresh_harness(q0);

    // Mint 5_000_000 (creates 5M YES + 5M NO).
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(parent, "eve", "evt-5", 5_000_000, 1))
        .await
        .expect("mint accepted");

    // FIXTURE-SIDE: flip event to Finalized via harness reseat. The live
    // FinalizeRewardTx system-emit path that produces this state in
    // production is covered by `tests/tb_8_minimal_payout.rs`; here we
    // pre-stage the resolution state to keep the test focused on the
    // merge accept arm. The merge invocation below remains fully live.
    let q_post_mint_then_finalized = {
        let mut q = h.seq.q_snapshot().unwrap();
        let entry = q
            .economic_state_t
            .task_markets_t
            .0
            .get_mut(&TaskId("evt-5".into()))
            .expect("task_markets_t entry");
        entry.state = TaskMarketState::Finalized;
        q
    };
    let mut h2 = fresh_harness(q_post_mint_then_finalized);

    // LIVE: redeem all 5M YES (winning side under Finalized) through the
    // sequencer accept arm.
    let parent2 = h2.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut h2,
        build_redeem(parent2, "eve", "evt-5", OutcomeSide::Yes, 5_000_000, 1),
    )
    .await
    .expect("YES redeem accepted (state==Finalized, YES wins)");

    // LIVE: post-redeem eve has 0 YES, 5M NO. Merge requires both sides;
    // this fails through the sequencer accept arm with
    // TransitionError::InsufficientSharesForMerge.
    let parent3 = h2.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h2,
        build_merge(parent3, "eve", "evt-5", 1_000_000, 1),
    )
    .await
    .expect_err("merge must fail after final redeem exhausts winning side");
    assert!(
        err.contains("InsufficientSharesForMerge"),
        "expected InsufficientSharesForMerge after redeem exhaustion, got: {err}"
    );
}
