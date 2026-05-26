//! True-suite ToolBench API tool-use evidence helper.
//!
//! This runner helper consumes a ToolBench-compatible benchmark sample, asks an
//! external model through the local OpenAI-compatible proxy to choose APIs, stores
//! the input/claim/evaluation as CAS evidence, records structural tool calls in
//! `ProposalTelemetry`, and submits a signed WorkTx through current ChainTape.
//! ToolBench accuracy is a capability signal only; the liveness proof is the
//! replayable ChainTape/CAS path plus hashed tool-call records.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;
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
    write_to_cas as write_proposal_telemetry_to_cas, ProposalTelemetry, TokenCounts, ToolCallRecord,
};
use turingosv4::runtime::{build_chaintape_sequencer_with_initial_q, RuntimeChaintapeConfig};
use turingosv4::state::q_state::{AgentId, Hash};
use turingosv4::state::typed_tx::TypedTx;

const SPONSOR_AGENT: &str = "Agent_user_0";
const SOLVER_AGENT: &str = "Agent_0";
const DEFAULT_MODEL: &str = "deepseek-chat";
const TASK_ESCROW_MICRO: i64 = 10_000;
const WORK_STAKE_MICRO: i64 = 100;
const MIN_RATIONALE_CHARS: usize = 80;

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
struct ToolBenchSample {
    schema_version: String,
    query_id: String,
    source_family: String,
    public_source: String,
    source_split: String,
    query: String,
    api_list: Value,
    relevant_apis: Value,
}

#[derive(Debug, Clone, Serialize)]
struct ToolBenchInputCapsule {
    schema_version: &'static str,
    query_id: String,
    source_family: String,
    public_source: String,
    source_split: String,
    query: String,
    api_list: Value,
    available_api_ids: Vec<String>,
    relevant_api_ids_sha256: String,
}

#[derive(Debug, Clone, Serialize)]
struct ToolBenchAnswerClaimCapsule {
    schema_version: &'static str,
    query_id: String,
    model_returned: String,
    selected_api_ids: Vec<String>,
    rationale: String,
    rationale_len_chars: usize,
    prompt_sha256: String,
    provider_response_sha256: String,
    raw_provider_response_persisted: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ToolBenchEvaluationCapsule {
    schema_version: &'static str,
    query_id: String,
    input_capsule_cid: String,
    answer_claim_capsule_cid: String,
    expected_api_ids: Vec<String>,
    selected_api_ids: Vec<String>,
    selected_apis_available: bool,
    rationale_guard_passed: bool,
    exact_match: bool,
    benchmark_verdict: String,
    failure_class: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct ToolBenchFcTraceReport {
    schema_version: &'static str,
    run_id: String,
    family_id: &'static str,
    fc_blocks_seen: Vec<&'static str>,
    input_capsule_cid: String,
    answer_claim_capsule_cid: String,
    evaluation_capsule_cid: String,
    proposal_telemetry_cid: String,
    work_tx_id: String,
    work_tx_landed: bool,
    tool_call_count: usize,
    tool_selection_exact_match: bool,
    closure_scope: &'static str,
    full_system_participation_required: bool,
    final_closure_possible: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ToolBenchEvidenceManifest {
    schema_version: &'static str,
    run_id: String,
    model_requested: String,
    model_returned: String,
    llm_proxy_url: String,
    query_id: String,
    source_family: String,
    public_source: String,
    source_split: String,
    prompt_sha256: String,
    provider_response_sha256: String,
    input_capsule_cid: String,
    answer_claim_capsule_cid: String,
    evaluation_capsule_cid: String,
    proposal_telemetry_cid: String,
    work_tx_id: String,
    work_tx_landed: bool,
    selected_apis_available: bool,
    rationale_guard_passed: bool,
    exact_match: bool,
    benchmark_verdict: String,
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
    selected_apis_available: bool,
    exact_match: bool,
    constitutional_rejection: bool,
    kernel_invariant_failure: bool,
    model_task_failure: bool,
    infrastructure_failure: bool,
}

#[derive(Debug, Clone)]
struct ParsedToolClaim {
    selected_api_ids: Vec<String>,
    rationale: String,
}

fn usage() -> &'static str {
    "usage: toolbench_api_tool_use_current_kernel --runtime-repo <PATH> --cas <PATH> --run-id <ID> --constitution <constitution.md> --sample-json <PATH> --llm-proxy-url <URL> [--model <MODEL>] --out-dir <PATH>"
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
            eprintln!("toolbench_api_tool_use_current_kernel: {msg}");
            eprintln!("{}", usage());
            return ExitCode::from(2);
        }
    };
    match run(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("toolbench_api_tool_use_current_kernel: {err}");
            ExitCode::from(1)
        }
    }
}

async fn run(args: Args) -> Result<(), String> {
    std::fs::create_dir_all(&args.runtime_repo).map_err(|e| format!("runtime repo dir: {e}"))?;
    std::fs::create_dir_all(&args.cas).map_err(|e| format!("cas dir: {e}"))?;
    std::fs::create_dir_all(&args.out_dir).map_err(|e| format!("out dir: {e}"))?;
    std::fs::create_dir_all(args.out_dir.join("tool_capsules"))
        .map_err(|e| format!("tool capsules dir: {e}"))?;

    let sample: ToolBenchSample = serde_json::from_slice(
        &std::fs::read(&args.sample_json).map_err(|e| format!("read sample json: {e}"))?,
    )
    .map_err(|e| format!("parse sample json: {e}"))?;
    validate_sample(&sample)?;

    let available_api_ids = available_api_ids(&sample.api_list)?;
    let expected_api_ids = relevant_api_ids(&sample.relevant_apis)?;
    if expected_api_ids.is_empty() {
        return Err("ToolBench sample relevant_apis yielded no API ids".to_string());
    }
    let expected_hash = sha256_hex(expected_api_ids.join("\n"));

    let input_capsule = ToolBenchInputCapsule {
        schema_version: "turingosv4.true_suite.toolbench_input_capsule.v1",
        query_id: sample.query_id.clone(),
        source_family: sample.source_family.clone(),
        public_source: sample.public_source.clone(),
        source_split: sample.source_split.clone(),
        query: sample.query.clone(),
        api_list: sample.api_list.clone(),
        available_api_ids: available_api_ids.clone(),
        relevant_api_ids_sha256: expected_hash,
    };
    write_pretty_json(
        &args
            .out_dir
            .join("tool_capsules")
            .join("input_capsule.json"),
        &input_capsule,
    )?;
    let input_cid = put_json(
        &args.cas,
        &input_capsule,
        ObjectType::EvidenceCapsule,
        "toolbench-input",
        1,
        "turingosv4.true_suite.toolbench_input_capsule.v1",
    )?;

    let prompt = build_prompt(&sample, &input_cid, &available_api_ids);
    let prompt_sha256 = sha256_hex(&prompt);
    let response = ResilientLLMClient::new(&args.llm_proxy_url, 180, 2)
        .generate(&GenerateRequest {
            model: args.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content:
                        "You solve ToolBench-style API selection tasks. Return only strict JSON."
                            .to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            temperature: Some(0.0),
            max_tokens: Some(1200),
        })
        .await
        .map_err(|e| format!("llm proxy generation failed: {e}"))?;
    let provider_response_sha256 = sha256_hex(&response.content);
    let parsed = parse_tool_claim(&response.content, &available_api_ids)?;
    let rationale_guard_passed = parsed.rationale.trim().chars().count() >= MIN_RATIONALE_CHARS;
    if !rationale_guard_passed {
        return Err(format!(
            "ToolBench answer rejected before WorkTx: rationale too short ({} chars, need >= {MIN_RATIONALE_CHARS})",
            parsed.rationale.trim().chars().count()
        ));
    }
    let selected_apis_available = parsed
        .selected_api_ids
        .iter()
        .all(|id| available_api_ids.contains(id));
    if !selected_apis_available {
        return Err("ToolBench selected API outside available API set".to_string());
    }

    let answer_claim = ToolBenchAnswerClaimCapsule {
        schema_version: "turingosv4.true_suite.toolbench_answer_claim_capsule.v1",
        query_id: sample.query_id.clone(),
        model_returned: response.model.clone(),
        selected_api_ids: parsed.selected_api_ids.clone(),
        rationale: parsed.rationale.clone(),
        rationale_len_chars: parsed.rationale.trim().chars().count(),
        prompt_sha256: prompt_sha256.clone(),
        provider_response_sha256: provider_response_sha256.clone(),
        raw_provider_response_persisted: false,
    };
    write_pretty_json(
        &args.out_dir.join("tool_capsules").join("answer_claim.json"),
        &answer_claim,
    )?;
    let answer_claim_cid = put_json(
        &args.cas,
        &answer_claim,
        ObjectType::EvidenceCapsule,
        "toolbench-answer-claim",
        2,
        "turingosv4.true_suite.toolbench_answer_claim_capsule.v1",
    )?;

    let exact_match = same_set(&parsed.selected_api_ids, &expected_api_ids);
    let benchmark_verdict = if exact_match {
        "exact_tool_match"
    } else {
        "tool_selection_mismatch"
    };
    let failure_class = (!exact_match).then(|| "model_task_failure".to_string());
    let evaluation = ToolBenchEvaluationCapsule {
        schema_version: "turingosv4.true_suite.toolbench_evaluation_capsule.v1",
        query_id: sample.query_id.clone(),
        input_capsule_cid: input_cid.hex(),
        answer_claim_capsule_cid: answer_claim_cid.hex(),
        expected_api_ids: expected_api_ids.clone(),
        selected_api_ids: parsed.selected_api_ids.clone(),
        selected_apis_available,
        rationale_guard_passed,
        exact_match,
        benchmark_verdict: benchmark_verdict.to_string(),
        failure_class: failure_class.clone(),
    };
    write_pretty_json(
        &args.out_dir.join("tool_capsules").join("evaluation.json"),
        &evaluation,
    )?;
    let evaluation_cid = put_json(
        &args.cas,
        &evaluation,
        ObjectType::ProposalPayload,
        "toolbench-evaluation",
        3,
        "turingosv4.true_suite.toolbench_evaluation_capsule.v1",
    )?;

    let tool_calls = parsed
        .selected_api_ids
        .iter()
        .map(|api_id| ToolCallRecord {
            tool_id: api_id.clone(),
            args_hash: hash_from_bytes(format!("{}:{api_id}:args", sample.query_id).as_bytes()),
            result_hash: hash_from_bytes(
                format!("{}:{api_id}:selection-only-result", sample.query_id).as_bytes(),
            ),
        })
        .collect::<Vec<_>>();
    let proposal_telemetry_cid = {
        let mut telemetry = ProposalTelemetry::new_root(
            AgentId(SOLVER_AGENT.to_string()),
            hash_from_hex_digest(&prompt_sha256)?,
            evaluation_cid,
            "toolbench_api_tool_selection".to_string(),
            TokenCounts {
                prompt_tokens: response.prompt_tokens as u64,
                completion_tokens: response.completion_tokens as u64,
                tool_tokens: tool_calls.len() as u64,
            },
            format!("{SOLVER_AGENT}.toolbench.b0"),
        );
        telemetry.tool_calls = tool_calls;
        let mut cas = CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
        write_proposal_telemetry_to_cas(&mut cas, &telemetry, "toolbench-proposal-telemetry", 4)
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
        .map_err(|e| format!("fresh ToolBench boot failed: {e}"))?;
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

    let task = format!("toolbench:{}", sanitize_id_fragment(&sample.query_id));
    let initial_root = seq
        .q_snapshot()
        .map_err(|e| format!("q_snapshot initial: {e:?}"))?
        .state_root_t;
    let task_open = make_real_task_open_signed_by(
        &mut keypairs,
        &task,
        SPONSOR_AGENT,
        initial_root,
        "true-suite-toolbench",
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
        "true-suite-toolbench",
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
        "true-suite-toolbench",
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
        .map_err(|e| format!("ToolBench chaintape shutdown failed: {e}"))?;
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

    let taxonomy = FailureTaxonomy {
        schema_version: "turingosv4.true_suite.toolbench_failure_taxonomy.v1",
        run_id: args.run_id.clone(),
        family_id: "toolbench_api_tool_use",
        failure_class: failure_class.clone(),
        selected_apis_available,
        exact_match,
        constitutional_rejection: false,
        kernel_invariant_failure: false,
        model_task_failure: !exact_match,
        infrastructure_failure: false,
    };
    write_pretty_json(&args.out_dir.join("failure_taxonomy.json"), &taxonomy)?;

    let fc_trace = ToolBenchFcTraceReport {
        schema_version: "turingosv4.true_suite.toolbench_fc_trace_report.v1",
        run_id: args.run_id.clone(),
        family_id: "toolbench_api_tool_use",
        fc_blocks_seen: vec!["FC1", "FC3"],
        input_capsule_cid: input_cid.hex(),
        answer_claim_capsule_cid: answer_claim_cid.hex(),
        evaluation_capsule_cid: evaluation_cid.hex(),
        proposal_telemetry_cid: proposal_telemetry_cid.hex(),
        work_tx_id: work_tx_id.clone(),
        work_tx_landed,
        tool_call_count: parsed.selected_api_ids.len(),
        tool_selection_exact_match: exact_match,
        closure_scope: "domain_adapter_smoke_only",
        full_system_participation_required: true,
        final_closure_possible: false,
    };
    write_pretty_json(&args.out_dir.join("fc_trace_report.json"), &fc_trace)?;

    let manifest = ToolBenchEvidenceManifest {
        schema_version: "turingosv4.true_suite.toolbench_api_tool_use.v1",
        run_id: args.run_id.clone(),
        model_requested: args.model,
        model_returned: response.model,
        llm_proxy_url: args.llm_proxy_url,
        query_id: sample.query_id,
        source_family: sample.source_family,
        public_source: sample.public_source,
        source_split: sample.source_split,
        prompt_sha256,
        provider_response_sha256,
        input_capsule_cid: input_cid.hex(),
        answer_claim_capsule_cid: answer_claim_cid.hex(),
        evaluation_capsule_cid: evaluation_cid.hex(),
        proposal_telemetry_cid: proposal_telemetry_cid.hex(),
        work_tx_id,
        work_tx_landed,
        selected_apis_available,
        rationale_guard_passed,
        exact_match,
        benchmark_verdict: benchmark_verdict.to_string(),
        final_state_root_hex: hash_hex(&after_work),
        runtime_repo: args.runtime_repo.display().to_string(),
        cas: args.cas.display().to_string(),
        notes: vec![
            "ToolBench benchmark input is hashed into CAS before the model call",
            "DeepSeek/SiliconFlow access is outside the kernel through the local LLM proxy",
            "raw prompt and raw provider response are not written to evidence",
            "selected APIs are recorded as ProposalTelemetry ToolCallRecord hashes",
            "ToolBench exact-match accuracy is a capability signal only, not OBL-005 closure",
            "this runner evaluates API selection and structural invocation records, not live third-party API side effects",
        ],
    };
    write_pretty_json(
        &args.out_dir.join("toolbench_api_tool_use_manifest.json"),
        &manifest,
    )?;

    println!(
        "toolbench_api_tool_use_current_kernel: work_tx_id={} verdict={} exact_match={} manifest={}",
        manifest.work_tx_id,
        manifest.benchmark_verdict,
        manifest.exact_match,
        args.out_dir
            .join("toolbench_api_tool_use_manifest.json")
            .display()
    );
    Ok(())
}

fn validate_sample(sample: &ToolBenchSample) -> Result<(), String> {
    if sample.query.trim().is_empty() {
        return Err("ToolBench query is empty".to_string());
    }
    if available_api_ids(&sample.api_list)?.is_empty() {
        return Err("ToolBench sample api_list yielded no API ids".to_string());
    }
    Ok(())
}

fn build_prompt(sample: &ToolBenchSample, input_cid: &Cid, available_ids: &[String]) -> String {
    let mut api_summaries = String::new();
    for (idx, api) in flatten_api_objects(&sample.api_list)
        .iter()
        .take(16)
        .enumerate()
    {
        let id = api_id_from_value(api).unwrap_or_else(|| format!("api_{idx}"));
        let desc = api
            .get("api_description")
            .or_else(|| api.get("description"))
            .and_then(Value::as_str)
            .unwrap_or("");
        let required = api
            .get("required_parameters")
            .map(compact_json)
            .unwrap_or_else(|| "[]".to_string());
        api_summaries.push_str(&format!(
            "- {id}: {}; required_parameters={}\n",
            truncate(desc, 220),
            truncate(&required, 180)
        ));
    }
    format!(
        "ToolBench query id: {}\nInput capsule cid: {}\n\nUser query:\n{}\n\nAvailable API ids:\n{}\n\nAPI summaries:\n{}\nReturn strict JSON only with fields:\n  selected_apis: array of exact API ids from the available list\n  rationale: 2-5 sentences explaining why these APIs satisfy the query.\nDo not invent APIs. Do not include hidden answer labels.",
        sample.query_id,
        input_cid.hex(),
        sample.query,
        available_ids.join(", "),
        api_summaries
    )
}

fn parse_tool_claim(
    content: &str,
    available_api_ids: &[String],
) -> Result<ParsedToolClaim, String> {
    let value = extract_json_object(content)?;
    let rationale = value
        .get("rationale")
        .or_else(|| value.get("reasoning"))
        .or_else(|| value.get("explanation"))
        .and_then(Value::as_str)
        .ok_or("external agent JSON missing `rationale`")?
        .trim()
        .to_string();
    let mut selected = Vec::new();
    if let Some(array) = value.get("selected_apis").and_then(Value::as_array) {
        for item in array {
            if let Some(raw) = item.as_str() {
                selected.push(canonical_api_id(raw, available_api_ids)?);
            } else if let Some(id) = api_id_from_value(item) {
                selected.push(canonical_api_id(&id, available_api_ids)?);
            }
        }
    }
    if selected.is_empty() {
        if let Some(array) = value.get("tool_calls").and_then(Value::as_array) {
            for item in array {
                if let Some(raw) = item
                    .get("api_id")
                    .or_else(|| item.get("tool_id"))
                    .or_else(|| item.get("name"))
                    .and_then(Value::as_str)
                {
                    selected.push(canonical_api_id(raw, available_api_ids)?);
                } else if let Some(id) = api_id_from_value(item) {
                    selected.push(canonical_api_id(&id, available_api_ids)?);
                }
            }
        }
    }
    selected.sort();
    selected.dedup();
    if selected.is_empty() {
        return Err("external agent selected no APIs".to_string());
    }
    Ok(ParsedToolClaim {
        selected_api_ids: selected,
        rationale,
    })
}

fn extract_json_object(content: &str) -> Result<Value, String> {
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

fn available_api_ids(api_list: &Value) -> Result<Vec<String>, String> {
    let mut ids = flatten_api_objects(api_list)
        .into_iter()
        .filter_map(api_id_from_value)
        .collect::<Vec<_>>();
    ids.sort();
    ids.dedup();
    if ids.is_empty() {
        return Err("api_list has no tool_name/api_name pairs".to_string());
    }
    Ok(ids)
}

fn relevant_api_ids(relevant_apis: &Value) -> Result<Vec<String>, String> {
    let mut ids = Vec::new();
    collect_relevant_api_ids(relevant_apis, &mut ids);
    ids.sort();
    ids.dedup();
    Ok(ids)
}

fn collect_relevant_api_ids(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::Array(items) => {
            if items.len() >= 2 {
                if let (Some(tool), Some(api)) = (items[0].as_str(), items[1].as_str()) {
                    out.push(format!("{tool}::{api}"));
                    return;
                }
            }
            for item in items {
                collect_relevant_api_ids(item, out);
            }
        }
        Value::Object(_) => {
            if let Some(id) = api_id_from_value(value) {
                out.push(id);
            }
        }
        Value::String(raw) => {
            if let Ok(parsed) = serde_json::from_str::<Value>(raw) {
                collect_relevant_api_ids(&parsed, out);
            }
        }
        _ => {}
    }
}

fn flatten_api_objects(value: &Value) -> Vec<&Value> {
    let mut out = Vec::new();
    collect_api_objects(value, &mut out);
    out
}

fn collect_api_objects<'a>(value: &'a Value, out: &mut Vec<&'a Value>) {
    match value {
        Value::Array(items) => {
            for item in items {
                collect_api_objects(item, out);
            }
        }
        Value::Object(map) => {
            if map.contains_key("tool_name") && map.contains_key("api_name") {
                out.push(value);
            } else {
                for item in map.values() {
                    collect_api_objects(item, out);
                }
            }
        }
        Value::String(raw) => {
            if let Ok(parsed) = serde_json::from_str::<Value>(raw) {
                let leaked: &'static Value = Box::leak(Box::new(parsed));
                collect_api_objects(leaked, out);
            }
        }
        _ => {}
    }
}

fn api_id_from_value(value: &Value) -> Option<String> {
    let tool = value.get("tool_name").and_then(Value::as_str)?;
    let api = value.get("api_name").and_then(Value::as_str)?;
    Some(format!("{tool}::{api}"))
}

fn canonical_api_id(raw: &str, available_api_ids: &[String]) -> Result<String, String> {
    let cleaned = raw.trim();
    for id in available_api_ids {
        if id.eq_ignore_ascii_case(cleaned) {
            return Ok(id.clone());
        }
    }
    if !cleaned.contains("::") {
        let mut matches = available_api_ids
            .iter()
            .filter(|id| {
                id.rsplit("::")
                    .next()
                    .is_some_and(|api| api.eq_ignore_ascii_case(cleaned))
            })
            .cloned()
            .collect::<Vec<_>>();
        matches.sort();
        matches.dedup();
        if matches.len() == 1 {
            return Ok(matches.remove(0));
        }
    }
    Err(format!(
        "selected API {raw:?} not present in available API list"
    ))
}

fn same_set(left: &[String], right: &[String]) -> bool {
    let a: BTreeSet<_> = left.iter().collect();
    let b: BTreeSet<_> = right.iter().collect();
    a == b
}

fn compact_json(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "null".to_string())
}

fn truncate(value: &str, max_chars: usize) -> String {
    let mut out = value.trim().chars().take(max_chars).collect::<String>();
    if value.trim().chars().count() > max_chars {
        out.push_str("...");
    }
    out
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

fn hash_from_bytes(input: &[u8]) -> Hash {
    Hash(Sha256::digest(input).into())
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
