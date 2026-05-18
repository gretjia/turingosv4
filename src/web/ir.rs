/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Rust types matching `experiments/tisr_ui_spike/ui_ir_schema.json`.
/// These types represent the TuringOS UI Intermediate Representation (IR) —
/// a materialized view derived from ChainTape/CAS. Never authoritative over
/// ChainTape/CAS (FC3-N31).
///
/// All items are `pub(crate)` — no public API leaks from this module.
use serde::{Deserialize, Serialize};

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Top-level page IR. Contains an ordered list of content blocks.
/// Corresponds to the root object in `ui_ir_schema.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct IRRoot {
    /// Stable identifier for this page view (e.g. `"dashboard:2026-05-17"`).
    pub(crate) id: String,
    /// Human-readable title displayed at the top of the rendered view.
    pub(crate) title: String,
    /// Ordered list of content blocks composing this page.
    pub(crate) blocks: Vec<Block>,
}

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// A single content unit within a page. The `kind` field is the discriminant.
/// Uses adjacently-tagged serde representation to match the JSON schema's
/// flat-object shape (id + kind + block-type-specific fields all at top level).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum Block {
    /// Plain prose or status paragraph.
    Text(TextBlock),
    /// Grid of typed cells organized as rows × columns.
    Table(TableBlock),
    /// Summary card for a single registered agent.
    AgentCard(AgentCardBlock),
    /// Summary card for a single proof task.
    TaskCard(TaskCardBlock),
    /// Ordered list of tape events for audit display.
    EventLog(EventLogBlock),
    /// Named key-value metric panel (solve rate, PPUT, attempt counts, etc.).
    DashboardPanel(DashboardPanelBlock),
}

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Plain prose or status paragraph block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TextBlock {
    /// Block-scoped identifier, unique within the page.
    pub(crate) id: String,
    /// Prose content to display. May contain newlines.
    pub(crate) content: String,
}

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Grid of typed cells organized as rows × columns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TableBlock {
    /// Block-scoped identifier, unique within the page.
    pub(crate) id: String,
    /// Optional table caption shown above the grid.
    #[serde(default)]
    pub(crate) caption: Option<String>,
    /// Column header labels in order.
    pub(crate) columns: Vec<String>,
    /// Data rows. Each row is an array of Cell objects matching column count.
    pub(crate) rows: Vec<Vec<Cell>>,
}

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Summary card for a single registered agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AgentCardBlock {
    /// Block-scoped identifier, unique within the page.
    pub(crate) id: String,
    /// Agent identity key (hex pubkey or human-readable mnemonic).
    pub(crate) agent_id: String,
    /// Agent role label as registered (e.g. `"ProofAgent"`, `"LibrarianAgent"`).
    pub(crate) role: String,
    /// Agent wallet balance in μCoin (integer; MUST NOT be float).
    pub(crate) balance_micro: u64,
    /// Current agent status label (e.g. `"active"`, `"paused"`, `"bankrupt"`).
    #[serde(default)]
    pub(crate) status: Option<String>,
}

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Summary card for a single proof task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TaskCardBlock {
    /// Block-scoped identifier, unique within the page.
    pub(crate) id: String,
    /// Canonical task transaction ID from ChainTape.
    pub(crate) task_id: String,
    /// Problem identifier (e.g. MiniF2F problem name).
    pub(crate) problem_id: String,
    /// Task lifecycle status.
    pub(crate) status: String,
    /// Task reward in μCoin (integer; 0 if not yet finalized).
    #[serde(default)]
    pub(crate) reward_micro: Option<u64>,
    /// Number of externalized LLM-Lean cycles recorded for this task.
    #[serde(default)]
    pub(crate) attempt_count: Option<u64>,
    /// Agent ID currently assigned, or null if unassigned.
    #[serde(default)]
    pub(crate) assigned_agent_id: Option<String>,
}

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Ordered list of tape events for audit display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EventLogBlock {
    /// Block-scoped identifier, unique within the page.
    pub(crate) id: String,
    /// Ordered tape events (L4 accepted or L4.E rejected).
    pub(crate) events: Vec<EventEntry>,
}

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Single tape event entry in an event log block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EventEntry {
    /// ChainTape transaction ID.
    pub(crate) tx_id: String,
    /// Transaction kind label (e.g. `"WorkTx"`, `"LeanFailed"`, `"TaskOpenTx"`).
    pub(crate) kind: String,
    /// Tape layer: `"L4"` (accepted) or `"L4E"` (rejected).
    pub(crate) layer: String,
    /// Short human-readable event summary for display.
    #[serde(default)]
    pub(crate) summary: Option<String>,
}

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Named key-value metric panel (solve rate, PPUT, attempt counts, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct DashboardPanelBlock {
    /// Block-scoped identifier, unique within the page.
    pub(crate) id: String,
    /// Panel heading shown above the metrics.
    pub(crate) panel_title: String,
    /// Ordered list of named metric entries.
    pub(crate) metrics: Vec<MetricEntry>,
}

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// A single named metric within a dashboard panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MetricEntry {
    /// Metric name (e.g. `"solve_rate"`, `"mean_pput"`, `"total_attempts"`).
    pub(crate) label: String,
    /// Metric value. May be string, integer, or float.
    pub(crate) value: MetricValue,
    /// Optional unit label (e.g. `"%"`, `"μC"`, `"tx"`, `"attempts"`).
    #[serde(default)]
    pub(crate) unit: Option<String>,
}

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Metric value variants — matches JSON schema `oneOf [string, integer, number]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum MetricValue {
    /// String metric value.
    Text(String),
    /// Integer metric value.
    Integer(i64),
    /// Floating-point metric value.
    Float(f64),
}

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// A single typed value within a table row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Cell {
    /// Cell type discriminant controlling display and validation.
    /// One of: `"string"`, `"integer"`, `"microcoin"`, `"agent_id"`,
    /// `"tx_id"`, `"cid"`.
    pub(crate) kind: String,
    /// Cell value. Type depends on kind.
    pub(crate) value: CellValue,
}

/// TRACE_MATRIX FC1-N5: read view materialization
///
/// Cell value — may be a JSON string or integer depending on `Cell::kind`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum CellValue {
    /// String cell value (used for string/agent_id/tx_id/cid kinds).
    Text(String),
    /// Integer cell value (used for integer/microcoin kinds).
    Integer(i64),
}

impl std::fmt::Display for CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellValue::Text(s) => write!(f, "{s}"),
            CellValue::Integer(n) => write!(f, "{n}"),
        }
    }
}

impl std::fmt::Display for MetricValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricValue::Text(s) => write!(f, "{s}"),
            MetricValue::Integer(n) => write!(f, "{n}"),
            MetricValue::Float(v) => write!(f, "{v}"),
        }
    }
}
