//! TB-15 Atom 2 — `AgentAutopsyCapsule` schema + writer (architect §6.2,
//! ruling 2026-05-02 + 2026-05-03).
//!
//! Per-agent, per-event capsule for a loss / bankruptcy / failed-market
//! event. CAS-resident; AuditOnly by default. Derived from ChainTape
//! evidence (positions, trades, prices, slippage, resolution, market
//! pool state) — NEVER from agent LLM self-narration (DECISION_LAMARCKIAN
//! §1.2 hard prohibition B).
//!
//! Anchored on `EconomicState.agent_autopsies_t[event_id]: Vec<Cid>`
//! (Atom 3). Public clustering surface (`cluster_autopsies` →
//! `Vec<TypicalErrorSummary>`) lands in Atom 4.
//!
//! Privacy contract:
//! - `public_summary`: low-info string surfaceable to broadcast IFF N≥3
//!   same-class cluster forms (CR-15.2).
//! - `private_detail_cid`: opaque CAS Cid; AuditOnly access only;
//!   NEVER enters `AgentVisibleProjection` (CR-15.1 + SG-15.2).
//! - `evidence_cids`: CAS Cids of pre-existing public ChainTape
//!   evidence (the loss tx, slash tx, ...); not new private bytes.
//!
//! TRACE_MATRIX FC1-N32 (writer) + Art. 0.2 (Tape Canonical: capsule
//! canonical bytes are themselves the CAS object referenced by
//! `capsule_id`) + Art. III.1 (raw failure shielding) + Art. III.2
//! (read-view scoping) + CR-15.3 (autopsy SUGGESTS via
//! `suggested_policy_patch: Option<Cid>`; never mutates predicates).

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::Cid;
use crate::economy::money::MicroCoin;
use crate::state::q_state::{AgentId, Hash};
use crate::state::typed_tx::{CapsulePrivacyPolicy, EventId, RiskRuleId};

/// TRACE_MATRIX TB-15 (architect §6.2 + DECISION_LAMARCKIAN §1.1) —
/// loss reason discriminator. Architect hint list = AdverseSelection /
/// Overleverage / Goodhart; runtime additions covering current TB-11..14
/// surface = SlashLoss / Bankruptcy / ChallengeUnsuccessful /
/// VerifierBondLost. `Other(String)` keeps forward extensibility without
/// per-TB enum bumps.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LossReasonClass {
    /// Solver lost stake to upheld challenge (RSP-3.2 / TB-9 forward
    /// trigger; not yet active in TB-15 v0).
    SlashLoss,
    /// Task entered bankruptcy via `TaskBankruptcyTx`. **TB-15 v0 sole
    /// production trigger** per charter §1.2.
    Bankruptcy,
    /// Challenger's NO bond slashed because challenge was dismissed.
    /// (RSP-3.2 forward trigger.)
    ChallengeUnsuccessful,
    /// Verifier's bond slashed due to incorrect verdict. (RSP-3.2
    /// forward trigger.)
    VerifierBondLost,
    /// Architect §1.1 hint — adverse selection (information asymmetry
    /// led to wrong-side position). TB-16+ scope.
    AdverseSelection,
    /// Architect §1.1 hint — over-leverage (position > Kelly cap).
    Overleverage,
    /// Architect §1.1 hint — Goodhart (chased a metric that was not the
    /// actual goal).
    Goodhart,
    /// Forward extensibility — caller-supplied class string.
    Other(String),
}

impl Default for LossReasonClass {
    fn default() -> Self {
        Self::Bankruptcy
    }
}

impl LossReasonClass {
    /// Stable string tag for clustering / dashboard rendering. Avoids
    /// `Debug`'s formatting volatility.
    ///
    /// TRACE_MATRIX FC2-N30 (TB-15 Atom 4): clustering-key surface for
    /// `cluster_autopsies` group-by; also dashboard §15 render tag
    /// (Atom 6).
    pub fn tag(&self) -> &str {
        match self {
            Self::SlashLoss => "SlashLoss",
            Self::Bankruptcy => "Bankruptcy",
            Self::ChallengeUnsuccessful => "ChallengeUnsuccessful",
            Self::VerifierBondLost => "VerifierBondLost",
            Self::AdverseSelection => "AdverseSelection",
            Self::Overleverage => "Overleverage",
            Self::Goodhart => "Goodhart",
            Self::Other(s) => s.as_str(),
        }
    }
}

/// TRACE_MATRIX TB-15 (architect §6.2 + DECISION_LAMARCKIAN §1.1) —
/// CAS-resident per-agent loss capsule. Default `privacy_policy =
/// AuditOnly` (re-uses TB-11 surface).
///
/// **Privacy** (architect §6.4):
/// - `public_summary`: low-info string; eligible for typical-error
///   broadcast only via Atom 4 `cluster_autopsies` (CR-15.2).
/// - `private_detail_cid`: opaque CAS Cid pointing at
///   `ObjectType::AutopsyPrivateDetail`; access requires audit role.
/// - `evidence_cids`: Cids of pre-existing public ChainTape objects
///   (loss tx CID, sequencer-side slash tx CID, market pool state CID).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentAutopsyCapsule {
    /// CAS Cid of the canonical-encoded `AgentAutopsyCapsule` itself.
    /// Computed by the writer (sha256 over canonical bytes with this
    /// field zeroed).
    pub capsule_id: Cid,

    /// Owner of the loss event.
    pub agent_id: AgentId,
    /// Event being autopsied (TB-13 `EventId(TaskId)`; TB-14+ may
    /// decouple per-node).
    pub event_id: EventId,

    /// Magnitude of the loss in MicroCoin.
    pub loss_amount: MicroCoin,
    /// Class discriminator (CR-15.2 clustering key).
    pub loss_reason_class: LossReasonClass,

    /// Protocol-level risk rule that the loss event violated, if any.
    /// `None` when the loss did not violate a registered rule (e.g.
    /// Bankruptcy = task ran out of escrow; not a per-agent violation).
    pub violated_risk_rule: Option<RiskRuleId>,

    /// Optional pointer to a `RiskPolicyPatch` CAS object describing a
    /// patch the autopsy *suggests*. **NEVER auto-applied** (CR-15.3 +
    /// SG-15.8); routing is ArchitectAI proposal → JudgeAI/VetoAI →
    /// canary (P5 v1 surface).
    pub suggested_policy_patch: Option<Cid>,

    /// CAS Cids of ChainTape evidence anchors (loss tx, slash tx,
    /// position state, market pool state, etc.). Pre-existing public
    /// objects only — autopsy does NOT mint new private evidence here.
    pub evidence_cids: Vec<Cid>,

    /// Low-information broadcast surface (CR-15.2). Format:
    /// `agent={agent_id} lost {amount}μC on event={event_id} reason={tag}`.
    pub public_summary: String,
    /// Opaque CAS Cid pointing at `ObjectType::AutopsyPrivateDetail`.
    /// Audit-only access. NEVER enters `AgentVisibleProjection`.
    pub private_detail_cid: Cid,

    /// Privacy default `CapsulePrivacyPolicy::AuditOnly` (architect §6.4).
    pub privacy_policy: CapsulePrivacyPolicy,

    /// SHA-256 of the canonical-encoded capsule bytes (with `capsule_id`
    /// zeroed). Defense-in-depth duplicate of `capsule_id`.
    pub sha256: Hash,

    /// Logical time at autopsy emission (sequencer-assigned).
    pub created_at_logical_t: u64,
    /// Round id at autopsy emission (sequencer-assigned).
    pub created_at_round: u64,
}

impl Default for AgentAutopsyCapsule {
    fn default() -> Self {
        Self {
            capsule_id: Cid::default(),
            agent_id: AgentId::default(),
            event_id: EventId::default(),
            loss_amount: MicroCoin::zero(),
            loss_reason_class: LossReasonClass::default(),
            violated_risk_rule: None,
            suggested_policy_patch: None,
            evidence_cids: Vec::new(),
            public_summary: String::new(),
            private_detail_cid: Cid::default(),
            privacy_policy: CapsulePrivacyPolicy::default(),
            sha256: Hash::ZERO,
            created_at_logical_t: 0,
            created_at_round: 0,
        }
    }
}

impl AgentAutopsyCapsule {
    /// TRACE_MATRIX architect §6.2 — deterministic public_summary
    /// formatter. Format (stable across runs; broadcast-eligible):
    ///
    /// `agent={agent_id} lost {amount}μC on event={event_task_id} reason={tag}`
    pub fn format_public_summary(
        agent_id: &AgentId,
        event_id: &EventId,
        loss_amount: MicroCoin,
        loss_reason_class: &LossReasonClass,
    ) -> String {
        format!(
            "agent={} lost {}μC on event={} reason={}",
            agent_id.0,
            loss_amount.micro_units(),
            (event_id.0).0,
            loss_reason_class.tag(),
        )
    }
}

// ────────────────────────────────────────────────────────────────────────────
// TB-15 Atom 2 — Writer
// ────────────────────────────────────────────────────────────────────────────

use crate::bottom_white::cas::schema::ObjectType;
use crate::bottom_white::cas::store::CasStore;
use crate::bottom_white::ledger::transition_ledger::canonical_encode;

/// TRACE_MATRIX TB-15 Atom 2 — writer error taxonomy.
#[derive(Debug)]
pub enum AutopsyWriteError {
    Cas(crate::bottom_white::cas::store::CasError),
    Encode(String),
    InternalLockPoisoned,
}

impl std::fmt::Display for AutopsyWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cas(e) => write!(f, "cas write failed: {e}"),
            Self::Encode(s) => write!(f, "encode failed: {s}"),
            Self::InternalLockPoisoned => write!(f, "internal lock poisoned"),
        }
    }
}
impl std::error::Error for AutopsyWriteError {}

impl From<crate::bottom_white::cas::store::CasError> for AutopsyWriteError {
    fn from(e: crate::bottom_white::cas::store::CasError) -> Self {
        Self::Cas(e)
    }
}

/// TRACE_MATRIX TB-15 Atom 2 (architect §6.2): write an
/// `AgentAutopsyCapsule` to CAS. Flow:
///
/// 1. Build canonical private-detail JSON from caller-supplied
///    `private_detail_payload` bytes → write to CAS as
///    `ObjectType::AutopsyPrivateDetail`. Cid is `private_detail_cid`.
/// 2. Build the capsule struct with `capsule_id = Cid::default()` +
///    `sha256 = Hash::ZERO`. Canonical-encode → sha256 → that's the
///    eventual `capsule_id`.
/// 3. Re-create the struct with `capsule_id` filled in + write to CAS
///    as `ObjectType::AgentAutopsyCapsule`.
///
/// Returns the populated `AgentAutopsyCapsule` (with `capsule_id` set).
///
/// **CR-15.3 / SG-15.8**: writer signature has NO mutable reference to
/// any predicate / tool / risk-policy registry. `suggested_policy_patch`
/// is an opaque `Option<Cid>` pointer; the writer does not interpret
/// or apply it.
#[allow(clippy::too_many_arguments)]
pub fn write_autopsy_capsule(
    cas: &std::sync::Arc<std::sync::RwLock<CasStore>>,
    agent_id: AgentId,
    event_id: EventId,
    loss_amount: MicroCoin,
    loss_reason_class: LossReasonClass,
    violated_risk_rule: Option<RiskRuleId>,
    suggested_policy_patch: Option<Cid>,
    evidence_cids: Vec<Cid>,
    private_detail_payload: &[u8],
    privacy: CapsulePrivacyPolicy,
    creator_str: &str,
    created_at_logical_t: u64,
    created_at_round: u64,
) -> Result<AgentAutopsyCapsule, AutopsyWriteError> {
    let mut cas_w = cas
        .write()
        .map_err(|_| AutopsyWriteError::InternalLockPoisoned)?;

    // Step 1: write private detail to CAS (caller-supplied opaque bytes).
    let private_detail_cid = cas_w.put(
        private_detail_payload,
        ObjectType::AutopsyPrivateDetail,
        creator_str,
        created_at_logical_t,
        Some("v1/autopsy_private_detail".into()),
    )?;

    // Step 2: build capsule with capsule_id = 0 + sha256 = 0; canonical
    // encode; sha256 of bytes is the eventual capsule_id.
    let public_summary = AgentAutopsyCapsule::format_public_summary(
        &agent_id,
        &event_id,
        loss_amount,
        &loss_reason_class,
    );
    let mut capsule = AgentAutopsyCapsule {
        capsule_id: Cid::default(),
        agent_id,
        event_id,
        loss_amount,
        loss_reason_class,
        violated_risk_rule,
        suggested_policy_patch,
        evidence_cids,
        public_summary,
        private_detail_cid,
        privacy_policy: privacy,
        sha256: Hash::ZERO,
        created_at_logical_t,
        created_at_round,
    };
    let prelim_bytes = canonical_encode(&capsule)
        .map_err(|e| AutopsyWriteError::Encode(format!("capsule prelim encode: {e:?}")))?;
    let capsule_cid = Cid::from_content(&prelim_bytes);
    capsule.capsule_id = capsule_cid;
    capsule.sha256 = Hash(capsule_cid.0);

    // Step 3: write the canonical-encoded capsule (with capsule_id +
    // sha256 filled in) to CAS as the AgentAutopsyCapsule object.
    let final_bytes = canonical_encode(&capsule)
        .map_err(|e| AutopsyWriteError::Encode(format!("capsule final encode: {e:?}")))?;
    let _ = cas_w.put(
        &final_bytes,
        ObjectType::AgentAutopsyCapsule,
        creator_str,
        created_at_logical_t,
        Some("v1/agent_autopsy_capsule".into()),
    )?;

    Ok(capsule)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::q_state::TaskId;

    /// TB-15 U1: capsule default round-trips through canonical bytes.
    #[test]
    fn autopsy_capsule_default_round_trip() {
        use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
        let c = AgentAutopsyCapsule::default();
        let bytes = canonical_encode(&c).expect("encode");
        let back: AgentAutopsyCapsule = canonical_decode(&bytes).expect("decode");
        assert_eq!(c, back);
    }

    /// TB-15 U2: format_public_summary embeds agent_id + amount + reason tag.
    #[test]
    fn format_public_summary_contains_agent_amount_reason() {
        let s = AgentAutopsyCapsule::format_public_summary(
            &AgentId("Agent_solver_3".into()),
            &EventId(TaskId("task:lean:t1".into())),
            MicroCoin::from_micro_units(1500),
            &LossReasonClass::Bankruptcy,
        );
        assert!(s.contains("Agent_solver_3"));
        assert!(s.contains("1500"));
        assert!(s.contains("task:lean:t1"));
        assert!(s.contains("Bankruptcy"));
    }

    /// TB-15 U3: privacy_policy default = AuditOnly (re-use TB-11
    /// CR-15.1 surface).
    #[test]
    fn privacy_policy_default_is_audit_only() {
        let c = AgentAutopsyCapsule::default();
        assert_eq!(c.privacy_policy, CapsulePrivacyPolicy::AuditOnly);
    }

    /// TB-15 Atom 2 — Writer: writes private_detail + capsule to CAS;
    /// returned capsule has populated capsule_id (Cid of canonical
    /// bytes) and matching sha256.
    #[test]
    fn write_autopsy_capsule_to_cas_round_trip() {
        use std::sync::{Arc, RwLock};
        use tempfile::TempDir;

        let tmp = TempDir::new().expect("tempdir");
        let cas = Arc::new(RwLock::new(
            crate::bottom_white::cas::store::CasStore::open(tmp.path()).expect("cas"),
        ));

        let private_detail = br#"{"position":[],"slippage":0,"pool_state":"empty"}"#;
        let cap = write_autopsy_capsule(
            &cas,
            AgentId("Agent_solver_0".into()),
            EventId(TaskId("task:lean:tb15:autopsy_writer".into())),
            MicroCoin::from_micro_units(2_500),
            LossReasonClass::Bankruptcy,
            None,
            None,
            vec![Cid::from_content(b"loss_tx_cid_placeholder")],
            private_detail,
            CapsulePrivacyPolicy::AuditOnly,
            "tb-15-writer",
            42,
            7,
        )
        .expect("writer succeeds");

        // Capsule_id populated and matches sha256.
        assert_ne!(cap.capsule_id, Cid::default());
        assert_eq!(cap.capsule_id.0, cap.sha256.0);

        // Private detail Cid populated.
        assert_ne!(cap.private_detail_cid, Cid::default());

        // Public summary has expected shape.
        assert!(cap.public_summary.contains("Agent_solver_0"));
        assert!(cap.public_summary.contains("2500"));
        assert!(cap.public_summary.contains("Bankruptcy"));

        // CAS contains 2 objects: private_detail + capsule.
        let cas_r = cas.read().expect("cas read");
        assert_eq!(
            cas_r.len(),
            2,
            "writer puts 2 CAS objects: private_detail + capsule"
        );

        // Private detail bytes retrievable.
        let retrieved = cas_r.get(&cap.private_detail_cid).expect("get priv");
        assert_eq!(retrieved, private_detail);
    }

    /// TB-15 Atom 2 — Writer: same inputs → same capsule_id (deterministic).
    #[test]
    fn write_autopsy_capsule_deterministic_capsule_id() {
        use std::sync::{Arc, RwLock};
        use tempfile::TempDir;

        let private_detail = b"deterministic-detail-bytes";
        let mk = || -> AgentAutopsyCapsule {
            let tmp = TempDir::new().unwrap();
            let cas = Arc::new(RwLock::new(
                crate::bottom_white::cas::store::CasStore::open(tmp.path()).unwrap(),
            ));
            write_autopsy_capsule(
                &cas,
                AgentId("Agent_X".into()),
                EventId(TaskId("task:tb15:det".into())),
                MicroCoin::from_micro_units(777),
                LossReasonClass::SlashLoss,
                Some(RiskRuleId("max_drawdown".into())),
                None,
                vec![Cid::from_content(b"ev1"), Cid::from_content(b"ev2")],
                private_detail,
                CapsulePrivacyPolicy::AuditOnly,
                "writer",
                3,
                1,
            )
            .expect("writer")
        };
        let a = mk();
        let b = mk();
        assert_eq!(a.capsule_id, b.capsule_id);
        assert_eq!(a.private_detail_cid, b.private_detail_cid);
    }

    /// TB-15 Atom 2 — LossReasonClass::tag is stable across all variants.
    #[test]
    fn loss_reason_class_tag_stable() {
        assert_eq!(LossReasonClass::SlashLoss.tag(), "SlashLoss");
        assert_eq!(LossReasonClass::Bankruptcy.tag(), "Bankruptcy");
        assert_eq!(
            LossReasonClass::ChallengeUnsuccessful.tag(),
            "ChallengeUnsuccessful"
        );
        assert_eq!(LossReasonClass::VerifierBondLost.tag(), "VerifierBondLost");
        assert_eq!(LossReasonClass::AdverseSelection.tag(), "AdverseSelection");
        assert_eq!(LossReasonClass::Overleverage.tag(), "Overleverage");
        assert_eq!(LossReasonClass::Goodhart.tag(), "Goodhart");
        assert_eq!(
            LossReasonClass::Other("CustomThing".into()).tag(),
            "CustomThing"
        );
    }
}
