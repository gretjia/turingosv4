//! LIVE-FC1 — REAL Lean-oracle proof runner (thin orchestrator), the A5-closing leg.
//!
//! This is an UNPINNED runner bin (genesis pin-count 0; nothing in
//! `genesis_payload.toml` references this path). It drives a small swarm of REAL
//! LLM agents over a few EASY core Lean theorems through the local
//! OpenAI-compatible `llm_proxy.py`, on ONE shared canonical ChainTape, and — the
//! crux — uses the **REAL Lean kernel** (`LeanJudge`) as the external ground-truth
//! oracle. A kernel-VERIFIED proof produces:
//!
//!   1. a CAS `VerificationResult { verified: true }` (the Lean-oracle witness),
//!   2. a `ProposalTelemetry.verification_result_cid` LINKING that witness to the
//!      accepted WorkTx (so `reconstruct_vpput_from_tape` resolves `oracle_verified`),
//!   3. an accepted L4 WorkTx spine (TaskOpen → EscrowLock → WorkTx), and
//!   4. a `TerminalSummary { run_outcome = OmegaAccepted }` naming that task.
//!
//! Those four together are EXACTLY the two gates the Phase-2 VPPUT reconstruction
//! requires for `progress = 1` (omega_terminal AND oracle_verified), so a real
//! Lean-verified theorem yields a NON-ZERO `verified_pput_micro` on the canonical
//! tape — closing acceptance A5 (the prior math-only swarm run was honestly
//! progress=0 because math has no external oracle).
//!
//! HONESTY (binding): `progress = 1` lands ONLY when the Lean checker actually
//! returns success (`LeanOutcome::is_verified()`). A FAILED Lean attempt is
//! recorded as a REAL L4.E `LeanFailed` rejection (token-spent, cost counted), NOT
//! a verified path. If no proof verifies within the bounded attempt budget, the
//! task's progress stays 0 — no fabricated witness.
//!
//! BRAND-GENERIC: any provider identity on the canonical CAS is the Phase-6
//! brand-GENERIC `ProviderHandleCapsule` (opaque sha256 handle); the brand→handle
//! mapping lives ONLY in an external sidecar.
//!
//! Class 2 (new UNPINNED bin; reuses LeanJudge + the swarm canonical-spine
//! machinery + the Phase-1..6 observe-only mechanisms; no §6 surface).

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Instant;

use serde::Serialize;
use sha2::{Digest, Sha256};

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    RejectionClass, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::transition_ledger::{canonical_encode, TxKind};
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
use turingosv4::economy::money::MicroCoin;
use turingosv4::judges::lean_judge::default_lean_bin;
use turingosv4::judges::lean_theorem_bank::{load_bank, LeanTheorem};
use turingosv4::runtime::adapter::{
    genesis_with_balances, make_real_escrow_lock_signed_by, make_real_task_open_signed_by,
    make_real_verifytx_signed_by, make_real_worktx_signed_by, tb11_emit_terminal_summary_for_run,
    tb8_await_state_root_advance,
};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::agent_scheduler::budget_ceiling::{
    budget_check, loaded_tape_spend_tokens, BudgetManifest, BudgetVerdict,
};
use turingosv4::runtime::agent_scheduler::provider_handle_capsule::write_provider_handle_capsule;
use turingosv4::runtime::audit_assertions::{load_tape, AuditInputs, LoadedTape};
use turingosv4::runtime::bootstrap::default_pput_preseed_pairs;
use turingosv4::runtime::genesis_report::GenesisReport;
use turingosv4::runtime::proposal_telemetry::{
    write_to_cas as write_proposal_telemetry_to_cas, ProposalTelemetry, TokenCounts,
};
use turingosv4::runtime::verification_result::{
    write_to_cas as write_verification_result_to_cas, VerificationResult,
};
use turingosv4::runtime::{build_chaintape_sequencer_with_initial_q, RuntimeChaintapeConfig};
use turingosv4::state::q_state::{AgentId, Hash, TaskId, TxId};
use turingosv4::state::typed_tx::{RunId, RunOutcome, TypedTx};

const SPONSOR_AGENT: &str = "Agent_user_0";
const VERIFIER_AGENT: &str = "Agent_lean_verifier";
const TASK_ESCROW_MICRO: i64 = 10_000;
const WORK_STAKE_MICRO: i64 = 100;
const VERIFY_BOND_MICRO: i64 = 500;

#[derive(Debug)]
struct Args {
    runtime_repo: PathBuf,
    cas: PathBuf,
    run_id: String,
    constitution: PathBuf,
    genesis: PathBuf,
    bank: PathBuf,
    problems: Vec<String>,
    proxy_url: String,
    model: String,
    brand_provider: String,
    max_attempts: usize,
    budget_manifest: PathBuf,
    out_dir: PathBuf,
}

/// One brand-laden provider axis. `brand_model` routes through the proxy; it NEVER
/// lands on the canonical tape — only the opaque generic handle does.
#[derive(Debug, Clone, Serialize)]
struct AttemptResult {
    problem_id: String,
    task_id: Option<String>,
    attempt_index: usize,
    /// "verified" | "lean_failed" | "parse_fail" | "llm_err"
    outcome: String,
    verdict_kind: String,
    prompt_tokens: u64,
    completion_tokens: u64,
    body_preview: String,
    feedback: String,
    work_tx_id: Option<String>,
    l4e_submit_id: Option<u64>,
    proposal_telemetry_cid: Option<String>,
    verification_result_cid: Option<String>,
    verified: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ProblemResult {
    problem_id: String,
    task_id: String,
    needs_mathlib: bool,
    attempts: usize,
    verified: bool,
    omega_accepted_emitted: bool,
    total_tokens: u64,
    provider_handle: String,
}

#[derive(Debug, Clone, Serialize)]
struct RunSidecar {
    schema_version: &'static str,
    run_id: String,
    proxy_url: String,
    model_brand: String,
    provider_brand: String,
    lean_bin: String,
    budget_ceiling_micro_units: i64,
    final_spend_tokens: u64,
    wall_clock_s: f64,
    problems: Vec<ProblemResult>,
    attempts: Vec<AttemptResult>,
    brand_sidecar: Vec<BrandRow>,
    fc2_terminal_omega_accepted: bool,
    notes: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
struct BrandRow {
    agent_id: String,
    provider_handle: String,
    brand_model_name: String,
    brand_model_provider: String,
}

fn usage() -> &'static str {
    "usage: livefc1_lean_runner --runtime-repo <P> --cas <P> --run-id <ID> \
     --constitution <constitution.md> --genesis <genesis_payload.toml> \
     --bank <lean_theorems.jsonl> --problems <id,id,id> --proxy-url <URL> \
     --model <brand_model> [--brand-provider <name>] [--max-attempts <N>] \
     --budget-manifest <P> --out-dir <P>"
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut m: BTreeMap<&str, String> = BTreeMap::new();
    let keys = [
        "--runtime-repo",
        "--cas",
        "--run-id",
        "--constitution",
        "--genesis",
        "--bank",
        "--problems",
        "--proxy-url",
        "--model",
        "--brand-provider",
        "--max-attempts",
        "--budget-manifest",
        "--out-dir",
    ];
    let mut i = 0;
    while i < argv.len() {
        let k = argv[i].as_str();
        if k == "--help" || k == "-h" {
            return Err(usage().into());
        }
        let key = keys
            .iter()
            .find(|&&kk| kk == k)
            .ok_or_else(|| format!("unknown arg: {k}"))?;
        i += 1;
        let v = argv.get(i).ok_or_else(|| format!("missing value after {key}"))?;
        m.insert(key, v.clone());
        i += 1;
    }
    let get = |k: &str| m.get(k).cloned().ok_or_else(|| format!("{k} required"));
    let problems: Vec<String> = get("--problems")?
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if problems.is_empty() {
        return Err("--problems must list at least one theorem id".into());
    }
    Ok(Args {
        runtime_repo: get("--runtime-repo")?.into(),
        cas: get("--cas")?.into(),
        run_id: get("--run-id")?,
        constitution: get("--constitution")?.into(),
        genesis: get("--genesis")?.into(),
        bank: m
            .get("--bank")
            .cloned()
            .unwrap_or_else(|| "tests/fixtures/lean_theorems.jsonl".into())
            .into(),
        problems,
        proxy_url: m
            .get("--proxy-url")
            .cloned()
            .unwrap_or_else(|| "http://localhost:8123".into()),
        model: m.get("--model").cloned().unwrap_or_else(|| "deepseek-chat".into()),
        brand_provider: m
            .get("--brand-provider")
            .cloned()
            .unwrap_or_else(|| "DeepSeek".into()),
        max_attempts: m
            .get("--max-attempts")
            .and_then(|s| s.parse().ok())
            .unwrap_or(2),
        budget_manifest: get("--budget-manifest")?.into(),
        out_dir: get("--out-dir")?.into(),
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse_args(&argv) {
        Ok(a) => a,
        Err(msg) => {
            eprintln!("livefc1_lean_runner: {msg}");
            return ExitCode::from(2);
        }
    };
    match run(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("livefc1_lean_runner: {e}");
            ExitCode::from(1)
        }
    }
}

async fn run(args: Args) -> Result<(), String> {
    // Boot-trust-root verify (CWD holds genesis_payload.toml).
    let trust_root_repo = std::env::current_dir().map_err(|e| format!("cwd: {e}"))?;
    turingosv4::boot::verify_trust_root(&trust_root_repo)
        .map_err(|e| format!("TRUST_ROOT_TAMPERED: {e}"))?;
    let t0 = Instant::now();

    std::fs::create_dir_all(&args.runtime_repo).map_err(|e| format!("runtime repo dir: {e}"))?;
    std::fs::create_dir_all(&args.cas).map_err(|e| format!("cas dir: {e}"))?;
    std::fs::create_dir_all(&args.out_dir).map_err(|e| format!("out dir: {e}"))?;

    // ── Lean oracle + problem set ───────────────────────────────────────────
    let lean_bin = default_lean_bin();
    if !(lean_bin.is_absolute() && lean_bin.exists()) {
        return Err(format!(
            "Lean oracle not ready: pinned toolchain bin {} absent (no external oracle => HALT, no paid batch)",
            lean_bin.display()
        ));
    }
    let bank = load_bank(&args.bank)?;
    let mut problems: Vec<LeanTheorem> = Vec::new();
    for id in &args.problems {
        let thm = bank
            .iter()
            .find(|t| &t.id == id)
            .ok_or_else(|| format!("problem `{id}` not in bank {}", args.bank.display()))?
            .clone();
        if thm.needs_mathlib {
            return Err(format!(
                "problem `{id}` needs Mathlib; this bounded runner only drives CORE theorems (no Mathlib cache build)"
            ));
        }
        problems.push(thm);
    }

    // Phase-5 signed budget manifest (separate unpinned TOML; never genesis).
    let budget = BudgetManifest::from_file(&args.budget_manifest)
        .map_err(|e| format!("load budget manifest: {e}"))?;
    let ceiling = budget.ceiling_micro();
    println!(
        "livefc1_lean_runner: budget ceiling = {} micro-units (0 = unlimited); lean_bin = {}",
        ceiling.micro_units(),
        lean_bin.display()
    );

    // ── Boot ONE shared canonical chaintape ─────────────────────────────────
    let preseed = default_pput_preseed_pairs();
    let mut initial_q = genesis_with_balances(&preseed);
    for extra in [SPONSOR_AGENT, VERIFIER_AGENT] {
        initial_q
            .economic_state_t
            .balances_t
            .0
            .entry(AgentId(extra.to_string()))
            .or_insert(MicroCoin::from_micro_units(100_000_000));
    }
    let mut solver_agents: Vec<String> = Vec::new();
    for i in 0..problems.len().max(1) {
        let a = format!("Agent_{i}");
        initial_q
            .economic_state_t
            .balances_t
            .0
            .entry(AgentId(a.clone()))
            .or_insert(MicroCoin::from_micro_units(5_000_000));
        solver_agents.push(a);
    }
    let cfg = RuntimeChaintapeConfig {
        runtime_repo_path: args.runtime_repo.clone(),
        cas_path: args.cas.clone(),
        run_id: args.run_id.clone(),
        queue_capacity: 64,
        resume_existing_chain: false,
    };
    let bundle = build_chaintape_sequencer_with_initial_q(&cfg, initial_q)
        .map_err(|e| format!("lean runner boot failed: {e}"))?;
    let seq = bundle.sequencer.clone();
    let rej_writer = bundle.rejection_writer.clone();
    let mut keypairs =
        AgentKeypairRegistry::open(&cfg.runtime_repo_path).map_err(|e| format!("{e}"))?;
    keypairs
        .get_or_create(&AgentId(SPONSOR_AGENT.to_string()))
        .map_err(|e| format!("create sponsor keypair: {e}"))?;
    keypairs
        .get_or_create(&AgentId(VERIFIER_AGENT.to_string()))
        .map_err(|e| format!("create verifier keypair: {e}"))?;
    for a in &solver_agents {
        keypairs
            .get_or_create(&AgentId(a.to_string()))
            .map_err(|e| format!("create keypair {a}: {e}"))?;
    }
    seq.set_agent_pubkeys(Arc::new(keypairs.manifest()))
        .map_err(|_| "agent pubkey manifest already set".to_string())?;

    let client = ResilientLLMClient::new(&args.proxy_url, 180, 2);
    let sys = Message {
        role: "system".into(),
        content: "You are a Lean 4 theorem-proving agent. Return ONLY a JSON object, no markdown."
            .into(),
    };

    let mut attempts: Vec<AttemptResult> = Vec::new();
    let mut problem_results: Vec<ProblemResult> = Vec::new();
    let mut brand_rows: Vec<BrandRow> = Vec::new();
    let mut logical_t: u64 = 100;
    let mut ord: u64 = 0;
    let mut last_root = seq
        .q_snapshot()
        .map_err(|e| format!("initial q_snapshot: {e:?}"))?
        .state_root_t;
    let mut any_omega = false;

    'problems: for (pi, thm) in problems.iter().enumerate() {
        let agent = solver_agents[pi % solver_agents.len()].clone();
        let judge = thm.judge(lean_bin.clone(), None);
        let task = format!("lean:{}-{}", sanitize(&thm.id), pi);

        // ── Phase-6 brand-GENERIC provider handle capsule on canonical CAS ──
        logical_t += 1;
        let external_descriptor = format!("{}::{}", args.brand_provider, args.model);
        let (_handle_cid, sidecar) = {
            let mut cas = CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
            write_provider_handle_capsule(
                &mut cas,
                &agent,
                &external_descriptor,
                &args.model,
                &args.brand_provider,
                logical_t,
            )
            .map_err(|e| format!("write provider handle capsule: {e}"))?
        };
        let provider_handle = sidecar.model_handle.clone();
        brand_rows.push(BrandRow {
            agent_id: agent.clone(),
            provider_handle: provider_handle.clone(),
            brand_model_name: args.model.clone(),
            brand_model_provider: args.brand_provider.clone(),
        });

        let mut problem_tokens = 0u64;
        let mut problem_verified = false;
        let mut omega_emitted = false;
        let mut last_feedback: Option<String> = None;
        let mut last_body: Option<String> = None;

        for attempt_index in 0..args.max_attempts {
            // Phase-5 budget gate (live tape spend vs signed ceiling).
            let spend = current_spend(&args)?;
            if let BudgetVerdict::Exceeded {
                spend_micro,
                ceiling_micro,
            } = budget_check(spend, ceiling)
            {
                println!(
                    "livefc1_lean_runner: BUDGET HALT before {task} attempt {attempt_index}: spend={spend_micro} >= ceiling={ceiling_micro}"
                );
                break 'problems;
            }

            ord += 1;
            logical_t += 1;
            let prompt = build_prompt(thm, last_body.as_deref(), last_feedback.as_deref());
            let prompt_sha = sha256_hex(&prompt);
            let gen = client
                .generate(&GenerateRequest {
                    model: args.model.clone(),
                    messages: vec![
                        sys.clone(),
                        Message {
                            role: "user".into(),
                            content: prompt,
                        },
                    ],
                    temperature: Some(0.4),
                    max_tokens: Some(700),
                })
                .await;

            let resp = match gen {
                Ok(r) => r,
                Err(err) => {
                    let submit_id = write_l4e(
                        &args,
                        &rej_writer,
                        last_root,
                        &agent,
                        &task,
                        &prompt_sha,
                        &format!("{err}"),
                        RejectionClass::LlmError,
                        "llm_err",
                    )?;
                    attempts.push(AttemptResult {
                        problem_id: thm.id.clone(),
                        task_id: Some(task.clone()),
                        attempt_index,
                        outcome: "llm_err".into(),
                        verdict_kind: "LlmError".into(),
                        prompt_tokens: 0,
                        completion_tokens: 0,
                        body_preview: String::new(),
                        feedback: "llm call failed".into(),
                        work_tx_id: None,
                        l4e_submit_id: Some(submit_id),
                        proposal_telemetry_cid: None,
                        verification_result_cid: None,
                        verified: false,
                    });
                    println!("livefc1_lean_runner: {task} attempt {attempt_index} LLM_ERR -> L4.E #{submit_id}");
                    continue;
                }
            };
            let prompt_tokens = resp.prompt_tokens as u64;
            let completion_tokens = resp.completion_tokens as u64;
            problem_tokens += prompt_tokens + completion_tokens;
            let tokens = TokenCounts {
                prompt_tokens,
                completion_tokens,
                tool_tokens: 0,
            };

            let body = match extract_proof_body(&resp.content) {
                Some(b) if !b.trim().is_empty() => b,
                _ => {
                    let submit_id = write_l4e(
                        &args,
                        &rej_writer,
                        last_root,
                        &agent,
                        &task,
                        &prompt_sha,
                        "model output did not parse as JSON proof_body",
                        RejectionClass::ParseFailed,
                        "parse_fail",
                    )?;
                    attempts.push(AttemptResult {
                        problem_id: thm.id.clone(),
                        task_id: Some(task.clone()),
                        attempt_index,
                        outcome: "parse_fail".into(),
                        verdict_kind: "ParseFailed".into(),
                        prompt_tokens,
                        completion_tokens,
                        body_preview: String::new(),
                        feedback: "parse fail".into(),
                        work_tx_id: None,
                        l4e_submit_id: Some(submit_id),
                        proposal_telemetry_cid: None,
                        verification_result_cid: None,
                        verified: false,
                    });
                    println!("livefc1_lean_runner: {task} attempt {attempt_index} PARSE_FAIL -> L4.E #{submit_id}");
                    continue;
                }
            };

            // ── REAL Lean kernel verdict (the external ground-truth oracle) ──
            let outcome = judge.verify(&body);
            let verified = outcome.is_verified();
            let assembled = judge.assemble(&body);

            // VerificationResult CAS object (the Lean-oracle witness).
            let artifact_cid = {
                let mut cas = CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
                cas.put(
                    assembled.as_bytes(),
                    ObjectType::Generic,
                    "lean-proof-artifact",
                    logical_t,
                    Some("turingosv4.lean.proof_artifact.v1".into()),
                )
                .map_err(|e| format!("put proof artifact: {e}"))?
            };
            let vr = VerificationResult::from_lean_run(
                TxId(format!("worktx-{task}-leanv{ord}")),
                AgentId(VERIFIER_AGENT.into()),
                outcome.exit_code,
                artifact_cid,
                &format!("lean-{}-{ord}.lean", sanitize(&thm.id)),
                assembled.as_bytes(),
            );
            let vr_cid = {
                let mut cas = CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
                write_verification_result_to_cas(&mut cas, &vr, "lean-verifier", logical_t)
                    .map_err(|e| format!("write VerificationResult: {e}"))?
            };

            // ProposalTelemetry carrying REAL token counts, LINKED to the
            // VerificationResult so the VPPUT reconstruction resolves oracle_verified.
            let proposal_cid = {
                let payload = serde_json::json!({
                    "schema": "turingosv4.livefc1.lean_proof.v1",
                    "problem_id": thm.id,
                    "proof_body_len": body.chars().count(),
                });
                let bytes =
                    serde_json::to_vec(&payload).map_err(|e| format!("ser proof eval: {e}"))?;
                let mut cas = CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
                let eval_cid = cas
                    .put(
                        &bytes,
                        ObjectType::ProposalPayload,
                        "lean-eval",
                        logical_t,
                        Some("turingosv4.livefc1.lean_proof.v1".into()),
                    )
                    .map_err(|e| format!("put eval: {e}"))?;
                let tel = ProposalTelemetry::new_root(
                    AgentId(agent.clone()),
                    hash_from_hex(&prompt_sha)?,
                    eval_cid,
                    "lean_proof".into(),
                    tokens,
                    format!("{agent}.lean.b{ord}"),
                )
                .with_verification_result(vr_cid);
                write_proposal_telemetry_to_cas(&mut cas, &tel, "lean-proposal-telemetry", logical_t)
                    .map_err(|e| format!("write ProposalTelemetry: {e}"))?
            };

            last_body = Some(body.clone());
            last_feedback = Some(outcome.feedback.clone());

            if !verified {
                // FAILED Lean attempt → REAL L4.E `LeanFailed` (token-spent, counted).
                // We store the FULL encoded WorkTx in CAS and point the L4.E row's
                // tx_payload_cid at it, so the VPPUT reconstruction can decode it and
                // attribute its proposal-telemetry tokens to the task (failed branches
                // MUST count toward C_i).
                let failed_work = make_real_worktx_signed_by(
                    &mut keypairs,
                    &task,
                    &agent,
                    last_root,
                    WORK_STAKE_MICRO,
                    &format!("leanfail{ord}"),
                    proposal_cid,
                    false,
                    logical_t,
                )
                .map_err(|e| format!("build failed WorkTx: {e}"))?;
                let failed_work_cid = {
                    let bytes = canonical_encode(&failed_work)
                        .map_err(|e| format!("encode failed WorkTx: {e:?}"))?;
                    let mut cas =
                        CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
                    cas.put(
                        &bytes,
                        ObjectType::Generic,
                        "lean-failed-worktx",
                        logical_t,
                        Some("turingosv4.lean.failed_worktx.v1".into()),
                    )
                    .map_err(|e| format!("put failed WorkTx: {e}"))?
                };
                let submit_id = append_l4e_work(
                    &rej_writer,
                    last_root,
                    &agent,
                    failed_work_cid,
                    RejectionClass::LeanFailed,
                    &format!(
                        "lean_failed: {} (token-spent attempt; failed branch counted)",
                        truncate(&outcome.feedback, 120)
                    ),
                )?;
                attempts.push(AttemptResult {
                    problem_id: thm.id.clone(),
                    task_id: Some(task.clone()),
                    attempt_index,
                    outcome: "lean_failed".into(),
                    verdict_kind: format!("{:?}", outcome.verdict_kind),
                    prompt_tokens,
                    completion_tokens,
                    body_preview: body.chars().take(80).collect(),
                    feedback: outcome.feedback.chars().take(160).collect(),
                    work_tx_id: None,
                    l4e_submit_id: Some(submit_id),
                    proposal_telemetry_cid: Some(proposal_cid.hex()),
                    verification_result_cid: Some(vr_cid.hex()),
                    verified: false,
                });
                println!(
                    "livefc1_lean_runner: {task} attempt {attempt_index} LEAN_FAILED ({:?}) -> L4.E #{submit_id}",
                    outcome.verdict_kind
                );
                continue;
            }

            // ── VERIFIED: accepted L4 spine TaskOpen → EscrowLock → WorkTx ──
            let (work_tx_id, new_root) = submit_verified_spine(
                &seq,
                &mut keypairs,
                &task,
                &agent,
                proposal_cid,
                last_root,
                ord,
            )
            .await?;
            last_root = new_root;

            // VerifyTx (Confirm) records the verifier's on-tape confirmation.
            logical_t += 1;
            if let Ok(vtx) = make_real_verifytx_signed_by(
                &mut keypairs,
                last_root,
                TxId(work_tx_id.clone()),
                VERIFIER_AGENT,
                VERIFY_BOND_MICRO,
                &format!("leanv{ord}"),
                true,
                logical_t,
            ) {
                if seq.submit_agent_tx(vtx).await.is_ok() {
                    if let Ok(r) = tb8_await_state_root_advance(&seq, last_root, 8_000).await {
                        last_root = r;
                    }
                }
            }

            problem_verified = true;
            problem_tokens += 0;
            attempts.push(AttemptResult {
                problem_id: thm.id.clone(),
                task_id: Some(task.clone()),
                attempt_index,
                outcome: "verified".into(),
                verdict_kind: format!("{:?}", outcome.verdict_kind),
                prompt_tokens,
                completion_tokens,
                body_preview: body.chars().take(80).collect(),
                feedback: String::new(),
                work_tx_id: Some(work_tx_id.clone()),
                l4e_submit_id: None,
                proposal_telemetry_cid: Some(proposal_cid.hex()),
                verification_result_cid: Some(vr_cid.hex()),
                verified: true,
            });
            println!(
                "livefc1_lean_runner: {task} attempt {attempt_index} VERIFIED work_tx={work_tx_id} (real Lean kernel success)"
            );

            // ── TerminalSummary OmegaAccepted for THIS task (omega_terminal gate) ──
            logical_t += 1;
            let terminal = tb11_emit_terminal_summary_for_run(
                &seq,
                RunId(format!("{}-{}", args.run_id, sanitize(&thm.id))),
                TaskId(task.clone()),
                RunOutcome::OmegaAccepted,
                (attempt_index + 1) as u32,
                BTreeMap::new(),
                logical_t,
                Some(AgentId(agent.clone())),
                None,
            )
            .await;
            match terminal {
                Ok(_) => {
                    if tb8_await_state_root_advance(&seq, last_root, 8_000).await.is_ok() {
                        omega_emitted = true;
                        any_omega = true;
                        if let Ok(q) = seq.q_snapshot() {
                            last_root = q.state_root_t;
                        }
                    }
                }
                Err(e) => eprintln!("livefc1_lean_runner: TerminalSummary emit failed: {e:?}"),
            }
            break; // one verified golden path per problem is enough
        }

        problem_results.push(ProblemResult {
            problem_id: thm.id.clone(),
            task_id: task,
            needs_mathlib: thm.needs_mathlib,
            attempts: args.max_attempts,
            verified: problem_verified,
            omega_accepted_emitted: omega_emitted,
            total_tokens: problem_tokens,
            provider_handle,
        });
    }

    // Drain + shutdown the chaintape.
    let seq_handle = seq.clone();
    bundle
        .shutdown()
        .await
        .map_err(|e| format!("chaintape shutdown: {e}"))?;
    let _ = seq_handle.q_snapshot();

    // genesis_report.json so off-tape audit/replay can resolve identities.
    let report = GenesisReport {
        constitution_hash: GenesisReport::hash_constitution_md(&args.constitution),
        runtime_repo: args.runtime_repo.display().to_string(),
        cas_path: args.cas.display().to_string(),
        system_pubkey_hash: GenesisReport::hash_system_pubkey_manifest(&args.runtime_repo),
        agent_pubkeys_path: "agent_pubkeys.json".to_string(),
        initial_balances: preseed
            .iter()
            .map(|(a, b)| (a.0.clone(), b.micro_units()))
            .collect(),
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
        .map_err(|e| format!("write genesis_report.json: {e}"))?;

    let final_spend = current_spend(&args)?;
    let sidecar = RunSidecar {
        schema_version: "turingosv4.livefc1.lean_run_sidecar.v1",
        run_id: args.run_id.clone(),
        proxy_url: args.proxy_url.clone(),
        model_brand: args.model.clone(),
        provider_brand: args.brand_provider.clone(),
        lean_bin: lean_bin.display().to_string(),
        budget_ceiling_micro_units: ceiling.micro_units(),
        final_spend_tokens: final_spend,
        wall_clock_s: t0.elapsed().as_secs_f64(),
        problems: problem_results.clone(),
        attempts: attempts.clone(),
        brand_sidecar: brand_rows,
        fc2_terminal_omega_accepted: any_omega,
        notes: vec![
            "REAL Lean kernel (LeanJudge) is the external ground-truth oracle; progress=1 ONLY on a kernel-Verified proof.",
            "A Verified proof links VerificationResult.verified=true into ProposalTelemetry.verification_result_cid AND emits TerminalSummary OmegaAccepted => VPPUT progress=1.",
            "Failed Lean attempts land on tape as L4.E LeanFailed (token-spent, counted) — not a verified path.",
            "Provider identity on canonical CAS is the brand-GENERIC ProviderHandleCapsule (opaque sha256 handle); brand->handle mapping is external-only.",
        ],
    };
    write_pretty(&args.out_dir.join("lean_run_sidecar.json"), &sidecar)?;

    let verified_problems = problem_results.iter().filter(|p| p.verified).count();
    println!(
        "livefc1_lean_runner: DONE problems={} verified={} omega_accepted={} final_spend_tokens={} sidecar={}",
        problem_results.len(),
        verified_problems,
        any_omega,
        final_spend,
        args.out_dir.join("lean_run_sidecar.json").display()
    );
    Ok(())
}

/// Accepted-spine submit for a VERIFIED proof: TaskOpen → EscrowLock → WorkTx.
#[allow(clippy::too_many_arguments)]
async fn submit_verified_spine(
    seq: &turingosv4::state::sequencer::Sequencer,
    keypairs: &mut AgentKeypairRegistry,
    task: &str,
    agent: &str,
    proposal_cid: Cid,
    parent_root: Hash,
    ord: u64,
) -> Result<(String, Hash), String> {
    let task_open = make_real_task_open_signed_by(
        keypairs,
        task,
        SPONSOR_AGENT,
        parent_root,
        "livefc1-lean",
        200 + ord,
    )
    .map_err(|e| format!("build TaskOpen: {e}"))?;
    seq.submit_agent_tx(task_open)
        .await
        .map_err(|e| format!("submit TaskOpen: {e:?}"))?;
    let after_open = tb8_await_state_root_advance(seq, parent_root, 8_000)
        .await
        .map_err(|_| "TaskOpen did not advance".to_string())?;

    let escrow = make_real_escrow_lock_signed_by(
        keypairs,
        task,
        SPONSOR_AGENT,
        TASK_ESCROW_MICRO,
        after_open,
        "livefc1-lean",
        300 + ord,
    )
    .map_err(|e| format!("build Escrow: {e}"))?;
    seq.submit_agent_tx(escrow)
        .await
        .map_err(|e| format!("submit Escrow: {e:?}"))?;
    let after_escrow = tb8_await_state_root_advance(seq, after_open, 8_000)
        .await
        .map_err(|_| "Escrow did not advance".to_string())?;

    let work = make_real_worktx_signed_by(
        keypairs,
        task,
        agent,
        after_escrow,
        WORK_STAKE_MICRO,
        "livefc1-lean",
        proposal_cid,
        true,
        400 + ord,
    )
    .map_err(|e| format!("build Work: {e}"))?;
    let work_tx_id = match &work {
        TypedTx::Work(w) => w.tx_id.0.clone(),
        _ => unreachable!(),
    };
    seq.submit_agent_tx(work)
        .await
        .map_err(|e| format!("submit Work: {e:?}"))?;
    let after_work = tb8_await_state_root_advance(seq, after_escrow, 8_000)
        .await
        .map_err(|_| "Work did not advance".to_string())?;
    Ok((work_tx_id, after_work))
}

type SharedRejectionWriter = std::sync::Arc<std::sync::RwLock<RejectionEvidenceWriter>>;

/// Append a REAL L4.E rejection whose `tx_payload_cid` is a pre-built CAS object
/// (for `LeanFailed`, the full encoded WorkTx so VPPUT can decode + count tokens).
fn append_l4e_work(
    rej: &SharedRejectionWriter,
    parent_state_root: Hash,
    agent: &str,
    tx_payload_cid: Cid,
    class: RejectionClass,
    public_summary: &str,
) -> Result<u64, String> {
    let mut writer = rej
        .write()
        .map_err(|_| "rejection writer lock poisoned".to_string())?;
    let submit_id = writer.len() as u64 + 1;
    writer.append_rejected(
        submit_id,
        parent_state_root,
        AgentId(agent.to_string()),
        TxKind::Work,
        tx_payload_cid,
        class,
        None,
        Some(public_summary.to_string()),
    );
    Ok(submit_id)
}

/// L4.E for llm_err / parse_fail (no decodable WorkTx; a minimal attempt capsule).
#[allow(clippy::too_many_arguments)]
fn write_l4e(
    args: &Args,
    rej: &SharedRejectionWriter,
    parent_state_root: Hash,
    agent: &str,
    task: &str,
    prompt_sha: &str,
    raw_err: &str,
    class: RejectionClass,
    public_class: &str,
) -> Result<u64, String> {
    let mut cas = CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
    let attempt_payload = serde_json::json!({
        "schema": "turingosv4.livefc1.lean_failed_attempt.v1",
        "task": task,
        "agent_id": agent,
        "prompt_sha256": prompt_sha,
        "public_class": public_class,
        "raw_error": truncate(raw_err, 200),
    });
    let attempt_bytes =
        serde_json::to_vec(&attempt_payload).map_err(|e| format!("ser att: {e}"))?;
    let attempt_cid = cas
        .put(
            &attempt_bytes,
            ObjectType::Generic,
            "lean-failed-attempt",
            0,
            Some("turingosv4.livefc1.lean_failed_attempt.v1".into()),
        )
        .map_err(|e| format!("put attempt: {e}"))?;
    append_l4e_work(
        rej,
        parent_state_root,
        agent,
        attempt_cid,
        class,
        &format!("{public_class}: external attempt failed (token-spent)"),
    )
}

/// Reconstruct cumulative live tape spend (reuses the Phase-5/2 cost path).
fn current_spend(args: &Args) -> Result<u64, String> {
    match load_tape_ro(args) {
        Ok(t) => Ok(loaded_tape_spend_tokens(&t)),
        Err(_) => Ok(0),
    }
}

fn load_tape_ro(args: &Args) -> Result<LoadedTape, String> {
    let inputs = AuditInputs {
        runtime_repo: args.runtime_repo.clone(),
        cas_dir: args.cas.clone(),
        agent_pubkeys: args.runtime_repo.join("agent_pubkeys.json"),
        pinned_pubkeys: args.runtime_repo.join("pinned_pubkeys.json"),
        genesis: args.genesis.clone(),
        constitution: args.constitution.clone(),
        markov_pointer: None,
        alignment_dir: None,
    };
    load_tape(&inputs).map_err(|e| format!("load_tape: {e}"))
}

fn build_prompt(thm: &LeanTheorem, parent_body: Option<&str>, parent_feedback: Option<&str>) -> String {
    let mut p = String::new();
    p.push_str("You are proving a theorem in Lean 4 (core/Std only; Mathlib is NOT available). Output ONLY a JSON object.\n\n");
    p.push_str("=== Target (prove the goal after `:= by`) ===\n");
    p.push_str(&thm.preamble);
    p.push('\n');
    if let (Some(body), Some(fb)) = (parent_body, parent_feedback) {
        p.push_str("\n=== A previous attempt FAILED — fix it ===\n--- attempt body ---\n");
        p.push_str(body);
        p.push_str("\n--- Lean error ---\n");
        p.push_str(fb);
        p.push('\n');
    }
    p.push_str(
        "\nUse only core Lean 4 tactics (e.g. rfl, simp, decide, omega, exact, intro, constructor). \
         Do NOT use sorry, admit, or native_decide.\n\
         Return EXACTLY: {\"proof_body\":\"<the Lean tactic block AFTER `:= by`, no theorem signature, no imports>\",\"confidence\":0.0-1.0}\n",
    );
    p
}

fn extract_proof_body(content: &str) -> Option<String> {
    let t = content
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();
    let v: serde_json::Value = serde_json::from_str(t)
        .ok()
        .or_else(|| {
            let s = t.find('{')?;
            let e = t.rfind('}')?;
            serde_json::from_str(&t[s..=e]).ok()
        })?;
    v.get("proof_body")
        .and_then(|x| x.as_str())
        .map(|s| s.to_string())
}

fn sanitize(v: &str) -> String {
    v.chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '-' })
        .collect()
}

fn truncate(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

fn write_pretty<T: Serialize>(path: &PathBuf, value: &T) -> Result<(), String> {
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p).map_err(|e| format!("mkdir {}: {e}", p.display()))?;
    }
    let bytes = serde_json::to_vec_pretty(value).map_err(|e| format!("ser: {e}"))?;
    std::fs::write(path, bytes).map_err(|e| format!("write {}: {e}", path.display()))
}

fn sha256_hex(input: impl AsRef<[u8]>) -> String {
    Sha256::digest(input.as_ref()).iter().map(|b| format!("{b:02x}")).collect()
}

fn hash_from_hex(hex: &str) -> Result<Hash, String> {
    if hex.len() != 64 {
        return Err(format!("sha256 hex must be 64 chars, got {}", hex.len()));
    }
    let mut bytes = [0u8; 32];
    for i in 0..32 {
        bytes[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16)
            .map_err(|e| format!("hex byte {i}: {e}"))?;
    }
    Ok(Hash::from_bytes(bytes))
}
