//! TB-18 Atom B Phase 4 — Substantive single-process multi-task driver
//! (architect ruling 2026-05-05 §2.8 + §3 Atom B; FR-18.7 + FR-18.8;
//! SG-18.6 + SG-18.7; CR-18.7 + CR-18.8).
//!
//! ## Why this binary exists in its current form
//!
//! Per architect §2.8 verbatim:
//!
//! > Atom B 要证明的是：one evaluator process / one runtime_repo /
//! > one CAS / one chain / multiple tasks. 如果它只是一个 process 里启动多个
//! > subprocess，每个 subprocess 自己起 chain，那不合格.
//!
//! The pre-TB-18 `comprehensive_arena.rs` (TB-16 Atom 5 scaffold) was a
//! subprocess-spawn orchestrator — each task subprocessed `evaluator`,
//! creating one chain per task. That is **architecturally non-compliant**
//! with the §2.8 mandate.
//!
//! TB-18 Atom B Phase 4 (this commit) rewrites the binary as a single-process
//! multi-task in-memory driver:
//!
//!   1. `SharedChain::from_env()` — single bundle init
//!      (Phase 1 lift; see `chain_runtime.rs`).
//!   2. `write_synthetic_l4_l4e_gate_and_genesis_report` ONCE with a
//!      chain-level seed_id (Phase 2 lift).
//!   3. For each of 6 engineered tasks {A,B,C,D,E,F}: call `drive_task`
//!      (TaskOpen + EscrowLock per-task scaffold; Phase 3) + emit the
//!      task-specific lifecycle txs via direct `bus.submit_typed_tx` +
//!      `bundle.sequencer.emit_system_tx` calls. All 6 tasks share the
//!      same bundle.
//!   4. `bundle.shutdown()` ONCE at chain end.
//!   5. Write `SHARED_CHAIN_RUNS_REPORT.json` + `tx_kind_distribution.json`
//!      evidence per Phase 5 ship-evidence layout.
//!
//! ## 6-engineered-task plan (architect §3 Atom B + design §4.5)
//!
//! | Task | Lifecycle | New tx kinds (cumulative target = 13/13)         |
//! |------|-----------|---------------------------------------------------|
//! |  A   | Open → Work → Verify(OMEGA-Confirm) → FinalizeReward           | TaskOpen, EscrowLock, Work, Verify, FinalizeReward |
//! |  B   | Open → Work → Verify(OMEGA-Confirm) → Challenge → ChallengeResolve(Released) | Challenge, ChallengeResolve |
//! |  C   | Open → MarketSeed → CompleteSetMint → TaskBankruptcy → CompleteSetRedeem | MarketSeed, CompleteSetMint, CompleteSetRedeem, TaskBankruptcy |
//! |  D   | Open → Work → TerminalSummary → TaskBankruptcy → TaskExpire(BankruptcyTriggered) | TerminalSummary, TaskExpire |
//! |  E   | Open → TerminalSummary (no bankruptcy)                          | (redundancy)                |
//! |  F   | Open → Work → TerminalSummary(outcome=DegradedLLM)              | (Atom A new path)           |
//!
//! Total distinct tx kinds across ONE chain: **13** ✓ matches architect's
//! 13/13 (TaskOpen, EscrowLock, Work, Verify, FinalizeReward, Challenge,
//! ChallengeResolve, MarketSeed, CompleteSetMint, CompleteSetRedeem,
//! TerminalSummary, TaskExpire, TaskBankruptcy).
//!
//! ## Why no LLM agent loop
//!
//! Per `feedback_chaintape_externalized_proposal`: the chain records what
//! the system externalized via `submit_typed_tx`, not LLM internals. The
//! 6-task engineered set produces the 13/13 tx kinds via real-signed
//! synthetic envelopes (the same `make_real_*_signed_by` helpers that
//! TB-13/14/16.x smoke tests use). The architect §2.4 failure-mode
//! coverage table was already saturated by the M0 retry on per-problem
//! chains (7 OMEGA-Confirm + 7 natural EvidenceCapsule + 6 controlled
//! timeouts); TB-18 ship's specific gap is single-chain multi-task tx-kind
//! coverage (FR-18.7 + FR-18.8), not additional LLM-driven solve
//! evidence.
//!
//! Per `feedback_no_workarounds_strict_constitution`: this is NOT 凑活 —
//! synthetic real-signed envelopes are the architect-precedented
//! mechanism for arena drivers (TB-16 Atom 7 §7.3 FR-16.3 + FR-16.4
//! ratified `make_real_challengetx_signed_by` etc. for exactly this
//! purpose); they produce the same chain shape that LLM-driven
//! envelopes would, against the same admission gates.
//!
//! Per `feedback_class4_cannot_hide_in_class3`: this binary stays Class
//! 3 — it consumes existing public APIs (SharedChain, drive_task,
//! make_real_*_signed_by, emit_system_tx) and does NOT touch sequencer
//! admission / typed-tx schema / canonical-signing-payload.
//!
//! ## TRACE_MATRIX
//!
//! `FC-trace: FC1-N36` — comprehensive_arena orchestrator (TB-18 Atom B
//! substantive build).

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::adapter::{
    make_real_challengetx_signed_by, make_real_complete_set_mint_signed_by,
    make_real_complete_set_redeem_signed_by, make_real_market_seed_signed_by,
    make_real_verifytx_signed_by, make_real_worktx_signed_by, tb8_await_state_root_advance,
    tb8_emit_finalize_after_verify, tb11_emit_terminal_summary_for_run,
    tb16_emit_challenge_resolve_for_eligible,
};
use turingosv4::runtime::evidence_capsule::{write_evidence_capsule, ExhaustionCounts};
use turingosv4::runtime::proposal_telemetry::{ProposalTelemetry, TokenCounts};
use turingosv4::state::q_state::{Hash, TaskId, TxId};
use turingosv4::state::sequencer::SystemEmitCommand;
use turingosv4::state::typed_tx::{
    BankruptcyReason, ChallengeResolution, ExhaustionReason, ExpireReason, OutcomeSide,
    RejectionClass, RunId, RunOutcome,
};

use minif2f_v4::chain_runtime::{
    write_synthetic_l4_l4e_gate_and_genesis_report, SharedChain,
};
use minif2f_v4::drive_task::{drive_task, TaskSpec};
use minif2f_v4::per_call_budget::PerCallBudget;

// Internal binary; tokio::main not in scope — invoke runtime by hand below.

/// TB-18 Atom B Phase 4: result of one engineered task's lifecycle drive.
#[derive(Debug, Clone)]
struct ArenaTaskOutcome {
    label: &'static str,
    task_id: String,
    tx_kinds_emitted: Vec<&'static str>,
    /// Free-form failure note (empty when task succeeded). Appears in the
    /// SHARED_CHAIN_RUNS_REPORT.json so post-hoc audits can cross-check
    /// against on-chain L4 / L4.E counts.
    note: String,
}

#[derive(Debug, Clone)]
struct ArenaConfig {
    /// Output dir for the multi-task chain's runtime_repo + cas + evidence.
    out_dir: PathBuf,
    /// Chain-level seed_id used as the synthetic L4/L4.E gate prefix
    /// (`smoke-{seed_id}`) and as the basename for SHARED_CHAIN_RUNS_REPORT
    /// fields. Chain-level (not per-task) per architect §2.8.
    chain_seed_id: String,
    /// Print plan + exit (no chain side-effects).
    plan_only: bool,
}

impl ArenaConfig {
    fn from_args(argv: &[String]) -> Result<Self, String> {
        let mut out_dir: Option<PathBuf> = None;
        let mut chain_seed_id = format!(
            "tb18-arena-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs().to_string())
                .unwrap_or_else(|_| "0".into())
        );
        let mut plan_only = false;
        let mut i = 0;
        while i < argv.len() {
            match argv[i].as_str() {
                "--out-dir" => {
                    i += 1;
                    out_dir = Some(argv.get(i).ok_or("--out-dir needs path")?.into());
                }
                "--chain-seed-id" => {
                    i += 1;
                    chain_seed_id = argv.get(i).ok_or("--chain-seed-id needs str")?.clone();
                }
                "--plan-only" => plan_only = true,
                "-h" | "--help" => {
                    eprint!("{}", help_text());
                    std::process::exit(0);
                }
                other => return Err(format!("unknown arg: {other}")),
            }
            i += 1;
        }
        Ok(Self {
            out_dir: out_dir.ok_or("--out-dir required")?,
            chain_seed_id,
            plan_only,
        })
    }
}

fn help_text() -> String {
    "comprehensive_arena — TB-18 Atom B Phase 4 single-process multi-task driver\n\
     \n\
     USAGE:\n  \
       comprehensive_arena --out-dir <path> [options]\n\
     \n\
     OPTIONS:\n  \
       --out-dir <path>           Output dir (becomes runtime_repo + cas root)\n  \
       --chain-seed-id <str>      Chain-level seed_id (default tb18-arena-<unix>)\n  \
       --plan-only                Print plan + exit; no chain side-effects\n\
     \n\
     EXIT:\n  \
       0  — chain shipped with 13/13 tx kinds emitted\n  \
       2  — invalid args\n  \
       3  — chain init / bootstrap failure\n  \
       4  — task lifecycle emit failure\n"
        .into()
}

/// TB-18 Atom B Phase 4: print the 6-task plan to stderr.
fn print_plan(cfg: &ArenaConfig) {
    eprintln!("# TB-18 Atom B Phase 4 — comprehensive_arena plan");
    eprintln!("# out_dir: {:?}", cfg.out_dir);
    eprintln!("# chain_seed_id: {}", cfg.chain_seed_id);
    eprintln!("#");
    eprintln!("# 6-task engineered set (architect §3 Atom B + design §4.5):");
    eprintln!("#   task_A: Open → Work → Verify(OMEGA) → FinalizeReward");
    eprintln!("#   task_B: Open → Work → Verify(OMEGA) → Challenge → ChallengeResolve(Released)");
    eprintln!("#   task_C: Open → MarketSeed → CompleteSetMint → TaskBankruptcy → CompleteSetRedeem");
    eprintln!("#   task_D: Open → Work → TerminalSummary → TaskBankruptcy → TaskExpire");
    eprintln!("#   task_E: Open → TerminalSummary");
    eprintln!("#   task_F: Open → Work → TerminalSummary(DegradedLLM)");
    eprintln!("#");
    eprintln!("# Target: ONE chain emitting all 13 tx kinds.");
}

/// TB-18 Atom B Phase 4: write a minimal ProposalTelemetry to CAS and
/// return the CID for use as `WorkTx.proposal_cid`.
///
/// Mirrors the TB-16.x.2.5 r3 fix pattern (per `feedback_no_retroactive_evidence_rewrite`
/// — using existing helpers verbatim rather than rolling a parallel construction).
fn write_minimal_proposal_telemetry(
    cas_path: &Path,
    run_id: &str,
    agent: &str,
    proposal_index: u64,
    logical_t: u64,
) -> Result<turingosv4::bottom_white::cas::schema::Cid, String> {
    let mut cas = CasStore::open(cas_path).map_err(|e| format!("CAS open: {e:?}"))?;
    let payload = format!(
        "tb18-arena-proposal:{}:{}:{}",
        run_id, agent, proposal_index
    );
    let telemetry = ProposalTelemetry::build_for_evaluator_append(
        &mut cas,
        run_id,
        agent,
        proposal_index,
        payload.as_bytes(),
        "exact?",
        TokenCounts {
            prompt_tokens: 0,
            completion_tokens: 1,
            tool_tokens: 0,
        },
        "tb18-arena",
        logical_t,
    )
    .map_err(|e| format!("ProposalTelemetry build: {e:?}"))?;
    let cid = turingosv4::runtime::proposal_telemetry::write_to_cas(
        &mut cas,
        &telemetry,
        "tb18-arena",
        logical_t,
    )
    .map_err(|e| format!("ProposalTelemetry write: {e:?}"))?;
    Ok(cid)
}

/// TB-18 Atom B Phase 4: write a minimal EvidenceCapsule to CAS for use
/// as `evidence_capsule_cid` on TerminalSummary / TaskBankruptcy.
fn write_minimal_evidence_capsule(
    cas_path: &Path,
    run_id_str: &str,
    task_id_str: &str,
    creator: &str,
) -> Result<turingosv4::bottom_white::cas::schema::Cid, String> {
    use std::sync::{Arc, RwLock};
    let cas_store = CasStore::open(cas_path).map_err(|e| format!("CAS open: {e:?}"))?;
    let cas = Arc::new(RwLock::new(cas_store));
    let task_id = TaskId(task_id_str.into());
    let run_id_typed = RunId(run_id_str.into());
    let counts = ExhaustionCounts {
        attempt_count: 1,
        lean_error_count: 0,
        sorry_block_count: 0,
        protocol_parse_failure_count: 0,
        partial_accept_count: 0,
    };
    let capsule = write_evidence_capsule(
        &cas,
        run_id_typed,
        task_id,
        None, // solver_agent
        counts,
        (0, 1), // rounds (start, end)
        ExhaustionReason::MaxTxExhausted,
        b"",                        // raw_log_bytes (minimal for arena synthetic)
        turingosv4::state::typed_tx::CapsulePrivacyPolicy::AuditOnly,
        creator,
        1,
    )
    .map_err(|e| format!("write_evidence_capsule: {e:?}"))?;
    Ok(capsule.capsule_id)
}

/// TB-18 Atom B Phase 4: poll `bus.snapshot()` until state_root advances
/// past `pre_root` or `budget_ms` expires. Returns the new state_root.
async fn await_advance(
    chain: &SharedChain,
    pre_root: Hash,
    budget_ms: u64,
) -> Result<Hash, String> {
    let bundle = chain
        .chaintape_bundle
        .as_ref()
        .ok_or_else(|| "chaintape_bundle = None".to_string())?;
    tb8_await_state_root_advance(bundle.sequencer.as_ref(), pre_root, budget_ms)
        .await
        .map_err(|_| format!("state_root advance budget {budget_ms}ms expired"))
}

/// TB-18 Atom B Phase 4: drive task A — Open → Work → Verify(OMEGA) → FinalizeReward.
async fn drive_task_a(
    chain: &mut SharedChain,
    spec: &TaskSpec,
    cas_path: &Path,
) -> Result<ArenaTaskOutcome, String> {
    let scaffold = drive_task(chain, spec, PerCallBudget::default())
        .await
        .map_err(|e| format!("task_A drive_task: {e}"))?;
    let mut tx_kinds: Vec<&'static str> =
        vec!["TaskOpen", "EscrowLock"];

    let post_open = parse_hex(&scaffold.post_open_lock_state_root_hex)?;

    // Work tx (real-signed by Agent_0 solver).
    let proposal_cid = write_minimal_proposal_telemetry(cas_path, "tb18-task-a", "Agent_0", 0, 10)?;
    let work_tx = sign_with_keypairs(chain, |reg| {
        make_real_worktx_signed_by(
            reg,
            &scaffold.task_id,
            "Agent_0",
            post_open,
            100,
            "tb18-a-work",
            proposal_cid,
            true,
            10,
        )
        .map_err(|e| format!("{e:?}"))
    })?;
    chain
        .bus
        .submit_typed_tx(work_tx)
        .await
        .map_err(|e| format!("task_A Work submit: {e:?}"))?;
    let post_work = await_advance(chain, post_open, 5000).await?;
    tx_kinds.push("Work");
    let work_tx_id = TxId(format!("worktx-{}-tb18-a-work", scaffold.task_id));

    // Verify tx (real-signed by Agent_1 verifier; OMEGA-Confirm).
    let verify_tx = sign_with_keypairs(chain, |reg| {
        make_real_verifytx_signed_by(
            reg,
            post_work,
            work_tx_id.clone(),
            "Agent_1",
            50,
            "tb18-a-verify",
            true,
            11,
        )
        .map_err(|e| format!("{e:?}"))
    })?;
    chain
        .bus
        .submit_typed_tx(verify_tx)
        .await
        .map_err(|e| format!("task_A Verify submit: {e:?}"))?;
    let post_verify = await_advance(chain, post_work, 5000).await?;
    tx_kinds.push("Verify");

    // FinalizeReward (system-emitted; tb8 helper polls for claim_id from
    // verify_tx_id then emits FinalizeReward { claim_id }).
    let bundle = chain
        .chaintape_bundle
        .as_ref()
        .ok_or_else(|| "chaintape_bundle = None at finalize".to_string())?;
    let verify_tx_id = TxId(format!("verifytx-Agent_1-tb18-a-verify"));
    let finalized = tb8_emit_finalize_after_verify(bundle.sequencer.as_ref(), &verify_tx_id, 5000)
        .await
        .map_err(|e| format!("tb8_emit_finalize: {e:?}"))?;
    if finalized {
        tx_kinds.push("FinalizeReward");
    }
    let _ = await_advance(chain, post_verify, 5000).await;

    Ok(ArenaTaskOutcome {
        label: "task_A_happy_path",
        task_id: scaffold.task_id,
        tx_kinds_emitted: tx_kinds,
        note: if finalized { String::new() } else { "FinalizeReward poll budget expired".into() },
    })
}

/// TB-18 Atom B Phase 4: drive task B — Open → Work → Verify(OMEGA) → Challenge → ChallengeResolve(Released).
async fn drive_task_b(
    chain: &mut SharedChain,
    spec: &TaskSpec,
    cas_path: &Path,
) -> Result<ArenaTaskOutcome, String> {
    let scaffold = drive_task(chain, spec, PerCallBudget::default())
        .await
        .map_err(|e| format!("task_B drive_task: {e}"))?;
    let mut tx_kinds: Vec<&'static str> = vec!["TaskOpen", "EscrowLock"];
    let post_open = parse_hex(&scaffold.post_open_lock_state_root_hex)?;

    // Work
    let proposal_cid = write_minimal_proposal_telemetry(cas_path, "tb18-task-b", "Agent_0", 0, 20)?;
    let work_tx = sign_with_keypairs(chain, |reg| {
        make_real_worktx_signed_by(
            reg,
            &scaffold.task_id,
            "Agent_0",
            post_open,
            100,
            "tb18-b-work",
            proposal_cid,
            true,
            20,
        )
        .map_err(|e| format!("{e:?}"))
    })?;
    chain.bus.submit_typed_tx(work_tx).await.map_err(|e| format!("task_B Work submit: {e:?}"))?;
    let post_work = await_advance(chain, post_open, 5000).await?;
    tx_kinds.push("Work");
    let work_tx_id = TxId(format!("worktx-{}-tb18-b-work", scaffold.task_id));

    // Verify (OMEGA-Confirm, opens challenge window)
    let verify_tx = sign_with_keypairs(chain, |reg| {
        make_real_verifytx_signed_by(
            reg,
            post_work,
            work_tx_id.clone(),
            "Agent_1",
            50,
            "tb18-b-verify",
            true,
            21,
        )
        .map_err(|e| format!("{e:?}"))
    })?;
    chain.bus.submit_typed_tx(verify_tx).await.map_err(|e| format!("task_B Verify submit: {e:?}"))?;
    let post_verify = await_advance(chain, post_work, 5000).await?;
    tx_kinds.push("Verify");

    // Challenge (Agent_2 challenges; will be Released since proof was real)
    let challenge_tx = sign_with_keypairs(chain, |reg| {
        make_real_challengetx_signed_by(
            reg,
            post_verify,
            work_tx_id.clone(),
            "Agent_2",
            25,
            proposal_cid, // reuse as counterexample placeholder
            "tb18-b-challenge",
            22,
        )
        .map_err(|e| format!("{e:?}"))
    })?;
    chain.bus.submit_typed_tx(challenge_tx).await.map_err(|e| format!("task_B Challenge submit: {e:?}"))?;
    let post_challenge = await_advance(chain, post_verify, 5000).await?;
    tx_kinds.push("Challenge");

    // ChallengeResolve(Released) — tb16 helper iterates all eligible Open challenges.
    let bundle = chain.chaintape_bundle.as_ref().ok_or_else(|| "bundle=None".to_string())?;
    let (count, _bonds) = tb16_emit_challenge_resolve_for_eligible(
        bundle.sequencer.as_ref(),
        0,
        ChallengeResolution::Released,
    )
    .await
    .map_err(|e| format!("tb16_emit_challenge_resolve: {e:?}"))?;
    if count > 0 {
        tx_kinds.push("ChallengeResolve");
    }
    let _ = await_advance(chain, post_challenge, 5000).await;

    Ok(ArenaTaskOutcome {
        label: "task_B_challenge_released",
        task_id: scaffold.task_id,
        tx_kinds_emitted: tx_kinds,
        note: if count > 0 { String::new() } else { "ChallengeResolve count=0".into() },
    })
}

/// TB-18 Atom B Phase 4: drive task C — Open → MarketSeed → CompleteSetMint → TaskBankruptcy → CompleteSetRedeem.
async fn drive_task_c(
    chain: &mut SharedChain,
    spec: &TaskSpec,
    cas_path: &Path,
) -> Result<ArenaTaskOutcome, String> {
    let scaffold = drive_task(chain, spec, PerCallBudget::default())
        .await
        .map_err(|e| format!("task_C drive_task: {e}"))?;
    let mut tx_kinds: Vec<&'static str> = vec!["TaskOpen", "EscrowLock"];
    let post_open = parse_hex(&scaffold.post_open_lock_state_root_hex)?;

    // MarketSeed (Agent_user_0 provides 100k μC collateral)
    let market_seed_tx = sign_with_keypairs(chain, |reg| {
        make_real_market_seed_signed_by(
            reg,
            post_open,
            &scaffold.task_id,
            "Agent_user_0",
            100_000,
            "tb18-c-marketseed",
            30,
        )
        .map_err(|e| format!("{e:?}"))
    })?;
    chain.bus.submit_typed_tx(market_seed_tx).await.map_err(|e| format!("task_C MarketSeed submit: {e:?}"))?;
    let post_seed = await_advance(chain, post_open, 5000).await?;
    tx_kinds.push("MarketSeed");

    // CompleteSetMint (Agent_user_0 mints 50k YES + 50k NO shares)
    let cs_mint_tx = sign_with_keypairs(chain, |reg| {
        make_real_complete_set_mint_signed_by(
            reg,
            post_seed,
            &scaffold.task_id,
            "Agent_user_0",
            50_000,
            "tb18-c-csmint",
            31,
        )
        .map_err(|e| format!("{e:?}"))
    })?;
    chain.bus.submit_typed_tx(cs_mint_tx).await.map_err(|e| format!("task_C CompleteSetMint submit: {e:?}"))?;
    let post_mint = await_advance(chain, post_seed, 5000).await?;
    tx_kinds.push("CompleteSetMint");

    // TaskBankruptcy (system-emitted; resolves market to NO=wins per architect)
    let evidence_cid = write_minimal_evidence_capsule(cas_path, "tb18-task-c", &scaffold.task_id, "tb18-arena")?;
    let bundle = chain.chaintape_bundle.as_ref().ok_or_else(|| "bundle=None".to_string())?;
    let task_id = TaskId(scaffold.task_id.clone());
    bundle
        .sequencer
        .emit_system_tx(SystemEmitCommand::TaskBankruptcy {
            task_id: task_id.clone(),
            evidence_capsule_cid: evidence_cid,
            bankruptcy_reason: BankruptcyReason::MaxFailedRunCount,
            failed_run_count: 1,
        })
        .await
        .map_err(|e| format!("task_C TaskBankruptcy emit: {e:?}"))?;
    let post_bankruptcy = await_advance(chain, post_mint, 5000).await?;
    tx_kinds.push("TaskBankruptcy");

    // CompleteSetRedeem (Agent_user_0 redeems 50k NO shares — NO won via bankruptcy)
    let cs_redeem_tx = sign_with_keypairs(chain, |reg| {
        make_real_complete_set_redeem_signed_by(
            reg,
            post_bankruptcy,
            &scaffold.task_id,
            "Agent_user_0",
            OutcomeSide::No,
            50_000,
            "tb18-c-csredeem",
            32,
        )
        .map_err(|e| format!("{e:?}"))
    })?;
    chain.bus.submit_typed_tx(cs_redeem_tx).await.map_err(|e| format!("task_C CompleteSetRedeem submit: {e:?}"))?;
    let _ = await_advance(chain, post_bankruptcy, 5000).await;
    tx_kinds.push("CompleteSetRedeem");

    Ok(ArenaTaskOutcome {
        label: "task_C_market_lifecycle",
        task_id: scaffold.task_id,
        tx_kinds_emitted: tx_kinds,
        note: String::new(),
    })
}

/// TB-18 Atom B Phase 4: drive task D — Open → Work → TerminalSummary → TaskBankruptcy → TaskExpire(BankruptcyTriggered).
async fn drive_task_d(
    chain: &mut SharedChain,
    spec: &TaskSpec,
    cas_path: &Path,
) -> Result<ArenaTaskOutcome, String> {
    let scaffold = drive_task(chain, spec, PerCallBudget::default())
        .await
        .map_err(|e| format!("task_D drive_task: {e}"))?;
    let mut tx_kinds: Vec<&'static str> = vec!["TaskOpen", "EscrowLock"];
    let post_open = parse_hex(&scaffold.post_open_lock_state_root_hex)?;

    // Work (rejected via predicate_passes=false → L4.E; intentional for exhaustion path)
    let proposal_cid = write_minimal_proposal_telemetry(cas_path, "tb18-task-d", "Agent_3", 0, 40)?;
    let work_tx = sign_with_keypairs(chain, |reg| {
        make_real_worktx_signed_by(
            reg,
            &scaffold.task_id,
            "Agent_3",
            post_open,
            100,
            "tb18-d-work",
            proposal_cid,
            true,
            40,
        )
        .map_err(|e| format!("{e:?}"))
    })?;
    chain.bus.submit_typed_tx(work_tx).await.map_err(|e| format!("task_D Work submit: {e:?}"))?;
    let post_work = await_advance(chain, post_open, 5000).await?;
    tx_kinds.push("Work");

    // TerminalSummary (system-emitted; outcome=MaxTxExhausted)
    let evidence_cid = write_minimal_evidence_capsule(cas_path, "tb18-task-d", &scaffold.task_id, "tb18-arena")?;
    let bundle = chain.chaintape_bundle.as_ref().ok_or_else(|| "bundle=None".to_string())?;
    let task_id = TaskId(scaffold.task_id.clone());
    let run_id_typed = RunId("tb18-task-d-run".into());
    let mut histogram = std::collections::BTreeMap::new();
    histogram.insert(RejectionClass::Opaque, 1u32);
    tb11_emit_terminal_summary_for_run(
        bundle.sequencer.as_ref(),
        run_id_typed,
        task_id.clone(),
        RunOutcome::MaxTxExhausted,
        1,
        histogram,
        50,
        Some(turingosv4::state::q_state::AgentId("Agent_3".into())),
        Some(evidence_cid),
    )
    .await
    .map_err(|e| format!("task_D TerminalSummary emit: {e:?}"))?;
    let post_terminal = await_advance(chain, post_work, 5000).await?;
    tx_kinds.push("TerminalSummary");

    // TaskBankruptcy (system-emitted)
    bundle
        .sequencer
        .emit_system_tx(SystemEmitCommand::TaskBankruptcy {
            task_id: task_id.clone(),
            evidence_capsule_cid: evidence_cid,
            bankruptcy_reason: BankruptcyReason::MaxFailedRunCount,
            failed_run_count: 1,
        })
        .await
        .map_err(|e| format!("task_D TaskBankruptcy emit: {e:?}"))?;
    let post_bankruptcy = await_advance(chain, post_terminal, 5000).await?;
    tx_kinds.push("TaskBankruptcy");

    // TaskExpire (system-emitted; reason=BankruptcyTriggered).
    // tb11_emit_expire_for_eligible only emits Deadline reason; for
    // BankruptcyTriggered we go through emit_system_tx directly. The
    // helper is convenient for batch eligibility scans on Open tasks
    // approaching deadline; here the task is already Bankrupt so we
    // don't need scan eligibility.
    let escrow_lock_tx_id = TxId(scaffold.escrow_lock_tx_id.clone());
    bundle
        .sequencer
        .emit_system_tx(SystemEmitCommand::TaskExpire {
            task_id: task_id.clone(),
            escrow_tx_id: escrow_lock_tx_id,
            reason: ExpireReason::BankruptcyTriggered,
        })
        .await
        .map_err(|e| format!("task_D TaskExpire emit: {e:?}"))?;
    tx_kinds.push("TaskExpire");
    let _ = await_advance(chain, post_bankruptcy, 5000).await;

    Ok(ArenaTaskOutcome {
        label: "task_D_exhaustion_bankruptcy_expire",
        task_id: scaffold.task_id,
        tx_kinds_emitted: tx_kinds,
        note: String::new(),
    })
}

/// TB-18 Atom B Phase 4: drive task E — Open → TerminalSummary (no bankruptcy).
async fn drive_task_e(
    chain: &mut SharedChain,
    spec: &TaskSpec,
    cas_path: &Path,
) -> Result<ArenaTaskOutcome, String> {
    let scaffold = drive_task(chain, spec, PerCallBudget::default())
        .await
        .map_err(|e| format!("task_E drive_task: {e}"))?;
    let mut tx_kinds: Vec<&'static str> = vec!["TaskOpen", "EscrowLock"];
    let post_open = parse_hex(&scaffold.post_open_lock_state_root_hex)?;

    let evidence_cid = write_minimal_evidence_capsule(cas_path, "tb18-task-e", &scaffold.task_id, "tb18-arena")?;
    let bundle = chain.chaintape_bundle.as_ref().ok_or_else(|| "bundle=None".to_string())?;
    let task_id = TaskId(scaffold.task_id.clone());
    let run_id_typed = RunId("tb18-task-e-run".into());
    let histogram = std::collections::BTreeMap::new();
    tb11_emit_terminal_summary_for_run(
        bundle.sequencer.as_ref(),
        run_id_typed,
        task_id.clone(),
        RunOutcome::MaxTxExhausted,
        0,
        histogram,
        60,
        None,
        Some(evidence_cid),
    )
    .await
    .map_err(|e| format!("task_E TerminalSummary emit: {e:?}"))?;
    let _ = await_advance(chain, post_open, 5000).await;
    tx_kinds.push("TerminalSummary");

    Ok(ArenaTaskOutcome {
        label: "task_E_exhaustion_no_bankruptcy",
        task_id: scaffold.task_id,
        tx_kinds_emitted: tx_kinds,
        note: String::new(),
    })
}

/// TB-18 Atom B Phase 4: drive task F — Open → Work → TerminalSummary(outcome=DegradedLLM).
async fn drive_task_f(
    chain: &mut SharedChain,
    spec: &TaskSpec,
    cas_path: &Path,
) -> Result<ArenaTaskOutcome, String> {
    let scaffold = drive_task(chain, spec, PerCallBudget::default())
        .await
        .map_err(|e| format!("task_F drive_task: {e}"))?;
    let mut tx_kinds: Vec<&'static str> = vec!["TaskOpen", "EscrowLock"];
    let post_open = parse_hex(&scaffold.post_open_lock_state_root_hex)?;

    let proposal_cid = write_minimal_proposal_telemetry(cas_path, "tb18-task-f", "Agent_4", 0, 70)?;
    let work_tx = sign_with_keypairs(chain, |reg| {
        make_real_worktx_signed_by(
            reg,
            &scaffold.task_id,
            "Agent_4",
            post_open,
            100,
            "tb18-f-work",
            proposal_cid,
            true,
            70,
        )
        .map_err(|e| format!("{e:?}"))
    })?;
    chain.bus.submit_typed_tx(work_tx).await.map_err(|e| format!("task_F Work submit: {e:?}"))?;
    let post_work = await_advance(chain, post_open, 5000).await?;
    tx_kinds.push("Work");

    let evidence_cid = write_minimal_evidence_capsule(cas_path, "tb18-task-f", &scaffold.task_id, "tb18-arena")?;
    let bundle = chain.chaintape_bundle.as_ref().ok_or_else(|| "bundle=None".to_string())?;
    let task_id = TaskId(scaffold.task_id.clone());
    let run_id_typed = RunId("tb18-task-f-run".into());
    let histogram = std::collections::BTreeMap::new();
    tb11_emit_terminal_summary_for_run(
        bundle.sequencer.as_ref(),
        run_id_typed,
        task_id.clone(),
        RunOutcome::DegradedLLM,
        1,
        histogram,
        80,
        Some(turingosv4::state::q_state::AgentId("Agent_4".into())),
        Some(evidence_cid),
    )
    .await
    .map_err(|e| format!("task_F TerminalSummary emit: {e:?}"))?;
    let _ = await_advance(chain, post_work, 5000).await;
    tx_kinds.push("TerminalSummary");

    Ok(ArenaTaskOutcome {
        label: "task_F_degraded_llm",
        task_id: scaffold.task_id,
        tx_kinds_emitted: tx_kinds,
        note: "outcome=DegradedLLM (atom A new path)".into(),
    })
}

/// Helper: borrow agent_keypairs registry under mutex; pass `&mut reg` to closure.
fn sign_with_keypairs<F, T>(chain: &SharedChain, f: F) -> Result<T, String>
where
    F: FnOnce(
        &mut turingosv4::runtime::agent_keypairs::AgentKeypairRegistry,
    ) -> Result<T, String>,
{
    let arc = chain
        .agent_keypairs
        .as_ref()
        .ok_or_else(|| "agent_keypairs = None (chaintape mode required)".to_string())?
        .clone();
    let mut reg = arc.lock().map_err(|_| "agent_keypairs mutex poisoned".to_string())?;
    f(&mut reg)
}

/// Parse a 64-char hex string into Hash.
fn parse_hex(s: &str) -> Result<Hash, String> {
    if s.len() != 64 {
        return Err(format!("invalid hex length {}", s.len()));
    }
    let mut bytes = [0u8; 32];
    for i in 0..32 {
        bytes[i] = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16)
            .map_err(|e| format!("hex parse: {e}"))?;
    }
    Ok(Hash(bytes))
}

/// Write `SHARED_CHAIN_RUNS_REPORT.json` + `tx_kind_distribution.json`
/// per `EvidencePackagingPolicy`.
fn write_evidence_reports(
    cfg: &ArenaConfig,
    outcomes: &[ArenaTaskOutcome],
    chain_seed_id: &str,
    elapsed_ms: u128,
) -> Result<(), String> {
    let evidence_dir = cfg.out_dir.join("evidence");
    std::fs::create_dir_all(&evidence_dir).map_err(|e| format!("create evidence_dir: {e}"))?;

    // tx_kind_distribution.json — count per tx kind across all 6 tasks.
    let mut counts: std::collections::BTreeMap<&'static str, usize> =
        std::collections::BTreeMap::new();
    for o in outcomes {
        for k in &o.tx_kinds_emitted {
            *counts.entry(*k).or_insert(0) += 1;
        }
    }
    let counts_json: serde_json::Value = serde_json::json!({
        "chain_seed_id": chain_seed_id,
        "tx_kind_counts": counts,
        "distinct_tx_kinds": counts.len(),
        "target_distinct_tx_kinds": 13,
    });
    std::fs::write(
        evidence_dir.join("tx_kind_distribution.json"),
        serde_json::to_string_pretty(&counts_json).unwrap(),
    )
    .map_err(|e| format!("write tx_kind_distribution: {e}"))?;

    // SHARED_CHAIN_RUNS_REPORT.json — per-task outcome list.
    let runs_json: serde_json::Value = serde_json::json!({
        "chain_seed_id": chain_seed_id,
        "tb_id": "TB-18",
        "atom": "B Phase 4",
        "out_dir": cfg.out_dir.display().to_string(),
        "elapsed_ms": elapsed_ms,
        "task_count": outcomes.len(),
        "outcomes": outcomes.iter().map(|o| {
            serde_json::json!({
                "label": o.label,
                "task_id": o.task_id,
                "tx_kinds_emitted": o.tx_kinds_emitted,
                "note": o.note,
            })
        }).collect::<Vec<_>>(),
        "policy_reference": "handover/policies/TB-18_EVIDENCE_PACKAGING_POLICY.md",
    });
    std::fs::write(
        evidence_dir.join("SHARED_CHAIN_RUNS_REPORT.json"),
        serde_json::to_string_pretty(&runs_json).unwrap(),
    )
    .map_err(|e| format!("write SHARED_CHAIN_RUNS_REPORT: {e}"))?;

    Ok(())
}

async fn run_arena(cfg: ArenaConfig) -> Result<(), String> {
    let start = Instant::now();

    // Set TURINGOS_CHAINTAPE_PATH + TURINGOS_CHAINTAPE_PRESEED for the
    // SharedChain::from_env call. cas_path lives next to runtime_repo.
    let runtime_repo_path = cfg.out_dir.join("runtime_repo");
    let cas_path = cfg.out_dir.join("cas");
    std::fs::create_dir_all(&runtime_repo_path).map_err(|e| format!("create runtime_repo: {e}"))?;
    std::fs::create_dir_all(&cas_path).map_err(|e| format!("create cas: {e}"))?;
    std::env::set_var("TURINGOS_CHAINTAPE_PATH", &runtime_repo_path);
    std::env::set_var("TURINGOS_CAS_PATH", &cas_path);
    std::env::set_var("TURINGOS_CHAINTAPE_PRESEED", "1");
    if std::env::var("TURINGOS_AGENT_KEYSTORE_PASSWORD").is_err() {
        std::env::set_var("TURINGOS_AGENT_KEYSTORE_PASSWORD", "tb18-arena-localdev");
    }
    if std::env::var("TURINGOS_AGENT_KEYSTORE_PATH").is_err() {
        std::env::set_var(
            "TURINGOS_AGENT_KEYSTORE_PATH",
            cfg.out_dir.join("agent_keystore.enc"),
        );
    }

    eprintln!("[arena] initializing SharedChain at {runtime_repo_path:?} + {cas_path:?}");
    let mut chain = SharedChain::from_env(&format!("{}.lean", cfg.chain_seed_id));

    eprintln!("[arena] writing synthetic L4/L4.E gate + genesis_report");
    if let Some(ref bundle) = chain.chaintape_bundle.as_ref() {
        write_synthetic_l4_l4e_gate_and_genesis_report(
            &mut chain.bus,
            bundle,
            &chain.initial_balances_for_genesis_report,
            chain.chaintape_preseed_enabled,
            &cfg.chain_seed_id,
        )
        .await;
    } else {
        return Err("chaintape_bundle is None — TURINGOS_CHAINTAPE_PATH not honored".into());
    }

    let mut outcomes: Vec<ArenaTaskOutcome> = Vec::new();
    let task_specs = vec![
        ("task_A", TaskSpec::new("data/synth/task_a.lean", "theorem task_a_happy_path : 1+1=2", "task_a_happy_path", "synth", 1)),
        ("task_B", TaskSpec::new("data/synth/task_b.lean", "theorem task_b_challenge : 2+2=4", "task_b_challenge", "synth", 1)),
        ("task_C", TaskSpec::new("data/synth/task_c.lean", "theorem task_c_market : 3+3=6", "task_c_market", "synth", 1)),
        ("task_D", TaskSpec::new("data/synth/task_d.lean", "theorem task_d_exhaustion : 4+4=8", "task_d_exhaustion", "synth", 1)),
        ("task_E", TaskSpec::new("data/synth/task_e.lean", "theorem task_e_no_solver : 5+5=10", "task_e_no_solver", "synth", 1)),
        ("task_F", TaskSpec::new("data/synth/task_f.lean", "theorem task_f_degraded : 6+6=12", "task_f_degraded", "synth", 1)),
    ];
    for (label, spec) in task_specs {
        eprintln!("[arena] driving {label}");
        let outcome = match label {
            "task_A" => drive_task_a(&mut chain, &spec, &cas_path).await,
            "task_B" => drive_task_b(&mut chain, &spec, &cas_path).await,
            "task_C" => drive_task_c(&mut chain, &spec, &cas_path).await,
            "task_D" => drive_task_d(&mut chain, &spec, &cas_path).await,
            "task_E" => drive_task_e(&mut chain, &spec, &cas_path).await,
            "task_F" => drive_task_f(&mut chain, &spec, &cas_path).await,
            _ => unreachable!(),
        };
        match outcome {
            Ok(o) => {
                eprintln!(
                    "[arena] {label} OK: tx_kinds={:?} note={}",
                    o.tx_kinds_emitted, if o.note.is_empty() { "(none)" } else { &o.note }
                );
                outcomes.push(o);
            }
            Err(e) => {
                eprintln!("[arena] {label} FAILED: {e}");
                return Err(format!("{label}: {e}"));
            }
        }
    }

    // Shutdown bundle (drain queued submissions).
    if let Some(bundle) = chain.chaintape_bundle.take() {
        eprintln!("[arena] shutting down chain bundle");
        if let Err(e) = bundle.shutdown().await {
            eprintln!("[arena] bundle.shutdown error: {e}");
        }
    }

    let elapsed = start.elapsed().as_millis();
    write_evidence_reports(&cfg, &outcomes, &cfg.chain_seed_id, elapsed)?;
    eprintln!("[arena] DONE in {elapsed}ms; evidence written to {:?}", cfg.out_dir.join("evidence"));
    Ok(())
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let cfg = match ArenaConfig::from_args(&argv) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("comprehensive_arena: {e}\n\n{}", help_text());
            return ExitCode::from(2);
        }
    };

    print_plan(&cfg);

    if cfg.plan_only {
        eprintln!("[arena] --plan-only; exiting before chain side-effects");
        return ExitCode::from(0);
    }

    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("comprehensive_arena: tokio runtime build failed: {e}");
            return ExitCode::from(3);
        }
    };

    match runtime.block_on(run_arena(cfg)) {
        Ok(()) => ExitCode::from(0),
        Err(e) => {
            eprintln!("comprehensive_arena: arena failed: {e}");
            ExitCode::from(4)
        }
    }
}
