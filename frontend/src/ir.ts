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
// Spec interview + generate types (W5 wire contract; consumed by W6 frontend)
// ---------------------------------------------------------------------------

/** GET /api/spec/questions response. Always 8 questions in interview order. */
export interface SpecQuestionsResponse {
  questions: string[];
}

/** POST /api/spec/submit request body. `answers` must have length 8. */
export interface SpecSubmitRequest {
  answers: string[];
  session_id?: string;
}

/** POST /api/spec/submit success response. */
export interface SpecSubmitResponse {
  session_id: string;
  spec_md: string;
  capsule_cid?: string | null;
  transcript_jsonl?: string | null;
}

/** POST /api/generate request body. */
export interface GenerateRequest {
  session_id: string;
  from_capsule?: boolean;
  max_files?: number;
}

/** One artifact file entry as returned by the backend. */
export interface ArtifactEntry {
  /** Path relative to <session-dir>/artifacts/ (e.g. "index.html"). */
  path: string;
  /** File size in bytes. */
  size_bytes: number;
  /** MIME content type sniffed by extension. */
  content_type: string;
}

/** POST /api/generate success response. */
export interface GenerateResponse {
  session_id: string;
  artifacts: ArtifactEntry[];
  transcript_excerpt?: string | null;
  /**
   * W8: how many attempts were needed before the artifact passed
   * heuristic verification. 1 means single-shot success; >=2 means at
   * least one retry happened. Frontend can show
   * "✓ (经过 N 次尝试)" if total_attempts > 1.
   */
  total_attempts: number;
}

// ---------------------------------------------------------------------------
// W7: welcome / onboarding wire contract
// ---------------------------------------------------------------------------

/** Which onboarding step the wizard should show as active. */
export type NextStep =
  | 'Init'
  | 'LlmConfig'
  | 'ApiKey'
  | 'AgentDeploy'
  | 'Spec'
  | 'Generate'
  | 'Done';

/** Snapshot of welcome / onboarding status returned by `/api/welcome/status`. */
export interface OnboardingStatus {
  workspace_path: string;
  init_done: boolean;
  llm_config_done: boolean;
  /** Reflects AppState only; never reflects disk state (key is never persisted). */
  api_key_set: boolean;
  agents_count: number;
  spec_done: boolean;
  spec_capsule_cid: string | null;
  artifacts_done: boolean;
  next_step: NextStep;
}

/** POST /api/welcome/api-key request body. */
export interface ApiKeyRequest {
  api_key: string;
}

// ---------------------------------------------------------------------------
// WebSocket event shapes (W2 + W4 + W5 contract)
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

/** W5: spec_complete WS broadcast — emitted after POST /api/spec/submit succeeds. */
export interface SpecCompleteEvent {
  msg_type: 'spec_complete';
  session_id: string;
  capsule_cid?: string | null;
}

/** W5: generate_started WS broadcast — reserved (not yet emitted by backend). */
export interface GenerateStartedEvent {
  msg_type: 'generate_started';
  session_id: string;
}

/** W5: generate_complete WS broadcast — emitted after POST /api/generate succeeds. */
export interface GenerateCompleteEvent {
  msg_type: 'generate_complete';
  session_id: string;
  artifacts: string[];
}

/**
 * W8: generate_attempt_started — emitted at the start of each retry
 * attempt (including attempt 1). Drives the live "尝试 N/M" progress chip.
 */
export interface GenerateAttemptStartedEvent {
  msg_type: 'generate_attempt_started';
  session_id: string;
  attempt: number;
  max_attempts: number;
}

/**
 * W8: generate_attempt_failed — emitted when an attempt fails heuristic
 * verification or shellout exit. `reason` is human-readable.
 */
export interface GenerateAttemptFailedEvent {
  msg_type: 'generate_attempt_failed';
  session_id: string;
  attempt: number;
  max_attempts: number;
  reason: string;
}

// ---------------------------------------------------------------------------
// W8 driven-mode grill WS events (Phase 6.3.x)
// ---------------------------------------------------------------------------

/**
 * W8: SpecTurnAdvanced — emitted after a grill turn is accepted.
 * Optimistic UI: usually redundant with the POST /api/spec/turn response.
 */
export interface SpecTurnAdvancedEvent {
  msg_type: 'SpecTurnAdvanced';
  session_id: string;
  turn_index: number;
  question_text: string;
}

/**
 * W8: SpecGrillComplete — emitted when the grill session terminates and
 * the spec capsule has been written to CAS.
 */
export interface SpecGrillCompleteEvent {
  msg_type: 'SpecGrillComplete';
  session_id: string;
  spec_capsule_cid: string;
}

/**
 * W8: SpecTurnTriageReject — emitted by the W4.5 blackbox triage atom
 * when the user answer is classified as off_topic, abusive, or gibberish
 * (R2 §A5). Frontend displays a nudge; the turn is NOT advanced.
 */
export interface SpecTurnTriageRejectEvent {
  msg_type: 'SpecTurnTriageReject';
  session_id: string;
  turn_index: number;
  /** One of "off_topic" | "abusive" | "gibberish". */
  triage_class: string;
  non_relevant_count: number;
}

/**
 * Union of all WebSocket message shapes.
 *
 * Discriminated on `msg_type`:
 *   - `'ir_update'`:                initial IR push or view refresh
 *   - `'task_created'`:             write-path event from POST /api/task/open
 *   - `'spec_complete'`:            POST /api/spec/submit success broadcast
 *   - `'generate_started'`:         reserved for future streaming
 *   - `'generate_complete'`:        POST /api/generate success broadcast
 *   - `'generate_attempt_started'`: W8 — retry attempt start
 *   - `'generate_attempt_failed'`:  W8 — retry attempt failure
 *   - `'SpecTurnAdvanced'`:         W8 driven-mode — turn accepted (optimistic)
 *   - `'SpecGrillComplete'`:        W8 driven-mode — session complete with CID
 *   - `'SpecTurnTriageReject'`:     W8 driven-mode — answer triage-rejected
 */
export type WsMessage =
  | IRUpdateEvent
  | TaskCreatedEvent
  | SpecCompleteEvent
  | GenerateStartedEvent
  | GenerateCompleteEvent
  | GenerateAttemptStartedEvent
  | GenerateAttemptFailedEvent
  | SpecTurnAdvancedEvent
  | SpecGrillCompleteEvent
  | SpecTurnTriageRejectEvent;
