//! REAL-6A — TaskOutcomeMarket evidence helpers.
//!
//! TaskOutcomeMarket moves the market earlier: the event is the task-level
//! proposition "this task will be solved within budget/deadline". This module
//! carries replayable public metadata and view helpers only; all economic state
//! changes still go through typed tx admission (`MarketSeedTx` + `CpmmPoolTx`).

use serde::{Deserialize, Serialize};

use crate::economy::money::MicroCoin;
use crate::runtime::real5_roles::PriceSignal;
use crate::state::q_state::{TaskId, TxId};
use crate::state::typed_tx::EventId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TaskOutcomeMarketKind {
    #[default]
    TaskOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskOutcomeEvent {
    pub kind: TaskOutcomeMarketKind,
    pub event_id: EventId,
    pub task_id: TaskId,
    pub deadline_round: u64,
    pub max_budget: MicroCoin,
    pub created_by_task_open_tx: TxId,
}

pub fn task_outcome_event_for_task(
    event_id: impl Into<String>,
    task_id: TaskId,
    deadline_round: u64,
    max_budget: MicroCoin,
    created_by_task_open_tx: TxId,
) -> TaskOutcomeEvent {
    let event_raw = event_id.into();
    TaskOutcomeEvent {
        kind: TaskOutcomeMarketKind::TaskOutcome,
        event_id: EventId(TaskId(event_raw)),
        task_id,
        deadline_round,
        max_budget,
        created_by_task_open_tx,
    }
}

pub fn task_outcome_price_signal(
    event: &TaskOutcomeEvent,
    price: impl Into<String>,
    depth: Option<i64>,
) -> PriceSignal {
    PriceSignal {
        event_id: event.event_id.0 .0.clone(),
        price: price.into(),
        depth,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskOutcomeMarketSeedOutcome {
    pub event_id: EventId,
    pub market_seed_tx_id: TxId,
    pub cpmm_pool_tx_id: TxId,
    pub post_pool_state_root_hex: String,
}
