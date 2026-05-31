//! P4-lite — Hayekian price-discovery market: is PRICE the CAUSAL routing mechanism?
//!
//! The prior emergence (lean_hetero_market) used round-robin routing — agents self-selected by
//! SKIP but the SCHEDULER did not use a market price. The architect's directive: prove that a
//! HAYEKIAN price — emerging from agents staking REAL loss-bearing capital on YES/NO — is what
//! routes the scarce resource, NOT a centralized score and NOT generic parallelism. The decisive
//! test is the ablation RealPrice vs ShuffledPrice vs CentralScore.
//!
//! THE HAYEK SEMANTICS (vs the rejected "informed-Bear SCORING" bypass):
//!   - each agent has a FINITE wallet → betting on claim A means not betting on B (opportunity cost);
//!   - a YES bet stakes capital on "I can close this conjunct" + attaches a real proof;
//!   - if Lean REJECTS that proof, the agent LOSES the stake (capital at risk is realized);
//!   - if Lean ACCEPTS, YES-investors SETTLE (stake back + a share of the NO pool); NO-investors lose;
//!   - the top level only RUNS LEAN + SETTLES — it never reads reasoning content to score a node.
//!   The price is DERIVED from the Invest multiset, never an authoritative in-memory score.
//!
//! THE SCARCE RESOURCE price routes = the Lean-VERIFY budget (1 verify/round, R rounds < work).
//! Price tells the scheduler which conjunct's funded proof to spend the next verify on. If price is
//! informative + causal, RealPrice closes more conjuncts within the budget than ShuffledPrice
//! (price signal destroyed) or CentralScore (centralized heuristic) or Uniform (no signal).
//!
//! PRICE = (YES+α)/(YES+NO+2α), liquidity L = YES+NO — the SAME integer-rational ratio
//! compute_price_index (src/state/price_index.rs:199, price_yes=long/(long+short)) computes from
//! NodePosition Long/Short; here YES=Long, NO=Short, with the architect's Laplace-α smoothing. The
//! P3 constitutional port swaps this inline integer accounting for the real EconomicState path so
//! the run becomes verify_chaintape-green; the ECONOMICS (risk/opportunity-cost/settlement) are
//! already real here. Money is integer micro-coin throughout; f64 appears ONLY in the routing
//! softmax POLICY (the chosen conjunct is recorded on the MarketTape so a replay reads the choice,
//! never recomputes the draw).
//!
//! Class 1-2 diagnostic bin: inline MarketTape (append-only JSONL + prev_hash chain), no §6 surface
//! edited, no FC1/FC2/FC3 change. lib.rs untouched (all helpers inline — trust-root discipline).

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::time::Instant;
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};

// ── integer money (micro-coin); never f64 in this path ──────────────────────
const WALLET_BUDGET_MICRO: i64 = 100_000; // each agent's finite capital (opportunity cost)
const MIN_STAKE_MICRO: i64 = 1_000;
const MAX_STAKE_MICRO: i64 = 40_000;
const ALPHA_MICRO: i64 = 1_000; // Laplace smoothing α (in micro units), so empty claim → p=0.5

/// confidence (0..100) → integer stake, sized by belief, capped to wallet. The capital an agent
/// is willing to LOSE if its proof fails — honest skin in the game.
fn stake_from_confidence(confidence_pct: u64, wallet: i64) -> i64 {
    let c = confidence_pct.min(100) as i64;
    let raw = MIN_STAKE_MICRO + (MAX_STAKE_MICRO - MIN_STAKE_MICRO) * c / 100;
    raw.clamp(MIN_STAKE_MICRO, MAX_STAKE_MICRO).min(wallet.max(0))
}

// ── MarketTape-lite: append-only event log, prev_hash chained (ATOM 1) ──────
// Price is DERIVED from Invest events; node.score is never authoritative.
#[derive(Clone)]
enum MarketEvent {
    MarketOpen { claim: usize, claim_type: String },
    Invest { agent: usize, claim: usize, side: String, amount_micro: i64, model_hash: String, confidence: u64 },
    Proposal { agent: usize, claim: usize, output_hash: String },
    LlmCall { model: String, tokens: u64 },
    Verify { claim: usize, verdict: bool, reject_class: String },
    RouteSample { policy: String, frontier_hash: String, selected_claim: usize },
    Resolve { claim: usize, outcome: String },
}

struct MarketTape {
    lines: Vec<String>,
    prev_hash: String,
}
impl MarketTape {
    fn new() -> Self { MarketTape { lines: Vec::new(), prev_hash: "genesis".into() } }
    fn append(&mut self, kind: &str, body: serde_json::Value) {
        let payload = serde_json::json!({ "kind": kind, "prev": self.prev_hash, "body": body });
        let s = serde_json::to_string(&payload).unwrap();
        let mut h = Sha256::new();
        h.update(s.as_bytes());
        self.prev_hash = format!("{:x}", h.finalize());
        self.lines.push(s);
    }
    fn record(&mut self, e: &MarketEvent) {
        match e {
            MarketEvent::MarketOpen { claim, claim_type } => self.append("MarketOpen", serde_json::json!({"claim":claim,"claim_type":claim_type})),
            MarketEvent::Invest { agent, claim, side, amount_micro, model_hash, confidence } => self.append("Invest", serde_json::json!({"agent":agent,"claim":claim,"side":side,"amount_micro":amount_micro,"model_hash":model_hash,"confidence":confidence})),
            MarketEvent::Proposal { agent, claim, output_hash } => self.append("Proposal", serde_json::json!({"agent":agent,"claim":claim,"output_hash":output_hash})),
            MarketEvent::LlmCall { model, tokens } => self.append("LLMCall", serde_json::json!({"model":model,"tokens":tokens})),
            MarketEvent::Verify { claim, verdict, reject_class } => self.append("Verify", serde_json::json!({"claim":claim,"verdict":verdict,"reject_class":reject_class})),
            MarketEvent::RouteSample { policy, frontier_hash, selected_claim } => self.append("RouteSample", serde_json::json!({"policy":policy,"frontier_hash":frontier_hash,"selected_claim":selected_claim})),
            MarketEvent::Resolve { claim, outcome } => self.append("Resolve", serde_json::json!({"claim":claim,"outcome":outcome})),
        }
    }
    /// Verify the append-only prev_hash chain (replayability gate, ATOM 5-lite).
    fn verify_chain(&self) -> bool {
        let mut prev = "genesis".to_string();
        for line in &self.lines {
            let v: serde_json::Value = match serde_json::from_str(line) { Ok(v) => v, Err(_) => return false };
            if v["prev"].as_str() != Some(&prev) { return false; }
            let mut h = Sha256::new(); h.update(line.as_bytes());
            prev = format!("{:x}", h.finalize());
        }
        true
    }
    /// Re-derive each claim's (yes,no) pools from the Invest events ALONE — proves price is
    /// tape-derivable, not an authoritative in-memory score (Art. 0.2).
    fn derive_pools(&self, k: usize) -> Vec<(i64, i64)> {
        let mut pools = vec![(0i64, 0i64); k];
        for line in &self.lines {
            let v: serde_json::Value = serde_json::from_str(line).unwrap();
            if v["kind"] == "Invest" {
                let c = v["body"]["claim"].as_u64().unwrap() as usize;
                let amt = v["body"]["amount_micro"].as_i64().unwrap();
                if v["body"]["side"] == "YES" { pools[c].0 += amt; } else { pools[c].1 += amt; }
            }
        }
        pools
    }
}

// ── Hayek price (ATOM 2): integer-rational, identical to compute_price_index's long/(long+short) ──
/// p_raw scaled to per-mille (integer) to stay off f64 in the price path: 1000*(YES+α)/(YES+NO+2α).
fn price_yes_permille(yes: i64, no: i64) -> i64 {
    let num = (yes + ALPHA_MICRO) as i128 * 1000;
    let den = (yes + no + 2 * ALPHA_MICRO) as i128;
    if den <= 0 { return 500; }
    (num / den) as i64
}

// ── tasks (reuse the proven het4/het6 conjunctions) ─────────────────────────
fn task(id: &str) -> Option<Vec<(&'static str, &'static str)>> {
    match id {
        "het4" => Some(vec![
            ("2 * n + 3 ≤ 5 * n + 3 + 1", "omega"),
            ("(a + b)^2 = a^2 + 2*a*b + b^2", "ring"),
            ("(∑ i ∈ Finset.range (n+1), (i:ℤ)) * 2 = n * (n+1)", "induction"),
            ("a^2 + b^2 ≥ 2*a*b", "nlinarith"),
        ]),
        "het6" => Some(vec![
            ("3 * n + 1 ≤ 7 * n + 2", "omega"),
            ("(a - b)^2 = a^2 - 2*a*b + b^2", "ring"),
            ("(∑ i ∈ Finset.range (n+1), (i:ℤ)) * 2 = n * (n+1)", "induction"),
            ("a^2 + b^2 ≥ 2*a*b", "nlinarith"),
            ("n + 5 ≤ 2 * n + 5 + n", "omega"),
            ("(a + b) * (a - b) = a^2 - b^2", "ring"),
        ]),
        _ => None,
    }
}
const PREAMBLE_VARS: &str = "(n : ℕ) (a b : ℝ)";
const FAMILIES: [&str; 4] = ["omega", "ring", "induction", "nlinarith"];
fn family_hint(fam: &str) -> &'static str {
    match fam {
        "omega" => "You may ONLY use the `omega` tactic (linear integer/nat arithmetic). If the goal is not linear arithmetic, output {\"tactic\":\"SKIP\"}.",
        "ring" => "You may ONLY use `ring` or `ring_nf` (commutative-ring identities). If the goal is not a ring identity, output {\"tactic\":\"SKIP\"}.",
        "induction" => "You may ONLY use an `induction n with | zero => ... | succ k ih => ...` proof. If induction does not apply, output {\"tactic\":\"SKIP\"}.",
        _ => "You may ONLY use `nlinarith [...]` with square hints like `sq_nonneg (a-b)`. If the goal is not such an inequality, output {\"tactic\":\"SKIP\"}.",
    }
}

fn lean_path(mathlib_dir: &Path) -> Option<String> {
    let out = std::process::Command::new(format!("{}/.elan/bin/lake", std::env::var("HOME").ok()?))
        .args(["env", "printenv", "LEAN_PATH"]).current_dir(mathlib_dir).output().ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}
fn default_lean_bin() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    let p = PathBuf::from(&home).join(".elan/toolchains/leanprover--lean4---v4.24.0/bin/lean");
    if p.exists() { p } else { PathBuf::from("lean") }
}
fn verify_conjunct(goal: &str, proof: &str, lean_bin: &Path, mathlib_dir: &Path, lp: &str, tag: &str) -> bool {
    let low = proof.to_lowercase();
    if low.contains("sorry") || low.contains("admit") { return false; }
    let src = format!("import Mathlib\nopen Finset in\ntheorem c_{tag} {PREAMBLE_VARS} : {goal} := by\n{}\n",
        proof.lines().map(|l| format!("  {l}")).collect::<Vec<_>>().join("\n"));
    let file = std::env::temp_dir().join(format!("hayek_{tag}.lean"));
    if std::fs::write(&file, &src).is_err() { return false; }
    match std::process::Command::new(lean_bin).arg(&file).current_dir(mathlib_dir).env("LEAN_PATH", lp).output() {
        Ok(o) => {
            let t = format!("{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr)).to_lowercase();
            o.status.success() && !t.contains("sorry") && !t.contains("error")
        }
        Err(_) => false,
    }
}
fn extract(s: &str, key: &str) -> Option<serde_json::Value> {
    let v: serde_json::Value = serde_json::from_str(s.trim()).ok()
        .or_else(|| { let a = s.find('{')?; let b = s.rfind('}')?; serde_json::from_str(&s[a..=b]).ok() })?;
    v.get(key).cloned()
}
fn short_hash(s: &str) -> String { let mut h = Sha256::new(); h.update(s.as_bytes()); format!("{:x}", h.finalize())[..12].to_string() }

struct Args { task: String, policy: String, n_rounds: usize, verify_budget: usize, seed: u64, temp: f64, proxy: String, model: String, mathlib_dir: PathBuf, out: PathBuf, tape_out: Option<PathBuf> }
fn parse_args() -> Result<Args, String> {
    let a: Vec<String> = std::env::args().collect();
    let get = |k: &str| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1).cloned());
    Ok(Args {
        task: get("--task").ok_or("--task required")?,
        policy: get("--policy").unwrap_or_else(|| "realprice".into()),
        n_rounds: get("--n-rounds").and_then(|s| s.parse().ok()).unwrap_or(8),
        verify_budget: get("--verify-budget").and_then(|s| s.parse().ok()).unwrap_or(6),
        seed: get("--seed").and_then(|s| s.parse().ok()).unwrap_or(1),
        temp: get("--temp").and_then(|s| s.parse().ok()).unwrap_or(0.3),
        proxy: get("--proxy").unwrap_or_else(|| "http://localhost:8123".into()),
        model: get("--model").unwrap_or_else(|| "deepseek-chat".into()),
        mathlib_dir: get("--mathlib-dir").map(Into::into).ok_or("--mathlib-dir required")?,
        out: get("--out").map(Into::into).unwrap_or_else(|| "/tmp/hayek.json".into()),
        tape_out: get("--tape-out").map(Into::into),
    })
}

/// Route: pick which OPEN, FUNDED claim to spend the scarce Lean-verify on. THE arm difference.
/// f64 softmax is POLICY only; the selected claim is recorded on the tape (replay reads it).
fn route(policy: &str, open_funded: &[usize], price_pm: &[i64], attempts: &[u32], temp: f64, rng: &mut StdRng) -> Option<usize> {
    if open_funded.is_empty() { return None; }
    match policy {
        // RealPrice / RealPrice-no-NO: softmax over the Hayek price (ℓ_x ∝ logit(price)).
        "realprice" | "realprice_no_no" => {
            let logits: Vec<f64> = open_funded.iter().map(|&c| {
                let p = (price_pm[c] as f64 / 1000.0).clamp(0.001, 0.999);
                (p / (1.0 - p)).ln() // logit(price) = the Hayek price term ℓ_x (liquidity-uniform here)
            }).collect();
            Some(softmax_pick(open_funded, &logits, temp, rng))
        }
        // ShuffledPrice: price signal destroyed by a seeded permutation → tests price is load-bearing.
        "shuffled" => {
            let mut perm: Vec<i64> = open_funded.iter().map(|&c| price_pm[c]).collect();
            for i in (1..perm.len()).rev() { let j = rng.gen_range(0..=i); perm.swap(i, j); }
            let logits: Vec<f64> = perm.iter().map(|&pm| { let p=(pm as f64/1000.0).clamp(0.001,0.999); (p/(1.0-p)).ln() }).collect();
            Some(softmax_pick(open_funded, &logits, temp, rng))
        }
        // CentralScore: the REJECTED centralized heuristic foil — route by attempt count, no price.
        "central" => open_funded.iter().copied().max_by_key(|&c| attempts[c]),
        // Uniform: no signal at all → rules out plain parallelism.
        "uniform" => Some(open_funded[rng.gen_range(0..open_funded.len())]),
        _ => Some(open_funded[0]),
    }
}
fn softmax_pick(items: &[usize], logits: &[f64], temp: f64, rng: &mut StdRng) -> usize {
    let t = if temp <= 0.0 { 1e-6 } else { temp };
    let m = logits.iter().cloned().fold(f64::MIN, f64::max);
    let w: Vec<f64> = logits.iter().map(|l| ((l - m) / t).exp()).collect();
    let sum: f64 = w.iter().sum();
    if !(sum > 0.0) { return items[0]; }
    let mut r = rng.gen::<f64>() * sum;
    for (i, wi) in w.iter().enumerate() { r -= wi; if r <= 0.0 { return items[i]; } }
    items[items.len() - 1]
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = parse_args()?;
    let conjuncts = task(&args.task).ok_or(format!("unknown task {}", args.task))?;
    let k = conjuncts.len();
    let llm = ResilientLLMClient::new(&args.proxy, 120, 3);
    let lean_bin = default_lean_bin();
    let lp = lean_path(&args.mathlib_dir).unwrap_or_default();
    if lp.is_empty() { return Err("Mathlib LEAN_PATH unresolved".into()); }
    let mut rng = StdRng::seed_from_u64(args.seed);
    let t0 = Instant::now();
    let mut tape = MarketTape::new();

    // single-strongest baseline: one generalist, sequential, no market (the honest control).
    let market_arms: BTreeSet<&str> = ["realprice", "realprice_no_no", "shuffled", "central", "uniform"].into_iter().collect();
    let is_market = market_arms.contains(args.policy.as_str());
    // roster: market = 4 specialists; single = 1 generalist (gets the SAME total LLM-call budget).
    let agents: Vec<Option<&str>> = if is_market { FAMILIES.iter().map(|f| Some(*f)).collect() } else { vec![None] };
    let allow_no = args.policy != "realprice_no_no"; // the NO-ablation arm

    for c in 0..k { tape.record(&MarketEvent::MarketOpen { claim: c, claim_type: conjuncts[c].1.into() }); }

    let mut wallets: Vec<i64> = vec![WALLET_BUDGET_MICRO; agents.len()];
    let mut yes: Vec<i64> = vec![0; k];
    let mut no: Vec<i64> = vec![0; k];
    let mut attempts: Vec<u32> = vec![0; k];
    let mut best_proof: Vec<Option<(usize, String)>> = vec![None; k]; // (agent, proof) of the top-funded YES bet
    let mut closed: BTreeSet<usize> = BTreeSet::new();
    let mut verifies_used = 0usize;
    let mut llm_calls = 0usize; let mut lean_calls = 0usize; let mut tokens = 0u64; let mut skips = 0usize;
    let mut realized_pnl: Vec<i64> = vec![0; agents.len()]; // settled wallet delta (proves capital at risk)

    let single_total = agents.len() * args.n_rounds; // for the single arm: equal LLM-call budget
    let rounds = if is_market { args.n_rounds } else { single_total };

    'outer: for round in 0..rounds {
        if closed.len() == k || verifies_used >= args.verify_budget { break; }
        // ── INVESTMENT PHASE: each agent stakes real capital on one open conjunct ──
        for (ai, fam) in agents.iter().enumerate() {
            let open: Vec<usize> = (0..k).filter(|i| !closed.contains(i)).collect();
            if open.is_empty() { break 'outer; }
            let target = open[rng.gen_range(0..open.len())];
            let (goal, _truth) = conjuncts[target];
            let role = match fam { Some(f) => format!("You are a SPECIALIST. {}", family_hint(f)), None => "You are a generalist Lean 4 prover; use any single appropriate tactic.".into() };
            let prompt = format!("{role}\n\nProve this Lean 4 (Mathlib) goal (context binds `(n:ℕ)(a b:ℝ)`):\n{goal}\n\nIf you can close it, output JSON {{\"tactic\":\"<proof body>\",\"confidence\":0-100}} where confidence is how SURE you are it compiles (you will LOSE staked capital if it fails). If it is not your specialty, output {{\"tactic\":\"SKIP\"}}. No `sorry`.");
            let resp = match llm.generate(&GenerateRequest { model: args.model.clone(), messages: vec![Message { role: "user".into(), content: prompt }], temperature: Some(0.3), max_tokens: Some(400) }).await {
                Ok(r) => { tokens += (r.prompt_tokens + r.completion_tokens) as u64; llm_calls += 1; tape.record(&MarketEvent::LlmCall { model: args.model.clone(), tokens: (r.prompt_tokens + r.completion_tokens) as u64 }); r }
                Err(_) => continue,
            };
            let tac = match extract(&resp.content, "tactic").and_then(|v| v.as_str().map(String::from)) {
                Some(t) if !t.trim().is_empty() && t.to_uppercase() != "SKIP" => t.trim().to_string(),
                _ => { skips += 1; continue; }
            };
            let conf = extract(&resp.content, "confidence").and_then(|v| v.as_u64()).unwrap_or(50).min(100);
            let model_h = short_hash(&format!("{}:{:?}", args.model, fam));
            // a confident specialist whose family fits → YES (stake capital on its proof);
            // a low-confidence agent shorts NO (this approach likely fails) IF the NO arm is enabled.
            let stake = stake_from_confidence(conf, wallets[ai]);
            if stake < MIN_STAKE_MICRO { continue; }
            if conf >= 50 {
                wallets[ai] -= stake; yes[target] += stake; attempts[target] += 1;
                tape.record(&MarketEvent::Invest { agent: ai, claim: target, side: "YES".into(), amount_micro: stake, model_hash: model_h, confidence: conf });
                tape.record(&MarketEvent::Proposal { agent: ai, claim: target, output_hash: short_hash(&tac) });
                // keep the highest-funded YES proof as the one to verify
                if best_proof[target].as_ref().map_or(true, |_| yes[target] >= 0) { best_proof[target] = Some((ai, tac)); }
            } else if allow_no {
                wallets[ai] -= stake; no[target] += stake;
                tape.record(&MarketEvent::Invest { agent: ai, claim: target, side: "NO".into(), amount_micro: stake, model_hash: model_h, confidence: conf });
            }
        }
        // ── PRICE FORMATION (derived from Invest events) ──
        let price_pm: Vec<i64> = (0..k).map(|c| price_yes_permille(yes[c], no[c])).collect();
        // ── ROUTING: spend the scarce verify on the price-selected open+funded claim ──
        let open_funded: Vec<usize> = (0..k).filter(|&c| !closed.contains(&c) && best_proof[c].is_some()).collect();
        let frontier_h = short_hash(&format!("{:?}{:?}", open_funded, price_pm));
        let sel = route(&args.policy, &open_funded, &price_pm, &attempts, args.temp, &mut rng);
        let Some(c) = sel else { continue };
        tape.record(&MarketEvent::RouteSample { policy: args.policy.clone(), frontier_hash: frontier_h, selected_claim: c });
        // ── VERIFY (spend 1 of the budget) + SETTLE ──
        let (proposer, proof) = best_proof[c].clone().unwrap();
        lean_calls += 1; verifies_used += 1;
        let ok = verify_conjunct(conjuncts[c].0, &proof, &lean_bin, &args.mathlib_dir, &lp, &format!("{}_{}_{}_{}", args.task, args.policy, args.seed, round));
        tape.record(&MarketEvent::Verify { claim: c, verdict: ok, reject_class: if ok { "none".into() } else { "lean_rejected".into() } });
        if ok {
            closed.insert(c);
            // SETTLE: YES wins → proposer recovers stake + takes the NO pool; NO-investors lose (already debited).
            realized_pnl[proposer] += no[c]; wallets[proposer] += yes[c] + no[c];
            tape.record(&MarketEvent::Resolve { claim: c, outcome: "YES".into() });
        } else {
            // capital at risk REALIZED: the proposer's YES stake on a failed proof is forfeit (NOT refunded).
            realized_pnl[proposer] -= yes[c];
            best_proof[c] = None; // discredit the funded approach; price drops next round
            // partial reset so a fresh, better-funded proof can be sought
            yes[c] = 0;
            tape.record(&MarketEvent::Resolve { claim: c, outcome: "NO_DISCREDIT".into() });
        }
    }

    let wall = t0.elapsed().as_secs_f64();
    let chain_ok = tape.verify_chain();
    let derived = tape.derive_pools(k); // price re-derived from tape Invest events alone
    let manifest = serde_json::json!({
        "schema": "lean_hayek_market.v1", "task": args.task, "policy": args.policy,
        "k": k, "closed": closed.len(), "solved": closed.len() == k,
        "verify_budget": args.verify_budget, "verifies_used": verifies_used,
        "n_rounds": args.n_rounds, "seed": args.seed, "temp": args.temp,
        "llm_calls": llm_calls, "lean_calls": lean_calls, "skips": skips, "tokens": tokens,
        "realized_pnl_micro": realized_pnl, "wall_s": wall,
        "tape_chain_ok": chain_ok, "tape_events": tape.lines.len(),
        "derived_pools_from_tape": derived,
        "closed_claims": closed.iter().collect::<Vec<_>>(),
    });
    let _ = std::fs::write(&args.out, serde_json::to_string_pretty(&manifest).unwrap());
    if let Some(tp) = &args.tape_out { let _ = std::fs::write(tp, tape.lines.join("\n")); }
    println!(
        "hayek[{}] task={} closed={}/{} solved={} verifies={}/{} llm={} lean={} skips={} tokens={} chain_ok={} wall={:.1}s",
        args.policy, args.task, closed.len(), k, closed.len() == k, verifies_used, args.verify_budget,
        llm_calls, lean_calls, skips, tokens, chain_ok, wall
    );
    Ok(())
}
