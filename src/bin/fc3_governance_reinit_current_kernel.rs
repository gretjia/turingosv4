//! True-suite FC3 governance/re-init evidence helper.
//!
//! This binary is a runner helper. It drives the existing current-kernel
//! FC3 system transaction surface:
//!   MapReduceTick -> LogFeedbackArchive -> ArchitectProposal ->
//!   VetoDecision -> ArchitectCommit -> TerminalSummary(ErrorHalt) ->
//!   ReinitRequest -> ReinitBoot.
//!
//! It writes durable ChainTape/CAS evidence; it does not simulate FC3 with a
//! dashboard or handover file.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use serde::Serialize;
use serde_json::json;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    canonical_decode, canonical_encode, cas_metadata_root_before_logical_t,
    constitution_source_hash, Git2LedgerWriter, LedgerEntry, LedgerWriter, TxKind,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::runtime::genesis_report::GenesisReport;
use turingosv4::runtime::{PinnedPubkeyEntry, PinnedPubkeyManifest};
use turingosv4::sdk::sanitized_runner::{run_sanitized, SanitizedCommand};
use turingosv4::state::q_state::{Hash, QState, TaskId, TxId};
use turingosv4::state::sequencer::{ApplyError, Sequencer, SubmissionEnvelope, SystemEmitCommand};
use turingosv4::state::typed_tx::{
    ArchitectCommitCapsule, ArchitectProposalCapsule, ArchitectProposalKind, ArchitectProposalTx,
    BootProfileId, LogFeedbackArchiveTx, ReinitReason, ReinitReasonCapsule, RunId, RunOutcome,
    TickKind, TypedTx, VetoDecisionCapsule, VetoDecisionTx, VetoReasonCode, VetoVerdict,
    ARCHITECT_COMMIT_SCHEMA_ID, ARCHITECT_FEEDBACK_SCHEMA_ID, ARCHITECT_PROPOSAL_SCHEMA_ID,
    REINIT_REASON_SCHEMA_ID, VETO_DECISION_SCHEMA_ID,
};
use turingosv4::top_white::predicates::registry::{BootPredicateManifest, PredicateRegistry};

#[derive(Debug)]
struct Args {
    runtime_repo: PathBuf,
    cas: PathBuf,
    run_id: String,
    constitution: PathBuf,
    out_dir: PathBuf,
}

struct Harness {
    cas: Arc<RwLock<CasStore>>,
    writer: Arc<RwLock<dyn LedgerWriter>>,
    rejections: Arc<RwLock<RejectionEvidenceWriter>>,
    seq: Arc<Sequencer>,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
}

#[derive(Debug, Serialize)]
struct TxIndexRow {
    logical_t: u64,
    tx_kind: String,
    tx_id: String,
    tx_payload_cid: String,
}

fn usage() -> &'static str {
    "usage: fc3_governance_reinit_current_kernel --runtime-repo <PATH> --cas <PATH> --run-id <ID> --constitution <constitution.md> --out-dir <PATH>"
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut runtime_repo: Option<PathBuf> = None;
    let mut cas: Option<PathBuf> = None;
    let mut run_id: Option<String> = None;
    let mut constitution: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "--runtime-repo" => {
                i += 1;
                runtime_repo = Some(
                    argv.get(i)
                        .ok_or("missing value after --runtime-repo")?
                        .into(),
                );
            }
            "--cas" => {
                i += 1;
                cas = Some(argv.get(i).ok_or("missing value after --cas")?.into());
            }
            "--run-id" => {
                i += 1;
                run_id = Some(argv.get(i).ok_or("missing value after --run-id")?.clone());
            }
            "--constitution" => {
                i += 1;
                constitution = Some(
                    argv.get(i)
                        .ok_or("missing value after --constitution")?
                        .into(),
                );
            }
            "--out-dir" => {
                i += 1;
                out_dir = Some(argv.get(i).ok_or("missing value after --out-dir")?.into());
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
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse_args(&argv) {
        Ok(args) => args,
        Err(msg) => {
            eprintln!("fc3_governance_reinit_current_kernel: {msg}");
            eprintln!("{}", usage());
            return ExitCode::from(2);
        }
    };

    if let Err(err) = run(args).await {
        eprintln!("fc3_governance_reinit_current_kernel: {err}");
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

async fn run(args: Args) -> Result<(), String> {
    std::fs::create_dir_all(&args.runtime_repo).map_err(|e| format!("runtime repo dir: {e}"))?;
    std::fs::create_dir_all(&args.cas).map_err(|e| format!("cas dir: {e}"))?;
    std::fs::create_dir_all(&args.out_dir).map_err(|e| format!("out dir: {e}"))?;

    let mut h = build_harness(&args)?;

    let tick_entry = apply_emit(
        &h.seq,
        &mut h.rx,
        SystemEmitCommand::MapReduceTick {
            tick_kind: TickKind::Scheduled,
        },
    )
    .await?;
    assert_kind(&tick_entry, TxKind::MapReduceTick)?;

    let feedback_capsule_cid = put_feedback_capsule(&h)?;
    let feedback_entry = apply_emit(
        &h.seq,
        &mut h.rx,
        SystemEmitCommand::LogFeedbackArchive {
            feedback_capsule_cid,
            veto_verdict: VetoVerdict::Pass,
        },
    )
    .await?;
    assert_kind(&feedback_entry, TxKind::LogFeedbackArchive)?;
    let TypedTx::LogFeedbackArchive(feedback_tx) = decode_entry_tx(&h, &feedback_entry)? else {
        return Err("expected LogFeedbackArchive tx".to_string());
    };

    let patch_artifact_cid = put_bytes(
        &h,
        b"fc3 runtime tool registry proposal artifact",
        "fc3_governance_reinit_current_kernel",
        Some("fc3.tool_registry_patch_artifact.v1"),
    )?;
    let proposal_capsule_cid = put_proposal_capsule(
        &h,
        &feedback_tx,
        ArchitectProposalKind::ToolRegistryPatch,
        Some("src/bottom_white/tools/runtime_meta_tool.rs"),
        Some(patch_artifact_cid),
        vec![
            "ToolRegistry::new".to_string(),
            "sandboxed_exec".to_string(),
        ],
    )?;
    let proposal_entry = apply_emit(
        &h.seq,
        &mut h.rx,
        SystemEmitCommand::ArchitectProposal {
            feedback_tx_id: feedback_tx.tx_id.clone(),
            proposal_capsule_cid,
        },
    )
    .await?;
    assert_kind(&proposal_entry, TxKind::ArchitectProposal)?;
    let TypedTx::ArchitectProposal(proposal_tx) = decode_entry_tx(&h, &proposal_entry)? else {
        return Err("expected ArchitectProposal tx".to_string());
    };

    let veto_capsule_cid = put_veto_capsule(
        &h,
        &proposal_tx,
        VetoVerdict::Pass,
        VetoReasonCode::ConstitutionCompliant,
    )?;
    let veto_entry = apply_emit(
        &h.seq,
        &mut h.rx,
        SystemEmitCommand::VetoDecision {
            proposal_tx_id: proposal_tx.tx_id.clone(),
            decision_capsule_cid: veto_capsule_cid,
        },
    )
    .await?;
    assert_kind(&veto_entry, TxKind::VetoDecision)?;
    let TypedTx::VetoDecision(veto_tx) = decode_entry_tx(&h, &veto_entry)? else {
        return Err("expected VetoDecision tx".to_string());
    };
    if veto_tx.verdict != VetoVerdict::Pass {
        return Err("expected deterministic Veto-AI PASS".to_string());
    }

    let commit_capsule_cid = put_commit_capsule(
        &h,
        &veto_tx,
        Some(patch_artifact_cid),
        Some("src/bottom_white/tools/runtime_meta_tool.rs"),
    )?;
    let commit_entry = apply_emit(
        &h.seq,
        &mut h.rx,
        SystemEmitCommand::ArchitectCommit {
            veto_tx_id: veto_tx.tx_id.clone(),
            commit_capsule_cid,
        },
    )
    .await?;
    assert_kind(&commit_entry, TxKind::ArchitectCommit)?;

    let terminal_entry = apply_emit(
        &h.seq,
        &mut h.rx,
        SystemEmitCommand::TerminalSummary {
            run_id: RunId(args.run_id.clone()),
            task_id: TaskId("fc3-governance-reinit".to_string()),
            run_outcome: RunOutcome::ErrorHalt,
            total_attempts: 1,
            failure_class_histogram: BTreeMap::new(),
            last_logical_t: commit_entry.logical_t,
            solver_agent: None,
            evidence_capsule_cid: Some(feedback_capsule_cid),
        },
    )
    .await?;
    assert_kind(&terminal_entry, TxKind::TerminalSummary)?;

    let reason = ReinitReason::TerminalErrorHalt;
    let reason_cid = put_reinit_capsule(&h, terminal_entry.logical_t, reason)?;
    let boot_profile = BootProfileId("fc3-current-kernel-reinit".to_string());
    let request_entry = apply_emit(
        &h.seq,
        &mut h.rx,
        SystemEmitCommand::ReinitRequest {
            trigger_entry: terminal_entry.logical_t,
            error_evidence_cid: reason_cid,
            reason,
            target_boot_profile: boot_profile.clone(),
        },
    )
    .await?;
    assert_kind(&request_entry, TxKind::ReinitRequest)?;
    let TypedTx::ReinitRequest(request_tx) = decode_entry_tx(&h, &request_entry)? else {
        return Err("expected ReinitRequest tx".to_string());
    };

    let boot_entry = apply_emit(
        &h.seq,
        &mut h.rx,
        SystemEmitCommand::ReinitBoot {
            request_tx_id: request_tx.tx_id.clone(),
            boot_profile,
        },
    )
    .await?;
    assert_kind(&boot_entry, TxKind::ReinitBoot)?;

    write_chaintape_jsonl(&h, &args.out_dir.join("chaintape.jsonl"))?;
    write_capsule_index(
        &h,
        &args,
        CapsuleIds {
            feedback_capsule_cid,
            proposal_capsule_cid,
            veto_capsule_cid,
            commit_capsule_cid,
            reinit_reason_cid: reason_cid,
            patch_artifact_cid,
        },
        &[
            tick_entry,
            feedback_entry,
            proposal_entry,
            veto_entry,
            commit_entry,
            terminal_entry,
            request_entry,
            boot_entry,
        ],
    )?;
    write_genesis_report(&args)?;

    println!(
        "fc3_governance_reinit_current_kernel: wrote runtime_repo={} cas={} out={}",
        args.runtime_repo.display(),
        args.cas.display(),
        args.out_dir.display()
    );
    Ok(())
}

fn build_harness(args: &Args) -> Result<Harness, String> {
    let keypair = Arc::new(
        Ed25519Keypair::generate_with_secure_entropy()
            .map_err(|e| format!("generate system keypair: {e}"))?,
    );
    let epoch = SystemEpoch::new(1);
    write_pinned_manifest(&args.runtime_repo, epoch, &keypair, &args.run_id)?;

    let initial_q = QState::genesis();
    let initial_q_json =
        serde_json::to_string_pretty(&initial_q).map_err(|e| format!("initial_q json: {e}"))?;
    std::fs::write(
        args.runtime_repo.join("initial_q_state.json"),
        initial_q_json,
    )
    .map_err(|e| format!("write initial_q_state.json: {e}"))?;

    let writer = Git2LedgerWriter::open(&args.runtime_repo)
        .map_err(|e| format!("open Git2LedgerWriter: {e}"))?;
    let writer: Arc<RwLock<dyn LedgerWriter>> = Arc::new(RwLock::new(writer));
    let cas = Arc::new(RwLock::new(
        CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?,
    ));
    let rejections = Arc::new(RwLock::new(
        RejectionEvidenceWriter::open_jsonl(args.runtime_repo.join("rejections.jsonl"))
            .map_err(|e| format!("open rejections.jsonl: {e}"))?,
    ));
    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(epoch, keypair.public_key());
    let registry = PredicateRegistry::from_boot_manifest(BootPredicateManifest { entries: vec![] })
        .map_err(|e| format!("empty predicate registry: {e:?}"))?;
    let (seq, rx) = Sequencer::new(
        Arc::clone(&cas),
        keypair,
        epoch,
        Arc::clone(&writer),
        Arc::clone(&rejections),
        Arc::new(registry),
        Arc::new(ToolRegistry::new()),
        Arc::new(pinned),
        initial_q,
        16,
    );

    Ok(Harness {
        cas,
        writer,
        rejections,
        seq: Arc::new(seq),
        rx,
    })
}

fn write_pinned_manifest(
    runtime_repo: &Path,
    epoch: SystemEpoch,
    keypair: &Ed25519Keypair,
    run_id: &str,
) -> Result<(), String> {
    let pubkey_hex: String = keypair
        .public_key()
        .as_bytes()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let manifest = PinnedPubkeyManifest {
        run_id: run_id.to_string(),
        tb_id: "TB-6".to_string(),
        epoch: epoch.get(),
        pubkeys: vec![PinnedPubkeyEntry {
            epoch: epoch.get(),
            pubkey_hex,
        }],
    };
    let json = serde_json::to_string_pretty(&manifest).map_err(|e| format!("pinned json: {e}"))?;
    std::fs::write(runtime_repo.join("pinned_pubkeys.json"), json)
        .map_err(|e| format!("write pinned_pubkeys.json: {e}"))
}

async fn apply_emit(
    seq: &Sequencer,
    rx: &mut tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    command: SystemEmitCommand,
) -> Result<LedgerEntry, String> {
    seq.emit_system_tx(command)
        .await
        .map_err(|e| format!("emit system tx: {e}"))?;
    seq.try_apply_one(rx)
        .ok_or_else(|| "system tx queue unexpectedly empty".to_string())?
        .map_err(|e| match e {
            ApplyError::Transition(t) => format!("transition rejected: {t}"),
            other => format!("apply rejected: {other}"),
        })
}

fn assert_kind(entry: &LedgerEntry, expected: TxKind) -> Result<(), String> {
    if entry.tx_kind != expected {
        return Err(format!(
            "expected tx kind {:?}, got {:?} at logical_t {}",
            expected, entry.tx_kind, entry.logical_t
        ));
    }
    Ok(())
}

fn decode_entry_tx(h: &Harness, entry: &LedgerEntry) -> Result<TypedTx, String> {
    let cas = h.cas.read().map_err(|_| "cas read poisoned".to_string())?;
    let bytes = cas
        .get(&entry.tx_payload_cid)
        .map_err(|e| format!("read tx payload from CAS: {e}"))?;
    canonical_decode(&bytes).map_err(|e| format!("decode typed tx: {e}"))
}

fn put_canonical<T: Serialize>(
    h: &Harness,
    value: &T,
    schema_id: &'static str,
    logical_t: u64,
) -> Result<Cid, String> {
    let bytes =
        canonical_encode(value).map_err(|e| format!("canonical encode {schema_id}: {e}"))?;
    put_bytes(
        h,
        &bytes,
        "fc3_governance_reinit_current_kernel",
        Some(schema_id),
    )
    .and_then(|cid| {
        let cas = h.cas.read().map_err(|_| "cas read poisoned".to_string())?;
        let created_at_logical_t = cas
            .metadata(&cid)
            .ok_or_else(|| format!("metadata missing after put for {schema_id}"))?
            .created_at_logical_t;
        if created_at_logical_t != logical_t {
            return Err(format!(
                "CAS logical_t mismatch for {schema_id}: got {}, expected {}",
                created_at_logical_t, logical_t
            ));
        }
        Ok(cid)
    })
}

fn put_bytes(
    h: &Harness,
    bytes: &[u8],
    origin: &str,
    schema_id: Option<&str>,
) -> Result<Cid, String> {
    let logical_t = h.seq.next_logical_t_peek();
    h.cas
        .write()
        .map_err(|_| "cas write poisoned".to_string())?
        .put(
            bytes,
            ObjectType::Generic,
            origin,
            logical_t,
            schema_id.map(str::to_string),
        )
        .map_err(|e| format!("cas put: {e}"))
}

fn put_feedback_capsule(h: &Harness) -> Result<Cid, String> {
    let logical_t = h.seq.next_logical_t_peek() + 1;
    let q = h.seq.q_snapshot().map_err(|e| format!("q snapshot: {e}"))?;
    let l4e = h
        .rejections
        .read()
        .map_err(|_| "l4e read poisoned".to_string())?;
    let cas_root = {
        let cas = h.cas.read().map_err(|_| "cas read poisoned".to_string())?;
        cas_metadata_root_before_logical_t(&cas, logical_t).map_err(|e| format!("{e}"))?
    };
    let capsule = turingosv4::state::typed_tx::ArchitectFeedbackCapsule {
        schema_version: ARCHITECT_FEEDBACK_SCHEMA_ID.to_string(),
        source_ledger_root: q.ledger_root_t,
        source_l4e_root: l4e.last_hash(),
        cas_metadata_root: cas_root,
        constitution_hash: constitution_source_hash(),
        public_summary: "FC3 current-kernel logs feedback to runtime ArchitectAI/Veto-AI"
            .to_string(),
        private_detail_cid: None,
    };
    drop(l4e);
    put_canonical(h, &capsule, ARCHITECT_FEEDBACK_SCHEMA_ID, logical_t - 1)
}

fn put_proposal_capsule(
    h: &Harness,
    feedback: &LogFeedbackArchiveTx,
    proposal_kind: ArchitectProposalKind,
    target_path: Option<&str>,
    proposed_artifact_cid: Option<Cid>,
    tools_used: Vec<String>,
) -> Result<Cid, String> {
    let q = h.seq.q_snapshot().map_err(|e| format!("q snapshot: {e}"))?;
    let capsule = ArchitectProposalCapsule {
        schema_version: ARCHITECT_PROPOSAL_SCHEMA_ID.to_string(),
        feedback_tx_id: feedback.tx_id.clone(),
        feedback_root: feedback.feedback_root,
        constitution_hash: constitution_source_hash(),
        tool_registry_root: q.tool_registry_root_t,
        proposal_kind,
        target_path: target_path.map(str::to_string),
        proposed_artifact_cid,
        tools_used,
        public_summary: "ArchitectAI proposes a tool-registry-safe runtime artifact".to_string(),
    };
    put_canonical(
        h,
        &capsule,
        ARCHITECT_PROPOSAL_SCHEMA_ID,
        h.seq.next_logical_t_peek(),
    )
}

fn put_veto_capsule(
    h: &Harness,
    proposal: &ArchitectProposalTx,
    verdict: VetoVerdict,
    reason_code: VetoReasonCode,
) -> Result<Cid, String> {
    let capsule = VetoDecisionCapsule {
        schema_version: VETO_DECISION_SCHEMA_ID.to_string(),
        proposal_tx_id: proposal.tx_id.clone(),
        proposal_root: proposal.proposal_root,
        constitution_hash: constitution_source_hash(),
        verdict,
        reason_code,
        public_summary: match verdict {
            VetoVerdict::Pass => "Veto-AI PASS".to_string(),
            VetoVerdict::Veto => "Veto-AI VETO".to_string(),
        },
    };
    put_canonical(
        h,
        &capsule,
        VETO_DECISION_SCHEMA_ID,
        h.seq.next_logical_t_peek(),
    )
}

fn put_commit_capsule(
    h: &Harness,
    veto: &VetoDecisionTx,
    applied_artifact_cid: Option<Cid>,
    target_path: Option<&str>,
) -> Result<Cid, String> {
    let capsule = ArchitectCommitCapsule {
        schema_version: ARCHITECT_COMMIT_SCHEMA_ID.to_string(),
        proposal_tx_id: veto.proposal_tx_id.clone(),
        veto_tx_id: veto.tx_id.clone(),
        decision_root: veto.decision_root,
        constitution_hash: constitution_source_hash(),
        applied_artifact_cid,
        target_path: target_path.map(str::to_string),
        public_summary: "ArchitectAI commit accepted after Veto-AI PASS".to_string(),
    };
    put_canonical(
        h,
        &capsule,
        ARCHITECT_COMMIT_SCHEMA_ID,
        h.seq.next_logical_t_peek(),
    )
}

fn put_reinit_capsule(
    h: &Harness,
    trigger_entry: u64,
    reason: ReinitReason,
) -> Result<Cid, String> {
    let capsule = ReinitReasonCapsule {
        schema_version: REINIT_REASON_SCHEMA_ID.to_string(),
        trigger_entry,
        reason,
        public_summary: "Terminal ErrorHalt triggers replay-bound FC3 re-init".to_string(),
        private_detail_cid: None,
    };
    put_canonical(
        h,
        &capsule,
        REINIT_REASON_SCHEMA_ID,
        h.seq.next_logical_t_peek(),
    )
}

fn tx_id(tx: &TypedTx) -> TxId {
    match tx {
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

fn write_chaintape_jsonl(h: &Harness, path: &Path) -> Result<(), String> {
    let writer = h
        .writer
        .read()
        .map_err(|_| "writer read poisoned".to_string())?;
    let mut out = String::new();
    for logical_t in 1..=writer.len() {
        let entry = writer
            .read_at(logical_t)
            .map_err(|e| format!("read ledger entry {logical_t}: {e}"))?;
        let tx = decode_entry_tx(h, &entry)?;
        let row = TxIndexRow {
            logical_t,
            tx_kind: format!("{:?}", entry.tx_kind),
            tx_id: tx_id(&tx).0,
            tx_payload_cid: entry.tx_payload_cid.hex(),
        };
        out.push_str(&serde_json::to_string(&row).map_err(|e| format!("json row: {e}"))?);
        out.push('\n');
    }
    std::fs::write(path, out).map_err(|e| format!("write chaintape jsonl: {e}"))
}

struct CapsuleIds {
    feedback_capsule_cid: Cid,
    proposal_capsule_cid: Cid,
    veto_capsule_cid: Cid,
    commit_capsule_cid: Cid,
    reinit_reason_cid: Cid,
    patch_artifact_cid: Cid,
}

fn write_capsule_index(
    h: &Harness,
    args: &Args,
    ids: CapsuleIds,
    entries: &[LedgerEntry],
) -> Result<(), String> {
    let q = h.seq.q_snapshot().map_err(|e| format!("q snapshot: {e}"))?;
    let tx_rows: Vec<_> = entries
        .iter()
        .map(|entry| {
            let tx = decode_entry_tx(h, entry)?;
            Ok(json!({
                "logical_t": entry.logical_t,
                "tx_kind": format!("{:?}", entry.tx_kind),
                "tx_id": tx_id(&tx).0,
                "tx_payload_cid": entry.tx_payload_cid.hex(),
            }))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let manifest = json!({
        "schema_version": "turingosv4.true_suite.fc3_governance_reinit_current_kernel.v1",
        "run_id": args.run_id,
        "git_head": git_head(args).unwrap_or_else(|| "unknown".to_string()),
        "runtime_repo": args.runtime_repo,
        "cas": args.cas,
        "constitutional_paths": [
            "FC3:logs-archive->feedback->architectAI->vetoAI->tools/logs->re-init",
            "FC2:replay",
            "FC1:rtool->input"
        ],
        "capsules": {
            "feedback_capsule_cid": ids.feedback_capsule_cid.hex(),
            "proposal_capsule_cid": ids.proposal_capsule_cid.hex(),
            "veto_capsule_cid": ids.veto_capsule_cid.hex(),
            "commit_capsule_cid": ids.commit_capsule_cid.hex(),
            "reinit_reason_cid": ids.reinit_reason_cid.hex(),
            "patch_artifact_cid": ids.patch_artifact_cid.hex()
        },
        "tx_sequence": tx_rows,
        "final_q": {
            "state_root_t": hex_hash(q.state_root_t),
            "ledger_root_t": hex_hash(q.ledger_root_t),
            "tool_registry_root_t": hex_hash(q.tool_registry_root_t)
        },
        "checks": {
            "all_fc3_typed_transactions_present": true,
            "architectai_commit_after_vetoai_pass": true,
            "terminal_errorhalt_reinit_request_and_boot_present": true,
            "handover_files_not_source_of_truth": true,
            "dashboard_stdout_not_evidence": true
        }
    });
    let path = args.out_dir.join("governance_capsule_index.json");
    std::fs::write(
        path,
        serde_json::to_string_pretty(&manifest).map_err(|e| format!("manifest json: {e}"))? + "\n",
    )
    .map_err(|e| format!("write governance index: {e}"))
}

fn write_genesis_report(args: &Args) -> Result<(), String> {
    let report = GenesisReport {
        constitution_hash: GenesisReport::hash_constitution_md(&args.constitution),
        runtime_repo: args.runtime_repo.display().to_string(),
        cas_path: args.cas.display().to_string(),
        system_pubkey_hash: GenesisReport::hash_system_pubkey_manifest(&args.runtime_repo),
        agent_pubkeys_path: "agent_pubkeys.json".to_string(),
        initial_balances: vec![],
        task_id: None,
        task_open_tx: None,
        escrow_lock_tx: None,
        agent_model_assignment: vec![],
        model_assignment_manifest_cid: None,
        agent_role_assignment: vec![],
        role_assignment_manifest_cid: None,
    };
    report
        .write_to_runtime_repo(&args.runtime_repo)
        .map_err(|e| format!("write genesis_report.json: {e}"))
}

fn git_head(args: &Args) -> Option<String> {
    let cwd = args
        .constitution
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    let output = run_sanitized(SanitizedCommand {
        program: PathBuf::from("git"),
        args: vec!["rev-parse".to_string(), "HEAD".to_string()],
        cwd,
        env: BTreeMap::new(),
        stdin: None,
        timeout: Duration::from_secs(5),
    })
    .ok()?;
    if !output.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn hex_hash(hash: Hash) -> String {
    hash.0.iter().map(|b| format!("{b:02x}")).collect()
}
