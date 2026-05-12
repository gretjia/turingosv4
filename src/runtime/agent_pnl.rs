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

// ────────────────────────────────────────────────────────────────────────
// TB-G G3.4 — §G PnL trajectory dashboard section + path-based wrapper.
//
// Charter: handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md
// §1 Module G3 atom G3.4.
//
// Directive: handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md
// §G3 SG-G3.5 "PnL is visible in dashboard as materialized view".
// ────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N7 + §15 + §17 (TB-G G3.4 2026-05-12): per-agent
/// trajectory row in the `## §G PnL trajectory` dashboard block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PnlTrajectoryRow {
    pub agent_id: AgentId,
    pub initial_balance_micro: i64,
    pub current_balance_micro: i64,
    pub realized_pnl: i64,
    pub unrealized_pnl: i64,
    pub solvency_status: SolvencyStatus,
    pub reputation_score: i64,
    pub open_position_count: usize,
}

impl PnlTrajectoryRow {
    /// TRACE_MATRIX FC1-N5 (TB-G G3.4 2026-05-12): `true` when the
    /// agent has zero PnL movement AND zero open positions AND zero
    /// reputation — i.e. the agent took no economic action across the
    /// batch. Used by the silent-zero-forbidden contract in
    /// `PnlTrajectorySection::render_section_g`.
    pub fn is_flat(&self) -> bool {
        self.realized_pnl == 0
            && self.unrealized_pnl == 0
            && self.open_position_count == 0
            && self.reputation_score == 0
    }
}

/// TRACE_MATRIX FC1-N7 + §15 + §17 (TB-G G3.4 2026-05-12): the §G
/// PnL trajectory section payload — per-agent rows over the canonical
/// preseed agent set plus an aggregate `all_flat` flag for the
/// silent-zero-forbidden contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PnlTrajectorySection {
    pub rows: Vec<PnlTrajectoryRow>,
    /// `true` when every row is flat (no PnL movement, no open positions).
    /// Triggers the MECHANISM BOTTLENECK explainer at render time.
    pub all_flat: bool,
}

impl PnlTrajectorySection {
    /// TRACE_MATRIX FC1-N5 (TB-G G3.4 2026-05-12; G-Phase directive §G3
    /// SG-G3.5 "PnL is visible in dashboard as materialized view"):
    /// pure walker — iterates the canonical preseed agent registry
    /// (`runtime::bootstrap::default_pput_preseed_pairs`) and computes a
    /// `PnlTrajectoryRow` for each.
    ///
    /// **Why preseed list, not balances_t.keys()**: a bankrupt agent
    /// may have been debited to balance 0 and removed from BalancesIndex
    /// via opportunistic cleanup; the preseed list is the authoritative
    /// roster of identities the dashboard should report on (matches
    /// architect's "13-agent persistence" framing).
    pub fn compute_from_q(q: &QState) -> Self {
        let pairs = crate::runtime::bootstrap::default_pput_preseed_pairs();
        let mut rows: Vec<PnlTrajectoryRow> = Vec::with_capacity(pairs.len());
        for (agent_id, initial) in &pairs {
            let initial_micro = initial.micro_units();
            let view = compute_agent_pnl(q, agent_id, initial_micro);
            rows.push(PnlTrajectoryRow {
                agent_id: agent_id.clone(),
                initial_balance_micro: initial_micro,
                current_balance_micro: view.balance,
                realized_pnl: view.realized_pnl,
                unrealized_pnl: view.unrealized_pnl,
                solvency_status: view.solvency_status,
                reputation_score: view.reputation_score,
                open_position_count: view.open_positions.len(),
            });
        }
        let all_flat = rows.iter().all(|r| r.is_flat());
        Self { rows, all_flat }
    }

    /// TRACE_MATRIX FC1-N7 (TB-G G3.4 2026-05-12; G-Phase directive §G3
    /// SG-G3.5 + charter §1 Module G3 atom G3.4 dashboard render
    /// contract).
    ///
    /// Render the `## §G PnL trajectory` block consumed by
    /// `audit_dashboard --run-report`. Includes an explicit
    /// MECHANISM BOTTLENECK explainer with ≥3 candidate causes when
    /// `all_flat == true` — silent zero forbidden per
    /// `feedback_no_workarounds_strict_constitution`.
    pub fn render_section_g(&self) -> String {
        let mut out = String::new();
        out.push_str("\n## §G PnL trajectory\n");
        out.push_str("  (per-agent realized/unrealized PnL over the batch; ");
        out.push_str("integer-rational μC; cost basis 1 μC/share-pair)\n");
        for row in &self.rows {
            let solvency_label = match row.solvency_status {
                SolvencyStatus::Solvent => "solvent",
                SolvencyStatus::NearInsolvent => "near_insolvent",
                SolvencyStatus::Bankrupt => "bankrupt",
            };
            out.push_str(&format!(
                "  - {agent}: balance={bal} μC (initial {init}); \
                 realized={rpnl}; unrealized={upnl}; \
                 positions={pos}; rep={rep}; {solv}\n",
                agent = row.agent_id.0,
                bal = row.current_balance_micro,
                init = row.initial_balance_micro,
                rpnl = row.realized_pnl,
                upnl = row.unrealized_pnl,
                pos = row.open_position_count,
                rep = row.reputation_score,
                solv = solvency_label,
            ));
        }
        if self.all_flat {
            out.push_str("  MECHANISM BOTTLENECK (architect §G3 SG-G3.5 / Drucker \
                          framing unmet — every agent shows flat PnL):\n");
            out.push_str("    1. No BuyWithCoinRouter activity — no router buy means\n");
            out.push_str("       no asymmetric share position means no signed unrealized\n");
            out.push_str("       PnL. G5.1 opportunity scheduler + 7-action menu is the\n");
            out.push_str("       canonical forward fix (round-robin scheduler under-\n");
            out.push_str("       samples invest path; same shape as G2P §F.X bottleneck).\n");
            out.push_str("    2. No accepted WorkTx → no stakes locked → no realized PnL\n");
            out.push_str("       movement. Confirm `BatchContinuationManifest.task_count`\n");
            out.push_str("       and preseed `TURINGOS_CHAINTAPE_PRESEED=1` so agents\n");
            out.push_str("       have stake budget across the persistent batch.\n");
            out.push_str("    3. No reputation accumulation in any sequencer arm —\n");
            out.push_str("       `reputations_t` mutation is forward-bound to G3.2 \n");
            out.push_str("       Class-4 sequencer admission (OBS_G2P_VERIFY_PEER_REWARD\n");
            out.push_str("       Gap-A). Until G3.2 ships, reputation_score will stay 0.\n");
        }
        out
    }
}

/// TRACE_MATRIX FC1-N7 (TB-G G3.4 2026-05-12): error wrapper for the
/// path-based `compute_pnl_trajectory_from_paths` entry point.
#[derive(Debug)]
pub enum PnlTrajectoryError {
    LedgerOpen(String),
    CasOpen(String),
    PinnedPubkeysIo(String),
    PinnedPubkeysParse(String),
    InitialQStateIo(String),
    InitialQStateParse(String),
    LedgerRead(String),
    Replay(String),
    HexDecode(String),
}

impl std::fmt::Display for PnlTrajectoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LedgerOpen(e) => write!(f, "open ledger: {e}"),
            Self::CasOpen(e) => write!(f, "open CAS: {e}"),
            Self::PinnedPubkeysIo(e) => write!(f, "read pinned_pubkeys.json: {e}"),
            Self::PinnedPubkeysParse(e) => write!(f, "parse pinned_pubkeys: {e}"),
            Self::InitialQStateIo(e) => write!(f, "read initial_q_state.json: {e}"),
            Self::InitialQStateParse(e) => write!(f, "parse initial_q_state: {e}"),
            Self::LedgerRead(e) => write!(f, "read ledger entry: {e}"),
            Self::Replay(e) => write!(f, "replay_full_transition: {e}"),
            Self::HexDecode(e) => write!(f, "hex decode: {e}"),
        }
    }
}

impl std::error::Error for PnlTrajectoryError {}

/// TRACE_MATRIX FC1-N7 (TB-G G3.4 2026-05-12; G-Phase directive §G3
/// SG-G3.5 + charter §1 Module G3 atom G3.4 dual-bind via SG-G1.7
/// one-continuous-ChainTape): path-based entry point for the
/// `audit_dashboard --run-report` §G section + the
/// `tests/constitution_g3_pnl_trajectory_evidence_binding.rs` dual-
/// binding test.
///
/// Performs a fresh `replay_full_transition` over the L4 chain rooted
/// at `runtime_repo_path` (loads `pinned_pubkeys.json` +
/// `initial_q_state.json` per the FC2 Boot replay contract), then walks
/// the canonical preseed agent registry to emit per-agent
/// `PnlTrajectoryRow`s.
pub fn compute_pnl_trajectory_from_paths(
    runtime_repo_path: &std::path::Path,
    cas_path: &std::path::Path,
) -> Result<PnlTrajectorySection, PnlTrajectoryError> {
    use crate::bottom_white::cas::store::CasStore;
    use crate::bottom_white::ledger::system_keypair::{
        PinnedSystemPubkeys, SystemEpoch, SystemPublicKey,
    };
    use crate::bottom_white::ledger::transition_ledger::{
        replay_full_transition, Git2LedgerWriter, LedgerCasView, LedgerEntry, LedgerWriter,
        ReplayError,
    };
    use crate::top_white::predicates::registry::PredicateRegistry;
    use crate::bottom_white::tools::registry::ToolRegistry;

    let pinned_path = runtime_repo_path.join("pinned_pubkeys.json");
    let pinned_text = std::fs::read_to_string(&pinned_path)
        .map_err(|e| PnlTrajectoryError::PinnedPubkeysIo(format!("{pinned_path:?}: {e}")))?;
    let pinned_manifest: crate::runtime::PinnedPubkeyManifest = serde_json::from_str(&pinned_text)
        .map_err(|e| PnlTrajectoryError::PinnedPubkeysParse(e.to_string()))?;
    let mut pinned = PinnedSystemPubkeys::new();
    for entry in &pinned_manifest.pubkeys {
        let bytes = decode_hex_32(&entry.pubkey_hex)
            .map_err(PnlTrajectoryError::HexDecode)?;
        pinned.insert(SystemEpoch::new(entry.epoch), SystemPublicKey::from_bytes(bytes));
    }

    let initial_q_path = runtime_repo_path.join("initial_q_state.json");
    let initial_q: QState = if initial_q_path.exists() {
        let s = std::fs::read_to_string(&initial_q_path)
            .map_err(|e| PnlTrajectoryError::InitialQStateIo(e.to_string()))?;
        serde_json::from_str::<QState>(&s)
            .map_err(|e| PnlTrajectoryError::InitialQStateParse(e.to_string()))?
    } else {
        QState::genesis()
    };

    let writer = Git2LedgerWriter::open(runtime_repo_path)
        .map_err(|e| PnlTrajectoryError::LedgerOpen(format!("{e:?}")))?;
    let cas = CasStore::open(cas_path)
        .map_err(|e| PnlTrajectoryError::CasOpen(e.to_string()))?;

    let chain_len = writer.len();
    let mut entries: Vec<LedgerEntry> = Vec::with_capacity(chain_len as usize);
    for t in 1..=chain_len {
        let entry = writer
            .read_at(t)
            .map_err(|e| PnlTrajectoryError::LedgerRead(format!("{e:?}")))?;
        entries.push(entry);
    }

    struct CasRef<'a>(&'a CasStore);
    impl<'a> LedgerCasView for CasRef<'a> {
        fn get_typed_payload(
            &self,
            cid: &crate::bottom_white::cas::schema::Cid,
        ) -> Result<Vec<u8>, ReplayError> {
            self.0.get(cid).map_err(|_| ReplayError::CasMissing { at: 0 })
        }
    }
    let cas_view = CasRef(&cas);
    let predicate_registry = PredicateRegistry::new();
    let tool_registry = ToolRegistry::new();
    let final_q = replay_full_transition(
        &initial_q,
        &entries,
        &cas_view,
        &pinned,
        &predicate_registry,
        &tool_registry,
    )
    .map_err(|e| PnlTrajectoryError::Replay(format!("{e:?}")))?;

    Ok(PnlTrajectorySection::compute_from_q(&final_q))
}

fn decode_hex_32(hex: &str) -> Result<[u8; 32], String> {
    let h = hex.trim();
    if h.len() != 64 {
        return Err(format!("expected 64 hex chars, got {}", h.len()));
    }
    let mut out = [0u8; 32];
    for i in 0..32 {
        let byte = u8::from_str_radix(&h[2 * i..2 * i + 2], 16)
            .map_err(|e| format!("hex parse at {i}: {e}"))?;
        out[i] = byte;
    }
    Ok(out)
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
