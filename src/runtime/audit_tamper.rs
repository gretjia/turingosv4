//! Library API for the audit-tape tamper-detection harness.
//!
//! Three corruption primitives, each guaranteed to make `audit_tape` return
//! `BLOCK` on the tampered copy:
//!
//!   1. [`flip_largest_reachable_l4_blob`] — destructively zero the back half
//!      of the largest L4-reachable loose object in the runtime_repo. Forces
//!      git2 zlib decode to fail on the next read of that object, which
//!      cascades into Layer-B assertion HALTs.
//!   2. [`flip_largest_cas_object`] — same idea, applied to the largest CAS
//!      loose object. Causes Cid mismatch + zlib failure on resolve.
//!   3. [`corrupt_chain_refs`] — zero the last 4 hex chars of every chain ref
//!      in the runtime_repo (`refs/chaintape/{l4,l4e}` + alias
//!      `refs/transitions/main`). Makes the refs unresolvable so
//!      `Git2LedgerWriter::open` errors → audit BLOCK.
//!
//! ## Authority
//!
//! Tamper-detection coverage is mandated by architect §B.9.3 ("prove no fake
//! accepted") and design §6.2 H + §7.7. The TB-16-era originals lived inside
//! `src/bin/audit_tape_tamper.rs`; they were authored 2026-05-04 (commit
//! `f2bb871`). Stage A3 (2026-05-08) introduced multi-ref ChainTape
//! (`refs/chaintape/{l4,l4e,cas}`) per CR-A3-HEAD-T-C2.5+6, but the tamper
//! primitives were not updated. M0 batch 2026-05-10 surfaced the regression:
//! `flip_byte_in_first_blob` selected the largest loose object regardless of
//! reachability, hitting orphan blobs (audit never reads → silent); and
//! `corrupt_l4_truncate_ref` truncated only the legacy alias
//! `refs/transitions/main`, leaving canonical `refs/chaintape/l4` intact.
//! Result: 1/3 detection vs architect-mandated 3/3.
//!
//! ## Constitutional posture
//!
//! Per `feedback_no_workarounds_strict_constitution` ("我不要凑活"): tamper
//! coverage at 1/3 is a constitutional violation of architect §B.9.3, not a
//! cosmetic flake. This library encodes the corrected primitives and is
//! exercised by `tests/constitution_audit_tamper_3_of_3.rs` so future drift
//! is caught at gate-time.
//!
//! `FC-trace: FC1-N35 + FC2-INV1`.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use git2::{Oid, Repository, TreeWalkMode, TreeWalkResult};

/// TRACE_MATRIX FC1-N35 (audit_tape_tamper coverage; architect §B.9.3
/// prove-no-fake-accepted): L4 refs whose reachable bodies are deep-verified
/// by `audit_tape` (entry_canonical + payload_cid + signature for each
/// accepted transition). `flip_largest_reachable_l4_blob` walks only these
/// so the corrupted blob is guaranteed to be on the audit's read path.
///
/// `refs/chaintape/l4e` is intentionally absent: L4 + L4.E have separate
/// deep-verify paths in `audit_assertions::run_all_assertions`. L4 bodies
/// are deep-verified by Layer B assertions #04/#05/#08-#11/#50 (state-root
/// continuity + signatures + payload CID + bytes-vs-CID); L4.E bodies are
/// deep-verified by assertion #51 `l4e_git_attestation_matches_jsonl`
/// (added 2026-05-10 session #34) which walks `refs/chaintape/l4e` and
/// asserts each commit's `rejection_record` blob hash matches the
/// JSONL-side record. Tamper coverage for L4.E uses [`L4E_REFS`] +
/// [`flip_largest_reachable_l4e_blob`].
pub const L4_REFS: &[&str] = &[
    "refs/chaintape/l4",     // Stage A3 canonical accepted-transition head
    "refs/transitions/main", // pre-A3 / backward-compat alias (CR-A3-HEAD-T-C2.6)
];

/// TRACE_MATRIX FC1-N35 + FC1-N34 (audit_tape_tamper coverage; architect
/// §B.9.3 prove-no-fake-accepted L4.E-side coverage; session #34
/// L4.E-body-integrity landing): refs whose reachable blob bodies are
/// deep-verified by `assert_51_l4e_git_attestation_matches_jsonl`.
/// `flip_largest_reachable_l4e_blob` walks only these so the corrupted
/// blob is guaranteed to be on the L4.E audit's read path.
///
/// Single-element today (no alias for L4.E). If a future stage introduces
/// a parallel rejection-chain ref, add it here AND update the assertion
/// walker.
pub const L4E_REFS: &[&str] = &[
    "refs/chaintape/l4e", // Stage A3 canonical rejection head
];

/// TRACE_MATRIX FC1-N35 (audit_tape_tamper coverage; architect §B.9.3):
/// all chain refs (L4 + L4.E + alias). Used by `corrupt_chain_refs` so a
/// dual-write tape is broken regardless of which ref the audit reads.
/// `corrupt_chain_refs` corrupts every ref in this list that exists.
pub const CHAIN_REFS: &[&str] = &[
    "refs/chaintape/l4",     // Stage A3 canonical accepted-transition head
    "refs/chaintape/l4e",    // Stage A3 canonical rejection head
    "refs/transitions/main", // pre-A3 / backward-compat alias (CR-A3-HEAD-T-C2.6)
];

fn make_writable(path: &Path) -> std::io::Result<()> {
    let mut perms = std::fs::metadata(path)?.permissions();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        perms.set_mode(0o644);
    }
    #[cfg(not(unix))]
    {
        perms.set_readonly(false);
    }
    std::fs::set_permissions(path, perms)
}

/// Walk every commit + tree + blob reachable from any ref in `refs_to_walk`
/// that exists in the repo. Returns the set of reachable Oids. Errors if
/// NONE of the supplied refs exist.
fn collect_reachable_oids(
    repo: &Repository,
    refs_to_walk: &[&str],
) -> Result<HashSet<Oid>, String> {
    let mut oids: HashSet<Oid> = HashSet::new();
    let mut found_any = false;
    for ref_name in refs_to_walk {
        let reference = match repo.find_reference(ref_name) {
            Ok(r) => r,
            Err(_) => continue,
        };
        let target = match reference.target() {
            Some(t) => t,
            None => continue,
        };
        found_any = true;
        let mut walk = repo
            .revwalk()
            .map_err(|e| format!("revwalk for {ref_name}: {e}"))?;
        walk.push(target)
            .map_err(|e| format!("revwalk push {ref_name} → {target}: {e}"))?;
        for commit_oid in walk {
            let commit_oid = commit_oid.map_err(|e| format!("revwalk iter: {e}"))?;
            oids.insert(commit_oid);
            if let Ok(commit) = repo.find_commit(commit_oid) {
                if let Ok(tree) = commit.tree() {
                    oids.insert(tree.id());
                    let _ = tree.walk(TreeWalkMode::PreOrder, |_, entry| {
                        oids.insert(entry.id());
                        TreeWalkResult::Ok
                    });
                }
            }
        }
    }
    if !found_any {
        return Err(format!(
            "no chain refs found in {:?}; expected at least one of {:?}",
            repo.path(),
            refs_to_walk
        ));
    }
    Ok(oids)
}

/// Walk loose objects under `repo/.git/objects/`, return only those whose
/// reconstructed Oid is in `reachable`. Returns (path, byte_len, oid) tuples
/// sorted by descending byte length so callers can pick the largest.
fn loose_objects_reachable(
    repo_path: &Path,
    reachable: &HashSet<Oid>,
) -> Result<Vec<(PathBuf, u64, Oid)>, String> {
    let objects = repo_path.join(".git").join("objects");
    let mut found: Vec<(PathBuf, u64, Oid)> = Vec::new();
    walk_loose(&objects, reachable, &mut found).map_err(|e| format!("walk objects: {e}"))?;
    found.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(found)
}

fn walk_loose(
    dir: &Path,
    reachable: &HashSet<Oid>,
    found: &mut Vec<(PathBuf, u64, Oid)>,
) -> std::io::Result<()> {
    let read = match std::fs::read_dir(dir) {
        Ok(r) => r,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e),
    };
    for entry in read {
        let e = entry?;
        let p = e.path();
        if p.is_dir() {
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            // Skip pack/info — we only target loose objects in xx/yyyy.. layout.
            if name == "pack" || name == "info" {
                continue;
            }
            walk_loose(&p, reachable, found)?;
        } else {
            // Reconstruct OID from path: <repo>/.git/objects/xx/yyyy.. (38 hex)
            let parent_name = p
                .parent()
                .and_then(|x| x.file_name())
                .and_then(|x| x.to_str())
                .unwrap_or("");
            let file_name = p.file_name().and_then(|x| x.to_str()).unwrap_or("");
            if parent_name.len() == 2 && file_name.len() == 38 {
                let oid_hex = format!("{}{}", parent_name, file_name);
                if let Ok(oid) = Oid::from_str(&oid_hex) {
                    if reachable.contains(&oid) {
                        let len = std::fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
                        found.push((p, len, oid));
                    }
                }
            }
        }
    }
    Ok(())
}

/// Zero the back half of `victim` in-place. Caller pre-flight: `make_writable`.
fn destructively_zero_back_half(victim: &Path) -> Result<u64, String> {
    let mut bytes = std::fs::read(victim).map_err(|e| format!("read victim: {e}"))?;
    if bytes.is_empty() {
        return Err("empty victim".into());
    }
    let original_len = bytes.len() as u64;
    let start = bytes.len() / 2;
    for b in &mut bytes[start..] {
        *b = 0;
    }
    make_writable(victim).map_err(|e| format!("chmod victim: {e}"))?;
    std::fs::write(victim, bytes).map_err(|e| format!("write tampered: {e}"))?;
    Ok(original_len / 2)
}

/// TRACE_MATRIX FC1-N35 (audit_tape_tamper Atom 3 / TB-16 Atom 7 closure;
/// architect §B.9.3 prove-no-fake-accepted L4-side coverage):
/// walk objects under `runtime_repo/.git/objects/`, restrict to those
/// reachable from any of `L4_REFS`, pick the largest by byte length, zero
/// its back half. Forces git2 zlib decode failure when `audit_tape` next
/// reads it.
///
/// Post-A3 fix (2026-05-10): the prior `flip_byte_in_first_blob` selected
/// the largest object regardless of reachability, which post-A3 multi-ref
/// + chain preseed could pick orphan blobs the audit never reads → false
/// PASS. Per `feedback_no_workarounds_strict_constitution`.
pub fn flip_largest_reachable_l4_blob(repo_path: &Path) -> Result<String, String> {
    let repo = Repository::open(repo_path).map_err(|e| format!("open repo: {e}"))?;
    // L4 only — see `L4_REFS` rustdoc for the L4 / L4.E split rationale.
    let reachable = collect_reachable_oids(&repo, L4_REFS)?;
    let candidates = loose_objects_reachable(repo_path, &reachable)?;
    let (victim, _len, oid) = candidates.into_iter().next().ok_or_else(|| {
        format!(
            "no reachable loose objects under {:?}/.git/objects (reachable oids: {})",
            repo_path,
            reachable.len()
        )
    })?;
    let zeroed = destructively_zero_back_half(&victim)?;
    Ok(format!(
        "destructively zeroed back half ({zeroed} bytes) of largest \
         L4-reachable loose object {oid} ({victim:?}) — forces git2 zlib \
         decode failure when audit_tape next reads"
    ))
}

/// TRACE_MATRIX FC1-N35 + FC1-N34 (audit_tape_tamper L4.E-side coverage;
/// architect §B.9.3 prove-no-fake-accepted; session #34 L4.E-body-integrity
/// landing): walk objects under `runtime_repo/.git/objects/`, restrict to
/// those reachable from any of [`L4E_REFS`], pick the largest by byte
/// length, zero its back half. Forces git2 zlib decode failure when
/// `assert_51_l4e_git_attestation_matches_jsonl` next reads it.
///
/// L4.E-only — the L4 path uses [`flip_largest_reachable_l4_blob`]. The
/// audit's L4.E deep-verify path is `assert_51_l4e_git_attestation_matches_jsonl`.
pub fn flip_largest_reachable_l4e_blob(repo_path: &Path) -> Result<String, String> {
    let repo = Repository::open(repo_path).map_err(|e| format!("open repo: {e}"))?;
    let reachable = collect_reachable_oids(&repo, L4E_REFS)?;
    let candidates = loose_objects_reachable(repo_path, &reachable)?;
    let (victim, _len, oid) = candidates.into_iter().next().ok_or_else(|| {
        format!(
            "no reachable loose objects under {:?}/.git/objects (L4E reachable oids: {})",
            repo_path,
            reachable.len()
        )
    })?;
    let zeroed = destructively_zero_back_half(&victim)?;
    Ok(format!(
        "destructively zeroed back half ({zeroed} bytes) of largest \
         L4E-reachable loose object {oid} ({victim:?}) — forces git2 zlib \
         decode failure when assert_51 next reads"
    ))
}

/// TRACE_MATRIX FC1-N35 (audit_tape_tamper Atom 3 / TB-16 Atom 7 closure;
/// architect §B.9.3 prove-no-fake-accepted CAS-side coverage):
/// pick largest CAS loose object by byte length and zero back half.
///
/// CAS dir is single-ref (no multi-ref aliasing); largest-by-bytes is
/// adequate. Skips files ≤32 bytes (header noise; not real CAS objects).
/// Forces Cid mismatch + zlib decode failure when audit resolves the CID.
pub fn flip_largest_cas_object(cas: &Path) -> Result<String, String> {
    let objects = cas.join(".git").join("objects");
    let dir = if objects.exists() {
        objects
    } else {
        cas.to_path_buf()
    };
    let mut largest: Option<(PathBuf, u64)> = None;
    fn walk(dir: &Path, largest: &mut Option<(PathBuf, u64)>) -> std::io::Result<()> {
        let read = match std::fs::read_dir(dir) {
            Ok(r) => r,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(e),
        };
        for entry in read {
            let e = entry?;
            let p = e.path();
            if p.is_dir() {
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == "pack" || name == "info" {
                    continue;
                }
                walk(&p, largest)?;
            } else {
                let len = std::fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
                if len > 32 && largest.as_ref().map(|(_, n)| *n).unwrap_or(0) < len {
                    *largest = Some((p, len));
                }
            }
        }
        Ok(())
    }
    walk(&dir, &mut largest).map_err(|e| format!("walk: {e}"))?;
    let (victim, len) = largest.ok_or("no CAS objects to corrupt")?;
    let zeroed = destructively_zero_back_half(&victim)?;
    Ok(format!(
        "destructively zeroed back half ({zeroed} bytes; original {len}) of largest \
         CAS object {victim:?}"
    ))
}

/// TRACE_MATRIX FC1-N35 + FC2-INV1 (audit_tape_tamper Atom 3 ref-truncation
/// closure; architect §B.9.3 + Stage A3 multi-ref dual-write):
/// zero the last 4 hex chars of EVERY chain ref present in the repo
/// (canonical `refs/chaintape/l4` + `refs/chaintape/l4e` + alias
/// `refs/transitions/main`). Returns Err if no chain ref exists.
///
/// Post-A3 fix (2026-05-10): the prior `corrupt_l4_truncate_ref` only
/// truncated `refs/transitions/main`. Stage A3 dual-write made
/// `refs/chaintape/l4` the canonical read path; truncating only the alias
/// was a no-op against multi-ref-aware audits. Per
/// `feedback_no_workarounds_strict_constitution`.
pub fn corrupt_chain_refs(repo_path: &Path) -> Result<String, String> {
    let mut tampered: Vec<String> = Vec::new();
    for ref_name in CHAIN_REFS {
        let path = repo_path.join(".git").join(ref_name);
        if !path.exists() {
            continue;
        }
        let s = std::fs::read_to_string(&path).map_err(|e| format!("read {ref_name}: {e}"))?;
        if s.len() < 5 {
            continue; // too short to corrupt safely
        }
        let mut chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        for i in (n - 5)..(n - 1) {
            chars[i] = '0';
        }
        let zeroed: String = chars.into_iter().collect();
        make_writable(&path).map_err(|e| format!("chmod {ref_name}: {e}"))?;
        std::fs::write(&path, zeroed).map_err(|e| format!("write {ref_name}: {e}"))?;
        tampered.push((*ref_name).to_string());
    }
    if tampered.is_empty() {
        return Err(format!(
            "no chain refs found to corrupt in {repo_path:?} (expected one of {:?})",
            CHAIN_REFS
        ));
    }
    Ok(format!(
        "zeroed last 4 hex chars in {} chain ref(s): {}",
        tampered.len(),
        tampered.join(", ")
    ))
}
