//! TRACE_MATRIX FC1a-substrate_seam + FC3-replay:
//! Phase E `GitTapeLedger` skeleton — Atom 20.
//!
//! Constitution Art. 0.4 mandates Path B (real-git substrate) for the TDMA
//! tape. This file lands the SKELETON: type definitions, repo open/init,
//! ImmutableTapeLedger trait wired with `todo!()` panics for the bodies that
//! Atoms 21 (commit/retrieve roundtrip) and 22 (verified_head + BBS via
//! refs/log) will fill in.
//!
//! Why a skeleton-first split (Karpathy ARCHITECT §3 micro-version-before-real):
//! Atom 20 proves the trait wiring, the `pub mod` registration, the
//! `run_proof_with_ledger` generic, and the Trust Root rehash discipline ALL
//! compile end-to-end BEFORE Atoms 21/22 commit canonical semantics. The
//! skeleton has Class 2 risk (additive trait impl + one-line generic
//! parameterization); Atoms 21/22 are Class 3 (tape semantics).
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_GENERATE_PHASE_E_DIRECTIVE_AND_§8.md

use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

use crate::ledger::{
    AttemptScope, CommitRequest, ImmutableTapeLedger, NodeKind, RetryBeliefState, TapeNode,
};

// ── Phase E configuration constants (frozen by plan §4.1) ──────────

/// TRACE_MATRIX FC1a-substrate_seam: Default bare-repo directory name within
/// a TuringOS workspace (`<workspace>/tdma_tape.git/`).
pub const GIT_LEDGER_REPO_DIR_DEFAULT: &str = "tdma_tape.git";

/// TRACE_MATRIX FC1a-substrate_seam: Deterministic author name pinned to the
/// kernel identity. Never the user's git config.
pub const GIT_LEDGER_AUTHOR_NAME: &str = "turingosv4 tdma kernel";

/// TRACE_MATRIX FC1a-substrate_seam: Deterministic author email — placeholder
/// domain, never reaches the network.
pub const GIT_LEDGER_AUTHOR_EMAIL: &str = "tape@turingos.local";

/// TRACE_MATRIX FC1a-substrate_seam: Segregated ref namespace for verified_head.
/// Constitution Art. 0.4 obligation #3 (verified_head -> git HEAD ref).
pub const GIT_LEDGER_HEAD_REF: &str = "refs/tdma/verified_head";

/// TRACE_MATRIX FC1a-substrate_seam: Ref for ledger tail (most-recent commit
/// across all kinds, verified or not).
pub const GIT_LEDGER_LEDGER_TAIL_REF: &str = "refs/tdma/ledger_tail";

/// TRACE_MATRIX FC1a-substrate_seam: Prefix for per-AttemptScope refs. The
/// scope-hashed suffix is appended at commit time.
pub const GIT_LEDGER_SCOPE_REF_PREFIX: &str = "refs/tdma/scopes/";

// ── Error type ─────────────────────────────────────────────────────

/// TRACE_MATRIX FC1a-substrate_seam: Failure modes for GitTapeLedger ops.
#[derive(Debug)]
pub enum GitTapeLedgerError {
    Io(std::io::Error),
    Git(git2::Error),
    MalformedNode(String),
    MissingRef(String),
}

impl std::fmt::Display for GitTapeLedgerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitTapeLedgerError::Io(e) => write!(f, "io: {}", e),
            GitTapeLedgerError::Git(e) => write!(f, "git: {}", e),
            GitTapeLedgerError::MalformedNode(s) => write!(f, "malformed node: {}", s),
            GitTapeLedgerError::MissingRef(s) => write!(f, "missing ref: {}", s),
        }
    }
}

impl std::error::Error for GitTapeLedgerError {}

impl From<std::io::Error> for GitTapeLedgerError {
    fn from(e: std::io::Error) -> Self {
        GitTapeLedgerError::Io(e)
    }
}

impl From<git2::Error> for GitTapeLedgerError {
    fn from(e: git2::Error) -> Self {
        GitTapeLedgerError::Git(e)
    }
}

// ── GitTapeLedger ──────────────────────────────────────────────────

/// TRACE_MATRIX FC1a-substrate_seam: Real-git implementation of the
/// `ImmutableTapeLedger` trait. Phase E Path B per constitution Art. 0.4.
///
/// Bodies for commit / count_nodes / latest_node / verified_head /
/// derive_latest_belief_state_from_tape are intentionally `todo!()` until
/// Atoms 21 + 22 land. Atom 20 verifies the surface compiles, the trait
/// wires up, and `Box<dyn ImmutableTapeLedger>` accepts the type.
pub struct GitTapeLedger {
    repo: git2::Repository,
    #[allow(dead_code)]
    next_seq: AtomicU64,
}

impl GitTapeLedger {
    /// TRACE_MATRIX FC1a-substrate_seam: Open an existing bare repo at `path`.
    pub fn open(path: &Path) -> Result<Self, GitTapeLedgerError> {
        let repo = git2::Repository::open_bare(path)?;
        Ok(Self {
            repo,
            next_seq: AtomicU64::new(1),
        })
    }

    /// TRACE_MATRIX FC1a-substrate_seam: Initialize a new bare repo at `path`.
    /// Idempotent if the path already contains a bare repo.
    pub fn init_bare(path: &Path) -> Result<Self, GitTapeLedgerError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let repo = git2::Repository::init_bare(path)?;
        Ok(Self {
            repo,
            next_seq: AtomicU64::new(1),
        })
    }

    /// TRACE_MATRIX FC1a-substrate_seam: Expose the underlying git2 repo
    /// for Atoms 21/22 implementation use. NOT part of the public Phase E
    /// API surface — module-private.
    #[allow(dead_code)]
    pub(crate) fn repo(&self) -> &git2::Repository {
        &self.repo
    }
}

// ── Atom 21 — canonical TapeNode <-> git commit OID mapping ────────

fn scope_ref_name(scope: &AttemptScope) -> String {
    let mut h = Sha256::new();
    h.update(scope.run_id.as_bytes());
    h.update(b"|");
    h.update(scope.task_id.as_bytes());
    h.update(b"|");
    h.update(scope.verified_parent.as_bytes());
    let digest = format!("{:x}", h.finalize());
    format!("{}{}", GIT_LEDGER_SCOPE_REF_PREFIX, &digest[..40])
}

fn node_kind_discriminant(k: &NodeKind) -> u8 {
    match k {
        NodeKind::StateAccepted => 0,
        NodeKind::AgentProposal => 1,
        NodeKind::RetryBeliefState => 2,
        NodeKind::CharterCore => 3,
        NodeKind::PromptAssembly => 4,
        NodeKind::Escalation => 5,
    }
}

fn node_kind_from_discriminant(d: u8) -> Option<NodeKind> {
    match d {
        0 => Some(NodeKind::StateAccepted),
        1 => Some(NodeKind::AgentProposal),
        2 => Some(NodeKind::RetryBeliefState),
        3 => Some(NodeKind::CharterCore),
        4 => Some(NodeKind::PromptAssembly),
        5 => Some(NodeKind::Escalation),
        _ => None,
    }
}

fn write_tree_for_request(
    repo: &git2::Repository,
    req: &CommitRequest,
    created_at_unix_ms: u64,
) -> Result<git2::Oid, GitTapeLedgerError> {
    let mut builder = repo.treebuilder(None)?;

    // 0:payload.json — canonical JSON of the payload
    let payload_bytes = serde_json::to_vec_pretty(&req.payload)
        .map_err(|e| GitTapeLedgerError::MalformedNode(format!("payload encode: {e}")))?;
    let payload_oid = repo.blob(&payload_bytes)?;
    builder.insert("0_payload.json", payload_oid, 0o100644)?;

    // 1:kind — one byte discriminant
    let kind_oid = repo.blob(&[node_kind_discriminant(&req.kind)])?;
    builder.insert("1_kind", kind_oid, 0o100644)?;

    // 2:verified — one byte
    let verified_oid = repo.blob(&[req.verified as u8])?;
    builder.insert("2_verified", verified_oid, 0o100644)?;

    // 3:scope.json — only if Some
    if let Some(scope) = &req.scope {
        let scope_bytes = serde_json::to_vec(scope)
            .map_err(|e| GitTapeLedgerError::MalformedNode(format!("scope encode: {e}")))?;
        let oid = repo.blob(&scope_bytes)?;
        builder.insert("3_scope.json", oid, 0o100644)?;
    }

    // 4:attempt_ordinal — u32 big-endian bytes if Some
    if let Some(n) = req.attempt_ordinal {
        let oid = repo.blob(&n.to_be_bytes())?;
        builder.insert("4_attempt_ordinal", oid, 0o100644)?;
    }

    // 5:reject_class — UTF-8 bytes if Some
    if let Some(s) = &req.reject_class {
        let oid = repo.blob(s.as_bytes())?;
        builder.insert("5_reject_class", oid, 0o100644)?;
    }

    // 6:token_count — u64 big-endian bytes if Some
    if let Some(n) = req.token_count {
        let bytes = (n as u64).to_be_bytes();
        let oid = repo.blob(&bytes)?;
        builder.insert("6_token_count", oid, 0o100644)?;
    }

    // 7:created_at_ms — u64 big-endian bytes (always present)
    let oid = repo.blob(&created_at_unix_ms.to_be_bytes())?;
    builder.insert("7_created_at_ms", oid, 0o100644)?;

    Ok(builder.write()?)
}

fn read_blob_bytes(
    repo: &git2::Repository,
    tree: &git2::Tree,
    name: &str,
) -> Result<Option<Vec<u8>>, GitTapeLedgerError> {
    match tree.get_name(name) {
        None => Ok(None),
        Some(entry) => {
            let blob = repo.find_blob(entry.id())?;
            Ok(Some(blob.content().to_vec()))
        }
    }
}

fn reconstruct_node_from_commit(
    repo: &git2::Repository,
    commit: &git2::Commit,
) -> Result<TapeNode, GitTapeLedgerError> {
    let tree = commit.tree()?;

    let payload_bytes = read_blob_bytes(repo, &tree, "0_payload.json")?
        .ok_or_else(|| GitTapeLedgerError::MalformedNode("missing 0_payload.json".into()))?;
    let payload: serde_json::Value = serde_json::from_slice(&payload_bytes)
        .map_err(|e| GitTapeLedgerError::MalformedNode(format!("payload decode: {e}")))?;

    let kind_bytes = read_blob_bytes(repo, &tree, "1_kind")?
        .ok_or_else(|| GitTapeLedgerError::MalformedNode("missing 1_kind".into()))?;
    if kind_bytes.len() != 1 {
        return Err(GitTapeLedgerError::MalformedNode(format!(
            "kind blob len {}",
            kind_bytes.len()
        )));
    }
    let kind = node_kind_from_discriminant(kind_bytes[0])
        .ok_or_else(|| GitTapeLedgerError::MalformedNode(format!("kind disc {}", kind_bytes[0])))?;

    let verified_bytes = read_blob_bytes(repo, &tree, "2_verified")?
        .ok_or_else(|| GitTapeLedgerError::MalformedNode("missing 2_verified".into()))?;
    if verified_bytes.len() != 1 {
        return Err(GitTapeLedgerError::MalformedNode("verified blob len".into()));
    }
    let verified = verified_bytes[0] != 0;

    let scope: Option<AttemptScope> = match read_blob_bytes(repo, &tree, "3_scope.json")? {
        Some(b) => Some(
            serde_json::from_slice(&b)
                .map_err(|e| GitTapeLedgerError::MalformedNode(format!("scope decode: {e}")))?,
        ),
        None => None,
    };

    let attempt_ordinal: Option<u32> = match read_blob_bytes(repo, &tree, "4_attempt_ordinal")? {
        Some(b) if b.len() == 4 => Some(u32::from_be_bytes([b[0], b[1], b[2], b[3]])),
        Some(_) => return Err(GitTapeLedgerError::MalformedNode("attempt_ordinal len".into())),
        None => None,
    };

    let reject_class: Option<String> = match read_blob_bytes(repo, &tree, "5_reject_class")? {
        Some(b) => Some(
            String::from_utf8(b)
                .map_err(|e| GitTapeLedgerError::MalformedNode(format!("reject_class utf8: {e}")))?,
        ),
        None => None,
    };

    let token_count: Option<usize> = match read_blob_bytes(repo, &tree, "6_token_count")? {
        Some(b) if b.len() == 8 => {
            let mut arr = [0u8; 8];
            arr.copy_from_slice(&b);
            Some(u64::from_be_bytes(arr) as usize)
        }
        Some(_) => return Err(GitTapeLedgerError::MalformedNode("token_count len".into())),
        None => None,
    };

    let created_at_unix_ms_bytes = read_blob_bytes(repo, &tree, "7_created_at_ms")?
        .ok_or_else(|| GitTapeLedgerError::MalformedNode("missing 7_created_at_ms".into()))?;
    if created_at_unix_ms_bytes.len() != 8 {
        return Err(GitTapeLedgerError::MalformedNode("created_at_ms len".into()));
    }
    let mut arr = [0u8; 8];
    arr.copy_from_slice(&created_at_unix_ms_bytes);
    let created_at_unix_ms = u64::from_be_bytes(arr);

    // The user-supplied TapeNode.parent lives in the canonical-JSON commit
    // message (NOT the git commit graph — the graph parent is the prior
    // scope-ref/ledger_tail tip for revwalk traversal).
    let msg_str = commit.message().unwrap_or("");
    let parent: Option<String> = serde_json::from_str::<serde_json::Value>(msg_str)
        .ok()
        .and_then(|v| {
            v.get("parent")
                .and_then(|p| p.as_str().map(|s| s.to_string()))
        });

    // Recover the monotonic id from commit time (commit() pins time-seconds = id).
    // The id is canonical "tn-N" form to match MemoryTapeLedger.
    let id_value = commit.time().seconds() as u64;
    Ok(TapeNode {
        id: format!("tn-{}", id_value),
        hash: commit.id().to_string(),
        kind,
        verified,
        parent,
        scope,
        attempt_ordinal,
        reject_class,
        token_count,
        payload,
        created_at_unix_ms,
    })
}

fn walk_commits<F>(
    repo: &git2::Repository,
    start_ref: &str,
    mut f: F,
) -> Result<(), GitTapeLedgerError>
where
    F: FnMut(&git2::Commit) -> Result<bool, GitTapeLedgerError>, // returns false to stop early
{
    let reference = match repo.find_reference(start_ref) {
        Ok(r) => r,
        Err(e) if e.code() == git2::ErrorCode::NotFound => return Ok(()),
        Err(e) => return Err(e.into()),
    };
    let start_oid = reference
        .target()
        .ok_or_else(|| GitTapeLedgerError::MissingRef(start_ref.into()))?;

    let mut revwalk = repo.revwalk()?;
    revwalk.push(start_oid)?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    for oid_res in revwalk {
        let oid = oid_res?;
        let commit = repo.find_commit(oid)?;
        if !f(&commit)? {
            break;
        }
    }
    Ok(())
}

impl ImmutableTapeLedger for GitTapeLedger {
    fn get_verified_head(&self) -> String {
        todo!("Atom 22: read refs/tdma/verified_head and return its commit OID hex (or H0 sentinel)")
    }

    fn set_verified_head(&mut self, _new_head: String) {
        todo!("Atom 22: update refs/tdma/verified_head with lock + flush")
    }

    fn commit(&mut self, req: CommitRequest) -> TapeNode {
        // Atom 21: use next_seq for BOTH id and created_at_unix_ms — matches
        // MemoryTapeLedger's monotonic-counter semantics so cross-impl tests
        // (KILL-git-1) can assert equality on created_at_unix_ms across impls.
        let id = self.next_seq.fetch_add(1, Ordering::SeqCst);
        let created_at_unix_ms: u64 = id;

        let tree_oid = write_tree_for_request(&self.repo, &req, created_at_unix_ms)
            .expect("Atom 21: write_tree_for_request failed");
        let tree = self
            .repo
            .find_tree(tree_oid)
            .expect("Atom 21: find_tree failed");

        // Pin signature time to id (deterministic; no wall-clock leak per §4.2
        // grep guard `! grep -rE 'std::time::SystemTime::now\(\)\.elapsed\(\)'`).
        let signature = git2::Signature::new(
            GIT_LEDGER_AUTHOR_NAME,
            GIT_LEDGER_AUTHOR_EMAIL,
            &git2::Time::new(id as i64, 0),
        )
        .expect("Atom 21: signature construction failed");

        // Git commit graph parent = prior tip on the appropriate ref so a
        // revwalk from the ref visits ALL committed nodes in reverse-chrono.
        // The TapeNode.parent field (req.parent, possibly None) is stored
        // independently in the canonical-JSON commit message.
        let graph_parent_ref = req
            .scope
            .as_ref()
            .map(scope_ref_name)
            .unwrap_or_else(|| GIT_LEDGER_LEDGER_TAIL_REF.to_string());
        let graph_parent_oid = self
            .repo
            .find_reference(&graph_parent_ref)
            .ok()
            .and_then(|r| r.target());

        let parent_commits: Vec<git2::Commit> = if let Some(oid) = graph_parent_oid {
            match self.repo.find_commit(oid) {
                Ok(c) => vec![c],
                Err(_) => Vec::new(),
            }
        } else {
            Vec::new()
        };
        let parent_refs: Vec<&git2::Commit> = parent_commits.iter().collect();

        let canonical_msg = serde_json::to_string(&serde_json::json!({
            "id": id,
            "kind": &req.kind,
            "verified": req.verified,
            "parent": &req.parent,
            "scope": &req.scope,
            "attempt_ordinal": &req.attempt_ordinal,
            "reject_class": &req.reject_class,
            "token_count": &req.token_count,
            "created_at_unix_ms": created_at_unix_ms,
        }))
        .unwrap_or_default();

        let commit_oid = self
            .repo
            .commit(None, &signature, &signature, &canonical_msg, &tree, &parent_refs)
            .expect("Atom 21: git commit failed");

        // Update refs: per-scope (if scope.is_some) + ledger_tail (always).
        if let Some(scope) = &req.scope {
            let ref_name = scope_ref_name(scope);
            // update_or_create: set_target if exists, else create.
            if let Ok(mut r) = self.repo.find_reference(&ref_name) {
                let _ = r.set_target(commit_oid, "tdma scope ref update");
            } else {
                let _ = self
                    .repo
                    .reference(&ref_name, commit_oid, true, "tdma scope ref init");
            }
        }
        if let Ok(mut r) = self.repo.find_reference(GIT_LEDGER_LEDGER_TAIL_REF) {
            let _ = r.set_target(commit_oid, "tdma ledger_tail update");
        } else {
            let _ = self.repo.reference(
                GIT_LEDGER_LEDGER_TAIL_REF,
                commit_oid,
                true,
                "tdma ledger_tail init",
            );
        }

        TapeNode {
            id: format!("tn-{}", id),
            hash: commit_oid.to_string(),
            kind: req.kind,
            verified: req.verified,
            parent: req.parent,
            scope: req.scope,
            attempt_ordinal: req.attempt_ordinal,
            reject_class: req.reject_class,
            token_count: req.token_count,
            payload: req.payload,
            created_at_unix_ms,
        }
    }

    fn count_nodes(
        &self,
        kind: Option<NodeKind>,
        verified: Option<bool>,
        parent: Option<&str>,
        scope: Option<&AttemptScope>,
    ) -> usize {
        let start_ref = match scope {
            Some(s) => scope_ref_name(s),
            None => GIT_LEDGER_LEDGER_TAIL_REF.to_string(),
        };

        let mut count = 0usize;
        let _ = walk_commits(&self.repo, &start_ref, |commit| {
            match reconstruct_node_from_commit(&self.repo, commit) {
                Ok(n) => {
                    let kind_ok = kind.as_ref().map(|k| &n.kind == k).unwrap_or(true);
                    let verified_ok = verified.map(|v| n.verified == v).unwrap_or(true);
                    let parent_ok = parent
                        .map(|p| n.parent.as_deref() == Some(p))
                        .unwrap_or(true);
                    let scope_ok = scope.map(|s| n.scope.as_ref() == Some(s)).unwrap_or(true);
                    if kind_ok && verified_ok && parent_ok && scope_ok {
                        count += 1;
                    }
                    Ok(true)
                }
                Err(_) => Ok(true), // skip malformed commits silently
            }
        });
        count
    }

    fn latest_node(&self, kind: NodeKind, scope: &AttemptScope) -> Option<TapeNode> {
        let start_ref = scope_ref_name(scope);
        let mut found: Option<TapeNode> = None;
        let _ = walk_commits(&self.repo, &start_ref, |commit| {
            if let Ok(n) = reconstruct_node_from_commit(&self.repo, commit) {
                if n.kind == kind && n.scope.as_ref() == Some(scope) {
                    found = Some(n);
                    return Ok(false); // stop walk
                }
            }
            Ok(true)
        });
        found
    }

    fn derive_latest_belief_state_from_tape(
        &self,
        _scope: &AttemptScope,
    ) -> Option<RetryBeliefState> {
        todo!("Atom 22: PURE FUNCTION — walk per-scope ref filtering for RetryBeliefState kind; return most-recent")
    }

    fn dump_all_nodes(&self) -> Vec<(String, TapeNode)> {
        let mut out: Vec<(String, TapeNode)> = Vec::new();
        let _ = walk_commits(&self.repo, GIT_LEDGER_LEDGER_TAIL_REF, |commit| {
            if let Ok(n) = reconstruct_node_from_commit(&self.repo, commit) {
                out.push((n.hash.clone(), n));
            }
            Ok(true)
        });
        out
    }
}

// ── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn open_and_close_smoke() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("tdma_tape.git");
        // Init first
        let l1 = GitTapeLedger::init_bare(&path).expect("init_bare");
        drop(l1);
        // Then reopen
        let l2 = GitTapeLedger::open(&path).expect("open after init");
        // Repo handle accessible (Atom 21/22 will use it)
        assert!(l2.repo().is_bare());
    }

    #[test]
    fn trait_object_dispatch_compiles() {
        // The point of this test is purely that the impl compiles cleanly
        // as a trait object. We don't call any todo!() methods.
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("tdma_tape.git");
        let ledger = GitTapeLedger::init_bare(&path).expect("init_bare");
        let _boxed: Box<dyn ImmutableTapeLedger> = Box::new(ledger);
        // If this line compiled, the trait impl satisfies the bound.
    }
}
