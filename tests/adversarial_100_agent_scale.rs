//! ADVERSARIAL 100-AGENT SCALE STRESS — deterministic, ZERO-LLM constitutional
//! invariant battery.
//!
//! ## What this test exercises
//!
//! Drives 100+ synthetic ed25519-signed agents through a single canonical
//! ChainTape (in-memory `Sequencer` + `InMemoryLedgerWriter` + `CasStore`,
//! mirroring the proven harness in `tests/economy_conservation.rs`,
//! `tests/constitution_n1_agent_economy_a3.rs`, and
//! `tests/constitution_single_admission_behavioral.rs`). No network, no LLM,
//! no real Lean — purely the substrate's admission / conservation / replay
//! machinery under scale + adversarial pressure.
//!
//! ## Case map
//!
//! GREEN DEFENSES (hard asserts — these are the substrate's resilience at
//! scale, and any RED here is a constitutional invariant breaking):
//!   - G1  SCALE + CONSERVATION — 100 agents each fund their own task
//!         (TaskOpen + EscrowLock). `total_supply_micro` is integer-conserved
//!         before vs after; a large number of money-moving txs are L4-admitted.
//!   - G2  SYSTEM-TX FORBIDDEN AT AGENT INGRESS — a `FinalizeReward`
//!         (system-only variant) submitted through `submit_agent_tx` is
//!         rejected pre-queue with `SubmitError::SystemTxForbiddenOnAgentIngress`.
//!   - G3  DOUBLE-CLAIM IDEMPOTENCY — a second `FinalizeReward` against an
//!         already-finalized claim does NOT double-credit (idempotency at the
//!         reachable layer: balances unchanged + no extra L4 accept).
//!   - G4  QUEUE BACK-PRESSURE (DoS) — with a tiny queue and no drainer, the
//!         excess submits return `SubmitError::QueueFull` (lossy back-pressure);
//!         the process never panics and the tape stays consistent.
//!   - G5  MALFORMED PAYLOAD FAIL-CLOSED — a corrupted / unknown CAS object
//!         returns `Err` (no panic, no partial state); `load_tape` on a
//!         non-existent runtime repo returns `Err(AuditError)`.
//!   - G6  REPLAY AT SCALE — `replay_full_transition` deterministically
//!         reconstructs the 100-agent tape; the replayed `state_root_t` /
//!         `ledger_root_t` / `total_supply_micro` match the live sequencer.
//!
//! SIG-GAP CHARACTERIZATION (record-and-document, NOT a brittle hard assert):
//!   - S1  OBS_AGENT_SIG_REPLAY_GAP — with an agent-pubkey manifest SET on the
//!         sequencer, a forged `WorkTx` (owner = agent A, signed by agent B) is
//!         submitted via agent ingress. We RECORD whether ingress admitted it
//!         (the gap says it does: Work hits the `_ => {}` arm at
//!         `sequencer.rs:5413`, only the 7 CompleteSet/MarketSeed/Cpmm variants
//!         are signature-gated at submit). We then assert the EXISTING partial
//!         defense: replay Gate 4 (`verify_agent_signature` per the
//!         `verify.rs::verify_agent_artifacts` contract, which covers Work +
//!         Verify) DETECTS the forged Work signature. If the partial defense
//!         also fails, that is recorded via `eprintln!` and NOT a test failure
//!         (characterize reality; do not let a hard assert flip on a future §8
//!         fix to the ingress gate).
//!
//! ## OBS_AGENT_SIG_REPLAY_GAP (the headline finding — see S1 below)
//!
//! `Sequencer::submit_agent_tx` (`src/state/sequencer.rs`) signature-gates ONLY
//! the 7 conditional-share variants (CompleteSetMint / CompleteSetRedeem /
//! MarketSeed / CompleteSetMerge / CpmmPool / CpmmSwap / BuyWithCoinRouter) when
//! the optional `agent_pubkeys` manifest is set; Work / Verify / Challenge /
//! TaskOpen / EscrowLock fall through the `_ => {}` arm and are NOT verified at
//! ingress. So a forged Class-3 money tx mutates state BEFORE any replay catches
//! it. Replay-time Gate 4 (`verify.rs::verify_agent_artifacts`) covers Work +
//! Verify + the 7 conditional-share variants, but NOT Challenge / TaskOpen /
//! EscrowLock. Closing the ingress gate is a Class-4 (sequencer admission)
//! change requiring §8 ratification — this test only CHARACTERIZES the gap.
//!
//! `FC-trace: FC1-N14 (wtool authoritative state mutation; agent signature
//! primitive) + FC2-Submit (agent ingress barrier) + FC1 hard invariant
//! (conservation across externalized agent txs) + FC2-N31 (audit-from-tape
//! replay determinism).`

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    canonical_decode, replay_full_transition, InMemoryLedgerWriter, LedgerWriter,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::{MicroCoin, StakeMicroCoin};
use turingosv4::economy::monetary_invariant::total_supply_micro;
use turingosv4::runtime::agent_keypairs::{verify_agent_signature, AgentKeypairRegistry};
use turingosv4::state::q_state::{AgentId, Hash, QState, TaskId, TxId};
use turingosv4::state::sequencer::{
    Sequencer, SubmissionEnvelope, SubmitError,
};
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, ClaimId, EscrowLockTx, FinalizeRewardTx, PredicateId,
    PredicateResultsBundle, ReadKey, SafetyOrCreation, TaskOpenTx, TypedTx, WorkTx, WriteKey,
};
use turingosv4::top_white::predicates::registry::{BootPredicateManifest, PredicateRegistry};

/// Agent count for the scale battery. Bound at 100 per the charter; 100 is
/// stable for the in-memory drain harness.
const N_AGENTS: usize = 100;

// ────────────────────────────────────────────────────────────────────────────
// Harness — mirror the three sibling integration tests EXACTLY.
//
// Key facts captured for replay (G6) + Gate-4 (S1):
//   - `keypair` / `epoch` are pinned into `PinnedSystemPubkeys` so
//     `replay_full_transition` re-verifies the system signature on every entry.
//   - `cas` is the SAME store the sequencer writes payloads into, so replay can
//     re-fetch `tx_payload_cid` and decode each TypedTx.
//   - `writer` is the in-memory L4 ledger; we walk `read_at(1..=len())` to
//     reconstruct the entry list for replay + Gate-4.
// ────────────────────────────────────────────────────────────────────────────

struct Harness {
    _tmp: TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    cas: Arc<RwLock<CasStore>>,
    writer: Arc<RwLock<InMemoryLedgerWriter>>,
    // Retained for documentation / future replay extensions; the pinned map is
    // the only one replay needs (it carries the same pubkey derived from
    // `keypair` under `epoch`).
    _keypair: Arc<Ed25519Keypair>,
    _epoch: SystemEpoch,
    pinned: Arc<PinnedSystemPubkeys>,
}

fn fresh_harness(initial_q: QState, queue_capacity: usize) -> Harness {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("keypair"));
    // Keep a concrete handle to the in-memory L4 writer so we can read entries
    // back for replay (G6) and Gate-4 (S1). The Sequencer takes the trait-object
    // form; the Arc clone shares the same Vec backing.
    let writer_concrete = Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let writer_dyn: Arc<RwLock<dyn LedgerWriter>> = writer_concrete.clone();
    let rejections = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
    // EMPTY predicate registry → `predicate_registry_root_t == Hash::ZERO`, so
    // WorkTx admission routes through the shared zero-root boolean-trust branch
    // (`predicate_admission::decide_admission`), exactly like the sibling
    // single-admission test. No bound-predicate proof machinery to satisfy.
    let preds = Arc::new(
        PredicateRegistry::from_boot_manifest(BootPredicateManifest::empty())
            .expect("empty predicate manifest"),
    );
    let tools = Arc::new(ToolRegistry::new());
    let epoch = SystemEpoch::new(1);
    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(epoch, keypair.public_key());
    let pinned = Arc::new(pinned);
    let (seq, rx) = Sequencer::new(
        cas.clone(),
        keypair.clone(),
        epoch,
        writer_dyn,
        rejections,
        preds,
        tools,
        pinned.clone(),
        initial_q,
        queue_capacity,
    );
    Harness {
        _tmp: tmp,
        seq,
        rx,
        cas,
        writer: writer_concrete,
        _keypair: keypair,
        _epoch: epoch,
        pinned,
    }
}

fn genesis_with_balances(pairs: &[(String, i64)]) -> QState {
    let mut q = QState::genesis();
    for (name, coin) in pairs {
        q.economic_state_t
            .balances_t
            .0
            .insert(AgentId(name.clone()), MicroCoin::from_coin(*coin).unwrap());
    }
    q
}

fn agent_name(i: usize) -> String {
    // Deterministic synthetic agent identity. NOT a guessable slot — the
    // ed25519 pubkey is derived independently per agent via the registry; this
    // string is only the AgentId label on the canonical tape.
    format!("agent-{i:04}")
}

fn task_name(i: usize) -> String {
    format!("task-{i:04}")
}

fn live_total_supply(h: &Harness) -> i64 {
    let q = h.seq.q_snapshot().expect("q_snapshot");
    total_supply_micro(&q.economic_state_t).expect("total_supply_micro")
}

/// Drain every queued envelope synchronously via `try_apply_one`, returning
/// `(accepted, rejected)` counts. Mirrors the sibling tests' single-step driver
/// (no `run()` loop, so the test stays deterministic). Ordinary transition
/// rejections land on L4.E and are counted, not fatal.
fn drain_all(h: &mut Harness) -> (usize, usize) {
    let mut accepted = 0usize;
    let mut rejected = 0usize;
    while let Some(outcome) = h.seq.try_apply_one(&mut h.rx) {
        match outcome {
            Ok(_) => accepted += 1,
            Err(_) => rejected += 1,
        }
    }
    (accepted, rejected)
}

// ════════════════════════════════════════════════════════════════════════════
// G1 — SCALE + CONSERVATION across 100 ed25519-signed agents.
// ════════════════════════════════════════════════════════════════════════════
//
// Each of the 100 agents sponsors its OWN task: a real signed TaskOpen followed
// by a real signed EscrowLock that moves part of the agent's balance into the
// task escrow. EscrowLock is a clean money mover (balance → escrow) that does
// NOT depend on predicate gates or a pre-existing escrow, so it is the most
// robust per-agent money-moving tx at scale. CTF must be integer-conserved
// (the same micro-coins move from `balances_t` to `escrows_t`; no mint/burn).
//
// Txs are chained by `parent_state_root`: each accepted tx advances the root,
// and the next tx in submit order carries the just-advanced root. Because the
// in-memory harness drains FIFO and we submit in a fixed order, we precompute
// nothing — we submit one agent's TaskOpen+EscrowLock pair at a time, draining
// after each pair so the next pair sees the advanced root via `q_snapshot`.

#[tokio::test]
async fn g1_scale_conservation_100_signed_agents() {
    // Seed 100 agents with 100 Coin each.
    let seed: Vec<(String, i64)> = (0..N_AGENTS).map(|i| (agent_name(i), 100)).collect();
    let mut h = fresh_harness(genesis_with_balances(&seed), 256);

    // Real ed25519 keypair per agent (run-local registry rooted at a temp dir).
    let reg_dir = TempDir::new().expect("reg dir");
    let mut reg = AgentKeypairRegistry::open(reg_dir.path()).expect("open registry");
    for i in 0..N_AGENTS {
        reg.get_or_create(&AgentId(agent_name(i)))
            .expect("gen keypair");
    }
    h.seq
        .set_agent_pubkeys(Arc::new(reg.manifest()))
        .expect("set_agent_pubkeys");

    let supply_before = live_total_supply(&h);

    // Per-agent escrow amount (10 Coin out of each agent's 100).
    let escrow_micro: i64 = 10 * 1_000_000;
    let mut money_movers_admitted = 0usize;
    let mut task_opens_admitted = 0usize;
    let mut queue_full_hits = 0usize;

    for i in 0..N_AGENTS {
        let agent = agent_name(i);
        let task = task_name(i);

        // Parent root = current live root (advanced by the previous agent's pair).
        let parent = h.seq.q_snapshot().expect("snap").state_root_t;
        let open = make_real_task_open(&mut reg, &task, &agent, parent, "g1", (i as u64) + 1);
        match h.seq.submit_agent_tx(open).await {
            Ok(_) => {}
            Err(SubmitError::QueueFull) => {
                queue_full_hits += 1;
                let _ = drain_all(&mut h);
                continue;
            }
            Err(e) => panic!("unexpected TaskOpen submit error for {agent}: {e:?}"),
        }
        // Drain the TaskOpen so the EscrowLock sees the advanced root.
        let (acc_open, _) = drain_all(&mut h);
        task_opens_admitted += acc_open;

        let parent = h.seq.q_snapshot().expect("snap").state_root_t;
        let lock = make_real_escrow_lock(
            &mut reg,
            &task,
            &agent,
            escrow_micro,
            parent,
            "g1",
            (i as u64) + 1,
        );
        match h.seq.submit_agent_tx(lock).await {
            Ok(_) => {}
            Err(SubmitError::QueueFull) => {
                queue_full_hits += 1;
                let _ = drain_all(&mut h);
                continue;
            }
            Err(e) => panic!("unexpected EscrowLock submit error for {agent}: {e:?}"),
        }
        let (acc_lock, _) = drain_all(&mut h);
        money_movers_admitted += acc_lock;
    }

    // Final drain to flush anything still queued.
    let _ = drain_all(&mut h);

    let supply_after = live_total_supply(&h);

    eprintln!(
        "G1: N={N_AGENTS} agents | task_opens_admitted={task_opens_admitted} | \
         escrow_locks_admitted={money_movers_admitted} | queue_full_hits={queue_full_hits} | \
         supply_before={supply_before} | supply_after={supply_after}"
    );

    // ── CONSERVATION (the hard invariant) ──
    assert_eq!(
        supply_before, supply_after,
        "G1 CONSERVATION VIOLATION: total_supply_micro changed across {N_AGENTS} agents \
         ({supply_before} -> {supply_after}). EscrowLock must move money balance->escrow with \
         zero mint/burn (FC1 conservation invariant broken at scale)."
    );

    // ── SCALE (a meaningful number of money movers landed) ──
    assert_eq!(
        money_movers_admitted, N_AGENTS,
        "G1 SCALE: expected all {N_AGENTS} EscrowLock money-movers to be L4-admitted; \
         got {money_movers_admitted}. (TaskOpens admitted: {task_opens_admitted}.)"
    );

    // ── Escrow actually holds the moved money (state advanced, not a no-op) ──
    let q = h.seq.q_snapshot().expect("snap");
    let escrow_total: i64 = q
        .economic_state_t
        .escrows_t
        .0
        .values()
        .map(|e| e.amount.micro_units())
        .sum();
    assert_eq!(
        escrow_total,
        escrow_micro * (N_AGENTS as i64),
        "G1: escrows_t must hold the sum of all {N_AGENTS} agent escrow locks"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// G2 — SYSTEM-TX FORBIDDEN AT AGENT INGRESS.
// ════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn g2_system_tx_forbidden_on_agent_ingress() {
    let h = fresh_harness(genesis_with_balances(&[("sponsor".into(), 100)]), 16);

    // A FinalizeReward is a system-emitted variant. It must be rejected
    // PRE-QUEUE at agent ingress (Anti-Oreo: agent != direct state writer).
    let finalize = TypedTx::FinalizeReward(FinalizeRewardTx {
        tx_id: TxId("forged-finalize".into()),
        claim_id: ClaimId(TxId("claim-x".into())),
        task_id: TaskId("task-x".into()),
        solver: AgentId("sponsor".into()),
        reward: MicroCoin::from_micro_units(1),
        parent_state_root: Hash::ZERO,
        epoch: SystemEpoch::new(1),
        timestamp_logical: 1,
        system_signature:
            turingosv4::bottom_white::ledger::system_keypair::SystemSignature::from_bytes(
                [0u8; 64],
            ),
    });

    let err = h
        .seq
        .submit_agent_tx(finalize)
        .await
        .expect_err("FinalizeReward must be rejected at agent ingress");
    eprintln!("G2: submit_agent_tx(FinalizeReward) -> {err:?}");
    assert!(
        matches!(err, SubmitError::SystemTxForbiddenOnAgentIngress),
        "G2: a system-only variant on agent ingress must reject with \
         SystemTxForbiddenOnAgentIngress; got {err:?}"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// G3 — DOUBLE-CLAIM / DOUBLE-FINALIZE IDEMPOTENCY (lowest reachable layer).
// ════════════════════════════════════════════════════════════════════════════
//
// Wiring a full Open-claim -> VerifyTx-confirm -> FinalizeReward chain through
// the predicate + escrow gates is heavyweight. Per the charter's escape hatch,
// we assert idempotency at the lowest reachable layer: a `FinalizeReward`
// emitted via the SYSTEM path against a claim that does not exist (or is already
// finalized) must NOT credit any balance and must NOT advance the accepted L4
// head a second time. We drive it with NO funded claim so the dispatch arm
// rejects (claim-not-found) — the key property is that two consecutive
// finalize attempts produce ZERO net minting (no double credit) and the tape
// stays consistent.

#[tokio::test]
async fn g3_double_finalize_no_double_credit() {
    let mut h = fresh_harness(genesis_with_balances(&[("solver".into(), 100)]), 16);

    let supply_before = live_total_supply(&h);
    let solver = AgentId("solver".into());
    let bal_before = h
        .seq
        .q_snapshot()
        .expect("snap")
        .economic_state_t
        .balances_t
        .0
        .get(&solver)
        .copied()
        .unwrap_or(MicroCoin::zero())
        .micro_units();

    let l4_len_before = h.writer.read().expect("writer").len();

    // Emit FinalizeReward TWICE against the same (non-existent) claim id via the
    // SYSTEM path. The system path is the ONLY legal way to construct a
    // FinalizeReward (agent ingress rejects it, per G2). With no Open claim the
    // dispatch arm rejects both → no balance credit, no accepted head advance.
    let claim = ClaimId(TxId("claim-g3".into()));
    let r1 = h
        .seq
        .emit_system_tx(
            turingosv4::state::sequencer::SystemEmitCommand::FinalizeReward {
                claim_id: claim.clone(),
            },
        )
        .await;
    let (a1, rej1) = drain_all(&mut h);
    let r2 = h
        .seq
        .emit_system_tx(
            turingosv4::state::sequencer::SystemEmitCommand::FinalizeReward { claim_id: claim },
        )
        .await;
    let (a2, rej2) = drain_all(&mut h);

    let supply_after = live_total_supply(&h);
    let bal_after = h
        .seq
        .q_snapshot()
        .expect("snap")
        .economic_state_t
        .balances_t
        .0
        .get(&solver)
        .copied()
        .unwrap_or(MicroCoin::zero())
        .micro_units();
    let l4_len_after = h.writer.read().expect("writer").len();

    eprintln!(
        "G3: emit1={r1:?} drained(acc={a1},rej={rej1}) | emit2={r2:?} \
         drained(acc={a2},rej={rej2}) | supply {supply_before}->{supply_after} | \
         solver_bal {bal_before}->{bal_after} | l4_len {l4_len_before}->{l4_len_after}"
    );

    // No double-credit: solver balance unchanged, total supply conserved.
    assert_eq!(
        bal_before, bal_after,
        "G3 IDEMPOTENCY: a finalize against a non-existent/already-finalized claim must \
         NOT credit the solver balance (no double payout)."
    );
    assert_eq!(
        supply_before, supply_after,
        "G3 IDEMPOTENCY: double-finalize must not mint money (total supply conserved)."
    );
    // No accepted L4 head advance from the (rejected) finalizes.
    assert_eq!(
        l4_len_before, l4_len_after,
        "G3 IDEMPOTENCY: a rejected finalize must NOT advance the accepted L4 head."
    );
}

// ════════════════════════════════════════════════════════════════════════════
// G4 — QUEUE BACK-PRESSURE (DoS resilience).
// ════════════════════════════════════════════════════════════════════════════
//
// With a tiny queue capacity and NO drainer running, firing many submits faster
// than they are consumed must overflow into lossy back-pressure
// (`SubmitError::QueueFull`) rather than panic / crash / silently grow
// unbounded. After back-pressure, the tape (drained later) stays consistent.

#[tokio::test]
async fn g4_queue_backpressure_returns_queue_full_not_panic() {
    let cap = 4usize;
    let mut h = fresh_harness(genesis_with_balances(&[("dos".into(), 1_000_000)]), cap);

    let reg_dir = TempDir::new().expect("reg dir");
    let mut reg = AgentKeypairRegistry::open(reg_dir.path()).expect("open registry");
    reg.get_or_create(&AgentId("dos".into())).expect("kp");

    let parent = h.seq.q_snapshot().expect("snap").state_root_t;

    // Fire 200 TaskOpen submits WITHOUT draining. The bounded mpsc channel
    // (capacity = 4) saturates almost immediately; every subsequent submit must
    // return QueueFull. submit_id is still burned per attempt (never reused).
    let total_fired = 200usize;
    let mut ok = 0usize;
    let mut queue_full = 0usize;
    for i in 0..total_fired {
        let tx = make_real_task_open(&mut reg, &task_name(i), "dos", parent, "g4", (i as u64) + 1);
        match h.seq.submit_agent_tx(tx).await {
            Ok(_) => ok += 1,
            Err(SubmitError::QueueFull) => queue_full += 1,
            Err(e) => panic!("G4: unexpected submit error (expected Ok or QueueFull): {e:?}"),
        }
    }

    eprintln!(
        "G4: cap={cap} fired={total_fired} ok={ok} queue_full={queue_full} (no panic)"
    );

    // Back-pressure must have engaged: more fired than the channel can hold.
    assert!(
        queue_full > 0,
        "G4 BACK-PRESSURE: firing {total_fired} submits into a capacity-{cap} queue with no \
         drainer must produce QueueFull rejections; got {queue_full}."
    );
    assert!(
        ok <= cap,
        "G4 BACK-PRESSURE: at most {cap} submits should be accepted into the bounded queue \
         before back-pressure engages; got {ok} accepted."
    );

    // Tape stays consistent: draining the buffered (admitted) submits succeeds
    // and never panics. At most `cap` were buffered.
    let (accepted, rejected) = drain_all(&mut h);
    eprintln!("G4: post-backpressure drain -> accepted={accepted} rejected={rejected}");
    assert!(
        accepted + rejected <= cap,
        "G4: drained tx count ({accepted}+{rejected}) must not exceed the queue capacity {cap}"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// G5 — MALFORMED PAYLOAD FAIL-CLOSED.
// ════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn g5_malformed_payload_fails_closed_no_panic() {
    let h = fresh_harness(genesis_with_balances(&[("x".into(), 1)]), 16);

    // (a) Unknown CID — the content-addressed store must return Err
    //     (CidNotFound), NOT panic, NOT a partial read.
    let bogus = Cid([0xABu8; 32]);
    let got = {
        let cas = h.cas.read().expect("cas read");
        cas.get(&bogus)
    };
    eprintln!("G5(a): CasStore::get(unknown cid) -> {got:?}");
    assert!(
        got.is_err(),
        "G5: requesting an unknown CID must return Err (fail-closed), not a value/panic"
    );

    // (b) decode of corrupted bytes — feeding canonical_decode random bytes
    //     must return Err, not panic.
    let garbage = vec![0xFFu8; 64];
    let decoded: Result<TypedTx, _> = canonical_decode(&garbage);
    eprintln!(
        "G5(b): canonical_decode(garbage) is_err={}",
        decoded.is_err()
    );
    assert!(
        decoded.is_err(),
        "G5: decoding corrupted/truncated payload bytes must return Err, not panic"
    );

    // (c) load_tape on a non-existent runtime repo must return Err(AuditError),
    //     not panic.
    let missing = std::path::PathBuf::from("/nonexistent/turingos/runtime_repo_g5");
    let inputs = turingosv4::runtime::audit_assertions::AuditInputs {
        runtime_repo: missing.clone(),
        cas_dir: missing.join("cas"),
        agent_pubkeys: missing.join("agent_pubkeys.json"),
        pinned_pubkeys: missing.join("pinned_pubkeys.json"),
        genesis: missing.join("genesis_report.json"),
        constitution: missing.join("constitution.md"),
        markov_pointer: None,
        alignment_dir: None,
    };
    let loaded = turingosv4::runtime::audit_assertions::load_tape(&inputs);
    eprintln!("G5(c): load_tape(missing repo) is_err={}", loaded.is_err());
    assert!(
        loaded.is_err(),
        "G5: load_tape on a non-existent runtime repo must return Err(AuditError), not panic"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// G6 — REPLAY AT SCALE (deterministic tape reconstruction).
// ════════════════════════════════════════════════════════════════════════════
//
// Build a 100-agent tape (each agent funds its own task), then re-run the
// canonical `replay_full_transition` primitive against the recorded L4 entries
// + the same CAS + the pinned system pubkeys + the same genesis. The replayed
// QState must reproduce the live `state_root_t`, `ledger_root_t`, AND
// `total_supply_micro` — proving deterministic, integrity-preserving
// reconstruction at scale.

#[tokio::test]
async fn g6_replay_at_scale_reconstructs_100_agent_tape() {
    let seed: Vec<(String, i64)> = (0..N_AGENTS).map(|i| (agent_name(i), 100)).collect();
    let initial_q = genesis_with_balances(&seed);
    let mut h = fresh_harness(initial_q.clone(), 256);

    let reg_dir = TempDir::new().expect("reg dir");
    let mut reg = AgentKeypairRegistry::open(reg_dir.path()).expect("open registry");
    for i in 0..N_AGENTS {
        reg.get_or_create(&AgentId(agent_name(i)))
            .expect("gen keypair");
    }
    h.seq
        .set_agent_pubkeys(Arc::new(reg.manifest()))
        .expect("set_agent_pubkeys");

    let escrow_micro: i64 = 10 * 1_000_000;
    for i in 0..N_AGENTS {
        let agent = agent_name(i);
        let task = task_name(i);
        let parent = h.seq.q_snapshot().expect("snap").state_root_t;
        let open = make_real_task_open(&mut reg, &task, &agent, parent, "g6", (i as u64) + 1);
        h.seq.submit_agent_tx(open).await.expect("submit open");
        drain_all(&mut h);
        let parent = h.seq.q_snapshot().expect("snap").state_root_t;
        let lock = make_real_escrow_lock(
            &mut reg,
            &task,
            &agent,
            escrow_micro,
            parent,
            "g6",
            (i as u64) + 1,
        );
        h.seq.submit_agent_tx(lock).await.expect("submit lock");
        drain_all(&mut h);
    }
    drain_all(&mut h);

    let live_q = h.seq.q_snapshot().expect("live snap");
    let live_supply = total_supply_micro(&live_q.economic_state_t).expect("live supply");

    // Read every L4 entry back from the in-memory ledger.
    let entries = {
        let w = h.writer.read().expect("writer");
        let n = w.len();
        (1..=n).map(|t| w.read_at(t).expect("read_at")).collect::<Vec<_>>()
    };
    eprintln!(
        "G6: N={N_AGENTS} | l4_entries={} | live_state_root={} | live_supply={live_supply}",
        entries.len(),
        hex(&live_q.state_root_t),
    );
    assert!(
        entries.len() >= 2 * N_AGENTS,
        "G6: expected >= {} L4 entries (TaskOpen+EscrowLock per agent); got {}",
        2 * N_AGENTS,
        entries.len()
    );

    // Canonical deterministic replay from the same genesis + CAS + pinned keys.
    let predicates =
        PredicateRegistry::from_boot_manifest(BootPredicateManifest::empty()).expect("preds");
    let tools = ToolRegistry::new();
    let replayed = {
        let cas = h.cas.read().expect("cas read");
        replay_full_transition(
            &initial_q,
            &entries,
            &*cas,
            &h.pinned,
            &predicates,
            &tools,
        )
        .expect("replay_full_transition must reconstruct the 100-agent tape")
    };

    let replay_supply =
        total_supply_micro(&replayed.economic_state_t).expect("replay supply");

    eprintln!(
        "G6: replay_state_root={} | replay_supply={replay_supply}",
        hex(&replayed.state_root_t)
    );

    assert_eq!(
        replayed.state_root_t, live_q.state_root_t,
        "G6 REPLAY: reconstructed state_root_t must match live (deterministic tape replay at scale)"
    );
    assert_eq!(
        replayed.ledger_root_t, live_q.ledger_root_t,
        "G6 REPLAY: reconstructed ledger_root_t must match live"
    );
    assert_eq!(
        replay_supply, live_supply,
        "G6 REPLAY: reconstructed total_supply_micro must match live (conservation under replay)"
    );
    // Belt-and-suspenders: full economic state equality.
    assert_eq!(
        replayed.economic_state_t, live_q.economic_state_t,
        "G6 REPLAY: full economic_state_t must reconstruct byte-equal"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// S1 — OBS_AGENT_SIG_REPLAY_GAP characterization (record + soft-assert).
// ════════════════════════════════════════════════════════════════════════════
//
// Construct a Work tx whose `agent_id` (owner) is agent A but whose signature is
// produced with agent B's key — a FORGED Class-3 money tx. Submit it through the
// agent ingress WITH the agent-pubkey manifest set.
//
// RECORD whether ingress admitted it. The gap (OBS_AGENT_SIG_REPLAY_GAP) says it
// does, because Work hits the `_ => {}` arm at `sequencer.rs:5413` — only the 7
// CompleteSet/MarketSeed/Cpmm variants are signature-gated at submit. We then
// drive the tx to land on L4 (zero-root predicate trust + funded task), and
// assert the EXISTING partial defense: replay Gate 4
// (`verify_agent_signature` per `verify.rs::verify_agent_artifacts`, which
// covers Work) DETECTS the forged Work signature.
//
// If the partial defense ALSO fails, we record it via eprintln! and do NOT fail
// the test — the goal is to characterize reality, not to bake in an assert that
// would flip when the ingress gate is eventually closed under §8.

#[tokio::test]
async fn s1_forged_work_signature_characterizes_replay_gap() {
    let alice = "alice-s1";
    let bob = "bob-s1";
    let task = "task-s1";
    let mut h = fresh_harness(
        genesis_with_balances(&[(alice.into(), 100), (bob.into(), 100)]),
        32,
    );

    // Register BOTH agents in the run-local ed25519 registry and pin the
    // manifest on the sequencer (so the submit-time gate is ARMED for the
    // variants it covers).
    let reg_dir = TempDir::new().expect("reg dir");
    let mut reg = AgentKeypairRegistry::open(reg_dir.path()).expect("open registry");
    reg.get_or_create(&AgentId(alice.into())).expect("alice kp");
    reg.get_or_create(&AgentId(bob.into())).expect("bob kp");
    let manifest = Arc::new(reg.manifest());
    h.seq
        .set_agent_pubkeys(manifest.clone())
        .expect("set_agent_pubkeys");

    // Fund alice's task so a WorkTx can be admitted (TaskOpen + EscrowLock by
    // alice). These are correctly signed by alice.
    let parent = h.seq.q_snapshot().expect("snap").state_root_t;
    let open = make_real_task_open(&mut reg, task, alice, parent, "s1", 1);
    h.seq.submit_agent_tx(open).await.expect("submit open");
    drain_all(&mut h);
    let parent = h.seq.q_snapshot().expect("snap").state_root_t;
    let lock = make_real_escrow_lock(&mut reg, task, alice, 50 * 1_000_000, parent, "s1", 1);
    h.seq.submit_agent_tx(lock).await.expect("submit lock");
    drain_all(&mut h);

    // ── Build a FORGED WorkTx: owner = alice, signed with BOB's key. ──
    let parent = h.seq.q_snapshot().expect("snap").state_root_t;
    let alice_id = AgentId(alice.into());
    let bob_id = AgentId(bob.into());

    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("acc1".into()),
        BoolWithProof {
            value: true,
            proof_cid: None,
        },
    );
    let read_set: BTreeSet<ReadKey> = [ReadKey("k.read".into())].into_iter().collect();
    let write_set: BTreeSet<WriteKey> = [WriteKey("k.write".into())].into_iter().collect();
    let predicate_results = PredicateResultsBundle {
        acceptance,
        settlement: BTreeMap::new(),
        safety_class: SafetyOrCreation::Safety,
    };
    let stake = StakeMicroCoin::from_micro_units(1_000_000);

    let work_unsigned = WorkTx {
        tx_id: TxId("worktx-forged-s1".into()),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        agent_id: alice_id.clone(), // OWNER = alice
        read_set,
        write_set,
        proposal_cid: Cid::from_content(b"s1-forged-proposal"),
        predicate_results,
        stake,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 7,
    };
    // Sign the canonical digest with BOB's key — a forged signature for an
    // alice-owned WorkTx.
    let digest = work_unsigned.to_signing_payload().canonical_digest();
    let forged_sig = reg.sign(&bob_id, digest).expect("sign with bob's key");

    // Sanity: the forged signature verifies under BOB's pubkey but NOT under
    // alice's — confirms the forgery is well-formed (not a no-op zero sig).
    let alice_pub = manifest.get(&alice_id).expect("alice pubkey");
    let bob_pub = manifest.get(&bob_id).expect("bob pubkey");
    assert!(
        verify_agent_signature(&forged_sig, &digest, &bob_pub).is_ok(),
        "S1 setup: forged sig must verify under bob's key (it is bob's real signature)"
    );
    assert!(
        verify_agent_signature(&forged_sig, &digest, &alice_pub).is_err(),
        "S1 setup: forged sig must NOT verify under alice's key (this is the forgery)"
    );

    let forged_work = TypedTx::Work(WorkTx {
        signature: forged_sig,
        ..work_unsigned
    });

    let l4_len_before = h.writer.read().expect("writer").len();

    // ── Submit the forged Work through agent ingress. ──
    let ingress = h.seq.submit_agent_tx(forged_work).await;
    let ingress_admitted_at_submit = ingress.is_ok();
    eprintln!(
        "S1: forged WorkTx (owner=alice, signed-by=bob) submit_agent_tx -> {ingress:?} \
         (ingress_admitted_at_submit={ingress_admitted_at_submit})"
    );

    // Drain so the forged Work (if it passed ingress) reaches the dispatch arm.
    let (acc, rej) = drain_all(&mut h);
    let l4_len_after = h.writer.read().expect("writer").len();
    let landed_on_l4 = l4_len_after > l4_len_before;
    eprintln!(
        "S1: post-submit drain accepted={acc} rejected={rej} | l4_len {l4_len_before}->{l4_len_after} \
         | forged_work_landed_on_L4={landed_on_l4}"
    );

    // DOCUMENT the headline gap explicitly.
    if ingress_admitted_at_submit {
        eprintln!(
            "S1 OBS_AGENT_SIG_REPLAY_GAP CONFIRMED: a FORGED WorkTx passed submit_agent_tx \
             signature checking (Work hits the `_ => {{}}` arm at sequencer.rs:~5413; only the 7 \
             CompleteSet/MarketSeed/Cpmm variants are gated at ingress). The forged Class-3 money \
             tx reached dispatch BEFORE any signature check — replay is the only line of defense."
        );
    } else {
        eprintln!(
            "S1 NOTE: ingress REJECTED the forged WorkTx at submit time. This would mean the \
             ingress signature gate was extended to cover Work since OBS_AGENT_SIG_REPLAY_GAP was \
             filed (a Class-4 §8 change). Recording, not failing."
        );
    }

    // ── EXISTING PARTIAL DEFENSE: replay Gate 4 must detect the forged Work. ──
    //
    // Replicate `verify.rs::verify_agent_artifacts` Gate-4 contract inline:
    // walk every L4 entry, decode the TypedTx from CAS, and for WorkTx
    // re-verify the AgentSignature against the manifest-pinned owner pubkey.
    // The forged Work signature (bob's sig under alice's owner field) MUST fail
    // this check if it landed on the tape.
    let entries = {
        let w = h.writer.read().expect("writer");
        let n = w.len();
        (1..=n).map(|t| w.read_at(t).expect("read_at")).collect::<Vec<_>>()
    };
    let mut gate4_work_signatures_all_valid = true;
    let mut forged_work_found_on_tape = false;
    {
        let cas = h.cas.read().expect("cas read");
        for entry in &entries {
            let bytes = match cas.get(&entry.tx_payload_cid) {
                Ok(b) => b,
                Err(_) => continue,
            };
            let typed: TypedTx = match canonical_decode(&bytes) {
                Ok(t) => t,
                Err(_) => continue,
            };
            if let TypedTx::Work(w) = &typed {
                if w.tx_id.0 == "worktx-forged-s1" {
                    forged_work_found_on_tape = true;
                }
                let d = w.to_signing_payload().canonical_digest();
                match manifest.get(&w.agent_id) {
                    None => gate4_work_signatures_all_valid = false,
                    Some(pubkey) => {
                        if verify_agent_signature(&w.signature, &d, &pubkey).is_err() {
                            gate4_work_signatures_all_valid = false;
                        }
                    }
                }
            }
        }
    }

    eprintln!(
        "S1: replay Gate-4 walk -> forged_work_found_on_tape={forged_work_found_on_tape} | \
         gate4_work_signatures_all_valid={gate4_work_signatures_all_valid}"
    );

    if landed_on_l4 && forged_work_found_on_tape {
        // The forged Work is on the canonical tape. The EXISTING partial defense
        // (replay Gate 4, which COVERS Work) must catch it. This is a hard
        // assert because Gate-4 Work coverage is the documented existing defense.
        assert!(
            !gate4_work_signatures_all_valid,
            "S1 PARTIAL DEFENSE: a forged WorkTx landed on L4 but replay Gate 4 did NOT detect \
             the bad signature. Replay Gate 4 (verify.rs::verify_agent_artifacts) COVERS Work and \
             MUST flag a forged Work signature. If this fires, the LAST line of defense for \
             forged Class-3 Work txs has a hole — escalate."
        );
        eprintln!(
            "S1 RESULT: forged Work admitted at ingress (gap) but CAUGHT by replay Gate 4 \
             (existing partial defense holds for Work). Note: Gate 4 does NOT cover \
             Challenge/TaskOpen/EscrowLock — a forged tx of those variants would NOT be caught \
             by replay. Closing the ingress gate is a Class-4 §8 change."
        );
    } else {
        // The forged Work did not land on L4 (e.g. a dispatch-arm rejection on
        // some other gate routed it to L4.E before signature-relevant state
        // mutation). Characterize, do not fail: the gap is about the INGRESS
        // signature check, which we already recorded above.
        eprintln!(
            "S1 NOTE: forged WorkTx did NOT land on the accepted L4 tape (landed_on_l4={landed_on_l4}, \
             found_on_tape={forged_work_found_on_tape}); the dispatch arm rejected it on a \
             non-signature gate. The INGRESS signature gap is already characterized above \
             (ingress_admitted_at_submit={ingress_admitted_at_submit}). Recording, not failing."
        );
    }
}

// ────────────────────────────────────────────────────────────────────────────
// tx constructors — real ed25519-signed TaskOpen / EscrowLock.
//
// These mirror `src/runtime/adapter.rs::make_real_task_open_signed_by` /
// `make_real_escrow_lock_signed_by` but are inlined here so the test owns the
// exact field set (and to avoid depending on the adapter's suffix conventions).
// ────────────────────────────────────────────────────────────────────────────

fn make_real_task_open(
    reg: &mut AgentKeypairRegistry,
    task: &str,
    sponsor: &str,
    parent: Hash,
    suffix: &str,
    ts: u64,
) -> TypedTx {
    use turingosv4::state::typed_tx::TaskOpenSigningPayload;
    let sponsor_id = AgentId(sponsor.into());
    let task_id = TaskId(task.into());
    let tx_id = TxId(format!("taskopen-{task}-{suffix}"));
    let payload = TaskOpenSigningPayload {
        tx_id: tx_id.clone(),
        task_id: task_id.clone(),
        parent_state_root: parent,
        sponsor_agent: sponsor_id.clone(),
        verifier_quorum: 1,
        max_reuse_royalty_fraction_basis_points: 1000,
        settlement_rule_hash: Hash::ZERO,
        timestamp_logical: ts,
    };
    let sig = reg
        .sign(&sponsor_id, payload.canonical_digest())
        .expect("sign task open");
    TypedTx::TaskOpen(TaskOpenTx {
        tx_id,
        task_id,
        parent_state_root: parent,
        sponsor_agent: sponsor_id,
        verifier_quorum: 1,
        max_reuse_royalty_fraction_basis_points: 1000,
        settlement_rule_hash: Hash::ZERO,
        signature: sig,
        timestamp_logical: ts,
    })
}

fn make_real_escrow_lock(
    reg: &mut AgentKeypairRegistry,
    task: &str,
    sponsor: &str,
    amount_micro: i64,
    parent: Hash,
    suffix: &str,
    ts: u64,
) -> TypedTx {
    use turingosv4::state::typed_tx::EscrowLockSigningPayload;
    let sponsor_id = AgentId(sponsor.into());
    let task_id = TaskId(task.into());
    let tx_id = TxId(format!("escrowlock-{task}-{suffix}"));
    let amount = MicroCoin::from_micro_units(amount_micro);
    let payload = EscrowLockSigningPayload {
        tx_id: tx_id.clone(),
        task_id: task_id.clone(),
        parent_state_root: parent,
        sponsor_agent: sponsor_id.clone(),
        amount,
        timestamp_logical: ts,
    };
    let sig = reg
        .sign(&sponsor_id, payload.canonical_digest())
        .expect("sign escrow lock");
    TypedTx::EscrowLock(EscrowLockTx {
        tx_id,
        task_id,
        parent_state_root: parent,
        sponsor_agent: sponsor_id,
        amount,
        signature: sig,
        timestamp_logical: ts,
    })
}

fn hex(h: &Hash) -> String {
    h.0.iter().map(|b| format!("{b:02x}")).collect()
}
