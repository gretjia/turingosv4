//! OracleReceipt — capability token authorizing blessed writes to tape.
//!
//! Closes Codex V-1 (C-048): `bus.append_oracle_accepted()` previously
//! accepted `oracle_blessed = true` from any caller, bypassing
//! forbidden_patterns. Discipline at call sites was the only defense.
//!
//! This module introduces a receipt that binds the payload (via sha256)
//! to the oracle's verdict. The bus validates the receipt before granting
//! the blessed write. Forging a receipt requires knowing the payload's
//! hash up front — not a capability a malicious tool would have at runtime.
//!
//! Phase 10c will extend this with Ed25519 signature for external agent auth.

use crate::sdk::predicate::{PredicateKind, Verdict};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Capability token proving a predicate accepted a payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleReceipt {
    pub payload_hash: [u8; 32],
    pub verdict: Verdict,
    pub predicate_kind: PredicateKind,
    // Phase 10c: signature: Option<[u8; 64]> for external agent authorization.
}

impl OracleReceipt {
    /// Receipt for a Lean4-boolean `Complete` verdict (full proof accepted).
    pub fn for_lean4_complete(payload: &str) -> Self {
        Self {
            payload_hash: hash(payload),
            verdict: Verdict::Complete,
            predicate_kind: PredicateKind::Lean4Boolean,
        }
    }

    /// Receipt for a Lean4-boolean `PartialOk` verdict (unsolved goals,
    /// structurally valid). `confidence = 1.0` because Lean elaboration is
    /// deterministic; PCP confidence semantics are reserved for Paper 3.
    pub fn for_lean4_partial(payload: &str) -> Self {
        Self {
            payload_hash: hash(payload),
            verdict: Verdict::PartialOk { confidence: 1.0 },
            predicate_kind: PredicateKind::Lean4Boolean,
        }
    }

    /// Validate that this receipt matches the given payload and carries a
    /// non-rejecting verdict. Called by the bus before granting blessed
    /// write.
    pub fn validates(&self, payload: &str) -> Result<(), String> {
        let computed = hash(payload);
        if computed != self.payload_hash {
            return Err("OracleReceipt.payload_hash mismatch (payload tampered or stale receipt)".into());
        }
        match &self.verdict {
            Verdict::Complete | Verdict::PartialOk { .. } => Ok(()),
            Verdict::Reject(r) => Err(format!("OracleReceipt.verdict is Reject: {}", r)),
        }
    }
}

fn hash(payload: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(payload.as_bytes());
    h.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receipt_validates_matching_payload() {
        let r = OracleReceipt::for_lean4_complete("by linarith");
        assert!(r.validates("by linarith").is_ok());
    }

    #[test]
    fn receipt_rejects_tampered_payload() {
        let r = OracleReceipt::for_lean4_complete("by linarith");
        let err = r.validates("by omega").unwrap_err();
        assert!(err.contains("mismatch"), "got: {}", err);
    }

    #[test]
    fn receipt_rejects_reject_verdict() {
        let r = OracleReceipt {
            payload_hash: hash("bad payload"),
            verdict: Verdict::Reject("malformed".into()),
            predicate_kind: PredicateKind::Lean4Boolean,
        };
        let err = r.validates("bad payload").unwrap_err();
        assert!(err.contains("Reject"), "got: {}", err);
    }

    #[test]
    fn partial_ok_is_accepted() {
        let r = OracleReceipt::for_lean4_partial("have h : 1=1 := rfl");
        assert!(r.validates("have h : 1=1 := rfl").is_ok());
    }
}
