//! K-1.1 shared test Harness for Shape B+D constitution_*.rs files.
//!
//! Per K-1.6 audit (handover/architect-insights/K-1-6_HARNESS_SHAPE_AUDIT.md):
//! 18 of 128 constitution_*.rs files (6 Shape B + 12 Shape D) re-implement the
//! same ~200 LOC of Sequencer + CasStore + RejectionEvidenceWriter setup. This
//! module extracts that shared pattern as one struct + one constructor.
//!
//! Karpathy intent (per skills/KARPATHY_ARCHITECT.md + A2 review): one file, one
//! struct, one constructor. Transparent data flow. No Manager/Factory/Engine
//! abstraction.
//!
//! Usage in a constitution_*.rs test file:
//!
//! ```ignore
//! mod support;
//! use support::{Harness, fresh_harness};
//!
//! #[test]
//! fn my_test() {
//!     let mut h = fresh_harness(QState::default());
//!     // h.seq is the Sequencer; h.rx is the SubmissionEnvelope receiver.
//! }
//! ```
//!
//! Shape F files (110 of 128) do NOT need this — they are source-grep tests
//! that read `src/` files and parse/scan; they instantiate no Sequencer.

#![allow(dead_code)]

use std::sync::{Arc, RwLock};

use tempfile::TempDir;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{InMemoryLedgerWriter, LedgerWriter};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::state::q_state::QState;
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

pub struct Harness {
    pub _tmp: TempDir,
    pub seq: Sequencer,
    pub rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    pub _ledger: Arc<RwLock<dyn LedgerWriter>>,
}

pub fn fresh_harness(initial_q: QState) -> Harness {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("kp"));
    let writer: Arc<RwLock<dyn LedgerWriter>> = Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
    let preds = Arc::new(
        PredicateRegistry::from_boot_manifest(
            turingosv4::top_white::predicates::registry::BootPredicateManifest::empty(),
        )
        .expect("empty predicate manifest"),
    );
    let tools = Arc::new(ToolRegistry::new());
    let epoch = SystemEpoch::new(1);
    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(epoch, keypair.public_key());
    let pinned_pubkeys = Arc::new(pinned);
    let (seq, rx) = Sequencer::new(
        cas,
        keypair,
        epoch,
        writer.clone(),
        rejection_writer,
        preds,
        tools,
        pinned_pubkeys,
        initial_q,
        16,
    );
    Harness {
        _tmp: tmp,
        seq,
        rx,
        _ledger: writer,
    }
}

// ── OBS_AGENT_SIG_REPLAY_GAP closure: fail-closed agent-signature test helpers ─
//
// §8 token APPROVE-AGENT-SIG-INGRESS-FAILCLOSED-ALL-12 made `submit_agent_tx`
// fail-closed for every signable ECONOMIC agent variant. Tests that previously
// submitted zero/placeholder-signed economic txs through an unconfigured
// sequencer now hit `AgentManifestRequired` / `AgentSignatureInvalid`. These
// helpers let a test (1) build a deterministic agent-pubkey manifest and pin it
// on the sequencer, and (2) re-sign any economic TypedTx by its signer field
// with the matching deterministic key, so the live ingress path admits it.
//
// Deterministic keys keep tests reproducible; never used in production.

use turingosv4::runtime::agent_keypairs::{AgentKeypair, AgentKeypairRegistry, AgentPubkeyManifest};
use turingosv4::state::q_state::AgentId;
use turingosv4::state::typed_tx::{AgentSignature, TypedTx};

/// Deterministic per-agent test keypair (seed = first 32 bytes of the agent
/// id, zero-padded). Stable across runs.
pub fn deterministic_agent_keypair(agent: &str) -> AgentKeypair {
    let mut seed = [0u8; 32];
    let bytes = agent.as_bytes();
    let n = bytes.len().min(32);
    seed[..n].copy_from_slice(&bytes[..n]);
    AgentKeypair::from_secret_bytes(seed)
}

/// Build an `AgentPubkeyManifest` over the named agents using deterministic keys.
pub fn manifest_for(agents: &[&str]) -> AgentPubkeyManifest {
    let mut m = AgentPubkeyManifest::default();
    for a in agents {
        m.agents.insert(
            (*a).to_string(),
            deterministic_agent_keypair(a).public_key().to_hex(),
        );
    }
    m
}

/// The signer `AgentId` for a signable economic variant (None for non-signable
/// variants like Reuse / system-emitted).
pub fn economic_signer(tx: &TypedTx) -> Option<AgentId> {
    match tx {
        TypedTx::Work(t) => Some(t.agent_id.clone()),
        TypedTx::Verify(t) => Some(t.verifier_agent.clone()),
        TypedTx::Challenge(t) => Some(t.challenger_agent.clone()),
        TypedTx::TaskOpen(t) => Some(t.sponsor_agent.clone()),
        TypedTx::EscrowLock(t) => Some(t.sponsor_agent.clone()),
        TypedTx::CompleteSetMint(t) => Some(t.owner.clone()),
        TypedTx::CompleteSetRedeem(t) => Some(t.owner.clone()),
        TypedTx::MarketSeed(t) => Some(t.provider.clone()),
        TypedTx::CompleteSetMerge(t) => Some(t.owner.clone()),
        TypedTx::CpmmPool(t) => Some(t.provider.clone()),
        TypedTx::CpmmSwap(t) => Some(t.trader.clone()),
        TypedTx::BuyWithCoinRouter(t) => Some(t.buyer.clone()),
        _ => None,
    }
}

/// Re-sign an economic TypedTx by its signer field with the matching
/// deterministic test key (over the canonical signing-payload digest), so the
/// fail-closed ingress gate admits it. Non-signable variants pass through
/// unchanged.
pub fn resign(tx: TypedTx) -> TypedTx {
    let signer = match economic_signer(&tx) {
        Some(s) => s,
        None => return tx,
    };
    let kp = deterministic_agent_keypair(&signer.0);
    macro_rules! r {
        ($t:expr, $ctor:path) => {{
            let mut t = $t;
            let digest = t.to_signing_payload().canonical_digest();
            t.signature = kp.sign_digest(digest).expect("test sign");
            $ctor(t)
        }};
    }
    match tx {
        TypedTx::Work(t) => r!(t, TypedTx::Work),
        TypedTx::Verify(t) => r!(t, TypedTx::Verify),
        TypedTx::Challenge(t) => r!(t, TypedTx::Challenge),
        TypedTx::TaskOpen(t) => r!(t, TypedTx::TaskOpen),
        TypedTx::EscrowLock(t) => r!(t, TypedTx::EscrowLock),
        TypedTx::CompleteSetMint(t) => r!(t, TypedTx::CompleteSetMint),
        TypedTx::CompleteSetRedeem(t) => r!(t, TypedTx::CompleteSetRedeem),
        TypedTx::MarketSeed(t) => r!(t, TypedTx::MarketSeed),
        TypedTx::CompleteSetMerge(t) => r!(t, TypedTx::CompleteSetMerge),
        TypedTx::CpmmPool(t) => r!(t, TypedTx::CpmmPool),
        TypedTx::CpmmSwap(t) => r!(t, TypedTx::CpmmSwap),
        TypedTx::BuyWithCoinRouter(t) => r!(t, TypedTx::BuyWithCoinRouter),
        other => other,
    }
}

/// A generous fixed set of simple agent ids used across the support-based
/// economic constitution tests (CompleteSet / Cpmm / MarketSeed / router /
/// audit-views / polymarket). Pinning a manifest over this set lets those
/// files' `resign`-ed fixtures pass fail-closed ingress without per-file lists.
pub const COMMON_TEST_AGENTS: &[&str] = &[
    "alice", "bob", "carol", "dave", "eve", "frank",
    "owner", "provider", "trader", "buyer", "maker", "taker",
    "sponsor", "solver", "verifier", "challenger", "treasury",
    "bob-overdraft", "trader_no", "trader_yes", "provider-replay", "sponsor-prov",
    "ghost", "mallory", "impostor",
    "forced_bear", "forced_bull", "poor", "robust-partial-redeem",
    "market-maker", "u1", "u2", "g2-5-c", "g2-5-d",
    "real8x-stale-parent", "real8x-verify-stale", "real8x-work",
    "MarketMakerBudget",
    "Agent_0", "Agent_1", "Agent_2", "Agent_3", "Agent_4", "Agent_5", "Agent_a3_4",
    "Trader_0", "Trader_1", "Trader_2", "Trader_3",
    "solver-a3-1", "solver-a3-2", "solver-a3-3",
    "solver-a4-1", "solver-a4-2", "solver-a4-3", "solver-a4-4", "solver-a4-5",
    "sponsor-a3-1", "sponsor-a3-2", "sponsor-a3-3",
    "sponsor-a4-1", "sponsor-a4-2", "sponsor-a4-3", "sponsor-a4-4", "sponsor-a4-5",
    "sponsor-b2-2", "sponsor-b2-3", "sponsor-b2-4", "sponsor-b2-6", "sponsor-b2-7",
    "verifier-a4-1", "verifier-a4-2", "verifier-a4-3", "verifier-a4-4", "verifier-a4-5",
    "predicate-test-agent", "fc3-agent", "flow-live-agent", "flow-live-agent-2",
    "sp-g3-b", "sp-g3-z", "sv-g3-b", "sv-g3-z",
    "flowchart-livenow-sponsor",
    // tb_6 / tb_7 / tb_11 / tb_12 / g1 / g1_2 / tb_18r cluster agents.
    "Agent_phantom", "Agent_real_for_mismatch_test",
    "agent-i90", "agent-i90e", "agent-i91", "agent-i92", "agent-t11", "agent-t12",
    "alice-g1_3", "bob-g1_3",
    "challenger-B", "challenger-C", "challenger-D", "challenger-E", "challenger-F",
    "challenger-G", "challenger-H",
    "solver-1", "solver-B", "solver-C", "solver-D", "solver-E", "solver-F",
    "solver-G", "solver-H", "solver-S",
    "sponsor-A", "sponsor-B", "sponsor-C", "sponsor-D", "sponsor-E", "sponsor-F",
    "sponsor-G", "sponsor-H", "sponsor-S", "sponsor-T",
    "sponsor-g1_2", "sponsor-g1_3", "sponsor-g1_4", "sponsor-g1_5",
    "sponsor-i90", "sponsor-i90c", "sponsor-i90d", "sponsor-i90f",
    "sponsor-i91", "sponsor-i92", "sponsor-t10", "sponsor-t12", "sponsor-t13",
    "verifier-B", "verifier-C", "verifier-D", "verifier-E",
    "g1_2_5-sponsor", "tb7-smoke-sponsor", "test-sponsor",
    "solver-A", "challenger-A", "verifier-A",
    "solver-X", "challenger-X", "sponsor-X", "verifier-X",
    "tb18r-r4-sponsor", "poor_trader",
];

/// Pin the COMMON_TEST_AGENTS deterministic manifest on `seq` (idempotent: a
/// no-op if a manifest is already set). Call right after building a sequencer
/// in a support-based economic test so `resign`-ed fixtures admit through the
/// fail-closed ingress gate.
pub fn pin_common_manifest(seq: &turingosv4::state::sequencer::Sequencer) {
    // Ignore the Err arm (manifest already set) — idempotent for tests that
    // construct multiple harnesses or already pinned their own manifest.
    let _ = seq.set_agent_pubkeys(std::sync::Arc::new(manifest_for(COMMON_TEST_AGENTS)));
}

/// Pin a manifest that MERGES the COMMON deterministic agents with a real
/// `AgentKeypairRegistry`'s manifest (idempotent). Use in tests that submit
/// BOTH synthetic `resign`-ed txs (deterministic keys → COMMON) AND
/// real-registry-signed txs (reg's keys → reg.manifest()). Agent-id collisions
/// resolve to the reg entry (inserted last). Idempotent: ignores the Err arm if
/// a manifest is already pinned.
pub fn pin_merged_manifest(
    seq: &turingosv4::state::sequencer::Sequencer,
    reg: &AgentKeypairRegistry,
) {
    let mut m = manifest_for(COMMON_TEST_AGENTS);
    for (id, pk) in reg.manifest().agents {
        m.agents.insert(id, pk);
    }
    let _ = seq.set_agent_pubkeys(std::sync::Arc::new(m));
}

/// Suppress dead-code warnings: a helper var so `AgentSignature` import stays
/// used even in files that only call `resign`.
#[allow(dead_code)]
fn _failclosed_helpers_used() {
    let _ = AgentSignature::from_bytes([0u8; 64]);
}
