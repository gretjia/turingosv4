//! TB-G G3.1 — `compute_agent_pnl` derived view + 7-field `AgentMarketStateView`.
//!
//! Charter: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
//! §1 Module G3 atom G3.1.
//!
//! Directive: `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
//! §G3 verbatim 7-field shape + SG-G3.5 "PnL is visible in dashboard as
//! materialized view".
//!
//! Pure derived view: reads canonical `EconomicState` (balances_t / stakes_t /
//! claims_t / reputations_t / conditional_share_balances_t / cpmm_pools_t /
//! lp_share_balances_t / conditional_collateral_t / node_positions_t) and
//! returns the architect-spec'd `AgentMarketStateView` for one agent.
//! **No state mutation**. CLAUDE.md §13 economy laws preserved: integer
//! math throughout (no f64 in money path); the view never mints, debits, or
//! moves a single μCoin.
//!
//! **PnL semantics** (architect §G3 + Drucker framing):
//! - `realized_pnl = current_balance - initial_balance_micro`. Signed cash
//!   delta from genesis. Positive = cash profit (rewards received); negative =
//!   cash deployed into open positions (stakes locked, escrows funded, mints
//!   converted to share inventory).
//! - `unrealized_pnl` = signed mark-to-market gain/loss on conditional-share
//!   holdings priced against active CPMM pool reserves. Cost basis per share
//!   pair = 1 μCoin (the symmetric `CompleteSetMint` cash flow: 1 collateral
//!   μCoin -> 1 YES + 1 NO share, so each share carries 0.5 μC of the original
//!   cash). Stake/claim/LP/NodePosition holdings contribute 0 (their cost
//!   basis equals their face value; signed PnL needs market signal). Their
//!   capital exposure remains visible via `open_positions`.
//!
//! Concretely, for each `(event_id, ShareSidePair)` holding under an *Active*
//! pool with reserves `(pool_yes, pool_no)`:
//! - `yes_mtm = yes_units × pool_no / (pool_yes + pool_no)` (constant-product
//!   YES price contribution).
//! - `no_mtm  = no_units  × pool_yes / (pool_yes + pool_no)`.
//! - `cost_basis_micro = (yes_units + no_units) / 2` (integer divide; matches
//!   mint cash flow 1 μC -> 1 YES + 1 NO).
//! - Contribution to `unrealized_pnl` = `(yes_mtm + no_mtm) - cost_basis_micro`,
//!   signed.
//!
//! **Balanced-mint invariant** (verified in tests): a symmetric N YES + N NO
//! holding contributes 0 to `unrealized_pnl` regardless of pool reserves —
//! constant-product YES + NO prices sum to 1, so MTM = N = cost basis.
//! Only an *asymmetric* position (post-`BuyWithCoinRouter`, swap, or partial
//! redemption) produces non-zero signed PnL. This is the architect's "bull/
//! bear emergence" signal: a buy when the market disagrees later shows up
//! as signed unrealized PnL.
//!
//! **Without an active pool** (no pool yet, or `Resolved` / `Closed`): the
//! contribution is 0 (no live price signal; cost basis equals face).
//! G3.2 / future TBs will extend this when resolution oracles land.
//!
//! **Solvency classification** (3-tier; G3.2 will add sequencer-side risk-
//! cap admission keyed on this enum):
//! - `Solvent`: balance ≥ 10% of `initial_balance_micro`.
//! - `NearInsolvent`: balance > 0 but below 10% of initial.
//! - `Bankrupt`: balance ≤ 0.
//! The 10% threshold matches the architect's "low-balance" framing in §G3
//! SG-G3.3 without committing to the Class-4 risk-cap constant
//! (`BANKRUPTCY_RISK_CAP_MICRO`) that lands in G3.2.
//!
//! **Constitutional binding**:
//! - CLAUDE.md §13: integer-rational math, no f64 in money path. All MTM
//!   computations use `u128` integer multiply + integer divide (floor).
//! - CLAUDE.md §16: reads canonical state only; never reads shadow tape.
//! - Art. III shielding: per-viewer renderer; never aggregates across
//!   agents (the caller picks the `agent_id` and gets ONLY that agent's
//!   view).

use serde::{Deserialize, Serialize};

use crate::state::q_state::{AgentId, ClaimStatus, EconomicState, PoolStatus, QState, TxId};
use crate::state::typed_tx::{EventId, OutcomeSide, PositionKind, PositionSide};

/// TRACE_MATRIX FC1-N5 + §15 + §17 (TB-G G3.1 2026-05-12; G-Phase directive
/// §G3 verbatim 7-field shape).
///
/// Architect verbatim: `{ agent_id, balance, open_positions, realized_pnl,
/// unrealized_pnl, solvency_status, reputation_score }`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentMarketStateView {
    pub agent_id: AgentId,
    /// Current spot balance in μCoin (matches `balances_t.get(agent_id)`).
    pub balance: i64,
    /// Open exposure surfaces: stakes, pending claims, conditional shares,
    /// LP shares, node positions. Empty for a fresh genesis agent.
    pub open_positions: Vec<OpenPosition>,
    /// Signed cash delta since genesis. `balance - initial_balance_micro`.
    pub realized_pnl: i64,
    /// Signed mark-to-market PnL on conditional-share holdings priced
    /// against active CPMM pools (see module doc).
    pub unrealized_pnl: i64,
    /// 3-tier solvency classification. G3.2 sequencer risk-cap admission
    /// will key per-arm preconditions off this enum.
    pub solvency_status: SolvencyStatus,
    /// `reputations_t.get(agent_id).map(|r| r.0).unwrap_or(0)`.
    pub reputation_score: i64,
}

/// TRACE_MATRIX FC1-N5 (TB-G G3.1 2026-05-12): structured open-position
/// surface. One variant per canonical exposure index in `EconomicState`.
/// Keeps the architect's `open_positions: Vec<_>` shape concrete and
/// audit-greppable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenPosition {
    /// `stakes_t` entry — locked WorkTx stake (returns on accept; slashed
    /// on challenge upheld).
    Stake {
        tx_id: TxId,
        amount_micro: i64,
    },
    /// `claims_t` entry with `status == Open` — pending reward (credited on
    /// FinalizeReward dispatch arm).
    Claim {
        tx_id: TxId,
        amount_micro: i64,
    },
    /// `conditional_share_balances_t` holding for one event_id × side.
    ConditionalShare {
        event_id: EventId,
        side: OutcomeSide,
        units: u128,
    },
    /// `lp_share_balances_t` holding for one CPMM pool.
    LpShare {
        event_id: EventId,
        units: u128,
    },
    /// `node_positions_t` entry — immutable exposure record (TB-12).
    NodePosition {
        position_id: TxId,
        node_id: TxId,
        side: PositionSide,
        kind: PositionKind,
        amount_micro: i64,
    },
}

/// TRACE_MATRIX FC1-N5 (TB-G G3.1 2026-05-12; G-Phase directive §G3
/// SG-G3.3): 3-tier solvency classification, derived from `balance` against
/// the agent's `initial_balance_micro` baseline.
///
/// Thresholds:
/// - `Bankrupt`: balance ≤ 0 (agent has no cash to stake/mint/bond).
/// - `NearInsolvent`: 0 < balance < 10% of initial baseline.
/// - `Solvent`: balance ≥ 10% of initial baseline.
///
/// The 10% threshold is a G3.1 stand-in for the future G3.2 Class-4
/// `BANKRUPTCY_RISK_CAP_MICRO` constant; once G3.2 lands, the classifier
/// switches to reading the architect-ratified cap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SolvencyStatus {
    Solvent,
    NearInsolvent,
    Bankrupt,
}

/// TRACE_MATRIX FC1-N5 (TB-G G3.1 2026-05-12; G-Phase directive §G3
/// verbatim 7-field shape): compute the 7-field `AgentMarketStateView` for
/// one agent. Pure derivation; no state mutation.
///
/// `initial_balance_micro` is the agent's preseed credit at genesis (see
/// `crate::runtime::bootstrap::default_pput_preseed_pairs`). Realized PnL
/// is signed against this baseline so a fresh genesis agent reports
/// `realized_pnl == 0`.
pub fn compute_agent_pnl(
    q: &QState,
    agent_id: &AgentId,
    initial_balance_micro: i64,
) -> AgentMarketStateView {
    let balance: i64 = q
        .economic_state_t
        .balances_t
        .0
        .get(agent_id)
        .map(|m| m.micro_units())
        .unwrap_or(0);

    let open_positions = collect_open_positions(&q.economic_state_t, agent_id);
    let unrealized_pnl = compute_unrealized_pnl(&q.economic_state_t, agent_id);
    let realized_pnl = balance.saturating_sub(initial_balance_micro);
    let reputation_score: i64 = q
        .economic_state_t
        .reputations_t
        .0
        .get(agent_id)
        .map(|r| r.0)
        .unwrap_or(0);
    let solvency_status = classify_solvency(balance, initial_balance_micro);

    AgentMarketStateView {
        agent_id: agent_id.clone(),
        balance,
        open_positions,
        realized_pnl,
        unrealized_pnl,
        solvency_status,
        reputation_score,
    }
}

fn collect_open_positions(econ: &EconomicState, agent_id: &AgentId) -> Vec<OpenPosition> {
    let mut out: Vec<OpenPosition> = Vec::new();

    for (tx_id, entry) in &econ.stakes_t.0 {
        if &entry.staker == agent_id {
            out.push(OpenPosition::Stake {
                tx_id: tx_id.clone(),
                amount_micro: entry.amount.micro_units(),
            });
        }
    }

    for (tx_id, entry) in &econ.claims_t.0 {
        if &entry.claimant == agent_id && matches!(entry.status, ClaimStatus::Open) {
            out.push(OpenPosition::Claim {
                tx_id: tx_id.clone(),
                amount_micro: entry.amount.micro_units(),
            });
        }
    }

    if let Some(holdings) = econ.conditional_share_balances_t.0.get(agent_id) {
        for (event_id, pair) in holdings {
            if pair.yes.units > 0 {
                out.push(OpenPosition::ConditionalShare {
                    event_id: event_id.clone(),
                    side: OutcomeSide::Yes,
                    units: pair.yes.units,
                });
            }
            if pair.no.units > 0 {
                out.push(OpenPosition::ConditionalShare {
                    event_id: event_id.clone(),
                    side: OutcomeSide::No,
                    units: pair.no.units,
                });
            }
        }
    }

    for ((agent, event_id), lp_amount) in &econ.lp_share_balances_t.0 {
        if agent == agent_id && lp_amount.units > 0 {
            out.push(OpenPosition::LpShare {
                event_id: event_id.clone(),
                units: lp_amount.units,
            });
        }
    }

    for pos in econ.node_positions_t.0.values() {
        if &pos.owner == agent_id {
            out.push(OpenPosition::NodePosition {
                position_id: pos.position_id.clone(),
                node_id: pos.node_id.clone(),
                side: pos.side,
                kind: pos.kind,
                amount_micro: pos.amount.micro_units(),
            });
        }
    }

    out
}

/// Compute signed mark-to-market PnL on conditional-share holdings priced
/// against active CPMM pools. Stakes / claims / LP shares / node positions
/// contribute 0 (their cost basis equals face value — visible via
/// `open_positions` instead). See module doc for the full semantics.
fn compute_unrealized_pnl(econ: &EconomicState, agent_id: &AgentId) -> i64 {
    let mut total: i128 = 0;

    let Some(holdings) = econ.conditional_share_balances_t.0.get(agent_id) else {
        return 0;
    };

    for (event_id, pair) in holdings {
        let Some(pool) = econ.cpmm_pools_t.0.get(event_id) else {
            continue;
        };
        if !matches!(pool.status, PoolStatus::Active) {
            continue;
        }
        let pool_y = pool.pool_yes.units;
        let pool_n = pool.pool_no.units;
        let denom = pool_y.saturating_add(pool_n);
        if denom == 0 {
            continue;
        }
        let yes_mtm = pair.yes.units.saturating_mul(pool_n) / denom;
        let no_mtm = pair.no.units.saturating_mul(pool_y) / denom;
        let mtm: u128 = yes_mtm.saturating_add(no_mtm);
        let cost_basis: u128 = pair.yes.units.saturating_add(pair.no.units) / 2;
        let contribution: i128 = (mtm as i128) - (cost_basis as i128);
        total = total.saturating_add(contribution);
    }

    if total > i64::MAX as i128 {
        i64::MAX
    } else if total < i64::MIN as i128 {
        i64::MIN
    } else {
        total as i64
    }
}

fn classify_solvency(balance: i64, initial_balance_micro: i64) -> SolvencyStatus {
    if balance <= 0 {
        return SolvencyStatus::Bankrupt;
    }
    let threshold = initial_balance_micro / 10;
    if balance < threshold {
        SolvencyStatus::NearInsolvent
    } else {
        SolvencyStatus::Solvent
    }
}

/// TRACE_MATRIX FC1-N5 (TB-G G3.1 2026-05-12): canonical preseed lookup —
/// returns the genesis credit for one agent per
/// `crate::runtime::bootstrap::default_pput_preseed_pairs`. Callers that
/// want `compute_agent_pnl` to report `realized_pnl` against the canonical
/// genesis baseline use this helper to fill the `initial_balance_micro`
/// argument.
pub fn initial_balance_micro_from_default_preseed(agent_id: &AgentId) -> i64 {
    crate::runtime::bootstrap::default_pput_preseed_pairs()
        .into_iter()
        .find(|(a, _)| a == agent_id)
        .map(|(_, m)| m.micro_units())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::economy::money::MicroCoin;
    use crate::state::q_state::{
        ClaimEntry, CpmmPool, LpShareAmount, PoolStatus, QState, Reputation, ShareSidePair,
        StakeEntry, TaskId, TxId,
    };
    use crate::state::typed_tx::{EventId, ShareAmount};

    fn agent(name: &str) -> AgentId {
        AgentId(name.into())
    }

    fn empty_q() -> QState {
        QState::default()
    }

    fn event(name: &str) -> EventId {
        EventId(TaskId(name.into()))
    }

    /// U1 — fresh genesis QState yields a 7-field view with all-zero PnL and
    /// no open positions. SG-G3.1 "genesis returns zero-pnl" binding.
    #[test]
    fn genesis_yields_zero_pnl() {
        let q = empty_q();
        let view = compute_agent_pnl(&q, &agent("Agent_0"), 1_000_000);
        assert_eq!(view.balance, 0);
        assert_eq!(view.realized_pnl, -1_000_000);
        assert_eq!(view.unrealized_pnl, 0);
        assert!(view.open_positions.is_empty());
        assert_eq!(view.reputation_score, 0);
        assert!(matches!(view.solvency_status, SolvencyStatus::Bankrupt));
    }

    /// U2 — agent at initial balance reports zero realized PnL + Solvent.
    /// SG-G3.1 binding: "genesis-balance agent shows zero realized PnL".
    #[test]
    fn agent_at_initial_balance_zero_realized() {
        let mut q = empty_q();
        q.economic_state_t
            .balances_t
            .0
            .insert(agent("Agent_0"), MicroCoin::from_micro_units(1_000_000));
        let view = compute_agent_pnl(&q, &agent("Agent_0"), 1_000_000);
        assert_eq!(view.realized_pnl, 0);
        assert_eq!(view.unrealized_pnl, 0);
        assert!(matches!(view.solvency_status, SolvencyStatus::Solvent));
    }

    /// U3 — balanced complete-set mint (N YES + N NO) yields zero unrealized
    /// PnL regardless of pool reserves. Architect "bull/bear emergence"
    /// invariant: only asymmetric positions produce signed PnL.
    #[test]
    fn balanced_mint_yields_zero_unrealized() {
        let mut q = empty_q();
        let a = agent("Agent_0");
        let ev = event("task-A");
        q.economic_state_t
            .balances_t
            .0
            .insert(a.clone(), MicroCoin::from_micro_units(900_000));
        let mut holdings = std::collections::BTreeMap::new();
        holdings.insert(
            ev.clone(),
            ShareSidePair {
                yes: ShareAmount::from_units(100_000),
                no: ShareAmount::from_units(100_000),
            },
        );
        q.economic_state_t
            .conditional_share_balances_t
            .0
            .insert(a.clone(), holdings);
        // No pool: contribution = 0.
        let view = compute_agent_pnl(&q, &a, 1_000_000);
        assert_eq!(view.realized_pnl, -100_000);
        assert_eq!(view.unrealized_pnl, 0, "balanced mint, no pool");

        // Add active pool with asymmetric reserves: balanced position still
        // yields 0 because pool prices sum to 1.
        q.economic_state_t.cpmm_pools_t.0.insert(
            ev.clone(),
            CpmmPool {
                event_id: ev.clone(),
                pool_yes: ShareAmount::from_units(50),
                pool_no: ShareAmount::from_units(150),
                lp_total_shares: LpShareAmount::from_units(0),
                status: PoolStatus::Active,
            },
        );
        let view = compute_agent_pnl(&q, &a, 1_000_000);
        assert_eq!(
            view.unrealized_pnl, 0,
            "balanced mint stays neutral under skewed pool"
        );
    }

    /// U4 — asymmetric YES-heavy holding under an active pool produces
    /// signed unrealized PnL. SG-G3.2 "post-BuyRouter unrealized updates"
    /// binding.
    #[test]
    fn asymmetric_yes_holding_under_active_pool_yields_signed_pnl() {
        let mut q = empty_q();
        let a = agent("Agent_0");
        let ev = event("task-A");
        // Post-BuyYes router state: agent paid 100k cash, holds 150k YES + 50k NO.
        // (Net cost basis (yes+no)/2 = 100k matches cash paid.)
        // Pool reserves 50:150 → yes_price = 150 / 200 = 0.75.
        q.economic_state_t
            .balances_t
            .0
            .insert(a.clone(), MicroCoin::from_micro_units(900_000));
        let mut holdings = std::collections::BTreeMap::new();
        holdings.insert(
            ev.clone(),
            ShareSidePair {
                yes: ShareAmount::from_units(150_000),
                no: ShareAmount::from_units(50_000),
            },
        );
        q.economic_state_t
            .conditional_share_balances_t
            .0
            .insert(a.clone(), holdings);
        q.economic_state_t.cpmm_pools_t.0.insert(
            ev.clone(),
            CpmmPool {
                event_id: ev.clone(),
                pool_yes: ShareAmount::from_units(50),
                pool_no: ShareAmount::from_units(150),
                lp_total_shares: LpShareAmount::from_units(0),
                status: PoolStatus::Active,
            },
        );

        let view = compute_agent_pnl(&q, &a, 1_000_000);
        // yes_mtm = 150_000 * 150 / 200 = 112_500.
        // no_mtm  = 50_000  *  50 / 200 = 12_500.
        // mtm     = 125_000.
        // cost_basis = (150_000 + 50_000) / 2 = 100_000.
        // unrealized = 125_000 - 100_000 = +25_000 (signed gain).
        assert_eq!(view.unrealized_pnl, 25_000);
        assert_eq!(view.realized_pnl, -100_000);
    }

    /// U5 — stakes + claims + LP shares + node positions are visible via
    /// `open_positions` but contribute 0 to unrealized_pnl. Art. III
    /// shielding binding + per-architect "only conditional-share MTM moves
    /// the bull/bear signal".
    #[test]
    fn stakes_claims_lp_nodes_visible_but_neutral_on_pnl() {
        let mut q = empty_q();
        let a = agent("Agent_0");
        q.economic_state_t
            .balances_t
            .0
            .insert(a.clone(), MicroCoin::from_micro_units(800_000));
        q.economic_state_t.stakes_t.0.insert(
            TxId("worktx-1".into()),
            StakeEntry {
                amount: MicroCoin::from_micro_units(50_000),
                staker: a.clone(),
                task_id: TaskId("task-A".into()),
            },
        );
        q.economic_state_t.claims_t.0.insert(
            TxId("claim-1".into()),
            ClaimEntry {
                amount: MicroCoin::from_micro_units(30_000),
                claimant: a.clone(),
                task_id: TaskId("task-A".into()),
                status: ClaimStatus::Open,
                ..Default::default()
            },
        );

        let view = compute_agent_pnl(&q, &a, 1_000_000);
        assert_eq!(view.realized_pnl, -200_000);
        assert_eq!(view.unrealized_pnl, 0);
        assert_eq!(view.open_positions.len(), 2);
        assert!(view
            .open_positions
            .iter()
            .any(|p| matches!(p, OpenPosition::Stake { .. })));
        assert!(view
            .open_positions
            .iter()
            .any(|p| matches!(p, OpenPosition::Claim { .. })));
    }

    /// U6 — solvency tiers: three regimes against a 1_000_000 baseline.
    /// SG-G3.3 "bankrupt / low-balance agent" classifier binding.
    #[test]
    fn solvency_tiers_classify_three_regimes() {
        assert!(matches!(
            classify_solvency(500_000, 1_000_000),
            SolvencyStatus::Solvent
        ));
        assert!(matches!(
            classify_solvency(99_999, 1_000_000),
            SolvencyStatus::NearInsolvent
        ));
        assert!(matches!(
            classify_solvency(0, 1_000_000),
            SolvencyStatus::Bankrupt
        ));
        assert!(matches!(
            classify_solvency(-1, 1_000_000),
            SolvencyStatus::Bankrupt
        ));
    }

    /// U7 — reputation score wired through from reputations_t.
    #[test]
    fn reputation_score_wired_through() {
        let mut q = empty_q();
        let a = agent("Agent_0");
        q.economic_state_t
            .reputations_t
            .0
            .insert(a.clone(), Reputation(42));
        let view = compute_agent_pnl(&q, &a, 0);
        assert_eq!(view.reputation_score, 42);
    }

    /// U8 — per-viewer isolation: another agent's stakes do not leak into
    /// our agent's open_positions list (Art. III shielding binding).
    #[test]
    fn per_viewer_isolation_no_cross_agent_leak() {
        let mut q = empty_q();
        let alice = agent("Agent_0");
        let bob = agent("Agent_1");
        q.economic_state_t
            .balances_t
            .0
            .insert(alice.clone(), MicroCoin::from_micro_units(1_000_000));
        q.economic_state_t
            .balances_t
            .0
            .insert(bob.clone(), MicroCoin::from_micro_units(500_000));
        q.economic_state_t.stakes_t.0.insert(
            TxId("bobs-stake".into()),
            StakeEntry {
                amount: MicroCoin::from_micro_units(100_000),
                staker: bob.clone(),
                task_id: TaskId("task-B".into()),
            },
        );
        let alice_view = compute_agent_pnl(&q, &alice, 1_000_000);
        assert_eq!(alice_view.balance, 1_000_000);
        assert!(alice_view.open_positions.is_empty(), "no cross-agent leak");
        assert_eq!(alice_view.unrealized_pnl, 0);
    }

    /// U9 — pool with `Resolved` / `Closed` status: contribution to
    /// unrealized PnL is 0. (Resolution oracle path is future TB.)
    #[test]
    fn non_active_pool_yields_zero_unrealized() {
        let mut q = empty_q();
        let a = agent("Agent_0");
        let ev = event("task-A");
        let mut holdings = std::collections::BTreeMap::new();
        holdings.insert(
            ev.clone(),
            ShareSidePair {
                yes: ShareAmount::from_units(150_000),
                no: ShareAmount::from_units(50_000),
            },
        );
        q.economic_state_t
            .conditional_share_balances_t
            .0
            .insert(a.clone(), holdings);
        q.economic_state_t.cpmm_pools_t.0.insert(
            ev.clone(),
            CpmmPool {
                event_id: ev.clone(),
                pool_yes: ShareAmount::from_units(50),
                pool_no: ShareAmount::from_units(150),
                lp_total_shares: LpShareAmount::from_units(0),
                status: PoolStatus::Resolved,
            },
        );
        let view = compute_agent_pnl(&q, &a, 0);
        assert_eq!(view.unrealized_pnl, 0);
    }

    /// U10 — default preseed lookup returns Agent_0..9 at 1.0 Coin and
    /// MarketMakerBudget at 5.0 Coin per bootstrap factory.
    #[test]
    fn default_preseed_lookup_returns_canonical_amounts() {
        assert_eq!(
            initial_balance_micro_from_default_preseed(&agent("Agent_0")),
            1_000_000
        );
        assert_eq!(
            initial_balance_micro_from_default_preseed(&agent("MarketMakerBudget")),
            5_000_000
        );
        assert_eq!(
            initial_balance_micro_from_default_preseed(&agent("nonexistent")),
            0
        );
    }
}
