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
