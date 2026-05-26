//! True-suite SWE-bench coding-repair evidence helper.
//!
//! This binary is a runner helper. It consumes a public SWE-bench-compatible
//! issue sample, asks an external model through the local OpenAI-compatible
//! proxy for a repair patch, stores the issue/patch/evaluation as CAS
//! evidence, then submits a signed WorkTx through the current ChainTape
//! sequencer. The structural patch verdict is a capability signal only; the
//! liveness proof is the replayable ChainTape/CAS path.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
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
use turingosv4::state::q_state::{AgentId, Hash};
use turingosv4::state::typed_tx::TypedTx;

const SPONSOR_AGENT: &str = "Agent_user_0";
const SOLVER_AGENT: &str = "Agent_0";
const DEFAULT_MODEL: &str = "deepseek-chat";
const TASK_ESCROW_MICRO: i64 = 10_000;
const WORK_STAKE_MICRO: i64 = 100;
const MIN_RATIONALE_CHARS: usize = 120;

#[derive(Debug)]
struct Args {
    runtime_repo: PathBuf,
    cas: PathBuf,
    run_id: String,
    constitution: PathBuf,
    sample_json: PathBuf,
    llm_proxy_url: String,
    model: String,
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SweBenchSample {
    schema_version: String,
    sample_id: String,
    source_family: String,
    public_source: String,
    source_file: String,
    repo: String,
    instance_id: String,
    base_commit: String,
    problem_statement: String,
    hints_text: Option<String>,
    gold_patch: String,
    test_patch: String,
    fail_to_pass: Vec<String>,
    pass_to_pass: Vec<String>,
    created_at: Option<String>,
    version: Option<String>,
    environment_setup_commit: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct SweBenchIssueCapsule {
    schema_version: &'static str,
    sample_id: String,
    source_family: String,
    public_source: String,
    source_file: String,
    repo: String,
    instance_id: String,
    base_commit: String,
    problem_statement: String,
    fail_to_pass: Vec<String>,
    pass_to_pass: Vec<String>,
    expected_target_files: Vec<String>,
    gold_patch_sha256: String,
    test_patch_sha256: String,
    hints_text_sha256: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct SweBenchPatchClaimCapsule {
    schema_version: &'static str,
    sample_id: String,
    model_returned: String,
    target_files: Vec<String>,
    patch: String,
    patch_sha256: String,
    rationale: String,
    rationale_len_chars: usize,
    prompt_sha256: String,
    provider_response_sha256: String,
    parse_error: Option<String>,
    raw_provider_response_persisted: bool,
}

#[derive(Debug, Clone, Serialize)]
struct SweBenchEvaluationCapsule {
    schema_version: &'static str,
    sample_id: String,
    issue_capsule_cid: String,
    patch_claim_capsule_cid: String,
    expected_target_files: Vec<String>,
    claimed_target_files: Vec<String>,
    has_unified_diff: bool,
    target_file_overlap: bool,
    parse_guard_passed: bool,
    rationale_guard_passed: bool,
    patch_structurally_plausible: bool,
    benchmark_verdict: String,
    failure_class: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct SweBenchEvidenceManifest {
    schema_version: &'static str,
    run_id: String,
    model_requested: String,
    model_returned: String,
    llm_proxy_url: String,
    sample_id: String,
    source_family: String,
    public_source: String,
    repo: String,
    base_commit: String,
    prompt_sha256: String,
    provider_response_sha256: String,
    issue_capsule_cid: String,
    patch_claim_capsule_cid: String,
    evaluation_capsule_cid: String,
    proposal_telemetry_cid: String,
    work_tx_id: String,
    work_tx_landed: bool,
    has_unified_diff: bool,
    target_file_overlap: bool,
    parse_guard_passed: bool,
    rationale_guard_passed: bool,
    patch_structurally_plausible: bool,
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
    has_unified_diff: bool,
    target_file_overlap: bool,
    parse_guard_passed: bool,
    rationale_guard_passed: bool,
    patch_structurally_plausible: bool,
    constitutional_rejection: bool,
    kernel_invariant_failure: bool,
    model_task_failure: bool,
    infrastructure_failure: bool,
}

fn usage() -> &'static str {
    "usage: swebench_live_coding_repair_current_kernel --runtime-repo <PATH> --cas <PATH> --run-id <ID> --constitution <constitution.md> --sample-json <PATH> --llm-proxy-url <URL> [--model <MODEL>] --out-dir <PATH>"
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut runtime_repo: Option<PathBuf> = None;
    let mut cas: Option<PathBuf> = None;
    let mut run_id: Option<String> = None;
    let mut constitution: Option<PathBuf> = None;
    let mut sample_json: Option<PathBuf> = None;
    let mut llm_proxy_url: Option<String> = None;
    let mut model: Option<String> = None;
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
            "--sample-json" => {
                i += 1;
                sample_json = Some(
                    argv.get(i)
                        .ok_or("missing value after --sample-json")?
                        .into(),
                );
            }
            "--llm-proxy-url" => {
                i += 1;
                llm_proxy_url = Some(
                    argv.get(i)
                        .ok_or("missing value after --llm-proxy-url")?
                        .clone(),
                );
            }
            "--model" => {
                i += 1;
                model = Some(argv.get(i).ok_or("missing value after --model")?.clone());
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
        sample_json: sample_json.ok_or("--sample-json required")?,
        llm_proxy_url: llm_proxy_url.ok_or("--llm-proxy-url required")?,
        model: model.unwrap_or_else(|| DEFAULT_MODEL.to_string()),
        out_dir: out_dir.ok_or("--out-dir required")?,
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse_args(&argv) {
        Ok(args) => args,
        Err(msg) => {
            eprintln!("swebench_live_coding_repair_current_kernel: {msg}");
            eprintln!("{}", usage());
            return ExitCode::from(2);
        }
    };
    match run(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("swebench_live_coding_repair_current_kernel: {err}");
            ExitCode::from(1)
        }
    }
}

async fn run(args: Args) -> Result<(), String> {
    std::fs::create_dir_all(&args.runtime_repo).map_err(|e| format!("runtime repo dir: {e}"))?;
    std::fs::create_dir_all(&args.cas).map_err(|e| format!("cas dir: {e}"))?;
    std::fs::create_dir_all(&args.out_dir).map_err(|e| format!("out dir: {e}"))?;

    let sample: SweBenchSample = serde_json::from_slice(
        &std::fs::read(&args.sample_json).map_err(|e| format!("read sample json: {e}"))?,
    )
    .map_err(|e| format!("parse sample json: {e}"))?;
    validate_sample(&sample)?;

    let expected_target_files = target_files_from_patch(&sample.gold_patch);
    let issue_capsule = SweBenchIssueCapsule {
        schema_version: "turingosv4.true_suite.swebench_issue_capsule.v1",
        sample_id: sample.sample_id.clone(),
        source_family: sample.source_family.clone(),
        public_source: sample.public_source.clone(),
        source_file: sample.source_file.clone(),
        repo: sample.repo.clone(),
        instance_id: sample.instance_id.clone(),
        base_commit: sample.base_commit.clone(),
        problem_statement: sample.problem_statement.clone(),
        fail_to_pass: sample.fail_to_pass.clone(),
        pass_to_pass: sample.pass_to_pass.clone(),
        expected_target_files: expected_target_files.clone(),
        gold_patch_sha256: sha256_hex(&sample.gold_patch),
        test_patch_sha256: sha256_hex(&sample.test_patch),
        hints_text_sha256: sample.hints_text.as_ref().map(sha256_hex),
    };
    let issue_cid = put_json(
        &args.cas,
        &issue_capsule,
        ObjectType::EvidenceCapsule,
        "swebench-issue",
        1,
        "turingosv4.true_suite.swebench_issue_capsule.v1",
    )?;

    let prompt = build_prompt(&sample, &issue_cid);
    let prompt_sha256 = sha256_hex(&prompt);
    let response = ResilientLLMClient::new(&args.llm_proxy_url, 180, 2)
        .generate(&GenerateRequest {
            model: args.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "You repair public software issues. Return only strict JSON."
                        .to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            temperature: Some(0.0),
            max_tokens: Some(1800),
        })
        .await
        .map_err(|e| format!("llm proxy generation failed: {e}"))?;
    let provider_response_sha256 = sha256_hex(&response.content);
    let parsed_result = parse_patch_claim(&response.content);
    let parse_error = parsed_result.as_ref().err().map(ToString::to_string);
    let parse_guard_passed = parse_error.is_none();
    let parsed = parsed_result.unwrap_or_else(|err| ParsedPatchClaim {
        target_files: Vec::new(),
        patch: String::new(),
        rationale: format!(
            "Strict JSON patch parse failed: {err}. The provider response is not persisted; provider_response_sha256={provider_response_sha256}."
        ),
    });

    let rationale_guard_passed =
        parse_guard_passed && parsed.rationale.trim().chars().count() >= MIN_RATIONALE_CHARS;
    let claimed_target_files = merged_target_files(&parsed.target_files, &parsed.patch);
    let has_unified_diff = looks_like_unified_diff(&parsed.patch);
    let target_file_overlap = intersects(&expected_target_files, &claimed_target_files);
    let patch_structurally_plausible =
        parse_guard_passed && rationale_guard_passed && has_unified_diff && target_file_overlap;
    let benchmark_verdict = if patch_structurally_plausible {
        "repair_patch_structurally_plausible"
    } else {
        "repair_patch_structurally_rejected"
    };
    let failure_class = (!patch_structurally_plausible).then(|| "model_task_failure".to_string());

    let patch_claim = SweBenchPatchClaimCapsule {
        schema_version: "turingosv4.true_suite.swebench_patch_claim_capsule.v1",
        sample_id: sample.sample_id.clone(),
        model_returned: response.model.clone(),
        target_files: claimed_target_files.clone(),
        patch: parsed.patch.clone(),
        patch_sha256: sha256_hex(&parsed.patch),
        rationale: parsed.rationale.clone(),
        rationale_len_chars: parsed.rationale.trim().chars().count(),
        prompt_sha256: prompt_sha256.clone(),
        provider_response_sha256: provider_response_sha256.clone(),
        parse_error: parse_error.clone(),
        raw_provider_response_persisted: false,
    };
    let patch_claim_cid = put_json(
        &args.cas,
        &patch_claim,
        ObjectType::EvidenceCapsule,
        "swebench-patch-claim",
        2,
        "turingosv4.true_suite.swebench_patch_claim_capsule.v1",
    )?;

    let evaluation = SweBenchEvaluationCapsule {
        schema_version: "turingosv4.true_suite.swebench_evaluation_capsule.v1",
        sample_id: sample.sample_id.clone(),
        issue_capsule_cid: issue_cid.hex(),
        patch_claim_capsule_cid: patch_claim_cid.hex(),
        expected_target_files: expected_target_files.clone(),
        claimed_target_files: claimed_target_files.clone(),
        has_unified_diff,
        target_file_overlap,
        parse_guard_passed,
        rationale_guard_passed,
        patch_structurally_plausible,
        benchmark_verdict: benchmark_verdict.to_string(),
        failure_class: failure_class.clone(),
    };
    let evaluation_cid = put_json(
        &args.cas,
        &evaluation,
        ObjectType::ProposalPayload,
        "swebench-evaluation",
        3,
        "turingosv4.true_suite.swebench_evaluation_capsule.v1",
    )?;
    let proposal_telemetry_cid = {
        let telemetry = ProposalTelemetry::new_root(
            AgentId(SOLVER_AGENT.to_string()),
            hash_from_hex_digest(&prompt_sha256)?,
            evaluation_cid,
            "swebench_live_coding_repair".to_string(),
            TokenCounts {
                prompt_tokens: response.prompt_tokens as u64,
                completion_tokens: response.completion_tokens as u64,
                tool_tokens: 0,
            },
            format!("{SOLVER_AGENT}.swebench.b0"),
        );
        let mut cas = CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
        write_proposal_telemetry_to_cas(&mut cas, &telemetry, "swebench-proposal-telemetry", 4)
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
        .map_err(|e| format!("fresh SWE-bench boot failed: {e}"))?;
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

    let task = format!("swebench:{}", sanitize_id_fragment(&sample.instance_id));
    let initial_root = seq
        .q_snapshot()
        .map_err(|e| format!("q_snapshot initial: {e:?}"))?
        .state_root_t;
    let task_open = make_real_task_open_signed_by(
        &mut keypairs,
        &task,
        SPONSOR_AGENT,
        initial_root,
        "true-suite-swebench",
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
        "true-suite-swebench",
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
        "true-suite-swebench",
        proposal_telemetry_cid,
        patch_structurally_plausible,
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
    let after_work = if patch_structurally_plausible {
        tb8_await_state_root_advance(&seq, after_escrow, 5_000)
            .await
            .map_err(|_| "WorkTx did not advance state_root".to_string())?
    } else {
        after_escrow
    };

    let seq_handle = seq.clone();
    bundle
        .shutdown()
        .await
        .map_err(|e| format!("SWE-bench chaintape shutdown failed: {e}"))?;
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
        task_id: Some(task.clone()),
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
        schema_version: "turingosv4.true_suite.swebench_failure_taxonomy.v1",
        run_id: args.run_id.clone(),
        family_id: "swebench_live_coding_repair",
        failure_class: failure_class.clone(),
        has_unified_diff,
        target_file_overlap,
        parse_guard_passed,
        rationale_guard_passed,
        patch_structurally_plausible,
        constitutional_rejection: false,
        kernel_invariant_failure: false,
        model_task_failure: !patch_structurally_plausible,
        infrastructure_failure: false,
    };
    write_pretty_json(&failure_taxonomy_path, &taxonomy)?;

    let manifest = SweBenchEvidenceManifest {
        schema_version: "turingosv4.true_suite.swebench_live_coding_repair.v1",
        run_id: args.run_id.clone(),
        model_requested: args.model,
        model_returned: response.model,
        llm_proxy_url: args.llm_proxy_url,
        sample_id: sample.sample_id,
        source_family: sample.source_family,
        public_source: sample.public_source,
        repo: sample.repo,
        base_commit: sample.base_commit,
        prompt_sha256,
        provider_response_sha256,
        issue_capsule_cid: issue_cid.hex(),
        patch_claim_capsule_cid: patch_claim_cid.hex(),
        evaluation_capsule_cid: evaluation_cid.hex(),
        proposal_telemetry_cid: proposal_telemetry_cid.hex(),
        work_tx_id,
        work_tx_landed,
        has_unified_diff,
        target_file_overlap,
        parse_guard_passed,
        rationale_guard_passed,
        patch_structurally_plausible,
        benchmark_verdict: benchmark_verdict.to_string(),
        closure_scope: "domain_adapter_smoke_only",
        full_system_participation_required: true,
        final_closure_possible: false,
        failure_taxonomy_path: failure_taxonomy_path.display().to_string(),
        final_state_root_hex: hash_hex(&after_work),
        runtime_repo: args.runtime_repo.display().to_string(),
        cas: args.cas.display().to_string(),
        notes: vec![
            "SWE-bench issue input is hashed into CAS before the model call",
            "gold patch and test patch remain benchmark-side evaluation data and are not written to the prompt",
            "DeepSeek/SiliconFlow access is outside the kernel through the local LLM proxy",
            "raw prompt and raw provider response are not written to evidence",
            "unparseable external responses become model_task_failure evidence instead of infrastructure failure",
            "structural patch plausibility is a capability signal only, not OBL-005 closure",
        ],
    };
    write_pretty_json(
        &args
            .out_dir
            .join("swebench_live_coding_repair_manifest.json"),
        &manifest,
    )?;

    println!(
        "swebench_live_coding_repair_current_kernel: work_tx_id={} verdict={} structural_pass={} manifest={}",
        manifest.work_tx_id,
        manifest.benchmark_verdict,
        manifest.patch_structurally_plausible,
        args.out_dir
            .join("swebench_live_coding_repair_manifest.json")
            .display()
    );
    Ok(())
}

#[derive(Debug)]
struct ParsedPatchClaim {
    target_files: Vec<String>,
    patch: String,
    rationale: String,
}

fn parse_patch_claim(content: &str) -> Result<ParsedPatchClaim, String> {
    let value = extract_json_object(content)?;
    let patch = value
        .get("patch")
        .or_else(|| value.get("diff"))
        .and_then(serde_json::Value::as_str)
        .ok_or("external agent JSON missing `patch`")?;
    let rationale = value
        .get("rationale")
        .or_else(|| value.get("reasoning"))
        .or_else(|| value.get("explanation"))
        .and_then(serde_json::Value::as_str)
        .ok_or("external agent JSON missing `rationale`")?;
    let target_files = value
        .get("target_files")
        .and_then(serde_json::Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(ParsedPatchClaim {
        target_files,
        patch: patch.trim().to_string(),
        rationale: rationale.trim().to_string(),
    })
}

fn extract_json_object(content: &str) -> Result<serde_json::Value, String> {
    let trimmed = content.trim();
    if let Ok(value) = serde_json::from_str(trimmed) {
        return Ok(value);
    }
    let start = trimmed
        .find('{')
        .ok_or("external agent response did not contain a JSON object")?;
    let end = trimmed
        .rfind('}')
        .ok_or("external agent response had no JSON object terminator")?;
    serde_json::from_str(&trimmed[start..=end])
        .map_err(|e| format!("parse external agent JSON object: {e}"))
}

fn validate_sample(sample: &SweBenchSample) -> Result<(), String> {
    if sample.problem_statement.trim().is_empty() {
        return Err("sample problem_statement is empty".to_string());
    }
    if sample.gold_patch.trim().is_empty() {
        return Err("sample gold_patch is empty".to_string());
    }
    if sample.repo.trim().is_empty() {
        return Err("sample repo is empty".to_string());
    }
    if sample.base_commit.trim().is_empty() {
        return Err("sample base_commit is empty".to_string());
    }
    Ok(())
}

fn build_prompt(sample: &SweBenchSample, issue_cid: &Cid) -> String {
    format!(
        "SWE-bench sample id: {}\nIssue capsule cid: {}\nRepository: {}\nBase commit: {}\nFailing tests to repair:\n{}\nRegression tests to preserve:\n{}\n\nProblem statement:\n{}\n\nReturn strict JSON only with fields:\n  target_files: array of repository paths you intend to change\n  patch: a unified diff beginning with diff --git\n  rationale: 3-8 sentences explaining why the patch addresses the issue.\nDo not use web lookup. Do not include or quote any hidden benchmark patch, test patch, or hints.",
        sample.sample_id,
        issue_cid.hex(),
        sample.repo,
        sample.base_commit,
        sample.fail_to_pass.join("\n"),
        sample.pass_to_pass.iter().take(8).cloned().collect::<Vec<_>>().join("\n"),
        sample.problem_statement,
    )
}

fn target_files_from_patch(patch: &str) -> Vec<String> {
    let mut files = BTreeSet::new();
    for line in patch.lines() {
        if let Some(rest) = line.strip_prefix("diff --git a/") {
            let mut parts = rest.split_whitespace();
            if let Some(left) = parts.next() {
                files.insert(left.to_string());
            }
            if let Some(right) = parts.next().and_then(|s| s.strip_prefix("b/")) {
                files.insert(right.to_string());
            }
        }
    }
    files.into_iter().collect()
}

fn merged_target_files(claimed: &[String], patch: &str) -> Vec<String> {
    let mut files: BTreeSet<String> = claimed
        .iter()
        .map(|s| s.trim().trim_start_matches("a/").trim_start_matches("b/"))
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .collect();
    for file in target_files_from_patch(patch) {
        files.insert(file);
    }
    files.into_iter().collect()
}

fn looks_like_unified_diff(patch: &str) -> bool {
    let p = patch.trim();
    p.contains("diff --git a/") && p.contains("--- a/") && p.contains("+++ b/") && p.contains("@@")
}

fn intersects(left: &[String], right: &[String]) -> bool {
    let right: BTreeSet<&str> = right.iter().map(String::as_str).collect();
    left.iter().any(|item| right.contains(item.as_str()))
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
