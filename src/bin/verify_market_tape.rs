//! verify_market_tape — standalone replay verifier for the MarketTape-lite PPUT gate (TP-0A.6).
//!
//! Reads a frozen `--tape` (the JSONL the producer wrote via `--tape-out`) + the run `--manifest` (.json),
//! reconstructs the headline integers FROM THE TAPE ALONE via the SHARED `market_tape_shared::derive_*`
//! (identical MODEL_RATES, so cost is recomputed not read-back), and asserts integer/byte equality with the
//! manifest. This is the TP-0A acceptance gate: a market arm whose PPUT numbers cannot be reconstructed from
//! a frozen tape is excluded from headlines. Read-only — never mutates the run. Exit 0 iff replay-clean.
//!
//! Schema-aware: the pinned reference arm is `lean_hayek_alloc.v2` (banked@B + cost_of_pass). Other schemas
//! get the universal checks (chain + GenesisPin-first); their bespoke fields (e.g. het4 realized_pnl) land in
//! later atoms. Emits `replay_report.json` with every check + the derived-vs-manifest values.

#[path = "../market_tape_shared.rs"]
mod market_tape_shared;
use market_tape_shared as mt;

fn arg(flag: &str) -> Option<String> {
    let a: Vec<String> = std::env::args().collect();
    a.iter().position(|x| x == flag).and_then(|i| a.get(i + 1).cloned())
}

fn main() {
    let tape_path = arg("--tape").expect("usage: verify_market_tape --tape <file> --manifest <file> [--out <file>]");
    let manifest_path = arg("--manifest").expect("--manifest <file> required");
    let out_path = arg("--out").unwrap_or_else(|| "/tmp/replay_report.json".into());

    let lines: Vec<String> = std::fs::read_to_string(&tape_path)
        .unwrap_or_else(|e| panic!("read tape {tape_path}: {e}"))
        .lines().filter(|l| !l.trim().is_empty()).map(|s| s.to_string()).collect();
    let manifest: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&manifest_path).unwrap_or_else(|e| panic!("read manifest {manifest_path}: {e}")))
        .unwrap_or_else(|e| panic!("parse manifest: {e}"));

    let schema = manifest["schema"].as_str().unwrap_or("").to_string();
    let mut report = serde_json::Map::new();
    let mut ok = true;
    let mut put = |k: &str, v: bool, rep: &mut serde_json::Map<String, serde_json::Value>, ok: &mut bool| {
        rep.insert(k.into(), serde_json::json!(v));
        if !v { *ok = false; }
    };

    // ── universal: hash chain intact + GenesisPin is the first event ──
    let chain_ok = mt::verify_chain_lines(&lines);
    put("chain_ok", chain_ok, &mut report, &mut ok);
    let genesis_first = mt::first_is_genesis(&lines);
    put("genesis_first", genesis_first, &mut report, &mut ok);
    if let Some(g) = mt::derive_genesis(&lines) {
        let sha = g["head_commit_sha"].as_str().unwrap_or("");
        let sha_ok = sha == "unknown" || (sha.len() == 40 && sha.chars().all(|c| c.is_ascii_hexdigit()));
        put("genesis_head_sha_wellformed", sha_ok, &mut report, &mut ok);
        report.insert("genesis".into(), g);
    } else {
        put("genesis_head_sha_wellformed", false, &mut report, &mut ok);
    }
    report.insert("schema".into(), serde_json::json!(schema));

    // ── schema-aware reconstruction ──
    if schema == "lean_hayek_alloc.v2" {
        let d_banked = mt::derive_banked(&lines) as i64;
        let m_banked = manifest["banked_at_B"].as_i64().unwrap_or(-1);
        put("banked_match", d_banked == m_banked, &mut report, &mut ok);

        let d_cost = mt::derive_cost(&lines);
        let m_cost = manifest["micro_usd"].as_i64().unwrap_or(-1);
        put("cost_match", d_cost == m_cost, &mut report, &mut ok);

        let d_cop = mt::derive_cost_of_pass(&lines);
        let m_cop = manifest["cost_of_pass_micro_usd"].as_i64().unwrap_or(-1);
        put("cost_of_pass_match", d_cop == m_cop, &mut report, &mut ok);

        let d_tok = mt::derive_total_completion(&lines) as i64;
        let m_tok = manifest["reasoner_completion_tokens"].as_i64().unwrap_or(0)
            + manifest["chat_completion_tokens"].as_i64().unwrap_or(0);
        put("tokens_match", d_tok == m_tok, &mut report, &mut ok);

        let d_calls = mt::derive_llm_calls(&lines) as i64;
        let m_calls = manifest["llm_calls"].as_i64().unwrap_or(-1);
        put("llm_calls_match", d_calls == m_calls, &mut report, &mut ok);

        report.insert("derived".into(), serde_json::json!({
            "banked": d_banked, "micro_usd": d_cost, "cost_of_pass_micro_usd": d_cop,
            "total_completion_tokens": d_tok, "llm_calls": d_calls, "failures_on_tape": mt::derive_failures(&lines),
        }));
        report.insert("manifest".into(), serde_json::json!({
            "banked_at_B": m_banked, "micro_usd": m_cost, "cost_of_pass_micro_usd": m_cop,
            "reasoner_plus_chat_tokens": m_tok, "llm_calls": m_calls,
        }));
    } else {
        // unknown schema: universal checks pass through; bespoke reconstruction is a later atom.
        report.insert("note".into(), serde_json::json!(format!("schema '{schema}' has only universal checks in TP-0A")));
    }

    report.insert("replay_clean".into(), serde_json::json!(ok));
    let report_val = serde_json::Value::Object(report);
    let _ = std::fs::write(&out_path, serde_json::to_string_pretty(&report_val).unwrap());
    println!("verify_market_tape: schema={schema} replay_clean={ok} → {out_path}");
    std::process::exit(if ok { 0 } else { 1 });
}
