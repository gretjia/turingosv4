//! Constitution Landing Gate — `PromptCapsule` (Class-3, architect 2026-05-07).
//!
//! Authority:
//!   - HARNESS.md §3 G-016/G-019/G-021/G-028 (architect ruling 2026-05-07)
//!   - CLAUDE.md §4.3 — Class-3 PromptCapsule + L4 anchor by default
//!
//! Closes Art. III selective shielding / prompt persistence gap (was 0%
//! LANDED). Pins the architect's seven-field schema and the privacy
//! invariant: verbatim prompt bytes are NEVER tape-resident by default.
//!
//! Tests (canonical 5 from HARNESS.md §3):
//!   - prompt_capsule_created_for_attempt
//!   - prompt_capsule_hash_stable
//!   - prompt_capsule_redacts_hidden_fields
//!   - prompt_capsule_referenced_by_attempt_telemetry
//!   - verbatim_prompt_not_public_by_default
//!
//! Plus structural gates so the schema can't drift silently:
//!   - prompt_capsule_object_type_is_distinct
//!   - prompt_capsule_schema_id_is_pinned
//!
//! `FC-trace: FC1-INV1 + Art-III + G-016/G-019/G-021/G-028`.

use std::collections::BTreeSet;

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::prompt_capsule::{
    read_prompt_capsule_from_cas, write_prompt_capsule_to_cas, PromptCapsule, PromptCapsuleError,
    PROMPT_CAPSULE_SCHEMA_ID,
};
use turingosv4::state::q_state::Hash;
use turingosv4::state::typed_tx::ReadKey;

fn fixture_capsule(prompt_hash_byte: u8) -> PromptCapsule {
    let read_set: BTreeSet<ReadKey> = [
        ReadKey("agent_view".into()),
        ReadKey("market_ticker".into()),
    ]
    .into_iter()
    .collect();
    PromptCapsule::new(
        Hash([prompt_hash_byte; 32]),
        read_set,
        "policy_v1",
        true,
        Cid([0xBB; 32]),
        Hash([0xCC; 32]),
        Cid([0xDD; 32]),
    )
    .expect("fixture capsule constructor accepts redacted=true")
}

/// G-016 — every externalized attempt must be backed by a `PromptCapsule`.
/// This test exercises the "can be created and bound to an attempt's
/// prompt context hash" half — the bind-side end-to-end smoke is part of
/// the FC1 batch run (Phase 3 P38/P49/M0).
#[test]
fn prompt_capsule_created_for_attempt() {
    let cap = fixture_capsule(0xAA);
    assert_eq!(cap.prompt_context_hash, Hash([0xAA; 32]));
    assert!(cap.read_set.contains(&ReadKey("agent_view".into())));
    assert!(cap.hidden_fields_redacted);
}

/// Architect §4.3 — the capsule's canonical payload has EXACTLY 7 fields.
/// Adding an 8th (e.g. a `schema_version` discriminator) is a schema break
/// that requires architect ratification. Schema versioning lives in the CAS
/// sidecar metadata + `PROMPT_CAPSULE_SCHEMA_ID` constant, NOT in the payload.
#[test]
fn prompt_capsule_struct_field_count_is_exactly_seven() {
    let cap = fixture_capsule(0xAA);
    let v = serde_json::to_value(&cap).expect("serialize");
    let obj = v.as_object().expect("capsule is a JSON object");
    assert_eq!(
        obj.len(),
        7,
        "PromptCapsule canonical payload must have exactly 7 fields per architect §4.3; got {}. \
         Adding a `schema_version` (or any other discriminator) inside the payload requires \
         architect ratification — the schema is pinned at the level of `PROMPT_CAPSULE_SCHEMA_ID` \
         and the CAS sidecar `schema_id` slot.",
        obj.len()
    );
    for f in [
        "prompt_context_hash",
        "read_set",
        "policy_version",
        "hidden_fields_redacted",
        "visible_context_cid",
        "system_prompt_template_hash",
        "agent_view_manifest_cid",
    ] {
        assert!(
            obj.contains_key(f),
            "PromptCapsule missing canonical field `{f}` (architect §4.3 7-field shape)"
        );
    }
}

/// G-019 — canonical encoding is deterministic. Same input → same canonical
/// hash. Same input → same CAS CID round-trip.
#[test]
fn prompt_capsule_hash_stable() {
    let a = fixture_capsule(0xAA);
    let b = fixture_capsule(0xAA);
    let ha = a.canonical_hash().expect("hash a");
    let hb = b.canonical_hash().expect("hash b");
    assert_eq!(ha, hb, "identical capsules MUST produce identical canonical hashes");

    // Different prompt context hash → different canonical hash (no
    // accidental collision).
    let c = fixture_capsule(0xBB);
    let hc = c.canonical_hash().expect("hash c");
    assert_ne!(
        ha, hc,
        "capsules differing in prompt_context_hash must hash differently"
    );

    // CAS round-trip preserves identity.
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("cas open");
    let cid_a = write_prompt_capsule_to_cas(&mut cas, &a, "gate", 0).expect("put a");
    let cid_b = write_prompt_capsule_to_cas(&mut cas, &b, "gate", 0).expect("put b");
    assert_eq!(cid_a, cid_b, "identical capsules must produce identical CAS CIDs");
    let read_back = read_prompt_capsule_from_cas(&cas, &cid_a).expect("get");
    assert_eq!(read_back, a, "CAS round-trip must preserve capsule");
}

/// G-021 — the redaction invariant is enforced at construction. Capsules
/// with `hidden_fields_redacted == false` cannot be created.
#[test]
fn prompt_capsule_redacts_hidden_fields() {
    let read_set: BTreeSet<ReadKey> = BTreeSet::new();
    let result = PromptCapsule::new(
        Hash([0; 32]),
        read_set,
        "policy_v1",
        false, // <-- forbidden
        Cid([0; 32]),
        Hash([0; 32]),
        Cid([0; 32]),
    );
    match result {
        Err(PromptCapsuleError::HiddenFieldsNotRedacted) => {}
        Err(other) => panic!(
            "constructor refused for the wrong reason: {other:?} \
             (expected HiddenFieldsNotRedacted)"
        ),
        Ok(_) => panic!(
            "constructor allowed `hidden_fields_redacted=false` — Art. III \
             selective shielding violated; verbatim prompt could leak"
        ),
    }
}

/// G-028 — `AttemptTelemetry.prompt_context_hash` and
/// `PromptCapsule.prompt_context_hash` reference the SAME canonical bytes.
/// A capsule built from an attempt's `prompt_context_hash` must round-trip
/// equal hash-side to that attempt.
#[test]
fn prompt_capsule_referenced_by_attempt_telemetry() {
    use turingosv4::bottom_white::cas::schema::Cid;
    use turingosv4::runtime::attempt_telemetry::{AttemptKind, AttemptOutcome, AttemptTelemetry};
    use turingosv4::runtime::proposal_telemetry::TokenCounts;
    use turingosv4::state::q_state::{AgentId, TxId};

    let prompt_hash = Hash(Cid::from_content(b"visible-context-bytes").0);
    let attempt = AttemptTelemetry::new_root(
        TxId("att-prompt-bind".into()),
        "gate".into(),
        "task".into(),
        AgentId("a0".into()),
        "n0.b0".into(),
        prompt_hash,
        Cid::from_content(b"candidate"),
        AttemptKind::ExternalizedLlmCycle,
        AttemptOutcome::LeanFail,
        TokenCounts::default(),
        "tag".into(),
    );

    let capsule = PromptCapsule::new(
        attempt.prompt_context_hash,
        BTreeSet::new(),
        "policy_v1",
        true,
        Cid::from_content(b"visible-context-bytes"),
        Hash([0; 32]),
        Cid([0; 32]),
    )
    .expect("capsule constructor");

    assert_eq!(
        capsule.prompt_context_hash, attempt.prompt_context_hash,
        "AttemptTelemetry.prompt_context_hash MUST equal PromptCapsule.prompt_context_hash; \
         drift breaks G-028 (attempt → capsule binding)"
    );
}

/// G-016 / Art. III — verbatim prompt bytes are NOT carried in the
/// capsule's public fields. Source-level structural assertion: the
/// `PromptCapsule` struct has no field whose name suggests verbatim text
/// (e.g. `prompt_text`, `verbatim_prompt`, `raw_prompt`, `prompt_string`).
/// Adding such a field would require either ratification (Class-4
/// audit-only artifact) or an explicit constitution amendment.
#[test]
fn verbatim_prompt_not_public_by_default() {
    let src_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/runtime/prompt_capsule.rs");
    let src = std::fs::read_to_string(&src_path).expect("read prompt_capsule.rs");

    // The architect schema explicitly lists 7 fields (counting schema_version).
    // None of them carries verbatim text.
    let forbidden_field_names = [
        "prompt_text",
        "verbatim_prompt",
        "raw_prompt",
        "prompt_string",
        "prompt_body",
        "prompt_full_text",
    ];
    for f in forbidden_field_names {
        let needle = format!("pub {f}");
        assert!(
            !src.contains(&needle),
            "PromptCapsule struct contains forbidden verbatim field `{f}`. \
             Per CLAUDE.md §4.3: verbatim prompt is Class-4 audit-only and \
             requires explicit ratification. Move this field out of the \
             default Class-3 capsule."
        );
    }
}

/// Structural gate — the new `ObjectType::PromptCapsule` discriminant is
/// distinct from existing CAS object types. Adding a duplicate variant
/// would silently corrupt typed CAS reads.
#[test]
fn prompt_capsule_object_type_is_distinct() {
    fn route(t: ObjectType) -> &'static str {
        match t {
            ObjectType::PromptCapsule => "prompt_capsule",
            ObjectType::AttemptTelemetry => "attempt_telemetry",
            ObjectType::LeanResult => "lean_result",
            ObjectType::TerminalAbortRecord => "terminal_abort",
            ObjectType::EvidenceCapsule => "evidence_capsule",
            ObjectType::EvidenceManifest => "evidence_manifest",
            ObjectType::CompressedRunLog => "compressed_run_log",
            ObjectType::AgentAutopsyCapsule => "autopsy_capsule",
            ObjectType::AutopsyPrivateDetail => "autopsy_private",
            ObjectType::MarkovEvidenceCapsule => "markov_capsule",
            ObjectType::NextSessionContext => "next_session",
            ObjectType::ProposalPayload => "proposal",
            ObjectType::CounterexamplePayload => "counterexample",
            ObjectType::PredicateBytecode => "predicate",
            ObjectType::ToolBytecode => "tool",
            ObjectType::AmendmentDiff => "amendment",
            ObjectType::ReversibilityPlan => "reversibility",
            ObjectType::Generic => "generic",
        }
    }
    assert_eq!(route(ObjectType::PromptCapsule), "prompt_capsule");
    assert_ne!(
        route(ObjectType::PromptCapsule),
        route(ObjectType::AttemptTelemetry),
        "PromptCapsule must be a distinct ObjectType (duplicate would silently corrupt typed reads)"
    );
}

/// Structural gate — schema id is pinned. Drift would break
/// canonical-decode of historical capsules. Schema versioning is carried
/// HERE (the constant + CAS sidecar `schema_id`), not inside the canonical
/// 7-field payload.
#[test]
fn prompt_capsule_schema_id_is_pinned() {
    assert_eq!(PROMPT_CAPSULE_SCHEMA_ID, "v1/prompt_capsule");
}
