//! TB-13 Atom 5 integration tests — CompleteSet + MarketSeedTx per architect
//! 2026-05-03 post-TB-12 ruling Part A §4.4 SG-13.1..8 + halting triggers.
//!
//! "CompleteSet + MarketSeedTx" — Polymarket / CTF conditional-share
//! substrate. **1 locked Coin = 1 YES_E + 1 NO_E.** TB-13 introduces
//! conditional collateral + share balance accounting; redeem requires
//! system-resolved task-market state (Finalized → Yes; Bankrupt → No).
//! TB-13 does NOT introduce trading / pricing / AMM / orderbook —
//! those are deferred to TB-14+.
//!
//! Coverage maps to architect SG-13.0..8 + halting triggers from
//! charter §3 Atom 5 (total_supply_micro mutation correctness / shares
//! NOT counted as Coin / MarketSeed without provider balance / no
//! legacy CPMM / no f64 / no AMM/CPMM router).
//!
//! - SG-13.0.1 legacy_cpm_api_not_imported_by_complete_set       (Atom 0.5 fence)
//! - SG-13.0.2 no_f64_in_complete_set_or_market_seed              (Atom 0.5 fence)
//! - SG-13.0.3 prediction_market_legacy_quarantined               (Atom 0.5 fence)
//! - SG-13.1   mint_one_coin_yields_one_yes_plus_one_no_total_coin_conserved
//! - SG-13.2   yes_no_shares_not_in_total_coin_supply
//! - SG-13.3   market_seed_fails_if_provider_lacks_balance
//! - SG-13.4   market_seed_cannot_create_liquidity_without_collateral
//! - SG-13.5   redeem_unavailable_before_outcome_resolution
//! - SG-13.6   redeem_after_yes_outcome_pays_yes_not_no
//! - SG-13.7   no_f64_in_new_complete_set_or_market_seed_path     (Atom 0.5 fence)
//! - SG-13.8   no_import_or_use_of_legacy_cpmm_in_tb13_modules    (Atom 0.5 fence)
//!
//! /// TRACE_MATRIX TB-13 Atom 5 (architect 2026-05-03 post-TB-12 ruling
//! Part A §4.4 + §4.7 forbidden list; SG-13.0..8).

use std::collections::BTreeMap;
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
    AgentId, ConditionalCollateralIndex, ConditionalShareBalances, QState,
    ShareSidePair, TaskId, TaskMarketEntry, TaskMarketState, TxId,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, CompleteSetMintTx, CompleteSetRedeemTx, EventId,
    MarketSeedTx, OutcomeSide, ResolutionRef, ShareAmount, TypedTx,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ── Harness ─────────────────────────────────────────────────────────────────

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

/// Pre-populate `task_markets_t[task]` with the given state. Used in
/// SG-13.5 / SG-13.6 to simulate a system-emitted resolution (Finalized
/// or Bankrupt) without going through the full FinalizeReward /
/// TaskBankruptcy flow. The state-flip itself is exercised by TB-8 +
/// TB-11 integration tests.
fn seed_task_market(q: &mut QState, task: &str, state: TaskMarketState) {
    let mut entry = TaskMarketEntry::default();
    entry.state = state;
    q.economic_state_t
        .task_markets_t
        .0
        .insert(TaskId(task.into()), entry);
}

/// Build a genesis QState with the given balances AND a single
/// task_markets_t entry in Open state for the given task. Used by the
/// happy-path tests post Gemini Q13 gate (mint/seed reject closed/missing
/// events).
fn genesis_with_balances_and_open_task(
    pairs: &[(&str, i64)],
    task: &str,
) -> QState {
    let mut q = genesis_with_balances(pairs);
    seed_task_market(&mut q, task, TaskMarketState::Open);
    q
}

/// Build a QState that simulates a post-mint snapshot — `mint_owner`
/// has minted `mint_amount_micro` against `task` (event_id) and the
/// task_markets_t state has been flipped to `final_state`. Used by
/// redeem-focused tests so they don't have to drive the actual mint
/// dispatch (which now requires task state == Open per Gemini round-2
/// CHALLENGE Q13 remediation). The post-mint Q-projection is identical
/// to what the dispatch arm would produce.
fn genesis_post_mint(
    pairs: &[(&str, i64)],
    mint_owner: &str,
    task: &str,
    mint_amount_micro: i64,
    final_state: TaskMarketState,
) -> QState {
    let mut q = genesis_with_balances(pairs);
    seed_task_market(&mut q, task, final_state);

    let agent_id = AgentId(mint_owner.into());
    let event_id = EventId(TaskId(task.into()));

    // Debit balance.
    let bal = q
        .economic_state_t
        .balances_t
        .0
        .get(&agent_id)
        .copied()
        .unwrap_or(MicroCoin::zero());
    q.economic_state_t.balances_t.0.insert(
        agent_id.clone(),
        MicroCoin::from_micro_units(bal.micro_units() - mint_amount_micro),
    );

    // Credit collateral.
    q.economic_state_t
        .conditional_collateral_t
        .0
        .insert(event_id.clone(), MicroCoin::from_micro_units(mint_amount_micro));

    // Credit equal YES + NO shares to the owner.
    let mut owner_shares = std::collections::BTreeMap::new();
    owner_shares.insert(
        event_id,
        ShareSidePair {
            yes: ShareAmount::from_units(mint_amount_micro as u128),
            no: ShareAmount::from_units(mint_amount_micro as u128),
        },
    );
    q.economic_state_t
        .conditional_share_balances_t
        .0
        .insert(agent_id, owner_shares);

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

fn build_mint(parent: turingosv4::state::q_state::Hash, owner: &str, task: &str, micro: i64, seq_no: u64) -> TypedTx {
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
        resolution_ref: ResolutionRef {
            resolution_tx_id: TxId(format!("resolution-fixture-{task}")),
            claimed_outcome: outcome,
        },
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 2000 + seq_no,
    })
}

fn build_seed(
    parent: turingosv4::state::q_state::Hash,
    provider: &str,
    task: &str,
    micro: i64,
    seq_no: u64,
) -> TypedTx {
    TypedTx::MarketSeed(MarketSeedTx {
        tx_id: TxId(format!("seed-{provider}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        provider: AgentId(provider.into()),
        collateral_amount: MicroCoin::from_micro_units(micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 3000 + seq_no,
    })
}

// ── SG-13.1 ─────────────────────────────────────────────────────────────────

/// SG-13.1 — Mint 1 Coin → 1 YES + 1 NO, total Coin conserved.
#[tokio::test]
async fn sg_13_1_mint_one_coin_yields_one_yes_plus_one_no_total_coin_conserved() {
    let q0 = genesis_with_balances_and_open_task(&[("alice", 100)], "task-A");
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(&mut h, build_mint(parent, "alice", "task-A", 5_000_000, 1))
        .await
        .expect("mint accepted");

    let q = h.seq.q_snapshot().unwrap();
    let alice_bal = q
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("alice".into()))
        .copied()
        .unwrap();
    assert_eq!(
        alice_bal.micro_units(),
        100_i64 * 1_000_000 - 5_000_000,
        "alice balance must be debited by mint amount"
    );

    let collateral = q
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("task-A".into())))
        .copied()
        .unwrap();
    assert_eq!(collateral.micro_units(), 5_000_000, "collateral credited");

    let pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("alice".into()))
        .and_then(|m| m.get(&EventId(TaskId("task-A".into()))))
        .copied()
        .unwrap();
    assert_eq!(pair.yes.units, 5_000_000_u128, "YES shares minted equal to amount");
    assert_eq!(pair.no.units, 5_000_000_u128, "NO shares minted equal to amount");

    // CTF preserved across mint via 6-holding sum (Atom 3 invariant).
    let q_pre = QState::genesis();
    let mut q_pre_balanced = q_pre.clone();
    q_pre_balanced.economic_state_t.balances_t.0.insert(
        AgentId("alice".into()),
        MicroCoin::from_coin(100).unwrap(),
    );
    assert_total_ctf_conserved(
        &q_pre_balanced.economic_state_t,
        &q.economic_state_t,
        &[],
    )
    .expect("CTF preserved across mint");
    assert_complete_set_balanced(&q.economic_state_t).expect("complete-set balanced post-mint");
}

// ── SG-13.2 ─────────────────────────────────────────────────────────────────

/// SG-13.2 — YES/NO shares are not counted in total Coin supply.
///
/// Asserts that `assert_total_ctf_conserved` passes pre/post a mint that
/// creates 5_000_000 YES + 5_000_000 NO shares — if shares were
/// double-counted as Coin, the post sum would be 10_000_000 micro larger
/// than the pre sum and the assertion would fail.
#[tokio::test]
async fn sg_13_2_yes_no_shares_not_in_total_coin_supply() {
    let q0 = genesis_with_balances_and_open_task(&[("alice", 50)], "task-Z");
    let mut h = fresh_harness(q0.clone());
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(&mut h, build_mint(parent, "alice", "task-Z", 12_345_678, 2))
        .await
        .expect("mint accepted");

    let q = h.seq.q_snapshot().unwrap();
    // Pre/post 6-holding total must be equal — the conditional shares
    // do NOT contribute to total_supply_micro per CR-13.3.
    assert_total_ctf_conserved(&q0.economic_state_t, &q.economic_state_t, &[])
        .expect("shares are not Coin; sum unchanged");
    assert_complete_set_balanced(&q.economic_state_t).expect("balanced");
}

// ── SG-13.3 ─────────────────────────────────────────────────────────────────

/// SG-13.3 — MarketSeedTx fails if provider lacks balance.
#[tokio::test]
async fn sg_13_3_market_seed_fails_if_provider_lacks_balance() {
    // Bob has NO balance row at all. Task-S is pre-Open so Q13 gate
    // (event state) passes; the balance gate is the one that fires.
    let q0 = genesis_with_balances_and_open_task(&[("alice", 100)], "task-S");
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let err = submit_and_apply(&mut h, build_seed(parent, "bob", "task-S", 1_000_000, 3))
        .await
        .expect_err("seed must fail without provider balance");
    assert!(
        err.contains("InsufficientBalanceForMint"),
        "expected InsufficientBalanceForMint, got: {err}"
    );
}

// ── SG-13.4 ─────────────────────────────────────────────────────────────────

/// SG-13.4 — MarketSeedTx cannot create liquidity without collateral
/// (architect §4.7 forbidden list "No automatic liquidity").
#[tokio::test]
async fn sg_13_4_market_seed_cannot_create_liquidity_without_collateral() {
    // Task-X is pre-Open so the Q13 event gate is not the one firing;
    // the zero-collateral gate is.
    let q0 = genesis_with_balances_and_open_task(&[("alice", 100)], "task-X");
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    // collateral_amount == 0 must fail with InsufficientCollateral.
    let err = submit_and_apply(&mut h, build_seed(parent, "alice", "task-X", 0, 4))
        .await
        .expect_err("seed with zero collateral must fail");
    assert!(
        err.contains("InsufficientCollateral"),
        "expected InsufficientCollateral, got: {err}"
    );
}

// ── SG-13.5 ─────────────────────────────────────────────────────────────────

/// SG-13.5 — Redeem unavailable before outcome resolution.
///
/// Mint shares; submit redeem when task_markets_t state is `Open`; expect
/// `RedeemBeforeResolution`. Per architect FR-13.4: "CompleteSetRedeemTx
/// is impossible before system-resolved outcome."
#[tokio::test]
async fn sg_13_5_redeem_unavailable_before_outcome_resolution() {
    // Sub-test 1: mint into Open task succeeds; immediate redeem against
    // the still-Open task is rejected with RedeemBeforeResolution.
    let q0 = genesis_with_balances_and_open_task(&[("alice", 100)], "task-O");
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(&mut h, build_mint(parent, "alice", "task-O", 5_000_000, 5))
        .await
        .expect("mint accepted");
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let err = submit_and_apply(
        &mut h,
        build_redeem(parent, "alice", "task-O", OutcomeSide::Yes, 1_000_000, 6),
    )
    .await
    .expect_err("redeem before resolution must fail");
    assert!(
        err.contains("RedeemBeforeResolution"),
        "expected RedeemBeforeResolution, got: {err}"
    );

    // Sub-test 2: Expired-state task with pre-baked post-mint state
    // (mint was rejected by Q13 gate when state==Expired, so we use
    // genesis_post_mint to construct the state directly). Redeem is
    // rejected with RedeemBeforeResolution.
    let q1 = genesis_post_mint(
        &[("bob", 100)],
        "bob",
        "task-E",
        2_000_000,
        TaskMarketState::Expired,
    );
    let mut h2 = fresh_harness(q1);
    let parent = h2.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h2,
        build_redeem(parent, "bob", "task-E", OutcomeSide::No, 500_000, 8),
    )
    .await
    .expect_err("redeem on expired must fail");
    assert!(
        err.contains("RedeemBeforeResolution"),
        "expected RedeemBeforeResolution on Expired state, got: {err}"
    );
}

// ── SG-13.6 ─────────────────────────────────────────────────────────────────

/// SG-13.6 — Redeem after YES outcome pays YES, not NO.
#[tokio::test]
async fn sg_13_6_redeem_after_yes_outcome_pays_yes_not_no() {
    // Use pre-baked post-mint state with task=Finalized — mint dispatch
    // would now reject Finalized state per Q13 gate, so we build the
    // post-mint state directly to focus the test on redeem semantics.
    let q0 = genesis_post_mint(
        &[("alice", 100)],
        "alice",
        "task-Y",
        4_000_000,
        TaskMarketState::Finalized,
    );
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(
        &mut h,
        build_redeem(parent, "alice", "task-Y", OutcomeSide::Yes, 4_000_000, 10),
    )
    .await
    .expect("redeem yes accepted");

    let q = h.seq.q_snapshot().unwrap();
    let alice_bal = q
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("alice".into()))
        .copied()
        .unwrap();
    // 100 Coin = 100_000_000 micro; -4M (mint) +4M (yes redeem) = 100M unchanged.
    assert_eq!(
        alice_bal.micro_units(),
        100_000_000_i64,
        "alice balance restored after YES redeem"
    );

    let pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("alice".into()))
        .and_then(|m| m.get(&EventId(TaskId("task-Y".into()))))
        .copied()
        .unwrap();
    assert_eq!(pair.yes.units, 0_u128, "YES shares debited (winning side)");
    assert_eq!(pair.no.units, 4_000_000_u128, "NO shares preserved (losing side)");

    // Now attempt redeem outcome=No on the SAME finalized event — must fail
    // because state is Finalized (YES wins) and the claimed_outcome is No.
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h,
        build_redeem(parent, "alice", "task-Y", OutcomeSide::No, 1_000_000, 11),
    )
    .await
    .expect_err("redeem outcome=No on Finalized event must fail");
    assert!(
        err.contains("InvalidResolutionRef"),
        "expected InvalidResolutionRef, got: {err}"
    );

    // Symmetric check: Bankrupt event with outcome=Yes must fail; with
    // outcome=No must succeed. Use pre-baked post-mint state.
    let q_b = genesis_post_mint(
        &[("bob", 50)],
        "bob",
        "task-B",
        1_000_000,
        TaskMarketState::Bankrupt,
    );
    let mut hb = fresh_harness(q_b);
    let parent = hb.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut hb,
        build_redeem(parent, "bob", "task-B", OutcomeSide::Yes, 500_000, 13),
    )
    .await
    .expect_err("Bankrupt with outcome=Yes must reject");
    assert!(err.contains("InvalidResolutionRef"));
    let parent = hb.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut hb,
        build_redeem(parent, "bob", "task-B", OutcomeSide::No, 500_000, 14),
    )
    .await
    .expect("Bankrupt with outcome=No must succeed");
}

// ── SG-13.7 / SG-13.8 — fence delegation ────────────────────────────────────
//
// SG-13.7 (no f64 in CompleteSet/MarketSeed path) and SG-13.8 (no
// import/use of legacy CPMM in TB-13 modules) are enforced by Atom 0.5
// forward-fence in `tests/tb_13_legacy_cpmm_forward_fence.rs`. This file
// records the delegation contract.

/// SG-13.7 (delegation marker) — `no_f64_in_new_complete_set_or_market_seed_path`
/// is enforced by `tests/tb_13_legacy_cpmm_forward_fence.rs::no_f64_in_complete_set_or_market_seed`.
/// This test exists to make the architect SG-13.7 ship gate visible in
/// the TB-13 integration test surface (per `feedback_workspace_test_canonical`
/// + ship gate exact-name discipline).
#[test]
fn sg_13_7_no_f64_in_new_complete_set_or_market_seed_path() {
    // Delegation: see tests/tb_13_legacy_cpmm_forward_fence.rs.
    // This test passes by construction: any f64 leak would be caught by
    // the forward-fence test at workspace time. We assert the delegation
    // is in place by checking the fence file exists and contains the
    // expected SG-13.0.2 test name.
    let fence_src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/tb_13_legacy_cpmm_forward_fence.rs"),
    )
    .expect("fence test file must exist");
    assert!(
        fence_src.contains("fn no_f64_in_complete_set_or_market_seed"),
        "SG-13.0.2 fence delegation broken: missing target test name"
    );
}

/// SG-13.8 (delegation marker) — `no_import_or_use_of_legacy_cpmm_in_tb13_modules`
/// is enforced by `tests/tb_13_legacy_cpmm_forward_fence.rs::legacy_cpm_api_not_imported_by_complete_set`.
#[test]
fn sg_13_8_no_import_or_use_of_legacy_cpmm_in_tb13_modules() {
    let fence_src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/tb_13_legacy_cpmm_forward_fence.rs"),
    )
    .expect("fence test file must exist");
    assert!(
        fence_src.contains("fn legacy_cpm_api_not_imported_by_complete_set"),
        "SG-13.0.1 fence delegation broken: missing target test name"
    );
}

// ── Halting-trigger guards (architect §4.8) ─────────────────────────────────

/// Halt: total_supply_micro must be unchanged across mint+redeem.
#[tokio::test]
async fn halt_total_supply_micro_unchanged_across_mint_redeem() {
    // Use pre-baked post-mint state with task=Finalized; the post-mint
    // QState reflects the would-be result of a successful mint into an
    // Open task. Compare against the genesis (pre-mint balance) to
    // verify total supply bit-equality. After redeem, alice's balance
    // is restored.
    let q_genesis = genesis_with_balances(&[("alice", 100)]);
    let q0 = genesis_post_mint(
        &[("alice", 100)],
        "alice",
        "task-H1",
        7_000_000,
        TaskMarketState::Finalized,
    );
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut h,
        build_redeem(parent, "alice", "task-H1", OutcomeSide::Yes, 7_000_000, 21),
    )
    .await
    .expect("redeem");

    let q = h.seq.q_snapshot().unwrap();
    // q_genesis vs q (post-redeem) — both should have alice at 100 Coin
    // (genesis 100; mint -7M; redeem +7M = 100). The pre-baked post-
    // mint state had collateral, but post-redeem collateral is 0.
    // Comparison must add task_markets_t state in pre as Finalized too,
    // or we use a different baseline. Simpler: check 6-holding sum is
    // the same as q_genesis.
    let genesis_with_finalized_market = {
        let mut g = q_genesis.clone();
        seed_task_market(&mut g, "task-H1", TaskMarketState::Finalized);
        g
    };
    assert_total_ctf_conserved(
        &genesis_with_finalized_market.economic_state_t,
        &q.economic_state_t,
        &[],
    )
    .expect("total_supply_micro bit-equal pre/post (mint→redeem cancels out)");
    assert_complete_set_balanced(&q.economic_state_t).expect("balanced");
}

/// Halt: shares are NOT counted as Coin (regression guard for SG-13.2).
#[tokio::test]
async fn halt_shares_not_counted_as_coin() {
    let q0 = genesis_with_balances_and_open_task(&[("alice", 100)], "task-H2");
    let mut h = fresh_harness(q0.clone());
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(parent, "alice", "task-H2", 9_876_543, 22))
        .await
        .expect("mint");
    let q = h.seq.q_snapshot().unwrap();
    assert_total_ctf_conserved(&q0.economic_state_t, &q.economic_state_t, &[])
        .expect("shares not in total_supply");
}

/// Halt: MarketSeed with zero-balance provider rejected (regression
/// guard for SG-13.3).
#[tokio::test]
async fn halt_market_seed_zero_balance_provider_rejected() {
    let mut q0 = QState::genesis();
    seed_task_market(&mut q0, "task-H3", TaskMarketState::Open);
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(&mut h, build_seed(parent, "ghost", "task-H3", 1_000, 23))
        .await
        .expect_err("seed must fail");
    assert!(err.contains("InsufficientBalanceForMint"));
}

/// Architect-mandated invariant: redeeming more shares than owned is
/// rejected with `RedeemMoreThanOwned`.
#[tokio::test]
async fn halt_redeem_more_than_owned_rejected() {
    let q0 = genesis_post_mint(
        &[("alice", 100)],
        "alice",
        "task-H4",
        1_000_000,
        TaskMarketState::Finalized,
    );
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h,
        build_redeem(parent, "alice", "task-H4", OutcomeSide::Yes, 5_000_000, 25),
    )
    .await
    .expect_err("over-redeem must fail");
    assert!(
        err.contains("RedeemMoreThanOwned"),
        "expected RedeemMoreThanOwned, got: {err}"
    );
}

/// Codex round-1 VETO TB13-V1 remediation: negative `MicroCoin` amount
/// in CompleteSetMintTx must be rejected. `MicroCoin` is i64-backed and
/// permits negative values at the type layer; the dispatch arm gates
/// `<= 0` (not just `== 0`). Without this gate, a negative mint would
/// credit balance + write negative collateral + cast to huge u128 shares.
#[tokio::test]
async fn halt_negative_mint_amount_rejected() {
    // Pre-seed task as Open so the Q13 gate isn't the one firing — we
    // want to verify the negative-amount check (Step 2) catches first.
    let q0 = genesis_with_balances_and_open_task(&[("alice", 100)], "task-NEG");
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let neg_mint = TypedTx::CompleteSetMint(CompleteSetMintTx {
        tx_id: TxId("neg-mint-fixture".into()),
        parent_state_root: parent,
        event_id: EventId(TaskId("task-NEG".into())),
        owner: AgentId("alice".into()),
        amount: MicroCoin::from_micro_units(-1_000_000),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 999,
    });
    let err = submit_and_apply(&mut h, neg_mint)
        .await
        .expect_err("negative mint must be rejected");
    assert!(
        err.contains("InsufficientBalanceForMint"),
        "expected InsufficientBalanceForMint for negative amount, got: {err}"
    );

    // Verify alice balance unchanged.
    let q = h.seq.q_snapshot().unwrap();
    let alice_bal = q
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("alice".into()))
        .copied()
        .unwrap();
    assert_eq!(
        alice_bal.micro_units(),
        100_i64 * 1_000_000,
        "alice balance MUST be unchanged after negative-mint rejection"
    );
    // Verify no collateral written.
    assert!(
        q.economic_state_t
            .conditional_collateral_t
            .0
            .get(&EventId(TaskId("task-NEG".into())))
            .is_none(),
        "no collateral must be written under negative-mint rejection"
    );
}

/// Codex round-1 VETO TB13-V1 remediation: negative `MicroCoin`
/// collateral_amount in MarketSeedTx must be rejected with
/// `InsufficientCollateral`. Same attack vector as halt_negative_mint
/// but via the seed path.
#[tokio::test]
async fn halt_negative_market_seed_collateral_rejected() {
    // Pre-seed task as Open so the Q13 gate isn't the one firing — we
    // want to verify the negative-collateral check (Step 1) catches first.
    let q0 = genesis_with_balances_and_open_task(&[("provider", 50)], "task-NEGS");
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let neg_seed = TypedTx::MarketSeed(MarketSeedTx {
        tx_id: TxId("neg-seed-fixture".into()),
        parent_state_root: parent,
        event_id: EventId(TaskId("task-NEGS".into())),
        provider: AgentId("provider".into()),
        collateral_amount: MicroCoin::from_micro_units(-500_000),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 998,
    });
    let err = submit_and_apply(&mut h, neg_seed)
        .await
        .expect_err("negative-collateral seed must be rejected");
    assert!(
        err.contains("InsufficientCollateral"),
        "expected InsufficientCollateral for negative collateral, got: {err}"
    );

    let q = h.seq.q_snapshot().unwrap();
    let provider_bal = q
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("provider".into()))
        .copied()
        .unwrap();
    assert_eq!(
        provider_bal.micro_units(),
        50_i64 * 1_000_000,
        "provider balance MUST be unchanged after negative-seed rejection"
    );
}

/// Gemini round-2 CHALLENGE Q13 remediation: CompleteSetMintTx against
/// a Finalized event must reject with `EventNotOpen` (closes the
/// post-resolution griefing surface where an agent could mint shares
/// against a closed event and immediately redeem winning side).
#[tokio::test]
async fn halt_q13_mint_against_finalized_event_rejected() {
    let mut q0 = genesis_with_balances(&[("alice", 100)]);
    seed_task_market(&mut q0, "task-Q13F", TaskMarketState::Finalized);
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let err = submit_and_apply(
        &mut h,
        build_mint(parent, "alice", "task-Q13F", 1_000_000, 100),
    )
    .await
    .expect_err("mint against Finalized event must be rejected");
    assert!(
        err.contains("EventNotOpen"),
        "expected EventNotOpen, got: {err}"
    );
}

/// Gemini round-2 CHALLENGE Q13 remediation: same gate against
/// MarketSeedTx — seed against Bankrupt event must reject with
/// `EventNotOpen`.
#[tokio::test]
async fn halt_q13_seed_against_bankrupt_event_rejected() {
    let mut q0 = genesis_with_balances(&[("provider", 100)]);
    seed_task_market(&mut q0, "task-Q13B", TaskMarketState::Bankrupt);
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let err = submit_and_apply(
        &mut h,
        build_seed(parent, "provider", "task-Q13B", 500_000, 101),
    )
    .await
    .expect_err("seed against Bankrupt event must be rejected");
    assert!(
        err.contains("EventNotOpen"),
        "expected EventNotOpen, got: {err}"
    );
}

/// Q13 gate: mint against an event with NO task_markets_t entry must
/// reject with `TaskNotOpen` (the missing-event case; EventId is 1:1
/// with TaskId in TB-13 so a task must exist for the event to be valid).
#[tokio::test]
async fn halt_q13_mint_against_missing_task_rejected() {
    let q0 = genesis_with_balances(&[("alice", 100)]);
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let err = submit_and_apply(
        &mut h,
        build_mint(parent, "alice", "task-NOEXIST", 1_000_000, 102),
    )
    .await
    .expect_err("mint against missing task must be rejected");
    assert!(
        err.contains("TaskNotOpen"),
        "expected TaskNotOpen, got: {err}"
    );
}

/// Architect-mandated invariant: complete-set balanced post-seed.
#[tokio::test]
async fn halt_complete_set_balanced_post_seed() {
    let q0 = genesis_with_balances_and_open_task(&[("provider", 50)], "task-H5");
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_seed(parent, "provider", "task-H5", 3_141_592, 26))
        .await
        .expect("seed");
    let q = h.seq.q_snapshot().unwrap();
    assert_complete_set_balanced(&q.economic_state_t).expect("balanced after seed");
    let collateral = q
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("task-H5".into())))
        .copied()
        .unwrap();
    assert_eq!(collateral.micro_units(), 3_141_592);
    let pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("provider".into()))
        .and_then(|m| m.get(&EventId(TaskId("task-H5".into()))))
        .copied()
        .unwrap();
    assert_eq!(pair.yes.units, 3_141_592_u128);
    assert_eq!(pair.no.units, 3_141_592_u128);
}

// Suppress unused import warnings — the harness types are referenced via
// trait constraints + the helper signatures.
#[allow(dead_code)]
fn _suppress_unused() {
    let _ = ConditionalCollateralIndex::default();
    let _ = ConditionalShareBalances::default();
    let _: BTreeMap<EventId, MicroCoin> = BTreeMap::new();
    let _ = ShareSidePair::default();
}

// ── TB13-AUTH (Codex VETO round-2 remediation) ──────────────────────────────

/// Codex round-2 VETO TB13-AUTH remediation: when `agent_pubkeys` is set on
/// the sequencer, TB-13 submissions with valid signatures are accepted and
/// submissions with forged (all-zero) signatures are rejected at
/// `submit_agent_tx` time with `SubmitError::AgentSignatureInvalid`.
///
/// Builds a real Ed25519 keypair, registers the agent in an
/// `AgentPubkeyManifest`, signs the canonical mint payload with the
/// keypair, and proves both the accept-on-valid and reject-on-forged paths.
#[tokio::test]
async fn tb13_auth_submit_time_signature_verification() {
    use std::sync::Arc;
    use turingosv4::runtime::agent_keypairs::{
        AgentKeypair, AgentPubkeyManifest,
    };
    use turingosv4::state::sequencer::SubmitError;

    let q0 = genesis_with_balances_and_open_task(&[("alice", 100)], "task-AUTH");
    let mut h = fresh_harness(q0);

    // Build a real keypair for "alice" + register in the manifest.
    let alice_keypair = AgentKeypair::generate().expect("generate alice keypair");
    let mut manifest = AgentPubkeyManifest::default();
    manifest.agents.insert(
        "alice".to_string(),
        alice_keypair.public_key().to_hex(),
    );
    h.seq
        .set_agent_pubkeys(Arc::new(manifest))
        .expect("set_agent_pubkeys must succeed once");

    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    // Path A — valid signature: build canonical signing payload digest,
    // sign with alice's keypair, attach as the mint's signature.
    let mint_unsigned = CompleteSetMintTx {
        tx_id: TxId("auth-mint-fixture".into()),
        parent_state_root: parent,
        event_id: EventId(TaskId("task-AUTH".into())),
        owner: AgentId("alice".into()),
        amount: MicroCoin::from_micro_units(1_000_000),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 500,
    };
    let signing_digest = mint_unsigned.to_signing_payload().canonical_digest();
    let valid_sig = alice_keypair
        .sign_digest(signing_digest)
        .expect("sign_digest");
    let mint_signed = CompleteSetMintTx {
        signature: valid_sig,
        ..mint_unsigned.clone()
    };
    submit_and_apply(&mut h, TypedTx::CompleteSetMint(mint_signed))
        .await
        .expect("valid-sig mint must be accepted");

    // Path B — forged signature: all-zero AgentSignature, valid payload.
    // Must be rejected at submit_agent_tx with AgentSignatureInvalid.
    let parent_b = h.seq.q_snapshot().unwrap().state_root_t;
    let forged_mint = CompleteSetMintTx {
        tx_id: TxId("auth-forged-fixture".into()),
        parent_state_root: parent_b,
        event_id: EventId(TaskId("task-AUTH".into())),
        owner: AgentId("alice".into()),
        amount: MicroCoin::from_micro_units(500_000),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 501,
    };
    let submit_err = h
        .seq
        .submit_agent_tx(TypedTx::CompleteSetMint(forged_mint))
        .await
        .expect_err("forged-sig mint must be rejected at submit");
    assert!(
        matches!(submit_err, SubmitError::AgentSignatureInvalid),
        "expected SubmitError::AgentSignatureInvalid, got: {submit_err:?}"
    );

    // Path C — unregistered agent: build a different keypair, sign with it
    // for "alice" (impostor); pubkey lookup matches alice's registered
    // key, signature verification fails. Must be rejected at submit.
    let impostor_keypair = AgentKeypair::generate().expect("generate impostor keypair");
    let mint_impostor_unsigned = CompleteSetMintTx {
        tx_id: TxId("auth-impostor-fixture".into()),
        parent_state_root: parent_b,
        event_id: EventId(TaskId("task-AUTH".into())),
        owner: AgentId("alice".into()),
        amount: MicroCoin::from_micro_units(500_000),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 502,
    };
    let imp_sig = impostor_keypair
        .sign_digest(mint_impostor_unsigned.to_signing_payload().canonical_digest())
        .expect("sign_digest");
    let impostor_mint = CompleteSetMintTx {
        signature: imp_sig,
        ..mint_impostor_unsigned
    };
    let submit_err = h
        .seq
        .submit_agent_tx(TypedTx::CompleteSetMint(impostor_mint))
        .await
        .expect_err("impostor-sig mint must be rejected at submit");
    assert!(
        matches!(submit_err, SubmitError::AgentSignatureInvalid),
        "expected SubmitError::AgentSignatureInvalid for impostor, got: {submit_err:?}"
    );
}
