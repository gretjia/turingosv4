//! TB-10 Atom 1 — Reusable preseed factory for chaintape genesis QState.
//!
//! Single source of truth for the initial `balances_t` map populated when a
//! fresh chaintape is bootstrapped. Both the evaluator binary and the new
//! `lean_market` user CLI call this factory so that whichever process
//! bootstraps the chain first produces the SAME genesis QState — ensuring
//! the user CLI and evaluator can both attach to the same on-disk chaintape
//! and observe consistent balances.
//!
//! **Constitutional gate** (Art. III.4 / P3 kill #1 — "no post-init mint"):
//! this factory is consumed ONLY at chaintape bootstrap (genesis QState
//! construction via `runtime::adapter::genesis_with_balances`). It is NOT a
//! runtime mint path. `assert_no_post_init_mint` continues to fire on every
//! subsequent typed_tx and rejects any non-genesis mint attempt.
//!
//! **Replay determinism**: the function is pure (no env reads, no clock,
//! no randomness). Two calls produce byte-identical Vec output. Past chains
//! continue to replay from their on-disk genesis_report.json regardless of
//! future edits to this factory; only fresh bootstraps consume the current
//! version.
//!
//! Per `handover/audits/CHARTER_RATIFICATION_TB_10_2026-05-02.md` §1 Q2 +
//! §2.4. Consolidates the inline literal previously at
//! `experiments/minif2f_v4/src/bin/evaluator.rs:716-731`.

use crate::economy::money::MicroCoin;
use crate::state::q_state::AgentId;

// Polymarket PR1 (2026-05-23, revised post-Codex audit 2026-05-23): TOML-driven
// genesis preseed for the `[treasury]` + `[worker_wallets]` tables landed in
// `genesis_payload.toml`. This is the second allow-listed entry point into
// `balances_t` mutation alongside `default_pput_preseed_pairs` — both are
// consumed ONLY at Q_0 construction (`fc2_no_memory_only_preseed` permits
// `src/runtime/bootstrap.rs`).
//
// Karpathy K10 fix: previously this module hand-rolled a ~170-LoC TOML
// subset parser. Replaced with the `toml` crate (in Cargo.toml). The public
// API + behavior + tests are unchanged.

/// TRACE_MATRIX FC2 Boot: TB-10 Atom 1 — sponsor + user-sponsor + 10 solver agent budgets;
/// **TB-N3 A0.5 (architect ruling 2026-05-11 amendment 6 + Q1+Q2 verdicts)**:
/// + 1 MarketMakerBudget genesis preseed entry.
///
/// The 13 entries (in stable insertion order):
///
/// 1. `tb7-7-sponsor` (10_000_000 micro = 10 Coin) — TB-7.7 D3 self-funded
///    sponsor used by evaluator's `--task-mode self|both` preseed branch
///    (`evaluator.rs:864-922`). Preserved for back-compat with TB-7+
///    smoke harness.
/// 2. `Agent_user_0` (10_000_000 micro = 10 Coin) — **TB-10 Atom 1 net-new**;
///    sponsor identity used by `lean_market post-task` subcommand.
///    `Agent_user_` prefix is the audit_dashboard §11 filter convention
///    (per ratification §2.3).
/// 3-12. `Agent_0..9` (1_000_000 micro = 1 Coin each) — solver budgets;
///    plenty for ~1000 WorkTx.stake at 1_000 each.
/// 13. `MarketMakerBudget` (5_000_000 micro = 5 Coin) — **TB-N3 A0.5 net-new
///     (2026-05-11; architect ruling Q2 + amendment 6)**: provider identity
///     used by TB-N3 A3 `tb_n3_emit_node_market_after_work_accept` to seed
///     `MarketSeedTx` + `CpmmPoolTx` against accepted WorkTx node markets.
///     Sized at 10× safety margin over the architect's recommended Phase-2
///     batch budget (architect Q1: DEFAULT_POOL_SEED = 100_000 microCoin
///     × ~5 accepted WorkTx per 9-problem batch = 500_000 microCoin draw;
///     5_000_000 micro budget = 10× headroom for stochastic batch growth).
///     No special permission semantics: identity acts as ordinary preseed
///     agent. Genesis insertion (NOT post-init mint) preserves
///     `assert_no_post_init_mint`; A3 helper signs each
///     TaskOpen/MarketSeed/CpmmPool tx via canonical agent paths.
///
/// Total preseed supply = 10_000_000 + 10_000_000 + 10 × 1_000_000 + 5_000_000
/// = 35_000_000 micro = 35 Coin.
///
/// **Why not env-driven**: env-driven preseed would break replay determinism
/// (genesis QState would depend on env at bootstrap time). The factory is
/// the deterministic substrate; specific runs that need different starting
/// balances should construct their own preseed Vec and call
/// `genesis_with_balances` directly.
pub fn default_pput_preseed_pairs() -> Vec<(AgentId, MicroCoin)> {
    let mut pairs: Vec<(AgentId, MicroCoin)> = vec![
        (
            AgentId("tb7-7-sponsor".into()),
            MicroCoin::from_micro_units(10_000_000),
        ),
        (
            AgentId("Agent_user_0".into()),
            MicroCoin::from_micro_units(10_000_000),
        ),
    ];
    for i in 0..10 {
        pairs.push((
            AgentId(format!("Agent_{i}")),
            MicroCoin::from_micro_units(1_000_000),
        ));
    }
    // TB-N3 A0.5 (architect ruling 2026-05-11 amendment 6 + Q2): the
    // MarketMakerBudget agent is the canonical provider for TB-N3 A3
    // auto-emitted node-market seed/pool transactions. 5M micro = 10×
    // headroom over Phase-2 expected draw at DEFAULT_POOL_SEED = 100k
    // micro per pool (architect Q1).
    pairs.push((
        AgentId("MarketMakerBudget".into()),
        MicroCoin::from_micro_units(5_000_000),
    ));
    pairs
}

// ─────────────────────────────────────────────────────────────────────
// Polymarket PR1 (2026-05-23) — TOML-driven Treasury + Worker preseed
// ─────────────────────────────────────────────────────────────────────

/// Error returned by [`parse_treasury_and_worker_preseed`] when the supplied
/// TOML text is missing one of the two required tables or has a malformed
/// `agent_id = ...` / `initial_balance_micro = ...` row.
///
/// TRACE_MATRIX FC2 Boot: Polymarket PR1 revision (2026-05-23).
#[derive(Debug)]
pub enum PreseedTomlError {
    MissingSection(&'static str),
    Parse(String),
}

impl std::fmt::Display for PreseedTomlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingSection(s) => write!(f, "missing [{s}] section in genesis_payload.toml"),
            Self::Parse(s) => write!(f, "genesis_payload.toml parse error: {s}"),
        }
    }
}

impl std::error::Error for PreseedTomlError {}

/// TRACE_MATRIX FC2 Boot: Polymarket PR1 (2026-05-23, revised post-audit) —
/// parse the new `[treasury]` + `[worker_wallets]` tables added to
/// `genesis_payload.toml` into `(AgentId, MicroCoin)` pairs.
///
/// Returns the **combined** preseed list (treasury first, then each worker
/// wallet in source order). Callers extend `default_pput_preseed_pairs()`
/// (or its successor) with these entries before constructing the genesis
/// QState via `runtime::adapter::genesis_with_balances`.
///
/// **Replay determinism**: pure parser (no env reads, no clock, no
/// randomness). Same TOML bytes in → byte-identical `Vec` out.
///
/// **`fc2_no_memory_only_preseed` invariant**: this function does NOT
/// mutate `economic_state_t` directly — it returns a `Vec` that the boot
/// path feeds into `genesis_with_balances`, which is the single allow-
/// listed mutation site.
///
/// Grammar accepted (full TOML — Karpathy K10 fix: was a hand-rolled subset
/// parser; now uses the `toml` crate):
///
/// ```toml
/// [treasury]
/// agent_id = "treasury"
/// initial_balance_micro = 100_000
///
/// [worker_wallets]
/// "worker-alpha" = 10_000
/// ```
pub fn parse_treasury_and_worker_preseed(
    text: &str,
) -> Result<Vec<(AgentId, MicroCoin)>, PreseedTomlError> {
    let doc: toml::Value = toml::from_str(text)
        .map_err(|e| PreseedTomlError::Parse(format!("toml: {e}")))?;

    let treasury = parse_treasury_section(&doc)?;
    let workers = parse_worker_wallets_section(&doc)?;
    let mut out = Vec::with_capacity(1 + workers.len());
    out.push(treasury);
    out.extend(workers);
    Ok(out)
}

fn parse_treasury_section(doc: &toml::Value) -> Result<(AgentId, MicroCoin), PreseedTomlError> {
    let tbl = doc
        .get("treasury")
        .ok_or(PreseedTomlError::MissingSection("treasury"))?
        .as_table()
        .ok_or_else(|| PreseedTomlError::Parse("[treasury] must be a table".to_string()))?;

    let agent_id = tbl
        .get("agent_id")
        .ok_or_else(|| PreseedTomlError::Parse("[treasury].agent_id missing".to_string()))?
        .as_str()
        .ok_or_else(|| {
            PreseedTomlError::Parse("[treasury].agent_id must be a string".to_string())
        })?
        .to_string();

    let balance_value = tbl.get("initial_balance_micro").ok_or_else(|| {
        PreseedTomlError::Parse("[treasury].initial_balance_micro missing".to_string())
    })?;
    let balance_micro = balance_value.as_integer().ok_or_else(|| {
        PreseedTomlError::Parse(format!(
            "[treasury].initial_balance_micro must be an integer (got {balance_value:?})"
        ))
    })?;

    Ok((AgentId(agent_id), MicroCoin::from_micro_units(balance_micro)))
}

fn parse_worker_wallets_section(
    doc: &toml::Value,
) -> Result<Vec<(AgentId, MicroCoin)>, PreseedTomlError> {
    let tbl = doc
        .get("worker_wallets")
        .ok_or(PreseedTomlError::MissingSection("worker_wallets"))?
        .as_table()
        .ok_or_else(|| PreseedTomlError::Parse("[worker_wallets] must be a table".to_string()))?;

    let mut entries = Vec::with_capacity(tbl.len());
    for (key, value) in tbl.iter() {
        let balance = value.as_integer().ok_or_else(|| {
            PreseedTomlError::Parse(format!(
                "[worker_wallets].{key} must be an integer (got {value:?})"
            ))
        })?;
        entries.push((AgentId(key.clone()), MicroCoin::from_micro_units(balance)));
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// U1 — factory returns 13 entries: 1 tb7-7-sponsor + 1 Agent_user_0 + 10 Agent_i
    /// + 1 MarketMakerBudget (TB-N3 A0.5 net-new 2026-05-11; architect
    /// ruling Q2 + amendment 6).
    #[test]
    fn returns_13_entries() {
        let pairs = default_pput_preseed_pairs();
        assert_eq!(
            pairs.len(),
            13,
            "expected 13 preseed entries (12 legacy + 1 TB-N3 MarketMakerBudget)"
        );
    }

    /// U2 — every entry has positive balance (no zero-funded agent).
    #[test]
    fn every_entry_has_positive_balance() {
        for (agent, balance) in default_pput_preseed_pairs() {
            assert!(
                balance.micro_units() > 0,
                "agent {} has zero balance",
                agent.0
            );
        }
    }

    /// U3 — Agent_user_0 is present with the documented sponsor budget.
    #[test]
    fn agent_user_0_present_with_sponsor_budget() {
        let pairs = default_pput_preseed_pairs();
        let user_entry = pairs
            .iter()
            .find(|(a, _)| a.0 == "Agent_user_0")
            .expect("Agent_user_0 must be in preseed list");
        assert_eq!(
            user_entry.1.micro_units(),
            10_000_000,
            "Agent_user_0 sponsor budget"
        );
    }

    /// U4 — tb7-7-sponsor is preserved (back-compat with TB-7.7 D3 evaluator preseed).
    #[test]
    fn tb_7_7_sponsor_preserved() {
        let pairs = default_pput_preseed_pairs();
        let sponsor_entry = pairs
            .iter()
            .find(|(a, _)| a.0 == "tb7-7-sponsor")
            .expect("tb7-7-sponsor must be in preseed list");
        assert_eq!(
            sponsor_entry.1.micro_units(),
            10_000_000,
            "tb7-7-sponsor budget"
        );
    }

    /// U5 — 10 solver agents Agent_0..Agent_9 each at 1_000_000 micro.
    #[test]
    fn ten_solver_agents_each_one_coin() {
        let pairs = default_pput_preseed_pairs();
        for i in 0..10 {
            let id = format!("Agent_{i}");
            let entry = pairs
                .iter()
                .find(|(a, _)| a.0 == id)
                .unwrap_or_else(|| panic!("Agent_{i} must be in preseed list"));
            assert_eq!(entry.1.micro_units(), 1_000_000, "Agent_{i} budget");
        }
    }

    /// U6 — total preseed supply is 35_000_000 micro (30M legacy +
    /// 5M TB-N3 MarketMakerBudget per architect Q1+Q2).
    #[test]
    fn total_preseed_supply_35m() {
        let total: i64 = default_pput_preseed_pairs()
            .iter()
            .map(|(_, m)| m.micro_units())
            .sum();
        assert_eq!(
            total, 35_000_000,
            "total preseed micro (30M legacy + 5M MarketMakerBudget)"
        );
    }

    /// U9 — TB-N3 A0.5 (architect ruling 2026-05-11 Q2 + amendment 6):
    /// MarketMakerBudget identity is present at the documented 5M micro
    /// (10× headroom over Phase-2 expected pool-seed draw per Q1).
    #[test]
    fn market_maker_budget_present_with_5m_micro() {
        let pairs = default_pput_preseed_pairs();
        let mmb = pairs
            .iter()
            .find(|(a, _)| a.0 == "MarketMakerBudget")
            .expect("MarketMakerBudget must be in TB-N3 preseed list");
        assert_eq!(mmb.1.micro_units(), 5_000_000, "MarketMakerBudget budget");
    }

    /// U7 — factory is deterministic: two calls produce byte-identical output.
    #[test]
    fn deterministic_across_calls() {
        let a = default_pput_preseed_pairs();
        let b = default_pput_preseed_pairs();
        assert_eq!(a.len(), b.len());
        for ((a_id, a_m), (b_id, b_m)) in a.iter().zip(b.iter()) {
            assert_eq!(a_id.0, b_id.0);
            assert_eq!(a_m.micro_units(), b_m.micro_units());
        }
    }

    /// U8 — feeding the factory output into genesis_with_balances yields a
    /// QState whose balances_t Σ matches the documented 35M total (TB-N3
    /// A0.5: 30M legacy + 5M MarketMakerBudget).
    #[test]
    fn genesis_construction_matches_total() {
        use crate::runtime::adapter::genesis_with_balances;
        let pairs = default_pput_preseed_pairs();
        let q = genesis_with_balances(&pairs);
        let total: i64 = q
            .economic_state_t
            .balances_t
            .0
            .values()
            .map(|m| m.micro_units())
            .sum();
        assert_eq!(total, 35_000_000, "genesis balances Σ");
    }

    // ─────────────────────────────────────────────────────────────────
    // Polymarket PR1 (2026-05-23) — Treasury + worker-wallets TOML parser
    // Karpathy K6: temporal `pr1_*` namespace dropped; tests renamed to
    // describe what they prove rather than which PR they shipped under.
    // ─────────────────────────────────────────────────────────────────

    const PRESEED_FIXTURE: &str = r#"
[treasury]
agent_id = "treasury"
initial_balance_micro = 100_000

[worker_wallets]
"worker-alpha" = 10_000
"#;

    #[test]
    fn treasury_preseed_returns_treasury_and_one_worker() {
        let pairs = parse_treasury_and_worker_preseed(PRESEED_FIXTURE).expect("parse ok");
        assert_eq!(pairs.len(), 2, "treasury + 1 worker entry");
        // First entry MUST be treasury (parser contract).
        assert_eq!(pairs[0].0 .0, "treasury");
        assert_eq!(pairs[0].1.micro_units(), 100_000);
        // Worker entries follow.
        let worker = pairs
            .iter()
            .find(|(a, _)| a.0 == "worker-alpha")
            .expect("worker-alpha entry");
        assert_eq!(worker.1.micro_units(), 10_000);
    }

    #[test]
    fn treasury_preseed_total_supply_is_110k() {
        let pairs = parse_treasury_and_worker_preseed(PRESEED_FIXTURE).expect("parse ok");
        let total: i64 = pairs.iter().map(|(_, m)| m.micro_units()).sum();
        assert_eq!(total, 110_000);
    }

    #[test]
    fn treasury_preseed_genesis_construction_balances_match() {
        use crate::runtime::adapter::genesis_with_balances;
        let pairs = parse_treasury_and_worker_preseed(PRESEED_FIXTURE).expect("parse ok");
        let q = genesis_with_balances(&pairs);
        let total: i64 = q
            .economic_state_t
            .balances_t
            .0
            .values()
            .map(|m| m.micro_units())
            .sum();
        assert_eq!(total, 110_000);
        let treasury = q
            .economic_state_t
            .balances_t
            .0
            .get(&AgentId("treasury".into()))
            .copied()
            .expect("treasury preseeded");
        assert_eq!(treasury.micro_units(), 100_000);
        let worker = q
            .economic_state_t
            .balances_t
            .0
            .get(&AgentId("worker-alpha".into()))
            .copied()
            .expect("worker-alpha preseeded");
        assert_eq!(worker.micro_units(), 10_000);
    }

    #[test]
    fn treasury_preseed_fails_when_treasury_section_missing() {
        let toml_text = r#"
[worker_wallets]
"worker-alpha" = 10_000
"#;
        let err = parse_treasury_and_worker_preseed(toml_text).expect_err("must fail");
        match err {
            PreseedTomlError::Parse(_) | PreseedTomlError::MissingSection(_) => {}
        }
    }

    #[test]
    fn treasury_preseed_fails_when_worker_wallets_section_missing() {
        let toml_text = r#"
[treasury]
agent_id = "treasury"
initial_balance_micro = 100_000
"#;
        let err = parse_treasury_and_worker_preseed(toml_text).expect_err("must fail");
        match err {
            PreseedTomlError::Parse(_) | PreseedTomlError::MissingSection(_) => {}
        }
    }

    #[test]
    fn treasury_preseed_deterministic() {
        let a = parse_treasury_and_worker_preseed(PRESEED_FIXTURE).unwrap();
        let b = parse_treasury_and_worker_preseed(PRESEED_FIXTURE).unwrap();
        assert_eq!(a.len(), b.len());
        for ((aid, am), (bid, bm)) in a.iter().zip(b.iter()) {
            assert_eq!(aid.0, bid.0);
            assert_eq!(am.micro_units(), bm.micro_units());
        }
    }

    /// The live repo-root `genesis_payload.toml` MUST carry the
    /// `[treasury]` + `[worker_wallets]` tables, and the parser MUST be
    /// able to construct a genesis QState from them.
    #[test]
    fn treasury_preseed_live_genesis_payload_toml_parses() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path = std::path::PathBuf::from(manifest_dir).join("genesis_payload.toml");
        let text = std::fs::read_to_string(&path).expect("read live genesis_payload.toml");
        let pairs = parse_treasury_and_worker_preseed(&text)
            .expect("live genesis_payload.toml must include treasury preseed sections");
        // Treasury entry present.
        let treasury = pairs
            .iter()
            .find(|(a, _)| a.0 == "treasury")
            .expect("treasury entry in live preseed");
        assert_eq!(treasury.1.micro_units(), 100_000);
        // worker-alpha entry present.
        let worker = pairs
            .iter()
            .find(|(a, _)| a.0 == "worker-alpha")
            .expect("worker-alpha entry in live preseed");
        assert_eq!(worker.1.micro_units(), 10_000);
    }
}
