//! LIVE-FC1 Phase 7 — REAL heterogeneous swarm-multi-LLM runner (thin orchestrator).
//!
//! This is an UNPINNED runner bin (genesis pin-count 0; nothing in
//! `genesis_payload.toml` references this path). It drives a REAL swarm of
//! agents over a few MATH-competition problems through the local OpenAI-compatible
//! `llm_proxy.py`, on ONE shared canonical ChainTape, and produces the live
//! evidence the LIVE-FC1 Phase 1-6 observers verify off-tape.
//!
//! What it does, per config-matrix cell (provider × temperature × prompt-variant):
//!   1. Phase-5 BUDGET CHECK — reconstruct cumulative spend from the live tape +
//!      compare against the signed budget manifest ceiling; HALT (skip the cell,
//!      no spend) when the hard ceiling is reached (FC2-HALT via fuel exhaustion).
//!   2. Phase-6 PROVIDER CAPSULE — anchor a brand-GENERIC `ProviderHandleCapsule`
//!      (opaque sha256 handle, NO brand) for the agent on the run's canonical CAS;
//!      the brand→handle mapping stays in an EXTERNAL sidecar file only.
//!   3. REAL LLM CALL — through the proxy; record real prompt/completion tokens.
//!   4. CANONICAL SPINE — TaskOpen → EscrowLock → WorkTx (ProposalTelemetry carrying
//!      the real token counts) on the shared tape.
//!
//! One config cell is a DELIBERATE FAULT (an unroutable model id → proxy error)
//! whose token-spent attempt is recorded as a REAL L4.E `LlmError` rejection
//! record (NOT a crash) — the FC1 failure arm.
//!
//! At the end it emits an FC2 `MapReduceTick` and a `TerminalSummary` so the FC2
//! boot/tick/terminal nodes fire on tape.
//!
//! Honesty: this is a TASK workload (math benchmark). It does NOT drive the Lean
//! oracle, so no `VerificationResult.verified` ground-truth witness exists —
//! VPPUT `progress` is honestly 0 for every task (the metric reconstructs cost +
//! ticks faithfully; the numerator is gated 0 without a verified golden path).
//! FC3 is reached only at the observable/canary leg if a canary is anchored.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    RejectionClass, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::transition_ledger::TxKind;
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::adapter::{
    genesis_with_balances, make_real_escrow_lock_signed_by, make_real_task_open_signed_by,
    make_real_worktx_signed_by, tb11_emit_terminal_summary_for_run, tb8_await_state_root_advance,
};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::agent_scheduler::budget_ceiling::{
    budget_check, loaded_tape_spend_tokens, reject_class_label, BudgetManifest, BudgetVerdict,
};
use turingosv4::runtime::agent_scheduler::provider_handle_capsule::{
    write_provider_handle_capsule, ProviderBrandSidecar,
};
use turingosv4::runtime::audit_assertions::{load_tape, AuditInputs, LoadedTape};
use turingosv4::runtime::bootstrap::default_pput_preseed_pairs;
use turingosv4::runtime::genesis_report::GenesisReport;
use turingosv4::runtime::proposal_telemetry::{
    write_to_cas as write_proposal_telemetry_to_cas, ProposalTelemetry, TokenCounts,
};
use turingosv4::runtime::{build_chaintape_sequencer_with_initial_q, RuntimeChaintapeConfig};
use turingosv4::state::q_state::{AgentId, Hash, TaskId, TxId};
use turingosv4::state::sequencer::SystemEmitCommand;
use turingosv4::state::typed_tx::{RunId, RunOutcome, TickKind, TypedTx};

const SPONSOR_AGENT: &str = "Agent_user_0";
const TASK_ESCROW_MICRO: i64 = 10_000;
const WORK_STAKE_MICRO: i64 = 100;

#[derive(Debug)]
struct Args {
    runtime_repo: PathBuf,
    cas: PathBuf,
    run_id: String,
    constitution: PathBuf,
    genesis: PathBuf,
    samples_json: PathBuf,
    llm_proxy_url: String,
    budget_manifest: PathBuf,
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MathSample {
    sample_id: String,
    subject: String,
    level: String,
    problem: String,
    solution: String,
    expected_answer: String,
}

/// One brand-laden config-matrix axis. `brand_model` is the proxy model id (it
/// routes to the provider); it NEVER lands on the canonical tape — only the
/// opaque generic handle does (Phase-6 ProviderHandleCapsule).
#[derive(Debug, Clone)]
struct ProviderCfg {
    brand_model: &'static str,
    brand_provider: &'static str,
}

#[derive(Debug, Clone)]
struct PromptVariant {
    label: &'static str,
    style: &'static str,
}

/// One config-matrix cell result, recorded to the EXTERNAL evidence sidecar.
#[derive(Debug, Clone, Serialize)]
struct CellResult {
    cell_id: String,
    agent_id: String,
    /// Generic sha256 provider handle on the canonical CAS (brand-free).
    provider_handle: String,
    /// EXTERNAL-only brand mapping (sidecar; never on canonical tape).
    brand_model: String,
    brand_provider: String,
    temperature_milli: u64,
    prompt_variant: String,
    sample_id: String,
    /// "ok" | "fault_injected_llm_err" | "budget_halt" | "parse_fail" | "llm_err"
    outcome: String,
    answer_correct: Option<bool>,
    predicted_answer: Option<String>,
    prompt_tokens: u64,
    completion_tokens: u64,
    work_tx_id: Option<String>,
    l4e_submit_id: Option<u64>,
    provider_handle_capsule_cid: String,
    proposal_telemetry_cid: Option<String>,
    note: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct RunSidecar {
    schema_version: &'static str,
    run_id: String,
    llm_proxy_url: String,
    budget_ceiling_micro_units: i64,
    final_spend_tokens: u64,
    cells: Vec<CellResult>,
    brand_sidecar: Vec<BrandSidecarRow>,
    fc2_map_reduce_tick_emitted: bool,
    fc2_terminal_summary_emitted: bool,
    notes: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
struct BrandSidecarRow {
    agent_id: String,
    provider_handle: String,
    brand_model_name: String,
    brand_model_provider: String,
}

impl From<ProviderBrandSidecar> for BrandSidecarRow {
    fn from(s: ProviderBrandSidecar) -> Self {
        Self {
            agent_id: s.agent_id,
            provider_handle: s.model_handle,
            brand_model_name: s.brand_model_name,
            brand_model_provider: s.brand_model_provider,
        }
    }
}

fn usage() -> &'static str {
    "usage: livefc1_swarm_runner --runtime-repo <PATH> --cas <PATH> --run-id <ID> \
     --constitution <constitution.md> --genesis <genesis_payload.toml> \
     --samples-json <PATH> --llm-proxy-url <URL> --budget-manifest <PATH> --out-dir <PATH>"
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut m: BTreeMap<&str, String> = BTreeMap::new();
    let mut i = 0;
    let keys = [
        "--runtime-repo",
        "--cas",
        "--run-id",
        "--constitution",
        "--genesis",
        "--samples-json",
        "--llm-proxy-url",
        "--budget-manifest",
        "--out-dir",
    ];
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
        let v = argv
            .get(i)
            .ok_or_else(|| format!("missing value after {key}"))?;
        m.insert(key, v.clone());
        i += 1;
    }
    let get = |k: &str| m.get(k).cloned().ok_or_else(|| format!("{k} required"));
    Ok(Args {
        runtime_repo: get("--runtime-repo")?.into(),
        cas: get("--cas")?.into(),
        run_id: get("--run-id")?,
        constitution: get("--constitution")?.into(),
        genesis: get("--genesis")?.into(),
        samples_json: get("--samples-json")?.into(),
        llm_proxy_url: get("--llm-proxy-url")?,
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
            eprintln!("livefc1_swarm_runner: {msg}");
            return ExitCode::from(2);
        }
    };
    match run(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("livefc1_swarm_runner: {e}");
            ExitCode::from(1)
        }
    }
}

async fn run(args: Args) -> Result<(), String> {
    // Boot-trust-root verify (mirrors math runner): CWD holds genesis_payload.toml.
    let trust_root_repo = std::env::current_dir().map_err(|e| format!("cwd: {e}"))?;
    turingosv4::boot::verify_trust_root(&trust_root_repo)
        .map_err(|e| format!("TRUST_ROOT_TAMPERED: {e}"))?;

    std::fs::create_dir_all(&args.runtime_repo).map_err(|e| format!("runtime repo dir: {e}"))?;
    std::fs::create_dir_all(&args.cas).map_err(|e| format!("cas dir: {e}"))?;
    std::fs::create_dir_all(&args.out_dir).map_err(|e| format!("out dir: {e}"))?;

    let samples: Vec<MathSample> = serde_json::from_slice(
        &std::fs::read(&args.samples_json).map_err(|e| format!("read samples: {e}"))?,
    )
    .map_err(|e| format!("parse samples json: {e}"))?;
    if samples.is_empty() {
        return Err("samples-json contained no tasks".into());
    }

    // Phase-5 signed budget manifest (a SEPARATE unpinned TOML, never genesis).
    let budget = BudgetManifest::from_file(&args.budget_manifest)
        .map_err(|e| format!("load budget manifest: {e}"))?;
    let ceiling = budget.ceiling_micro();
    println!(
        "livefc1_swarm_runner: budget ceiling = {} micro-units (0 = unlimited)",
        ceiling.micro_units()
    );

    // ── Boot ONE shared canonical chaintape ─────────────────────────────────
    let preseed = default_pput_preseed_pairs();
    let mut initial_q = genesis_with_balances(&preseed);
    // Ensure sponsor + the solver agents we use carry balances.
    initial_q
        .economic_state_t
        .balances_t
        .0
        .entry(AgentId(SPONSOR_AGENT.to_string()))
        .or_insert(MicroCoin::from_micro_units(100_000_000));
    let cfg = RuntimeChaintapeConfig {
        runtime_repo_path: args.runtime_repo.clone(),
        cas_path: args.cas.clone(),
        run_id: args.run_id.clone(),
        queue_capacity: 64,
        resume_existing_chain: false,
    };
    let bundle = build_chaintape_sequencer_with_initial_q(&cfg, initial_q)
        .map_err(|e| format!("swarm boot failed: {e}"))?;
    let seq = bundle.sequencer.clone();
    // The single L4.E rejection writer (owned by the sequencer). Provider-fault
    // L4.E rows are appended through THIS shared writer so there is one coherent
    // `rejections.jsonl` chain.
    let rej_writer = bundle.rejection_writer.clone();
    let mut keypairs =
        AgentKeypairRegistry::open(&cfg.runtime_repo_path).map_err(|e| format!("{e}"))?;
    // Sponsor + 10 sandbox solver agents.
    let mut solver_agents: Vec<String> = Vec::new();
    for i in 0..10 {
        solver_agents.push(format!("Agent_{i}"));
    }
    keypairs
        .get_or_create(&AgentId(SPONSOR_AGENT.to_string()))
        .map_err(|e| format!("create sponsor keypair: {e}"))?;
    for a in &solver_agents {
        keypairs
            .get_or_create(&AgentId(a.to_string()))
            .map_err(|e| format!("create keypair {a}: {e}"))?;
    }
    seq.set_agent_pubkeys(Arc::new(keypairs.manifest()))
        .map_err(|_| "agent pubkey manifest already set".to_string())?;

    // ── Config matrix ───────────────────────────────────────────────────────
    // Two LIVE providers proven at smoke (DeepSeek + SiliconFlow) + DashScope as
    // a third heterogeneity cell. Two temperatures, two prompt variants. Kept
    // SMALL for bounded cost; scaling to 100 agents is purely a config change.
    let providers = [
        ProviderCfg {
            brand_model: "deepseek-chat",
            brand_provider: "DeepSeek",
        },
        ProviderCfg {
            brand_model: "Qwen/Qwen2.5-72B-Instruct",
            brand_provider: "SiliconFlow",
        },
        ProviderCfg {
            brand_model: "qwen3-8b",
            brand_provider: "DashScope",
        },
    ];
    let temps_milli = [0u64, 700u64];
    let prompts = [
        PromptVariant {
            label: "terse",
            style: "Answer concisely.",
        },
        PromptVariant {
            label: "verbose",
            style: "Show your full reasoning step by step.",
        },
    ];

    // Build the matrix cells (provider × temp × prompt × task), bounded.
    // We cap the number of REAL llm cells to keep cost low.
    let max_real_cells = 12usize;
    let mut cells: Vec<MatrixCell> = Vec::new();
    let mut agent_idx = 0usize;
    'outer: for (pi, prov) in providers.iter().enumerate() {
        for (ti, &temp) in temps_milli.iter().enumerate() {
            for (vi, pv) in prompts.iter().enumerate() {
                // One task per cell, round-robin across the sample set.
                let sample = &samples[(pi + ti + vi + agent_idx) % samples.len()];
                cells.push(MatrixCell {
                    cell_id: format!("p{pi}-t{ti}-v{vi}"),
                    agent_id: solver_agents[agent_idx % solver_agents.len()].clone(),
                    provider: prov.clone(),
                    temperature_milli: temp,
                    prompt: pv.clone(),
                    sample: sample.clone(),
                });
                agent_idx += 1;
                if cells.len() >= max_real_cells {
                    break 'outer;
                }
            }
        }
    }

    let client = ResilientLLMClient::new(&args.llm_proxy_url, 120, 1);
    let mut results: Vec<CellResult> = Vec::new();
    let mut brand_rows: Vec<BrandSidecarRow> = Vec::new();
    let mut logical_t: u64 = 100; // capsule recorded_at marker; monotone within run
    let mut last_root = seq
        .q_snapshot()
        .map_err(|e| format!("initial q_snapshot: {e:?}"))?
        .state_root_t;

    for (ci, cell) in cells.iter().enumerate() {
        logical_t += 1;
        // ── Phase-5 budget gate (live tape spend vs signed ceiling) ─────────
        let spend = current_spend(&args, &cfg)?;
        match budget_check(spend, ceiling) {
            BudgetVerdict::Exceeded {
                spend_micro,
                ceiling_micro,
            } => {
                println!(
                    "livefc1_swarm_runner: BUDGET HALT at cell {} ({}): spend_micro={} >= ceiling_micro={} class={}",
                    ci,
                    cell.cell_id,
                    spend_micro,
                    ceiling_micro,
                    reject_class_label()
                );
                results.push(CellResult {
                    cell_id: cell.cell_id.clone(),
                    agent_id: cell.agent_id.clone(),
                    provider_handle: String::new(),
                    brand_model: cell.provider.brand_model.to_string(),
                    brand_provider: cell.provider.brand_provider.to_string(),
                    temperature_milli: cell.temperature_milli,
                    prompt_variant: cell.prompt.label.to_string(),
                    sample_id: cell.sample.sample_id.clone(),
                    outcome: "budget_halt".to_string(),
                    answer_correct: None,
                    predicted_answer: None,
                    prompt_tokens: 0,
                    completion_tokens: 0,
                    work_tx_id: None,
                    l4e_submit_id: None,
                    provider_handle_capsule_cid: String::new(),
                    proposal_telemetry_cid: None,
                    note: Some(format!("FC2-HALT fuel-exhausted: {}", reject_class_label())),
                });
                break;
            }
            v => {
                if let BudgetVerdict::Within {
                    spend_micro,
                    ceiling_micro,
                } = v
                {
                    println!(
                        "livefc1_swarm_runner: cell {ci} ({}) WITHIN budget spend={spend_micro} ceiling={ceiling_micro}",
                        cell.cell_id
                    );
                }
            }
        }

        // ── Phase-6 brand-GENERIC provider handle capsule on canonical CAS ──
        // External descriptor (brand-laden) is hashed into the generic handle
        // and DROPPED; only the opaque sha256 handle lands on CAS.
        let external_descriptor = format!(
            "{}::{}::temp{}",
            cell.provider.brand_provider, cell.provider.brand_model, cell.temperature_milli
        );
        let (handle_cid, sidecar) = {
            let mut cas = CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
            write_provider_handle_capsule(
                &mut cas,
                &cell.agent_id,
                &external_descriptor,
                cell.provider.brand_model,
                cell.provider.brand_provider,
                logical_t,
            )
            .map_err(|e| format!("write provider handle capsule: {e}"))?
        };
        brand_rows.push(BrandSidecarRow::from(sidecar.clone()));
        let provider_handle = sidecar.model_handle.clone();

        // ── Deliberate FAULT injection on exactly one cell ───────────────────
        // Cell index 3 uses an UNROUTABLE model id → the proxy returns a non-200
        // → the resilient client yields a DriverError. We record the
        // token-spent (here: zero, the call never completed) attempt as a REAL
        // L4.E `LlmError` rejection record — NOT a crash.
        let inject_fault = ci == 3;
        let model_id = if inject_fault {
            "this-model-does-not-exist-livefc1-fault"
        } else {
            cell.provider.brand_model
        };

        let prompt = build_prompt(&cell.sample, cell.prompt.style);
        let prompt_sha = sha256_hex(&prompt);
        let temp_f = cell.temperature_milli as f64 / 1000.0;
        let gen = client
            .generate(&GenerateRequest {
                model: model_id.to_string(),
                messages: vec![
                    Message {
                        role: "system".into(),
                        content: "You solve MATH competition problems. Return strict JSON with fields final_answer and rationale.".into(),
                    },
                    Message {
                        role: "user".into(),
                        content: prompt,
                    },
                ],
                temperature: Some(temp_f),
                max_tokens: Some(900),
            })
            .await;

        match gen {
            Err(err) => {
                // Provider error (the injected fault, OR a real 429/timeout). GOOD
                // DATA: record a REAL L4.E `LlmError` rejection — no head advance.
                let class = if inject_fault {
                    "fault_injected_llm_err"
                } else {
                    "llm_err"
                };
                let submit_id = write_l4e_llm_error(
                    &args,
                    &rej_writer,
                    last_root,
                    &cell.agent_id,
                    &cell.sample.sample_id,
                    &prompt_sha,
                    &format!("{err}"),
                )?;
                println!(
                    "livefc1_swarm_runner: cell {ci} ({}) LLM_ERR -> L4.E submit_id={submit_id} ({class}): {err}",
                    cell.cell_id
                );
                results.push(CellResult {
                    cell_id: cell.cell_id.clone(),
                    agent_id: cell.agent_id.clone(),
                    provider_handle: provider_handle.clone(),
                    brand_model: cell.provider.brand_model.to_string(),
                    brand_provider: cell.provider.brand_provider.to_string(),
                    temperature_milli: cell.temperature_milli,
                    prompt_variant: cell.prompt.label.to_string(),
                    sample_id: cell.sample.sample_id.clone(),
                    outcome: class.to_string(),
                    answer_correct: None,
                    predicted_answer: None,
                    prompt_tokens: 0,
                    completion_tokens: 0,
                    work_tx_id: None,
                    l4e_submit_id: Some(submit_id),
                    provider_handle_capsule_cid: handle_cid.hex(),
                    proposal_telemetry_cid: None,
                    note: Some(if inject_fault {
                        "deliberate fault: unroutable model id".into()
                    } else {
                        "real provider failure (recorded as L4.E, not a crash)".into()
                    }),
                });
                continue;
            }
            Ok(resp) => {
                let prompt_tokens = resp.prompt_tokens as u64;
                let completion_tokens = resp.completion_tokens as u64;
                let parsed = parse_answer(&resp.content);
                match parsed {
                    Err(perr) => {
                        // Parse failure on a token-spent response → REAL L4.E
                        // `ParseFailed` rejection (still counts as spend).
                        let submit_id = write_l4e_parse_fail(
                            &args,
                            &rej_writer,
                            last_root,
                            &cell.agent_id,
                            &cell.sample.sample_id,
                            &prompt_sha,
                            &perr,
                        )?;
                        results.push(CellResult {
                            cell_id: cell.cell_id.clone(),
                            agent_id: cell.agent_id.clone(),
                            provider_handle: provider_handle.clone(),
                            brand_model: cell.provider.brand_model.to_string(),
                            brand_provider: cell.provider.brand_provider.to_string(),
                            temperature_milli: cell.temperature_milli,
                            prompt_variant: cell.prompt.label.to_string(),
                            sample_id: cell.sample.sample_id.clone(),
                            outcome: "parse_fail".to_string(),
                            answer_correct: None,
                            predicted_answer: None,
                            prompt_tokens,
                            completion_tokens,
                            work_tx_id: None,
                            l4e_submit_id: Some(submit_id),
                            provider_handle_capsule_cid: handle_cid.hex(),
                            proposal_telemetry_cid: None,
                            note: Some("model output did not parse as JSON".into()),
                        });
                        continue;
                    }
                    Ok((final_answer, rationale)) => {
                        let correct =
                            normalize(&final_answer) == normalize(&cell.sample.expected_answer);
                        // Write ProposalTelemetry (carries REAL token counts) →
                        // referenced by WorkTx.proposal_cid → VPPUT cost reconstructs.
                        let (work_tx_id, proposal_cid, new_root) = submit_work_spine(
                            &seq,
                            &mut keypairs,
                            &args,
                            &cell.agent_id,
                            &cell.sample.sample_id,
                            &prompt_sha,
                            &rationale,
                            TokenCounts {
                                prompt_tokens,
                                completion_tokens,
                                tool_tokens: 0,
                            },
                            last_root,
                            ci as u64,
                        )
                        .await?;
                        last_root = new_root;
                        println!(
                            "livefc1_swarm_runner: cell {ci} ({}) OK work_tx={work_tx_id} correct={correct} pt={prompt_tokens} ct={completion_tokens}",
                            cell.cell_id
                        );
                        results.push(CellResult {
                            cell_id: cell.cell_id.clone(),
                            agent_id: cell.agent_id.clone(),
                            provider_handle: provider_handle.clone(),
                            brand_model: cell.provider.brand_model.to_string(),
                            brand_provider: cell.provider.brand_provider.to_string(),
                            temperature_milli: cell.temperature_milli,
                            prompt_variant: cell.prompt.label.to_string(),
                            sample_id: cell.sample.sample_id.clone(),
                            outcome: "ok".to_string(),
                            answer_correct: Some(correct),
                            predicted_answer: Some(final_answer),
                            prompt_tokens,
                            completion_tokens,
                            work_tx_id: Some(work_tx_id),
                            l4e_submit_id: None,
                            provider_handle_capsule_cid: handle_cid.hex(),
                            proposal_telemetry_cid: Some(proposal_cid.hex()),
                            note: None,
                        });
                    }
                }
            }
        }
    }

    // ── FC2 map-reduce tick + terminal (boot/tick/terminal nodes fire) ──────
    let pre_tick_root = seq
        .q_snapshot()
        .map_err(|e| format!("q_snapshot pre-tick: {e:?}"))?
        .state_root_t;
    let fc2_tick = seq
        .emit_system_tx(SystemEmitCommand::MapReduceTick {
            tick_kind: TickKind::Scheduled,
        })
        .await
        .map(|_| true)
        .map_err(|e| format!("emit MapReduceTick: {e:?}"))?;
    let after_tick = tb8_await_state_root_advance(&seq, pre_tick_root, 8_000)
        .await
        .unwrap_or(pre_tick_root);

    // TerminalSummary for the run. This is a TASK workload (no Lean oracle), so
    // outcome is HONESTLY a non-omega terminal (the run reached its end without a
    // verified golden path). We use MaxTxExhausted as the honest run-end class.
    // The terminal references the LAST real on-tape task (the most recent Work
    // cell) so it names a task that actually exists in task_markets_t.
    let last_real_task = results
        .iter()
        .rev()
        .find(|c| c.work_tx_id.is_some())
        .and_then(|c| c.work_tx_id.as_ref())
        .map(|wid| {
            // worktx-<task>-livefc1-swarm -> recover the <task> portion.
            wid.strip_prefix("worktx-")
                .and_then(|s| s.strip_suffix("-livefc1-swarm"))
                .unwrap_or(wid)
                .to_string()
        })
        .unwrap_or_else(|| format!("math:{}", sanitize(&samples[0].sample_id)));
    let terminal_task = TaskId(last_real_task);
    let mut hist: BTreeMap<turingosv4::state::typed_tx::RejectionClass, u32> = BTreeMap::new();
    let l4e_total = rej_writer
        .read()
        .map_err(|_| "rejection writer lock poisoned".to_string())?
        .len();
    if l4e_total > 0 {
        // The L4.E LlmError/ParseFailed classes live in the rejection-evidence
        // taxonomy; the TerminalSummary histogram uses the typed_tx taxonomy, so
        // the run-level failure rollup records them under the generic `Opaque`
        // class (the honest catch-all for externalized LLM/parse failures).
        hist.insert(
            turingosv4::state::typed_tx::RejectionClass::Opaque,
            l4e_total as u32,
        );
    }
    let terminal_receipt = tb11_emit_terminal_summary_for_run(
        &seq,
        RunId(args.run_id.clone()),
        terminal_task,
        RunOutcome::MaxTxExhausted,
        results.len() as u32,
        hist,
        logical_t + 1,
        Some(AgentId(solver_agents[0].clone())),
        None,
    )
    .await
    .map_err(|e| format!("emit TerminalSummary: {e:?}"))?;
    let _ = terminal_receipt;
    // Confirm the terminal landed by awaiting a state-root advance (the terminal
    // mutator advances the root). If it does not advance, the terminal did NOT
    // commit to L4 — report honestly rather than claim FC2-terminal fired.
    let fc2_terminal = tb8_await_state_root_advance(&seq, after_tick, 8_000)
        .await
        .is_ok();

    // Drain + shutdown the chaintape.
    let seq_handle = seq.clone();
    bundle
        .shutdown()
        .await
        .map_err(|e| format!("chaintape shutdown: {e}"))?;
    let _ = seq_handle.q_snapshot();

    // Write the genesis_report.json so off-tape audit/replay can resolve identities.
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

    // Final tape spend (post-run).
    let final_spend = current_spend(&args, &cfg)?;

    let sidecar = RunSidecar {
        schema_version: "turingosv4.livefc1.swarm_run_sidecar.v1",
        run_id: args.run_id.clone(),
        llm_proxy_url: args.llm_proxy_url.clone(),
        budget_ceiling_micro_units: ceiling.micro_units(),
        final_spend_tokens: final_spend,
        cells: results.clone(),
        brand_sidecar: brand_rows,
        fc2_map_reduce_tick_emitted: fc2_tick,
        fc2_terminal_summary_emitted: fc2_terminal,
        notes: vec![
            "Provider identity on canonical CAS is the brand-GENERIC ProviderHandleCapsule (opaque sha256 handle).",
            "The brand->handle mapping lives ONLY in brand_sidecar (external); no brand name on the canonical tape.",
            "VPPUT progress is honestly 0 for math tasks: no Lean oracle => no VerificationResult.verified ground-truth witness.",
            "Injected fault + any real 429/timeout land on the tape as L4.E (LlmError) — not a crash.",
        ],
    };
    write_pretty(&args.out_dir.join("swarm_run_sidecar.json"), &sidecar)?;

    println!(
        "livefc1_swarm_runner: DONE cells={} ok={} l4e={} final_spend_tokens={} sidecar={}",
        results.len(),
        results.iter().filter(|c| c.outcome == "ok").count(),
        l4e_total,
        final_spend,
        args.out_dir.join("swarm_run_sidecar.json").display()
    );
    Ok(())
}

struct MatrixCell {
    cell_id: String,
    agent_id: String,
    provider: ProviderCfg,
    temperature_milli: u64,
    prompt: PromptVariant,
    sample: MathSample,
}

/// Reconstruct cumulative live tape spend via the Phase-5 helper (which itself
/// reuses the Phase-2 VPPUT C_i cost reconstruction). Loads the tape read-only.
fn current_spend(args: &Args, cfg: &RuntimeChaintapeConfig) -> Result<u64, String> {
    let _ = cfg;
    let tape = match load_tape_ro(args) {
        Ok(t) => t,
        // A not-yet-populated tape (first cell, before any L4 entry) reads as 0
        // spend — the honest unlimited-start case.
        Err(_) => return Ok(0),
    };
    Ok(loaded_tape_spend_tokens(&tape))
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

#[allow(clippy::too_many_arguments)]
async fn submit_work_spine(
    seq: &turingosv4::state::sequencer::Sequencer,
    keypairs: &mut AgentKeypairRegistry,
    args: &Args,
    agent: &str,
    sample_id: &str,
    prompt_sha: &str,
    rationale: &str,
    tokens: TokenCounts,
    parent_root: Hash,
    ord: u64,
) -> Result<(String, Cid, Hash), String> {
    // Write a small evaluation payload + ProposalTelemetry to CAS.
    let eval_cid = {
        let payload = serde_json::json!({
            "schema": "turingosv4.livefc1.swarm_eval.v1",
            "sample_id": sample_id,
            "rationale_len": rationale.chars().count(),
        });
        let bytes = serde_json::to_vec(&payload).map_err(|e| format!("ser eval: {e}"))?;
        let mut cas = CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
        cas.put(
            &bytes,
            ObjectType::ProposalPayload,
            "swarm-eval",
            ord,
            Some("turingosv4.livefc1.swarm_eval.v1".to_string()),
        )
        .map_err(|e| format!("put eval: {e}"))?
    };
    let proposal_cid = {
        let tel = ProposalTelemetry::new_root(
            AgentId(agent.to_string()),
            hash_from_hex(prompt_sha)?,
            eval_cid,
            "swarm_math".to_string(),
            tokens,
            format!("{agent}.swarm.b{ord}"),
        );
        let mut cas = CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
        write_proposal_telemetry_to_cas(&mut cas, &tel, "swarm-proposal-telemetry", ord)
            .map_err(|e| format!("write ProposalTelemetry: {e}"))?
    };

    let task = format!("math:{}-{}", sanitize(sample_id), ord);
    let task_open = make_real_task_open_signed_by(
        keypairs,
        &task,
        SPONSOR_AGENT,
        parent_root,
        "livefc1-swarm",
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
        &task,
        SPONSOR_AGENT,
        TASK_ESCROW_MICRO,
        after_open,
        "livefc1-swarm",
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
        &task,
        agent,
        after_escrow,
        WORK_STAKE_MICRO,
        "livefc1-swarm",
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

    Ok((work_tx_id, proposal_cid, after_work))
}

type SharedRejectionWriter = std::sync::Arc<std::sync::RwLock<RejectionEvidenceWriter>>;

/// Append a REAL L4.E `LlmError` rejection record THROUGH the sequencer's own
/// shared rejection writer (the single owner of `rejections.jsonl`). The
/// token-spent attempt is preserved as evidence (not a crash). The raw error
/// string is shielded behind a CAS handle; only a bounded public summary crosses
/// the agent boundary.
fn write_l4e_llm_error(
    args: &Args,
    rej: &SharedRejectionWriter,
    parent_state_root: Hash,
    agent: &str,
    sample_id: &str,
    prompt_sha: &str,
    raw_err: &str,
) -> Result<u64, String> {
    write_l4e(
        args,
        rej,
        parent_state_root,
        agent,
        sample_id,
        prompt_sha,
        raw_err,
        RejectionClass::LlmError,
        "llm_err",
    )
}

fn write_l4e_parse_fail(
    args: &Args,
    rej: &SharedRejectionWriter,
    parent_state_root: Hash,
    agent: &str,
    sample_id: &str,
    prompt_sha: &str,
    raw_err: &str,
) -> Result<u64, String> {
    write_l4e(
        args,
        rej,
        parent_state_root,
        agent,
        sample_id,
        prompt_sha,
        raw_err,
        RejectionClass::ParseFailed,
        "parse_fail",
    )
}

#[allow(clippy::too_many_arguments)]
fn write_l4e(
    args: &Args,
    rej: &SharedRejectionWriter,
    parent_state_root: Hash,
    agent: &str,
    sample_id: &str,
    prompt_sha: &str,
    raw_err: &str,
    class: RejectionClass,
    public_class: &str,
) -> Result<u64, String> {
    // Shield the raw diagnostic behind a CAS handle (never agent-facing).
    let mut cas = CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
    let raw_payload = serde_json::json!({
        "schema": "turingosv4.livefc1.swarm_rejection_diag.v1",
        "sample_id": sample_id,
        "prompt_sha256": prompt_sha,
        "raw_error": raw_err,
    });
    let raw_bytes = serde_json::to_vec(&raw_payload).map_err(|e| format!("ser diag: {e}"))?;
    let raw_cid = cas
        .put(
            &raw_bytes,
            ObjectType::Generic,
            "swarm-rejection-diag",
            0,
            Some("turingosv4.livefc1.swarm_rejection_diag.v1".to_string()),
        )
        .map_err(|e| format!("put diag: {e}"))?;
    // The rejected source payload reference: a minimal capsule (the attempt
    // payload) anchored to CAS so the L4.E row points at a real object.
    let attempt_payload = serde_json::json!({
        "schema": "turingosv4.livefc1.swarm_failed_attempt.v1",
        "sample_id": sample_id,
        "agent_id": agent,
        "public_class": public_class,
    });
    let attempt_bytes =
        serde_json::to_vec(&attempt_payload).map_err(|e| format!("ser att: {e}"))?;
    let attempt_cid = cas
        .put(
            &attempt_bytes,
            ObjectType::Generic,
            "swarm-failed-attempt",
            0,
            Some("turingosv4.livefc1.swarm_failed_attempt.v1".to_string()),
        )
        .map_err(|e| format!("put attempt: {e}"))?;

    // Append THROUGH the sequencer's own shared rejection writer so there is a
    // SINGLE coherent L4.E chain on `rejections.jsonl` (the sequencer also
    // appends its own admission rejections to this same writer — two independent
    // writers on one file would break the prev_hash chain on reload).
    let mut writer = rej
        .write()
        .map_err(|_| "rejection writer lock poisoned".to_string())?;
    let submit_id = writer.len() as u64 + 1;
    // IMPORTANT: `raw_diagnostic_cid` is `#[serde(skip_serializing)]` — it is part
    // of the record HASH but is DROPPED on JSONL persist (RSP-0 in-memory-only).
    // Passing `Some(..)` here would make the reloaded chain fail re-verify (the
    // documented limitation in rejection_evidence.rs). We pass `None` to keep the
    // persisted L4.E chain reload-verifiable, and reference the shielded
    // raw-diagnostic CAS object via the bounded public summary (raw error bytes
    // stay behind the CAS handle, never inline in the L4.E record).
    let _ = raw_cid;
    writer.append_rejected(
        submit_id,
        parent_state_root,
        AgentId(agent.to_string()),
        TxKind::Work,
        attempt_cid,
        class,
        None,
        Some(format!(
            "{public_class}: external attempt failed (token-spent); diag_cas={}",
            raw_cid.hex()
        )),
    );
    let _ = TxId(String::new()); // keep TxId import live (used elsewhere)
    Ok(submit_id)
}

fn build_prompt(s: &MathSample, style: &str) -> String {
    format!(
        "MATH sample id: {}\nSubject: {}\nLevel: {}\n\nProblem:\n{}\n\n{style}\nReturn strict JSON only with fields:\n  final_answer: the final answer only\n  rationale: 2-6 sentences of reasoning.",
        s.sample_id, s.subject, s.level, s.problem
    )
}

fn parse_answer(content: &str) -> Result<(String, String), String> {
    let v = extract_json(content)?;
    let fa = v
        .get("final_answer")
        .or_else(|| v.get("answer"))
        .and_then(|x| x.as_str())
        .ok_or("missing final_answer")?;
    let r = v
        .get("rationale")
        .or_else(|| v.get("reasoning"))
        .and_then(|x| x.as_str())
        .ok_or("missing rationale")?;
    Ok((fa.trim().to_string(), r.trim().to_string()))
}

fn extract_json(content: &str) -> Result<serde_json::Value, String> {
    let t = content.trim();
    if let Ok(v) = serde_json::from_str(t) {
        return Ok(v);
    }
    let s = t.find('{').ok_or("no json object")?;
    let e = t.rfind('}').ok_or("no json terminator")?;
    serde_json::from_str(&t[s..=e]).map_err(|e| format!("parse json: {e}"))
}

fn normalize(a: &str) -> String {
    let t = a.trim();
    let unboxed = boxed(t).unwrap_or(t);
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

fn boxed(a: &str) -> Option<&str> {
    let m = "\\boxed{";
    let start = a.rfind(m)? + m.len();
    let mut depth = 1usize;
    for (off, ch) in a[start..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&a[start..start + off]);
                }
            }
            _ => {}
        }
    }
    None
}

fn sanitize(v: &str) -> String {
    v.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

fn write_pretty<T: Serialize>(path: &PathBuf, value: &T) -> Result<(), String> {
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p).map_err(|e| format!("mkdir {}: {e}", p.display()))?;
    }
    let bytes = serde_json::to_vec_pretty(value).map_err(|e| format!("ser: {e}"))?;
    std::fs::write(path, bytes).map_err(|e| format!("write {}: {e}", path.display()))
}

fn sha256_hex(input: impl AsRef<[u8]>) -> String {
    Sha256::digest(input.as_ref())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
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
