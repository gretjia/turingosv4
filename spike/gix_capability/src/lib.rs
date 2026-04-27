//! CO1.3.1 git Substrate Capability Spike
//!
//! **Pivot rationale**: Initial attempt used `gix` 0.66.0 directly; high-level commit
//! ergonomic API has gaps (Tree::edit / rev_walk / config setters). Per pre-flight
//! `handover/specs/CO1_3_1_GIX_SPIKE_PREFLIGHT_v1_2026-04-27.md` § 4 ("PIVOT to git2-rs"),
//! switched to `git2` (libgit2 Rust binding; mature, complete API surface).
//!
//! Constitutional check: Constitution Art. 0.4 specifies "Path B = real git substrate";
//! authorizes either gix or git2-rs (both wrap the same git protocol). Pre-flight § 4
//! decision tree explicitly allows this pivot. ✅ no constitutional violation.
//!
//! Tests: 8 capabilities (C1-C8) per pre-flight § 1 + integration test.
pub mod c1_init;
pub mod c2_multi_parent;
pub mod c3_tree_parent_read;
pub mod c4_concurrent_init;
pub mod c5_cas_blob;
pub mod c6_perf_benchmark;
pub mod c7_replay;
pub mod c8_hooks_compat;

pub use anyhow;
