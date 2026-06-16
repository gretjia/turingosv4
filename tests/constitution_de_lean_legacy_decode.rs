//! De-Lean kernel migration (2026-06-15, Class-4 §8) — historical-decode regression.
//!
//! Authority: `handover/tracer_bullets/DE_LEAN_KERNEL_MIGRATION_SPEC_2026-06-15.md`
//! (architect-ratified §8). The migration renames Lean-named kernel identities to
//! generic ones. Because three of those identities are burned into historical tape
//! BY NAME/NUMBER, the rename is forward-compat (serde `rename`/`alias`, discriminant
//! numbers held), NOT a raw rename. Per AGENTS.md §8 ("Never retroactively rewrite
//! old ChainTape/L4/L4.E/CAS evidence") + the spec implementation plan ("Add a
//! regression test that a historical `LeanResult`/`LeanFailed`-named record still
//! decodes after the rename"), this test is the machine check that no historical
//! decode broke.
//!
//! Style mirrors the proposal_telemetry v1→v2 legacy-decode precedent
//! (`src/runtime/proposal_telemetry.rs::v1_record_still_decodes_after_v2_bump`):
//! encode/author a record in its EXACT historical on-wire form, then prove the
//! current renamed type still decodes it byte-faithfully.
//!
//! Three burned-in identities, one test each:
//!   (a) CAS `ObjectType` serde NAME `"LeanResult"` (hashed into the CAS
//!       `canonical_hash` Merkle, `schema.rs`) → `ObjectType::DomainProofResult`.
//!   (b) L4.E rejection JSONL/serde record `rejection_class` NAMES `"LeanFailed"` /
//!       `"SorryBlocked"` (serde alias, `rejection_evidence.rs`) →
//!       `RejectionClass::{CheckerFailed, IncompleteProofBlocked}`.
//!   (c) `BenchmarkManifest` JSON keys `"lean_version"` / `"mathlib_commit"`
//!       (serde alias, `benchmark_manifest.rs`) →
//!       `{verifier_version, verifier_library_commit}`.

use turingosv4::bottom_white::cas::schema::{CasObjectMetadata, Cid, ObjectType};
use turingosv4::bottom_white::ledger::rejection_evidence::{
    RejectedSubmissionRecord, RejectionClass,
};
use turingosv4::runtime::benchmark_manifest::BenchmarkManifest;

// ─────────────────────────────────────────────────────────────────────────────
// (a) ObjectType — the highest-blast-radius atom. The serde NAME `"LeanResult"`
// is serialized into `CasObjectMetadata::canonical_hash` (`schema.rs`:
// `h.update(serde_json::to_vec(&self.object_type)...)`). A naive variant rename
// would silently change every historical CAS object's hash and break the whole
// Merkle reconstruction. `#[serde(rename = "LeanResult")]` pins the on-wire string.
// ─────────────────────────────────────────────────────────────────────────────

/// A historical `CasObjectMetadata` whose `object_type` was written to the wire as
/// `"LeanResult"` must (1) deserialize into the renamed `ObjectType::DomainProofResult`
/// variant, and (2) reconstruct the IDENTICAL `canonical_hash` — proving the rename
/// did not perturb a single byte of any historical CAS Merkle leaf.
#[test]
fn de_lean_object_type_lean_result_wire_string_still_decodes() {
    // (1) The bare on-wire ObjectType form a historical writer emitted.
    let decoded: ObjectType =
        serde_json::from_str("\"LeanResult\"").expect("legacy \"LeanResult\" must deserialize");
    assert_eq!(
        decoded,
        ObjectType::DomainProofResult,
        "historical CAS object_type wire-string \"LeanResult\" must read as the renamed \
         ObjectType::DomainProofResult variant (serde rename is bidirectional)"
    );

    // (2) Author a historical CAS metadata JSON line carrying object_type:"LeanResult"
    // — exactly the bytes a pre-migration writer persisted — and deserialize it. This
    // is the form whose `canonical_hash` is a Merkle leaf.
    let legacy_metadata_json = r#"{
        "cid": [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32],
        "backend_oid_hex": "deadbeef",
        "object_type": "LeanResult",
        "creator": "system",
        "created_at_logical_t": 42,
        "schema_id": "turingosv4.lean_result.v2",
        "size_bytes": 128
    }"#;
    let legacy_meta: CasObjectMetadata =
        serde_json::from_str(legacy_metadata_json).expect("historical CAS metadata must decode");
    assert_eq!(legacy_meta.object_type, ObjectType::DomainProofResult);

    // The renamed variant must reproduce the EXACT historical Merkle leaf hash. We
    // build today's metadata with the renamed variant and identical other fields; its
    // canonical_hash must byte-equal the one reconstructed from the legacy wire bytes.
    let mut cid_bytes = [0u8; 32];
    for (i, b) in cid_bytes.iter_mut().enumerate() {
        *b = (i + 1) as u8;
    }
    let renamed_meta = CasObjectMetadata {
        cid: Cid(cid_bytes),
        backend_oid_hex: "deadbeef".to_string(),
        object_type: ObjectType::DomainProofResult,
        creator: "system".to_string(),
        created_at_logical_t: 42,
        schema_id: Some("turingosv4.lean_result.v2".to_string()),
        size_bytes: 128,
    };
    assert_eq!(
        legacy_meta.canonical_hash(),
        renamed_meta.canonical_hash(),
        "the de-Lean rename MUST preserve the historical CAS canonical_hash Merkle leaf \
         byte-for-byte (DE_LEAN §8 highest-blast-radius atom); a mismatch means every \
         historical DomainProofResult CAS object's hash changed under us"
    );

    // Going-forward serialization must still emit the pinned on-wire string so newly
    // written objects remain in the same Merkle namespace as historical ones.
    assert_eq!(
        serde_json::to_string(&ObjectType::DomainProofResult).expect("serialize"),
        "\"LeanResult\"",
        "DomainProofResult must serialize back to the pinned on-wire \"LeanResult\" string"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// (b) RejectionClass — the L4.E rejection record. The variant NAME is serialized
// into the JSONL L4.E forensic sidecar (and `agent_audit_trail.rs`); the
// discriminant NUMBER is hashed into the RejectionDigest. The rename keeps the
// number (CheckerFailed=6, IncompleteProofBlocked=8) AND adds
// `#[serde(alias="LeanFailed")]` / `#[serde(alias="SorryBlocked")]` so every
// historical JSONL row still deserializes.
// ─────────────────────────────────────────────────────────────────────────────

/// A historical L4.E rejection JSONL/serde record whose `rejection_class` was
/// written as `"LeanFailed"` or `"SorryBlocked"` must still deserialize into the
/// renamed `RejectionClass::{CheckerFailed, IncompleteProofBlocked}` variants —
/// via the serde aliases — and the discriminant numbers must be unchanged (6 / 8),
/// since those numbers are hashed into the historical RejectionDigest.
#[test]
fn de_lean_rejection_class_legacy_names_still_deserialize() {
    // Bare-enum alias check (the minimal serde contract).
    assert_eq!(
        serde_json::from_str::<RejectionClass>("\"LeanFailed\"")
            .expect("legacy \"LeanFailed\" must deserialize"),
        RejectionClass::CheckerFailed,
    );
    assert_eq!(
        serde_json::from_str::<RejectionClass>("\"SorryBlocked\"")
            .expect("legacy \"SorryBlocked\" must deserialize"),
        RejectionClass::IncompleteProofBlocked,
    );

    // Discriminant NUMBERS held — these are the values hashed into the historical
    // RejectionDigest (`rejection_evidence.rs`); changing them would silently break
    // every historical digest.
    assert_eq!(RejectionClass::CheckerFailed as u8, 6);
    assert_eq!(RejectionClass::IncompleteProofBlocked as u8, 8);

    // Full historical L4.E forensic JSONL row (the persisted serde record, with the
    // legacy "LeanFailed" rejection_class name) must deserialize whole.
    // `raw_diagnostic_cid` is `#[serde(skip_serializing, default)]` on the public
    // record, so the persisted line omits it — mirroring the real sidecar shape.
    let zero32 = "[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]";
    let legacy_jsonl_lean_failed = format!(
        r#"{{
            "submit_id": 7,
            "parent_state_root": {zero32},
            "agent_id": "legacy_agent_n1",
            "tx_kind": "Verify",
            "tx_payload_cid": {zero32},
            "rejection_class": "LeanFailed",
            "public_summary": "verifier rejected the proof",
            "prev_hash": {zero32},
            "hash": {zero32}
        }}"#
    );
    let rec: RejectedSubmissionRecord = serde_json::from_str(&legacy_jsonl_lean_failed)
        .expect("historical L4.E JSONL row with \"LeanFailed\" must deserialize");
    assert_eq!(rec.rejection_class, RejectionClass::CheckerFailed);
    assert_eq!(rec.submit_id, 7);

    let legacy_jsonl_sorry_blocked = format!(
        r#"{{
            "submit_id": 8,
            "parent_state_root": {zero32},
            "agent_id": "legacy_agent_n2",
            "tx_kind": "Verify",
            "tx_payload_cid": {zero32},
            "rejection_class": "SorryBlocked",
            "public_summary": "proof incomplete",
            "prev_hash": {zero32},
            "hash": {zero32}
        }}"#
    );
    let rec2: RejectedSubmissionRecord = serde_json::from_str(&legacy_jsonl_sorry_blocked)
        .expect("historical L4.E JSONL row with \"SorryBlocked\" must deserialize");
    assert_eq!(rec2.rejection_class, RejectionClass::IncompleteProofBlocked);
    assert_eq!(rec2.submit_id, 8);
}

// ─────────────────────────────────────────────────────────────────────────────
// (c) BenchmarkManifest — the manifest JSON keys `"lean_version"` /
// `"mathlib_commit"` were renamed to `verifier_version` / `verifier_library_commit`
// with `#[serde(alias)]` so every historical manifest JSON still deserializes.
// ─────────────────────────────────────────────────────────────────────────────

/// A historical `BenchmarkManifest` JSON carrying the OLD keys `"lean_version"` and
/// `"mathlib_commit"` must still deserialize, mapping those values into the renamed
/// `verifier_version` / `verifier_library_commit` fields via the serde aliases.
#[test]
fn de_lean_benchmark_manifest_legacy_keys_still_deserialize() {
    let forty_hex = "0".repeat(40);
    let legacy_manifest_json = format!(
        r#"{{
            "batch_id": "tb_18b_m1_legacy",
            "problem_ids": ["p1", "p2"],
            "model_id": "deepseek-chat",
            "model_ver": "v3",
            "temperature_decimal": "0.0",
            "max_tx_budget": 200,
            "n_per_problem": 3,
            "seeds": [1, 2, 3],
            "lean_version": "4.7.0",
            "mathlib_commit": "{forty_hex}",
            "turingos_commit": "{forty_hex}",
            "strategy": "M1 legacy 3-seed",
            "schema_id": "turingosv4.benchmark_manifest.v1"
        }}"#
    );
    let m: BenchmarkManifest = serde_json::from_str(&legacy_manifest_json)
        .expect("historical manifest with lean_version/mathlib_commit must deserialize");
    assert_eq!(
        m.verifier_version, "4.7.0",
        "legacy \"lean_version\" must map into verifier_version via serde alias"
    );
    assert_eq!(
        m.verifier_library_commit, forty_hex,
        "legacy \"mathlib_commit\" must map into verifier_library_commit via serde alias"
    );

    // And the legacy manifest must still pass validation (the rename did not change
    // the field semantics, only the identifier).
    m.validate()
        .expect("legacy-decoded manifest must still validate");
}
