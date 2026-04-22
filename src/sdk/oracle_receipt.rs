//! OracleReceipt — capability token authorizing blessed writes to tape.
//!
//! Closes Codex V-1 (C-067): `bus.append_oracle_accepted()` accepts
//! `oracle_blessed = true` but only if the caller produces a receipt that
//! matches (a) the payload, (b) the parent-chain context, and (c) an oracle
//! nonce that was explicitly registered with the bus.
//!
//! **Threat model (Paper 1, closed-world in-process)**:
//! - Private fields force receipt creation through the `new` constructor
//!   which takes an `oracle_nonce: u64`.
//! - Bus only accepts receipts whose nonce was registered via
//!   `TuringBus::register_oracle(nonce)`. The nonce is generated inside
//!   `Lean4Oracle::new()` and held in a private field; only code that
//!   owns the `Lean4Oracle` reference can read it.
//! - Context hash binds the receipt to `parent_id` → cross-context replay
//!   (step-mode tactic with different parent chain) is rejected.
//!
//! **Not yet defended (Phase 10c)**: external agent impersonation requires
//! Ed25519 signature from oracle's secret key. Documented as known limit.
//!
//! See `cases/C-067_oracle_receipt_capability.yaml` for ruling.

use crate::sdk::predicate::{PredicateKind, Verdict};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Capability token proving a predicate accepted a payload in a specific context.
///
/// Fields are private. Construct via `new_*` constructors, which require
/// an `oracle_nonce` (only available to code with a `Lean4Oracle` ref).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleReceipt {
    payload_hash: [u8; 32],
    /// Hash of the parent chain context this receipt was issued against.
    /// Prevents cross-context replay (e.g. step-mode tactic verified against
    /// tape chain A but submitted with tape chain B).
    context_hash: [u8; 32],
    /// The issuing oracle's per-instance nonce. Bus only accepts receipts
    /// whose nonce it registered via `register_oracle()`.
    oracle_nonce: u64,
    verdict: Verdict,
    predicate_kind: PredicateKind,
    // Phase 10c: signature: Option<[u8; 64]> for external agent authorization.
}

impl OracleReceipt {
    /// Low-level constructor. Prefer `new_lean4_complete/partial` which
    /// compute the hashes.
    pub fn new(
        payload_hash: [u8; 32],
        context_hash: [u8; 32],
        oracle_nonce: u64,
        verdict: Verdict,
        predicate_kind: PredicateKind,
    ) -> Self {
        Self {
            payload_hash,
            context_hash,
            oracle_nonce,
            verdict,
            predicate_kind,
        }
    }

    /// Receipt for a Lean4-boolean `Complete` verdict (full proof accepted).
    /// `parent_id` is the tape node this proof cites (None for oneshot).
    /// `oracle_nonce` must match a nonce registered on the destination bus.
    pub fn new_lean4_complete(
        payload: &str,
        parent_id: Option<&str>,
        oracle_nonce: u64,
    ) -> Self {
        Self::new(
            hash_payload(payload),
            hash_context(parent_id),
            oracle_nonce,
            Verdict::Complete,
            PredicateKind::Lean4Boolean,
        )
    }

    /// Receipt for a Lean4-boolean `PartialOk` verdict (unsolved goals,
    /// structurally valid). `confidence = 1.0` because Lean elaboration is
    /// deterministic; PCP confidence semantics are reserved for Paper 3.
    pub fn new_lean4_partial(
        payload: &str,
        parent_id: Option<&str>,
        oracle_nonce: u64,
    ) -> Self {
        Self::new(
            hash_payload(payload),
            hash_context(parent_id),
            oracle_nonce,
            Verdict::PartialOk { confidence: 1.0 },
            PredicateKind::Lean4Boolean,
        )
    }

    pub fn oracle_nonce(&self) -> u64 {
        self.oracle_nonce
    }

    /// Validate receipt against payload + expected context. Called by the
    /// bus before granting blessed write.
    pub fn validates(
        &self,
        payload: &str,
        expected_context: &[u8; 32],
    ) -> Result<(), String> {
        let computed = hash_payload(payload);
        if computed != self.payload_hash {
            return Err("payload_hash mismatch (payload tampered or stale receipt)".into());
        }
        if self.context_hash != *expected_context {
            return Err("context_hash mismatch (receipt replay across different parent)".into());
        }
        match &self.verdict {
            Verdict::Complete | Verdict::PartialOk { .. } => Ok(()),
            Verdict::Reject(r) => Err(format!("verdict is Reject: {}", r)),
        }
    }
}

/// Hash a payload string (sha256).
pub fn hash_payload(payload: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(payload.as_bytes());
    h.finalize().into()
}

/// Hash the parent-chain context. Currently: `sha256(parent_id || "")`.
/// For Paper 1 closed-world, binding to parent_id is sufficient — each
/// parent_id uniquely identifies a tape history in-process (tx_count
/// monotone + per-author). Phase 10c will extend to chain-full hash when
/// Ed25519 signatures are added.
pub fn hash_context(parent_id: Option<&str>) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(parent_id.unwrap_or("").as_bytes());
    h.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_NONCE: u64 = 0xDEADBEEFCAFE_u64;

    #[test]
    fn receipt_validates_matching_payload_and_context() {
        let r = OracleReceipt::new_lean4_complete("by linarith", None, TEST_NONCE);
        let ctx = hash_context(None);
        assert!(r.validates("by linarith", &ctx).is_ok());
    }

    #[test]
    fn receipt_rejects_tampered_payload() {
        let r = OracleReceipt::new_lean4_complete("by linarith", None, TEST_NONCE);
        let ctx = hash_context(None);
        let err = r.validates("by omega", &ctx).unwrap_err();
        assert!(err.contains("payload_hash mismatch"), "got: {}", err);
    }

    #[test]
    fn receipt_rejects_wrong_context() {
        // Receipt issued for parent A, replayed under parent B → reject.
        let r = OracleReceipt::new_lean4_complete("by linarith", Some("node_A"), TEST_NONCE);
        let wrong_ctx = hash_context(Some("node_B"));
        let err = r.validates("by linarith", &wrong_ctx).unwrap_err();
        assert!(err.contains("context_hash mismatch"), "got: {}", err);
    }

    #[test]
    fn receipt_rejects_reject_verdict() {
        let r = OracleReceipt::new(
            hash_payload("bad payload"),
            hash_context(None),
            TEST_NONCE,
            Verdict::Reject("malformed".into()),
            PredicateKind::Lean4Boolean,
        );
        let ctx = hash_context(None);
        let err = r.validates("bad payload", &ctx).unwrap_err();
        assert!(err.contains("Reject"), "got: {}", err);
    }

    #[test]
    fn partial_ok_is_accepted() {
        let r = OracleReceipt::new_lean4_partial("have h : 1=1 := rfl", None, TEST_NONCE);
        let ctx = hash_context(None);
        assert!(r.validates("have h : 1=1 := rfl", &ctx).is_ok());
    }

    #[test]
    fn nonce_is_accessible() {
        let r = OracleReceipt::new_lean4_complete("p", None, 0x1234);
        assert_eq!(r.oracle_nonce(), 0x1234);
    }

    #[test]
    fn context_hash_differs_by_parent() {
        let h_none = hash_context(None);
        let h_a = hash_context(Some("node_A"));
        let h_b = hash_context(Some("node_B"));
        assert_ne!(h_none, h_a);
        assert_ne!(h_a, h_b);
        assert_eq!(h_a, hash_context(Some("node_A")));  // deterministic
    }
}
