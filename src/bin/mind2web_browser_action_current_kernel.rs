//! True-suite Mind2Web browser-action evidence helper.
//!
//! This helper consumes a Mind2Web-compatible offline webpage snapshot, asks an
//! external model through the local OpenAI-compatible proxy to choose the next
//! browser action, stores the snapshot/input/claim/evaluation as CAS evidence,
//! records a structural browser action in `ProposalTelemetry`, and submits a
//! signed WorkTx through current ChainTape. Mind2Web action accuracy is a
//! capability signal only; the liveness proof is the replayable ChainTape/CAS
//! path plus hash-bound browser action trace.

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
const HTML_PROMPT_CHAR_CAP: usize = 18_000;
const CANDIDATE_PROMPT_CAP: usize = 36;
const MISSING_BACKEND_NODE_ID: &str = "__missing_backend_node_id__";

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
struct Mind2WebSample {
    schema_version: String,
    sample_id: String,
    source_family: String,
    public_source: String,
    source_file: String,
    website: String,
    domain: String,
    subdomain: String,
    annotation_id: String,
    confirmed_task: String,
    action_index: usize,
    action_repr: String,
    cleaned_html: String,
    operation: Mind2WebOperation,
    pos_candidates: Vec<Mind2WebCandidate>,
    neg_candidates: Vec<Mind2WebCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Mind2WebOperation {
    op: String,
    #[serde(default)]
    original_op: Option<String>,
    #[serde(default)]
    value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Mind2WebCandidate {
    tag: String,
    attributes: String,
    backend_node_id: String,
    #[serde(default)]
    is_original_target: Option<bool>,
    #[serde(default)]
    is_top_level_target: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
struct Mind2WebInputCapsule {
    schema_version: &'static str,
    sample_id: String,
    source_family: String,
    public_source: String,
    source_file: String,
    website: String,
    domain: String,
    subdomain: String,
    annotation_id: String,
    confirmed_task: String,
    action_index: usize,
    page_snapshot_cid: String,
    page_snapshot_sha256: String,
    candidate_backend_node_ids: Vec<String>,
    target_backend_node_ids_sha256: String,
    expected_operation_sha256: String,
}

#[derive(Debug, Clone, Serialize)]
struct Mind2WebAnswerClaimCapsule {
    schema_version: &'static str,
    sample_id: String,
    model_returned: String,
    selected_backend_node_id: String,
    operation: String,
    value: Option<String>,
    rationale: String,
    rationale_len_chars: usize,
    prompt_sha256: String,
    provider_response_sha256: String,
    raw_provider_response_persisted: bool,
}

#[derive(Debug, Clone, Serialize)]
struct Mind2WebBrowserActionTraceCapsule {
    schema_version: &'static str,
    sample_id: String,
    input_capsule_cid: String,
    selected_backend_node_id: String,
    operation: String,
    value: Option<String>,
    selected_candidate_available: bool,
    action_trace_sha256: String,
}

#[derive(Debug, Clone, Serialize)]
struct Mind2WebEvaluationCapsule {
    schema_version: &'static str,
    sample_id: String,
    input_capsule_cid: String,
    answer_claim_capsule_cid: String,
    browser_action_trace_cid: String,
    expected_backend_node_ids: Vec<String>,
    selected_backend_node_id: String,
    selected_candidate_available: bool,
    target_match: bool,
    operation_match: bool,
    value_match: bool,
    exact_match: bool,
    benchmark_verdict: String,
    failure_class: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct Mind2WebFcTraceReport {
    schema_version: &'static str,
    run_id: String,
    family_id: &'static str,
    fc_blocks_seen: Vec<&'static str>,
    input_capsule_cid: String,
    answer_claim_capsule_cid: String,
    browser_action_trace_cid: String,
    evaluation_capsule_cid: String,
    proposal_telemetry_cid: String,
    work_tx_id: String,
    work_tx_landed: bool,
    selected_candidate_available: bool,
    browser_action_exact_match: bool,
    final_closure_possible: bool,
}

#[derive(Debug, Clone, Serialize)]
struct Mind2WebEvidenceManifest {
    schema_version: &'static str,
    run_id: String,
    model_requested: String,
    model_returned: String,
    llm_proxy_url: String,
    sample_id: String,
    source_family: String,
    public_source: String,
    source_file: String,
    prompt_sha256: String,
    provider_response_sha256: String,
    page_snapshot_cid: String,
    input_capsule_cid: String,
    answer_claim_capsule_cid: String,
    browser_action_trace_cid: String,
    evaluation_capsule_cid: String,
    proposal_telemetry_cid: String,
    work_tx_id: String,
    work_tx_landed: bool,
    selected_candidate_available: bool,
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
    selected_candidate_available: bool,
    exact_match: bool,
    constitutional_rejection: bool,
    kernel_invariant_failure: bool,
    model_task_failure: bool,
    infrastructure_failure: bool,
    website_schema_drift: bool,
}

#[derive(Debug, Clone)]
struct ParsedBrowserAction {
    selected_backend_node_id: String,
    operation: String,
    value: Option<String>,
    rationale: String,
}

fn usage() -> &'static str {
    "usage: mind2web_browser_action_current_kernel --runtime-repo <PATH> --cas <PATH> --run-id <ID> --constitution <constitution.md> --sample-json <PATH> --llm-proxy-url <URL> [--model <MODEL>] --out-dir <PATH>"
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
            eprintln!("mind2web_browser_action_current_kernel: {msg}");
            eprintln!("{}", usage());
            return ExitCode::from(2);
        }
    };
    match run(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("mind2web_browser_action_current_kernel: {err}");
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
    std::fs::create_dir_all(args.out_dir.join("page_snapshots"))
        .map_err(|e| format!("page snapshots dir: {e}"))?;

    let sample: Mind2WebSample = serde_json::from_slice(
        &std::fs::read(&args.sample_json).map_err(|e| format!("read sample json: {e}"))?,
    )
    .map_err(|e| format!("parse sample json: {e}"))?;
    validate_sample(&sample)?;

    let candidate_ids = candidate_backend_node_ids(&sample);
    let expected_ids = expected_backend_node_ids(&sample);
    let expected_operation = normalized_op(&sample.operation.op);
    let expected_value = sample.operation.value.as_deref().map(normalized_value);
    let page_snapshot_sha256 = sha256_hex(&sample.cleaned_html);
    let snapshot_path = args.out_dir.join("page_snapshots").join(format!(
        "{}_step_{}.html",
        sanitize_id_fragment(&sample.annotation_id),
        sample.action_index
    ));
    std::fs::write(&snapshot_path, sample.cleaned_html.as_bytes())
        .map_err(|e| format!("write page snapshot {}: {e}", snapshot_path.display()))?;
    let page_snapshot_cid = put_bytes(
        &args.cas,
        sample.cleaned_html.as_bytes(),
        ObjectType::EvidenceCapsule,
        "mind2web-page-snapshot",
        1,
        "turingosv4.true_suite.mind2web_page_snapshot_html.v1",
    )?;

    let input_capsule = Mind2WebInputCapsule {
        schema_version: "turingosv4.true_suite.mind2web_input_capsule.v1",
        sample_id: sample.sample_id.clone(),
        source_family: sample.source_family.clone(),
        public_source: sample.public_source.clone(),
        source_file: sample.source_file.clone(),
        website: sample.website.clone(),
        domain: sample.domain.clone(),
        subdomain: sample.subdomain.clone(),
        annotation_id: sample.annotation_id.clone(),
        confirmed_task: sample.confirmed_task.clone(),
        action_index: sample.action_index,
        page_snapshot_cid: page_snapshot_cid.hex(),
        page_snapshot_sha256: page_snapshot_sha256.clone(),
        candidate_backend_node_ids: candidate_ids.clone(),
        target_backend_node_ids_sha256: sha256_hex(expected_ids.join("\n")),
        expected_operation_sha256: sha256_hex(format!(
            "{}\n{}",
            expected_operation,
            expected_value.clone().unwrap_or_default()
        )),
    };
    write_pretty_json(
        &args
            .out_dir
            .join("input_capsules")
            .join("input_capsule.json"),
        &input_capsule,
    )?;
    let input_cid = put_json(
        &args.cas,
        &input_capsule,
        ObjectType::EvidenceCapsule,
        "mind2web-input",
        2,
        "turingosv4.true_suite.mind2web_input_capsule.v1",
    )?;

    let prompt = build_prompt(&sample, &input_cid, &page_snapshot_cid, &candidate_ids);
    let prompt_sha256 = sha256_hex(&prompt);
    let response = ResilientLLMClient::new(&args.llm_proxy_url, 180, 2)
        .generate(&GenerateRequest {
            model: args.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "You solve offline Mind2Web browser action-selection tasks. Return only strict JSON.".to_string(),
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
    let parsed = parse_browser_action(&response.content)?;
    let selected_candidate_available = candidate_ids.contains(&parsed.selected_backend_node_id);

    let answer_claim = Mind2WebAnswerClaimCapsule {
        schema_version: "turingosv4.true_suite.mind2web_answer_claim_capsule.v1",
        sample_id: sample.sample_id.clone(),
        model_returned: response.model.clone(),
        selected_backend_node_id: parsed.selected_backend_node_id.clone(),
        operation: parsed.operation.clone(),
        value: parsed.value.clone(),
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
        "mind2web-answer-claim",
        3,
        "turingosv4.true_suite.mind2web_answer_claim_capsule.v1",
    )?;

    let browser_trace_seed = serde_json::json!({
        "sample_id": sample.sample_id,
        "selected_backend_node_id": parsed.selected_backend_node_id,
        "operation": parsed.operation,
        "value": parsed.value,
        "input_capsule_cid": input_cid.hex(),
    });
    let browser_trace_seed_bytes = serde_json::to_vec(&browser_trace_seed)
        .map_err(|e| format!("serialize browser trace seed: {e}"))?;
    let action_trace_sha256 = sha256_hex(&browser_trace_seed_bytes);
    let browser_trace = Mind2WebBrowserActionTraceCapsule {
        schema_version: "turingosv4.true_suite.mind2web_browser_action_trace.v1",
        sample_id: sample.sample_id.clone(),
        input_capsule_cid: input_cid.hex(),
        selected_backend_node_id: parsed.selected_backend_node_id.clone(),
        operation: parsed.operation.clone(),
        value: parsed.value.clone(),
        selected_candidate_available,
        action_trace_sha256: action_trace_sha256.clone(),
    };
    write_pretty_json(
        &args
            .out_dir
            .join("input_capsules")
            .join("browser_action_trace.json"),
        &browser_trace,
    )?;
    let browser_trace_cid = put_json(
        &args.cas,
        &browser_trace,
        ObjectType::ProposalPayload,
        "mind2web-browser-action-trace",
        4,
        "turingosv4.true_suite.mind2web_browser_action_trace.v1",
    )?;

    let target_match = expected_ids.contains(&parsed.selected_backend_node_id);
    let operation_match = normalized_op(&parsed.operation) == expected_operation;
    let value_match = match expected_value.as_ref() {
        Some(expected) => parsed
            .value
            .as_deref()
            .map(normalized_value)
            .is_some_and(|got| got == *expected),
        None => true,
    };
    let exact_match =
        selected_candidate_available && target_match && operation_match && value_match;
    let benchmark_verdict = if exact_match {
        "exact_browser_action_match"
    } else {
        "browser_action_mismatch"
    };
    let failure_class = (!exact_match).then(|| "model_task_failure".to_string());
    let evaluation = Mind2WebEvaluationCapsule {
        schema_version: "turingosv4.true_suite.mind2web_evaluation_capsule.v1",
        sample_id: sample.sample_id.clone(),
        input_capsule_cid: input_cid.hex(),
        answer_claim_capsule_cid: answer_claim_cid.hex(),
        browser_action_trace_cid: browser_trace_cid.hex(),
        expected_backend_node_ids: expected_ids.clone(),
        selected_backend_node_id: parsed.selected_backend_node_id.clone(),
        selected_candidate_available,
        target_match,
        operation_match,
        value_match,
        exact_match,
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
        "mind2web-evaluation",
        5,
        "turingosv4.true_suite.mind2web_evaluation_capsule.v1",
    )?;

    let proposal_telemetry_cid = {
        let tool_call = ToolCallRecord {
            tool_id: format!("browser_sandbox::{}", normalized_op(&parsed.operation)),
            args_hash: hash_from_bytes(browser_trace_seed_bytes.as_slice()),
            result_hash: hash_from_bytes(action_trace_sha256.as_bytes()),
        };
        let mut telemetry = ProposalTelemetry::new_root(
            AgentId(SOLVER_AGENT.to_string()),
            hash_from_hex_digest(&prompt_sha256)?,
            evaluation_cid,
            "mind2web_browser_action_selection".to_string(),
            TokenCounts {
                prompt_tokens: response.prompt_tokens as u64,
                completion_tokens: response.completion_tokens as u64,
                tool_tokens: 1,
            },
            format!("{SOLVER_AGENT}.mind2web.b0"),
        );
        telemetry.tool_calls = vec![tool_call];
        let mut cas = CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
        write_proposal_telemetry_to_cas(&mut cas, &telemetry, "mind2web-proposal-telemetry", 6)
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
        .map_err(|e| format!("fresh Mind2Web boot failed: {e}"))?;
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

    let task = format!("mind2web:{}", sanitize_id_fragment(&sample.annotation_id));
    let initial_root = seq
        .q_snapshot()
        .map_err(|e| format!("q_snapshot initial: {e:?}"))?
        .state_root_t;
    let task_open = make_real_task_open_signed_by(
        &mut keypairs,
        &task,
        SPONSOR_AGENT,
        initial_root,
        "true-suite-mind2web",
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
        "true-suite-mind2web",
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
        "true-suite-mind2web",
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
        .map_err(|e| format!("Mind2Web chaintape shutdown failed: {e}"))?;
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
        schema_version: "turingosv4.true_suite.mind2web_failure_taxonomy.v1",
        run_id: args.run_id.clone(),
        family_id: "mind2web_open_web",
        failure_class: failure_class.clone(),
        selected_candidate_available,
        exact_match,
        constitutional_rejection: false,
        kernel_invariant_failure: false,
        model_task_failure: !exact_match,
        infrastructure_failure: false,
        website_schema_drift: false,
    };
    write_pretty_json(&args.out_dir.join("failure_taxonomy.json"), &taxonomy)?;

    let fc_trace = Mind2WebFcTraceReport {
        schema_version: "turingosv4.true_suite.mind2web_fc_trace_report.v1",
        run_id: args.run_id.clone(),
        family_id: "mind2web_open_web",
        fc_blocks_seen: vec!["FC1", "FC3"],
        input_capsule_cid: input_cid.hex(),
        answer_claim_capsule_cid: answer_claim_cid.hex(),
        browser_action_trace_cid: browser_trace_cid.hex(),
        evaluation_capsule_cid: evaluation_cid.hex(),
        proposal_telemetry_cid: proposal_telemetry_cid.hex(),
        work_tx_id: work_tx_id.clone(),
        work_tx_landed,
        selected_candidate_available,
        browser_action_exact_match: exact_match,
        final_closure_possible: false,
    };
    write_pretty_json(&args.out_dir.join("fc_trace_report.json"), &fc_trace)?;

    let manifest = Mind2WebEvidenceManifest {
        schema_version: "turingosv4.true_suite.mind2web_browser_action.v1",
        run_id: args.run_id.clone(),
        model_requested: args.model,
        model_returned: response.model,
        llm_proxy_url: args.llm_proxy_url,
        sample_id: sample.sample_id,
        source_family: sample.source_family,
        public_source: sample.public_source,
        source_file: sample.source_file,
        prompt_sha256,
        provider_response_sha256,
        page_snapshot_cid: page_snapshot_cid.hex(),
        input_capsule_cid: input_cid.hex(),
        answer_claim_capsule_cid: answer_claim_cid.hex(),
        browser_action_trace_cid: browser_trace_cid.hex(),
        evaluation_capsule_cid: evaluation_cid.hex(),
        proposal_telemetry_cid: proposal_telemetry_cid.hex(),
        work_tx_id,
        work_tx_landed,
        selected_candidate_available,
        exact_match,
        benchmark_verdict: benchmark_verdict.to_string(),
        final_state_root_hex: hash_hex(&after_work),
        runtime_repo: args.runtime_repo.display().to_string(),
        cas: args.cas.display().to_string(),
        notes: vec![
            "Mind2Web webpage snapshot is hashed into CAS before the model call",
            "the hidden positive candidate and action_repr are not included in the model prompt",
            "DeepSeek/SiliconFlow access is outside the kernel through the local LLM proxy",
            "raw prompt and raw provider response are not written to evidence",
            "browser action is structural offline snapshot evidence, not live website side effects",
            "Mind2Web action accuracy is a capability signal only, not OBL-005 closure",
        ],
    };
    write_pretty_json(
        &args.out_dir.join("mind2web_browser_action_manifest.json"),
        &manifest,
    )?;

    println!(
        "mind2web_browser_action_current_kernel: work_tx_id={} verdict={} exact_match={} manifest={}",
        manifest.work_tx_id,
        manifest.benchmark_verdict,
        manifest.exact_match,
        args.out_dir
            .join("mind2web_browser_action_manifest.json")
            .display()
    );
    Ok(())
}

fn validate_sample(sample: &Mind2WebSample) -> Result<(), String> {
    if sample.confirmed_task.trim().is_empty() {
        return Err("Mind2Web confirmed_task is empty".to_string());
    }
    if sample.cleaned_html.trim().is_empty() {
        return Err("Mind2Web cleaned_html is empty".to_string());
    }
    if sample.pos_candidates.is_empty() {
        return Err("Mind2Web sample has no positive candidates".to_string());
    }
    if candidate_backend_node_ids(sample).is_empty() {
        return Err("Mind2Web sample yielded no candidate backend_node_ids".to_string());
    }
    Ok(())
}

fn build_prompt(
    sample: &Mind2WebSample,
    input_cid: &Cid,
    page_snapshot_cid: &Cid,
    candidate_ids: &[String],
) -> String {
    let candidates = candidate_summaries(sample, CANDIDATE_PROMPT_CAP);
    format!(
        "Mind2Web sample id: {}\nInput capsule cid: {}\nPage snapshot cid: {}\nWebsite: {} / {} / {}\nTask:\n{}\n\nCandidate backend_node_ids:\n{}\n\nCandidate summaries:\n{}\n\nCleaned HTML snapshot excerpt:\n{}\n\nReturn strict JSON only with fields:\n  backend_node_id: exact id from the candidate list\n  operation: one of CLICK, SELECT, TYPE, HOVER, ENTER, OTHER\n  value: string or null\n  rationale: 2-5 sentences explaining why the selected element/action is the next step.\nDo not include hidden labels. Do not claim live website side effects.",
        sample.sample_id,
        input_cid.hex(),
        page_snapshot_cid.hex(),
        sample.website,
        sample.domain,
        sample.subdomain,
        sample.confirmed_task,
        candidate_ids.join(", "),
        candidates,
        truncate(&sample.cleaned_html, HTML_PROMPT_CHAR_CAP)
    )
}

fn candidate_summaries(sample: &Mind2WebSample, cap: usize) -> String {
    let mut rows = Vec::new();
    for candidate in sample
        .pos_candidates
        .iter()
        .chain(sample.neg_candidates.iter())
        .take(cap)
    {
        let attrs = parse_candidate_attrs(&candidate.attributes);
        let label = attrs
            .get("aria_label")
            .or_else(|| attrs.get("name"))
            .or_else(|| attrs.get("id"))
            .or_else(|| attrs.get("placeholder"))
            .and_then(Value::as_str)
            .unwrap_or("");
        let class = attrs.get("class").and_then(Value::as_str).unwrap_or("");
        rows.push(format!(
            "- backend_node_id={} tag={} label={} class={}",
            candidate.backend_node_id,
            candidate.tag,
            truncate(label, 120),
            truncate(class, 120)
        ));
    }
    rows.join("\n")
}

fn parse_browser_action(content: &str) -> Result<ParsedBrowserAction, String> {
    let value = extract_json_object(content)?;
    let selected_backend_node_id = value
        .get("backend_node_id")
        .or_else(|| value.get("selected_backend_node_id"))
        .or_else(|| value.get("target_backend_node_id"))
        .and_then(Value::as_str)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| MISSING_BACKEND_NODE_ID.to_string());
    let operation = value
        .get("operation")
        .or_else(|| value.get("op"))
        .and_then(Value::as_str)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "OTHER".to_string());
    let value_field = value
        .get("value")
        .and_then(Value::as_str)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let rationale = value
        .get("rationale")
        .or_else(|| value.get("reasoning"))
        .or_else(|| value.get("explanation"))
        .and_then(Value::as_str)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| {
            "external agent returned parseable JSON without a rationale".to_string()
        });
    Ok(ParsedBrowserAction {
        selected_backend_node_id,
        operation,
        value: value_field,
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

fn expected_backend_node_ids(sample: &Mind2WebSample) -> Vec<String> {
    let mut ids = sample
        .pos_candidates
        .iter()
        .map(|c| c.backend_node_id.clone())
        .filter(|id| !id.trim().is_empty())
        .collect::<Vec<_>>();
    ids.sort();
    ids.dedup();
    ids
}

fn candidate_backend_node_ids(sample: &Mind2WebSample) -> Vec<String> {
    let mut ids = sample
        .pos_candidates
        .iter()
        .chain(sample.neg_candidates.iter())
        .map(|c| c.backend_node_id.clone())
        .filter(|id| !id.trim().is_empty())
        .collect::<Vec<_>>();
    ids.sort();
    ids.dedup();
    ids
}

fn parse_candidate_attrs(raw: &str) -> Value {
    serde_json::from_str(raw).unwrap_or(Value::Null)
}

fn normalized_op(raw: &str) -> String {
    raw.trim().to_ascii_uppercase()
}

fn normalized_value(raw: &str) -> String {
    raw.trim().to_ascii_lowercase()
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
