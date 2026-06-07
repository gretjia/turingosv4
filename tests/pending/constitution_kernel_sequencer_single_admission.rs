//! PENDING GATE (G2 / M07) — single shared predicate-admission contract.
//!
//! STATUS: PENDING / EXPECTED-RED until the Class-4 src/ admission change lands
//! under the user's §8 token `APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE`.
//!
//! ── WHAT THIS GATE PROVES ────────────────────────────────────────────────
//! Today TuringOS has TWO independent admission authorities that can both
//! advance canonical accepted state, and they DISAGREE on whether predicates
//! gate the advance:
//!
//!   AUTHORITY A — `MemoryKernel` (FC1 runtime loop).
//!     `src/memory_kernel.rs:171-188` commits `NodeKind::StateAccepted` and
//!     calls `tape.set_verified_head(..)` purely on
//!     `(parsed_header.status == Proceed, env_result.success)`. It NEVER calls
//!     `verify_work_predicates`, builds no `WorkTx`, and binds no
//!     `PredicateRegistry`. → predicate-BLIND.
//!
//!   AUTHORITY B — `Sequencer` (typed-tx admission).
//!     `src/state/sequencer.rs:1225 verify_work_predicates` IS the predicate
//!     oracle; it runs inside the `WorkTx` dispatch path and routes a failing
//!     acceptance/settlement predicate to L4.E (no state-root advance).
//!     → predicate-AWARE.
//!
//! Because the two authorities apply DIFFERENT admission rules to "advance
//! accepted state", a worker self-report of `Proceed` is sufficient to advance
//! the kernel's verified head even though the SAME logical claim, routed through
//! the sequencer as a `WorkTx` whose acceptance predicate is `false`, is
//! REJECTED. The desired M07 invariant is a SINGLE shared predicate-admission
//! contract: any path that advances canonical accepted state must consult the
//! same predicate oracle and reach the same verdict.
//!
//! ── HOW THE GATE OBSERVES THE SPLIT (public behavior only) ───────────────
//! We construct one logical "failing-predicate" outcome and feed it to BOTH
//! authorities, then assert they agree:
//!   * Kernel leg: a worker `Proceed` whose work would FAIL acceptance — the
//!     kernel has no way to express "predicate failed" (no predicate hook), so
//!     it advances the verified head regardless. We capture `kernel_admitted`.
//!   * Sequencer leg: the equivalent `WorkTx` with a `false` acceptance
//!     predicate under a zero-root `QState` — the sequencer REJECTS it
//!     (`verify_work_predicates` zero-root branch, sequencer.rs:1231-1235). We
//!     capture `sequencer_admitted == false`.
//! The post-fix invariant is `kernel_admitted == sequencer_admitted` for the
//! same logical claim. TODAY `true != false`, so the gate FAILS (expected-red),
//! proving the dual-authority split. When the M07 single-admission gate lands,
//! the kernel routes its accept through the same predicate oracle, both legs
//! agree, and this gate turns GREEN.
//!
//! ── EXCLUSION MECHANISM (same as G1) ─────────────────────────────────────
//! Lives under `tests/pending/` → cargo does NOT auto-discover .rs files in
//! tests/ subdirectories, so `cargo test --workspace` never builds it and NO
//! Cargo.toml change is made (Cargo.toml is Trust-Root-pinned via
//! genesis_payload.toml; editing it would trip TRUST_ROOT_TAMPERED, Class-4,
//! forbidden PRE-§8). Not named `constitution_*.rs` at the tests/ top level and
//! not in `scripts/constitution_gates.manifest.toml`, so neither
//! `scripts/run_constitution_gates.sh` (flat `ls tests/constitution_*.rs` glob)
//! nor `tests/constitution_matrix_drift.rs` (manifest-driven) sees it. The
//! dedicated runner `scripts/run_pending_agentic_os_kill_conditions.sh`
//! compiles it standalone via `rustc --test` and OBSERVES RED.

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
use turingosv4::memory_kernel::{EnvironmentResult, KernelStep, MemoryKernel, Task};
use turingosv4::state::q_state::{AgentId, Hash, QState, TaskId, TxId};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, EscrowLockTx, PredicateId, PredicateResultsBundle, ReadKey,
    SafetyOrCreation, TaskOpenTx, TypedTx, WorkTx, WriteKey,
};
use turingosv4::tokenizer::Tokenizer;
use turingosv4::top_white::predicates::registry::{BootPredicateManifest, PredicateRegistry};

// ── shared tiny async-driver (no tokio proc-macro; plain builder + block_on) ──
fn block_on<F: std::future::Future>(fut: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("current-thread runtime")
        .block_on(fut)
}

// ── AUTHORITY A: memory kernel ───────────────────────────────────────────────

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

/// Drive the kernel happy path and report whether it ADVANCED the verified head
/// (its notion of "admitted accepted state").
fn kernel_admits_on_worker_proceed() -> bool {
    let mut tape = MemoryTapeLedger::new();
    tape.set_verified_head("H0".into());
    let charter = compile_charter_core(
        "# Constitution\n## Art. 0.4 — Q_t version control\nFC1a tape_t.\n".as_bytes(),
        "v1.0",
        &Tokenizer::new(),
    );
    let mut k = MemoryKernel::new(tape, "run-m07-g2", charter);
    let task = Task {
        id: "t1".into(),
        prompt: "do the thing".into(),
    };
    let before = k.tape.get_verified_head();
    let step = k.step_forward(&task, proceed_env("t1"));
    // Precondition sanity: kernel took the Proceed happy path.
    assert!(
        matches!(step, KernelStep::Proceed { .. }),
        "G2 precondition: kernel takes the Proceed happy path on success+Proceed."
    );
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
    TypedTx::TaskOpen(TaskOpenTx {
        tx_id: TxId(format!("taskopen-{task}")),
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

fn make_escrow_lock(task: &str, sponsor: &str, amount_micro: i64, parent: Hash) -> TypedTx {
    TypedTx::EscrowLock(EscrowLockTx {
        tx_id: TxId(format!("escrowlock-{task}")),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        sponsor_agent: AgentId(sponsor.into()),
        amount: MicroCoin::from_micro_units(amount_micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
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
    TypedTx::Work(WorkTx {
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
    })
}

/// Drive the sequencer with a `WorkTx` whose acceptance predicate is FALSE and
/// report whether the sequencer ADMITTED it (advanced state). The sequencer's
/// zero-root predicate branch (sequencer.rs:1231-1235) rejects a false
/// acceptance predicate → `sequencer_admits == false`.
fn sequencer_admits_failing_predicate_worktx() -> bool {
    let mut h = fresh_seq(genesis_with_balances(&[("sponsor-g2", 100), ("solver-g2", 10)]));
    let task = "task-g2";

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

    // The failing-predicate WorkTx (acceptance value = false).
    let work = make_worktx(task, "solver-g2", parent, false);
    block_on(h.seq.submit(work)).expect("work submit");
    let outcome = h.seq.try_apply_one(&mut h.rx).expect("work env");
    outcome.is_ok()
}

/// G2 — kernel and sequencer must reach the SAME predicate-admission verdict for
/// the same logical claim (single shared admission contract).
///
/// EXPECTED RESULT AT PRE-§8: **RED**. The kernel admits (advances verified head)
/// on a bare worker `Proceed`, while the sequencer rejects the equivalent
/// failing-predicate `WorkTx` → two authorities, two verdicts. When the M07
/// single-admission predicate gate lands (kernel routes its accept through the
/// same predicate oracle as the sequencer), both legs agree and this turns GREEN.
#[test]
fn m07_kernel_and_sequencer_must_share_one_predicate_admission_contract() {
    let kernel_admitted = kernel_admits_on_worker_proceed();
    let sequencer_admitted = sequencer_admits_failing_predicate_worktx();

    // Document the observed split for the auditor reading the failure.
    assert!(
        kernel_admitted,
        "G2 precondition: today the kernel advances the verified head on a bare \
         worker Proceed (predicate-blind)."
    );
    assert!(
        !sequencer_admitted,
        "G2 precondition: the sequencer rejects the equivalent failing-predicate \
         WorkTx (predicate-aware zero-root branch, sequencer.rs:1231-1235)."
    );

    assert_eq!(
        kernel_admitted, sequencer_admitted,
        "M07 DUAL-ADMISSION-AUTHORITY DEMONSTRATED (PENDING / EXPECTED-RED): the \
         memory kernel ADMITTED (advanced verified_head) on a bare worker Proceed \
         with no predicate consultation, while the sequencer REJECTED the \
         equivalent failing-predicate WorkTx via its predicate oracle. Two \
         admission authorities (src/memory_kernel.rs:171-188 vs \
         src/state/sequencer.rs:1225) apply different rules to advance canonical \
         accepted state. The desired single-admission predicate gate (§8 token \
         APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE) must route BOTH paths \
         through ONE shared predicate-admission contract so they reach the same \
         verdict. Until that Class-4 src/ change lands, this gate stays RED."
    );
}
