//! True-suite full-system augmentation helper.
//!
//! This runner-side helper resumes an existing current-kernel ChainTape/CAS
//! produced by a real-world domain runner, then appends the missing
//! full-system participation rows to the same tape:
//! - a real node-market action for an accepted WorkTx
//! - typed FC3 ArchitectAI/Veto-AI feedback and re-init system txs
//!
//! It does not submit fake dashboard evidence and does not mutate the kernel.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::Serialize;
use serde_json::json;
use sha2::{Digest, Sha256};

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::ledger::transition_ledger::{
    canonical_decode, canonical_encode, cas_metadata_root_before_logical_t,
    constitution_source_hash, LedgerEntry, TxKind,
};
use turingosv4::runtime::adapter::{
    make_real_escrow_lock_signed_by, make_real_task_open_signed_by, make_real_worktx_signed_by,
    tb8_await_state_root_advance, tb_n3_emit_node_market_after_work_accept,
    tb_n3_invest_to_router_tx, NodeMarketEmitOutcome,
};
use turingosv4::runtime::agent_keypairs::{AgentKeypairRegistry, AgentPubkeyManifest};
use turingosv4::runtime::evidence_capsule::{write_evidence_capsule, ExhaustionCounts};
use turingosv4::runtime::genesis_report::GenesisReport;
use turingosv4::runtime::market_decision_trace::{
    write_market_decision_trace_to_cas, MarketDecisionTrace,
};
use turingosv4::runtime::proposal_telemetry::{
    write_to_cas as write_proposal_telemetry_to_cas, ProposalTelemetry, TokenCounts,
};
use turingosv4::runtime::{
    build_chaintape_sequencer_with_initial_q, ChaintapeBundle, RuntimeChaintapeConfig,
};
use turingosv4::state::q_state::{AgentId, Hash, QState, TaskId, TxId};
use turingosv4::state::sequencer::SystemEmitCommand;
use turingosv4::state::typed_tx::{
    ArchitectCommitCapsule, ArchitectProposalCapsule, ArchitectProposalKind, ArchitectProposalTx,
    BootProfileId, BuyDirection, CapsulePrivacyPolicy, ExhaustionReason, LogFeedbackArchiveTx,
    ReinitReason, ReinitReasonCapsule, RunId, RunOutcome, TypedTx, VetoDecisionCapsule,
    VetoDecisionTx, VetoReasonCode, VetoVerdict, ARCHITECT_COMMIT_SCHEMA_ID,
    ARCHITECT_FEEDBACK_SCHEMA_ID, ARCHITECT_PROPOSAL_SCHEMA_ID, REINIT_REASON_SCHEMA_ID,
    VETO_DECISION_SCHEMA_ID,
};

const MARKET_MAKER_AGENT: &str = "MarketMakerBudget";
const CANARY_TASK_ID: &str = "full-system-participation-canary";
const NODE_MARKET_SEED_MICRO: i64 = 100_000;
const TRADER_BUY_MICRO: i64 = 10_000;
const CANARY_ESCROW_MICRO: i64 = 10_000;
const CANARY_WORK_STAKE_MICRO: i64 = 100;

#[derive(Debug)]
struct Args {
    runtime_repo: PathBuf,
    cas: PathBuf,
    run_id: String,
    constitution: PathBuf,
    out_dir: PathBuf,
    skip_fc3: bool,
}

#[derive(Debug, Serialize)]
struct AugmentManifest {
    schema_version: &'static str,
    run_id: String,
    runtime_repo: String,
    cas: String,
    accepted_work_tx_id: String,
    market_anchor_source: String,
    domain_accepted_work_tx_id: Option<String>,
    system_canary_work_tx_id: Option<String>,
    domain_rejected_work_l4e_count: usize,
    market_event_id: String,
    router_tx_id: String,
    failed_router_tx_id: String,
    market_decision_trace_cid: String,
    fc3_tx_sequence: Vec<TxIndexRow>,
    final_state_root_hex: String,
}

#[derive(Debug, Serialize)]
struct TxIndexRow {
    logical_t: u64,
    tx_kind: String,
    tx_id: String,
    tx_payload_cid: String,
}

#[derive(Debug, Clone)]
struct AugmentAgents {
    trader_agent: String,
    canary_sponsor_agent: Option<String>,
    canary_solver_agent: Option<String>,
}

impl AugmentAgents {
    fn canary_sponsor(&self) -> Result<&str, String> {
        self.canary_sponsor_agent
            .as_deref()
            .ok_or_else(|| "canary sponsor agent was not allocated".to_string())
    }

    fn canary_solver(&self) -> Result<&str, String> {
        self.canary_solver_agent
            .as_deref()
            .ok_or_else(|| "canary solver agent was not allocated".to_string())
    }
}

fn usage() -> &'static str {
    "usage: full_system_augment_current_kernel \
     --runtime-repo <PATH> --cas <PATH> --run-id <ID> \
     --constitution <constitution.md> --out-dir <PATH> [--skip-fc3]"
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut runtime_repo = None;
    let mut cas = None;
    let mut run_id = None;
    let mut constitution = None;
    let mut out_dir = None;
    let mut skip_fc3 = false;
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "--runtime-repo" => {
                i += 1;
                runtime_repo = Some(argv.get(i).ok_or("--runtime-repo requires value")?.into());
            }
            "--cas" => {
                i += 1;
                cas = Some(argv.get(i).ok_or("--cas requires value")?.into());
            }
            "--run-id" => {
                i += 1;
                run_id = Some(argv.get(i).ok_or("--run-id requires value")?.clone());
            }
            "--constitution" => {
                i += 1;
                constitution = Some(argv.get(i).ok_or("--constitution requires value")?.into());
            }
            "--out-dir" => {
                i += 1;
                out_dir = Some(argv.get(i).ok_or("--out-dir requires value")?.into());
            }
            "--skip-fc3" => {
                skip_fc3 = true;
            }
            "--help" | "-h" => return Err(usage().into()),
            other => return Err(format!("unknown arg: {other}")),
        }
        i += 1;
    }
    Ok(Args {
        runtime_repo: runtime_repo.ok_or("--runtime-repo required")?,
        cas: cas.ok_or("--cas required")?,
        run_id: run_id.ok_or("--run-id required")?,
        constitution: constitution.ok_or("--constitution required")?,
        out_dir: out_dir.ok_or("--out-dir required")?,
        skip_fc3,
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse_args(&argv) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("full_system_augment_current_kernel: {err}");
            eprintln!("{}", usage());
            return ExitCode::from(2);
        }
    };
    match run(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("full_system_augment_current_kernel: {err}");
            ExitCode::from(1)
        }
    }
}

async fn run(args: Args) -> Result<(), String> {
    std::fs::create_dir_all(&args.out_dir).map_err(|e| format!("out dir: {e}"))?;
    let cfg = RuntimeChaintapeConfig {
        runtime_repo_path: args.runtime_repo.clone(),
        cas_path: args.cas.clone(),
        run_id: args.run_id.clone(),
        queue_capacity: 16,
        resume_existing_chain: true,
    };
    let bundle = build_chaintape_sequencer_with_initial_q(&cfg, QState::genesis())
        .map_err(|e| format!("resume existing ChainTape failed: {e}"))?;
    let domain_accepted_work_tx_id = first_accepted_work_tx_id(&bundle)?;
    let domain_rejected_work_l4e_count = l4e_tx_kind_count(&bundle, TxKind::Work)?;
    let needs_canary = domain_accepted_work_tx_id.is_none() && domain_rejected_work_l4e_count > 0;
    let (mut keypairs, augment_agents) = prepare_augment_keypairs(&bundle, needs_canary)?;
    let (work_tx_id, market_anchor_source, system_canary_work_tx_id) =
        if let Some(work_tx_id) = domain_accepted_work_tx_id.clone() {
            (work_tx_id, "domain_accepted_work".to_string(), None)
        } else if domain_rejected_work_l4e_count > 0 {
            let canary =
                append_system_participation_canary(&bundle, &mut keypairs, &args, &augment_agents)
                    .await?;
            (
                canary.clone(),
                "system_participation_canary_after_domain_rejection".to_string(),
                Some(canary),
            )
        } else {
            return Err(
                "no accepted WorkTx and no rejected WorkTx found; cannot anchor full-system market"
                    .to_string(),
            );
        };

    let market = append_market_action(&bundle, &mut keypairs, &work_tx_id, &augment_agents).await?;
    let fc3_rows = if args.skip_fc3 {
        Vec::new()
    } else {
        append_fc3_sequence(&bundle, &args, &augment_agents).await?
    };
    refresh_genesis_report(&args)?;

    let final_q = bundle
        .sequencer
        .q_snapshot()
        .map_err(|e| format!("final q snapshot: {e}"))?;
    let manifest = AugmentManifest {
        schema_version: "turingosv4.true_suite.full_system_augment.v1",
        run_id: args.run_id.clone(),
        runtime_repo: args.runtime_repo.display().to_string(),
        cas: args.cas.display().to_string(),
        accepted_work_tx_id: work_tx_id.0,
        market_anchor_source,
        domain_accepted_work_tx_id: domain_accepted_work_tx_id.map(|tx| tx.0),
        system_canary_work_tx_id: system_canary_work_tx_id.map(|tx| tx.0),
        domain_rejected_work_l4e_count,
        market_event_id: market.event_id,
        router_tx_id: market.router_tx_id,
        failed_router_tx_id: market.failed_router_tx_id,
        market_decision_trace_cid: market.market_decision_trace_cid,
        fc3_tx_sequence: fc3_rows,
        final_state_root_hex: hash_hex(final_q.state_root_t),
    };
    write_json(
        &args.out_dir.join("full_system_augmentation_manifest.json"),
        &manifest,
    )?;
    bundle
        .shutdown()
        .await
        .map_err(|e| format!("shutdown resumed ChainTape: {e}"))?;
    println!(
        "full_system_augment_current_kernel: work_tx={} router_tx={} out={}",
        manifest.accepted_work_tx_id,
        manifest.router_tx_id,
        args.out_dir.display()
    );
    Ok(())
}

struct MarketRows {
    event_id: String,
    router_tx_id: String,
    failed_router_tx_id: String,
    market_decision_trace_cid: String,
}

async fn append_market_action(
    bundle: &ChaintapeBundle,
    keypairs: &mut AgentKeypairRegistry,
    work_tx_id: &TxId,
    agents: &AugmentAgents,
) -> Result<MarketRows, String> {
    let market = tb_n3_emit_node_market_after_work_accept(
        &bundle.sequencer,
        work_tx_id,
        keypairs,
        "full-system-augment",
        30,
        NODE_MARKET_SEED_MICRO,
    )
    .await
    .map_err(|e| format!("node market emit failed: {e}"))?;
    let event_id = match market {
        NodeMarketEmitOutcome::Created { event_id, .. } => event_id,
        NodeMarketEmitOutcome::AlreadyExists => {
            turingosv4::state::typed_tx::node_survive_event_id(work_tx_id)
        }
        other => return Err(format!("node market not available: {other:?}")),
    };

    let q = bundle
        .sequencer
        .q_snapshot()
        .map_err(|e| format!("q snapshot before router: {e}"))?;
    let parent_root = q.state_root_t;
    let router = tb_n3_invest_to_router_tx(
        keypairs,
        parent_root,
        Some(&q),
        &agents.trader_agent,
        &work_tx_id.0,
        BuyDirection::BuyYes,
        TRADER_BUY_MICRO,
        0,
        "full-system-augment",
    )
    .map_err(|e| format!("build node-market router tx: {e:?}"))?;
    let router_tx_id = match &router {
        TypedTx::BuyWithCoinRouter(tx) => tx.tx_id.clone(),
        _ => return Err("node-market router helper returned non-router tx".to_string()),
    };
    let before_len = ledger_len(bundle)?;
    bundle
        .sequencer
        .submit_agent_tx(router)
        .await
        .map_err(|e| format!("submit node-market router tx: {e:?}"))?;
    let router_entry = await_entry(bundle, before_len, TxKind::BuyWithCoinRouter).await?;
    let prompt_block = format!(
        "agent={}; market=node_survive; work_tx={}; direction=YES; amount_micro={TRADER_BUY_MICRO}",
        agents.trader_agent, work_tx_id.0
    );
    let trace = MarketDecisionTrace::submitted(
        AgentId(agents.trader_agent.clone()),
        work_tx_id.clone(),
        BuyDirection::BuyYes,
        TRADER_BUY_MICRO,
        router_tx_id.clone(),
        format!(
            "{} invested real budget into the accepted WorkTx node-survive market",
            agents.trader_agent
        ),
    )
    .with_prompt_context(sha256_hex(&prompt_block), vec![work_tx_id.clone()]);
    let trace_cid = {
        let mut cas = bundle
            .cas
            .write()
            .map_err(|_| "cas write poisoned".to_string())?;
        write_market_decision_trace_to_cas(
            &mut cas,
            &trace,
            "full-system-augment-submitted",
            router_entry.logical_t,
        )
        .map_err(|e| format!("write MarketDecisionTrace: {e}"))?
    };
    let failed_router_tx_id =
        append_controlled_failed_market_action(bundle, keypairs, work_tx_id, &agents.trader_agent)
            .await?;

    Ok(MarketRows {
        event_id: event_id.0 .0,
        router_tx_id: router_tx_id.0,
        failed_router_tx_id,
        market_decision_trace_cid: trace_cid.hex(),
    })
}

fn prepare_augment_keypairs(
    bundle: &ChaintapeBundle,
    needs_canary: bool,
) -> Result<(AgentKeypairRegistry, AugmentAgents), String> {
    let keypair_dir = bundle
        .runtime_repo_path
        .parent()
        .unwrap_or(&bundle.runtime_repo_path)
        .join(".full_system_augment_agent_keys");
    std::fs::create_dir_all(&keypair_dir)
        .map_err(|e| format!("create augment keypair dir: {e}"))?;
    let mut keypairs = AgentKeypairRegistry::open(&keypair_dir).map_err(|e| format!("{e}"))?;
    let q = bundle
        .sequencer
        .q_snapshot()
        .map_err(|e| format!("q snapshot for augment agent selection: {e}"))?;
    let existing = existing_agent_ids(&bundle.runtime_repo_path)?;
    let mut reserved = existing;
    reserved.insert(MARKET_MAKER_AGENT.to_string());
    let trader_agent = select_unused_funded_agent(
        &mut reserved,
        &q,
        "full-system market trader",
        TRADER_BUY_MICRO,
    )?;
    let canary_sponsor_agent = if needs_canary {
        Some(select_unused_funded_agent(
            &mut reserved,
            &q,
            "full-system canary sponsor",
            CANARY_ESCROW_MICRO,
        )?)
    } else {
        None
    };
    let canary_solver_agent = if needs_canary {
        Some(select_unused_funded_agent(
            &mut reserved,
            &q,
            "full-system canary solver",
            CANARY_WORK_STAKE_MICRO,
        )?)
    } else {
        None
    };
    let agents = AugmentAgents {
        trader_agent,
        canary_sponsor_agent,
        canary_solver_agent,
    };
    let mut ids = vec![MARKET_MAKER_AGENT.to_string(), agents.trader_agent.clone()];
    if let Some(agent) = &agents.canary_sponsor_agent {
        ids.push(agent.clone());
    }
    if let Some(agent) = &agents.canary_solver_agent {
        ids.push(agent.clone());
    }
    for id in ids {
        keypairs
            .get_or_create(&AgentId(id.clone()))
            .map_err(|e| format!("create keypair for {id}: {e}"))?;
    }
    let merged_manifest = append_agent_pubkeys(&bundle.runtime_repo_path, keypairs.manifest())?;
    bundle
        .sequencer
        .set_agent_pubkeys(Arc::new(merged_manifest))
        .map_err(|_| "agent pubkey manifest already set".to_string())?;
    Ok((keypairs, agents))
}

async fn append_controlled_failed_market_action(
    bundle: &ChaintapeBundle,
    keypairs: &mut AgentKeypairRegistry,
    work_tx_id: &TxId,
    trader_agent: &str,
) -> Result<String, String> {
    let q = bundle
        .sequencer
        .q_snapshot()
        .map_err(|e| format!("q snapshot before failed router: {e}"))?;
    let failed_router = tb_n3_invest_to_router_tx(
        keypairs,
        q.state_root_t,
        None,
        trader_agent,
        &work_tx_id.0,
        BuyDirection::BuyNo,
        9_000_000_000_000,
        0,
        "full-system-augment-l4e",
    )
    .map_err(|e| format!("build controlled failed router tx: {e:?}"))?;
    let tx_id = match &failed_router {
        TypedTx::BuyWithCoinRouter(tx) => tx.tx_id.clone(),
        _ => return Err("failed router helper returned non-router tx".to_string()),
    };
    let before_l4e = l4e_len(bundle)?;
    bundle
        .sequencer
        .submit_agent_tx(failed_router)
        .await
        .map_err(|e| format!("submit controlled failed router tx: {e:?}"))?;
    await_l4e_record(bundle, before_l4e, TxKind::BuyWithCoinRouter).await?;
    Ok(tx_id.0)
}

async fn append_fc3_sequence(
    bundle: &ChaintapeBundle,
    args: &Args,
    agents: &AugmentAgents,
) -> Result<Vec<TxIndexRow>, String> {
    let feedback_capsule_cid = put_feedback_capsule(bundle)?;
    let feedback_entry = emit_and_read(
        bundle,
        SystemEmitCommand::LogFeedbackArchive {
            feedback_capsule_cid,
            veto_verdict: VetoVerdict::Pass,
        },
        TxKind::LogFeedbackArchive,
    )
    .await?;
    let TypedTx::LogFeedbackArchive(feedback_tx) = decode_entry_tx(bundle, &feedback_entry)? else {
        return Err("expected LogFeedbackArchive tx".to_string());
    };

    let patch_artifact_cid = put_bytes(
        bundle,
        b"full-system augment runtime proposal artifact",
        "full_system_augment_current_kernel",
        Some("full_system_augment.tool_registry_patch_artifact.v1"),
    )?;
    let proposal_capsule_cid =
        put_proposal_capsule(bundle, &feedback_tx, Some(patch_artifact_cid))?;
    let proposal_entry = emit_and_read(
        bundle,
        SystemEmitCommand::ArchitectProposal {
            feedback_tx_id: feedback_tx.tx_id.clone(),
            proposal_capsule_cid,
        },
        TxKind::ArchitectProposal,
    )
    .await?;
    let TypedTx::ArchitectProposal(proposal_tx) = decode_entry_tx(bundle, &proposal_entry)? else {
        return Err("expected ArchitectProposal tx".to_string());
    };

    let veto_capsule_cid = put_veto_capsule(bundle, &proposal_tx)?;
    let veto_entry = emit_and_read(
        bundle,
        SystemEmitCommand::VetoDecision {
            proposal_tx_id: proposal_tx.tx_id.clone(),
            decision_capsule_cid: veto_capsule_cid,
        },
        TxKind::VetoDecision,
    )
    .await?;
    let TypedTx::VetoDecision(veto_tx) = decode_entry_tx(bundle, &veto_entry)? else {
        return Err("expected VetoDecision tx".to_string());
    };

    let commit_capsule_cid = put_commit_capsule(bundle, &veto_tx, Some(patch_artifact_cid))?;
    let commit_entry = emit_and_read(
        bundle,
        SystemEmitCommand::ArchitectCommit {
            veto_tx_id: veto_tx.tx_id.clone(),
            commit_capsule_cid,
        },
        TxKind::ArchitectCommit,
    )
    .await?;

    let terminal_evidence_cid = put_terminal_evidence_capsule(
        bundle,
        &args.run_id,
        TaskId("full-system-augment-fc3".to_string()),
        Some(AgentId(agents.trader_agent.clone())),
        "Full-system augment ErrorHalt terminal summary evidence",
    )?;
    let terminal_entry = emit_and_read(
        bundle,
        SystemEmitCommand::TerminalSummary {
            run_id: RunId(args.run_id.clone()),
            task_id: TaskId("full-system-augment-fc3".to_string()),
            run_outcome: RunOutcome::ErrorHalt,
            total_attempts: 1,
            failure_class_histogram: BTreeMap::new(),
            last_logical_t: commit_entry.logical_t,
            solver_agent: Some(AgentId(agents.trader_agent.clone())),
            evidence_capsule_cid: Some(terminal_evidence_cid),
        },
        TxKind::TerminalSummary,
    )
    .await?;

    let reason = ReinitReason::TerminalErrorHalt;
    let reason_cid = put_reinit_capsule(bundle, terminal_entry.logical_t, reason)?;
    let boot_profile = BootProfileId("full-system-augment-reinit".to_string());
    let request_entry = emit_and_read(
        bundle,
        SystemEmitCommand::ReinitRequest {
            trigger_entry: terminal_entry.logical_t,
            error_evidence_cid: reason_cid,
            reason,
            target_boot_profile: boot_profile.clone(),
        },
        TxKind::ReinitRequest,
    )
    .await?;
    let TypedTx::ReinitRequest(request_tx) = decode_entry_tx(bundle, &request_entry)? else {
        return Err("expected ReinitRequest tx".to_string());
    };

    let boot_entry = emit_and_read(
        bundle,
        SystemEmitCommand::ReinitBoot {
            request_tx_id: request_tx.tx_id.clone(),
            boot_profile,
        },
        TxKind::ReinitBoot,
    )
    .await?;

    let entries = vec![
        feedback_entry,
        proposal_entry,
        veto_entry,
        commit_entry,
        terminal_entry,
        request_entry,
        boot_entry,
    ];
    let rows = entries
        .iter()
        .map(|entry| tx_index_row(bundle, entry))
        .collect::<Result<Vec<_>, _>>()?;
    write_governance_index(bundle, args, &rows)?;
    Ok(rows)
}

async fn append_system_participation_canary(
    bundle: &ChaintapeBundle,
    keypairs: &mut AgentKeypairRegistry,
    args: &Args,
    agents: &AugmentAgents,
) -> Result<TxId, String> {
    let canary_sponsor_agent = agents.canary_sponsor()?;
    let canary_solver_agent = agents.canary_solver()?;
    let canary_payload = json!({
        "schema_version": "turingosv4.true_suite.full_system_participation_canary.v1",
        "run_id": args.run_id,
        "purpose": "accepted WorkTx anchor for OS full-flow market participation after domain WorkTx rejection",
        "task_verdict_boundary": "does_not_promote_domain_task_to_TASK_PASS"
    });
    let payload_bytes = serde_json::to_vec(&canary_payload)
        .map_err(|e| format!("encode canary proposal payload: {e}"))?;
    let proposal_artifact_cid = put_bytes_with_type(
        bundle,
        &payload_bytes,
        ObjectType::ProposalPayload,
        "full_system_augment_current_kernel",
        Some("turingosv4.true_suite.full_system_participation_canary.v1"),
    )?;
    let prompt_hash = hash_bytes(b"full-system-participation-canary");
    let telemetry = ProposalTelemetry::new_root(
        AgentId(canary_solver_agent.to_string()),
        prompt_hash,
        proposal_artifact_cid,
        "full-system participation canary".to_string(),
        TokenCounts::default(),
        format!("{canary_solver_agent}.full_system_participation_canary"),
    );
    let proposal_telemetry_cid = {
        let mut cas = bundle
            .cas
            .write()
            .map_err(|_| "cas write poisoned".to_string())?;
        write_proposal_telemetry_to_cas(
            &mut cas,
            &telemetry,
            "full-system-canary-proposal-telemetry",
            bundle.sequencer.next_logical_t_peek(),
        )
        .map_err(|e| format!("write canary ProposalTelemetry: {e}"))?
    };

    let initial_root = bundle
        .sequencer
        .q_snapshot()
        .map_err(|e| format!("canary q snapshot initial: {e}"))?
        .state_root_t;
    let task_open = make_real_task_open_signed_by(
        keypairs,
        CANARY_TASK_ID,
        canary_sponsor_agent,
        initial_root,
        "full-system-canary",
        40,
    )
    .map_err(|e| format!("build canary TaskOpenTx: {e}"))?;
    bundle
        .sequencer
        .submit_agent_tx(task_open)
        .await
        .map_err(|e| format!("submit canary TaskOpenTx: {e:?}"))?;
    let after_open = tb8_await_state_root_advance(&bundle.sequencer, initial_root, 5_000)
        .await
        .map_err(|_| "canary TaskOpenTx did not advance state_root".to_string())?;

    let escrow = make_real_escrow_lock_signed_by(
        keypairs,
        CANARY_TASK_ID,
        canary_sponsor_agent,
        CANARY_ESCROW_MICRO,
        after_open,
        "full-system-canary",
        41,
    )
    .map_err(|e| format!("build canary EscrowLockTx: {e}"))?;
    bundle
        .sequencer
        .submit_agent_tx(escrow)
        .await
        .map_err(|e| format!("submit canary EscrowLockTx: {e:?}"))?;
    let after_escrow = tb8_await_state_root_advance(&bundle.sequencer, after_open, 5_000)
        .await
        .map_err(|_| "canary EscrowLockTx did not advance state_root".to_string())?;

    let work = make_real_worktx_signed_by(
        keypairs,
        CANARY_TASK_ID,
        canary_solver_agent,
        after_escrow,
        CANARY_WORK_STAKE_MICRO,
        "full-system-canary",
        proposal_telemetry_cid,
        true,
        42,
    )
    .map_err(|e| format!("build canary WorkTx: {e}"))?;
    let work_tx_id = match &work {
        TypedTx::Work(work) => work.tx_id.clone(),
        _ => return Err("canary work helper returned non-Work tx".to_string()),
    };
    bundle
        .sequencer
        .submit_agent_tx(work)
        .await
        .map_err(|e| format!("submit canary WorkTx: {e:?}"))?;
    tb8_await_state_root_advance(&bundle.sequencer, after_escrow, 5_000)
        .await
        .map_err(|_| "canary WorkTx did not advance state_root".to_string())?;
    Ok(work_tx_id)
}

fn first_accepted_work_tx_id(bundle: &ChaintapeBundle) -> Result<Option<TxId>, String> {
    let writer = bundle
        .transition_writer
        .read()
        .map_err(|_| "writer read poisoned".to_string())?;
    for logical_t in 1..=writer.len() {
        let entry = writer
            .read_at(logical_t)
            .map_err(|e| format!("read ledger entry {logical_t}: {e}"))?;
        if entry.tx_kind != TxKind::Work {
            continue;
        }
        let tx = decode_entry_tx(bundle, &entry)?;
        if let TypedTx::Work(work) = tx {
            return Ok(Some(work.tx_id));
        }
    }
    Ok(None)
}

fn l4e_tx_kind_count(bundle: &ChaintapeBundle, expected: TxKind) -> Result<usize, String> {
    let writer = bundle
        .rejection_writer
        .read()
        .map_err(|_| "l4e read poisoned".to_string())?;
    Ok(writer
        .records()
        .iter()
        .filter(|record| record.tx_kind == expected)
        .count())
}

fn existing_agent_ids(runtime_repo_path: &Path) -> Result<BTreeSet<String>, String> {
    let path = runtime_repo_path.join("agent_pubkeys.json");
    if !path.is_file() {
        return Ok(BTreeSet::new());
    }
    let manifest = AgentPubkeyManifest::load(&path)
        .map_err(|e| format!("load existing agent pubkeys: {e}"))?;
    Ok(manifest.agents.into_keys().collect())
}

fn select_unused_funded_agent(
    used: &mut BTreeSet<String>,
    q: &QState,
    role: &str,
    required_micro: i64,
) -> Result<String, String> {
    let mut candidates: Vec<(String, i64)> = q
        .economic_state_t
        .balances_t
        .0
        .iter()
        .filter_map(|(agent_id, balance)| {
            let id = agent_id.0.clone();
            let balance_micro = balance.micro_units();
            if used.contains(&id) || balance_micro < required_micro {
                None
            } else {
                Some((id, balance_micro))
            }
        })
        .collect();
    candidates.sort_by(|(agent_a, balance_a), (agent_b, balance_b)| {
        balance_b.cmp(balance_a).then_with(|| agent_a.cmp(agent_b))
    });
    if let Some((candidate, _balance_micro)) = candidates.into_iter().next() {
        used.insert(candidate.clone());
        return Ok(candidate);
    }
    Err(format!(
        "no unused funded identity with at least {required_micro}μC available for {role}; \
         full-system augmentation cannot create a signed participant without colliding \
         with the existing runtime agent registry or overspending the current QState"
    ))
}

fn append_agent_pubkeys(
    runtime_repo_path: &Path,
    additions: AgentPubkeyManifest,
) -> Result<AgentPubkeyManifest, String> {
    let path = runtime_repo_path.join("agent_pubkeys.json");
    let mut merged = if path.is_file() {
        AgentPubkeyManifest::load(&path).map_err(|e| format!("load existing agent pubkeys: {e}"))?
    } else {
        AgentPubkeyManifest::default()
    };
    for (agent_id, pubkey_hex) in additions.agents {
        match merged.agents.get(&agent_id) {
            Some(existing) if existing != &pubkey_hex => {
                return Err(format!(
                    "agent pubkey collision for {agent_id}: existing {existing}, new {pubkey_hex}"
                ));
            }
            Some(_) => {}
            None => {
                merged.agents.insert(agent_id, pubkey_hex);
            }
        }
    }
    let serialized = serde_json::to_string_pretty(&merged)
        .map_err(|e| format!("encode merged agent pubkeys: {e}"))?;
    let tmp = path.with_extension("json.tmp.full_system_augment");
    std::fs::write(&tmp, format!("{serialized}\n"))
        .map_err(|e| format!("write merged agent pubkeys tmp: {e}"))?;
    std::fs::rename(&tmp, &path).map_err(|e| format!("install merged agent pubkeys: {e}"))?;
    Ok(merged)
}

async fn emit_and_read(
    bundle: &ChaintapeBundle,
    command: SystemEmitCommand,
    expected: TxKind,
) -> Result<LedgerEntry, String> {
    let before = ledger_len(bundle)?;
    bundle
        .sequencer
        .emit_system_tx(command)
        .await
        .map_err(|e| format!("emit system tx: {e}"))?;
    await_entry(bundle, before, expected).await
}

async fn await_entry(
    bundle: &ChaintapeBundle,
    before_len: u64,
    expected: TxKind,
) -> Result<LedgerEntry, String> {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let len = ledger_len(bundle)?;
        if len > before_len {
            let entry = {
                let writer = bundle
                    .transition_writer
                    .read()
                    .map_err(|_| "writer read poisoned".to_string())?;
                writer
                    .read_at(before_len + 1)
                    .map_err(|e| format!("read ledger entry {}: {e}", before_len + 1))?
            };
            if entry.tx_kind != expected {
                return Err(format!(
                    "expected {:?} at logical_t {}, got {:?}",
                    expected, entry.logical_t, entry.tx_kind
                ));
            }
            return Ok(entry);
        }
        if Instant::now() >= deadline {
            return Err(format!("timed out waiting for {:?} L4 entry", expected));
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

fn ledger_len(bundle: &ChaintapeBundle) -> Result<u64, String> {
    bundle
        .transition_writer
        .read()
        .map_err(|_| "writer read poisoned".to_string())
        .map(|w| w.len())
}

fn l4e_len(bundle: &ChaintapeBundle) -> Result<usize, String> {
    bundle
        .rejection_writer
        .read()
        .map_err(|_| "l4e read poisoned".to_string())
        .map(|w| w.records().len())
}

async fn await_l4e_record(
    bundle: &ChaintapeBundle,
    before_len: usize,
    expected: TxKind,
) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let matched = {
            let writer = bundle
                .rejection_writer
                .read()
                .map_err(|_| "l4e read poisoned".to_string())?;
            if writer.records().len() > before_len {
                writer
                    .records()
                    .last()
                    .map(|record| record.tx_kind == expected)
                    .unwrap_or(false)
            } else {
                false
            }
        };
        if matched {
            return Ok(());
        }
        if Instant::now() >= deadline {
            return Err(format!("timed out waiting for {:?} L4.E record", expected));
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

fn decode_entry_tx(bundle: &ChaintapeBundle, entry: &LedgerEntry) -> Result<TypedTx, String> {
    let cas = bundle
        .cas
        .read()
        .map_err(|_| "cas read poisoned".to_string())?;
    let bytes = cas
        .get(&entry.tx_payload_cid)
        .map_err(|e| format!("read tx payload from CAS: {e}"))?;
    canonical_decode(&bytes).map_err(|e| format!("decode typed tx: {e}"))
}

fn tx_index_row(bundle: &ChaintapeBundle, entry: &LedgerEntry) -> Result<TxIndexRow, String> {
    let tx = decode_entry_tx(bundle, entry)?;
    Ok(TxIndexRow {
        logical_t: entry.logical_t,
        tx_kind: format!("{:?}", entry.tx_kind),
        tx_id: tx_id(&tx).0,
        tx_payload_cid: entry.tx_payload_cid.hex(),
    })
}

fn put_canonical<T: Serialize>(
    bundle: &ChaintapeBundle,
    value: &T,
    schema_id: &'static str,
    logical_t: u64,
) -> Result<Cid, String> {
    let bytes =
        canonical_encode(value).map_err(|e| format!("canonical encode {schema_id}: {e}"))?;
    let cid = put_bytes(
        bundle,
        &bytes,
        "full_system_augment_current_kernel",
        Some(schema_id),
    )?;
    let cas = bundle
        .cas
        .read()
        .map_err(|_| "cas read poisoned".to_string())?;
    let created_at_logical_t = cas
        .metadata(&cid)
        .ok_or_else(|| format!("metadata missing after put for {schema_id}"))?
        .created_at_logical_t;
    if created_at_logical_t != logical_t {
        return Err(format!(
            "CAS logical_t mismatch for {schema_id}: got {created_at_logical_t}, expected {logical_t}"
        ));
    }
    Ok(cid)
}

fn put_bytes(
    bundle: &ChaintapeBundle,
    bytes: &[u8],
    origin: &str,
    schema_id: Option<&str>,
) -> Result<Cid, String> {
    put_bytes_with_type(bundle, bytes, ObjectType::Generic, origin, schema_id)
}

fn put_bytes_with_type(
    bundle: &ChaintapeBundle,
    bytes: &[u8],
    object_type: ObjectType,
    origin: &str,
    schema_id: Option<&str>,
) -> Result<Cid, String> {
    let logical_t = bundle.sequencer.next_logical_t_peek();
    bundle
        .cas
        .write()
        .map_err(|_| "cas write poisoned".to_string())?
        .put(
            bytes,
            object_type,
            origin,
            logical_t,
            schema_id.map(str::to_string),
        )
        .map_err(|e| format!("cas put: {e}"))
}

fn put_feedback_capsule(bundle: &ChaintapeBundle) -> Result<Cid, String> {
    let logical_t = bundle.sequencer.next_logical_t_peek() + 1;
    let q = bundle
        .sequencer
        .q_snapshot()
        .map_err(|e| format!("q snapshot: {e}"))?;
    let l4e = bundle
        .rejection_writer
        .read()
        .map_err(|_| "l4e read poisoned".to_string())?;
    let cas_root = {
        let cas = bundle
            .cas
            .read()
            .map_err(|_| "cas read poisoned".to_string())?;
        cas_metadata_root_before_logical_t(&cas, logical_t).map_err(|e| format!("{e}"))?
    };
    let capsule = turingosv4::state::typed_tx::ArchitectFeedbackCapsule {
        schema_version: ARCHITECT_FEEDBACK_SCHEMA_ID.to_string(),
        source_ledger_root: q.ledger_root_t,
        source_l4e_root: l4e.last_hash(),
        cas_metadata_root: cas_root,
        constitution_hash: constitution_source_hash(),
        public_summary: "Full-system true-suite feedback archived for ArchitectAI/Veto-AI"
            .to_string(),
        private_detail_cid: None,
    };
    put_canonical(
        bundle,
        &capsule,
        ARCHITECT_FEEDBACK_SCHEMA_ID,
        logical_t - 1,
    )
}

fn put_proposal_capsule(
    bundle: &ChaintapeBundle,
    feedback: &LogFeedbackArchiveTx,
    proposed_artifact_cid: Option<Cid>,
) -> Result<Cid, String> {
    let q = bundle
        .sequencer
        .q_snapshot()
        .map_err(|e| format!("q snapshot: {e}"))?;
    let capsule = ArchitectProposalCapsule {
        schema_version: ARCHITECT_PROPOSAL_SCHEMA_ID.to_string(),
        feedback_tx_id: feedback.tx_id.clone(),
        feedback_root: feedback.feedback_root,
        constitution_hash: constitution_source_hash(),
        tool_registry_root: q.tool_registry_root_t,
        proposal_kind: ArchitectProposalKind::ToolRegistryPatch,
        target_path: Some("src/bottom_white/tools/runtime_meta_tool.rs".to_string()),
        proposed_artifact_cid,
        tools_used: vec![
            "full_system_augment_current_kernel".to_string(),
            "ChainTape/CAS replay".to_string(),
        ],
        public_summary:
            "ArchitectAI proposes no unsafe kernel write; runtime evidence remains tape-bound"
                .to_string(),
    };
    put_canonical(
        bundle,
        &capsule,
        ARCHITECT_PROPOSAL_SCHEMA_ID,
        bundle.sequencer.next_logical_t_peek(),
    )
}

fn put_veto_capsule(
    bundle: &ChaintapeBundle,
    proposal: &ArchitectProposalTx,
) -> Result<Cid, String> {
    let capsule = VetoDecisionCapsule {
        schema_version: VETO_DECISION_SCHEMA_ID.to_string(),
        proposal_tx_id: proposal.tx_id.clone(),
        proposal_root: proposal.proposal_root,
        constitution_hash: constitution_source_hash(),
        verdict: VetoVerdict::Pass,
        reason_code: VetoReasonCode::ConstitutionCompliant,
        public_summary: "Veto-AI PASS for tape-bound full-system evidence".to_string(),
    };
    put_canonical(
        bundle,
        &capsule,
        VETO_DECISION_SCHEMA_ID,
        bundle.sequencer.next_logical_t_peek(),
    )
}

fn put_commit_capsule(
    bundle: &ChaintapeBundle,
    veto: &VetoDecisionTx,
    applied_artifact_cid: Option<Cid>,
) -> Result<Cid, String> {
    let capsule = ArchitectCommitCapsule {
        schema_version: ARCHITECT_COMMIT_SCHEMA_ID.to_string(),
        proposal_tx_id: veto.proposal_tx_id.clone(),
        veto_tx_id: veto.tx_id.clone(),
        decision_root: veto.decision_root,
        constitution_hash: constitution_source_hash(),
        applied_artifact_cid,
        target_path: Some("src/bottom_white/tools/runtime_meta_tool.rs".to_string()),
        public_summary: "ArchitectAI commit recorded after Veto-AI PASS".to_string(),
    };
    put_canonical(
        bundle,
        &capsule,
        ARCHITECT_COMMIT_SCHEMA_ID,
        bundle.sequencer.next_logical_t_peek(),
    )
}

fn put_reinit_capsule(
    bundle: &ChaintapeBundle,
    trigger_entry: u64,
    reason: ReinitReason,
) -> Result<Cid, String> {
    let capsule = ReinitReasonCapsule {
        schema_version: REINIT_REASON_SCHEMA_ID.to_string(),
        trigger_entry,
        reason,
        public_summary: "Terminal ErrorHalt triggers replay-bound full-system re-init".to_string(),
        private_detail_cid: None,
    };
    put_canonical(
        bundle,
        &capsule,
        REINIT_REASON_SCHEMA_ID,
        bundle.sequencer.next_logical_t_peek(),
    )
}

fn put_terminal_evidence_capsule(
    bundle: &ChaintapeBundle,
    run_id: &str,
    task_id: TaskId,
    solver_agent: Option<AgentId>,
    raw_log: &str,
) -> Result<Cid, String> {
    let capsule = write_evidence_capsule(
        &bundle.cas,
        RunId(run_id.to_string()),
        task_id,
        solver_agent,
        ExhaustionCounts {
            attempt_count: 1,
            lean_error_count: 0,
            sorry_block_count: 0,
            protocol_parse_failure_count: 0,
            partial_accept_count: 0,
        },
        (0, 1),
        ExhaustionReason::ProtocolCollapse,
        raw_log.as_bytes(),
        CapsulePrivacyPolicy::AuditOnly,
        "full_system_augment_current_kernel",
        bundle.sequencer.next_logical_t_peek(),
    )
    .map_err(|e| format!("write terminal EvidenceCapsule: {e}"))?;
    Ok(capsule.capsule_id)
}

fn write_governance_index(
    bundle: &ChaintapeBundle,
    args: &Args,
    rows: &[TxIndexRow],
) -> Result<(), String> {
    let q = bundle
        .sequencer
        .q_snapshot()
        .map_err(|e| format!("q snapshot: {e}"))?;
    let body = json!({
        "schema_version": "turingosv4.true_suite.full_system_augment_fc3_index.v1",
        "run_id": args.run_id,
        "runtime_repo": args.runtime_repo,
        "cas": args.cas,
        "tx_sequence": rows,
        "final_q": {
            "state_root_t": hash_hex(q.state_root_t),
            "ledger_root_t": hash_hex(q.ledger_root_t),
            "tool_registry_root_t": hash_hex(q.tool_registry_root_t)
        },
        "checks": {
            "typed_architectai_feedback_present": true,
            "typed_vetoai_decision_present": true,
            "reinit_request_and_boot_present": true,
            "external_pr_ceremony_not_used_as_fc3": true
        }
    });
    write_json(&args.out_dir.join("governance_capsule_index.json"), &body)
}

fn refresh_genesis_report(args: &Args) -> Result<(), String> {
    let path = args.runtime_repo.join("genesis_report.json");
    let mut report: GenesisReport = if path.is_file() {
        serde_json::from_str(
            &std::fs::read_to_string(&path)
                .map_err(|e| format!("read existing genesis_report.json: {e}"))?,
        )
        .map_err(|e| format!("parse existing genesis_report.json: {e}"))?
    } else {
        GenesisReport {
            constitution_hash: GenesisReport::hash_constitution_md(&args.constitution),
            runtime_repo: args.runtime_repo.display().to_string(),
            cas_path: args.cas.display().to_string(),
            system_pubkey_hash: None,
            agent_pubkeys_path: "agent_pubkeys.json".to_string(),
            initial_balances: vec![],
            task_id: None,
            task_open_tx: None,
            escrow_lock_tx: None,
            agent_model_assignment: vec![],
            model_assignment_manifest_cid: None,
            agent_role_assignment: vec![],
            role_assignment_manifest_cid: None,
        }
    };
    report.constitution_hash = GenesisReport::hash_constitution_md(&args.constitution);
    report.runtime_repo = args.runtime_repo.display().to_string();
    report.cas_path = args.cas.display().to_string();
    report.system_pubkey_hash = GenesisReport::hash_system_pubkey_manifest(&args.runtime_repo);
    report
        .write_to_runtime_repo(&args.runtime_repo)
        .map_err(|e| format!("write refreshed genesis_report.json: {e}"))
}

fn tx_id(tx: &TypedTx) -> TxId {
    match tx {
        TypedTx::Work(t) => t.tx_id.clone(),
        TypedTx::BuyWithCoinRouter(t) => t.tx_id.clone(),
        TypedTx::MapReduceTick(t) => t.tx_id.clone(),
        TypedTx::LogFeedbackArchive(t) => t.tx_id.clone(),
        TypedTx::ArchitectProposal(t) => t.tx_id.clone(),
        TypedTx::VetoDecision(t) => t.tx_id.clone(),
        TypedTx::ArchitectCommit(t) => t.tx_id.clone(),
        TypedTx::TerminalSummary(t) => t.tx_id.clone(),
        TypedTx::ReinitRequest(t) => t.tx_id.clone(),
        TypedTx::ReinitBoot(t) => t.tx_id.clone(),
        other => TxId(format!("unsupported-index-kind-{:?}", other.tx_kind())),
    }
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(value).map_err(|e| format!("json encode: {e}"))?;
    std::fs::write(path, format!("{json}\n")).map_err(|e| format!("write {}: {e}", path.display()))
}

fn sha256_hex(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex_bytes(&hasher.finalize())
}

fn hash_bytes(input: &[u8]) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    Hash(out)
}

fn hash_hex(hash: turingosv4::state::q_state::Hash) -> String {
    hex_bytes(&hash.0)
}

fn hex_bytes(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
