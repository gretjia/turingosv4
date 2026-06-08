//! LIVE-FC1 — FC-liveness OBSERVER: non-vacuous, mutation-proven constitution gate.
//!
//! Proves the keystone self-verification witness
//! `runtime::agent_scheduler::fc_liveness_observer::observe_fc_liveness` actually
//! RECONSTRUCTS FC1/FC2/FC3 node liveness from the canonical tape (L4 accepted
//! spine + L4.E rejection chain + CAS payloads) and nothing else. The four
//! coupled properties this gate locks:
//!
//!   1. **RECONSTRUCTS ALL FC** — a realistic `LoadedTape` + CAS carrying
//!      FC1 (a predicate-gated `Work` advance + >=1 each of
//!      step_reject/parse_fail/llm_err L4.E rejection rows), FC2 (boot
//!      trust-root verified + a `MapReduceTick` + a `TerminalSummary`), and
//!      FC3-observable (a REAL non-Noop `ArchitectProposal` + a canary
//!      `MetricEstimateCapsule` whose terminal == `sandbox:canary_only`) makes
//!      `observe_fc_liveness` mark EACH expected node `Live` with the right
//!      bounded evidence label.
//!
//!   2. **NO-ZOMBIE** — a module in the inventory with NO tape footprint and NO
//!      honest excuse is flagged `Zombie`; the same shape WITH an honest excuse
//!      is NOT a zombie (the excuse is the sole discriminator).
//!
//!   3. **HONESTY (anti-overclaim)** — `report.fc3_disposition ==
//!      "reached_observable_canary"`, NO field/string anywhere in the rendered
//!      report claims FC3 "fully closed / live self-modification", and a positive
//!      control tape with NO FC3 events leaves the FC3 nodes
//!      `ReachableNotFired` (never `Live`, never `Zombie` when the inventory
//!      honestly excuses them).
//!
//!   4. **OBSERVE-ONLY** — the tape bytes (every L4 entry's canonical payload, the
//!      L4.E chain hash, the CAS object set) and the genesis QState are
//!      byte-unchanged after `observe_fc_liveness`, and the function returns ONLY
//!      a report value (no head / no `QState` handle in its type).
//!
//! Plus a mutation witness: removing one FC node's events from the fixture tape
//! makes the observer mark that node NOT-`Live`, flipping a dedicated assert RED
//! (proving the gate is non-vacuous). The mutation is performed in-test against a
//! mutated copy of the fixture; the canonical fixture stays green.
//!
//! Every assertion is built to FAIL if the corresponding property breaks (a node
//! silently downgraded to `ReachableNotFired`, a zombie silently excused, the FC3
//! disposition silently promoted to a loop-closing terminal, or the observer
//! mutating the tape it reads).

use std::collections::BTreeMap;

use sha2::{Digest, Sha256};
use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    RejectionClass, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::system_keypair::{
    PinnedSystemPubkeys, SystemEpoch, SystemSignature,
};
use turingosv4::bottom_white::ledger::transition_ledger::{canonical_encode, LedgerEntry, TxKind};
use turingosv4::runtime::agent_keypairs::AgentPubkeyManifest;
use turingosv4::runtime::agent_scheduler::fc_liveness_observer::{
    build_inventory, observe_fc_liveness, persist_fc_liveness_report,
    read_fc_liveness_report_from_cas, render_fc_liveness_summary, FcNodeStatus,
    FC3_DISPOSITION_REACHED_OBSERVABLE_CANARY,
};
use turingosv4::runtime::audit_assertions::LoadedTape;
use turingosv4::runtime::real5_roles::fc3_canary::{
    MetricEstimateCapsule, CANARY_ONLY_TERMINAL_STATUS, FC3_METRIC_ESTIMATE_SCHEMA_ID,
};
use turingosv4::runtime::real5_roles::MetricEstimate;
use turingosv4::runtime::PinnedPubkeyManifest;
use turingosv4::state::q_state::{AgentId, Hash, QState};
use turingosv4::state::typed_tx::{
    ArchitectProposalCapsule, ArchitectProposalKind, ArchitectProposalTx, MetaRoleMode,
    PredicateId, TypedTx,
};

// ─────────────────────────────────────────────────────────────────────────
// Fixture construction — a realistic LoadedTape + real CAS, hand-built so the
// gate exercises the SAME L4 / L4.E / CAS reconstruction paths the live
// observer uses, without needing a full booted runtime.
// ─────────────────────────────────────────────────────────────────────────

fn sha256_hash(bytes: &[u8]) -> Hash {
    let mut h = Sha256::new();
    h.update(bytes);
    Hash(h.finalize().into())
}

fn hex_of(h: &Hash) -> String {
    h.0.iter().map(|b| format!("{b:02x}")).collect::<String>()
}

/// A minimal accepted L4 entry of a given kind whose `tx_payload_cid` points at
/// `payload_cid`. The non-load-bearing fields (roots, sig) are zeroed — the
/// observer keys only on `tx_kind` + `tx_payload_cid`.
fn ledger_entry(logical_t: u64, kind: TxKind, payload_cid: Cid) -> LedgerEntry {
    LedgerEntry {
        logical_t,
        parent_state_root: Hash::ZERO,
        parent_ledger_root: Hash::ZERO,
        tx_kind: kind,
        tx_payload_cid: payload_cid,
        resulting_state_root: Hash::ZERO,
        resulting_ledger_root: Hash::ZERO,
        timestamp_logical: logical_t,
        epoch: SystemEpoch::new(0),
        extensions: BTreeMap::new(),
        system_signature: SystemSignature::default(),
    }
}

/// Put canonical-encoded `TypedTx` / capsule bytes into CAS and return the Cid.
fn put<T: serde::Serialize>(cas: &mut CasStore, value: &T, schema_id: Option<&str>) -> Cid {
    let bytes = canonical_encode(value).expect("canonical_encode fixture object");
    cas.put(
        &bytes,
        ObjectType::Generic,
        "fixture",
        0,
        schema_id.map(|s| s.to_string()),
    )
    .expect("cas put fixture object")
}

/// Write a REAL (non-Noop) ArchitectProposal: store the capsule, store the
/// `TypedTx::ArchitectProposal` that points at it, and return an L4 entry whose
/// `tx_payload_cid` is the proposal tx. This is exactly the shape
/// `architect_proposal_is_real` walks (tx → capsule → proposal_kind != Noop).
fn real_architect_proposal_entry(cas: &mut CasStore, logical_t: u64) -> LedgerEntry {
    let capsule = ArchitectProposalCapsule {
        schema_version: "v1".to_string(),
        feedback_tx_id: Default::default(),
        feedback_root: Hash::ZERO,
        constitution_hash: Hash::ZERO,
        tool_registry_root: Hash::ZERO,
        // NON-Noop: this is the discriminator the observer keys on.
        proposal_kind: ArchitectProposalKind::ToolRegistryPatch,
        target_path: Some("src/runtime/example_tool.rs".to_string()),
        proposed_artifact_cid: None,
        tools_used: vec!["grep".to_string(), "read".to_string()],
        public_summary: "tighten the tool registry admission check".to_string(),
    };
    let capsule_cid = put(cas, &capsule, None);

    let tx = TypedTx::ArchitectProposal(ArchitectProposalTx {
        tx_id: Default::default(),
        parent_state_root: Hash::ZERO,
        feedback_tx_id: Default::default(),
        feedback_root: Hash::ZERO,
        proposal_capsule_cid: capsule_cid,
        proposal_root: Hash::ZERO,
        constitution_hash: Hash::ZERO,
        tool_registry_root: Hash::ZERO,
        role_mode: MetaRoleMode::Runtime,
        epoch: SystemEpoch::new(0),
        timestamp_logical: logical_t,
        system_signature: SystemSignature::default(),
    });
    let tx_cid = put(cas, &tx, None);
    ledger_entry(logical_t, TxKind::ArchitectProposal, tx_cid)
}

/// Write a canary `MetricEstimateCapsule` (terminal == sandbox:canary_only) to
/// CAS under the FC3 metric schema_id, so the observer discovers it by schema_id
/// and structurally confirms the terminal. Integer-only metric.
fn write_canary_metric(cas: &mut CasStore) -> Cid {
    let capsule = MetricEstimateCapsule {
        schema_version: "v1".to_string(),
        candidate_artifact_cid: Cid::default(),
        predicate_id: PredicateId("predicate:lean_verify_v1".to_string()),
        predicate_code_hash: [7u8; 32],
        registry_root: Hash::ZERO,
        predicate_passed: true,
        metric: MetricEstimate {
            metric: "proved_fraction".to_string(),
            numerator_delta: 1,
            denominator: 4,
        },
        terminal_status: CANARY_ONLY_TERMINAL_STATUS.to_string(),
    };
    put(cas, &capsule, Some(FC3_METRIC_ESTIMATE_SCHEMA_ID))
}

/// What an FC fixture controls, so the mutation witness can drop one FC node's
/// events while keeping the rest intact.
#[derive(Clone, Copy)]
struct FixtureSpec {
    fc1_advance: bool,
    fc1_step_reject: bool,
    fc1_parse_fail: bool,
    fc1_llm_err: bool,
    fc2_boot_trust_root: bool,
    fc2_map_reduce_tick: bool,
    fc2_terminal: bool,
    fc3_proposal: bool,
    fc3_canary: bool,
}

impl FixtureSpec {
    /// The realistic "everything fires" tape (FC1 advance + all three failure
    /// arms, FC2 boot/tick/terminal, FC3 proposal + canary).
    fn full() -> Self {
        Self {
            fc1_advance: true,
            fc1_step_reject: true,
            fc1_parse_fail: true,
            fc1_llm_err: true,
            fc2_boot_trust_root: true,
            fc2_map_reduce_tick: true,
            fc2_terminal: true,
            fc3_proposal: true,
            fc3_canary: true,
        }
    }

    /// A tape with NO FC3 events at all (positive control for HONESTY: FC3 nodes
    /// must come back ReachableNotFired, never Live, never Zombie when excused).
    fn no_fc3() -> Self {
        let mut s = Self::full();
        s.fc3_proposal = false;
        s.fc3_canary = false;
        s
    }
}

/// Build a `LoadedTape` (with a live CAS in `tmp`) per the spec. Returns the tape
/// plus the genesis QState bytes so callers can prove observe-only.
fn build_tape(tmp: &TempDir, spec: FixtureSpec) -> LoadedTape {
    let cas_dir = tmp.path().join("cas");
    std::fs::create_dir_all(&cas_dir).expect("mkdir cas");
    let mut cas = CasStore::open(&cas_dir).expect("open cas");

    let constitution_bytes = b"FIXTURE CONSTITUTION BYTES (LIVE-FC1 gate)".to_vec();
    let constitution_hash = sha256_hash(&constitution_bytes);

    let mut entries: Vec<LedgerEntry> = Vec::new();
    let mut t: u64 = 1;

    if spec.fc1_advance {
        // A predicate-gated advance: an accepted WorkTx whose payload is on CAS.
        let work_cid = put(&mut cas, &"fixture-work-payload", None);
        entries.push(ledger_entry(t, TxKind::Work, work_cid));
        t += 1;
    }
    if spec.fc2_map_reduce_tick {
        let tick_cid = put(&mut cas, &"fixture-map-reduce-tick", None);
        entries.push(ledger_entry(t, TxKind::MapReduceTick, tick_cid));
        t += 1;
    }
    if spec.fc3_proposal {
        let e = real_architect_proposal_entry(&mut cas, t);
        entries.push(e);
        t += 1;
    }
    if spec.fc2_terminal {
        let term_cid = put(&mut cas, &"fixture-terminal-summary", None);
        entries.push(ledger_entry(t, TxKind::TerminalSummary, term_cid));
        t += 1;
    }
    let _ = t;

    if spec.fc3_canary {
        write_canary_metric(&mut cas);
    }

    // L4.E rejection chain — the three FC1 failure arms.
    let mut l4e = RejectionEvidenceWriter::new();
    let mut submit_id: u64 = 1;
    let mut push_reject = |class: RejectionClass| {
        l4e.append_rejected(
            submit_id,
            Hash::ZERO,
            AgentId("agent:fixture".to_string()),
            TxKind::Work,
            Cid::default(),
            class,
            None,
            None,
        );
        submit_id += 1;
    };
    if spec.fc1_step_reject {
        push_reject(RejectionClass::LeanFailed);
    }
    if spec.fc1_parse_fail {
        push_reject(RejectionClass::ParseFailed);
    }
    if spec.fc1_llm_err {
        push_reject(RejectionClass::LlmError);
    }

    let genesis_root_hex = if spec.fc2_boot_trust_root {
        // Boot attestation: genesis [constitution_root] == loaded constitution
        // hash. Matching hex is what the observer reads as "boot trust-root
        // verified" (it does NOT re-run boot — observe only).
        Some(hex_of(&constitution_hash))
    } else {
        None
    };

    LoadedTape {
        runtime_repo: tmp.path().to_path_buf(),
        cas_dir,
        entries,
        l4e_writer: l4e,
        cas,
        pinned: PinnedSystemPubkeys::new(),
        pinned_manifest: PinnedPubkeyManifest {
            run_id: "fixture".to_string(),
            tb_id: "LIVE-FC1".to_string(),
            epoch: 0,
            pubkeys: Vec::new(),
        },
        agent_manifest: AgentPubkeyManifest::default(),
        initial_q: QState::genesis(),
        replayed_q: None,
        replay_error: None,
        constitution_bytes,
        constitution_hash,
        markov_capsule: None,
        genesis_constitution_root_hex: genesis_root_hex,
    }
}

/// Snapshot of every observable tape byte the observer could touch — used to
/// prove observe-only (byte-unchanged + no head advance).
#[derive(PartialEq, Eq, Debug)]
struct TapeByteSnapshot {
    entry_payload_bytes: Vec<Vec<u8>>,
    l4e_chain_hash: [u8; 32],
    l4e_len: usize,
    entry_count: usize,
    cas_cids: Vec<Cid>,
    cas_object_bytes: Vec<Vec<u8>>,
    genesis_q_bytes: Vec<u8>,
}

fn snapshot_tape(tape: &LoadedTape) -> TapeByteSnapshot {
    let mut cas_cids = tape.cas.list_all_cids();
    cas_cids.sort_by(|a, b| a.0.cmp(&b.0));
    let cas_object_bytes = cas_cids
        .iter()
        .map(|c| tape.cas.get(c).expect("cas get for snapshot"))
        .collect();
    TapeByteSnapshot {
        entry_payload_bytes: tape
            .entries
            .iter()
            .map(|e| tape.cas.get(&e.tx_payload_cid).unwrap_or_default())
            .collect(),
        l4e_chain_hash: tape.l4e_writer.last_hash().0,
        l4e_len: tape.l4e_writer.len(),
        entry_count: tape.entries.len(),
        cas_cids,
        cas_object_bytes,
        genesis_q_bytes: canonical_encode(&tape.initial_q).expect("encode genesis q"),
    }
}

// ─────────────────────────────────────────────────────────────────────────
// (1) RECONSTRUCTS ALL FC
// ─────────────────────────────────────────────────────────────────────────

/// A realistic full tape makes the observer mark every FC1/FC2/FC3 node `Live`
/// with the right evidence, reconstructed PURELY from L4 + L4.E + CAS.
#[test]
fn reconstructs_all_fc_nodes_live_from_tape() {
    let tmp = TempDir::new().expect("tmp");
    let tape = build_tape(&tmp, FixtureSpec::full());
    let inventory = build_inventory(std::iter::empty());

    let report = observe_fc_liveness(&tape, &inventory);

    // The tape carried real footprints — counts must be reconstructed, not zero.
    assert_eq!(report.l4_entry_count, tape.entries.len() as u64);
    assert_eq!(
        report.l4e_entry_count, 3,
        "three FC1 failure-arm rejections"
    );

    // --- FC1: predicate-gated advance + all three failure arms + bridge ---
    let advance = node(&report.fc1_nodes, "FC1:predicate_gated_advance");
    assert_eq!(
        advance.status,
        FcNodeStatus::Live,
        "FC1 advance reconstructed Live"
    );
    assert!(
        advance.evidence_cid.is_some(),
        "advance carries a CAS witness cid"
    );
    assert!(advance.footprint_count >= 1);

    assert_eq!(
        node(&report.fc1_nodes, "FC1:failure_arm/step_reject").status,
        FcNodeStatus::Live,
        "step_reject arm reconstructed Live from L4.E LeanFailed"
    );
    assert_eq!(
        node(&report.fc1_nodes, "FC1:failure_arm/parse_fail").status,
        FcNodeStatus::Live,
        "parse_fail arm reconstructed Live from L4.E ParseFailed"
    );
    assert_eq!(
        node(&report.fc1_nodes, "FC1:failure_arm/llm_err").status,
        FcNodeStatus::Live,
        "llm_err arm reconstructed Live from L4.E LlmError"
    );
    assert_eq!(
        node(&report.fc1_nodes, "FC1:rtool_wtool_bridge").status,
        FcNodeStatus::Live,
        "agent typed-write ingress bridge reconstructed Live from accepted WorkTx"
    );

    // --- FC2: boot trust-root + map-reduce-tick + terminal ---
    assert_eq!(
        node(&report.fc2_nodes, "FC2:boot_trust_root_verified").status,
        FcNodeStatus::Live,
        "boot trust-root verified reconstructed from genesis[constitution_root]==hash"
    );
    assert_eq!(
        node(&report.fc2_nodes, "FC2:map_reduce_tick").status,
        FcNodeStatus::Live,
        "map-reduce tick reconstructed Live from MapReduceTickTx"
    );
    assert_eq!(
        node(&report.fc2_nodes, "FC2:terminal_halt").status,
        FcNodeStatus::Live,
        "terminal reconstructed Live from TerminalSummaryTx"
    );

    // --- FC3-observable: real proposal + canary metric + canary-only terminal ---
    let proposer = node(&report.fc3_nodes, "FC3:proposer_architect_proposal");
    assert_eq!(
        proposer.status,
        FcNodeStatus::Live,
        "real (non-Noop) ArchitectProposal reconstructed Live"
    );
    assert!(proposer.evidence_cid.is_some());
    assert_eq!(
        node(&report.fc3_nodes, "FC3:canary_metric_estimate").status,
        FcNodeStatus::Live,
        "canary MetricEstimateCapsule reconstructed Live (discovered by schema_id)"
    );
    assert_eq!(
        node(&report.fc3_nodes, "FC3:canary_only_terminal").status,
        FcNodeStatus::Live,
        "sandbox:canary_only terminal reconstructed Live"
    );

    // No node was zombie on a fully-fired tape.
    assert!(report.no_zombies(), "fully-fired tape has no zombie nodes");

    // The persisted witness round-trips through CAS (Art. 0.2 tape-anchoring).
    let cas_dir = tmp.path().join("cas2");
    std::fs::create_dir_all(&cas_dir).expect("mkdir cas2");
    let mut sink = CasStore::open(&cas_dir).expect("cas2");
    let report_id = persist_fc_liveness_report(&mut sink, &report, 99).expect("persist");
    let back = read_fc_liveness_report_from_cas(&sink, &report_id).expect("read back");
    assert_eq!(back.fc1_nodes, report.fc1_nodes);
    assert_eq!(back.fc3_disposition, report.fc3_disposition);
    // Self-addressing (Art. 0.2 / R3): the restored report_id equals the sha256
    // of the stored CAS bytes.
    let stored_bytes = sink.get(&report_id).expect("stored report bytes");
    assert_eq!(
        back.report_id,
        Cid::from_content(&stored_bytes),
        "report self-addresses on CAS"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// (2) NO-ZOMBIE
// ─────────────────────────────────────────────────────────────────────────

/// A claimed-live module with NO tape footprint and NO excuse is flagged Zombie;
/// an otherwise-identical claim WITH an honest excuse is NOT a zombie. The excuse
/// is the sole discriminator.
#[test]
fn module_without_tape_footprint_is_zombie_unless_honestly_excused() {
    let tmp = TempDir::new().expect("tmp");
    // A tape with NO FC3 footprint, so an FC3-anchored claim has nothing to
    // witness it.
    let tape = build_tape(&tmp, FixtureSpec::no_fc3());

    let inventory = build_inventory(vec![
        // Claims to be live, anchored to FC3, but the tape has no FC3 footprint
        // and there is NO excuse → ZOMBIE.
        (
            "fc3_phantom_module".to_string(),
            vec!["FC3:constitution".to_string()],
            None,
        ),
        // Same anchor, same absent footprint, but an HONEST excuse → not zombie.
        (
            "fc3_excused_module".to_string(),
            vec!["FC3:constitution".to_string()],
            Some("not-exercised-by-this-task-workload".to_string()),
        ),
    ]);

    let report = observe_fc_liveness(&tape, &inventory);

    let zombie = report
        .no_zombie
        .iter()
        .find(|r| r.module == "fc3_phantom_module")
        .expect("zombie row present");
    assert!(zombie.is_zombie, "no footprint + no excuse MUST be Zombie");
    assert!(!zombie.footprint_present);
    assert!(zombie.excused_reason.is_none());

    let excused = report
        .no_zombie
        .iter()
        .find(|r| r.module == "fc3_excused_module")
        .expect("excused row present");
    assert!(
        !excused.is_zombie,
        "no footprint BUT honestly excused MUST NOT be Zombie"
    );
    assert!(excused.excused_reason.is_some());

    // The report-level zombie tally counts exactly the one un-excused phantom.
    assert_eq!(report.zombie_count, 1, "exactly one zombie counted");
    assert!(!report.no_zombies());

    // SOLE-DISCRIMINATOR control: dropping the excuse flips the SAME claim to
    // zombie (proves the excuse, not the module name, is what suppresses it).
    let inventory_no_excuse = build_inventory(vec![(
        "fc3_excused_module".to_string(),
        vec!["FC3:constitution".to_string()],
        None,
    )]);
    let report2 = observe_fc_liveness(&tape, &inventory_no_excuse);
    assert!(
        report2.no_zombie[0].is_zombie,
        "same module, excuse removed → now Zombie (excuse is the sole discriminator)"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// (3) HONESTY (anti-overclaim)
// ─────────────────────────────────────────────────────────────────────────

/// The report stamps FC3 as reached_observable_canary, NEVER claims FC3
/// fully-closed / live self-modification, and stays observable-only.
#[test]
fn fc3_is_reported_reached_observable_canary_never_fully_closed() {
    let tmp = TempDir::new().expect("tmp");
    let tape = build_tape(&tmp, FixtureSpec::full());
    let report = observe_fc_liveness(&tape, &build_inventory(std::iter::empty()));

    // Hard-stamped disposition.
    assert_eq!(
        report.fc3_disposition, FC3_DISPOSITION_REACHED_OBSERVABLE_CANARY,
        "FC3 disposition MUST be reached_observable_canary"
    );
    assert_eq!(report.fc3_disposition, "reached_observable_canary");
    assert!(
        report.fc3_stays_observable(),
        "observer asserts FC3 loop stays open (observable/canary only)"
    );

    // No surface anywhere claims the loop is closed / self-modification is live.
    // Check the structured disposition AND the full rendered summary.
    let rendered = render_fc_liveness_summary(&report);
    let haystacks = [
        report.fc3_disposition.to_lowercase(),
        rendered.to_lowercase(),
    ];
    for h in &haystacks {
        for forbidden in [
            "fully_closed",
            "fully closed",
            "loop_closed",
            "loop closed",
            "self-modification",
            "self_modification",
            "irreversible",
            "committed",
            "reinit",
            "re-init",
        ] {
            assert!(
                !h.contains(forbidden),
                "HONESTY: report MUST NOT claim FC3 `{forbidden}` (found in: {h})"
            );
        }
    }
}

/// POSITIVE CONTROL for honesty: a tape with NO FC3 events leaves the FC3 nodes
/// ReachableNotFired — not Live (no overclaim) and not Zombie when the inventory
/// honestly excuses them.
#[test]
fn no_fc3_events_leaves_fc3_reachable_not_fired_not_live_not_zombie() {
    let tmp = TempDir::new().expect("tmp");
    let tape = build_tape(&tmp, FixtureSpec::no_fc3());

    // Inventory honestly excuses the FC3 module (this workload didn't drive it).
    let inventory = build_inventory(vec![(
        "fc3_meta_loop".to_string(),
        vec!["FC3:meta".to_string()],
        Some("task-workload-does-not-drive-meta-loop".to_string()),
    )]);

    let report = observe_fc_liveness(&tape, &inventory);

    for node_id in [
        "FC3:proposer_architect_proposal",
        "FC3:canary_metric_estimate",
        "FC3:canary_only_terminal",
    ] {
        let n = node(&report.fc3_nodes, node_id);
        assert_eq!(
            n.status,
            FcNodeStatus::ReachableNotFired,
            "{node_id} with no FC3 events MUST be ReachableNotFired (no overclaim Live)"
        );
        assert_ne!(n.status, FcNodeStatus::Live);
        assert_ne!(n.status, FcNodeStatus::Zombie);
    }

    // The honestly-excused inventory module is NOT a zombie even with no FC3
    // footprint, and the disposition is STILL the observable/canary stamp.
    assert!(report.no_zombies(), "excused FC3 module is not a zombie");
    assert_eq!(
        report.fc3_disposition,
        FC3_DISPOSITION_REACHED_OBSERVABLE_CANARY
    );
}

// ─────────────────────────────────────────────────────────────────────────
// (4) OBSERVE-ONLY
// ─────────────────────────────────────────────────────────────────────────

/// `observe_fc_liveness` mutates nothing: the tape bytes (L4 payloads, L4.E
/// chain, CAS object set), the L4.E head length, and the genesis QState are
/// byte-identical before and after; and it returns ONLY a report value.
#[test]
fn observe_fc_liveness_is_byte_unchanged_observe_only() {
    let tmp = TempDir::new().expect("tmp");
    let tape = build_tape(&tmp, FixtureSpec::full());

    let before = snapshot_tape(&tape);

    // `observe_fc_liveness(&tape, &inventory)` takes SHARED refs only — the type
    // system already forbids mutation; this asserts the byte-level invariant too.
    let report = observe_fc_liveness(&tape, &build_inventory(std::iter::empty()));

    let after = snapshot_tape(&tape);
    assert_eq!(
        before, after,
        "OBSERVE-ONLY: tape + CAS + L4.E + genesis QState byte-unchanged after observe"
    );

    // No head advanced: L4 entry count and L4.E length are unchanged (snapshot
    // equality already covers this; assert explicitly for the gate's intent).
    assert_eq!(before.entry_count, after.entry_count, "no L4 head advance");
    assert_eq!(before.l4e_len, after.l4e_len, "no L4.E head advance");
    assert_eq!(
        before.l4e_chain_hash, after.l4e_chain_hash,
        "L4.E chain hash frozen"
    );

    // The function returns ONLY a report — `report.observe_only` is the
    // type-level promise it carries no head / QState handle.
    assert!(report.observe_only, "report is flagged observe_only");
}

// ─────────────────────────────────────────────────────────────────────────
// MUTATION WITNESS — non-vacuity proof
// ─────────────────────────────────────────────────────────────────────────

/// Removing one FC node's events from the fixture tape makes the observer mark
/// that node NOT-Live, and the assertion that the node is Live FLIPS RED. This
/// proves the green asserts above are not vacuous: they actually depend on the
/// tape carrying the node's events.
///
/// We perform the mutation against a SEPARATE tape (drop the MapReduceTick
/// events), observe RED on the "tick is Live" assertion, then re-observe the
/// canonical full fixture and confirm GREEN — the gate is mechanically tied to
/// tape footprints, not to a constant.
#[test]
fn mutation_removing_fc_node_events_flips_live_assertion_red_then_green() {
    let tmp = TempDir::new().expect("tmp");
    let inventory = build_inventory(std::iter::empty());

    // GREEN baseline: the full tape marks FC2 map-reduce-tick Live.
    let full = build_tape(&tmp, FixtureSpec::full());
    let green = observe_fc_liveness(&full, &inventory);
    assert_eq!(
        node(&green.fc2_nodes, "FC2:map_reduce_tick").status,
        FcNodeStatus::Live,
        "baseline: tick is Live on the full tape"
    );

    // MUTATION: drop the MapReduceTick events from the tape.
    let tmp_mut = TempDir::new().expect("tmp_mut");
    let mut spec = FixtureSpec::full();
    spec.fc2_map_reduce_tick = false;
    let mutated = build_tape(&tmp_mut, spec);
    let red = observe_fc_liveness(&mutated, &inventory);

    // The SAME assertion that was green now observes the node is NOT Live —
    // proving non-vacuity. We assert the RED condition holds (status changed).
    let tick = node(&red.fc2_nodes, "FC2:map_reduce_tick");
    assert_ne!(
        tick.status,
        FcNodeStatus::Live,
        "MUTATION: with tick events removed, the node is NOT Live (assert would flip RED)"
    );
    assert_eq!(
        tick.status,
        FcNodeStatus::ReachableNotFired,
        "removed tick → honest ReachableNotFired, witnessing the assert is tape-driven"
    );
    assert_eq!(tick.footprint_count, 0, "no tick footprint after mutation");

    // REVERT to green: the canonical full fixture still marks the node Live.
    let reverted = observe_fc_liveness(&full, &inventory);
    assert_eq!(
        node(&reverted.fc2_nodes, "FC2:map_reduce_tick").status,
        FcNodeStatus::Live,
        "revert: tick is Live again on the unmutated fixture (gate back to GREEN)"
    );

    // Cross-mutation on an FC1 arm for breadth: drop the llm_err rejection row.
    let tmp_mut2 = TempDir::new().expect("tmp_mut2");
    let mut spec2 = FixtureSpec::full();
    spec2.fc1_llm_err = false;
    let mutated2 = build_tape(&tmp_mut2, spec2);
    let red2 = observe_fc_liveness(&mutated2, &inventory);
    assert_ne!(
        node(&red2.fc1_nodes, "FC1:failure_arm/llm_err").status,
        FcNodeStatus::Live,
        "MUTATION: with the llm_err L4.E row removed, that arm is NOT Live"
    );
    assert_eq!(
        red2.l4e_entry_count, 2,
        "one fewer L4.E rejection after mutation"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// helpers
// ─────────────────────────────────────────────────────────────────────────

fn node<'a>(
    rows: &'a [turingosv4::runtime::agent_scheduler::fc_liveness_observer::FcNodeLiveness],
    id: &str,
) -> &'a turingosv4::runtime::agent_scheduler::fc_liveness_observer::FcNodeLiveness {
    rows.iter()
        .find(|r| r.node_id == id)
        .unwrap_or_else(|| panic!("FC node `{id}` must be present in the report"))
}
