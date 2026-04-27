//! Predicate visibility policy (Goodhart shield per Const Art III.4 + WP § 9.4).
//!
//! Three classes:
//! - `Public`: schema + permission + basic tests visible to agent
//! - `Private`: hidden benchmarks; agent CANNOT read source/results beyond pass/fail
//! - `CommitReveal`: hash committed first; sample revealed at fixed logical_t
//!
//! /// TRACE_MATRIX Const-Art-III.4 + Inv-10 + WP-spec-§1.5: Goodhart visibility

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    /// Agent sees full predicate metadata + schema + can read public test cases.
    Public,
    /// Agent sees only pass/fail outcome. Predicate source + test corpus private.
    /// Used for hidden benchmarks that prevent Goodhart attacks.
    Private,
    /// Predicate hash is published; sample revealed at `reveal_at_logical_t`.
    /// Until reveal, agents cannot test against it; after reveal, becomes Public.
    CommitReveal { reveal_at_logical_t: u64, predicate_hash: [u8; 32] },
}

impl Visibility {
    /// Whether the predicate's content (source / tests / inputs) is visible to agents NOW.
    /// `now` is the current logical_t.
    pub fn content_visible_to_agent(&self, now: u64) -> bool {
        match self {
            Self::Public => true,
            Self::Private => false,
            Self::CommitReveal { reveal_at_logical_t, .. } => now >= *reveal_at_logical_t,
        }
    }

    /// Whether the predicate's commit hash (only) is visible.
    /// All visibility classes expose at least the hash.
    pub fn hash_visible_to_agent(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_always_visible() {
        let v = Visibility::Public;
        assert!(v.content_visible_to_agent(0));
        assert!(v.content_visible_to_agent(u64::MAX));
        assert!(v.hash_visible_to_agent());
    }

    #[test]
    fn private_never_content_visible() {
        let v = Visibility::Private;
        assert!(!v.content_visible_to_agent(0));
        assert!(!v.content_visible_to_agent(u64::MAX));
        // hash still visible
        assert!(v.hash_visible_to_agent());
    }

    #[test]
    fn commit_reveal_pre_reveal() {
        let v = Visibility::CommitReveal {
            reveal_at_logical_t: 1000,
            predicate_hash: [0u8; 32],
        };
        assert!(!v.content_visible_to_agent(999));
        assert!(v.content_visible_to_agent(1000));
        assert!(v.content_visible_to_agent(1001));
    }

    #[test]
    fn serde_round_trip_public() {
        let v = Visibility::Public;
        let s = serde_json::to_string(&v).unwrap();
        let v2: Visibility = serde_json::from_str(&s).unwrap();
        assert_eq!(v, v2);
    }

    #[test]
    fn serde_round_trip_commit_reveal() {
        let v = Visibility::CommitReveal {
            reveal_at_logical_t: 42,
            predicate_hash: [0xab; 32],
        };
        let s = serde_json::to_string(&v).unwrap();
        let v2: Visibility = serde_json::from_str(&s).unwrap();
        assert_eq!(v, v2);
    }
}
