//! LIVE-FC1 Phase 6 — brand-GENERIC provider identity on the canonical CAS.
//!
//! **Why this exists** (user directive 2026-06-08, BRAND-GENERIC binding):
//! a heterogeneous swarm run must be able to PROVE `>= 2` distinct providers
//! from the tape — WITHOUT any LLM brand name (`deepseek`/`Qwen`/`SiliconFlow`/
//! …) or model-specific detail ever touching the canonical tape/CAS. The
//! brand-laden [`ModelAssignmentManifest`](crate::runtime::genesis_report)
//! (with `model_name`/`model_family`/`model_provider`) is exactly the thing we
//! must NOT write to the canonical tape.
//!
//! **The remedy** (mirrors the explicit-id hallucination remedy in
//! [`crate::sdk::id_handle`] from #328): the provider identity on the canonical
//! tape is a GENERIC opaque sha256 HANDLE
//! `id_handle::handle("model", <external model descriptor>)`. Two DISTINCT
//! external models ⇒ two DISTINCT handles ⇒ heterogeneity is provable by
//! counting distinct handles, WITHOUT brands. The brand→handle mapping
//! (which handle = which brand model) lives ONLY in an EXTERNAL sidecar struct
//! ([`ProviderBrandSidecar`]) that is NEVER serialized into the CAS capsule.
//!
//! **Observe-only / tape-canonical discipline** (mirrors
//! [`crate::runtime::agent_scheduler::boltzmann_selection_trace`]):
//! the [`ProviderHandleCapsule`] is a read-view identity record persisted to
//! CAS via [`CasStore::put`] (`ObjectType::Generic` + a free-form `schema_id`).
//! It is reconstructable from CAS alone (Art.0.2 tape-canonical) —
//! `restore`/`read`/`cids` round-trip — never a source of truth, never a
//! sequencer-admission/L4/L4.E predicate input, and there is NO
//! `std::fs::write` (no filesystem side-store). The capsule mutates no
//! `QState`/`EconomicState` and advances no head.
//!
//! **Integer-only**: every persisted numeric field is an integer
//! (`recorded_at_logical_t: u64`). No `f64`/`f32` ever touches this path. The
//! only string fields are the opaque hex `model_handle`, the `agent_id`, and
//! the free-form `schema_tag` — none of which carries a brand.
//!
//! TRACE_MATRIX FC2-N31 (boot model-assignment derived view) + FC1-N7
//! (heterogeneity read-view): mirrors the `boltzmann_selection_trace` /
//! `agent_scheduler.rs` observe-only module-family role. This module is nested
//! as a `#[path]` submodule under the UNPINNED `agent_scheduler.rs`
//! (genesis-pinned-count 0), so the trust-root-pinned `runtime/mod.rs` stays
//! byte-identical and ZERO pinned files change.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
use crate::sdk::id_handle;

/// TRACE_MATRIX FC2-N31 + FC1-N7: free-form CAS schema id for the brand-generic
/// provider handle capsule (mirrors `BOLTZMANN_SELECTION_TRACE_SCHEMA_ID`).
/// Used both as the `cas.put` `schema_id` and for discovery via
/// [`provider_handle_capsule_cids`].
pub const PROVIDER_HANDLE_CAPSULE_SCHEMA_ID: &str = "v1/provider_handle_capsule";

/// TRACE_MATRIX FC2-N31 + FC1-N7: render-membrane domain tag for the generic
/// model handle. Scopes the sha256 handle to the "model identity" surface so
/// the same external descriptor yields a stable, surface-specific opaque token
/// (matches `id_handle::handle("model", …)` callers).
pub const MODEL_HANDLE_DOMAIN: &str = "model";

/// TRACE_MATRIX FC2-N31 + FC1-N7: per-agent BRAND-GENERIC provider identity
/// capsule. This is the ONLY provider-identity object that touches the
/// canonical CAS, and it carries NO brand name and NO model-specific detail —
/// `model_handle` is the opaque sha256 short-prefix
/// `id_handle::handle("model", <external descriptor>)`.
///
/// Self-addressing per R3: stored bytes have `capsule_id` zeroed so
/// `Cid::from_content(stored_bytes) == capsule_id` (matches the
/// `BoltzmannSelectionTrace` self-addressing discipline).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderHandleCapsule {
    /// TRACE_MATRIX FC2-N31 + FC1-N7: CAS Cid of this capsule's canonical bytes
    /// (with `capsule_id` zeroed during the hash). Computed by the writer.
    pub capsule_id: Cid,
    /// TRACE_MATRIX FC2-N31 + FC1-N7: the agent this provider handle is bound
    /// to. An opaque agent id string (e.g. an `AgentId.0`); NOT a brand.
    pub agent_id: String,
    /// TRACE_MATRIX FC2-N31 + FC1-N7: the GENERIC opaque provider identity =
    /// `id_handle::handle("model", <external model descriptor>)` — a sha256
    /// short-prefix hex. Distinct external models ⇒ distinct handles. Carries
    /// NO brand name and NO model-specific detail. This is the canonical
    /// provider identity for heterogeneity proofs.
    pub model_handle: String,
    /// TRACE_MATRIX FC2-N31 + FC1-N7: integer logical time at which the handle
    /// was recorded. Integer-only (no float).
    pub recorded_at_logical_t: u64,
    /// TRACE_MATRIX FC2-N31 + FC1-N7: free-form schema tag for discovery
    /// (`PROVIDER_HANDLE_CAPSULE_SCHEMA_ID`). Carries no brand.
    pub schema_tag: String,
}

impl ProviderHandleCapsule {
    /// TRACE_MATRIX FC2-N31 + FC1-N7: build a capsule from an `agent_id` and an
    /// EXTERNAL model descriptor by hashing the descriptor into the generic
    /// `model_handle` via [`id_handle::handle`]. The external descriptor is
    /// consumed ONLY to compute the hash — it is NOT stored on the capsule, so
    /// no brand or model-specific detail can leak onto the canonical tape.
    ///
    /// `capsule_id` is left zeroed (R3 self-addressing); the writer fills it in
    /// from `Cid::from_content` of the stored bytes.
    pub fn from_external_descriptor(
        agent_id: &str,
        external_model_descriptor: &str,
        recorded_at_logical_t: u64,
    ) -> Self {
        Self {
            capsule_id: Cid::default(),
            agent_id: agent_id.to_string(),
            // GENERIC handle: sha256 short-prefix of the external descriptor.
            // The descriptor itself never lands on the capsule.
            model_handle: id_handle::handle(MODEL_HANDLE_DOMAIN, external_model_descriptor),
            recorded_at_logical_t,
            schema_tag: PROVIDER_HANDLE_CAPSULE_SCHEMA_ID.to_string(),
        }
    }
}

/// TRACE_MATRIX FC2-N31 + FC1-N7: EXTERNAL sidecar carrying the brand→handle
/// mapping. This is the ONLY place the brand `model_name`/`model_provider`
/// appears, and it is NEVER serialized into the CAS capsule — it is returned to
/// the caller for an out-of-band sidecar (e.g. a local report or `.env` audit
/// note). Keeping it `Serialize` is deliberate: callers may persist it to their
/// OWN external store, but [`write_provider_handle_capsule`] writes ONLY the
/// brand-free [`ProviderHandleCapsule`] to CAS.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderBrandSidecar {
    /// TRACE_MATRIX FC2-N31 + FC1-N7: the agent id the handle is bound to.
    pub agent_id: String,
    /// TRACE_MATRIX FC2-N31 + FC1-N7: the GENERIC handle as it appears on the
    /// canonical capsule (the join key back to the CAS object).
    pub model_handle: String,
    /// TRACE_MATRIX FC2-N31 + FC1-N7: brand-laden model name (e.g. a provider's
    /// model string). EXTERNAL ONLY — never on the canonical tape.
    pub brand_model_name: String,
    /// TRACE_MATRIX FC2-N31 + FC1-N7: brand-laden provider name. EXTERNAL ONLY
    /// — never on the canonical tape.
    pub brand_model_provider: String,
}

/// TRACE_MATRIX FC2-N31 + FC1-N7: write the BRAND-GENERIC provider handle
/// capsule for one agent to CAS and return both the self-addressed CAS `Cid`
/// AND the EXTERNAL brand→handle sidecar.
///
/// The `external_model_descriptor` (which MAY contain brand/model detail, e.g.
/// a `.env` provider string) is hashed into the generic `model_handle` and then
/// DROPPED from the canonical path — only its sha256 handle is persisted. The
/// returned [`ProviderBrandSidecar`] carries the brand mapping for the caller's
/// OWN external store; it is NEVER written to CAS here.
///
/// Persistence is CAS-only (`ObjectType::Generic` + free-form `schema_id`),
/// self-addressed per R3 (zero-then-hash). There is NO `std::fs::write`.
/// Integer-only: `recorded_at_logical_t: u64`.
pub fn write_provider_handle_capsule(
    cas: &mut CasStore,
    agent_id: &str,
    external_model_descriptor: &str,
    brand_model_name: &str,
    brand_model_provider: &str,
    recorded_at_logical_t: u64,
) -> Result<(Cid, ProviderBrandSidecar), CasError> {
    // Build the capsule with capsule_id zeroed (R3 self-addressing). The
    // external descriptor is consumed here only to compute the generic handle;
    // it is NOT carried onto the capsule struct.
    let mut capsule = ProviderHandleCapsule::from_external_descriptor(
        agent_id,
        external_model_descriptor,
        recorded_at_logical_t,
    );

    // R3: store the bytes with capsule_id zeroed so
    // Cid::from_content(stored_bytes) == capsule_id, and cas.get(&capsule_id)
    // resolves the very bytes we stored.
    let stored_bytes = canonical_encode(&capsule).map_err(|e| {
        CasError::BackendCorruption(format!("provider handle capsule encode: {e:?}"))
    })?;
    let returned_cid = cas.put(
        &stored_bytes,
        ObjectType::Generic,
        "live-fc1-provider-handle-capsule",
        recorded_at_logical_t,
        Some(PROVIDER_HANDLE_CAPSULE_SCHEMA_ID.to_string()),
    )?;
    debug_assert_eq!(
        returned_cid,
        Cid::from_content(&stored_bytes),
        "CAS-returned cid must equal sha256(stored_bytes); CasStore::put contract"
    );
    capsule.capsule_id = returned_cid;

    // The brand mapping is returned ONLY in the external sidecar — it is never
    // serialized into the CAS object above.
    let sidecar = ProviderBrandSidecar {
        agent_id: agent_id.to_string(),
        model_handle: capsule.model_handle.clone(),
        brand_model_name: brand_model_name.to_string(),
        brand_model_provider: brand_model_provider.to_string(),
    };

    Ok((returned_cid, sidecar))
}

/// TRACE_MATRIX FC2-N31 + FC1-N7: rebuild a `ProviderHandleCapsule` from
/// CAS-resident bytes. Caller supplies the bytes returned by
/// `cas.get(&capsule_id)`. Re-derives `capsule_id` from
/// `Cid::from_content(bytes)` (R3 round-trip), returning the ergonomic in-memory
/// view identical to what the writer returned.
pub fn restore_provider_handle_capsule_from_cas_bytes(
    bytes: &[u8],
) -> Result<ProviderHandleCapsule, CasError> {
    let mut capsule: ProviderHandleCapsule = canonical_decode(bytes).map_err(|e| {
        CasError::BackendCorruption(format!("provider handle capsule decode: {e:?}"))
    })?;
    capsule.capsule_id = Cid::from_content(bytes);
    Ok(capsule)
}

/// TRACE_MATRIX FC2-N31 + FC1-N7: read + restore a capsule by Cid from CAS.
pub fn read_provider_handle_capsule_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<ProviderHandleCapsule, CasError> {
    let bytes = cas.get(cid)?;
    restore_provider_handle_capsule_from_cas_bytes(&bytes)
}

/// TRACE_MATRIX FC2-N31 + FC1-N7: discover all provider-handle-capsule Cids in
/// a CAS by schema_id (mirrors `boltzmann_selection_trace_cids`).
pub fn provider_handle_capsule_cids(cas: &CasStore) -> Vec<Cid> {
    cas.list_all_cids()
        .into_iter()
        .filter(|cid| {
            cas.metadata(cid).and_then(|meta| meta.schema_id.as_deref())
                == Some(PROVIDER_HANDLE_CAPSULE_SCHEMA_ID)
        })
        .collect()
}

/// TRACE_MATRIX FC2-N31 + FC1-N7: the SWARM HETEROGENEITY acceptance helper.
/// Reconstructs every provider-handle capsule from CAS alone (Art.0.2) and
/// returns the count of DISTINCT `model_handle`s on the tape. A run proves
/// `>= 2` distinct providers iff this returns `>= 2` — WITHOUT any brand name,
/// because only the generic sha256 handles are read.
///
/// Integer-only: returns a `usize` count. Reconstruction-bounded: a capsule
/// whose bytes cannot be decoded is skipped (it cannot inflate the distinct
/// count), so the count is a conservative tape-derived witness.
pub fn distinct_provider_handles_on_tape(cas: &CasStore) -> usize {
    let mut handles: BTreeSet<String> = BTreeSet::new();
    for cid in provider_handle_capsule_cids(cas) {
        if let Ok(capsule) = read_provider_handle_capsule_from_cas(cas, &cid) {
            handles.insert(capsule.model_handle);
        }
    }
    handles.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn open_cas() -> (TempDir, CasStore) {
        let tmp = TempDir::new().expect("tempdir");
        let cas = CasStore::open(tmp.path()).expect("cas");
        (tmp, cas)
    }

    /// The capsule struct + its serialized bytes carry NO brand name and NO
    /// external descriptor — only the opaque sha256 handle. This is the
    /// BRAND-GENERIC invariant.
    #[test]
    fn capsule_carries_no_brand_only_opaque_handle() {
        let (_tmp, mut cas) = open_cas();
        // A brand-laden external descriptor (the kind a `.env` provider string
        // would carry). It must NEVER appear on the capsule or its bytes.
        let brandy_descriptor = "deepseek-chat::SiliconFlow::Qwen2.5-72B";
        let (cid, sidecar) = write_provider_handle_capsule(
            &mut cas,
            "Agent_0",
            brandy_descriptor,
            "deepseek-chat",
            "SiliconFlow",
            7,
        )
        .unwrap();
        let capsule = read_provider_handle_capsule_from_cas(&cas, &cid).unwrap();

        // model_handle is the generic sha256 short-prefix — hex only, fixed len.
        assert_eq!(capsule.model_handle.len(), id_handle::HANDLE_PREFIX_LEN);
        assert!(capsule.model_handle.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(
            capsule.model_handle,
            id_handle::handle(MODEL_HANDLE_DOMAIN, brandy_descriptor),
            "handle must be the deterministic generic hash of the descriptor"
        );

        // The stored canonical bytes contain NO brand substring and NOT the
        // descriptor.
        let bytes = cas.get(&cid).unwrap();
        let json = serde_json::to_string(&capsule).unwrap();
        for needle in ["deepseek", "SiliconFlow", "Qwen", brandy_descriptor] {
            assert!(
                !json.to_ascii_lowercase().contains(&needle.to_ascii_lowercase()),
                "capsule JSON leaked brand/descriptor `{needle}`: {json}"
            );
            let raw = String::from_utf8_lossy(&bytes).to_ascii_lowercase();
            assert!(
                !raw.contains(&needle.to_ascii_lowercase()),
                "capsule CAS bytes leaked brand/descriptor `{needle}`"
            );
        }

        // The brand mapping is carried ONLY in the external sidecar.
        assert_eq!(sidecar.brand_model_name, "deepseek-chat");
        assert_eq!(sidecar.brand_model_provider, "SiliconFlow");
        assert_eq!(sidecar.model_handle, capsule.model_handle);
    }

    /// R3 self-addressing: capsule_id round-trips and reconstruction from CAS
    /// alone yields the identical capsule (Art.0.2 tape-canonical).
    #[test]
    fn capsule_self_addresses_and_reconstructs() {
        let (_tmp, mut cas) = open_cas();
        let (cid, _sidecar) =
            write_provider_handle_capsule(&mut cas, "Agent_1", "model-A", "A", "ProvA", 3).unwrap();
        let capsule = read_provider_handle_capsule_from_cas(&cas, &cid).unwrap();
        assert_eq!(capsule.capsule_id, cid, "R3: capsule_id self-addresses bytes");
        assert_eq!(capsule.agent_id, "Agent_1");
        assert_eq!(capsule.recorded_at_logical_t, 3);
    }

    /// Two DISTINCT external models ⇒ two DISTINCT handles ⇒ the swarm
    /// heterogeneity helper proves `>= 2` providers WITHOUT any brand.
    #[test]
    fn two_distinct_models_prove_two_providers_no_brand() {
        let (_tmp, mut cas) = open_cas();
        write_provider_handle_capsule(
            &mut cas,
            "Agent_0",
            "external-descriptor-model-X",
            "brand-X",
            "prov-X",
            1,
        )
        .unwrap();
        write_provider_handle_capsule(
            &mut cas,
            "Agent_1",
            "external-descriptor-model-Y",
            "brand-Y",
            "prov-Y",
            2,
        )
        .unwrap();
        assert_eq!(
            distinct_provider_handles_on_tape(&cas),
            2,
            "two distinct external models ⇒ two distinct handles on tape"
        );
    }

    /// Same external descriptor ⇒ same handle ⇒ NOT counted as a second
    /// provider (deterministic, replay-stable; collapses to one).
    #[test]
    fn same_descriptor_collapses_to_one_handle() {
        let (_tmp, mut cas) = open_cas();
        write_provider_handle_capsule(&mut cas, "Agent_0", "same-model", "b", "p", 1).unwrap();
        write_provider_handle_capsule(&mut cas, "Agent_1", "same-model", "b", "p", 2).unwrap();
        assert_eq!(
            distinct_provider_handles_on_tape(&cas),
            1,
            "identical descriptor ⇒ identical handle ⇒ one distinct provider"
        );
    }

    /// Discovery: schema_id filter finds the capsules we wrote and only those.
    #[test]
    fn discovery_lists_written_capsules() {
        let (_tmp, mut cas) = open_cas();
        let (cid, _s) =
            write_provider_handle_capsule(&mut cas, "Agent_0", "m", "b", "p", 1).unwrap();
        let cids = provider_handle_capsule_cids(&cas);
        assert!(cids.contains(&cid), "discovery must find the written capsule");
        assert_eq!(cids.len(), 1);
    }

    /// Writing the capsule does NOT touch the filesystem outside CAS (no
    /// `std::fs::write` side-store): the capsule is reconstructable from the CAS
    /// `cas.get` path alone, which is the Art.0.2 tape-canonical contract.
    #[test]
    fn capsule_is_reconstructable_from_cas_alone() {
        let (_tmp, mut cas) = open_cas();
        let (cid, _s) =
            write_provider_handle_capsule(&mut cas, "Agent_0", "m", "b", "p", 5).unwrap();
        // Round-trip purely through cas.get → restore.
        let bytes = cas.get(&cid).unwrap();
        let restored = restore_provider_handle_capsule_from_cas_bytes(&bytes).unwrap();
        assert_eq!(restored.capsule_id, cid);
        assert_eq!(restored.schema_tag, PROVIDER_HANDLE_CAPSULE_SCHEMA_ID);
    }
}
