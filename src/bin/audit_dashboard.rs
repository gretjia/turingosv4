//! TB-8 — Audit Dashboard CLI.
//!
//! Per TB-7 charter §13.1: "Audit dashboard — UI / CLI to inspect what the
//! Agent saw + submitted + how the system judged, on a per-run basis."
//!
//! Reads a chain-backed runtime_repo + cas directory and prints a
//! structured per-run report. Composes the existing TB-6 / TB-7
//! library surface (verify_chaintape + chain_derived_run_facts +
//! run_summary + agent_keypairs + agent_audit_trail) — does NOT
//! duplicate replay logic.
//!
//! Usage:
//! ```text
//!   audit_dashboard --repo <runtime_repo> --cas <cas> [--json] [--out <path>]
//! ```
//!
//! Output sections (text mode):
//! 1. Run metadata (run_id, epoch, head commit, state/ledger roots)
//! 2. Chain stats (L4 / L4.E counts; verify_chaintape 7 indicators)
//! 3. ChainDerivedRunFacts §4.4 structural fact set
//! 4. Per-agent activity (counts of submitted Work / Verify per agent_id)
//! 5. Proposal flow (chronological list of accepted + rejected tx)
//! 6. Branch lineage (from ProposalTelemetry branch_id + parent_tx)
//! 7. Verification status summary (Gate 1 / 4 / 5 closure indicators)
//!
//! TRACE_MATRIX FC1-N14: TB-7 §13.1 forward — diagnostic CLI over the
//! authoritative chain artifacts.

use std::collections::BTreeMap;
use std::path::PathBuf;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::transition_ledger::{canonical_decode, Git2LedgerWriter, LedgerEntry, LedgerWriter, TxKind};
use turingosv4::runtime::agent_audit_trail::AgentAuditTrailIndex;
use turingosv4::runtime::agent_keypairs::AgentPubkeyManifest;
use turingosv4::runtime::chain_derived_run_facts::{
    compute_run_facts_from_chain, ChainDerivedRunFacts,
};
use turingosv4::runtime::proposal_telemetry::read_from_cas as read_proposal_telemetry;
use turingosv4::runtime::verify::{verify_chaintape, ReplayReport, VerifyOptions};
use turingosv4::state::typed_tx::TypedTx;

#[derive(Debug)]
struct Args {
    repo: PathBuf,
    cas: PathBuf,
    json: bool,
    out: Option<PathBuf>,
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut repo: Option<PathBuf> = None;
    let mut cas: Option<PathBuf> = None;
    let mut json = false;
    let mut out: Option<PathBuf> = None;
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "--repo" => {
                i += 1;
                repo = Some(argv.get(i).ok_or("missing value after --repo")?.into());
            }
            "--cas" => {
                i += 1;
                cas = Some(argv.get(i).ok_or("missing value after --cas")?.into());
            }
            "--json" => json = true,
            "--out" => {
                i += 1;
                out = Some(argv.get(i).ok_or("missing value after --out")?.into());
            }
            "--help" | "-h" => return Err("--help requested".into()),
            other => return Err(format!("unknown arg: {other}")),
        }
        i += 1;
    }
    Ok(Args {
        repo: repo.ok_or("--repo required")?,
        cas: cas.ok_or("--cas required")?,
        json,
        out,
    })
}

#[derive(Debug, serde::Serialize)]
struct DashboardReport {
    run_id: String,
    epoch: u64,
    chain: ChainStats,
    indicators: IndicatorStatus,
    run_facts: ChainDerivedRunFacts,
    per_agent: BTreeMap<String, AgentActivity>,
    proposal_flow: Vec<ProposalFlowEntry>,
    branch_lineage: Vec<BranchEdge>,
    cross_checks: CrossCheck,
}

#[derive(Debug, serde::Serialize)]
struct ChainStats {
    l4_entries: u64,
    l4e_entries: u64,
    head_commit_oid_hex: Option<String>,
    final_state_root_hex: Option<String>,
    final_ledger_root_hex: Option<String>,
    initial_q_state_loaded_from_disk: bool,
}

#[derive(Debug, serde::Serialize)]
struct IndicatorStatus {
    ledger_root_verified: bool,
    system_signatures_verified: bool,
    state_reconstructed: bool,
    economic_state_reconstructed: bool,
    cas_payloads_retrievable: bool,
    agent_signatures_verified: bool,
    proposal_telemetry_cas_retrievable: bool,
    all_pass: bool,
}

#[derive(Debug, Default, serde::Serialize)]
struct AgentActivity {
    work_tx_accepted: u64,
    work_tx_rejected: u64,
    verify_tx_accepted: u64,
    verify_tx_rejected: u64,
    has_pubkey: bool,
}

#[derive(Debug, serde::Serialize)]
struct ProposalFlowEntry {
    logical_t: u64,
    side: &'static str, // "L4" or "L4.E"
    tx_kind: String,
    agent_id: Option<String>,
    tx_id: Option<String>,
    candidate_tactic: Option<String>,
    branch_id: Option<String>,
    rejection_class: Option<String>,
}

#[derive(Debug, serde::Serialize)]
struct BranchEdge {
    parent_tx: String,
    child_tx: String,
    branch_id: String,
}

#[derive(Debug, serde::Serialize)]
struct CrossCheck {
    audit_trail_rows: u64,
    chain_proposal_count: u64,
    proposal_count_matches_audit_rows: bool,
    agent_audit_trail_chain_valid: bool,
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let parsed = match parse_args(&argv[1..]) {
        Ok(a) => a,
        Err(msg) => {
            eprintln!("audit_dashboard: {msg}");
            eprintln!(
                "usage: audit_dashboard --repo <runtime_repo> --cas <cas> [--json] [--out <path>]"
            );
            std::process::exit(2);
        }
    };

    let report = match build_report(&parsed.repo, &parsed.cas) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("audit_dashboard: build failed: {e}");
            std::process::exit(2);
        }
    };

    let rendered = if parsed.json {
        match serde_json::to_string_pretty(&report) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("audit_dashboard: serialize failed: {e}");
                std::process::exit(2);
            }
        }
    } else {
        render_text(&report)
    };

    if let Some(out) = parsed.out.as_ref() {
        if let Err(e) = std::fs::write(out, &rendered) {
            eprintln!("audit_dashboard: write {out:?} failed: {e}");
            std::process::exit(2);
        }
    } else {
        println!("{rendered}");
    }
}

fn build_report(repo: &std::path::Path, cas_path: &std::path::Path) -> Result<DashboardReport, String> {
    // Replay verifier — gives us the 7 indicators + chain root state.
    let replay: ReplayReport = verify_chaintape(repo, cas_path, &VerifyOptions::default())
        .map_err(|e| format!("verify_chaintape: {e:?}"))?;

    let run_facts = compute_run_facts_from_chain(repo, cas_path)
        .map_err(|e| format!("chain_derived_run_facts: {e:?}"))?;

    // Walk L4 entries to populate per_agent + proposal_flow + branch_lineage.
    let writer = Git2LedgerWriter::open(repo)
        .map_err(|e| format!("open ledger: {e:?}"))?;
    let l4_count = writer.len();
    let entries: Vec<LedgerEntry> = (1..=l4_count)
        .map(|t| writer.read_at(t))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("read l4: {e:?}"))?;
    let cas = CasStore::open(cas_path).map_err(|e| format!("cas open: {e}"))?;

    // Manifest of agent pubkeys.
    let manifest_path = repo.join("agent_pubkeys.json");
    let manifest = if manifest_path.exists() {
        Some(AgentPubkeyManifest::load(&manifest_path).map_err(|e| format!("manifest: {e}"))?)
    } else {
        None
    };

    let mut per_agent: BTreeMap<String, AgentActivity> = BTreeMap::new();
    if let Some(m) = manifest.as_ref() {
        for agent_id in m.agents.keys() {
            per_agent.entry(agent_id.clone()).or_default().has_pubkey = true;
        }
    }

    let mut proposal_flow: Vec<ProposalFlowEntry> = Vec::new();
    let mut branch_lineage: Vec<BranchEdge> = Vec::new();

    for entry in &entries {
        let payload_bytes = match cas.get(&entry.tx_payload_cid) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let typed_tx: TypedTx = match canonical_decode(&payload_bytes) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let logical_t = entry.logical_t;
        match &typed_tx {
            TypedTx::Work(work) => {
                let acct = per_agent.entry(work.agent_id.0.clone()).or_default();
                acct.work_tx_accepted += 1;
                let mut tactic: Option<String> = None;
                let mut branch_id: Option<String> = None;
                let mut parent_tx: Option<String> = None;
                if work.proposal_cid.0 != [0u8; 32] {
                    if let Ok(tel) = read_proposal_telemetry(&cas, &work.proposal_cid) {
                        tactic = Some(tel.candidate_tactic.clone());
                        branch_id = Some(tel.branch_id.clone());
                        parent_tx = tel.parent_tx.as_ref().map(|t| t.0.clone());
                    }
                }
                if let (Some(parent), Some(branch)) = (parent_tx.as_ref(), branch_id.as_ref()) {
                    branch_lineage.push(BranchEdge {
                        parent_tx: parent.clone(),
                        child_tx: work.tx_id.0.clone(),
                        branch_id: branch.clone(),
                    });
                }
                proposal_flow.push(ProposalFlowEntry {
                    logical_t,
                    side: "L4",
                    tx_kind: "Work".into(),
                    agent_id: Some(work.agent_id.0.clone()),
                    tx_id: Some(work.tx_id.0.clone()),
                    candidate_tactic: tactic,
                    branch_id,
                    rejection_class: None,
                });
            }
            TypedTx::Verify(verify) => {
                let acct = per_agent.entry(verify.verifier_agent.0.clone()).or_default();
                acct.verify_tx_accepted += 1;
                proposal_flow.push(ProposalFlowEntry {
                    logical_t,
                    side: "L4",
                    tx_kind: "Verify".into(),
                    agent_id: Some(verify.verifier_agent.0.clone()),
                    tx_id: Some(verify.tx_id.0.clone()),
                    candidate_tactic: None,
                    branch_id: None,
                    rejection_class: None,
                });
            }
            TypedTx::TaskOpen(task) => {
                proposal_flow.push(ProposalFlowEntry {
                    logical_t,
                    side: "L4",
                    tx_kind: "TaskOpen".into(),
                    agent_id: Some(task.sponsor_agent.0.clone()),
                    tx_id: Some(task.tx_id.0.clone()),
                    candidate_tactic: None,
                    branch_id: None,
                    rejection_class: None,
                });
            }
            TypedTx::EscrowLock(lock) => {
                proposal_flow.push(ProposalFlowEntry {
                    logical_t,
                    side: "L4",
                    tx_kind: "EscrowLock".into(),
                    agent_id: Some(lock.sponsor_agent.0.clone()),
                    tx_id: Some(lock.tx_id.0.clone()),
                    candidate_tactic: None,
                    branch_id: None,
                    rejection_class: None,
                });
            }
            _ => {
                proposal_flow.push(ProposalFlowEntry {
                    logical_t,
                    side: "L4",
                    tx_kind: format!("{:?}", typed_tx.tx_kind()),
                    agent_id: None,
                    tx_id: None,
                    candidate_tactic: None,
                    branch_id: None,
                    rejection_class: None,
                });
            }
        }
    }

    // L4.E walk via RunSummary's existing path: load rejections.jsonl
    // through RejectionEvidenceWriter (gives us the records).
    use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
    let rejections_path = repo.join("rejections.jsonl");
    let l4e_writer = if rejections_path.exists() {
        RejectionEvidenceWriter::open_jsonl(rejections_path)
            .map_err(|e| format!("l4.e open: {e:?}"))?
    } else {
        RejectionEvidenceWriter::new()
    };
    for record in l4e_writer.records() {
        let acct = per_agent.entry(record.agent_id.0.clone()).or_default();
        match record.tx_kind {
            TxKind::Work => acct.work_tx_rejected += 1,
            TxKind::Verify => acct.verify_tx_rejected += 1,
            _ => {}
        }
        // For rejected tx, try to resolve telemetry for tactic / branch context.
        let mut tactic: Option<String> = None;
        let mut branch_id: Option<String> = None;
        if let Ok(payload_bytes) = cas.get(&record.tx_payload_cid) {
            if let Ok(typed_tx) = canonical_decode::<TypedTx>(&payload_bytes) {
                if let TypedTx::Work(w) = typed_tx {
                    if w.proposal_cid.0 != [0u8; 32] {
                        if let Ok(tel) = read_proposal_telemetry(&cas, &w.proposal_cid) {
                            tactic = Some(tel.candidate_tactic.clone());
                            branch_id = Some(tel.branch_id.clone());
                        }
                    }
                }
            }
        }
        proposal_flow.push(ProposalFlowEntry {
            logical_t: 0, // L4.E records are keyed by submit_id, not logical_t
            side: "L4.E",
            tx_kind: format!("{:?}", record.tx_kind),
            agent_id: Some(record.agent_id.0.clone()),
            tx_id: None,
            candidate_tactic: tactic,
            branch_id,
            rejection_class: Some(format!("{:?}", record.rejection_class)),
        });
    }

    // Sort proposal_flow by logical_t (then by side: L4 first, L4.E after).
    proposal_flow.sort_by_key(|p| (p.logical_t, if p.side == "L4" { 0 } else { 1 }));

    // Audit-trail cross-check
    let audit_trail_index = AgentAuditTrailIndex::open(repo).ok();
    let audit_trail_rows = audit_trail_index.as_ref().map(|i| i.len() as u64).unwrap_or(0);
    let chain_proposal_count = run_facts.proposal_count;
    // Best-effort: if any audit rows exist, the chain integrity is already
    // checked at AgentAuditTrailIndex::open time (returns ChainBroken on
    // tamper). Reaching this point with Some(_) means valid.
    let agent_audit_trail_chain_valid = audit_trail_index.is_some();

    // Note: we don't enforce strict equality here because audit_trail rows
    // are written only by Atom 5 synthetic-seed hook + future per-LLM-proposal
    // hook (not yet wired in real run). This dashboard reports the gap honestly.
    let proposal_count_matches_audit_rows = audit_trail_rows == chain_proposal_count;

    let cross_checks = CrossCheck {
        audit_trail_rows,
        chain_proposal_count,
        proposal_count_matches_audit_rows,
        agent_audit_trail_chain_valid,
    };

    let all_pass = replay.all_indicators_pass();
    Ok(DashboardReport {
        run_id: replay.run_id.clone(),
        epoch: replay.epoch,
        chain: ChainStats {
            l4_entries: replay.l4_entries,
            l4e_entries: replay.l4e_entries,
            head_commit_oid_hex: replay.detail.head_commit_oid_hex.clone(),
            final_state_root_hex: replay.detail.final_state_root_hex.clone(),
            final_ledger_root_hex: replay.detail.final_ledger_root_hex.clone(),
            initial_q_state_loaded_from_disk: replay.detail.initial_q_state_loaded_from_disk,
        },
        indicators: IndicatorStatus {
            all_pass,
            ledger_root_verified: replay.ledger_root_verified,
            system_signatures_verified: replay.system_signatures_verified,
            state_reconstructed: replay.state_reconstructed,
            economic_state_reconstructed: replay.economic_state_reconstructed,
            cas_payloads_retrievable: replay.cas_payloads_retrievable,
            agent_signatures_verified: replay.agent_signatures_verified,
            proposal_telemetry_cas_retrievable: replay.proposal_telemetry_cas_retrievable,
        },
        run_facts,
        per_agent,
        proposal_flow,
        branch_lineage,
        cross_checks,
    })
}

fn render_text(r: &DashboardReport) -> String {
    let mut s = String::new();
    s.push_str("=================================================================\n");
    s.push_str(&format!(" TB-8 Audit Dashboard — run_id={} epoch={}\n", r.run_id, r.epoch));
    s.push_str("=================================================================\n\n");

    // §1 Run metadata
    s.push_str("§1 Run metadata\n");
    s.push_str("---------------\n");
    s.push_str(&format!(
        "  head_commit_oid: {}\n",
        r.chain.head_commit_oid_hex.as_deref().unwrap_or("(empty chain)")
    ));
    s.push_str(&format!(
        "  final_state_root: {}\n",
        r.chain.final_state_root_hex.as_deref().unwrap_or("-")
    ));
    s.push_str(&format!(
        "  final_ledger_root: {}\n",
        r.chain.final_ledger_root_hex.as_deref().unwrap_or("-")
    ));
    s.push_str(&format!(
        "  initial_q_state_loaded_from_disk: {}\n",
        r.chain.initial_q_state_loaded_from_disk
    ));
    s.push('\n');

    // §2 Chain stats + 7 indicators
    s.push_str("§2 Chain stats + 7 indicators\n");
    s.push_str("------------------------------\n");
    s.push_str(&format!("  L4 entries:  {}\n", r.chain.l4_entries));
    s.push_str(&format!("  L4.E entries: {}\n", r.chain.l4e_entries));
    s.push_str(&format!(
        "  ledger_root_verified              : {}\n",
        if r.indicators.ledger_root_verified { "✓" } else { "✗" }
    ));
    s.push_str(&format!(
        "  system_signatures_verified        : {}\n",
        if r.indicators.system_signatures_verified { "✓" } else { "✗" }
    ));
    s.push_str(&format!(
        "  state_reconstructed               : {}\n",
        if r.indicators.state_reconstructed { "✓" } else { "✗" }
    ));
    s.push_str(&format!(
        "  economic_state_reconstructed      : {}\n",
        if r.indicators.economic_state_reconstructed { "✓" } else { "✗" }
    ));
    s.push_str(&format!(
        "  cas_payloads_retrievable          : {}\n",
        if r.indicators.cas_payloads_retrievable { "✓" } else { "✗" }
    ));
    s.push_str(&format!(
        "  agent_signatures_verified [Gate 4]: {}\n",
        if r.indicators.agent_signatures_verified { "✓" } else { "✗" }
    ));
    s.push_str(&format!(
        "  proposal_telemetry_cas_retrievable [Gate 5]: {}\n",
        if r.indicators.proposal_telemetry_cas_retrievable { "✓" } else { "✗" }
    ));
    s.push_str(&format!(
        "  ALL 7 PASS                        : {}\n\n",
        if r.indicators.all_pass { "GREEN" } else { "RED" }
    ));

    // §3 ChainDerivedRunFacts
    s.push_str("§3 ChainDerivedRunFacts (§4.4 bit-exact set)\n");
    s.push_str("---------------------------------------------\n");
    s.push_str(&format!("  solved                  : {}\n", r.run_facts.solved));
    s.push_str(&format!("  verified                : {}\n", r.run_facts.verified));
    s.push_str(&format!("  tx_count                : {}\n", r.run_facts.tx_count));
    s.push_str(&format!("  proposal_count          : {}\n", r.run_facts.proposal_count));
    s.push_str(&format!("  golden_path_token_count : {}\n", r.run_facts.golden_path_token_count));
    s.push_str(&format!(
        "  gp_payload (CID hex)    : {}\n",
        r.run_facts.gp_payload.as_deref().unwrap_or("-")
    ));
    s.push_str(&format!(
        "  gp_path                 : {}\n",
        r.run_facts.gp_path.as_deref().unwrap_or("-")
    ));
    s.push_str(&format!("  tactic_diversity        : {}\n", r.run_facts.tactic_diversity));
    s.push_str(&format!("  failed_branch_count     : {}\n", r.run_facts.failed_branch_count));
    s.push_str("  tool_dist:\n");
    if r.run_facts.tool_dist.is_empty() {
        s.push_str("    (empty)\n");
    } else {
        for (tactic, count) in &r.run_facts.tool_dist {
            s.push_str(&format!("    {tactic}: {count}\n"));
        }
    }
    s.push('\n');

    // §4 Per-agent activity
    s.push_str("§4 Per-agent activity\n");
    s.push_str("---------------------\n");
    if r.per_agent.is_empty() {
        s.push_str("  (no agent activity recorded)\n");
    } else {
        s.push_str("  agent_id          | pubkey | Work✓ | Work✗ | Verify✓ | Verify✗\n");
        s.push_str("  ------------------+--------+-------+-------+---------+--------\n");
        for (agent_id, act) in &r.per_agent {
            s.push_str(&format!(
                "  {:<17} | {:<6} | {:<5} | {:<5} | {:<7} | {}\n",
                agent_id,
                if act.has_pubkey { "✓" } else { "✗" },
                act.work_tx_accepted,
                act.work_tx_rejected,
                act.verify_tx_accepted,
                act.verify_tx_rejected,
            ));
        }
    }
    s.push('\n');

    // §5 Proposal flow
    s.push_str("§5 Proposal flow (chronological by logical_t)\n");
    s.push_str("----------------------------------------------\n");
    if r.proposal_flow.is_empty() {
        s.push_str("  (no proposals)\n");
    } else {
        s.push_str("  side  | t   | tx_kind         | agent      | tactic     | branch     | reject\n");
        s.push_str("  ------+-----+-----------------+------------+------------+------------+-------\n");
        for entry in &r.proposal_flow {
            s.push_str(&format!(
                "  {:<5} | {:>3} | {:<15} | {:<10} | {:<10} | {:<10} | {}\n",
                entry.side,
                entry.logical_t,
                entry.tx_kind,
                entry.agent_id.as_deref().unwrap_or("-"),
                entry.candidate_tactic.as_deref().unwrap_or("-"),
                entry.branch_id.as_deref().unwrap_or("-"),
                entry.rejection_class.as_deref().unwrap_or("-"),
            ));
        }
    }
    s.push('\n');

    // §6 Branch lineage
    s.push_str("§6 Branch lineage (parent_tx → child_tx via ProposalTelemetry.parent_tx)\n");
    s.push_str("------------------------------------------------------------------------\n");
    if r.branch_lineage.is_empty() {
        s.push_str("  (no branch edges — proposals are root-only or telemetry parent_tx is None)\n");
    } else {
        for edge in &r.branch_lineage {
            s.push_str(&format!(
                "  [{}] {} → {}\n",
                edge.branch_id, edge.parent_tx, edge.child_tx
            ));
        }
    }
    s.push('\n');

    // §7 Cross-checks
    s.push_str("§7 Cross-checks\n");
    s.push_str("---------------\n");
    s.push_str(&format!("  audit_trail_rows         : {}\n", r.cross_checks.audit_trail_rows));
    s.push_str(&format!("  chain_proposal_count     : {}\n", r.cross_checks.chain_proposal_count));
    s.push_str(&format!(
        "  audit_rows == proposal_count: {}\n",
        if r.cross_checks.proposal_count_matches_audit_rows { "✓" } else { "✗ (gap)" }
    ));
    s.push_str(&format!(
        "  audit_trail_chain_valid     : {}\n",
        if r.cross_checks.agent_audit_trail_chain_valid { "✓" } else { "✗" }
    ));
    s.push_str("  (Note: pre-TB-7.6 the agent_audit_trail.jsonl is populated only\n");
    s.push_str("   by the synthetic-seed hook; full per-LLM-proposal audit-trail\n");
    s.push_str("   wiring is part of TB-7.6 carry-forward action #4 / #5.)\n");

    s
}
