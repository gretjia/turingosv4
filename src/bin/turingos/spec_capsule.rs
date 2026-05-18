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

use std::path::Path;
use std::process::ExitCode;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;

/// TRACE_MATRIX FC2-N16 + FC3-N4 (CAS evidence binding):
/// Schema-id tag for spec capsules — lets `welcome` find them in the index
/// without scanning bytes. Versioned so a future format bump (e.g. a binary
/// canonical encoding) can coexist with the v1 markdown form.
pub(crate) const SPEC_CAPSULE_SCHEMA_ID: &str = "turingos-spec-capsule-v1";

/// TRACE_MATRIX FC2-N16: error taxonomy for spec-capsule CAS operations.
#[derive(Debug)]
pub(crate) enum CapsuleError {
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
pub(crate) fn cas_path(workspace: &Path) -> std::path::PathBuf {
    workspace.join("cas")
}

/// TRACE_MATRIX FC2-N16 + FC3-N4: write a spec capsule into CAS, returning the CID hex.
///
/// `creator` is the agent_id submitting the capsule (or "user" for an
/// interactive spec session). `logical_t` is a monotonic counter; the
/// `turingos` CLI uses Unix-epoch seconds as the source so multiple runs
/// against the same workspace produce monotonic-enough timestamps without
/// needing a sequencer call.
pub(crate) fn write_spec_capsule(
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
pub(crate) fn latest_spec_capsule_cid(workspace: &Path) -> Result<Option<String>, CapsuleError> {
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
pub(crate) fn read_spec_capsule(workspace: &Path, cid_hex: &str) -> Result<Vec<u8>, CapsuleError> {
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
pub(crate) fn capsule_error_exit(prefix: &str, err: CapsuleError) -> ExitCode {
    eprintln!("{prefix}: {err}");
    ExitCode::from(2)
}
