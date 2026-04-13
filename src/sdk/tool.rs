// Tier 2: TuringTool trait — SKILL lifecycle hooks
// Constitutional basis: Art. V.1 (three powers separation at tool level)
// V3 lessons: V3L-39 (encode constraints to tools, not prompts)

use std::any::Any;

/// Signal returned by tool hooks to control the bus lifecycle.
/// V3L-39: constraints are ENCODED here, not in prompts.
#[derive(Debug, Clone)]
pub enum ToolSignal {
    /// No objection, proceed with append.
    Pass,
    /// Reject the action with reason. V3L-09: always give explicit reason.
    Veto(String),
    /// Accept with reward signal (for market pricing).
    YieldReward { reward: f64 },
    /// Route to investment (buy YES/NO on existing node, skip append).
    InvestOnly {
        target_node: String,
        amount: f64,
        direction: BetDirection,
    },
}

/// Direction of a market bet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BetDirection {
    Long,  // Buy YES
    Short, // Buy NO
}

/// The SKILL lifecycle trait for all tools.
///
/// Bus calls these hooks in order:
/// on_boot → on_init → [on_pre_append → on_post_append]* → on_halt
///
/// Tools encode constraints that would be ineffective as prompt rules (V3L-39).
pub trait TuringTool: Send + Sync {
    /// Tool identifier.
    fn manifest(&self) -> &str;

    /// Called once at system boot.
    fn on_boot(&mut self) {}

    /// Called once per problem/experiment initialization.
    /// `agent_ids`: list of participating agents.
    fn on_init(&mut self, _agent_ids: &[String]) {}

    /// Called before each append. Return Veto to reject.
    /// This is where constitutional constraints are ENFORCED (not suggested).
    fn on_pre_append(&mut self, _author: &str, _payload: &str) -> ToolSignal {
        ToolSignal::Pass
    }

    /// Called after successful append.
    fn on_post_append(&mut self, _author: &str, _node_id: &str) {}

    /// Called at system halt (settlement).
    fn on_halt(&mut self, _golden_path: &[String]) {}

    /// Query tool state. Returns None if key not recognized.
    /// V3L-09: explicit None with meaning, not silent failure.
    fn query_state(&self, _key: &str) -> Option<String> {
        None
    }

    /// Downcast support.
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
