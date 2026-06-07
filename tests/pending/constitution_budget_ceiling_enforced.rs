//! PENDING GATE (G4 / M07) — Art. V.2 budget ceiling is admission-enforced.
//!
//! STATUS: **STANDING PENDING** — NOT a "fix is coming" red. This gate is gated
//! on a USER §8 DECISION, not just on the M07 single-admission work:
//!
//!   PROMOTION REQUIRES A §8 RULING ON:
//!     (1) whether the Art. V.2 numbers ("系统的总算力消耗不得超过 10000",
//!         constitution.md:796; "必须在 24 小时内给出结果", line 797) are HARD
//!         admission ceilings or illustrative "可能的宪法级约束" examples (the
//!         section is literally headed "下面给出一些可能的宪法级约束" — possible
//!         constraints), AND
//!     (2) the requirement that the concrete ceiling values MUST come from
//!         genesis / a signed manifest (Trust-Root pinned), NEVER hardcoded in
//!         src/ — per CLAUDE.md "Forbidden: hardcoded behavior parameter".
//!   Until the user rules on (1)+(2), this gate stays RED by design and is NOT
//!   auto-promoted to a `constitution_*` gate. It is a kill-condition standing
//!   pending §8, not a defect with a known fix.
//!
//! ── WHAT THIS GATE PROVES ────────────────────────────────────────────────
//! `src/state/q_state.rs:153-160` — `BudgetSnapshot::default()` sets
//! `cost_ceiling_microcoin = MicroCoin::zero()`, `wall_clock_remaining_ms = 0`,
//! `compute_cap_remaining = 0`. A genesis `QState` therefore carries a budget
//! snapshot with ZERO remaining compute / wall-clock / cost headroom. Yet NO
//! admission gate in `src/state/sequencer.rs` ever compares `budget_state_t`
//! against any ceiling (grep `budget_state_t` / `compute_cap_remaining` /
//! `cost_ceiling_microcoin` in sequencer.rs → 0 hits). So a `WorkTx` is admitted
//! with zero remaining budget, and an over-budget run is never routed to L4.E
//! rejection. The budget snapshot is a lazy, never-checked field.
//!
//! ── HOW THE GATE OBSERVES IT (public behavior only) ──────────────────────
//! We construct a genesis `QState` and explicitly drain its budget to a
//! definitively-OVER-budget fixture (all three remaining fields = 0 — i.e. no
//! compute, no wall-clock, no cost headroom left), then submit a normal,
//! otherwise-admissible `WorkTx`. The DESIRED post-§8 invariant is that an over-
//! budget run is REJECTED at admission. Today the WorkTx is ADMITTED (no budget
//! gate exists), so the "over-budget run rejected" assertion FAILS (expected-
//! red), cleanly proving the missing ceiling enforcement.
//!
//! NOTE on integer-only math: budget comparisons live on the money/compute path
//! and MUST be integer-only (`MicroCoin` / `u64`) — no `f64`. This fixture uses
//! only integer fields, and the eventual enforcement must too.
//!
//! ── EXCLUSION MECHANISM (same as G1/G2/G3) ───────────────────────────────
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
}

fn fresh_seq(initial_q: QState) -> SeqHarness {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("keypair"));
    let writer: Arc<RwLock<dyn LedgerWriter>> = Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let rejections = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
    let preds = Arc::new(
        PredicateRegistry::from_boot_manifest(BootPredicateManifest::empty()).expect("empty reg"),
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

/// Genesis QState with funded agents AND a definitively over-budget snapshot:
/// zero remaining compute cap, zero wall-clock, zero cost ceiling headroom.
fn over_budget_genesis(pairs: &[(&str, i64)]) -> QState {
    let mut q = QState::genesis();
    for (name, coin) in pairs {
        q.economic_state_t
            .balances_t
            .0
            .insert(AgentId((*name).into()), MicroCoin::from_coin(*coin).unwrap());
    }
    // Explicit over-budget fixture: no remaining headroom on any axis. (This is
    // also exactly the genesis default — see q_state.rs:153-160 — which is the
    // point: a genesis run is already "out of budget" yet admits work.)
    q.budget_state_t.cost_ceiling_microcoin = MicroCoin::zero();
    q.budget_state_t.wall_clock_remaining_ms = 0;
    q.budget_state_t.compute_cap_remaining = 0;
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

fn make_worktx(task: &str, agent: &str, parent: Hash) -> TypedTx {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("acc1".into()),
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

/// G4 — an over-budget run (Art. V.2 ceiling exhausted) must be routed to
/// rejection at admission.
///
/// EXPECTED RESULT AT PRE-§8: **RED (STANDING)**. With `budget_state_t` drained
/// to zero remaining on every axis, an otherwise-admissible `WorkTx` is still
/// ADMITTED, because no sequencer admission gate compares `budget_state_t`
/// against any Art. V.2 ceiling. Promotion to a real gate requires a USER §8
/// ruling that the Art. V.2 numbers are hard admission ceilings AND that the
/// concrete values come from genesis/manifest (never hardcoded). See the top
/// comment.
#[test]
fn m07_over_budget_run_must_be_rejected_at_admission() {
    let mut h = fresh_seq(over_budget_genesis(&[("sponsor-g4", 100), ("solver-g4", 10)]));
    let task = "task-g4";

    // Fund the task so the only remaining gate could be the budget ceiling.
    let pre = h.seq.q_snapshot().expect("pre snap").state_root_t;
    let open = make_task_open(task, "sponsor-g4", pre);
    block_on(h.seq.submit(open)).expect("open submit");
    let _ = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("open env")
        .expect("open accepted");
    let parent = h.seq.q_snapshot().expect("post-open").state_root_t;
    let lock = make_escrow_lock(task, "sponsor-g4", 50 * 1_000_000, parent);
    block_on(h.seq.submit(lock)).expect("lock submit");
    let _ = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("lock env")
        .expect("lock accepted");
    let parent = h.seq.q_snapshot().expect("post-lock").state_root_t;

    // Sanity: confirm the snapshot really is over-budget on every axis.
    let q = h.seq.q_snapshot().expect("snap");
    assert_eq!(
        q.budget_state_t.compute_cap_remaining, 0,
        "G4 fixture precondition: compute cap must be exhausted (0 remaining)."
    );
    assert_eq!(
        q.budget_state_t.wall_clock_remaining_ms, 0,
        "G4 fixture precondition: wall-clock must be exhausted (0 remaining)."
    );
    assert_eq!(
        q.budget_state_t.cost_ceiling_microcoin,
        MicroCoin::zero(),
        "G4 fixture precondition: cost ceiling headroom must be exhausted."
    );

    let work = make_worktx(task, "solver-g4", parent);
    block_on(h.seq.submit(work)).expect("work submit");
    let outcome = h.seq.try_apply_one(&mut h.rx).expect("work env");

    // DESIRED post-§8 invariant: an over-budget run is REJECTED. RED today
    // because no budget admission gate exists, so the WorkTx is admitted.
    assert!(
        outcome.is_err(),
        "M07 BUDGET CEILING UNENFORCED (PENDING / STANDING / EXPECTED-RED): a \
         WorkTx was ADMITTED under a budget snapshot with ZERO remaining compute \
         cap, ZERO wall-clock, and ZERO cost-ceiling headroom. \
         src/state/q_state.rs:153-160 defaults the budget snapshot to all-zero \
         and NO sequencer admission gate compares budget_state_t against any \
         Art. V.2 ceiling (constitution.md:796-797). The desired invariant is \
         that an over-budget run is routed to rejection at admission. PROMOTION \
         to a real constitution gate requires a USER §8 ruling: (1) are the \
         Art. V.2 numbers hard ceilings or illustrative examples, and (2) the \
         concrete values must come from genesis/manifest, never hardcoded \
         (CLAUDE.md forbids hardcoded behavior parameters). Until that §8 \
         decision, this gate stays RED by design (standing pending), got: {outcome:?}"
    );
}
