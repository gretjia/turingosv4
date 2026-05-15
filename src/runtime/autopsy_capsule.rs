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
    // R3 closure (Codex R2 VETO TB15-CAS-ID): identical pattern to
    // write_markov_capsule. Store the bytes whose sha256 equals
    // capsule_id, NOT the post-population bytes (which would have a
    // different sha256, breaking cas.get(&capsule_id) resolvability).
    // The in-memory struct returned to caller has populated
    // capsule_id+sha256; on-CAS bytes have these zeroed.
    let stored_bytes = canonical_encode(&capsule)
        .map_err(|e| AutopsyWriteError::Encode(format!("capsule canonical encode: {e:?}")))?;
    let capsule_cid = Cid::from_content(&stored_bytes);
    let cas_returned_cid = cas_w.put(
        &stored_bytes,
        ObjectType::AgentAutopsyCapsule,
        creator_str,
        created_at_logical_t,
        Some("v1/agent_autopsy_capsule".into()),
    )?;
    debug_assert_eq!(
        cas_returned_cid, capsule_cid,
        "CAS-returned cid must equal sha256(stored_bytes); CasStore::put contract"
    );
    capsule.capsule_id = capsule_cid;
    capsule.sha256 = Hash(capsule_cid.0);

    Ok(capsule)
}

/// TRACE_MATRIX TB-15 R3 closure (Codex R2 VETO TB15-CAS-ID): rebuild
/// an `AgentAutopsyCapsule` from CAS-resident bytes. Symmetric helper
/// to `restore_markov_capsule_from_cas_bytes`. Caller supplies the
/// bytes returned by `cas.get(&capsule_id)`; helper canonical-decodes
/// + re-derives capsule_id/sha256 from `Cid::from_content(&bytes)`.
pub fn restore_autopsy_capsule_from_cas_bytes(
    bytes: &[u8],
) -> Result<AgentAutopsyCapsule, AutopsyWriteError> {
    use crate::bottom_white::ledger::transition_ledger::canonical_decode;
    let mut cap: AgentAutopsyCapsule = canonical_decode(bytes)
        .map_err(|e| AutopsyWriteError::Encode(format!("capsule decode: {e:?}")))?;
    let cid = Cid::from_content(bytes);
    cap.capsule_id = cid;
    cap.sha256 = Hash(cid.0);
    Ok(cap)
}

// ────────────────────────────────────────────────────────────────────────────
// TB-15 R2 closure — Activation gate (Gemini R1 VETO Q12; replay-determinism)
// ────────────────────────────────────────────────────────────────────────────
//
// Per Gemini R1 audit 2026-05-04 Q12 VETO + `feedback_no_retroactive_evidence_rewrite`:
// replaying a pre-TB-15 chain post-TB-15-deployment must NOT spuriously
// generate `AgentAutopsyCapsule` entries that did not exist in the
// original live execution.
//
// Verification baseline (2026-05-04): grep across all on-disk
// `handover/evidence/*/runtime_repo` chains found ZERO production
// `TaskBankruptcyTx` rows pre-TB-15 (TB-11 added the variant; no
// production chain has fired one). The structural concern is real but
// no chain currently triggers it.
//
// The activation gate is set at compile time. Default = 0 means
// "always active for fresh chains shipped at or after TB-15
// (commit 2337381 + onwards)" — every new chain starts at logical_t=1
// which trivially satisfies `>= 0`. For pre-TB-15 chain replay
// (no such chain exists today; future migrations would set this
// non-zero), the cutoff would be the first logical_t at which the
// post-TB-15 sequencer becomes authoritative.
//
// **Constitutional alignment** (Art.0.2 Tape Canonical):
// post-activation replay reconstructs identical agent_autopsies_t
// entries by deterministic helper. Pre-activation rows pass through
// the dispatch arm without autopsy mutation, preserving the original
// EconomicState shape.
//
// Future migration story: when a pre-TB-15 chain with TaskBankruptcyTx
// rows needs to be replayed, the operator overrides
// `TB15_AUTOPSY_ACTIVATION_LOGICAL_T` to the cutoff at deployment.
// Pre-cutoff TaskBankruptcyTx rows replay cleanly without spurious
// autopsy entries.

/// TRACE_MATRIX TB-15 R2 closure (Gemini R1 VETO Q12; activation gate
/// for replay-determinism). Default 0 = always active for fresh chains
/// (TB-15 ship commit 2337381 onwards; every new chain starts at
/// logical_t=1 ≥ 0 → trivially active). Overridable at compile time
/// for pre-TB-15 chain migration scenarios.
pub const TB15_AUTOPSY_ACTIVATION_LOGICAL_T: u64 = 0;

/// TRACE_MATRIX TB-15 R2 closure: gate predicate. Returns true iff
/// autopsy emission is enabled for a TaskBankruptcyTx with the given
/// timestamp_logical. Pure-fn over the activation constant; identical
/// in dispatch arm and apply_one Stage 3.5.
#[inline]
pub fn is_autopsy_active_at(timestamp_logical: u64) -> bool {
    timestamp_logical >= TB15_AUTOPSY_ACTIVATION_LOGICAL_T
}

// ────────────────────────────────────────────────────────────────────────────
// TB-15 Atom 4 — TypicalErrorBroadcast clustering (architect §3.2.3 + CR-15.2)
// ────────────────────────────────────────────────────────────────────────────
//
// `cluster_autopsies` groups input autopsies by `loss_reason_class`, and
// emits a `TypicalErrorSummary` for each class whose count meets or
// exceeds the broadcast threshold (default N=3 per
// DECISION_LAMARCKIAN §3.2.3 + spec test 3.2.3 verbatim).
//
// **CR-15.2 + halt-trigger #5**: the output struct embeds
// `public_summary` strings + `capsule_id` Cids only — NEVER
// `private_detail_cid` payload bytes. Halt-trigger #5 verifies this by
// serializing the output and scanning for any input
// `private_detail_cid` byte sequence.

/// TRACE_MATRIX FC2-N30 (TB-15 Atom 4; architect §3.2.3 + CR-15.2):
/// public broadcast summary for an N≥threshold cluster of same-class
/// autopsies. Embeds `public_summary` text + capsule Cids only;
/// `private_detail_cid` bytes are NEVER included (halt-trigger #5).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypicalErrorSummary {
    /// The shared loss reason class for the cluster.
    pub loss_reason_class: LossReasonClass,
    /// Count of capsules in this cluster.
    pub count: u32,
    /// Public broadcast text — concatenation / first-N exemplars of
    /// each capsule's `public_summary`. Joined with " ; " separator.
    pub exemplar_public_summary: String,
    /// Cids of the contributing capsules (audit can fetch them with
    /// AuditOnly access). NEVER private_detail_cids.
    pub exemplar_capsule_cids: Vec<Cid>,
}

/// TRACE_MATRIX FC2-N30 (TB-15 Atom 4; architect §3.2.3): cluster
/// autopsies by `loss_reason_class`. Emit a `TypicalErrorSummary` for
/// each class whose count is `>= threshold`. Default architect
/// threshold = 3 (DECISION_LAMARCKIAN §3.2.3 + spec test 3.2.3).
///
/// **Pure** — no CAS access, no env, no clock. Order-stable: input
/// order preserved within each class; classes themselves emerge in
/// `LossReasonClass::tag()` lexicographic order (BTreeMap iteration)
/// for replay-determinism.
///
/// **CR-15.2 + halt-trigger #5**: output never embeds
/// `private_detail_cid` bytes — only `public_summary` strings +
/// `capsule_id` Cids.
pub fn cluster_autopsies(
    autopsies: &[AgentAutopsyCapsule],
    threshold: u8,
) -> Vec<TypicalErrorSummary> {
    use std::collections::BTreeMap;
    // Group by loss_reason_class.tag() for deterministic iteration.
    let mut groups: BTreeMap<String, Vec<&AgentAutopsyCapsule>> = BTreeMap::new();
    for c in autopsies {
        groups
            .entry(c.loss_reason_class.tag().to_string())
            .or_default()
            .push(c);
    }
    let mut out = Vec::new();
    let threshold_usize = threshold as usize;
    for (_tag, members) in groups {
        if members.len() < threshold_usize {
            continue;
        }
        let exemplar_public_summary = members
            .iter()
            .map(|c| c.public_summary.as_str())
            .collect::<Vec<_>>()
            .join(" ; ");
        let exemplar_capsule_cids: Vec<Cid> = members.iter().map(|c| c.capsule_id).collect();
        out.push(TypicalErrorSummary {
            // All members share the same class by construction.
            loss_reason_class: members[0].loss_reason_class.clone(),
            count: members.len() as u32,
            exemplar_public_summary,
            exemplar_capsule_cids,
        });
    }
    out
}

// ────────────────────────────────────────────────────────────────────────────
// TB-15 Atom 3 — `derive_autopsies_for_bankruptcy` (PURE deterministic helper)
// ────────────────────────────────────────────────────────────────────────────
//
// Pure function consumed by both the dispatch arm (to populate
// `EconomicState.agent_autopsies_t` with deterministic Cids) AND by
// the apply_one post-dispatch hook (to write the same bytes to CAS so
// they're retrievable). Replay-determinism: identical inputs → identical
// `(Cid, AgentAutopsyCapsule, private_detail_bytes)` triples.

use crate::state::q_state::EconomicState;
use crate::state::typed_tx::TaskBankruptcyTx;

/// TRACE_MATRIX FC1-N33 (TB-15 Atom 3; architect §6.2 + DECISION_LAMARCKIAN
/// §1.1): pure-deterministic derivation of `AgentAutopsyCapsule`s for a
/// `TaskBankruptcyTx`. Returns one capsule per agent with an active
/// `StakeEntry` pointing at the bankrupted task — `loss_reason_class =
/// Bankruptcy`; `loss_amount = stake.amount`; `evidence_cids = [Cid of
/// stake_tx_id]`. BTreeMap iteration is sorted by `TxId` → output order
/// is deterministic.
///
/// **Pure**: takes pre-bankruptcy `EconomicState` snapshot + the
/// `TaskBankruptcyTx`; no CAS writes, no env access. Used by:
/// - dispatch arm: capsule_id population into `agent_autopsies_t`
/// - apply_one hook: CAS write of the same deterministic bytes
///
/// Replay determinism (Art.0.2): identical `(pre_econ, bk, round, t)` →
/// identical `Vec<BankruptcyAutopsyDerivation>` (same Cids, same bytes,
/// same order).
pub fn derive_autopsies_for_bankruptcy(
    pre_econ: &EconomicState,
    bk: &TaskBankruptcyTx,
    created_at_round: u64,
    created_at_logical_t: u64,
) -> Vec<BankruptcyAutopsyDerivation> {
    let event_id = EventId(bk.task_id.clone());
    let mut out = Vec::new();

    for (stake_tx_id, stake) in pre_econ.stakes_t.0.iter() {
        if stake.task_id != bk.task_id {
            continue;
        }
        // Deterministic private_detail JSON.
        let private_detail = format!(
            "{{\"event_kind\":\"task_bankruptcy\",\"task_id\":\"{}\",\
             \"stake_tx_id\":\"{}\",\"staker\":\"{}\",\
             \"stake_amount_micro\":{}}}",
            stake.task_id.0,
            stake_tx_id.0,
            stake.staker.0,
            stake.amount.micro_units()
        );
        let private_bytes = private_detail.into_bytes();
        let private_detail_cid = Cid::from_content(&private_bytes);

        let public_summary = AgentAutopsyCapsule::format_public_summary(
            &stake.staker,
            &event_id,
            stake.amount,
            &LossReasonClass::Bankruptcy,
        );

        let mut capsule = AgentAutopsyCapsule {
            capsule_id: Cid::default(),
            agent_id: stake.staker.clone(),
            event_id: event_id.clone(),
            loss_amount: stake.amount,
            loss_reason_class: LossReasonClass::Bankruptcy,
            violated_risk_rule: None,
            suggested_policy_patch: None,
            evidence_cids: vec![Cid::from_content(stake_tx_id.0.as_bytes())],
            public_summary,
            private_detail_cid,
            privacy_policy: CapsulePrivacyPolicy::AuditOnly,
            sha256: Hash::ZERO,
            created_at_logical_t,
            created_at_round,
        };
        // R3 closure (Codex R2 VETO TB15-CAS-ID): canonical_encode
        // BEFORE populating capsule_id/sha256 — these stored bytes are
        // what apply_one writes to CAS. capsule_id = sha256 of these
        // bytes, ensuring cas.get(&capsule_id) returns these exact
        // bytes on retrieval.
        let stored_bytes =
            canonical_encode(&capsule).expect("AgentAutopsyCapsule is canonical-encodable");
        let cid = Cid::from_content(&stored_bytes);
        capsule.capsule_id = cid;
        capsule.sha256 = Hash(cid.0);

        out.push(BankruptcyAutopsyDerivation {
            capsule,
            private_bytes,
            stored_capsule_bytes: stored_bytes,
        });
    }
    out
}

/// TRACE_MATRIX TB-15 R3 closure (Codex R2 VETO TB15-CAS-ID): bundle
/// the deterministic outputs of `derive_autopsies_for_bankruptcy`. The
/// dispatch arm reads only `capsule.capsule_id`; apply_one writes
/// `private_bytes` + `stored_capsule_bytes` to CAS keyed by the
/// matching Cids. Replay-safe: identical pre-econ + tx → identical
/// (capsule_id, private_bytes, stored_capsule_bytes) tuple.
#[derive(Debug, Clone)]
pub struct BankruptcyAutopsyDerivation {
    pub capsule: AgentAutopsyCapsule,
    pub private_bytes: Vec<u8>,
    pub stored_capsule_bytes: Vec<u8>,
}

/// TRACE_MATRIX FC1-N33 (TB-15 Atom 3): apply_one post-dispatch hook —
/// writes deterministic autopsy bytes to CAS for a successfully-accepted
/// `TaskBankruptcyTx`. Re-derives the capsule list using
/// `derive_autopsies_for_bankruptcy` (same inputs → same Cids as the
/// dispatch arm already populated into `agent_autopsies_t`).
///
/// Idempotent: CAS `put` of identical bytes returns the existing Cid
/// (replay-safe — re-running apply_one yields the same CAS state).
pub fn write_bankruptcy_autopsies_to_cas(
    cas: &std::sync::Arc<std::sync::RwLock<CasStore>>,
    pre_econ: &EconomicState,
    bk: &TaskBankruptcyTx,
    created_at_round: u64,
    created_at_logical_t: u64,
    creator_str: &str,
) -> Result<Vec<Cid>, AutopsyWriteError> {
    let derived =
        derive_autopsies_for_bankruptcy(pre_econ, bk, created_at_round, created_at_logical_t);
    let mut cids = Vec::with_capacity(derived.len());
    let mut cas_w = cas
        .write()
        .map_err(|_| AutopsyWriteError::InternalLockPoisoned)?;
    for d in derived {
        // R3 closure (Codex R2 VETO TB15-CAS-ID): write the EXACT
        // stored_capsule_bytes returned by the derive helper. CAS keys
        // by sha256(bytes), which equals capsule.capsule_id by helper
        // construction. Idempotent: identical bytes → identical Cid →
        // CAS dedupe. cas.get(&capsule.capsule_id) is now resolvable.
        let _ = cas_w.put(
            &d.private_bytes,
            ObjectType::AutopsyPrivateDetail,
            creator_str,
            created_at_logical_t,
            Some("v1/autopsy_private_detail".into()),
        )?;
        let cas_returned_cid = cas_w.put(
            &d.stored_capsule_bytes,
            ObjectType::AgentAutopsyCapsule,
            creator_str,
            created_at_logical_t,
            Some("v1/agent_autopsy_capsule".into()),
        )?;
        debug_assert_eq!(
            cas_returned_cid, d.capsule.capsule_id,
            "CAS-returned cid must equal capsule.capsule_id (CasStore::put contract)"
        );
        cids.push(d.capsule.capsule_id);
    }
    Ok(cids)
}

// ────────────────────────────────────────────────────────────────────────────
// TB-G G3.2 (charter §1 Module G3; 2026-05-12) — per-task-end bankruptcy
// autopsy emit. Architect Q6 verdict: emit at each TerminalSummaryTx
// boundary (per-task / per-bankruptcy-event), not run-end aggregate. Walks
// `pre_econ.balances_t` for agents below `bankruptcy_risk_cap_micro =
// initial_balance_micro / 10` (architect Q1 verdict; reuses G3.1 SHIPPED
// `classify_solvency` threshold).
// ────────────────────────────────────────────────────────────────────────────

use crate::state::typed_tx::TerminalSummaryTx;

/// TRACE_MATRIX FC3-N12 (TB-G G3.2 2026-05-12): pure-deterministic
/// derivation of `AgentAutopsyCapsule`s at the per-task-end boundary
/// represented by a `TerminalSummaryTx`. Returns one capsule per agent in
/// `pre_econ.balances_t` whose balance is below the risk-cap floor — the
/// architect-mandated bankruptcy gate (Q1: `initial_balance_micro / 10`).
/// `loss_reason_class = Bankruptcy`; `loss_amount = initial - balance`
/// (architect-rendered loss view). BTreeMap iteration is sorted by
/// `AgentId` → output order is deterministic for replay-determinism.
///
/// **Pure**: takes pre-TerminalSummary `EconomicState` snapshot + the
/// `TerminalSummaryTx`; no CAS writes, no env access. Used by:
/// - dispatch arm: capsule_id population into `agent_autopsies_t`
/// - apply_one hook: CAS write of the same deterministic bytes
///
/// Replay determinism (Art.0.2): identical `(pre_econ, ts, round, t)` →
/// identical `Vec<BankruptcyAutopsyDerivation>` (same Cids, same bytes,
/// same order).
///
/// **Architect §7.3 (Markov capsule scope)**: emit per-task is the
/// per-event capsule write. The latest-only read view (NOT historical
/// prompt stuffing) is enforced at the read-side (`UniverseSnapshot` /
/// `compute_agent_pnl` viewer scope), not at the write-side here.
pub fn derive_g3_2_terminal_summary_bankrupt_autopsies(
    pre_econ: &EconomicState,
    ts: &TerminalSummaryTx,
    created_at_round: u64,
    created_at_logical_t: u64,
) -> Vec<BankruptcyAutopsyDerivation> {
    let event_id = EventId(ts.task_id.clone());
    let mut out = Vec::new();

    for (agent_id, balance) in pre_econ.balances_t.0.iter() {
        let initial_micro =
            crate::runtime::agent_pnl::initial_balance_micro_from_default_preseed(agent_id);
        if initial_micro <= 0 {
            // Unknown agent (no preseed entry) — risk-cap is 0, balance
            // cannot be below 0 — skip per Q1 fail-closed semantics.
            continue;
        }
        let risk_cap_micro = initial_micro / 10;
        let bal_micro = balance.micro_units();
        if bal_micro >= risk_cap_micro {
            continue;
        }
        // Loss = initial - current (saturating-non-negative).
        let loss_micro = initial_micro.saturating_sub(bal_micro).max(0);
        let loss_amount = crate::economy::money::MicroCoin::from_micro_units(loss_micro);

        // Deterministic private_detail JSON. Carries task_id + agent_id +
        // initial / current / loss amounts for audit-only inspection.
        let private_detail = format!(
            "{{\"event_kind\":\"g3_2_terminal_summary_bankrupt\",\
             \"task_id\":\"{}\",\"agent_id\":\"{}\",\
             \"initial_balance_micro\":{},\"current_balance_micro\":{},\
             \"risk_cap_micro\":{},\"loss_micro\":{},\
             \"last_logical_t\":{}}}",
            ts.task_id.0,
            agent_id.0,
            initial_micro,
            bal_micro,
            risk_cap_micro,
            loss_micro,
            ts.last_logical_t
        );
        let private_bytes = private_detail.into_bytes();
        let private_detail_cid = Cid::from_content(&private_bytes);

        let public_summary = AgentAutopsyCapsule::format_public_summary(
            agent_id,
            &event_id,
            loss_amount,
            &LossReasonClass::Bankruptcy,
        );

        let mut capsule = AgentAutopsyCapsule {
            capsule_id: Cid::default(),
            agent_id: agent_id.clone(),
            event_id: event_id.clone(),
            loss_amount,
            loss_reason_class: LossReasonClass::Bankruptcy,
            violated_risk_rule: None,
            suggested_policy_patch: None,
            // Evidence anchor: deterministic Cid of the
            // (task_id || agent_id) tuple — links the capsule to the
            // task-end boundary that triggered emit.
            evidence_cids: vec![Cid::from_content(
                format!("{}::{}", ts.task_id.0, agent_id.0).as_bytes(),
            )],
            public_summary,
            private_detail_cid,
            privacy_policy: CapsulePrivacyPolicy::AuditOnly,
            sha256: Hash::ZERO,
            created_at_logical_t,
            created_at_round,
        };
        // R3-style closure: canonical_encode BEFORE populating
        // capsule_id/sha256; stored bytes are what apply_one writes; cid
        // = sha256(stored_bytes); cas.get(&capsule_id) resolves identical
        // bytes (matches TB-15 derive_autopsies_for_bankruptcy pattern).
        let stored_bytes =
            canonical_encode(&capsule).expect("AgentAutopsyCapsule is canonical-encodable");
        let cid = Cid::from_content(&stored_bytes);
        capsule.capsule_id = cid;
        capsule.sha256 = Hash(cid.0);

        out.push(BankruptcyAutopsyDerivation {
            capsule,
            private_bytes,
            stored_capsule_bytes: stored_bytes,
        });
    }
    out
}

/// TRACE_MATRIX FC3-N12 (TB-G G3.2 2026-05-12): apply_one post-dispatch
/// hook — writes deterministic G3.2 terminal-summary bankruptcy autopsies
/// to CAS. Re-derives via `derive_g3_2_terminal_summary_bankrupt_autopsies`;
/// identical inputs produce identical Cids matching `agent_autopsies_t`.
/// Idempotent: re-running apply_one yields the same CAS state (CAS put of
/// identical bytes returns the existing Cid).
pub fn write_g3_2_terminal_summary_bankrupt_autopsies_to_cas(
    cas: &std::sync::Arc<std::sync::RwLock<CasStore>>,
    pre_econ: &EconomicState,
    ts: &TerminalSummaryTx,
    created_at_round: u64,
    created_at_logical_t: u64,
    creator_str: &str,
) -> Result<Vec<Cid>, AutopsyWriteError> {
    let derived = derive_g3_2_terminal_summary_bankrupt_autopsies(
        pre_econ,
        ts,
        created_at_round,
        created_at_logical_t,
    );
    let mut cids = Vec::with_capacity(derived.len());
    let mut cas_w = cas
        .write()
        .map_err(|_| AutopsyWriteError::InternalLockPoisoned)?;
    for d in derived {
        let _ = cas_w.put(
            &d.private_bytes,
            ObjectType::AutopsyPrivateDetail,
            creator_str,
            created_at_logical_t,
            Some("v1/autopsy_private_detail".into()),
        )?;
        let cas_returned_cid = cas_w.put(
            &d.stored_capsule_bytes,
            ObjectType::AgentAutopsyCapsule,
            creator_str,
            created_at_logical_t,
            Some("v1/agent_autopsy_capsule".into()),
        )?;
        debug_assert_eq!(
            cas_returned_cid, d.capsule.capsule_id,
            "CAS-returned cid must equal capsule.capsule_id (CasStore::put contract)"
        );
        cids.push(d.capsule.capsule_id);
    }
    Ok(cids)
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

    // ───────────────────────────────────────────────────────────────────
    // Atom 3 — derive_autopsies_for_bankruptcy tests
    // ───────────────────────────────────────────────────────────────────

    use crate::state::q_state::{
        BalancesIndex, EconomicState, StakeEntry, StakesIndex, TaskMarketEntry, TaskMarketState,
        TaskMarketsIndex, TxId,
    };
    use crate::state::typed_tx::TaskBankruptcyTx;

    fn synthetic_econ_with_stakes(task_id: &str, stakers: &[(&str, &str, i64)]) -> EconomicState {
        let mut econ = EconomicState::default();
        // Add a TaskMarketEntry so the dispatch arm could find the task —
        // not strictly needed by derive_autopsies_for_bankruptcy itself.
        econ.task_markets_t = TaskMarketsIndex::default();
        econ.task_markets_t.0.insert(
            TaskId(task_id.into()),
            TaskMarketEntry {
                state: TaskMarketState::Open,
                ..Default::default()
            },
        );
        // Pre-bankruptcy stakes for the target task (and one off-target
        // stake to verify the filter works).
        let mut stakes = StakesIndex::default();
        for (stake_tx_id, staker_id, amt) in stakers {
            stakes.0.insert(
                TxId((*stake_tx_id).into()),
                StakeEntry {
                    amount: MicroCoin::from_micro_units(*amt),
                    staker: AgentId((*staker_id).into()),
                    task_id: TaskId(task_id.into()),
                },
            );
        }
        // One off-target stake — same Map, different task_id; must be
        // filtered out.
        stakes.0.insert(
            TxId("stake_off_target".into()),
            StakeEntry {
                amount: MicroCoin::from_micro_units(999),
                staker: AgentId("Agent_off_target".into()),
                task_id: TaskId("task:other".into()),
            },
        );
        econ.stakes_t = stakes;
        econ.balances_t = BalancesIndex::default();
        econ
    }

    fn synthetic_bk(task_id: &str) -> TaskBankruptcyTx {
        TaskBankruptcyTx {
            task_id: TaskId(task_id.into()),
            timestamp_logical: 100,
            ..Default::default()
        }
    }

    /// TB-15 Atom 3 — derive_autopsies_for_bankruptcy: per-staker
    /// emission for the target task; off-target stakes filtered out.
    #[test]
    fn derive_autopsies_emits_one_per_staker_target_only() {
        let task = "task:tb15:bankruptcy";
        let econ = synthetic_econ_with_stakes(
            task,
            &[
                ("stake_tx_a", "Agent_A", 1000),
                ("stake_tx_b", "Agent_B", 2000),
            ],
        );
        let bk = synthetic_bk(task);

        let derived =
            derive_autopsies_for_bankruptcy(&econ, &bk, /*round=*/ 5, /*t=*/ 100);

        assert_eq!(
            derived.len(),
            2,
            "2 stakers on the target task → 2 capsules; off-target stake filtered out"
        );
        let agents: Vec<&str> = derived
            .iter()
            .map(|d| d.capsule.agent_id.0.as_str())
            .collect();
        assert!(agents.contains(&"Agent_A"));
        assert!(agents.contains(&"Agent_B"));
        assert!(!agents.contains(&"Agent_off_target"));

        // Each capsule reports the correct event_id, loss_amount,
        // loss_reason_class, and a populated capsule_id.
        for d in &derived {
            let c = &d.capsule;
            assert_eq!(c.event_id.0 .0, task);
            assert_eq!(c.loss_reason_class, LossReasonClass::Bankruptcy);
            assert_ne!(c.capsule_id, Cid::default());
            assert_eq!(c.capsule_id.0, c.sha256.0);
            assert!(c.public_summary.contains(task));
            assert!(c.public_summary.contains("Bankruptcy"));
            // R3 closure (Codex R2 VETO TB15-CAS-ID): capsule_id MUST
            // equal sha256(stored_capsule_bytes).
            assert_eq!(
                c.capsule_id,
                Cid::from_content(&d.stored_capsule_bytes),
                "capsule_id must equal sha256(stored_capsule_bytes) — \
                 cas.get(&capsule_id) resolvability contract"
            );
        }
    }

    /// TB-15 Atom 3 — derive_autopsies_for_bankruptcy: same inputs →
    /// identical (Cid, capsule, bytes) — replay-determinism foundation
    /// (Art.0.2). Underwrites the dispatch / apply_one Cid agreement.
    #[test]
    fn derive_autopsies_deterministic_across_calls() {
        let task = "task:tb15:det";
        let econ = synthetic_econ_with_stakes(
            task,
            &[
                ("stake_tx_x", "Agent_X", 500),
                ("stake_tx_y", "Agent_Y", 750),
            ],
        );
        let bk = synthetic_bk(task);

        let a = derive_autopsies_for_bankruptcy(&econ, &bk, 3, 50);
        let b = derive_autopsies_for_bankruptcy(&econ, &bk, 3, 50);

        assert_eq!(a.len(), b.len());
        for (i, (da, db)) in a.iter().zip(b.iter()).enumerate() {
            assert_eq!(
                da.capsule.capsule_id, db.capsule.capsule_id,
                "capsule {i} cid mismatch"
            );
            assert_eq!(da.capsule, db.capsule, "capsule {i} struct mismatch");
            assert_eq!(
                da.private_bytes, db.private_bytes,
                "capsule {i} private_detail bytes mismatch"
            );
            assert_eq!(
                da.stored_capsule_bytes, db.stored_capsule_bytes,
                "capsule {i} stored_capsule_bytes mismatch"
            );
        }
    }

    /// TB-15 Atom 3 — derive_autopsies_for_bankruptcy: no stakers on
    /// the bankrupted task → empty Vec (no capsules emitted).
    #[test]
    fn derive_autopsies_empty_when_no_stakers() {
        let task = "task:tb15:nostakers";
        let mut econ = EconomicState::default();
        econ.task_markets_t.0.insert(
            TaskId(task.into()),
            TaskMarketEntry {
                state: TaskMarketState::Open,
                ..Default::default()
            },
        );
        let bk = synthetic_bk(task);
        let derived = derive_autopsies_for_bankruptcy(&econ, &bk, 0, 0);
        assert!(derived.is_empty());
    }

    // ───────────────────────────────────────────────────────────────────
    // Atom 4 — cluster_autopsies tests
    // ───────────────────────────────────────────────────────────────────

    fn mk_autopsy(agent: &str, class: LossReasonClass, priv_byte: u8) -> AgentAutopsyCapsule {
        let mut cap = AgentAutopsyCapsule::default();
        cap.agent_id = AgentId(agent.into());
        cap.event_id = EventId(TaskId("task:tb15:cluster".into()));
        cap.loss_amount = MicroCoin::from_micro_units(1_000);
        cap.loss_reason_class = class.clone();
        cap.public_summary = AgentAutopsyCapsule::format_public_summary(
            &cap.agent_id,
            &cap.event_id,
            cap.loss_amount,
            &class,
        );
        cap.private_detail_cid = Cid([priv_byte; 32]);
        cap.capsule_id = Cid::from_content(agent.as_bytes());
        cap
    }

    /// TB-15 Atom 4 — 3 same-class autopsies → exactly 1 TypicalErrorSummary.
    #[test]
    fn cluster_autopsies_three_same_class_emits_one() {
        let autopsies = vec![
            mk_autopsy("A", LossReasonClass::Bankruptcy, 0xAA),
            mk_autopsy("B", LossReasonClass::Bankruptcy, 0xBB),
            mk_autopsy("C", LossReasonClass::Bankruptcy, 0xCC),
        ];
        let summaries = cluster_autopsies(&autopsies, 3);
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].count, 3);
        assert_eq!(summaries[0].loss_reason_class, LossReasonClass::Bankruptcy);
        assert_eq!(summaries[0].exemplar_capsule_cids.len(), 3);
    }

    /// TB-15 Atom 4 — 2 same-class autopsies → 0 broadcasts (below
    /// threshold).
    #[test]
    fn cluster_autopsies_two_same_class_emits_zero() {
        let autopsies = vec![
            mk_autopsy("A", LossReasonClass::Bankruptcy, 0xAA),
            mk_autopsy("B", LossReasonClass::Bankruptcy, 0xBB),
        ];
        let summaries = cluster_autopsies(&autopsies, 3);
        assert_eq!(summaries.len(), 0);
    }

    /// TB-15 Atom 4 — mixed classes: only ones with count >= threshold
    /// emerge; ordering deterministic (BTreeMap by class tag).
    #[test]
    fn cluster_autopsies_mixed_classes_filters_below_threshold() {
        let autopsies = vec![
            mk_autopsy("A", LossReasonClass::Bankruptcy, 0xAA),
            mk_autopsy("B", LossReasonClass::Bankruptcy, 0xBB),
            mk_autopsy("C", LossReasonClass::Bankruptcy, 0xCC),
            mk_autopsy("D", LossReasonClass::SlashLoss, 0xDD),
            mk_autopsy("E", LossReasonClass::SlashLoss, 0xEE),
            mk_autopsy("F", LossReasonClass::SlashLoss, 0xFF),
            mk_autopsy("G", LossReasonClass::SlashLoss, 0x11),
            mk_autopsy("H", LossReasonClass::Goodhart, 0x22),
        ];
        let summaries = cluster_autopsies(&autopsies, 3);
        // Bankruptcy (3) + SlashLoss (4) = 2 broadcasts; Goodhart (1) below threshold.
        assert_eq!(summaries.len(), 2);
        let counts: Vec<u32> = summaries.iter().map(|s| s.count).collect();
        assert!(counts.contains(&3));
        assert!(counts.contains(&4));
    }

    /// TB-15 Atom 4 — halt-trigger #5: TypicalErrorSummary serialization
    /// MUST NOT contain any input private_detail_cid bytes.
    #[test]
    fn cluster_autopsies_output_never_embeds_private_detail_bytes() {
        let priv_bytes = [0x77u8, 0x88u8, 0x99u8];
        let autopsies = vec![
            mk_autopsy("A", LossReasonClass::Bankruptcy, priv_bytes[0]),
            mk_autopsy("B", LossReasonClass::Bankruptcy, priv_bytes[1]),
            mk_autopsy("C", LossReasonClass::Bankruptcy, priv_bytes[2]),
        ];
        let summaries = cluster_autopsies(&autopsies, 3);
        let bytes = serde_json::to_vec(&summaries).expect("serialize summaries");
        for &priv_byte in &priv_bytes {
            // Each Cid is 32 identical bytes; checking for any 32-byte run.
            let private_cid = [priv_byte; 32];
            for window in bytes.windows(32) {
                assert!(
                    window != private_cid,
                    "halt-trigger #5: TypicalErrorSummary serialization contains \
                     private_detail_cid byte run for byte=0x{:02x}",
                    priv_byte
                );
            }
        }
    }

    /// TB-15 Atom 4 — empty input → empty output (no panic).
    #[test]
    fn cluster_autopsies_empty_input() {
        let summaries = cluster_autopsies(&[], 3);
        assert!(summaries.is_empty());
    }

    // ───────────────────────────────────────────────────────────────────
    // R2 closure — activation gate tests (Gemini R1 VETO Q12)
    // ───────────────────────────────────────────────────────────────────

    /// R2 closure: activation gate predicate is true at default
    /// constant (TB15_AUTOPSY_ACTIVATION_LOGICAL_T = 0); fresh chains
    /// (any timestamp_logical >= 0) trivially satisfy the gate.
    #[test]
    fn activation_gate_default_is_always_active_for_fresh_chains() {
        // Default constant is 0; any u64 (including 0 itself) is >= 0.
        assert!(
            is_autopsy_active_at(0),
            "logical_t 0 must be active under default const 0"
        );
        assert!(is_autopsy_active_at(1), "logical_t 1 must be active");
        assert!(
            is_autopsy_active_at(u64::MAX),
            "logical_t MAX must be active"
        );
        // Documentation: TB15_AUTOPSY_ACTIVATION_LOGICAL_T == 0 is the
        // shipped default; pre-TB-15 chain migration would override the
        // const to a non-zero cutoff.
        assert_eq!(
            TB15_AUTOPSY_ACTIVATION_LOGICAL_T, 0,
            "shipped default must be 0 (always-active for fresh chains)"
        );
    }

    /// TB-15 Atom 3 — write_bankruptcy_autopsies_to_cas: writes
    /// 2 CAS objects per staker (capsule + private_detail). Returned
    /// Cids match the dispatch arm's deterministic derivation.
    #[test]
    fn write_bankruptcy_autopsies_to_cas_round_trip() {
        use std::sync::{Arc, RwLock};
        use tempfile::TempDir;

        let task = "task:tb15:cas_writeback";
        let econ = synthetic_econ_with_stakes(
            task,
            &[("stake_w1", "Agent_W1", 100), ("stake_w2", "Agent_W2", 200)],
        );
        let bk = synthetic_bk(task);

        let tmp = TempDir::new().expect("tempdir");
        let cas = Arc::new(RwLock::new(
            crate::bottom_white::cas::store::CasStore::open(tmp.path()).expect("cas"),
        ));

        let cids = write_bankruptcy_autopsies_to_cas(&cas, &econ, &bk, 7, 42, "tb15-test-writer")
            .expect("write succeeds");

        assert_eq!(cids.len(), 2);

        // Cids match what derive returns (replay-determinism contract).
        let derived = derive_autopsies_for_bankruptcy(&econ, &bk, 7, 42);
        let derived_cids: Vec<Cid> = derived.iter().map(|d| d.capsule.capsule_id).collect();
        assert_eq!(cids, derived_cids);

        // CAS now contains 4 objects per 2 stakers: 2 private_detail + 2 capsule.
        let cas_r = cas.read().expect("cas read");
        assert_eq!(
            cas_r.len(),
            4,
            "2 stakers × 2 CAS objects (private_detail + capsule) = 4"
        );

        // R3 closure (Codex R2 VETO TB15-CAS-ID): cas.get(&capsule.capsule_id)
        // MUST succeed for every emitted Cid. This is the contract Codex
        // R2 found broken in the prior implementation.
        for cid in &cids {
            let bytes = cas_r
                .get(cid)
                .expect("R3 contract: cas.get(&capsule_id) MUST succeed");
            // The retrieved bytes' sha256 MUST equal the cid (CAS
            // content-addressed integrity).
            assert_eq!(
                Cid::from_content(&bytes),
                *cid,
                "R3 contract: sha256(retrieved bytes) == capsule_id"
            );
            // The retrieved bytes canonical_decode to a struct with the
            // same field values (modulo capsule_id/sha256 which are
            // derived; restored via restore_autopsy_capsule_from_cas_bytes).
            let restored = restore_autopsy_capsule_from_cas_bytes(&bytes)
                .expect("R3 contract: canonical_decode + restore succeeds");
            assert_eq!(
                restored.capsule_id, *cid,
                "R3 contract: restored capsule_id matches CAS Cid"
            );
        }
    }
}
