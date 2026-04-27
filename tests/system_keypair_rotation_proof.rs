use ed25519_dalek::{Signer, SigningKey};
use turingosv4::bottom_white::ledger::system_keypair::{
    canonical_digest, verify_epoch_rotation_proof, CanonicalMessage, EpochRotationProof,
    PinnedSystemPubkeys, SystemEpoch, SystemPublicKey, SystemSignature,
};

#[test]
fn rotation_proof_requires_old_and_new_epoch_signatures() {
    let old_key = SigningKey::from_bytes(&[1u8; 32]);
    let new_key = SigningKey::from_bytes(&[2u8; 32]);
    let old_pubkey = SystemPublicKey::from_bytes(old_key.verifying_key().to_bytes());
    let new_pubkey = SystemPublicKey::from_bytes(new_key.verifying_key().to_bytes());

    let proof = EpochRotationProof::new(
        SystemEpoch::new(1),
        SystemEpoch::new(2),
        old_pubkey,
        new_pubkey,
        1_777_000_000,
    );
    let message = CanonicalMessage::EpochRotationProof(proof.clone());
    let old_signature =
        SystemSignature::from_bytes(old_key.sign(&canonical_digest(&message)).to_bytes());
    let new_signature =
        SystemSignature::from_bytes(new_key.sign(&canonical_digest(&message)).to_bytes());

    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(SystemEpoch::new(1), old_pubkey);
    pinned.insert(SystemEpoch::new(2), new_pubkey);
    assert!(verify_epoch_rotation_proof(
        &proof,
        &old_signature,
        &new_signature,
        &pinned
    ));

    let wrong_new_signature =
        SystemSignature::from_bytes(old_key.sign(&canonical_digest(&message)).to_bytes());
    assert!(!verify_epoch_rotation_proof(
        &proof,
        &old_signature,
        &wrong_new_signature,
        &pinned
    ));
}
