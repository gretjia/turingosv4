//! A09 replay-only economy projection.
//!
//! The projection is fully derived from an A05 `TapeEventEnvelope` prefix plus
//! CAS payload bytes. It is not persisted as authority and it does not write
//! back to ChainTape, wallet, or market state.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::{CasError, CasStore};
use crate::bottom_white::ledger::transition_ledger::{
    canonical_decode, CanonicalCodecError, TxKind,
};
use crate::economy::events::{economy_event_from_tape, EconomyEvent};
use crate::economy::money::MicroCoin;
use crate::economy::settlement::{settlement_projection_from_receipt, SettlementProjection};
use crate::runtime::predicate_receipt::PredicateReceipt;
use crate::runtime::tape_event::{TapeEventEnvelope, TapeEventError, TapeEventKind};
use crate::state::price_index::{compute_price_index, NodeMarketEntry};
use crate::state::q_state::{
    AgentId, CpmmPool, EconomicState, EscrowEntry, Hash, LpShareAmount, ShareSidePair, TaskId,
    TaskMarketEntry, TaskMarketState, TxId,
};
use crate::state::typed_tx::{
    BuyDirection, EventId, NodePosition, OutcomeSide, PositionKind, PositionSide, ShareAmount,
    TypedTx,
};

/// TRACE_MATRIX Art.0.2 + FC2-N21/N28: stable id for the replay-only L6 economy projection.
pub const ECONOMY_PROJECTION_ID: &str = "economy.v0";
/// TRACE_MATRIX Art.0.2 + FC2-N21/N28: schema version for the replay-only L6 economy projection.
pub const ECONOMY_PROJECTION_VERSION: u32 = 0;

/// TRACE_MATRIX Art.0.2 + FC2-N21/N28: replay-only economy projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EconomyProjection {
    pub projection_id: String,
    pub projection_version: u32,
    pub derived_from_tape_head: String,
    pub last_applied_logical_t: u64,
    pub accepted_events: Vec<EconomyEvent>,
    pub wallet_balances: BTreeMap<AgentId, MicroCoin>,
    pub escrows: BTreeMap<TxId, EscrowEntry>,
    pub market_books: BTreeMap<TaskId, TaskMarketEntry>,
    pub node_positions: BTreeMap<TxId, NodePosition>,
    pub conditional_collateral: BTreeMap<EventId, MicroCoin>,
    pub share_balances: BTreeMap<AgentId, BTreeMap<EventId, ShareSidePair>>,
    pub cpmm_pools: BTreeMap<EventId, CpmmPool>,
    pub price_index: BTreeMap<TxId, NodeMarketEntry>,
    pub settlements: BTreeMap<TxId, SettlementProjection>,
    pub conservation_root: Hash,
}

impl EconomyProjection {
    /// TRACE_MATRIX Art.0.2: construct an empty read-view for a specific tape watermark.
    pub fn empty_for_tape_head(
        derived_from_tape_head: String,
        last_applied_logical_t: u64,
    ) -> Self {
        Self {
            projection_id: ECONOMY_PROJECTION_ID.to_string(),
            projection_version: ECONOMY_PROJECTION_VERSION,
            derived_from_tape_head,
            last_applied_logical_t,
            accepted_events: Vec::new(),
            wallet_balances: BTreeMap::new(),
            escrows: BTreeMap::new(),
            market_books: BTreeMap::new(),
            node_positions: BTreeMap::new(),
            conditional_collateral: BTreeMap::new(),
            share_balances: BTreeMap::new(),
            cpmm_pools: BTreeMap::new(),
            price_index: BTreeMap::new(),
            settlements: BTreeMap::new(),
            conservation_root: Hash::ZERO,
        }
    }
}

/// TRACE_MATRIX Art.0.2 + FC2-N21/N28: derive economy projection from tape prefix + CAS only.
pub fn derive_economy_projection(
    cas: &CasStore,
    events: &[TapeEventEnvelope],
) -> Result<EconomyProjection, EconomyProjectionError> {
    let mut projection = EconomyProjection::empty_for_tape_head(String::new(), 0);
    let mut work_task_by_tx: BTreeMap<TxId, TaskId> = BTreeMap::new();

    for event in events {
        event.validate()?;
        projection.derived_from_tape_head = event.tape_ref.head_oid_hex().to_string();
        projection.last_applied_logical_t = event.logical_t;

        if event.kind != TapeEventKind::AcceptedTransition {
            continue;
        }
        let payload_cid =
            event
                .payload_cid
                .ok_or(EconomyProjectionError::MissingAcceptedPayload {
                    logical_t: event.logical_t,
                })?;
        let bytes = cas.get(&payload_cid)?;
        let tx: TypedTx = canonical_decode(&bytes)?;
        if event.source_tx_kind != Some(tx.tx_kind()) {
            return Err(EconomyProjectionError::SourceKindMismatch {
                logical_t: event.logical_t,
                envelope: event.source_tx_kind,
                payload: tx.tx_kind(),
            });
        }

        if let Some(economy_event) = economy_event_from_tape(event, &tx, None)? {
            projection.accepted_events.push(economy_event);
        }
        apply_typed_tx(&mut projection, &mut work_task_by_tx, &tx, event.logical_t)?;
    }

    let econ = projection_to_economic_state(&projection);
    projection.price_index = compute_price_index(&econ);
    projection.conservation_root =
        crate::economy::conservation::conservation_report_from_projection(&projection)
            .map_err(|e| EconomyProjectionError::Conservation(e.to_string()))?
            .conservation_root;
    Ok(projection)
}

fn apply_typed_tx(
    projection: &mut EconomyProjection,
    work_task_by_tx: &mut BTreeMap<TxId, TaskId>,
    tx: &TypedTx,
    logical_t: u64,
) -> Result<(), EconomyProjectionError> {
    match tx {
        TypedTx::TaskOpen(tx) => {
            projection.market_books.insert(
                tx.task_id.clone(),
                TaskMarketEntry {
                    publisher: tx.sponsor_agent.clone(),
                    verifier_quorum: tx.verifier_quorum,
                    max_reuse_royalty_fraction_basis_points: tx
                        .max_reuse_royalty_fraction_basis_points,
                    settlement_rule_hash: tx.settlement_rule_hash,
                    opened_at_logical_t: tx.timestamp_logical,
                    ..TaskMarketEntry::default()
                },
            );
        }
        TypedTx::EscrowLock(tx) => {
            debit_wallet(projection, &tx.sponsor_agent, tx.amount)?;
            projection.escrows.insert(
                tx.tx_id.clone(),
                EscrowEntry {
                    amount: tx.amount,
                    depositor: tx.sponsor_agent.clone(),
                    task_id: tx.task_id.clone(),
                },
            );
            let market = projection
                .market_books
                .entry(tx.task_id.clone())
                .or_insert_with(TaskMarketEntry::default);
            market.total_escrow = checked_add_money(
                market.total_escrow,
                tx.amount,
                "task market escrow projection",
            )?;
            market.escrow_lock_tx_ids.insert(tx.tx_id.clone());
        }
        TypedTx::Work(tx) => {
            let stake = tx.stake.as_micro_coin();
            debit_wallet(projection, &tx.agent_id, stake)?;
            work_task_by_tx.insert(tx.tx_id.clone(), tx.task_id.clone());
            projection.node_positions.insert(
                tx.tx_id.clone(),
                NodePosition {
                    position_id: tx.tx_id.clone(),
                    node_id: tx.tx_id.clone(),
                    task_id: tx.task_id.clone(),
                    owner: tx.agent_id.clone(),
                    side: PositionSide::Long,
                    kind: PositionKind::FirstLong,
                    amount: stake,
                    source_tx: tx.tx_id.clone(),
                    opened_at_round: logical_t,
                },
            );
        }
        TypedTx::Challenge(tx) => {
            let stake = tx.stake.as_micro_coin();
            debit_wallet(projection, &tx.challenger_agent, stake)?;
            let task_id = work_task_by_tx
                .get(&tx.target_work_tx)
                .cloned()
                .unwrap_or_default();
            projection.node_positions.insert(
                tx.tx_id.clone(),
                NodePosition {
                    position_id: tx.tx_id.clone(),
                    node_id: tx.target_work_tx.clone(),
                    task_id,
                    owner: tx.challenger_agent.clone(),
                    side: PositionSide::Short,
                    kind: PositionKind::ChallengeShort,
                    amount: stake,
                    source_tx: tx.tx_id.clone(),
                    opened_at_round: logical_t,
                },
            );
        }
        TypedTx::MarketSeed(tx) => {
            debit_wallet(projection, &tx.provider, tx.collateral_amount)?;
            add_collateral(projection, &tx.event_id, tx.collateral_amount)?;
            add_share_pair(
                projection,
                &tx.provider,
                &tx.event_id,
                ShareAmount::from_units(micro_to_share_units(tx.collateral_amount)?),
                ShareAmount::from_units(micro_to_share_units(tx.collateral_amount)?),
            )?;
        }
        TypedTx::CpmmPool(tx) => {
            subtract_share_pair(
                projection,
                &tx.provider,
                &tx.event_id,
                tx.seed_yes,
                tx.seed_no,
            )?;
            projection.cpmm_pools.insert(
                tx.event_id.clone(),
                CpmmPool {
                    event_id: tx.event_id.clone(),
                    pool_yes: tx.seed_yes,
                    pool_no: tx.seed_no,
                    lp_total_shares: LpShareAmount::from_units(tx.seed_yes.units),
                    status: crate::state::q_state::PoolStatus::Active,
                },
            );
        }
        TypedTx::BuyWithCoinRouter(tx) => {
            debit_wallet(projection, &tx.buyer, tx.pay_coin)?;
            add_collateral(projection, &tx.event_id, tx.pay_coin)?;
            let pay_units = micro_to_share_units(tx.pay_coin)?;
            let retained = ShareAmount::from_units(pay_units);
            match tx.direction {
                BuyDirection::BuyYes => {
                    add_share_pair(
                        projection,
                        &tx.buyer,
                        &tx.event_id,
                        retained,
                        ShareAmount::zero(),
                    )?;
                }
                BuyDirection::BuyNo => {
                    add_share_pair(
                        projection,
                        &tx.buyer,
                        &tx.event_id,
                        ShareAmount::zero(),
                        retained,
                    )?;
                }
            }
        }
        TypedTx::EventResolve(tx) => {
            let state = match tx.outcome {
                OutcomeSide::Yes => TaskMarketState::Finalized,
                OutcomeSide::No => TaskMarketState::Bankrupt,
            };
            let market = projection
                .market_books
                .entry(tx.task_id.clone())
                .or_insert_with(TaskMarketEntry::default);
            market.state = state;
            if state == TaskMarketState::Bankrupt {
                market.bankruptcy_at_logical_t = Some(logical_t);
            }
        }
        TypedTx::FinalizeReward(tx) => {
            let available_escrow = projection
                .market_books
                .get(&tx.task_id)
                .map(|m| m.total_escrow)
                .unwrap_or_else(MicroCoin::zero);
            let settlement = settlement_projection_from_receipt(
                tx,
                None::<&PredicateReceipt>,
                None,
                available_escrow,
                None,
            )
            .map_err(|e| EconomyProjectionError::Settlement(e.to_string()))?;
            projection.settlements.insert(tx.tx_id.clone(), settlement);
        }
        _ => {}
    }
    Ok(())
}

fn projection_to_economic_state(projection: &EconomyProjection) -> EconomicState {
    let mut econ = EconomicState::default();
    econ.node_positions_t.0 = projection.node_positions.clone();
    econ.conditional_share_balances_t.0 = projection.share_balances.clone();
    econ.cpmm_pools_t.0 = projection.cpmm_pools.clone();
    econ
}

fn debit_wallet(
    projection: &mut EconomyProjection,
    agent: &AgentId,
    amount: MicroCoin,
) -> Result<(), EconomyProjectionError> {
    let delta = MicroCoin::zero()
        .checked_sub(amount)
        .ok_or_else(|| EconomyProjectionError::MoneyOverflow("wallet debit".into()))?;
    add_wallet_delta(projection, agent, delta)
}

fn add_wallet_delta(
    projection: &mut EconomyProjection,
    agent: &AgentId,
    delta: MicroCoin,
) -> Result<(), EconomyProjectionError> {
    let current = projection
        .wallet_balances
        .get(agent)
        .copied()
        .unwrap_or_else(MicroCoin::zero);
    let next = checked_add_money(current, delta, "wallet delta projection")?;
    projection.wallet_balances.insert(agent.clone(), next);
    Ok(())
}

fn add_collateral(
    projection: &mut EconomyProjection,
    event_id: &EventId,
    amount: MicroCoin,
) -> Result<(), EconomyProjectionError> {
    let current = projection
        .conditional_collateral
        .get(event_id)
        .copied()
        .unwrap_or_else(MicroCoin::zero);
    let next = checked_add_money(current, amount, "conditional collateral projection")?;
    projection
        .conditional_collateral
        .insert(event_id.clone(), next);
    Ok(())
}

fn add_share_pair(
    projection: &mut EconomyProjection,
    owner: &AgentId,
    event_id: &EventId,
    yes: ShareAmount,
    no: ShareAmount,
) -> Result<(), EconomyProjectionError> {
    let pair = projection
        .share_balances
        .entry(owner.clone())
        .or_default()
        .entry(event_id.clone())
        .or_default();
    pair.yes = ShareAmount::from_units(
        pair.yes
            .units
            .checked_add(yes.units)
            .ok_or_else(|| EconomyProjectionError::ShareOverflow("YES share add".into()))?,
    );
    pair.no = ShareAmount::from_units(
        pair.no
            .units
            .checked_add(no.units)
            .ok_or_else(|| EconomyProjectionError::ShareOverflow("NO share add".into()))?,
    );
    Ok(())
}

fn subtract_share_pair(
    projection: &mut EconomyProjection,
    owner: &AgentId,
    event_id: &EventId,
    yes: ShareAmount,
    no: ShareAmount,
) -> Result<(), EconomyProjectionError> {
    let pair = projection
        .share_balances
        .entry(owner.clone())
        .or_default()
        .entry(event_id.clone())
        .or_default();
    pair.yes = ShareAmount::from_units(
        pair.yes
            .units
            .checked_sub(yes.units)
            .ok_or_else(|| EconomyProjectionError::ShareUnderflow("YES share subtract".into()))?,
    );
    pair.no = ShareAmount::from_units(
        pair.no
            .units
            .checked_sub(no.units)
            .ok_or_else(|| EconomyProjectionError::ShareUnderflow("NO share subtract".into()))?,
    );
    Ok(())
}

fn checked_add_money(
    lhs: MicroCoin,
    rhs: MicroCoin,
    context: &'static str,
) -> Result<MicroCoin, EconomyProjectionError> {
    lhs.checked_add(rhs)
        .ok_or_else(|| EconomyProjectionError::MoneyOverflow(context.to_string()))
}

fn micro_to_share_units(amount: MicroCoin) -> Result<u128, EconomyProjectionError> {
    let micro = amount.micro_units();
    if micro < 0 {
        return Err(EconomyProjectionError::NegativeShareSource { amount });
    }
    Ok(micro as u128)
}

/// TRACE_MATRIX Art.0.2 + FC2-N21/N28: projection failure domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EconomyProjectionError {
    TapeEvent(TapeEventError),
    MissingAcceptedPayload {
        logical_t: u64,
    },
    Cas(String),
    Codec(String),
    SourceKindMismatch {
        logical_t: u64,
        envelope: Option<TxKind>,
        payload: TxKind,
    },
    MoneyOverflow(String),
    ShareOverflow(String),
    ShareUnderflow(String),
    NegativeShareSource {
        amount: MicroCoin,
    },
    Settlement(String),
    Conservation(String),
}

impl From<TapeEventError> for EconomyProjectionError {
    fn from(value: TapeEventError) -> Self {
        Self::TapeEvent(value)
    }
}

impl From<CasError> for EconomyProjectionError {
    fn from(value: CasError) -> Self {
        Self::Cas(value.to_string())
    }
}

impl From<CanonicalCodecError> for EconomyProjectionError {
    fn from(value: CanonicalCodecError) -> Self {
        Self::Codec(value.to_string())
    }
}

impl std::fmt::Display for EconomyProjectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TapeEvent(e) => write!(f, "invalid tape event: {e}"),
            Self::MissingAcceptedPayload { logical_t } => {
                write!(f, "accepted event at logical_t={logical_t} missing payload")
            }
            Self::Cas(e) => write!(f, "cas error: {e}"),
            Self::Codec(e) => write!(f, "codec error: {e}"),
            Self::SourceKindMismatch {
                logical_t,
                envelope,
                payload,
            } => write!(
                f,
                "tx kind mismatch at logical_t={logical_t}: envelope={envelope:?}, payload={payload:?}"
            ),
            Self::MoneyOverflow(context) => write!(f, "money overflow in {context}"),
            Self::ShareOverflow(context) => write!(f, "share overflow in {context}"),
            Self::ShareUnderflow(context) => write!(f, "share underflow in {context}"),
            Self::NegativeShareSource { amount } => {
                write!(f, "negative coin amount cannot mint shares: {amount}")
            }
            Self::Settlement(e) => write!(f, "settlement error: {e}"),
            Self::Conservation(e) => write!(f, "conservation error: {e}"),
        }
    }
}

impl std::error::Error for EconomyProjectionError {}
