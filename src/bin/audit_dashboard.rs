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
    /// TB-7.7 D6: golden path steps (only populated when chain_oracle_verified=true).
    golden_path: Vec<GoldenPathStep>,
    cross_checks: CrossCheck,
    /// TB-8 Atom 6: per-claim audit-row (Open / Finalized) with payout amount.
    /// Populated by walking L4 entries and matching VerifyTx{Confirm} → claim
    /// derivation against any subsequent FinalizeRewardTx with the same claim_id.
    claims: Vec<ClaimAuditRow>,
    /// TB-10 Atom 4: per-user-task audit-row. Populated by filtering TaskOpen
    /// entries whose sponsor_agent.0 starts with "Agent_user_" (lean_market
    /// CLI convention) and cross-referencing with claims for payout status.
    /// The aggregate sum of bounty_micro across all rows is the user's total
    /// committed liquidity at this snapshot.
    user_tasks: Vec<UserTaskRow>,
    /// TB-11 Atom 5 (architect §6.2): exhausted runs from TerminalSummaryTx
    /// L4 entries (architect's RunExhaustedTx role).
    exhausted_runs: Vec<ExhaustedRunRow>,
    /// TB-11 Atom 5 (architect §6.2): expired tasks from TaskExpireTx L4
    /// entries (capital release path).
    expired_tasks: Vec<ExpiredTaskRow>,
    /// TB-11 Atom 5 (architect §6.2): bankrupt tasks from TaskBankruptcyTx
    /// L4 entries (death certificate for future TB-12 NodeMarket Short / NO
    /// settlement anchor).
    bankrupt_tasks: Vec<BankruptTaskRow>,
    /// TB-12 Atom 4 (architect 2026-05-03 ruling §8 Atom 4): exposure
    /// records derived from accepted WorkTx (FirstLong) + ChallengeTx
    /// (ChallengeShort) L4 entries. Architect §10: IMMUTABLE EXPOSURE
    /// RECORD, NOT active position balance. Label discipline: "Exposure
    /// records", NOT "Open market balances".
    exposures: Vec<ExposureRecordRow>,
}

/// TB-12 Atom 4 (architect 2026-05-03 ruling §8 Atom 4) — per-NodePosition
/// audit row for §13. Architect's label discipline: "Exposure records"
/// (NOT "Open market balances" — TB-12 is exposure index, not trading
/// market; live share balances land in TB-13 CompleteSet).
#[derive(Debug, serde::Serialize)]
struct ExposureRecordRow {
    position_id: String,
    node_id: String,
    task_id: String,
    owner: String,
    /// "Long" or "Short".
    side: String,
    /// "FirstLong" or "ChallengeShort".
    kind: String,
    /// MicroCoin amount of the position. **NOT a Coin holding** per CR-12.1
    /// + CR-12.2; explicitly excluded from total_supply_micro.
    amount_micro: i64,
    /// Backref to the source typed-tx that derived this position
    /// (FirstLong: WorkTx.tx_id; ChallengeShort: ChallengeTx.tx_id).
    source_tx: String,
    opened_at_round: u64,
}

/// TB-11 Atom 5 (architect §6.2 ruling 2026-05-02) — per-RunExhausted
/// audit row for §12. Surfaces architect's RunExhaustedTx (≡
/// TerminalSummaryTx in the failure path) on chain.
#[derive(Debug, serde::Serialize)]
struct ExhaustedRunRow {
    run_id: String,
    task_id: String,
    run_outcome: String,
    attempt_count: u32,
    /// Hex of evidence_capsule_cid; "—" if None (OmegaAccepted path).
    evidence_capsule_cid_hex: String,
    solver: String,
    last_logical_t: u64,
}

/// TB-11 Atom 5 — per-Expired-task audit row for §12 (capital release).
#[derive(Debug, serde::Serialize)]
struct ExpiredTaskRow {
    task_id: String,
    sponsor: String,
    refund_micro: i64,
    reason: String,
    expired_at_logical_t: u64,
}

/// TB-11 Atom 5 — per-Bankrupt-task audit row for §12 (death certificate).
#[derive(Debug, serde::Serialize)]
struct BankruptTaskRow {
    task_id: String,
    evidence_capsule_cid_hex: String,
    bankruptcy_reason: String,
    failed_run_count: u32,
    bankrupted_at_logical_t: u64,
}

/// TB-10 Atom 4 — per-user-task audit row for the dashboard's §11 section.
///
/// Filter convention: TaskOpenTx whose sponsor_agent starts with `Agent_user_`
/// (lean_market CLI's runtime preseed factory binds Agent_user_0 as the
/// canonical sponsor identity). Solver and payout fields are populated from
/// the matching ClaimAuditRow whose task_id equals this row's task_id.
#[derive(Debug, serde::Serialize)]
struct UserTaskRow {
    task_id: String,
    sponsor: String,
    bounty_micro: i64,
    /// Solver's durable AgentId (from TB-9 keystore); "(no solver yet)" if
    /// no Confirm-VerifyTx has been observed for this task.
    solver: String,
    /// "Open" or "Finalized" or "(no claim yet)".
    claim_status: String,
    /// Payout amount in MicroCoin if Finalized; None otherwise.
    payout_micro: Option<i64>,
    /// L4 logical_t of the TaskOpen.
    opened_at_logical_t: u64,
}

/// TB-8 Atom 6 — per-claim audit row for the dashboard's claims section.
///
/// Reflects the chain-derived claim lifecycle: a Confirm VerifyTx implies a
/// claim creation (claim_id = "claim-<verify.tx_id>"); a subsequent
/// FinalizeRewardTx with that claim_id flips status to Finalized and
/// records the payout amount. Both columns satisfy the user-minimum
/// requirement "dashboard shows payout" plus the broader status discriminator.
#[derive(Debug, serde::Serialize)]
struct ClaimAuditRow {
    claim_id: String,
    task_id: String,
    solver: String,
    /// "Open" or "Finalized" or "n/a" (no claim discoverable).
    claim_status: String,
    /// Payout amount in MicroCoin if Finalized; "—" otherwise.
    payout_amount_micro: Option<i64>,
    /// L4 logical_t of the Verify-Confirm that created this claim.
    created_at_logical_t: u64,
    /// L4 logical_t of the FinalizeReward that closed this claim, if any.
    finalized_at_logical_t: Option<u64>,
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
    /// TB-7.7 D6: payload preview from CAS (first 80 bytes of proposal_artifact_cid content).
    proposal_artifact_preview: Option<String>,
    /// TB-7.7 D6: oracle_verified flag from VerificationResult (None = no VR; Some(true) = Lean accepted).
    oracle_verified: Option<bool>,
}

#[derive(Debug, serde::Serialize)]
struct BranchEdge {
    parent_tx: String,
    child_tx: String,
    branch_id: String,
}

/// TB-7.7 D6: golden path step on a solved problem. Each entry walks from
/// root → ... → the oracle-verified WorkTx, reading payload bytes from CAS.
#[derive(Debug, serde::Serialize)]
struct GoldenPathStep {
    depth: usize,
    tx_id: String,
    agent_id: String,
    candidate_tactic: String,
    payload_preview: String,
    oracle_verified: bool,
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
    // TB-8 Atom 6: claim audit rows. Built in two passes within the entry
    // walk: Confirm VerifyTx → Open row; FinalizeRewardTx → Finalized.
    let mut claims_in_progress: Vec<ClaimAuditRow> = Vec::new();
    // TB-10 Atom 4: user-task audit rows. Built by filtering TaskOpen entries
    // whose sponsor_agent starts with "Agent_user_" + matching EscrowLockTx
    // for bounty amount + cross-referencing claims_in_progress for status.
    let mut user_tasks_in_progress: Vec<UserTaskRow> = Vec::new();
    // TB-11 Atom 5 (architect §6.2): exhausted/expired/bankrupt collectors.
    let mut exhausted_runs_in_progress: Vec<ExhaustedRunRow> = Vec::new();
    let mut expired_tasks_in_progress: Vec<ExpiredTaskRow> = Vec::new();
    let mut bankrupt_tasks_in_progress: Vec<BankruptTaskRow> = Vec::new();
    // TB-12 Atom 4 (architect 2026-05-03 §8 Atom 4): exposure records
    // collected by walking L4 — accepted WorkTx with stake>0 → FirstLong;
    // accepted ChallengeTx with stake>0 → ChallengeShort.
    let mut exposures_in_progress: Vec<ExposureRecordRow> = Vec::new();
    // TB-7.7 D6: oracle_verified_worktx_ids — set of accepted L4 WorkTx
    // tx_ids whose ProposalTelemetry.verification_result_cid resolves to
    // VerificationResult { verified: true }. Plus their telemetry for
    // golden-path reconstruction.
    let mut oracle_verified_worktx: BTreeMap<
        String,
        (String, String, String), // (agent_id, candidate_tactic, payload_preview)
    > = BTreeMap::new();
    let mut work_parent_by_tx_id: BTreeMap<String, Option<String>> = BTreeMap::new();
    use turingosv4::runtime::verification_result::read_from_cas as read_verification_result;

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
                let mut payload_preview: Option<String> = None;
                let mut oracle_verified: Option<bool> = None;
                if work.proposal_cid.0 != [0u8; 32] {
                    if let Ok(tel) = read_proposal_telemetry(&cas, &work.proposal_cid) {
                        tactic = Some(tel.candidate_tactic.clone());
                        branch_id = Some(tel.branch_id.clone());
                        parent_tx = tel.parent_tx.as_ref().map(|t| t.0.clone());
                        // TB-7.7 D6: payload preview from CAS via proposal_artifact_cid.
                        if let Ok(payload) = cas.get(&tel.proposal_artifact_cid) {
                            let preview = String::from_utf8_lossy(&payload)
                                .chars()
                                .take(80)
                                .collect::<String>();
                            payload_preview = Some(preview);
                        }
                        // TB-7.7 D6: oracle_verified from VerificationResult.
                        if let Some(vr_cid) = tel.verification_result_cid.as_ref() {
                            if let Ok(vr) = read_verification_result(&cas, vr_cid) {
                                oracle_verified = Some(vr.verified);
                                if vr.verified {
                                    oracle_verified_worktx.insert(
                                        work.tx_id.0.clone(),
                                        (
                                            work.agent_id.0.clone(),
                                            tel.candidate_tactic.clone(),
                                            payload_preview.clone().unwrap_or_default(),
                                        ),
                                    );
                                }
                            }
                        }
                    }
                }
                work_parent_by_tx_id.insert(work.tx_id.0.clone(), parent_tx.clone());
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
                    proposal_artifact_preview: payload_preview,
                    oracle_verified,
                });
                // TB-12 Atom 4 (architect 2026-05-03 §8 Atom 4): if accepted
                // WorkTx has stake>0, derive a FirstLong exposure record
                // (mirror of dispatch arm in src/state/sequencer.rs).
                if work.stake.micro_units() > 0 {
                    exposures_in_progress.push(ExposureRecordRow {
                        position_id: work.tx_id.0.clone(),
                        node_id: work.tx_id.0.clone(),
                        task_id: work.task_id.0.clone(),
                        owner: work.agent_id.0.clone(),
                        side: "Long".into(),
                        kind: "FirstLong".into(),
                        amount_micro: work.stake.micro_units(),
                        source_tx: work.tx_id.0.clone(),
                        opened_at_round: work.timestamp_logical,
                    });
                }
            }
            // TB-12 Atom 4 (architect 2026-05-03 §8 Atom 4): accepted
            // ChallengeTx with stake>0 → ChallengeShort exposure record.
            TypedTx::Challenge(challenge) => {
                if challenge.stake.micro_units() > 0 {
                    exposures_in_progress.push(ExposureRecordRow {
                        position_id: challenge.tx_id.0.clone(),
                        // node_id targets the challenged WorkTx (FR-12.5).
                        node_id: challenge.target_work_tx.0.clone(),
                        // task_id is best-effort: dashboard walks L4
                        // sequentially and does not have stakes_t available;
                        // the ChainTape replay validates the final state.
                        // For dashboard rendering, leave empty if unresolved
                        // — TB-12 charter §3 Atom 4 forbids "Open market
                        // balances" framing anyway, so this is a render-only
                        // approximation; SOURCE OF TRUTH is the QState
                        // node_positions_t after replay.
                        task_id: String::new(),
                        owner: challenge.challenger_agent.0.clone(),
                        side: "Short".into(),
                        kind: "ChallengeShort".into(),
                        amount_micro: challenge.stake.micro_units(),
                        source_tx: challenge.tx_id.0.clone(),
                        opened_at_round: challenge.timestamp_logical,
                    });
                }
                proposal_flow.push(ProposalFlowEntry {
                    logical_t,
                    side: "L4",
                    tx_kind: "Challenge".into(),
                    agent_id: Some(challenge.challenger_agent.0.clone()),
                    tx_id: Some(challenge.tx_id.0.clone()),
                    candidate_tactic: None,
                    branch_id: None,
                    rejection_class: None,
                    proposal_artifact_preview: None,
                    oracle_verified: None,
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
                    proposal_artifact_preview: None,
                    oracle_verified: None,
                });
                // TB-8 Atom 6: a Confirm verdict creates a ClaimEntry; record
                // the audit row here so the dashboard's claims section can
                // flip Open → Finalized later when a matching FinalizeReward
                // entry is observed.
                if verify.verdict == turingosv4::state::typed_tx::VerifyVerdict::Confirm {
                    // Best-effort solver lookup: walk back through entries
                    // to find the WorkTx whose tx_id matches verify.target_work_tx
                    // and read its agent_id. (Cheap O(n) linear scan; n is
                    // small for TB-8 MVP runs.)
                    let solver = entries
                        .iter()
                        .filter_map(|prev| {
                            let bytes = cas.get(&prev.tx_payload_cid).ok()?;
                            let tx: TypedTx = canonical_decode(&bytes).ok()?;
                            if let TypedTx::Work(w) = tx {
                                if w.tx_id == verify.target_work_tx {
                                    return Some((w.agent_id.0.clone(), w.task_id.0.clone()));
                                }
                            }
                            None
                        })
                        .next();
                    let (solver_id, task_id) = solver.unwrap_or_else(|| ("(unknown)".into(), "(unknown)".into()));
                    claims_in_progress.push(ClaimAuditRow {
                        claim_id: format!("claim-{}", verify.tx_id.0),
                        task_id,
                        solver: solver_id,
                        claim_status: "Open".into(),
                        payout_amount_micro: None,
                        created_at_logical_t: logical_t,
                        finalized_at_logical_t: None,
                    });
                }
            }
            // TB-8 Atom 6: FinalizeRewardTx — flip the matching claim row to
            // Finalized and record the payout amount.
            TypedTx::FinalizeReward(fr) => {
                if let Some(row) = claims_in_progress
                    .iter_mut()
                    .find(|r| r.claim_id == fr.claim_id.as_tx_id().0)
                {
                    row.claim_status = "Finalized".into();
                    row.payout_amount_micro = Some(fr.reward.micro_units());
                    row.finalized_at_logical_t = Some(logical_t);
                    // Q-derived authoritative fields (already set at row
                    // creation, but FinalizeReward wire fields are the
                    // ledger-summary attestation; cross-check by overwriting
                    // — they MUST agree by Atom 3 step 5 anti-forgery gate).
                    row.solver = fr.solver.0.clone();
                    row.task_id = fr.task_id.0.clone();
                }
                proposal_flow.push(ProposalFlowEntry {
                    logical_t,
                    side: "L4",
                    tx_kind: "FinalizeReward".into(),
                    agent_id: Some(format!("system (solver={})", fr.solver.0)),
                    tx_id: Some(fr.tx_id.0.clone()),
                    candidate_tactic: None,
                    branch_id: None,
                    rejection_class: None,
                    proposal_artifact_preview: None,
                    oracle_verified: None,
                });
            }
            TypedTx::TaskOpen(task) => {
                // TB-10 Atom 4: register a user-task row when the TaskOpen
                // sponsor matches the Agent_user_* convention. Bounty +
                // solver + status fields are filled in by subsequent
                // EscrowLock + Verify + FinalizeReward entries.
                if task.sponsor_agent.0.starts_with("Agent_user_") {
                    user_tasks_in_progress.push(UserTaskRow {
                        task_id: task.task_id.0.clone(),
                        sponsor: task.sponsor_agent.0.clone(),
                        bounty_micro: 0,
                        solver: "(no solver yet)".into(),
                        claim_status: "(no claim yet)".into(),
                        payout_micro: None,
                        opened_at_logical_t: logical_t,
                    });
                }
                proposal_flow.push(ProposalFlowEntry {
                    logical_t,
                    side: "L4",
                    tx_kind: "TaskOpen".into(),
                    agent_id: Some(task.sponsor_agent.0.clone()),
                    tx_id: Some(task.tx_id.0.clone()),
                    candidate_tactic: None,
                    branch_id: None,
                    rejection_class: None,
                    proposal_artifact_preview: None,
                    oracle_verified: None,
                });
            }
            TypedTx::EscrowLock(lock) => {
                // TB-10 Atom 4: when an EscrowLock matches a user-task row by
                // task_id, accumulate the bounty.
                if lock.sponsor_agent.0.starts_with("Agent_user_") {
                    if let Some(row) = user_tasks_in_progress
                        .iter_mut()
                        .find(|r| r.task_id == lock.task_id.0)
                    {
                        row.bounty_micro += lock.amount.micro_units();
                    }
                }
                proposal_flow.push(ProposalFlowEntry {
                    logical_t,
                    side: "L4",
                    tx_kind: "EscrowLock".into(),
                    agent_id: Some(lock.sponsor_agent.0.clone()),
                    tx_id: Some(lock.tx_id.0.clone()),
                    candidate_tactic: None,
                    branch_id: None,
                    rejection_class: None,
                    proposal_artifact_preview: None,
                    oracle_verified: None,
                });
            }
            // TB-11 Atom 5 (architect §6.2): TerminalSummary → §12 row.
            TypedTx::TerminalSummary(ts) => {
                exhausted_runs_in_progress.push(ExhaustedRunRow {
                    run_id: ts.run_id.0.clone(),
                    task_id: ts.task_id.0.clone(),
                    run_outcome: format!("{:?}", ts.run_outcome),
                    attempt_count: ts.total_attempts,
                    evidence_capsule_cid_hex: ts
                        .evidence_capsule_cid
                        .as_ref()
                        .map(|c| c.hex())
                        .unwrap_or_else(|| "—".into()),
                    solver: ts
                        .solver_agent
                        .as_ref()
                        .map(|a| a.0.clone())
                        .unwrap_or_else(|| "(none)".into()),
                    last_logical_t: ts.last_logical_t,
                });
                proposal_flow.push(ProposalFlowEntry {
                    logical_t,
                    side: "L4",
                    tx_kind: "TerminalSummary".into(),
                    agent_id: ts.solver_agent.as_ref().map(|a| a.0.clone()),
                    tx_id: Some(ts.tx_id.0.clone()),
                    candidate_tactic: None,
                    branch_id: None,
                    rejection_class: None,
                    proposal_artifact_preview: None,
                    oracle_verified: None,
                });
            }
            // TB-11 Atom 5 (architect §6.2): TaskExpire → §12 row.
            TypedTx::TaskExpire(expire) => {
                expired_tasks_in_progress.push(ExpiredTaskRow {
                    task_id: expire.task_id.0.clone(),
                    sponsor: expire.sponsor_agent.0.clone(),
                    refund_micro: expire.bounty_refunded.micro_units(),
                    reason: format!("{:?}", expire.reason),
                    expired_at_logical_t: expire.timestamp_logical,
                });
                proposal_flow.push(ProposalFlowEntry {
                    logical_t,
                    side: "L4",
                    tx_kind: "TaskExpire".into(),
                    agent_id: Some(expire.sponsor_agent.0.clone()),
                    tx_id: Some(expire.tx_id.0.clone()),
                    candidate_tactic: None,
                    branch_id: None,
                    rejection_class: None,
                    proposal_artifact_preview: None,
                    oracle_verified: None,
                });
            }
            // TB-11 Atom 5 (architect §6.2): TaskBankruptcy → §12 row.
            TypedTx::TaskBankruptcy(bk) => {
                bankrupt_tasks_in_progress.push(BankruptTaskRow {
                    task_id: bk.task_id.0.clone(),
                    evidence_capsule_cid_hex: bk.evidence_capsule_cid.hex(),
                    bankruptcy_reason: format!("{:?}", bk.bankruptcy_reason),
                    failed_run_count: bk.failed_run_count,
                    bankrupted_at_logical_t: bk.timestamp_logical,
                });
                proposal_flow.push(ProposalFlowEntry {
                    logical_t,
                    side: "L4",
                    tx_kind: "TaskBankruptcy".into(),
                    agent_id: None,
                    tx_id: Some(bk.tx_id.0.clone()),
                    candidate_tactic: None,
                    branch_id: None,
                    rejection_class: None,
                    proposal_artifact_preview: None,
                    oracle_verified: None,
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
                    proposal_artifact_preview: None,
                    oracle_verified: None,
                });
            }
        }
    }

    // TB-7.7 D6: golden path reconstruction. For each oracle-verified
    // WorkTx, walk parent_tx links upward to root; output the path.
    // Pick the FIRST oracle_verified_worktx as the canonical golden
    // path (deterministic per BTreeMap order).
    let mut golden_path: Vec<GoldenPathStep> = Vec::new();
    if let Some((winner_tx_id, (agent, tactic, payload))) = oracle_verified_worktx.iter().next() {
        let mut chain: Vec<(String, String, String, String, bool)> = Vec::new();
        chain.push((
            winner_tx_id.clone(),
            agent.clone(),
            tactic.clone(),
            payload.clone(),
            true,
        ));
        let mut cursor = work_parent_by_tx_id
            .get(winner_tx_id)
            .cloned()
            .flatten();
        let mut safety = 0;
        while let Some(parent) = cursor {
            safety += 1;
            if safety > 100 {
                break; // cycle safety
            }
            // Look up parent in proposal_flow for metadata.
            let entry = proposal_flow
                .iter()
                .find(|e| e.tx_id.as_deref() == Some(parent.as_str()));
            if let Some(p) = entry {
                chain.push((
                    parent.clone(),
                    p.agent_id.clone().unwrap_or_default(),
                    p.candidate_tactic.clone().unwrap_or_default(),
                    p.proposal_artifact_preview.clone().unwrap_or_default(),
                    p.oracle_verified.unwrap_or(false),
                ));
            } else {
                chain.push((parent.clone(), String::new(), String::new(), String::new(), false));
            }
            cursor = work_parent_by_tx_id.get(&parent).cloned().flatten();
        }
        // Reverse so root → winner.
        chain.reverse();
        for (depth, (tx_id, ag, tac, pl, vr)) in chain.into_iter().enumerate() {
            golden_path.push(GoldenPathStep {
                depth,
                tx_id,
                agent_id: ag,
                candidate_tactic: tac,
                payload_preview: pl,
                oracle_verified: vr,
            });
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
            proposal_artifact_preview: None,
            oracle_verified: None,
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

    // TB-10 Atom 4: cross-reference user-task rows with claim audit rows so
    // §11 can show solver + status + payout for each user-sponsored task.
    for ut in user_tasks_in_progress.iter_mut() {
        if let Some(claim) = claims_in_progress
            .iter()
            .find(|c| c.task_id == ut.task_id)
        {
            ut.solver = claim.solver.clone();
            ut.claim_status = claim.claim_status.clone();
            ut.payout_micro = claim.payout_amount_micro;
        }
    }

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
        golden_path,
        cross_checks,
        claims: claims_in_progress,
        user_tasks: user_tasks_in_progress,
        exhausted_runs: exhausted_runs_in_progress,
        expired_tasks: expired_tasks_in_progress,
        bankrupt_tasks: bankrupt_tasks_in_progress,
        exposures: exposures_in_progress,
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
    s.push_str(&format!(
        "  chain_oracle_verified   : {} {}\n",
        r.run_facts.chain_oracle_verified,
        if r.run_facts.chain_oracle_verified { "✓ (Lean accepted ≥1 proof; oracle-level)" } else { "(no oracle-verified WorkTx)" }
    ));
    s.push_str(&format!(
        "  chain_economic_finalized: {} (always false in TB-7; settlement = TB-9 territory)\n",
        r.run_facts.chain_economic_finalized
    ));
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
        s.push_str("  side  | t   | tx_kind         | agent      | tactic     | branch     | oracle | reject\n");
        s.push_str("  ------+-----+-----------------+------------+------------+------------+--------+-------\n");
        for entry in &r.proposal_flow {
            let oracle_marker = match entry.oracle_verified {
                Some(true) => "✓",
                Some(false) => "✗",
                None => "-",
            };
            s.push_str(&format!(
                "  {:<5} | {:>3} | {:<15} | {:<10} | {:<10} | {:<10} | {:<6} | {}\n",
                entry.side,
                entry.logical_t,
                entry.tx_kind,
                entry.agent_id.as_deref().unwrap_or("-"),
                entry.candidate_tactic.as_deref().unwrap_or("-"),
                entry.branch_id.as_deref().unwrap_or("-"),
                oracle_marker,
                entry.rejection_class.as_deref().unwrap_or("-"),
            ));
            // TB-7.7 D6: payload preview from CAS (per-Work entries that have it).
            if let Some(prev) = entry.proposal_artifact_preview.as_deref() {
                if !prev.is_empty() {
                    let one_line = prev.replace('\n', " ⏎ ");
                    s.push_str(&format!("        payload: {}\n", one_line));
                }
            }
        }
    }
    s.push('\n');

    // §6 Branch lineage + parent_tx state (TB-7R 2026-05-02)
    // Per architect verdict 2026-05-02 (parent_tx ParentTx/DAG/Smoke ruling),
    // the dashboard MUST distinguish:
    //   - SingletonGoldenPathValid (B′ singleton solve; parent_tx=None correct)
    //   - NoMultiAttemptObserved (DAG not exercised; conformance test demonstrates plumbing)
    //   - MultiAttemptDagValid (≥1 multi-attempt branch with all parent_tx populated)
    //   - MissingParentTxViolation (≥1 multi-attempt branch with missing parent_tx)
    s.push_str("§6 Branch lineage (parent_tx → child_tx via ProposalTelemetry.parent_tx)\n");
    s.push_str("------------------------------------------------------------------------\n");
    let pt_state_label = match r.run_facts.parent_tx_state {
        turingosv4::runtime::chain_derived_run_facts::ParentTxState::SingletonGoldenPathValid =>
            "SingletonGoldenPathValid (B′ singleton solve — parent_tx=None correct; conformance test demonstrates plumbing)",
        turingosv4::runtime::chain_derived_run_facts::ParentTxState::NoMultiAttemptObserved =>
            "NoMultiAttemptObserved (DAG not exercised this run — conformance test demonstrates plumbing)",
        turingosv4::runtime::chain_derived_run_facts::ParentTxState::MultiAttemptDagValid =>
            "MultiAttemptDagValid ✓ (≥1 multi-attempt branch with all parent_tx edges present)",
        turingosv4::runtime::chain_derived_run_facts::ParentTxState::MissingParentTxViolation =>
            "MissingParentTxViolation ✗ (≥1 multi-attempt branch with missing parent_tx — wiring broken)",
    };
    s.push_str(&format!("  parent_tx_state: {}\n", pt_state_label));
    if r.branch_lineage.is_empty() {
        s.push_str("  edges: (none — see parent_tx_state above for interpretation)\n");
    } else {
        s.push_str("  edges:\n");
        for edge in &r.branch_lineage {
            s.push_str(&format!(
                "    [{}] {} → {}\n",
                edge.branch_id, edge.parent_tx, edge.child_tx
            ));
        }
    }
    s.push('\n');

    // §7 Golden path (TB-7.7 D6)
    s.push_str("§7 Golden path (root → oracle-verified WorkTx)\n");
    s.push_str("------------------------------------------------\n");
    if r.golden_path.is_empty() {
        if r.run_facts.chain_oracle_verified {
            s.push_str("  (chain_oracle_verified=true but golden path empty — likely VR linkage missing)\n");
        } else {
            s.push_str("  (no oracle-verified WorkTx on chain — chain_oracle_verified=false)\n");
        }
    } else {
        for step in &r.golden_path {
            let marker = if step.oracle_verified { "✓" } else { " " };
            s.push_str(&format!(
                "  {}depth={:<2} {} | agent={} | tactic={} | tx={}\n",
                marker,
                step.depth,
                if step.oracle_verified { "[ORACLE]" } else { "        " },
                step.agent_id,
                step.candidate_tactic,
                step.tx_id,
            ));
            if !step.payload_preview.is_empty() {
                let one_line = step.payload_preview.replace('\n', " ⏎ ");
                s.push_str(&format!("           payload: {}\n", one_line));
            }
        }
    }
    s.push('\n');

    // §8 Cross-checks
    s.push_str("§8 Cross-checks\n");
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

    // §9 TB-8 Claims (Atom 6) — claim_status + payout_amount per row.
    // Per user-minimum requirement: dashboard MUST show payout. The
    // payout_amount column is populated when a FinalizeRewardTx for the
    // claim_id appears on chain. The cross-check FinalizeRewardTx.reward
    // == claim.amount is enforced at the dispatch arm (Atom 3 step 5);
    // the dashboard reflects what landed on chain.
    s.push('\n');
    s.push_str("§9 TB-8 Claims (claim_status + payout_amount)\n");
    s.push_str("----------------------------------------------\n");
    if r.claims.is_empty() {
        s.push_str("  (no Confirm-VerifyTx observed; n/a — claim_status / payout: n/a)\n");
    } else {
        s.push_str(
            "  claim_id                          | task_id        | solver        | status     | payout_micro | created@t | finalized@t\n"
        );
        s.push_str(
            "  ----------------------------------+----------------+---------------+------------+--------------+-----------+------------\n"
        );
        for c in &r.claims {
            s.push_str(&format!(
                "  {:<33} | {:<14} | {:<13} | {:<10} | {:>12} | {:>9} | {}\n",
                trunc(&c.claim_id, 33),
                trunc(&c.task_id, 14),
                trunc(&c.solver, 13),
                c.claim_status,
                c.payout_amount_micro
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| "—".into()),
                c.created_at_logical_t,
                c.finalized_at_logical_t
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "—".into()),
            ));
        }
        // Aggregate: total payout sum (Finalized claims only).
        let total_payout: i64 = r
            .claims
            .iter()
            .filter_map(|c| c.payout_amount_micro)
            .sum();
        let n_open = r.claims.iter().filter(|c| c.claim_status == "Open").count();
        let n_finalized = r.claims.iter().filter(|c| c.claim_status == "Finalized").count();
        s.push_str(&format!(
            "\n  Aggregate: {} claims observed | {} Open | {} Finalized | total_payout = {} micro\n",
            r.claims.len(), n_open, n_finalized, total_payout
        ));
    }

    // §10 TB-9 Durable identity (Atom 6) — surface the agent_pubkeys manifest
    // alongside the (env-resolved) durable keystore path. Per architect
    // mandate "持仓、payout、future NodeMarket 都必须归属于 durable identity",
    // every Work-tx-signing pubkey on chain is bound to a row in the durable
    // keystore. The dashboard reflects the per-run manifest (snapshot) and
    // names the durable keystore path so an auditor can independently verify
    // that the pubkey survives evaluator restart.
    s.push('\n');
    s.push_str("§10 TB-9 Durable identity (agent keystore registry)\n");
    s.push_str("---------------------------------------------------\n");
    let keystore_path = std::env::var("TURINGOS_AGENT_KEYSTORE_PATH")
        .ok()
        .or_else(|| {
            std::env::var("HOME").ok().map(|h| {
                format!("{}/.turingos/keystore/agent_keystore.enc", h)
            })
        })
        .unwrap_or_else(|| "<unset; set TURINGOS_AGENT_KEYSTORE_PATH or HOME>".into());
    s.push_str(&format!("  durable_keystore_path: {}\n", keystore_path));
    let durable_present = std::path::Path::new(&keystore_path).exists();
    s.push_str(&format!(
        "  durable_keystore_present: {}\n",
        if durable_present { "✓ (cross-run identity available)" } else { "✗ (run-local only)" }
    ));
    s.push_str(&format!(
        "  agents_in_manifest: {}\n",
        r.per_agent.values().filter(|a| a.has_pubkey).count()
    ));
    s.push_str("  agent_id          | pubkey_in_manifest | tape_activity\n");
    s.push_str("  ------------------+--------------------+---------------\n");
    for (id, act) in &r.per_agent {
        if !act.has_pubkey { continue; }
        let activity = format!(
            "Work✓={} Work✗={} Verify✓={} Verify✗={}",
            act.work_tx_accepted, act.work_tx_rejected,
            act.verify_tx_accepted, act.verify_tx_rejected
        );
        s.push_str(&format!(
            "  {:<17} | {:<18} | {}\n",
            trunc(id, 17), "✓ (durable-backed)", activity
        ));
    }
    if r.per_agent.values().filter(|a| a.has_pubkey).count() == 0 {
        s.push_str("  (no agents with manifest pubkey on this run)\n");
    }
    s.push_str("\n  Note: cross-run identity is empirically observable by\n");
    s.push_str("  comparing this run's `agent_pubkeys.json` to a sibling run\n");
    s.push_str("  that loaded the same TURINGOS_AGENT_KEYSTORE_PATH — equal\n");
    s.push_str("  pubkey rows ⇒ TB-9 mandate \"agent identity survives run\n");
    s.push_str("  restart\" satisfied.\n");

    // §11 TB-10 User Tasks (first user-facing product).
    //
    // Filter convention: TaskOpenTx whose sponsor_agent starts with
    // `Agent_user_` (lean_market CLI binds `Agent_user_0` as the canonical
    // sponsor identity per runtime preseed factory `default_pput_preseed_pairs`).
    // Per TB-10 charter §3 Atom 4 + ratification §2.3.
    s.push('\n');
    s.push_str("§11 TB-10 User Tasks (sponsored by Agent_user_*; lean_market product surface)\n");
    s.push_str("------------------------------------------------------------------------------\n");
    if r.user_tasks.is_empty() {
        s.push_str("  (no Agent_user_*-sponsored TaskOpen on chain; lean_market run-task\n");
        s.push_str("   not invoked, or evaluator ran in self-funded preseed mode\n");
        s.push_str("   [TURINGOS_USER_TASK_MODE unset]; n/a)\n");
    } else {
        s.push_str(
            "  task_id              | sponsor      | bounty_micro | solver       | claim_status | payout_micro | opened@t\n"
        );
        s.push_str(
            "  ---------------------+--------------+--------------+--------------+--------------+--------------+---------\n"
        );
        for ut in &r.user_tasks {
            s.push_str(&format!(
                "  {:<20} | {:<12} | {:>12} | {:<12} | {:<12} | {:>12} | {:>7}\n",
                trunc(&ut.task_id, 20),
                trunc(&ut.sponsor, 12),
                ut.bounty_micro,
                trunc(&ut.solver, 12),
                ut.claim_status,
                ut.payout_micro
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| "—".into()),
                ut.opened_at_logical_t,
            ));
        }
        let total_bounty: i64 = r.user_tasks.iter().map(|u| u.bounty_micro).sum();
        let total_paid: i64 = r.user_tasks.iter().filter_map(|u| u.payout_micro).sum();
        let n_finalized = r
            .user_tasks
            .iter()
            .filter(|u| u.claim_status == "Finalized")
            .count();
        s.push_str(&format!(
            "\n  Aggregate: {} user task(s) | {} Finalized | total bounty = {} micro | total paid = {} micro\n",
            r.user_tasks.len(), n_finalized, total_bounty, total_paid
        ));
        s.push_str(
            "\n  Architect mandate (line 1594) ✓ when total paid > 0:\n"
        );
        s.push_str(
            "    user posts task → agent solves → system verifies → system pays → dashboard auditable.\n"
        );
        s.push_str(
            "    solver durable agent_id receives payout via TB-9 keystore-bound balances_t entry.\n"
        );
    }

    // §12 TB-11 Epistemic Exhaust + Capital Liberation (architect §6.2 ruling
    // 2026-05-02). Surfaces architect-mandated chain-resident anchors:
    //   - Exhausted runs (TerminalSummaryTx ≡ RunExhausted): O(N) audit via
    //     evidence_capsule_cid → CAS bytes; raw log shielded by AuditOnly default.
    //   - Expired tasks (TaskExpireTx): capital release path; CTF preserved.
    //   - Bankrupt tasks (TaskBankruptcyTx): future TB-12 Short / NO settlement
    //     death-cert anchor.
    s.push('\n');
    s.push_str("§12 TB-11 Epistemic Exhaust + Capital Liberation (architect §6.2; 2026-05-02)\n");
    s.push_str("------------------------------------------------------------------------------\n");

    if r.exhausted_runs.is_empty() {
        s.push_str("  (no TerminalSummary L4 entries — no runs have been anchored as exhausted/completed yet)\n");
    } else {
        s.push_str("  Exhausted runs (RunExhaustedTx ≡ TerminalSummaryTx):\n");
        s.push_str("    run_id         | task_id            | outcome         | attempts | evidence_capsule_cid (hex)\n");
        s.push_str("    ---------------+--------------------+-----------------+----------+--------------------------------\n");
        for er in &r.exhausted_runs {
            let cap_short = if er.evidence_capsule_cid_hex.len() > 32 {
                format!("{}…", &er.evidence_capsule_cid_hex[0..31])
            } else {
                er.evidence_capsule_cid_hex.clone()
            };
            s.push_str(&format!(
                "    {:<14} | {:<18} | {:<15} | {:>8} | {}\n",
                trunc(&er.run_id, 14),
                trunc(&er.task_id, 18),
                trunc(&er.run_outcome, 15),
                er.attempt_count,
                cap_short,
            ));
        }
    }

    if !r.expired_tasks.is_empty() {
        s.push('\n');
        s.push_str("  Expired tasks (TaskExpireTx; capital released):\n");
        s.push_str("    task_id            | sponsor      | refund_micro | reason             | @logical_t\n");
        s.push_str("    -------------------+--------------+--------------+--------------------+-----------\n");
        let mut total_refund: i64 = 0;
        for ex in &r.expired_tasks {
            total_refund += ex.refund_micro;
            s.push_str(&format!(
                "    {:<18} | {:<12} | {:>12} | {:<18} | {:>9}\n",
                trunc(&ex.task_id, 18),
                trunc(&ex.sponsor, 12),
                ex.refund_micro,
                trunc(&ex.reason, 18),
                ex.expired_at_logical_t,
            ));
        }
        s.push_str(&format!(
            "    ─── total refunded: {} micro across {} expired task(s) ───\n",
            total_refund,
            r.expired_tasks.len()
        ));
    }

    if !r.bankrupt_tasks.is_empty() {
        s.push('\n');
        s.push_str("  Bankrupt tasks (TaskBankruptcyTx; chain-resident death certificate):\n");
        s.push_str("    task_id            | reason                | failed_runs | evidence_capsule_cid (hex)\n");
        s.push_str("    -------------------+-----------------------+-------------+--------------------------------\n");
        for bk in &r.bankrupt_tasks {
            let cap_short = if bk.evidence_capsule_cid_hex.len() > 32 {
                format!("{}…", &bk.evidence_capsule_cid_hex[0..31])
            } else {
                bk.evidence_capsule_cid_hex.clone()
            };
            s.push_str(&format!(
                "    {:<18} | {:<21} | {:>11} | {}\n",
                trunc(&bk.task_id, 18),
                trunc(&bk.bankruptcy_reason, 21),
                bk.failed_run_count,
                cap_short,
            ));
        }
    }

    s.push('\n');
    s.push_str("  Architect mandate (§6.2 ruling 2026-05-02) ✓:\n");
    s.push_str("    O(1) chain cost / O(N) auditability — failure evidence anchored on L4\n");
    s.push_str("    via system-emitted system_signature; raw log requires audit-role access\n");
    s.push_str("    (CapsulePrivacyPolicy::AuditOnly default; only public_summary surfaces here).\n");

    // §13 TB-12 Node exposure records (architect 2026-05-03 ruling §3 + §10).
    s.push_str(&render_section_13(&r.exposures));
    s
}

/// TRACE_MATRIX TB-12 Atom 4 (architect 2026-05-03 ruling §8 Atom 4 + §10):
/// §13 Node exposure records render. Pure function over Vec<ExposureRecordRow>;
/// extracted for SG-12.6 unit-testability. ARCHITECT-MANDATED LABEL:
/// "Exposure records", NOT "Open market balances". TB-12 is exposure
/// index, NOT trading market — NodePosition is IMMUTABLE EXPOSURE RECORD
/// (architect §10), not active position balance. CR-12.1 + CR-12.2.
fn render_section_13(exposures: &[ExposureRecordRow]) -> String {
    let mut s = String::new();
    s.push('\n');
    s.push_str("§13 TB-12 Node exposure records (architect 2026-05-03 §3 + §10)\n");
    s.push_str("------------------------------------------------------------------------------\n");

    if exposures.is_empty() {
        s.push_str("  (no NodePosition records — no accepted WorkTx/ChallengeTx with stake>0 on this chaintape)\n");
    } else {
        s.push_str("  NodePosition exposure records (immutable; NOT Coin holdings; NOT in total_supply):\n");
        s.push_str("    position_id      | node_id          | side  | kind            | owner          | amount_micro | @round\n");
        s.push_str("    -----------------+------------------+-------+-----------------+----------------+--------------+--------\n");
        let mut total_long: i64 = 0;
        let mut total_short: i64 = 0;
        for ex in exposures {
            if ex.side == "Long" {
                total_long += ex.amount_micro;
            } else if ex.side == "Short" {
                total_short += ex.amount_micro;
            }
            s.push_str(&format!(
                "    {:<16} | {:<16} | {:<5} | {:<15} | {:<14} | {:>12} | {:>6}\n",
                trunc(&ex.position_id, 16),
                trunc(&ex.node_id, 16),
                ex.side,
                ex.kind,
                trunc(&ex.owner, 14),
                ex.amount_micro,
                ex.opened_at_round,
            ));
        }
        s.push_str(&format!(
            "    ─── Total Long: {} micro | Total Short: {} micro | exposure rows: {} ───\n",
            total_long,
            total_short,
            exposures.len()
        ));

        // Per-node aggregation.
        use std::collections::BTreeMap as RenderBTreeMap;
        let mut by_node: RenderBTreeMap<&str, (i64, i64)> = RenderBTreeMap::new();
        for ex in exposures {
            let entry = by_node.entry(&ex.node_id).or_insert((0, 0));
            if ex.side == "Long" {
                entry.0 += ex.amount_micro;
            } else if ex.side == "Short" {
                entry.1 += ex.amount_micro;
            }
        }
        if by_node.len() > 1 {
            s.push('\n');
            s.push_str("  Per-node exposure aggregation:\n");
            s.push_str("    node_id          | long_micro | short_micro | net (long − short)\n");
            s.push_str("    -----------------+------------+-------------+--------------------\n");
            for (nid, (lo, sh)) in by_node.iter() {
                s.push_str(&format!(
                    "    {:<16} | {:>10} | {:>11} | {:>18}\n",
                    trunc(nid, 16),
                    lo,
                    sh,
                    lo - sh
                ));
            }
        }
    }

    s.push('\n');
    s.push_str("  Architect mandate (§3 + §10 ruling 2026-05-03) ✓:\n");
    s.push_str("    NodePosition is an IMMUTABLE EXPOSURE RECORD, NOT active position balance.\n");
    s.push_str("    NodePosition.amount is NOT a Coin holding (CR-12.1) and is NOT counted in\n");
    s.push_str("    total_supply_micro (CR-12.2). NO trading. NO price. NO settlement in TB-12.\n");
    s.push_str("    NodeMarketEntry is TB-14 derived view; flat NodePositionsIndex is canonical.\n");
    s
}

/// TB-8 Atom 6 — truncate a string to width, padding/truncating with '…'
/// for clean dashboard alignment.
fn trunc(s: &str, width: usize) -> String {
    if s.len() <= width {
        s.to_string()
    } else if width >= 1 {
        let mut t: String = s.chars().take(width.saturating_sub(1)).collect();
        t.push('…');
        t
    } else {
        String::new()
    }
}

// ────────────────────────────────────────────────────────────────────────────
// TB-12 Atom 4 + Atom 6(a) — SG-12.6 dashboard rendering tests
// (architect 2026-05-03 §9.3 ruling).
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tb12_render_tests {
    use super::*;

    fn make_long(position_id: &str, node_id: &str, owner: &str, amount: i64) -> ExposureRecordRow {
        ExposureRecordRow {
            position_id: position_id.into(),
            node_id: node_id.into(),
            task_id: format!("task-{position_id}"),
            owner: owner.into(),
            side: "Long".into(),
            kind: "FirstLong".into(),
            amount_micro: amount,
            source_tx: position_id.into(),
            opened_at_round: 1,
        }
    }

    fn make_short(position_id: &str, node_id: &str, owner: &str, amount: i64) -> ExposureRecordRow {
        ExposureRecordRow {
            position_id: position_id.into(),
            node_id: node_id.into(),
            task_id: format!("task-{position_id}"),
            owner: owner.into(),
            side: "Short".into(),
            kind: "ChallengeShort".into(),
            amount_micro: amount,
            source_tx: position_id.into(),
            opened_at_round: 2,
        }
    }

    /// SG-12.6 (architect §9.3 exact name): dashboard view-positions /
    /// §13 rendering works. Verifies:
    /// - empty exposures list renders empty-state message
    /// - non-empty exposures render the architect-mandated label
    ///   "Exposure records" (NOT "Open market balances")
    /// - row content includes position_id / node_id / side / kind / owner /
    ///   amount per architect §8 Atom 4 spec
    /// - aggregation totals computed correctly (Total Long / Total Short)
    /// - per-node aggregation when ≥2 distinct nodes
    /// - architect mandate footer present (CR-12.1 + CR-12.2 immutability +
    ///   non-Coin claims)
    #[test]
    fn sg_12_6_dashboard_view_positions_works() {
        // Case 1: empty.
        let s_empty = render_section_13(&[]);
        assert!(s_empty.contains("§13 TB-12 Node exposure records"));
        assert!(s_empty.contains("(no NodePosition records"));
        // Architect mandate footer always present.
        assert!(s_empty.contains("IMMUTABLE EXPOSURE RECORD"));
        assert!(s_empty.contains("CR-12.1"));
        assert!(s_empty.contains("CR-12.2"));
        // LABEL DISCIPLINE: must NOT use "Open market balances".
        assert!(
            !s_empty.contains("Open market balances"),
            "architect §8 Atom 4 label discipline: must NOT use 'Open market balances'"
        );

        // Case 2: single FirstLong only.
        let exposures = vec![make_long("work-A", "work-A", "solver-A", 50_000)];
        let s_one = render_section_13(&exposures);
        assert!(
            s_one.contains("exposure records"),
            "architect §8 Atom 4 label discipline: contains 'exposure records' phrase"
        );
        assert!(s_one.contains("work-A"));
        assert!(s_one.contains("solver-A"));
        assert!(s_one.contains("FirstLong"));
        assert!(s_one.contains("Long"));
        assert!(s_one.contains("50000"));
        assert!(s_one.contains("Total Long: 50000 micro"));
        assert!(s_one.contains("Total Short: 0 micro"));
        // Per-node aggregation only renders when ≥2 nodes; single node => no per-node section.
        assert!(!s_one.contains("Per-node exposure aggregation"));

        // Case 3: FirstLong + ChallengeShort on same node → 1 node, no per-node block.
        let same_node = vec![
            make_long("work-B", "work-B", "solver-B", 30_000),
            make_short("chal-B", "work-B", "challenger-B", 20_000),
        ];
        let s_same = render_section_13(&same_node);
        assert!(s_same.contains("FirstLong"));
        assert!(s_same.contains("ChallengeShort"));
        assert!(s_same.contains("Total Long: 30000 micro"));
        assert!(s_same.contains("Total Short: 20000 micro"));
        assert!(s_same.contains("exposure rows: 2"));

        // Case 4: 2 nodes → per-node aggregation block renders.
        let two_nodes = vec![
            make_long("work-C", "work-C", "solver-C", 75_000),
            make_long("work-D", "work-D", "solver-D", 25_000),
            make_short("chal-D", "work-D", "challenger-D", 10_000),
        ];
        let s_two = render_section_13(&two_nodes);
        assert!(s_two.contains("Per-node exposure aggregation"));
        // node "work-C": long=75000, short=0, net=75000
        assert!(s_two.contains("work-C"));
        // node "work-D": long=25000, short=10000, net=15000
        assert!(s_two.contains("work-D"));
        assert!(s_two.contains("Total Long: 100000 micro"));
        assert!(s_two.contains("Total Short: 10000 micro"));
        assert!(s_two.contains("exposure rows: 3"));

        // FORBIDDEN tokens (architect §9.4): must NOT appear in dashboard
        // (this catches accidental drift if a future patch adds price/trading
        // language to §13 rendering).
        for forbidden in &[
            "Open market balances",
            "MarketBuy",
            "MarketSell",
            "MarketOrder",
            "MarketTrade",
            "price_yes",
            "price_no",
            "automatic liquidity",
            "ghost liquidity",
        ] {
            assert!(
                !s_two.contains(forbidden),
                "architect §9.4 forbidden token '{forbidden}' must NOT appear in §13 render"
            );
        }
    }
}
