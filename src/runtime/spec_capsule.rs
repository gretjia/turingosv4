//! TRACE_MATRIX FC2-N16 + FC3 evidence binding: turingos spec CAS wire
//!
//! Phase 6.3 closes the spec → CAS wire. A completed `turingos spec` grill
//! produces three artifacts:
//!
//!   1. `spec.md`             — human-readable spec (the seven-row "fridge
//!                              note" + EARS + GWT + Never sections)
//!   2. `spec_transcript.jsonl` — every LLM turn (system / user / assistant)
//!                              with timestamp, model, usage tokens.
//!   3. EvidenceCapsule in `cas/` — bytes of spec.md anchored by sha256;
//!                              schema_id = "turingos-spec-capsule-v1";
//!                              recorded in `.turingos_cas_index.jsonl`.
//!
//! The capsule CID becomes the auditable proof that `turingos spec` actually
//! ran — `turingos welcome` reads the CID from the CAS index to flip the
//! "spec done" status from `[ ]` to `[x]`.
//!
//! This is Class 2 production wire-up: uses the existing `turingosv4::
//! bottom_white::cas::store::CasStore` public surface. No Class 4 schema
//! change. ObjectType::EvidenceCapsule + schema_id tag keep the spec
//! capsule cleanly separable from any other EvidenceCapsule the rest of
//! the kernel might emit on the same workspace.
//!
//! A6 LIBRARY-IZATION (2026-05-19):
//! Module was relocated from `src/bin/turingos/spec_capsule.rs` (binary-only)
//! to `src/runtime/spec_capsule.rs` (library) so the `turingos_web` binary can
//! synthesise spec capsules in-process at predicate-pass + done=true. Pre-move
//! this surface was visibility-trapped in the `turingos` CLI binary; the web
//! handler could only shell out (deleted in F6) or surface
//! `termination_reason: "predicate_done_no_spec_pending_synthesis"`. The fix
//! is a pure move + `pub(crate) → pub` visibility promotion; no schema-id,
//! no on-wire byte change, no Class 4 surface touched.

use std::path::Path;
use std::process::ExitCode;

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::CasStore;

/// TRACE_MATRIX FC2-N16 + FC3-N4 (CAS evidence binding):
/// Schema-id tag for spec capsules — lets `welcome` find them in the index
/// without scanning bytes. Versioned so a future format bump (e.g. a binary
/// canonical encoding) can coexist with the v1 markdown form.
pub const SPEC_CAPSULE_SCHEMA_ID: &str = "turingos-spec-capsule-v1";

/// TRACE_MATRIX FC2-N16: error taxonomy for spec-capsule CAS operations.
#[derive(Debug)]
pub enum CapsuleError {
    /// CAS store could not be opened (e.g. workspace/cas missing or libgit2 error).
    Open(String),
    /// CAS put failed.
    Put(String),
    /// Reading existing capsules from the sidecar index failed.
    Read(String),
}

impl std::fmt::Display for CapsuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open(e) => write!(f, "CAS open: {e}"),
            Self::Put(e) => write!(f, "CAS put: {e}"),
            Self::Read(e) => write!(f, "CAS read: {e}"),
        }
    }
}

/// TRACE_MATRIX FC2-N16: workspace-local CAS path resolver.
///
/// Resolve the per-workspace CAS path. `<workspace>/cas/` is created by
/// `turingos init`. If `cas/` doesn't yet exist as a directory, CasStore::open
/// will create it via git2 Repository::init (which creates the dir + .git/).
pub fn cas_path(workspace: &Path) -> std::path::PathBuf {
    workspace.join("cas")
}

/// TRACE_MATRIX FC2-N16 + FC3-N4: write a spec capsule into CAS, returning the CID hex.
///
/// `creator` is the agent_id submitting the capsule (or "user" for an
/// interactive spec session). `logical_t` is a monotonic counter; the
/// `turingos` CLI uses Unix-epoch seconds as the source so multiple runs
/// against the same workspace produce monotonic-enough timestamps without
/// needing a sequencer call.
pub fn write_spec_capsule(
    workspace: &Path,
    spec_md: &str,
    creator: &str,
    logical_t: u64,
) -> Result<String, CapsuleError> {
    let cas_dir = cas_path(workspace);
    std::fs::create_dir_all(&cas_dir)
        .map_err(|e| CapsuleError::Open(format!("create cas dir: {e}")))?;

    let mut store = CasStore::open(&cas_dir).map_err(|e| CapsuleError::Open(e.to_string()))?;

    let cid = store
        .put(
            spec_md.as_bytes(),
            ObjectType::EvidenceCapsule,
            creator,
            logical_t,
            Some(SPEC_CAPSULE_SCHEMA_ID.to_string()),
        )
        .map_err(|e| CapsuleError::Put(e.to_string()))?;

    Ok(cid.hex())
}

/// TRACE_MATRIX FC2-N16 + FC3-N4: latest spec-capsule CID lookup (used by `welcome`).
///
/// Return Some(cid_hex) if a spec capsule exists in this workspace's CAS,
/// or None. Picks the most-recent by created_at_logical_t when multiple
/// exist (welcome wants the latest, not the first).
pub fn latest_spec_capsule_cid(workspace: &Path) -> Result<Option<String>, CapsuleError> {
    let cas_dir = cas_path(workspace);
    if !cas_dir.exists() {
        return Ok(None);
    }
    let store = match CasStore::open(&cas_dir) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };
    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
    let mut best: Option<(u64, Cid)> = None;
    for cid in cids {
        if let Some(meta) = store.metadata(&cid) {
            if meta.schema_id.as_deref() == Some(SPEC_CAPSULE_SCHEMA_ID) {
                match best {
                    Some((t, _)) if t >= meta.created_at_logical_t => {}
                    _ => best = Some((meta.created_at_logical_t, cid)),
                }
            }
        }
    }
    Ok(best.map(|(_, cid)| cid.hex()))
}

/// TRACE_MATRIX FC2-N16 + FC3-N4: CAS readback by CID (used by `generate --from-capsule`).
///
/// Read spec.md bytes back from CAS by CID hex. Used by `turingos generate`
/// to re-hydrate the spec from canonical evidence rather than re-reading
/// the on-disk spec.md (which could be stale or hand-edited).
pub fn read_spec_capsule(workspace: &Path, cid_hex: &str) -> Result<Vec<u8>, CapsuleError> {
    let cas_dir = cas_path(workspace);
    let store = CasStore::open(&cas_dir).map_err(|e| CapsuleError::Read(e.to_string()))?;
    let cid_bytes =
        decode_cid_hex(cid_hex).map_err(|e| CapsuleError::Read(format!("bad cid hex: {e}")))?;
    let cid = Cid(cid_bytes);
    store
        .get(&cid)
        .map_err(|e| CapsuleError::Read(e.to_string()))
}

fn decode_cid_hex(s: &str) -> Result<[u8; 32], String> {
    if s.len() != 64 {
        return Err(format!("expected 64 hex chars, got {}", s.len()));
    }
    let mut out = [0u8; 32];
    for i in 0..32 {
        let byte_str = &s[i * 2..i * 2 + 2];
        out[i] = u8::from_str_radix(byte_str, 16).map_err(|e| e.to_string())?;
    }
    Ok(out)
}

/// TRACE_MATRIX FC2-N16: best-effort CLI error printer (CapsuleError → ExitCode).
///
/// Convenience: best-effort error printer that maps CapsuleError to a CLI
/// ExitCode + clear stderr message. Used by the spec/generate handlers.
#[allow(dead_code)]
pub fn capsule_error_exit(prefix: &str, err: CapsuleError) -> ExitCode {
    eprintln!("{prefix}: {err}");
    ExitCode::from(2)
}

// ---------------------------------------------------------------------------
// Phase 6.3.x — Software 3.0 LLM-driven grill capsule schemas
// R2 §A1: grill turns embed GrillAttemptRecord (no AttemptTelemetry reuse).
// R2 §A6: independent grill_attempt_count tally (not summed into evaluator).
// ---------------------------------------------------------------------------

use crate::runtime::grill_predicates::PredicateBundle;

/// Schema id for per-turn EvidenceCapsule body. Tail-additive; do not change.
pub const SPEC_GRILL_TURN_CAPSULE_SCHEMA_ID: &str = "turingos-spec-grill-turn-v1";

/// Schema id for session-rollup EvidenceCapsule body. Tail-additive.
pub const SPEC_GRILL_SESSION_CAPSULE_SCHEMA_ID: &str = "turingos-spec-grill-session-v1";

/// Grill-specific outcome enum. Byte-stable discriminants. Tail-additive only.
/// R2 §A1: this is NOT an extension of `AttemptOutcome`; grill turns do NOT
/// reuse the Lean-evaluator AttemptOutcome enum (would silently break
/// CLAUDE.md §6 evaluator_reported_completed_llm_calls semantics).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum GrillAttemptOutcome {
    PredicatesPassed = 0,
    SchemaParseFailed = 1,
    KindMismatch = 2,
    UnknownSlot = 3,
    NonMonotonic = 4,
    TurnOutOfRange = 5,
    LanguageMismatch = 6,
    LlmApiError = 7,
    DoubleRetryFailed = 8,
    TerminationGated = 9,
    // R2 §A5 reserved for W4.5: TriageNonRelevant = 10
}

/// Token usage triplet (mirrors siliconflow_client usage shape).
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GrillTokenCounts {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Grill-specific telemetry record. Embedded in the turn capsule body
/// (NOT a stand-alone CAS object; R2 §A1).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GrillAttemptRecord {
    pub schema_version: u32, // = 1
    pub session_id: String,
    pub turn_index: u32,
    pub model_id: String,
    pub prompt_context_hash: String,   // hex sha256
    pub candidate_payload_cid: String, // hex cid
    pub outcome: GrillAttemptOutcome,
    pub token_counts: GrillTokenCounts,
    pub elapsed_ms: u64,
    pub retry_index: u32, // 0 = first try, 1 = retry
}

/// Per-turn EvidenceCapsule body. Schema id: SPEC_GRILL_TURN_CAPSULE_SCHEMA_ID.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GrillTurnCapsuleBody {
    pub session_id: String,
    pub turn_index: u32,
    pub prompt_capsule_cid: String,
    pub user_answer_cid: Option<String>, // None on turn 1 initial prompt
    pub parent_turn_cid: Option<String>, // None on turn 1
    pub grill_attempt_record: GrillAttemptRecord,
    /// Use PredicateBundle from grill_predicates (serializable).
    pub predicate_verdicts: PredicateBundle,
    pub turn_payload_snapshot: serde_json::Value,
    pub logical_t: u64,
}

impl PartialEq for GrillTurnCapsuleBody {
    fn eq(&self, other: &Self) -> bool {
        self.session_id == other.session_id
            && self.turn_index == other.turn_index
            && self.prompt_capsule_cid == other.prompt_capsule_cid
            && self.user_answer_cid == other.user_answer_cid
            && self.parent_turn_cid == other.parent_turn_cid
            && self.grill_attempt_record == other.grill_attempt_record
            && self.predicate_verdicts.p1_schema_parse_ok
                == other.predicate_verdicts.p1_schema_parse_ok
            && self.predicate_verdicts.p2_kind_ok == other.predicate_verdicts.p2_kind_ok
            && self.predicate_verdicts.p3_slots_in_vocab
                == other.predicate_verdicts.p3_slots_in_vocab
            && self.predicate_verdicts.p4_monotonic == other.predicate_verdicts.p4_monotonic
            && self.predicate_verdicts.p5_turn_bounded == other.predicate_verdicts.p5_turn_bounded
            && self.predicate_verdicts.p6_question_nonempty_lang
                == other.predicate_verdicts.p6_question_nonempty_lang
            && self.turn_payload_snapshot == other.turn_payload_snapshot
            && self.logical_t == other.logical_t
    }
}

/// Independent grill_attempt_count tally (R2 §A6). Reported alongside but
/// NOT summed into evaluator_reported_completed_llm_calls (CLAUDE.md §6 LHS).
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GrillAttemptTally {
    pub meta_turns_accepted: u32,
    pub meta_turns_rejected: u32,
    pub triage_calls_relevant: u32,     // R2 §A5 W4.5 reserved
    pub triage_calls_non_relevant: u32, // R2 §A5 W4.5 reserved
    pub synthesis_calls: u32,           // = 1 on success, 0 on abort
}

/// Session-rollup EvidenceCapsule body. Schema id: SPEC_GRILL_SESSION_CAPSULE_SCHEMA_ID.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GrillSessionCapsuleBody {
    pub session_id: String,
    pub turn_cids: Vec<String>,
    pub final_spec_capsule_cid: String,
    pub termination_reason: String,
    pub total_turns: u32,
    pub partial_session: bool,
    pub lang: String,
    pub grill_attempt_tally: GrillAttemptTally,
    pub logical_t: u64,
}

// ---------------------------------------------------------------------------
// Writers / readers (mirror existing write_spec_capsule pattern at line 73)
// ---------------------------------------------------------------------------

pub fn write_grill_turn_capsule(
    workspace: &Path,
    body: &GrillTurnCapsuleBody,
) -> Result<String, CapsuleError> {
    let cas_dir = cas_path(workspace);
    std::fs::create_dir_all(&cas_dir)
        .map_err(|e| CapsuleError::Open(format!("create cas dir: {e}")))?;

    let mut store = CasStore::open(&cas_dir).map_err(|e| CapsuleError::Open(e.to_string()))?;

    let body_bytes =
        serde_json::to_vec(body).map_err(|e| CapsuleError::Put(format!("serialize body: {e}")))?;

    let cid = store
        .put(
            &body_bytes,
            ObjectType::EvidenceCapsule,
            "grill_system",
            body.logical_t,
            Some(SPEC_GRILL_TURN_CAPSULE_SCHEMA_ID.to_string()),
        )
        .map_err(|e| CapsuleError::Put(e.to_string()))?;

    Ok(cid.hex())
}

pub fn read_grill_turn_capsule(
    workspace: &Path,
    cid_hex: &str,
) -> Result<GrillTurnCapsuleBody, CapsuleError> {
    let cas_dir = cas_path(workspace);
    let store = CasStore::open(&cas_dir).map_err(|e| CapsuleError::Read(e.to_string()))?;
    let cid_bytes =
        decode_cid_hex(cid_hex).map_err(|e| CapsuleError::Read(format!("bad cid hex: {e}")))?;
    let cid = Cid(cid_bytes);
    let body_bytes = store
        .get(&cid)
        .map_err(|e| CapsuleError::Read(e.to_string()))?;
    serde_json::from_slice(&body_bytes)
        .map_err(|e| CapsuleError::Read(format!("deserialize body: {e}")))
}

pub fn write_grill_session_capsule(
    workspace: &Path,
    body: &GrillSessionCapsuleBody,
) -> Result<String, CapsuleError> {
    let cas_dir = cas_path(workspace);
    std::fs::create_dir_all(&cas_dir)
        .map_err(|e| CapsuleError::Open(format!("create cas dir: {e}")))?;

    let mut store = CasStore::open(&cas_dir).map_err(|e| CapsuleError::Open(e.to_string()))?;

    let body_bytes =
        serde_json::to_vec(body).map_err(|e| CapsuleError::Put(format!("serialize body: {e}")))?;

    let cid = store
        .put(
            &body_bytes,
            ObjectType::EvidenceCapsule,
            "grill_system",
            body.logical_t,
            Some(SPEC_GRILL_SESSION_CAPSULE_SCHEMA_ID.to_string()),
        )
        .map_err(|e| CapsuleError::Put(e.to_string()))?;

    Ok(cid.hex())
}

pub fn read_grill_session_capsule(
    workspace: &Path,
    cid_hex: &str,
) -> Result<GrillSessionCapsuleBody, CapsuleError> {
    let cas_dir = cas_path(workspace);
    let store = CasStore::open(&cas_dir).map_err(|e| CapsuleError::Read(e.to_string()))?;
    let cid_bytes =
        decode_cid_hex(cid_hex).map_err(|e| CapsuleError::Read(format!("bad cid hex: {e}")))?;
    let cid = Cid(cid_bytes);
    let body_bytes = store
        .get(&cid)
        .map_err(|e| CapsuleError::Read(e.to_string()))?;
    serde_json::from_slice(&body_bytes)
        .map_err(|e| CapsuleError::Read(format!("deserialize body: {e}")))
}

pub fn list_grill_session_capsules(workspace: &Path) -> Result<Vec<String>, CapsuleError> {
    let cas_dir = cas_path(workspace);
    if !cas_dir.exists() {
        return Ok(Vec::new());
    }
    let store = match CasStore::open(&cas_dir) {
        Ok(s) => s,
        Err(_) => return Ok(Vec::new()),
    };
    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
    let mut results = Vec::new();
    for cid in cids {
        if let Some(meta) = store.metadata(&cid) {
            if meta.schema_id.as_deref() == Some(SPEC_GRILL_SESSION_CAPSULE_SCHEMA_ID) {
                results.push(cid.hex());
            }
        }
    }
    Ok(results)
}

#[cfg(test)]
mod grill_capsule_tests {
    use super::*;
    use crate::runtime::grill_predicates::PredicateVerdict;

    fn temp_workspace() -> tempfile::TempDir {
        tempfile::tempdir().expect("create temp workspace")
    }

    fn make_predicate_bundle_all_pass() -> PredicateBundle {
        PredicateBundle {
            p1_schema_parse_ok: PredicateVerdict::Pass,
            p2_kind_ok: PredicateVerdict::Pass,
            p3_slots_in_vocab: PredicateVerdict::Pass,
            p4_monotonic: PredicateVerdict::Pass,
            p5_turn_bounded: PredicateVerdict::Pass,
            p6_question_nonempty_lang: PredicateVerdict::Pass,
        }
    }

    fn make_turn_body() -> GrillTurnCapsuleBody {
        GrillTurnCapsuleBody {
            session_id: "session_test_001".into(),
            turn_index: 1,
            prompt_capsule_cid: "aaa111".into(),
            user_answer_cid: None,
            parent_turn_cid: None,
            grill_attempt_record: GrillAttemptRecord {
                schema_version: 1,
                session_id: "session_test_001".into(),
                turn_index: 1,
                model_id: "deepseek-ai/DeepSeek-V3.2".into(),
                prompt_context_hash: "bbb222".into(),
                candidate_payload_cid: "ccc333".into(),
                outcome: GrillAttemptOutcome::PredicatesPassed,
                token_counts: GrillTokenCounts::default(),
                elapsed_ms: 0,
                retry_index: 0,
            },
            predicate_verdicts: make_predicate_bundle_all_pass(),
            turn_payload_snapshot: serde_json::json!({"turn": 1, "question": "hi"}),
            logical_t: 1716000000,
        }
    }

    #[test]
    fn write_then_read_turn_capsule_roundtrip() {
        let ws = temp_workspace();
        let body = make_turn_body();
        let cid = write_grill_turn_capsule(ws.path(), &body).expect("write should succeed");
        let read_back = read_grill_turn_capsule(ws.path(), &cid).expect("read should succeed");
        assert_eq!(read_back, body);
    }

    #[test]
    fn turn_capsule_with_no_parent_for_turn_1() {
        let body = make_turn_body();
        assert!(body.parent_turn_cid.is_none());
        assert_eq!(body.turn_index, 1);
    }

    #[test]
    fn write_then_read_session_capsule_roundtrip() {
        let ws = temp_workspace();
        let body = GrillSessionCapsuleBody {
            session_id: "session_test_002".into(),
            turn_cids: vec!["cid1".into(), "cid2".into(), "cid3".into()],
            final_spec_capsule_cid: "fff999".into(),
            termination_reason: "llm_done_predicate_pass".into(),
            total_turns: 3,
            partial_session: false,
            lang: "zh".into(),
            grill_attempt_tally: GrillAttemptTally {
                meta_turns_accepted: 3,
                meta_turns_rejected: 0,
                triage_calls_relevant: 0,
                triage_calls_non_relevant: 0,
                synthesis_calls: 1,
            },
            logical_t: 1716001000,
        };
        let cid = write_grill_session_capsule(ws.path(), &body).expect("write");
        let read_back = read_grill_session_capsule(ws.path(), &cid).expect("read");
        assert_eq!(read_back, body);
    }

    #[test]
    fn list_returns_only_grill_session_schemas() {
        let ws = temp_workspace();
        // Write 2 turn capsules + 1 session capsule
        let _t1 = write_grill_turn_capsule(ws.path(), &make_turn_body()).unwrap();
        let _t2 = write_grill_turn_capsule(ws.path(), &make_turn_body()).unwrap();
        let s_body = GrillSessionCapsuleBody {
            session_id: "session_xxx".into(),
            turn_cids: vec!["a".into()],
            final_spec_capsule_cid: "final".into(),
            termination_reason: "llm_done_predicate_pass".into(),
            total_turns: 1,
            partial_session: false,
            lang: "zh".into(),
            grill_attempt_tally: GrillAttemptTally::default(),
            logical_t: 0,
        };
        let _s = write_grill_session_capsule(ws.path(), &s_body).unwrap();

        let list = list_grill_session_capsules(ws.path()).expect("list");
        assert_eq!(
            list.len(),
            1,
            "list_grill_session_capsules should return only session-schema capsules, got {} items",
            list.len()
        );
    }

    #[test]
    fn grill_attempt_outcome_discriminant_lock() {
        assert_eq!(GrillAttemptOutcome::PredicatesPassed as u8, 0);
        assert_eq!(GrillAttemptOutcome::SchemaParseFailed as u8, 1);
        assert_eq!(GrillAttemptOutcome::KindMismatch as u8, 2);
        assert_eq!(GrillAttemptOutcome::UnknownSlot as u8, 3);
        assert_eq!(GrillAttemptOutcome::NonMonotonic as u8, 4);
        assert_eq!(GrillAttemptOutcome::TurnOutOfRange as u8, 5);
        assert_eq!(GrillAttemptOutcome::LanguageMismatch as u8, 6);
        assert_eq!(GrillAttemptOutcome::LlmApiError as u8, 7);
        assert_eq!(GrillAttemptOutcome::DoubleRetryFailed as u8, 8);
        assert_eq!(GrillAttemptOutcome::TerminationGated as u8, 9);
    }

    #[test]
    fn existing_spec_capsule_schema_id_unchanged() {
        assert_eq!(SPEC_CAPSULE_SCHEMA_ID, "turingos-spec-capsule-v1");
    }

    #[test]
    fn grill_turn_capsule_schema_id_is_v1() {
        assert_eq!(
            SPEC_GRILL_TURN_CAPSULE_SCHEMA_ID,
            "turingos-spec-grill-turn-v1"
        );
    }

    #[test]
    fn grill_session_capsule_schema_id_is_v1() {
        assert_eq!(
            SPEC_GRILL_SESSION_CAPSULE_SCHEMA_ID,
            "turingos-spec-grill-session-v1"
        );
    }
}
