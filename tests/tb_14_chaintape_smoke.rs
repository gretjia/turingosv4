//! TB-14 Atom 6 — ChainTape smoke (chain-backed, deterministic, non-LLM).
//!
//! ## Why this test exists
//!
//! Atom 6 production wire-swap excises legacy decimal-float CPMM
//! scaffolding (`src/prediction_market.rs`, `kernel.markets`,
//! `BoltzmannParams`/`boltzmann_select_parent`) and re-routes the bus
//! snapshot's price-signal surface through the integer-rational
//! `state::compute_price_index` + `state::compute_mask_set` derived
//! views. The L4 chain mechanics (`Sequencer::dispatch_transition`,
//! state-root mutator chain, replay determinism) are untouched — but
//! that invariant must be evidenced post-merge by a chain-backed smoke.
//!
//! Per `feedback_smoke_evidence_naming` (2026-05-01 architect ruling
//! D5): only chain-backed (`Sequencer::apply_one` + on-disk
//! `LedgerEntry` chain + replay-verifiable + tampering-detectable)
//! tests count as "ChainTape smoke" / "smoke tape" / "tape". This
//! file qualifies; the harness's stdout-only `--smoke` is "smoke
//! evidence" only.
//!
//! ## What this proves (post-Atom-6 specific)
//!
//! 1. The chaintape sequencer bootstraps + submits real signed
//!    `CompleteSetMintTx` + `CompleteSetRedeemTx` end-to-end via
//!    `submit_agent_tx` → driver → `Git2LedgerWriter` persist (mirrors
//!    `tests/tb_13_chaintape_smoke.rs` baseline behavior).
//! 2. `verify_chaintape` reconstructs a `QState` from persisted
//!    artifacts whose `final_state_root_hex` matches live
//!    `state_root_t` — Atom 6's snapshot wire-swap did NOT regress
//!    chain-replay determinism (architect §0.2 Tape Canonical).
//! 3. **TB-14 NEW**: `compute_price_index(&live_q.economic_state_t)`
//!    is byte-equal to `compute_price_index(&replayed_q.economic_state_t)`.
//!    The TB-14 derived view is therefore replay-deterministic by
//!    composition (pure function over a byte-equal-replayed
//!    `EconomicState`). FR-14.x / FC3-N42 evidence at the chain-backed
//!    layer.
//! 4. **TB-14 NEW**: `compute_price_index` is idempotent across N
//!    calls (deterministic given fixed input — Art.0.2 enforcement at
//!    the derived-view layer).
//! 5. **TB-14 NEW**: empty `node_positions_t` → empty PriceIndex
//!    BTreeMap (FR-14.3 + halt-trigger #5 extended to the
//!    zero-position case at the chaintape integration layer; the
//!    in-memory unit tests in `tests/tb_14_halt_triggers.rs` already
//!    prove the per-position case).
//!
//! TRACE_MATRIX TB-14 Atom 6 (FC3-N42 + FC2-N28 chaintape replay
//! integration; closes OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03).

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
    build_chaintape_sequencer_with_initial_q, PinnedPubkeyManifest, RuntimeChaintapeConfig,
};
use turingosv4::state::compute_price_index;
use turingosv4::state::q_state::{
    AgentId, QState, ShareSidePair, TaskId, TaskMarketEntry, TaskMarketState, TxId,
};
use turingosv4::state::sequencer::complete_set_mint_accept_state_root;
use turingosv4::state::typed_tx::{
    AgentSignature, CompleteSetMintTx, CompleteSetRedeemTx, EventId, OutcomeSide, ShareAmount,
    TypedTx,
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

    let redeem_event = EventId(TaskId(redeem_task.into()));
    q.economic_state_t.conditional_collateral_t.0.insert(
        redeem_event.clone(),
        MicroCoin::from_micro_units(redeem_units),
    );
    let mut alice_shares: std::collections::BTreeMap<EventId, ShareSidePair> =
        std::collections::BTreeMap::new();
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

fn manual_replay_from_disk(
    runtime_repo_path: &std::path::Path,
    cas_path: &std::path::Path,
) -> QState {
    let initial_q_path = runtime_repo_path.join("initial_q_state.json");
    let initial_q_json =
        std::fs::read_to_string(&initial_q_path).expect("read initial_q_state.json");
    let initial_q: QState =
        serde_json::from_str(&initial_q_json).expect("parse initial_q_state.json");

    let writer = Git2LedgerWriter::open(runtime_repo_path).expect("open Git2LedgerWriter");
    let n = writer.len();
    let entries: Vec<LedgerEntry> = (1..=n)
        .map(|t| writer.read_at(t).expect("read_at"))
        .collect();

    let manifest_path = runtime_repo_path.join("pinned_pubkeys.json");
    let manifest_json = std::fs::read_to_string(&manifest_path).expect("read pinned_pubkeys.json");
    let manifest: PinnedPubkeyManifest =
        serde_json::from_str(&manifest_json).expect("parse pinned_pubkeys.json");
    let mut pinned = PinnedSystemPubkeys::new();
    for entry in &manifest.pubkeys {
        let bytes: Vec<u8> = (0..entry.pubkey_hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&entry.pubkey_hex[i..i + 2], 16).expect("hex"))
            .collect();
        let arr: [u8; 32] = bytes.as_slice().try_into().expect("32-byte pubkey");
        pinned.insert(
            SystemEpoch::new(entry.epoch),
            SystemPublicKey::from_bytes(arr),
        );
    }

    let cas = CasStore::open(cas_path).expect("open cas");
    let predicates = PredicateRegistry::new();
    let tools = ToolRegistry::new();

    replay_full_transition(&initial_q, &entries, &cas, &pinned, &predicates, &tools)
        .expect("replay_full_transition")
}

#[tokio::test]
async fn tb_14_atom_6_post_wire_swap_chaintape_replay_preserves_price_index_determinism() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = RuntimeChaintapeConfig {
        runtime_repo_path: tmp.path().join("runtime_repo"),
        cas_path: tmp.path().join("cas"),
        run_id: "tb14-atom6-smoke".to_string(),
        queue_capacity: 16,
        resume_existing_chain: false,
    };

    let alice = "alice";
    let alice_id = AgentId(alice.into());
    let mint_task = "task-tb14-mint";
    let redeem_task = "task-tb14-redeem";
    let mint_amount_micro: i64 = 2_000_000;
    let redeem_units: i64 = 4_000_000;

    let initial_q = build_smoke_initial_q(alice, mint_task, redeem_task, redeem_units);
    let bundle = build_chaintape_sequencer_with_initial_q(&cfg, initial_q)
        .expect("bootstrap chaintape sequencer");

    let mut reg =
        AgentKeypairRegistry::open(&cfg.runtime_repo_path).expect("open agent keypair registry");
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

    // Mint tx
    let mint_unsigned = CompleteSetMintTx {
        tx_id: TxId("tb14-mint-1".into()),
        parent_state_root: initial_root,
        event_id: EventId(TaskId(mint_task.into())),
        owner: alice_id.clone(),
        amount: MicroCoin::from_micro_units(mint_amount_micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 100,
    };
    let mint_digest = mint_unsigned.to_signing_payload().canonical_digest();
    let mint_sig = reg.sign(&alice_id, mint_digest).expect("sign mint");
    let mint_tx = TypedTx::CompleteSetMint(CompleteSetMintTx {
        signature: mint_sig,
        ..mint_unsigned
    });

    let after_mint_root = complete_set_mint_accept_state_root(&initial_root, &mint_tx);

    let redeem_unsigned = CompleteSetRedeemTx {
        tx_id: TxId("tb14-redeem-1".into()),
        parent_state_root: after_mint_root,
        event_id: EventId(TaskId(redeem_task.into())),
        owner: alice_id.clone(),
        outcome: OutcomeSide::Yes,
        share_amount: ShareAmount::from_units(redeem_units as u128),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 101,
    };
    let redeem_digest = redeem_unsigned.to_signing_payload().canonical_digest();
    let redeem_sig = reg.sign(&alice_id, redeem_digest).expect("sign redeem");
    let redeem_tx = TypedTx::CompleteSetRedeem(CompleteSetRedeemTx {
        signature: redeem_sig,
        ..redeem_unsigned
    });

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

    let seq_handle = bundle.sequencer.clone();
    bundle.shutdown().await.expect("shutdown drain");

    let live_q = seq_handle.q_snapshot().expect("post-drain q_snapshot");
    let live_state_root = live_q.state_root_t;

    // Sanity: TB-13 substrate populated.
    assert!(
        live_q.economic_state_t.conditional_collateral_t.0.len() >= 2,
        "TB-13 sanity: ≥2 conditional_collateral_t entries (pre-seeded redeem + new mint)"
    );

    // ── Atom 6 invariant 1: chain-replay determinism preserved ─────────
    let report = verify_chaintape(
        &cfg.runtime_repo_path,
        &cfg.cas_path,
        &VerifyOptions::default(),
    )
    .expect("verify_chaintape");
    assert!(
        report.l4_entries >= 2,
        "expected ≥2 L4 entries; got {}",
        report.l4_entries
    );
    assert!(
        report.all_indicators_pass(),
        "Atom 6 must NOT regress verify_chaintape — all 7 indicators GREEN. \
         report={report:?}"
    );
    let live_state_root_hex: String = live_state_root
        .0
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let final_state_root_hex = report
        .detail
        .final_state_root_hex
        .as_ref()
        .expect("final_state_root_hex present");
    assert_eq!(
        &live_state_root_hex, final_state_root_hex,
        "Atom 6 must NOT regress chain-replay state-root match"
    );

    // ── Atom 6 invariant 2: derived-view replay determinism ────────────
    //
    // FR-14.x / FC3-N42: compute_price_index over byte-equal-replayed
    // EconomicState yields byte-equal BTreeMap<TxId, NodeMarketEntry>.
    // The TB-14 derived view is replay-deterministic by composition.
    let replayed_q = manual_replay_from_disk(&cfg.runtime_repo_path, &cfg.cas_path);
    assert_eq!(
        replayed_q.economic_state_t, live_q.economic_state_t,
        "Atom 6 must NOT regress EconomicState byte-equality across replay"
    );

    let live_price_index = compute_price_index(&live_q.economic_state_t);
    let replayed_price_index = compute_price_index(&replayed_q.economic_state_t);
    assert_eq!(
        live_price_index, replayed_price_index,
        "TB-14 FC3-N42: compute_price_index must be byte-equal across live vs \
         replay (Art.0.2 derived-view determinism)"
    );

    // ── Atom 6 invariant 3: compute_price_index idempotent ─────────────
    //
    // Calling compute_price_index N times on the same EconomicState must
    // produce N byte-equal BTreeMaps (Art.0.2 pure-function determinism).
    for _ in 0..5 {
        assert_eq!(
            compute_price_index(&live_q.economic_state_t),
            live_price_index,
            "TB-14 FC3-N42: compute_price_index must be idempotent"
        );
    }

    // ── Atom 6 invariant 4: empty node_positions_t → empty PriceIndex ──
    //
    // FR-14.3 / halt-trigger #5 extended: this smoke's CompleteSet flow
    // does NOT mutate node_positions_t (TB-12 substrate untouched here),
    // so the resulting PriceIndex is empty by construction. This pins
    // down the invariant at the chaintape integration layer.
    assert!(
        live_q.economic_state_t.node_positions_t.0.is_empty(),
        "TB-14 chaintape smoke pre-condition: node_positions_t empty after \
         CompleteSet-only flow (TB-12 substrate untouched)"
    );
    assert!(
        live_price_index.is_empty(),
        "TB-14 FR-14.3 + halt-trigger #5: empty node_positions_t → empty \
         PriceIndex (BTreeMap)"
    );

    // ── Persist evidence to canonical handover dir (best-effort) ───────
    //
    // 2026-05-07 evidence-immutability fix: gated behind
    // TURINGOS_TEST_REGENERATE_EVIDENCE=1. See
    // OBS_EVIDENCE_DRIFT_ROOT_CAUSE_2026-05-07.md.
    let evidence_dir = std::path::Path::new("handover/evidence/tb_14_chaintape_smoke_2026-05-03");
    let regen_enabled = std::env::var("TURINGOS_TEST_REGENERATE_EVIDENCE")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if regen_enabled && std::fs::create_dir_all(evidence_dir).is_ok() {
        let report_json = serde_json::to_string_pretty(&report).expect("serialize report");
        let _ = std::fs::write(evidence_dir.join("replay_report.json"), report_json);
        let agent_pubkeys_src = cfg.runtime_repo_path.join("agent_pubkeys.json");
        if agent_pubkeys_src.exists() {
            let _ = std::fs::copy(&agent_pubkeys_src, evidence_dir.join("agent_pubkeys.json"));
        }
        let pinned_src = cfg.runtime_repo_path.join("pinned_pubkeys.json");
        if pinned_src.exists() {
            let _ = std::fs::copy(&pinned_src, evidence_dir.join("pinned_pubkeys.json"));
        }
        let initial_q_src = cfg.runtime_repo_path.join("initial_q_state.json");
        if initial_q_src.exists() {
            let _ = std::fs::copy(&initial_q_src, evidence_dir.join("genesis_report.json"));
        }
        let _ = std::fs::write(
            evidence_dir.join("README.md"),
            format!(
                "# TB-14 Atom 6 — ChainTape smoke (post-wire-swap regression)\n\
                 \n\
                 **Date**: 2026-05-03\n\
                 **Source**: `tests/tb_14_chaintape_smoke.rs::tb_14_atom_6_post_wire_swap_chaintape_replay_preserves_price_index_determinism`\n\
                 **Trigger**: TB-14 Atom 6 production wire-swap (excise legacy CPMM scaffolding; reroute bus snapshot price-signal surface through `compute_price_index` + `compute_mask_set` integer-rational derived views).\n\
                 \n\
                 ## Headline\n\
                 \n\
                 - L4 entries: {l4} (mint + redeem)\n\
                 - L4.E entries: {l4e}\n\
                 - All 7 ReplayReport indicators GREEN: {all_pass}\n\
                 - Live `state_root_t`: `{live_root}`\n\
                 - Replay `final_state_root_hex`: `{replay_root}`\n\
                 - `live.economic_state_t == replayed.economic_state_t`: byte-equal\n\
                 - `compute_price_index(live)` == `compute_price_index(replayed)`: byte-equal\n\
                 - `compute_price_index` idempotent across 5 invocations: ✓\n\
                 - Empty `node_positions_t` → empty PriceIndex BTreeMap: ✓\n\
                 \n\
                 ## What this evidence proves (Atom 6 specific)\n\
                 \n\
                 1. The Atom 6 production wire-swap (excised `prediction_market.rs`, `kernel.markets`, `BoltzmannParams`, legacy f64 `boltzmann_select_parent`; rewired `bus.snapshot` to derive `price_index` + `mask_set` from `Sequencer::q_snapshot`'s `EconomicState`) does NOT regress chain-replay determinism.\n\
                 2. `verify_chaintape` reconstructs a `QState` from persisted artifacts whose `final_state_root_hex` matches live `state_root_t` (Art.0.2 Tape Canonical preserved across the wire-swap).\n\
                 3. The TB-14 derived view (`compute_price_index(econ)`) is replay-deterministic by composition: pure function over a byte-equal-replayed `EconomicState` yields byte-equal `BTreeMap<TxId, NodeMarketEntry>` (FR-14.x / FC3-N42 chaintape integration evidence).\n\
                 4. `compute_price_index` is idempotent across N calls (Art.0.2 pure-function determinism at the derived-view layer).\n\
                 5. Empty `node_positions_t` → empty PriceIndex (FR-14.3 / halt-trigger #5 extended at the chaintape integration layer).\n\
                 \n\
                 ## What is NOT in scope here\n\
                 \n\
                 - **Non-empty PriceIndex via WorkTx**: this smoke uses CompleteSet flow only (TB-13 substrate). A WorkTx-creates-NodePosition flow (TB-12 substrate that produces non-empty PriceIndex) is covered by the in-memory unit tests at `tests/tb_14_price_index.rs` + halt-triggers + `src/state/price_index.rs` inline tests. Per `feedback_chaintape_externalized_proposal`, the chaintape smoke records what the system externalizes via `submit_typed_tx` end-to-end; the per-position aggregation is pure-function-tested elsewhere.\n\
                 - **`mask_set` via Tape children**: `compute_mask_set` requires a Tape; this smoke does not exercise mask computation (covered by `tests/tb_14_mask_set.rs` + halt-triggers #3 / #6).\n\
                 - **Boltzmann v2 selector**: covered by inline tests in `src/sdk/actor.rs::tests::v2_*`. Production wire-up at `experiments/minif2f_v4/src/bin/evaluator.rs:~1559` is exercised by the `--smoke` / `--half` evaluator runs.\n",
                l4 = report.l4_entries,
                l4e = report.l4e_entries,
                all_pass = report.all_indicators_pass(),
                live_root = live_state_root_hex,
                replay_root = final_state_root_hex,
            ),
        );
    }
}
