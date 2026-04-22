//! ReadTool — the `rtool` node of Art. IV mermaid.
//!
//! Constitutional basis:
//! - Art. IV    `⟨q_i, s_i⟩ = rtool(⟨q_t, tape_t, HEAD_t⟩)` — read-only
//!   projection of Q_t into the agent's input.
//! - Art. III.3 correlation shielding — per-agent filters live here, not
//!   in bus core (e.g., private_ticker hides own-node prices).
//! - Art. V.1   Generator ≠ Evaluator — ReadTool mutates nothing; all
//!   state changes flow through the separate WriteTool path.
//!
//! Phase Z (2026-04-22): introduced as a trait so agent-specific view
//! policies (Paper 2+ Art. III.3 enforcement, role-based filters) can
//! be plugged without forking bus.snapshot().

use crate::sdk::snapshot::UniverseSnapshot;

/// A read-only projection from bus state to an agent-visible snapshot.
/// The default implementation (`DefaultReadTool`) returns the same
/// snapshot for every caller — full visibility.
pub trait ReadTool: Send + Sync {
    /// Project bus state into `⟨q_i, s_i⟩` for a given agent.
    /// `agent_hint` is Some for agent-aware projection, None for
    /// tooling / audit callers who want the full snapshot.
    fn project(
        &self,
        bus: &crate::bus::TuringBus,
        agent_hint: Option<&str>,
    ) -> UniverseSnapshot;
}

/// Default policy: identity projection. Every agent sees the same
/// snapshot. Preserves current behavior until a per-agent filter
/// wants to override.
pub struct DefaultReadTool;

impl ReadTool for DefaultReadTool {
    fn project(
        &self,
        bus: &crate::bus::TuringBus,
        _agent_hint: Option<&str>,
    ) -> UniverseSnapshot {
        bus.snapshot()
    }
}
