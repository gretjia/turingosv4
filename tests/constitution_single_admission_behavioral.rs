//! LIVE CONSTITUTION GATE (G2 / M07) — single-admission BEHAVIORAL witness.
//!
//! STATUS: LIVE / GREEN. Added under the user's §8 token
//! `APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE` (2026-06-07), per the
//! §8 decision packet
//! `handover/section8/M07_G2_G3_GATE_REDESIGN_DECISION_2026-06-07.md` §5.
//!
//! ── WHY THIS GATE REPLACES THE OLD PENDING G2 ────────────────────────────
//! The original pending file
//! `tests/pending/constitution_kernel_sequencer_single_admission.rs` was
//! logically self-contradictory: it asserted `kernel_admitted == true` AND
//! `sequencer_admitted == false` AND `kernel_admitted == sequencer_admitted`
//! (i.e. `true == false`). No source can satisfy that — it is a broken test,
//! not a falsifiable invariant (AGENTS.md §7). Crucially its kernel leg fed the
//! 3-arg `step_forward` shim an EMPTY claim set (zero-root PASS → admit=true)
//! while its sequencer leg submitted a FALSE-predicate WorkTx (reject=false):
//! the two legs were deciding DIFFERENT logical claims, then asserting they
//! must agree. It compared apples to oranges.
//!
//! ── WHAT THIS GATE PROVES (the real single-admission invariant) ──────────
//! Feed BOTH admission authorities the SAME logical claim and assert they reach
//! the SAME verdict — the falsifiable runtime property the structural grep gate
//! (`tests/constitution_single_admission_contract.rs`) cannot prove:
//!
//!   * Kernel leg: drive a worker `Proceed` carrying a FAILING acceptance claim
//!     (`value=false`) through `MemoryKernel::step_forward_with_claims`
//!     (`src/memory_kernel.rs`). The kernel routes that claim through the SHARED
//!     `predicate_admission::decide_admission` contract → `Fail` →
//!     `handle_rejection` → the verified head is NOT advanced. `kernel_admitted
//!     == false`.
//!   * Sequencer leg: submit the equivalent `WorkTx` whose acceptance predicate
//!     is `false` under a zero-root `QState` → the sequencer (same shared
//!     contract, zero-root branch) REJECTS it. `seq_admitted == false`.
//!
//! Both authorities REJECT the same false claim → `kernel_admitted ==
//! seq_admitted == false`. A symmetric positive control feeds BOTH a PASSING
//! claim and asserts both ADMIT (`true == true`).
//!
//! This is a REAL falsifiable gate: if the kernel ever stops consulting
//! `decide_admission` (the predicate-blind bypass re-opens), the failing-claim
//! `Proceed` would advance the head again, `kernel_admitted` flips to `true`,
//! and the agreement assertion (`false == false`) goes RED.
//!
//! ── TRIPLE-COUPLING ──────────────────────────────────────────────────────
//! Registered in `scripts/constitution_gates.manifest.toml`
//! (`constitution_single_admission_behavioral`) and referenced in
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`. Discovered by the flat
//! `ls tests/constitution_*.rs` glob in `scripts/run_constitution_gates.sh` and
//! built by `cargo test --workspace`.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{InMemoryLedgerWriter, LedgerWriter};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::charter_core::compile_charter_core;
use turingosv4::economy::money::{MicroCoin, StakeMicroCoin};
use turingosv4::ledger::{ImmutableTapeLedger, MemoryTapeLedger};
use turingosv4::memory_kernel::{EnvironmentResult, MemoryKernel, Task};
use turingosv4::predicate_admission::{PredicateClaim, PredicateClaimSet};
use turingosv4::state::q_state::{AgentId, Hash, QState, TaskId, TxId};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, EscrowLockTx, PredicateId, PredicateResultsBundle, ReadKey,
    SafetyOrCreation, TaskOpenTx, TypedTx, WorkTx, WriteKey,
};
use turingosv4::tokenizer::Tokenizer;
use turingosv4::top_white::predicates::registry::{BootPredicateManifest, PredicateRegistry};

mod support;

// ── shared tiny async-driver (no tokio proc-macro; plain builder + block_on) ──
fn block_on<F: std::future::Future>(fut: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("current-thread runtime")
        .block_on(fut)
}

// ── AUTHORITY A: memory kernel ───────────────────────────────────────────────

/// A worker `EnvironmentResult` the kernel treats as a happy-path candidate:
/// `success == true` plus a parseable prefix-JSON header with `status:"Proceed"`.
/// The ADMISSION decision is taken AFTER this, on the supplied claim set.
fn proceed_env(task_id: &str) -> EnvironmentResult {
    EnvironmentResult {
        raw_output: format!(
            r#"{{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"{task_id}","action":"PROCEED"}}
---BODY---
done"#
        ),
        raw_stderr: String::new(),
        success: true,
    }
}

/// Build a single-acceptance-claim set with the given truth value.
fn acceptance_claim_set(value: bool) -> PredicateClaimSet {
    PredicateClaimSet {
        acceptance: vec![PredicateClaim {
            id: PredicateId("acc1".into()),
            value,
            proof_cid: None,
        }],
        settlement: vec![],
    }
}

/// Drive the kernel `Proceed` path WITH a claim set and report whether it
/// ADMITTED (advanced the verified head). The head advance is gated on the
/// shared `decide_admission` contract: a failing acceptance claim yields a
/// non-advancing rejection (`kernel_admits == false`); a passing claim advances
/// (`kernel_admits == true`).
fn kernel_admits_with_claim(value: bool) -> bool {
    let mut tape = MemoryTapeLedger::new();
    tape.set_verified_head("H0".into());
    let charter = compile_charter_core(
        "# Constitution\n## Art. 0.4 — Q_t version control\nFC1a tape_t.\n".as_bytes(),
        "v1.0",
        &Tokenizer::new(),
    );
    let mut k = MemoryKernel::new(tape, "run-m07-g2-behavioral", charter);
    let task = Task {
        id: "t1".into(),
        prompt: "do the thing".into(),
    };
    let before = k.tape.get_verified_head();
    let _step = k.step_forward_with_claims(&task, proceed_env("t1"), acceptance_claim_set(value));
    // "Admitted" == the kernel advanced its verified head (committed a new
    // accepted state). On a FAIL verdict the head stays frozen.
    k.tape.get_verified_head() != before
}

// ── AUTHORITY B: sequencer ───────────────────────────────────────────────────

struct SeqHarness {
    _tmp: tempfile::TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
}

fn fresh_seq(initial_q: QState) -> SeqHarness {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("keypair"));
    let writer: Arc<RwLock<dyn LedgerWriter>> = Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let rejections = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
    let preds = Arc::new(
        PredicateRegistry::from_boot_manifest(BootPredicateManifest::empty())
            .expect("empty predicate manifest"),
    );
    let tools = Arc::new(ToolRegistry::new());
    let epoch = SystemEpoch::new(1);
    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(epoch, keypair.public_key());
    let (seq, rx) = Sequencer::new(
        cas,
        keypair,
        epoch,
        writer,
        rejections,
        preds,
        tools,
        Arc::new(pinned),
        initial_q,
        16,
    );
    // OBS_AGENT_SIG_REPLAY_GAP closure: pin the deterministic test manifest so
    // fail-closed ingress admits the resigned TaskOpen/EscrowLock/Work fixtures.
    seq.set_agent_pubkeys(Arc::new(support::manifest_for(&["sponsor-g2", "solver-g2"])))
        .expect("set test manifest once");
    SeqHarness {
        _tmp: tmp,
        seq,
        rx,
    }
}

fn genesis_with_balances(pairs: &[(&str, i64)]) -> QState {
    let mut q = QState::genesis();
    for (name, coin) in pairs {
        q.economic_state_t
            .balances_t
            .0
            .insert(AgentId((*name).into()), MicroCoin::from_coin(*coin).unwrap());
    }
    q
}

fn make_task_open(task: &str, sponsor: &str, parent: Hash) -> TypedTx {
    support::resign(TypedTx::TaskOpen(TaskOpenTx {
        tx_id: TxId(format!("taskopen-{task}")),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        sponsor_agent: AgentId(sponsor.into()),
        verifier_quorum: 1,
        max_reuse_royalty_fraction_basis_points: 1000,
        settlement_rule_hash: Hash::ZERO,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    }))
}

fn make_escrow_lock(task: &str, sponsor: &str, amount_micro: i64, parent: Hash) -> TypedTx {
    support::resign(TypedTx::EscrowLock(EscrowLockTx {
        tx_id: TxId(format!("escrowlock-{task}")),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        sponsor_agent: AgentId(sponsor.into()),
        amount: MicroCoin::from_micro_units(amount_micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    }))
}

fn make_worktx(task: &str, agent: &str, parent: Hash, predicate_passes: bool) -> TypedTx {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("acc1".into()),
        BoolWithProof {
            value: predicate_passes,
            proof_cid: None,
        },
    );
    support::resign(TypedTx::Work(WorkTx {
        tx_id: TxId(format!("worktx-{task}")),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        agent_id: AgentId(agent.into()),
        read_set: [ReadKey("k.read".into())].into_iter().collect::<BTreeSet<_>>(),
        write_set: [WriteKey("k.write".into())]
            .into_iter()
            .collect::<BTreeSet<_>>(),
        proposal_cid: Default::default(),
        predicate_results: PredicateResultsBundle {
            acceptance,
            settlement: BTreeMap::new(),
            safety_class: SafetyOrCreation::Safety,
        },
        stake: StakeMicroCoin::from_micro_units(1),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    }))
}

/// Drive the sequencer with a funded `WorkTx` whose acceptance predicate carries
/// `predicate_passes` and report whether the sequencer ADMITTED it (advanced
/// state). The zero-root predicate branch routes through the SAME shared
/// `decide_admission` contract as the kernel: `false` → rejected → `Err`
/// (`seq_admits == false`); `true` → accepted (`seq_admits == true`).
fn sequencer_admits_predicate_worktx(predicate_passes: bool) -> bool {
    let suffix = if predicate_passes { "pass" } else { "fail" };
    let mut h = fresh_seq(genesis_with_balances(&[
        ("sponsor-g2", 100),
        ("solver-g2", 10),
    ]));
    let task = &format!("task-g2-{suffix}");

    // Fund the task: TaskOpen then EscrowLock.
    let pre = h.seq.q_snapshot().expect("pre snap").state_root_t;
    let open = make_task_open(task, "sponsor-g2", pre);
    block_on(h.seq.submit(open)).expect("open submit");
    let _ = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("open env")
        .expect("open accepted");
    let parent = h.seq.q_snapshot().expect("post-open").state_root_t;
    let lock = make_escrow_lock(task, "sponsor-g2", 50 * 1_000_000, parent);
    block_on(h.seq.submit(lock)).expect("lock submit");
    let _ = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("lock env")
        .expect("lock accepted");
    let parent = h.seq.q_snapshot().expect("post-lock").state_root_t;

    // The WorkTx with the given acceptance predicate value.
    let work = make_worktx(task, "solver-g2", parent, predicate_passes);
    block_on(h.seq.submit(work)).expect("work submit");
    let outcome = h.seq.try_apply_one(&mut h.rx).expect("work env");
    outcome.is_ok()
}

/// G2 (negative leg) — a FAILING acceptance claim fed to BOTH authorities must
/// be REJECTED by BOTH, and they must AGREE. This is the falsifiable
/// single-admission invariant: one shared predicate-admission contract, one
/// verdict per logical claim.
///
/// LIVE RESULT: **GREEN** (`false == false`). If the kernel ever stops consulting
/// `decide_admission`, the failing-claim `Proceed` advances the head again →
/// `kernel_admitted` flips to `true` → this goes RED.
#[test]
fn m07_kernel_and_sequencer_reject_the_same_failing_claim() {
    let kernel_admitted = kernel_admits_with_claim(false);
    let seq_admitted = sequencer_admits_predicate_worktx(false);

    assert!(
        !kernel_admitted,
        "M07 single-admission BYPASS REGRESSION: the kernel ADMITTED (advanced \
         verified_head) a worker Proceed carrying a FALSE acceptance claim. The \
         kernel Proceed branch in src/memory_kernel.rs must route the claim \
         through predicate_admission::decide_admission, which returns Fail for a \
         false acceptance claim → handle_rejection (no head advance). A pass here \
         means the predicate-blind kernel bypass re-opened \
         (§8 APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE)."
    );
    assert!(
        !seq_admitted,
        "M07 single-admission: the sequencer ADMITTED a WorkTx whose acceptance \
         predicate is false. The sequencer zero-root branch must reject it via \
         the shared predicate_admission::decide_admission contract \
         (src/state/sequencer.rs)."
    );
    assert_eq!(
        kernel_admitted, seq_admitted,
        "M07 SINGLE-ADMISSION VIOLATION: the memory kernel and the sequencer \
         reached DIFFERENT verdicts for the SAME failing acceptance claim \
         (kernel_admitted={kernel_admitted}, seq_admitted={seq_admitted}). Both \
         admission authorities must consult the one shared predicate-admission \
         contract (src/predicate_admission.rs::decide_admission) and agree."
    );
}

/// G2 (positive leg / control) — a PASSING acceptance claim fed to BOTH
/// authorities must be ADMITTED by BOTH, and they must AGREE. Without this
/// control the negative leg could be satisfied by an authority that rejects
/// EVERYTHING; the control proves both authorities discriminate on the claim
/// value identically.
///
/// LIVE RESULT: **GREEN** (`true == true`).
#[test]
fn m07_kernel_and_sequencer_admit_the_same_passing_claim() {
    let kernel_admitted = kernel_admits_with_claim(true);
    let seq_admitted = sequencer_admits_predicate_worktx(true);

    assert!(
        kernel_admitted,
        "M07 single-admission control: the kernel must ADMIT (advance \
         verified_head) a worker Proceed carrying a TRUE acceptance claim \
         (decide_admission returns Pass under the zero root). A failure here \
         means the kernel over-rejects valid claims."
    );
    assert!(
        seq_admitted,
        "M07 single-admission control: the sequencer must ADMIT a WorkTx whose \
         acceptance predicate is true under a funded zero-root task."
    );
    assert_eq!(
        kernel_admitted, seq_admitted,
        "M07 SINGLE-ADMISSION VIOLATION: the memory kernel and the sequencer \
         reached DIFFERENT verdicts for the SAME passing acceptance claim \
         (kernel_admitted={kernel_admitted}, seq_admitted={seq_admitted}). Both \
         authorities must consult the one shared predicate-admission contract \
         and agree."
    );
}
