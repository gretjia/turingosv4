//! TB-15 Atom 5 — `MarkovEvidenceCapsule` schema + writer + default-deny
//! deep-history read gate (architect §6.2 + DECISION_LAMARCKIAN §1).
//!
//! End-of-TB rollup binding `constitution_hash` + L4 root + L4.E root +
//! CAS root + previous capsule + typical_errors + unresolved_obs +
//! `next_session_context_cid`. Default next-session bootstrap source per
//! FR-15.4. Deeper history requires `TURINGOS_MARKOV_OVERRIDE=1`
//! (FR-15.5 + halt-trigger #6).
//!
//! Generator surface: `write_markov_capsule(...)` (CAS-emitting) + binary
//! `src/bin/generate_markov_capsule.rs` (CLI wrapper).
//!
//! TRACE_MATRIX FC3-N43 + Art. 0.2 (Tape Canonical: capsule canonical
//! bytes are themselves the CAS object referenced by `capsule_id`) +
//! CR-15.5 (capsules are evidence compression, not hidden source of
//! truth — every field is derivable from the chain + CAS) +
//! CR-15.6 (Markov default prevents context poisoning).

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::CasStore;
use crate::bottom_white::ledger::transition_ledger::canonical_encode;
use crate::runtime::autopsy_capsule::TypicalErrorSummary;
use crate::state::q_state::Hash;

/// TRACE_MATRIX TB-15 (architect §6.2): unresolved OBS identifier.
/// Opaque string newtype carrying the relative path of an `OBS_*.md`
/// file under `handover/alignment/` (the project's de-facto observation
/// register). Cross-session continuity hint per CR-15.5.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct ObsId(pub String);

/// TRACE_MATRIX TB-15 (architect §6.2 + FR-15.4 + FR-15.5): end-of-TB
/// rollup. Default next-session bootstrap source: future agent reads
/// `constitution.md` (referenced by `constitution_hash`) + this capsule
/// (referenced by `capsule_id`) — no deeper history without
/// `TURINGOS_MARKOV_OVERRIDE=1`.
///
/// **CR-15.5**: every field is derivable from the chain + CAS at
/// generation time (constitution_hash from constitution.md, l4_root
/// from L4 chain head, l4e_root from L4.E chain head, cas_root from CAS
/// metadata digest, typical_errors from cluster_autopsies(...) over
/// CAS-resident capsules, unresolved_obs from `handover/alignment/OBS_*.md`).
/// Capsule is evidence compression, not hidden source of truth.
///
/// **Markov chain**: `previous_capsule_cid` points to the prior capsule
/// (None for genesis Markov capsule); next-session context defaults to
/// {constitution + this capsule}. Deeper history (older capsules; L4
/// rows pre-dating `previous_capsule_cid`'s `l4_root`) requires
/// `TURINGOS_MARKOV_OVERRIDE=1` per CR-15.6.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarkovEvidenceCapsule {
    /// CAS Cid of this capsule's canonical bytes (with `capsule_id`
    /// zeroed during the hash). Computed by writer.
    pub capsule_id: Cid,

    /// Cid of the previous Markov capsule in the chain. `None` for the
    /// first ever capsule (genesis Markov).
    pub previous_capsule_cid: Option<Cid>,

    /// SHA-256 of `constitution.md` bytes at generation time.
    /// SG-15.7: capsule must reference constitution hash.
    pub constitution_hash: Hash,

    /// TRACE_MATRIX TB-15 R2 closure (Codex R1 Q8/RQ7 + Gemini R1 Q7):
    /// canonical SHA-256 hashes of the four constitution flowcharts
    /// (Flowchart 1a / 1b / 2 / 3) per
    /// `handover/alignment/TRACE_FLOWCHART_MATRIX.md` §2 (architect
    /// 2026-05-02 directive — flowcharts elevated to SHA-anchored
    /// architectural contracts). Empty Vec for backward-compat with
    /// pre-R2 chain snapshots; populated by the generator binary in
    /// canonical-order (1a, 1b, 2, 3). Closes the literal SG-15.7 spec
    /// "constitution hash AND flowchart hashes" requirement.
    #[serde(default)]
    pub flowchart_hashes: Vec<Hash>,

    /// L4 transition_ledger root at generation time.
    pub l4_root: Hash,
    /// L4.E rejection_evidence ledger root at generation time.
    pub l4e_root: Hash,
    /// CAS metadata root (digest of CAS object metadata) at generation
    /// time. Strictly informational; CR-15.5 — capsule does not
    /// duplicate CAS contents.
    pub cas_root: Hash,

    /// Typical-error rollup at generation time (TB-15 Atom 4 surface).
    pub typical_errors: Vec<TypicalErrorSummary>,
    /// Open observation register entries at generation time (relative
    /// paths under `handover/alignment/OBS_*.md`).
    pub unresolved_obs: Vec<ObsId>,

    /// CAS Cid of a JSON blob describing the next session's default
    /// boot context (`{constitution_hash, latest_markov_cid, boot_seq}`).
    /// FR-15.4 + halt-trigger #6 entry point.
    pub next_session_context_cid: Cid,

    /// SHA-256 of this capsule's canonical bytes. Defense-in-depth
    /// duplicate of `capsule_id`.
    pub sha256: Hash,
    /// Logical time at generation (sequencer or generator-supplied).
    pub created_at_logical_t: u64,
    /// Free-form TB tag — e.g. `"TB-15"`. Strictly informational.
    pub tb_tag: String,
}

impl Default for MarkovEvidenceCapsule {
    fn default() -> Self {
        Self {
            capsule_id: Cid::default(),
            previous_capsule_cid: None,
            constitution_hash: Hash::ZERO,
            flowchart_hashes: Vec::new(),
            l4_root: Hash::ZERO,
            l4e_root: Hash::ZERO,
            cas_root: Hash::ZERO,
            typical_errors: Vec::new(),
            unresolved_obs: Vec::new(),
            next_session_context_cid: Cid::default(),
            sha256: Hash::ZERO,
            created_at_logical_t: 0,
            tb_tag: String::new(),
        }
    }
}

impl MarkovEvidenceCapsule {
    /// TRACE_MATRIX TB-15 Atom 5 — convenience constructor used by
    /// halt-trigger #2 to pin `constitution_hash` to a known value
    /// (verifies SG-15.7 from a fixture without spinning up the
    /// generator binary).
    pub fn with_constitution_hash(hash_bytes: [u8; 32]) -> Self {
        Self {
            constitution_hash: Hash(hash_bytes),
            ..Self::default()
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// TB-15 Atom 5 — Writer + default-deny gate
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-15 Atom 5 — writer / generator error taxonomy.
#[derive(Debug)]
pub enum MarkovGenError {
    /// Default-deny: caller asked for deep-history read without
    /// `TURINGOS_MARKOV_OVERRIDE=1`. SG-15.4 + halt-trigger #6.
    DeepHistoryReadDenied,
    Cas(crate::bottom_white::cas::store::CasError),
    Encode(String),
    Io(std::io::Error),
    InternalLockPoisoned,
}

impl std::fmt::Display for MarkovGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeepHistoryReadDenied => write!(
                f,
                "deep-history read denied: set TURINGOS_MARKOV_OVERRIDE=1 to enable"
            ),
            Self::Cas(e) => write!(f, "cas: {e}"),
            Self::Encode(s) => write!(f, "encode: {s}"),
            Self::Io(e) => write!(f, "io: {e}"),
            Self::InternalLockPoisoned => write!(f, "internal lock poisoned"),
        }
    }
}
impl std::error::Error for MarkovGenError {}

impl From<crate::bottom_white::cas::store::CasError> for MarkovGenError {
    fn from(e: crate::bottom_white::cas::store::CasError) -> Self {
        Self::Cas(e)
    }
}
impl From<std::io::Error> for MarkovGenError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// TRACE_MATRIX TB-15 Atom 5 (architect FR-15.5 + halt-trigger #6):
/// default-deny gate for deep-history reads. Returns
/// `Err(DeepHistoryReadDenied)` unless `override_set` is true.
///
/// In production, `override_set` is wired to env
/// `TURINGOS_MARKOV_OVERRIDE=1` by the binary; this helper isolates the
/// decision so it can be exercised by halt-trigger #6 without process-
/// global env mutation (env mutation racy under cargo's parallel test
/// runner per `feedback_env_var_test_lock`).
pub fn try_deep_history_read_with_override_check(override_set: bool) -> Result<(), MarkovGenError> {
    if override_set {
        Ok(())
    } else {
        Err(MarkovGenError::DeepHistoryReadDenied)
    }
}

/// TRACE_MATRIX TB-15 Atom 5 (architect FR-15.5): bool wrapper that
/// reads `TURINGOS_MARKOV_OVERRIDE` from process env. Used by the
/// generator binary; isolated here so the decision is auditable.
pub fn override_set_from_env() -> bool {
    std::env::var("TURINGOS_MARKOV_OVERRIDE")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// TRACE_MATRIX TB-15 Atom 5: write a `MarkovEvidenceCapsule` to CAS.
/// Flow:
///
/// 1. Build the next-session-context JSON blob → write to CAS as
///    `ObjectType::NextSessionContext`. Cid is `next_session_context_cid`.
/// 2. Build the capsule struct with `capsule_id = Cid::default()` +
///    `sha256 = Hash::ZERO`. Canonical-encode → sha256 → that's the
///    eventual `capsule_id`.
/// 3. Re-create the struct with `capsule_id` filled in + write to CAS
///    as `ObjectType::MarkovEvidenceCapsule`.
///
/// **CR-15.5**: caller supplies `constitution_hash` / `l4_root` /
/// `l4e_root` / `cas_root` / `typical_errors` / `unresolved_obs` —
/// each derived from the chain + CAS at generation time. Writer does
/// NOT mint new ground truth.
#[allow(clippy::too_many_arguments)]
pub fn write_markov_capsule(
    cas: &std::sync::Arc<std::sync::RwLock<CasStore>>,
    previous_capsule_cid: Option<Cid>,
    constitution_hash: Hash,
    flowchart_hashes: Vec<Hash>,
    l4_root: Hash,
    l4e_root: Hash,
    cas_root: Hash,
    typical_errors: Vec<TypicalErrorSummary>,
    unresolved_obs: Vec<ObsId>,
    tb_tag: String,
    creator_str: &str,
    created_at_logical_t: u64,
) -> Result<MarkovEvidenceCapsule, MarkovGenError> {
    let mut cas_w = cas
        .write()
        .map_err(|_| MarkovGenError::InternalLockPoisoned)?;

    // Step 1: build + write next_session_context JSON.
    let next_session_json = serde_json::json!({
        "schema_version": "v1/next_session_context",
        "constitution_hash_hex": hex(&constitution_hash.0),
        "flowchart_hashes_hex": flowchart_hashes.iter().map(|h| hex(&h.0)).collect::<Vec<_>>(),
        "previous_markov_cid_hex": previous_capsule_cid.map(|c| c.hex()),
        "tb_tag": tb_tag,
        "boot_seq": [
            "1. read constitution.md (verify sha256 == constitution_hash)",
            "2. read TRACE_FLOWCHART_MATRIX.md (verify each flowchart sha256 == flowchart_hashes[i])",
            "3. read CAS<this_markov_capsule_cid>",
            "4. read CAS<previous_markov_capsule_cid> (if present)",
            "5. DEFAULT-DENY deeper history; set TURINGOS_MARKOV_OVERRIDE=1 to override (audit-only)"
        ],
    });
    let next_session_bytes = serde_json::to_vec(&next_session_json)
        .map_err(|e| MarkovGenError::Encode(format!("next_session_context: {e}")))?;
    let next_session_context_cid = cas_w.put(
        &next_session_bytes,
        ObjectType::NextSessionContext,
        creator_str,
        created_at_logical_t,
        Some("v1/next_session_context".into()),
    )?;

    // Step 2: build capsule with capsule_id = 0 + sha256 = 0.
    let mut capsule = MarkovEvidenceCapsule {
        capsule_id: Cid::default(),
        previous_capsule_cid,
        constitution_hash,
        flowchart_hashes,
        l4_root,
        l4e_root,
        cas_root,
        typical_errors,
        unresolved_obs,
        next_session_context_cid,
        sha256: Hash::ZERO,
        created_at_logical_t,
        tb_tag,
    };

    // R3 closure (Codex R2 VETO Q3 + TB15-CAS-ID): the CAS-stored bytes
    // MUST be the bytes whose sha256 equals `capsule_id`, otherwise
    // `cas.get(&capsule.capsule_id)` fails. PRIOR BUG: we computed
    // `capsule_id = sha256(prelim_bytes)` then stored DIFFERENT
    // `final_bytes` (with capsule_id+sha256 populated) — those final
    // bytes had a different sha256, so the CAS index keyed under that
    // different sha256, and the published `capsule_id` was unresolvable.
    //
    // FIX: store the prelim bytes (with capsule_id=Cid::default,
    // sha256=Hash::ZERO). The in-memory `MarkovEvidenceCapsule`
    // returned to the caller has populated capsule_id/sha256 for
    // ergonomic use; on-disk CAS bytes have these fields zeroed.
    // Consumers reading from CAS:
    //   1. Fetch bytes via cas.get(&capsule.capsule_id) → SUCCEEDS
    //      (capsule_id == sha256(stored_bytes))
    //   2. canonical_decode(bytes) → struct with capsule_id=default
    //   3. Re-derive capsule_id := sha256(bytes); set field; set sha256
    //      identical to the in-memory writer-returned struct.
    let stored_bytes = canonical_encode(&capsule)
        .map_err(|e| MarkovGenError::Encode(format!("capsule canonical encode: {e:?}")))?;
    let cid = Cid::from_content(&stored_bytes);
    let cas_returned_cid = cas_w.put(
        &stored_bytes,
        ObjectType::MarkovEvidenceCapsule,
        creator_str,
        created_at_logical_t,
        Some("v1/markov_evidence_capsule".into()),
    )?;
    debug_assert_eq!(
        cas_returned_cid, cid,
        "CAS-returned cid must equal sha256(stored_bytes); CasStore::put contract"
    );
    // Populate the struct fields from the derived cid. The struct
    // returned to the caller is the ergonomic view; on-CAS bytes have
    // capsule_id+sha256 zeroed so cas.get(&cid) is retrievable.
    capsule.capsule_id = cid;
    capsule.sha256 = Hash(cid.0);

    Ok(capsule)
}

/// TRACE_MATRIX TB-15 R3 closure (Codex R2 VETO Q3): rebuild a
/// `MarkovEvidenceCapsule` from CAS-resident bytes. Caller supplies the
/// bytes returned by `cas.get(&capsule_id)`. This helper canonical-
/// decodes them and re-derives `capsule_id` + `sha256` from
/// `Cid::from_content(&bytes)`, returning the ergonomic in-memory view
/// identical to what `write_markov_capsule` returned at write time.
///
/// Invariant: for any capsule written by `write_markov_capsule`,
/// `restore_markov_capsule_from_cas_bytes(cas.get(&cap.capsule_id)?) ==
///  cap` (modulo non-stored field ordering — both have populated
/// capsule_id/sha256).
pub fn restore_markov_capsule_from_cas_bytes(
    bytes: &[u8],
) -> Result<MarkovEvidenceCapsule, MarkovGenError> {
    use crate::bottom_white::ledger::transition_ledger::canonical_decode;
    let mut cap: MarkovEvidenceCapsule = canonical_decode(bytes)
        .map_err(|e| MarkovGenError::Encode(format!("capsule decode: {e:?}")))?;
    let cid = Cid::from_content(bytes);
    cap.capsule_id = cid;
    cap.sha256 = Hash(cid.0);
    Ok(cap)
}

/// TRACE_MATRIX TB-15 Atom 5: scan `<repo>/handover/alignment/OBS_*.md`
/// for unresolved-observation file paths. Pure read; returns sorted
/// `Vec<ObsId>` (BTreeSet ordering) for replay-determinism. CR-15.5 —
/// capsule references existing files, never mints new ones.
pub fn scan_unresolved_obs(alignment_dir: &std::path::Path) -> Result<Vec<ObsId>, MarkovGenError> {
    use std::collections::BTreeSet;
    let mut out: BTreeSet<String> = BTreeSet::new();
    if !alignment_dir.is_dir() {
        return Ok(Vec::new());
    }
    for entry in std::fs::read_dir(alignment_dir)? {
        let entry = entry?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str.starts_with("OBS_") && name_str.ends_with(".md") {
            out.insert(format!("handover/alignment/{}", name_str));
        }
    }
    Ok(out.into_iter().map(ObsId).collect())
}

/// TRACE_MATRIX TB-15 Atom 5: deterministic SHA-256 of constitution.md
/// at the given path. Public so binaries + tests can compute it
/// uniformly. CR-15.5 + SG-15.7.
pub fn sha256_of_file(path: &std::path::Path) -> Result<Hash, MarkovGenError> {
    use sha2::{Digest, Sha256};
    let bytes = std::fs::read(path)?;
    let mut h = Sha256::new();
    h.update(&bytes);
    let digest: [u8; 32] = h.finalize().into();
    Ok(Hash(digest))
}

/// TRACE_MATRIX TB-15 R2 closure (Codex R1 Q8/RQ7 + Gemini R1 Q7): parse
/// `handover/alignment/TRACE_FLOWCHART_MATRIX.md` §2 and return the four
/// canonical flowchart hashes in canonical order (1a, 1b, 2, 3).
///
/// Format the parser expects (verbatim from the matrix file §2):
/// ```text
/// Flowchart 1a — ...
///   SHA256: <64-hex>
/// Flowchart 1b — ...
///   SHA256: <64-hex>
/// Flowchart 2 — ...
///   SHA256: <64-hex>
/// Flowchart 3 — ...
///   SHA256: <64-hex>
/// ```
///
/// Returns exactly 4 hashes in canonical order. Errors if any
/// flowchart's hash is not parseable or the count is not 4.
/// CR-15.5 (capsule references existing canonical artifact) + SG-15.7
/// literal "constitution hash AND flowchart hashes".
pub fn read_flowchart_hashes_from_matrix(
    matrix_path: &std::path::Path,
) -> Result<Vec<Hash>, MarkovGenError> {
    let body = std::fs::read_to_string(matrix_path)?;
    let mut hashes = Vec::with_capacity(4);
    for label in &[
        "Flowchart 1a",
        "Flowchart 1b",
        "Flowchart 2 ",
        "Flowchart 3 ",
    ] {
        // Find label, then the next "SHA256:" line.
        let label_pos = body.find(label).ok_or_else(|| {
            MarkovGenError::Encode(format!("flowchart label `{}` not found in matrix", label))
        })?;
        let after = &body[label_pos..];
        let sha_pos = after.find("SHA256:").ok_or_else(|| {
            MarkovGenError::Encode(format!("SHA256: marker not found after `{}`", label))
        })?;
        let hex_start = label_pos + sha_pos + "SHA256:".len();
        // Skip whitespace; take next 64 hex chars.
        let rest = &body[hex_start..];
        let hex_str: String = rest
            .chars()
            .skip_while(|c| c.is_whitespace())
            .take(64)
            .collect();
        if hex_str.len() != 64 {
            return Err(MarkovGenError::Encode(format!(
                "flowchart `{}` hash is {} chars, expected 64",
                label,
                hex_str.len()
            )));
        }
        let mut bytes = [0u8; 32];
        for i in 0..32 {
            bytes[i] = u8::from_str_radix(&hex_str[i * 2..i * 2 + 2], 16).map_err(|e| {
                MarkovGenError::Encode(format!("flowchart `{}` byte {}: {}", label, i, e))
            })?;
        }
        hashes.push(Hash(bytes));
    }
    if hashes.len() != 4 {
        return Err(MarkovGenError::Encode(format!(
            "expected 4 flowchart hashes; got {}",
            hashes.len()
        )));
    }
    Ok(hashes)
}

/// Hex helper for next-session JSON formatting (32-byte hashes).
fn hex(bytes: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    /// TB-15 Atom 5 — capsule default round-trips through canonical bytes.
    #[test]
    fn markov_capsule_default_round_trip() {
        use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
        let c = MarkovEvidenceCapsule::default();
        let bytes = canonical_encode(&c).expect("encode");
        let back: MarkovEvidenceCapsule = canonical_decode(&bytes).expect("decode");
        assert_eq!(c, back);
    }

    /// TB-15 Atom 5 — with_constitution_hash plumbs the hash through.
    #[test]
    fn with_constitution_hash_sets_field() {
        let hash = [0xABu8; 32];
        let c = MarkovEvidenceCapsule::with_constitution_hash(hash);
        assert_eq!(c.constitution_hash.0, hash);
    }

    /// TB-15 Atom 5 — try_deep_history_read_with_override_check:
    /// false → Err(DeepHistoryReadDenied); true → Ok(()).
    #[test]
    fn deep_history_default_deny_works() {
        match try_deep_history_read_with_override_check(false) {
            Err(MarkovGenError::DeepHistoryReadDenied) => {}
            other => panic!("expected DeepHistoryReadDenied; got {other:?}"),
        }
        assert!(try_deep_history_read_with_override_check(true).is_ok());
    }

    /// TB-15 Atom 5 — write_markov_capsule writes 2 CAS objects
    /// (next_session_context + capsule), and capsule_id is the
    /// canonical sha256 (with field zeroed for prelim encode).
    #[test]
    fn write_markov_capsule_to_cas_round_trip() {
        use std::sync::{Arc, RwLock};
        use tempfile::TempDir;

        let tmp = TempDir::new().expect("tempdir");
        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));

        let mut h = Sha256::new();
        h.update(b"fake constitution body");
        let constitution_hash: [u8; 32] = h.finalize().into();

        let cap = write_markov_capsule(
            &cas,
            None, // genesis Markov
            Hash(constitution_hash),
            vec![
                Hash([0xAAu8; 32]),
                Hash([0xBBu8; 32]),
                Hash([0xCCu8; 32]),
                Hash([0xDDu8; 32]),
            ],
            Hash([0x01u8; 32]),
            Hash([0x02u8; 32]),
            Hash([0x03u8; 32]),
            vec![],
            vec![ObsId("handover/alignment/OBS_X.md".into())],
            "TB-15".into(),
            "tb15-test-writer",
            7,
        )
        .expect("writer succeeds");

        assert_ne!(cap.capsule_id, Cid::default());
        assert_eq!(cap.capsule_id.0, cap.sha256.0);
        assert_ne!(cap.next_session_context_cid, Cid::default());
        assert_eq!(cap.constitution_hash.0, constitution_hash);
        assert_eq!(cap.flowchart_hashes.len(), 4);

        let cas_r = cas.read().expect("cas read");
        assert_eq!(
            cas_r.len(),
            2,
            "writer puts 2 CAS objects: next_session_context + capsule"
        );
    }

    /// TB-15 Atom 5 — write_markov_capsule deterministic (same inputs →
    /// same capsule_id + same next_session_context_cid).
    #[test]
    fn write_markov_capsule_deterministic_capsule_id() {
        use std::sync::{Arc, RwLock};
        use tempfile::TempDir;

        let mk = || {
            let tmp = TempDir::new().unwrap();
            let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).unwrap()));
            write_markov_capsule(
                &cas,
                Some(Cid([0xCDu8; 32])),
                Hash([0x10u8; 32]),
                vec![],
                Hash([0x20u8; 32]),
                Hash([0x30u8; 32]),
                Hash([0x40u8; 32]),
                vec![],
                vec![],
                "TB-15-det".into(),
                "writer",
                42,
            )
            .expect("writer")
        };
        let a = mk();
        let b = mk();
        assert_eq!(a.capsule_id, b.capsule_id);
        assert_eq!(a.next_session_context_cid, b.next_session_context_cid);
    }

    /// TB-15 Atom 5 — Markov chain: each capsule references the prior
    /// via `previous_capsule_cid`.
    #[test]
    fn markov_chain_links_via_previous_capsule_cid() {
        use std::sync::{Arc, RwLock};
        use tempfile::TempDir;

        let tmp = TempDir::new().unwrap();
        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).unwrap()));

        let cap_1 = write_markov_capsule(
            &cas,
            None,
            Hash([0x01u8; 32]),
            vec![],
            Hash([0x10u8; 32]),
            Hash([0x20u8; 32]),
            Hash([0x30u8; 32]),
            vec![],
            vec![],
            "TB-15-prev".into(),
            "writer",
            1,
        )
        .expect("cap 1");
        let cap_2 = write_markov_capsule(
            &cas,
            Some(cap_1.capsule_id),
            Hash([0x01u8; 32]),
            vec![],
            Hash([0x11u8; 32]), // L4 advanced
            Hash([0x20u8; 32]),
            Hash([0x30u8; 32]),
            vec![],
            vec![],
            "TB-15-next".into(),
            "writer",
            2,
        )
        .expect("cap 2");

        assert_eq!(cap_2.previous_capsule_cid, Some(cap_1.capsule_id));
        assert_ne!(cap_1.capsule_id, cap_2.capsule_id);
    }

    /// TB-15 Atom 5 — sha256_of_file matches manual sha256.
    #[test]
    fn sha256_of_file_matches_manual() {
        use std::io::Write;
        use tempfile::NamedTempFile;
        let mut f = NamedTempFile::new().unwrap();
        let body = b"test constitution body";
        f.write_all(body).unwrap();
        let path = f.path().to_path_buf();
        let computed = sha256_of_file(&path).expect("sha256");
        let mut h = Sha256::new();
        h.update(body);
        let manual: [u8; 32] = h.finalize().into();
        assert_eq!(computed.0, manual);
    }

    /// TB-15 Atom 5 — scan_unresolved_obs picks up OBS_*.md files only,
    /// in sorted order.
    #[test]
    fn scan_unresolved_obs_filters_and_sorts() {
        use std::fs;
        use tempfile::TempDir;
        let tmp = TempDir::new().unwrap();
        let p = tmp.path();
        fs::write(p.join("OBS_zeta.md"), "z").unwrap();
        fs::write(p.join("OBS_alpha.md"), "a").unwrap();
        fs::write(p.join("not_an_obs.md"), "x").unwrap();
        fs::write(p.join("OBS_beta.txt"), "b").unwrap(); // wrong extension

        let obs = scan_unresolved_obs(p).expect("scan");
        assert_eq!(obs.len(), 2);
        // Sorted (BTreeSet semantics).
        assert!(obs[0].0.ends_with("OBS_alpha.md"));
        assert!(obs[1].0.ends_with("OBS_zeta.md"));
    }

    /// TB-15 R2 closure (Codex Q8/RQ7 + Gemini Q7): read_flowchart_hashes_from_matrix
    /// returns 4 hashes in canonical order (1a, 1b, 2, 3).
    #[test]
    fn read_flowchart_hashes_from_real_matrix() {
        let manifest = env!("CARGO_MANIFEST_DIR");
        let path =
            std::path::PathBuf::from(manifest).join("handover/alignment/TRACE_FLOWCHART_MATRIX.md");
        let hashes = read_flowchart_hashes_from_matrix(&path).expect("matrix parse");
        assert_eq!(hashes.len(), 4, "exactly 4 canonical flowchart hashes");
        // Architect 2026-05-02 §2 verbatim canonical hashes.
        let expected_1a = "a474c6b9ded766504a4f644a4a1b3c545316d418f0250f36ec692fcdf98f09f5";
        let expected_1b = "b822717b10332a2d8e789ba6af96fd4da4ff43a74afab679d1b82add9c32b64d";
        let expected_2 = "6a4bc9195bafd55bde968fd445cdd2926d6906a7f6a2b38071d4774a7f0de333";
        let expected_3 = "c159413984d0c6c5daa06605fea3a86a2ad4ab9c4284d0d20e0e525bf03aa9cd";
        assert_eq!(hex(&hashes[0].0), expected_1a, "FC 1a");
        assert_eq!(hex(&hashes[1].0), expected_1b, "FC 1b");
        assert_eq!(hex(&hashes[2].0), expected_2, "FC 2");
        assert_eq!(hex(&hashes[3].0), expected_3, "FC 3");
    }

    /// TB-15 R3 closure (Codex R2 VETO Q3): cas.get(&cap.capsule_id)
    /// MUST succeed; the retrieved bytes MUST canonical-decode +
    /// restore to the in-memory capsule. This closes the SG-15.3
    /// "next session can bootstrap from latest Markov capsule" gate
    /// at the structural level.
    #[test]
    fn write_markov_capsule_cas_resolvable_by_capsule_id() {
        use std::sync::{Arc, RwLock};
        use tempfile::TempDir;

        let tmp = TempDir::new().expect("tempdir");
        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
        let cap = write_markov_capsule(
            &cas,
            None,
            Hash([0x42u8; 32]),
            vec![
                Hash([0xAAu8; 32]),
                Hash([0xBBu8; 32]),
                Hash([0xCCu8; 32]),
                Hash([0xDDu8; 32]),
            ],
            Hash([0x10u8; 32]),
            Hash([0x20u8; 32]),
            Hash([0x30u8; 32]),
            vec![],
            vec![ObsId("handover/alignment/OBS_X.md".into())],
            "TB-15-R3".into(),
            "tb15-r3-test",
            42,
        )
        .expect("writer");

        // R3 contract: cas.get(&cap.capsule_id) MUST succeed.
        let cas_r = cas.read().expect("cas read");
        let retrieved = cas_r
            .get(&cap.capsule_id)
            .expect("R3 contract: cas.get(&capsule.capsule_id) MUST succeed");

        // The retrieved bytes' sha256 MUST equal capsule.capsule_id.
        assert_eq!(
            Cid::from_content(&retrieved),
            cap.capsule_id,
            "R3 contract: sha256(retrieved bytes) == capsule.capsule_id"
        );

        // canonical_decode + restore reproduces the in-memory capsule.
        let restored =
            restore_markov_capsule_from_cas_bytes(&retrieved).expect("restore from CAS bytes");
        assert_eq!(
            restored.capsule_id, cap.capsule_id,
            "R3 contract: restored capsule_id matches written capsule_id"
        );
        assert_eq!(restored.constitution_hash, cap.constitution_hash);
        assert_eq!(restored.flowchart_hashes, cap.flowchart_hashes);
        assert_eq!(restored.tb_tag, cap.tb_tag);
        assert_eq!(restored.created_at_logical_t, cap.created_at_logical_t);

        // next_session_context_cid MUST also be resolvable.
        let _next_ctx = cas_r
            .get(&cap.next_session_context_cid)
            .expect("R3 contract: cas.get(&next_session_context_cid) MUST succeed");
    }

    /// TB-15 R2 closure: capsule.flowchart_hashes round-trips through canonical encode.
    #[test]
    fn capsule_flowchart_hashes_round_trip() {
        use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
        let mut c = MarkovEvidenceCapsule::default();
        c.flowchart_hashes = vec![
            Hash([0xAAu8; 32]),
            Hash([0xBBu8; 32]),
            Hash([0xCCu8; 32]),
            Hash([0xDDu8; 32]),
        ];
        let bytes = canonical_encode(&c).expect("encode");
        let back: MarkovEvidenceCapsule = canonical_decode(&bytes).expect("decode");
        assert_eq!(back.flowchart_hashes.len(), 4);
        assert_eq!(back.flowchart_hashes[0].0, [0xAAu8; 32]);
    }
}
