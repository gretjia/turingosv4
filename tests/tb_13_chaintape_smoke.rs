//! TB-13 Atom 6 round-5 — Codex RQ3 remediation: non-empty TB-13 chaintape
//! replay smoke.
//!
//! ## Why this test exists
//!
//! Codex round-3 RQ3 found that the existing real-LLM regression smoke at
//! `handover/evidence/tb_13_real_llm_smoke_2026-05-03/` proves that
//! `EconomicState`'s 13-sub-field schema round-trips with **EMPTY** TB-13
//! maps — the LLM-driven solver path doesn't submit `CompleteSetMint` /
//! `CompleteSetRedeem` / `MarketSeed` (those are user-economic actions,
//! not solver actions). So the smoke's chaintape contains zero TB-13
//! entries, and `verify_chaintape`'s `economic_state_reconstructed: true`
//! indicator only proves the schema-shape round-trip with empty maps.
//!
//! This deterministic non-LLM smoke closes that gap by:
//!
//! 1. Bootstrapping a chain-backed sequencer with `initial_q` containing
//!    pre-seeded balances (alice = 100 Coin), an open task `task-MINT`
//!    (so the Q13 mint gate passes), a finalized task `task-REDEEM` with
//!    pre-seeded YES/NO shares + collateral (so the redeem gate passes).
//! 2. Wiring a real `AgentKeypair` via `AgentKeypairRegistry` (writes
//!    `agent_pubkeys.json` to runtime_repo_path) + `set_agent_pubkeys`
//!    on the sequencer (closes submit-time Class 3 admission control).
//! 3. Submitting a real signed `CompleteSetMintTx` against `task-MINT`
//!    + a real signed `CompleteSetRedeemTx` against `task-REDEEM`. Both
//!    flow through `submit_agent_tx` → driver → Git2LedgerWriter persist.
//! 4. Shutting down the bundle (drains queue) + holding a clone of
//!    `Arc<Sequencer>` to read the post-drain live `q_snapshot()`.
//! 5. Asserting that pre-shutdown live `conditional_collateral_t` and
//!    `conditional_share_balances_t` are NON-EMPTY (sanity).
//! 6. Running `verify_chaintape` on the persisted runtime_repo + cas →
//!    asserting all 7 indicators GREEN, l4_entries ≥ 2, and the
//!    replay-reconstructed `final_state_root_hex` matches the live
//!    `state_root_t`. Codex round-4 RQ3 follow-up (2026-05-03): the
//!    state-root mutator hashes `domain || prev_root || canonical_tx`,
//!    NOT the full QState. So state-root equality on its own proves
//!    deterministic tx-chain replay (same initial_q + same canonical-
//!    encoded txs in the same order + the same pure dispatcher → same
//!    root); it does NOT directly assert byte-equal QState
//!    reconstruction.
//! 7. Running `replay_full_transition` (pub API) manually against the
//!    persisted runtime_repo + cas + pinned_pubkeys + initial_q, and
//!    asserting BYTE-EQUAL map reconstruction of
//!    `conditional_collateral_t` and `conditional_share_balances_t`
//!    against the live state. This is direct map-equality evidence —
//!    no inference from dispatch-determinism required. Closes Codex
//!    round-4 RQ3.
//!
//! ## What this proves
//!
//! - Non-empty `conditional_collateral_t` reconstructs byte-equal under
//!   replay (direct map-equality assertion, step 7).
//! - Non-empty `conditional_share_balances_t` reconstructs byte-equal
//!   under replay (direct map-equality assertion, step 7).
//! - State-root chain match across replay (verify_chaintape, step 6).
//! - Submit-time + replay-time agent signature verification (Gate 4
//!   covers all 3 TB-13 typed-tx variants).
//! - Two-tx state-root chain (initial → mint → redeem) replays
//!   deterministically end-to-end.
//!
//! TRACE_MATRIX TB-13 Atom 6 round-6 (Codex round-4 RQ3 remediation
//! 2026-05-03; FC3-N1 chaintape replay determinism + direct map-equality
//! reconstruction for non-empty TB-13 maps).

use std::collections::BTreeMap;
use std::sync::Arc;

use tempfile::TempDir;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::system_keypair::{
    PinnedSystemPubkeys, SystemEpoch, SystemPublicKey,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    replay_full_transition, Git2LedgerWriter, LedgerEntry, LedgerWriter,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::verify::{verify_chaintape, VerifyOptions};
use turingosv4::runtime::{
    build_chaintape_sequencer_with_initial_q, PinnedPubkeyManifest,
    RuntimeChaintapeConfig,
};
use turingosv4::state::q_state::{
    AgentId, QState, ShareSidePair, TaskId, TaskMarketEntry, TaskMarketState, TxId,
};
use turingosv4::state::sequencer::complete_set_mint_accept_state_root;
use turingosv4::state::typed_tx::{
    AgentSignature, CompleteSetMintTx, CompleteSetRedeemTx, EventId, OutcomeSide,
    ShareAmount, TypedTx,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

fn build_smoke_initial_q(
    alice: &str,
    mint_task: &str,
    redeem_task: &str,
    redeem_units: i64,
) -> QState {
    let mut q = QState::genesis();
    let alice_id = AgentId(alice.into());

    q.economic_state_t
        .balances_t
        .0
        .insert(alice_id.clone(), MicroCoin::from_coin(100).unwrap());

    let mut mint_entry = TaskMarketEntry::default();
    mint_entry.state = TaskMarketState::Open;
    q.economic_state_t
        .task_markets_t
        .0
        .insert(TaskId(mint_task.into()), mint_entry);

    let mut redeem_entry = TaskMarketEntry::default();
    redeem_entry.state = TaskMarketState::Finalized;
    q.economic_state_t
        .task_markets_t
        .0
        .insert(TaskId(redeem_task.into()), redeem_entry);

    // Pre-seed the redeem-task collateral + alice's YES/NO shares so the
    // redeem gate passes. The MIN-balanced invariant holds at
    // min(redeem_units, redeem_units) == collateral.
    let redeem_event = EventId(TaskId(redeem_task.into()));
    q.economic_state_t.conditional_collateral_t.0.insert(
        redeem_event.clone(),
        MicroCoin::from_micro_units(redeem_units),
    );
    let mut alice_shares: BTreeMap<EventId, ShareSidePair> = BTreeMap::new();
    alice_shares.insert(
        redeem_event,
        ShareSidePair {
            yes: ShareAmount::from_units(redeem_units as u128),
            no: ShareAmount::from_units(redeem_units as u128),
        },
    );
    q.economic_state_t
        .conditional_share_balances_t
        .0
        .insert(alice_id, alice_shares);

    q
}

/// Re-runs `replay_full_transition` against the persisted runtime_repo +
/// cas, returning the reconstructed `QState`. Mirrors the steps that
/// `verify_chaintape` performs internally (verify.rs:225..308) but
/// returns the QState so the caller can assert directly on the
/// reconstructed `economic_state_t` sub-fields. Codex round-4 RQ3
/// remediation 2026-05-03: closes the "state-root equality is cryptographic
/// proof of map equality" overclaim by providing direct map equality
/// evidence instead of an inference from dispatch determinism.
fn manual_replay_from_disk(
    runtime_repo_path: &std::path::Path,
    cas_path: &std::path::Path,
) -> QState {
    // Load initial_q from disk (replay starts from the same state as live).
    let initial_q_path = runtime_repo_path.join("initial_q_state.json");
    let initial_q_json =
        std::fs::read_to_string(&initial_q_path).expect("read initial_q_state.json");
    let initial_q: QState =
        serde_json::from_str(&initial_q_json).expect("parse initial_q_state.json");

    // Read all L4 entries from the persisted Git ledger.
    let writer = Git2LedgerWriter::open(runtime_repo_path).expect("open Git2LedgerWriter");
    let n = writer.len();
    let entries: Vec<LedgerEntry> = (1..=n)
        .map(|t| writer.read_at(t).expect("read_at"))
        .collect();

    // Load pinned-pubkey manifest from disk + decode into PinnedSystemPubkeys.
    let manifest_path = runtime_repo_path.join("pinned_pubkeys.json");
    let manifest_json =
        std::fs::read_to_string(&manifest_path).expect("read pinned_pubkeys.json");
    let manifest: PinnedPubkeyManifest =
        serde_json::from_str(&manifest_json).expect("parse pinned_pubkeys.json");
    let mut pinned = PinnedSystemPubkeys::new();
    for entry in &manifest.pubkeys {
        let bytes: Vec<u8> = (0..entry.pubkey_hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&entry.pubkey_hex[i..i + 2], 16).expect("hex"))
            .collect();
        let arr: [u8; 32] = bytes.as_slice().try_into().expect("32-byte pubkey");
        pinned.insert(SystemEpoch::new(entry.epoch), SystemPublicKey::from_bytes(arr));
    }

    let cas = CasStore::open(cas_path).expect("open cas");
    let predicates = PredicateRegistry::new();
    let tools = ToolRegistry::new();

    replay_full_transition(&initial_q, &entries, &cas, &pinned, &predicates, &tools)
        .expect("replay_full_transition")
}

#[tokio::test]
async fn rq3_non_empty_tb13_chaintape_replays_with_state_root_match() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = RuntimeChaintapeConfig {
        runtime_repo_path: tmp.path().join("runtime_repo"),
        cas_path: tmp.path().join("cas"),
        run_id: "rq3-tb13-smoke".to_string(),
        queue_capacity: 16,
        resume_existing_chain: false,
    };

    let alice = "alice";
    let alice_id = AgentId(alice.into());
    let mint_task = "task-rq3-mint";
    let redeem_task = "task-rq3-redeem";
    let mint_amount_micro: i64 = 2_000_000;
    let redeem_units: i64 = 4_000_000;

    let initial_q =
        build_smoke_initial_q(alice, mint_task, redeem_task, redeem_units);
    let bundle = build_chaintape_sequencer_with_initial_q(&cfg, initial_q)
        .expect("bootstrap chaintape sequencer");

    // Register alice in an AgentKeypairRegistry rooted at runtime_repo —
    // this writes <runtime_repo>/agent_pubkeys.json which verify_chaintape
    // Gate 4 reads on replay.
    let mut reg = AgentKeypairRegistry::open(&cfg.runtime_repo_path)
        .expect("open agent keypair registry");
    reg.get_or_create(&alice_id)
        .expect("generate alice keypair");
    bundle
        .sequencer
        .set_agent_pubkeys(Arc::new(reg.manifest()))
        .expect("set_agent_pubkeys must succeed once");

    let initial_root = bundle
        .sequencer
        .q_snapshot()
        .expect("initial q_snapshot")
        .state_root_t;

    // ── Build + sign mint tx (parent = initial_root) ────────────────────────
    let mint_unsigned = CompleteSetMintTx {
        tx_id: TxId("rq3-mint-1".into()),
        parent_state_root: initial_root,
        event_id: EventId(TaskId(mint_task.into())),
        owner: alice_id.clone(),
        amount: MicroCoin::from_micro_units(mint_amount_micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 100,
    };
    let mint_digest = mint_unsigned
        .to_signing_payload()
        .canonical_digest();
    let mint_sig = reg.sign(&alice_id, mint_digest).expect("sign mint");
    let mint_tx = TypedTx::CompleteSetMint(CompleteSetMintTx {
        signature: mint_sig,
        ..mint_unsigned
    });

    // ── Pre-compute the post-mint state_root + build redeem at that parent ──
    //
    // Because the canonical state-root mutator is pure-deterministic in the
    // tx fields, we can pre-compute the parent_state_root the redeem must
    // carry without racing the driver. The dispatcher will compute the same
    // hash when applying the mint; the redeem's parent_state_root then
    // matches q.state_root_t at apply-time (no StaleParent rejection).
    let after_mint_root =
        complete_set_mint_accept_state_root(&initial_root, &mint_tx);

    let redeem_unsigned = CompleteSetRedeemTx {
        tx_id: TxId("rq3-redeem-1".into()),
        parent_state_root: after_mint_root,
        event_id: EventId(TaskId(redeem_task.into())),
        owner: alice_id.clone(),
        outcome: OutcomeSide::Yes,
        share_amount: ShareAmount::from_units(redeem_units as u128),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 101,
    };
    let redeem_digest = redeem_unsigned
        .to_signing_payload()
        .canonical_digest();
    let redeem_sig = reg.sign(&alice_id, redeem_digest).expect("sign redeem");
    let redeem_tx = TypedTx::CompleteSetRedeem(CompleteSetRedeemTx {
        signature: redeem_sig,
        ..redeem_unsigned
    });

    // ── Submit both; rely on driver+shutdown drain to apply in FIFO order ──
    bundle
        .sequencer
        .submit_agent_tx(mint_tx)
        .await
        .expect("submit mint");
    bundle
        .sequencer
        .submit_agent_tx(redeem_tx)
        .await
        .expect("submit redeem");

    // Hold a clone of Arc<Sequencer> across shutdown so we can read live
    // post-drain state. ChaintapeBundle::shutdown consumes self; the
    // Arc keeps the Sequencer alive for our q_snapshot read below.
    let seq_handle = bundle.sequencer.clone();
    bundle.shutdown().await.expect("shutdown drain");

    let live_q = seq_handle
        .q_snapshot()
        .expect("post-drain q_snapshot");
    let live_state_root = live_q.state_root_t;

    // Sanity — non-empty TB-13 maps. mint_task added a new collateral entry
    // (size 2: pre-seeded redeem + new mint); alice has shares for both
    // events post-redeem (yes side debited on redeem, no side preserved).
    let collateral_count = live_q.economic_state_t.conditional_collateral_t.0.len();
    let share_owner_count = live_q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .len();
    assert!(
        collateral_count >= 2,
        "expected ≥2 conditional_collateral_t entries (pre-seeded redeem task + mint task); got {collateral_count}"
    );
    assert!(
        share_owner_count >= 1,
        "expected alice in conditional_share_balances_t; got {share_owner_count} owner entries"
    );

    // Confirm both txs landed by chain-fold position.
    let alice_balance_post = live_q
        .economic_state_t
        .balances_t
        .0
        .get(&alice_id)
        .copied()
        .unwrap()
        .micro_units();
    // Pre-test: 100 Coin = 100_000_000 micro.
    // Post-mint: -2_000_000 (debited for mint).
    // Post-redeem: +4_000_000 (credited for YES redeem).
    // Net: 100_000_000 - 2_000_000 + 4_000_000 = 102_000_000.
    assert_eq!(
        alice_balance_post, 102_000_000,
        "alice balance after mint+redeem must be 100M - 2M + 4M = 102M micro"
    );

    // ── Replay verification ─────────────────────────────────────────────────
    let report = verify_chaintape(
        &cfg.runtime_repo_path,
        &cfg.cas_path,
        &VerifyOptions::default(),
    )
    .expect("verify_chaintape");

    assert!(
        report.l4_entries >= 2,
        "expected ≥2 L4 entries (mint + redeem); got {}",
        report.l4_entries
    );
    assert!(
        report.all_indicators_pass(),
        "all 7 indicators must pass; report = {report:?}"
    );
    assert!(
        report.detail.initial_q_state_loaded_from_disk,
        "initial_q_state.json must be loaded from disk for replay determinism"
    );

    // ── RQ3 check 1: state-root chain matches live ─────────────────────────
    //
    // Codex round-4 follow-up (2026-05-03): state-root equality alone proves
    // deterministic tx-chain replay (same initial_q + same canonical-encoded
    // tx sequence + same pure dispatcher → same root) — it does NOT directly
    // hash the full QState (the mutator hashes `domain || prev_root ||
    // canonical_tx`). So we record the chain-replay match here and follow
    // up with a direct map-equality check below.
    let live_state_root_hex: String = live_state_root
        .0
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let final_state_root_hex = report
        .detail
        .final_state_root_hex
        .as_ref()
        .expect("final_state_root_hex present after non-empty replay");
    assert_eq!(
        &live_state_root_hex, final_state_root_hex,
        "RQ3: replay state_root must match live state_root → confirms tx-chain replays deterministically"
    );

    // ── RQ3 check 2: direct map-equality after manual re-replay ─────────────
    //
    // Codex round-4 RQ3 remediation 2026-05-03: re-run `replay_full_transition`
    // (pub API) against the persisted artifacts and assert byte-equal
    // reconstruction of the TB-13 sub-fields. This closes the "state-root
    // overclaim" gap by proving map equality directly, not by relying on
    // dispatch-determinism implication.
    let replayed_q = manual_replay_from_disk(&cfg.runtime_repo_path, &cfg.cas_path);

    assert_eq!(
        replayed_q.state_root_t,
        live_state_root,
        "manual replay state_root must equal live state_root (sanity)"
    );
    assert_eq!(
        replayed_q.economic_state_t.conditional_collateral_t,
        live_q.economic_state_t.conditional_collateral_t,
        "RQ3 direct check: replayed conditional_collateral_t must equal live (byte-equal map reconstruction)"
    );
    assert_eq!(
        replayed_q.economic_state_t.conditional_share_balances_t,
        live_q.economic_state_t.conditional_share_balances_t,
        "RQ3 direct check: replayed conditional_share_balances_t must equal live (byte-equal map reconstruction)"
    );
    // Belt-and-suspenders: full economic_state_t equality.
    assert_eq!(
        replayed_q.economic_state_t, live_q.economic_state_t,
        "RQ3 direct check: full replayed economic_state_t must equal live"
    );

    // ── Persist evidence to canonical handover dir (best-effort) ────────────
    //
    // Mirrors TB-7 chain-backed smoke pattern. If the dir is unwritable
    // (CI sandbox), the on-disk witness under TempDir is still authoritative
    // for the test's correctness assertions — evidence dump is forensic.
    //
    // 2026-05-07 evidence-immutability fix: gated behind
    // TURINGOS_TEST_REGENERATE_EVIDENCE=1. See
    // OBS_EVIDENCE_DRIFT_ROOT_CAUSE_2026-05-07.md.
    let evidence_dir = std::path::Path::new(
        "handover/evidence/tb_13_chaintape_smoke_2026-05-03",
    );
    let regen_enabled = std::env::var("TURINGOS_TEST_REGENERATE_EVIDENCE")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if regen_enabled && std::fs::create_dir_all(evidence_dir).is_ok() {
        let report_json =
            serde_json::to_string_pretty(&report).expect("serialize report");
        let _ = std::fs::write(
            evidence_dir.join("replay_report.json"),
            report_json,
        );
        let agent_pubkeys_src = cfg.runtime_repo_path.join("agent_pubkeys.json");
        if agent_pubkeys_src.exists() {
            let _ = std::fs::copy(
                &agent_pubkeys_src,
                evidence_dir.join("agent_pubkeys.json"),
            );
        }
        let _ = std::fs::write(
            evidence_dir.join("README.md"),
            format!(
                "# TB-13 Atom 6 round-6 — non-empty TB-13 chaintape replay smoke\n\
                 \n\
                 **Date**: 2026-05-03\n\
                 **Source**: `tests/tb_13_chaintape_smoke.rs::rq3_non_empty_tb13_chaintape_replays_with_state_root_match`\n\
                 **Trigger**: Codex round-3 RQ3 finding (the existing real-LLM smoke proves the EconomicState 13-sub-field schema round-trips with EMPTY TB-13 maps; non-empty replay determinism was not directly evidenced) + Codex round-4 RQ3 follow-up (the round-5 closure overclaimed state-root equality as cryptographic proof of map equality — fixed in round-6 by adding a direct map-equality assertion via manual `replay_full_transition` re-replay).\n\
                 \n\
                 ## Headline\n\
                 \n\
                 - L4 entries: {l4} (mint + redeem)\n\
                 - L4.E entries: {l4e}\n\
                 - All 7 ReplayReport indicators GREEN: {all_pass}\n\
                 - Live `state_root_t` (post-drain): `{live_root}`\n\
                 - Replay `final_state_root_hex`: `{replay_root}`\n\
                 - Pre-shutdown `conditional_collateral_t` size: {coll_count}\n\
                 - Pre-shutdown `conditional_share_balances_t` owner count: {owners}\n\
                 \n\
                 ## What this evidence proves (RQ3 closure — round-6)\n\
                 \n\
                 1. Two real signed TB-13 typed-tx (CompleteSetMint + CompleteSetRedeem) flow through the full production path: `submit_agent_tx` → driver → `Git2LedgerWriter` persist → on-disk L4 chain.\n\
                 2. Pre-shutdown live state has non-empty TB-13 maps (sanity).\n\
                 3. `verify_chaintape` reconstructs a `QState` from the persisted runtime_repo + cas + initial_q_state.json + agent_pubkeys.json + pinned_pubkeys.json whose `final_state_root_hex` matches the live `state_root_t`. Codex round-4 follow-up clarification: the state-root mutator hashes `domain || prev_root || canonical_tx`, NOT the full QState — so state-root equality on its own proves deterministic tx-chain replay (same initial state + same canonical-encoded txs + same pure dispatcher → same root); it does NOT directly assert byte-equal QState reconstruction.\n\
                 4. **Round-6 direct map-equality check**: the smoke also runs `replay_full_transition` manually against the persisted artifacts and asserts `replayed_q.economic_state_t.conditional_collateral_t == live_q.economic_state_t.conditional_collateral_t` AND `... .conditional_share_balances_t == ...` AND full `economic_state_t` equality. This is the direct map-equality evidence that closes RQ3 without relying on dispatch-determinism implication.\n\
                 5. Submit-time + replay-time agent signature verification is exercised end-to-end for both `CompleteSetMint` and `CompleteSetRedeem` (Gate 4 covers both).\n\
                 6. Two-tx state-root chain (initial → mint → redeem) replays deterministically.\n\
                 \n\
                 ## What is NOT in scope here\n\
                 \n\
                 - **`MarketSeedTx`**: not exercised in this smoke. Coverage lives in `tests/tb_13_complete_set.rs::sg_13_3` / `sg_13_4` + canonical encode round-trip in `typed_tx.rs` U3. Adding seed to this smoke would not add chaintape-replay evidence beyond what mint already proves (seed mutates the same maps).\n\
                 - **Resolution mid-test flip**: `task-REDEEM` is pre-seeded as `Finalized` in `initial_q` rather than flipped via a system-emitted `FinalizeReward` / `TaskBankruptcy` mid-test. The state-flip mechanism itself is exercised by TB-8 / TB-11 integration tests; here we focus on the TB-13 mint+redeem chaintape replay determinism.\n\
                 - **Per-tactic decomposition**: per `feedback_chaintape_externalized_proposal`, ChainTape records what the system externalized via `submit_typed_tx`, not private CoT. 1 LLM call → 1 compound payload = 1 Attempt Node remains in effect.\n",
                l4 = report.l4_entries,
                l4e = report.l4e_entries,
                all_pass = report.all_indicators_pass(),
                live_root = live_state_root_hex,
                replay_root = final_state_root_hex,
                coll_count = collateral_count,
                owners = share_owner_count,
            ),
        );
    }
}
