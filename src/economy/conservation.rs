//! A09 conservation projection.
//!
//! This reducer checks the derived economy projection for zero-sum money
//! movement over a tape prefix. It does not replace the production
//! `monetary_invariant` holding-list reducer.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::bottom_white::ledger::transition_ledger::{canonical_encode, CanonicalCodecError};
use crate::economy::money::MicroCoin;
use crate::economy::projections::EconomyProjection;
use crate::state::q_state::Hash;

/// TRACE_MATRIX 基本法 1 + Art.0.2: derived conservation report for one projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EconomyConservationReport {
    pub derived_from_tape_head: String,
    pub last_applied_logical_t: u64,
    pub wallet_delta_micro: i64,
    pub escrow_micro: i64,
    pub open_position_micro: i64,
    pub conditional_collateral_micro: i64,
    pub minted_total_micro: i64,
    pub total_supply_delta_micro: i64,
    pub conservation_root: Hash,
}

/// TRACE_MATRIX 基本法 1 + Art.0.2: derive the conservation report from projection fields.
pub fn conservation_report_from_projection(
    projection: &EconomyProjection,
) -> Result<EconomyConservationReport, EconomyConservationError> {
    let wallet_delta_micro = sum_microcoins(
        projection.wallet_balances.values().copied(),
        "wallet balance delta",
    )?;
    let escrow_micro = sum_microcoins(
        projection.escrows.values().map(|entry| entry.amount),
        "escrow projection",
    )?;
    let open_position_micro = sum_microcoins(
        projection.node_positions.values().map(|entry| entry.amount),
        "open position projection",
    )?;
    let conditional_collateral_micro = sum_microcoins(
        projection.conditional_collateral.values().copied(),
        "conditional collateral projection",
    )?;
    let minted_total_micro = 0;
    let total_supply_delta_micro = checked_sum_i64(
        &[
            wallet_delta_micro,
            escrow_micro,
            open_position_micro,
            conditional_collateral_micro,
            -minted_total_micro,
        ],
        "total supply delta",
    )?;
    let root_payload = ConservationRootPayload {
        derived_from_tape_head: projection.derived_from_tape_head.clone(),
        last_applied_logical_t: projection.last_applied_logical_t,
        wallet_delta_micro,
        escrow_micro,
        open_position_micro,
        conditional_collateral_micro,
        minted_total_micro,
        total_supply_delta_micro,
    };
    let conservation_root = hash_root_payload(&root_payload)?;

    Ok(EconomyConservationReport {
        derived_from_tape_head: projection.derived_from_tape_head.clone(),
        last_applied_logical_t: projection.last_applied_logical_t,
        wallet_delta_micro,
        escrow_micro,
        open_position_micro,
        conditional_collateral_micro,
        minted_total_micro,
        total_supply_delta_micro,
        conservation_root,
    })
}

/// TRACE_MATRIX 基本法 1 + Art.0.2: fail unless the projection is conserved.
pub fn assert_projection_conserved(
    projection: &EconomyProjection,
) -> Result<(), EconomyConservationError> {
    let report = conservation_report_from_projection(projection)?;
    if report.total_supply_delta_micro != 0 {
        return Err(EconomyConservationError::SupplyDelta {
            total_supply_delta_micro: report.total_supply_delta_micro,
        });
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ConservationRootPayload {
    derived_from_tape_head: String,
    last_applied_logical_t: u64,
    wallet_delta_micro: i64,
    escrow_micro: i64,
    open_position_micro: i64,
    conditional_collateral_micro: i64,
    minted_total_micro: i64,
    total_supply_delta_micro: i64,
}

fn sum_microcoins<I>(values: I, context: &'static str) -> Result<i64, EconomyConservationError>
where
    I: IntoIterator<Item = MicroCoin>,
{
    let mut total = 0i64;
    for value in values {
        total = total.checked_add(value.micro_units()).ok_or_else(|| {
            EconomyConservationError::Overflow {
                context: context.to_string(),
            }
        })?;
    }
    Ok(total)
}

fn checked_sum_i64(values: &[i64], context: &'static str) -> Result<i64, EconomyConservationError> {
    let mut total = 0i64;
    for value in values {
        total = total
            .checked_add(*value)
            .ok_or_else(|| EconomyConservationError::Overflow {
                context: context.to_string(),
            })?;
    }
    Ok(total)
}

fn hash_root_payload(payload: &ConservationRootPayload) -> Result<Hash, EconomyConservationError> {
    let bytes = canonical_encode(payload)?;
    Ok(Hash::from_bytes(Sha256::digest(bytes).into()))
}

/// TRACE_MATRIX 基本法 1 + Art.0.2: conservation projection failure domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EconomyConservationError {
    Overflow { context: String },
    Codec(String),
    SupplyDelta { total_supply_delta_micro: i64 },
}

impl From<CanonicalCodecError> for EconomyConservationError {
    fn from(value: CanonicalCodecError) -> Self {
        Self::Codec(value.to_string())
    }
}

impl std::fmt::Display for EconomyConservationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overflow { context } => write!(f, "integer overflow in {context}"),
            Self::Codec(e) => write!(f, "codec error: {e}"),
            Self::SupplyDelta {
                total_supply_delta_micro,
            } => write!(
                f,
                "economy projection not conserved: total_supply_delta_micro={total_supply_delta_micro}"
            ),
        }
    }
}

impl std::error::Error for EconomyConservationError {}
