// Phase A atom A5 — explicit per-agent budget regime (`BUDGET_REGIME` +
// `MAX_TRANSACTIONS` env vars).
//
// Constitutional anchor: FC2-N22 (HALT node, `MaxTxExhausted` variant).
// The transaction loop in `run_swarm` terminates on either OMEGA accept
// (FC1-E17) or budget exhaustion (FC2-N22 / `MaxTxExhausted`). The budget
// regime declares HOW the run-level budget is partitioned across the N
// δ instances (Agent_i, FC1-N7) so that PPUT comparisons across N values
// answer a well-posed question.
//
// Codex / Gemini N-agents brainstorm 2026-04-25 § A.3 frames four regimes:
//   - fixed transaction budget         (tx=200 for all N)
//   - fixed proposal / N×tx budget     (each agent gets tx=base proposals)
//   - fixed token budget               (cap on total LLM tokens)
//   - fixed wall-clock budget          (cap on T_i)
//
// In our codebase the inner loop (`for tx in 0..max_transactions`) invokes
// exactly ONE agent per `tx` (Boltzmann-routed). So "tx" already counts
// proposals, and the brainstorm's "fixed transaction budget" maps to
// `total_proposal` (loop bound = base, regardless of N — current
// behavior). The brainstorm's "N × tx = constant" is orthogonal: we want
// the loop bound to scale with N so each agent receives the same number
// of proposals — that's `per_agent`.
//
// PREREG_AMENDMENT_p0_defer § 3 condition 3 (2026-04-25) names this atom
// as a re-calibration prerequisite: "current max_transactions=200 is
// fixed-tx budget; PREREG § 5.5 implicitly assumes tx-budget but doesn't
// specify; need explicit budget regime declaration for calibration to be
// reproducible." A5 satisfies that by stamping the regime + base budget
// on every emitted v2 row.
//
// Phase A scope: implement `total_proposal` + `per_agent` (the two
// regimes that fall out of the existing tx loop). `token_total` /
// `wall_clock` require new exit conditions (cost / clock thresholds) and
// are declared startup-fatal `UnimplementedRegime` so a misconfigured
// `BUDGET_REGIME=token_total` aborts before burning LLM budget. These
// land in a later atom once the cost/clock exit machinery exists.

use std::fmt;

/// TRACE_MATRIX FC2-N22: env var selecting how the run-level transaction
/// budget partitions across N δ agents. Default (unset/empty) =
/// `total_proposal`, preserving Phase B baseline behavior bit-for-bit.
pub const BUDGET_REGIME_ENV_VAR: &str = "BUDGET_REGIME";

/// TRACE_MATRIX FC2-N22: env var setting the base transaction budget.
/// The effective loop bound is `effective_max_tx(regime, base, N)`.
/// Default 200 (Phase B baseline).
pub const MAX_TRANSACTIONS_ENV_VAR: &str = "MAX_TRANSACTIONS";

/// Default base budget when `MAX_TRANSACTIONS` env is unset.
/// Preserves the long-standing `let max_transactions = 200` baseline.
pub const DEFAULT_MAX_TRANSACTIONS: u32 = 200;

/// TRACE_MATRIX FC2-N22: budget regime variants. The first two are
/// implemented in Phase A; the latter two are declared so a downstream
/// run that wants them aborts at startup (UnimplementedRegime) instead
/// of silently falling back and burning budget under the wrong regime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetRegime {
    /// Loop bound = `base`, regardless of N. Each agent ends up with
    /// roughly `base / N` proposals. **Phase B baseline + default.**
    /// Brainstorm § A.3 "fixed transaction budget".
    TotalProposal,
    /// Loop bound = `base × N`. Each agent receives `base` proposals
    /// regardless of swarm size. Brainstorm § A.3 "N × tx = constant"
    /// reframed as "constant per-agent".
    PerAgent,
    /// Cap total LLM tokens (declared, not yet implemented). Requires a
    /// new exit condition tied to `RunCostAccumulator` thresholds.
    TokenTotal,
    /// Cap wall-clock T_i (declared, not yet implemented). Requires a
    /// new exit condition tied to `RunWallClock`.
    WallClock,
}

impl BudgetRegime {
    /// Stable string label stamped on jsonl rows. Stable across releases;
    /// downstream PPUT analysis joins on this exact string.
    pub fn label(&self) -> &'static str {
        match self {
            BudgetRegime::TotalProposal => "total_proposal",
            BudgetRegime::PerAgent => "per_agent",
            BudgetRegime::TokenTotal => "token_total",
            BudgetRegime::WallClock => "wall_clock",
        }
    }
}

/// TRACE_MATRIX FC2-N22: startup-fatal failure modes for the regime
/// resolver. Each variant aborts before the first LLM call so a
/// misconfigured run cannot consume API budget.
#[derive(Debug, PartialEq, Eq)]
pub enum BudgetError {
    /// `BUDGET_REGIME` value not in
    /// {`total_proposal`, `per_agent`, `token_total`, `wall_clock`}.
    UnknownRegime(String),
    /// `MAX_TRANSACTIONS` not parseable as positive u32.
    InvalidMaxTransactions(String),
    /// Caller asked for a regime whose exit machinery is not yet wired
    /// (`token_total` / `wall_clock`). Carries the requested variant so
    /// the startup error names what is missing.
    UnimplementedRegime(BudgetRegime),
    /// Effective loop bound would overflow u32 (`base × N > u32::MAX`).
    /// Realistically unreachable (would require base × N ≥ 2^32) but
    /// expressed in the type so the callers cannot panic on overflow.
    EffectiveBudgetOverflow { base: u32, n_agents: usize },
}

impl fmt::Display for BudgetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownRegime(s) => write!(
                f,
                "BUDGET_REGIME='{}' is not a known regime \
                 (expected total_proposal | per_agent | token_total | wall_clock)",
                s
            ),
            Self::InvalidMaxTransactions(s) => write!(
                f,
                "MAX_TRANSACTIONS='{}' is not a positive integer",
                s
            ),
            Self::UnimplementedRegime(r) => write!(
                f,
                "BUDGET_REGIME='{}' declared but its exit machinery is not yet \
                 implemented (Phase A scope = total_proposal + per_agent only). \
                 Aborting startup to avoid silent fallback under a different regime.",
                r.label()
            ),
            Self::EffectiveBudgetOverflow { base, n_agents } => write!(
                f,
                "effective_max_tx overflow: base={} × n_agents={} exceeds u32::MAX",
                base, n_agents
            ),
        }
    }
}

impl std::error::Error for BudgetError {}

/// TRACE_MATRIX FC2-N22: pure parser for the `BUDGET_REGIME` env value.
/// Empty (unset / blank-after-trim) → default `TotalProposal`. No env
/// access — testable without process-global state.
pub fn parse_budget_regime(env_str: &str) -> Result<BudgetRegime, BudgetError> {
    let trimmed = env_str.trim();
    if trimmed.is_empty() {
        return Ok(BudgetRegime::TotalProposal);
    }
    match trimmed {
        "total_proposal" => Ok(BudgetRegime::TotalProposal),
        "per_agent" => Ok(BudgetRegime::PerAgent),
        "token_total" => Ok(BudgetRegime::TokenTotal),
        "wall_clock" => Ok(BudgetRegime::WallClock),
        other => Err(BudgetError::UnknownRegime(other.to_string())),
    }
}

/// TRACE_MATRIX FC2-N22: pure parser for the `MAX_TRANSACTIONS` env
/// value. Empty (unset / blank-after-trim) → default
/// `DEFAULT_MAX_TRANSACTIONS`. Pure (no env access).
pub fn parse_max_transactions(env_str: &str) -> Result<u32, BudgetError> {
    let trimmed = env_str.trim();
    if trimmed.is_empty() {
        return Ok(DEFAULT_MAX_TRANSACTIONS);
    }
    match trimmed.parse::<u32>() {
        Ok(0) => Err(BudgetError::InvalidMaxTransactions(trimmed.to_string())),
        Ok(v) => Ok(v),
        Err(_) => Err(BudgetError::InvalidMaxTransactions(trimmed.to_string())),
    }
}

/// TRACE_MATRIX FC2-N22: scale the base budget by the regime + swarm
/// size. Pure. Returns the loop bound (`for tx in 0..effective_max_tx`).
///
/// - TotalProposal → base
/// - PerAgent      → base × n_agents (overflow-checked)
/// - TokenTotal / WallClock → UnimplementedRegime (Phase A scope)
///
/// `n_agents == 0` is rejected upstream (run_swarm precondition); we
/// pass it through here to stay pure but the multiplication is safe
/// (`base × 0 = 0`, which fails the `for tx in 0..0` loop fast).
pub fn effective_max_tx(
    regime: BudgetRegime,
    base: u32,
    n_agents: usize,
) -> Result<u32, BudgetError> {
    match regime {
        BudgetRegime::TotalProposal => Ok(base),
        BudgetRegime::PerAgent => {
            let n = n_agents as u64;
            let prod = (base as u64).saturating_mul(n);
            if prod > u32::MAX as u64 {
                return Err(BudgetError::EffectiveBudgetOverflow { base, n_agents });
            }
            Ok(prod as u32)
        }
        BudgetRegime::TokenTotal | BudgetRegime::WallClock => {
            Err(BudgetError::UnimplementedRegime(regime))
        }
    }
}

/// TRACE_MATRIX FC2-N22: env-coupled resolver invoked once at run_swarm
/// startup. Returns `(regime, base_max_tx, effective_max_tx)` so the
/// caller can both run the loop AND stamp the regime + base on the
/// emitted v2 row. Errors abort the run before the first LLM call.
pub fn resolve_budget(n_agents: usize) -> Result<(BudgetRegime, u32, u32), BudgetError> {
    let regime_raw = std::env::var(BUDGET_REGIME_ENV_VAR).unwrap_or_default();
    let base_raw = std::env::var(MAX_TRANSACTIONS_ENV_VAR).unwrap_or_default();
    let regime = parse_budget_regime(&regime_raw)?;
    let base = parse_max_transactions(&base_raw)?;
    let eff = effective_max_tx(regime, base, n_agents)?;
    Ok((regime, base, eff))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Per memory `feedback_env_var_test_lock`: tests that mutate
    // process-global env vars (BUDGET_REGIME / MAX_TRANSACTIONS) must
    // serialise to survive cargo's parallel runner.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    // Phase A atom A5 unit tests. Pure-fn surface first; the env-coupled
    // `resolve_budget` is tested at the bottom under the env mutex.

    #[test]
    fn parse_regime_empty_defaults_to_total_proposal() {
        assert_eq!(parse_budget_regime("").unwrap(), BudgetRegime::TotalProposal);
        assert_eq!(parse_budget_regime("   ").unwrap(), BudgetRegime::TotalProposal);
    }

    #[test]
    fn parse_regime_known_values() {
        assert_eq!(
            parse_budget_regime("total_proposal").unwrap(),
            BudgetRegime::TotalProposal
        );
        assert_eq!(
            parse_budget_regime("per_agent").unwrap(),
            BudgetRegime::PerAgent
        );
        assert_eq!(
            parse_budget_regime("token_total").unwrap(),
            BudgetRegime::TokenTotal
        );
        assert_eq!(
            parse_budget_regime("wall_clock").unwrap(),
            BudgetRegime::WallClock
        );
    }

    #[test]
    fn parse_regime_unknown_rejected() {
        match parse_budget_regime("foobar") {
            Err(BudgetError::UnknownRegime(s)) => assert_eq!(s, "foobar"),
            other => panic!("expected UnknownRegime, got {:?}", other),
        }
    }

    #[test]
    fn parse_max_transactions_empty_defaults_to_200() {
        assert_eq!(parse_max_transactions("").unwrap(), DEFAULT_MAX_TRANSACTIONS);
        assert_eq!(parse_max_transactions("   ").unwrap(), DEFAULT_MAX_TRANSACTIONS);
        assert_eq!(DEFAULT_MAX_TRANSACTIONS, 200);
    }

    #[test]
    fn parse_max_transactions_valid() {
        assert_eq!(parse_max_transactions("50").unwrap(), 50);
        assert_eq!(parse_max_transactions("1000").unwrap(), 1000);
    }

    #[test]
    fn parse_max_transactions_zero_rejected() {
        // 0 would make the loop never enter — almost certainly a config
        // bug, not an intentional zero-iteration request.
        match parse_max_transactions("0") {
            Err(BudgetError::InvalidMaxTransactions(s)) => assert_eq!(s, "0"),
            other => panic!("expected InvalidMaxTransactions, got {:?}", other),
        }
    }

    #[test]
    fn parse_max_transactions_negative_rejected() {
        match parse_max_transactions("-5") {
            Err(BudgetError::InvalidMaxTransactions(s)) => assert_eq!(s, "-5"),
            other => panic!("expected InvalidMaxTransactions, got {:?}", other),
        }
    }

    #[test]
    fn parse_max_transactions_garbage_rejected() {
        match parse_max_transactions("abc") {
            Err(BudgetError::InvalidMaxTransactions(s)) => assert_eq!(s, "abc"),
            other => panic!("expected InvalidMaxTransactions, got {:?}", other),
        }
    }

    #[test]
    fn effective_total_proposal_invariant_under_n() {
        // The defining property of TotalProposal: loop bound is
        // independent of N. This is what makes per-agent invocations
        // ≈ base/N at large N.
        for n in [1, 2, 3, 8, 13, 34usize] {
            assert_eq!(
                effective_max_tx(BudgetRegime::TotalProposal, 200, n).unwrap(),
                200,
                "TotalProposal should not scale with N (n={})", n
            );
        }
    }

    #[test]
    fn effective_per_agent_scales_linearly_in_n() {
        // The defining property of PerAgent: loop bound = base × N.
        for (base, n) in [(200, 1), (200, 8), (50, 13), (100, 3)] {
            let expected = (base as usize * n) as u32;
            assert_eq!(
                effective_max_tx(BudgetRegime::PerAgent, base, n).unwrap(),
                expected,
                "PerAgent should scale linearly (base={}, n={})", base, n
            );
        }
    }

    #[test]
    fn effective_token_total_unimplemented() {
        match effective_max_tx(BudgetRegime::TokenTotal, 200, 8) {
            Err(BudgetError::UnimplementedRegime(BudgetRegime::TokenTotal)) => {}
            other => panic!("expected UnimplementedRegime(TokenTotal), got {:?}", other),
        }
    }

    #[test]
    fn effective_wall_clock_unimplemented() {
        match effective_max_tx(BudgetRegime::WallClock, 200, 8) {
            Err(BudgetError::UnimplementedRegime(BudgetRegime::WallClock)) => {}
            other => panic!("expected UnimplementedRegime(WallClock), got {:?}", other),
        }
    }

    #[test]
    fn effective_per_agent_overflow_rejected() {
        // Construct a base × N that overflows u32. Realistically
        // unreachable (200 × 34 = 6800; the swarm cap is N_max = 34),
        // but the type-level guarantee matters under
        // misconfiguration.
        let huge = u32::MAX;
        match effective_max_tx(BudgetRegime::PerAgent, huge, 2) {
            Err(BudgetError::EffectiveBudgetOverflow { base, n_agents }) => {
                assert_eq!(base, huge);
                assert_eq!(n_agents, 2);
            }
            other => panic!("expected EffectiveBudgetOverflow, got {:?}", other),
        }
    }

    #[test]
    fn label_strings_are_stable() {
        // Downstream PPUT analysis joins on these exact strings;
        // changing them is a breaking change for the v2 schema.
        assert_eq!(BudgetRegime::TotalProposal.label(), "total_proposal");
        assert_eq!(BudgetRegime::PerAgent.label(), "per_agent");
        assert_eq!(BudgetRegime::TokenTotal.label(), "token_total");
        assert_eq!(BudgetRegime::WallClock.label(), "wall_clock");
    }

    #[test]
    fn n_agents_zero_does_not_panic() {
        // run_swarm enforces n_agents >= 1 upstream, but this module is
        // pure and must not panic on 0.
        assert_eq!(
            effective_max_tx(BudgetRegime::TotalProposal, 200, 0).unwrap(),
            200
        );
        assert_eq!(
            effective_max_tx(BudgetRegime::PerAgent, 200, 0).unwrap(),
            0
        );
    }

    /// Env-coupled wrapper round-trip: empty env (default) preserves
    /// the Phase B baseline (TotalProposal × 200).
    #[test]
    fn resolve_budget_default_preserves_phase_b_baseline() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::remove_var(BUDGET_REGIME_ENV_VAR);
        std::env::remove_var(MAX_TRANSACTIONS_ENV_VAR);

        let (regime, base, eff) = resolve_budget(8).unwrap();
        assert_eq!(regime, BudgetRegime::TotalProposal);
        assert_eq!(base, DEFAULT_MAX_TRANSACTIONS);
        assert_eq!(eff, DEFAULT_MAX_TRANSACTIONS);
    }

    /// PerAgent regime via env scales the loop bound linearly in N.
    /// Codex/Gemini brainstorm § A.3 "fixed proposal budget" reframed
    /// as constant per-agent.
    #[test]
    fn resolve_budget_per_agent_via_env() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var(BUDGET_REGIME_ENV_VAR, "per_agent");
        std::env::set_var(MAX_TRANSACTIONS_ENV_VAR, "50");

        let (regime, base, eff) = resolve_budget(8).unwrap();
        assert_eq!(regime, BudgetRegime::PerAgent);
        assert_eq!(base, 50);
        assert_eq!(eff, 400);

        std::env::remove_var(BUDGET_REGIME_ENV_VAR);
        std::env::remove_var(MAX_TRANSACTIONS_ENV_VAR);
    }

    /// Declared-but-unimplemented regime aborts startup so a
    /// misconfigured run cannot silently fall back to a different
    /// regime and burn LLM budget under the wrong partitioning rule.
    #[test]
    fn resolve_budget_token_total_startup_fatal() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var(BUDGET_REGIME_ENV_VAR, "token_total");
        std::env::remove_var(MAX_TRANSACTIONS_ENV_VAR);

        match resolve_budget(3) {
            Err(BudgetError::UnimplementedRegime(BudgetRegime::TokenTotal)) => {}
            other => panic!("expected UnimplementedRegime(TokenTotal), got {:?}", other),
        }

        std::env::remove_var(BUDGET_REGIME_ENV_VAR);
    }

    /// Unknown regime spelling aborts startup with the offending
    /// string surfaced in the error (operator-friendly diagnostic).
    #[test]
    fn resolve_budget_unknown_regime_via_env() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var(BUDGET_REGIME_ENV_VAR, "fixed_tx");

        match resolve_budget(3) {
            Err(BudgetError::UnknownRegime(s)) => assert_eq!(s, "fixed_tx"),
            other => panic!("expected UnknownRegime, got {:?}", other),
        }

        std::env::remove_var(BUDGET_REGIME_ENV_VAR);
    }
}
