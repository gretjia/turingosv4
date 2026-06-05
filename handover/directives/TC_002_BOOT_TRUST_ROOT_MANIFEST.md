# TC-002 Boot Trust-Root Manifest

Status: implemented first slice, non-authoritative verifier.

Authority: user TC-000..TC-021 operationalization request, OBL-014, and TC-000
Path B decision. This atom verifies the TC boot trust-root manifest without
changing `genesis_payload.toml`, typed transaction schemas, sequencer admission,
canonical signing payloads, CAS schema, or constitution/flowchart authority.

Risk class: Class 2 verifier/CLI wire-up, handled with Class 3 caution because
it checks trust-root evidence. This directive does not authorize Class 4 trust
root mutation.

Scope:

- Verify `constitution.md` SHA-256 against an explicit TC manifest.
- Verify `genesis_payload.toml` SHA-256 as the current trust-root payload.
- Verify `BootPredicateManifest::v8_production` root.
- Verify the locked Path B refs from `TcHeadRefs`.
- Expose explicit CLI gates:
  - `turingos boot --verify-manifest`
  - `turingos boot --verify-constitution-hash`
  - `turingos boot --verify-predicates`

Ship gate:

- `cargo test --test constitution_tc_boot_trust_root_manifest --no-fail-fast`
- `cargo test --test constitution_matrix_drift --no-fail-fast`
- one SHA mismatch fixture must fail closed.

Claim boundary: TC-002 is a verifier slice only. It is not a TC completion
receipt and does not prove TC-003..TC-021.
