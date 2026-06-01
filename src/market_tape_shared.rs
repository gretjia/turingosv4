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
