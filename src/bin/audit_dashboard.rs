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

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::system_keypair::{
    PinnedSystemPubkeys, SystemEpoch, SystemPublicKey,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    canonical_decode, replay_full_transition, Git2LedgerWriter, LedgerCasView, LedgerEntry,
    LedgerWriter, ReplayError, TxKind,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::agent_audit_trail::AgentAuditTrailIndex;
use turingosv4::runtime::agent_keypairs::AgentPubkeyManifest;
use turingosv4::runtime::chain_derived_run_facts::{
    compute_run_facts_from_chain, ChainDerivedRunFacts,
};
use turingosv4::runtime::proposal_telemetry::read_from_cas as read_proposal_telemetry;
use turingosv4::runtime::verify::{verify_chaintape, ReplayReport, VerifyOptions};
use turingosv4::runtime::PinnedPubkeyManifest;
use turingosv4::state::q_state::QState;
use turingosv4::state::q_state::{AgentId, EconomicState};
use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide, TypedTx};
use turingosv4::state::{compute_price_index, NodeMarketEntry, TaskId, TxId};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

#[derive(Debug)]
struct Args {
    repo: PathBuf,
    cas: PathBuf,
    json: bool,
    out: Option<PathBuf>,
    /// TB-16.x.fix (architect OBS_R022 Option α): explicit Markov capsule
    /// cid (hex). Replaces the prior implicit read from the global
    /// `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt` file (Art.
    /// 0.2 parallel ledger; de-canonicalized 2026-05-04). Caller is
    /// expected to derive the cid from the chain itself or supply it
    /// explicitly when rendering audit dashboards. Absence ≡ no Markov
    /// capsule pointer surfaced; §15 renders the empty-state hint.
    markov_capsule_cid: Option<String>,
    /// TB-N3 A5 (architect ruling 2026-05-11 SG-N3.12 + charter v3 §3):
    /// when true, emit a TB-N3 run-report (§A citation tree + §B role
    /// activity + §C market tx counts + §D top contested nodes + §E
    /// budget burn + §F MarketDecisionTrace aggregate + §G signal-not-
    /// truth banner) APPENDED to the existing dashboard render. Reads
    /// the chain + CAS the same way as the legacy dashboard; pure
    /// materialized view per CLAUDE.md §17 Report Standard.
    run_report: bool,
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut repo: Option<PathBuf> = None;
    let mut cas: Option<PathBuf> = None;
    let mut json = false;
    let mut out: Option<PathBuf> = None;
    let mut markov_capsule_cid: Option<String> = None;
    let mut run_report: bool = false;
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
            "--markov-capsule-cid" => {
                i += 1;
                markov_capsule_cid = Some(
                    argv.get(i)
                        .ok_or("missing value after --markov-capsule-cid")?
                        .clone(),
                );
            }
            "--run-report" => run_report = true,
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
        markov_capsule_cid,
        run_report,
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
    /// TB-14 Atom 6 (architect 2026-05-03 ruling §5.1 + §5.5 SG-14.6):
    /// derived price-index view per `compute_price_index` over a synthetic
    /// `EconomicState` rebuilt from `exposures`. Renders in §14 with the
    /// `PRICE IS SIGNAL, NOT TRUTH` banner per architect §5.1 ("Price is
    /// signal, not truth") and SG-14.6 unit test discipline. NEVER shown
    /// as decimal — every price is rendered as `numerator/denominator`
    /// integer-rational pair (charter §5 forbidden list: no f64 / no
    /// decimal float in TB-14 module surface).
    price_index: BTreeMap<TxId, NodeMarketEntry>,
    /// TB-15 Atom 6 (architect §6.5 SG-15.6): per-event autopsy Cid
    /// counts derived from on-chain `EconomicState.agent_autopsies_t`
    /// at snapshot time. Empty Vec when no TaskBankruptcyTx has fired.
    /// Architect §6.4 privacy: dashboard surfaces COUNTS + COMPRESSED
    /// `public_summary` strings only — never `private_detail_cid` bytes.
    autopsy_event_counts: Vec<(String /*event_id*/, u32 /*cid_count*/)>,
    /// TB-15 Atom 6 / TB-16.x.fix (architect OBS_R022 Option α): latest
    /// Markov capsule cid (hex). Previously sourced from the global
    /// `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt` file (Art.
    /// 0.2 parallel ledger; de-canonicalized 2026-05-04). Now supplied
    /// explicitly via `--markov-capsule-cid <hex>` CLI arg or `None`
    /// (empty-state hint rendered in §15). Full in-tape resolver lands
    /// with TB-16.x.2.4 / 2.6 (β chain continuation).
    latest_markov_capsule_cid_hex: Option<String>,
    /// TRACE_MATRIX FC2-N33 (TB-16 Atom 4; architect §7.4 CR-16.7 +
    /// §7.5 SG-16.8): true when ANY agent_id encountered during the L4
    /// walk OR in the agent_pubkeys.json manifest matches a sandbox-only
    /// prefix (Agent_solver_/Agent_verifier_/Agent_user_/tb7-7-sponsor/
    /// tb16-). Drives §16 banner; prevents dashboard readers from
    /// interpreting sandbox prices/positions as production signals.
    sandbox_run: bool,
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
    l4e_last_hash_hex: String,
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
    challenge_tx_accepted: u64,
    invest_tx_accepted: u64,
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
                "usage: audit_dashboard --repo <runtime_repo> --cas <cas> [--json] \
                 [--out <path>] [--markov-capsule-cid <hex>]"
            );
            std::process::exit(2);
        }
    };

    let report = match build_report(
        &parsed.repo,
        &parsed.cas,
        parsed.markov_capsule_cid.as_deref(),
    ) {
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
        let mut out = render_text(&report);
        if parsed.run_report {
            // TB-N3 A5 — append §A..§G TB-N3 run report sections.
            let tb_n3 = render_tb_n3_run_report(&report, &parsed.repo, &parsed.cas);
            out.push_str("\n");
            out.push_str(&tb_n3);
        }
        out
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

fn build_report(
    repo: &std::path::Path,
    cas_path: &std::path::Path,
    markov_capsule_cid: Option<&str>,
) -> Result<DashboardReport, String> {
    // Replay verifier — gives us the 7 indicators + chain root state.
    let replay: ReplayReport = verify_chaintape(repo, cas_path, &VerifyOptions::default())
        .map_err(|e| format!("verify_chaintape: {e:?}"))?;

    let run_facts = compute_run_facts_from_chain(repo, cas_path)
        .map_err(|e| format!("chain_derived_run_facts: {e:?}"))?;

    // Walk L4 entries to populate per_agent + proposal_flow + branch_lineage.
    let writer = Git2LedgerWriter::open(repo).map_err(|e| format!("open ledger: {e:?}"))?;
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
                let acct = per_agent
                    .entry(challenge.challenger_agent.0.clone())
                    .or_default();
                acct.challenge_tx_accepted += 1;
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
                let acct = per_agent
                    .entry(verify.verifier_agent.0.clone())
                    .or_default();
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
                    let (solver_id, task_id) =
                        solver.unwrap_or_else(|| ("(unknown)".into(), "(unknown)".into()));
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
            TypedTx::BuyWithCoinRouter(router) => {
                let acct = per_agent.entry(router.buyer.0.clone()).or_default();
                acct.invest_tx_accepted += 1;
                proposal_flow.push(ProposalFlowEntry {
                    logical_t,
                    side: "L4",
                    tx_kind: "BuyWithCoinRouter".into(),
                    agent_id: Some(router.buyer.0.clone()),
                    tx_id: Some(router.tx_id.0.clone()),
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
        let mut cursor = work_parent_by_tx_id.get(winner_tx_id).cloned().flatten();
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
                chain.push((
                    parent.clone(),
                    String::new(),
                    String::new(),
                    String::new(),
                    false,
                ));
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
    let audit_trail_rows = audit_trail_index
        .as_ref()
        .map(|i| i.len() as u64)
        .unwrap_or(0);
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
        if let Some(claim) = claims_in_progress.iter().find(|c| c.task_id == ut.task_id) {
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
            l4e_last_hash_hex: replay.detail.l4e_last_hash_hex.clone(),
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
        price_index: price_index_from_exposures(&exposures_in_progress),
        exposures: exposures_in_progress,
        // TB-16 Atom 4 — closes OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16: live
        // regeneration via replay_full_transition over the L4 entries.
        // Rebuilds EconomicState from chain alone (Art.0.2 Tape Canonical;
        // P1-Exit8 state.db deletable). Falls back to Vec::new() if
        // replay fails (e.g. partial chain, missing CAS) — failure mode
        // surfaces via §15 banner, not a silent zero.
        autopsy_event_counts: rebuild_autopsy_event_counts(repo, &entries, &cas),
        sandbox_run: detect_sandbox_run(&entries, &cas, manifest.as_ref()),
        // TB-16.x.fix (architect OBS_R022 Option α): explicit caller-
        // supplied cid. The previous `read_latest_markov_pointer()`
        // helper read `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt`
        // — that path was an Art. 0.2 parallel ledger and has been
        // removed from the runtime path per architect ruling §B.7.1.
        latest_markov_capsule_cid_hex: markov_capsule_cid.map(|s| s.trim().to_string()),
    })
}

/// TRACE_MATRIX TB-15 Atom 6 (FR-15.4 + SG-15.6): best-effort read of
/// `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt` from the
/// repo-root convention path. Returns None when the file is absent
/// (e.g. fresh repo without TB-15 generation yet) or unreadable.
/// TRACE_MATRIX FC2-N33 (TB-16 Atom 4; architect §7.4 CR-16.7 + §7.5
/// SG-16.8): scan all L4 entries + agent_pubkeys manifest for any
/// agent_id matching a sandbox-only prefix.
fn detect_sandbox_run(
    entries: &[LedgerEntry],
    cas: &CasStore,
    manifest: Option<&AgentPubkeyManifest>,
) -> bool {
    let is_sandbox = |id: &str| -> bool {
        id.starts_with("Agent_solver_")
            || id.starts_with("Agent_verifier_")
            || id.starts_with("Agent_user_")
            || id == "tb7-7-sponsor"
            || id.starts_with("tb16-")
    };
    if let Some(m) = manifest {
        for k in m.agents.keys() {
            if is_sandbox(k) {
                return true;
            }
        }
    }
    for entry in entries {
        let payload = match cas.get(&entry.tx_payload_cid) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let typed: TypedTx = match canonical_decode(&payload) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let id = match &typed {
            TypedTx::Work(w) => w.agent_id.0.clone(),
            TypedTx::Verify(v) => v.verifier_agent.0.clone(),
            TypedTx::Challenge(c) => c.challenger_agent.0.clone(),
            TypedTx::TaskOpen(t) => t.sponsor_agent.0.clone(),
            TypedTx::EscrowLock(e) => e.sponsor_agent.0.clone(),
            TypedTx::CompleteSetMint(m) => m.owner.0.clone(),
            TypedTx::CompleteSetRedeem(r) => r.owner.0.clone(),
            TypedTx::MarketSeed(s) => s.provider.0.clone(),
            _ => continue,
        };
        if is_sandbox(&id) {
            return true;
        }
    }
    false
}

/// TRACE_MATRIX FC2-N32 (TB-16 Atom 4; architect §7.5 SG-16.2; closes
/// OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04): live regeneration of
/// `agent_autopsies_t` event counts via `replay_full_transition` over the
/// L4 chain. Returns `Vec<(event_id_string, cid_count)>` per architect
/// SG-15.6 + Codex R1 Q9 closure.
///
/// Falls back to `Vec::new()` on any replay error (chain corruption, CAS
/// missing, signature mismatch). The §15 render still shows a banner
/// distinguishing "no autopsies" from "replay failed".
fn rebuild_autopsy_event_counts(
    repo: &std::path::Path,
    entries: &[LedgerEntry],
    cas: &CasStore,
) -> Vec<(String, u32)> {
    // Resolve pinned pubkeys from runtime_repo/pinned_pubkeys.json.
    let pinned_path = repo.join("pinned_pubkeys.json");
    if !pinned_path.exists() {
        return Vec::new();
    }
    let pinned_text = match std::fs::read_to_string(&pinned_path) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };
    let pinned_manifest: PinnedPubkeyManifest = match serde_json::from_str(&pinned_text) {
        Ok(m) => m,
        Err(_) => return Vec::new(),
    };
    let mut pinned = PinnedSystemPubkeys::new();
    for entry in &pinned_manifest.pubkeys {
        let bytes = match hex_to_bytes(&entry.pubkey_hex) {
            Some(b) => b,
            None => return Vec::new(),
        };
        let arr: [u8; 32] = match bytes.as_slice().try_into() {
            Ok(a) => a,
            Err(_) => return Vec::new(),
        };
        pinned.insert(
            SystemEpoch::new(entry.epoch),
            SystemPublicKey::from_bytes(arr),
        );
    }

    // Initial QState (genesis or persisted snapshot).
    let initial_q_path = repo.join("initial_q_state.json");
    let initial_q = if initial_q_path.exists() {
        let s = match std::fs::read_to_string(&initial_q_path) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        match serde_json::from_str::<QState>(&s) {
            Ok(q) => q,
            Err(_) => return Vec::new(),
        }
    } else {
        QState::genesis()
    };

    let predicate_registry = PredicateRegistry::new();
    let tool_registry = ToolRegistry::new();
    let cas_view = AuditCasRef(cas);
    let final_q = match replay_full_transition(
        &initial_q,
        entries,
        &cas_view,
        &pinned,
        &predicate_registry,
        &tool_registry,
    ) {
        Ok(q) => q,
        Err(_) => return Vec::new(),
    };

    let mut out: Vec<(String, u32)> = Vec::new();
    for (event_id, cids) in &final_q.economic_state_t.agent_autopsies_t.0 {
        out.push((event_id.0 .0.clone(), cids.len() as u32));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn hex_to_bytes(hex: &str) -> Option<Vec<u8>> {
    let h = hex.trim();
    if h.len() % 2 != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(h.len() / 2);
    let mut iter = h.bytes();
    while let (Some(a), Some(b)) = (iter.next(), iter.next()) {
        let hi = char_hex(a)?;
        let lo = char_hex(b)?;
        out.push((hi << 4) | lo);
    }
    Some(out)
}

fn hex_encode_bytes(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn char_hex(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

struct AuditCasRef<'a>(&'a CasStore);
impl<'a> LedgerCasView for AuditCasRef<'a> {
    fn get_typed_payload(&self, cid: &Cid) -> Result<Vec<u8>, ReplayError> {
        self.0
            .get(cid)
            .map_err(|_| ReplayError::CasMissing { at: 0 })
    }
}

/// TRACE_MATRIX TB-14 Atom 6 (FC3-N42; architect §5.1 + §5.5 SG-14.6):
/// rebuild a synthetic `EconomicState` from the dashboard's `exposures`
/// vec and call the canonical `state::compute_price_index` over it.
///
/// **Why synthetic**: the dashboard does not run a full `replay_full_transition`
/// to produce a final `QState`; it walks the L4 chain forward to accumulate
/// audit rows. The exposures vec already carries `(node_id, side, amount_micro,
/// owner, task_id, source_tx, opened_at_round)` for every accepted FirstLong
/// (WorkTx) and ChallengeShort (ChallengeTx) — exactly the inputs `compute_price_index`
/// needs. By going through `compute_price_index` rather than re-implementing
/// the long/short aggregation here, the dashboard's price view is canonically
/// identical to the bus snapshot's price view (architect §5.1 "no second
/// source-of-truth"; charter §7 auto-resolution A).
///
/// The `kind` field is irrelevant to `compute_price_index` (which reads only
/// `side` + `amount` + `node_id` + `task_id`); we map by side as a placeholder.
/// `conditional_share_balances_t` is left empty, so the resulting
/// `NodeMarketEntry.yes_share_depth` / `no_share_depth` are zero — TB-14 v0
/// derives price from `node_positions_t` only (FR-14.1 / FR-14.2); share
/// depths are reported but not used in the price computation.
fn price_index_from_exposures(exposures: &[ExposureRecordRow]) -> BTreeMap<TxId, NodeMarketEntry> {
    let mut econ = EconomicState::default();
    for row in exposures {
        let (side, kind) = match row.side.as_str() {
            "Long" => (PositionSide::Long, PositionKind::FirstLong),
            "Short" => (PositionSide::Short, PositionKind::ChallengeShort),
            _ => continue, // unknown side string — drop defensively
        };
        let position = NodePosition {
            position_id: TxId(row.position_id.clone()),
            node_id: TxId(row.node_id.clone()),
            task_id: TaskId(row.task_id.clone()),
            owner: AgentId(row.owner.clone()),
            side,
            kind,
            amount: MicroCoin::from_micro_units(row.amount_micro),
            source_tx: TxId(row.source_tx.clone()),
            opened_at_round: row.opened_at_round,
        };
        econ.node_positions_t
            .0
            .insert(position.position_id.clone(), position);
    }
    compute_price_index(&econ)
}

fn render_text(r: &DashboardReport) -> String {
    let mut s = String::new();
    s.push_str("=================================================================\n");
    s.push_str(&format!(
        " TB-8 Audit Dashboard — run_id={} epoch={}\n",
        r.run_id, r.epoch
    ));
    s.push_str("=================================================================\n\n");

    // §1 Run metadata
    s.push_str("§1 Run metadata\n");
    s.push_str("---------------\n");
    s.push_str(&format!(
        "  head_commit_oid: {}\n",
        r.chain
            .head_commit_oid_hex
            .as_deref()
            .unwrap_or("(empty chain)")
    ));
    s.push_str(&format!("  l4e_last_hash: {}\n", r.chain.l4e_last_hash_hex));
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
        if r.indicators.ledger_root_verified {
            "✓"
        } else {
            "✗"
        }
    ));
    s.push_str(&format!(
        "  system_signatures_verified        : {}\n",
        if r.indicators.system_signatures_verified {
            "✓"
        } else {
            "✗"
        }
    ));
    s.push_str(&format!(
        "  state_reconstructed               : {}\n",
        if r.indicators.state_reconstructed {
            "✓"
        } else {
            "✗"
        }
    ));
    s.push_str(&format!(
        "  economic_state_reconstructed      : {}\n",
        if r.indicators.economic_state_reconstructed {
            "✓"
        } else {
            "✗"
        }
    ));
    s.push_str(&format!(
        "  cas_payloads_retrievable          : {}\n",
        if r.indicators.cas_payloads_retrievable {
            "✓"
        } else {
            "✗"
        }
    ));
    s.push_str(&format!(
        "  agent_signatures_verified [Gate 4]: {}\n",
        if r.indicators.agent_signatures_verified {
            "✓"
        } else {
            "✗"
        }
    ));
    s.push_str(&format!(
        "  proposal_telemetry_cas_retrievable [Gate 5]: {}\n",
        if r.indicators.proposal_telemetry_cas_retrievable {
            "✓"
        } else {
            "✗"
        }
    ));
    s.push_str(&format!(
        "  ALL 7 PASS                        : {}\n\n",
        if r.indicators.all_pass {
            "GREEN"
        } else {
            "RED"
        }
    ));

    // §3 ChainDerivedRunFacts
    s.push_str("§3 ChainDerivedRunFacts (§4.4 bit-exact set)\n");
    s.push_str("---------------------------------------------\n");
    s.push_str(&format!(
        "  solved                  : {}\n",
        r.run_facts.solved
    ));
    s.push_str(&format!(
        "  verified                : {}\n",
        r.run_facts.verified
    ));
    s.push_str(&format!(
        "  tx_count                : {}\n",
        r.run_facts.tx_count
    ));
    s.push_str(&format!(
        "  proposal_count          : {}\n",
        r.run_facts.proposal_count
    ));
    s.push_str(&format!(
        "  golden_path_token_count : {}\n",
        r.run_facts.golden_path_token_count
    ));
    s.push_str(&format!(
        "  gp_payload (CID hex)    : {}\n",
        r.run_facts.gp_payload.as_deref().unwrap_or("-")
    ));
    s.push_str(&format!(
        "  gp_path                 : {}\n",
        r.run_facts.gp_path.as_deref().unwrap_or("-")
    ));
    s.push_str(&format!(
        "  tactic_diversity        : {}\n",
        r.run_facts.tactic_diversity
    ));
    s.push_str(&format!(
        "  failed_branch_count     : {}\n",
        r.run_facts.failed_branch_count
    ));
    s.push_str(&format!(
        "  chain_oracle_verified   : {} {}\n",
        r.run_facts.chain_oracle_verified,
        if r.run_facts.chain_oracle_verified {
            "✓ (Lean accepted ≥1 proof; oracle-level)"
        } else {
            "(no oracle-verified WorkTx)"
        }
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
                if step.oracle_verified {
                    "[ORACLE]"
                } else {
                    "        "
                },
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
    s.push_str(&format!(
        "  audit_trail_rows         : {}\n",
        r.cross_checks.audit_trail_rows
    ));
    s.push_str(&format!(
        "  chain_proposal_count     : {}\n",
        r.cross_checks.chain_proposal_count
    ));
    s.push_str(&format!(
        "  audit_rows == proposal_count: {}\n",
        if r.cross_checks.proposal_count_matches_audit_rows {
            "✓"
        } else {
            "✗ (gap)"
        }
    ));
    s.push_str(&format!(
        "  audit_trail_chain_valid     : {}\n",
        if r.cross_checks.agent_audit_trail_chain_valid {
            "✓"
        } else {
            "✗"
        }
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
        let total_payout: i64 = r.claims.iter().filter_map(|c| c.payout_amount_micro).sum();
        let n_open = r.claims.iter().filter(|c| c.claim_status == "Open").count();
        let n_finalized = r
            .claims
            .iter()
            .filter(|c| c.claim_status == "Finalized")
            .count();
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
            std::env::var("HOME")
                .ok()
                .map(|h| format!("{}/.turingos/keystore/agent_keystore.enc", h))
        })
        .unwrap_or_else(|| "<unset; set TURINGOS_AGENT_KEYSTORE_PATH or HOME>".into());
    s.push_str(&format!("  durable_keystore_path: {}\n", keystore_path));
    let durable_present = std::path::Path::new(&keystore_path).exists();
    s.push_str(&format!(
        "  durable_keystore_present: {}\n",
        if durable_present {
            "✓ (cross-run identity available)"
        } else {
            "✗ (run-local only)"
        }
    ));
    s.push_str(&format!(
        "  agents_in_manifest: {}\n",
        r.per_agent.values().filter(|a| a.has_pubkey).count()
    ));
    s.push_str("  agent_id          | pubkey_in_manifest | tape_activity\n");
    s.push_str("  ------------------+--------------------+---------------\n");
    for (id, act) in &r.per_agent {
        if !act.has_pubkey {
            continue;
        }
        let activity = format!(
            "Work✓={} Work✗={} Verify✓={} Verify✗={}",
            act.work_tx_accepted,
            act.work_tx_rejected,
            act.verify_tx_accepted,
            act.verify_tx_rejected
        );
        s.push_str(&format!(
            "  {:<17} | {:<18} | {}\n",
            trunc(id, 17),
            "✓ (durable-backed)",
            activity
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
        s.push_str("\n  Architect mandate (line 1594) ✓ when total paid > 0:\n");
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
    s.push_str(
        "    (CapsulePrivacyPolicy::AuditOnly default; only public_summary surfaces here).\n",
    );

    // §13 TB-12 Node exposure records (architect 2026-05-03 ruling §3 + §10).
    s.push_str(&render_section_13(&r.exposures));

    // §14 TB-14 PriceIndex (architect 2026-05-03 ruling §5.1 + §5.5 SG-14.6).
    s.push_str(&render_section_14(&r.price_index));

    // §15 TB-15 Autopsy + Markov (architect 2026-05-02 §6.5 SG-15.6).
    s.push_str(&render_section_15(
        &r.autopsy_event_counts,
        r.latest_markov_capsule_cid_hex.as_deref(),
    ));

    // §16 TB-16 Sandbox banner (architect 2026-05-03 §7.4 CR-16.7 +
    // §7.5 SG-16.8). Rendered when ANY agent_id surfaced in the report
    // matches a sandbox-only prefix (Agent_solver_*, Agent_verifier_*,
    // Agent_user_*, tb7-7-sponsor, tb16-*). Scans per_agent +
    // claims.solver/sponsor + user_tasks.sponsor + exhausted_runs.solver +
    // exposures.owner so a sponsor-only chain (TaskOpen + EscrowLock +
    // TerminalSummary, no Work) still trips the banner.
    s.push_str(&render_section_16(r));

    s
}

/// TRACE_MATRIX FC2-N33 (TB-16 Atom 4; architect §7.4 CR-16.7 + §7.5
/// SG-16.8): SANDBOX banner render. Source-fence — emit when
/// `report.sandbox_run` is true (computed in build_report by scanning
/// the L4 walk + agent_pubkeys manifest); otherwise no banner.
fn render_section_16(r: &DashboardReport) -> String {
    if !r.sandbox_run {
        return String::new();
    }
    let mut s = String::new();
    s.push('\n');
    s.push_str("§16 TB-16 SANDBOX BANNER (architect 2026-05-03 §7.4 CR-16.7 + §7.5 SG-16.8)\n");
    s.push_str("==========================================================================\n");
    s.push_str("  ⚠ SANDBOX-RUN — NOT PRODUCTION — NO REAL FUNDS\n");
    s.push_str("    Agent IDs are sandbox-prefixed (Agent_solver_/Agent_verifier_/\n");
    s.push_str("    Agent_user_/tb7-7-sponsor/tb16-). Total Coin sourced from\n");
    s.push_str("    runtime::bootstrap::default_pput_preseed_pairs() (35_000_000 μC\n");
    s.push_str("    on_init mint; assert_no_post_init_mint enforced).\n");
    s.push_str("\n");
    s.push_str("    Architect §7.6 forbidden:\n");
    s.push_str("      - No public chain.\n");
    s.push_str("      - No real-money market.\n");
    s.push_str("      - No external domain (Lean only; no medical/legal/financial).\n");
    s.push_str("      - No production user funds.\n");
    s.push_str("\n");
    s.push_str("    Prices, positions, masks, autopsies surfaced above are SIGNAL\n");
    s.push_str("    only — never to be interpreted as real-money valuations.\n");
    s
}

/// TRACE_MATRIX TB-15 Atom 6 (architect §6.5 SG-15.6 + §6.4 privacy):
/// §15 Autopsy + Markov render. Pure function over (event Cid counts +
/// optional Markov capsule pointer); extracted for SG-15.6
/// unit-testability.
///
/// **ARCHITECT-MANDATED PRIVACY BANNER**: the section opens with the
/// literal phrase "AUTOPSY IS PRIVATE" (architect §6.4 + CR-15.1).
/// Re-rendering this banner in every dashboard frame is the SG-15.6 ship
/// gate's enforcement surface. Dashboard surfaces COUNTS + Markov
/// pointer ONLY — never `private_detail_cid` payload bytes (CR-15.1 +
/// halt-trigger #1 + halt-trigger #4).
///
/// **NO RAW PRIVATE DETAIL**: the function signature accepts only
/// `Vec<(String, u32)>` event counts + an optional Markov pointer hex.
/// Raw private bytes are structurally absent from the input set, so the
/// rendered output cannot leak them.
fn render_section_15(
    autopsy_event_counts: &[(String, u32)],
    latest_markov_capsule_cid_hex: Option<&str>,
) -> String {
    let mut s = String::new();
    s.push('\n');
    s.push_str("§15 TB-15 Autopsy + Markov (architect 2026-05-02 §6.5 SG-15.6)\n");
    s.push_str("--------------------------------------------------------------\n");
    s.push_str("  AUTOPSY IS PRIVATE — public summary shown only when typical\n");
    s.push_str("  (≥3 cluster). Raw private details require audit-role access.\n");
    s.push_str("    Architect §6.4 ruling 2026-05-02: capsule audit detail is\n");
    s.push_str("    AuditOnly; NEVER enters AgentVisibleProjection (CR-15.1 +\n");
    s.push_str("    halt-trigger #1 + #4).\n");
    s.push_str("    Typical-error broadcast surface uses public_summary text\n");
    s.push_str("    only (CR-15.2 + halt-trigger #5).\n\n");

    if autopsy_event_counts.is_empty() {
        s.push_str("  (no agent_autopsies_t entries in this snapshot — no\n");
        s.push_str("  TaskBankruptcyTx has fired during the chain window)\n");
        s.push_str("  Acceptable signal-state: a run with zero accepted\n");
        s.push_str("  TaskBankruptcyTx yields an empty AutopsyIndex by\n");
        s.push_str("  TB-15 Atom 3 charter scope (single trigger site).\n\n");
    } else {
        s.push_str("  Per-event Cid counts (capsule bytes live in CAS;\n");
        s.push_str("  audit-role required to fetch private_detail):\n\n");
        s.push_str(&format!("    {:<48}  {:>10}\n", "event_id", "cid_count"));
        s.push_str("    ");
        s.push_str(&"-".repeat(60));
        s.push('\n');
        let mut total_cids: u32 = 0;
        for (event_id, count) in autopsy_event_counts {
            total_cids += *count;
            s.push_str(&format!("    {:<48}  {:>10}\n", trunc(event_id, 48), count,));
        }
        s.push_str(&format!(
            "    ─── total: {} capsule Cid(s) across {} event(s) ───\n\n",
            total_cids,
            autopsy_event_counts.len()
        ));
    }

    s.push_str("  Markov default (FR-15.4): next-session boot reads\n");
    s.push_str("  constitution.md + latest Markov capsule. deeper history\n");
    s.push_str("  requires TURINGOS_MARKOV_OVERRIDE=1 (CR-15.6 +\n");
    s.push_str("  halt-trigger #6 — default-deny gate).\n\n");

    match latest_markov_capsule_cid_hex {
        Some(cid_hex) if !cid_hex.is_empty() => {
            s.push_str(&format!(
                "  Latest Markov capsule cid (supplied via\n  \
                --markov-capsule-cid; in-tape resolver lands with\n  \
                TB-16.x.2.4 / 2.6 β chain continuation):\n    {}\n",
                cid_hex
            ));
        }
        _ => {
            s.push_str("  (no latest Markov capsule pointer — supply\n");
            s.push_str("  --markov-capsule-cid <hex> on the audit_dashboard\n");
            s.push_str("  invocation, or run `generate_markov_capsule` to\n");
            s.push_str("  emit a per-run capsule and pass its cid here.\n");
            s.push_str("  Per architect OBS_R022 ruling 2026-05-04 the\n");
            s.push_str("  global LATEST_MARKOV_CAPSULE.txt file has been\n");
            s.push_str("  de-canonicalized — runtime path no longer reads it)\n");
        }
    }

    s.push('\n');
    s.push_str("  Architect mandate (§6.5 SG-15.6 + §6.4 ruling 2026-05-02) ✓:\n");
    s.push_str("    Dashboard regenerates capsule summary from ChainTape + CAS;\n");
    s.push_str("    NO raw private detail in dashboard output. Markov default\n");
    s.push_str("    prevents context poisoning — full failure history not auto-\n");
    s.push_str("    replayed; only constitution + latest capsule by default.\n");
    s
}

/// TRACE_MATRIX TB-14 Atom 6 (architect 2026-05-03 ruling §5.1 + §5.5 SG-14.6):
/// §14 PriceIndex render. Pure function over the derived view; extracted for
/// SG-14.6 unit-testability.
///
/// **ARCHITECT-MANDATED BANNER**: the section opens with the literal phrase
/// "PRICE IS SIGNAL, NOT TRUTH" (architect §5.1: "Price is signal, not
/// truth."). Re-rendering this banner in every dashboard frame is the
/// SG-14.6 ship gate's enforcement surface.
///
/// **NO DECIMAL** (charter §5 forbidden + G-14.11 ship gate "no f64 in TB-14
/// module surface"): every `price_yes` / `price_no` is rendered as
/// `numerator/denominator` integer-rational. The dashboard NEVER divides.
fn render_section_14(price_index: &BTreeMap<TxId, NodeMarketEntry>) -> String {
    let mut s = String::new();
    s.push('\n');
    s.push_str("§14 TB-14 PriceIndex (architect 2026-05-03 §5.1 + §5.5 SG-14.6)\n");
    s.push_str("---------------------------------------------------------------\n");
    s.push_str("  PRICE IS SIGNAL, NOT TRUTH.\n");
    s.push_str("    Architect §5.1 ruling 2026-05-03: the price index is a\n");
    s.push_str("    derived statistical broadcast over canonical NodePositionsIndex\n");
    s.push_str("    long/short interest. It MUST NOT influence predicate gates\n");
    s.push_str("    (CR-14.1 / halt-trigger #1) or L4/L4.E classification\n");
    s.push_str("    (CR-14.2 / halt-trigger #2). Boolean predicates establish\n");
    s.push_str("    absolute bounds; the price view is for relative-effectiveness\n");
    s.push_str("    measurement only.\n\n");

    if price_index.is_empty() {
        s.push_str("  (no node positions recorded — price index is empty)\n");
        s.push_str("  Acceptable signal-state: a run with zero accepted WorkTx +\n");
        s.push_str("  ChallengeTx yields an empty PriceIndex by FR-14.3 / halt-\n");
        s.push_str("  trigger #5 (zero-liquidity → price=None) extended to the\n");
        s.push_str("  zero-position case.\n");
        return s;
    }

    s.push_str("  Per-node entries (price as integer-rational n/d, never decimal):\n\n");
    s.push_str(&format!(
        "    {:<32}  {:>14}  {:>14}  {:>16}  {:>16}\n",
        "node_id", "long_micro", "short_micro", "price_yes(n/d)", "price_no(n/d)"
    ));
    s.push_str("    ");
    s.push_str(&"-".repeat(98));
    s.push('\n');

    for (node_id, entry) in price_index.iter() {
        let yes_str = match &entry.price_yes {
            Some(p) => format!("{}/{}", p.numerator, p.denominator),
            None => "None".to_string(),
        };
        let no_str = match &entry.price_no {
            Some(p) => format!("{}/{}", p.numerator, p.denominator),
            None => "None".to_string(),
        };
        s.push_str(&format!(
            "    {:<32}  {:>14}  {:>14}  {:>16}  {:>16}\n",
            trunc(&node_id.0, 32),
            entry.long_interest.micro_units(),
            entry.short_interest.micro_units(),
            yes_str,
            no_str,
        ));
    }

    s.push('\n');
    s.push_str("  Architect mandate (§5.1 ruling 2026-05-03) ✓:\n");
    s.push_str("    Price is signal, not truth. NodeMarketEntry is a derived view —\n");
    s.push_str("    NOT canonical state. NO trading. NO automatic liquidity. NO AMM.\n");
    s.push_str("    NO price-based settlement. NO Goodhart leak of private predicates.\n");
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
    s.push_str(
        "    NodeMarketEntry is TB-14 derived view; flat NodePositionsIndex is canonical.\n",
    );
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

// ────────────────────────────────────────────────────────────────────────────
// TB-N3 A5 — Run report renderer (architect ruling 2026-05-11 §3 §A..§G +
// SG-N3.12). Sections all sourced from ChainTape + CAS only — no external
// state. Pure materialized view per CLAUDE.md §17 Report Standard.
// ────────────────────────────────────────────────────────────────────────────

/// TB-N3 A5 — render the §A..§G run-report block for a TB-N3 batch run.
/// Walks L4 + CAS to surface tx_kind counts, top contested nodes, budget
/// burn, MarketDecisionTrace aggregate, signal-not-truth banner. Caller
/// (`audit_dashboard --run-report`) appends this to the legacy 16-section
/// dashboard render.
fn render_tb_n3_run_report(
    report: &DashboardReport,
    repo: &std::path::Path,
    cas_path: &std::path::Path,
) -> String {
    use std::collections::BTreeMap;
    use turingosv4::bottom_white::cas::store::CasStore;
    use turingosv4::bottom_white::ledger::transition_ledger::{canonical_decode, Git2LedgerWriter};
    // TB-G G2.2 lift: §F walker + renderer moved to
    // `turingosv4::runtime::market_decision_trace_summary`. The
    // MarketDecisionTrace / NoTradeReason / TraceOutcome imports are no
    // longer needed in this binary scope (helper consumes them).

    let mut out = String::new();
    out.push_str("\n=== TB-N3 RUN REPORT ===\n");
    out.push_str(&format!("run_id: {}\n", report.run_id));
    out.push_str(&format!("epoch: {}\n", report.epoch));

    // Walk the L4 ledger once to count tx-kinds and gather node-survive
    // pool / market-seed / router activity.
    let writer = match Git2LedgerWriter::open(repo) {
        Ok(w) => w,
        Err(e) => {
            out.push_str(&format!("\n[error] open L4 ledger: {e:?}\n"));
            return out;
        }
    };
    let l4_count = writer.len();
    let cas = match CasStore::open(cas_path) {
        Ok(c) => c,
        Err(e) => {
            out.push_str(&format!("\n[error] open CAS: {e}\n"));
            return out;
        }
    };

    #[derive(Default)]
    struct ModelFamilyActivity {
        attempt_count: u64,
        accepted_worktx: u64,
        l4e_rejection: u64,
        verify_count: u64,
        challenge_count: u64,
        invest_count: u64,
        pnl_micro: i64,
    }

    let genesis_model_family_by_agent: BTreeMap<String, String> =
        std::fs::read_to_string(repo.join("genesis_report.json"))
            .ok()
            .and_then(|json| {
                serde_json::from_str::<turingosv4::runtime::genesis_report::GenesisReport>(&json)
                    .ok()
            })
            .map(|genesis| {
                genesis
                    .agent_model_assignment
                    .into_iter()
                    .map(|a| (a.agent_id, a.model_family))
                    .collect()
            })
            .unwrap_or_default();
    let mut model_family_activity: BTreeMap<String, ModelFamilyActivity> = BTreeMap::new();

    let mut tx_kind_counts: BTreeMap<String, u64> = BTreeMap::new();
    let mut market_seed_total_micro: i64 = 0;
    let mut pools_created: u64 = 0;
    let mut task_outcome_market_count: u64 = 0;
    let mut router_count_yes: u64 = 0;
    let mut router_count_no: u64 = 0;
    let mut node_event_seeds: BTreeMap<String, i64> = BTreeMap::new();
    let mut scheduler_price_signals: Vec<turingosv4::runtime::real5_roles::PriceSignal> =
        Vec::new();
    let mut scheduler_pnl_signals: Vec<turingosv4::runtime::agent_scheduler::SchedulerPnlSignal> =
        Vec::new();

    for t in 1..=l4_count {
        let entry = match writer.read_at(t) {
            Ok(e) => e,
            Err(_) => continue,
        };
        let payload = match cas.get(&entry.tx_payload_cid) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let typed_tx: TypedTx = match canonical_decode(&payload) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let kind = format!("{:?}", typed_tx.tx_kind());
        *tx_kind_counts.entry(kind).or_insert(0) += 1;
        match &typed_tx {
            TypedTx::Work(work) => {
                if let Some(family) = genesis_model_family_by_agent.get(&work.agent_id.0).cloned() {
                    model_family_activity
                        .entry(family)
                        .or_default()
                        .accepted_worktx += 1;
                }
            }
            TypedTx::Verify(verify) => {
                if let Some(family) = genesis_model_family_by_agent
                    .get(&verify.verifier_agent.0)
                    .cloned()
                {
                    model_family_activity
                        .entry(family)
                        .or_default()
                        .verify_count += 1;
                }
            }
            TypedTx::Challenge(challenge) => {
                if let Some(family) = genesis_model_family_by_agent
                    .get(&challenge.challenger_agent.0)
                    .cloned()
                {
                    model_family_activity
                        .entry(family)
                        .or_default()
                        .challenge_count += 1;
                }
            }
            TypedTx::MarketSeed(seed) => {
                let inner = &seed.event_id.0 .0;
                if inner.starts_with("node_survive:") {
                    market_seed_total_micro += seed.collateral_amount.micro_units();
                    node_event_seeds
                        .entry(inner.clone())
                        .and_modify(|v| *v += seed.collateral_amount.micro_units())
                        .or_insert(seed.collateral_amount.micro_units());
                }
            }
            TypedTx::CpmmPool(pool) => {
                let inner = &pool.event_id.0 .0;
                let total = pool.seed_yes.units.saturating_add(pool.seed_no.units);
                let price = if total == 0 {
                    "None".to_string()
                } else {
                    format!("{}/{}", pool.seed_yes.units, total)
                };
                // REAL-6D: scheduler observation consumes all active CPMM
                // price signals, including REAL-6A TaskOutcomeMarket
                // (`task-*` / future `task_outcome:*`) and TB-N3 node markets.
                if inner.starts_with("node_survive:")
                    || inner.starts_with("task-")
                    || inner.starts_with("task_outcome:")
                {
                    scheduler_price_signals.push(turingosv4::runtime::real5_roles::PriceSignal {
                        event_id: inner.clone(),
                        price,
                        depth: i64::try_from(total).ok(),
                    });
                }
                if inner.starts_with("task-") || inner.starts_with("task_outcome:") {
                    task_outcome_market_count += 1;
                }
                if inner.starts_with("node_survive:") {
                    pools_created += 1;
                }
            }
            TypedTx::BuyWithCoinRouter(router) => {
                use turingosv4::state::typed_tx::BuyDirection;
                if let Some(family) = genesis_model_family_by_agent.get(&router.buyer.0).cloned() {
                    model_family_activity
                        .entry(family)
                        .or_default()
                        .invest_count += 1;
                }
                match router.direction {
                    BuyDirection::BuyYes => router_count_yes += 1,
                    BuyDirection::BuyNo => router_count_no += 1,
                }
            }
            _ => {}
        }
    }

    for cid in cas.list_cids_by_object_type(
        turingosv4::bottom_white::cas::schema::ObjectType::AttemptTelemetry,
    ) {
        let attempt =
            match turingosv4::runtime::attempt_telemetry::read_attempt_telemetry_shared_slot_from_cas(
                &cas, &cid,
            ) {
                Ok(Some(attempt)) => attempt,
                Ok(None) => continue,
                Err(e) => {
                    out.push_str(&format!(
                        "\n[error] AttemptTelemetry shared-slot decode failed for {cid}: {e}\n"
                    ));
                    return out;
                }
            };
        if attempt.model_family.is_some()
            || genesis_model_family_by_agent.contains_key(&attempt.agent_id.0)
        {
            let family = attempt
                .model_family
                .clone()
                .unwrap_or_else(|| genesis_model_family_by_agent[&attempt.agent_id.0].clone());
            model_family_activity
                .entry(family)
                .or_default()
                .attempt_count += 1;
        }
    }

    {
        use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
        let rejections_path = repo.join("rejections.jsonl");
        let l4e_writer = if rejections_path.exists() {
            RejectionEvidenceWriter::open_jsonl(rejections_path).ok()
        } else {
            Some(RejectionEvidenceWriter::new())
        };
        if let Some(l4e_writer) = l4e_writer {
            for record in l4e_writer.records() {
                if let Some(family) = genesis_model_family_by_agent
                    .get(&record.agent_id.0)
                    .cloned()
                {
                    model_family_activity
                        .entry(family)
                        .or_default()
                        .l4e_rejection += 1;
                }
            }
        }
    }

    // §A Citation tree (minimal first-cut: list accepted Work tx_ids
    // grouped by agent; full tree is a forward enhancement).
    out.push_str("\n## §A Citation tree (accepted WorkTx by agent)\n");
    let mut work_by_agent: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (agent, act) in &report.per_agent {
        if act.work_tx_accepted == 0 {
            continue;
        }
        work_by_agent
            .entry(agent.clone())
            .or_default()
            .push(format!("{} accepted WorkTx", act.work_tx_accepted));
    }
    if work_by_agent.is_empty() {
        out.push_str("  (no accepted WorkTx in run)\n");
    } else {
        for (agent, lines) in &work_by_agent {
            out.push_str(&format!("  - {}: {}\n", agent, lines.join(", ")));
        }
    }

    // §B Role activity (already in DashboardReport per_agent).
    out.push_str("\n## §B Role activity\n");
    for (agent, act) in &report.per_agent {
        out.push_str(&format!(
            "  - {}: work_accepted={} work_rejected={} verify_accepted={} verify_rejected={} challenge_accepted={} invest_accepted={}\n",
            agent,
            act.work_tx_accepted,
            act.work_tx_rejected,
            act.verify_tx_accepted,
            act.verify_tx_rejected,
            act.challenge_tx_accepted,
            act.invest_tx_accepted,
        ));
    }

    // §C Market tx counts (from L4 walk).
    out.push_str("\n## §C Market tx counts\n");
    for kind in [
        "TaskOpen",
        "EscrowLock",
        "Work",
        "Verify",
        "FinalizeReward",
        "EventResolve",
        "CompleteSetMint",
        "MarketSeed",
        "CpmmPool",
        "CpmmSwap",
        "BuyWithCoinRouter",
        "CompleteSetRedeem",
        "CompleteSetMerge",
        "Challenge",
        "ChallengeResolve",
        "TaskBankruptcy",
        "TaskExpire",
        "TerminalSummary",
    ] {
        let n = tx_kind_counts.get(kind).copied().unwrap_or(0);
        out.push_str(&format!("  {kind}: {n}\n"));
    }
    let accepted_work = report
        .per_agent
        .values()
        .map(|a| a.work_tx_accepted as u64)
        .sum::<u64>();
    out.push_str(&format!("  accepted_work_tx_total: {accepted_work}\n"));

    // §D Top contested nodes — from cpmm pool reserves (read live from
    // sequencer if present; absent here so we approximate from the
    // node_event_seeds map populated by the L4 walk).
    out.push_str("\n## §D Top contested nodes (by seed total μC)\n");
    let mut by_seed: Vec<(&String, &i64)> = node_event_seeds.iter().collect();
    by_seed.sort_by(|a, b| b.1.cmp(a.1));
    if by_seed.is_empty() {
        out.push_str("  (no node-survive pools seeded)\n");
    } else {
        for (event, seed) in by_seed.iter().take(10) {
            out.push_str(&format!("  - {}: {} μC\n", event, seed));
        }
    }

    // §E Budget burn report (architect §8.5).
    let mmb_genesis_micro: i64 = 5_000_000;
    let pools_skipped_budget: i64 = accepted_work as i64 - pools_created as i64;
    out.push_str("\n## §E Budget burn report\n");
    out.push_str(&format!("  pools_created: {}\n", pools_created));
    out.push_str(&format!(
        "  market_seed_total: {} μC\n",
        market_seed_total_micro
    ));
    out.push_str(&format!(
        "  treasury_budget_start: {} μC (MarketMakerBudget genesis)\n",
        mmb_genesis_micro
    ));
    out.push_str(&format!(
        "  treasury_budget_end: {} μC (= start - market_seed_total)\n",
        mmb_genesis_micro - market_seed_total_micro
    ));
    out.push_str(&format!(
        "  pools_skipped_budget: {}\n",
        pools_skipped_budget.max(0)
    ));
    out.push_str(&format!("  router_buy_yes: {}\n", router_count_yes));
    out.push_str(&format!("  router_buy_no: {}\n", router_count_no));

    // §F MarketDecisionTrace summary — TB-G G2.2 (charter §1 Module G2
    // atom G2.2; G-Phase directive §G2 SG-G2.3 "NoTradeReason appears in
    // dashboard"). The §F walker + renderer was lifted into
    // `turingosv4::runtime::market_decision_trace_summary` for library-
    // test access (G2.2 SG-G2.4 fixture renders per-variant counts).
    // The new helper adds:
    //   - `submitted_vs_traced_ratio: <s>/<t> = <pct>%` row.
    //   - `## §F.A NoTradeReason exhaustive breakdown` (13-row stable
    //     block iterating `NoTradeReason::ALL`, zeros included for
    //     forward grep stability).
    let summary =
        turingosv4::runtime::market_decision_trace_summary::MarketDecisionTraceSummary::compute_from_cas(&cas);
    out.push_str(&summary.render_section_f());

    // §F.X Peer-verify coverage (TB-G G2P.2; charter §1 Module G2P;
    // G-Phase directive §0.6 amendment G-2 "verify_peer=0 比 invest=0
    // 更危险"). Walker derives per-agent peer_verify_count + coverage
    // % + non_solver_verifications from canonical L4 + CAS. Closes
    // user 2026-05-12 病灶3 "0 verify" by quantifying coverage in the
    // post-batch dashboard so silent-zero outcomes surface explicit
    // mechanism-bottleneck explanations per architect §8.5 +
    // CROSS_PROBLEM_PERSISTENCE_REPORT §4 Q6.6.
    match turingosv4::runtime::peer_verify_coverage::compute_peer_verify_coverage_from_paths(
        repo, cas_path,
    ) {
        Ok(cov) => {
            out.push_str(&cov.render_section_f_x());
        }
        Err(e) => {
            out.push_str(&format!("\n## §F.X Peer-verify coverage\n  [error] {e}\n"));
        }
    }

    // §G PnL trajectory (TB-G G3.4; charter §1 Module G3 atom G3.4;
    // G-Phase directive §G3 SG-G3.5 "PnL is visible in dashboard as
    // materialized view"). Walker replays the full L4 chain to obtain
    // the final QState, then iterates the canonical preseed agent
    // registry and emits per-agent realized/unrealized PnL +
    // open-position count + solvency status. Silent-zero-forbidden
    // contract: if every row is flat (no PnL movement, no positions),
    // a MECHANISM BOTTLENECK explainer with ≥3 candidate causes is
    // auto-rendered (mirrors the §F.X / G2P.2 silent-zero contract).
    let mut pnl_delta_count: u64 = 0;
    match turingosv4::runtime::agent_pnl::compute_pnl_trajectory_from_paths(repo, cas_path) {
        Ok(traj) => {
            for row in &traj.rows {
                if row.realized_pnl != 0 || row.unrealized_pnl != 0 {
                    pnl_delta_count += 1;
                }
                if let Some(family) = genesis_model_family_by_agent.get(&row.agent_id.0).cloned() {
                    model_family_activity.entry(family).or_default().pnl_micro +=
                        row.realized_pnl + row.unrealized_pnl;
                }
                scheduler_pnl_signals.push(
                    turingosv4::runtime::agent_scheduler::SchedulerPnlSignal {
                        agent_id: row.agent_id.clone(),
                        realized_pnl: row.realized_pnl,
                        unrealized_pnl: row.unrealized_pnl,
                        available_micro: row.current_balance_micro,
                        risk_cap_micro: turingosv4::runtime::agent_pnl::bankruptcy_risk_cap_micro(
                            &row.agent_id,
                            &QState::default(),
                        ),
                    },
                );
            }
            out.push_str(&traj.render_section_g());
        }
        Err(e) => {
            out.push_str(&format!("\n## §G PnL trajectory\n  [error] {e}\n"));
        }
    }

    // §G.2 RiskCapImpactReport (TB-G G3.2; architect §7.1): derive
    // bankruptcy-risk-cap admission rejections from L4.E + CAS + replayed
    // QState and render the regression-attribution columns required by the
    // G3.2 §8 supplementary packet.
    match turingosv4::runtime::risk_cap_impact_report::compute_risk_cap_impact_report_from_paths(
        repo, cas_path,
    ) {
        Ok(report) => {
            let _: &turingosv4::runtime::risk_cap_impact_report::RiskCapImpactReport = &report;
            out.push_str(&report.render_section_g_2());
        }
        Err(e) => {
            out.push_str(&format!("\n## §G.2 RiskCapImpactReport\n  [error] {e}\n"));
        }
    }

    // §G.3 Model-family activity (G4.2). This is an attribution/divergence
    // materialized view from GenesisReport + AttemptTelemetry + ChainTape +
    // CAS. It is not a model leaderboard and makes no "model X is better"
    // claim.
    out.push_str("\n## §G.3 Model-family activity\n");
    out.push_str("  source: GenesisReport + AttemptTelemetry + ChainTape + CAS\n");
    out.push_str("  interpretation: activity/divergence only; no model ranking\n");
    match turingosv4::runtime::audit_assertions::audit_model_identity_from_paths(repo, cas_path) {
        Ok(identity_report) => {
            out.push_str(&format!(
                "  hidden_switch_verdict: {:?}\n",
                identity_report.verdict
            ));
            if !identity_report.hidden_switches.is_empty() {
                out.push_str("  hidden_switch_blocking_report:\n");
                for line in identity_report.render_blocking_report().lines() {
                    out.push_str(&format!("    {line}\n"));
                }
            }
        }
        Err(e) => out.push_str(&format!("  hidden_switch_verdict: ERROR ({e})\n")),
    }
    if model_family_activity.is_empty() {
        out.push_str("  (no model-family activity derived)\n");
    } else {
        for (family, act) in &model_family_activity {
            out.push_str(&format!(
                "  - {family}: attempt_count_by_model_family={} accepted_worktx_by_model_family={} l4e_rejection_by_model_family={} verify_count_by_model_family={} challenge_count_by_model_family={} invest_count_by_model_family={} pnl_by_model_family={}μC\n",
                act.attempt_count,
                act.accepted_worktx,
                act.l4e_rejection,
                act.verify_count,
                act.challenge_count,
                act.invest_count,
                act.pnl_micro
            ));
        }
    }

    // §I Role activity classifier (TB-G G5). Derived from public per-agent
    // activity counts only; no prompt body, completion, or CoT input.
    let role_rows: Vec<(
        String,
        turingosv4::runtime::agent_role_classifier::RoleActivity,
    )> = report
        .per_agent
        .iter()
        .map(|(agent, act)| {
            (
                agent.clone(),
                turingosv4::runtime::agent_role_classifier::RoleActivity {
                    work_tx_accepted: act.work_tx_accepted,
                    verify_tx_accepted: act.verify_tx_accepted,
                    challenge_tx_accepted: act.challenge_tx_accepted,
                    invest_tx_accepted: act.invest_tx_accepted,
                },
            )
        })
        .collect();
    out.push_str(
        &turingosv4::runtime::agent_role_classifier::render_role_activity_section(&role_rows),
    );

    // §J Epistemic pricing feedback (TB-G G6). Observe-only rows: these
    // correlate visible price/trace signals with action counts, but do not
    // become predicate authority or a model ranking surface.
    out.push_str("\n## §J Epistemic pricing feedback (observe-only)\n");
    out.push_str("  source: MarketDecisionTrace + ChainTape market activity\n");
    out.push_str("  interpretation: price is signal, not truth; no predicate authority\n");
    out.push_str(&format!(
        "  citation_vs_price: submitted_market_traces={} total_market_traces={}\n",
        summary.submitted_count, summary.total_traces
    ));
    let market_visible_actions = router_count_yes + router_count_no + pools_created;
    out.push_str(&format!(
        "  high_price_selection_rate: observed_market_visible_actions={} (integer count; benchmark protocol required before ranking claims)\n",
        market_visible_actions
    ));
    out.push_str(
        "  unresolved_challenged_filter: open Challenge targets are excluded from prompt market_context top-K\n",
    );
    let visible_agents: Vec<AgentId> = report
        .per_agent
        .keys()
        .map(|agent| AgentId(agent.clone()))
        .collect();
    let visible_nodes: Vec<TxId> = node_event_seeds
        .keys()
        .map(|event| TxId(event.clone()))
        .collect();
    let scheduler_head_t = format!(
        "HEAD_t(l4_head={},l4e_head={},cas_root={},state_root={},run_id={})",
        report
            .chain
            .head_commit_oid_hex
            .as_deref()
            .unwrap_or("(empty-l4)"),
        report.chain.l4e_last_hash_hex,
        hex_encode_bytes(&cas.merkle_root()),
        report
            .chain
            .final_state_root_hex
            .as_deref()
            .unwrap_or("(replay-unavailable)"),
        report.run_id
    );
    let persisted_scheduler_trace_cids =
        turingosv4::runtime::agent_scheduler::scheduler_decision_trace_cids(&cas);
    let recommended_agent = visible_agents.first().cloned();
    let recommended_role = if market_visible_actions > 0 || !scheduler_price_signals.is_empty() {
        Some(turingosv4::runtime::real5_roles::AgentRole::Trader)
    } else if accepted_work > 0 {
        Some(turingosv4::runtime::real5_roles::AgentRole::Verifier)
    } else {
        None
    };
    let recommended_action = recommended_role.map(|role| match role {
        turingosv4::runtime::real5_roles::AgentRole::Trader => "observe_market_signal".to_string(),
        turingosv4::runtime::real5_roles::AgentRole::Verifier => {
            "observe_peer_verify_queue".to_string()
        }
        _ => "observe_only".to_string(),
    });
    let scheduler_trace = turingosv4::runtime::agent_scheduler::build_observe_only_scheduler_trace(
        scheduler_head_t,
        visible_agents,
        visible_nodes,
        scheduler_price_signals,
        scheduler_pnl_signals,
        recommended_agent,
        recommended_role,
        recommended_action,
    );
    out.push_str(
        &turingosv4::runtime::agent_scheduler::render_scheduler_trace_section(&scheduler_trace),
    );
    out.push_str(&format!(
        "  persisted_scheduler_trace_cas_count: {}\n",
        persisted_scheduler_trace_cids.len()
    ));
    for cid in persisted_scheduler_trace_cids.iter().take(3) {
        out.push_str(&format!("    - scheduler_trace_cid={cid}\n"));
    }
    let persisted_scheduler_traces: Vec<_> = persisted_scheduler_trace_cids
        .iter()
        .filter_map(|cid| {
            turingosv4::runtime::agent_scheduler::read_scheduler_decision_trace_from_cas(&cas, cid)
                .ok()
        })
        .collect();

    // §K G7 structural smoke. This is a materialized dashboard view over
    // current run-report inputs; SG-G closeout still depends on the dedicated
    // G7 evidence dir or clean-negative forward-TB stub.
    let no_trade_reason_count = summary.outcome_counts.get("no_trade").copied().unwrap_or(0);
    let role_turn_summary =
        turingosv4::runtime::real5_roles::summarize_role_turn_traces_from_cas(&cas);
    let scripted_attempt_prediction_market_count = cas
        .list_all_cids()
        .into_iter()
        .filter(|cid| {
            cas.metadata(cid).and_then(|meta| meta.schema_id.as_deref())
                == Some(turingosv4::runtime::real6_attempt_prediction::REAL6B_SCHEMA_ID)
        })
        .count() as u64;
    let task_count = tx_kind_counts.get("TaskOpen").copied().unwrap_or(0);
    let verify_tx_count = tx_kind_counts.get("Verify").copied().unwrap_or(0);
    let challenge_tx_count = tx_kind_counts.get("Challenge").copied().unwrap_or(0);
    let event_resolve_count = tx_kind_counts.get("EventResolve").copied().unwrap_or(0);
    let task_bankruptcy_count = tx_kind_counts.get("TaskBankruptcy").copied().unwrap_or(0);
    let task_expire_count = tx_kind_counts.get("TaskExpire").copied().unwrap_or(0);
    let autopsy_capsule_count = cas
        .list_cids_by_object_type(
            turingosv4::bottom_white::cas::schema::ObjectType::AgentAutopsyCapsule,
        )
        .len() as u64;
    let loss_occurred = task_bankruptcy_count > 0 || task_expire_count > 0;
    let structural_market_visible_actions =
        router_count_yes + router_count_no + task_outcome_market_count + pools_created;
    let g7_guard_summary =
        turingosv4::runtime::g7_structural_smoke::summarize_g7_structural_guards_from_cas(&cas);
    let aggregate_assertions_pass = |names: &[&str]| -> bool {
        let Some(run_dir) = repo.parent() else {
            return false;
        };
        let Ok(raw) = std::fs::read_to_string(run_dir.join("aggregate_verdict.json")) else {
            return false;
        };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&raw) else {
            return false;
        };
        let Some(assertions) = value.get("assertions").and_then(|v| v.as_array()) else {
            return false;
        };
        names.iter().all(|name| {
            assertions.iter().any(|assertion| {
                assertion.get("name").and_then(|v| v.as_str()) == Some(*name)
                    && assertion.get("result").and_then(|v| v.as_str()) == Some("Pass")
            })
        })
    };
    let persisted_scheduler_observe_only = !persisted_scheduler_traces.is_empty()
        && persisted_scheduler_traces
            .iter()
            .all(|trace| trace.observe_only);
    let g7_price_observe_only =
        persisted_scheduler_observe_only && g7_guard_summary.price_observe_only_all;
    let g7_no_price_as_truth = g7_price_observe_only
        && g7_guard_summary.no_price_as_truth_all
        && aggregate_assertions_pass(&[
            "accepted_work_predicate_results_true",
            "tx_kind_envelope_matches_payload",
            "replay_state_root_matches_head",
        ]);
    let g7_no_ghost_liquidity = g7_guard_summary.no_ghost_liquidity_all
        && aggregate_assertions_pass(&[
            "no_post_init_mint",
            "total_supply_conserved",
            "total_supply_conserved_per_block",
            "complete_set_min_balanced",
            "conditional_shares_excluded_from_supply",
        ]);
    let no_forced_live_investment = g7_guard_summary.no_forced_live_investment_all
        && summary.submitted_count == 0
        && (router_count_yes + router_count_no) > 0
        && no_trade_reason_count > 0;
    let g7_clean_v3_comparison = g7_guard_summary.clean_v3_comparison_all;
    let g7_role_classifier_output =
        role_turn_summary.total_traces > 0 && role_turn_summary.by_role.len() >= 3;
    let g7_dashboard_regenerated = repo.exists() && l4_count > 0 && !cas.list_all_cids().is_empty();
    let g7_report = turingosv4::runtime::g7_structural_smoke::evaluate_g7_structural_smoke(
        turingosv4::runtime::g7_structural_smoke::G7SmokeInput {
            one_runtime_repo: repo.exists(),
            multi_agent: report.per_agent.len() > 1,
            persistent_state: report.run_facts.tx_count > 0,
            agent_count: report.per_agent.len() as u64,
            active_role_count: role_turn_summary.by_role.len() as u64,
            task_count,
            task_outcome_market_count,
            scripted_attempt_prediction_market_count,
            buy_yes_router_count: router_count_yes,
            buy_no_or_short_count: router_count_no + challenge_tx_count,
            verify_tx_count,
            challenge_tx_or_no_challenge_reason_count: challenge_tx_count
                + role_turn_summary.no_challenge_count,
            event_resolve_count,
            pnl_delta_count,
            loss_occurred,
            autopsy_capsule_count,
            forced_live_investment: !no_forced_live_investment,
            market_actions_chain_visible: structural_market_visible_actions > 0,
            no_ghost_liquidity: g7_no_ghost_liquidity,
            clean_v3_comparison: g7_clean_v3_comparison,
            proof_related_actions: accepted_work + verify_tx_count,
            market_visible_actions: structural_market_visible_actions,
            no_trade_reason_count,
            role_classifier_output: g7_role_classifier_output,
            price_observe_only: g7_price_observe_only,
            no_price_as_truth: g7_no_price_as_truth,
            dashboard_regenerated: g7_dashboard_regenerated,
        },
    );
    out.push_str(&g7_report.render_section_k());
    out.push_str(&format!(
        "  g7_guard_cas_count: {}\n",
        g7_guard_summary.total_guards
    ));
    out.push_str(&format!(
        "  aggregate_audit_guard_source: {}\n",
        repo.parent()
            .map(|p| p.join("aggregate_verdict.json").display().to_string())
            .unwrap_or_else(|| "(missing run dir)".into())
    ));

    // §H Banner (architect "no price as truth"). Renamed from §G to §H
    // by TB-G G3.4 to free the §G label for PnL trajectory; SG-14.6
    // architect-mandated banner contract still enforced via
    // `render_section_14` (separate section; unchanged).
    out.push_str("\n## §H PRICE IS SIGNAL, NOT TRUTH\n");
    out.push_str("  Pool reserves and prices in this report are derived\n");
    out.push_str("  views over ChainTape + CAS evidence. Prices are\n");
    out.push_str("  expressed as integer-rational (numerator/denominator).\n");
    out.push_str("  No Coin minted post-init; no ghost liquidity; no f64.\n");

    out
}

// ────────────────────────────────────────────────────────────────────────────

// ────────────────────────────────────────────────────────────────────────────
// TB-14 Atom 6 — SG-14.6 dashboard PriceIndex render tests
// (architect 2026-05-03 §5.1 + §5.5 SG-14.6).
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tb14_render_tests {
    use super::*;
    use turingosv4::state::RationalPrice;

    fn make_entry(
        node: &str,
        long: i64,
        short: i64,
        py: Option<(u128, u128)>,
        pn: Option<(u128, u128)>,
    ) -> (TxId, NodeMarketEntry) {
        (
            TxId(node.into()),
            NodeMarketEntry {
                node_id: TxId(node.into()),
                task_id: TaskId(format!("task-{node}")),
                event_id: turingosv4::state::typed_tx::EventId(TaskId(format!("task-{node}"))),
                long_interest: MicroCoin::from_micro_units(long),
                short_interest: MicroCoin::from_micro_units(short),
                yes_share_depth: turingosv4::state::typed_tx::ShareAmount::from_units(0),
                no_share_depth: turingosv4::state::typed_tx::ShareAmount::from_units(0),
                price_yes: py.map(|(n, d)| RationalPrice {
                    numerator: n,
                    denominator: d,
                }),
                price_no: pn.map(|(n, d)| RationalPrice {
                    numerator: n,
                    denominator: d,
                }),
                liquidity_depth: MicroCoin::from_micro_units(long + short),
            },
        )
    }

    /// SG-14.6 ARCHITECT-MANDATED: dashboard §14 carries the literal banner
    /// "PRICE IS SIGNAL, NOT TRUTH". This is the structural enforcement of
    /// architect §5.1 ("Price is signal, not truth.") at the read-view
    /// surface; future maintainers adding signal-as-truth language must
    /// fail this test.
    #[test]
    fn sg_14_6_dashboard_carries_price_is_signal_not_truth_banner() {
        let pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
        let s = render_section_14(&pi);
        assert!(
            s.contains("PRICE IS SIGNAL, NOT TRUTH"),
            "SG-14.6: §14 must contain the architect-mandated banner \
             `PRICE IS SIGNAL, NOT TRUTH`. Got render:\n{s}"
        );
    }

    /// SG-14.6 ARCHITECT-MANDATED: dashboard §14 NEVER renders prices as
    /// decimal fractions — only `numerator/denominator` integer-rational
    /// pairs. The renderer must not contain any `format!("{:.N}", ...)`
    /// invocation against a price value, and the rendered string must
    /// not contain a decimal point inside any per-row token.
    #[test]
    fn sg_14_6_dashboard_renders_price_as_integer_rational_never_decimal() {
        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
        let (k1, e1) = make_entry(
            "n_alpha",
            700_000,
            300_000,
            Some((700_000, 1_000_000)),
            Some((300_000, 1_000_000)),
        );
        pi.insert(k1, e1);
        let (k2, e2) = make_entry(
            "n_beta",
            500_000,
            500_000,
            Some((500_000, 1_000_000)),
            Some((500_000, 1_000_000)),
        );
        pi.insert(k2, e2);

        let s = render_section_14(&pi);
        // Spot-check rendering of a known rational pair.
        assert!(
            s.contains("700000/1000000"),
            "SG-14.6: per-node price_yes must render as `n/d` integer-rational. Got:\n{s}"
        );
        assert!(
            s.contains("500000/1000000"),
            "SG-14.6: per-node price_yes must render as `n/d` integer-rational. Got:\n{s}"
        );
        // Architect §5.6 forbidden: NO decimal float in TB-14 surface render.
        // Spot-check no `0.7` / `70.0%` / similar decimal strings appear in any
        // per-row context (banner text may contain commas; no decimals).
        for forbidden in &["0.7", "0.3", "0.5", "70.0%", "30.0%", "50.0%"] {
            assert!(
                !s.contains(forbidden),
                "SG-14.6: §14 render MUST NOT contain decimal price token `{forbidden}` \
                 (architect §5.6 forbidden: no f64 / no decimal). Got:\n{s}"
            );
        }
    }

    /// SG-14.6 + FR-14.3: when the price index is empty (no recorded
    /// positions), §14 renders an explicit empty-state message rather than
    /// falling back to a stale or fabricated number.
    #[test]
    fn sg_14_6_dashboard_empty_price_index_renders_explicit_empty_state() {
        let pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
        let s = render_section_14(&pi);
        assert!(
            s.contains("price index is empty"),
            "SG-14.6: empty PriceIndex must render an explicit empty-state \
             message, not fabricate a number. Got:\n{s}"
        );
    }

    /// SG-14.6 + FR-14.3: a node with `price_yes == None` (zero-liquidity)
    /// must render as `None`, never as `0/0`, `0.0`, or any synthesized
    /// fraction.
    #[test]
    fn sg_14_6_dashboard_renders_none_for_zero_liquidity_nodes() {
        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
        let (k, e) = make_entry("n_zero", 0, 0, None, None);
        pi.insert(k, e);
        let s = render_section_14(&pi);
        assert!(
            s.contains("None"),
            "SG-14.6: zero-liquidity node must render `None` (FR-14.3 / \
             halt-trigger #5). Got:\n{s}"
        );
    }

    // ───────────────────────────────────────────────────────────────────
    // TB-15 Atom 6 — §15 Autopsy + Markov render tests (SG-15.6)
    // ───────────────────────────────────────────────────────────────────

    /// SG-15.6 ARCHITECT-MANDATED: dashboard §15 must render the literal
    /// privacy banner `AUTOPSY IS PRIVATE` (architect §6.4 ruling
    /// 2026-05-02 + CR-15.1). This test pins the banner string.
    #[test]
    fn sg_15_6_dashboard_carries_autopsy_is_private_banner() {
        let s = render_section_15(&[], None);
        assert!(
            s.contains("AUTOPSY IS PRIVATE"),
            "SG-15.6: §15 must contain the architect-mandated banner \
             `AUTOPSY IS PRIVATE`. Got render:\n{s}"
        );
    }

    /// SG-15.6 + halt-trigger #5: dashboard §15 input signature carries
    /// only `(String, u32)` event counts and an Option<&str> Markov
    /// pointer hex — no raw private bytes possible. Render output
    /// surfaces counts + pointer only; never `private_detail_cid` payload.
    #[test]
    fn sg_15_6_dashboard_renders_event_counts_only_no_raw_bytes() {
        let counts = vec![
            ("event:tb15:event_a".to_string(), 2u32),
            ("event:tb15:event_b".to_string(), 5u32),
        ];
        let s = render_section_15(&counts, Some("abcd1234"));
        // Surfaces event_id + count + Markov pointer.
        assert!(
            s.contains("event:tb15:event_a"),
            "missing event_a; got:\n{s}"
        );
        assert!(
            s.contains("event:tb15:event_b"),
            "missing event_b; got:\n{s}"
        );
        assert!(
            s.contains(" 7 capsule"),
            "missing total cid count; got:\n{s}"
        );
        assert!(
            s.contains("abcd1234"),
            "missing markov pointer hex; got:\n{s}"
        );
        // Never embeds raw bytes (signature precludes; defense-in-depth: no
        // `0xPRIVATE` token would have been formattable from this input).
        assert!(!s.contains("private_detail_cid"));
    }

    /// SG-15.6 + FR-15.4: when no Markov capsule pointer is present, the
    /// dashboard tells the audit-reader how to generate one — does not
    /// silently omit the field.
    #[test]
    fn sg_15_6_dashboard_explains_when_no_markov_pointer() {
        let s = render_section_15(&[], None);
        assert!(
            s.contains("no latest Markov capsule pointer"),
            "SG-15.6: empty Markov pointer must render an explicit \
             generation hint, not silently omit. Got:\n{s}"
        );
        assert!(
            s.contains("generate_markov_capsule"),
            "SG-15.6: empty Markov pointer must hint the binary name"
        );
    }

    /// SG-15.6 + Markov default banner (CR-15.6 + halt-trigger #6): the
    /// dashboard explains that next-session boot defaults to constitution
    /// + latest capsule, with TURINGOS_MARKOV_OVERRIDE=1 required for
    /// deeper history.
    #[test]
    fn sg_15_6_dashboard_carries_markov_default_deny_explanation() {
        let s = render_section_15(&[], Some("deadbeef"));
        assert!(
            s.contains("TURINGOS_MARKOV_OVERRIDE=1"),
            "missing override env hint"
        );
        assert!(s.contains("deeper history"), "missing default-deny hint");
    }
}

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
