// TRACE_MATRIX FC1-N5: read view materialization
//
// TypeScript mirror of src/web/ir.rs — TuringOS UI Intermediate Representation.
// These types represent the TuringOS UI IR materialized view. Never authoritative
// over ChainTape/CAS (FC3-N31).
//
// Serde mapping:
//   Block variants use #[serde(tag = "kind", rename_all = "snake_case")]
//   CellValue and MetricValue use #[serde(untagged)]

// ---------------------------------------------------------------------------
// Primitive value types (untagged unions)
// ---------------------------------------------------------------------------

/** Cell value — either a JSON string or integer depending on Cell.kind. */
export type CellValue = string | number;

/** Metric value — string, integer, or float. */
export type MetricValue = string | number;

// ---------------------------------------------------------------------------
// Supporting record types
// ---------------------------------------------------------------------------

/** A single typed value within a table row. */
export interface Cell {
  /** One of: "string" | "integer" | "microcoin" | "agent_id" | "tx_id" | "cid" */
  kind: string;
  /** Value depends on kind. */
  value: CellValue;
}

/** Single tape event entry in an event log block. */
export interface EventEntry {
  /** ChainTape transaction ID. */
  tx_id: string;
  /** Transaction kind label e.g. "WorkTx", "LeanFailed", "TaskOpenTx". */
  kind: string;
  /** Tape layer: "L4" (accepted) or "L4E" (rejected). */
  layer: string;
  /** Short human-readable event summary for display. */
  summary?: string;
}

/** A single named metric within a dashboard panel. */
export interface MetricEntry {
  /** Metric name e.g. "solve_rate", "mean_pput", "total_attempts". */
  label: string;
  /** Metric value. May be string, integer, or float. */
  value: MetricValue;
  /** Optional unit label e.g. "%", "μC", "tx", "attempts". */
  unit?: string;
}

// ---------------------------------------------------------------------------
// Block payload types (inner structs)
// ---------------------------------------------------------------------------

export interface TextBlock {
  id: string;
  /** Prose content to display. May contain newlines. */
  content: string;
}

export interface TableBlock {
  id: string;
  /** Optional table caption shown above the grid. */
  caption?: string;
  /** Column header labels in order. */
  columns: string[];
  /** Data rows. Each row is an array of Cell objects. */
  rows: Cell[][];
}

export interface AgentCardBlock {
  id: string;
  /** Agent identity key (hex pubkey or human-readable mnemonic). */
  agent_id: string;
  /** Agent role label e.g. "ProofAgent", "LibrarianAgent". */
  role: string;
  /** Agent wallet balance in μCoin (integer; MUST NOT be float). */
  balance_micro: number;
  /** Current agent status label e.g. "active", "paused", "bankrupt". */
  status?: string;
}

export interface TaskCardBlock {
  id: string;
  /** Canonical task transaction ID from ChainTape. */
  task_id: string;
  /** Problem identifier e.g. MiniF2F problem name. */
  problem_id: string;
  /** Task lifecycle status. */
  status: string;
  /** Task reward in μCoin (integer; 0 if not yet finalized). */
  reward_micro?: number;
  /** Number of externalized LLM-Lean cycles recorded for this task. */
  attempt_count?: number;
  /** Agent ID currently assigned, or null if unassigned. */
  assigned_agent_id?: string;
}

export interface EventLogBlock {
  id: string;
  /** Ordered tape events (L4 accepted or L4E rejected). */
  events: EventEntry[];
}

export interface DashboardPanelBlock {
  id: string;
  /** Panel heading shown above the metrics. */
  panel_title: string;
  /** Ordered list of named metric entries. */
  metrics: MetricEntry[];
}

// ---------------------------------------------------------------------------
// Block discriminated union — mirrors Rust #[serde(tag = "kind", rename_all = "snake_case")]
// ---------------------------------------------------------------------------

export type Block =
  | ({ kind: 'text' } & TextBlock)
  | ({ kind: 'table' } & TableBlock)
  | ({ kind: 'agent_card' } & AgentCardBlock)
  | ({ kind: 'task_card' } & TaskCardBlock)
  | ({ kind: 'event_log' } & EventLogBlock)
  | ({ kind: 'dashboard_panel' } & DashboardPanelBlock);

// ---------------------------------------------------------------------------
// IRRoot — top-level page IR
// ---------------------------------------------------------------------------

/** Top-level page IR. Contains an ordered list of content blocks. */
export interface IRRoot {
  /** Stable identifier for this page view e.g. "dashboard:2026-05-17". */
  id: string;
  /** Human-readable title displayed at the top of the rendered view. */
  title: string;
  /** Ordered list of content blocks composing this page. */
  blocks: Block[];
}

// ---------------------------------------------------------------------------
// WebSocket event shapes (W2 + W4 contract)
// ---------------------------------------------------------------------------

/** Shape of e.detail when "turingos:ir_update" fires from the W2 inline WS script. */
export interface IRUpdateEvent {
  msg_type: 'ir_update';
  view: 'dashboard' | 'agents' | 'tasks';
  ir: IRRoot;
}

/** Shape of a task_created broadcast message (W4). */
export interface TaskCreatedEvent {
  msg_type: 'task_created';
  task_id: string;
  agent_id: string;
  problem_id: string;
  bounty: number;
}

/**
 * Union of all WebSocket message shapes.
 *
 * Discriminated on `msg_type`:
 *   - `'ir_update'`:    initial IR push or view refresh
 *   - `'task_created'`: write-path event from POST /api/task/open
 */
export type WsMessage = IRUpdateEvent | TaskCreatedEvent;
