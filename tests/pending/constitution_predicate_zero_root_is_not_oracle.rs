//! PENDING GATE (G3 / M07) — zero-root admission is verdict-trusting, not oracle.
//!
//! STATUS: PENDING / EXPECTED-RED until the Class-4 src/ admission change lands
//! under the user's §8 token `APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE`.
//!
//! ── WHAT THIS GATE PROVES ────────────────────────────────────────────────
//! `src/state/sequencer.rs:1231` — `verify_work_predicates` zero-root branch:
//!
//!     if q.predicate_registry_root_t == Hash::ZERO {
//!         for (pid, bwp) in work.predicate_results.acceptance.iter() {
//!             if !bwp.value { return Err(AcceptancePredicateFailed(pid)); }
//!         }
//!         ... return Ok(());   // <- trusts the SELF-REPORTED booleans
//!     }
//!
//! When `predicate_registry_root_t == Hash::ZERO`, admission TRUSTS the
//! agent-supplied `BoolWithProof.value` booleans verbatim. It does NOT bind a
//! `PredicateRegistry`, does NOT load any proof from CAS, and does NOT
//! re-execute the predicate against the work. A submitter can therefore self-
//! assert `acceptance = {pid: true}` and be ADMITTED with zero oracle re-
//! execution. Only when `predicate_registry_root_t != Hash::ZERO` (line ~1245)
//! does the sequencer re-execute against the bound registry + CAS proofs.
//!
//! For an OS-qualified run this is the wrong default: admission must be an
//! ORACLE (re-execute the predicate), not a verdict-trusting pass-through of an
//! agent's own boolean. The desired M07 invariant: an OS-qualified run cannot
//! admit / oracle-replay with `predicate_registry_root_t == Hash::ZERO`; it must
//! carry a NON-ZERO bound registry root so the sequencer re-executes (the
//! line-1245 branch).
//!
//! ── HOW THE GATE OBSERVES IT (public behavior only) ──────────────────────
//! We submit a `WorkTx` under a genesis `QState` (`predicate_registry_root_t ==
//! Hash::ZERO`) whose acceptance predicate is a SELF-ASSERTED `true` with NO
//! proof (`proof_cid: None`). Today the sequencer ADMITS it (zero-root verdict-
//! trust). The post-fix invariant we assert is the DESIRED state: such a run is
//! NOT OS-qualified, i.e. admission under a zero registry root must be REFUSED
//! (forcing a non-zero bound root + real re-execution). Today admission
//! succeeds, so the assertion that "zero-root admission was refused" FAILS
//! (expected-red), cleanly proving the verdict-trust bypass.
//!
//! As a paired positive control we also show that the SAME claim under a
//! NON-ZERO bound registry root routes through the oracle branch (line ~1245):
//! with an empty bound registry, an unexpected self-asserted predicate key is
//! REJECTED (`AcceptancePredicateUnexpected`) — i.e. the bound path does not
//! blindly trust the booleans. This control documents the exact branch the M07
//! fix must make mandatory for OS-qualified runs.
//!
//! ── EXCLUSION MECHANISM (same as G1/G2) ──────────────────────────────────
//! Under `tests/pending/` (not auto-compiled; no Cargo.toml edit — Cargo.toml is
//! Trust-Root-pinned), not `constitution_*.rs` at top level, not in the
//! constitution gates manifest → invisible to `cargo test --workspace`,
//! `run_constitution_gates.sh`, and `constitution_matrix_drift`. Run on demand
//! by `scripts/run_pending_agentic_os_kill_conditions.sh` via `rustc --test`.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{InMemoryLedgerWriter, LedgerWriter};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::{MicroCoin, StakeMicroCoin};
use turingosv4::state::q_state::{AgentId, Hash, QState, TaskId, TxId};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, EscrowLockTx, PredicateId, PredicateResultsBundle, ReadKey,
    SafetyOrCreation, TaskOpenTx, TypedTx, WorkTx, WriteKey,
};
use turingosv4::top_white::predicates::registry::{BootPredicateManifest, PredicateRegistry};

fn block_on<F: std::future::Future>(fut: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("current-thread runtime")
        .block_on(fut)
}

struct SeqHarness {
    _tmp: tempfile::TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    registry_root: Hash,
}

/// Build a sequencer whose bound `PredicateRegistry` is the (empty) one. We also
/// stash its merkle root so a test can set `q.predicate_registry_root_t` to it
/// and exercise the NON-ZERO (oracle) branch. Note: an EMPTY registry's root is
/// `sha256("")` — a NON-ZERO hash — so binding it routes through line ~1245.
fn fresh_seq(initial_q: QState) -> SeqHarness {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("keypair"));
    let writer: Arc<RwLock<dyn LedgerWriter>> = Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let rejections = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
    let registry =
        PredicateRegistry::from_boot_manifest(BootPredicateManifest::empty()).expect("empty reg");
    let registry_root = registry.merkle_root_hash();
    let preds = Arc::new(registry);
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
        registry_root,
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

/// A `WorkTx` whose acceptance carries a SELF-ASSERTED `true` boolean with NO
/// proof (`proof_cid: None`). Under the zero-root branch this is trusted
/// verbatim; under the bound branch the unexpected key is rejected.
fn make_self_asserted_worktx(task: &str, agent: &str, parent: Hash) -> TypedTx {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("self_asserted_acc".into()),
        BoolWithProof {
            value: true,
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

/// Fund a task and submit a self-asserted-true WorkTx; return whether it was
/// ADMITTED. `bind_registry_root == true` sets `q.predicate_registry_root_t` to
/// the sequencer's (non-zero) empty-registry root, exercising the oracle branch.
fn admits_self_asserted(task: &str, sponsor: &str, solver: &str, bind_registry_root: bool) -> bool {
    let mut q = genesis_with_balances(&[(sponsor, 100), (solver, 10)]);
    // Build first so we know the registry root; then re-build with the bound
    // root in the initial QState if requested.
    let probe = fresh_seq(q.clone());
    let registry_root = probe.registry_root;
    drop(probe);
    if bind_registry_root {
        q.predicate_registry_root_t = registry_root;
    }
    let mut h = fresh_seq(q);

    let pre = h.seq.q_snapshot().expect("pre snap").state_root_t;
    let open = make_task_open(task, sponsor, pre);
    block_on(h.seq.submit(open)).expect("open submit");
    let _ = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("open env")
        .expect("open accepted");
    let parent = h.seq.q_snapshot().expect("post-open").state_root_t;
    let lock = make_escrow_lock(task, sponsor, 50 * 1_000_000, parent);
    block_on(h.seq.submit(lock)).expect("lock submit");
    let _ = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("lock env")
        .expect("lock accepted");
    let parent = h.seq.q_snapshot().expect("post-lock").state_root_t;

    let work = make_self_asserted_worktx(task, solver, parent);
    block_on(h.seq.submit(work)).expect("work submit");
    let outcome = h.seq.try_apply_one(&mut h.rx).expect("work env");
    outcome.is_ok()
}

/// G3 — an OS-qualified run must NOT admit/oracle-replay with a ZERO predicate
/// registry root; zero-root admission trusts self-reported booleans instead of
/// re-executing the predicate oracle.
///
/// EXPECTED RESULT AT PRE-§8: **RED**. Under `predicate_registry_root_t ==
/// Hash::ZERO` a self-asserted `true` WorkTx is ADMITTED with no oracle re-
/// execution, so the desired "zero-root admission refused for OS-qualified runs"
/// invariant is violated. When the M07 fix lands (OS-qualified admission
/// requires a non-zero bound registry root → real re-execution), the zero-root
/// self-assertion is refused and this gate turns GREEN.
#[test]
fn m07_os_qualified_run_must_not_admit_under_zero_predicate_registry_root() {
    // PAIRED POSITIVE CONTROL: under a NON-ZERO bound registry root the oracle
    // branch (sequencer.rs:1245) does NOT blindly trust the boolean — an
    // unexpected self-asserted predicate key is REJECTED. This documents the
    // branch the M07 fix must make mandatory.
    let admitted_bound = admits_self_asserted("task-g3-bound", "sp-g3-b", "sv-g3-b", true);
    assert!(
        !admitted_bound,
        "G3 control: under a NON-ZERO bound registry root the oracle branch must \
         reject an unexpected self-asserted predicate (no blind boolean trust). \
         If this fails, the bound branch itself drifted."
    );

    // THE BYPASS: under a ZERO registry root the self-asserted true is admitted.
    let admitted_zero_root = admits_self_asserted("task-g3-zero", "sp-g3-z", "sv-g3-z", false);

    // DESIRED post-fix invariant: zero-root admission must be REFUSED for an
    // OS-qualified run. RED today because the run is admitted.
    assert!(
        !admitted_zero_root,
        "M07 ZERO-ROOT VERDICT-TRUST DEMONSTRATED (PENDING / EXPECTED-RED): a \
         WorkTx with a SELF-ASSERTED acceptance predicate (value=true, \
         proof_cid=None) was ADMITTED under predicate_registry_root_t == \
         Hash::ZERO with NO oracle re-execution. src/state/sequencer.rs:1231 \
         trusts the agent-supplied booleans verbatim in the zero-root branch — it \
         binds no PredicateRegistry and loads no CAS proof. An OS-qualified run \
         must NOT admit/oracle-replay at a zero registry root; it must carry a \
         NON-ZERO bound root so the sequencer re-executes (line ~1245). The \
         desired single-admission predicate gate (§8 token \
         APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE) must refuse zero-root \
         admission for OS-qualified runs. Until that Class-4 src/ change lands, \
         this gate stays RED."
    );
}
