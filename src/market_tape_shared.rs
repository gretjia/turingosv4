//! MarketTape-lite — bin-local shared substrate (TP-0A.1, behavior-preserving extraction).
//!
//! Extracted verbatim from `src/bin/lean_hayek_market.rs:107-170` so that the producer bin
//! (`lean_hayek_market`) AND the replay verifier (`verify_market_tape`, TP-0A.6) link the IDENTICAL
//! event schema + hash-chain logic — the tape a run emits is replayable by a standalone verifier to
//! integer/byte equality with the run manifest (the TP-0A PPUT-auditability gate).
//!
//! Deliberately lives in `src/` but is NOT declared in `lib.rs` (adding a `mod` to lib.rs is a
//! trust-root/constitution touch per project memory). It is pulled into each consumer via
//! `#[path = "../market_tape_shared.rs"] mod market_tape_shared;` — so it is neither a cargo binary
//! (those are only `src/bin/*.rs`) nor part of the library surface. Each consumer compiles its own
//! copy of these pure types/impls; identical source ⇒ identical tape format by construction.
//!
//! Price is DERIVED from `Invest` events; `node.score` is never authoritative (constitution Art.0.2).

use sha2::{Digest, Sha256};

#[derive(Clone)]
pub enum MarketEvent {
    MarketOpen { claim: usize, claim_type: String },
    Invest { agent: usize, claim: usize, side: String, amount_micro: i64, model_hash: String, confidence: u64 },
    Proposal { agent: usize, claim: usize, output_hash: String },
    LlmCall { model: String, prompt_tokens: u64, completion_tokens: u64 },
    Verify { claim: usize, verdict: bool, reject_class: String },
    RouteSample { policy: String, frontier_hash: String, selected_claim: usize },
    Resolve { claim: usize, outcome: String },
    /// MUST be the first event on every TP-0A replay-gated tape: pins the run identity + provenance so the
    /// standalone verifier binds the manifest to a specific (seed, arm, roster, budget, axiom-whitelist, git HEAD).
    GenesisPin { run_id: String, seed: u64, policy: String, model_roster: Vec<String>, budget_b: u64, axiom_whitelist: Vec<String>, head_commit_sha: String },
}

pub struct MarketTape {
    /// `pub` because the producer bin writes `tape.lines.join("\n")` to the `--tape-out` file
    /// (lean_hayek_market.rs:581/787). The replay verifier reads the file back line-by-line.
    pub lines: Vec<String>,
    prev_hash: String,
}
impl MarketTape {
    pub fn new() -> Self { MarketTape { lines: Vec::new(), prev_hash: "genesis".into() } }
    fn append(&mut self, kind: &str, body: serde_json::Value) {
        let payload = serde_json::json!({ "kind": kind, "prev": self.prev_hash, "body": body });
        let s = serde_json::to_string(&payload).unwrap();
        let mut h = Sha256::new();
        h.update(s.as_bytes());
        self.prev_hash = format!("{:x}", h.finalize());
        self.lines.push(s);
    }
    pub fn record(&mut self, e: &MarketEvent) {
        match e {
            MarketEvent::MarketOpen { claim, claim_type } => self.append("MarketOpen", serde_json::json!({"claim":claim,"claim_type":claim_type})),
            MarketEvent::Invest { agent, claim, side, amount_micro, model_hash, confidence } => self.append("Invest", serde_json::json!({"agent":agent,"claim":claim,"side":side,"amount_micro":amount_micro,"model_hash":model_hash,"confidence":confidence})),
            MarketEvent::Proposal { agent, claim, output_hash } => self.append("Proposal", serde_json::json!({"agent":agent,"claim":claim,"output_hash":output_hash})),
            MarketEvent::LlmCall { model, prompt_tokens, completion_tokens } => self.append("LLMCall", serde_json::json!({"model":model,"prompt_tokens":prompt_tokens,"completion_tokens":completion_tokens})),
            MarketEvent::Verify { claim, verdict, reject_class } => self.append("Verify", serde_json::json!({"claim":claim,"verdict":verdict,"reject_class":reject_class})),
            MarketEvent::RouteSample { policy, frontier_hash, selected_claim } => self.append("RouteSample", serde_json::json!({"policy":policy,"frontier_hash":frontier_hash,"selected_claim":selected_claim})),
            MarketEvent::Resolve { claim, outcome } => self.append("Resolve", serde_json::json!({"claim":claim,"outcome":outcome})),
            MarketEvent::GenesisPin { run_id, seed, policy, model_roster, budget_b, axiom_whitelist, head_commit_sha } => self.append("GenesisPin", serde_json::json!({"run_id":run_id,"seed":seed,"policy":policy,"model_roster":model_roster,"budget_b":budget_b,"axiom_whitelist":axiom_whitelist,"head_commit_sha":head_commit_sha})),
        }
    }
    /// Verify the append-only prev_hash chain (replayability gate, ATOM 5-lite).
    pub fn verify_chain(&self) -> bool {
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
    pub fn derive_pools(&self, k: usize) -> Vec<(i64, i64)> {
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

// ── Per-model cost (moved here in TP-0A.3 so the producer bin AND the standalone verify_market_tape
// verifier link the IDENTICAL rate table — derive_cost recomputes micro_usd from the tape's LLMCall events
// alone and NEVER reads the manifest). Integer-only (constitution §12). MODEL_RATES ordered
// most-specific-first; the bare "deepseek" catch-all MUST stay last (it is a substring of deepseek-v4-pro,
// the OBL-012 under-bill bug). Price provenance is documented at the original site in lean_hayek_market.rs.
pub const MODEL_RATES: &[(&str, i64, i64)] = &[
    ("deepseek-ai/DeepSeek-V3.2", 270_000, 410_000),
    ("Qwen/Qwen3-32B", 140_000, 570_000),
    ("Qwen/Qwen2.5-72B-Instruct", 590_000, 590_000),
    ("deepseek-v4-pro", 435_000, 870_000),
    ("deepseek-v4-flash", 140_000, 280_000),
    ("reasoner", 550_000, 2_190_000),
    ("deepseek", 270_000, 1_100_000),
];
pub const FALLBACK_IN_UPMT: i64 = 270_000;
pub const FALLBACK_OUT_UPMT: i64 = 1_100_000;

/// integer micro-USD for one LLM call. First MODEL_RATES substring match wins (most-specific-first); else
/// FALLBACK. Role-independent (cost is a function of model + tokens only).
pub fn call_micro_usd(model: &str, prompt_tok: u64, completion_tok: u64) -> i64 {
    let mut rate = (FALLBACK_IN_UPMT, FALLBACK_OUT_UPMT);
    for &(id, in_upmt, out_upmt) in MODEL_RATES {
        if model.contains(id) { rate = (in_upmt, out_upmt); break; }
    }
    (prompt_tok as i64 * rate.0 + completion_tok as i64 * rate.1) / 1_000_000
}

// ── Tape replay derivations (TP-0A.3): reconstruct the manifest's headline integers from the frozen tape
// LINES ALONE (the same JSONL the producer writes via --tape-out), so a standalone verifier proves the PPUT
// numbers are auditable, not read-back. NOTE (honest scope): reasoner-vs-chat token SPLIT is role-dependent
// and the current LLMCall schema records no role, so only the TOTAL completion is tape-derivable here; the
// split needs a `role` marker on LLMCall (deferred to when the T2 budget-parity gate must audit from tape).
fn parsed(lines: &[String]) -> impl Iterator<Item = serde_json::Value> + '_ {
    lines.iter().filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
}
/// Verify the prev_hash chain over RAW tape lines (the standalone verifier has only the file, no MarketTape).
/// Identical logic to MarketTape::verify_chain; a one-byte tamper anywhere breaks the chain → false.
pub fn verify_chain_lines(lines: &[String]) -> bool {
    let mut prev = "genesis".to_string();
    for line in lines {
        let v: serde_json::Value = match serde_json::from_str(line) { Ok(v) => v, Err(_) => return false };
        if v["prev"].as_str() != Some(&prev) { return false; }
        let mut h = Sha256::new(); h.update(line.as_bytes());
        prev = format!("{:x}", h.finalize());
    }
    true
}
/// banked@B = count of DISTINCT claims that ever verified clean (a Verify{verdict:true}). A banked theorem
/// IS a clean verify; run_alloc banks in the INITIAL path (Verify{true}, no Resolve) AND the reasoner-repair
/// path (Verify{true} + Resolve{YES}), so counting Verify-true distinct claims is the tape-faithful banked
/// count (counting Resolve-YES alone undercounts — caught by the TP-0A real-run replay gate 2026-06-01).
pub fn derive_banked(lines: &[String]) -> usize {
    let mut banked = std::collections::BTreeSet::new();
    for v in parsed(lines) {
        if v["kind"] == "Verify" && v["body"]["verdict"] == true {
            if let Some(c) = v["body"]["claim"].as_u64() { banked.insert(c); }
        }
    }
    banked.len()
}
/// total micro-USD recomputed from every LLMCall via the SHARED MODEL_RATES (never reads the manifest).
pub fn derive_cost(lines: &[String]) -> i64 {
    parsed(lines).filter(|v| v["kind"] == "LLMCall")
        .map(|v| call_micro_usd(v["body"]["model"].as_str().unwrap_or(""),
            v["body"]["prompt_tokens"].as_u64().unwrap_or(0), v["body"]["completion_tokens"].as_u64().unwrap_or(0)))
        .sum()
}
/// cost-of-pass = total micro-USD / banked (i64::MAX if nothing banked) — recomputed, matches finish_alloc.
pub fn derive_cost_of_pass(lines: &[String]) -> i64 {
    let b = derive_banked(lines);
    if b > 0 { derive_cost(lines) / b as i64 } else { i64::MAX }
}
/// total completion tokens across all LLMCall events (= reasoner_completion + chat_completion in the manifest).
pub fn derive_total_completion(lines: &[String]) -> u64 {
    parsed(lines).filter(|v| v["kind"] == "LLMCall").map(|v| v["body"]["completion_tokens"].as_u64().unwrap_or(0)).sum()
}
/// number of LLMCall events (= manifest llm_calls).
pub fn derive_llm_calls(lines: &[String]) -> usize { parsed(lines).filter(|v| v["kind"] == "LLMCall").count() }
/// failed branches on tape = Verify{verdict:false} — a parse-fail/Lean-rejected attempt is auditable.
pub fn derive_failures(lines: &[String]) -> usize {
    parsed(lines).filter(|v| v["kind"] == "Verify" && v["body"]["verdict"] == false).count()
}

// ── GenesisPin (TP-0A.2): mandatory-first provenance record + verifier helpers ──
/// best-effort 40-hex git HEAD at run start — the MINIMAL provenance surrogate (NOT the real Art.0.4 Q_t/
/// HEAD_t rebuild, which is out of scope). Falls back to "unknown" if git is unavailable.
pub fn head_commit_sha() -> String {
    std::process::Command::new("git").args(["rev-parse", "HEAD"]).output().ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| s.len() == 40 && s.chars().all(|c| c.is_ascii_hexdigit()))
        .unwrap_or_else(|| "unknown".into())
}
/// the replay-gate invariant: the FIRST tape line must be a GenesisPin.
pub fn first_is_genesis(lines: &[String]) -> bool {
    lines.first().and_then(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .map(|v| v["kind"] == "GenesisPin").unwrap_or(false)
}
/// parse the GenesisPin body (the run's pinned identity), if present as the first event.
pub fn derive_genesis(lines: &[String]) -> Option<serde_json::Value> {
    if !first_is_genesis(lines) { return None; }
    serde_json::from_str::<serde_json::Value>(&lines[0]).ok().map(|v| v["body"].clone())
}
