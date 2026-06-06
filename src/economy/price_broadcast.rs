//! A09 price broadcast.
//!
//! This is a read-only view over an `EconomyProjection` and carries the tape
//! head watermark that produced the price index.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::economy::projections::EconomyProjection;
use crate::state::price_index::NodeMarketEntry;
use crate::state::q_state::TxId;

/// TRACE_MATRIX Art.0.2 + FC2-N28: tape-prefix-bound price broadcast.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PriceBroadcast {
    pub derived_from_tape_head: String,
    pub last_applied_logical_t: u64,
    pub price_index: BTreeMap<TxId, NodeMarketEntry>,
}

/// TRACE_MATRIX Art.0.2 + FC2-N28: expose price without mutating economy state.
pub fn price_broadcast_from_projection(projection: &EconomyProjection) -> PriceBroadcast {
    PriceBroadcast {
        derived_from_tape_head: projection.derived_from_tape_head.clone(),
        last_applied_logical_t: projection.last_applied_logical_t,
        price_index: projection.price_index.clone(),
    }
}
