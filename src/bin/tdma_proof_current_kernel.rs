//! True-suite TDMA/proof ChainTape bridge.
//!
//! `turingos tdma run` writes a dedicated proof-work GitTapeLedger. This helper
//! does not replace that tape. It reads the TDMA evidence, stores a compact
//! CAS-backed proof summary, and submits a signed WorkTx into the canonical
//! ChainTape so the same run can be checked by the full-system participation
//! gate.

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::adapter::{
    genesis_with_balances, make_real_escrow_lock_signed_by, make_real_task_open_signed_by,
    make_real_worktx_signed_by, tb8_await_state_root_advance,
};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::bootstrap::default_pput_preseed_pairs;
use turingosv4::runtime::genesis_report::GenesisReport;
use turingosv4::runtime::proposal_telemetry::{
    write_to_cas as write_proposal_telemetry_to_cas, ProposalTelemetry, TokenCounts,
};
use turingosv4::runtime::{build_chaintape_sequencer_with_initial_q, RuntimeChaintapeConfig};
use turingosv4::sdk::sanitized_runner::{
    env_allowlist_from_current, run_sanitized, SanitizedCommand,
};
use turingosv4::state::q_state::{AgentId, Hash};
use turingosv4::state::typed_tx::TypedTx;

const SPONSOR_AGENT: &str = "Agent_user_0";
const SOLVER_AGENT: &str = "Agent_0";
const TASK_ESCROW_MICRO: i64 = 10_000;
const WORK_STAKE_MICRO: i64 = 100;

#[derive(Debug)]
struct Args {
    runtime_repo: PathBuf,
    cas: PathBuf,
    run_id: String,
    constitution: PathBuf,
    tdma_evidence_dir: PathBuf,
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
struct TdmaManifest {
    problem_label: String,
    model_label: String,
    stages_total: u64,
    stages_completed: u64,
    total_attempts: u64,
    all_prompts_within_budget: bool,
    leak_in_any_prompt: bool,
    chaintape_sha256: String,
    probes_sha256: String,
}

#[derive(Debug, Clone, Serialize)]
struct TdmaProofCapsule {
    schema_version: &'static str,
    run_id: String,
    problem_label: String,
    model_label: String,
    stages_total: u64,
    stages_completed: u64,
    total_attempts: u64,
    all_prompts_within_budget: bool,
    leak_in_any_prompt: bool,
    chaintape_sha256: String,
    probes_sha256: String,
    manifest_sha256: String,
    tdma_git_tape_present: bool,
    tdma_git_tape_verified_head_present: bool,
}

#[derive(Debug, Clone, Serialize)]
struct TdmaEvaluationCapsule {
    schema_version: &'static str,
    run_id: String,
    proof_capsule_cid: String,
    stages_completed_all: bool,
    hash_checks_passed: bool,
    budget_guard_passed: bool,
    raw_stderr_leak_guard_passed: bool,
    benchmark_verdict: String,
    failure_class: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct TdmaEvidenceManifest {
    schema_version: &'static str,
    run_id: String,
    problem_label: String,
    model_label: String,
    tdma_evidence_dir: String,
    proof_capsule_cid: String,
    evaluation_capsule_cid: String,
    proposal_telemetry_cid: String,
    work_tx_id: String,
    work_tx_landed: bool,
    stages_completed_all: bool,
    benchmark_verdict: String,
    closure_scope: &'static str,
    full_system_participation_required: bool,
    final_closure_possible: bool,
    failure_taxonomy_path: String,
    final_state_root_hex: String,
    runtime_repo: String,
    cas: String,
    notes: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
struct FailureTaxonomy {
    schema_version: &'static str,
    run_id: String,
    family_id: &'static str,
    failure_class: Option<String>,
    stages_completed_all: bool,
    constitutional_rejection: bool,
    kernel_invariant_failure: bool,
    model_task_failure: bool,
    infrastructure_failure: bool,
}

fn usage() -> &'static str {
    "usage: tdma_proof_current_kernel --runtime-repo <PATH> --cas <PATH> \
     --run-id <ID> --constitution <constitution.md> \
     --tdma-evidence-dir <PATH> --out-dir <PATH>"
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut runtime_repo = None;
    let mut cas = None;
    let mut run_id = None;
    let mut constitution = None;
    let mut tdma_evidence_dir = None;
    let mut out_dir = None;
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
            "--tdma-evidence-dir" => {
                i += 1;
                tdma_evidence_dir = Some(
                    argv.get(i)
                        .ok_or("--tdma-evidence-dir requires value")?
                        .into(),
                );
            }
            "--out-dir" => {
                i += 1;
                out_dir = Some(argv.get(i).ok_or("--out-dir requires value")?.into());
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
        tdma_evidence_dir: tdma_evidence_dir.ok_or("--tdma-evidence-dir required")?,
        out_dir: out_dir.ok_or("--out-dir required")?,
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse_args(&argv) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("tdma_proof_current_kernel: {err}");
            eprintln!("{}", usage());
            return ExitCode::from(2);
        }
    };
    match run(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("tdma_proof_current_kernel: {err}");
            ExitCode::from(1)
        }
    }
}

async fn run(args: Args) -> Result<(), String> {
    std::fs::create_dir_all(&args.runtime_repo).map_err(|e| format!("runtime repo dir: {e}"))?;
    std::fs::create_dir_all(&args.cas).map_err(|e| format!("cas dir: {e}"))?;
    std::fs::create_dir_all(&args.out_dir).map_err(|e| format!("out dir: {e}"))?;

    let manifest_path = args.tdma_evidence_dir.join("manifest.json");
    let manifest_raw = std::fs::read(&manifest_path)
        .map_err(|e| format!("read {}: {e}", manifest_path.display()))?;
    let manifest: TdmaManifest =
        serde_json::from_slice(&manifest_raw).map_err(|e| format!("parse TDMA manifest: {e}"))?;
    let chaintape_path = args.tdma_evidence_dir.join("chaintape.jsonl");
    let probes_path = args.tdma_evidence_dir.join("per_attempt_probes.jsonl");
    let tdma_tape = args.tdma_evidence_dir.join("tdma_tape.git");
    require_file(&chaintape_path)?;
    require_file(&probes_path)?;

    let chaintape_sha256 = sha256_file(&chaintape_path)?;
    let probes_sha256 = sha256_file(&probes_path)?;
    let hash_checks_passed =
        chaintape_sha256 == manifest.chaintape_sha256 && probes_sha256 == manifest.probes_sha256;
    let stages_completed_all = manifest.stages_completed == manifest.stages_total;
    let budget_guard_passed = manifest.all_prompts_within_budget;
    let raw_stderr_leak_guard_passed = !manifest.leak_in_any_prompt;
    if !(hash_checks_passed
        && stages_completed_all
        && budget_guard_passed
        && raw_stderr_leak_guard_passed)
    {
        return Err(format!(
            "TDMA evidence did not pass bridge gates: hash={hash_checks_passed} stages={stages_completed_all} budget={budget_guard_passed} leak_guard={raw_stderr_leak_guard_passed}"
        ));
    }

    let proof_capsule = TdmaProofCapsule {
        schema_version: "turingosv4.true_suite.tdma_proof_capsule.v1",
        run_id: args.run_id.clone(),
        problem_label: manifest.problem_label.clone(),
        model_label: manifest.model_label.clone(),
        stages_total: manifest.stages_total,
        stages_completed: manifest.stages_completed,
        total_attempts: manifest.total_attempts,
        all_prompts_within_budget: manifest.all_prompts_within_budget,
        leak_in_any_prompt: manifest.leak_in_any_prompt,
        chaintape_sha256,
        probes_sha256,
        manifest_sha256: sha256_hex(&manifest_raw),
        tdma_git_tape_present: tdma_tape.is_dir(),
        tdma_git_tape_verified_head_present: tdma_verified_head_present(&tdma_tape),
    };
    let proof_cid = put_json(
        &args.cas,
        &proof_capsule,
        ObjectType::EvidenceCapsule,
        "tdma-proof-capsule",
        1,
        "turingosv4.true_suite.tdma_proof_capsule.v1",
    )?;
    let evaluation = TdmaEvaluationCapsule {
        schema_version: "turingosv4.true_suite.tdma_evaluation_capsule.v1",
        run_id: args.run_id.clone(),
        proof_capsule_cid: proof_cid.hex(),
        stages_completed_all,
        hash_checks_passed,
        budget_guard_passed,
        raw_stderr_leak_guard_passed,
        benchmark_verdict: "tdma_proof_completed".to_string(),
        failure_class: None,
    };
    let evaluation_cid = put_json(
        &args.cas,
        &evaluation,
        ObjectType::ProposalPayload,
        "tdma-evaluation",
        2,
        "turingosv4.true_suite.tdma_evaluation_capsule.v1",
    )?;
    let prompt_hash = hash_from_hex_digest(&sha256_hex(format!(
        "{}:{}:{}:{}",
        args.run_id,
        manifest.problem_label,
        manifest.model_label,
        proof_cid.hex()
    )))?;
    let proposal_telemetry_cid = {
        let telemetry = ProposalTelemetry::new_root(
            AgentId(SOLVER_AGENT.to_string()),
            prompt_hash,
            evaluation_cid,
            "tdma_putnam_b3_proof".to_string(),
            TokenCounts {
                prompt_tokens: 0,
                completion_tokens: 0,
                tool_tokens: manifest.total_attempts,
            },
            format!("{SOLVER_AGENT}.tdma.b0"),
        );
        let mut cas = CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
        write_proposal_telemetry_to_cas(&mut cas, &telemetry, "tdma-proposal-telemetry", 3)
            .map_err(|e| format!("write ProposalTelemetry: {e}"))?
    };

    let preseed = default_pput_preseed_pairs();
    let mut initial_q = genesis_with_balances(&preseed);
    initial_q
        .economic_state_t
        .balances_t
        .0
        .entry(AgentId(SPONSOR_AGENT.to_string()))
        .or_insert(MicroCoin::from_micro_units(100_000));
    initial_q
        .economic_state_t
        .balances_t
        .0
        .entry(AgentId(SOLVER_AGENT.to_string()))
        .or_insert(MicroCoin::from_micro_units(100_000));
    let cfg = RuntimeChaintapeConfig {
        runtime_repo_path: args.runtime_repo.clone(),
        cas_path: args.cas.clone(),
        run_id: args.run_id.clone(),
        queue_capacity: 16,
        resume_existing_chain: false,
    };
    let bundle = build_chaintape_sequencer_with_initial_q(&cfg, initial_q)
        .map_err(|e| format!("fresh TDMA ChainTape boot failed: {e}"))?;
    let seq = bundle.sequencer.clone();
    let mut keypairs =
        AgentKeypairRegistry::open(&cfg.runtime_repo_path).map_err(|e| format!("{e}"))?;
    for id in [SPONSOR_AGENT, SOLVER_AGENT] {
        keypairs
            .get_or_create(&AgentId(id.to_string()))
            .map_err(|e| format!("create keypair for {id}: {e}"))?;
    }
    seq.set_agent_pubkeys(Arc::new(keypairs.manifest()))
        .map_err(|_| "agent pubkey manifest already set".to_string())?;

    let task = format!("tdma:{}", sanitize_id_fragment(&manifest.problem_label));
    let initial_root = seq
        .q_snapshot()
        .map_err(|e| format!("q_snapshot initial: {e:?}"))?
        .state_root_t;
    let task_open = make_real_task_open_signed_by(
        &mut keypairs,
        &task,
        SPONSOR_AGENT,
        initial_root,
        "true-suite-tdma",
        10,
    )
    .map_err(|e| format!("build TaskOpenTx: {e}"))?;
    seq.submit_agent_tx(task_open)
        .await
        .map_err(|e| format!("submit TaskOpenTx: {e:?}"))?;
    let after_open = tb8_await_state_root_advance(&seq, initial_root, 5_000)
        .await
        .map_err(|_| "TaskOpenTx did not advance state_root".to_string())?;

    let escrow = make_real_escrow_lock_signed_by(
        &mut keypairs,
        &task,
        SPONSOR_AGENT,
        TASK_ESCROW_MICRO,
        after_open,
        "true-suite-tdma",
        11,
    )
    .map_err(|e| format!("build EscrowLockTx: {e}"))?;
    seq.submit_agent_tx(escrow)
        .await
        .map_err(|e| format!("submit EscrowLockTx: {e:?}"))?;
    let after_escrow = tb8_await_state_root_advance(&seq, after_open, 5_000)
        .await
        .map_err(|_| "EscrowLockTx did not advance state_root".to_string())?;

    let work = make_real_worktx_signed_by(
        &mut keypairs,
        &task,
        SOLVER_AGENT,
        after_escrow,
        WORK_STAKE_MICRO,
        "true-suite-tdma",
        proposal_telemetry_cid,
        true,
        12,
    )
    .map_err(|e| format!("build WorkTx: {e}"))?;
    let work_tx_id = match &work {
        TypedTx::Work(w) => w.tx_id.0.clone(),
        _ => unreachable!("work helper returns WorkTx"),
    };
    seq.submit_agent_tx(work)
        .await
        .map_err(|e| format!("submit WorkTx: {e:?}"))?;
    let after_work = tb8_await_state_root_advance(&seq, after_escrow, 5_000)
        .await
        .map_err(|_| "WorkTx did not advance state_root".to_string())?;

    let seq_handle = seq.clone();
    bundle
        .shutdown()
        .await
        .map_err(|e| format!("TDMA ChainTape shutdown failed: {e}"))?;
    let post_q = seq_handle
        .q_snapshot()
        .map_err(|e| format!("post-drain q_snapshot: {e:?}"))?;
    let work_tx_landed = post_q
        .economic_state_t
        .stakes_t
        .0
        .contains_key(&turingosv4::state::q_state::TxId(work_tx_id.clone()));

    let report = GenesisReport {
        constitution_hash: GenesisReport::hash_constitution_md(&args.constitution),
        runtime_repo: args.runtime_repo.display().to_string(),
        cas_path: args.cas.display().to_string(),
        system_pubkey_hash: GenesisReport::hash_system_pubkey_manifest(&args.runtime_repo),
        agent_pubkeys_path: "agent_pubkeys.json".to_string(),
        initial_balances: preseed
            .iter()
            .map(|(agent, balance)| (agent.0.clone(), balance.micro_units()))
            .collect(),
        task_id: Some(task),
        task_open_tx: None,
        escrow_lock_tx: None,
        agent_model_assignment: vec![],
        model_assignment_manifest_cid: None,
        agent_role_assignment: vec![],
        role_assignment_manifest_cid: None,
    };
    report
        .write_to_runtime_repo(&args.runtime_repo)
        .map_err(|e| format!("write genesis_report.json: {e}"))?;

    let failure_taxonomy_path = args.out_dir.join("failure_taxonomy.json");
    let taxonomy = FailureTaxonomy {
        schema_version: "turingosv4.true_suite.tdma_failure_taxonomy.v1",
        run_id: args.run_id.clone(),
        family_id: "tdma_proof",
        failure_class: None,
        stages_completed_all,
        constitutional_rejection: false,
        kernel_invariant_failure: false,
        model_task_failure: false,
        infrastructure_failure: false,
    };
    write_pretty_json(&failure_taxonomy_path, &taxonomy)?;

    let evidence = TdmaEvidenceManifest {
        schema_version: "turingosv4.true_suite.tdma_proof_current_kernel.v1",
        run_id: args.run_id.clone(),
        problem_label: manifest.problem_label,
        model_label: manifest.model_label,
        tdma_evidence_dir: args.tdma_evidence_dir.display().to_string(),
        proof_capsule_cid: proof_cid.hex(),
        evaluation_capsule_cid: evaluation_cid.hex(),
        proposal_telemetry_cid: proposal_telemetry_cid.hex(),
        work_tx_id,
        work_tx_landed,
        stages_completed_all,
        benchmark_verdict: "tdma_proof_completed".to_string(),
        closure_scope: "domain_adapter_smoke_only",
        full_system_participation_required: true,
        final_closure_possible: false,
        failure_taxonomy_path: failure_taxonomy_path.display().to_string(),
        final_state_root_hex: hash_hex(&after_work),
        runtime_repo: args.runtime_repo.display().to_string(),
        cas: args.cas.display().to_string(),
        notes: vec![
            "TDMA proof-work tape remains a dedicated GitTapeLedger evidence source",
            "This bridge writes only a compact CAS summary and signed WorkTx to canonical ChainTape",
            "Raw provider prompt/response bytes are not written by this bridge",
            "Full-system liveness still requires replay and participation helpers",
        ],
    };
    write_pretty_json(&args.out_dir.join("tdma_proof_manifest.json"), &evidence)?;

    println!(
        "tdma_proof_current_kernel: work_tx_id={} verdict={} manifest={}",
        evidence.work_tx_id,
        evidence.benchmark_verdict,
        args.out_dir.join("tdma_proof_manifest.json").display()
    );
    Ok(())
}

fn require_file(path: &Path) -> Result<(), String> {
    if path.is_file() {
        Ok(())
    } else {
        Err(format!(
            "required TDMA evidence file missing: {}",
            path.display()
        ))
    }
}

fn tdma_verified_head_present(tdma_tape: &Path) -> bool {
    if !tdma_tape.is_dir() {
        return false;
    }
    let Some(git_dir) = tdma_tape.to_str() else {
        return false;
    };
    let cwd = tdma_tape
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    run_sanitized(SanitizedCommand {
        program: PathBuf::from("git"),
        args: vec![
            "--git-dir".to_string(),
            git_dir.to_string(),
            "rev-parse".to_string(),
            "--verify".to_string(),
            "refs/tdma/verified_head".to_string(),
        ],
        cwd,
        env: env_allowlist_from_current(&["PATH"]),
        stdin: None,
        timeout: Duration::from_secs(5),
    })
    .is_ok_and(|out| out.success())
}

fn sanitize_id_fragment(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

fn put_json<T: Serialize>(
    cas_path: &PathBuf,
    value: &T,
    object_type: ObjectType,
    creator: &str,
    logical_t: u64,
    schema_id: &str,
) -> Result<Cid, String> {
    let bytes =
        serde_json::to_vec(value).map_err(|e| format!("serialize CAS object {schema_id}: {e}"))?;
    let mut cas = CasStore::open(cas_path).map_err(|e| format!("open CAS: {e}"))?;
    cas.put(
        &bytes,
        object_type,
        creator,
        logical_t,
        Some(schema_id.to_string()),
    )
    .map_err(|e| format!("put CAS object {schema_id}: {e}"))
}

fn write_pretty_json<T: Serialize>(path: &PathBuf, value: &T) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    let bytes = serde_json::to_vec_pretty(value).map_err(|e| format!("serialize json: {e}"))?;
    std::fs::write(path, bytes).map_err(|e| format!("write {}: {e}", path.display()))
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    Ok(sha256_hex(bytes))
}

fn sha256_hex(input: impl AsRef<[u8]>) -> String {
    let digest = Sha256::digest(input.as_ref());
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

fn hash_hex(h: &Hash) -> String {
    h.0.iter().map(|b| format!("{b:02x}")).collect()
}

fn hash_from_hex_digest(hex: &str) -> Result<Hash, String> {
    if hex.len() != 64 {
        return Err(format!("sha256 hex digest must be 64 chars, got {hex}"));
    }
    let mut bytes = [0u8; 32];
    for i in 0..32 {
        bytes[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16)
            .map_err(|e| format!("parse sha256 hex byte {i}: {e}"))?;
    }
    Ok(Hash::from_bytes(bytes))
}
