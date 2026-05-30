//! G1 — Live-LLM multi-agent constitutional market (real price discovery + OMEGA + PPUT).
//!
//! The capability layer above G0: N LIVE DeepSeek agents (via the local proxy,
//! NO scripted stubs) collaboratively build a ζ-regularization (sum of naturals
//! = -1/12) proof DAG on the canonical ChainTape. Each agent reads a SHIELDED
//! market view, proposes a proof STEP with a self-reported confidence, a permissive
//! ζ step-judge (mirrors v3's OfflineHeuristicJudge) verdicts it, and an accepted
//! step becomes a priced node (per-task model → no monetary_invariant) whose
//! WorkTx-Long stake is SCALED BY CONFIDENCE — so prices DISCOVER (unlike G0's
//! flat 0.67) and a ChallengeTx-Short gives each node a price_yes. OMEGA = a step
//! containing "[COMPLETE]" and "-1/12". PPUT = golden-path tokens / (total tokens
//! × wall-clock). Class 2 (new binary; reuses existing adapter helpers; no §6).
//!
//! Reproduces (and on the v4 substrate aims to exceed) the v3 ζ Run-6 emergent DAG.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use rand::SeedableRng;
use rand::rngs::StdRng;
use serde::Serialize;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::adapter::{
    genesis_with_balances, make_real_challengetx_signed_by, make_real_cpmm_pool_signed_by,
    make_real_escrow_lock_signed_by, make_real_market_seed_signed_by, make_real_task_open_signed_by,
    make_real_worktx_signed_by, tb8_await_state_root_advance,
};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::bootstrap::default_pput_preseed_pairs;
use turingosv4::runtime::genesis_report::GenesisReport;
use turingosv4::runtime::proposal_telemetry::{
    ProposalTelemetry, TokenCounts, write_to_cas as write_proposal_telemetry_to_cas,
};
use turingosv4::runtime::{RuntimeChaintapeConfig, build_chaintape_sequencer_with_initial_q};
use turingosv4::sdk::actor::boltzmann_select_parent_v2;
use turingosv4::state::BoltzmannMaskPolicy;
use turingosv4::state::price_index::compute_price_index;
use turingosv4::state::q_state::{AgentId, Hash, TaskId, TaskMarketState, TxId};
use turingosv4::state::sequencer::{Sequencer, SystemEmitCommand};
use turingosv4::state::typed_tx::{OutcomeSide, TypedTx};

const SPONSOR_AGENT: &str = "Agent_user_0";
const PROVIDER_AGENT: &str = "Agent_user_1";
const MARKET_SEED_MICRO: i64 = 100_000;
const TASK_ESCROW_MICRO: i64 = 2_000; // only needs >0 for WorkTx admission; small → many nodes
const CHALLENGE_STAKE_MICRO: i64 = 500;
const MIN_STAKE_MICRO: i64 = 250;
const MAX_STAKE_MICRO: i64 = 20_000;
const BASE_WORK_STAKE: i64 = 1_000;

const ZETA_TASK: &str = "Prove that the Ramanujan/zeta-regularized sum 1+2+3+... = -1/12, \
using the hint M(m,N) = m*exp(-m/N)*cos(m/N) and S(N) = sum_{m>=0} M(m,N), then take N->infinity.";

struct Args {
    runtime_repo: PathBuf,
    cas: PathBuf,
    run_id: String,
    constitution: PathBuf,
    out: PathBuf,
    proxy_url: String,
    model: String,
    n_agents: usize,
    n_rounds: usize,
    continue_past_omega: bool,
    strict: bool,
}

#[derive(Debug, Clone, Serialize)]
struct StepNode {
    node_tx: String,
    task: String,
    by_agent: String,
    parent_tx: Option<String>,
    confidence_pct: u64,
    work_stake_micro: i64,
    price_yes_num: Option<u128>,
    price_yes_den: Option<u128>,
    step_preview: String,
    tokens: u64,
    is_omega: bool,
}

#[derive(Debug, Serialize)]
struct G1Manifest {
    schema_version: &'static str,
    run_id: String,
    model: String,
    n_agents: usize,
    n_rounds: usize,
    llm_calls: usize,
    parse_fails: usize,
    judge_pass: usize,
    judge_reject: usize,
    nodes: Vec<StepNode>,
    distinct_price_ratios: usize,
    price_discovery: bool,
    omega_reached: bool,
    omega_node: Option<String>,
    golden_path: Vec<String>,
    golden_path_tokens: u64,
    total_tokens: u64,
    wall_clock_s: f64,
    progress_pct: f64,
    pput: f64,
    final_state_root_hex: String,
    runtime_repo: String,
    cas: String,
    notes: Vec<String>,
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut m: BTreeMap<String, String> = BTreeMap::new();
    let mut i = 0;
    while i < argv.len() {
        let k = &argv[i];
        if let Some(stripped) = k.strip_prefix("--") {
            let v = argv.get(i + 1).cloned().ok_or(format!("missing value after {k}"))?;
            m.insert(stripped.to_string(), v);
            i += 2;
        } else {
            return Err(format!("unexpected arg {k}"));
        }
    }
    let get = |k: &str| m.get(k).cloned();
    let runtime_repo: PathBuf = get("runtime-repo").ok_or("--runtime-repo required")?.into();
    Ok(Args {
        out: get("out").map(Into::into).unwrap_or_else(|| runtime_repo.join("g1_manifest.json")),
        runtime_repo,
        cas: get("cas").ok_or("--cas required")?.into(),
        run_id: get("run-id").ok_or("--run-id required")?,
        constitution: get("constitution").ok_or("--constitution required")?.into(),
        proxy_url: get("proxy-url").unwrap_or_else(|| "http://localhost:8123".into()),
        model: get("model").unwrap_or_else(|| "deepseek-chat".into()),
        n_agents: get("n-agents").and_then(|s| s.parse().ok()).unwrap_or(5),
        n_rounds: get("n-rounds").and_then(|s| s.parse().ok()).unwrap_or(6),
        continue_past_omega: get("continue-past-omega").map(|s| s == "true").unwrap_or(true),
        strict: get("strict").map(|s| s == "true").unwrap_or(false),
    })
}

fn hash_hex(h: &Hash) -> String {
    h.0.iter().map(|b| format!("{b:02x}")).collect()
}

/// Deterministic ζ step-judge (NO LLM, NO market data — shielded). Returns
/// (pass, claims_complete). claims_complete = the step asserts the result; the
/// loop additionally requires (in --strict mode) that the accepted chain has
/// passed the real derivation milestones before OMEGA fires.
fn zeta_judge(step: &str) -> (bool, bool) {
    let s = step.to_ascii_lowercase();
    let claims_complete = step.contains("[COMPLETE]") && s.contains("-1/12");
    let signals = [
        "s(n)", "m(m,n)", "abel", "cesaro", "cesàro", "euler", "maclaurin", "asymptot",
        "regulariz", "analytic", "continuation", "zeta", "-1/12", "limit", "sum", "series",
    ];
    let pass = claims_complete || (step.trim().len() > 12 && signals.iter().any(|w| s.contains(w)));
    (pass, claims_complete)
}

/// Which derivation milestone(s) a step covers (for the --strict verifier).
fn step_milestones(step: &str) -> Vec<&'static str> {
    let s = step.to_ascii_lowercase();
    let mut ms = Vec::new();
    if s.contains("s(n)") || s.contains("regulariz") || s.contains("define") { ms.push("def_S"); }
    if s.contains("(1-x)") || s.contains("x^m") || s.contains("geometric") || s.contains("x/(1") || s.contains("sum_{m") { ms.push("series"); }
    if s.contains("real part") || s.contains("re[") || s.contains("euler") || s.contains("asymptot") || s.contains("abel") || s.contains("maclaurin") || s.contains("cesaro") { ms.push("asymptotic"); }
    ms
}

fn stake_from_confidence(confidence_pct: u64) -> i64 {
    // integer money path: multiplier 0.25x..4.0x of BASE over confidence 0..100.
    let mult_num = (25 + 375 * confidence_pct.min(100) as i64 / 100).max(25); // 25..400
    (BASE_WORK_STAKE.saturating_mul(mult_num) / 100).clamp(MIN_STAKE_MICRO, MAX_STAKE_MICRO)
}

fn extract_json_object(content: &str) -> Option<serde_json::Value> {
    let t = content.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
    if let Ok(v) = serde_json::from_str(t) {
        return Some(v);
    }
    let start = t.find('{')?;
    let end = t.rfind('}')?;
    serde_json::from_str(&t[start..=end]).ok()
}

async fn submit_await(seq: &Sequencer, tx: TypedTx, pre: Hash, label: &str) -> Result<Hash, String> {
    seq.submit_agent_tx(tx).await.map_err(|e| format!("submit {label}: {e:?}"))?;
    tb8_await_state_root_advance(seq, pre, 5_000).await.map_err(|_| format!("{label} did not advance"))
}

fn put_proposal(cas_path: &PathBuf, run_id: &str, agent: &str, idx: u64, parent: Option<TxId>, step: &str, tokens: TokenCounts, lt: u64) -> Result<Cid, String> {
    let mut cas = CasStore::open(cas_path).map_err(|e| format!("open CAS: {e}"))?;
    let tel = ProposalTelemetry::build_for_evaluator_append_with_parent(
        &mut cas, run_id, agent, idx, step.as_bytes(), "g1_step", tokens, "g1-agent", lt, parent,
    ).map_err(|e| format!("ProposalTelemetry: {e}"))?;
    write_proposal_telemetry_to_cas(&mut cas, &tel, "g1-proposal-telemetry", lt + 1).map_err(|e| format!("write telemetry: {e}"))
}

fn put_counterexample(cas_path: &PathBuf, work_tx: &str, lt: u64) -> Result<Cid, String> {
    let mut cas = CasStore::open(cas_path).map_err(|e| format!("open CAS: {e}"))?;
    let blob = serde_json::json!({"schema":"g1.counterexample.v1","target":work_tx});
    cas.put(serde_json::to_vec(&blob).unwrap().as_slice(), ObjectType::EvidenceCapsule, "g1-challenger", lt, Some("g1.counterexample.v1".into()))
        .map_err(|e| format!("put counterexample: {e}"))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse_args(&argv) {
        Ok(a) => a,
        Err(e) => { eprintln!("g1: {e}"); return ExitCode::from(2); }
    };
    match run(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => { eprintln!("g1: {e}"); ExitCode::from(1) }
    }
}

async fn run(args: Args) -> Result<(), String> {
    let t0 = Instant::now();
    let market_task = format!("g1-market-{}", args.run_id);
    let agents: Vec<String> = (0..args.n_agents).map(|i| format!("Agent_{i}")).collect();
    let challengers: Vec<String> = (0..args.n_agents).map(|i| format!("Chal_{i}")).collect();

    // ── Genesis + keypairs ───────────────────────────────────────────
    let mut balances = default_pput_preseed_pairs();
    for extra in [SPONSOR_AGENT, PROVIDER_AGENT] {
        if !balances.iter().any(|(a, _)| a.0 == extra) {
            balances.push((AgentId(extra.into()), MicroCoin::from_micro_units(5_000_000)));
        }
    }
    for a in agents.iter().chain(challengers.iter()) {
        if !balances.iter().any(|(x, _)| &x.0 == a) {
            balances.push((AgentId(a.clone()), MicroCoin::from_micro_units(5_000_000)));
        }
    }
    let initial_q = genesis_with_balances(&balances);
    let cfg = RuntimeChaintapeConfig {
        runtime_repo_path: args.runtime_repo.clone(), cas_path: args.cas.clone(),
        run_id: args.run_id.clone(), queue_capacity: 64, resume_existing_chain: false,
    };
    let bundle = build_chaintape_sequencer_with_initial_q(&cfg, initial_q).map_err(|e| format!("boot: {e}"))?;
    let seq = bundle.sequencer.clone();
    let mut kp = AgentKeypairRegistry::open(&cfg.runtime_repo_path).map_err(|e| format!("{e}"))?;
    let mut all: Vec<&str> = vec![SPONSOR_AGENT, PROVIDER_AGENT];
    all.extend(agents.iter().map(|s| s.as_str()));
    all.extend(challengers.iter().map(|s| s.as_str()));
    for id in &all { kp.get_or_create(&AgentId(id.to_string())).map_err(|e| format!("keypair {id}: {e}"))?; }
    seq.set_agent_pubkeys(std::sync::Arc::new(kp.manifest())).map_err(|_| "pubkeys set".to_string())?;

    // ── Market task scaffold (for an overall task-outcome pool) ──────
    let mut root = seq.q_snapshot().map_err(|e| format!("{e:?}"))?.state_root_t;
    let mut lt = 10u64;
    root = submit_await(&seq, make_real_task_open_signed_by(&mut kp, &market_task, SPONSOR_AGENT, root, "g1", lt).map_err(|e| format!("TaskOpen: {e}"))?, root, "TaskOpen").await?; lt += 1;
    root = submit_await(&seq, make_real_market_seed_signed_by(&mut kp, root, &market_task, PROVIDER_AGENT, MARKET_SEED_MICRO, "g1", lt).map_err(|e| format!("Seed: {e}"))?, root, "MarketSeed").await?; lt += 1;
    root = submit_await(&seq, make_real_cpmm_pool_signed_by(&mut kp, root, &market_task, PROVIDER_AGENT, MARKET_SEED_MICRO as u128, "g1").map_err(|e| format!("Pool: {e}"))?, root, "CpmmPool").await?; lt += 1;

    let llm = ResilientLLMClient::new(&args.proxy_url, 120, 3);
    let sys = Message { role: "system".into(), content: "You are a TuringOS market agent proving a hard theorem step by step. Return ONLY a JSON object, no markdown.".into() };

    let mut nodes: Vec<StepNode> = Vec::new();
    let mut node_tx_ids: Vec<TxId> = Vec::new();
    let mut accepted_steps: Vec<String> = Vec::new();
    let (mut llm_calls, mut parse_fails, mut judge_pass, mut judge_reject) = (0usize, 0usize, 0usize, 0usize);
    let mut omega_node: Option<String> = None;
    let mut step_idx = 0u64;
    let mut milestones: BTreeSet<&str> = BTreeSet::new();

    'outer: for round in 0..args.n_rounds {
        for ai in 0..agents.len() {
            let agent = agents[ai].clone();
            let q = seq.q_snapshot().map_err(|e| format!("{e:?}"))?;
            root = q.state_root_t;
            let pi = compute_price_index(&q.economic_state_t);
            // Shielded read-view: node ids + prices + recent steps (NO judge internals / other balances).
            let mut market = String::new();
            for n in nodes.iter().rev().take(8) {
                market.push_str(&format!("- node {} price_yes={}/{}\n", &n.node_tx[..n.node_tx.len().min(16)],
                    n.price_yes_num.unwrap_or(0), n.price_yes_den.unwrap_or(1)));
            }
            let recent = accepted_steps.iter().rev().take(3).cloned().collect::<Vec<_>>().join(" | ");
            // Price-driven parent hint (boltzmann argmax + epsilon over the live price_index) → branching.
            let parent_hint = {
                let mut rng = StdRng::seed_from_u64(0xB01 + round as u64 * 31 + ai as u64);
                boltzmann_select_parent_v2(&pi, &BTreeSet::new(), &BoltzmannMaskPolicy::default(), &mut rng)
                    .map(|t| t.0).or_else(|| node_tx_ids.last().map(|t| t.0.clone()))
            };
            let prompt = format!(
                "=== Task ===\n{ZETA_TASK}\n=== Market (price is signal, not truth) ===\n{market}\n=== Recent accepted steps ===\n{recent}\n=== Your turn (round {round}, you are {agent}) ===\n\
Propose the NEXT proof step that advances toward the result. If the proof is finished, your step_text MUST contain \"[COMPLETE]\" and \"-1/12\".\n\
suggested_parent: {pp}\nReturn EXACTLY: {{\"action\":\"propose\",\"parent_node\":\"<node_tx or null>\",\"step_text\":\"<one concise proof step>\",\"confidence\":0.0-1.0}}",
                pp = parent_hint.clone().unwrap_or_else(|| "null".into())
            );
            let resp = match llm.generate(&GenerateRequest {
                model: args.model.clone(),
                messages: vec![sys.clone(), Message { role: "user".into(), content: prompt }],
                temperature: Some(0.7), max_tokens: Some(400),
            }).await {
                Ok(r) => r,
                Err(e) => { eprintln!("g1 llm_err {agent}: {e:?}"); continue; }
            };
            llm_calls += 1;
            let tokens = TokenCounts { prompt_tokens: resp.prompt_tokens as u64, completion_tokens: resp.completion_tokens as u64, tool_tokens: 0 };
            let v = match extract_json_object(&resp.content) {
                Some(v) => v,
                None => { parse_fails += 1; continue; }
            };
            let step_text = v.get("step_text").and_then(|x| x.as_str()).unwrap_or("").to_string();
            if step_text.trim().is_empty() { parse_fails += 1; continue; }
            let confidence_pct = (v.get("confidence").and_then(|x| x.as_f64()).unwrap_or(0.6).clamp(0.0, 1.0) * 100.0) as u64;
            let parent_tx: Option<TxId> = v.get("parent_node").and_then(|x| x.as_str())
                .filter(|s| *s != "null" && !s.is_empty())
                .and_then(|s| node_tx_ids.iter().find(|t| t.0.starts_with(s) || s.starts_with(&t.0[..t.0.len().min(16)])).cloned())
                .or_else(|| node_tx_ids.last().cloned());

            let (pass, claims_complete) = zeta_judge(&step_text);
            if !pass { judge_reject += 1; continue; }
            judge_pass += 1;
            for m in step_milestones(&step_text) { milestones.insert(m); }
            // --strict: OMEGA only after the real derivation milestones (def_S → series → asymptotic) are present.
            let is_omega = claims_complete && (!args.strict || milestones.len() >= 3);

            // Accepted step → its own task node (per-task model → no monetary_invariant).
            let work_stake = stake_from_confidence(confidence_pct);
            let node_task = format!("g1-node{step_idx}-{}", args.run_id);
            root = submit_await(&seq, make_real_task_open_signed_by(&mut kp, &node_task, SPONSOR_AGENT, root, "g1", lt).map_err(|e| format!("TaskOpen node: {e}"))?, root, "TaskOpen(node)").await?; lt += 1;
            root = submit_await(&seq, make_real_escrow_lock_signed_by(&mut kp, &node_task, SPONSOR_AGENT, TASK_ESCROW_MICRO, root, "g1", lt).map_err(|e| format!("Escrow node: {e}"))?, root, "Escrow(node)").await?; lt += 1;
            let pcid = put_proposal(&args.cas, &args.run_id, &agent, step_idx, parent_tx.clone(), &step_text, tokens, lt)?; lt += 2;
            let work = make_real_worktx_signed_by(&mut kp, &node_task, &agent, root, work_stake, "g1", pcid, true, lt).map_err(|e| format!("WorkTx: {e}"))?;
            let work_tx_id = match &work { TypedTx::Work(w) => w.tx_id.0.clone(), _ => return Err("not WorkTx".into()) };
            root = submit_await(&seq, work, root, "WorkTx").await?; lt += 1;
            node_tx_ids.push(TxId(work_tx_id.clone()));
            accepted_steps.push(step_text.clone());
            // Short challenge by a DISTINCT challenger agent → gives the node a price_yes. Non-fatal.
            let challenger = challengers[ai % challengers.len()].clone();
            if let Ok(ce) = put_counterexample(&args.cas, &work_tx_id, lt) {
                lt += 1;
                match make_real_challengetx_signed_by(&mut kp, root, TxId(work_tx_id.clone()), &challenger, CHALLENGE_STAKE_MICRO, ce, "g1", lt) {
                    Ok(chal) => match submit_await(&seq, chal, root, "ChallengeTx").await {
                        Ok(r) => { root = r; lt += 1; }
                        Err(e) => eprintln!("g1 challenge skip node{step_idx}: {e}"),
                    },
                    Err(e) => eprintln!("g1 challenge build skip: {e}"),
                }
            }

            let price = compute_price_index(&seq.q_snapshot().map_err(|e| format!("{e:?}"))?.economic_state_t);
            let pe = price.get(&TxId(work_tx_id.clone()));
            nodes.push(StepNode {
                node_tx: work_tx_id.clone(), task: node_task, by_agent: agent.clone(),
                parent_tx: parent_tx.map(|t| t.0),
                confidence_pct, work_stake_micro: work_stake,
                price_yes_num: pe.and_then(|e| e.price_yes.as_ref().map(|p| p.numerator)),
                price_yes_den: pe.and_then(|e| e.price_yes.as_ref().map(|p| p.denominator)),
                step_preview: step_text.chars().take(80).collect(),
                tokens: tokens.prompt_tokens + tokens.completion_tokens,
                is_omega,
            });
            step_idx += 1;
            if is_omega && omega_node.is_none() { omega_node = Some(work_tx_id.clone()); }
            if is_omega && !args.continue_past_omega { break 'outer; }
        }
    }

    // ── Settlement: Yes if OMEGA reached, else No ────────────────────
    let outcome = if omega_node.is_some() { OutcomeSide::Yes } else { OutcomeSide::No };
    if seq.emit_system_tx(SystemEmitCommand::EventResolve { task_id: TaskId(market_task.clone()), outcome }).await.is_ok() {
        let _ = tb8_await_state_root_advance(&seq, root, 5_000).await;
    }
    let _settled = seq.q_snapshot().map_err(|e| format!("{e:?}"))?.economic_state_t
        .task_markets_t.0.get(&TaskId(market_task.clone())).map(|m| m.state != TaskMarketState::Open).unwrap_or(false);

    let seq_handle = seq.clone();
    bundle.shutdown().await.map_err(|e| format!("shutdown: {e}"))?;
    let final_root = seq_handle.q_snapshot().map_err(|e| format!("{e:?}"))?.state_root_t;
    let _ = GenesisReport {
        constitution_hash: GenesisReport::hash_constitution_md(&args.constitution),
        runtime_repo: args.runtime_repo.display().to_string(), cas_path: args.cas.display().to_string(),
        system_pubkey_hash: GenesisReport::hash_system_pubkey_manifest(&args.runtime_repo),
        agent_pubkeys_path: "agent_pubkeys.json".into(),
        initial_balances: balances.iter().map(|(a, b)| (a.0.clone(), b.micro_units())).collect(),
        task_id: Some(market_task.clone()), task_open_tx: None, escrow_lock_tx: None,
        agent_model_assignment: vec![], model_assignment_manifest_cid: None,
        agent_role_assignment: vec![], role_assignment_manifest_cid: None,
    }.write_to_runtime_repo(&args.runtime_repo);

    // ── Golden path (ancestor chain of OMEGA node) + PPUT ────────────
    let parent_of: BTreeMap<String, Option<String>> = nodes.iter().map(|n| (n.node_tx.clone(), n.parent_tx.clone())).collect();
    let tokens_of: BTreeMap<String, u64> = nodes.iter().map(|n| (n.node_tx.clone(), n.tokens)).collect();
    let mut golden_path: Vec<String> = Vec::new();
    let mut golden_path_tokens = 0u64;
    if let Some(o) = &omega_node {
        let mut cur = Some(o.clone());
        while let Some(c) = cur {
            golden_path.push(c.clone());
            golden_path_tokens += tokens_of.get(&c).copied().unwrap_or(0);
            cur = parent_of.get(&c).cloned().flatten();
        }
        golden_path.reverse();
    }
    let total_tokens: u64 = nodes.iter().map(|n| n.tokens).sum();
    let wall_clock_s = t0.elapsed().as_secs_f64();
    let progress_pct = if omega_node.is_some() { 100.0 } else if !nodes.is_empty() { 1.0 } else { 0.0 };
    // PPUT (architect's definition): golden-path progress (token count) per unit
    // time; ZERO if no completion (no golden path → PPUT=0). Not cost-normalized.
    let pput = if omega_node.is_none() || wall_clock_s <= 0.0 { 0.0 }
        else { golden_path_tokens as f64 / wall_clock_s };

    // price discovery: count distinct price_yes ratios across nodes.
    let mut ratios: BTreeSet<(u128, u128)> = BTreeSet::new();
    for n in &nodes { if let (Some(a), Some(b)) = (n.price_yes_num, n.price_yes_den) { ratios.insert((a, b)); } }
    let distinct_price_ratios = ratios.len();

    let manifest = G1Manifest {
        schema_version: "turingosv4.g1.live_market.v1",
        run_id: args.run_id.clone(), model: args.model.clone(),
        n_agents: args.n_agents, n_rounds: args.n_rounds,
        llm_calls, parse_fails, judge_pass, judge_reject,
        distinct_price_ratios, price_discovery: distinct_price_ratios > 1,
        omega_reached: omega_node.is_some(), omega_node: omega_node.clone(),
        golden_path, golden_path_tokens, total_tokens, wall_clock_s, progress_pct, pput,
        nodes,
        final_state_root_hex: hash_hex(&final_root),
        runtime_repo: args.runtime_repo.display().to_string(), cas: args.cas.display().to_string(),
        notes: vec![
            "LIVE deepseek-chat agents (no scripts); real ChainTape L4 + CAS; confidence-scaled WorkTx-Long stakes → price discovery".into(),
            "ζ step-judge mirrors v3 OfflineHeuristicJudge (OMEGA = step contains [COMPLETE] and -1/12)".into(),
            "PPUT = progress / (total_tokens × wall_clock) × 1e6; progress=100 if OMEGA else 1 if any node else 0".into(),
        ],
    };
    if let Some(p) = args.out.parent() { std::fs::create_dir_all(p).ok(); }
    std::fs::write(&args.out, serde_json::to_string_pretty(&manifest).map_err(|e| format!("ser: {e}"))?).map_err(|e| format!("write: {e}"))?;
    println!(
        "g1_live_market: agents={} rounds={} llm_calls={} pass={} reject={} parse_fail={} nodes={} distinct_prices={} price_discovery={} omega={} golden_path_len={} gp_tokens={} total_tokens={} wall={:.1}s pput={:.4} manifest={}",
        args.n_agents, args.n_rounds, llm_calls, judge_pass, judge_reject, parse_fails, manifest.nodes.len(),
        distinct_price_ratios, manifest.price_discovery, manifest.omega_reached, manifest.golden_path.len(),
        golden_path_tokens, total_tokens, wall_clock_s, pput, args.out.display()
    );
    Ok(())
}
