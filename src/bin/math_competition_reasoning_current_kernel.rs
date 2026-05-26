//! True-suite MATH competition-reasoning evidence helper.
//!
//! This binary is a runner helper. It consumes a MATH-compatible sample file,
//! asks an external model through the local OpenAI-compatible proxy, stores the
//! problem/claim/evaluation as CAS evidence, then submits a signed WorkTx
//! through the current ChainTape sequencer. Benchmark accuracy is recorded as a
//! capability signal only; the liveness proof is the replayable ChainTape/CAS
//! path.

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
struct MathSample {
    schema_version: String,
    sample_id: String,
    source_family: String,
    public_source: String,
    source_file: String,
    subject: String,
    level: String,
    problem: String,
    solution: String,
    expected_answer: String,
    canary_string: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct MathProblemCapsule {
    schema_version: &'static str,
    sample_id: String,
    source_family: String,
    public_source: String,
    source_file: String,
    subject: String,
    level: String,
    problem: String,
    solution_sha256: String,
    expected_answer_sha256: String,
    canary_string_present: bool,
}

#[derive(Debug, Clone, Serialize)]
struct MathAnswerClaimCapsule {
    schema_version: &'static str,
    sample_id: String,
    model_returned: String,
    final_answer: String,
    normalized_final_answer: String,
    rationale: String,
    rationale_len_chars: usize,
    prompt_sha256: String,
    provider_response_sha256: String,
    raw_provider_response_persisted: bool,
}

#[derive(Debug, Clone, Serialize)]
struct MathEvaluationCapsule {
    schema_version: &'static str,
    sample_id: String,
    problem_capsule_cid: String,
    answer_claim_capsule_cid: String,
    expected_answer: String,
    predicted_answer: String,
    normalized_expected_answer: String,
    normalized_predicted_answer: String,
    rationale_guard_passed: bool,
    answer_correct: bool,
    benchmark_verdict: String,
    failure_class: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct MathEvidenceManifest {
    schema_version: &'static str,
    run_id: String,
    model_requested: String,
    model_returned: String,
    llm_proxy_url: String,
    sample_id: String,
    source_family: String,
    public_source: String,
    prompt_sha256: String,
    provider_response_sha256: String,
    problem_capsule_cid: String,
    answer_claim_capsule_cid: String,
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
}

fn usage() -> &'static str {
    "usage: math_competition_reasoning_current_kernel --runtime-repo <PATH> --cas <PATH> --run-id <ID> --constitution <constitution.md> --sample-json <PATH> --llm-proxy-url <URL> [--model <MODEL>] --out-dir <PATH>"
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
            eprintln!("math_competition_reasoning_current_kernel: {msg}");
            eprintln!("{}", usage());
            return ExitCode::from(2);
        }
    };
    match run(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("math_competition_reasoning_current_kernel: {err}");
            ExitCode::from(1)
        }
    }
}

async fn run(args: Args) -> Result<(), String> {
    std::fs::create_dir_all(&args.runtime_repo).map_err(|e| format!("runtime repo dir: {e}"))?;
    std::fs::create_dir_all(&args.cas).map_err(|e| format!("cas dir: {e}"))?;
    std::fs::create_dir_all(&args.out_dir).map_err(|e| format!("out dir: {e}"))?;

    let sample: MathSample = serde_json::from_slice(
        &std::fs::read(&args.sample_json).map_err(|e| format!("read sample json: {e}"))?,
    )
    .map_err(|e| format!("parse sample json: {e}"))?;
    validate_sample(&sample)?;

    let problem_capsule = MathProblemCapsule {
        schema_version: "turingosv4.true_suite.math_problem_capsule.v1",
        sample_id: sample.sample_id.clone(),
        source_family: sample.source_family.clone(),
        public_source: sample.public_source.clone(),
        source_file: sample.source_file.clone(),
        subject: sample.subject.clone(),
        level: sample.level.clone(),
        problem: sample.problem.clone(),
        solution_sha256: sha256_hex(&sample.solution),
        expected_answer_sha256: sha256_hex(&sample.expected_answer),
        canary_string_present: sample.canary_string.is_some(),
    };
    let problem_cid = put_json(
        &args.cas,
        &problem_capsule,
        ObjectType::EvidenceCapsule,
        "math-problem",
        1,
        "turingosv4.true_suite.math_problem_capsule.v1",
    )?;

    let prompt = build_prompt(&sample, &problem_cid);
    let prompt_sha256 = sha256_hex(&prompt);
    let response = ResilientLLMClient::new(&args.llm_proxy_url, 180, 2)
        .generate(&GenerateRequest {
            model: args.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "You solve MATH competition problems. Return only strict JSON."
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
    let parsed = parse_answer_claim(&response.content)?;
    let rationale_guard_passed = parsed.rationale.trim().chars().count() >= MIN_RATIONALE_CHARS;
    if !rationale_guard_passed {
        return Err(format!(
            "Math answer rejected before WorkTx: rationale too short ({} chars, need >= {MIN_RATIONALE_CHARS})",
            parsed.rationale.trim().chars().count()
        ));
    }

    let answer_claim = MathAnswerClaimCapsule {
        schema_version: "turingosv4.true_suite.math_answer_claim_capsule.v1",
        sample_id: sample.sample_id.clone(),
        model_returned: response.model.clone(),
        final_answer: parsed.final_answer.clone(),
        normalized_final_answer: normalize_math_answer(&parsed.final_answer),
        rationale: parsed.rationale.clone(),
        rationale_len_chars: parsed.rationale.trim().chars().count(),
        prompt_sha256: prompt_sha256.clone(),
        provider_response_sha256: provider_response_sha256.clone(),
        raw_provider_response_persisted: false,
    };
    let answer_claim_cid = put_json(
        &args.cas,
        &answer_claim,
        ObjectType::EvidenceCapsule,
        "math-answer-claim",
        2,
        "turingosv4.true_suite.math_answer_claim_capsule.v1",
    )?;

    let normalized_expected = normalize_math_answer(&sample.expected_answer);
    let normalized_predicted = normalize_math_answer(&parsed.final_answer);
    let answer_correct = normalized_predicted == normalized_expected;
    let benchmark_verdict = if answer_correct {
        "correct_with_rationale"
    } else {
        "incorrect_with_rationale"
    };
    let failure_class = (!answer_correct).then(|| "model_task_failure".to_string());
    let evaluation = MathEvaluationCapsule {
        schema_version: "turingosv4.true_suite.math_evaluation_capsule.v1",
        sample_id: sample.sample_id.clone(),
        problem_capsule_cid: problem_cid.hex(),
        answer_claim_capsule_cid: answer_claim_cid.hex(),
        expected_answer: sample.expected_answer.clone(),
        predicted_answer: parsed.final_answer.clone(),
        normalized_expected_answer: normalized_expected,
        normalized_predicted_answer: normalized_predicted,
        rationale_guard_passed,
        answer_correct,
        benchmark_verdict: benchmark_verdict.to_string(),
        failure_class: failure_class.clone(),
    };
    let evaluation_cid = put_json(
        &args.cas,
        &evaluation,
        ObjectType::ProposalPayload,
        "math-evaluation",
        3,
        "turingosv4.true_suite.math_evaluation_capsule.v1",
    )?;
    let proposal_telemetry_cid = {
        let telemetry = ProposalTelemetry::new_root(
            AgentId(SOLVER_AGENT.to_string()),
            hash_from_hex_digest(&prompt_sha256)?,
            evaluation_cid,
            "math_competition_reasoning".to_string(),
            TokenCounts {
                prompt_tokens: response.prompt_tokens as u64,
                completion_tokens: response.completion_tokens as u64,
                tool_tokens: 0,
            },
            format!("{SOLVER_AGENT}.math.b0"),
        );
        let mut cas = CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
        write_proposal_telemetry_to_cas(&mut cas, &telemetry, "math-proposal-telemetry", 4)
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
        .map_err(|e| format!("fresh Math boot failed: {e}"))?;
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

    let task = format!("math:{}", sanitize_id_fragment(&sample.sample_id));
    let initial_root = seq
        .q_snapshot()
        .map_err(|e| format!("q_snapshot initial: {e:?}"))?
        .state_root_t;
    let task_open = make_real_task_open_signed_by(
        &mut keypairs,
        &task,
        SPONSOR_AGENT,
        initial_root,
        "true-suite-math",
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
        "true-suite-math",
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
        "true-suite-math",
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
        .map_err(|e| format!("Math chaintape shutdown failed: {e}"))?;
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
        schema_version: "turingosv4.true_suite.math_failure_taxonomy.v1",
        run_id: args.run_id.clone(),
        family_id: "math_competition_reasoning",
        failure_class: failure_class.clone(),
        rationale_guard_passed,
        answer_correct,
        constitutional_rejection: false,
        kernel_invariant_failure: false,
        model_task_failure: !answer_correct,
        infrastructure_failure: false,
    };
    write_pretty_json(&failure_taxonomy_path, &taxonomy)?;

    let manifest = MathEvidenceManifest {
        schema_version: "turingosv4.true_suite.math_competition_reasoning.v1",
        run_id: args.run_id.clone(),
        model_requested: args.model,
        model_returned: response.model,
        llm_proxy_url: args.llm_proxy_url,
        sample_id: sample.sample_id,
        source_family: sample.source_family,
        public_source: sample.public_source,
        prompt_sha256,
        provider_response_sha256,
        problem_capsule_cid: problem_cid.hex(),
        answer_claim_capsule_cid: answer_claim_cid.hex(),
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
            "Math dataset input is hashed into CAS before the model call",
            "DeepSeek/SiliconFlow access is outside the kernel through the local LLM proxy",
            "raw prompt and raw provider response are not written to evidence",
            "parsed answer claim and benchmark evaluation are CAS capsules",
            "benchmark accuracy is a capability signal only, not OBL-005 closure",
        ],
    };
    write_pretty_json(
        &args
            .out_dir
            .join("math_competition_reasoning_manifest.json"),
        &manifest,
    )?;

    println!(
        "math_competition_reasoning_current_kernel: work_tx_id={} verdict={} correct={} manifest={}",
        manifest.work_tx_id,
        manifest.benchmark_verdict,
        manifest.answer_correct,
        args.out_dir
            .join("math_competition_reasoning_manifest.json")
            .display()
    );
    Ok(())
}

#[derive(Debug)]
struct ParsedAnswerClaim {
    final_answer: String,
    rationale: String,
}

fn parse_answer_claim(content: &str) -> Result<ParsedAnswerClaim, String> {
    let value = extract_json_object(content)?;
    let final_answer = value
        .get("final_answer")
        .or_else(|| value.get("answer"))
        .and_then(serde_json::Value::as_str)
        .ok_or("external agent JSON missing `final_answer`")?;
    let rationale = value
        .get("rationale")
        .or_else(|| value.get("reasoning"))
        .or_else(|| value.get("explanation"))
        .and_then(serde_json::Value::as_str)
        .ok_or("external agent JSON missing `rationale`")?;
    Ok(ParsedAnswerClaim {
        final_answer: final_answer.trim().to_string(),
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

fn validate_sample(sample: &MathSample) -> Result<(), String> {
    if sample.problem.trim().is_empty() {
        return Err("sample problem is empty".to_string());
    }
    if sample.expected_answer.trim().is_empty() {
        return Err("sample expected_answer is empty".to_string());
    }
    if sample.solution.trim().is_empty() {
        return Err("sample solution is empty".to_string());
    }
    Ok(())
}

fn build_prompt(sample: &MathSample, problem_cid: &Cid) -> String {
    format!(
        "MATH sample id: {}\nProblem capsule cid: {}\nSubject: {}\nLevel: {}\n\nProblem:\n{}\n\nReturn strict JSON only with fields:\n  final_answer: the final answer only, without explanatory prose\n  rationale: 3-8 sentences explaining the mathematical reasoning.\nDo not use web lookup. Do not include hidden solution text or benchmark metadata.",
        sample.sample_id,
        problem_cid.hex(),
        sample.subject,
        sample.level,
        sample.problem,
    )
}

fn normalize_math_answer(answer: &str) -> String {
    let trimmed = answer.trim();
    let unboxed = extract_boxed_answer(trimmed).unwrap_or(trimmed);
    unboxed
        .replace("\\left", "")
        .replace("\\right", "")
        .replace('$', "")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .trim_matches('.')
        .to_string()
}

fn extract_boxed_answer(answer: &str) -> Option<&str> {
    let marker = "\\boxed{";
    let start = answer.rfind(marker)? + marker.len();
    let mut depth = 1usize;
    for (offset, ch) in answer[start..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&answer[start..start + offset]);
                }
            }
            _ => {}
        }
    }
    None
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
