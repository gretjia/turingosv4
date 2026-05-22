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
use std::sync::atomic::AtomicU64;

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

impl ImmutableTapeLedger for GitTapeLedger {
    fn get_verified_head(&self) -> String {
        todo!("Atom 22: read refs/tdma/verified_head and return its commit OID hex (or H0 sentinel)")
    }

    fn set_verified_head(&mut self, _new_head: String) {
        todo!("Atom 22: update refs/tdma/verified_head with lock + flush")
    }

    fn commit(&mut self, _req: CommitRequest) -> TapeNode {
        todo!("Atom 21: serialize TapeNode into 8-blob commit tree + per-scope ref update")
    }

    fn count_nodes(
        &self,
        _kind: Option<NodeKind>,
        _verified: Option<bool>,
        _parent: Option<&str>,
        _scope: Option<&AttemptScope>,
    ) -> usize {
        todo!("Atom 21: walk per-scope ref's git log and apply filters in-walk")
    }

    fn latest_node(&self, _kind: NodeKind, _scope: &AttemptScope) -> Option<TapeNode> {
        todo!("Atom 21: walk per-scope ref in log order; return first matching kind")
    }

    fn derive_latest_belief_state_from_tape(
        &self,
        _scope: &AttemptScope,
    ) -> Option<RetryBeliefState> {
        todo!("Atom 22: PURE FUNCTION — walk per-scope ref filtering for RetryBeliefState kind; return most-recent")
    }

    fn dump_all_nodes(&self) -> Vec<(String, TapeNode)> {
        // Atom 20 skeleton: empty until Atom 21/22 lands the real walk.
        // run_proof_with_ledger calls this for chaintape.jsonl emission;
        // an empty vec yields an empty chaintape, which is correct for
        // a skeleton ledger (nothing committed = nothing to dump).
        Vec::new()
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
