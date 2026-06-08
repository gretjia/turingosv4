//! S5 interop surface — tape-anchored AgentCard / CapabilityDescriptor +
//! inbound A2A-message capsules.
//!
//! ## Why this module exists
//!
//! Before S5, external-agent interop existed only as:
//!   * an HTML-only `AgentCard` read-view block (`src/web/ir.rs` — a derived
//!     view, never authoritative over ChainTape/CAS), and
//!   * a bare `"task-a2a"` string literal in `src/runtime/adapter.rs`.
//!
//! Neither is on the canonical tape, so an external-agent/tool interaction was
//! NOT auditable as a TuringOS run (AGENTS.md §1: "if meaningful activity is
//! not on tape, it is not a TuringOS run"). This module makes the two interop
//! primitives REAL evidence objects:
//!
//!   1. [`AgentCardCapsule`] — a capability descriptor for an (external or
//!      local) agent: its declared tool/resource/action tags + the constitution
//!      version it was admitted under. CAS-resident, self-addressed by `card_id`.
//!   2. [`A2aMessageCapsule`] — one inbound agent-to-agent message: who it came
//!      from (`from_agent_card_cid`), its intent, a CAS Cid for the payload
//!      bytes, the `received_logical_t`, and — critically — a TAINT LABEL.
//!
//! ## Ingress shield (CRITICAL, non-negotiable)
//!
//! An inbound external message is UNTRUSTED. Its content is labelled
//! [`ArgTaint::UntrustedExternal`](crate::predicate_admission::arg_taint::ArgTaint)
//! — the most-tainted lattice label — and stored as the capsule's
//! [`A2aMessageCapsule::taint_tag`]. The typed accessor
//! [`A2aMessageCapsule::taint`] reconstructs the [`ArgTaint`] label via
//! `from_tag` (which fails CLOSED to `UntrustedExternal` on any unrecognised
//! tag). This is the same label the existing `arg_taint_v1` hard-gate consumes:
//! if inbound message content is ever fed as a wtool argument into a privileged
//! sink, it flows through `decide_admission_with_taint` and is REJECTED as a
//! confused-deputy hazard. We never trust raw inbound content.
//!
//! ## Tape canonical (Art. 0.2)
//!
//! The canonical store is the TAPE/CAS, never the filesystem. This module never
//! calls `std::fs::write`; the only write entry points
//! ([`write_agent_card_capsule`], [`write_a2a_message_capsule`]) persist ONLY
//! through `CasStore::put`, whose commit-chain (`refs/chaintape/cas`) is the
//! canonical evidence ledger (the L4 anchor). A capsule is fully
//! reconstructable from CAS + tape alone
//! (`restore_*_from_cas_bytes` over `cas.get(&cid)`).
//!
//! ## Schema without a pinned edit (least-pinned discipline)
//!
//! `cas/schema.rs` (the `ObjectType` enum) is a trust-root-pinned file. Rather
//! than add new enum variants there, an interop capsule is stored as
//! `ObjectType::Generic` with a free-form `schema_id`
//! ([`AGENT_CARD_CAPSULE_SCHEMA_ID`] / [`A2A_MESSAGE_CAPSULE_SCHEMA_ID`]).
//! `schema_id` participates in `CasObjectMetadata::canonical_hash`, so it is
//! collision-distinct from other `Generic` blobs and audit-discoverable via
//! `CasStore::list_cids_by_schema_id`. This module is itself nested as a
//! `#[path]` submodule of the UNPINNED `src/runtime/markov_capsule.rs`
//! (genesis pin-count 0), so it adds ZERO trust-root-pinned-file edits.
//!
//! TRACE_MATRIX FC3-N28 (meta-architecture: external tools/logs feed the
//! constitution loop — interop interactions become tape evidence, not opaque
//! side channels) + Art. 0.2 (Tape Canonical: capsule canonical bytes ARE the
//! CAS object referenced by the capsule Cid; CAS commit-chain is the L4 anchor)
//! + Inv 10 (ingress shield: inbound external content is taint-labelled
//! `UntrustedExternal`, never trusted raw).

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::CasStore;
use crate::bottom_white::ledger::transition_ledger::{
    canonical_decode, canonical_encode, CanonicalCodecError,
};
use crate::predicate_admission::arg_taint::ArgTaint;
use crate::state::q_state::Hash;

/// TRACE_MATRIX Art. 0.2: free-form CAS `schema_id` for an [`AgentCardCapsule`].
/// Used in lieu of a (pinned) `ObjectType` enum variant; it is part of
/// `CasObjectMetadata::canonical_hash`, so it is collision-distinct from other
/// `ObjectType::Generic` blobs and discoverable via
/// `CasStore::list_cids_by_schema_id(AGENT_CARD_CAPSULE_SCHEMA_ID)`.
pub const AGENT_CARD_CAPSULE_SCHEMA_ID: &str = "v1/agent_card_capsule";

/// TRACE_MATRIX Art. 0.2: free-form CAS `schema_id` for an
/// [`A2aMessageCapsule`]. Distinct from the agent-card schema id so the two
/// interop object families never collide in a `list_cids_by_schema_id` scan.
pub const A2A_MESSAGE_CAPSULE_SCHEMA_ID: &str = "v1/a2a_message_capsule";

/// TRACE_MATRIX FC3-N28 + Art. 0.2: a tape-anchored capability descriptor for
/// one agent (external or local). This is the REAL evidence object replacing
/// the HTML-only `AgentCard` read-view block at `src/web/ir.rs`.
///
/// CAS-resident and self-addressed: `card_id` is the Cid of the capsule's
/// canonical bytes (with `card_id` zeroed during the hash, per the markov R3
/// discipline), so `cas.get(&card_id)` resolves and
/// `Cid::from_content(stored_bytes) == card_id`.
///
/// **Provenance**: `constitution_hash` binds the descriptor to the constitution
/// version the agent was admitted under — an audit can prove which axiom set a
/// declared capability set was valid against.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentCardCapsule {
    /// CAS Cid of this capsule's canonical bytes (self-address). Computed by
    /// the writer with this field zeroed during the hash.
    /// TRACE_MATRIX Art. 0.2: self-addressing CAS Cid (capsule canonical bytes).
    pub card_id: Cid,

    /// Stable identifier of the agent this card describes (e.g. an A2A agent
    /// handle / public-key fingerprint string). Provenance, not authority.
    /// TRACE_MATRIX FC3-N28: agent identity this capability descriptor binds.
    pub agent_id: String,

    /// Declared capability tags — the tool / resource / action labels this
    /// agent advertises (e.g. `"tool:lean_verify"`, `"resource:cas_read"`,
    /// `"action:submit_proof"`). Declarative only: a declared capability is NOT
    /// a granted authority; admission/permission decisions are made elsewhere.
    /// TRACE_MATRIX FC3-N28: declared tool/resource/action capability tags.
    pub declared_capabilities: Vec<String>,

    /// SHA-256 of `constitution.md` bytes at the time this card was recorded.
    /// Binds the declared capability set to the constitution version it was
    /// admitted under (mirrors `MarkovEvidenceCapsule.constitution_hash`).
    /// TRACE_MATRIX Art. 0.2: constitution-version binding for the descriptor.
    pub constitution_hash: Hash,

    /// Logical time at which this card was recorded (sequencer- or
    /// generator-supplied).
    /// TRACE_MATRIX FC3-N28: logical timestamp at descriptor recording.
    pub recorded_at_logical_t: u64,

    /// Schema tag — `AGENT_CARD_CAPSULE_SCHEMA_ID`. Strictly informational (the
    /// authoritative discriminator is the CAS metadata `schema_id`); duplicated
    /// in-struct for self-describing replay.
    /// TRACE_MATRIX Art. 0.2: in-struct schema tag for self-describing replay.
    pub schema_tag: String,
}

impl Default for AgentCardCapsule {
    fn default() -> Self {
        Self {
            card_id: Cid::default(),
            agent_id: String::new(),
            declared_capabilities: Vec::new(),
            constitution_hash: Hash::ZERO,
            recorded_at_logical_t: 0,
            schema_tag: AGENT_CARD_CAPSULE_SCHEMA_ID.to_string(),
        }
    }
}

/// TRACE_MATRIX FC3-N28 + Inv 10: a tape-anchored record of one INBOUND
/// agent-to-agent message. This is the REAL evidence object replacing the bare
/// `"task-a2a"` string literal in `src/runtime/adapter.rs`.
///
/// **Ingress shield (Inv 10)**: an inbound external message is UNTRUSTED. Its
/// content provenance is recorded as [`taint_tag`](Self::taint_tag) =
/// `ArgTaint::UntrustedExternal.tag()` (`"untrusted_external"`). The typed
/// accessor [`A2aMessageCapsule::taint`] reconstructs the [`ArgTaint`] label
/// via `from_tag`, which fails CLOSED to `UntrustedExternal` on any
/// unrecognised tag. This is the same label the `arg_taint_v1` admission
/// hard-gate consumes; raw inbound content is never trusted at a privileged
/// sink.
///
/// CAS-resident and self-addressed by `message_id`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct A2aMessageCapsule {
    /// CAS Cid of this capsule's canonical bytes (self-address). Computed by
    /// the writer with this field zeroed during the hash.
    /// TRACE_MATRIX Art. 0.2: self-addressing CAS Cid (message capsule bytes).
    pub message_id: Cid,

    /// Cid of the sending agent's [`AgentCardCapsule`]. Binds the message to a
    /// declared, tape-anchored capability descriptor — an audit can resolve who
    /// the message claims to be from and what that agent advertised.
    /// TRACE_MATRIX FC3-N28: sender capability-descriptor Cid (provenance link).
    pub from_agent_card_cid: Cid,

    /// The message intent tag (e.g. `"submit_proof"`, `"request_task"`). A
    /// low-information routing label; the substantive content lives in the CAS
    /// payload referenced by `payload_cid`.
    /// TRACE_MATRIX FC3-N28: inbound message intent routing tag.
    pub intent: String,

    /// CAS Cid of the message payload bytes. The payload is stored as a
    /// separate CAS object (so this capsule stays a compact L4-anchorable
    /// header) and is itself UNTRUSTED content — see `taint_tag`.
    /// TRACE_MATRIX Art. 0.2: CAS Cid of the (untrusted) payload bytes.
    pub payload_cid: Cid,

    /// Provenance taint label for the inbound content, as a stable string tag.
    /// ALWAYS `ArgTaint::UntrustedExternal.tag()` (`"untrusted_external"`) for
    /// an inbound external message. Stored as a tag (not a serde-derived enum)
    /// because `ArgTaint` deliberately carries no `Serialize` derive; the typed
    /// accessor [`taint`](Self::taint) reconstructs the label fail-closed.
    /// TRACE_MATRIX Inv 10: ingress-shield taint label for untrusted content.
    pub taint_tag: String,

    /// Logical time at which this message was received.
    /// TRACE_MATRIX FC3-N28: logical timestamp at message receipt.
    pub received_logical_t: u64,

    /// Schema tag — `A2A_MESSAGE_CAPSULE_SCHEMA_ID`. Strictly informational;
    /// duplicated in-struct for self-describing replay.
    /// TRACE_MATRIX Art. 0.2: in-struct schema tag for self-describing replay.
    pub schema_tag: String,
}

impl Default for A2aMessageCapsule {
    fn default() -> Self {
        Self {
            message_id: Cid::default(),
            from_agent_card_cid: Cid::default(),
            intent: String::new(),
            payload_cid: Cid::default(),
            // Fail-closed default: an un-set provenance is maximally tainted.
            taint_tag: ArgTaint::UntrustedExternal.tag().to_string(),
            received_logical_t: 0,
            schema_tag: A2A_MESSAGE_CAPSULE_SCHEMA_ID.to_string(),
        }
    }
}

impl A2aMessageCapsule {
    /// TRACE_MATRIX Inv 10: reconstruct the typed [`ArgTaint`] provenance label
    /// from the stored `taint_tag`. Uses `ArgTaint::from_tag`, which fails
    /// CLOSED to `UntrustedExternal` on any unrecognised tag — an inbound
    /// message with a corrupted/unknown taint tag is treated as maximally
    /// dangerous, never silently trusted.
    pub fn taint(&self) -> ArgTaint {
        ArgTaint::from_tag(&self.taint_tag)
    }
}

/// TRACE_MATRIX FC3-N28 + Art. 0.2: interop-capsule write error taxonomy
/// (mirrors `MarkovGenError` / `SkillConsolidationError`).
#[derive(Debug)]
pub enum InteropCapsuleError {
    /// CAS write/read failed.
    Cas(crate::bottom_white::cas::store::CasError),
    /// Canonical encode/decode failed.
    Codec(String),
    /// The CAS lock was poisoned by a panicking peer.
    InternalLockPoisoned,
}

impl std::fmt::Display for InteropCapsuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cas(e) => write!(f, "cas: {e}"),
            Self::Codec(s) => write!(f, "codec: {s}"),
            Self::InternalLockPoisoned => write!(f, "internal lock poisoned"),
        }
    }
}
impl std::error::Error for InteropCapsuleError {}

impl From<crate::bottom_white::cas::store::CasError> for InteropCapsuleError {
    fn from(e: crate::bottom_white::cas::store::CasError) -> Self {
        Self::Cas(e)
    }
}

impl From<CanonicalCodecError> for InteropCapsuleError {
    fn from(e: CanonicalCodecError) -> Self {
        Self::Codec(format!("{e:?}"))
    }
}

/// TRACE_MATRIX FC3-N28 + Art. 0.2: write an [`AgentCardCapsule`] to CAS and
/// (via the CAS commit-chain `refs/chaintape/cas`) anchor it on the tape.
///
/// **R3 self-addressing discipline** (cloned from `write_markov_capsule` /
/// `consolidate_skill_capsule`): the CAS-stored bytes have `card_id` ZEROED, so
/// `Cid::from_content(stored_bytes) == card_id` and `cas.get(&card_id)`
/// resolves. The returned in-memory struct has `card_id` populated.
///
/// **Tape canonical (Art. 0.2)**: persistence is ONLY via `CasStore::put` — no
/// `std::fs::write`. `put` advances `refs/chaintape/cas` fail-closed; if the
/// chain update fails, the capsule is not accepted.
pub fn write_agent_card_capsule(
    cas: &std::sync::Arc<std::sync::RwLock<CasStore>>,
    agent_id: &str,
    declared_capabilities: &[String],
    constitution_hash: Hash,
    creator_str: &str,
    recorded_at_logical_t: u64,
) -> Result<AgentCardCapsule, InteropCapsuleError> {
    let mut card = AgentCardCapsule {
        card_id: Cid::default(),
        agent_id: agent_id.to_string(),
        declared_capabilities: declared_capabilities.to_vec(),
        constitution_hash,
        recorded_at_logical_t,
        schema_tag: AGENT_CARD_CAPSULE_SCHEMA_ID.to_string(),
    };

    let mut cas_w = cas
        .write()
        .map_err(|_| InteropCapsuleError::InternalLockPoisoned)?;

    // R3 closure: encode with card_id zeroed; the stored bytes' Cid IS the
    // card_id, guaranteeing cas.get(&card_id) resolvability.
    let stored_bytes = canonical_encode(&card)?;
    let cid = Cid::from_content(&stored_bytes);
    let cas_returned_cid = cas_w.put(
        &stored_bytes,
        ObjectType::Generic,
        creator_str,
        recorded_at_logical_t,
        Some(AGENT_CARD_CAPSULE_SCHEMA_ID.to_string()),
    )?;
    debug_assert_eq!(
        cas_returned_cid, cid,
        "CAS-returned cid must equal sha256(stored_bytes); CasStore::put contract"
    );
    card.card_id = cid;

    Ok(card)
}

/// TRACE_MATRIX FC3-N28 + Inv 10 + Art. 0.2: record one INBOUND A2A message on
/// CAS + tape. The `payload` bytes are stored as a separate UNTRUSTED CAS
/// object first; the returned capsule references it by `payload_cid` and
/// stamps `taint_tag = ArgTaint::UntrustedExternal.tag()`.
///
/// **Ingress shield**: the taint label is hard-coded to `UntrustedExternal` —
/// this entry point is for INBOUND external messages, which are untrusted by
/// definition. There is no parameter to lower the taint; an inbound message
/// cannot be admitted as `Trusted` through this path.
///
/// **R3 self-addressing discipline**: the capsule bytes have `message_id`
/// ZEROED during the hash, so `cas.get(&message_id)` resolves.
pub fn write_a2a_message_capsule(
    cas: &std::sync::Arc<std::sync::RwLock<CasStore>>,
    from_agent_card_cid: Cid,
    intent: &str,
    payload: &[u8],
    creator_str: &str,
    received_logical_t: u64,
) -> Result<A2aMessageCapsule, InteropCapsuleError> {
    let mut cas_w = cas
        .write()
        .map_err(|_| InteropCapsuleError::InternalLockPoisoned)?;

    // Store the untrusted payload bytes as a separate CAS object so the message
    // capsule stays a compact, L4-anchorable header. The payload is UNTRUSTED
    // external content; the capsule's taint_tag labels it as such.
    let payload_cid = cas_w.put(
        payload,
        ObjectType::Generic,
        creator_str,
        received_logical_t,
        Some(A2A_MESSAGE_CAPSULE_SCHEMA_ID.to_string()),
    )?;

    let mut msg = A2aMessageCapsule {
        message_id: Cid::default(),
        from_agent_card_cid,
        intent: intent.to_string(),
        payload_cid,
        // Ingress shield: inbound external content is maximally tainted.
        taint_tag: ArgTaint::UntrustedExternal.tag().to_string(),
        received_logical_t,
        schema_tag: A2A_MESSAGE_CAPSULE_SCHEMA_ID.to_string(),
    };

    let stored_bytes = canonical_encode(&msg)?;
    let cid = Cid::from_content(&stored_bytes);
    let cas_returned_cid = cas_w.put(
        &stored_bytes,
        ObjectType::Generic,
        creator_str,
        received_logical_t,
        Some(A2A_MESSAGE_CAPSULE_SCHEMA_ID.to_string()),
    )?;
    debug_assert_eq!(
        cas_returned_cid, cid,
        "CAS-returned cid must equal sha256(stored_bytes); CasStore::put contract"
    );
    msg.message_id = cid;

    Ok(msg)
}

/// TRACE_MATRIX Art. 0.2 (reconstructability): rebuild an [`AgentCardCapsule`]
/// from CAS-resident bytes. Caller supplies the bytes returned by
/// `cas.get(&card_id)`; this canonical-decodes them and re-derives `card_id`
/// from `Cid::from_content(&bytes)`.
///
/// Invariant: for any capsule written by [`write_agent_card_capsule`],
/// `restore_agent_card_from_cas_bytes(cas.get(&card.card_id)?) == card`.
pub fn restore_agent_card_from_cas_bytes(
    bytes: &[u8],
) -> Result<AgentCardCapsule, InteropCapsuleError> {
    let mut card: AgentCardCapsule = canonical_decode(bytes)?;
    card.card_id = Cid::from_content(bytes);
    Ok(card)
}

/// TRACE_MATRIX Art. 0.2 (reconstructability) + Inv 10: rebuild an
/// [`A2aMessageCapsule`] from CAS-resident bytes and re-derive `message_id`.
/// The reconstructed capsule's [`taint`](A2aMessageCapsule::taint) is recovered
/// from the stored tag (fail-closed via `ArgTaint::from_tag`), so the ingress
/// shield survives a round-trip.
///
/// Invariant: for any capsule written by [`write_a2a_message_capsule`],
/// `restore_a2a_message_from_cas_bytes(cas.get(&msg.message_id)?) == msg`.
pub fn restore_a2a_message_from_cas_bytes(
    bytes: &[u8],
) -> Result<A2aMessageCapsule, InteropCapsuleError> {
    let mut msg: A2aMessageCapsule = canonical_decode(bytes)?;
    msg.message_id = Cid::from_content(bytes);
    Ok(msg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, RwLock};
    use tempfile::TempDir;

    fn open_cas() -> (TempDir, Arc<RwLock<CasStore>>) {
        let tmp = TempDir::new().expect("tempdir");
        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
        (tmp, cas)
    }

    #[test]
    fn agent_card_default_round_trips_canonical_bytes() {
        let c = AgentCardCapsule::default();
        let bytes = canonical_encode(&c).expect("encode");
        let back: AgentCardCapsule = canonical_decode(&bytes).expect("decode");
        assert_eq!(c, back);
    }

    #[test]
    fn a2a_message_default_is_untrusted_external() {
        // The fail-closed default provenance is the most-tainted label.
        let m = A2aMessageCapsule::default();
        assert_eq!(m.taint(), ArgTaint::UntrustedExternal);
        assert_eq!(m.taint_tag, "untrusted_external");
    }

    #[test]
    fn agent_card_cas_round_trip_self_addressed() {
        let (_tmp, cas) = open_cas();
        let card = write_agent_card_capsule(
            &cas,
            "agent:external-7",
            &["tool:lean_verify".into(), "action:submit_proof".into()],
            Hash([0x33; 32]),
            "system",
            5,
        )
        .expect("write card");

        assert_ne!(card.card_id, Cid::default());
        let cas_r = cas.read().expect("read");
        let bytes = cas_r.get(&card.card_id).expect("cas.get resolves");
        assert_eq!(Cid::from_content(&bytes), card.card_id);
        let restored = restore_agent_card_from_cas_bytes(&bytes).expect("restore");
        assert_eq!(restored, card);
        // Discoverable by free-form schema id (no pinned ObjectType variant).
        assert!(cas_r
            .list_cids_by_schema_id(AGENT_CARD_CAPSULE_SCHEMA_ID)
            .contains(&card.card_id));
    }

    #[test]
    fn a2a_message_cas_round_trip_taint_survives() {
        let (_tmp, cas) = open_cas();
        let card = write_agent_card_capsule(
            &cas,
            "agent:external-7",
            &["tool:lean_verify".into()],
            Hash([0x33; 32]),
            "system",
            1,
        )
        .expect("write card");

        let msg = write_a2a_message_capsule(
            &cas,
            card.card_id,
            "submit_proof",
            b"<<untrusted inbound payload>>",
            "system",
            2,
        )
        .expect("write message");

        // Ingress shield: inbound content is taint-labelled UntrustedExternal.
        assert_eq!(msg.taint(), ArgTaint::UntrustedExternal);
        assert_eq!(msg.from_agent_card_cid, card.card_id);

        let cas_r = cas.read().expect("read");
        // Capsule reconstructable; taint survives round-trip.
        let mbytes = cas_r.get(&msg.message_id).expect("cas.get message");
        let restored = restore_a2a_message_from_cas_bytes(&mbytes).expect("restore");
        assert_eq!(restored, msg);
        assert_eq!(restored.taint(), ArgTaint::UntrustedExternal);
        // Payload bytes reconstructable from the referenced Cid.
        let pbytes = cas_r.get(&msg.payload_cid).expect("cas.get payload");
        assert_eq!(pbytes, b"<<untrusted inbound payload>>");
    }
}
