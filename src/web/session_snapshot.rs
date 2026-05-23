//! TRACE_MATRIX FC1-N12 + FC3-replay: Private web-layer GrillSession snapshot
//! for cross-CLI / cross-restart resume.
//!
//! TB-SOFTWARE-3-0 Atom S2 (2026-05-23). Closes PHASE_E_REAL_VALIDATION §3
//! gap "No cross-CLI session resume from CAS".
//!
//! Design (per package §8, "保留方向，删除过度工程"):
//! - This is a **derived view**, NOT a truth source. ChainTape + CAS remain
//!   canonical; the in-memory `AppState.sessions` HashMap is a cache.
//! - Snapshot is written after every successful handler mutation, BEFORE
//!   returning Ok.
//! - On cache miss (server restart, HashMap cleared), the handler tries to
//!   reload the latest snapshot from CAS and rebuild the GrillSession cache.
//! - Uses the **existing** `EvidenceCapsule` ObjectType — no new CAS schema
//!   registration. Schema id `turingos-web-grill-session-snapshot-v1` is a
//!   string tag, not a registered schema in `src/bottom_white/cas/schema.rs`.
//! - Private (`pub(crate)`); no external API contract is exported.

#![cfg(feature = "web")]

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::path::Path;

use serde::{Deserialize, Serialize};

use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;

use super::ws::{GrillSession, SlotState};

/// TRACE_MATRIX FC3-replay: schema id string tag for snapshot capsules.
///
/// NOT a CAS schema registration — just a serde tag used to distinguish
/// these EvidenceCapsule bodies from other EvidenceCapsule payloads in the
/// same CAS store. Tail-additive; do not change.
pub(crate) const GRILL_SESSION_SNAPSHOT_SCHEMA_ID: &str =
    "turingos-web-grill-session-snapshot-v1";

/// TRACE_MATRIX FC1-N12: serialized snapshot of a GrillSession.
///
/// Mirrors ONLY the fields needed for cache rebuild. Tail-additive: future
/// fields append to the bottom; absence is tolerated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GrillSessionSnapshot {
    pub session_id: String,
    pub turn_count: u32,
    pub lang: String,
    /// Serialized `coverage_state` (slot_id → state-as-string for serde portability).
    pub coverage_state: BTreeMap<String, String>,
    /// Most-recent triage-relevant Q/A pairs (max 3, oldest-first).
    pub last_3_turns: Vec<(String, String)>,
    pub turn_cids: Vec<String>,
    pub parent_turn_cid: Option<String>,
    pub terminated: bool,
    pub non_relevant_count: u32,
    pub last_prev_covered: Vec<String>,
    // Tallies
    pub meta_turns_accepted: u32,
    pub meta_turns_rejected: u32,
    pub triage_calls_relevant: u32,
    pub triage_calls_non_relevant: u32,
    pub last_question_emitted: String,
    pub all_user_answers: Vec<String>,
    pub slot_evidence: BTreeMap<String, String>,
    pub created_at_unix: u64,
    /// Monotonic-ish ordinal used to pick "latest" when scanning CAS. The
    /// caller supplies a value (typically turn_count or unix-secs at write
    /// time); the load helper picks the snapshot with the highest value.
    pub logical_t: u64,
}

fn slot_state_to_str(s: &SlotState) -> &'static str {
    match s {
        SlotState::Empty => "empty",
        SlotState::Partial => "partial",
        SlotState::Satisfied => "satisfied",
    }
}

fn str_to_slot_state(s: &str) -> SlotState {
    match s {
        "satisfied" => SlotState::Satisfied,
        "partial" => SlotState::Partial,
        _ => SlotState::Empty,
    }
}

impl GrillSessionSnapshot {
    /// TRACE_MATRIX FC1-N12: derive a snapshot from the in-memory GrillSession.
    pub(crate) fn from_session(session: &GrillSession, logical_t: u64) -> Self {
        let mut coverage_state = BTreeMap::new();
        for (k, v) in &session.coverage_state {
            coverage_state.insert(k.clone(), slot_state_to_str(v).to_string());
        }
        let last_3_turns: Vec<(String, String)> =
            session.last_3_turns.iter().cloned().collect();
        Self {
            session_id: session.session_id.clone(),
            turn_count: session.turn_count,
            lang: session.lang.clone(),
            coverage_state,
            last_3_turns,
            turn_cids: session.turn_cids.clone(),
            parent_turn_cid: session.parent_turn_cid.clone(),
            terminated: session.terminated,
            non_relevant_count: session.non_relevant_count,
            last_prev_covered: session.last_prev_covered.clone(),
            meta_turns_accepted: session.meta_turns_accepted,
            meta_turns_rejected: session.meta_turns_rejected,
            triage_calls_relevant: session.triage_calls_relevant,
            triage_calls_non_relevant: session.triage_calls_non_relevant,
            last_question_emitted: session.last_question_emitted.clone(),
            all_user_answers: session.all_user_answers.clone(),
            slot_evidence: session.slot_evidence.clone(),
            created_at_unix: session.created_at_unix,
            logical_t,
        }
    }

    /// TRACE_MATRIX FC1-N12: rebuild a GrillSession from a snapshot.
    pub(crate) fn into_session(self) -> GrillSession {
        let mut coverage_state: HashMap<String, SlotState> = HashMap::new();
        for (k, v) in self.coverage_state {
            coverage_state.insert(k, str_to_slot_state(&v));
        }
        let mut last_3_turns: VecDeque<(String, String)> = VecDeque::new();
        for pair in self.last_3_turns {
            last_3_turns.push_back(pair);
        }
        GrillSession {
            session_id: self.session_id,
            turn_count: self.turn_count,
            lang: self.lang,
            coverage_state,
            last_3_turns,
            turn_cids: self.turn_cids,
            terminated: self.terminated,
            parent_turn_cid: self.parent_turn_cid,
            created_at_unix: self.created_at_unix,
            non_relevant_count: self.non_relevant_count,
            last_prev_covered: self.last_prev_covered,
            meta_turns_accepted: self.meta_turns_accepted,
            meta_turns_rejected: self.meta_turns_rejected,
            triage_calls_relevant: self.triage_calls_relevant,
            triage_calls_non_relevant: self.triage_calls_non_relevant,
            last_question_emitted: self.last_question_emitted,
            all_user_answers: self.all_user_answers,
            slot_evidence: self.slot_evidence,
        }
    }
}

/// TRACE_MATRIX FC3-replay: write a snapshot capsule into the session-local CAS.
///
/// Path: `<workspace>/sessions/<session_id>/cas/`. Returns the CID hex on success.
/// Errors are returned as `String` so the caller can log + continue without
/// failing the user-facing request — snapshot writes are best-effort cache
/// instrumentation, not a truth-write.
pub(crate) fn write_snapshot(
    workspace: &Path,
    session: &GrillSession,
    logical_t: u64,
) -> Result<String, String> {
    let snapshot = GrillSessionSnapshot::from_session(session, logical_t);
    let bytes = serde_json::to_vec(&snapshot)
        .map_err(|e| format!("snapshot serde: {e}"))?;
    let cas_dir = workspace
        .join("sessions")
        .join(&session.session_id)
        .join("cas");
    std::fs::create_dir_all(&cas_dir).map_err(|e| format!("mkdir cas: {e}"))?;
    let mut store = CasStore::open(&cas_dir).map_err(|e| format!("cas open: {e}"))?;
    let cid = store
        .put(
            &bytes,
            ObjectType::EvidenceCapsule,
            "web_grill_session_snapshot",
            logical_t,
            Some(GRILL_SESSION_SNAPSHOT_SCHEMA_ID.to_string()),
        )
        .map_err(|e| format!("cas put: {e}"))?;
    Ok(cid.hex())
}

/// TRACE_MATRIX FC3-replay: load the latest snapshot for a session_id.
///
/// Scans `<workspace>/sessions/<session_id>/cas/` for snapshot capsules,
/// picks the one with the highest `logical_t`. Returns `None` if no
/// session dir, no CAS dir, no matching capsules, or any read/parse error
/// (best-effort cache rebuild — failures fall through to the existing
/// invalid-input behavior).
pub(crate) fn load_latest_snapshot(
    workspace: &Path,
    session_id: &str,
) -> Option<GrillSessionSnapshot> {
    let cas_dir = workspace.join("sessions").join(session_id).join("cas");
    if !cas_dir.exists() {
        return None;
    }
    let store = CasStore::open(&cas_dir).ok()?;
    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
    let mut best: Option<GrillSessionSnapshot> = None;
    for cid in cids {
        let bytes = match store.get(&cid) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let snap: GrillSessionSnapshot = match serde_json::from_slice(&bytes) {
            Ok(s) => s,
            Err(_) => continue, // other EvidenceCapsule schemas in this dir
        };
        if snap.session_id != session_id {
            continue;
        }
        match &best {
            None => best = Some(snap),
            Some(cur) if snap.logical_t > cur.logical_t => best = Some(snap),
            _ => {}
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, VecDeque};

    fn fixture_session(session_id: &str, turn_count: u32) -> GrillSession {
        let mut coverage_state = HashMap::new();
        coverage_state.insert("job".to_string(), SlotState::Satisfied);
        coverage_state.insert("memory".to_string(), SlotState::Partial);
        let mut last_3_turns = VecDeque::new();
        last_3_turns.push_back(("Q1".to_string(), "A1".to_string()));
        last_3_turns.push_back(("Q2".to_string(), "A2".to_string()));
        let mut slot_evidence = BTreeMap::new();
        slot_evidence.insert("job".to_string(), "user-says-X".to_string());
        GrillSession {
            session_id: session_id.to_string(),
            turn_count,
            lang: "zh".into(),
            coverage_state,
            last_3_turns,
            turn_cids: vec!["cid1".into(), "cid2".into()],
            terminated: false,
            parent_turn_cid: Some("cid2".into()),
            created_at_unix: 1_700_000_000,
            non_relevant_count: 0,
            last_prev_covered: vec!["job".into()],
            meta_turns_accepted: turn_count,
            meta_turns_rejected: 0,
            triage_calls_relevant: turn_count,
            triage_calls_non_relevant: 0,
            last_question_emitted: "Q3".into(),
            all_user_answers: vec!["A1".into(), "A2".into()],
            slot_evidence,
        }
    }

    #[test]
    fn snapshot_roundtrip_preserves_canonical_fields() {
        let original = fixture_session("test-session-001", 2);
        let snapshot = GrillSessionSnapshot::from_session(&original, 2);
        let rebuilt = snapshot.into_session();

        assert_eq!(rebuilt.session_id, original.session_id);
        assert_eq!(rebuilt.turn_count, original.turn_count);
        assert_eq!(rebuilt.lang, original.lang);
        assert_eq!(rebuilt.terminated, original.terminated);
        assert_eq!(rebuilt.parent_turn_cid, original.parent_turn_cid);
        assert_eq!(rebuilt.turn_cids, original.turn_cids);
        assert_eq!(rebuilt.meta_turns_accepted, original.meta_turns_accepted);
        assert_eq!(rebuilt.triage_calls_relevant, original.triage_calls_relevant);
        assert_eq!(rebuilt.last_question_emitted, original.last_question_emitted);
        assert_eq!(rebuilt.all_user_answers, original.all_user_answers);
        assert_eq!(rebuilt.slot_evidence, original.slot_evidence);
        assert_eq!(rebuilt.last_prev_covered, original.last_prev_covered);
        assert_eq!(rebuilt.last_3_turns.len(), original.last_3_turns.len());
        for (a, b) in rebuilt.last_3_turns.iter().zip(original.last_3_turns.iter()) {
            assert_eq!(a, b);
        }
        assert_eq!(rebuilt.coverage_state.len(), original.coverage_state.len());
        for (k, v) in &original.coverage_state {
            assert_eq!(rebuilt.coverage_state.get(k), Some(v));
        }
    }

    #[test]
    fn write_and_load_roundtrip_via_cas() {
        let temp = tempfile::TempDir::new().expect("tempdir");
        let workspace = temp.path();
        let session = fixture_session("kill-resume-1", 3);

        // Write twice with increasing logical_t to verify "latest wins"
        write_snapshot(workspace, &session, 1).expect("write 1");
        let mut later = session.clone();
        later.turn_count = 5;
        later.all_user_answers.push("A3".into());
        write_snapshot(workspace, &later, 5).expect("write 2");

        // Simulate cache miss: load latest
        let loaded = load_latest_snapshot(workspace, "kill-resume-1")
            .expect("load_latest_snapshot must return Some after writes");
        assert_eq!(loaded.logical_t, 5, "must pick highest logical_t");
        assert_eq!(loaded.turn_count, 5);
        assert_eq!(loaded.all_user_answers.len(), 3);

        // Rebuild into GrillSession
        let rebuilt = loaded.into_session();
        assert_eq!(rebuilt.session_id, "kill-resume-1");
        assert_eq!(rebuilt.turn_count, 5);
        assert_eq!(rebuilt.all_user_answers.last().unwrap(), "A3");
    }

    #[test]
    fn load_latest_returns_none_for_unknown_session() {
        let temp = tempfile::TempDir::new().expect("tempdir");
        let workspace = temp.path();
        assert!(load_latest_snapshot(workspace, "does-not-exist").is_none());
    }
}
