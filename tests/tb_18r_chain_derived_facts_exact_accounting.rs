//! TB-18R R4 — exact accounting of FR-18R.4 v2 six fields.
//!
//! Asserts `compute_run_facts_from_chain_with_invariant()` populates
//! `expected_completed_attempts`, `l4_work_attempt_count`,
//! `l4e_work_attempt_count`, `attempt_aborted_count`, `delta`, and
//! `terminal_halt_class` exactly per charter §1.2 FR-18R.4 v2.
//!
//! See `handover/ai-direct/TB-18R_R4_STEP_B_invariant.md` §5 test plan.

use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bus::{BusConfig, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::runtime::adapter::{make_real_worktx_signed_by, make_synthetic_task_open};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::attempt_telemetry::{
    write_terminal_abort_record_to_cas, AbortCause, TerminalAbortRecord,
};
use turingosv4::runtime::chain_derived_run_facts::{
    compute_run_facts_from_chain_with_invariant, AttemptCountInvariantInputs,
};
use turingosv4::runtime::proposal_telemetry::{
    write_to_cas as write_telemetry, ProposalTelemetry, TokenCounts,
};
use turingosv4::runtime::{build_chaintape_sequencer, RuntimeChaintapeConfig};
use turingosv4::state::q_state::{AgentId, Hash, TxId};
use turingosv4::state::typed_tx::RunOutcome;

fn fresh_config(tmp: &TempDir, run_id: &str) -> RuntimeChaintapeConfig {
    RuntimeChaintapeConfig {
        runtime_repo_path: tmp.path().join("runtime_repo"),
        cas_path: tmp.path().join("cas"),
        run_id: run_id.to_string(),
        queue_capacity: 32,
        resume_existing_chain: false,
    }
}

/// FR-18R.4 v2: bootstrap → submit a TaskOpen + N zero-stake WorkTx →
/// shutdown → compute facts with invariant inputs → assert all 6 fields
/// populate correctly.
#[tokio::test]
async fn compute_with_invariant_populates_all_six_fields() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "tb18r-r4-acct");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    let bus = TuringBus::with_sequencer(
        Kernel::new(),
        BusConfig::default(),
        bundle.sequencer.clone(),
    );

    // 1× synthetic TaskOpen → L4 (NOT a Work entry; doesn't enter
    // l4_work_attempt_count).
    let task_open = make_synthetic_task_open(
        "task-r4-acct",
        "tb18r-r4-sponsor",
        Hash::ZERO,
        "r4-seed",
    );
    bus.submit_typed_tx(task_open).await.expect("TaskOpen submit");

    let mut reg = AgentKeypairRegistry::open(&cfg.runtime_repo_path).expect("open keypairs");
    let mut cas_store = CasStore::open(&cfg.cas_path).expect("open cas");

    // 3× zero-stake WorkTx (real-signature + ProposalTelemetry-linked) →
    // L4.E (zero-stake admission rejection). Each lands as a WorkTx in
    // L4.E with tx_kind=Work, populating l4e_work_attempt_count=3.
    let agents = ["agent_a", "agent_b", "agent_c"];
    for (idx, agent) in agents.iter().enumerate() {
        let pt = ProposalTelemetry::new_root(
            AgentId(agent.to_string()),
            Hash([0xaa + idx as u8; 32]),
            Cid([0xbb + idx as u8; 32]),
            "nlinarith".into(),
            TokenCounts {
                prompt_tokens: 100,
                completion_tokens: 50,
                tool_tokens: 0,
            },
            format!("{agent}.b{idx}"),
        );
        let tel_cid =
            write_telemetry(&mut cas_store, &pt, "tb18r-r4-acct", 1).expect("write telemetry");
        let work_tx = make_real_worktx_signed_by(
            &mut reg,
            "task-r4-acct",
            agent,
            Hash::ZERO,
            0, // zero stake → routes to L4.E
            &format!("p{idx}"),
            tel_cid,
            true,
            (idx + 1) as u64,
        )
        .expect("real WorkTx");
        bus.submit_typed_tx(work_tx).await.expect("WorkTx submit");
    }

    bundle.shutdown().await.expect("drain");

    // Inputs: evaluator reports 3 completed (matching the 3 L4.E Work
    // entries); halt class = MaxTxExhausted (clean halt; no aborted).
    let inputs = AttemptCountInvariantInputs {
        expected_completed_attempts: 3,
        terminal_halt_class: RunOutcome::MaxTxExhausted,
    };
    let facts = compute_run_facts_from_chain_with_invariant(
        &cfg.runtime_repo_path,
        &cfg.cas_path,
        inputs,
    )
    .expect("compute facts");

    assert_eq!(facts.expected_completed_attempts, 3);
    assert_eq!(facts.l4_work_attempt_count, 0, "no Work tx accepted on L4 (zero-stake)");
    assert_eq!(facts.l4e_work_attempt_count, 3, "3 zero-stake Work tx → L4.E");
    assert_eq!(facts.attempt_aborted_count, 0, "no TerminalAbortRecord written");
    assert_eq!(facts.delta, 0, "0 + 3 - 3 = 0 (clean halt)");
    assert_eq!(facts.terminal_halt_class, RunOutcome::MaxTxExhausted);
}

/// FR-18R.4 v2 sanity: legacy `proposal_count` (TB-7.5 fix #2 union of
/// L4 + L4.E Work) equals `l4_work_attempt_count + l4e_work_attempt_count`.
#[tokio::test]
async fn l4_l4e_split_count_matches_chain_walk() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "tb18r-r4-split");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    let bus = TuringBus::with_sequencer(
        Kernel::new(),
        BusConfig::default(),
        bundle.sequencer.clone(),
    );

    let task_open = make_synthetic_task_open(
        "task-r4-split",
        "tb18r-r4-sponsor",
        Hash::ZERO,
        "split-seed",
    );
    bus.submit_typed_tx(task_open).await.expect("TaskOpen submit");

    let mut reg = AgentKeypairRegistry::open(&cfg.runtime_repo_path).expect("open keypairs");
    let mut cas_store = CasStore::open(&cfg.cas_path).expect("open cas");
    for idx in 0..5 {
        let agent = format!("agent_{idx}");
        let pt = ProposalTelemetry::new_root(
            AgentId(agent.clone()),
            Hash([0xcc + idx as u8; 32]),
            Cid([0xdd + idx as u8; 32]),
            "rfl".into(),
            TokenCounts::default(),
            format!("{agent}.b{idx}"),
        );
        let tel_cid =
            write_telemetry(&mut cas_store, &pt, "tb18r-r4-split", 1).expect("write telemetry");
        let work_tx = make_real_worktx_signed_by(
            &mut reg,
            "task-r4-split",
            &agent,
            Hash::ZERO,
            0,
            &format!("s{idx}"),
            tel_cid,
            true,
            (idx + 1) as u64,
        )
        .expect("real WorkTx");
        bus.submit_typed_tx(work_tx).await.expect("WorkTx submit");
    }
    bundle.shutdown().await.expect("drain");

    let inputs = AttemptCountInvariantInputs {
        expected_completed_attempts: 5,
        terminal_halt_class: RunOutcome::MaxTxExhausted,
    };
    let facts = compute_run_facts_from_chain_with_invariant(
        &cfg.runtime_repo_path,
        &cfg.cas_path,
        inputs,
    )
    .expect("compute facts");

    let split_sum = facts.l4_work_attempt_count + facts.l4e_work_attempt_count;
    assert_eq!(
        split_sum, facts.proposal_count,
        "l4 + l4e Work count must equal legacy proposal_count (TB-7.5 fix #2 union)"
    );
    assert!(facts.l4e_work_attempt_count >= 5, "≥5 zero-stake L4.E Work entries");
}

/// FR-18R.4 v2: pre-write N TerminalAbortRecord CAS objects → assert
/// `attempt_aborted_count == N` after `compute_run_facts_from_chain_with_invariant`.
#[tokio::test]
async fn terminal_abort_record_count_from_cas_index() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "tb18r-r4-aborted");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");

    // Pre-write 4 TerminalAbortRecord CAS objects directly via a separate
    // CasStore handle (mirroring the evaluator-side write pattern; the
    // handle CasStore::open at compute-facts time will load the durable
    // sidecar fresh, so split-brain is not a concern here per R3.fix
    // pattern).
    {
        let mut cas = CasStore::open(&cfg.cas_path).expect("open cas");
        for i in 0..4 {
            let record = TerminalAbortRecord {
                attempt_id: TxId(format!("aborted-{i}")),
                run_id: "tb18r-r4-aborted".into(),
                cause: AbortCause::WallClockCapDuringLean,
                aborted_at_logical_t: i,
                partial_lean_result_cid: None,
            };
            write_terminal_abort_record_to_cas(&mut cas, &record, "test", i)
                .expect("write abort record");
        }
    }

    bundle.shutdown().await.expect("drain");

    // For abort-halt class: expected=0 + aborted=4, l4=0 + l4e=0
    // → 0+4 != 0+0 → invariant fails (which is correct because we didn't
    // wire the L4.E records for the aborted attempts in this synthetic
    // test). The test below specifically asserts the COUNT field, not the
    // invariant pass.
    let inputs = AttemptCountInvariantInputs {
        expected_completed_attempts: 0,
        terminal_halt_class: RunOutcome::WallClockCap,
    };
    let facts = compute_run_facts_from_chain_with_invariant(
        &cfg.runtime_repo_path,
        &cfg.cas_path,
        inputs,
    )
    .expect("compute facts");

    assert_eq!(
        facts.attempt_aborted_count, 4,
        "4 TerminalAbortRecord CAS objects → attempt_aborted_count=4"
    );
}
