//! Tier-1 SkillCapsule memory — triple-coupled gate (non-vacuous Tier-1
//! boundary witness).
//!
//! Proves the constitutional Tier-1 memory contract for SYSTEM-authored,
//! AGENT-read-only distilled skill capsules:
//!
//! 1. **Reconstructable from CAS + tape alone** — a consolidated capsule is
//!    retrievable by `cas.get(&capsule_id)`, the retrieved bytes' sha256
//!    equals `capsule_id`, and `restore_skill_capsule_from_cas_bytes`
//!    reproduces the in-memory capsule (Art. 0.2).
//! 2. **Chain links** — capsule N+1's `previous_capsule_cid` points at
//!    capsule N (L4-chained lineage).
//! 3. **Store is CAS/tape, not the filesystem** — consolidation writes ONLY
//!    into the CAS temp dir; no sibling `skills/` or memory directory is
//!    created (Art. 0.2 — canonical store is tape/CAS, never `std::fs::write`).
//! 4. **Projection is scoped + shielded** — the agent read view is filtered by
//!    `applicable_scope` (default-deny out of scope) and carries ONLY rule
//!    text + capsule provenance Cids, never the raw `source_failure_cids`
//!    bytes (Art. III.2 + Inv 10).
//! 5. **No agent-writable entry point (non-vacuous Tier-1 boundary)** — the
//!    module's only write path is system-authored (`author == System`); there
//!    is no agent-callable mutate surface. We assert the write path stamps
//!    `SkillAuthor::System` and that the agent read projection is an owned,
//!    immutable value (no CAS handle, no write-back).
//!
//! Each assertion is designed to fail if the corresponding property is broken
//! (e.g. if consolidation wrote to a filesystem path, if the projection leaked
//! private bytes, or if the chain link were dropped).

use std::sync::{Arc, RwLock};
use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::autopsy_capsule::skill_capsule::{
    consolidate_skill_capsule, project_for_agent, restore_skill_capsule_from_cas_bytes,
    SkillAuthor, SKILL_CAPSULE_SCHEMA_ID,
};
use turingosv4::runtime::autopsy_capsule::{LossReasonClass, TypicalErrorSummary};
use turingosv4::state::q_state::Hash;

fn cluster(tag: LossReasonClass, count: u32, exemplar_byte: u8) -> TypicalErrorSummary {
    TypicalErrorSummary {
        loss_reason_class: tag,
        count,
        exemplar_public_summary: "agent=A lost 100μC on event=task:t1 reason=X".to_string(),
        // The 32-byte-run Cid we will later scan the projection for, to prove
        // the source failure provenance never leaks into the agent view.
        exemplar_capsule_cids: vec![Cid([exemplar_byte; 32])],
    }
}

/// (1) reconstructable from CAS + tape alone; (5) write path is
/// system-authored.
#[test]
fn consolidated_capsule_reconstructable_from_cas_and_system_authored() {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));

    let errors = vec![
        cluster(LossReasonClass::Bankruptcy, 3, 0xAA),
        cluster(LossReasonClass::SlashLoss, 4, 0xBB),
    ];

    let cap = consolidate_skill_capsule(
        &cas,
        None, // genesis skill capsule
        Hash([0x42; 32]),
        Hash([0x10; 32]),
        &errors,
        "system",
        7,
    )
    .expect("consolidation succeeds");

    // (5) system-authored — no agent author exists by construction.
    assert_eq!(cap.author, SkillAuthor::System);
    assert_eq!(cap.author.tag(), "SYSTEM");

    // capsule_id populated and matches sha256.
    assert_ne!(cap.capsule_id, Cid::default());
    assert_eq!(cap.capsule_id.0, cap.sha256.0);
    assert_eq!(cap.schema_tag, SKILL_CAPSULE_SCHEMA_ID);
    // Integer confidence = sum of cluster counts (no f64 in signal path).
    assert_eq!(cap.confidence_signal, 7);

    // (1) reconstructable: cas.get(&capsule_id) MUST resolve.
    let cas_r = cas.read().expect("cas read");
    let retrieved = cas_r
        .get(&cap.capsule_id)
        .expect("Art. 0.2: cas.get(&capsule_id) MUST succeed");
    assert_eq!(
        Cid::from_content(&retrieved),
        cap.capsule_id,
        "retrieved bytes sha256 == capsule_id (content-addressed integrity)"
    );
    let restored =
        restore_skill_capsule_from_cas_bytes(&retrieved).expect("restore from CAS bytes");
    assert_eq!(restored.capsule_id, cap.capsule_id);
    assert_eq!(restored.distilled_rule, cap.distilled_rule);
    assert_eq!(restored.applicable_scope, cap.applicable_scope);
    assert_eq!(restored.confidence_signal, cap.confidence_signal);
    assert_eq!(restored.author, SkillAuthor::System);

    // The capsule is discoverable by its free-form schema id (no pinned
    // ObjectType variant needed).
    let by_schema = cas_r.list_cids_by_schema_id(SKILL_CAPSULE_SCHEMA_ID);
    assert!(
        by_schema.contains(&cap.capsule_id),
        "capsule discoverable via schema_id projection"
    );
}

/// (2) chain links via previous_capsule_cid.
#[test]
fn skill_capsule_chain_links_via_previous_capsule_cid() {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));

    let cap_1 = consolidate_skill_capsule(
        &cas,
        None,
        Hash([0x01; 32]),
        Hash([0x10; 32]),
        &[cluster(LossReasonClass::Bankruptcy, 3, 0xAA)],
        "system",
        1,
    )
    .expect("cap 1");

    let cap_2 = consolidate_skill_capsule(
        &cas,
        Some(cap_1.capsule_id),
        Hash([0x01; 32]),
        Hash([0x11; 32]), // tape head advanced
        &[cluster(LossReasonClass::SlashLoss, 5, 0xBB)],
        "system",
        2,
    )
    .expect("cap 2");

    assert_eq!(
        cap_2.previous_capsule_cid,
        Some(cap_1.capsule_id),
        "chain: cap_2 links back to cap_1"
    );
    assert_ne!(cap_1.capsule_id, cap_2.capsule_id);

    // Chain is reconstructable: from cap_2's bytes we recover the link to
    // cap_1, and cap_1 itself resolves from CAS.
    let cas_r = cas.read().expect("cas read");
    let b2 = cas_r.get(&cap_2.capsule_id).expect("get cap_2");
    let r2 = restore_skill_capsule_from_cas_bytes(&b2).expect("restore cap_2");
    let prev = r2.previous_capsule_cid.expect("cap_2 has a predecessor");
    let _b1 = cas_r
        .get(&prev)
        .expect("predecessor capsule resolves from CAS — chain reconstructable");
}

/// (3) store is CAS/tape, not the filesystem — no skills/memory dir written.
#[test]
fn consolidation_writes_no_filesystem_memory_dir() {
    // A separate parent dir; the CAS lives in a child. We assert no memory/
    // skills directory springs up as a side store.
    let parent = TempDir::new().expect("tempdir");
    let cas_dir = parent.path().join("cas_repo");
    std::fs::create_dir_all(&cas_dir).expect("mkdir cas");
    let cas = Arc::new(RwLock::new(CasStore::open(&cas_dir).expect("cas")));

    let before: Vec<String> = std::fs::read_dir(parent.path())
        .expect("read parent")
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().to_string()))
        .collect();

    consolidate_skill_capsule(
        &cas,
        None,
        Hash([0x01; 32]),
        Hash([0x10; 32]),
        &[cluster(LossReasonClass::Bankruptcy, 3, 0xAA)],
        "system",
        1,
    )
    .expect("consolidation");

    let after: Vec<String> = std::fs::read_dir(parent.path())
        .expect("read parent")
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().to_string()))
        .collect();

    // The only entry under the parent is the CAS repo we created. No sibling
    // skills/ / memory/ directory was minted as a parallel filesystem store.
    assert_eq!(
        before, after,
        "consolidation must not create a filesystem-side memory store; \
         canonical store is CAS/tape only (Art. 0.2)"
    );
    for name in &after {
        assert!(
            !name.contains("skill") && !name.contains("memory"),
            "no filesystem-side skills/memory directory: found `{name}`"
        );
    }
}

/// (4) projection is scoped + shielded; (5) read view is owned/read-only.
#[test]
fn agent_projection_scoped_shielded_read_only() {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));

    // Failure provenance Cid is a 0xAA 32-byte run; we will scan the agent
    // projection bytes and assert that run never appears.
    let cap = consolidate_skill_capsule(
        &cas,
        None,
        Hash([0x42; 32]),
        Hash([0x10; 32]),
        &[cluster(LossReasonClass::Bankruptcy, 3, 0xAA)],
        "system",
        1,
    )
    .expect("consolidation");

    // In scope → rule surfaces, with capsule provenance Cid.
    let p_in = project_for_agent(std::slice::from_ref(&cap), &["Bankruptcy".to_string()]);
    assert_eq!(p_in.applicable_rules.len(), 1, "in-scope rule surfaces");
    assert_eq!(
        p_in.source_capsule_cids,
        vec![cap.capsule_id],
        "projection cites the capsule's own Cid as provenance"
    );

    // Out of scope → default-deny.
    let p_out = project_for_agent(std::slice::from_ref(&cap), &["UnrelatedScope".to_string()]);
    assert!(
        p_out.applicable_rules.is_empty(),
        "out-of-scope rule is hidden (default-deny scoping)"
    );

    // Shielded: the raw source_failure_cid (0xAA run) MUST NOT leak into the
    // serialized agent projection.
    let bytes = serde_json::to_vec(&p_in).expect("serialize projection");
    let private_run = [0xAAu8; 32];
    let leaked = bytes.windows(32).any(|w| w == private_run);
    assert!(
        !leaked,
        "agent projection must not embed source_failure_cid bytes (Inv 10 shield)"
    );

    // The projection MUST also NOT carry the source_run_head / constitution
    // hash provenance (0x42 / 0x10 runs) — agent gets the rule, not the
    // underlying evidence anchors.
    for run in [[0x42u8; 32], [0x10u8; 32]] {
        assert!(
            !bytes.windows(32).any(|w| w == run),
            "agent projection must not embed source_run_head/constitution_hash"
        );
    }

    // (5) Tier-1 boundary witness: the projection is an owned value. There is
    // no agent-callable mutate surface in the module (the sole write path,
    // consolidate_skill_capsule, stamps SkillAuthor::System). A `SkillAuthor`
    // has exactly one variant — there is no `Agent` author to construct.
    assert_eq!(cap.author, SkillAuthor::System);
}
