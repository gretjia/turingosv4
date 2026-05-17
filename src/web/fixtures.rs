/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Compile-time fixture loader for the three UI IR sample fixtures.
/// Fixtures are embedded at compile time via `include_str!` so the binary
/// is fully self-contained; no runtime file I/O is required for read endpoints.
///
/// All items are `pub(crate)` — no public API leaks from this module.
use super::ir::IRRoot;

/// Raw JSON for the dashboard fixture, embedded at compile time.
const DASHBOARD_JSON: &str =
    include_str!("../../experiments/tisr_ui_spike/fixtures/dashboard_sample.json");

/// Raw JSON for the agent-view fixture, embedded at compile time.
const AGENT_VIEW_JSON: &str =
    include_str!("../../experiments/tisr_ui_spike/fixtures/agent_view_sample.json");

/// Raw JSON for the task-view fixture, embedded at compile time.
const TASK_VIEW_JSON: &str =
    include_str!("../../experiments/tisr_ui_spike/fixtures/task_view_sample.json");

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Returns the parsed dashboard IR fixture.
/// Panics on parse failure — fixture data is an invariant of the build.
pub(crate) fn dashboard() -> IRRoot {
    serde_json::from_str(DASHBOARD_JSON).expect("fixture invariant: dashboard_sample.json")
}

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Returns the parsed agent-view IR fixture.
/// Panics on parse failure — fixture data is an invariant of the build.
pub(crate) fn agent_view() -> IRRoot {
    serde_json::from_str(AGENT_VIEW_JSON).expect("fixture invariant: agent_view_sample.json")
}

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Returns the parsed task-view IR fixture.
/// Panics on parse failure — fixture data is an invariant of the build.
pub(crate) fn task_view() -> IRRoot {
    serde_json::from_str(TASK_VIEW_JSON).expect("fixture invariant: task_view_sample.json")
}
