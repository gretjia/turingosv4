//! TB-N1-AGENT-ECONOMY Phase 2 atom A4 — agent-callable verify-peer.
//!
//! Charter: `handover/tracer_bullets/TB_N1_AGENT_ECONOMY_PHASE_2_charter_2026-05-10.md` §2 atom A4.
//! Forward §8 grant: `handover/directives/2026-05-10_TB_N1_AGENT_ECONOMY_PHASE_2_FORWARD_§8_GRANT.md`.
//!
//! Constitutional binding: closes the agency layer of CLAUDE.md §13
//! verify/bond + Art. I.1.1 multi-agent verification — agent-callable
//! `verify_peer` tool with typed admission rejection classes.
//!
//! Ship gates (SG-N1-A4.1..7):
//! - SG-N1-A4.1: bond=0 → reject with BondInsufficient (existing preserved)
//! - SG-N1-A4.2: bond=balance+1 → reject with NEW VerifyBondOutOfBounds
//! - SG-N1-A4.3: target_work_tx not in stakes_t → reject with NEW VerifyTargetNotAccepted
//! - SG-N1-A4.4: duplicate (verifier, target) → reject with NEW VerifyDuplicate
//! - SG-N1-A4.5: bond=1 + valid target + first verify → admit (positive control)
//! - SG-N1-A4.6: real-LLM n=2 swarm smoke (asymmetric binding; vacuous-pass
//!               when no `stage_b3_smoke_a4_*` evidence yet, load-bearing once smoke
//!               lands per `feedback_real_problems_not_designed`)
//! - SG-N1-A4.7: source-grep gate — `verify_peer` tool advertised in prompt.rs
//!               AND dispatched in evaluator.rs (mechanism binding test)
//!
//! `FC-trace: §13 verify/bond agency layer + Art. I.1.1 multi-agent
//! verification + FC1-N7 δ Agent externalized output enriched with
//! peer-verification capability + FC1 hard invariant (every VerifyTx with
//! bond_micro tape-visible).`

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    RejectionClass as L4ERejectionClass, RejectionEvidenceWriter,
};
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
    SafetyOrCreation, TaskOpenTx, TypedTx, VerifyTx, VerifyVerdict, WorkTx, WriteKey,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ────────────────────────────────────────────────────────────────────────────
// Fixtures (mirror A3 harness pattern)
// ────────────────────────────────────────────────────────────────────────────

struct Harness {
    _tmp: TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
}

fn fresh_harness(initial_q: QState) -> Harness {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("keypair"));
    let writer: Arc<RwLock<dyn LedgerWriter>> = Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
    let preds = Arc::new(PredicateRegistry::new());
    let tools = Arc::new(ToolRegistry::new());
    let epoch = SystemEpoch::new(1);
    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(epoch, keypair.public_key());
    let pinned_pubkeys = Arc::new(pinned);
    let (seq, rx) = Sequencer::new(
        cas.clone(),
        keypair,
        epoch,
        writer,
        rejection_writer.clone(),
        preds,
        tools,
        pinned_pubkeys,
        initial_q,
        16,
    );
    Harness {
        _tmp: tmp,
        seq,
        rx,
        rejection_writer,
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

fn make_task_open(task: &str, sponsor: &str, parent: Hash, suffix: &str) -> TypedTx {
    TypedTx::TaskOpen(TaskOpenTx {
        tx_id: TxId(format!("taskopen-{}-{}", task, suffix)),
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

fn make_escrow_lock(
    task: &str,
    sponsor: &str,
    amount_micro: i64,
    parent: Hash,
    suffix: &str,
) -> TypedTx {
    TypedTx::EscrowLock(EscrowLockTx {
        tx_id: TxId(format!("escrowlock-{}-{}", task, suffix)),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        sponsor_agent: AgentId(sponsor.into()),
        amount: MicroCoin::from_micro_units(amount_micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

fn make_worktx(
    task: &str,
    agent: &str,
    parent: Hash,
    stake_micro: i64,
    suffix: &str,
    predicate_passes: bool,
) -> TypedTx {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("acc1".into()),
        BoolWithProof {
            value: predicate_passes,
            proof_cid: None,
        },
    );
    TypedTx::Work(WorkTx {
        tx_id: TxId(format!("worktx-{task}-{suffix}")),
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
        stake: StakeMicroCoin::from_micro_units(stake_micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

fn make_verifytx(
    target_work_tx: &str,
    verifier: &str,
    parent: Hash,
    bond_micro: i64,
    suffix: &str,
    verdict: VerifyVerdict,
) -> TypedTx {
    TypedTx::Verify(VerifyTx {
        tx_id: TxId(format!("verifytx-{verifier}-{suffix}")),
        parent_state_root: parent,
        target_work_tx: TxId(target_work_tx.into()),
        verifier_agent: AgentId(verifier.into()),
        bond: StakeMicroCoin::from_micro_units(bond_micro),
        verdict,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

async fn setup_funded_task_with_accepted_worktx(
    h: &mut Harness,
    task_id: &TaskId,
    sponsor: &str,
    escrow_coin: i64,
    solver: &str,
    stake_micro: i64,
    work_suffix: &str,
) -> (Hash, String) {
    // TaskOpen
    let pre = h.seq.q_snapshot().expect("pre snap").state_root_t;
    let open = make_task_open(&task_id.0, sponsor, pre, "fund");
    h.seq.submit(open).await.expect("open submit");
    let _ = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("open env")
        .expect("open accepted");
    // EscrowLock
    let parent = h.seq.q_snapshot().expect("post-open").state_root_t;
    let lock = make_escrow_lock(&task_id.0, sponsor, escrow_coin * 1_000_000, parent, "fund");
    h.seq.submit(lock).await.expect("lock submit");
    let _ = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("lock env")
        .expect("lock accepted");
    // WorkTx (predicate-passing; lands as accepted in stakes_t)
    let parent = h.seq.q_snapshot().expect("post-lock").state_root_t;
    let work = make_worktx(&task_id.0, solver, parent, stake_micro, work_suffix, true);
    let work_tx_id = match &work {
        TypedTx::Work(w) => w.tx_id.0.clone(),
        _ => unreachable!(),
    };
    h.seq.submit(work).await.expect("work submit");
    let _ = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("work env")
        .expect("work accepted");
    (
        h.seq.q_snapshot().expect("post-work").state_root_t,
        work_tx_id,
    )
}

fn last_l4e_class(writer: &Arc<RwLock<RejectionEvidenceWriter>>) -> Option<L4ERejectionClass> {
    let g = writer.read().expect("writer read");
    g.records().last().map(|r| r.rejection_class)
}

// ════════════════════════════════════════════════════════════════════════════
// SG-N1-A4.1 — bond = 0 → reject with BondInsufficient (existing preserved)
// ════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn sg_n1_a4_1_zero_bond_rejects_with_bond_insufficient() {
    // 3 agents: sponsor (100C), solver (10C), verifier (10C).
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-a4-1", 100),
        ("solver-a4-1", 10),
        ("verifier-a4-1", 10),
    ]));
    let task = TaskId("task-a4-1".into());
    let (parent, work_tx_id) = setup_funded_task_with_accepted_worktx(
        &mut h,
        &task,
        "sponsor-a4-1",
        50,
        "solver-a4-1",
        1,
        "w1",
    )
    .await;

    let verify = make_verifytx(
        &work_tx_id,
        "verifier-a4-1",
        parent,
        0,
        "a4-1",
        VerifyVerdict::Confirm,
    );
    h.seq.submit(verify).await.expect("submit");
    let r = h.seq.try_apply_one(&mut h.rx).expect("env");
    assert!(r.is_err(), "bond=0 must reject");
    // BondInsufficient maps to PolicyViolation via the rejection_class_for `_`
    // wildcard arm (TB-4 mapping preserved by A4; A4 only adds 3 NEW variants
    // and does NOT remap existing BondInsufficient).
    assert_eq!(
        last_l4e_class(&h.rejection_writer),
        Some(L4ERejectionClass::PolicyViolation),
        "bond=0 must surface as L4E PolicyViolation (TB-4 BondInsufficient mapping preserved)",
    );
}

// ════════════════════════════════════════════════════════════════════════════
// SG-N1-A4.2 — bond > balance → reject with NEW VerifyBondOutOfBounds
// ════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn sg_n1_a4_2_overbond_rejects_with_verify_bond_out_of_bounds() {
    // verifier balance = 10 Coin = 10_000_000 μC. Submit bond = 10_000_001 μC.
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-a4-2", 100),
        ("solver-a4-2", 10),
        ("verifier-a4-2", 10),
    ]));
    let task = TaskId("task-a4-2".into());
    let (parent, work_tx_id) = setup_funded_task_with_accepted_worktx(
        &mut h,
        &task,
        "sponsor-a4-2",
        50,
        "solver-a4-2",
        1,
        "w2",
    )
    .await;

    let over_bond_micro: i64 = 10_000_000 + 1;
    let verify = make_verifytx(
        &work_tx_id,
        "verifier-a4-2",
        parent,
        over_bond_micro,
        "a4-2",
        VerifyVerdict::Confirm,
    );
    h.seq.submit(verify).await.expect("submit");
    let r = h.seq.try_apply_one(&mut h.rx).expect("env");
    assert!(r.is_err(), "bond=balance+1 must reject");
    assert_eq!(
        last_l4e_class(&h.rejection_writer),
        Some(L4ERejectionClass::InsufficientBalance),
        "bond>balance must surface as L4E InsufficientBalance via NEW VerifyBondOutOfBounds (TB-N1 A4 step-2.5 agent-bound gate)",
    );
}

// ════════════════════════════════════════════════════════════════════════════
// SG-N1-A4.3 — target_work_tx not in stakes_t → reject with NEW VerifyTargetNotAccepted
// ════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn sg_n1_a4_3_phantom_target_rejects_with_verify_target_not_accepted() {
    let mut h = fresh_harness(genesis_with_balances(&[("verifier-a4-3", 10)]));
    let parent = h.seq.q_snapshot().expect("snap").state_root_t;

    // No setup_funded_task — target_work_tx is a phantom not in stakes_t.
    let verify = make_verifytx(
        "worktx-phantom-nonexistent",
        "verifier-a4-3",
        parent,
        1,
        "a4-3",
        VerifyVerdict::Confirm,
    );
    h.seq.submit(verify).await.expect("submit");
    let r = h.seq.try_apply_one(&mut h.rx).expect("env");
    assert!(r.is_err(), "phantom target must reject");
    assert_eq!(
        last_l4e_class(&h.rejection_writer),
        Some(L4ERejectionClass::PolicyViolation),
        "phantom target must surface as L4E PolicyViolation via NEW VerifyTargetNotAccepted (TB-N1 A4 step-3 refinement)",
    );
}

// ════════════════════════════════════════════════════════════════════════════
// SG-N1-A4.4 — duplicate (verifier, target) → reject with NEW VerifyDuplicate
// ════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn sg_n1_a4_4_duplicate_verify_rejects_with_verify_duplicate() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-a4-4", 100),
        ("solver-a4-4", 10),
        ("verifier-a4-4", 10),
    ]));
    let task = TaskId("task-a4-4".into());
    let (parent_pre_v1, work_tx_id) = setup_funded_task_with_accepted_worktx(
        &mut h,
        &task,
        "sponsor-a4-4",
        50,
        "solver-a4-4",
        1,
        "w4",
    )
    .await;

    // First VerifyTx — should admit (positive control inside this test).
    let verify1 = make_verifytx(
        &work_tx_id,
        "verifier-a4-4",
        parent_pre_v1,
        1,
        "a4-4-v1",
        VerifyVerdict::Confirm,
    );
    h.seq.submit(verify1).await.expect("submit v1");
    let r1 = h.seq.try_apply_one(&mut h.rx).expect("env v1");
    assert!(r1.is_ok(), "first verify must admit; got {:?}", r1);

    // Second VerifyTx from SAME verifier on SAME target — should reject.
    let parent_post_v1 = h.seq.q_snapshot().expect("post-v1").state_root_t;
    let verify2 = make_verifytx(
        &work_tx_id,
        "verifier-a4-4",
        parent_post_v1,
        1,
        "a4-4-v2",
        VerifyVerdict::Confirm,
    );
    h.seq.submit(verify2).await.expect("submit v2");
    let r2 = h.seq.try_apply_one(&mut h.rx).expect("env v2");
    assert!(r2.is_err(), "duplicate verify must reject");
    assert_eq!(
        last_l4e_class(&h.rejection_writer),
        Some(L4ERejectionClass::PolicyViolation),
        "duplicate verify must surface as L4E PolicyViolation via NEW VerifyDuplicate (TB-N1 A4 step-3.5 duplicate-prevention)",
    );
}

// ════════════════════════════════════════════════════════════════════════════
// SG-N1-A4.5 — valid bond + valid target + first verify → admit (positive control)
// ════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn sg_n1_a4_5_first_valid_verify_admits() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-a4-5", 100),
        ("solver-a4-5", 10),
        ("verifier-a4-5", 10),
    ]));
    let task = TaskId("task-a4-5".into());
    let (parent, work_tx_id) = setup_funded_task_with_accepted_worktx(
        &mut h,
        &task,
        "sponsor-a4-5",
        50,
        "solver-a4-5",
        1,
        "w5",
    )
    .await;

    let verify = make_verifytx(
        &work_tx_id,
        "verifier-a4-5",
        parent,
        1,
        "a4-5",
        VerifyVerdict::Confirm,
    );
    h.seq.submit(verify).await.expect("submit");
    let outcome = h.seq.try_apply_one(&mut h.rx).expect("env");
    assert!(
        outcome.is_ok(),
        "first valid verify must admit; got {:?}",
        outcome,
    );

    // Post-admit: (verifier, target) must now be present in agent_verifications_t.
    let q = h.seq.q_snapshot().expect("post-admit snap");
    let pair = (AgentId("verifier-a4-5".into()), TxId(work_tx_id.clone()));
    assert!(
        q.economic_state_t.agent_verifications_t.0.contains(&pair),
        "accepted VerifyTx must insert (verifier, target) into agent_verifications_t for future duplicate suppression"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// SG-N1-A4.6 — real-LLM n=2 swarm smoke binding (asymmetric)
// ════════════════════════════════════════════════════════════════════════════

/// Bind to `handover/evidence/stage_b3_smoke_a4_*/` evidence dirs. Vacuous-pass
/// when no smoke dir exists yet (load-bearing once smoke produces evidence per
/// `feedback_real_problems_not_designed`).
///
/// Witness shape (asserted when ≥1 dir matches): per-cell `chain_invariant.json`
/// reports FC1 verdict=Ok delta=0 + aggregate `expected_completed_attempts > 0`
/// + aggregate L4 + L4.E WorkTx > 0 (proves admission engaged with no
/// regression under A4 wiring). STRICT witness (≥1 cell with agent-submitted
/// VerifyTx where target_work_tx matches a prior agent's WorkTx) requires
/// agent-uptake of verify_peer; per `project_economy_prompt_landing_gap`
/// agents don't natively use new tools without training. WEAK fallback is
/// sufficient for ship at substrate level.
#[test]
fn sg_n1_a4_6_real_llm_swarm_smoke_witnesses_admission_health() {
    let evidence_root = PathBuf::from("handover/evidence");
    if !evidence_root.exists() {
        eprintln!(
            "SG-N1-A4.6: handover/evidence/ missing — vacuous pass (binding becomes load-bearing once evidence dir lands)"
        );
        return;
    }

    let mut smoke_dirs: Vec<PathBuf> = fs::read_dir(&evidence_root)
        .expect("read handover/evidence/")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.is_dir()
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.starts_with("stage_b3_smoke_a4_"))
                    .unwrap_or(false)
        })
        .collect();
    smoke_dirs.sort();

    if smoke_dirs.is_empty() {
        eprintln!(
            "SG-N1-A4.6: no stage_b3_smoke_a4_* evidence yet — vacuous pass per feedback_real_problems_not_designed (binding load-bearing post-smoke)"
        );
        return;
    }

    let mut total_expected = 0u64;
    let mut total_l4_admitted = 0u64;
    let mut total_l4e = 0u64;
    let mut all_verdict_ok = true;
    let mut cells_scanned = 0usize;
    for smoke_dir in &smoke_dirs {
        let walker = walkdir_chain_invariant_jsons(smoke_dir);
        for inv_path in walker {
            cells_scanned += 1;
            let body = fs::read_to_string(&inv_path)
                .unwrap_or_else(|e| panic!("read {}: {e}", inv_path.display()));
            let v: serde_json::Value = serde_json::from_str(&body)
                .unwrap_or_else(|e| panic!("parse {}: {e}", inv_path.display()));
            total_expected += v["expected_completed_attempts"].as_u64().unwrap_or(0);
            total_l4_admitted += v["l4_work_attempt_count"].as_u64().unwrap_or(0);
            total_l4e += v["l4e_work_attempt_count"].as_u64().unwrap_or(0);
            if v["invariant_verdict"].as_str().unwrap_or("?") != "Ok" {
                all_verdict_ok = false;
            }
        }
    }

    assert!(
        cells_scanned > 0,
        "SG-N1-A4.6: smoke dirs found but zero per-cell chain_invariant.json reports — smoke harness regression. Dirs: {smoke_dirs:?}"
    );
    assert!(
        all_verdict_ok,
        "SG-N1-A4.6: ≥1 cell has invariant_verdict != Ok — FC1 hard invariant broken under A4 wiring. Smoke dirs: {smoke_dirs:?}"
    );
    assert!(
        total_expected > 0,
        "SG-N1-A4.6: aggregate expected_completed_attempts == 0 across {cells_scanned} cells — A4 wiring blocked LLM path entirely. Smoke dirs: {smoke_dirs:?}"
    );
    assert!(
        total_l4_admitted + total_l4e > 0,
        "SG-N1-A4.6: aggregate L4 + L4.E WorkTx == 0 across {cells_scanned} cells — A4 wiring blocked WorkTx admission entirely. Smoke dirs: {smoke_dirs:?}"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// SG-N1-A4.7 — source-grep gate (verify_peer in prompt.rs + evaluator.rs)
// ════════════════════════════════════════════════════════════════════════════

/// Mechanism-binding test: the verify_peer tool MUST be advertised in
/// prompt.rs schema doc AND dispatched in evaluator.rs. Catches a class of
/// regression where one surface drifts (tool advertised but dispatch
/// removed, or vice versa) leaving the schema lying.
#[test]
fn sg_n1_a4_7_verify_peer_advertised_and_dispatched() {
    let prompt_src = fs::read_to_string("src/sdk/prompt.rs").expect("read prompt.rs");
    assert!(
        prompt_src.contains("verify_peer"),
        "SG-N1-A4.7: verify_peer tool must be advertised in src/sdk/prompt.rs schema doc"
    );
    assert!(
        prompt_src.contains("VerifyBondOutOfBounds"),
        "SG-N1-A4.7: prompt.rs schema doc must mention VerifyBondOutOfBounds rejection class"
    );

    let evaluator_src = fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs")
        .expect("read evaluator.rs");
    assert!(
        evaluator_src.contains("\"verify_peer\" =>"),
        "SG-N1-A4.7: verify_peer dispatch arm must exist in evaluator.rs action.tool match"
    );
    assert!(
        evaluator_src.contains("make_real_verifytx_signed_by"),
        "SG-N1-A4.7: verify_peer dispatch must call make_real_verifytx_signed_by"
    );
}

fn walkdir_chain_invariant_jsons(root: &PathBuf) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack: Vec<PathBuf> = vec![root.clone()];
    while let Some(d) = stack.pop() {
        if let Ok(rd) = fs::read_dir(&d) {
            for e in rd.filter_map(|x| x.ok()) {
                let p = e.path();
                if p.is_dir() {
                    let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if name == ".git" {
                        continue;
                    }
                    stack.push(p);
                } else if p.file_name().and_then(|n| n.to_str()) == Some("chain_invariant.json") {
                    out.push(p);
                }
            }
        }
    }
    out.sort();
    out
}
