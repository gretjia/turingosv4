//! OracleReceipt — capability token authorizing blessed writes to tape.
//!
//! Phase 8.C v3 (Codex VETO, C-067 R1-α): **real crypto-grade unforgeability**
//! via Ed25519 signature. Prior versions used only payload/context sha256 +
//! an in-process "nonce" — both were forgeable by any caller that held
//! `&mut TuringBus` (Codex 2026-04-22 re-audit VETO).
//!
//! **Threat model (Paper 1)**:
//! - Oracle holds a private `SigningKey` (Ed25519). Only that oracle can sign
//!   a receipt that verifies against its published `VerifyingKey`.
//! - Bus maintains a set of trusted `VerifyingKey` bytes, registered at setup
//!   via `register_oracle(pub_key)`. Registration is frozen on first `init()`
//!   or resume — a malicious caller cannot inject new trusted pubkeys mid-run.
//! - Capability is cryptographically unforgeable: even code with `&mut Bus`
//!   cannot mint a valid receipt without the oracle's signing key.
//!
//! **Binding in the signed message**: payload_hash || context_hash || kind
//! || verdict_encoding. This prevents (a) content tampering, (b) cross-parent
//! replay, (c) verdict swap, (d) predicate-kind swap.
//!
//! See `cases/C-067_oracle_receipt_capability.yaml` for full ruling.

use crate::sdk::predicate::{PredicateKind, Verdict};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

/// Capability token proving a predicate accepted a payload in a specific context.
///
/// Crypto-grade: signature binds payload + context + verdict. Fields are
/// private — external callers can only inspect via accessor methods.
///
/// Note: `Serialize`/`Deserialize` not derived because `[u8; 64]` (sig)
/// lacks default serde support. Receipts are runtime-only (pass-by-ref);
/// Phase 10c will revisit if receipts need to cross process boundaries.
#[derive(Debug, Clone)]
pub struct OracleReceipt {
    payload_hash: [u8; 32],
    context_hash: [u8; 32],
    verdict: Verdict,
    predicate_kind: PredicateKind,
    issuer_pub: [u8; 32],
    signature: [u8; 64],
}

impl OracleReceipt {
    /// Sign a new receipt. Only callers with access to a `SigningKey` can
    /// invoke this — namely, the oracles that own their own key.
    pub fn sign_new(
        payload: &str,
        parent_id: Option<&str>,
        verdict: Verdict,
        predicate_kind: PredicateKind,
        signing_key: &SigningKey,
    ) -> Self {
        let payload_hash = hash_payload(payload);
        let context_hash = hash_context(parent_id);
        let msg = signable_message(&payload_hash, &context_hash, &verdict, predicate_kind);
        let signature = signing_key.sign(&msg);
        let issuer_pub = signing_key.verifying_key().to_bytes();
        Self {
            payload_hash,
            context_hash,
            verdict,
            predicate_kind,
            issuer_pub,
            signature: signature.to_bytes(),
        }
    }

    /// The Ed25519 verifying key of the issuer. Bus uses this to look up
    /// whether the issuer is in its trusted set.
    pub fn issuer_pub(&self) -> &[u8; 32] {
        &self.issuer_pub
    }

    /// Full cryptographic validation:
    ///   1. Signature verifies against `verifying_key` for the canonical msg.
    ///   2. `verifying_key` bytes match `self.issuer_pub`.
    ///   3. `payload` hashes to `self.payload_hash`.
    ///   4. Expected context hash matches `self.context_hash`.
    ///   5. Verdict is non-rejecting.
    ///
    /// Called by the bus inside `append_oracle_accepted`. Independent step
    /// ordering — any single failure aborts.
    pub fn verify_and_match(
        &self,
        payload: &str,
        expected_context: &[u8; 32],
        verifying_key: &VerifyingKey,
    ) -> Result<(), String> {
        // 1. Consistency: the caller-supplied verifier must match the receipt's
        //    own claimed issuer. Otherwise someone is handing us a receipt and
        //    an unrelated pubkey.
        if verifying_key.to_bytes() != self.issuer_pub {
            return Err("issuer_pub does not match provided verifying key".into());
        }
        // 2. Canonical message reconstruction.
        let msg = signable_message(
            &self.payload_hash,
            &self.context_hash,
            &self.verdict,
            self.predicate_kind,
        );
        // 3. Ed25519 signature verification.
        let sig = Signature::from_bytes(&self.signature);
        verifying_key
            .verify(&msg, &sig)
            .map_err(|e| format!("signature verification failed: {}", e))?;
        // 4. Payload binding.
        let computed_payload = hash_payload(payload);
        if computed_payload != self.payload_hash {
            return Err("payload_hash mismatch (payload tampered or stale receipt)".into());
        }
        // 5. Context binding.
        if self.context_hash != *expected_context {
            return Err("context_hash mismatch (receipt replay across different parent)".into());
        }
        // 6. Verdict must be accepting.
        match &self.verdict {
            Verdict::Complete | Verdict::PartialOk { .. } => Ok(()),
            Verdict::Reject(r) => Err(format!("verdict is Reject: {}", r)),
        }
    }
}

/// Hash a payload string (sha256) — public so tests / audit scripts can
/// reconstruct.
pub fn hash_payload(payload: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(payload.as_bytes());
    h.finalize().into()
}

/// Hash the parent-chain context (sha256 of parent_id str). Bus uses this
/// to compute the expected context during validation. For Paper 1 closed-
/// world, parent_id is sufficient since each parent uniquely identifies a
/// tape history in-process.
pub fn hash_context(parent_id: Option<&str>) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(parent_id.unwrap_or("").as_bytes());
    h.finalize().into()
}

/// Canonical message bytes that get signed. Layout:
///   [0..32)   payload_hash (sha256)
///   [32..64)  context_hash (sha256 of parent_id str)
///   [64]      predicate_kind byte
///   [65]      verdict tag (0=Complete, 1=PartialOk, 2=Reject)
///   for PartialOk: next 8 bytes = confidence f64 LE
///   for Reject:    next 4 bytes = reason len LE, then reason bytes
fn signable_message(
    payload_hash: &[u8; 32],
    context_hash: &[u8; 32],
    verdict: &Verdict,
    kind: PredicateKind,
) -> Vec<u8> {
    let mut msg = Vec::with_capacity(66);
    msg.extend_from_slice(payload_hash);
    msg.extend_from_slice(context_hash);
    msg.push(kind as u8);
    match verdict {
        Verdict::Complete => msg.push(0),
        Verdict::PartialOk { confidence } => {
            msg.push(1);
            msg.extend_from_slice(&confidence.to_le_bytes());
        }
        Verdict::Reject(reason) => {
            msg.push(2);
            let bytes = reason.as_bytes();
            msg.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
            msg.extend_from_slice(bytes);
        }
    }
    msg
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    fn test_key() -> SigningKey {
        SigningKey::generate(&mut OsRng)
    }

    #[test]
    fn receipt_verifies_correct_payload_and_context() {
        let sk = test_key();
        let vk = sk.verifying_key();
        let r = OracleReceipt::sign_new(
            "by linarith", None, Verdict::Complete, PredicateKind::Lean4Boolean, &sk,
        );
        let ctx = hash_context(None);
        assert!(r.verify_and_match("by linarith", &ctx, &vk).is_ok());
    }

    #[test]
    fn receipt_rejects_tampered_payload() {
        let sk = test_key();
        let vk = sk.verifying_key();
        let r = OracleReceipt::sign_new(
            "by linarith", None, Verdict::Complete, PredicateKind::Lean4Boolean, &sk,
        );
        let ctx = hash_context(None);
        let err = r.verify_and_match("by omega", &ctx, &vk).unwrap_err();
        assert!(err.contains("payload_hash mismatch"), "got: {}", err);
    }

    #[test]
    fn receipt_rejects_wrong_context() {
        let sk = test_key();
        let vk = sk.verifying_key();
        let r = OracleReceipt::sign_new(
            "by linarith", Some("node_A"), Verdict::Complete, PredicateKind::Lean4Boolean, &sk,
        );
        let wrong_ctx = hash_context(Some("node_B"));
        let err = r.verify_and_match("by linarith", &wrong_ctx, &vk).unwrap_err();
        assert!(err.contains("context_hash mismatch"), "got: {}", err);
    }

    #[test]
    fn receipt_rejects_wrong_verifier() {
        // Two oracles with distinct keys — receipt from A must not verify under B.
        let sk_a = test_key();
        let sk_b = test_key();
        let vk_b = sk_b.verifying_key();
        let r = OracleReceipt::sign_new(
            "p", None, Verdict::Complete, PredicateKind::Lean4Boolean, &sk_a,
        );
        let ctx = hash_context(None);
        let err = r.verify_and_match("p", &ctx, &vk_b).unwrap_err();
        assert!(err.contains("issuer_pub") || err.contains("signature"),
            "got: {}", err);
    }

    #[test]
    fn tampered_signature_rejected() {
        let sk = test_key();
        let vk = sk.verifying_key();
        let mut r = OracleReceipt::sign_new(
            "p", None, Verdict::Complete, PredicateKind::Lean4Boolean, &sk,
        );
        // Flip a byte in the signature — must fail verification.
        r.signature[0] ^= 0xFF;
        let ctx = hash_context(None);
        let err = r.verify_and_match("p", &ctx, &vk).unwrap_err();
        assert!(err.contains("signature"), "got: {}", err);
    }

    #[test]
    fn attacker_cannot_forge_with_own_key_under_victim_pub() {
        // Attacker generates own SigningKey, signs fake receipt, but claims
        // victim's pubkey. verify_and_match must reject because the signature
        // won't verify under the provided (victim) verifier.
        let victim_sk = test_key();
        let victim_vk = victim_sk.verifying_key();
        let attacker_sk = test_key();
        // Sign receipt with attacker's key, but we'll hand bus the victim's vk.
        let mut fake = OracleReceipt::sign_new(
            "malicious", None, Verdict::Complete, PredicateKind::Lean4Boolean, &attacker_sk,
        );
        // Swap the claimed issuer_pub to victim's so the consistency check passes —
        // but the signature was still made by attacker.
        fake.issuer_pub = victim_vk.to_bytes();
        let ctx = hash_context(None);
        let err = fake.verify_and_match("malicious", &ctx, &victim_vk).unwrap_err();
        assert!(err.contains("signature"),
            "attacker forgery must be caught by sig verification; got: {}", err);
    }

    #[test]
    fn partial_ok_receipt_accepted() {
        let sk = test_key();
        let vk = sk.verifying_key();
        let r = OracleReceipt::sign_new(
            "have h : 1=1 := rfl", None,
            Verdict::PartialOk { confidence: 1.0 },
            PredicateKind::Lean4Boolean, &sk,
        );
        let ctx = hash_context(None);
        assert!(r.verify_and_match("have h : 1=1 := rfl", &ctx, &vk).is_ok());
    }

    #[test]
    fn reject_verdict_rejected() {
        let sk = test_key();
        let vk = sk.verifying_key();
        let r = OracleReceipt::sign_new(
            "bad", None,
            Verdict::Reject("malformed".into()),
            PredicateKind::Lean4Boolean, &sk,
        );
        let ctx = hash_context(None);
        let err = r.verify_and_match("bad", &ctx, &vk).unwrap_err();
        assert!(err.contains("Reject"), "got: {}", err);
    }
}
