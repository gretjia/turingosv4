//! True-suite full-system participation report helper.
//!
//! This binary is runner-side evidence accounting. It reads on-disk ChainTape,
//! CAS, replay, and runner manifests, then writes
//! `full_system_participation.json`. It does not mutate the kernel, submit new
//! transactions, or replace replay as source of truth.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde::Serialize;
use serde_json::Value;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::transition_ledger::{Git2LedgerWriter, LedgerWriter};
use turingosv4::runtime::audit_assertions::TxKindCounts;
use turingosv4::runtime::market_decision_trace_summary::MarketDecisionTraceSummary;
use turingosv4::runtime::market_opportunity_trace::market_opportunity_trace_cids;
use turingosv4::runtime::verify::ReplayReport;

const SCHEMA_VERSION: &str = "turingosv4.true_suite.full_system_participation.v1";

#[derive(Debug)]
struct Args {
    run_id: String,
    family_id: String,
    entrypoint: String,
    runtime_repo: PathBuf,
    cas: PathBuf,
    replay_report: PathBuf,
    genesis_report: Option<PathBuf>,
    domain_manifest: Option<PathBuf>,
    fc3_index: Option<PathBuf>,
    out: PathBuf,
    require_full_system: bool,
}

fn usage() -> &'static str {
    "usage: full_system_participation_current_kernel \
     --run-id <ID> --family-id <ID> --entrypoint <PATH> \
     --runtime-repo <PATH> --cas <PATH> --replay-report <PATH> --out <PATH> \
     [--genesis-report <PATH>] [--domain-manifest <PATH>] [--fc3-index <PATH>] \
     [--require-full-system]"
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut run_id = None;
    let mut family_id = None;
    let mut entrypoint = None;
    let mut runtime_repo = None;
    let mut cas = None;
    let mut replay_report = None;
    let mut genesis_report = None;
    let mut domain_manifest = None;
    let mut fc3_index = None;
    let mut out = None;
    let mut require_full_system = false;

    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "--run-id" => {
                i += 1;
                run_id = Some(argv.get(i).ok_or("--run-id requires value")?.clone());
            }
            "--family-id" => {
                i += 1;
                family_id = Some(argv.get(i).ok_or("--family-id requires value")?.clone());
            }
            "--entrypoint" => {
                i += 1;
                entrypoint = Some(argv.get(i).ok_or("--entrypoint requires value")?.clone());
            }
            "--runtime-repo" => {
                i += 1;
                runtime_repo = Some(argv.get(i).ok_or("--runtime-repo requires value")?.into());
            }
            "--cas" => {
                i += 1;
                cas = Some(argv.get(i).ok_or("--cas requires value")?.into());
            }
            "--replay-report" => {
                i += 1;
                replay_report = Some(argv.get(i).ok_or("--replay-report requires value")?.into());
            }
            "--genesis-report" => {
                i += 1;
                genesis_report = Some(argv.get(i).ok_or("--genesis-report requires value")?.into());
            }
            "--domain-manifest" => {
                i += 1;
                domain_manifest = Some(
                    argv.get(i)
                        .ok_or("--domain-manifest requires value")?
                        .into(),
                );
            }
            "--fc3-index" => {
                i += 1;
                fc3_index = Some(argv.get(i).ok_or("--fc3-index requires value")?.into());
            }
            "--out" => {
                i += 1;
                out = Some(argv.get(i).ok_or("--out requires value")?.into());
            }
            "--require-full-system" => require_full_system = true,
            "--help" | "-h" => return Err(usage().into()),
            other => return Err(format!("unknown arg: {other}")),
        }
        i += 1;
    }

    Ok(Args {
        run_id: run_id.ok_or("--run-id required")?,
        family_id: family_id.ok_or("--family-id required")?,
        entrypoint: entrypoint.ok_or("--entrypoint required")?,
        runtime_repo: runtime_repo.ok_or("--runtime-repo required")?,
        cas: cas.ok_or("--cas required")?,
        replay_report: replay_report.ok_or("--replay-report required")?,
        genesis_report,
        domain_manifest,
        fc3_index,
        out: out.ok_or("--out required")?,
        require_full_system,
    })
}

#[derive(Debug, Serialize)]
struct EvidencePaths {
    runtime_repo: String,
    cas: String,
    replay_report: String,
    genesis_report: Option<String>,
    domain_manifest: Option<String>,
    fc3_index: Option<String>,
}

#[derive(Debug, Serialize)]
struct ReplaySummary {
    l4_entries: u64,
    l4e_entries: u64,
    ledger_root_verified: bool,
    system_signatures_verified: bool,
    state_reconstructed: bool,
    economic_state_reconstructed: bool,
    cas_payloads_retrievable: bool,
    agent_signatures_verified: bool,
    proposal_telemetry_cas_retrievable: bool,
    all_indicators_pass: bool,
    initial_q_state_loaded_from_disk: bool,
    final_state_root_hex: Option<String>,
    final_ledger_root_hex: Option<String>,
    head_commit_oid_hex: Option<String>,
}

#[derive(Debug, Serialize)]
struct Fc1Participation {
    present: bool,
    accepted_work_txs: u64,
    l4e_entries: u64,
    work_or_rejection_landed: bool,
    proposal_telemetry_cas_retrievable: bool,
    domain_manifest_work_tx_landed: Option<bool>,
}

#[derive(Debug, Serialize)]
struct Fc2Participation {
    present: bool,
    genesis_report_present: bool,
    initial_q_state_loaded_from_disk: bool,
    map_reduce_tick_present: bool,
    replay_verified: bool,
}

#[derive(Debug, Serialize)]
struct Fc3Participation {
    present: bool,
    typed_meta_roles_present: bool,
    reinit_semantics_present: bool,
    log_feedback_archive_txs: u64,
    architect_proposal_txs: u64,
    veto_decision_txs: u64,
    architect_commit_txs: u64,
    reinit_request_txs: u64,
    reinit_boot_txs: u64,
    fc3_index_present: bool,
    external_pr_ceremony_used_as_fc3: bool,
}

#[derive(Debug, Serialize)]
struct MarketParticipation {
    present: bool,
    mode: String,
    l4_market_tx_count: u64,
    market_seed_txs: u64,
    cpmm_pool_txs: u64,
    cpmm_swap_txs: u64,
    buy_with_coin_router_txs: u64,
    event_resolve_txs: u64,
    agent_market_action_txs: u64,
    market_decision_trace_count: u64,
    market_decision_submitted_count: u64,
    market_decision_no_trade_count: u64,
    market_decision_declined_count: u64,
    market_opportunity_trace_count: u64,
}

#[derive(Debug, Serialize)]
struct Verdict {
    full_system_participation: bool,
    full_system_verdict: String,
    missing: Vec<String>,
    final_closure_possible: bool,
}

#[derive(Debug, Serialize)]
struct FlowchartNodeReceipt {
    flowchart: &'static str,
    node_id: &'static str,
    label: &'static str,
    status: String,
    receipt_kind: &'static str,
    evidence: Vec<String>,
}

#[derive(Debug, Serialize)]
struct FlowchartNodeReceipts {
    verdict: String,
    missing: Vec<String>,
    fc1: Vec<FlowchartNodeReceipt>,
    fc2: Vec<FlowchartNodeReceipt>,
    fc3: Vec<FlowchartNodeReceipt>,
}

#[derive(Debug, Serialize)]
struct FullSystemParticipationReport {
    schema_version: &'static str,
    run_id: String,
    family_id: String,
    entrypoint: String,
    authority: &'static str,
    evidence_paths: EvidencePaths,
    replay: ReplaySummary,
    tx_kind_counts: TxKindCounts,
    tx_kind_counts_source: &'static str,
    tx_kind_sequence: Vec<String>,
    fc1: Fc1Participation,
    fc2: Fc2Participation,
    fc3: Fc3Participation,
    market: MarketParticipation,
    flowchart_node_receipts: FlowchartNodeReceipts,
    domain_manifest: Option<Value>,
    verdict: Verdict,
}

fn read_json<T: serde::de::DeserializeOwned>(path: &PathBuf) -> Result<T, String> {
    let raw = std::fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse {}: {e}", path.display()))
}

fn optional_json(path: &Option<PathBuf>) -> Result<Option<Value>, String> {
    match path {
        Some(path) => read_json(path).map(Some),
        None => Ok(None),
    }
}

fn first_existing_file(candidates: Vec<PathBuf>) -> Option<PathBuf> {
    candidates.into_iter().find(|path| path.is_file())
}

fn sibling_named(path: &Path, file_name: &str) -> Option<PathBuf> {
    path.parent().map(|parent| parent.join(file_name))
}

fn effective_fc3_index_path(args: &Args) -> Option<PathBuf> {
    if let Some(path) = &args.fc3_index {
        return Some(path.clone());
    }

    let mut candidates = Vec::new();
    if let Some(path) = sibling_named(&args.out, "governance_capsule_index.json") {
        candidates.push(path);
    }
    if let Some(path) = sibling_named(&args.replay_report, "governance_capsule_index.json") {
        candidates.push(path);
    }
    if let Some(path) = args
        .genesis_report
        .as_ref()
        .and_then(|genesis| sibling_named(genesis, "governance_capsule_index.json"))
    {
        candidates.push(path);
    }
    if let Some(parent) = args.runtime_repo.parent() {
        candidates.push(parent.join("governance_capsule_index.json"));
    }
    first_existing_file(candidates)
}

fn bool_field(value: Option<&Value>, key: &str) -> Option<bool> {
    value.and_then(|v| v.get(key)).and_then(Value::as_bool)
}

fn tx_entries(
    runtime_repo: &PathBuf,
) -> Result<
    (
        Vec<turingosv4::bottom_white::ledger::transition_ledger::LedgerEntry>,
        Vec<String>,
    ),
    String,
> {
    if !runtime_repo.join(".git").is_dir() {
        return Err(format!(
            "runtime repo must already be an initialized ChainTape git store: {}",
            runtime_repo.display()
        ));
    }
    let writer =
        Git2LedgerWriter::open(runtime_repo).map_err(|e| format!("open L4 writer: {e}"))?;
    let mut entries = Vec::new();
    let mut sequence = Vec::new();
    for logical_t in 1..=writer.len() {
        let entry = writer
            .read_at(logical_t)
            .map_err(|e| format!("read L4 entry {logical_t}: {e}"))?;
        sequence.push(format!("{:?}", entry.tx_kind));
        entries.push(entry);
    }
    Ok((entries, sequence))
}

fn l4_market_tx_count(c: &TxKindCounts) -> u64 {
    c.market_seed
        + c.complete_set_mint
        + c.complete_set_redeem
        + c.complete_set_merge
        + c.cpmm_pool
        + c.cpmm_swap
        + c.buy_with_coin_router
        + c.event_resolve
}

fn no_trade_count(summary: &MarketDecisionTraceSummary) -> u64 {
    summary.outcome_counts.get("no_trade").copied().unwrap_or(0)
}

fn declined_count(summary: &MarketDecisionTraceSummary) -> u64 {
    summary.outcome_counts.get("declined").copied().unwrap_or(0)
}

fn build_report(args: Args) -> Result<(FullSystemParticipationReport, bool), String> {
    let replay: ReplayReport = read_json(&args.replay_report)?;
    let replay_green = replay.all_indicators_pass();
    let domain_manifest = optional_json(&args.domain_manifest)?;
    let fc3_index_path = effective_fc3_index_path(&args);
    let fc3_index = optional_json(&fc3_index_path)?;
    let (entries, tx_kind_sequence) = tx_entries(&args.runtime_repo)?;
    let counts = TxKindCounts::from_entries(&entries);
    let cas = CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
    let market_summary = MarketDecisionTraceSummary::compute_from_cas(&cas);
    let market_opportunity_count = market_opportunity_trace_cids(&cas).len() as u64;
    let l4_market_count = l4_market_tx_count(&counts);
    let agent_market_action_count = counts.complete_set_mint
        + counts.complete_set_redeem
        + counts.complete_set_merge
        + counts.cpmm_swap
        + counts.buy_with_coin_router;
    let market_no_trade_count = no_trade_count(&market_summary);
    let market_declined_count = declined_count(&market_summary);

    let fc1 = Fc1Participation {
        present: counts.work > 0 || replay.l4e_entries > 0,
        accepted_work_txs: counts.work,
        l4e_entries: replay.l4e_entries,
        work_or_rejection_landed: counts.work > 0 || replay.l4e_entries > 0,
        proposal_telemetry_cas_retrievable: replay.proposal_telemetry_cas_retrievable,
        domain_manifest_work_tx_landed: bool_field(domain_manifest.as_ref(), "work_tx_landed"),
    };

    let genesis_report_present = args
        .genesis_report
        .as_ref()
        .is_some_and(|path| path.is_file())
        || args.runtime_repo.join("genesis_report.json").is_file();
    let fc2 = Fc2Participation {
        present: genesis_report_present
            && replay.detail.initial_q_state_loaded_from_disk
            && counts.map_reduce_tick > 0
            && replay_green,
        genesis_report_present,
        initial_q_state_loaded_from_disk: replay.detail.initial_q_state_loaded_from_disk,
        map_reduce_tick_present: counts.map_reduce_tick > 0,
        replay_verified: replay_green,
    };

    let fc3_typed = counts.log_feedback_archive > 0
        && counts.architect_proposal > 0
        && counts.veto_decision > 0;
    let fc3_reinit = counts.reinit_request > 0 && counts.reinit_boot > 0;
    let fc3 = Fc3Participation {
        present: fc3_typed,
        typed_meta_roles_present: fc3_typed,
        reinit_semantics_present: fc3_reinit,
        log_feedback_archive_txs: counts.log_feedback_archive,
        architect_proposal_txs: counts.architect_proposal,
        veto_decision_txs: counts.veto_decision,
        architect_commit_txs: counts.architect_commit,
        reinit_request_txs: counts.reinit_request,
        reinit_boot_txs: counts.reinit_boot,
        fc3_index_present: fc3_index.is_some(),
        external_pr_ceremony_used_as_fc3: false,
    };

    let market_mode = if agent_market_action_count > 0 && market_summary.submitted_count > 0 {
        "invest"
    } else if market_summary.submitted_count > 0 {
        "submitted_trace_missing_l4_market_action"
    } else if market_no_trade_count > 0 || market_declined_count > 0 {
        "abstain_with_tape_visible_market_decision"
    } else if l4_market_count > 0 {
        "structural_market_only_missing_agent_choice"
    } else if market_opportunity_count > 0 {
        "opportunity_visible_missing_agent_choice"
    } else {
        "missing"
    };
    let market = MarketParticipation {
        present: market_mode == "invest"
            || market_mode == "abstain_with_tape_visible_market_decision",
        mode: market_mode.to_string(),
        l4_market_tx_count: l4_market_count,
        market_seed_txs: counts.market_seed,
        cpmm_pool_txs: counts.cpmm_pool,
        cpmm_swap_txs: counts.cpmm_swap,
        buy_with_coin_router_txs: counts.buy_with_coin_router,
        event_resolve_txs: counts.event_resolve,
        agent_market_action_txs: agent_market_action_count,
        market_decision_trace_count: market_summary.total_traces,
        market_decision_submitted_count: market_summary.submitted_count,
        market_decision_no_trade_count: market_no_trade_count,
        market_decision_declined_count: market_declined_count,
        market_opportunity_trace_count: market_opportunity_count,
    };

    let mut missing = Vec::new();
    if !fc1.present {
        missing.push("FC1_runtime_work_or_l4e".to_string());
    }
    if !fc2.present {
        missing.push("FC2_boot_tick_replay".to_string());
    }
    if !fc3.typed_meta_roles_present {
        missing.push("FC3_typed_architect_veto_feedback".to_string());
    }
    if !fc3.reinit_semantics_present {
        missing.push("FC3_reinit_semantics".to_string());
    }
    if !market.present {
        missing.push("market_economy_invest_or_visible_abstention".to_string());
    }
    if !replay_green {
        missing.push("replay_all_indicators_pass".to_string());
    }

    let flowchart_node_receipts = build_flowchart_node_receipts(
        &args,
        &fc3_index_path,
        &replay,
        replay_green,
        &counts,
        &fc1,
        &fc2,
        &fc3,
        &market,
    );
    if flowchart_node_receipts.verdict != "FLOW-PASS" {
        missing.extend(flowchart_node_receipts.missing.clone());
    }
    let full = missing.is_empty();
    let report = FullSystemParticipationReport {
        schema_version: SCHEMA_VERSION,
        run_id: args.run_id,
        family_id: args.family_id,
        entrypoint: args.entrypoint,
        authority: "ChainTape + CAS + replay verifier; stdout/dashboard are non-authoritative",
        evidence_paths: EvidencePaths {
            runtime_repo: args.runtime_repo.display().to_string(),
            cas: args.cas.display().to_string(),
            replay_report: args.replay_report.display().to_string(),
            genesis_report: args
                .genesis_report
                .as_ref()
                .map(|p| p.display().to_string()),
            domain_manifest: args
                .domain_manifest
                .as_ref()
                .map(|p| p.display().to_string()),
            fc3_index: fc3_index_path.as_ref().map(|p| p.display().to_string()),
        },
        replay: ReplaySummary {
            l4_entries: replay.l4_entries,
            l4e_entries: replay.l4e_entries,
            ledger_root_verified: replay.ledger_root_verified,
            system_signatures_verified: replay.system_signatures_verified,
            state_reconstructed: replay.state_reconstructed,
            economic_state_reconstructed: replay.economic_state_reconstructed,
            cas_payloads_retrievable: replay.cas_payloads_retrievable,
            agent_signatures_verified: replay.agent_signatures_verified,
            proposal_telemetry_cas_retrievable: replay.proposal_telemetry_cas_retrievable,
            all_indicators_pass: replay_green,
            initial_q_state_loaded_from_disk: replay.detail.initial_q_state_loaded_from_disk,
            final_state_root_hex: replay.detail.final_state_root_hex,
            final_ledger_root_hex: replay.detail.final_ledger_root_hex,
            head_commit_oid_hex: replay.detail.head_commit_oid_hex,
        },
        tx_kind_counts: counts,
        tx_kind_counts_source: "derived from Git2LedgerWriter L4 entries",
        tx_kind_sequence,
        fc1,
        fc2,
        fc3,
        market,
        flowchart_node_receipts,
        domain_manifest,
        verdict: Verdict {
            full_system_participation: full,
            full_system_verdict: if full {
                "FULL_SYSTEM_LIT".to_string()
            } else {
                "PARTIAL_RUNNER_ONLY".to_string()
            },
            missing,
            final_closure_possible: false,
        },
    };
    Ok((report, full))
}

#[allow(clippy::too_many_arguments)]
fn build_flowchart_node_receipts(
    args: &Args,
    fc3_index_path: &Option<PathBuf>,
    replay: &ReplayReport,
    replay_green: bool,
    counts: &TxKindCounts,
    fc1: &Fc1Participation,
    fc2: &Fc2Participation,
    fc3: &Fc3Participation,
    market: &MarketParticipation,
) -> FlowchartNodeReceipts {
    let mut missing = Vec::new();
    let l4 = replay.l4_entries;
    let l4e = replay.l4e_entries;
    let runtime_repo = args.runtime_repo.display().to_string();
    let cas = args.cas.display().to_string();
    let replay_path = args.replay_report.display().to_string();
    let genesis = args
        .genesis_report
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| {
            args.runtime_repo
                .join("genesis_report.json")
                .display()
                .to_string()
        });
    let domain = args
        .domain_manifest
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "domain_manifest_absent".to_string());
    let fc3_index = args
        .fc3_index
        .as_ref()
        .or(fc3_index_path.as_ref())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "fc3_index_absent".to_string());

    let fc1_receipts = vec![
        receipt(
            "FC1",
            "FC1-N1",
            "Q_t loaded",
            replay.detail.initial_q_state_loaded_from_disk,
            "initial_q_state",
            vec![runtime_repo.clone(), replay_path.clone()],
            &mut missing,
        ),
        receipt(
            "FC1",
            "FC1-N2",
            "q_t state root",
            replay.state_reconstructed,
            "replay_state_root",
            vec![format!(
                "final_state_root={:?}",
                replay.detail.final_state_root_hex
            )],
            &mut missing,
        ),
        receipt(
            "FC1",
            "FC1-N3",
            "HEAD_t / ChainTape head",
            replay.ledger_root_verified,
            "git_l4_head",
            vec![format!(
                "head_commit={:?}",
                replay.detail.head_commit_oid_hex
            )],
            &mut missing,
        ),
        receipt(
            "FC1",
            "FC1-N4",
            "tape_t / CAS",
            replay.cas_payloads_retrievable,
            "cas_replay",
            vec![cas.clone()],
            &mut missing,
        ),
        receipt(
            "FC1",
            "FC1-N5",
            "rtool read set",
            fc1.proposal_telemetry_cas_retrievable,
            "proposal_telemetry_cas",
            vec![domain.clone()],
            &mut missing,
        ),
        receipt(
            "FC1",
            "FC1-N6",
            "input capsule",
            fc1.proposal_telemetry_cas_retrievable,
            "proposal_telemetry_cas",
            vec![domain.clone()],
            &mut missing,
        ),
        receipt(
            "FC1",
            "FC1-N7",
            "agent delta",
            counts.work > 0,
            "WorkTx",
            vec![format!("accepted_work_txs={}", counts.work)],
            &mut missing,
        ),
        receipt(
            "FC1",
            "FC1-N8",
            "agent output",
            fc1.work_or_rejection_landed,
            "L4_or_L4E",
            vec![format!("l4={l4} l4e={l4e}")],
            &mut missing,
        ),
        receipt(
            "FC1",
            "FC1-N9",
            "action/output object",
            counts.work > 0 || market.agent_market_action_txs > 0,
            "typed_tx",
            vec![format!(
                "work={} market_actions={}",
                counts.work, market.agent_market_action_txs
            )],
            &mut missing,
        ),
        receipt(
            "FC1",
            "FC1-N10",
            "tool/economy output",
            market.present,
            "market_tx",
            vec![format!("market_mode={}", market.mode)],
            &mut missing,
        ),
        receipt(
            "FC1",
            "FC1-N11",
            "predicate check",
            replay_green,
            "replay_predicates",
            vec![replay_path.clone()],
            &mut missing,
        ),
        receipt(
            "FC1",
            "FC1-N12",
            "predicate verdict split",
            counts.work > 0 && replay.l4e_entries > 0,
            "L4_and_L4E",
            vec![format!(
                "accepted_work={} l4e={}",
                counts.work, replay.l4e_entries
            )],
            &mut missing,
        ),
        receipt(
            "FC1",
            "FC1-N13",
            "wtool append",
            replay.ledger_root_verified && l4 > 0,
            "L4_append",
            vec![format!("l4_entries={l4}")],
            &mut missing,
        ),
        receipt(
            "FC1",
            "FC1-N14",
            "accepted branch",
            counts.work > 0,
            "accepted_L4_WorkTx",
            vec![format!("accepted_work={}", counts.work)],
            &mut missing,
        ),
        receipt(
            "FC1",
            "FC1-N15",
            "rejected branch",
            replay.l4e_entries > 0,
            "L4E_rejection",
            vec![format!("l4e_entries={}", replay.l4e_entries)],
            &mut missing,
        ),
    ];

    let fc2_receipts = vec![
        receipt(
            "FC2",
            "FC2-N16",
            "InitAI / boot entry",
            fc2.initial_q_state_loaded_from_disk,
            "initial_q_state",
            vec![runtime_repo.clone()],
            &mut missing,
        ),
        receipt_with_status(
            "FC2",
            "FC2-N17",
            "human/project spec",
            fc2.genesis_report_present,
            "document-pinned",
            "genesis_report",
            vec![genesis.clone()],
            &mut missing,
        ),
        receipt_with_status(
            "FC2",
            "FC2-N18",
            "constitution/law",
            fc2.genesis_report_present,
            "document-pinned",
            "constitution_hash",
            vec![genesis.clone()],
            &mut missing,
        ),
        receipt(
            "FC2",
            "FC2-N19",
            "predicate boot",
            fc2.genesis_report_present || counts.predicate_binding_activate > 0,
            "predicate_boot_or_genesis",
            vec![
                format!(
                    "predicate_binding_activate={}",
                    counts.predicate_binding_activate
                ),
                genesis.clone(),
            ],
            &mut missing,
        ),
        receipt(
            "FC2",
            "FC2-N20",
            "map-reduce tick",
            counts.map_reduce_tick > 0,
            "MapReduceTick",
            vec![format!("map_reduce_tick={}", counts.map_reduce_tick)],
            &mut missing,
        ),
        receipt(
            "FC2",
            "FC2-N21",
            "Q0",
            fc2.initial_q_state_loaded_from_disk,
            "initial_q_state",
            vec![runtime_repo.clone()],
            &mut missing,
        ),
        receipt(
            "FC2",
            "FC2-N22",
            "halt",
            counts.terminal_summary > 0,
            "TerminalSummary",
            vec![format!("terminal_summary={}", counts.terminal_summary)],
            &mut missing,
        ),
        receipt(
            "FC2",
            "FC2-N23",
            "halt reason",
            counts.terminal_summary > 0,
            "TerminalSummary/ErrorHalt",
            vec![format!("terminal_summary={}", counts.terminal_summary)],
            &mut missing,
        ),
        receipt(
            "FC2",
            "FC2-N24",
            "replay verifier",
            replay_green,
            "verify_chaintape",
            vec![replay_path.clone()],
            &mut missing,
        ),
        receipt(
            "FC2",
            "FC2-N25",
            "CAS evidence",
            replay.cas_payloads_retrievable,
            "CAS",
            vec![cas.clone()],
            &mut missing,
        ),
        receipt(
            "FC2",
            "FC2-N26",
            "tool/runtime state",
            counts.task_open > 0 || counts.work > 0 || market.present,
            "task_or_market_tx",
            vec![format!(
                "task_open={} work={} market={}",
                counts.task_open, counts.work, market.l4_market_tx_count
            )],
            &mut missing,
        ),
        receipt(
            "FC2",
            "FC2-N27",
            "tick persisted to tape",
            fc2.map_reduce_tick_present,
            "MapReduceTick_L4",
            vec![format!("map_reduce_tick={}", counts.map_reduce_tick)],
            &mut missing,
        ),
        receipt(
            "FC2",
            "FC2-N28",
            "tools other / economy",
            market.present,
            "market_economy_tx",
            vec![format!("market_tx_count={}", market.l4_market_tx_count)],
            &mut missing,
        ),
    ];

    let fc3_receipts = vec![
        receipt(
            "FC3",
            "FC3-N29",
            "boot",
            fc2.initial_q_state_loaded_from_disk && replay_green,
            "boot_replay",
            vec![runtime_repo, replay_path.clone()],
            &mut missing,
        ),
        receipt_with_status(
            "FC3",
            "FC3-N30",
            "constitution",
            fc2.genesis_report_present,
            "document-pinned",
            "constitution_hash",
            vec![genesis],
            &mut missing,
        ),
        receipt(
            "FC3",
            "FC3-N31",
            "logs archive",
            fc3.log_feedback_archive_txs > 0,
            "LogFeedbackArchive",
            vec![format!(
                "log_feedback_archive={}",
                fc3.log_feedback_archive_txs
            )],
            &mut missing,
        ),
        receipt(
            "FC3",
            "FC3-N32",
            "ArchitectAI proposal",
            fc3.architect_proposal_txs > 0,
            "ArchitectProposal",
            vec![format!("architect_proposal={}", fc3.architect_proposal_txs)],
            &mut missing,
        ),
        receipt(
            "FC3",
            "FC3-N33",
            "Veto-AI verdict",
            fc3.veto_decision_txs > 0,
            "VetoDecision_PASS_OR_VETO",
            vec![format!("veto_decision={}", fc3.veto_decision_txs)],
            &mut missing,
        ),
        receipt(
            "FC3",
            "FC3-N34",
            "readonly guard",
            replay_green,
            "trust_root_replay_guard",
            vec![replay_path],
            &mut missing,
        ),
        receipt(
            "FC3",
            "FC3-N35",
            "tools/logs feedback",
            fc3.fc3_index_present,
            "governance_capsule_index",
            vec![fc3_index],
            &mut missing,
        ),
        receipt(
            "FC3",
            "FC3-N36",
            "feedback edge",
            fc3.log_feedback_archive_txs > 0 && fc3.architect_proposal_txs > 0,
            "LogFeedbackArchive_to_ArchitectProposal",
            vec![format!(
                "feedback={} proposal={}",
                fc3.log_feedback_archive_txs, fc3.architect_proposal_txs
            )],
            &mut missing,
        ),
        receipt(
            "FC3",
            "FC3-N37",
            "re-init request",
            fc3.reinit_request_txs > 0,
            "ReinitRequest",
            vec![format!("reinit_request={}", fc3.reinit_request_txs)],
            &mut missing,
        ),
        receipt(
            "FC3",
            "FC3-N38",
            "re-init boot / Q update",
            fc3.reinit_boot_txs > 0,
            "ReinitBoot",
            vec![format!("reinit_boot={}", fc3.reinit_boot_txs)],
            &mut missing,
        ),
    ];

    FlowchartNodeReceipts {
        verdict: if missing.is_empty() {
            "FLOW-PASS"
        } else {
            "FLOW-FAIL"
        }
        .to_string(),
        missing,
        fc1: fc1_receipts,
        fc2: fc2_receipts,
        fc3: fc3_receipts,
    }
}

fn receipt(
    flowchart: &'static str,
    node_id: &'static str,
    label: &'static str,
    present: bool,
    receipt_kind: &'static str,
    evidence: Vec<String>,
    missing: &mut Vec<String>,
) -> FlowchartNodeReceipt {
    receipt_with_status(
        flowchart,
        node_id,
        label,
        present,
        "present",
        receipt_kind,
        evidence,
        missing,
    )
}

#[allow(clippy::too_many_arguments)]
fn receipt_with_status(
    flowchart: &'static str,
    node_id: &'static str,
    label: &'static str,
    present: bool,
    present_status: &'static str,
    receipt_kind: &'static str,
    evidence: Vec<String>,
    missing: &mut Vec<String>,
) -> FlowchartNodeReceipt {
    if !present {
        missing.push(node_id.to_string());
    }
    FlowchartNodeReceipt {
        flowchart,
        node_id,
        label,
        status: if present { present_status } else { "missing" }.to_string(),
        receipt_kind,
        evidence,
    }
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse_args(&argv) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("full_system_participation_current_kernel: {err}");
            eprintln!("{}", usage());
            return ExitCode::from(2);
        }
    };
    let require_full_system = args.require_full_system;
    let out = args.out.clone();
    let (report, full) = match build_report(args) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("full_system_participation_current_kernel: {err}");
            return ExitCode::from(1);
        }
    };
    let json = match serde_json::to_string_pretty(&report) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("full_system_participation_current_kernel: encode report: {err}");
            return ExitCode::from(1);
        }
    };
    if let Some(parent) = out.parent() {
        if let Err(err) = std::fs::create_dir_all(parent) {
            eprintln!(
                "full_system_participation_current_kernel: create {}: {err}",
                parent.display()
            );
            return ExitCode::from(1);
        }
    }
    if let Err(err) = std::fs::write(&out, format!("{json}\n")) {
        eprintln!(
            "full_system_participation_current_kernel: write {}: {err}",
            out.display()
        );
        return ExitCode::from(1);
    }
    if require_full_system && !full {
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}
