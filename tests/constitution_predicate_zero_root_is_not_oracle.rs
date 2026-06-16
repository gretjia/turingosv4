//! LIVE CONSTITUTION GATE (G3 / M07) — zero-root admission is verdict-trusting,
//! not oracle; an OS-qualified run must REFUSE it.
//! TRACE_MATRIX FC1a-predicates: G3 oracle-not-verdict-trust kill-condition.
//!
//! STATUS: LIVE / GREEN. Promoted from `tests/pending/` after the Class-4 src/
//! change landed under the user's §8 token
//! `APPROVE-M07-G3-OS-QUALIFIED-RUN-FIELD` (2026-06-07). G3 needed a run-level
//! `os_qualified` signal INDEPENDENT of `predicate_registry_root_t` — under the
//! old `os_qualified = (registry_root != ZERO)` derivation the refuse-path was
//! structurally dead inside the zero-root branch (the root IS zero there). The
//! field `QState::os_qualified_t` (folded into `state_root_t`, replayable from
//! tape; flipped `false→true` by the system-only `PredicateBindingActivate`
//! accept) makes the refuse-path live, so the post-fix invariant below holds and
//! this gate is GREEN.
//!
//! Triple-coupled: registered in `scripts/constitution_gates.manifest.toml`
//! (`constitution_predicate_zero_root_is_not_oracle`) and referenced in
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`. Discovered by the flat
//! `ls tests/constitution_*.rs` glob in `scripts/run_constitution_gates.sh` and
//! built by `cargo test --workspace`.
//!
//! ── WHAT THIS GATE PROVES ────────────────────────────────────────────────
//! `src/state/sequencer.rs` — `verify_work_predicates` zero-root branch:
//!
//!     if q.predicate_registry_root_t == Hash::ZERO {
//!         let os_qualified = q.os_qualified_t;   // M07 G3: run-level field
//!         decide_admission(zero_root_hex, claims, os_qualified) ...
//!     }
//!
//! When `predicate_registry_root_t == Hash::ZERO`, the legacy branch TRUSTS the
//! agent-supplied `BoolWithProof.value` booleans verbatim — it binds no
//! `PredicateRegistry`, loads no CAS proof, and does NOT re-execute the
//! predicate. A submitter can self-assert `acceptance = {pid: true}` and be
//! ADMITTED with zero oracle re-execution. For an OS-qualified run that is the
//! wrong default: admission must be an ORACLE, not a verdict-trusting
//! pass-through. The enforced M07 invariant: an OS-qualified run
//! (`os_qualified_t == true`) cannot admit at `predicate_registry_root_t ==
//! Hash::ZERO`; it must carry a NON-ZERO bound registry root so the sequencer
//! re-executes against the bound registry + CAS proofs.
//!
//! ── HOW THE GATE OBSERVES IT (public behavior only) ──────────────────────
//! We submit a `WorkTx` under an OS-qualified `QState` (`os_qualified_t == true`,
//! `predicate_registry_root_t == Hash::ZERO`) whose acceptance predicate is a
//! SELF-ASSERTED `true` with NO proof (`proof_cid: None`). With the field live,
//! `decide_admission` returns `ZeroRootRefusedForOsQualifiedRun` → the sequencer
//! maps it to `PredicateRegistryRootMismatch` → the WorkTx is REJECTED. The
//! assertion "zero-root admission was refused" therefore holds (GREEN). If the
//! field were ever rewired back to `registry_root != ZERO`, the run would be
//! admitted and this gate would flip RED.
//!
//! As a paired positive control we also show that the SAME claim under a
//! NON-ZERO bound registry root routes through the oracle branch: with an empty
//! bound registry, an unexpected self-asserted predicate key is REJECTED — i.e.
//! the bound path does not blindly trust the booleans.

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

mod support;

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
    // OBS_AGENT_SIG_REPLAY_GAP closure: pin the deterministic test manifest;
    // builders re-sign via support::resign so fail-closed ingress admits them.
    support::pin_common_manifest(&seq);
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
        q.economic_state_t.balances_t.0.insert(
            AgentId((*name).into()),
            MicroCoin::from_coin(*coin).unwrap(),
        );
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
    support::resign(TypedTx::Work(WorkTx {
        tx_id: TxId(format!("worktx-{task}")),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        agent_id: AgentId(agent.into()),
        read_set: [ReadKey("k.read".into())]
            .into_iter()
            .collect::<BTreeSet<_>>(),
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

/// Fund a task and submit a self-asserted-true WorkTx; return whether it was
/// ADMITTED. `bind_registry_root == true` sets `q.predicate_registry_root_t` to
/// the sequencer's (non-zero) empty-registry root, exercising the oracle branch.
///
/// M07 G3 (2026-06-07; §8 token APPROVE-M07-G3-OS-QUALIFIED-RUN-FIELD): the run
/// is marked OS-qualified via `q.os_qualified_t = true` — the run-level field the
/// shared admission contract reads (independent of `predicate_registry_root_t`,
/// so a zero-root OS-qualified run is reachable and refused). The system-only
/// `PredicateBindingActivate` accept is what flips this field on a real boot;
/// this test seeds it directly in the initial QState to exercise the refuse-path
/// without a full activation dance.
fn admits_self_asserted(task: &str, sponsor: &str, solver: &str, bind_registry_root: bool) -> bool {
    let mut q = genesis_with_balances(&[(sponsor, 100), (solver, 10)]);
    // OS-qualified run: the field that makes zero-root admission a REFUSE.
    q.os_qualified_t = true;
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
