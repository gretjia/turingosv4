//! Top White layer (per Anti-Oreo § 3 / Constitution Art I.1).
//!
//! Sees agent inputs but NOT agent internal state. Decides accept/reject via
//! predicates + signals. v4 first iteration: predicates module only.
//!
//! /// TRACE_MATRIX Const-Art-I.1 + WP-arch-§3: Top White layer root

pub mod predicates;
