# TB-6 Atom 1 Preflight v2 — Production ChainTape Bootstrap (with persistent L4.E)

**Date**: 2026-05-01
**Status**: DRAFT v2.1 (post Codex round-2 audit `CHALLENGE-2`; round-cap=2 hit; auto-execute exception applied per `feedback_elon_mode_policy` since both remediations are determinate-best surgical patches). v2.0 at commit `37b1929`; v1 at `ca8d644`. v2.1 deltas confined to §3.2 (driver lifecycle) + §6 (T7 split + T10 direct fixture).
**Atom**: TB-6 Atom 1 — Production runtime repo bootstrap.
**Binding authority**:
- TB-6 charter § 5 + § 7 + § 12 (`handover/tracer_bullets/TB-6_charter_2026-05-01.md`)
- Architect ruling 2026-05-01 § 3.5 + § 3.6 Atom 1 (`handover/directives/2026-05-01_TB6_ARCHITECT_RULING.md`)
- Codex round-1 audit `CHALLENGE-6` (returned 2026-05-01; verdict at task `a2c57d750d22f0eb4`)
- STEP_B_PROTOCOL.md (necessity audit framing for Phase 0)

This v2 incorporates all 6 Codex CHALLENGE findings + an architect-aligned scope decision: Atom 1 expands to include **persistent L4.E writer** (because architect § 3.5 explicitly requires `refs/rejections/main 或等价结构` on-disk; Codex F1 surfaced that `RejectionEvidenceWriter` is currently in-memory only). atom count stays at 8 per architect § 3.6.

---

## §0 v1 → v2 delta banner

| # | Change | Driver |
|---|---|---|
| Δ1 | **Atom 1 includes persistent L4.E writer** (JSONL append-only with chain hashes; "等价结构" per architect § 3.5). `RejectionEvidenceWriter` gains an optional persistence backend; no `Sequencer` field-type change → no STEP_B-restricted-file touch. | Codex F1 + architect § 3.5 |
| Δ2 | `ChaintapeBundle` includes `driver_handle: JoinHandle` + `shutdown_tx` + explicit drain/shutdown contract | Codex Q7 + F2 |
| Δ3 | Atom 1 fail-closed on non-empty runtime_repo (counter reconstruction deferred to a future TB) | Codex F3 |
| Δ4 | Drop §8.3 (`QState::genesis()` already exists at `src/state/q_state.rs:447-448` and equals `QState::default()`) | Codex F4 |
| Δ5 | WAL_DIR + TURINGOS_CHAINTAPE_PATH coexistence rule + regression test (orthogonal layers; both can be on; chain_tape is on Sequencer, WAL is on TuringBus event log) | Codex F5 + Q5b |
| Δ6 | Persist pinned pubkey alongside chain in evidence dir (`runtime_repo/pinned_pubkeys.json`) so `verify_chaintape` can re-verify signatures without separate config | Codex Q5b |
| Δ7 | Test plan rebuilt — drop T6 (oneshot doesn't exercise bus per Codex Q6); add chaintape-on construction + drain/shutdown + reopen-fail-closed + L4.E JSONL chain-integrity + WAL coexistence | Codex Q6 |
| Δ8 | atom 1 sub-plan: 4 commits (no atom-count change) | architect § 3.6 |

---

## §1 Necessity (preserved from v1; 2 line refs added)

### §1.1 Observable behavior currently broken

After TB-5 ship (`cargo test --workspace 617/617`):

- `experiments/minif2f_v4/src/bin/evaluator.rs:16-26` imports `TuringBus`, `Kernel`, `BusConfig` etc. but does NOT import `turingosv4::state::sequencer::*` or `turingosv4::bottom_white::ledger::transition_ledger::*`. The evaluator builds the bus via `TuringBus::new` at `experiments/minif2f_v4/src/bin/evaluator.rs:698`; `sequencer: Option<Arc<Sequencer>>` (`src/bus.rs:73`) is therefore `None`. (Codex confirmed Q1.)
- `src/main.rs` (the v4 root binary) is 19 lines that only run `boot::verify_trust_root`. It never constructs a TuringBus, Sequencer, or any ledger writer.
- The chain-backed types (`transition_ledger::LedgerEntry`, `Git2LedgerWriter` at `src/bottom_white/ledger/transition_ledger.rs:642`, `Sequencer::apply_one` stages 1.5/6/7/9, replay tests `tests/tb_3_rsp1_formal_surface.rs::I29` + `tests/tb_5_challenge_resolve_surface.rs::I80`) all exist and run inside `cargo test --workspace`. `InMemoryLedgerWriter` (`transition_ledger.rs:243`) is the only writer used by tests.
- **NEW v2 disclosure (Codex F1)**: `RejectionEvidenceWriter` at `src/bottom_white/ledger/rejection_evidence.rs:234` is **in-memory only**. `Vec<RejectedSubmissionRecord>` chained via `prev_hash` (line 21 doc comment + line 234 struct). Git persistence explicitly deferred per the L4 / L4.E split decision record. There is no `Git2RejectionEvidenceWriter` and no `RejectionEvidenceWriter::open(path)` constructor.

### §1.2 What this means for TB-6 § 3.5 deliverable

Architect's required evidence directory shape includes both `runtime_repo/.git/refs/transitions/main` AND `runtime_repo/.git/refs/rejections/main 或等价结构`. With L4.E in-memory only, the L4.E side of the deliverable cannot be produced from a real run — **regardless of how cleanly we wire the L4 (transitions) side**. Atom 1 must therefore include L4.E persistence.

### §1.3 Necessity verdict (Codex agreed Q1)

Path A (expand Atom 1 to dual-writer wire-up: `Git2LedgerWriter` for L4 + persistent `RejectionEvidenceWriter` for L4.E) is the only path that simultaneously honors:
- Architect § 3.5 deliverable shape (both refs / both chains on disk)
- Architect § 3.6 Atom 3 ≥1 L4 + ≥1 L4.E entry hard requirement
- Architect § 3.6 atom count = 8

---

## §2 Surface map (Codex Q3 verified all 8 line refs clean; +5 v2 additions)

### §2.1 What ALREADY exists (TB-6 Atom 1 will USE; will NOT modify)

| Symbol | Location | Codex Q3 verified | TB-6 Atom 1 use |
|---|---|---|---|
| `TuringBus::sequencer: Option<Arc<Sequencer>>` field | `src/bus.rs:73` | ✅ no drift | UNCHANGED; populated by `with_sequencer` |
| `TuringBus::new(kernel, config)` | `src/bus.rs:97` | ✅ no drift | UNCHANGED; legacy default |
| `TuringBus::with_sequencer(kernel, config, sequencer)` | `src/bus.rs:117` | ✅ no drift | USED by chaintape mode |
| `TuringBus::submit_typed_tx(&self, tx)` | `src/bus.rs:135` | ✅ no drift | USED by Atom 2 adapter (not Atom 1) |
| `TuringBus::with_wal_path(kernel, config, wal_path)` | `src/bus.rs:149` | (v2 add) | UNCHANGED; orthogonal layer (see §3.6 WAL coexistence) |
| `Sequencer::new(...)` | `src/state/sequencer.rs:1138` | ✅ no drift | USED unchanged |
| `Sequencer::run(&self, queue_rx)` | `src/state/sequencer.rs:1350` | (v2 add per Codex Q4) | USED — driver loop spawned via `tokio::spawn` |
| `Sequencer.ledger_writer: Arc<RwLock<dyn LedgerWriter>>` | `src/state/sequencer.rs:1098` | ✅ no drift | USED with `Git2LedgerWriter` |
| `Sequencer.rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>` | (concrete type — NOT trait) | (v2 add per Codex F1) | UNCHANGED API; persistent backend lives inside the struct |
| `Sequencer.queue_tx` | `src/state/sequencer.rs:1093` | (v2 add per Codex F2) | UNCHANGED — but `Arc<Sequencer>` keeps it alive; shutdown signals via `oneshot::Sender` |
| `Git2LedgerWriter` struct | `src/bottom_white/ledger/transition_ledger.rs:642` | ✅ no drift | UNCHANGED |
| `Git2LedgerWriter::open(repo_path)` | `src/bottom_white/ledger/transition_ledger.rs:659` | ✅ no drift | USED — auto-init's empty repo; non-empty repo handled via Atom 1 fail-closed (§3.4) |
| `Git2LedgerWriter::head_commit_oid` | `src/bottom_white/ledger/transition_ledger.rs:707` | (v2 add for fail-closed check) | USED for non-empty detection |
| `RejectionEvidenceWriter` struct | `src/bottom_white/ledger/rejection_evidence.rs:234` | (v2 add per Codex F1) | EXTENDED with optional JSONL persistence backend (see §3.3) |
| `RejectionEvidenceWriter::append_rejected(...)` | `src/bottom_white/ledger/rejection_evidence.rs:265` | (v2 add) | UNCHANGED public signature |
| `RejectionEvidenceWriter::verify_chain()` | `src/bottom_white/ledger/rejection_evidence.rs:309` | (v2 add) | EXTENDED to verify both in-memory + JSONL |
| `RejectionEvidenceWriter::last_hash()` | `src/bottom_white/ledger/rejection_evidence.rs:255` | (v2 add) | UNCHANGED |
| `CasStore::open(repo_path)` | `src/bottom_white/cas/store.rs:148` (per Codex Q5) | (v2 add per Codex Q2.a) | USED; factory opens it from `cas_path` |
| `QState::genesis()` ≡ `QState::default()` | `src/state/q_state.rs:447-448` | (v2 add per Codex F4) | USED directly; v1's "open question" §8.3 removed |
| `Ed25519Keypair::generate_with_secure_entropy` | `src/bottom_white/ledger/system_keypair.rs` (per Codex search) | (v2 add) | USED for per-run keypair |
| `PinnedSystemPubkeys` | `src/bottom_white/ledger/system_keypair.rs:257` | (v2 add) | USED; pinned set persisted to `runtime_repo/pinned_pubkeys.json` (v2 Δ6) |
| `LedgerEntry.extensions: BTreeMap<...>` | `src/bottom_white/ledger/transition_ledger.rs:99-102` (per Codex Q6) | (v2 add) | UNCHANGED; reserved for Atom 5 audit-trail back-link only |

### §2.2 What Atom 1 WILL change (v2 expanded)

| File | Touch class | Restricted? | Justification |
|---|---|---|---|
| `src/runtime/mod.rs` (NEW) | additive new module | NO | factory + ChaintapeBundle + RuntimeChaintapeConfig + BootstrapError |
| `src/lib.rs` | 1-line `pub mod runtime;` | NO | re-export |
| **`src/bottom_white/ledger/rejection_evidence.rs` (v2 EXTENDED)** | additive: `Backend` enum + `open_jsonl(path)` constructor + JSONL append/load | **NO (not in restricted list)** | preserves struct type seen by `Sequencer.rejection_writer`; backend internal to the struct; signature of `append_rejected` unchanged |
| `experiments/minif2f_v4/src/bin/evaluator.rs` | env-flag-gated branch around bus construction; if `TURINGOS_CHAINTAPE_PATH` set → factory + with_sequencer + driver spawn + shutdown registration; else legacy | NO | sub-crate experiment binary |
| `src/main.rs` | UNCHANGED | — | Atom 1 does NOT need main.rs; evaluator is the production-like binary |
| `tests/tb_6_runtime_chaintape_bootstrap.rs` (NEW) | additive integration tests | NO | T1-T9 (v2 expanded; see §6) |

### §2.3 What Atom 1 will NOT touch (v2 reaffirmed)

- `src/bus.rs` — `with_sequencer` already exists.
- `src/state/sequencer.rs` — `Sequencer::new` API unchanged; `rejection_writer` field type unchanged (still concrete `Arc<RwLock<RejectionEvidenceWriter>>`).
- `src/kernel.rs`, `src/sdk/tools/wallet.rs` — not on the path.
- `src/state/q_state.rs` — no schema mutation; just call existing `QState::genesis()`.
- `src/state/typed_tx.rs` — no new variant.
- `src/economy/monetary_invariant.rs` — no cascade.
- `src/bottom_white/ledger/transition_ledger.rs` — no API change.
- `constitution.md` — D7 binding.

### §2.4 STEP_B applicability (Codex Q4 reaffirmed)

No restricted file modified. STEP_B Phase-1 parallel-branch A/B is NOT triggered. D3 production-wire-up Codex impl audit IS still required (round-1 done; round-2 narrow on the v1→v2 diff is the next gate).

---

## §3 Minimum sufficient version (v2 expanded)

### §3.1 RuntimeChaintapeConfig (v2; refined per Codex Q2.a)

```rust
// src/runtime/mod.rs (new file)

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RuntimeChaintapeConfig {
    /// Filesystem path to the on-disk runtime repo.
    /// `Git2LedgerWriter` rooted here writes refs/transitions/main.
    /// `RejectionEvidenceWriter::open_jsonl` writes <runtime_repo_path>/rejections.jsonl.
    /// Pinned pubkey file lives at <runtime_repo_path>/pinned_pubkeys.json.
    pub runtime_repo_path: PathBuf,
    /// CAS root directory. `CasStore::open(cas_path)` opened by the factory
    /// (was caller-ownership in v1; v2 factory owns it per Codex Q2.a).
    pub cas_path: PathBuf,
    /// Run identity for evidence dir naming + audit trail. Defaults to
    /// timestamp if `TURINGOS_RUN_ID` env var unset.
    pub run_id: String,
    /// Sequencer mpsc channel capacity. Default 64.
    pub queue_capacity: usize,
}

impl RuntimeChaintapeConfig {
    pub fn from_env() -> Option<Self> {
        let runtime_repo_path = std::env::var("TURINGOS_CHAINTAPE_PATH").ok()?.into();
        // cas_path defaults to <runtime_repo_path>/../cas/<run_id>
        // run_id from env or timestamp default
        // queue_capacity from env or 64
        ...
    }
}
```

### §3.2 ChaintapeBundle (v2.1; Codex round-2 CHALLENGE-1 applied — runtime-side driver wrapper)

**Codex round-2 ground truth** (verified against `src/state/sequencer.rs`):
- `Sequencer::run` (`:1350-1363`) is `while let Some(env) = queue_rx.recv().await { ... apply_one(env) }`. NO shutdown branch.
- `Sequencer.queue_tx` (`:1093`) is owned by `Sequencer`. Holding `Arc<Sequencer>` keeps the sender alive → `recv()` never returns `None`.
- `SequencerError` enum (`:1040-1043`) has only `ReceiverAlreadyTaken`. `DriverPanic` does NOT exist.
- `Sequencer::apply_one` (`:1475`) is `pub(crate)`. `src/runtime/` lives in the same crate → callable.

v2.0's shutdown_tx wiring was dead. v2.1 fix: **don't call `Sequencer::run`. Write a runtime-side driver loop in `src/runtime/mod.rs`** that owns the receiver, calls `Sequencer::apply_one` directly, and uses `tokio::select!` on a shutdown channel. Sequencer.rs untouched (no STEP_B trigger).

```rust
// src/runtime/mod.rs

use tokio::task::JoinHandle;
use tokio::sync::oneshot;
use crate::state::sequencer::{Sequencer, SubmissionEnvelope};

pub struct ChaintapeBundle {
    pub sequencer: Arc<Sequencer>,
    pub transition_writer: Arc<RwLock<dyn LedgerWriter>>,
    pub rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
    pub driver_handle: JoinHandle<Result<(), DriverError>>,
    pub shutdown_tx: oneshot::Sender<()>,
}

/// Runtime-local driver error (NOT a `Sequencer` enum addition).
#[derive(Debug, thiserror::Error)]
pub enum DriverError {
    #[error("driver task panicked: {0}")]
    JoinError(String),
}

/// Runtime-side driver wrapper. Spawned by `build_chaintape_sequencer`.
/// Owns the queue receiver (transferred from `Sequencer::new`'s tuple return);
/// calls `Sequencer::apply_one` directly (pub(crate); same crate).
async fn run_chaintape_driver(
    sequencer: Arc<Sequencer>,
    mut queue_rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    mut shutdown_rx: oneshot::Receiver<()>,
) {
    loop {
        tokio::select! {
            biased; // shutdown wins races
            _ = &mut shutdown_rx => {
                queue_rx.close(); // refuse new sends from any clone of queue_tx
                // Drain remaining envelopes synchronously
                while let Some(env) = queue_rx.recv().await {
                    if let Err(e) = sequencer.apply_one(env) {
                        log::debug!("drain apply_one rejected: {e}");
                    }
                }
                return;
            }
            env = queue_rx.recv() => {
                match env {
                    Some(envelope) => {
                        if let Err(e) = sequencer.apply_one(envelope) {
                            log::debug!("apply_one rejected: {e}");
                        }
                    }
                    None => return, // all senders dropped
                }
            }
        }
    }
}

impl ChaintapeBundle {
    /// Drain + shutdown contract:
    /// 1. Send shutdown signal (consumes shutdown_tx).
    /// 2. Driver wrapper sees signal → closes queue_rx → drains remaining → exits.
    /// 3. `driver_handle.await` blocks until drain completes.
    /// 4. JoinError (panic) is wrapped into DriverError::JoinError; clean exit returns Ok.
    pub async fn shutdown(self) -> Result<(), DriverError> {
        let _ = self.shutdown_tx.send(());
        match self.driver_handle.await {
            Ok(()) => Ok(()),
            Err(join_err) => Err(DriverError::JoinError(join_err.to_string())),
        }
    }
}
```

**Why this works** (vs v2.0's dead wiring):
- The runtime-side wrapper owns `queue_rx`. It can call `queue_rx.close()` to refuse further sends.
- After `close()`, all calls to `queue_tx.send()` from anywhere (including `Sequencer.queue_tx` clones via `bus.submit_typed_tx`) return `Err(SendError(...))` — but txs already in the channel are still drained.
- `tokio::select! biased` ensures shutdown signal wins races against pending recv (otherwise busy queues could starve shutdown).
- `Sequencer::apply_one` is called directly. No need for `Sequencer::run` to terminate; we never call it.

`Sequencer.queue_tx` (`src/state/sequencer.rs:1093`) is still owned by `Sequencer` and stays alive while `Arc<Sequencer>` is held — but that's fine: `bus.submit_typed_tx` uses it, and after `queue_rx.close()` those sends fail cleanly.

Tests T4 (drain) + T5 (clean exit on empty queue) verify this lifecycle. T10 (NEW; see §6) directly exercises `bus.submit_typed_tx` → `apply_one` → L4 entry path without involving evaluator.

### §3.3 RejectionEvidenceWriter — v2 JSONL backend extension (Codex F1)

`src/bottom_white/ledger/rejection_evidence.rs` extends `RejectionEvidenceWriter` (concrete struct; no trait conversion) with an internal optional persistence backend:

```rust
// src/bottom_white/ledger/rejection_evidence.rs (v2 EXTENDED)

#[derive(Debug, Clone, Default)]
pub struct RejectionEvidenceWriter {
    records: Vec<RejectedSubmissionRecord>,
    // v2 NEW (Codex F1):
    backend: Backend,
}

#[derive(Debug, Clone, Default)]
enum Backend {
    #[default]
    InMemory,
    /// JSONL-backed: each `append_rejected` writes a chain-hashed record to <path>.
    /// "等价结构" per architect § 3.5 (chain-hash equivalence to refs/rejections/main).
    JsonlAppend { path: PathBuf },
}

impl RejectionEvidenceWriter {
    /// EXISTING — InMemory backend; tests use this.
    pub fn new() -> Self { Self::default() }

    /// NEW v2 — JSONL persistent backend; production chaintape mode uses this.
    /// On open: if file exists, replay JSONL into `records` (rebuilds chain).
    /// If file does not exist, creates parent dirs + opens for append.
    /// Replay validates chain integrity on load (rejects tampering).
    pub fn open_jsonl(path: PathBuf) -> Result<Self, RejectionEvidenceError> { ... }

    /// EXISTING — public signature unchanged.
    pub fn append_rejected(...) -> Hash {
        // 1. Compute chain hash + push to records (existing logic).
        // 2. If backend == JsonlAppend: serialize the new record + append a single
        //    JSONL line + fsync.
        // 3. Return chain hash.
    }
}
```

The 8-arg `append_rejected` signature (`src/bottom_white/ledger/rejection_evidence.rs:265`) is unchanged. `Sequencer::new` accepts the same `Arc<RwLock<RejectionEvidenceWriter>>` — no sequencer.rs change.

JSONL chain integrity: each line is a single `RejectedSubmissionRecord` with embedded `prev_hash` + `hash`. Tampering with any line breaks the chain at that line; `verify_chain()` walks records (in-memory or JSONL-loaded) and detects the break.

### §3.4 Fail-closed on non-empty runtime_repo (Codex F3)

`Git2LedgerWriter::open` resumes `len` from existing repo; `Sequencer::new` always starts `next_logical_t` at 0. To avoid mismatch:

```rust
// src/runtime/mod.rs

pub fn build_chaintape_sequencer(...) -> Result<ChaintapeBundle, BootstrapError> {
    let writer = Git2LedgerWriter::open(&config.runtime_repo_path)?;
    if writer.head_commit_oid().is_some() {
        // Non-empty repo: Sequencer would re-issue logical_t=1 over an
        // existing chain → digest mismatch on next commit.
        return Err(BootstrapError::NonEmptyRuntimeRepo {
            path: config.runtime_repo_path.clone(),
            existing_head: writer.head_commit_oid_hex().unwrap_or_default(),
            hint: "TB-6 Atom 1 fail-closes on non-empty repo. Reconstruction \
                   from existing chain is deferred to a future TB. To start a \
                   fresh run, point TURINGOS_CHAINTAPE_PATH at a new directory.",
        });
    }
    // proceed with empty-repo bootstrap
    ...
}
```

Reconstruction (resume mode) is explicitly deferred — Atom 1 ships the empty-bootstrap-only path. Resume becomes a future TB enhancement.

### §3.5 Pinned pubkey persistence (Codex Q5b + Δ6)

Per-run keypair → `PinnedSystemPubkeys::from_iter([(epoch, kp.public_key())])`. The pinned set is serialized to `<runtime_repo_path>/pinned_pubkeys.json` at bootstrap. `verify_chaintape` Atom 4 reads this file to verify entry signatures without separate config. Format:

```json
{
  "epoch": 1,
  "pubkeys": [
    {"epoch": 1, "pubkey_hex": "..."}
  ],
  "run_id": "tb6-smoke-2026-05-XX-...",
  "tb_id": "TB-6"
}
```

Genesis-pinned production keypair (sourced from `genesis_payload.toml [system_pubkeys]`) is a future refinement — Codex confirmed this is acceptable for TB-6 scope.

### §3.6 WAL_DIR + TURINGOS_CHAINTAPE_PATH coexistence (Codex F5)

WAL and ChainTape are **orthogonal persistence layers**:

| Layer | Operates on | Persists |
|---|---|---|
| WAL (`WAL_DIR`, `TuringBus::with_wal_path` at `src/bus.rs:149`) | TuringBus event log (clock, tx_count, generation, graveyard) | JSONL events for replay |
| ChainTape (`TURINGOS_CHAINTAPE_PATH`, this preflight) | Sequencer kernel ledger (`LedgerEntry` chain + L4.E) | git refs/transitions/main + rejections.jsonl |

**Precedence rule**: both can be on simultaneously. Order:
1. If `TURINGOS_CHAINTAPE_PATH` set → bus is built via `with_sequencer(kernel, config, sequencer)`, NOT `with_wal_path`. Chain mode wins for bus construction.
2. If only `WAL_DIR` set → legacy `with_wal_path` path; chaintape mode off.
3. If neither set → `TuringBus::new` legacy in-memory.
4. If both set → **chain wins**; WAL writes are silently disabled for the run; warning logged at `info!("[chaintape] WAL_DIR ignored when TURINGOS_CHAINTAPE_PATH is set")`. (Codex round-2 may challenge this — alternative is to error if both set.)

Regression test T8 verifies WAL_DIR-only mode still works after the env-flag branch refactor.

### §3.7 Evaluator integration sketch (v2 expanded)

```rust
// experiments/minif2f_v4/src/bin/evaluator.rs (around line 668)

let (mut bus, chaintape_bundle) = if let Some(chaintape_config) = RuntimeChaintapeConfig::from_env() {
    if std::env::var("WAL_DIR").is_ok() {
        info!("[chaintape] WAL_DIR ignored when TURINGOS_CHAINTAPE_PATH is set");
    }
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy());
    let bundle = build_chaintape_sequencer(&chaintape_config, keypair, /* ... */)?;
    let bus = TuringBus::with_sequencer(kernel, config, bundle.sequencer.clone());
    (bus, Some(bundle))
} else if let Ok(wal_dir) = std::env::var("WAL_DIR") {
    // existing WAL path unchanged
    (legacy_wal_bus(&wal_dir, kernel, config)?, None)
} else {
    (TuringBus::new(kernel, config), None)
};

// ... evaluator main loop ...

// At evaluator exit (success or panic):
if let Some(bundle) = chaintape_bundle {
    bundle.shutdown().await?;
}
```

The `chaintape_bundle: Option<ChaintapeBundle>` is held across the run; the explicit `bundle.shutdown().await` at exit ensures all queued txs commit before the binary terminates.

---

## §4 Atom 1 sub-plan (v2 expanded; 4 commits, 1 atom per architect § 3.6)

```text
Atom 1.1 — src/runtime/mod.rs (RuntimeChaintapeConfig + ChaintapeBundle + build_chaintape_sequencer + BootstrapError) + src/lib.rs re-export. Fail-closed on non-empty repo. Pure additive. cargo test --workspace baseline preserved.
Atom 1.2 — src/bottom_white/ledger/rejection_evidence.rs Backend enum + open_jsonl + JSONL append/load + verify_chain extension. Existing append_rejected signature unchanged. tests/tb_6_l4e_jsonl_persistence.rs T_R1-T_R5 (chain integrity, reopen, tampering detection).
Atom 1.3 — experiments/minif2f_v4/src/bin/evaluator.rs env-flag-gated chaintape branch + WAL coexistence + ChaintapeBundle.shutdown() at evaluator exit. tests/tb_6_runtime_chaintape_bootstrap.rs T1-T9 (construction, drain, reopen-fail-closed, WAL-coexistence, pinned-pubkey persistence).
Atom 1.4 — Trust Root manifest rehash for the new src/runtime/mod.rs file (R-014 protocol; non-sudo per R-018). cargo test --workspace 617 + ~14 new TB-6 tests = 631+ green.
```

Each Atom 1.N reports `cargo test --workspace` count delta per ruling D4. STEP_B not triggered (no restricted file). 24h iteration cap: production-wire-up exception applies (charter § 7.2; Atom 3 must run within 72h of Atom 0 ship).

---

## §5 Q1-Q6 resolutions (v2 + Codex round-1 challenges)

| Q | v1 proposal | Codex round-1 verdict | v2 resolution |
|---|---|---|---|
| Q1 TuringBus extension | use existing `with_sequencer` | not challenged | unchanged |
| Q2 runtime_repo path | configurable via config | challenge: `cas_path` ambiguous | factory opens `CasStore::open(&config.cas_path)`; ChaintapeBundle does NOT include CAS handle (caller already has it) |
| Q3 env trigger | `TURINGOS_CHAINTAPE_PATH` | challenge: WAL precedence | §3.6 — chain wins; WAL silent-off if both set |
| Q4 keypair source | per-run fresh | narrow challenge: must persist pubkey for replay | §3.5 — pinned pubkey written to `runtime_repo/pinned_pubkeys.json`; verify_chaintape (Atom 4) reads it |
| Q5 synthetic-rejection trigger | stake-insufficient WorkTx | trigger good; **L4.E in-memory only** | §3.3 — JSONL persistence backend resolves the persistence gap; trigger choice unchanged |
| Q6 audit trail location | CAS only | not challenged | unchanged; Atom 5 territory |

---

## §6 Test plan (v2 rebuilt per Codex Q6)

### §6.1 New test files

`tests/tb_6_runtime_chaintape_bootstrap.rs` (Atom 1.3):
- **T1** `build_chaintape_sequencer_returns_non_none_sequencer_with_git_writer`
- **T2** `build_chaintape_sequencer_writes_pinned_pubkeys_json_to_runtime_repo`
- **T3** `build_chaintape_sequencer_fails_on_non_empty_repo` (Codex F3)
- **T4** `chaintape_bundle_shutdown_drains_pending_submissions_before_join` (Codex Q7+F2; verifies runtime-side wrapper drains queue after shutdown signal)
- **T5** `chaintape_bundle_shutdown_returns_clean_on_empty_queue`
- **T6** ~~`evaluator_legacy_mode_prompt_context_hash_is_a1f43584a17d1226`~~ (DROPPED per Codex round-1 Q6 — `oneshot` doesn't traverse the bus)
- **T7** `evaluator_chaintape_mode_sets_bus_sequencer_field_to_some` (v2.1 SCOPE-NARROWED per Codex round-2 Q6 — `run_swarm` does NOT call `submit_typed_tx`; T7 is now construction-only: env-flag set → `bus.sequencer.is_some()` after constructor returns)
- **T8** `evaluator_legacy_wal_mode_unchanged_when_chaintape_off` (Codex F5 regression)
- **T9** `chaintape_mode_silently_disables_wal_when_both_env_vars_set` (Codex F5 precedence)
- **T10** `direct_bus_submit_typed_tx_synthetic_worktx_appends_l4_entry` (NEW v2.1 per Codex round-2 Q6) — bypasses evaluator entirely. Constructs synthetic signed `WorkTx` envelope with valid keypair + pinned pubkey + seeded EconomicState (escrow + balance fixtures from `tests/tb_3_rsp1_formal_surface.rs::I23` shape). Calls `bus.submit_typed_tx(tx).await` directly. Awaits driver. Asserts: ≥1 `LedgerEntry` in transition writer; chain root advances; replay reconstructs Q. This is the test that proves Atom 1's L4 path works WITHOUT depending on Atom 2's evaluator adapter.

`tests/tb_6_l4e_jsonl_persistence.rs` (Atom 1.2):
- **T_R1** `rejection_evidence_writer_open_jsonl_creates_empty_file`
- **T_R2** `append_rejected_persists_jsonl_line_and_in_memory_record`
- **T_R3** `reopen_jsonl_replays_existing_records_into_in_memory_chain`
- **T_R4** `tampering_with_jsonl_line_fails_verify_chain_on_reopen`
- **T_R5** `concurrent_open_jsonl_then_append_does_not_double_write` (file-lock test if applicable; otherwise document single-writer expectation)

Total Atom 1 new tests: 10 + 5 = **15 tests** (v2.1; T6 dropped, T10 added per Codex round-2).

### §6.2 cargo test --workspace targets

Pre-Atom 1.1: 617 (TB-5 baseline).
Post-Atom 1.1: 617 (no new tests; module compiles).
Post-Atom 1.2: 617 + 5 (T_R1-T_R5) = 622.
Post-Atom 1.3: 622 + 10 (T1-T5, T7-T10; T6 dropped) = 632.
Post-Atom 1.4: 632 (Trust Root only; no new tests).

Every commit reports `cargo test --workspace` count + delta per ruling D4.

### §6.3 Test isolation

- `tempfile::TempDir` for runtime repo + cas paths.
- `std::sync::Mutex` static for env-var-mutating tests (T7-T9 + T_R3) per `feedback_env_var_test_lock`.
- Each test sets its env vars under lock + clears them before drop.

---

## §7 Audit gate (v2 — round-2 narrow Codex on diff)

Round-1 verdict (CHALLENGE-6 with high confidence) closed by this v2. Round-2 is narrow: audit the v1→v2 diff for whether the 6 remediations are correctly applied + no new findings.

### §7.1 Round-2 Codex audit brief

1. **F1 remediation**: does §3.3 JSONL backend correctly preserve `append_rejected` signature + `verify_chain` semantics? Does the JSONL append produce a real chain (prev_hash + hash) per record? Does reopen-replay rebuild the in-memory chain correctly?
2. **Q7+F2 remediation**: does §3.2 `ChaintapeBundle.shutdown()` actually drain queued submissions before driver_handle returns? Does `Sequencer::run` honor a shutdown signal — and if not, what's the actual termination mechanism (drop queue_tx? wait for empty?)? Read `src/state/sequencer.rs:1350+` to verify.
3. **F3 remediation**: does §3.4 `BootstrapError::NonEmptyRuntimeRepo` correctly fail-close before construction continues? Is the error informative enough?
4. **F4 application**: §8.3 of v1 dropped — does §1 / §2 / §3 of v2 correctly use `QState::genesis()` (or `::default()`) without re-introducing the open question?
5. **F5 remediation**: does §3.6 WAL precedence rule cleanly handle all 4 env-var combinations? Should "both set" be an error instead of silent-WAL-off?
6. **Q6 remediation**: do the 14 new tests adequately exercise the chaintape path, including drain/shutdown + reopen + JSONL chain integrity? Specifically, does T7 `run_swarm` actually exercise the Sequencer (via `submit_typed_tx` or otherwise) such that L4 entries are produced — or does `run_swarm` still only emit pre-runtime PputResults?
7. **No-new-finding check**: any hidden bus.rs / sequencer.rs / kernel.rs / wallet.rs touch we missed in v2?

### §7.2 Verdict shape

- **PASS**: Phase 1 implementation enters immediately.
- **CHALLENGE-N (N ≤ 3)**: small remediations applied via auto-execute exception (Elon-mode round-cap=2).
- **CHALLENGE-N (N ≥ 4)**: redesign needed; v3 required.
- **VETO**: structural showstopper; user-architect escalation.

---

## §8 Risks + open questions (v2 reduced; F4 closed)

### §8.1 Disk pressure (RESOLVED)

`cargo clean` freed 8.2 GiB; disk now 7.7G free. Phase 1 unblocked.

### §8.2 Sequencer::run shutdown semantics (CLOSED v2.1 — runtime-side wrapper, no STEP_B trigger)

Codex round-2 verified `Sequencer::run` (`src/state/sequencer.rs:1350-1363`) has NO shutdown branch + `Sequencer` owns `queue_tx` (`:1093`) → option (a) "drop queue_tx via Arc count" cannot work because the driver task itself holds `Arc<Sequencer>` keeping the sender alive. Option (b) (modify `Sequencer::run` signature) would trigger STEP_B. **v2.1 picks option (c)**: replace the call to `Sequencer::run` with a runtime-side driver wrapper in `src/runtime/mod.rs` (see §3.2). The wrapper owns the receiver, uses `tokio::select!` on shutdown_rx, and calls `Sequencer::apply_one` (`pub(crate)`; same crate) directly. Sequencer.rs untouched. STEP_B safe. T4 + T10 verify lifecycle correctness.

### §8.3 ~~QState seed shape~~ (CLOSED — Codex F4)

`QState::genesis()` exists at `src/state/q_state.rs:447-448` and is just `QState::default()`. v2 uses it directly.

### §8.4 No real LLM run in Atom 1 (preserved from v1)

Atom 1 produces NO chain entries from a real LLM run — that's Atom 3. Atom 1 ships infrastructure + tests that the wiring is correct.

### §8.5 Worktree creation for Phase 1

Per `feedback_step_b_protocol` recommendation, atom-level isolation via `experiment/tb6-chaintape-bootstrap` worktree gives clean rollback even though STEP_B isn't strictly triggered. Disk now permits worktree creation.

---

## §9 Cross-references (v2)

- **Codex round-1 audit verdict**: task `a2c57d750d22f0eb4`; verdict CHALLENGE-6 (preserved in this preflight § 0 / §3 / §6 / §8 by reference; raw verdict in agent transcript; full-fidelity excerpt below).
- **TB-6 charter**: `handover/tracer_bullets/TB-6_charter_2026-05-01.md`
- **Architect ruling**: `handover/directives/2026-05-01_TB6_ARCHITECT_RULING.md`
- **TB-5 self-audit (gap discovery)**: `handover/audits/SELF_AUDIT_TB_5_SMOKE_TAPE_2026-05-01.md`
- **STEP_B protocol**: `handover/ai-direct/STEP_B_PROTOCOL.md`
- **Surface citations** (v2 expanded):
  - `src/bus.rs:73,97,117,135,149` (sequencer field, ctors, submit, with_wal_path)
  - `src/state/sequencer.rs:1093,1098,1138,1350` (queue_tx, ledger_writer field, ::new, ::run)
  - `src/bottom_white/ledger/transition_ledger.rs:99-102,243,642,659,707` (LedgerEntry.extensions, InMemoryLedgerWriter, Git2LedgerWriter, ::open, head_commit_oid)
  - `src/bottom_white/ledger/rejection_evidence.rs:21,234,255,265,309` (in-memory comment, struct, last_hash, append_rejected, verify_chain)
  - `src/bottom_white/cas/store.rs:148` (CasStore::open per Codex Q5)
  - `src/state/q_state.rs:447-448` (QState::genesis ≡ default per Codex F4)
  - `src/main.rs` (19 lines; trust-root only; UNCHANGED in Atom 1)
  - `experiments/minif2f_v4/src/bin/evaluator.rs:16-26,665-699` (TuringBus import + WAL_DIR branch site)
- **Memory rules consulted**:
  - `feedback_chaintape_wire_up_priority` (Path A binding)
  - `feedback_smoke_evidence_naming` (D5)
  - `feedback_workspace_test_canonical` (D4 reporting shape)
  - `feedback_dual_audit` (hybrid-by-risk; round-2 narrow)
  - `feedback_iteration_cap_24h` (production wire-up 72h-to-Atom-3)
  - `feedback_smoke_before_batch` (T7-T9 cover regression)
  - `feedback_env_var_test_lock` (T7-T9, T_R3 need static Mutex)
  - `feedback_no_fake_menus` (Q1-Q6 are recommendations, not menus; Path A derived from architect § 3.5 + § 3.6 — no menu offered)
  - `feedback_step_b_protocol` (not triggered for Atom 1 per §2.4)
  - `feedback_elon_mode_policy` (round-cap=2; auto-execute on determinate-best surgical patch)
