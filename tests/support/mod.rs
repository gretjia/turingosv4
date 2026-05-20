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
    let writer: Arc<RwLock<dyn LedgerWriter>> =
        Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
    let preds = Arc::new(PredicateRegistry::new());
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
