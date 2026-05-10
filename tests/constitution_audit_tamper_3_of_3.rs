//! Constitution gate — tamper-detection 3/3 invariant for `audit_tape_tamper`.
//!
//! Authority: architect §B.9.3 ("prove no fake accepted") + design §6.2 H +
//! §7.7. Tamper-detection coverage at 1/3 is a constitutional violation, not
//! a cosmetic flake. Per `feedback_no_workarounds_strict_constitution`
//! ("我不要凑活"): the M0 batch 2026-05-10 surfaced TB-16-era drift where
//! the in-binary tamper primitives stopped detecting L4 corruption after
//! Stage A3 multi-ref ChainTape (refs/chaintape/{l4,l4e,cas}) landed.
//!
//! Specifically:
//!   - `flip_byte_in_first_blob` (TB-16 era) selected the largest loose
//!     object regardless of reachability. Post-A3, runtime_repo contains
//!     orphan blobs (preseed artifacts) that are larger than any chain-
//!     reachable blob → tamper hit the orphan, audit never reads it,
//!     PROCEED, "not detected".
//!   - `corrupt_l4_truncate_ref` (TB-16 era) truncated only
//!     `refs/transitions/main`. Stage A3 made `refs/chaintape/l4` the
//!     canonical read path; truncating only the alias was a no-op.
//!
//! Fix: tamper primitives moved to library `runtime::audit_tamper` with:
//!   - reachability-aware blob selection (walk all chain refs, pick
//!     largest *reachable* loose object)
//!   - dual-ref corruption (corrupt every ref in `CHAIN_REFS` that exists)
//!
//! This gate is the executable face of the constitutional invariant:
//! BOTH primitives must operate on chain-relevant objects/refs, not stale
//! TB-16 assumptions. Future drift (e.g. introducing a new ref name in
//! Stage D+ without updating `CHAIN_REFS`) is caught here at gate-time.
//!
//! `FC-trace: FC1-N35 + FC2-INV1 + Art-0.4`.

use std::collections::HashSet;
use std::path::Path;

use git2::{Repository, Signature, Time};
use tempfile::TempDir;

use turingosv4::runtime::audit_tamper::{
    corrupt_chain_refs, flip_largest_cas_object, flip_largest_reachable_l4_blob, CHAIN_REFS,
    L4_REFS,
};

// ── Synthetic-repo fixtures ────────────────────────────────────────────────

/// Build a bare-bones git repo with a tiny commit history reachable from
/// each `ref_name` in `chain_refs`. Each commit holds a blob of `blob_size`
/// bytes plus a tiny tag-marker blob. Returns (TempDir, repo_path).
///
/// The "chain" objects (commit + tree + blob) are reachable from the
/// supplied refs. Optionally an orphan blob of `orphan_size` bytes is
/// added (NOT referenced by any commit) — the gate uses it to verify
/// reachability filtering.
fn build_synthetic_repo(
    chain_refs: &[&str],
    blob_size: usize,
    orphan_size: usize,
) -> (TempDir, std::path::PathBuf) {
    let tmp = TempDir::new().expect("tempdir");
    let path = tmp.path().to_path_buf();
    let repo = Repository::init_bare(&path).expect("init bare repo");

    // Create reachable blob (target of tamper). Use non-compressible
    // pseudorandom content so loose-file size approximates the byte count
    // (single-byte fill compresses to ~30 bytes regardless of input size,
    // breaking size-comparison fixture invariants).
    let blob_bytes: Vec<u8> = (0..blob_size).map(|i| seeded_byte(0xA5, i)).collect();
    let blob_oid = repo.blob(&blob_bytes).expect("blob");

    let mut tb = repo.treebuilder(None).expect("treebuilder");
    tb.insert("payload", blob_oid, 0o100644).expect("tree insert");
    let tree_oid = tb.write().expect("tree write");
    let tree = repo.find_tree(tree_oid).expect("find_tree");

    let time = Time::new(1714998000, 0);
    let sig = Signature::new("audit-tamper-gate", "gate@local", &time).expect("sig");
    let commit_oid = repo
        .commit(None, &sig, &sig, "synthetic l4 transition", &tree, &[])
        .expect("commit");

    for ref_name in chain_refs {
        repo.reference(ref_name, commit_oid, true, "synthetic L4 head")
            .unwrap_or_else(|e| panic!("write ref {ref_name}: {e}"));
    }

    if orphan_size > 0 {
        // Orphan must (a) have distinct content from the chain blob (different
        // seed) AND (b) be sufficiently non-compressible that its loose-file
        // size comfortably exceeds the chain blob's.
        let orphan_bytes: Vec<u8> = (0..orphan_size).map(|i| seeded_byte(0x5A, i)).collect();
        let orphan_oid = repo.blob(&orphan_bytes).expect("orphan blob");
        assert!(
            orphan_size > blob_size,
            "fixture invariant: orphan must be larger than chain blob",
        );
        // Confirm the orphan landed as a loose file (sanity for the test).
        let parent = path
            .join("objects")
            .join(&orphan_oid.to_string()[..2]);
        let file = parent.join(&orphan_oid.to_string()[2..]);
        assert!(
            file.exists(),
            "orphan blob {orphan_oid} did not land as loose object at {file:?}"
        );
    }

    (tmp, path)
}

/// Deterministic non-compressible byte stream. Byte at index `i` =
/// `(seed * 31 + i * 17 + (i >> 3) * 251) mod 256` xor with a feistel-ish
/// per-position salt. Not cryptographic; just ensures zlib can't shrink the
/// stream below ~95% of original length so loose-file size ≈ content size.
fn seeded_byte(seed: u8, i: usize) -> u8 {
    let s = seed as u64;
    let i64 = i as u64;
    let mix = s.wrapping_mul(31)
        .wrapping_add(i64.wrapping_mul(17))
        .wrapping_add((i64 >> 3).wrapping_mul(251))
        .wrapping_add((i64 >> 5).wrapping_mul(7919));
    (mix ^ (mix >> 8) ^ (mix >> 16)) as u8
}

/// Bare repos use `objects/` directly (no `.git/` prefix). The tamper
/// library assumes `repo/.git/objects/` for L4 blob walk. To exercise
/// the library against a bare repo, we mirror the bare repo's contents
/// into a `<dst>/.git/` layout.
fn wrap_bare_as_dotgit(bare_path: &Path) -> (TempDir, std::path::PathBuf) {
    let tmp = TempDir::new().expect("tempdir wrap");
    let dst = tmp.path().to_path_buf();
    let dotgit = dst.join(".git");
    std::fs::create_dir_all(&dotgit).expect("mkdir .git");
    copy_dir_recursive(bare_path, &dotgit).expect("copy bare to .git");
    (tmp, dst)
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else {
            std::fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

/// Read a loose object's bytes from `<repo>/.git/objects/xx/yyyy..`.
fn read_loose(repo_path: &Path, oid_hex: &str) -> Vec<u8> {
    let path = repo_path
        .join(".git")
        .join("objects")
        .join(&oid_hex[..2])
        .join(&oid_hex[2..]);
    std::fs::read(&path).unwrap_or_else(|e| panic!("read loose {oid_hex} at {path:?}: {e}"))
}

fn ref_value(repo_path: &Path, ref_name: &str) -> String {
    let path = repo_path.join(".git").join(ref_name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read ref {ref_name} at {path:?}: {e}"))
}

fn ref_exists(repo_path: &Path, ref_name: &str) -> bool {
    repo_path.join(".git").join(ref_name).exists()
}

// ── Tests ───────────────────────────────────────────────────────────────────

/// Constitutional invariant: `flip_largest_reachable_l4_blob` must select a
/// blob reachable from at least one of `L4_REFS` (NOT `L4_REFS ∪ L4E_REFS`).
/// Orphan blobs and L4.E-reachable blobs must NOT be selected, because the
/// audit only deep-verifies L4 entry_canonical bodies.
#[test]
fn flip_largest_reachable_l4_blob_picks_reachable_not_orphan_a3_canonical() {
    let (_bare_tmp, bare_path) =
        build_synthetic_repo(&["refs/chaintape/l4"], 200, 800);
    let (_wrapped_tmp, wrapped_path) = wrap_bare_as_dotgit(&bare_path);

    // List loose object OIDs + sizes for diagnostics.
    let chain_blob_hex = list_largest_loose_oid(&wrapped_path, /*reachable_only=*/ true);
    let any_largest_hex = list_largest_loose_oid(&wrapped_path, /*reachable_only=*/ false);
    assert_ne!(
        chain_blob_hex, any_largest_hex,
        "fixture invariant: orphan blob must be largest by bytes; chain blob must be largest reachable"
    );

    // Snapshot pre-tamper state of orphan + chain blobs.
    let orphan_pre = read_loose(&wrapped_path, &any_largest_hex);
    let chain_pre = read_loose(&wrapped_path, &chain_blob_hex);

    let detail = flip_largest_reachable_l4_blob(&wrapped_path)
        .expect("flip_largest_reachable_l4_blob should succeed on a repo with refs/chaintape/l4");

    // Detail must mention the chain blob's OID (not the orphan).
    assert!(
        detail.contains(&chain_blob_hex),
        "tamper detail must reference the L4-reachable blob OID {chain_blob_hex}; got: {detail}"
    );
    assert!(
        !detail.contains(&any_largest_hex),
        "tamper detail must NOT reference the orphan blob OID {any_largest_hex}; got: {detail}"
    );

    // Reachable blob must be CHANGED; orphan must be UNCHANGED.
    let chain_post = read_loose(&wrapped_path, &chain_blob_hex);
    let orphan_post = read_loose(&wrapped_path, &any_largest_hex);
    assert_ne!(
        chain_pre, chain_post,
        "L4-reachable chain blob must be tampered (was {} bytes)", chain_pre.len()
    );
    assert_eq!(
        orphan_pre, orphan_post,
        "orphan blob must be UNTOUCHED (reachability filter must protect non-chain objects)"
    );
}

/// Pre-A3 fallback: a repo with only `refs/transitions/main` (alias) and no
/// `refs/chaintape/*` must still tamper successfully via the alias entry in
/// `CHAIN_REFS`. Catches regression where multi-ref-only logic forgets the
/// alias.
#[test]
fn flip_largest_reachable_l4_blob_works_with_alias_only_pre_a3() {
    let (_bare_tmp, bare_path) =
        build_synthetic_repo(&["refs/transitions/main"], 200, 0);
    let (_wrapped_tmp, wrapped_path) = wrap_bare_as_dotgit(&bare_path);

    let detail = flip_largest_reachable_l4_blob(&wrapped_path)
        .expect("alias-only repo (pre-A3) must still allow tampering via refs/transitions/main");
    assert!(
        detail.contains("L4-reachable"),
        "detail string must indicate reachability filtering: {detail}"
    );
}

/// `flip_largest_reachable_l4_blob` MUST error if no chain ref exists.
/// Strict constitution: an empty-chain repo is unauditable, not silently OK.
#[test]
fn flip_largest_reachable_l4_blob_errors_on_no_chain_refs() {
    // Build repo with refs in a NON-chain namespace (refs/heads/main).
    let tmp = TempDir::new().expect("tempdir");
    let path = tmp.path().to_path_buf();
    let repo = Repository::init_bare(&path).expect("init bare");
    let blob_oid = repo.blob(b"x").expect("blob");
    let mut tb = repo.treebuilder(None).expect("tb");
    tb.insert("p", blob_oid, 0o100644).expect("ti");
    let tree_oid = tb.write().expect("tw");
    let tree = repo.find_tree(tree_oid).expect("ft");
    let sig = Signature::new("a", "b@c", &Time::new(0, 0)).expect("sig");
    let cid = repo.commit(None, &sig, &sig, "x", &tree, &[]).expect("c");
    repo.reference("refs/heads/main", cid, true, "x").expect("r");

    let (_w, wrapped) = wrap_bare_as_dotgit(&path);
    let res = flip_largest_reachable_l4_blob(&wrapped);
    let err = res.expect_err("must error on a repo with no CHAIN_REFS");
    assert!(
        err.contains("no chain refs"),
        "error message must mention 'no chain refs'; got: {err}"
    );
}

/// `corrupt_chain_refs` MUST corrupt every ref in `CHAIN_REFS` that exists.
/// Stage A3 dual-write tape has both `refs/chaintape/l4` and
/// `refs/transitions/main` pointing at the same OID; a tamper that touches
/// only one leaves the other intact and audit reads via the untouched one.
#[test]
fn corrupt_chain_refs_corrupts_all_present_refs_a3_dual_write() {
    let (_bare_tmp, bare_path) = build_synthetic_repo(
        &[
            "refs/chaintape/l4",
            "refs/chaintape/l4e",
            "refs/transitions/main",
        ],
        200,
        0,
    );
    let (_wrapped_tmp, wrapped_path) = wrap_bare_as_dotgit(&bare_path);

    // Snapshot pre-tamper ref values.
    let pre_l4 = ref_value(&wrapped_path, "refs/chaintape/l4");
    let pre_l4e = ref_value(&wrapped_path, "refs/chaintape/l4e");
    let pre_alias = ref_value(&wrapped_path, "refs/transitions/main");

    let detail = corrupt_chain_refs(&wrapped_path).expect("must succeed on multi-ref repo");

    let post_l4 = ref_value(&wrapped_path, "refs/chaintape/l4");
    let post_l4e = ref_value(&wrapped_path, "refs/chaintape/l4e");
    let post_alias = ref_value(&wrapped_path, "refs/transitions/main");

    assert_ne!(pre_l4, post_l4, "refs/chaintape/l4 MUST be corrupted (post-A3 canonical)");
    assert_ne!(
        pre_l4e, post_l4e,
        "refs/chaintape/l4e MUST be corrupted (post-A3 rejection canonical)"
    );
    assert_ne!(
        pre_alias, post_alias,
        "refs/transitions/main MUST be corrupted (pre-A3 / backward-compat alias)"
    );

    // Detail must mention the count "3" + each ref name (otherwise a future
    // refactor that silently skips one would not show up in the report).
    assert!(
        detail.contains("3 chain ref(s)"),
        "detail must report count 3 for a tape with all three refs; got: {detail}"
    );
    for ref_name in CHAIN_REFS {
        assert!(
            detail.contains(ref_name),
            "detail must list every corrupted ref by name; missing {ref_name}: {detail}"
        );
    }
}

/// Pre-A3 fallback: a repo with ONLY `refs/transitions/main` must still be
/// tampered (single-ref count = 1).
#[test]
fn corrupt_chain_refs_works_with_alias_only_pre_a3() {
    let (_bare_tmp, bare_path) = build_synthetic_repo(&["refs/transitions/main"], 200, 0);
    let (_wrapped_tmp, wrapped_path) = wrap_bare_as_dotgit(&bare_path);

    let detail = corrupt_chain_refs(&wrapped_path)
        .expect("alias-only repo (pre-A3) must still allow ref-truncation tamper");
    assert!(
        detail.contains("1 chain ref(s)"),
        "detail must report count 1 for alias-only repo; got: {detail}"
    );
    assert!(
        detail.contains("refs/transitions/main"),
        "detail must name the alias: {detail}"
    );
}

/// `corrupt_chain_refs` MUST error if no chain ref is present.
#[test]
fn corrupt_chain_refs_errors_on_no_chain_refs() {
    let tmp = TempDir::new().expect("tempdir");
    let path = tmp.path().to_path_buf();
    let repo = Repository::init_bare(&path).expect("init bare");
    let blob = repo.blob(b"x").expect("blob");
    let mut tb = repo.treebuilder(None).expect("tb");
    tb.insert("p", blob, 0o100644).expect("ti");
    let tree_oid = tb.write().expect("tw");
    let tree = repo.find_tree(tree_oid).expect("ft");
    let sig = Signature::new("a", "b@c", &Time::new(0, 0)).expect("sig");
    let cid = repo.commit(None, &sig, &sig, "x", &tree, &[]).expect("c");
    repo.reference("refs/heads/main", cid, true, "x").expect("r");

    let (_w, wrapped) = wrap_bare_as_dotgit(&path);
    for r in CHAIN_REFS {
        assert!(
            !ref_exists(&wrapped, r),
            "fixture invariant: no chain ref should exist; found {r}"
        );
    }

    let err = corrupt_chain_refs(&wrapped).expect_err("must error when no chain ref present");
    assert!(
        err.contains("no chain refs found"),
        "error must mention 'no chain refs found'; got: {err}"
    );
}

/// `flip_largest_cas_object` works on a CAS-shaped dir (no `.git` prefix
/// when the dir contains an `objects/` layout directly).
#[test]
fn flip_largest_cas_object_corrupts_largest() {
    // Build CAS-like dir: `cas/objects/xx/yyyy..` containing some loose blobs.
    let tmp = TempDir::new().expect("tempdir");
    let cas = tmp.path().join("cas");
    let bare = cas.join(".git");
    let _repo = Repository::init_bare(&bare).expect("init bare cas");
    let r = Repository::open(&bare).expect("open");
    // Non-compressible content (see seeded_byte rationale) so loose-file
    // size approximates content size.
    let big: Vec<u8> = (0..800).map(|i| seeded_byte(0xE3, i)).collect();
    let small: Vec<u8> = (0..100).map(|i| seeded_byte(0x21, i)).collect();
    let big_oid = r.blob(&big).expect("big");
    let _small_oid = r.blob(&small).expect("small");

    let big_pre = std::fs::read(
        bare.join("objects")
            .join(&big_oid.to_string()[..2])
            .join(&big_oid.to_string()[2..]),
    )
    .expect("read big pre");
    assert!(big_pre.len() > 32, "fixture: big blob must exceed 32-byte threshold");

    let detail = flip_largest_cas_object(&cas).expect("CAS tamper must succeed");
    assert!(detail.contains("CAS object"), "detail must mention CAS: {detail}");

    let big_post = std::fs::read(
        bare.join("objects")
            .join(&big_oid.to_string()[..2])
            .join(&big_oid.to_string()[2..]),
    )
    .expect("read big post");
    assert_ne!(big_pre, big_post, "largest CAS blob must be corrupted");
}

/// CHAIN_REFS list MUST contain exactly the post-A3 canonical refs +
/// the pre-A3 alias. Catches accidental removal/rename.
#[test]
fn chain_refs_list_is_complete_for_post_a3_multi_ref() {
    let expected: HashSet<&str> = [
        "refs/chaintape/l4",
        "refs/chaintape/l4e",
        "refs/transitions/main",
    ]
    .into_iter()
    .collect();
    let actual: HashSet<&str> = CHAIN_REFS.iter().copied().collect();
    assert_eq!(
        actual, expected,
        "CHAIN_REFS must contain exactly the post-A3 multi-ref names + pre-A3 alias.\n\
         Adding a new canonical ref name (e.g. Stage D+) requires also updating tamper \
         coverage and re-running this gate."
    );
}

/// `L4_REFS` MUST be the strict subset of `CHAIN_REFS` that points at refs
/// whose reachable blob bodies are deep-verified by `audit_tape`. The
/// rejection-chain `refs/chaintape/l4e` MUST be in `CHAIN_REFS` (for ref
/// truncation) but MUST NOT be in `L4_REFS` (audit doesn't deep-verify
/// rejection_record bodies; tampering one would be silent).
#[test]
fn l4_refs_is_strict_subset_of_chain_refs_excluding_l4e() {
    let l4: HashSet<&str> = L4_REFS.iter().copied().collect();
    let chain: HashSet<&str> = CHAIN_REFS.iter().copied().collect();
    assert!(
        l4.is_subset(&chain),
        "L4_REFS must be a subset of CHAIN_REFS; got L4_REFS={:?} CHAIN_REFS={:?}",
        L4_REFS,
        CHAIN_REFS
    );
    assert!(
        !l4.contains("refs/chaintape/l4e"),
        "L4_REFS must NOT include refs/chaintape/l4e: audit does not deep-verify L4.E \
         rejection_record bodies, so tampering an L4.E blob is silent. Adding it would \
         re-introduce the M0 2026-05-10 false-PASS regression. (Constitutional gap: \
         L4.E body integrity verification is a forward item; until it lands, L4.E \
         tampering is constitutionally undetectable and must not be claimed.)"
    );
    assert!(
        chain.contains("refs/chaintape/l4e"),
        "CHAIN_REFS must include refs/chaintape/l4e: ref truncation MUST corrupt the \
         L4.E head ref so audit can't load the rejection chain at all."
    );
}

// ── Internal helpers ────────────────────────────────────────────────────────

/// Walk loose objects under `repo/.git/objects/`, return hex OID of largest
/// (optionally filtered to objects reachable from `CHAIN_REFS`). Used by the
/// reachability test as the oracle for "what should the library pick?".
fn list_largest_loose_oid(repo_path: &Path, reachable_only: bool) -> String {
    let repo = Repository::open(repo_path).expect("open repo");
    let mut reachable: HashSet<git2::Oid> = HashSet::new();
    if reachable_only {
        for ref_name in CHAIN_REFS {
            let Ok(reference) = repo.find_reference(ref_name) else {
                continue;
            };
            let Some(target) = reference.target() else {
                continue;
            };
            let mut walk = repo.revwalk().expect("revwalk");
            walk.push(target).expect("push");
            for c in walk {
                let cid = c.expect("walk");
                reachable.insert(cid);
                if let Ok(commit) = repo.find_commit(cid) {
                    if let Ok(tree) = commit.tree() {
                        reachable.insert(tree.id());
                        let _ = tree.walk(git2::TreeWalkMode::PreOrder, |_, e| {
                            reachable.insert(e.id());
                            git2::TreeWalkResult::Ok
                        });
                    }
                }
            }
        }
    }
    let objects = repo_path.join(".git").join("objects");
    let mut largest: Option<(String, u64)> = None;
    walk(&objects, &reachable, reachable_only, &mut largest);
    largest.expect("at least one loose object").0
}

fn walk(
    dir: &Path,
    reachable: &HashSet<git2::Oid>,
    reachable_only: bool,
    largest: &mut Option<(String, u64)>,
) {
    let Ok(read) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in read.flatten() {
        let p = entry.path();
        if p.is_dir() {
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "pack" || name == "info" {
                continue;
            }
            walk(&p, reachable, reachable_only, largest);
        } else {
            let parent_name = p
                .parent()
                .and_then(|x| x.file_name())
                .and_then(|x| x.to_str())
                .unwrap_or("");
            let file_name = p.file_name().and_then(|x| x.to_str()).unwrap_or("");
            if parent_name.len() == 2 && file_name.len() == 38 {
                let oid_hex = format!("{}{}", parent_name, file_name);
                let oid = git2::Oid::from_str(&oid_hex).expect("oid parse");
                if reachable_only && !reachable.contains(&oid) {
                    continue;
                }
                let len = std::fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
                if largest.as_ref().map(|(_, n)| *n).unwrap_or(0) < len {
                    *largest = Some((oid_hex, len));
                }
            }
        }
    }
}
