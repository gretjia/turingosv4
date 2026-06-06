//! A09 settlement projection.
//!
//! Settlement eligibility is derived from a predicate receipt and escrow
//! availability. Price is carried only as an observed signal and never changes
//! payout eligibility.

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::Cid;
use crate::economy::money::MicroCoin;
use crate::runtime::predicate_receipt::PredicateReceipt;
use crate::state::price_index::RationalPrice;
use crate::state::q_state::{AgentId, TaskId, TxId};
use crate::state::typed_tx::{ClaimId, FinalizeRewardTx};

/// TRACE_MATRIX FC1-N11/N12: settlement outcome status derived from predicate receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum SettlementStatus {
    Eligible = 0,
    Blocked = 1,
}

/// TRACE_MATRIX FC1-N11/N12: fail-closed reasons for non-payable settlement projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum SettlementBlockReason {
    MissingPredicateReceipt = 0,
    PredicateFailed = 1,
    SubjectMismatch = 2,
    EscrowInsufficient = 3,
}

/// TRACE_MATRIX FC1-N11/N12 + FC2-N28: non-authoritative settlement projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementProjection {
    pub tx_id: TxId,
    pub claim_id: ClaimId,
    pub task_id: TaskId,
    pub solver: AgentId,
    pub reward: MicroCoin,
    pub payout_amount: MicroCoin,
    pub predicate_receipt_cid: Option<Cid>,
    pub observed_price: Option<RationalPrice>,
    pub eligible: bool,
    pub status: SettlementStatus,
    pub block_reason: Option<SettlementBlockReason>,
}

/// TRACE_MATRIX FC1-N11/N12: derive payout eligibility without consulting price.
pub fn settlement_projection_from_receipt(
    tx: &FinalizeRewardTx,
    receipt: Option<&PredicateReceipt>,
    predicate_receipt_cid: Option<Cid>,
    available_escrow: MicroCoin,
    observed_price: Option<RationalPrice>,
) -> Result<SettlementProjection, SettlementError> {
    let block_reason = match receipt {
        None => Some(SettlementBlockReason::MissingPredicateReceipt),
        Some(receipt) if receipt.subject_tx_id != *tx.claim_id.as_tx_id() => {
            Some(SettlementBlockReason::SubjectMismatch)
        }
        Some(receipt) if !receipt.result => Some(SettlementBlockReason::PredicateFailed),
        Some(_) if available_escrow < tx.reward => Some(SettlementBlockReason::EscrowInsufficient),
        Some(_) => None,
    };

    let (eligible, status, payout_amount) = match block_reason {
        None => (true, SettlementStatus::Eligible, tx.reward),
        Some(_) => (false, SettlementStatus::Blocked, MicroCoin::zero()),
    };

    Ok(SettlementProjection {
        tx_id: tx.tx_id.clone(),
        claim_id: tx.claim_id.clone(),
        task_id: tx.task_id.clone(),
        solver: tx.solver.clone(),
        reward: tx.reward,
        payout_amount,
        predicate_receipt_cid,
        observed_price,
        eligible,
        status,
        block_reason,
    })
}

/// TRACE_MATRIX FC1-N11/N12: settlement projection failure domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettlementError {
    InvalidReward { tx_id: TxId, reward: MicroCoin },
}

impl std::fmt::Display for SettlementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidReward { tx_id, reward } => {
                write!(f, "invalid reward for {}: {}", tx_id.0, reward)
            }
        }
    }
}

impl std::error::Error for SettlementError {}
