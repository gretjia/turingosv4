use ed25519_dalek::{Signer, SigningKey};
use turingosv4::bottom_white::ledger::system_keypair::{
    canonical_digest, verify_system_signature, CanonicalMessage, PinnedSystemPubkeys,
    RejectedAttemptSummary, SystemEpoch, SystemPublicKey, SystemSignature,
};

#[test]
fn verify_round_trip_uses_correct_epoch_pubkey_lookup() {
    let signing_key = SigningKey::from_bytes(&[42u8; 32]);
    let public_key = SystemPublicKey::from_bytes(signing_key.verifying_key().to_bytes());
    let epoch = SystemEpoch::new(7);

    let message = CanonicalMessage::RejectedAttemptSummary(RejectedAttemptSummary::new(
        "run-7",
        "attempt-3",
        "predicate_reject",
        [9u8; 32],
    ));
    let signature =
        SystemSignature::from_bytes(signing_key.sign(&canonical_digest(&message)).to_bytes());

    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(epoch, public_key);
    assert!(verify_system_signature(
        &signature, &message, epoch, &pinned
    ));
    assert!(!verify_system_signature(
        &signature,
        &message,
        SystemEpoch::new(8),
        &pinned
    ));

    let tampered = CanonicalMessage::RejectedAttemptSummary(RejectedAttemptSummary::new(
        "run-7",
        "attempt-4",
        "predicate_reject",
        [9u8; 32],
    ));
    assert!(!verify_system_signature(
        &signature, &tampered, epoch, &pinned
    ));
}
