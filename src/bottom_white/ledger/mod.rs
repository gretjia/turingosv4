//! L4 ledger support modules for Bottom White.
//!
//! /// TRACE_MATRIX FC1-Sig+FC3-Sig: ledger crypto support root

/// TRACE_MATRIX FC1-Sig+FC3-Sig: system runtime signature key lifecycle.
pub mod system_keypair;

/// TRACE_MATRIX Inv 7 + Inv 10 + ROADMAP P1:6/P1:9: L4.E rejection-evidence ledger (RSP-0).
/// Disjoint from L4 (`transition_ledger`); accepted spine and rejection-evidence are separate.
pub mod rejection_evidence;

/// TRACE_MATRIX FC2-Append + WP § 5.L4: L4 transition ledger (CO1.7 type skeleton).
/// Status: pre-audit type skeleton; bodies that need transition functions are deferred.
pub mod transition_ledger;
