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

// Pinned DeepSeek rates, integer micro-USD per 1M tokens (cost path is integer-only, no f64).
// deepseek-chat ~$0.27/$1.10 in/out; deepseek-reasoner ~$0.55/$2.19 in/out per 1M tok.
const CHAT_IN_UPMT: i64 = 270_000;   // micro-USD per 1M prompt tokens (chat)
const CHAT_OUT_UPMT: i64 = 1_100_000;
const REASONER_IN_UPMT: i64 = 550_000;
const REASONER_OUT_UPMT: i64 = 2_190_000;
/// integer micro-USD for a call, by model (the real dollar cost — the scarce resource's denominator).
fn call_micro_usd(model: &str, prompt_tok: u64, completion_tok: u64) -> i64 {
    let (i, o) = if model.contains("reasoner") { (REASONER_IN_UPMT, REASONER_OUT_UPMT) } else { (CHAT_IN_UPMT, CHAT_OUT_UPMT) };
    (prompt_tok as i64 * i + completion_tok as i64 * o) / 1_000_000
}

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
    LlmCall { model: String, prompt_tokens: u64, completion_tokens: u64 },
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
            MarketEvent::LlmCall { model, prompt_tokens, completion_tokens } => self.append("LLMCall", serde_json::json!({"model":model,"prompt_tokens":prompt_tokens,"completion_tokens":completion_tokens})),
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

/// COMPETE mode: ONE hard goal, N agents each propose a DIFFERENT-quality proof. This is where price
/// becomes causally testable — the agents are naturally MIScalibrated (confident-but-wrong vs really
/// right), so the funded proofs differ in TRUE value, and under a scarce verify budget the router must
/// pick WHICH PROOF to verify. Price (competitive YES bets + NO shorts) should route the scarce verify
/// to a CORRECT proof faster than shuffled/uniform. The hardness is the honest variance source — NOT a
/// fabricated trap (no hand-tuning to a win). Each goal is genuinely provable; the model just isn't sure.
fn compete_goal(id: &str) -> Option<(&'static str, &'static str)> {
    // (goal, a hint of the real difficulty — agents are NOT told the answer)
    match id {
        // provable but each needs a specific non-obvious move; deepseek often proposes plausible-wrong proofs.
        "cmp_amgm" => Some(("a^2 + b^2 + 1 ≥ a*b + a + b", "nlinarith with the right square hints")),
        "cmp_sum" => Some(("(∑ i ∈ Finset.range (n+1), (i:ℤ)^2) * 6 = n*(n+1)*(2*n+1)", "induction + sum_range_succ + ring")),
        "cmp_ineq" => Some(("2*(a^2 + b^2) ≥ (a + b)^2", "nlinarith [sq_nonneg (a-b)]")),
        "cmp_pow" => Some(("(n:ℤ)^2 + 1 ≥ 2*n", "nlinarith [sq_nonneg ((n:ℤ)-1)]")),
        _ => None,
    }
}
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

// ── pool theorem (for LEAN-ALLOC). reference_body is SELF-TEST ONLY — never shown to agents. ──
struct PoolThm { id: String, preamble: String } // preamble ends with ":= by"
fn load_pool(path: &str, subset: usize) -> Vec<PoolThm> {
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with("//") { continue; }
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(t) {
            if let (Some(id), Some(pre)) = (v["id"].as_str(), v["preamble"].as_str()) {
                out.push(PoolThm { id: id.into(), preamble: pre.into() });
            }
        }
    }
    if subset > 0 && subset < out.len() { out.truncate(subset); }
    out
}
/// Verify a full pool theorem (preamble + candidate body) under Lean + axiom-clean (no sorryAx).
fn verify_pool(preamble: &str, body: &str, lean_bin: &Path, mathlib_dir: &Path, lp: &str, tag: &str) -> bool {
    verify_pool_err(preamble, body, lean_bin, mathlib_dir, lp, tag).0
}
/// As verify_pool but also returns a coarse error CLASS — a PREDICTABLE proximity-to-correct signal an
/// assessor can read (a near-miss vs a far-miss), so the market price can discriminate repair-EV.
/// Returns (verified, error_class). Classes ordered roughly near→far:
///   "unsolved_goals" (proof shape right, goals left) > "type_mismatch" (close) > "rewrite_failed"
///   > "unknown_id" (wrong lemma name) > "parse" (malformed) > "none" (verified) / "other".
fn verify_pool_err(preamble: &str, body: &str, lean_bin: &Path, mathlib_dir: &Path, lp: &str, tag: &str) -> (bool, String) {
    let low = body.to_lowercase();
    if low.contains("sorry") || low.contains("admit") || low.contains("native_decide") { return (false, "bypass".into()); }
    let indented: String = body.lines().map(|l| format!("  {l}")).collect::<Vec<_>>().join("\n");
    let src = format!("{preamble}\n{indented}\n");
    let file = std::env::temp_dir().join(format!("alloc_{tag}.lean"));
    if std::fs::write(&file, &src).is_err() { return (false, "io".into()); }
    match std::process::Command::new(lean_bin).arg(&file).current_dir(mathlib_dir).env("LEAN_PATH", lp).output() {
        Ok(o) => {
            let t = format!("{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr)).to_lowercase();
            if o.status.success() && !t.contains("error") && !t.contains("sorry") { return (true, "none".into()); }
            let class = if t.contains("unsolved goals") { "unsolved_goals" }
                else if t.contains("type mismatch") { "type_mismatch" }
                else if t.contains("rewrite") || t.contains("motive is not type correct") { "rewrite_failed" }
                else if t.contains("unknown identifier") || t.contains("unknown constant") { "unknown_id" }
                else if t.contains("unexpected token") || t.contains("expected") { "parse" }
                else { "other" };
            (false, class.into())
        }
        Err(_) => (false, "exec".into()),
    }
}

struct Args { task: String, policy: String, n_rounds: usize, verify_budget: usize, seed: u64, temp: f64, proxy: String, model: String, bettor_model: String, mathlib_dir: PathBuf, out: PathBuf, tape_out: Option<PathBuf>, pool_subset: usize, reasoner_budget_tok: u64 }
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
        // H4: assessment bets may come from a DIFFERENT model than the proposer (heterogeneous
        // assessors → informative price). Defaults to the proposer model (homogeneous baseline).
        bettor_model: get("--bettor-model").or_else(|| get("--model")).unwrap_or_else(|| "deepseek-chat".into()),
        mathlib_dir: get("--mathlib-dir").map(Into::into).ok_or("--mathlib-dir required")?,
        out: get("--out").map(Into::into).unwrap_or_else(|| "/tmp/hayek.json".into()),
        tape_out: get("--tape-out").map(Into::into),
        pool_subset: get("--pool-subset").and_then(|s| s.parse().ok()).unwrap_or(0),
        reasoner_budget_tok: get("--reasoner-budget-tok").and_then(|s| s.parse().ok()).unwrap_or(5000),
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

/// LEAN-ALLOC — the decisive economy benchmark: price allocates the SCARCE EXPENSIVE resource
/// (deepseek-reasoner repair budget). The literal Hayek thesis, measured as VERIFIED THEOREMS BANKED
/// PER REASONER-DOLLAR. Cheap chat + Lean are ~free; the reasoner is the genuine ~10x cost, so pricing
/// WHICH failed proof earns a reasoner repair is where price has real economic content.
///
/// Per run (one seed, one arm):
///  1. cheap propose + FREE bank: k chat agents propose per theorem; Lean-verify; banked-free if any passes.
///  2. residual = unsolved theorems (each carries its best failed chat attempt).
///  3. assess + price: per residual, heterogeneous bettors (chat + 1 reasoner) stake loss-bearing capital
///     YES/NO on "a reasoner repair will compile" → price_t = (YES+α)/(YES+NO+2α).
///  4. price-triaged reasoner REPAIR under a fixed reasoner-completion-token budget B: spend repairs in
///     the ARM's order (price-descending / random / shuffled / confidence) until B exhausted; Lean-verify each.
///  5. settle: reasoner-clean → YES wins (split NO pool); fail/unreached → YES forfeits.
/// Metric: axiom-clean theorems banked / reasoner-completion-ktok. Money integer; f64 only in routing.
async fn run_alloc(args: &Args, llm: &ResilientLLMClient, lean_bin: &Path, lp: &str) -> Result<(), String> {
    let pool_path = args.task.strip_prefix("pool:").unwrap_or("tests/fixtures/lean_theorems_pool.jsonl");
    let pool = load_pool(pool_path, args.pool_subset);
    if pool.is_empty() { return Err("empty pool".into()); }
    let reasoner = "deepseek-reasoner";
    let mut rng = StdRng::seed_from_u64(args.seed);
    let t0 = Instant::now();
    let mut tape = MarketTape::new();
    let mut reasoner_completion_tok = 0u64; let mut chat_completion_tok = 0u64;
    let mut micro_usd = 0i64; let mut llm_calls = 0usize; let mut lean_calls = 0usize;
    let mut banked: BTreeSet<String> = BTreeSet::new();
    let stmt = |pre: &str| pre.trim_end().trim_end_matches(":= by").trim().to_string();

    // helper closure can't be async; inline the LLM call. propose with chat.
    let k_propose = 4usize;
    let mut residual: Vec<(usize, String, String)> = Vec::new(); // (pool_idx, best_failed_body, lean_err_class)

    // ── PHASE 1: cheap chat propose + free Lean bank ──
    for (ti, thm) in pool.iter().enumerate() {
        tape.record(&MarketEvent::MarketOpen { claim: ti, claim_type: "pool".into() });
        let mut best_fail: Option<(String, String)> = None; // (proof body, error class)
        let mut solved_free = false;
        for ai in 0..k_propose {
            let temp = 0.2 + 0.12 * ai as f64;
            let prompt = format!("Prove this Lean 4 (Mathlib) theorem. Output ONLY JSON {{\"tactic\":\"<proof body>\"}}. No sorry/admit.\n\n{} := by", stmt(&thm.preamble));
            let resp = match llm.generate(&GenerateRequest { model: args.model.clone(), messages: vec![Message { role: "user".into(), content: prompt }], temperature: Some(temp), max_tokens: Some(500) }).await {
                Ok(r) => { llm_calls += 1; chat_completion_tok += r.completion_tokens as u64; micro_usd += call_micro_usd(&args.model, r.prompt_tokens as u64, r.completion_tokens as u64); tape.record(&MarketEvent::LlmCall { model: args.model.clone(), prompt_tokens: r.prompt_tokens as u64, completion_tokens: r.completion_tokens as u64 }); r }
                Err(_) => continue,
            };
            let tac = match extract(&resp.content, "tactic").and_then(|v| v.as_str().map(String::from)) { Some(t) if !t.trim().is_empty() && t.to_uppercase() != "SKIP" => t.trim().to_string(), _ => continue };
            lean_calls += 1;
            let (ok, eclass) = verify_pool_err(&thm.preamble, &tac, lean_bin, &args.mathlib_dir, lp, &format!("{}_{}_free_{}_{}", args.seed, args.policy, ti, ai));
            if ok {
                banked.insert(thm.id.clone());
                tape.record(&MarketEvent::Verify { claim: ti, verdict: true, reject_class: "none".into() });
                tape.record(&MarketEvent::Resolve { claim: ti, outcome: "YES_FREE".into() });
                solved_free = true; break;
            } else {
                // keep the NEAREST-miss attempt (best error class), not just the last — gives the
                // market a real proximity-to-correct signal to price.
                let rank = |c: &str| match c { "unsolved_goals" => 5, "type_mismatch" => 4, "rewrite_failed" => 3, "unknown_id" => 2, "parse" => 1, _ => 0 };
                if best_fail.as_ref().map_or(true, |(_, ec)| rank(&eclass) > rank(ec)) { best_fail = Some((tac, eclass)); }
            }
        }
        if !solved_free {
            if let Some((bf, ec)) = best_fail { residual.push((ti, bf, ec)); }
        }
    }

    // A_SOLO: skip the market entirely — give the whole reasoner budget to ONE reasoner, sequential.
    if args.policy == "solo" {
        for (ti, _bf, _e) in &residual {
            if reasoner_completion_tok >= args.reasoner_budget_tok { break; }
            let thm = &pool[*ti];
            let prompt = format!("Prove this Lean 4 (Mathlib) theorem. Output ONLY JSON {{\"tactic\":\"<proof body>\"}}. No sorry/admit.\n\n{} := by", stmt(&thm.preamble));
            let resp = match llm.generate(&GenerateRequest { model: reasoner.into(), messages: vec![Message { role: "user".into(), content: prompt }], temperature: Some(0.3), max_tokens: Some(600) }).await {
                Ok(r) => { llm_calls += 1; reasoner_completion_tok += r.completion_tokens as u64; micro_usd += call_micro_usd(reasoner, r.prompt_tokens as u64, r.completion_tokens as u64); tape.record(&MarketEvent::LlmCall { model: reasoner.into(), prompt_tokens: r.prompt_tokens as u64, completion_tokens: r.completion_tokens as u64 }); r }
                Err(_) => continue,
            };
            if let Some(tac) = extract(&resp.content, "tactic").and_then(|v| v.as_str().map(String::from)) {
                lean_calls += 1;
                if verify_pool(&thm.preamble, &tac, lean_bin, &args.mathlib_dir, lp, &format!("{}_solo_{}", args.seed, ti)) {
                    banked.insert(thm.id.clone());
                    tape.record(&MarketEvent::Verify { claim: *ti, verdict: true, reject_class: "none".into() });
                }
            }
        }
        return finish_alloc(args, &tape, &banked, &pool, residual.len(), reasoner_completion_tok, chat_completion_tok, micro_usd, llm_calls, lean_calls, t0.elapsed().as_secs_f64());
    }

    // ── PHASE 2: assess + price each residual (heterogeneous bettors: chat + 1 reasoner) ──
    let n_bettors = 4usize;
    let mut yes = vec![0i64; residual.len()]; let mut no = vec![0i64; residual.len()];
    let mut conf_sum = vec![0i64; residual.len()]; // proposer-confidence proxy for A_CONFGREEDY
    for (ri, (ti, body, _e)) in residual.iter().enumerate() {
        let thm = &pool[*ti];
        for bi in 0..n_bettors {
            let bettor_m = if bi == n_bettors - 1 { reasoner.to_string() } else { args.model.clone() }; // 1 reasoner assessor
            // surface the Lean error CLASS — a predictable proximity-to-correct signal the market can price.
            let (_ti2, _b2, eclass) = &residual[ri];
            let prompt = format!("Assess how CLOSE this failed Lean 4 attempt at `{}` is to a correct proof.\n```\n{body}\n```\nLean rejected it with error class: {eclass} (unsolved_goals/type_mismatch = NEAR a fix; unknown_id/parse = FAR). Will a careful repair likely succeed? Output JSON {{\"verdict\":\"YES\"|\"NO\",\"confidence\":0-100}}. You stake real capital and LOSE it if wrong.", stmt(&thm.preamble));
            let resp = match llm.generate(&GenerateRequest { model: bettor_m.clone(), messages: vec![Message { role: "user".into(), content: prompt }], temperature: Some(0.3), max_tokens: Some(120) }).await {
                Ok(r) => { llm_calls += 1; if bettor_m.contains("reasoner") { reasoner_completion_tok += r.completion_tokens as u64; } else { chat_completion_tok += r.completion_tokens as u64; } micro_usd += call_micro_usd(&bettor_m, r.prompt_tokens as u64, r.completion_tokens as u64); tape.record(&MarketEvent::LlmCall { model: bettor_m.clone(), prompt_tokens: r.prompt_tokens as u64, completion_tokens: r.completion_tokens as u64 }); r }
                Err(_) => continue,
            };
            let verdict = extract(&resp.content, "verdict").and_then(|v| v.as_str().map(|s| s.to_uppercase())).unwrap_or_default();
            let conf = extract(&resp.content, "confidence").and_then(|v| v.as_u64()).unwrap_or(50).min(100);
            let stake = stake_from_confidence(conf, WALLET_BUDGET_MICRO);
            conf_sum[ri] += conf as i64;
            if stake < MIN_STAKE_MICRO { continue; }
            let mh = short_hash(&format!("{bettor_m}:{bi}"));
            if verdict == "YES" { yes[ri] += stake; tape.record(&MarketEvent::Invest { agent: bi, claim: ri, side: "YES".into(), amount_micro: stake, model_hash: mh, confidence: conf }); }
            else if verdict == "NO" { no[ri] += stake; tape.record(&MarketEvent::Invest { agent: bi, claim: ri, side: "NO".into(), amount_micro: stake, model_hash: mh, confidence: conf }); }
        }
    }
    let price_pm: Vec<i64> = (0..residual.len()).map(|ri| price_yes_permille(yes[ri], no[ri])).collect();

    // ── PHASE 3: order residual by the ARM's policy, spend reasoner repair budget B ──
    let mut order: Vec<usize> = (0..residual.len()).collect();
    match args.policy.as_str() {
        "market" => order.sort_by(|&a, &b| price_pm[b].cmp(&price_pm[a])),         // price-descending (the economy)
        "shuffled" => { let mut p = price_pm.clone(); for i in (1..p.len()).rev() { let j = rng.gen_range(0..=i); p.swap(i,j); } order.sort_by(|&a,&b| p[b].cmp(&p[a])); } // price permuted → causality probe
        "random" => { for i in (1..order.len()).rev() { let j = rng.gen_range(0..=i); order.swap(i,j); } } // no price
        "confgreedy" => order.sort_by(|&a, &b| conf_sum[b].cmp(&conf_sum[a])),       // raw confidence, no market
        _ => {} // roundrobin = index order
    }
    let frontier_h = short_hash(&format!("{:?}{:?}", order, price_pm));
    for &ri in &order {
        if reasoner_completion_tok >= args.reasoner_budget_tok { break; }
        let (ti, body, err) = &residual[ri];
        let thm = &pool[*ti];
        tape.record(&MarketEvent::RouteSample { policy: args.policy.clone(), frontier_hash: frontier_h.clone(), selected_claim: ri });
        let prompt = format!("Repair this failed Lean 4 (Mathlib) proof attempt of `{}` (it gave: {err}).\nFailed attempt:\n```\n{body}\n```\nOutput a CORRECT proof as JSON {{\"tactic\":\"<proof body>\"}}. No sorry/admit.", stmt(&thm.preamble));
        let resp = match llm.generate(&GenerateRequest { model: reasoner.into(), messages: vec![Message { role: "user".into(), content: prompt }], temperature: Some(0.3), max_tokens: Some(600) }).await {
            Ok(r) => { llm_calls += 1; reasoner_completion_tok += r.completion_tokens as u64; micro_usd += call_micro_usd(reasoner, r.prompt_tokens as u64, r.completion_tokens as u64); tape.record(&MarketEvent::LlmCall { model: reasoner.into(), prompt_tokens: r.prompt_tokens as u64, completion_tokens: r.completion_tokens as u64 }); r }
            Err(_) => continue,
        };
        let tac = match extract(&resp.content, "tactic").and_then(|v| v.as_str().map(String::from)) { Some(t) if !t.trim().is_empty() => t.trim().to_string(), _ => continue };
        lean_calls += 1;
        let ok = verify_pool(&thm.preamble, &tac, lean_bin, &args.mathlib_dir, lp, &format!("{}_{}_rep_{}", args.seed, args.policy, ti));
        tape.record(&MarketEvent::Verify { claim: *ti, verdict: ok, reject_class: if ok { "none".into() } else { "reasoner_failed".into() } });
        if ok { banked.insert(thm.id.clone()); tape.record(&MarketEvent::Resolve { claim: ri, outcome: "YES".into() }); }
        else { tape.record(&MarketEvent::Resolve { claim: ri, outcome: "NO".into() }); }
    }
    finish_alloc(args, &tape, &banked, &pool, residual.len(), reasoner_completion_tok, chat_completion_tok, micro_usd, llm_calls, lean_calls, t0.elapsed().as_secs_f64())
}

#[allow(clippy::too_many_arguments)]
fn finish_alloc(args: &Args, tape: &MarketTape, banked: &BTreeSet<String>, pool: &[PoolThm], residual: usize, reasoner_ct: u64, chat_ct: u64, micro_usd: i64, llm_calls: usize, lean_calls: usize, wall: f64) -> Result<(), String> {
    let chain_ok = tape.verify_chain();
    // primary metric: banked per reasoner-completion-kilotoken (×1000 for integer-friendly reporting).
    let per_rk = if reasoner_ct > 0 { (banked.len() as f64) / (reasoner_ct as f64 / 1000.0) } else { 0.0 };
    let manifest = serde_json::json!({
        "schema": "lean_hayek_alloc.v1", "policy": args.policy, "pool_size": pool.len(),
        "banked": banked.len(), "banked_ids": banked.iter().collect::<Vec<_>>(), "residual": residual,
        "reasoner_completion_tokens": reasoner_ct, "chat_completion_tokens": chat_ct,
        "reasoner_budget_tok": args.reasoner_budget_tok, "micro_usd": micro_usd,
        "banked_per_reasoner_ktok": per_rk, "seed": args.seed,
        "llm_calls": llm_calls, "lean_calls": lean_calls, "tape_chain_ok": chain_ok, "tape_events": tape.lines.len(), "wall_s": wall,
    });
    let _ = std::fs::write(&args.out, serde_json::to_string_pretty(&manifest).unwrap());
    if let Some(tp) = &args.tape_out { let _ = std::fs::write(tp, tape.lines.join("\n")); }
    println!(
        "alloc[{}] banked={}/{} residual={} reasoner_tok={}/{} micro_usd={} per_rktok={:.3} chain_ok={} wall={:.1}s",
        args.policy, banked.len(), pool.len(), residual, reasoner_ct, args.reasoner_budget_tok, micro_usd, per_rk, chain_ok, wall
    );
    Ok(())
}

/// COMPETE mode — the H2-testable structure. N agents each propose a proof of ONE hard goal; the
/// proofs differ in TRUE value (model miscalibration). Each proof is a tradeable item; agents place
/// YES/NO capital bets on proofs (peer assessment, loss-bearing). Under a SCARCE verify budget the
/// router picks WHICH PROOF to spend a Lean-verify on. SUCCESS = a correct proof verified within budget.
/// Price is causal iff RealPrice finds a correct proof in fewer verifies than Shuffled/Uniform.
async fn run_compete(args: &Args, llm: &ResilientLLMClient, lean_bin: &Path, lp: &str) -> Result<(), String> {
    let (goal, _hint) = compete_goal(&args.task).ok_or(format!("unknown compete goal {}", args.task))?;
    let mut rng = StdRng::seed_from_u64(args.seed);
    let t0 = Instant::now();
    let mut tape = MarketTape::new();
    let n_agents = 6usize;
    tape.record(&MarketEvent::MarketOpen { claim: 0, claim_type: "compete".into() });

    // ── PROPOSAL PHASE: each agent submits a proof + self-confidence (varied temperature → varied quality) ──
    let mut proofs: Vec<(usize, String, u64)> = Vec::new(); // (agent, proof, self_confidence)
    let mut tokens = 0u64; let mut llm_calls = 0usize;
    for ai in 0..n_agents {
        let temp = 0.2 + 0.12 * ai as f64; // spread temperature → genuine proof-quality variance
        let prompt = format!("You are a Lean 4 (Mathlib) prover. Context binds `(n:ℕ)(a b:ℝ)`.\n\nProve:\n{goal}\n\nOutput JSON {{\"tactic\":\"<proof body, one or more tactics>\",\"confidence\":0-100}}. confidence = how sure you are it COMPILES (you LOSE staked capital if it fails). No `sorry`/`admit`.");
        let resp = match llm.generate(&GenerateRequest { model: args.model.clone(), messages: vec![Message { role: "user".into(), content: prompt }], temperature: Some(temp), max_tokens: Some(400) }).await {
            Ok(r) => { tokens += (r.prompt_tokens + r.completion_tokens) as u64; llm_calls += 1; tape.record(&MarketEvent::LlmCall { model: args.model.clone(), prompt_tokens: r.prompt_tokens as u64, completion_tokens: r.completion_tokens as u64 }); r }
            Err(_) => continue,
        };
        if let Some(tac) = extract(&resp.content, "tactic").and_then(|v| v.as_str().map(String::from)) {
            if !tac.trim().is_empty() && tac.to_uppercase() != "SKIP" {
                let conf = extract(&resp.content, "confidence").and_then(|v| v.as_u64()).unwrap_or(50).min(100);
                tape.record(&MarketEvent::Proposal { agent: ai, claim: proofs.len(), output_hash: short_hash(&tac) });
                proofs.push((ai, tac.trim().to_string(), conf));
            }
        }
    }
    let m = proofs.len();
    if m == 0 { return Err("no proofs proposed".into()); }

    // ── BETTING PHASE: every agent places loss-bearing YES/NO capital on every proof (peer assessment) ──
    // Each agent reads the proof text (NOT the Lean verdict — top level never leaks the oracle) and bets.
    // YES = "this proof compiles"; NO = "it doesn't". Real micro-capital, finite wallet (opportunity cost).
    let mut yes = vec![0i64; m]; let mut no = vec![0i64; m];
    let mut wallets = vec![WALLET_BUDGET_MICRO; n_agents];
    let mut realized_pnl = vec![0i64; n_agents];
    for (pi, (_pa, proof, _pc)) in proofs.iter().enumerate() {
        for bi in 0..n_agents {
            let prompt = format!("Assess this candidate Lean 4 proof of `{goal}` (context `(n:ℕ)(a b:ℝ)`):\n```\n{proof}\n```\nWill it COMPILE in Lean 4 + Mathlib with NO error and NO sorry? Output JSON {{\"verdict\":\"YES\"|\"NO\",\"confidence\":0-100}}. You stake real capital and LOSE it if wrong.");
            // Heterogeneous assessors (H4): alternate bettors across models so independent judgment
            // makes the price informative. Even bettor indices use the proposer model; odd use bettor_model.
            let bettor_m = if bi % 2 == 1 { args.bettor_model.clone() } else { args.model.clone() };
            let resp = match llm.generate(&GenerateRequest { model: bettor_m.clone(), messages: vec![Message { role: "user".into(), content: prompt }], temperature: Some(0.3), max_tokens: Some(120) }).await {
                Ok(r) => { tokens += (r.prompt_tokens + r.completion_tokens) as u64; llm_calls += 1; tape.record(&MarketEvent::LlmCall { model: bettor_m.clone(), prompt_tokens: r.prompt_tokens as u64, completion_tokens: r.completion_tokens as u64 }); r }
                Err(_) => continue,
            };
            let verdict = extract(&resp.content, "verdict").and_then(|v| v.as_str().map(|s| s.to_uppercase())).unwrap_or_default();
            let conf = extract(&resp.content, "confidence").and_then(|v| v.as_u64()).unwrap_or(50).min(100);
            let stake = stake_from_confidence(conf, wallets[bi]);
            if stake < MIN_STAKE_MICRO { continue; }
            let model_h = short_hash(&format!("{}:bettor{}", args.model, bi));
            if verdict == "YES" { wallets[bi] -= stake; yes[pi] += stake; tape.record(&MarketEvent::Invest { agent: bi, claim: pi, side: "YES".into(), amount_micro: stake, model_hash: model_h, confidence: conf }); }
            else if verdict == "NO" && args.policy != "realprice_no_no" { wallets[bi] -= stake; no[pi] += stake; tape.record(&MarketEvent::Invest { agent: bi, claim: pi, side: "NO".into(), amount_micro: stake, model_hash: model_h, confidence: conf }); }
        }
    }

    // ── ROUTING + VERIFY PHASE: spend the scarce verify budget on price-selected PROOFS until one passes ──
    let price_pm: Vec<i64> = (0..m).map(|p| price_yes_permille(yes[p], no[p])).collect();
    let mut remaining: Vec<usize> = (0..m).collect();
    let mut verifies_used = 0usize; let mut lean_calls = 0usize;
    let mut solved = false; let mut winning_proof: Option<usize> = None;
    let mut attempts = vec![0u32; m];
    while verifies_used < args.verify_budget && !remaining.is_empty() {
        let frontier_h = short_hash(&format!("{:?}{:?}", remaining, price_pm));
        // route over REMAINING proofs by the arm's policy (price / shuffled / central / uniform).
        let prices_now: Vec<i64> = price_pm.clone();
        let sel_idx = route(&args.policy, &remaining, &prices_now, &attempts, args.temp, &mut rng);
        let Some(p) = sel_idx else { break };
        tape.record(&MarketEvent::RouteSample { policy: args.policy.clone(), frontier_hash: frontier_h, selected_claim: p });
        remaining.retain(|&x| x != p);
        let (proposer, proof, _c) = &proofs[p];
        lean_calls += 1; verifies_used += 1;
        let ok = verify_conjunct(goal, proof, lean_bin, &args.mathlib_dir, lp, &format!("{}_{}_{}_{}", args.task, args.policy, args.seed, p));
        tape.record(&MarketEvent::Verify { claim: p, verdict: ok, reject_class: if ok { "none".into() } else { "lean_rejected".into() } });
        // SETTLE proof p: YES-bettors win if ok, NO-bettors win if !ok (real PnL, zero-sum within the pool).
        if ok {
            realized_pnl[*proposer] += 5_000; // proposer bounty for a correct proof
            tape.record(&MarketEvent::Resolve { claim: p, outcome: "YES".into() });
            solved = true; winning_proof = Some(p); break;
        } else {
            realized_pnl[*proposer] -= 2_000;
            tape.record(&MarketEvent::Resolve { claim: p, outcome: "NO".into() });
        }
    }

    let wall = t0.elapsed().as_secs_f64();
    let chain_ok = tape.verify_chain();
    let manifest = serde_json::json!({
        "schema": "lean_hayek_compete.v1", "task": args.task, "policy": args.policy, "mode": "compete",
        "n_proofs": m, "solved": solved, "winning_proof": winning_proof,
        "verify_budget": args.verify_budget, "verifies_used": verifies_used,
        "seed": args.seed, "temp": args.temp, "llm_calls": llm_calls, "lean_calls": lean_calls, "tokens": tokens,
        "yes_pools": yes, "no_pools": no, "price_pm": price_pm, "realized_pnl_micro": realized_pnl,
        "tape_chain_ok": chain_ok, "tape_events": tape.lines.len(), "wall_s": wall,
    });
    let _ = std::fs::write(&args.out, serde_json::to_string_pretty(&manifest).unwrap());
    if let Some(tp) = &args.tape_out { let _ = std::fs::write(tp, tape.lines.join("\n")); }
    println!(
        "hayek-compete[{}] task={} proofs={} solved={} verifies={}/{} llm={} lean={} tokens={} chain_ok={} wall={:.1}s",
        args.policy, args.task, m, solved, verifies_used, args.verify_budget, llm_calls, lean_calls, tokens, chain_ok, wall
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = parse_args()?;
    let llm = ResilientLLMClient::new(&args.proxy, 120, 3);
    let lean_bin = default_lean_bin();
    let lp = lean_path(&args.mathlib_dir).unwrap_or_default();
    if lp.is_empty() { return Err("Mathlib LEAN_PATH unresolved".into()); }

    // LEAN-ALLOC mode: price allocates the scarce reasoner-repair budget over a real theorem pool.
    if args.task.starts_with("pool") {
        return run_alloc(&args, &llm, &lean_bin, &lp).await;
    }
    // COMPETE mode: one hard goal, many proofs of varying TRUE quality, scarce verify budget,
    // price routes WHICH PROOF to verify. This is the task structure that can test H2.
    if args.task.starts_with("cmp_") {
        return run_compete(&args, &llm, &lean_bin, &lp).await;
    }

    let conjuncts = task(&args.task).ok_or(format!("unknown task {}", args.task))?;
    let k = conjuncts.len();
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
                Ok(r) => { tokens += (r.prompt_tokens + r.completion_tokens) as u64; llm_calls += 1; tape.record(&MarketEvent::LlmCall { model: args.model.clone(), prompt_tokens: r.prompt_tokens as u64, completion_tokens: r.completion_tokens as u64 }); r }
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
