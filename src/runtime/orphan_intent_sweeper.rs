//! A06 boot-time orphan Intent sweeper.
//!
//! On boot, stale `PendingIntent` events must be closed on tape rather than
//! trusted through memory-only counters. The sweeper appends an Abandoned
//! `TerminalExternalCall` for each stale pending Intent and never reissues the
//! provider call.

use crate::bottom_white::cas::CasStore;
use crate::runtime::external_call::{ExternalCallError, ExternalCallRecorder, ExternalCallState};

/// TRACE_MATRIX Art.0.2 + FC2-N22: crash-recovery reason for orphan external calls closed during boot.
pub const OS_CRASH_RECOVERY: &str = "OS_CRASH_RECOVERY";

/// TRACE_MATRIX Art.0.2 + FC2-N22: boot orphan sweep settings derived from current ChainTape replay state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrphanSweepConfig {
    pub stale_at_or_before_logical_t: u64,
    pub may_have_spent: bool,
    pub creator: String,
}

/// TRACE_MATRIX Art.0.2 + FC2-N22: summary of Abandoned terminal rows appended by the orphan sweeper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrphanSweepReport {
    pub abandoned_call_ids: Vec<String>,
}

/// TRACE_MATRIX Art.0.2 + FC2-N22: close stale pending external-call Intents with Abandoned terminals.
pub fn sweep_orphan_external_call_intents(
    cas: &mut CasStore,
    recorder: &mut ExternalCallRecorder,
    config: OrphanSweepConfig,
) -> Result<OrphanSweepReport, ExternalCallError> {
    let state = ExternalCallState::derive_from_tape(recorder.records())?;
    let mut abandoned_call_ids = Vec::new();

    for intent in state.pending_intents() {
        if intent.logical_t <= config.stale_at_or_before_logical_t {
            recorder.record_abandoned_terminal(
                cas,
                intent,
                OS_CRASH_RECOVERY,
                config.may_have_spent,
                &config.creator,
            )?;
            abandoned_call_ids.push(intent.call_id.clone());
        }
    }

    Ok(OrphanSweepReport { abandoned_call_ids })
}
