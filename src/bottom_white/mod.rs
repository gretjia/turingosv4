//! Bottom White layer (per Anti-Oreo § 3 / Constitution Art I.1).
//!
//! Deterministic, append-only substrate. tape, CAS, ledger, sandbox, materializer, tools.
//! v4 first iteration: tools module only (rest land in subsequent atoms).
//!
//! /// TRACE_MATRIX Const-Art-I.1 + WP-arch-§3 + WP-arch-§5.L0-L6: Bottom White layer root

pub mod cas;
/// TRACE_MATRIX FC1-Sig+FC3-Sig: Bottom White ledger crypto modules.
pub mod ledger;
pub mod tools;
