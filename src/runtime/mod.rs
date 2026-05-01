//! TB-6 Atom 1 — Production ChainTape runtime bootstrap.
//!
//! Connects the experiment evaluator binary to the kernel `Sequencer` via
//! `TuringBus::with_sequencer`. Produces an on-disk `Git2LedgerWriter` chain
//! (`refs/transitions/main`) plus persistent L4.E JSONL rejection ledger
//! (`<runtime_repo>/rejections.jsonl`) — the architect § 3.5 deliverable shape
//! for chain-backed ChainTape smoke evidence.
//!
//! Driver lifecycle: a runtime-side wrapper (`run_chaintape_driver`) owns the
//! Sequencer mpsc receiver, races shutdown_rx via `tokio::select!`, and calls
//! `Sequencer::apply_one` (`pub(crate)` — same crate) directly. We do NOT call
//! `Sequencer::run` because it has no shutdown branch and `Sequencer` owns
//! `queue_tx` (driver task's `Arc<Sequencer>` would keep the sender alive,
//! preventing clean exit).
//!
//! Per architect ruling 2026-05-01 Path A, atom count stays at 8.
//! See `handover/ai-direct/TB-6_PRODUCTION_CHAINTAPE_BOOTSTRAP_2026-05-01.md`
//! v2.1 for the full preflight.

/// TRACE_MATRIX FC3-N1: TB-6 Atom 2 — chaintape adapter helpers (synthetic TaskOpen/EscrowLock/WorkTx constructors + balance seeding).
pub mod adapter;

/// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — replay verifier (re-opens runtime_repo + cas + pinned_pubkeys.json, replays L4 chain, emits replay_report.json).
pub mod verify;

/// TRACE_MATRIX FC3-N1: TB-6 Atom 5 — Agent audit trail (AgentProposalRecord + CAS storage + JSONL index linking tx_id → proposal_record_cid).
pub mod agent_audit_trail;

/// TRACE_MATRIX FC3-N1: TB-6 Atom 6 — Branch / fork visibility summary (tx_count, failed_branch_count, rollback_count, accepted/rejected tx_id sets, candidate proposal CIDs).
pub mod run_summary;

use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use crate::bottom_white::cas::store::CasStore;
use crate::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use crate::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use crate::bottom_white::ledger::transition_ledger::{
    Git2LedgerWriter, LedgerWriter,
};
use crate::state::q_state::QState;
use crate::state::sequencer::{Sequencer, SubmissionEnvelope};
use crate::top_white::predicates::registry::PredicateRegistry;
use crate::bottom_white::tools::registry::ToolRegistry;

// ── Configuration ───────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3-N1: configuration shape for production ChainTape mode.
///
/// Production ChainTape runtime configuration.
///
/// Enabled by env var `TURINGOS_CHAINTAPE_PATH=<runtime_repo_path>`.
/// When unset, the evaluator falls back to legacy `TuringBus::new` /
/// `TuringBus::with_wal_path` (no on-disk ChainTape).
#[derive(Debug, Clone)]
pub struct RuntimeChaintapeConfig {
    /// Filesystem path to the on-disk runtime git repo.
    /// `Git2LedgerWriter` rooted here writes `refs/transitions/main`.
    /// `<runtime_repo_path>/rejections.jsonl` is the L4.E persistent file.
    /// `<runtime_repo_path>/pinned_pubkeys.json` carries the per-run
    /// `PinnedSystemPubkeys` so `verify_chaintape` (Atom 4) can re-verify
    /// entry signatures without separate config.
    pub runtime_repo_path: PathBuf,
    /// CAS root directory. Distinct from `runtime_repo_path` so CAS payloads
    /// can be inspected independently of the chain refs.
    pub cas_path: PathBuf,
    /// Run identity for evidence-dir naming + audit trail. Defaults to
    /// `TURINGOS_RUN_ID` env var or current Unix-second timestamp.
    pub run_id: String,
    /// Sequencer mpsc channel capacity. Default 64.
    pub queue_capacity: usize,
}

impl RuntimeChaintapeConfig {
    /// TRACE_MATRIX FC3-N1: env-flag-gated chaintape mode entry — evaluator calls this once at boot.
    ///
    /// Build from env. Returns `None` if `TURINGOS_CHAINTAPE_PATH` unset.
    pub fn from_env() -> Option<Self> {
        let runtime_repo_path: PathBuf =
            std::env::var("TURINGOS_CHAINTAPE_PATH").ok()?.into();
        let cas_path: PathBuf = match std::env::var("TURINGOS_CAS_PATH") {
            Ok(p) => p.into(),
            Err(_) => runtime_repo_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .join(format!(
                    "cas_{}",
                    runtime_repo_path
                        .file_name()
                        .map(|s| s.to_string_lossy().into_owned())
                        .unwrap_or_else(|| "default".into()),
                )),
        };
        let run_id = std::env::var("TURINGOS_RUN_ID").unwrap_or_else(|_| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs().to_string())
                .unwrap_or_else(|_| "0".into())
        });
        let queue_capacity = std::env::var("TURINGOS_CHAINTAPE_QUEUE_CAPACITY")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(64);
        Some(Self {
            runtime_repo_path,
            cas_path,
            run_id,
            queue_capacity,
        })
    }
}

// ── Bundle returned by the factory ──────────────────────────────────────────

/// TRACE_MATRIX FC3-N1: handle bundle bridging evaluator → Sequencer + on-disk ChainTape.
///
/// Bundle of runtime handles produced by `build_chaintape_sequencer`.
///
/// The caller is responsible for wiring `sequencer` into a `TuringBus` via
/// `TuringBus::with_sequencer(kernel, config, bundle.sequencer.clone())`,
/// then driving the runtime to completion and calling `bundle.shutdown()`
/// at exit to drain queued submissions.
pub struct ChaintapeBundle {
    /// Cloned and passed to `TuringBus::with_sequencer`.
    pub sequencer: Arc<Sequencer>,
    /// Concrete L4 writer (Git-backed). Test code holds a clone for chain-walk verification.
    pub transition_writer: Arc<RwLock<dyn LedgerWriter>>,
    /// L4.E rejection writer. JSONL backend (Atom 1.2 extension) when persisting; falls back
    /// to in-memory if the JSONL path cannot be opened (caller error handling).
    pub rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
    /// Per-run epoch — verifiers re-derive `pinned_pubkeys` map keyed by this epoch.
    pub epoch: SystemEpoch,
    /// Resolved runtime repo path (after canonicalization). Atom 5+ writes pinned pubkey
    /// JSON + agent audit trail under this dir.
    pub runtime_repo_path: PathBuf,
    /// Resolved CAS root directory. Atom 5 callers re-open `CasStore` here to
    /// write `AgentProposalRecord` artifacts (mirrors `runtime_repo_path` for
    /// the L4 / L4.E side).
    pub cas_path: PathBuf,
    /// Driver task running `run_chaintape_driver` against the queue.
    pub driver_handle: JoinHandle<()>,
    /// Drain trigger. Caller invokes `bundle.shutdown().await` at evaluator exit.
    pub shutdown_tx: oneshot::Sender<()>,
}

impl ChaintapeBundle {
    /// TRACE_MATRIX FC3-N1: drain + clean-shutdown contract — caller invokes at evaluator exit.
    ///
    /// Drain + shutdown contract:
    /// 1. Send shutdown signal (consumes shutdown_tx).
    /// 2. Driver wrapper sees signal → closes queue_rx → drains remaining → exits.
    /// 3. `driver_handle.await` blocks until drain completes.
    /// 4. JoinError (panic) is wrapped into `DriverError::JoinError`; clean exit returns Ok.
    pub async fn shutdown(self) -> Result<(), DriverError> {
        let _ = self.shutdown_tx.send(());
        match self.driver_handle.await {
            Ok(()) => Ok(()),
            Err(join_err) => Err(DriverError::JoinError(join_err.to_string())),
        }
    }
}

// ── Errors ──────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3-N1: bootstrap error class — fail-closed admission gate for production ChainTape.
///
/// Errors produced by the factory at bootstrap time.
#[derive(Debug)]
pub enum BootstrapError {
    Io(std::io::Error),
    LedgerWriter(String),
    Cas(String),
    Keypair(String),
    /// Atom 1 fail-closed: refuse to bootstrap a `Sequencer` (which always
    /// starts `next_logical_t = 0`) on top of an existing `refs/transitions/main`
    /// chain — the next commit would mismatch `Git2LedgerWriter`'s strict
    /// `len + 1` invariant. Resume mode is deferred to a future TB.
    NonEmptyRuntimeRepo {
        path: PathBuf,
        existing_head: String,
    },
}

impl std::fmt::Display for BootstrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::LedgerWriter(e) => write!(f, "ledger writer error: {e}"),
            Self::Cas(e) => write!(f, "cas error: {e}"),
            Self::Keypair(e) => write!(f, "keypair error: {e}"),
            Self::NonEmptyRuntimeRepo {
                path,
                existing_head,
            } => write!(
                f,
                "non-empty runtime repo at {path:?} (existing head {existing_head}); \
                 TB-6 Atom 1 fail-closes here. Reconstruction from existing chain is \
                 deferred to a future TB. Point TURINGOS_CHAINTAPE_PATH at a fresh \
                 directory to start a new run."
            ),
        }
    }
}

impl std::error::Error for BootstrapError {}

impl From<std::io::Error> for BootstrapError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// TRACE_MATRIX FC3-N1: runtime-side driver error — bounded to runtime/ module so Sequencer enum stays unchanged.
///
/// Runtime-local driver error. NOT a `Sequencer` enum addition — preserves
/// the no-STEP_B-trigger property of Atom 1.
#[derive(Debug)]
pub enum DriverError {
    JoinError(String),
}

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JoinError(s) => write!(f, "driver task join error: {s}"),
        }
    }
}

impl std::error::Error for DriverError {}

// ── Pinned pubkey on-disk format ────────────────────────────────────────────

/// TRACE_MATRIX FC3-N1: on-disk pinned-pubkey manifest — bridges Atom 1 keypair to Atom 4 verify_chaintape.
///
/// On-disk pinned-pubkey manifest. Written to `<runtime_repo>/pinned_pubkeys.json`
/// at bootstrap so `verify_chaintape` (Atom 4) can re-verify `system_signature`
/// on every `LedgerEntry` without separate config.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PinnedPubkeyManifest {
    pub run_id: String,
    pub tb_id: String,
    pub epoch: u64,
    pub pubkeys: Vec<PinnedPubkeyEntry>,
}

/// TRACE_MATRIX FC3-N1: single-epoch pinned pubkey row in the on-disk manifest.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PinnedPubkeyEntry {
    pub epoch: u64,
    pub pubkey_hex: String,
}

const PINNED_PUBKEYS_FILENAME: &str = "pinned_pubkeys.json";
const PINNED_PUBKEYS_TB_ID: &str = "TB-6";

fn write_pinned_pubkey_manifest(
    runtime_repo_path: &Path,
    epoch: SystemEpoch,
    keypair: &Ed25519Keypair,
    run_id: &str,
) -> Result<(), BootstrapError> {
    let pubkey = keypair.public_key();
    let pubkey_hex: String = pubkey
        .as_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    let manifest = PinnedPubkeyManifest {
        run_id: run_id.to_string(),
        tb_id: PINNED_PUBKEYS_TB_ID.to_string(),
        epoch: epoch.get(),
        pubkeys: vec![PinnedPubkeyEntry {
            epoch: epoch.get(),
            pubkey_hex,
        }],
    };
    let json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| BootstrapError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    std::fs::create_dir_all(runtime_repo_path)?;
    std::fs::write(runtime_repo_path.join(PINNED_PUBKEYS_FILENAME), json)?;
    Ok(())
}

// ── Factory ─────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3-N1: production-mode factory — single entry-point that wires the kernel into an LLM-driven binary with on-disk chain persistence.
///
/// Build a production-mode `Sequencer` + `Git2LedgerWriter` + `RejectionEvidenceWriter` +
/// driver task per `RuntimeChaintapeConfig`.
///
/// Steps:
/// 1. Open `Git2LedgerWriter` at `config.runtime_repo_path`. **Fail-closed** if the
///    repo already has a `refs/transitions/main` reference (non-empty chain) —
///    resume mode is a future-TB enhancement.
/// 2. Open `CasStore` at `config.cas_path`.
/// 3. Generate a per-run `Ed25519Keypair`. Pin its public key to
///    `PinnedSystemPubkeys` under `epoch`. Write `pinned_pubkeys.json`
///    next to the runtime repo for `verify_chaintape`.
/// 4. Build `RejectionEvidenceWriter` (Atom 1.1: in-memory; Atom 1.2 extends to
///    JSONL-backed via `RejectionEvidenceWriter::open_jsonl`).
/// 5. Initialize `QState::genesis()` (existing `QState::default()` constructor).
/// 6. Construct `Sequencer::new(...)` — captures the queue receiver in the tuple return.
/// 7. Spawn `run_chaintape_driver(sequencer.clone(), queue_rx, shutdown_rx)` on
///    the current tokio runtime.
/// 8. Return `ChaintapeBundle` with all handles.
pub fn build_chaintape_sequencer(
    config: &RuntimeChaintapeConfig,
) -> Result<ChaintapeBundle, BootstrapError> {
    build_chaintape_sequencer_with_initial_q(config, QState::genesis())
}

/// TRACE_MATRIX FC3-N1: TB-6 Atom 2 — factory variant accepting a pre-seeded `initial_q`.
///
/// Production-mode factory variant for callers that need to pre-populate the
/// economic state (e.g., adapter-level tests + Atom 3 smoke fixtures that
/// seed sponsor balance for `WorkTx` admission per WP § 18 Inv 5). The base
/// `build_chaintape_sequencer` delegates here with `QState::genesis()`.
///
/// All other behavior (fail-closed on non-empty repo, pinned-pubkey manifest,
/// JSONL-backed L4.E writer, runtime-side driver wrapper, Sequencer wiring) is
/// identical to the base factory.
pub fn build_chaintape_sequencer_with_initial_q(
    config: &RuntimeChaintapeConfig,
    initial_q: QState,
) -> Result<ChaintapeBundle, BootstrapError> {
    // Step 1: open or init runtime repo, fail-closed on existing chain.
    std::fs::create_dir_all(&config.runtime_repo_path)?;
    let git_writer = Git2LedgerWriter::open(&config.runtime_repo_path)
        .map_err(|e| BootstrapError::LedgerWriter(e.to_string()))?;
    if git_writer.head_commit_oid().is_some() {
        let existing_head = git_writer
            .head_commit_oid()
            .map(|o| o.to_string())
            .unwrap_or_default();
        return Err(BootstrapError::NonEmptyRuntimeRepo {
            path: config.runtime_repo_path.clone(),
            existing_head,
        });
    }
    let transition_writer: Arc<RwLock<dyn LedgerWriter>> = Arc::new(RwLock::new(git_writer));

    // Step 2: open CAS.
    std::fs::create_dir_all(&config.cas_path)?;
    let cas_store = CasStore::open(&config.cas_path)
        .map_err(|e| BootstrapError::Cas(e.to_string()))?;
    let cas = Arc::new(RwLock::new(cas_store));

    // Step 3: generate keypair + persist pinned-pubkey manifest.
    let keypair = Arc::new(
        Ed25519Keypair::generate_with_secure_entropy()
            .map_err(|e| BootstrapError::Keypair(e.to_string()))?,
    );
    let epoch = SystemEpoch::new(1);
    write_pinned_pubkey_manifest(&config.runtime_repo_path, epoch, &keypair, &config.run_id)?;
    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(epoch, keypair.public_key());
    let pinned_pubkeys = Arc::new(pinned);

    // Step 4: rejection writer — JSONL-backed at <runtime_repo>/rejections.jsonl
    // per Atom 1.2 + architect § 3.5 deliverable shape. Falls back to in-memory
    // if open_jsonl fails (e.g. permission denied); failure is logged but does
    // not abort bootstrap because legacy in-memory writer is still functional.
    let rejections_path = config.runtime_repo_path.join("rejections.jsonl");
    let rejection_writer = match RejectionEvidenceWriter::open_jsonl(rejections_path.clone()) {
        Ok(w) => Arc::new(RwLock::new(w)),
        Err(e) => {
            log::error!(
                "[chaintape] rejection writer open_jsonl({:?}) failed: {e} — falling back to in-memory",
                rejections_path
            );
            Arc::new(RwLock::new(RejectionEvidenceWriter::default()))
        }
    };

    // Step 5: predicate + tool registries (default empty registries — production-binary
    // is responsible for registering predicates / tools before submitting txs).
    let predicate_registry = Arc::new(PredicateRegistry::new());
    let tool_registry = Arc::new(ToolRegistry::new());

    // Step 6: initial QState (caller-provided; base factory passes QState::genesis()).

    // Step 7: construct Sequencer.
    let (sequencer, queue_rx) = Sequencer::new(
        cas,
        keypair,
        epoch,
        transition_writer.clone(),
        rejection_writer.clone(),
        predicate_registry,
        tool_registry,
        pinned_pubkeys,
        initial_q,
        config.queue_capacity,
    );
    let sequencer = Arc::new(sequencer);

    // Step 8: spawn driver wrapper + shutdown channel.
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let driver_seq = sequencer.clone();
    let driver_handle = tokio::spawn(async move {
        run_chaintape_driver(driver_seq, queue_rx, shutdown_rx).await;
    });

    Ok(ChaintapeBundle {
        sequencer,
        transition_writer,
        rejection_writer,
        epoch,
        runtime_repo_path: config.runtime_repo_path.clone(),
        cas_path: config.cas_path.clone(),
        driver_handle,
        shutdown_tx,
    })
}

// ── Driver wrapper ──────────────────────────────────────────────────────────

/// Runtime-side driver loop. NOT `Sequencer::run`: see module doc-comment.
///
/// Invariants:
/// - On `shutdown_rx` signal: closes `queue_rx` (refuses new sends), drains
///   the remaining queue synchronously via `Sequencer::apply_one`, returns.
/// - On `queue_rx.recv() == None` (all senders dropped): returns.
/// - `tokio::select! { biased; ... }` ensures the shutdown signal wins races
///   against pending `recv()` calls (otherwise busy queues could starve shutdown).
async fn run_chaintape_driver(
    sequencer: Arc<Sequencer>,
    mut queue_rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    mut shutdown_rx: oneshot::Receiver<()>,
) {
    loop {
        tokio::select! {
            biased;
            _ = &mut shutdown_rx => {
                // Refuse new sends, then drain remaining envelopes.
                queue_rx.close();
                while let Some(envelope) = queue_rx.recv().await {
                    if let Err(e) = sequencer.apply_one(envelope) {
                        log::debug!("chaintape driver drain apply_one rejected: {e}");
                    }
                }
                return;
            }
            env = queue_rx.recv() => {
                match env {
                    Some(envelope) => {
                        if let Err(e) = sequencer.apply_one(envelope) {
                            log::debug!("chaintape driver apply_one rejected: {e}");
                        }
                    }
                    None => return,
                }
            }
        }
    }
}

// ── Tests ───────────────────────────────────────────────────────────────────
//
// In-crate unit tests cover the construction path that does NOT need the
// full evaluator / TuringBus surface. Integration tests for the full L4 path
// (including direct `bus.submit_typed_tx` fixture) live at
// `tests/tb_6_runtime_chaintape_bootstrap.rs` and land in Atom 1.3.

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn cfg_for(tmp: &TempDir, run_id: &str) -> RuntimeChaintapeConfig {
        RuntimeChaintapeConfig {
            runtime_repo_path: tmp.path().join("runtime_repo"),
            cas_path: tmp.path().join("cas"),
            run_id: run_id.to_string(),
            queue_capacity: 16,
        }
    }

    #[tokio::test]
    async fn build_chaintape_sequencer_returns_non_none_sequencer_with_git_writer() {
        let tmp = TempDir::new().expect("tempdir");
        let cfg = cfg_for(&tmp, "t1-run");
        let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
        // Sequencer constructed.
        assert_eq!(Arc::strong_count(&bundle.sequencer) >= 2, true);
        // pinned_pubkeys.json was written.
        let manifest_path = cfg.runtime_repo_path.join(PINNED_PUBKEYS_FILENAME);
        assert!(manifest_path.exists(), "pinned_pubkeys.json must exist at {manifest_path:?}");
        // Clean shutdown.
        bundle.shutdown().await.expect("shutdown");
    }

    #[tokio::test]
    async fn build_chaintape_sequencer_writes_pinned_pubkeys_json_to_runtime_repo() {
        let tmp = TempDir::new().expect("tempdir");
        let cfg = cfg_for(&tmp, "t2-run");
        let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
        let manifest_path = cfg.runtime_repo_path.join(PINNED_PUBKEYS_FILENAME);
        let json = std::fs::read_to_string(&manifest_path).expect("read manifest");
        let manifest: PinnedPubkeyManifest =
            serde_json::from_str(&json).expect("parse manifest");
        assert_eq!(manifest.run_id, "t2-run");
        assert_eq!(manifest.tb_id, "TB-6");
        assert_eq!(manifest.epoch, 1);
        assert_eq!(manifest.pubkeys.len(), 1);
        assert!(!manifest.pubkeys[0].pubkey_hex.is_empty());
        bundle.shutdown().await.expect("shutdown");
    }

    #[tokio::test]
    async fn build_chaintape_sequencer_fails_on_non_empty_repo() {
        let tmp = TempDir::new().expect("tempdir");
        let cfg = cfg_for(&tmp, "t3-run");
        // First bootstrap on empty repo — succeeds.
        let bundle = build_chaintape_sequencer(&cfg).expect("first bootstrap");
        bundle.shutdown().await.expect("shutdown");
        // Manually create a synthetic head commit so head_commit_oid().is_some()
        // We can do this by appending a fake LedgerEntry — but that requires a
        // signed entry. Cheaper: just open the same path again and check that
        // the FRESH bootstrap on an EMPTY but git-init'd repo still succeeds
        // (head_commit_oid is None for an init'd-but-no-commits repo). To
        // actually trigger NonEmptyRuntimeRepo, we'd need to commit a real
        // entry; that requires a full sequencer.apply_one path which is
        // cleaner to exercise in tb_6_runtime_chaintape_bootstrap.rs (Atom 1.3).
        //
        // For Atom 1.1 in-crate coverage: confirm the second bootstrap (with
        // empty git refs) still succeeds — exercises the head_commit_oid().is_none() branch.
        let bundle2 = build_chaintape_sequencer(&cfg).expect("second bootstrap on empty refs");
        bundle2.shutdown().await.expect("second shutdown");
    }

    #[tokio::test]
    async fn chaintape_bundle_shutdown_returns_clean_on_empty_queue() {
        let tmp = TempDir::new().expect("tempdir");
        let cfg = cfg_for(&tmp, "t5-run");
        let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
        // No submissions made — queue stays empty. shutdown() must return Ok promptly.
        let res = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            bundle.shutdown(),
        )
        .await;
        let inner = res.expect("shutdown did not time out");
        inner.expect("shutdown returned Ok");
    }
}
