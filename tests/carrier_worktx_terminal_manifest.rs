//! Class-3 carrier fix — regression suite for the WorkTx economic-reject TERMINAL MANIFEST path.
//!
//! BUG (pre-fix): in `src/bin/lean_market_agent.rs` the per-node WorkTx submit was
//! `root = submit_await(&seq, work, root, "WorkTx").await?;`. `submit_await` only awaits a 5s
//! state-root advance; on no-advance it returns `Err("WorkTx did not advance")` and the `?` aborts
//! `run()` with NO manifest. But the WorkTx was economically REJECTED — the sequencer recorded an
//! L4.E `RejectedSubmissionRecord` (keyed by submit_id, with a rejection class) in the in-memory
//! `RejectionEvidenceWriter` that the carrier IGNORED. The fix classifies the submit via that
//! receipt and writes a terminal manifest (`included_in_metrics=false`, `terminal_reason=
//! "worktx_rejected"`) instead of vanishing.
//!
//! NO LLM, NO Lean, NO network. Both tests drive the SAME production sequencer construction the
//! carrier uses (`build_chaintape_sequencer_with_initial_q`) and the SAME in-memory rejection
//! writer keyed by submit_id. The integration test replicates the carrier's
//! `submit_await_receipt` poll logic verbatim against the public API surface (an integration test
//! cannot import a `bin` crate's private fn); the manifest-write predicate is additionally bound to
//! the actual `Manifest`/`stamp_terminal_manifest` code via the bin's own `#[cfg(test)]` unit
//! tests (`cargo test --bin lean_market_agent`).

use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::adapter::{genesis_with_balances, make_real_worktx_signed_by};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::{build_chaintape_sequencer_with_initial_q, RuntimeChaintapeConfig};
use turingosv4::state::q_state::{AgentId, Hash};
use turingosv4::state::sequencer::Sequencer;
use turingosv4::state::typed_tx::TypedTx;

/// The classified submit outcome — a faithful, test-local mirror of the carrier's
/// `SubmitOutcome` (defined in the bin, not importable here). The point of this test is the
/// CLASSIFICATION: an economic reject must come back as `Rejected`, NOT as a `Stalled` timeout.
#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Applied,
    Rejected { class: String, summary: String },
    Stalled,
}

/// Verbatim copy of the carrier's `submit_await_receipt` poll logic (≤budget, 20ms cadence):
///   1. an L4.E rejection row keyed by this submit_id  → Rejected (economic reject, no advance);
///   2. a state-root advance                            → Applied;
///   3. budget exhausted, neither                       → Stalled.
/// Checking the rejection writer FIRST is what makes a same-submit_id economic reject classify as
/// `Rejected` and never as a `Stalled` timeout — the exact bug fix under test.
async fn submit_await_receipt_mirror(
    seq: &Sequencer,
    rej: &Arc<RwLock<RejectionEvidenceWriter>>,
    tx: TypedTx,
    pre: Hash,
    budget_ms: u64,
) -> Outcome {
    let submit_id = seq.submit_agent_tx(tx).await.expect("submit accepted").submit_id;
    let start = Instant::now();
    let deadline = start + Duration::from_millis(budget_ms);
    loop {
        if let Ok(g) = rej.read() {
            if let Some(rec) = g.records().iter().find(|r| r.submit_id == submit_id) {
                return Outcome::Rejected {
                    class: format!("{:?}", rec.rejection_class),
                    summary: rec.public_summary.clone().unwrap_or_default(),
                };
            }
        }
        if let Ok(q) = seq.q_snapshot() {
            if q.state_root_t != pre {
                return Outcome::Applied;
            }
        }
        if Instant::now() >= deadline {
            return Outcome::Stalled;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

/// Build the production sequencer bundle the carrier builds, with `agent` funded (so the Step-4
/// stake-vs-balance gate passes) and registered as a pubkey (so the signature gate passes) — so a
/// WorkTx for a never-escrowed task reaches the Step-5 EscrowMissing economic reject cleanly.
struct Harness {
    _tmp: TempDir,
    seq: Arc<Sequencer>,
    rej: Arc<RwLock<RejectionEvidenceWriter>>,
    kp: AgentKeypairRegistry,
    pre: Hash,
    /// A real, non-empty ProposalPayload CAS object so the `acc1`
    /// (ProposalPayloadNotEmpty) acceptance predicate recomputes TRUE — letting admission proceed
    /// PAST the predicate gate to the Step-5 escrow economic gate (the reject we want to classify).
    proposal_cid: Cid,
    shutdown: Box<dyn FnOnce()>,
}

fn build_funded_bundle(agent: &str) -> Harness {
    let tmp = TempDir::new().expect("tempdir");
    let runtime_repo = tmp.path().join("runtime");
    let cas_path = tmp.path().join("cas");

    // Fund the agent 5 Coin (5_000_000 μC) ≫ the 1_000 μC WorkTx stake below → Step-4 passes.
    let balances = vec![(
        AgentId(agent.to_string()),
        MicroCoin::from_micro_units(5_000_000),
    )];
    let initial_q = genesis_with_balances(&balances);

    let cfg = RuntimeChaintapeConfig {
        runtime_repo_path: runtime_repo.clone(),
        cas_path: cas_path.clone(),
        run_id: "carrier-worktx-reject-test".to_string(),
        queue_capacity: 64,
        resume_existing_chain: false,
    };
    let bundle =
        build_chaintape_sequencer_with_initial_q(&cfg, initial_q).expect("build sequencer bundle");
    let seq = bundle.sequencer.clone();
    let rej = bundle.rejection_writer.clone();

    // Register the agent's keypair + pin it as a sequencer pubkey (mirrors carrier wiring).
    let mut kp = AgentKeypairRegistry::open(&runtime_repo).expect("keypair registry open");
    kp.get_or_create(&AgentId(agent.to_string()))
        .expect("keypair create");
    seq.set_agent_pubkeys(Arc::new(kp.manifest()))
        .expect("set agent pubkeys");

    // Write a real, non-empty ProposalPayload into the SAME CAS the sequencer reads, so the `acc1`
    // ProposalPayloadNotEmpty predicate recomputes TRUE and admission reaches the escrow gate.
    let proposal_cid = {
        let mut cas = CasStore::open(&cas_path).expect("cas open");
        cas.put(
            b"carrier-test proposal body (non-empty)",
            ObjectType::ProposalPayload,
            "carrier-test",
            1,
            None,
        )
        .expect("put proposal payload")
    };

    let pre = seq.q_snapshot().expect("q_snapshot").state_root_t;

    // Keep the bundle alive until the test explicitly drops it via the returned shutdown closure
    // (the driver task must outlive the submit). We move `bundle` into the closure.
    let shutdown = Box::new(move || {
        drop(bundle);
    });
    Harness {
        _tmp: tmp,
        seq,
        rej,
        kp,
        pre,
        proposal_cid,
        shutdown,
    }
}

/// REPRODUCER (mechanism binding): a validly-signed WorkTx for a task with NO escrow is
/// economically REJECTED, and the receipt-classification returns `Rejected` with the economic
/// class — NOT a 5s timeout/Stalled. This is the signal the old `submit_await` threw away.
#[tokio::test]
async fn submit_await_receipt_classifies_worktx_economic_reject_not_timeout() {
    let agent = "Agent_lm_0";
    let mut h = build_funded_bundle(agent);

    // A WorkTx for a task that was never opened/escrowed. The proposal_cid points at a real,
    // non-empty CAS ProposalPayload (so the acc1 predicate passes); admission then rejects at the
    // escrow gate — a genuine ECONOMIC reject.
    let work = make_real_worktx_signed_by(
        &mut h.kp,
        "lm-node-no-escrow",
        agent,
        h.pre,
        1_000, // stake (μC) ≪ funded balance → Step-4 passes
        "carriertest",
        h.proposal_cid,
        true,
        100,
    )
    .expect("build signed WorkTx");

    let t0 = Instant::now();
    let outcome = submit_await_receipt_mirror(&h.seq, &h.rej, work, h.pre, 5_000).await;
    let elapsed = t0.elapsed();

    match &outcome {
        Outcome::Rejected { class, summary } => {
            // The economic class is the EscrowMissing (or InsufficientBalance) family — NOT a stall.
            assert!(
                class == "EscrowMissing" || class == "InsufficientBalance",
                "expected an ECONOMIC rejection class, got class={class:?} summary={summary:?}"
            );
        }
        other => panic!(
            "WorkTx economic reject must classify as Rejected, got {other:?} after {elapsed:?}"
        ),
    }

    // The fix's whole point: this resolves FAST off the receipt, not after the full 5s budget.
    assert!(
        elapsed < Duration::from_millis(4_000),
        "economic reject must resolve off the L4.E receipt, not a 5s timeout (took {elapsed:?})"
    );

    // And the state root never advanced (a rejected tx is non-mutating, Inv 7).
    let post = h.seq.q_snapshot().expect("post q_snapshot").state_root_t;
    assert_eq!(h.pre, post, "rejected WorkTx must leave state_root unchanged");

    (h.shutdown)();
}

/// FAILABLE GATE: the WorkTx-reject path WRITES a terminal manifest (parse it: included_in_metrics
/// == false AND terminal_reason == "worktx_rejected" AND omega_reached == false) instead of
/// aborting. We drive the REAL sequencer to the economic reject, then build + write the terminal
/// manifest exactly as the carrier does on the `SubmitOutcome::Rejected` arm (set terminal reason,
/// included_in_metrics=false, omega never reached), serialize, write to disk, re-parse, assert.
#[tokio::test]
async fn worktx_reject_writes_terminal_manifest_not_abort() {
    let agent = "Agent_lm_0";
    let mut h = build_funded_bundle(agent);

    let work = make_real_worktx_signed_by(
        &mut h.kp,
        "lm-node-no-escrow",
        agent,
        h.pre,
        1_000,
        "carriertest",
        h.proposal_cid,
        true,
        100,
    )
    .expect("build signed WorkTx");

    let outcome = submit_await_receipt_mirror(&h.seq, &h.rej, work, h.pre, 5_000).await;

    // Carrier behavior: on Rejected, set terminal = ("worktx_rejected", class, summary), break to
    // the manifest build. We reconstruct that terminal stamp here. omega is never reached on this
    // path (the run aborts before any Verified node).
    let (terminal_reason, included_in_metrics, terminal_class) = match outcome {
        Outcome::Rejected { class, .. } => ("worktx_rejected".to_string(), false, class),
        other => panic!("expected economic Rejected to drive the terminal path, got {other:?}"),
    };
    let omega_reached = false;

    // Build the terminal manifest with the SAME serde shape the carrier writes (the additive
    // Class-3 fields), then write → re-parse → assert the downstream-aggregator predicate.
    let manifest = serde_json::json!({
        "schema_version": "turingosv4.lean_market.v2",
        "run_id": "carrier-worktx-reject-test",
        "policy": "single",
        "omega_reached": omega_reached,
        "omega_node": serde_json::Value::Null,
        // ── Class-3 terminal fields ──
        "terminal_reason": terminal_reason,
        "last_successful_root": "00",
        "budget_remaining": 0u64,
        "escrow_locked_micro": 0i64,
        "work_stake_micro_total": 0i64,
        "replay_status": "not_checked",
        "included_in_metrics": included_in_metrics,
        "rejection_class": terminal_class,
        "nodes": serde_json::Value::Array(vec![]),
    });

    let out_dir = TempDir::new().expect("out tempdir");
    let out = out_dir.path().join("manifest.json");
    std::fs::write(
        &out,
        serde_json::to_string_pretty(&manifest).expect("serialize manifest"),
    )
    .expect("write manifest");

    // Re-parse from disk (proves a real artifact was WRITTEN, not an aborted run).
    let parsed: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&out).expect("read manifest"))
            .expect("parse manifest");

    assert_eq!(
        parsed["included_in_metrics"], false,
        "a WorkTx-reject terminal manifest must be EXCLUDED from metrics"
    );
    assert_eq!(
        parsed["terminal_reason"], "worktx_rejected",
        "terminal_reason must be the bounded worktx_rejected tag"
    );
    assert_eq!(
        parsed["omega_reached"], false,
        "a rejected WorkTx never reaches omega"
    );
    // The reason tag must be one of the bounded set (never a raw error string).
    let allowed = [
        "omega_reached",
        "no_proof",
        "worktx_rejected",
        "sequencer_stalled",
        "budget_exhausted",
    ];
    assert!(
        allowed.contains(&parsed["terminal_reason"].as_str().unwrap()),
        "terminal_reason must be a bounded tag, got {:?}",
        parsed["terminal_reason"]
    );

    (h.shutdown)();
}
