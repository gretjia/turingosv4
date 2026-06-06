//! A09 economy event envelope.
//!
//! This is a derived event shape over A05 `TapeEventEnvelope` and decoded
//! `TypedTx` payloads. It is not a writer and it is not an economy ledger.

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::Cid;
use crate::runtime::tape_event::{TapeEventEnvelope, TapeEventError};
use crate::state::q_state::TxId;
use crate::state::typed_tx::TypedTx;

/// TRACE_MATRIX FC2-N34: Generic CAS schema id for optional economy projection receipts.
pub const ECONOMY_EVENT_SCHEMA_ID: &str = "turingosv4.economy_event.v1";

/// TRACE_MATRIX Art.0.2 + FC2-N21/N28: economy-relevant accepted transition classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum EconomyEventKind {
    WorkSubmitted = 0,
    ChallengeSubmitted = 1,
    TaskMarketOpened = 2,
    EscrowLocked = 3,
    MarketSeeded = 4,
    CpmmPoolCreated = 5,
    RouterBuy = 6,
    EventResolved = 7,
    RewardFinalized = 8,
}

/// TRACE_MATRIX Art.0.2 + FC2-N21/N28: replay-only economy event projected from tape/CAS.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EconomyEvent {
    pub event_id: String,
    pub tape_head_oid: String,
    pub logical_t: u64,
    pub tx_id: TxId,
    pub event_kind: EconomyEventKind,
    pub payload_cid: Cid,
    pub predicate_receipt_cid: Option<Cid>,
}

/// TRACE_MATRIX Art.0.2: derive the A09 economy event metadata from canonical tape.
pub fn economy_event_from_tape(
    event: &TapeEventEnvelope,
    tx: &TypedTx,
    predicate_receipt_cid: Option<Cid>,
) -> Result<Option<EconomyEvent>, TapeEventError> {
    event.validate()?;
    let Some((tx_id, event_kind)) = economy_tx_id_and_kind(tx) else {
        return Ok(None);
    };
    let payload_cid = event
        .payload_cid
        .expect("TapeEventEnvelope::validate checked accepted payload");
    let tape_head_oid = event.tape_ref.head_oid_hex().to_string();
    Ok(Some(EconomyEvent {
        event_id: format!("{}:{}:{}", tape_head_oid, event.logical_t, tx_id.0),
        tape_head_oid,
        logical_t: event.logical_t,
        tx_id,
        event_kind,
        payload_cid,
        predicate_receipt_cid,
    }))
}

fn economy_tx_id_and_kind(tx: &TypedTx) -> Option<(TxId, EconomyEventKind)> {
    match tx {
        TypedTx::Work(tx) => Some((tx.tx_id.clone(), EconomyEventKind::WorkSubmitted)),
        TypedTx::Challenge(tx) => Some((tx.tx_id.clone(), EconomyEventKind::ChallengeSubmitted)),
        TypedTx::FinalizeReward(tx) => Some((tx.tx_id.clone(), EconomyEventKind::RewardFinalized)),
        TypedTx::TaskOpen(tx) => Some((tx.tx_id.clone(), EconomyEventKind::TaskMarketOpened)),
        TypedTx::EscrowLock(tx) => Some((tx.tx_id.clone(), EconomyEventKind::EscrowLocked)),
        TypedTx::MarketSeed(tx) => Some((tx.tx_id.clone(), EconomyEventKind::MarketSeeded)),
        TypedTx::CpmmPool(tx) => Some((tx.tx_id.clone(), EconomyEventKind::CpmmPoolCreated)),
        TypedTx::BuyWithCoinRouter(tx) => Some((tx.tx_id.clone(), EconomyEventKind::RouterBuy)),
        TypedTx::EventResolve(tx) => Some((tx.tx_id.clone(), EconomyEventKind::EventResolved)),
        _ => None,
    }
}
