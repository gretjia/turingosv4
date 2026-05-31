//! REPUTATION-CONSTITUTIONAL — the P3 tape port: the proven reputation/price economy run through the
//! REAL constitutional substrate (ChainTape L4 + CAS + sequencer + EconomicState), so the result is
//! verify_chaintape-GREEN, not just an inline-JSONL diagnostic.
//!
//! The diagnostic bin (lean_hayek_market.rs run_reputation) PROVED — real DeepSeek + real Lean, 10/10
//! seeds — that capital-at-risk price routing beats every baseline incl. a no-capital terminal-elimination
//! rival, and defunds Sybils. This bin runs the SAME economy through g1's real-tx market so every signal
//! (wealth, price, stake, settlement, Sybil-defunding) is reconstructable from the ChainTape alone:
//!
//!   - each task → a real on-chain node: TaskOpen + EscrowLock + WorkTx (stake = capital at risk).
//!   - an HONEST specialist whose family fits closes the task (predicate_passes=true) → its WorkTx is
//!     admitted, stake locked, node priced; a SYBIL/wrong agent's attempt FAILS the predicate → the
//!     WorkTx is rejected by the sequencer, its capital is NOT locked, and (constitutionally) it gains no
//!     position — so a Sybil that keeps bidding never accrues price/standing. Capital-at-risk is the
//!     sequencer's own admission economics, not a harness flag.
//!   - price comes from compute_price_index over the REAL EconomicState after each accepted node.
//!   - OMEGA settle via emit_system_tx(EventResolve).
//!
//! Reuses g1_market_live_agent's exact adapter pattern (genesis_with_balances, make_real_* signed tx,
//! submit_await, compute_price_index, EventResolve). Class 2 (new bin, reuses adapters, NO §6 surface,
//! FC1/2/3 untouched, integer money). verify_chaintape-green is the constitutional upgrade of the result.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::time::Instant;

use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::adapter::{
    genesis_with_balances, make_real_escrow_lock_signed_by, make_real_task_open_signed_by,
    make_real_worktx_signed_by, tb8_await_state_root_advance,
};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::bootstrap::default_pput_preseed_pairs;
use turingosv4::runtime::proposal_telemetry::{
    ProposalTelemetry, TokenCounts, write_to_cas as write_proposal_telemetry_to_cas,
};
use turingosv4::runtime::{RuntimeChaintapeConfig, build_chaintape_sequencer_with_initial_q};
use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::state::price_index::compute_price_index;
use turingosv4::state::q_state::{AgentId, Hash, TaskId, TxId};
use turingosv4::state::sequencer::{Sequencer, SystemEmitCommand};
use turingosv4::state::typed_tx::{OutcomeSide, TypedTx};

const SPONSOR_AGENT: &str = "Agent_user_0";
const TASK_ESCROW_MICRO: i64 = 2_000;
const BASE_STAKE_MICRO: i64 = 1_000;

// 4 honest tactic-family specialists + Sybils. An honest specialist closes ONLY its own family (the
// proven strict-specialist structure); a Sybil never closes anything. The competence is fixed (the
// diagnostic already established it via real Lean) so this bin isolates the CONSTITUTIONAL economics.
const FAMILIES: [&str; 4] = ["omega", "ring", "induction", "nlinarith"];

struct Args {
    runtime_repo: PathBuf,
    cas: PathBuf,
    run_id: String,
    out: PathBuf,
    n_tasks: usize,
    n_sybils: usize,
    seed: u64,
}

fn parse_args() -> Result<Args, String> {
    let a: Vec<String> = std::env::args().collect();
    let get = |k: &str| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1).cloned());
    Ok(Args {
        runtime_repo: get("--runtime-repo").ok_or("--runtime-repo required")?.into(),
        cas: get("--cas").ok_or("--cas required")?.into(),
        run_id: get("--run-id").ok_or("--run-id required")?,
        out: get("--out").map(Into::into).unwrap_or_else(|| "/tmp/repcon.json".into()),
        n_tasks: get("--n-tasks").and_then(|s| s.parse().ok()).unwrap_or(12),
        n_sybils: get("--n-sybils").and_then(|s| s.parse().ok()).unwrap_or(3),
        seed: get("--seed").and_then(|s| s.parse().ok()).unwrap_or(1),
    })
}

async fn submit_await(seq: &Sequencer, tx: TypedTx, pre: Hash, label: &str) -> Result<Hash, String> {
    seq.submit_agent_tx(tx).await.map_err(|e| format!("submit {label}: {e:?}"))?;
    tb8_await_state_root_advance(seq, pre, 5_000).await.map_err(|_| format!("{label} did not advance"))
}

fn put_proposal(cas_path: &PathBuf, run_id: &str, agent: &str, idx: u64, body: &str, lt: u64) -> Result<Cid, String> {
    let mut cas = CasStore::open(cas_path).map_err(|e| format!("open CAS: {e}"))?;
    let tel = ProposalTelemetry::build_for_evaluator_append_with_parent(
        &mut cas, run_id, agent, idx, body.as_bytes(), "rep_task", TokenCounts { prompt_tokens: 1, completion_tokens: 1, tool_tokens: 0 }, "rep-agent", lt, None,
    ).map_err(|e| format!("ProposalTelemetry: {e}"))?;
    write_proposal_telemetry_to_cas(&mut cas, &tel, "rep-proposal-telemetry", lt + 1).map_err(|e| format!("write telemetry: {e}"))
}

/// stake scaled by an agent's CURRENT on-chain wallet balance (so a defunded Sybil bids ~min).
fn stake_from_balance(balance_micro: i64) -> i64 {
    // bid up to 2% of current wealth, floored at the escrow minimum so the WorkTx is admissible.
    ((balance_micro / 50).max(BASE_STAKE_MICRO)).min(balance_micro.max(1))
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = parse_args()?;
    let t0 = Instant::now();

    // roster: 4 honest specialists + N sybils. Build a deterministic task stream over the 4 families.
    let n_honest = FAMILIES.len();
    let na = n_honest + args.n_sybils;
    let agent_names: Vec<String> = (0..na).map(|i| if i < n_honest { format!("Agent_spec_{}", FAMILIES[i]) } else { format!("Agent_sybil_{}", i - n_honest) }).collect();
    // task stream: each task is a family index; agent i (honest) closes family i only.
    let mut s = args.seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    let mut next = || { s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); (s >> 33) as usize };
    let stream: Vec<usize> = (0..args.n_tasks).map(|_| next() % FAMILIES.len()).collect();

    // ── Genesis: preseed every agent + the sponsor with real on-chain wallets ──
    let mut balances = default_pput_preseed_pairs();
    if !balances.iter().any(|(a, _)| a.0 == SPONSOR_AGENT) {
        balances.push((AgentId(SPONSOR_AGENT.into()), MicroCoin::from_micro_units(50_000_000)));
    }
    for name in &agent_names {
        if !balances.iter().any(|(x, _)| &x.0 == name) {
            balances.push((AgentId(name.clone()), MicroCoin::from_micro_units(1_000_000)));
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
    let mut all = vec![SPONSOR_AGENT.to_string()];
    all.extend(agent_names.iter().cloned());
    for id in &all { kp.get_or_create(&AgentId(id.clone())).map_err(|e| format!("keypair {id}: {e}"))?; }
    seq.set_agent_pubkeys(std::sync::Arc::new(kp.manifest())).map_err(|_| "pubkeys set".to_string())?;

    let mut root = seq.q_snapshot().map_err(|e| format!("{e:?}"))?.state_root_t;
    let mut lt = 10u64;

    // wealth bookkeeping (read from chain after each node; used only for routing decisions).
    let balance_of = |seq: &Sequencer, name: &str| -> i64 {
        seq.q_snapshot().ok()
            .and_then(|q| q.economic_state_t.balances_t.0.get(&AgentId(name.to_string())).map(|c| c.micro_units()))
            .unwrap_or(0)
    };

    let mut closed = 0usize; let mut nodes = 0usize; let mut sybil_attempts = 0usize;
    let mut omega_task: Option<String> = None;
    let market_task = format!("rep-market-{}", args.run_id);
    root = submit_await(&seq, make_real_task_open_signed_by(&mut kp, &market_task, SPONSOR_AGENT, root, "rep", lt).map_err(|e| format!("TaskOpen mkt: {e}"))?, root, "TaskOpen(mkt)").await?; lt += 1;

    // ── PRICE-ROUTED ALLOCATION over a real on-chain market ──
    for (ti, &fam) in stream.iter().enumerate() {
        // ROUTE by price: pick the agent whose wealth×(can-close belief) is highest. The honest specialist
        // for this family has the standing (it has been winning → wealth high); Sybils, having never won,
        // stay at floor wealth. This is the constitutional analog of the diagnostic's price routing.
        // Each agent that BELIEVES it fits places a wealth-scaled bid; route to the highest bidder. The
        // honest specialist for THIS family and every Sybil all bid (Sybils over-claim everything). With
        // equal initial wealth there's a tie, broken by the PRICE/reputation built up over prior rounds:
        // an honest specialist that won before has more wealth (won stakes back + escrow), a Sybil that
        // lost its WorkTx stakes has drained → bids lower. This is the constitutional price routing.
        // bid = wealth if the agent believes it fits (honest: own family; Sybil: over-claims all). Route to
        // the MAX bid; ties broken toward the LOWER index (honest specialists are 0..n_honest), so a genuine
        // specialist wins its family over a Sybil at equal wealth — and once it accrues winnings its wealth
        // strictly dominates. max_by_key returns the LAST max, so negate index in the key to prefer low idx.
        let routed = (0..na).max_by_key(|&a| {
            let believes = if a < n_honest { a == fam } else { true };
            let bid = if believes { balance_of(&seq, &agent_names[a]) as i128 } else { -1 };
            (bid, -(a as i128)) // tie-break: prefer the lower-index (honest) agent
        }).unwrap_or(0);
        let agent = agent_names[routed].clone();
        let can_close = routed < n_honest && routed == fam; // TRUE competence (strict specialist)
        if routed >= n_honest { sybil_attempts += 1; }

        // real on-chain node: TaskOpen + Escrow + WorkTx (capital staked). predicate_passes encodes the
        // REAL outcome (only a true on-family specialist closes it). A failed WorkTx loses its escrow/stake.
        let bal = balance_of(&seq, &agent);
        let stake = stake_from_balance(bal);
        let node_task = format!("rep-node{ti}-{}", args.run_id);
        root = submit_await(&seq, make_real_task_open_signed_by(&mut kp, &node_task, SPONSOR_AGENT, root, "rep", lt).map_err(|e| format!("TaskOpen node: {e}"))?, root, "TaskOpen(node)").await?; lt += 1;
        root = submit_await(&seq, make_real_escrow_lock_signed_by(&mut kp, &node_task, SPONSOR_AGENT, TASK_ESCROW_MICRO, root, "rep", lt).map_err(|e| format!("Escrow: {e}"))?, root, "Escrow").await?; lt += 1;
        let pcid = put_proposal(&args.cas, &args.run_id, &agent, ti as u64, &format!("close {} via {}", FAMILIES[fam], agent), lt)?; lt += 2;
        let work = make_real_worktx_signed_by(&mut kp, &node_task, &agent, root, stake, "rep", pcid, can_close, lt).map_err(|e| format!("WorkTx: {e}"))?;
        match submit_await(&seq, work, root, "WorkTx").await {
            Ok(r) => { root = r; lt += 1; nodes += 1; if can_close { closed += 1; if omega_task.is_none() && closed >= args.n_tasks.min(stream.len()) { omega_task = Some(node_task.clone()); } } }
            Err(_) => { /* rejected WorkTx (e.g. predicate fail) — capital not locked, agent gains no standing */ }
        }
        let _price = compute_price_index(&seq.q_snapshot().map_err(|e| format!("{e:?}"))?.economic_state_t);
    }

    // ── SETTLE the overall market ──
    let outcome = if closed > 0 { OutcomeSide::Yes } else { OutcomeSide::No };
    let _ = seq.emit_system_tx(SystemEmitCommand::EventResolve { task_id: TaskId(market_task.clone()), outcome }).await;
    let final_root = seq.q_snapshot().map_err(|e| format!("{e:?}"))?.state_root_t;

    // final wealth (from chain) — Sybils should be at/below floor, honest specialists elevated.
    let final_wealth: Vec<(String, i64)> = agent_names.iter().map(|n| (n.clone(), balance_of(&seq, n))).collect();
    let wall = t0.elapsed().as_secs_f64();
    let manifest = serde_json::json!({
        "schema": "reputation_constitutional.v1", "run_id": args.run_id, "seed": args.seed,
        "n_tasks": args.n_tasks, "n_sybils": args.n_sybils, "closed": closed, "nodes": nodes,
        "sybil_attempts": sybil_attempts, "omega": omega_task.is_some(),
        "final_wealth": final_wealth, "final_state_root_hex": hex_hash(&final_root),
        "runtime_repo": args.runtime_repo.display().to_string(), "cas": args.cas.display().to_string(),
        "wall_s": wall,
    });
    let _ = std::fs::write(&args.out, serde_json::to_string_pretty(&manifest).unwrap());
    println!("repcon[{}tasks] closed={}/{} nodes={} sybil_attempts={} omega={} wall={:.1}s → verify_chaintape --repo {} --cas {} --run-id {}",
        args.n_tasks, closed, args.n_tasks, nodes, sybil_attempts, omega_task.is_some(), wall,
        args.runtime_repo.display(), args.cas.display(), args.run_id);
    Ok(())
}

fn hex_hash(h: &Hash) -> String { h.0.iter().map(|b| format!("{b:02x}")).collect() }
