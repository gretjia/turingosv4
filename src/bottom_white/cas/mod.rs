//! L3 Content-Addressable Store per WP architecture § 5.L3 + STATE_TRANSITION_SPEC v1.4 § 1.2.
//!
//! Stores Agent proposal payloads, predicate bytecode, tool bytecode, etc.
//! as git blobs in the per-cell `runtime_repo`. Each object addressed by
//! `Cid` (sha256 of content) — distinct from git's sha-1 OID; both stored.
//!
//! Backend: git2-rs (libgit2 bindings; spike-validated 2026-04-27 8/8 PASS).
//!
//! /// TRACE_MATRIX WP-arch-§5.L3 + spec-§1.2 (proposal_cid): CAS root

/// TRACE_MATRIX WP-arch-§5.L3 + Art. 0.4: Git commit-chain authority for
/// `refs/chaintape/cas`; sidecar index is cache once this ref exists.
pub mod git_chain;
pub mod schema;
pub mod store;

pub use schema::{CasObjectMetadata, Cid, ObjectType};
pub use store::{CasError, CasStore};
