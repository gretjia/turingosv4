//! True-suite WebArena web-agent evidence helper.
//!
//! This helper consumes a WebArena-compatible browser task plus an offline
//! observation snapshot, asks an external model through the local
//! OpenAI-compatible proxy to produce a final answer and browser action, stores
//! the task/observation/claim/evaluation as CAS evidence, records a structural
//! browser tool call in `ProposalTelemetry`, and submits a signed WorkTx through
//! current ChainTape. WebArena exact match is a capability signal only; the
//! liveness proof is the replayable ChainTape/CAS path plus hash-bound browser
//! action trace.

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
const OBSERVATION_PROMPT_CHAR_CAP: usize = 18_000;

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
struct WebArenaSample {
    schema_version: String,
    sample_id: String,
    source_family: String,
    public_source: String,
    source_file: String,
    task_id: String,
    intent: String,
    start_url: String,
    #[serde(default)]
    sites: Vec<String>,
    #[serde(default)]
    observation_html: String,
    #[serde(default)]
    observation_text: String,
    #[serde(default)]
    allowed_tools: Vec<String>,
    reference_answer: String,
}

#[derive(Debug, Clone, Serialize)]
struct WebArenaObservationCapsule {
    schema_version: &'static str,
    sample_id: String,
    task_id: String,
    observation_sha256: String,
    observation_kind: &'static str,
    start_url: String,
    sites: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct WebArenaTaskCapsule {
    schema_version: &'static str,
    sample_id: String,
    source_family: String,
    public_source: String,
    source_file: String,
    task_id: String,
    intent: String,
    start_url: String,
    sites: Vec<String>,
    allowed_tools: Vec<String>,
    observation_capsule_cid: String,
    reference_answer_sha256: String,
}

#[derive(Debug, Clone, Serialize)]
struct WebArenaAnswerClaimCapsule {
    schema_version: &'static str,
    sample_id: String,
    model_returned: String,
    final_answer: String,
    browser_action: String,
    rationale: String,
    rationale_len_chars: usize,
    prompt_sha256: String,
    provider_response_sha256: String,
    raw_provider_response_persisted: bool,
}

#[derive(Debug, Clone, Serialize)]
struct WebArenaBrowserActionTraceCapsule {
    schema_version: &'static str,
    sample_id: String,
    task_capsule_cid: String,
    observation_capsule_cid: String,
    browser_action: String,
    action_trace_sha256: String,
}

#[derive(Debug, Clone, Serialize)]
struct WebArenaEvaluationCapsule {
    schema_version: &'static str,
    sample_id: String,
    task_capsule_cid: String,
    observation_capsule_cid: String,
    answer_claim_capsule_cid: String,
    browser_action_trace_cid: String,
    reference_answer_sha256: String,
    predicted_answer: String,
    rationale_guard_passed: bool,
    answer_correct: bool,
    benchmark_verdict: String,
    failure_class: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct WebArenaEvidenceManifest {
    schema_version: &'static str,
    run_id: String,
    model_requested: String,
    model_returned: String,
    llm_proxy_url: String,
    sample_id: String,
    source_family: String,
    public_source: String,
    source_file: String,
    task_id: String,
    prompt_sha256: String,
    provider_response_sha256: String,
    observation_capsule_cid: String,
    task_capsule_cid: String,
    answer_claim_capsule_cid: String,
    browser_action_trace_cid: String,
    evaluation_capsule_cid: String,
    proposal_telemetry_cid: String,
    work_tx_id: String,
    work_tx_landed: bool,
    rationale_guard_passed: bool,
    answer_correct: bool,
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
struct WebArenaFcTraceReport {
    schema_version: &'static str,
    run_id: String,
    family_id: &'static str,
    fc_blocks_seen: Vec<&'static str>,
    observation_capsule_cid: String,
    task_capsule_cid: String,
    answer_claim_capsule_cid: String,
    browser_action_trace_cid: String,
    evaluation_capsule_cid: String,
    proposal_telemetry_cid: String,
    work_tx_id: String,
    work_tx_landed: bool,
    answer_correct: bool,
    closure_scope: &'static str,
    full_system_participation_required: bool,
    final_closure_possible: bool,
}

#[derive(Debug, Clone, Serialize)]
struct FailureTaxonomy {
    schema_version: &'static str,
    run_id: String,
    family_id: &'static str,
    failure_class: Option<String>,
    rationale_guard_passed: bool,
    answer_correct: bool,
    constitutional_rejection: bool,
    kernel_invariant_failure: bool,
    model_task_failure: bool,
    infrastructure_failure: bool,
    browser_state_drift: bool,
}

#[derive(Debug)]
struct ParsedAnswerClaim {
    final_answer: String,
    browser_action: String,
    rationale: String,
}

fn usage() -> &'static str {
    "usage: webarena_web_agent_current_kernel --runtime-repo <PATH> --cas <PATH> --run-id <ID> --constitution <constitution.md> --sample-json <PATH> --llm-proxy-url <URL> [--model <MODEL>] --out-dir <PATH>"
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
            eprintln!("webarena_web_agent_current_kernel: {msg}");
            eprintln!("{}", usage());
            return ExitCode::from(2);
        }
    };
    match run(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("webarena_web_agent_current_kernel: {err}");
            ExitCode::from(1)
        }
    }
}

async fn run(args: Args) -> Result<(), String> {
    std::fs::create_dir_all(&args.runtime_repo).map_err(|e| format!("runtime repo dir: {e}"))?;
    std::fs::create_dir_all(&args.cas).map_err(|e| format!("cas dir: {e}"))?;
    std::fs::create_dir_all(&args.out_dir).map_err(|e| format!("out dir: {e}"))?;
    std::fs::create_dir_all(args.out_dir.join("input_capsules"))
        .map_err(|e| format!("input capsules dir: {e}"))?;
    std::fs::create_dir_all(args.out_dir.join("browser_traces"))
        .map_err(|e| format!("browser traces dir: {e}"))?;

    let sample: WebArenaSample = serde_json::from_slice(
        &std::fs::read(&args.sample_json).map_err(|e| format!("read sample json: {e}"))?,
    )
    .map_err(|e| format!("parse sample json: {e}"))?;
    validate_sample(&sample)?;

    let observation = visible_observation(&sample);
    let observation_sha256 = sha256_hex(&observation);
    let observation_path = args.out_dir.join("browser_traces").join(format!(
        "{}_observation.html",
        sanitize_id_fragment(&sample.task_id)
    ));
    std::fs::write(&observation_path, observation.as_bytes()).map_err(|e| {
        format!(
            "write observation snapshot {}: {e}",
            observation_path.display()
        )
    })?;
    let observation_capsule = WebArenaObservationCapsule {
        schema_version: "turingosv4.true_suite.webarena_observation_capsule.v1",
        sample_id: sample.sample_id.clone(),
        task_id: sample.task_id.clone(),
        observation_sha256: observation_sha256.clone(),
        observation_kind: observation_kind(&sample),
        start_url: sample.start_url.clone(),
        sites: sample.sites.clone(),
    };
    let observation_cid = put_json(
        &args.cas,
        &observation_capsule,
        ObjectType::EvidenceCapsule,
        "webarena-observation",
        1,
        "turingosv4.true_suite.webarena_observation_capsule.v1",
    )?;
    let observation_blob_cid = put_bytes(
        &args.cas,
        observation.as_bytes(),
        ObjectType::EvidenceCapsule,
        "webarena-observation-html",
        2,
        "turingosv4.true_suite.webarena_observation_html.v1",
    )?;

    let allowed_tools = if sample.allowed_tools.is_empty() {
        vec!["browser_sandbox".to_string()]
    } else {
        sample.allowed_tools.clone()
    };
    let task_capsule = WebArenaTaskCapsule {
        schema_version: "turingosv4.true_suite.webarena_task_capsule.v1",
        sample_id: sample.sample_id.clone(),
        source_family: sample.source_family.clone(),
        public_source: sample.public_source.clone(),
        source_file: sample.source_file.clone(),
        task_id: sample.task_id.clone(),
        intent: sample.intent.clone(),
        start_url: sample.start_url.clone(),
        sites: sample.sites.clone(),
        allowed_tools: allowed_tools.clone(),
        observation_capsule_cid: observation_cid.hex(),
        reference_answer_sha256: sha256_hex(&sample.reference_answer),
    };
    write_pretty_json(
        &args
            .out_dir
            .join("input_capsules")
            .join("task_capsule.json"),
        &task_capsule,
    )?;
    let task_cid = put_json(
        &args.cas,
        &task_capsule,
        ObjectType::EvidenceCapsule,
        "webarena-task",
        3,
        "turingosv4.true_suite.webarena_task_capsule.v1",
    )?;

    let prompt = build_prompt(
        &sample,
        &allowed_tools,
        &task_cid,
        &observation_cid,
        &observation_blob_cid,
        &observation,
    );
    let prompt_sha256 = sha256_hex(&prompt);
    let response = ResilientLLMClient::new(&args.llm_proxy_url, 180, 2)
        .generate(&GenerateRequest {
            model: args.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "You solve offline WebArena browser tasks from a visible observation snapshot. Return only strict JSON.".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            temperature: Some(0.0),
            max_tokens: Some(1000),
        })
        .await
        .map_err(|e| format!("llm proxy generation failed: {e}"))?;
    let provider_response_sha256 = sha256_hex(&response.content);
    let parsed = parse_answer_claim(&response.content)?;
    let rationale_guard_passed = parsed.rationale.trim().chars().count() >= MIN_RATIONALE_CHARS;
    if !rationale_guard_passed {
        return Err(format!(
            "WebArena answer rejected before WorkTx: rationale too short ({} chars, need >= {MIN_RATIONALE_CHARS})",
            parsed.rationale.trim().chars().count()
        ));
    }

    let answer_claim = WebArenaAnswerClaimCapsule {
        schema_version: "turingosv4.true_suite.webarena_answer_claim_capsule.v1",
        sample_id: sample.sample_id.clone(),
        model_returned: response.model.clone(),
        final_answer: parsed.final_answer.clone(),
        browser_action: parsed.browser_action.clone(),
        rationale: parsed.rationale.clone(),
        rationale_len_chars: parsed.rationale.trim().chars().count(),
        prompt_sha256: prompt_sha256.clone(),
        provider_response_sha256: provider_response_sha256.clone(),
        raw_provider_response_persisted: false,
    };
    write_pretty_json(
        &args
            .out_dir
            .join("input_capsules")
            .join("answer_claim.json"),
        &answer_claim,
    )?;
    let answer_claim_cid = put_json(
        &args.cas,
        &answer_claim,
        ObjectType::EvidenceCapsule,
        "webarena-answer-claim",
        4,
        "turingosv4.true_suite.webarena_answer_claim_capsule.v1",
    )?;

    let browser_trace_seed = serde_json::json!({
        "sample_id": sample.sample_id,
        "task_capsule_cid": task_cid.hex(),
        "observation_capsule_cid": observation_cid.hex(),
        "browser_action": parsed.browser_action,
        "final_answer": parsed.final_answer,
    });
    let browser_trace_seed_bytes = serde_json::to_vec(&browser_trace_seed)
        .map_err(|e| format!("serialize browser trace seed: {e}"))?;
    let action_trace_sha256 = sha256_hex(&browser_trace_seed_bytes);
    let browser_trace = WebArenaBrowserActionTraceCapsule {
        schema_version: "turingosv4.true_suite.webarena_browser_action_trace.v1",
        sample_id: sample.sample_id.clone(),
        task_capsule_cid: task_cid.hex(),
        observation_capsule_cid: observation_cid.hex(),
        browser_action: parsed.browser_action.clone(),
        action_trace_sha256: action_trace_sha256.clone(),
    };
    write_pretty_json(
        &args
            .out_dir
            .join("browser_traces")
            .join("browser_action_trace.json"),
        &browser_trace,
    )?;
    let browser_trace_cid = put_json(
        &args.cas,
        &browser_trace,
        ObjectType::ProposalPayload,
        "webarena-browser-action-trace",
        5,
        "turingosv4.true_suite.webarena_browser_action_trace.v1",
    )?;

    let answer_correct =
        normalize_answer(&parsed.final_answer) == normalize_answer(&sample.reference_answer);
    let benchmark_verdict = if answer_correct {
        "correct_with_browser_action"
    } else {
        "browser_task_answer_mismatch"
    };
    let failure_class = (!answer_correct).then(|| "model_task_failure".to_string());
    let evaluation = WebArenaEvaluationCapsule {
        schema_version: "turingosv4.true_suite.webarena_evaluation_capsule.v1",
        sample_id: sample.sample_id.clone(),
        task_capsule_cid: task_cid.hex(),
        observation_capsule_cid: observation_cid.hex(),
        answer_claim_capsule_cid: answer_claim_cid.hex(),
        browser_action_trace_cid: browser_trace_cid.hex(),
        reference_answer_sha256: sha256_hex(&sample.reference_answer),
        predicted_answer: parsed.final_answer.clone(),
        rationale_guard_passed,
        answer_correct,
        benchmark_verdict: benchmark_verdict.to_string(),
        failure_class: failure_class.clone(),
    };
    write_pretty_json(
        &args.out_dir.join("input_capsules").join("evaluation.json"),
        &evaluation,
    )?;
    let evaluation_cid = put_json(
        &args.cas,
        &evaluation,
        ObjectType::ProposalPayload,
        "webarena-evaluation",
        6,
        "turingosv4.true_suite.webarena_evaluation_capsule.v1",
    )?;

    let proposal_telemetry_cid = {
        let tool_call = ToolCallRecord {
            tool_id: "browser_sandbox::answer_from_visible_observation".to_string(),
            args_hash: hash_from_bytes(browser_trace_seed_bytes.as_slice()),
            result_hash: hash_from_bytes(action_trace_sha256.as_bytes()),
        };
        let mut telemetry = ProposalTelemetry::new_root(
            AgentId(SOLVER_AGENT.to_string()),
            hash_from_hex_digest(&prompt_sha256)?,
            evaluation_cid,
            "webarena_web_agent_browser_task".to_string(),
            TokenCounts {
                prompt_tokens: response.prompt_tokens as u64,
                completion_tokens: response.completion_tokens as u64,
                tool_tokens: 1,
            },
            format!("{SOLVER_AGENT}.webarena.b0"),
        );
        telemetry.tool_calls = vec![tool_call];
        let mut cas = CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
        write_proposal_telemetry_to_cas(&mut cas, &telemetry, "webarena-proposal-telemetry", 7)
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
        .map_err(|e| format!("fresh WebArena boot failed: {e}"))?;
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

    let task = format!("webarena:{}", sanitize_id_fragment(&sample.task_id));
    let initial_root = seq
        .q_snapshot()
        .map_err(|e| format!("q_snapshot initial: {e:?}"))?
        .state_root_t;
    let task_open = make_real_task_open_signed_by(
        &mut keypairs,
        &task,
        SPONSOR_AGENT,
        initial_root,
        "true-suite-webarena",
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
        "true-suite-webarena",
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
        "true-suite-webarena",
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
        .map_err(|e| format!("WebArena chaintape shutdown failed: {e}"))?;
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
        schema_version: "turingosv4.true_suite.webarena_failure_taxonomy.v1",
        run_id: args.run_id.clone(),
        family_id: "webarena_web_agent",
        failure_class: failure_class.clone(),
        rationale_guard_passed,
        answer_correct,
        constitutional_rejection: false,
        kernel_invariant_failure: false,
        model_task_failure: !answer_correct,
        infrastructure_failure: false,
        browser_state_drift: false,
    };
    write_pretty_json(&failure_taxonomy_path, &taxonomy)?;

    let fc_trace = WebArenaFcTraceReport {
        schema_version: "turingosv4.true_suite.webarena_fc_trace_report.v1",
        run_id: args.run_id.clone(),
        family_id: "webarena_web_agent",
        fc_blocks_seen: vec!["FC1", "FC2", "FC3"],
        observation_capsule_cid: observation_cid.hex(),
        task_capsule_cid: task_cid.hex(),
        answer_claim_capsule_cid: answer_claim_cid.hex(),
        browser_action_trace_cid: browser_trace_cid.hex(),
        evaluation_capsule_cid: evaluation_cid.hex(),
        proposal_telemetry_cid: proposal_telemetry_cid.hex(),
        work_tx_id: work_tx_id.clone(),
        work_tx_landed,
        answer_correct,
        closure_scope: "domain_adapter_smoke_only",
        full_system_participation_required: true,
        final_closure_possible: false,
    };
    write_pretty_json(&args.out_dir.join("fc_trace_report.json"), &fc_trace)?;

    let manifest = WebArenaEvidenceManifest {
        schema_version: "turingosv4.true_suite.webarena_web_agent.v1",
        run_id: args.run_id.clone(),
        model_requested: args.model,
        model_returned: response.model,
        llm_proxy_url: args.llm_proxy_url,
        sample_id: sample.sample_id,
        source_family: sample.source_family,
        public_source: sample.public_source,
        source_file: sample.source_file,
        task_id: sample.task_id,
        prompt_sha256,
        provider_response_sha256,
        observation_capsule_cid: observation_cid.hex(),
        task_capsule_cid: task_cid.hex(),
        answer_claim_capsule_cid: answer_claim_cid.hex(),
        browser_action_trace_cid: browser_trace_cid.hex(),
        evaluation_capsule_cid: evaluation_cid.hex(),
        proposal_telemetry_cid: proposal_telemetry_cid.hex(),
        work_tx_id,
        work_tx_landed,
        rationale_guard_passed,
        answer_correct,
        benchmark_verdict: benchmark_verdict.to_string(),
        closure_scope: "domain_adapter_smoke_only",
        full_system_participation_required: true,
        final_closure_possible: false,
        failure_taxonomy_path: failure_taxonomy_path.display().to_string(),
        final_state_root_hex: hash_hex(&after_work),
        runtime_repo: args.runtime_repo.display().to_string(),
        cas: args.cas.display().to_string(),
        notes: vec![
            "WebArena task and visible observation are hashed into CAS before the model call",
            "the hidden reference answer is evaluation-only and not included in the model prompt",
            "DeepSeek/SiliconFlow access is outside the kernel through the local LLM proxy",
            "raw prompt and raw provider response are not written to evidence",
            "browser action is structural offline observation evidence, not live website side effects",
            "WebArena exact-match accuracy is a capability signal only, not OBL-005 closure",
        ],
    };
    write_pretty_json(
        &args.out_dir.join("webarena_web_agent_manifest.json"),
        &manifest,
    )?;

    println!(
        "webarena_web_agent_current_kernel: work_tx_id={} verdict={} correct={} manifest={}",
        manifest.work_tx_id,
        manifest.benchmark_verdict,
        manifest.answer_correct,
        args.out_dir
            .join("webarena_web_agent_manifest.json")
            .display()
    );
    Ok(())
}

fn validate_sample(sample: &WebArenaSample) -> Result<(), String> {
    if sample.intent.trim().is_empty() {
        return Err("WebArena intent is empty".to_string());
    }
    if sample.start_url.trim().is_empty() {
        return Err("WebArena start_url is empty".to_string());
    }
    if sample.reference_answer.trim().is_empty() {
        return Err("WebArena reference_answer is empty".to_string());
    }
    if visible_observation(sample).trim().is_empty() {
        return Err("WebArena visible observation is empty".to_string());
    }
    Ok(())
}

fn build_prompt(
    sample: &WebArenaSample,
    allowed_tools: &[String],
    task_cid: &Cid,
    observation_cid: &Cid,
    observation_blob_cid: &Cid,
    observation: &str,
) -> String {
    let sites = if sample.sites.is_empty() {
        "none declared".to_string()
    } else {
        sample.sites.join(", ")
    };
    format!(
        "WebArena sample id: {}\nTask capsule cid: {}\nObservation capsule cid: {}\nObservation blob cid: {}\nTask id: {}\nSites: {}\nStart URL: {}\nAllowed tools: {}\n\nIntent:\n{}\n\nVisible observation snapshot excerpt:\n{}\n\nReturn strict JSON only with fields:\n  final_answer: concise final answer string\n  browser_action: short description of the browser observation/action used\n  rationale: 2-5 sentences explaining the visible evidence used.\nUse only the visible observation snapshot. Do not claim live account or website side effects. Do not include hidden evaluation labels.",
        sample.sample_id,
        task_cid.hex(),
        observation_cid.hex(),
        observation_blob_cid.hex(),
        sample.task_id,
        sites,
        sample.start_url,
        allowed_tools.join(", "),
        sample.intent,
        truncate(observation, OBSERVATION_PROMPT_CHAR_CAP)
    )
}

fn parse_answer_claim(content: &str) -> Result<ParsedAnswerClaim, String> {
    let value = extract_json_object(content)?;
    let final_answer = value
        .get("final_answer")
        .or_else(|| value.get("answer"))
        .or_else(|| value.get("answer_text"))
        .and_then(Value::as_str)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or("external agent JSON missing `final_answer`")?;
    let browser_action = value
        .get("browser_action")
        .or_else(|| value.get("action"))
        .or_else(|| value.get("browser_trace"))
        .and_then(Value::as_str)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or("external agent JSON missing `browser_action`")?;
    let rationale = value
        .get("rationale")
        .or_else(|| value.get("reasoning"))
        .or_else(|| value.get("explanation"))
        .and_then(Value::as_str)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or("external agent JSON missing `rationale`")?;
    Ok(ParsedAnswerClaim {
        final_answer,
        browser_action,
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

fn visible_observation(sample: &WebArenaSample) -> String {
    if !sample.observation_html.trim().is_empty() {
        sample.observation_html.clone()
    } else {
        sample.observation_text.clone()
    }
}

fn observation_kind(sample: &WebArenaSample) -> &'static str {
    if !sample.observation_html.trim().is_empty() {
        "html"
    } else {
        "text"
    }
}

fn normalize_answer(answer: &str) -> String {
    answer
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else if c.is_whitespace() {
                Some(' ')
            } else {
                None
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn truncate(value: &str, max_chars: usize) -> String {
    let trimmed = value.trim();
    let mut out = trimmed.chars().take(max_chars).collect::<String>();
    if trimmed.chars().count() > max_chars {
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
    put_bytes(cas_path, &bytes, object_type, creator, logical_t, schema_id)
}

fn put_bytes(
    cas_path: &PathBuf,
    bytes: &[u8],
    object_type: ObjectType,
    creator: &str,
    logical_t: u64,
    schema_id: &str,
) -> Result<Cid, String> {
    let mut cas = CasStore::open(cas_path).map_err(|e| format!("open CAS: {e}"))?;
    cas.put(
        bytes,
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
