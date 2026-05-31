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

// ── per-model micro-USD rates (cost path is integer-only, no f64) ────────────
// Each row: (model-id substring, in_micro_usd_per_1M_prompt_tok, out_micro_usd_per_1M_completion_tok).
// ROWS ORDERED MOST-SPECIFIC FIRST — call_micro_usd bills the FIRST row whose substring is contained in
// the model id, so a heterogeneous strong model is priced at ITS real rate, never silently at the
// deepseek-chat proxy. The liberal "reasoner"/"deepseek" catch-alls MUST stay last: a full slash-id like
// "deepseek-ai/DeepSeek-V3.2" also contains "deepseek" and must match its own row first. Adding a model
// to the roster = add a row here; an unlisted id falls through to FALLBACK below. (tests guard the order.)
//
// SiliconFlow rows — any slash-form id ("Org/Model") routes to api.siliconflow.cn
// (src/drivers/llm_proxy.py::detect_provider), so the true price is SiliconFlow's published USD list
// price. Retrieved 2026-05-31 from https://www.siliconflow.com :
//   deepseek-ai/DeepSeek-V3.2    $0.27 in / $0.41 out  (blog: "DeepSeek-V3.2-Exp Now on SiliconFlow")
//   Qwen/Qwen3-32B               $0.14 in / $0.57 out  (/models/qwen-qwen3-32b)
//   Qwen/Qwen2.5-72B-Instruct    $0.59 in / $0.59 out  (/models/qwen-qwen2-5-72b-instruct)
// DeepSeek rows — bare "deepseek-*" ids route to api.deepseek.com; pinned to the DeepSeek API USD price
// (https://api-docs.deepseek.com/quick_start/pricing, retrieved 2026-05-31). The live official catalog is
// now exactly {deepseek-v4-flash, deepseek-v4-pro}; deepseek-chat/deepseek-reasoner are being deprecated
// (they map to flash non-thinking / thinking). v4-pro/v4-flash MUST precede the bare "deepseek" catch-all:
// "deepseek" is a substring of "deepseek-v4-pro", so an earlier liberal row would steal the match and
// under-bill the flagship — the exact OBL-012 class of bug. The legacy reasoner/deepseek baseline pins are
// kept (after the specific rows) so earlier banked-per-dollar tapes stay comparable — re-pin deliberately.
const MODEL_RATES: &[(&str, i64, i64)] = &[
    ("deepseek-ai/DeepSeek-V3.2", 270_000, 410_000),   // SiliconFlow $0.27 / $0.41
    ("Qwen/Qwen3-32B", 140_000, 570_000),              // SiliconFlow $0.14 / $0.57
    ("Qwen/Qwen2.5-72B-Instruct", 590_000, 590_000),   // SiliconFlow $0.59 / $0.59
    ("deepseek-v4-pro", 435_000, 870_000),             // DeepSeek API $0.435 / $0.87 (75%-off promo; regular $1.74/$3.48 = 1_740_000/3_480_000 — re-pin when promo ends)
    ("deepseek-v4-flash", 140_000, 280_000),           // DeepSeek API $0.14 cache-miss / $0.28
    ("reasoner", 550_000, 2_190_000),                  // DeepSeek API $0.55 / $2.19 (legacy baseline pin)
    ("deepseek", 270_000, 1_100_000),                  // DeepSeek API $0.27 / $1.10 (legacy baseline catch-all — MUST stay last)
];
// FALLBACK for an id not in MODEL_RATES — clearly a PROXY, not a true price (deepseek-chat-class). An
// unlisted model is a roster gap to close (add a row above), never a license to under-bill the metric.
const FALLBACK_IN_UPMT: i64 = 270_000;
const FALLBACK_OUT_UPMT: i64 = 1_100_000;

/// integer micro-USD for a call, by model (the real dollar cost — the scarce resource's denominator).
/// First MODEL_RATES row whose substring is in `model` wins (most-specific-first); else FALLBACK.
fn call_micro_usd(model: &str, prompt_tok: u64, completion_tok: u64) -> i64 {
    let (i, o) = {
        let mut rate = (FALLBACK_IN_UPMT, FALLBACK_OUT_UPMT);
        for &(id, in_upmt, out_upmt) in MODEL_RATES {
            if model.contains(id) {
                rate = (in_upmt, out_upmt);
                break;
            }
        }
        rate
    };
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
        // het8: held-out conjunction (tuning guard) — same 4 families, fresh goals, more contention.
        "het8" => Some(vec![
            ("4 * n + 2 ≤ 9 * n + 2 + n", "omega"),
            ("(a + b)^2 - (a - b)^2 = 4*a*b", "ring"),
            ("(∑ i ∈ Finset.range (n+1), (i:ℤ)) * 2 = n * (n+1)", "induction"),
            ("a^2 + b^2 + 1 ≥ a*b + a*b", "nlinarith"),
            ("2 * n + 7 ≤ 5 * n + 7 + n", "omega"),
            ("(a + 2*b) * (a - 2*b) = a^2 - 4*b^2", "ring"),
            ("a^2 + 4*b^2 ≥ 4*a*b", "nlinarith"),
            ("3 * n ≤ 8 * n + 1", "omega"),
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
// ── axiom-clean gate (real `#print axioms`, not a string ban) ───────────────
// A proof that merely COMPILES is not sound: it can smuggle `sorryAx` (sorry/admit), the
// native_decide trust axioms (`Lean.ofReduceBool`/`Lean.trustCompiler`), or a hand-declared `axiom`,
// all of which produce NO "error" and slip past an exit-success + string-ban check. The only honest
// certificate is Lean's own `#print axioms`: the proof's transitive axiom set must be ⊆ the standard
// classical trust base. THIS is what makes the "axiom-clean (no sorryAx)" doc claims true.
const AXIOM_ALLOWLIST: [&str; 3] = ["propext", "Classical.choice", "Quot.sound"];

/// Parse the dependency set printed by `#print axioms <name>` out of Lean's raw (ORIGINAL-case)
/// output. Lean emits exactly one of:
///   `'<name>' depends on axioms: [propext, Classical.choice, Quot.sound]`
///   `'<name>' does not depend on any axioms`
/// Returns the axiom names (empty set for the "no axioms" case), or None if no such line is present
/// (which co-occurs with a hard compile error). Case-sensitive — axiom names are (sorryAx, Quot.sound).
fn parse_axiom_set(raw: &str) -> Option<BTreeSet<String>> {
    if raw.contains("does not depend on any axioms") { return Some(BTreeSet::new()); }
    let after = &raw[raw.find("depends on axioms:")? + "depends on axioms:".len()..];
    let lb = after.find('[')?;
    let rb = after[lb..].find(']')? + lb;
    Some(after[lb + 1..rb].split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
}

/// True iff every axiom the proof depends on is in AXIOM_ALLOWLIST. None (no `#print axioms` line was
/// parsed) is NOT clean — fail-closed: never certify "axiom-clean" without having read the axiom set.
fn axiom_set_clean(axioms: &Option<BTreeSet<String>>) -> bool {
    match axioms {
        Some(set) => set.iter().all(|a| AXIOM_ALLOWLIST.contains(&a.as_str())),
        None => false,
    }
}

/// Wrap a candidate proof's full source (which MUST define a theorem named `thm_name`), append
/// `#print axioms <thm_name>`, run Lean ONCE, and report (compiles_clean, axiom_set, lowercased_out).
/// The spec'd pair is (compiles_clean, axiom_set); the third value is the lowercased Lean output,
/// returned so callers reuse the SAME run for their error-class signal (we never run Lean twice).
/// `compiles_clean` = exit 0 ∧ no "error" in output. A `sorry` is a *warning*, deliberately NOT failed
/// here — it surfaces as `sorryAx` in `axiom_set`, which is the principled (and string-ban-proof) catch.
fn run_lean_axioms(src: &str, thm_name: &str, lean_bin: &Path, mathlib_dir: &Path, lp: &str, tag: &str)
    -> (bool, Option<BTreeSet<String>>, String) {
    let full = format!("{src}\n#print axioms {thm_name}\n");
    let file = std::env::temp_dir().join(format!("axck_{tag}.lean"));
    if std::fs::write(&file, &full).is_err() { return (false, None, "io".into()); }
    match std::process::Command::new(lean_bin).arg(&file).current_dir(mathlib_dir).env("LEAN_PATH", lp).output() {
        Ok(o) => {
            let raw = format!("{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr));
            let low = raw.to_lowercase();
            let compiles_clean = o.status.success() && !low.contains("error");
            let axioms = parse_axiom_set(&raw);
            (compiles_clean, axioms, low)
        }
        Err(_) => (false, None, "exec".into()),
    }
}

/// Verify a competing conjunct proof: it must type-check under Lean AND be axiom-clean (axiom set ⊆
/// AXIOM_ALLOWLIST — no sorryAx, no native_decide trust axioms, no hand-declared axiom). The theorem
/// is named `c_{tag}` so `#print axioms` can certify it (one Lean run, inside run_lean_axioms).
fn verify_conjunct(goal: &str, proof: &str, lean_bin: &Path, mathlib_dir: &Path, lp: &str, tag: &str) -> bool {
    let low = proof.to_lowercase();
    if low.contains("sorry") || low.contains("admit") { return false; }
    let name = format!("c_{tag}");
    let src = format!("import Mathlib\nopen Finset in\ntheorem {name} {PREAMBLE_VARS} : {goal} := by\n{}",
        proof.lines().map(|l| format!("  {l}")).collect::<Vec<_>>().join("\n"));
    let (compiles_clean, axioms, _) = run_lean_axioms(&src, &name, lean_bin, mathlib_dir, lp, tag);
    compiles_clean && axiom_set_clean(&axioms)
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
/// Ensure a pool preamble's LAST declaration is a NAMED theorem so `#print axioms <name>` can run.
/// A preamble ends in `<decl> ... := by` where <decl> is either `example` (anonymous) or a named
/// `theorem`/`lemma`. Returns (named_preamble, name): for `example` the keyword is rewritten to
/// `theorem <gen>`; an already-named decl is reused verbatim. The target is the LAST decl (the one the
/// appended body completes) — earlier helper decls are left alone. Assumes line (`--`) comments only.
fn ensure_named(preamble: &str, tag: &str) -> (String, String) {
    let gen: String = format!(
        "pool_{}",
        tag.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect::<String>()
    );
    let ident = |c: char| c.is_alphanumeric() || c == '_' || c == '\'' || c == '.';
    let is_example = |t: &str| match t.strip_prefix("example") {
        Some(r) => r.chars().next().map_or(true, |c| !ident(c)),
        None => false,
    };
    let lines: Vec<&str> = preamble.lines().collect();
    let mut decl_idx: Option<usize> = None;
    for (i, line) in lines.iter().enumerate() {
        let t = line.trim_start();
        if is_example(t) || t.starts_with("theorem ") || t.starts_with("lemma ") { decl_idx = Some(i); }
    }
    let di = match decl_idx { Some(i) => i, None => return (preamble.to_string(), gen) };
    let line = lines[di];
    let indent = &line[..line.len() - line.trim_start().len()];
    let t = line.trim_start();
    let (new_line, name) = if is_example(t) {
        (format!("{indent}theorem {gen}{}", &t["example".len()..]), gen.clone())
    } else {
        let kw = if t.starts_with("theorem ") { "theorem " } else { "lemma " };
        let nm: String = t[kw.len()..].trim_start().chars().take_while(|&c| ident(c)).collect();
        if nm.is_empty() { return (preamble.to_string(), gen); }
        (line.to_string(), nm)
    };
    let mut out: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
    out[di] = new_line;
    (out.join("\n"), name)
}
/// Verify a full pool theorem (preamble + candidate body) under Lean AND certify it is axiom-clean:
/// `#print axioms` must report only axioms in AXIOM_ALLOWLIST (no sorryAx, no native_decide axioms, no
/// hand-declared axiom). The real gate lives in verify_pool_err / run_lean_axioms.
fn verify_pool(preamble: &str, body: &str, lean_bin: &Path, mathlib_dir: &Path, lp: &str, tag: &str) -> bool {
    verify_pool_err(preamble, body, lean_bin, mathlib_dir, lp, tag).0
}
/// As verify_pool but also returns a coarse error CLASS — a PREDICTABLE proximity-to-correct signal an
/// assessor can read (a near-miss vs a far-miss), so the market price can discriminate repair-EV.
/// Returns (verified, error_class). Classes ordered roughly near→far:
///   "unsolved_goals" (proof shape right, goals left) > "type_mismatch" (close) > "rewrite_failed"
///   > "unknown_id" (wrong lemma name) > "parse" (malformed) > "none" (verified) / "other".
/// Soundness classes (compiled with NO error but axiom-DIRTY, so the old gate would have wrongly
/// passed them): "sorry_axiom" (sorryAx — sorry/admit), "nonstandard_axiom" (native_decide trust
/// axioms or a hand-declared `axiom`), "axiom_unparsed" (no `#print axioms` line — fail-closed).
fn verify_pool_err(preamble: &str, body: &str, lean_bin: &Path, mathlib_dir: &Path, lp: &str, tag: &str) -> (bool, String) {
    let low = body.to_lowercase();
    if low.contains("sorry") || low.contains("admit") || low.contains("native_decide") { return (false, "bypass".into()); }
    let (named_preamble, name) = ensure_named(preamble, tag);
    let indented: String = body.lines().map(|l| format!("  {l}")).collect::<Vec<_>>().join("\n");
    let src = format!("{named_preamble}\n{indented}");
    let (compiles_clean, axioms, t) = run_lean_axioms(&src, &name, lean_bin, mathlib_dir, lp, tag);
    if compiles_clean {
        // AXIOM GATE — the real `#print axioms` check. The proof type-checks; it counts ONLY if every
        // axiom it depends on is in AXIOM_ALLOWLIST. sorryAx, native_decide's Lean.ofReduceBool /
        // Lean.trustCompiler, and hand-declared axioms all compile WITHOUT any "error" and would slip
        // past the old exit-success + string-ban check; here they are rejected.
        if axiom_set_clean(&axioms) { return (true, "none".into()); }
        let class = match &axioms {
            Some(s) if s.contains("sorryAx") => "sorry_axiom",
            Some(_) => "nonstandard_axiom",
            None => "axiom_unparsed",
        };
        return (false, class.into());
    }
    // Genuine compile error → the existing proximity-to-correct class (price-discrimination signal).
    let class = if t.contains("unsolved goals") { "unsolved_goals" }
        else if t.contains("type mismatch") { "type_mismatch" }
        else if t.contains("rewrite") || t.contains("motive is not type correct") { "rewrite_failed" }
        else if t.contains("unknown identifier") || t.contains("unknown constant") { "unknown_id" }
        else if t.contains("unexpected token") || t.contains("expected") { "parse" }
        else { "other" };
    (false, class.into())
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

/// SKILLSWEEP — real-data reproduction of the architect's routing-policy A/B test (autonomous price
/// discovery vs softmax forced router), sweeping agent SKILL. The user's Monte-Carlo sim found a
/// crossover near skill≈0.45-0.60: below it softmax forced routing wins, above it autonomous price
/// discovery wins (high-skill agents find hidden gems). This validates that crossover on a REAL
/// (agent × task) competence matrix collected from real DeepSeek + real Lean — the one thing a synthetic
/// sim cannot pin: where DeepSeek's actual skill falls.
///
/// SKILL model: an agent's routing decision uses a self-estimate of "can I close task t" = a blend of its
/// TRUE competence (success[a][t]) and NOISE, mixed by skill∈[0,1]. skill=1 → perfect self-knowledge
/// (always routes to a task it can actually close); skill=0 → pure noise (random self-belief). This is
/// exactly the user's "ability to interpret price / identify hidden gems" axis, grounded in real Lean
/// outcomes. AUTONOMOUS arm: each task goes to the agent with the highest self-estimate (decentralized
/// self-selection). SOFTMAX arm: top-level samples by softmax(true-price/τ=0.10) — price mechanically
/// broadcast, no agent self-judgment. HIDDEN-GEM tasks (rare, only one specialist closes them) reward
/// skilled autonomous discovery and punish noisy self-belief — reproducing the sim's key asymmetry.
async fn run_skillsweep(args: &Args, llm: &ResilientLLMClient, lean_bin: &Path, lp: &str) -> Result<(), String> {
    let mut rng = StdRng::seed_from_u64(args.seed);
    let t0 = Instant::now();
    let mut tape = MarketTape::new();
    let conj = task("het6").unwrap();
    let families = FAMILIES;
    // agent pool: the 4 honest specialists (each truly closes its own family) — clean skill axis, no Sybil.
    let specialists = [Some("omega"), Some("ring"), Some("induction"), Some("nlinarith")];
    let na = specialists.len();
    let mut tokens = 0u64; let mut llm_calls = 0usize; let mut lean_calls = 0usize;
    // PHASE 1: real (agent × family) competence + the per-family TRUE price (= fraction of agents that close it).
    let mut success = vec![vec![false; families.len()]; na];
    for (ai, fam) in specialists.iter().enumerate() {
        for (fi, f) in families.iter().enumerate() {
            let goal = conj.iter().find(|(_, tf)| tf == f).map(|(g, _)| *g).unwrap_or(conj[0].0);
            let role = fam.map(|sf| format!("You are a {sf} specialist. {}", family_hint(sf))).unwrap_or_default();
            let prompt = format!("{role}\n\nProve (context `(n:ℕ)(a b:ℝ)`):\n{goal} := by\nOutput JSON {{\"tactic\":\"<proof>\"}}. No sorry. If not your specialty, {{\"tactic\":\"SKIP\"}}.");
            let resp = match llm.generate(&GenerateRequest { model: args.model.clone(), messages: vec![Message { role: "user".into(), content: prompt }], temperature: Some(0.3), max_tokens: Some(300) }).await {
                Ok(r) => { tokens += (r.prompt_tokens+r.completion_tokens) as u64; llm_calls += 1; r }
                Err(_) => continue,
            };
            let tac = extract(&resp.content, "tactic").and_then(|v| v.as_str().map(String::from)).unwrap_or_default();
            if !tac.trim().is_empty() && tac.to_uppercase() != "SKIP" {
                lean_calls += 1;
                success[ai][fi] = verify_conjunct(goal, &tac, lean_bin, &args.mathlib_dir, lp, &format!("sk_{}_{}_{}", args.seed, ai, fi));
            }
        }
    }
    // PHASE 2: replay autonomous vs softmax across a SKILL sweep on the frozen matrix (cheap, deterministic).
    // includes HIDDEN-GEM tasks: a rare task family only ONE specialist closes → rewards skilled discovery.
    // HIDDEN-GEM family: pick a family that exactly ONE agent closes (rare competence) and assign it a high
    // REWARD (3×). Price (= fraction of agents that close it) is LOW for it (only 1/na), so a softmax router
    // UNDER-routes it; but a high-skill autonomous agent that KNOWS it can close this rare-but-valuable task
    // captures the reward. This reproduces the sim's "low-price high-reward hidden node" asymmetry on real data.
    let agent_closes: Vec<usize> = (0..families.len()).map(|fi| (0..na).filter(|&a| success[a][fi]).count()).collect();
    let gem_fam = (0..families.len()).filter(|&fi| agent_closes[fi] == 1).min_by_key(|&fi| agent_closes[fi]);
    let gem_reward = 3i64;
    let skills = [0.15, 0.30, 0.45, 0.60, 0.75, 0.90];
    let stream_len = args.n_rounds.max(40);
    // stream over the families that ARE closable by someone (incl. the gem); reward-weighted scoring.
    let closable: Vec<usize> = (0..families.len()).filter(|&fi| agent_closes[fi] >= 1).collect();
    let stream: Vec<usize> = (0..stream_len).map(|_| closable[rng.gen_range(0..closable.len())]).collect();
    let reward = |fi: usize| -> i64 { if Some(fi) == gem_fam { gem_reward } else { 1 } };
    let mut sweep = serde_json::Map::new();
    for &skill in &skills {
        // self-estimate[a][t] = blend(true success, noise) by skill. Deterministic per (skill, agent, fam).
        let mut auto_closed = 0i64; let mut soft_closed = 0i64; // REWARD-weighted (gem worth 3×)
        let mut srng = StdRng::seed_from_u64(args.seed ^ ((skill * 1000.0) as u64));
        for &fi in &stream {
            // AUTONOMOUS: each agent forms a noisy self-estimate; task → highest self-estimate.
            let est: Vec<f64> = (0..na).map(|a| {
                let truth = if success[a][fi] { 1.0 } else { 0.0 };
                skill * truth + (1.0 - skill) * srng.gen::<f64>()
            }).collect();
            let auto_pick = (0..na).max_by(|&x, &y| est[x].partial_cmp(&est[y]).unwrap()).unwrap();
            if success[auto_pick][fi] { auto_closed += reward(fi); }
            // SOFTMAX forced: top-level samples by softmax(competence-price / τ). Price = fraction of agents
            // that close this family — the gem's price is LOW (1/na), so softmax under-prioritizes it.
            let tau = 0.10f64;
            let price: Vec<f64> = (0..na).map(|a| if success[a][fi] { agent_closes[fi] as f64 / na as f64 } else { 0.0 }).collect();
            let mx = price.iter().cloned().fold(f64::MIN, f64::max);
            let w: Vec<f64> = price.iter().map(|p| ((p - mx) / tau).exp()).collect();
            let sum: f64 = w.iter().sum();
            let mut r = srng.gen::<f64>() * sum; let mut soft_pick = 0;
            for (a, wi) in w.iter().enumerate() { r -= wi; if r <= 0.0 { soft_pick = a; break; } }
            if success[soft_pick][fi] { soft_closed += reward(fi); }
        }
        let delta = auto_closed - soft_closed; // Δ = autonomous − softmax (reward-weighted; >0 ⇒ autonomous wins)
        sweep.insert(format!("skill_{:.2}", skill), serde_json::json!({"autonomous": auto_closed, "softmax": soft_closed, "delta": delta}));
        for c in 0..4 { tape.record(&MarketEvent::RouteSample { policy: format!("skill{:.2}", skill), frontier_hash: short_hash(&format!("{skill}{c}")), selected_claim: c }); }
    }
    let _gem = gem_fam;
    let wall = t0.elapsed().as_secs_f64();
    let chain_ok = tape.verify_chain();
    let mut manifest = serde_json::json!({
        "schema": "lean_skillsweep.v2", "policy": "skillsweep", "seed": args.seed, "stream_len": stream_len,
        "competence_matrix": success, "hidden_gem_family": gem_fam, "gem_reward": gem_reward,
        "llm_calls": llm_calls, "lean_calls": lean_calls, "tokens": tokens,
        "tape_chain_ok": chain_ok, "wall_s": wall,
    });
    for (k, v) in sweep { manifest.as_object_mut().unwrap().insert(k, v); }
    let _ = std::fs::write(&args.out, serde_json::to_string_pretty(&manifest).unwrap());
    if let Some(tp) = &args.tape_out { let _ = std::fs::write(tp, tape.lines.join("\n")); }
    print!("skillsweep[seed{}] Δ(auto−softmax) by skill:", args.seed);
    for &s in &skills { let d = manifest.get(&format!("skill_{:.2}", s)).and_then(|v| v.get("delta")).and_then(|v| v.as_i64()).unwrap_or(0); print!(" {:.2}={:+}", s, d); }
    println!(" chain_ok={} wall={:.1}s", chain_ok, wall);
    Ok(())
}

/// REPUTATION — price as a no-regret COMPETENCE allocator under agent heterogeneity + SPAM (the regime
/// the literature says capital-at-risk uniquely wins, and the regime TuringOS's design is actually FOR).
///
/// Prior negatives tested price as an AGGREGATOR of correlated judgments (a known null). This tests the
/// DIFFERENT, design-native claim: a stream of proof TASKS arrives; agents have heterogeneous, UNKNOWN-
/// in-advance competence (different specialists solve different tasks) PLUS a SPAM agent that always
/// claims it can solve everything with high confidence (models are systematically over-confident —
/// arXiv:2508.06225). A scarce execution budget must be routed each round to ONE agent per task. The
/// question: does PRICE (persistent capital — agents bid loss-bearing stake from a wallet that compounds
/// on success / drains on failure) route the scarce budget to the genuinely-competent agent and DEFUND
/// the spammer FASTER than (a) trust-everyone confidence routing [the spam-vulnerable baseline], (b)
/// fixed round-robin, (c) random? This is the Chen-Vaughan no-regret claim: converge to the best agent
/// per task-type WITHOUT knowing competence in advance, and be ROBUST to a strategic over-claimer that
/// fools static confidence. Capital-at-risk's provable edge is exactly Sybil/spam-resistance + dynamic
/// reweighting — a PERFORMANCE-relevant causal advantage a static confidence-weighted scheme cannot have.
///
/// Method (efficient + replayable): FIRST collect a real (agent × task) competence matrix — each
/// specialist's actual Lean-verified success + its self-reported confidence on each task (real LLM, real
/// Lean, once). THEN replay every routing policy on the SAME frozen matrix (deterministic, no re-querying)
/// so the comparison is apples-to-apples and cheap. Price wins iff, summed over the task stream under a
/// binding budget, capital-routing closes more tasks than confidence-routing (spam-fooled) and round-robin.
async fn run_reputation(args: &Args, llm: &ResilientLLMClient, lean_bin: &Path, lp: &str) -> Result<(), String> {
    let mut rng = StdRng::seed_from_u64(args.seed);
    let t0 = Instant::now();
    let mut tape = MarketTape::new();
    // task stream: cycle the het6 conjuncts (each conjunct = one task with a known truth-family).
    let conj = task("het6").unwrap();
    let stream_len = args.n_rounds.max(12);
    let stream: Vec<usize> = (0..stream_len).map(|_| rng.gen_range(0..conj.len())).collect();
    // agents: 4 honest specialists + 3 SYBIL spammers that flood MAX confidence (100) but never solve.
    // Sybil flooding is the canonical attack confidence-routing CANNOT resist (more identities = more
    // max-confidence claims) but capital-at-risk CAN (each Sybil must stake real capital and loses it on
    // failure → all Sybils drain to zero, permanently defunded). This is the literature's exact
    // price-wins regime + TuringOS's design rationale (Sybil-resistance as a performance property).
    let specialists: Vec<Option<&str>> = vec![Some("omega"), Some("ring"), Some("induction"), Some("nlinarith"), None, None, None];
    let spam_lo = 4usize; // agents 4,5,6 are Sybils
    let spam_idx = spam_lo;
    let na = specialists.len();
    let mut tokens = 0u64; let mut llm_calls = 0usize; let mut lean_calls = 0usize;

    // ── PHASE 1: collect the REAL (agent × task-family) competence + self-confidence matrix, once ──
    // success[a][f] = does agent a actually Lean-close a task of family f? conf[a][f] = its self-claim.
    let families = FAMILIES;
    let mut success = vec![vec![false; families.len()]; na];
    let mut conf = vec![vec![0u64; families.len()]; na];
    for (ai, fam) in specialists.iter().enumerate() {
        for (fi, f) in families.iter().enumerate() {
            // representative goal of this family (first het6 conjunct of that family).
            let goal = conj.iter().find(|(_, tf)| tf == f).map(|(g, _)| *g).unwrap_or(conj[0].0);
            let is_spam = ai >= spam_lo; // agents spam_lo.. are ALL Sybils (was: only ==spam_idx → 5,6 leaked)
            let role = match fam {
                Some(sf) => format!("You are a {sf} specialist. {}", family_hint(sf)),
                None if is_spam => "You are a confident generalist — you can prove ANYTHING; always try and report high confidence.".to_string(),
                None => "You are a generalist prover.".to_string(),
            };
            let prompt = format!("{role}\n\nProve (context `(n:ℕ)(a b:ℝ)`):\n{goal} := by\nOutput JSON {{\"tactic\":\"<proof>\",\"confidence\":0-100}}. No sorry. If not your specialty, {{\"tactic\":\"SKIP\",\"confidence\":0}}.");
            let resp = match llm.generate(&GenerateRequest { model: args.model.clone(), messages: vec![Message { role: "user".into(), content: prompt }], temperature: Some(0.3), max_tokens: Some(300) }).await {
                Ok(r) => { tokens += (r.prompt_tokens+r.completion_tokens) as u64; llm_calls += 1; r }
                Err(_) => continue,
            };
            let tac = extract(&resp.content, "tactic").and_then(|v| v.as_str().map(String::from)).unwrap_or_default();
            let mut c = extract(&resp.content, "confidence").and_then(|v| v.as_u64()).unwrap_or(50).min(100);
            if is_spam {
                // SYBIL: floods MAX confidence (100, tying any real specialist) but NEVER solves. Static
                // confidence-routing splits its pick among all max-confidence claimants → mostly picks a
                // Sybil (3 Sybils vs 1 real ⇒ ~3/4 of probes wasted). Capital-at-risk DEFUNDS each Sybil
                // after its first failure, so price recovers to the real specialist — the performance-causal
                // Sybil-resistance the literature credits to capital-at-risk + TuringOS's design rationale.
                conf[ai][fi] = 100;
                success[ai][fi] = false; // the over-claim is empty
                continue;
            }
            conf[ai][fi] = c;
            if !tac.trim().is_empty() && tac.to_uppercase() != "SKIP" {
                lean_calls += 1;
                success[ai][fi] = verify_conjunct(goal, &tac, lean_bin, &args.mathlib_dir, lp, &format!("rep_{}_{}_{}", args.seed, ai, fi));
            }
            // STRICT-SPECIALIST constraint (referee's combination/single_best fairness fix): an honest
            // specialist's REAL competence is its own family only. DeepSeek sometimes closes off-family
            // goals (a generalist leak) → that makes "single best agent" unfairly strong (it covers many
            // families alone) and confounds "economy > single agent". Pin honest agent i to TRUE 1/4: it
            // succeeds ONLY on its own family. Sybils stay 0/4. Now NO single agent covers >1 family, so
            // single_best caps at ~1/4 of the stream and the economy MUST combine specialists to win. The
            // success values are still REAL (Lean-verified on-family); we only zero the off-family leaks.
            if let Some(my_fam) = fam {
                if families[fi] != *my_fam { success[ai][fi] = false; }
            }
        }
    }

    // ── PHASE 2: replay every routing policy on the SAME frozen matrix (cheap, deterministic) ──
    // Each round, one task arrives; route to ONE agent (per the policy); it consumes a probe; closes iff
    // success[a][family]. PRICE arm: persistent reputation-capital, agents bid stake∝(wealth×conf); route
    // to top bid; success compounds wealth, failure drains it → spammer defunded. CONFIDENCE arm: route to
    // highest raw self-confidence (the spam-vulnerable baseline). ROUNDROBIN / RANDOM: ignore signals.
    let fam_idx = |f: &str| families.iter().position(|x| x == &f).unwrap_or(0);
    let run_policy = |policy: &str, rng: &mut StdRng| -> (i64, Vec<i64>) {
        let mut wealth = vec![100_000i64; na];
        // conf_learned baseline: a FAIR strong rival — per-(agent,family) running success rate (Laplace),
        // routes to the best EMPIRICAL performer. This is the real test: does capital-at-risk beat an
        // adaptive success-tracking router, not just a naive static-confidence one?
        let mut wins = vec![vec![0i64; families.len()]; na];
        let mut tries = vec![vec![0i64; families.len()]; na];
        // elim_global: the DECISIVE no-capital rival (referee's gap). Agent-level pooled success tracker
        // with terminal elimination — mirrors price's terminal global defunding but with ZERO capital.
        // Isolates "capital-at-risk" from "global-vs-per-family state granularity".
        let mut gwins = vec![0i64; na];
        let mut gtries = vec![0i64; na];
        let mut alive = vec![true; na];
        let mut closed = 0i64; let mut rr = 0usize;
        for &task_c in &stream {
            let fi = fam_idx(conj[task_c].1);
            let chosen = match policy {
                "price" => {
                    // bid = stake ∝ wealth × self-confidence; route to the highest bidder. A defunded Sybil
                    // (wealth→0) bids ~0 and is never chosen again.
                    (0..na).max_by_key(|&a| wealth[a].max(0) as i128 * conf[a][fi] as i128).unwrap()
                }
                "confidence" => {
                    // naive static confidence — uniform among max-confidence claimants (Sybil-flooded).
                    let mx = (0..na).map(|a| conf[a][fi]).max().unwrap();
                    let top: Vec<usize> = (0..na).filter(|&a| conf[a][fi] == mx).collect();
                    top[rng.gen_range(0..top.len())]
                }
                "conf_learned" => {
                    // FAIR strong baseline: route to best (Laplace-smoothed) empirical success rate; explore
                    // unseen agents first. Tracks realized outcomes just like the market does — but with NO
                    // capital, so it cannot DEFUND a Sybil, only down-rank it after wasted probes.
                    (0..na).max_by_key(|&a| {
                        let w = wins[a][fi]; let t = tries[a][fi];
                        if t == 0 { 1_000_000i64 } else { (w * 1000) / (t + 1) } // unseen → explore
                    }).unwrap()
                }
                "softmax" => {
                    // SOFTMAX FORCED ROUTER (architect's path 2, τ=0.10): top-level samples the agent by
                    // softmax(competence_price / τ) over the per-family price signal. Agents do NOT self-
                    // select; the whitebox mechanically broadcasts price as a probability distribution.
                    // Uses each agent's self-confidence as the price proxy (tape-reconstructible signal).
                    let tau = 0.10f64;
                    let prices: Vec<f64> = (0..na).map(|a| conf[a][fi] as f64 / 100.0).collect();
                    let mx = prices.iter().cloned().fold(f64::MIN, f64::max);
                    let w: Vec<f64> = prices.iter().map(|p| ((p - mx) / tau).exp()).collect();
                    let sum: f64 = w.iter().sum();
                    let mut r = rng.gen::<f64>() * sum; let mut pick = 0;
                    for (a, wi) in w.iter().enumerate() { r -= wi; if r <= 0.0 { pick = a; break; } }
                    pick
                }
                "elim_global" => {
                    // route to the ALIVE agent with the best Laplace GLOBAL success rate (pooled across all
                    // families, NOT per-family); unseen-alive agents explored first. No capital.
                    (0..na).filter(|&a| alive[a]).max_by_key(|&a| {
                        if gtries[a] == 0 { i64::MAX } else { (gwins[a] * 1000) / (gtries[a] + 1) }
                    }).unwrap_or(0)
                }
                "single_best" => {
                    // NO-ECONOMY control: every task goes to the ONE agent with the highest TOTAL competence
                    // (most families closed) — "just rely on the single best generalist". The economy must
                    // beat this to prove multi-agent coordination adds value over picking one strong agent.
                    (0..na).max_by_key(|&a| (0..families.len()).filter(|&f| success[a][f]).count()).unwrap()
                }
                _ => rng.gen_range(0..na), // random
            };
            let ok = success[chosen][fi];
            if ok { closed += 1; }
            tries[chosen][fi] += 1; if ok { wins[chosen][fi] += 1; }
            // global trackers (for elim_global): pool outcomes across families + terminal elimination.
            gtries[chosen] += 1; if ok { gwins[chosen] += 1; }
            if policy == "elim_global" && gtries[chosen] >= 1 && gwins[chosen] == 0 { alive[chosen] = false; }
            // settle (price arm only): winner's wealth grows, loser's drains — the no-regret reweighting.
            if policy == "price" {
                let stake = (wealth[chosen].max(0) * conf[chosen][fi] as i64 / 100 / 5).max(1);
                if ok { wealth[chosen] += stake / 2; } else { wealth[chosen] -= stake; }
            }
        }
        (closed, wealth)
    };

    let mut results = serde_json::Map::new();
    let mut wealth_price = vec![];
    // "softmax" = architect path 2 (forced router τ=0.10); "confidence" doubles as path-1-autonomous
    // (agent self-selects by its own confidence signal — the autonomous-skill axis the user's sim swept).
    for policy in ["price", "softmax", "confidence", "conf_learned", "elim_global", "roundrobin", "random", "single_best"] {
        let mut prng = StdRng::seed_from_u64(args.seed ^ 0x9e3779b9);
        let (closed, wealth) = run_policy(policy, &mut prng);
        results.insert(format!("{policy}_closed"), serde_json::json!(closed));
        for c in 0..stream_len { tape.record(&MarketEvent::RouteSample { policy: policy.into(), frontier_hash: short_hash(&format!("{policy}{c}")), selected_claim: c }); }
        if policy == "price" { wealth_price = wealth; }
    }

    let wall = t0.elapsed().as_secs_f64();
    let chain_ok = tape.verify_chain();
    let mut manifest = serde_json::json!({
        "schema": "lean_reputation.v1", "policy": "reputation", "seed": args.seed, "stream_len": stream_len,
        "competence_matrix_success": success, "self_confidence": conf, "spam_agent_idx": spam_idx,
        "final_wealth_price": wealth_price, "llm_calls": llm_calls, "lean_calls": lean_calls,
        "tokens": tokens, "tape_chain_ok": chain_ok, "wall_s": wall,
    });
    for (k, v) in results { manifest.as_object_mut().unwrap().insert(k, v); }
    let _ = std::fs::write(&args.out, serde_json::to_string_pretty(&manifest).unwrap());
    if let Some(tp) = &args.tape_out { let _ = std::fs::write(tp, tape.lines.join("\n")); }
    let g = |k: &str| manifest.get(k).and_then(|v| v.as_i64()).unwrap_or(0);
    println!("reputation[{}tasks] price={} elim_global={} conf_learned={} softmax={} confidence={} single_best={} roundrobin={} random={} chain_ok={} wall={:.1}s",
        stream_len, g("price_closed"), g("elim_global_closed"), g("conf_learned_closed"), g("softmax_closed"), g("confidence_closed"), g("single_best_closed"), g("roundrobin_closed"), g("random_closed"), chain_ok, wall);
    Ok(())
}

/// REPEATED — the ONE price-causality test the literature says CAN win (researched). Single-shot, a
/// market price IS a confidence-weighted average (Kelly-bettor theorem, arXiv:1201.6655) — provably no
/// better than averaging. Capital-at-risk's UNIQUE, provable edge is ACROSS REPEATED ROUNDS with
/// PERSISTENT WEALTH: accurate bettors' capital compounds, so the wealth-weighted (market) verdict
/// DYNAMICALLY self-calibrates and is no-regret vs the BEST bettor — WITHOUT knowing who that is in
/// advance (Chen-Vaughan FTRL O(√T), arXiv:1003.0034). This is also exactly the constitution's intent:
/// a PERSISTENT ledger across tasks, not a single shot.
///
/// Design: a sequence of T proof-verification questions (does proof P_t of goal G_t compile?). N
/// heterogeneous assessors (different models/temps → genuinely DISPERSED noisy private judgments — the
/// Hong-Page diversity the prior negatives lacked) each round bet capital YES/NO from a PERSISTENT
/// wallet; the wealth-weighted market verdict is recorded; Lean settles (ground truth); winners' wealth
/// compounds. Compare the MARKET verdict's accuracy to (a) the best single assessor, (b) a FIXED-weight
/// (unweighted) average, (c) a static confidence-weighted average. GO if the market (dynamic wealth
/// weighting) beats the fixed-weight baselines AND approaches the best single assessor over T rounds —
/// the no-regret convergence the theory predicts and a static average cannot do without an oracle.
async fn run_repeated(args: &Args, llm: &ResilientLLMClient, lean_bin: &Path, lp: &str) -> Result<(), String> {
    let mut rng = StdRng::seed_from_u64(args.seed);
    let t0 = Instant::now();
    let mut tape = MarketTape::new();
    // Heterogeneous assessor pool: distinct (model, temperature) → dispersed noisy judgments.
    let assessors: Vec<(&str, f64)> = vec![
        (args.model.as_str(), 0.2), (args.model.as_str(), 0.7),
        ("deepseek-reasoner", 0.3), (args.model.as_str(), 1.0),
        ("deepseek-reasoner", 0.6),
    ];
    let na = assessors.len();
    let mut wealth = vec![100_000i64; na];     // PERSISTENT wallets — the load-bearing change
    let mut correct = vec![0i64; na];          // per-assessor running accuracy (for "best single")
    let mut total = vec![0i64; na];
    // Build a sequence of (goal, proof, ground_truth) verification questions from cmp_* goals: for each
    // goal, generate a few proofs at varied temp and Lean-label them — a stream of real YES/NO questions.
    let goals = ["cmp_ineq", "cmp_pow", "cmp_amgm", "cmp_sum"];
    let mut questions: Vec<(String, String, bool)> = Vec::new();
    let mut tokens = 0u64; let mut llm_calls = 0usize; let mut lean_calls = 0usize;
    for g in goals.iter().cycle().take(args.n_rounds) {
        let (goal, _h) = compete_goal(g).ok_or("bad goal")?;
        let temp = 0.2 + 0.8 * rng.gen::<f64>();
        let prompt = format!("Prove this Lean 4 (Mathlib) goal (context `(n:ℕ)(a b:ℝ)`):\n{goal}\nOutput ONLY JSON {{\"tactic\":\"<proof body>\"}}. No sorry.");
        let resp = match llm.generate(&GenerateRequest { model: args.model.clone(), messages: vec![Message { role: "user".into(), content: prompt }], temperature: Some(temp), max_tokens: Some(400) }).await {
            Ok(r) => { tokens += (r.prompt_tokens+r.completion_tokens) as u64; llm_calls += 1; r }
            Err(_) => continue,
        };
        let tac = match extract(&resp.content, "tactic").and_then(|v| v.as_str().map(String::from)) { Some(t) if !t.trim().is_empty() => t.trim().to_string(), _ => continue };
        lean_calls += 1;
        let gt = verify_conjunct(goal, &tac, lean_bin, &args.mathlib_dir, lp, &format!("rep_{}_{}", args.seed, questions.len()));
        questions.push((goal.to_string(), tac, gt));
    }
    if questions.is_empty() { return Err("no questions generated".into()); }

    // ── REPEATED BETTING: each round, assessors bet from PERSISTENT wealth; market = wealth-weighted ──
    let (mut market_correct, mut unweighted_correct, mut confwt_correct) = (0i64, 0i64, 0i64);
    let n_rounds_real = questions.len();
    for (qi, (goal, proof, gt)) in questions.iter().enumerate() {
        tape.record(&MarketEvent::MarketOpen { claim: qi, claim_type: "verify_q".into() });
        let mut yes_wealth = 0i64; let mut no_wealth = 0i64;        // wealth-weighted market
        let mut yes_count = 0i64; let mut no_count = 0i64;          // unweighted vote
        let mut yes_conf = 0i64; let mut no_conf = 0i64;            // confidence-weighted (static)
        let mut votes: Vec<(bool, u64, i64)> = Vec::new();          // (said_yes, conf, stake) per assessor
        for ai in 0..na {
            let (m, t) = assessors[ai];
            let prompt = format!("Will this Lean 4 (Mathlib) proof of `{goal}` COMPILE with no error/sorry?\n```\n{proof}\n```\nOutput JSON {{\"verdict\":\"YES\"|\"NO\",\"confidence\":0-100}}. You stake capital and LOSE it if wrong.");
            let resp = match llm.generate(&GenerateRequest { model: m.into(), messages: vec![Message { role: "user".into(), content: prompt }], temperature: Some(t), max_tokens: Some(100) }).await {
                Ok(r) => { tokens += (r.prompt_tokens+r.completion_tokens) as u64; llm_calls += 1; tape.record(&MarketEvent::LlmCall { model: m.into(), prompt_tokens: r.prompt_tokens as u64, completion_tokens: r.completion_tokens as u64 }); r }
                Err(_) => continue,
            };
            let said_yes = extract(&resp.content, "verdict").and_then(|v| v.as_str().map(|s| s.to_uppercase())).map(|s| s.contains("YES")).unwrap_or(false);
            let conf = extract(&resp.content, "confidence").and_then(|v| v.as_u64()).unwrap_or(50).min(100);
            // stake = fraction of PERSISTENT wealth scaled by confidence (Kelly-ish; capital at risk).
            let stake = (wealth[ai].max(0) * conf as i64 / 100 / 4).clamp(0, wealth[ai].max(0));
            votes.push((said_yes, conf, stake));
            if said_yes { yes_wealth += stake; yes_count += 1; yes_conf += conf as i64; }
            else { no_wealth += stake; no_count += 1; no_conf += conf as i64; }
            total[ai] += 1; if said_yes == *gt { correct[ai] += 1; }
            tape.record(&MarketEvent::Invest { agent: ai, claim: qi, side: if said_yes {"YES".into()} else {"NO".into()}, amount_micro: stake, model_hash: short_hash(&format!("{m}:{t}")), confidence: conf });
        }
        // three aggregated verdicts:
        let market_says_yes = yes_wealth > no_wealth;          // WEALTH-weighted (the market)
        let unweighted_says_yes = yes_count > no_count;        // 1-agent-1-vote
        let confwt_says_yes = yes_conf > no_conf;              // static confidence-weighted
        if market_says_yes == *gt { market_correct += 1; }
        if unweighted_says_yes == *gt { unweighted_correct += 1; }
        if confwt_says_yes == *gt { confwt_correct += 1; }
        // SETTLE: winners (bet == gt) gain a share of losers' staked capital → wealth COMPOUNDS.
        let loser_pool: i64 = votes.iter().filter(|(sy,_,_)| sy != gt).map(|(_,_,s)| *s).sum();
        let winner_stake: i64 = votes.iter().filter(|(sy,_,_)| sy == gt).map(|(_,_,s)| *s).sum::<i64>().max(1);
        for (ai, (sy, _c, s)) in votes.iter().enumerate() {
            if sy == gt { wealth[ai] += loser_pool * s / winner_stake; }  // win: take share of loser pool
            else { wealth[ai] -= s; }                                      // lose: forfeit stake
        }
        tape.record(&MarketEvent::Resolve { claim: qi, outcome: if *gt {"YES".into()} else {"NO".into()} });
    }
    // best single assessor accuracy (the strong individual baseline).
    let best_single = (0..na).map(|i| if total[i]>0 { correct[i]*1000/total[i] } else { 0 }).max().unwrap_or(0);
    let wall = t0.elapsed().as_secs_f64();
    let chain_ok = tape.verify_chain();
    let manifest = serde_json::json!({
        "schema": "lean_repeated.v1", "policy": "repeated", "rounds": n_rounds_real, "seed": args.seed,
        "market_acc_pm": market_correct*1000/n_rounds_real as i64,
        "unweighted_acc_pm": unweighted_correct*1000/n_rounds_real as i64,
        "confwt_acc_pm": confwt_correct*1000/n_rounds_real as i64,
        "best_single_acc_pm": best_single,
        "final_wealth": wealth, "per_assessor_acc_pm": (0..na).map(|i| if total[i]>0 {correct[i]*1000/total[i]} else {0}).collect::<Vec<_>>(),
        "llm_calls": llm_calls, "lean_calls": lean_calls, "tokens": tokens, "tape_chain_ok": chain_ok, "wall_s": wall,
    });
    let _ = std::fs::write(&args.out, serde_json::to_string_pretty(&manifest).unwrap());
    if let Some(tp) = &args.tape_out { let _ = std::fs::write(tp, tape.lines.join("\n")); }
    println!("repeated[{}rounds] market_acc={} unweighted={} confwt={} best_single={} (per-mille) chain_ok={} wall={:.1}s",
        n_rounds_real, market_correct*1000/n_rounds_real as i64, unweighted_correct*1000/n_rounds_real as i64,
        confwt_correct*1000/n_rounds_real as i64, best_single, chain_ok, wall);
    Ok(())
}

/// PROBE-ALLOC — the decisive price-causality test (literature + strategy converged here). Price
/// allocates a SCARCE SHARED PROBE budget over COMPLEMENTARY specialists. The key fix vs het4: bind
/// the PROBE (the proof-GENERATING attempt), not the verify — so misallocating a probe to an
/// off-family specialist COSTS a closed conjunct. This is the one structure with BOTH real loss-bearing
/// price AND predictable variance (specialist↔subtask match is knowable pre-Lean), the pair all 3
/// prior negatives lacked (Selection Bottleneck Q=s·O+(1−s)·M: here Δ and s are both real).
///
/// One probe = one specialist LLM attempt on one conjunct + ≤1 Lean verify; consumed even on SKIP/fail.
/// Arms: market (price routes probe to highest-bid (conjunct,specialist)) / roundrobin (blind sweep,
/// same B) / shuffled (bid-prices permuted) / flatbid (constant bids — THE causal firewall) /
/// uniform (random) / single_strong (reasoner alone, given B). Metric: conjuncts closed within B,
/// each Lean-reverified, aggregate over seeds. Money integer; f64 only in routing softmax.
async fn run_probe_alloc(args: &Args, llm: &ResilientLLMClient, lean_bin: &Path, lp: &str) -> Result<(), String> {
    let conjuncts = task(&args.task).ok_or(format!("unknown het task {}", args.task))?;
    let k = conjuncts.len();
    let budget_b = if args.reasoner_budget_tok > 0 && args.reasoner_budget_tok < 100 { args.reasoner_budget_tok as usize } else { k + 2 }; // B = k+2 probes (binding); override via --reasoner-budget-tok (reused as probe count when <100)
    let mut rng = StdRng::seed_from_u64(args.seed);
    let t0 = Instant::now();
    let mut tape = MarketTape::new();
    let mut tokens = 0u64; let mut llm_calls = 0usize; let mut lean_calls = 0usize;
    let mut closed: BTreeSet<usize> = BTreeSet::new();
    let mut probes_left = budget_b;
    let single_strong = args.policy == "single_strong";
    let model = if single_strong { "deepseek-reasoner".to_string() } else { args.model.clone() };
    // roster: market arms use the 4 specialists; single_strong uses ONE generalist reasoner.
    let agents: Vec<Option<&str>> = if single_strong { vec![None] } else { FAMILIES.iter().map(|f| Some(*f)).collect() };
    for c in 0..k { tape.record(&MarketEvent::MarketOpen { claim: c, claim_type: conjuncts[c].1.into() }); }
    let mut wallets = vec![WALLET_BUDGET_MICRO; agents.len().max(1)];
    let mut realized_pnl = vec![0i64; agents.len().max(1)];

    // ── SINGLE_STRONG: one reasoner sweeps open conjuncts, each attempt = one probe ──
    if single_strong {
        let mut ci = 0usize;
        while probes_left > 0 && closed.len() < k {
            let open: Vec<usize> = (0..k).filter(|i| !closed.contains(i)).collect();
            if open.is_empty() { break; }
            let target = open[ci % open.len()]; ci += 1;
            let prompt = format!("Prove this Lean 4 (Mathlib) goal (context `(n:ℕ)(a b:ℝ)`):\n{} := by\nOutput ONLY JSON {{\"tactic\":\"<proof body>\"}}. No sorry.", conjuncts[target].0);
            probes_left -= 1;
            let resp = match llm.generate(&GenerateRequest { model: model.clone(), messages: vec![Message { role: "user".into(), content: prompt }], temperature: Some(0.3), max_tokens: Some(500) }).await {
                Ok(r) => { llm_calls += 1; tokens += (r.prompt_tokens+r.completion_tokens) as u64; tape.record(&MarketEvent::LlmCall { model: model.clone(), prompt_tokens: r.prompt_tokens as u64, completion_tokens: r.completion_tokens as u64 }); r }
                Err(_) => continue,
            };
            if let Some(tac) = extract(&resp.content, "tactic").and_then(|v| v.as_str().map(String::from)) {
                lean_calls += 1;
                if verify_conjunct(conjuncts[target].0, &tac, lean_bin, &args.mathlib_dir, lp, &format!("{}_ss_{}", args.seed, target)) {
                    closed.insert(target);
                    tape.record(&MarketEvent::Verify { claim: target, verdict: true, reject_class: "none".into() });
                }
            }
        }
        return finish_probe(args, &tape, &closed, k, budget_b, &realized_pnl, llm_calls, lean_calls, tokens, t0.elapsed().as_secs_f64());
    }

    // ── MARKET FAMILY: round-by-round sealed-bid allocation of the shared probe budget ──
    let mut round = 0usize;
    while probes_left > 0 && closed.len() < k {
        let open: Vec<usize> = (0..k).filter(|i| !closed.contains(i)).collect();
        if open.is_empty() { break; }
        // BID PHASE: each specialist places a loss-bearing YES bid on each open conjunct it thinks is its.
        // Bid size = stake_from_confidence on a cheap pre-Lean self-judgment of family fit (NO Lean call).
        // bids[c] accumulates YES capital across specialists; bidder_of[c] = the top bidder for that conjunct.
        let mut yes = vec![0i64; k]; let mut bidder_of: Vec<Option<(usize, u64)>> = vec![None; k]; // (agent, conf)
        for (ai, fam) in agents.iter().enumerate() {
            for &c in &open {
                // a specialist bids on a conjunct iff it judges the family fits — flatbid overrides to constant.
                let (fit, conf) = specialist_fit(fam.unwrap_or(""), conjuncts[c].1);
                if !fit { continue; }
                let stake = if args.policy == "flatbid" { (MIN_STAKE_MICRO + MAX_STAKE_MICRO) / 2 } else { stake_from_confidence(conf, wallets[ai]) };
                if stake < MIN_STAKE_MICRO { continue; }
                yes[c] += stake;
                if bidder_of[c].map_or(true, |(_, pc)| conf > pc) { bidder_of[c] = Some((ai, conf)); }
                tape.record(&MarketEvent::Invest { agent: ai, claim: c, side: "YES".into(), amount_micro: stake, model_hash: short_hash(fam.unwrap_or("")), confidence: conf });
            }
        }
        // PRICE: per-mille from the YES bids (no NO side in allocation; price = bid intensity / max).
        let max_yes = yes.iter().copied().max().unwrap_or(1).max(1);
        let price_pm: Vec<i64> = (0..k).map(|c| if closed.contains(&c) { -1 } else { (yes[c] * 1000) / max_yes }).collect();
        // ROUTE: pick which open+funded conjunct gets this probe, per the ARM's policy.
        let funded: Vec<usize> = open.iter().copied().filter(|&c| bidder_of[c].is_some()).collect();
        let sel = match args.policy.as_str() {
            "market" | "flatbid" => funded.iter().copied().max_by_key(|&c| price_pm[c]),
            "shuffled" => { let mut p = price_pm.clone(); for i in (1..k).rev() { let j = rng.gen_range(0..=i); p.swap(i,j); } funded.iter().copied().max_by_key(|&c| p[c]) }
            "uniform" => if funded.is_empty() { open.first().copied() } else { Some(funded[rng.gen_range(0..funded.len())]) },
            "roundrobin" => open.get(round % open.len()).copied(), // blind sweep, ignores bids
            _ => funded.first().copied(),
        };
        let Some(c) = sel.or_else(|| open.first().copied()) else { break };
        let frontier_h = short_hash(&format!("{:?}{:?}", open, price_pm));
        tape.record(&MarketEvent::RouteSample { policy: args.policy.clone(), frontier_hash: frontier_h, selected_claim: c });
        // the specialist that attempts = top bidder for c (market) or a round-robin agent.
        let (proposer, _conf) = match args.policy.as_str() {
            "roundrobin" | "uniform" => (round % agents.len(), 0u64),
            _ => bidder_of[c].unwrap_or((round % agents.len(), 0)),
        };
        let fam = agents[proposer];
        // SPEND ONE PROBE (consumed regardless of outcome — the binding budget).
        probes_left -= 1; round += 1;
        let role = match fam { Some(f) => format!("You are a SPECIALIST. {}", family_hint(f)), None => "You are a Lean 4 prover; use any single tactic.".into() };
        let prompt = format!("{role}\n\nProve this goal (context `(n:ℕ)(a b:ℝ)`):\n{} := by\nOutput ONLY JSON {{\"tactic\":\"<proof body>\"}}. No sorry. If not your specialty, {{\"tactic\":\"SKIP\"}}.", conjuncts[c].0);
        let resp = match llm.generate(&GenerateRequest { model: model.clone(), messages: vec![Message { role: "user".into(), content: prompt }], temperature: Some(0.3), max_tokens: Some(400) }).await {
            Ok(r) => { llm_calls += 1; tokens += (r.prompt_tokens+r.completion_tokens) as u64; tape.record(&MarketEvent::LlmCall { model: model.clone(), prompt_tokens: r.prompt_tokens as u64, completion_tokens: r.completion_tokens as u64 }); r }
            Err(_) => continue,
        };
        let tac = match extract(&resp.content, "tactic").and_then(|v| v.as_str().map(String::from)) { Some(t) if !t.trim().is_empty() && t.to_uppercase() != "SKIP" => t.trim().to_string(), _ => continue };
        lean_calls += 1;
        let ok = verify_conjunct(conjuncts[c].0, &tac, lean_bin, &args.mathlib_dir, lp, &format!("{}_{}_{}_{}", args.task, args.policy, args.seed, c));
        tape.record(&MarketEvent::Verify { claim: c, verdict: ok, reject_class: if ok { "none".into() } else { "lean_rejected".into() } });
        if ok {
            closed.insert(c);
            realized_pnl[proposer] += yes[c]; wallets[proposer] += yes[c];
            tape.record(&MarketEvent::Resolve { claim: c, outcome: "YES".into() });
        } else {
            realized_pnl[proposer] -= stake_from_confidence(50, WALLET_BUDGET_MICRO);
            tape.record(&MarketEvent::Resolve { claim: c, outcome: "NO".into() });
        }
    }
    finish_probe(args, &tape, &closed, k, budget_b, &realized_pnl, llm_calls, lean_calls, tokens, t0.elapsed().as_secs_f64())
}

/// Cheap pre-Lean specialist self-judgment: does my tactic family fit this conjunct's family? Returns
/// (will_bid, confidence). A real specialist mostly bids on-family with HIGH confidence, but — like a real
/// LLM agent — sometimes OVER-CLAIMS an adjacent family at LOWER confidence (omega↔nlinarith both touch
/// inequalities; ring↔induction both touch algebra). This creates genuine CONTENTION (multiple bidders per
/// conjunct, varying confidence) so the price has a real allocation decision to make — and a wrong-family
/// bid that wins a probe and FAILS Lean wastes a scarce probe, which is exactly what price must avoid.
/// The confidence is the predictable Δ: on-family bids are reliably higher, so a calibrated price routes
/// the probe to the agent most likely to actually close it. (This is the legible specialist↔subtask match
/// the literature says is the ONE regime where price beats random — Selection Bottleneck high-s, high-Δ.)
fn specialist_fit(my_family: &str, conjunct_family: &str) -> (bool, u64) {
    if my_family == conjunct_family { return (true, 90); }     // on-family → high-confidence bid
    // adjacency: an agent may OVER-CLAIM a related family at low confidence (real miscalibration).
    let adjacent = matches!((my_family, conjunct_family),
        ("omega", "nlinarith") | ("nlinarith", "omega") |      // both inequality-ish
        ("ring", "induction") | ("induction", "ring"));         // both algebraic-identity-ish
    if adjacent { (true, 35) } else { (false, 0) }              // low-confidence over-claim, or decline
}

#[allow(clippy::too_many_arguments)]
fn finish_probe(args: &Args, tape: &MarketTape, closed: &BTreeSet<usize>, k: usize, budget_b: usize, pnl: &[i64], llm_calls: usize, lean_calls: usize, tokens: u64, wall: f64) -> Result<(), String> {
    let chain_ok = tape.verify_chain();
    let manifest = serde_json::json!({
        "schema": "lean_probe_alloc.v1", "task": args.task, "policy": args.policy,
        "k": k, "closed": closed.len(), "solved": closed.len() == k, "budget_probes": budget_b,
        "closed_claims": closed.iter().collect::<Vec<_>>(), "seed": args.seed,
        "realized_pnl_micro": pnl, "llm_calls": llm_calls, "lean_calls": lean_calls, "tokens": tokens,
        "tape_chain_ok": chain_ok, "tape_events": tape.lines.len(), "wall_s": wall,
    });
    let _ = std::fs::write(&args.out, serde_json::to_string_pretty(&manifest).unwrap());
    if let Some(tp) = &args.tape_out { let _ = std::fs::write(tp, tape.lines.join("\n")); }
    println!("probe[{}] task={} closed={}/{} budget={} llm={} lean={} chain_ok={} wall={:.1}s",
        args.policy, args.task, closed.len(), k, budget_b, llm_calls, lean_calls, chain_ok, wall);
    Ok(())
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
    let mut reasoner_conf = vec![0i64; residual.len()]; // the SINGLE reasoner-bettor's raw p_success (skeptic-rerank)
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
            // skeptic-rerank signal: the reasoner bettor's raw success belief (signed by verdict), NO capital.
            if bettor_m.contains("reasoner") { reasoner_conf[ri] = if verdict == "YES" { conf as i64 } else { -(conf as i64) }; }
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
        // skeptic-rerank: order by the SINGLE reasoner-bettor's raw p_success (a strong critic, NO capital,
        // NO market). Rules out "a strong judge helped, not the market" — market must beat THIS too.
        "skeptic_rerank" => order.sort_by(|&a, &b| reasoner_conf[b].cmp(&reasoner_conf[a])),
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
    // PRIMARY metric: banked_at_B = axiom-clean theorems banked at the FIXED reasoner-token budget B (== pass
    // rate at equal compute; least gameable). All arms share the same B, so banked.len() IS banked@B.
    let banked_at_b = banked.len();
    // SECONDARY (honest cost context, NEVER the gate): Cost-of-Pass = micro-USD per Lean-verified solve
    // (arXiv 2504.13359), and the old per-rktok ratio (rewards under-spend → demoted to secondary).
    let cost_of_pass_micro_usd = if banked_at_b > 0 { micro_usd / banked_at_b as i64 } else { i64::MAX };
    let per_rk = if reasoner_ct > 0 { (banked_at_b as f64) / (reasoner_ct as f64 / 1000.0) } else { 0.0 };
    let manifest = serde_json::json!({
        "schema": "lean_hayek_alloc.v2", "policy": args.policy, "pool_size": pool.len(),
        "banked_at_B": banked_at_b, "banked_ids": banked.iter().collect::<Vec<_>>(), "residual": residual,
        "reasoner_completion_tokens": reasoner_ct, "chat_completion_tokens": chat_ct,
        "reasoner_budget_tok": args.reasoner_budget_tok, "micro_usd": micro_usd,
        "cost_of_pass_micro_usd": cost_of_pass_micro_usd, "banked_per_reasoner_ktok": per_rk, "seed": args.seed,
        "llm_calls": llm_calls, "lean_calls": lean_calls, "tape_chain_ok": chain_ok, "tape_events": tape.lines.len(), "wall_s": wall,
    });
    let _ = std::fs::write(&args.out, serde_json::to_string_pretty(&manifest).unwrap());
    if let Some(tp) = &args.tape_out { let _ = std::fs::write(tp, tape.lines.join("\n")); }
    println!(
        "alloc[{}] banked_at_B={}/{} residual={} reasoner_tok={}/{} cost_of_pass_uusd={} micro_usd={} chain_ok={} wall={:.1}s",
        args.policy, banked_at_b, pool.len(), residual, reasoner_ct, args.reasoner_budget_tok, cost_of_pass_micro_usd, micro_usd, chain_ok, wall
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
    // conf_signed[p] = Σ over bettors of (+conf if YES, −conf if NO): the CONFIDENCE-WEIGHTED-AVERAGE
    // signal (the literature's STRONG baseline — Brier/confidence weighting captures the aggregation
    // gains; the question is whether CAPITAL-AT-RISK price beats THIS, not just random). No capital here.
    let mut conf_signed = vec![0i64; m];
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
            if verdict == "YES" { wallets[bi] -= stake; yes[pi] += stake; conf_signed[pi] += conf as i64; tape.record(&MarketEvent::Invest { agent: bi, claim: pi, side: "YES".into(), amount_micro: stake, model_hash: model_h, confidence: conf }); }
            else if verdict == "NO" { conf_signed[pi] -= conf as i64; if args.policy != "realprice_no_no" { wallets[bi] -= stake; no[pi] += stake; tape.record(&MarketEvent::Invest { agent: bi, claim: pi, side: "NO".into(), amount_micro: stake, model_hash: model_h, confidence: conf }); } }
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
        // route over REMAINING proofs by the arm's policy.
        // conf_avg = the STRONG baseline: pick the highest confidence-weighted-average proof (no capital).
        // market(realprice) must beat THIS, not just random, to prove capital-at-risk adds value.
        let sel_idx = if args.policy == "conf_avg" {
            remaining.iter().copied().max_by_key(|&p| conf_signed[p])
        } else {
            let prices_now: Vec<i64> = price_pm.clone();
            route(&args.policy, &remaining, &prices_now, &attempts, args.temp, &mut rng)
        };
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

    // REPEATED mode: persistent-wealth market over a stream of verify-questions — the ONE design the
    // literature says capital-at-risk can win (no-regret dynamic reweighting vs static averaging).
    if args.task == "repeated" {
        return run_repeated(&args, &llm, &lean_bin, &lp).await;
    }
    // REPUTATION mode: price as no-regret COMPETENCE allocator under heterogeneity + a SPAM over-claimer —
    // the regime TuringOS's design is FOR (Sybil/spam-resistance + dynamic reweighting, performance-causal).
    if args.task == "reputation" {
        return run_reputation(&args, &llm, &lean_bin, &lp).await;
    }
    // SKILLSWEEP mode: real-data reproduction of the architect's autonomous-vs-softmax A/B crossover test.
    if args.task == "skillsweep" {
        return run_skillsweep(&args, &llm, &lean_bin, &lp).await;
    }
    // PROBE-ALLOC mode: price allocates a scarce SHARED PROBE budget over complementary specialists
    // (het6/het8 conjunctions). The decisive price-causality test (research + strategy converged here).
    if args.task.starts_with("het") {
        return run_probe_alloc(&args, &llm, &lean_bin, &lp).await;
    }
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

#[cfg(test)]
mod axiom_gate_tests {
    //! Soundness checks for the `#print axioms` gate. The PURE tests (parser + namer) always run. The
    //! prover test requires the LIVE Lean + Mathlib toolchain the binary uses — set `MATHLIB_DIR` to a
    //! Mathlib checkout to run it:
    //!   `MATHLIB_DIR=/path/to/mathlib4 cargo test --bin lean_hayek_market axiom_gate`
    //! Absent that, it SKIPS LOUDLY rather than silently passing — it cannot run without a real prover.
    use super::*;

    fn set(xs: &[&str]) -> BTreeSet<String> { xs.iter().map(|s| s.to_string()).collect() }

    fn toolchain() -> Option<(PathBuf, PathBuf, String)> {
        let mathlib = PathBuf::from(std::env::var("MATHLIB_DIR").ok()?);
        if !mathlib.exists() { return None; }
        let lp = lean_path(&mathlib)?;
        Some((default_lean_bin(), mathlib, lp))
    }

    // Pin the exact `#print axioms` line shapes + allowlist semantics (no toolchain needed).
    #[test]
    fn parse_and_allowlist_semantics() {
        assert_eq!(parse_axiom_set("'_t' depends on axioms: [propext]"), Some(set(&["propext"])));
        assert_eq!(parse_axiom_set("'t' depends on axioms: [propext, Classical.choice, Quot.sound]"),
            Some(set(&["propext", "Classical.choice", "Quot.sound"])));
        assert_eq!(parse_axiom_set("'t' depends on axioms: [sorryAx]"), Some(set(&["sorryAx"])));
        assert_eq!(parse_axiom_set("'t' does not depend on any axioms"), Some(BTreeSet::new()));
        assert_eq!(parse_axiom_set("no print-axioms line here"), None);

        assert!(axiom_set_clean(&parse_axiom_set("'t' depends on axioms: [propext, Quot.sound]")));
        assert!(axiom_set_clean(&parse_axiom_set("'t' does not depend on any axioms")));
        assert!(!axiom_set_clean(&parse_axiom_set("'t' depends on axioms: [sorryAx]")));
        assert!(!axiom_set_clean(&parse_axiom_set("'t' depends on axioms: [Lean.ofReduceBool, Lean.trustCompiler]")));
        assert!(!axiom_set_clean(&None)); // fail-closed: never certify without reading the axiom set
    }

    // `example` preambles must become a named theorem so `#print axioms <name>` resolves; already-named
    // decls are reused verbatim; the LAST decl is the target even behind a helper decl.
    #[test]
    fn ensure_named_renames_example_keeps_named_picks_last() {
        let (src, name) = ensure_named("import Mathlib\n\nexample (a : ℤ) : a = a := by", "1_rp_free_3_2");
        assert_eq!(name, "pool_1_rp_free_3_2");
        assert!(src.contains("theorem pool_1_rp_free_3_2 (a : ℤ) : a = a := by"), "got: {src}");
        assert!(!src.contains("example"));

        let (src2, name2) = ensure_named("import Mathlib\n\ntheorem foo (n : ℕ) : n = n := by", "x");
        assert_eq!(name2, "foo");
        assert!(src2.contains("theorem foo (n : ℕ) : n = n := by")); // unchanged

        let (_, name3) = ensure_named("import Mathlib\ntheorem helper : True := by trivial\ntheorem tgt : True := by", "y");
        assert_eq!(name3, "tgt");
    }

    // THE soundness gate against the real prover: the axiom gate catches what the string ban cannot.
    #[test]
    fn axiom_gate_beats_string_ban() {
        let (lean, mathlib, lp) = match toolchain() {
            Some(t) => t,
            None => { eprintln!("SKIP axiom_gate_beats_string_ban: set MATHLIB_DIR to a Lean+Mathlib checkout to run it"); return; }
        };

        // (A) CLEAN: axiom set ⊆ allowlist (propext) → PASSES.
        let (ok, cls) = verify_pool_err(
            "import Mathlib\n\ntheorem t_clean (n : ℕ) : n + 0 = n := by", "simp",
            &lean, &mathlib, &lp, "ut_clean");
        assert!(ok, "a propext/Quot.sound proof must pass the axiom gate (class={cls})");
        assert_eq!(cls, "none");

        // (B) SORRY smuggled via a PREAMBLE helper: the body is `exact t_helper`, so the body string-ban
        // (which scans the body only) does NOT fire. Rejection must come from the AXIOM GATE (sorryAx).
        let (ok, cls) = verify_pool_err(
            "import Mathlib\n\ntheorem t_helper : (2 : Nat) = 3 := by sorry\ntheorem t_sm : (2 : Nat) = 3 := by",
            "exact t_helper", &lean, &mathlib, &lp, "ut_sorry");
        assert!(!ok, "a sorry-backed proof must be rejected");
        assert_ne!(cls, "bypass", "must NOT be caught by the body string-ban — that would miss the point");
        assert_eq!(cls, "sorry_axiom", "must be rejected specifically by the axiom gate (sorryAx)");

        // (C) NONSTANDARD AXIOM: a hand-declared axiom compiles with NO error and NO sorry, so the OLD
        // exit-success + string-ban gate would WRONGLY accept it. Only the allowlist rejects it.
        let (ok, cls) = verify_pool_err(
            "import Mathlib\n\naxiom ut_cheat : (2 : Nat) = 3\ntheorem t_ax : (2 : Nat) = 3 := by",
            "exact ut_cheat", &lean, &mathlib, &lp, "ut_axiom");
        assert!(!ok, "a hand-axiom proof must be rejected by the axiom gate");
        assert_eq!(cls, "nonstandard_axiom");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Guards the money path: every roster model bills at ITS published rate, the most-specific row
    // wins, and the arithmetic stays integer. If a future edit reorders MODEL_RATES so a catch-all
    // shadows a specific row (e.g. bare "deepseek" before "deepseek-ai/DeepSeek-V3.2"), this fails.
    #[test]
    fn call_micro_usd_bills_each_model_at_its_true_rate() {
        // strong SiliconFlow models — NOT the deepseek-chat proxy (the bug this table fixes).
        assert_eq!(call_micro_usd("deepseek-ai/DeepSeek-V3.2", 1_000_000, 0), 270_000);
        assert_eq!(call_micro_usd("deepseek-ai/DeepSeek-V3.2", 0, 1_000_000), 410_000);
        assert_eq!(call_micro_usd("Qwen/Qwen3-32B", 1_000_000, 1_000_000), 140_000 + 570_000);
        assert_eq!(call_micro_usd("Qwen/Qwen2.5-72B-Instruct", 0, 1_000_000), 590_000);
        // ordering guard: "deepseek-reasoner" contains both "reasoner" and "deepseek"; reasoner row wins.
        assert_eq!(call_micro_usd("deepseek-reasoner", 0, 1_000_000), 2_190_000);
        assert_eq!(call_micro_usd("deepseek-chat", 0, 1_000_000), 1_100_000);
        // ordering guard (OBL-012 / v4 workhorse): "deepseek-v4-pro" CONTAINS the bare "deepseek"
        // catch-all substring — its specific row MUST win, or the flagship under-bills at $0.27/$1.10.
        assert_eq!(call_micro_usd("deepseek-v4-pro", 1_000_000, 0), 435_000);
        assert_eq!(call_micro_usd("deepseek-v4-pro", 0, 1_000_000), 870_000);
        assert_eq!(call_micro_usd("deepseek-v4-flash", 1_000_000, 0), 140_000);
        assert_eq!(call_micro_usd("deepseek-v4-flash", 0, 1_000_000), 280_000);
        // unknown id → labeled fallback, never a strong-model rate by accident.
        assert_eq!(
            call_micro_usd("Qwen/Qwen3-Coder-30B-A3B-Instruct", 0, 1_000_000),
            FALLBACK_OUT_UPMT
        );
        // integer-only, sub-1M scaling: 500k output tok of V3.2 = 410_000 * 500_000 / 1_000_000.
        assert_eq!(call_micro_usd("deepseek-ai/DeepSeek-V3.2", 0, 500_000), 205_000);
    }
}
