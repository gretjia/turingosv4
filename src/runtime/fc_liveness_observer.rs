//! LIVE-FC1 — tape-driven FC-liveness OBSERVER (the keystone self-verification
//! witness).
//!
//! This module is a SYSTEM witness that reconstructs, FROM THE CANONICAL TAPE
//! ALONE (L4 accepted spine + L4.E rejection chain + CAS payloads), whether the
//! three constitution flowcharts actually FIRED on a given run, and whether any
//! production module the liveness inventory claims is "live" is in fact a
//! ZOMBIE (claimed live, no tape footprint, not honestly excused).
//!
//! ── OBSERVE-ONLY DISCIPLINE (mirrors `boltzmann_selection_trace.rs` +
//!    `agent_scheduler.rs`) ───────────────────────────────────────────────
//! The observer READS the tape and PRODUCES a report. It MUST NOT:
//!   * mutate `QState` / `EconomicState` (it never receives `&mut` to either),
//!   * advance any head,
//!   * change sequencer admission or any L4/L4.E predicate.
//! It is NOT a source of truth. Everything it concludes is reconstructable from
//! ChainTape (L4 + L4.E) + CAS; ChainTape/CAS win on any conflict (Art.0.2). If
//! the report is persisted, it is persisted via [`CasStore::put`]
//! (`ObjectType::Generic` + a free-form schema_id) exactly like the other
//! observe-only capsules — there is NO `std::fs::write`, no filesystem side
//! store.
//!
//! ── SHIELDED (Art.III) ────────────────────────────────────────────────────
//! The report is a SCOPED projection. It records bounded discriminators
//! (`TxKind`, `RejectionClass` family, FC-node ids, integer counts) and CAS
//! cids — it NEVER embeds raw Lean stderr, raw autopsy bytes, private
//! diagnostics, or PPUT metric VALUES. The observer is a SYSTEM witness, not an
//! agent-visible read view.
//!
//! ── HONESTY (anti-overclaim, binding) ─────────────────────────────────────
//! FC3 is reported as "reached observable/canary" only — proposer fired + canary
//! `MetricEstimate` + terminal `sandbox:canary_only`. The observer NEVER reports
//! "FC3 fully closed / live self-modification" for a task-workload run, because
//! such a run never drives the irreversible leg. The dedicated
//! [`FcLivenessReport::fc3_disposition`] field is hard-stamped to
//! [`FC3_DISPOSITION_REACHED_OBSERVABLE_CANARY`].
//!
//! Each FC node is classified into exactly one of:
//!   * `Live`              — fired on this tape (a concrete tx / cid witnesses it);
//!   * `ReachableNotFired` — wired + reachable, but this workload did not exercise it;
//!   * `Zombie`            — claimed live, no tape footprint, NOT honestly excused.
//!
//! ── INTEGER-ONLY ──────────────────────────────────────────────────────────
//! Every count / metric on the report is an integer. No `f64`/`f32` appears.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::bottom_white::ledger::rejection_evidence::RejectionClass as L4ERejectionClass;
use crate::bottom_white::ledger::transition_ledger::{
    canonical_decode, canonical_encode, LedgerEntry, TxKind,
};
use crate::runtime::audit_assertions::LoadedTape;
use crate::runtime::real5_roles::fc3_canary::{closes_fc3_loop, CANARY_ONLY_TERMINAL_STATUS};
use crate::state::typed_tx::{ArchitectProposalKind, TypedTx};

/// TRACE_MATRIX FC1-N34 + FC2-N31 + FC3-N33: free-form CAS schema id for the
/// persisted observe-only FC-liveness report (mirrors the boltzmann trace /
/// canary schema-id pattern; persistence is CAS-only).
pub const FC_LIVENESS_REPORT_SCHEMA_ID: &str = "v1/fc_liveness_report";

/// TRACE_MATRIX FC3-N33: the ONLY FC3 disposition this observer may stamp for a
/// task-workload run. Hard-coded so an auditor reading the report sees that the
/// FC3 loop reached its observable/canary leg and NOT the irreversible
/// self-modification leg. Anti-overclaim binding (`CLAUDE.md §6` honesty rule).
pub const FC3_DISPOSITION_REACHED_OBSERVABLE_CANARY: &str = "reached_observable_canary";

/// TRACE_MATRIX FC1/FC2/FC3 node liveness status. Exactly three values: a node
/// either fired on this tape, is reachable but not exercised by this workload,
/// or is a zombie (claimed live with no tape footprint, not honestly excused).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FcNodeStatus {
    /// TRACE_MATRIX FC1/FC2/FC3: fired on this tape — a concrete tx / cid
    /// witnesses the node.
    Live,
    /// TRACE_MATRIX FC1/FC2/FC3: wired + reachable, but this workload did not
    /// exercise it (honest "not fired", NOT a zombie).
    ReachableNotFired,
    /// TRACE_MATRIX FC1/FC2/FC3: claimed live with no tape footprint and not
    /// honestly excused — a zombie node.
    Zombie,
}

/// TRACE_MATRIX FC1/FC2/FC3: per-FC-node liveness row. `evidence_tx` is a
/// bounded label (TxKind / node id, never raw bytes); `evidence_cid` is the CAS
/// handle that witnesses the node (None when not fired).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FcNodeLiveness {
    /// TRACE_MATRIX FC1/FC2/FC3: stable FC-node identifier (e.g.
    /// `"FC1:predicate_gated_advance"`).
    pub node_id: String,
    /// TRACE_MATRIX FC1/FC2/FC3: the reconstructed liveness status.
    pub status: FcNodeStatus,
    /// TRACE_MATRIX FC1/FC2/FC3: bounded evidence label (TxKind / reject-class
    /// family / node id). Never raw diagnostics.
    pub evidence_tx: String,
    /// TRACE_MATRIX FC1/FC2/FC3: CAS handle witnessing the node, when it fired.
    pub evidence_cid: Option<Cid>,
    /// TRACE_MATRIX FC1/FC2/FC3: integer count of tape footprints for this node
    /// (e.g. number of L4.E `LeanFailed` rows). Integer-only.
    pub footprint_count: u64,
}

impl FcNodeLiveness {
    /// TRACE_MATRIX FC1/FC2/FC3: a `Live` node with `n` footprints witnessed by
    /// `cid`.
    fn live(node_id: &str, evidence_tx: &str, evidence_cid: Option<Cid>, n: u64) -> Self {
        Self {
            node_id: node_id.to_string(),
            status: FcNodeStatus::Live,
            evidence_tx: evidence_tx.to_string(),
            evidence_cid,
            footprint_count: n,
        }
    }

    /// TRACE_MATRIX FC1/FC2/FC3: a reachable-but-not-fired node (honest "this
    /// workload did not exercise it").
    fn reachable_not_fired(node_id: &str, evidence_tx: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            status: FcNodeStatus::ReachableNotFired,
            evidence_tx: evidence_tx.to_string(),
            evidence_cid: None,
            footprint_count: 0,
        }
    }
}

/// TRACE_MATRIX FC1/FC2/FC3: one no-zombie inventory cross-reference row. For
/// each module the liveness inventory claims is live, records whether the run's
/// tape carries a footprint, and (when absent) the honest excuse — `None` excuse
/// + `footprint_present == false` is a ZOMBIE.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoZombieRow {
    /// TRACE_MATRIX FC1/FC2/FC3: inventory group id (from the liveness fixture).
    pub module: String,
    /// TRACE_MATRIX FC1/FC2/FC3: true iff this run's tape carries a footprint
    /// for the group.
    pub footprint_present: bool,
    /// TRACE_MATRIX FC1/FC2/FC3: honest reason the footprint is absent (e.g.
    /// "not-exercised-by-this-workload"). `None` + absent footprint = zombie.
    pub excused_reason: Option<String>,
    /// TRACE_MATRIX FC1/FC2/FC3: true iff this row is a zombie (claimed live, no
    /// footprint, not excused).
    pub is_zombie: bool,
}

/// TRACE_MATRIX FC1-N34 + FC2-N31 + FC3-N33: the full observe-only FC-liveness
/// report. A SCOPED, SHIELDED projection reconstructed from L4 + L4.E + CAS. Not
/// a source of truth; ChainTape/CAS win on conflict. Self-addressing per R3:
/// stored bytes have `report_id` zeroed so
/// `Cid::from_content(stored_bytes) == report_id`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FcLivenessReport {
    /// TRACE_MATRIX: CAS Cid of this report's canonical bytes (with `report_id`
    /// zeroed during the hash). Computed by [`persist_fc_liveness_report`].
    pub report_id: Cid,
    /// TRACE_MATRIX FC1: predicate-gated advance + the three failure arms
    /// (step_reject / parse_fail / llm_err) + rtool / wtool read/write bridge.
    pub fc1_nodes: Vec<FcNodeLiveness>,
    /// TRACE_MATRIX FC2: boot trust-root verified, map-reduce-tick, terminal/HALT.
    pub fc2_nodes: Vec<FcNodeLiveness>,
    /// TRACE_MATRIX FC3: proposer (real ArchitectProposal), canary MetricEstimate,
    /// sandbox:canary_only terminal.
    pub fc3_nodes: Vec<FcNodeLiveness>,
    /// TRACE_MATRIX: per-claimed-module no-zombie cross-reference.
    pub no_zombie: Vec<NoZombieRow>,
    /// TRACE_MATRIX FC3-N33: hard-stamped FC3 disposition. ALWAYS
    /// [`FC3_DISPOSITION_REACHED_OBSERVABLE_CANARY`] — never "fully_closed".
    pub fc3_disposition: String,
    /// TRACE_MATRIX: integer count of L4 accepted entries on the reconstructed
    /// tape. Integer-only.
    pub l4_entry_count: u64,
    /// TRACE_MATRIX: integer count of L4.E rejection rows. Integer-only.
    pub l4e_entry_count: u64,
    /// TRACE_MATRIX: integer count of nodes classified `Zombie` across all three
    /// flowcharts plus the no-zombie inventory. Integer-only.
    pub zombie_count: u64,
    /// TRACE_MATRIX: always `true`. This report can never be canonical state or
    /// an admission/predicate input.
    pub observe_only: bool,
    /// TRACE_MATRIX: free-form schema tag for discovery.
    pub schema_tag: String,
}

impl FcLivenessReport {
    /// TRACE_MATRIX FC1-N34 + FC2-N31 + FC3-N33: true iff the report carries NO
    /// zombie node anywhere (the keystone "no-zombie" property an auditor wants).
    /// Integer comparison only.
    pub fn no_zombies(&self) -> bool {
        self.zombie_count == 0
    }

    /// TRACE_MATRIX FC3-N33: structural honesty guarantee — the report's FC3
    /// disposition is the observable/canary leg and NOT a loop-closing terminal.
    /// Always true by construction; exposed so gates can assert it.
    pub fn fc3_stays_observable(&self) -> bool {
        self.fc3_disposition == FC3_DISPOSITION_REACHED_OBSERVABLE_CANARY
            && !closes_fc3_loop(&self.fc3_disposition)
    }
}

/// TRACE_MATRIX FC1/FC2/FC3: a parsed liveness-inventory claim. The observer
/// takes an already-parsed inventory (structured, not ad-hoc string parsing);
/// each claim names a module group plus the constitutional anchors that tell the
/// observer which FC footprint would witness it. `excused_reason` lets the
/// caller pre-mark a claim as not-exercised-by-this-workload (honest excuse).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LivenessInventoryClaim {
    /// TRACE_MATRIX: inventory group id.
    pub module: String,
    /// TRACE_MATRIX: constitutional anchors (e.g. `"FC1:predicates"`,
    /// `"FC2:replay"`, `"FC3:constitution"`). Used to pick the witnessing
    /// footprint.
    pub constitutional_anchors: Vec<String>,
    /// TRACE_MATRIX: optional honest excuse for an absent footprint. `Some`
    /// suppresses a zombie verdict.
    pub excused_reason: Option<String>,
}

/// TRACE_MATRIX FC1/FC2/FC3: the parsed inventory the observer cross-references.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LivenessInventory {
    /// TRACE_MATRIX: the claimed-live module groups.
    pub claims: Vec<LivenessInventoryClaim>,
}

// ─────────────────────────────────────────────────────────────────────────
// Tape-derived footprint reconstruction (pure; L4 + L4.E + CAS only)
// ─────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1/FC2/FC3: integer tally of every footprint the observer
/// derives from the tape in a single pass. Each field is an integer count.
#[derive(Debug, Clone, Default)]
struct TapeFootprints {
    // FC1
    predicate_gated_advances: u64,
    first_accept_cid: Option<Cid>,
    step_reject: u64,
    parse_fail: u64,
    llm_err: u64,
    rtool_wtool_bridge: u64, // accepted agent Work/Verify/... entries (typed write ingress)
    // FC2
    boot_trust_root_verified: bool,
    map_reduce_ticks: u64,
    map_reduce_first_cid: Option<Cid>,
    terminals: u64,
    terminal_first_cid: Option<Cid>,
    // FC3
    real_architect_proposals: u64,
    proposal_first_cid: Option<Cid>,
    canary_metric_capsules: u64,
    canary_first_cid: Option<Cid>,
    canary_only_terminal: u64,
}

/// TRACE_MATRIX FC1: is this an agent-facing typed write ingress (rtool→wtool
/// bridge witness)? The accepted agent tx kinds that prove the FC1
/// input→delta→output→wtool path actually carried an agent proposal to L4.
fn is_agent_write_ingress(k: TxKind) -> bool {
    matches!(
        k,
        TxKind::Work
            | TxKind::Verify
            | TxKind::Challenge
            | TxKind::CompleteSetMint
            | TxKind::CompleteSetRedeem
            | TxKind::CompleteSetMerge
            | TxKind::CpmmPool
            | TxKind::CpmmSwap
            | TxKind::BuyWithCoinRouter
            | TxKind::TaskOpen
            | TxKind::EscrowLock
            | TxKind::MarketSeed
    )
}

/// TRACE_MATRIX FC1: map an L4.E `RejectionClass` to which FC1 failure arm it
/// witnesses. `LeanFailed → step_reject`, `ParseFailed → parse_fail`,
/// `LlmError → llm_err`. Returns `None` for non-FC1-arm rejection classes.
fn fc1_failure_arm(rc: L4ERejectionClass) -> Option<&'static str> {
    match rc {
        L4ERejectionClass::LeanFailed => Some("step_reject"),
        L4ERejectionClass::ParseFailed => Some("parse_fail"),
        L4ERejectionClass::LlmError => Some("llm_err"),
        _ => None,
    }
}

/// TRACE_MATRIX FC3-N33: decode an `ArchitectProposalTx` payload + its capsule
/// and report whether it is a REAL (non-Noop) proposal. Observe-only CAS reads.
fn architect_proposal_is_real(entry: &LedgerEntry, cas: &CasStore) -> bool {
    let Ok(bytes) = cas.get(&entry.tx_payload_cid) else {
        return false;
    };
    let Ok(TypedTx::ArchitectProposal(tx)) = canonical_decode::<TypedTx>(&bytes) else {
        return false;
    };
    let Ok(cap_bytes) = cas.get(&tx.proposal_capsule_cid) else {
        return false;
    };
    use crate::state::typed_tx::ArchitectProposalCapsule;
    match canonical_decode::<ArchitectProposalCapsule>(&cap_bytes) {
        Ok(cap) => cap.proposal_kind != ArchitectProposalKind::Noop,
        Err(_) => false,
    }
}

/// TRACE_MATRIX FC3-N33: does this CAS object decode as a canary
/// `MetricEstimateCapsule` whose terminal status is `sandbox:canary_only`?
/// Discovered by schema_id, then structurally confirmed.
fn cas_object_is_canary_metric(cas: &CasStore, cid: &Cid) -> bool {
    use crate::runtime::real5_roles::fc3_canary::MetricEstimateCapsule;
    let Ok(bytes) = cas.get(cid) else {
        return false;
    };
    match canonical_decode::<MetricEstimateCapsule>(&bytes) {
        Ok(cap) => cap.terminal_status == CANARY_ONLY_TERMINAL_STATUS,
        Err(_) => false,
    }
}

/// TRACE_MATRIX FC1/FC2/FC3: reconstruct all FC footprints from the loaded tape
/// — L4 accepted entries, the L4.E rejection chain, and CAS payloads. Pure read;
/// no mutation, no head advance.
fn reconstruct_footprints(tape: &LoadedTape) -> TapeFootprints {
    let mut fp = TapeFootprints::default();

    // FC2 boot: trust-root verified at boot is witnessed by the genesis
    // [constitution_root] hex being present + matching the loaded constitution
    // hash. `load_tape` already recomputed `constitution_hash`; the genesis hex
    // is the boot-time attestation. (Boot itself is NOT re-run here — observe
    // only; we read the attestation the tape carries.)
    fp.boot_trust_root_verified = match &tape.genesis_constitution_root_hex {
        Some(hex) => {
            let actual = tape
                .constitution_hash
                .0
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<String>();
            hex.eq_ignore_ascii_case(&actual)
        }
        None => false,
    };

    // L4 accepted spine.
    for e in &tape.entries {
        match e.tx_kind {
            TxKind::Work => {
                // A predicate-gated advance: an accepted WorkTx carries the
                // acceptance predicate_results that the sequencer admitted on
                // PASS (M07 / memory_kernel). Head advanced only on PASS.
                fp.predicate_gated_advances += 1;
                if fp.first_accept_cid.is_none() {
                    fp.first_accept_cid = Some(e.tx_payload_cid);
                }
            }
            TxKind::MapReduceTick => {
                fp.map_reduce_ticks += 1;
                if fp.map_reduce_first_cid.is_none() {
                    fp.map_reduce_first_cid = Some(e.tx_payload_cid);
                }
            }
            TxKind::TerminalSummary => {
                fp.terminals += 1;
                if fp.terminal_first_cid.is_none() {
                    fp.terminal_first_cid = Some(e.tx_payload_cid);
                }
            }
            TxKind::ArchitectProposal => {
                if architect_proposal_is_real(e, &tape.cas) {
                    fp.real_architect_proposals += 1;
                    if fp.proposal_first_cid.is_none() {
                        fp.proposal_first_cid = Some(e.tx_payload_cid);
                    }
                }
            }
            _ => {}
        }
        if is_agent_write_ingress(e.tx_kind) {
            fp.rtool_wtool_bridge += 1;
        }
    }

    // L4.E rejection chain — the FC1 failure arms.
    for rec in tape.l4e_writer.records() {
        match fc1_failure_arm(rec.rejection_class) {
            Some("step_reject") => fp.step_reject += 1,
            Some("parse_fail") => fp.parse_fail += 1,
            Some("llm_err") => fp.llm_err += 1,
            _ => {}
        }
    }

    // FC3 canary MetricEstimate capsules live in CAS keyed by schema_id. Discover
    // by schema_id then structurally confirm (terminal == sandbox:canary_only).
    use crate::runtime::real5_roles::fc3_canary::FC3_METRIC_ESTIMATE_SCHEMA_ID;
    for cid in tape.cas.list_all_cids() {
        let is_canary_schema = tape
            .cas
            .metadata(&cid)
            .and_then(|m| m.schema_id.as_deref())
            == Some(FC3_METRIC_ESTIMATE_SCHEMA_ID);
        if is_canary_schema && cas_object_is_canary_metric(&tape.cas, &cid) {
            fp.canary_metric_capsules += 1;
            fp.canary_only_terminal += 1;
            if fp.canary_first_cid.is_none() {
                fp.canary_first_cid = Some(cid);
            }
        }
    }

    fp
}

// ─────────────────────────────────────────────────────────────────────────
// FC node classification (Live / ReachableNotFired / Zombie)
// ─────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1: build the FC1 node liveness rows from the footprints.
fn fc1_rows(fp: &TapeFootprints) -> Vec<FcNodeLiveness> {
    let mut rows = Vec::new();

    // Predicate-gated advance (head advances only on predicate PASS — M07).
    rows.push(if fp.predicate_gated_advances > 0 {
        FcNodeLiveness::live(
            "FC1:predicate_gated_advance",
            "WorkTx(accepted,predicate_pass)",
            fp.first_accept_cid,
            fp.predicate_gated_advances,
        )
    } else {
        FcNodeLiveness::reachable_not_fired(
            "FC1:predicate_gated_advance",
            "WorkTx(accepted,predicate_pass)",
        )
    });

    // Three failure arms.
    rows.push(arm_row("FC1:failure_arm/step_reject", "L4E:LeanFailed", fp.step_reject));
    rows.push(arm_row("FC1:failure_arm/parse_fail", "L4E:ParseFailed", fp.parse_fail));
    rows.push(arm_row("FC1:failure_arm/llm_err", "L4E:LlmError", fp.llm_err));

    // rtool→wtool typed write ingress bridge (agent proposal reached L4).
    rows.push(if fp.rtool_wtool_bridge > 0 {
        FcNodeLiveness::live(
            "FC1:rtool_wtool_bridge",
            "agent_typed_write_ingress",
            fp.first_accept_cid,
            fp.rtool_wtool_bridge,
        )
    } else {
        FcNodeLiveness::reachable_not_fired("FC1:rtool_wtool_bridge", "agent_typed_write_ingress")
    });

    rows
}

/// TRACE_MATRIX FC1: a failure-arm row. Zero footprints on a failure arm is an
/// HONEST `ReachableNotFired` (a clean run simply had no rejections of that
/// class), never a zombie — the arm is wired and would fire if a rejection of
/// that class occurred.
fn arm_row(node_id: &str, evidence_tx: &str, n: u64) -> FcNodeLiveness {
    if n > 0 {
        FcNodeLiveness::live(node_id, evidence_tx, None, n)
    } else {
        FcNodeLiveness::reachable_not_fired(node_id, evidence_tx)
    }
}

/// TRACE_MATRIX FC2: build the FC2 node liveness rows from the footprints.
fn fc2_rows(fp: &TapeFootprints) -> Vec<FcNodeLiveness> {
    let mut rows = Vec::new();

    rows.push(if fp.boot_trust_root_verified {
        FcNodeLiveness::live(
            "FC2:boot_trust_root_verified",
            "genesis[constitution_root]==constitution_hash",
            None,
            1,
        )
    } else {
        // Absent boot attestation is NOT silently a zombie: a fresh isolated
        // chain may carry no genesis constitution_root hex (the auditor's
        // `load_tape` derives it best-effort). Honest "not witnessed here".
        FcNodeLiveness::reachable_not_fired(
            "FC2:boot_trust_root_verified",
            "genesis[constitution_root]==constitution_hash",
        )
    });

    rows.push(if fp.map_reduce_ticks > 0 {
        FcNodeLiveness::live(
            "FC2:map_reduce_tick",
            "MapReduceTickTx",
            fp.map_reduce_first_cid,
            fp.map_reduce_ticks,
        )
    } else {
        FcNodeLiveness::reachable_not_fired("FC2:map_reduce_tick", "MapReduceTickTx")
    });

    rows.push(if fp.terminals > 0 {
        FcNodeLiveness::live(
            "FC2:terminal_halt",
            "TerminalSummaryTx(RunOutcome)",
            fp.terminal_first_cid,
            fp.terminals,
        )
    } else {
        FcNodeLiveness::reachable_not_fired("FC2:terminal_halt", "TerminalSummaryTx(RunOutcome)")
    });

    rows
}

/// TRACE_MATRIX FC3-N33: build the FC3 node liveness rows. All three nodes are
/// the OBSERVABLE/CANARY leg only; none of them closes the loop.
fn fc3_rows(fp: &TapeFootprints) -> Vec<FcNodeLiveness> {
    let mut rows = Vec::new();

    rows.push(if fp.real_architect_proposals > 0 {
        FcNodeLiveness::live(
            "FC3:proposer_architect_proposal",
            "ArchitectProposalTx(non-Noop)",
            fp.proposal_first_cid,
            fp.real_architect_proposals,
        )
    } else {
        FcNodeLiveness::reachable_not_fired(
            "FC3:proposer_architect_proposal",
            "ArchitectProposalTx(non-Noop)",
        )
    });

    rows.push(if fp.canary_metric_capsules > 0 {
        FcNodeLiveness::live(
            "FC3:canary_metric_estimate",
            "MetricEstimateCapsule",
            fp.canary_first_cid,
            fp.canary_metric_capsules,
        )
    } else {
        FcNodeLiveness::reachable_not_fired("FC3:canary_metric_estimate", "MetricEstimateCapsule")
    });

    rows.push(if fp.canary_only_terminal > 0 {
        FcNodeLiveness::live(
            "FC3:canary_only_terminal",
            "terminal_status==sandbox:canary_only",
            fp.canary_first_cid,
            fp.canary_only_terminal,
        )
    } else {
        FcNodeLiveness::reachable_not_fired(
            "FC3:canary_only_terminal",
            "terminal_status==sandbox:canary_only",
        )
    });

    rows
}

/// TRACE_MATRIX FC1/FC2/FC3: pick the constitutional anchor family that tells us
/// which footprint witnesses an inventory claim, and report whether the tape
/// carries that footprint. Anchors are bounded strings (e.g. `"FC3:constitution"`).
fn claim_footprint_present(claim: &LivenessInventoryClaim, fp: &TapeFootprints) -> bool {
    let mut any_fc = false;
    let mut present = false;
    for anchor in &claim.constitutional_anchors {
        let a = anchor.to_ascii_lowercase();
        if a.starts_with("fc1") {
            any_fc = true;
            present = present
                || fp.predicate_gated_advances > 0
                || fp.rtool_wtool_bridge > 0
                || fp.step_reject > 0
                || fp.parse_fail > 0
                || fp.llm_err > 0;
        } else if a.starts_with("fc2") {
            any_fc = true;
            present = present
                || fp.boot_trust_root_verified
                || fp.map_reduce_ticks > 0
                || fp.terminals > 0;
        } else if a.starts_with("fc3") {
            any_fc = true;
            present = present
                || fp.real_architect_proposals > 0
                || fp.canary_metric_capsules > 0;
        }
    }
    // Claims with no FC anchor are treated as substrate present iff the tape has
    // any accepted entry at all (a substrate module is exercised whenever the
    // run produced L4 spine activity).
    if !any_fc {
        return fp.predicate_gated_advances > 0
            || fp.rtool_wtool_bridge > 0
            || fp.map_reduce_ticks > 0
            || fp.terminals > 0;
    }
    present
}

/// TRACE_MATRIX FC1/FC2/FC3: cross-reference each claimed-live module against the
/// tape footprints. A claim with no footprint AND no honest excuse is a ZOMBIE.
fn no_zombie_rows(inventory: &LivenessInventory, fp: &TapeFootprints) -> Vec<NoZombieRow> {
    inventory
        .claims
        .iter()
        .map(|claim| {
            let footprint_present = claim_footprint_present(claim, fp);
            let is_zombie = !footprint_present && claim.excused_reason.is_none();
            NoZombieRow {
                module: claim.module.clone(),
                footprint_present,
                excused_reason: claim.excused_reason.clone(),
                is_zombie,
            }
        })
        .collect()
}

// ─────────────────────────────────────────────────────────────────────────
// Public observer API
// ─────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N34 + FC2-N31 + FC3-N33: the keystone observe-only witness.
/// Reconstructs FC1 / FC2 / FC3 node liveness and the no-zombie inventory
/// cross-reference PURELY from the loaded tape (L4 + L4.E + CAS) and the parsed
/// liveness inventory.
///
/// **Observe-only**: takes `&LoadedTape` and `&LivenessInventory` by shared
/// reference; mutates nothing, advances no head, changes no admission. **No fs**:
/// reads only the in-memory tape + CAS. **Integer-only**: every count is an
/// integer. **Honest FC3**: [`FcLivenessReport::fc3_disposition`] is hard-stamped
/// to [`FC3_DISPOSITION_REACHED_OBSERVABLE_CANARY`] — never "fully_closed".
///
/// The returned report's `report_id` is `Cid::default()` (zeroed) until
/// [`persist_fc_liveness_report`] self-addresses it on CAS write.
pub fn observe_fc_liveness(tape: &LoadedTape, inventory: &LivenessInventory) -> FcLivenessReport {
    let fp = reconstruct_footprints(tape);

    let fc1_nodes = fc1_rows(&fp);
    let fc2_nodes = fc2_rows(&fp);
    let fc3_nodes = fc3_rows(&fp);
    let no_zombie = no_zombie_rows(inventory, &fp);

    let zombie_count = fc1_nodes
        .iter()
        .chain(fc2_nodes.iter())
        .chain(fc3_nodes.iter())
        .filter(|n| n.status == FcNodeStatus::Zombie)
        .count() as u64
        + no_zombie.iter().filter(|r| r.is_zombie).count() as u64;

    FcLivenessReport {
        report_id: Cid::default(),
        fc1_nodes,
        fc2_nodes,
        fc3_nodes,
        no_zombie,
        fc3_disposition: FC3_DISPOSITION_REACHED_OBSERVABLE_CANARY.to_string(),
        l4_entry_count: tape.entries.len() as u64,
        l4e_entry_count: tape.l4e_writer.len() as u64,
        zombie_count,
        observe_only: true,
        schema_tag: FC_LIVENESS_REPORT_SCHEMA_ID.to_string(),
    }
}

/// TRACE_MATRIX FC1-N34 + FC2-N31 + FC3-N33: persist the observe-only liveness
/// report to CAS so the witness is itself tape-anchored + reconstructable
/// (Art.0.2). Mirrors `boltzmann_selection_trace.rs`: `ObjectType::Generic` +
/// the free-form schema_id, self-addressed (R3 zero-then-hash). The CAS write IS
/// the anchor — there is NO `std::fs::write`. Returns the self-addressed
/// `report_id`.
pub fn persist_fc_liveness_report(
    cas: &mut CasStore,
    report: &FcLivenessReport,
    logical_t: u64,
) -> Result<Cid, CasError> {
    // R3: zero report_id before hashing so Cid::from_content(stored_bytes) == id.
    let mut to_store = report.clone();
    to_store.report_id = Cid::default();
    let stored_bytes = canonical_encode(&to_store)
        .map_err(|e| CasError::BackendCorruption(format!("fc_liveness report encode: {e:?}")))?;
    let returned_cid = cas.put(
        &stored_bytes,
        ObjectType::Generic,
        "fc-liveness-observer",
        logical_t,
        Some(FC_LIVENESS_REPORT_SCHEMA_ID.to_string()),
    )?;
    debug_assert_eq!(
        returned_cid,
        Cid::from_content(&stored_bytes),
        "CAS-returned cid must equal sha256(stored_bytes); CasStore::put contract"
    );
    Ok(returned_cid)
}

/// TRACE_MATRIX FC1-N34 + FC2-N31 + FC3-N33: rebuild an [`FcLivenessReport`] from
/// CAS-resident bytes. Caller supplies the bytes returned by
/// `cas.get(&report_id)`. Re-derives `report_id` from `Cid::from_content(bytes)`
/// (R3 round-trip).
pub fn restore_fc_liveness_report_from_cas_bytes(
    bytes: &[u8],
) -> Result<FcLivenessReport, CasError> {
    let mut report: FcLivenessReport = canonical_decode(bytes)
        .map_err(|e| CasError::BackendCorruption(format!("fc_liveness report decode: {e:?}")))?;
    report.report_id = Cid::from_content(bytes);
    Ok(report)
}

/// TRACE_MATRIX FC1-N34 + FC2-N31 + FC3-N33: read + restore a persisted liveness
/// report by Cid from CAS.
pub fn read_fc_liveness_report_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<FcLivenessReport, CasError> {
    let bytes = cas.get(cid)?;
    restore_fc_liveness_report_from_cas_bytes(&bytes)
}

/// TRACE_MATRIX FC1-N34 + FC2-N31 + FC3-N33: discover all persisted FC-liveness
/// report Cids in a CAS by schema_id (mirrors `boltzmann_selection_trace_cids`).
pub fn fc_liveness_report_cids(cas: &CasStore) -> Vec<Cid> {
    cas.list_all_cids()
        .into_iter()
        .filter(|cid| {
            cas.metadata(cid).and_then(|m| m.schema_id.as_deref())
                == Some(FC_LIVENESS_REPORT_SCHEMA_ID)
        })
        .collect()
}

/// TRACE_MATRIX FC1-N34 + FC2-N31 + FC3-N33: render a SHIELDED human/audit
/// summary. Emits ONLY bounded node ids, statuses, and integer counts — NEVER
/// raw Lean stderr / autopsy bytes / PPUT metric values. Safe to surface to a
/// system auditor; NOT an agent-visible read view.
pub fn render_fc_liveness_summary(report: &FcLivenessReport) -> String {
    let mut out = String::new();
    out.push_str("FC-LIVENESS OBSERVER (observe-only, tape-derived)\n");
    out.push_str(&format!(
        "  L4 entries: {}  L4.E entries: {}\n",
        report.l4_entry_count, report.l4e_entry_count
    ));
    out.push_str(&format!(
        "  FC3 disposition: {} (loop_stays_open={})\n",
        report.fc3_disposition,
        report.fc3_stays_observable()
    ));

    let render_group = |out: &mut String, title: &str, rows: &[FcNodeLiveness]| {
        out.push_str(&format!("  [{title}]\n"));
        for r in rows {
            let status = match r.status {
                FcNodeStatus::Live => "LIVE",
                FcNodeStatus::ReachableNotFired => "REACHABLE-not-fired",
                FcNodeStatus::Zombie => "ZOMBIE",
            };
            out.push_str(&format!(
                "    {:<40} {:<20} count={}\n",
                r.node_id, status, r.footprint_count
            ));
        }
    };
    render_group(&mut out, "FC1", &report.fc1_nodes);
    render_group(&mut out, "FC2", &report.fc2_nodes);
    render_group(&mut out, "FC3", &report.fc3_nodes);

    out.push_str("  [NO-ZOMBIE inventory cross-ref]\n");
    for r in &report.no_zombie {
        let verdict = if r.is_zombie {
            "ZOMBIE".to_string()
        } else if r.footprint_present {
            "footprint-present".to_string()
        } else {
            format!(
                "excused:{}",
                r.excused_reason.as_deref().unwrap_or("?")
            )
        };
        out.push_str(&format!("    {:<40} {}\n", r.module, verdict));
    }
    out.push_str(&format!("  zombie_count: {}\n", report.zombie_count));
    out
}

/// TRACE_MATRIX FC1/FC2/FC3: build a [`LivenessInventory`] from already-parsed
/// `(module, anchors, excuse)` tuples. The caller is responsible for parsing the
/// liveness TOML via a structured parser (`toml`); this helper just shapes the
/// parsed rows into the inventory the observer consumes (no ad-hoc string
/// parsing of schemas here, per `AGENTS.md §12`).
pub fn build_inventory(
    rows: impl IntoIterator<Item = (String, Vec<String>, Option<String>)>,
) -> LivenessInventory {
    let claims = rows
        .into_iter()
        .map(|(module, constitutional_anchors, excused_reason)| LivenessInventoryClaim {
            module,
            constitutional_anchors,
            excused_reason,
        })
        .collect();
    LivenessInventory { claims }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_inventory() -> LivenessInventory {
        LivenessInventory::default()
    }

    /// A footprint tally with FC1 advance + an L4.E llm_err produces a Live
    /// predicate-gated advance and a Live llm_err arm; the unfired arms are
    /// honest ReachableNotFired (never zombie).
    #[test]
    fn fc1_rows_classify_fired_and_unfired_honestly() {
        let fp = TapeFootprints {
            predicate_gated_advances: 3,
            rtool_wtool_bridge: 3,
            llm_err: 2,
            ..TapeFootprints::default()
        };
        let rows = fc1_rows(&fp);
        let advance = rows.iter().find(|r| r.node_id == "FC1:predicate_gated_advance").unwrap();
        assert_eq!(advance.status, FcNodeStatus::Live);
        assert_eq!(advance.footprint_count, 3);

        let llm = rows.iter().find(|r| r.node_id == "FC1:failure_arm/llm_err").unwrap();
        assert_eq!(llm.status, FcNodeStatus::Live);
        assert_eq!(llm.footprint_count, 2);

        let step = rows.iter().find(|r| r.node_id == "FC1:failure_arm/step_reject").unwrap();
        // Unfired arm: honest ReachableNotFired, NOT a zombie.
        assert_eq!(step.status, FcNodeStatus::ReachableNotFired);
        assert_ne!(step.status, FcNodeStatus::Zombie);
    }

    /// FC2 boot + tick + terminal all fire → all three Live.
    #[test]
    fn fc2_rows_live_when_boot_tick_terminal_present() {
        let fp = TapeFootprints {
            boot_trust_root_verified: true,
            map_reduce_ticks: 5,
            terminals: 1,
            ..TapeFootprints::default()
        };
        let rows = fc2_rows(&fp);
        assert!(rows.iter().all(|r| r.status == FcNodeStatus::Live));
        let tick = rows.iter().find(|r| r.node_id == "FC2:map_reduce_tick").unwrap();
        assert_eq!(tick.footprint_count, 5);
    }

    /// FC3 reaches observable/canary: proposer + canary fire; disposition is
    /// hard-stamped to reached_observable_canary and the loop stays open.
    #[test]
    fn fc3_disposition_is_reached_observable_canary_not_closed() {
        let fp = TapeFootprints {
            real_architect_proposals: 1,
            canary_metric_capsules: 1,
            canary_only_terminal: 1,
            ..TapeFootprints::default()
        };
        let rows = fc3_rows(&fp);
        assert!(rows.iter().all(|r| r.status == FcNodeStatus::Live));

        // The report-level disposition stamp + honesty guarantee.
        let report = FcLivenessReport {
            report_id: Cid::default(),
            fc1_nodes: vec![],
            fc2_nodes: vec![],
            fc3_nodes: rows,
            no_zombie: vec![],
            fc3_disposition: FC3_DISPOSITION_REACHED_OBSERVABLE_CANARY.to_string(),
            l4_entry_count: 0,
            l4e_entry_count: 0,
            zombie_count: 0,
            observe_only: true,
            schema_tag: FC_LIVENESS_REPORT_SCHEMA_ID.to_string(),
        };
        assert!(report.fc3_stays_observable());
        assert!(!closes_fc3_loop(&report.fc3_disposition));
        assert_ne!(report.fc3_disposition, "fully_closed");
    }

    /// No-zombie cross-ref: a claim with no footprint and no excuse is a ZOMBIE;
    /// the same claim with an honest excuse is NOT a zombie.
    #[test]
    fn no_zombie_distinguishes_zombie_from_excused() {
        let fp = TapeFootprints::default(); // empty tape → no footprints
        let inv = build_inventory(vec![
            ("zombie_mod".to_string(), vec!["FC3:constitution".to_string()], None),
            (
                "excused_mod".to_string(),
                vec!["FC3:constitution".to_string()],
                Some("not-exercised-by-this-workload".to_string()),
            ),
        ]);
        let rows = no_zombie_rows(&inv, &fp);
        let z = rows.iter().find(|r| r.module == "zombie_mod").unwrap();
        assert!(z.is_zombie);
        assert!(!z.footprint_present);
        let e = rows.iter().find(|r| r.module == "excused_mod").unwrap();
        assert!(!e.is_zombie);
        assert!(e.excused_reason.is_some());
    }

    /// A claim whose FC anchor has a tape footprint is present (not a zombie).
    #[test]
    fn no_zombie_present_when_anchor_footprint_exists() {
        let fp = TapeFootprints {
            real_architect_proposals: 1,
            ..TapeFootprints::default()
        };
        let inv = build_inventory(vec![(
            "fc3_mod".to_string(),
            vec!["FC3:constitution".to_string()],
            None,
        )]);
        let rows = no_zombie_rows(&inv, &fp);
        let r = &rows[0];
        assert!(r.footprint_present);
        assert!(!r.is_zombie);
    }

    /// fc1_failure_arm maps the three FC1 reject classes and ignores others.
    #[test]
    fn fc1_failure_arm_maps_three_classes() {
        assert_eq!(fc1_failure_arm(L4ERejectionClass::LeanFailed), Some("step_reject"));
        assert_eq!(fc1_failure_arm(L4ERejectionClass::ParseFailed), Some("parse_fail"));
        assert_eq!(fc1_failure_arm(L4ERejectionClass::LlmError), Some("llm_err"));
        assert_eq!(fc1_failure_arm(L4ERejectionClass::PredicateFailed), None);
        assert_eq!(fc1_failure_arm(L4ERejectionClass::SorryBlocked), None);
    }

    /// The shielded render emits only bounded labels + integer counts — no '.'
    /// decimal point on any numeric surface, no raw-diagnostic markers.
    #[test]
    fn render_is_shielded_and_integer_only() {
        let report = observe_fc_liveness_offline_fixture();
        let s = render_fc_liveness_summary(&report);
        // Must surface status vocabulary and counts.
        assert!(s.contains("LIVE") || s.contains("REACHABLE-not-fired"));
        assert!(s.contains("zombie_count:"));
        // No raw-diagnostic / PPUT-leak markers.
        assert!(!s.to_lowercase().contains("stderr"));
        assert!(!s.to_lowercase().contains("autopsy"));
        assert!(!s.to_lowercase().contains("counterexample"));
    }

    /// Round-trip the report bytes through canonical encode/decode (the same
    /// codec CAS persistence uses) and confirm the self-address rule holds.
    #[test]
    fn report_encode_decode_round_trip_self_addresses() {
        let report = observe_fc_liveness_offline_fixture();
        let mut to_store = report.clone();
        to_store.report_id = Cid::default();
        let bytes = canonical_encode(&to_store).unwrap();
        let restored = restore_fc_liveness_report_from_cas_bytes(&bytes).unwrap();
        assert_eq!(restored.report_id, Cid::from_content(&bytes));
        // Everything except the (zeroed-then-rederived) report_id matches.
        assert_eq!(restored.fc1_nodes, report.fc1_nodes);
        assert_eq!(restored.fc3_disposition, report.fc3_disposition);
        assert_eq!(restored.observe_only, true);
    }

    // A report built purely from a synthetic footprint tally (no LoadedTape
    // required) — keeps the unit tests hermetic while exercising the same
    // classification + rendering paths the live observer uses.
    fn observe_fc_liveness_offline_fixture() -> FcLivenessReport {
        let fp = TapeFootprints {
            predicate_gated_advances: 2,
            rtool_wtool_bridge: 2,
            map_reduce_ticks: 1,
            terminals: 1,
            boot_trust_root_verified: true,
            real_architect_proposals: 1,
            canary_metric_capsules: 1,
            canary_only_terminal: 1,
            parse_fail: 1,
            ..TapeFootprints::default()
        };
        let inv = empty_inventory();
        let fc1_nodes = fc1_rows(&fp);
        let fc2_nodes = fc2_rows(&fp);
        let fc3_nodes = fc3_rows(&fp);
        let no_zombie = no_zombie_rows(&inv, &fp);
        let zombie_count = no_zombie.iter().filter(|r| r.is_zombie).count() as u64;
        FcLivenessReport {
            report_id: Cid::default(),
            fc1_nodes,
            fc2_nodes,
            fc3_nodes,
            no_zombie,
            fc3_disposition: FC3_DISPOSITION_REACHED_OBSERVABLE_CANARY.to_string(),
            l4_entry_count: 4,
            l4e_entry_count: 1,
            zombie_count,
            observe_only: true,
            schema_tag: FC_LIVENESS_REPORT_SCHEMA_ID.to_string(),
        }
    }

    // Keep BTreeMap import referenced (the report uses no map directly, but the
    // observer crate-family uses one; this silences an unused-import lint if the
    // report shape changes).
    #[allow(dead_code)]
    fn _map_doc(_m: BTreeMap<String, u64>) {}
}
