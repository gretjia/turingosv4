// TRACE_MATRIX FC1-N5 + FC1-N10: Phase 6.3.x W8 — driven-mode type contracts.
//
// Mirrors the Rust grill_envelope.rs wire format and the W5 /api/spec/turn
// HTTP contract. These are frontend-only types; they are NOT authoritative
// over ChainTape/CAS (FC3-N31). When the Rust wire schema changes, update
// this file to stay in sync.

/**
 * TurnPayload — mirror of Rust src/runtime/grill_envelope.rs::TurnPayload.
 * The JSON envelope the LLM emits per turn.
 */
export interface TurnPayload {
    turn: number;
    question: string | null;
    covered_slots: string[];
    open_slots: string[];
    confidence: number;
    done: boolean;
    rationale: string;
    playback?: string;
}

/**
 * Request body for POST /api/spec/turn.
 */
export interface TurnRequest {
    session_id: string;
    user_answer: string | null;
    lang?: 'zh' | 'en';
}

/**
 * Response body from POST /api/spec/turn.
 */
export interface TurnResponse {
    turn_index: number;
    question_text: string | null;
    covered_slots: string[];
    open_slots: string[];
    confidence: number;
    done: boolean;
    playback: string | null;
    terminated: boolean;
    spec_capsule_cid: string | null;
    turn_capsule_cid: string;
    /**
     * Present only on triage bounce-back responses. Values: "off_topic" |
     * "gibberish" | "abusive". When set, question_text repeats the previous
     * question. Use this to render nudge text without depending on the WS
     * SpecTurnTriageReject event.
     */
    triage_class?: string | null;
}

// ---------------------------------------------------------------------------
// Polymarket PR1: agent attempt + market view types
// ---------------------------------------------------------------------------

/**
 * One candidate agent's admission record as returned by
 * GET /api/market/by-session/<session_id>.
 * Strictly read-only — no betting UI (user-decision #3).
 */
export interface AgentCandidateView {
  agent_id: string;
  proposal_cid: string;
  stake_micro: number;
  l4_state: 'accepted' | 'rejected' | 'pending_dispatch';
  rejection_class: string | null;
  predicate_results: Record<string, boolean>;
  yes_signal_bp: number | null;
  is_winner: boolean;
}

export interface RouterTradeView {
  buyer: string;
  direction: 'buy_yes' | 'buy_no';
  pay_coin_micro: number;
}

/**
 * Top-level response from GET /api/market/by-session/<session_id>.
 */
export interface MarketViewResponse {
  session_id: string;
  task_id: string;
  market_state: 'open' | 'finalized' | 'all_rejected';
  treasury_bounty_micro: number;
  candidates: AgentCandidateView[];
  router_trades: RouterTradeView[];
  buy_yes_count: number;
  buy_no_count: number;
  winner_agent_id: string | null;
}

/**
 * One DERIVED-EVIDENCE progress marker from GET /api/progress/by-session/<id>.
 * NOT canonical — the live cursor that the committed market tree replaces.
 */
export interface ProgressEvent {
  session_id: string;
  stage: 'worker_start' | 'worker_done' | 'market_settled';
  agent: string;
  artifact_cid: string | null;
  t_unix_ms: number;
}

/** Top-level response from GET /api/progress/by-session/<session_id>. */
export interface ProgressViewResponse {
  session_id: string;
  events: ProgressEvent[];
}

/**
 * Frontend driven-mode state machine.
 */
export type GrillState =
    | { kind: 'idle' }
    | { kind: 'awaiting_first_turn' }
    | { kind: 'awaiting_user_answer'; turn_index: number; question: string }
    | { kind: 'playback_review'; playback: string; session_id: string }
    | { kind: 'complete'; spec_capsule_cid: string };
