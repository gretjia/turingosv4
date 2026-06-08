//! LIVE-FC1 Phase 6 — BRAND-GENERIC provider identity + from-genesis replay gate.
//!
//! Authority: user directive 2026-06-08 (BRAND-GENERIC binding) — the canonical
//! tape/CAS MUST carry NO LLM brand name (`deepseek`/`Qwen`/`SiliconFlow`/`gpt`/
//! …) or model-specific detail; the provider identity on the tape is a GENERIC
//! opaque sha256 HANDLE = `id_handle::handle("model", <external descriptor>)`
//! (reuses `src/sdk/id_handle.rs` from #328). Two DISTINCT external models ⇒ two
//! DISTINCT handles ⇒ swarm heterogeneity (`>= 2` providers) is provable from the
//! tape ALONE, WITHOUT brands. The brand→handle mapping lives only in an EXTERNAL
//! sidecar, NEVER on the canonical tape.
//!
//! This gate proves three coupled properties, each with a paired mutation that
//! flips a dedicated assert RED (non-vacuous / falsifiable):
//!
//!   (1) TWO DISTINCT PROVIDERS — anchor `>= 2` `ProviderHandleCapsule`s with
//!       DIFFERENT generic handles (from two different external descriptors) on a
//!       canonical CAS; reconstruct them from the CAS alone (Art. 0.2) and assert
//!       `>= 2` DISTINCT handles are present. MUTATION: give both capsules the
//!       SAME external descriptor ⇒ identical handles collapse to ONE ⇒ the
//!       `>= 2`-distinct assert flips RED.
//!
//!   (2) NO BRAND ON TAPE — assert the SERIALIZED capsule bytes (and the CAS-
//!       resident bytes) contain NO brand literal (`deepseek`/`qwen`/`siliconflow`
//!       /`gpt`, case-insensitive) and NOT the external descriptor; the brand→
//!       handle mapping appears ONLY in the external sidecar. MUTATION witness: a
//!       hypothetical brand-laden capsule (serialized with the brand on it) WOULD
//!       contain the literal — so the same scan that passes on the generic capsule
//!       fails on a brand-bearing object, proving the scan actually bites.
//!
//!   (3) REPLAY IDENTICAL ROOTS — build a small VALID chaintape on disk
//!       (`build_chaintape_sequencer` → submit a synthetic `TaskOpen` → shutdown
//!       drain), then run the PINNED `verify_chaintape` from GENESIS via
//!       `replay_roots_match_genesis_at_paths` and assert the reconstructed
//!       `state_root`/`ledger_root` equal the recorded roots (`true`). MUTATION:
//!       tamper the on-disk `initial_q_state.json` so the from-genesis fold starts
//!       from a DIFFERENT root ⇒ the first entry's recorded `parent_state_root` no
//!       longer matches the reconstructed `state_root` ⇒ the replay diverges ⇒ the
//!       acceptance flips to `false` (RED).
//!
//! ZERO genesis-pinned-file edits: the mechanism lives in the UNPINNED
//! `src/runtime/provider_handle_capsule.rs` + `src/runtime/replay_diff_acceptance.rs`
//! (both nested as `#[path]` submodules of the UNPINNED `src/runtime/agent_scheduler.rs`,
//! pin-count 0), reusing the PINNED-FREE `crate::sdk::id_handle` (#328) and the
//! PINNED `crate::runtime::verify::verify_chaintape` (TB-6 Atom 4). This gate file
//! touches only `tests/` + the manifest/matrix coupling — `genesis_payload.toml`
//! and `constitution.md` are NOT in the diff.
//!
//! TRACE_MATRIX FC2-N31 (boot provider-identity derived view) + FC1-N7
//! (heterogeneity read-view) + FC3-N1 / FC1-N34 (from-genesis replay verifier).

use tempfile::TempDir;

use turingosv4::bus::{BusConfig, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::runtime::adapter::make_synthetic_task_open;
use turingosv4::runtime::agent_scheduler::provider_handle_capsule::{
    distinct_provider_handles_on_tape, provider_handle_capsule_cids,
    read_provider_handle_capsule_from_cas, write_provider_handle_capsule, MODEL_HANDLE_DOMAIN,
    PROVIDER_HANDLE_CAPSULE_SCHEMA_ID,
};
use turingosv4::runtime::agent_scheduler::replay_diff_acceptance::replay_roots_match_genesis_at_paths;
use turingosv4::runtime::{build_chaintape_sequencer, RuntimeChaintapeConfig};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::sdk::id_handle;

// Brand literals that MUST NEVER appear on the canonical tape (case-insensitive).
const FORBIDDEN_BRAND_LITERALS: &[&str] = &["deepseek", "qwen", "siliconflow", "gpt"];

fn open_cas(tmp: &TempDir) -> CasStore {
    let cas_dir = tmp.path().join("cas");
    std::fs::create_dir_all(&cas_dir).expect("mkdir cas");
    CasStore::open(&cas_dir).expect("open cas")
}

// ─────────────────────────────────────────────────────────────────────────
// (1) TWO DISTINCT PROVIDERS — heterogeneity provable from the tape, no brand
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn two_distinct_providers_reconstructable_from_canonical_cas() {
    let tmp = TempDir::new().expect("tmp");
    let mut cas = open_cas(&tmp);

    // Anchor two capsules with TWO DIFFERENT external descriptors (the brand
    // detail in the descriptor is consumed only to compute the hash; it never
    // lands on the capsule). Two distinct descriptors ⇒ two distinct handles.
    let (cid_a, sidecar_a) = write_provider_handle_capsule(
        &mut cas,
        "Agent_0",
        "deepseek-chat::SiliconFlow", // external descriptor A (brand-laden)
        "deepseek-chat",
        "SiliconFlow",
        1,
    )
    .expect("write capsule A");
    let (cid_b, sidecar_b) = write_provider_handle_capsule(
        &mut cas,
        "Agent_1",
        "Qwen2.5-72B::SiliconFlow", // external descriptor B (DIFFERENT model)
        "Qwen2.5-72B",
        "SiliconFlow",
        2,
    )
    .expect("write capsule B");

    // Reconstruct BOTH capsules from the canonical CAS ALONE (Art. 0.2): no
    // sidecar, no filesystem side-store — only `cas.get` → restore.
    let cids = provider_handle_capsule_cids(&cas);
    assert!(cids.contains(&cid_a) && cids.contains(&cid_b));
    assert_eq!(cids.len(), 2, "exactly the two capsules we anchored are discoverable");

    let cap_a = read_provider_handle_capsule_from_cas(&cas, &cid_a).expect("restore A");
    let cap_b = read_provider_handle_capsule_from_cas(&cas, &cid_b).expect("restore B");

    // The two reconstructed generic handles are DISTINCT (heterogeneity witness).
    assert_ne!(
        cap_a.model_handle, cap_b.model_handle,
        "two distinct external models must yield two DISTINCT generic handles"
    );

    // The swarm acceptance: `>= 2` distinct provider handles reconstructable from
    // the tape, WITHOUT reading any brand (only the generic sha256 handles).
    assert!(
        distinct_provider_handles_on_tape(&cas) >= 2,
        "the canonical CAS proves >= 2 distinct providers from generic handles alone"
    );

    // The brand mapping is carried ONLY in the external sidecar (not on tape).
    assert_eq!(sidecar_a.model_handle, cap_a.model_handle);
    assert_eq!(sidecar_b.model_handle, cap_b.model_handle);
    assert_eq!(sidecar_a.brand_model_name, "deepseek-chat");
    assert_eq!(sidecar_b.brand_model_name, "Qwen2.5-72B");

    // ── MUTATION ── give BOTH capsules the SAME external descriptor ⇒ identical
    // handles ⇒ the distinct count collapses to ONE ⇒ the `>= 2` assert flips RED.
    let tmp2 = TempDir::new().expect("tmp2");
    let mut cas2 = open_cas(&tmp2);
    write_provider_handle_capsule(&mut cas2, "Agent_0", "same-descriptor", "b", "p", 1)
        .expect("write mutated A");
    write_provider_handle_capsule(&mut cas2, "Agent_1", "same-descriptor", "b", "p", 2)
        .expect("write mutated B");
    assert_eq!(
        distinct_provider_handles_on_tape(&cas2),
        1,
        "MUTATION: identical descriptors collapse to ONE handle — the >=2-distinct \
         heterogeneity proof would be RED (1 < 2)"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// (2) NO BRAND ON TAPE — the canonical capsule bytes carry no brand literal
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn no_brand_or_descriptor_literal_on_canonical_capsule_bytes() {
    let tmp = TempDir::new().expect("tmp");
    let mut cas = open_cas(&tmp);

    // A maximally brand-laden external descriptor — exactly the kind of `.env`
    // provider string we must keep OFF the canonical tape.
    let brandy_descriptor = "deepseek-chat::SiliconFlow::Qwen2.5-72B::gpt-4o";
    let (cid, sidecar) = write_provider_handle_capsule(
        &mut cas,
        "Agent_0",
        brandy_descriptor,
        "deepseek-chat",
        "SiliconFlow",
        7,
    )
    .expect("write capsule");

    let capsule = read_provider_handle_capsule_from_cas(&cas, &cid).expect("restore");

    // The on-canonical-tape provider identity is the GENERIC opaque sha256 handle.
    assert_eq!(capsule.model_handle.len(), id_handle::HANDLE_PREFIX_LEN);
    assert!(capsule.model_handle.chars().all(|c| c.is_ascii_hexdigit()));
    assert_eq!(
        capsule.model_handle,
        id_handle::handle(MODEL_HANDLE_DOMAIN, brandy_descriptor),
        "the canonical handle is the deterministic generic hash of the descriptor"
    );

    // The SERIALIZED capsule bytes (JSON view) AND the raw CAS-resident bytes
    // contain NO brand literal and NOT the external descriptor.
    let cas_bytes = cas.get(&cid).expect("cas get bytes");
    let raw_lc = String::from_utf8_lossy(&cas_bytes).to_ascii_lowercase();
    let json = serde_json::to_string(&capsule).expect("serialize capsule");
    let json_lc = json.to_ascii_lowercase();

    for brand in FORBIDDEN_BRAND_LITERALS {
        let needle = brand.to_ascii_lowercase();
        assert!(
            !json_lc.contains(&needle),
            "BRAND-GENERIC violated: capsule JSON leaked brand literal `{brand}`: {json}"
        );
        assert!(
            !raw_lc.contains(&needle),
            "BRAND-GENERIC violated: canonical CAS bytes leaked brand literal `{brand}`"
        );
    }
    // The full descriptor (its model-specific detail) is likewise absent.
    assert!(
        !json_lc.contains(&brandy_descriptor.to_ascii_lowercase()),
        "the external descriptor must not appear on the capsule"
    );
    assert!(
        !raw_lc.contains(&brandy_descriptor.to_ascii_lowercase()),
        "the external descriptor must not appear in the canonical CAS bytes"
    );

    // The brand mapping is present ONLY in the EXTERNAL sidecar (never on tape).
    let sidecar_json = serde_json::to_string(&sidecar).expect("serialize sidecar");
    assert!(
        sidecar_json.to_ascii_lowercase().contains("deepseek"),
        "the brand→handle mapping lives in the external sidecar (where the brand IS allowed)"
    );

    // ── MUTATION witness ── the no-brand scan actually BITES: the very same
    // case-insensitive scan, run over a brand-bearing serialized object, FINDS
    // the brand. (Here: the external sidecar, which legitimately carries the
    // brand.) If the scan were vacuous it would pass on this too. It does not.
    let mut scan_caught_a_brand = false;
    for brand in FORBIDDEN_BRAND_LITERALS {
        if sidecar_json
            .to_ascii_lowercase()
            .contains(&brand.to_ascii_lowercase())
        {
            scan_caught_a_brand = true;
        }
    }
    assert!(
        scan_caught_a_brand,
        "MUTATION: the no-brand scan must DETECT a brand when one is present \
         (a brand-laden capsule would flip the on-tape assert RED)"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// (3) REPLAY IDENTICAL ROOTS — from-genesis reconstruction matches recorded
// ─────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn from_genesis_replay_reconstructs_identical_roots_and_tamper_flips_red() {
    let tmp = TempDir::new().expect("tmp");
    let runtime_repo = tmp.path().join("runtime_repo");
    let cas_path = tmp.path().join("cas");
    let cfg = RuntimeChaintapeConfig {
        runtime_repo_path: runtime_repo.clone(),
        cas_path: cas_path.clone(),
        run_id: "provider-replay".to_string(),
        queue_capacity: 16,
        resume_existing_chain: false,
    };

    // Build a small VALID chaintape on disk: boot the production-mode sequencer,
    // submit a synthetic TaskOpen (≥1 L4 entry), drain on shutdown.
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    let kernel = Kernel::new();
    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());
    let boot_root = bundle
        .sequencer
        .q_snapshot()
        .expect("post-activation q")
        .state_root_t;
    let task_open = make_synthetic_task_open("task-prov", "sponsor-prov", boot_root, "prov-1");
    bus.submit_typed_tx(task_open).await.expect("submit TaskOpen");
    bundle.shutdown().await.expect("shutdown");
    drop(bus);

    // GREEN: replay FROM GENESIS via the PINNED verify_chaintape reconstructs the
    // SAME state_root/ledger_root the tape recorded.
    let accepted = replay_roots_match_genesis_at_paths(&runtime_repo, &cas_path)
        .expect("verify_chaintape ran");
    assert!(
        accepted,
        "from-genesis replay must reconstruct IDENTICAL state_root/ledger_root"
    );

    // ── MUTATION ── tamper the on-disk `initial_q_state.json` so the from-genesis
    // fold starts from a DIFFERENT root. The first L4 entry's recorded
    // `parent_state_root` then no longer matches the reconstructed `state_root_t`
    // ⇒ the replay diverges ⇒ the acceptance flips to false.
    let initial_q_path = runtime_repo.join("initial_q_state.json");
    let raw = std::fs::read_to_string(&initial_q_path).expect("read initial_q_state.json");
    let mut parsed: serde_json::Value = serde_json::from_str(&raw).expect("parse initial_q");
    // Flip the recorded state_root_t to a NON-genesis value (32 ones). state_root_t
    // serializes as a 32-element byte array.
    let tampered_root = serde_json::Value::Array(
        (0..32u32)
            .map(|_| serde_json::Value::Number(1u8.into()))
            .collect(),
    );
    let obj = parsed.as_object_mut().expect("initial_q is a JSON object");
    assert!(
        obj.contains_key("state_root_t"),
        "initial_q_state.json must carry state_root_t to tamper"
    );
    obj.insert("state_root_t".to_string(), tampered_root);
    std::fs::write(
        &initial_q_path,
        serde_json::to_string(&parsed).expect("reserialize tampered initial_q"),
    )
    .expect("write tampered initial_q");

    // After the tamper, the from-genesis fold begins from a divergent root, so the
    // recorded roots can no longer be reconstructed → acceptance is RED.
    let tampered_accepted =
        replay_roots_match_genesis_at_paths(&runtime_repo, &cas_path).unwrap_or(false);
    assert!(
        !tampered_accepted,
        "MUTATION: a tampered from-genesis initial root must make the replay diverge \
         from the recorded roots — acceptance flips to false (RED)"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// Discovery sanity — the schema_id binds the capsules to a discoverable class
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn capsules_discoverable_by_brand_free_schema_id() {
    let tmp = TempDir::new().expect("tmp");
    let mut cas = open_cas(&tmp);
    let (cid, _s) =
        write_provider_handle_capsule(&mut cas, "Agent_0", "model-Z", "z", "provZ", 4).unwrap();
    let cids = provider_handle_capsule_cids(&cas);
    assert!(cids.contains(&cid));
    // The schema id itself carries no brand.
    let schema_lc = PROVIDER_HANDLE_CAPSULE_SCHEMA_ID.to_ascii_lowercase();
    for brand in FORBIDDEN_BRAND_LITERALS {
        assert!(
            !schema_lc.contains(&brand.to_ascii_lowercase()),
            "the discovery schema_id must be brand-free"
        );
    }
}
