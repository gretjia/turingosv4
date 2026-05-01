# TB-6 Atom 1 Preflight v1 — Production ChainTape Bootstrap

**Date**: 2026-05-01
**Status**: DRAFT v1 (pre-audit). Authored on `main` @ `7970d2d` (TB-6 Atom 0 shipped).
**Atom**: TB-6 Atom 1 — Production runtime repo bootstrap.
**Binding authority**:
- TB-6 charter § 5 + § 7 + § 12 (`handover/tracer_bullets/TB-6_charter_2026-05-01.md`)
- Architect ruling 2026-05-01 § 3.6 Atom 1 (`handover/directives/2026-05-01_TB6_ARCHITECT_RULING.md`)
- STEP_B_PROTOCOL.md (necessity audit per Phase 0)

This preflight is the Phase 0 input for an external Codex impl audit (D3 production-wire-up class). It does NOT yet authorize Phase 1 implementation; Phase 1 enters only after audit verdict.

---

## §0 Headline (one paragraph)

Atom 1 introduces a `RuntimeChaintapeConfig` + a reusable `runtime_chaintape::build_chaintape_sequencer(...)` factory, then routes the experiment evaluator binary `experiments/minif2f_v4/src/bin/evaluator.rs` through `TuringBus::with_sequencer(...)` (an existing opt-in constructor) instead of `TuringBus::new(...)`. The factory constructs a real `Git2LedgerWriter` rooted at a configurable runtime repo path + the existing `RejectionEvidenceWriter` + a fresh `Sequencer` with the same keypair / pinned_pubkeys / predicate_registry / tool_registry / initial_q shape that `cargo test --workspace` already exercises. Activation is env-flag-gated (`TURINGOS_CHAINTAPE_PATH=/path/to/runtime_repo`); legacy mode (sequencer=None, no on-disk chain) remains the default. **No `src/bus.rs`, `src/state/sequencer.rs`, or `src/kernel.rs` internals are touched** — Atom 1 only USES already-existing public surface. STEP_B parallel-branch A/B is therefore NOT triggered for Atom 1; the architect-ruling D3 production-wire-up audit IS still required.

---

## §1 Necessity (STEP_B Phase 0 question 1)

### §1.1 Observable behavior currently broken

After TB-5 ship at `cargo test --workspace 617/617`:

- `experiments/minif2f_v4/src/bin/evaluator.rs:16-26` imports `TuringBus`, `Kernel`, `BusConfig` etc. but does NOT import `turingosv4::state::sequencer::*` or `turingosv4::bottom_white::ledger::transition_ledger::*`. The evaluator builds the bus via `TuringBus::new` (`src/bus.rs:97`); `sequencer: Option<Arc<Sequencer>>` (`src/bus.rs:73`) is therefore `None`.
- `src/main.rs` (the v4 root binary) is 19 lines that only run `boot::verify_trust_root`. It never constructs a TuringBus, Sequencer, or any ledger writer — production-mode boot just verifies the trust root and exits.
- The chain-backed types — `transition_ledger::LedgerEntry` with `parent_ledger_root + system_signature + tx_payload_cid`; `Git2LedgerWriter` (`src/bottom_white/ledger/transition_ledger.rs:642`); `Sequencer::apply_one` stage 1.5 / stage 6 / stage 7 / stage 9 signing+folding+commit; replay tests `tests/tb_3_rsp1_formal_surface.rs::I29` + `tests/tb_5_challenge_resolve_surface.rs::I80` reconstructing economic state — ALL exist and ALL run inside `cargo test --workspace`. `InMemoryLedgerWriter` (`transition_ledger.rs:243`) is the only writer used by tests.

**Result**: no on-disk ChainTape has ever been produced from any LLM-driven run in TuringOS history. The kernel is genuinely Anti-Oreo and replay-deterministic, but those properties only show up in cargo-test in-memory state — not in artifacts that an external auditor or the user can walk after a real evaluator run.

### §1.2 What the gap MEANS in failure terms

If anyone tampers with two characters in the existing `handover/evidence/tb_{1..5}_smoke_*/n*_run.log`, no invariant in the codebase would catch it. There is no signature on those `.log` files; no parent-chain entry pointing to them; no replay path against them. The smoke evidence is paper trail (per `handover/audits/SELF_AUDIT_TB_5_SMOKE_TAPE_2026-05-01.md` § 3.4). Calling them "smoke tape" is an honest-naming gap that compounds with every kernel-only TB.

### §1.3 Necessity verdict (self-recorded; pending external)

**This change is necessary because**: every additional TB that adds kernel functionality without a production wire-up widens the gap between "tested in memory" and "verifiable from on-disk artifact". TB-1..TB-5 cumulative debt = 5 TBs. Without Atom 1, TB-6 itself becomes a 6th kernel-only TB; TB-7+ accumulate further. The architect ruling D1 explicitly selected Path A precisely to stop this expansion.

**Less-invasive alternative considered**:
- **(a) Only touch evaluator.rs; no new lib module**: feasible; the factory could live inline in evaluator.rs. Rejected because Atom 2's adapter + Atom 4's `verify_chaintape` CLI / test will both need the factory; placing it in an inline binary makes it unreachable from `tests/` and from a future replay-verifier CLI.
- **(b) Use `InMemoryLedgerWriter` even in production binary, persist a JSONL snapshot**: defeats the entire point of D2. Tampering with a JSONL still wouldn't break a chain — there'd still be no chain.
- **(c) Defer Atom 1 entirely; have evaluator emit signed `LedgerEntry`s without a Sequencer / TuringBus**: would require duplicating the apply_one stage 6 + stage 7 logic outside the kernel. Rejected as drift from "kernel-as-single-source-of-truth".

**Failure mode if NOT done**: TB-6 becomes a 6th kernel-only TB; the 5-TB ChainTape debt becomes 6-TB; subsequent TBs (Slash, NodeMarket) compound the gap further; the project's "kernel claims" remain unverifiable from on-disk LLM-driven artifacts indefinitely.

---

## §2 Surface map (line-grounded src refs)

### §2.1 What ALREADY exists (TB-6 Atom 1 will USE; will NOT modify)

| Symbol | Location | TB-6 Atom 1 use |
|---|---|---|
| `TuringBus::new(kernel, config) -> TuringBus` | `src/bus.rs:97` | UNCHANGED; legacy path stays for tests / non-chaintape mode |
| `TuringBus::with_sequencer(kernel, config, sequencer: Arc<Sequencer>) -> TuringBus` | `src/bus.rs:117` | USED by chaintape mode; **opt-in already exists** |
| `TuringBus::sequencer: Option<Arc<Sequencer>>` field | `src/bus.rs:73` | UNCHANGED; populated by `with_sequencer` |
| `TuringBus::submit_typed_tx(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError>` | `src/bus.rs:135` | USED by Atom 2 adapter |
| `Sequencer::new(cas, keypair, epoch, ledger_writer, rejection_writer, predicate_registry, tool_registry, pinned_pubkeys, initial_q, queue_capacity)` | `src/state/sequencer.rs:1138` | USED with `Git2LedgerWriter` for chaintape; UNCHANGED API |
| `Git2LedgerWriter::open(repo_path: &Path) -> Result<Self, LedgerWriterError>` | `src/bottom_white/ledger/transition_ledger.rs:659` | USED to bootstrap on-disk repo; auto-init if absent |
| `InMemoryLedgerWriter` | `src/bottom_white/ledger/transition_ledger.rs:243` | UNCHANGED; remains the default for cargo-test |
| `RejectionEvidenceWriter` | `src/bottom_white/ledger/rejection_evidence.rs` | USED with same shape as cargo-test fixtures |
| `PinnedSystemPubkeys` | `src/bottom_white/ledger/system_keypair.rs:257` | USED with `keypair.public_key()` pinned under `epoch` |
| `Ed25519Keypair` | (existing) | USED — same source as cargo-test fixtures (NOT genesis_payload) for Atom 1; genesis-pinned production keypair is a future refinement |

### §2.2 What Atom 1 WILL change (lib + binaries)

| File | Touch class | Restricted? | Justification |
|---|---|---|---|
| `src/runtime/mod.rs` (NEW) OR `src/runtime_chaintape.rs` (NEW) | additive new module | NO | factory + config struct + light helpers |
| `src/lib.rs` | 1-line `pub mod runtime;` (or equivalent) | NO | re-export of new module |
| `experiments/minif2f_v4/src/bin/evaluator.rs` | env-flag-gated branch around bus construction; if `TURINGOS_CHAINTAPE_PATH` set, build chaintape sequencer + use `TuringBus::with_sequencer`; otherwise legacy `TuringBus::new` | NO (sub-crate experiment binary) | minimal — only the bus-construction site changes |
| `src/main.rs` | OPTIONAL — none, OR add a `--chaintape-init <path>` mode that initializes an empty runtime repo without an LLM run | NO | not on critical path; nice-to-have for `verify_chaintape` Atom 4 fixtures |
| `tests/tb_6_runtime_chaintape_bootstrap.rs` (NEW) | additive integration tests | NO | 3-5 tests proving factory builds a non-None sequencer + opens git repo + idempotent re-bootstrap |

### §2.3 What Atom 1 will NOT touch (binding)

- `src/bus.rs` — `with_sequencer` already exists; no internals modified.
- `src/state/sequencer.rs` — `Sequencer::new` already accepts `Arc<RwLock<dyn LedgerWriter>>`; no API change.
- `src/kernel.rs` — not on the path.
- `src/sdk/tools/wallet.rs` — not on the path.
- `src/state/q_state.rs` — no schema mutation (charter § 6 #10).
- `src/state/typed_tx.rs` — no new variant (charter § 6 #6).
- `src/economy/monetary_invariant.rs` — no cascade (charter § 6 #9).
- `src/bottom_white/ledger/transition_ledger.rs` — no API change; `Git2LedgerWriter` used as-is.
- `constitution.md` — D7 binding (no amendment).

### §2.4 STEP_B applicability

STEP_B_PROTOCOL.md scope says "any change to files in CLAUDE.md's restricted list". Atom 1 changes none of those files. Therefore:

- **STEP_B Phase 0 (necessity audit)**: D3 architect ruling still requires Codex implementation audit on production-wire-up class atoms. This document IS the input for that audit. (Re-using STEP_B Phase 0 framing for the audit even though the file restriction itself is not triggered is consistent with `feedback_dual_audit` hybrid-by-risk.)
- **STEP_B Phase 1 (parallel branch A/B)**: NOT required at the file-restriction level. However, atom-level isolation via `experiment/tb6-chaintape-bootstrap` branch is recommended for clean rollback if the evaluator changes break smoke regression.
- **STEP_B Phase 2 (statistical A/B with N=50 paired sample on solve rate)**: NOT required. Atom 1 enables additive logging behind an env flag; with the env flag unset, evaluator behavior is bit-identical to TB-5 ship (same `prompt_context_hash`, same PputResult emit path). Atom 3's chain-backed smoke is the structural gate; solve-rate A/B is not the right test for "does the chain get written".

This determination is the highest-value input the Codex audit should challenge.

---

## §3 Minimum sufficient version (STEP_B Phase 0 question 3)

### §3.1 RuntimeChaintapeConfig (proposed shape)

```rust
// src/runtime/mod.rs (new file)

use std::path::PathBuf;

/// Runtime configuration for production / production-like ChainTape mode.
///
/// When `runtime_repo_path` is `Some`, the binary builds a real
/// `Sequencer` + `Git2LedgerWriter` and routes typed-tx submissions
/// through the on-disk chain. When `None`, the binary runs in legacy
/// mode (sequencer=None; pre-runtime PputResult emit path only).
#[derive(Debug, Clone)]
pub struct RuntimeChaintapeConfig {
    pub runtime_repo_path: PathBuf,
    pub cas_path: PathBuf,                // distinct from runtime_repo_path
    pub run_id: String,                   // for evidence dir naming
    pub queue_capacity: usize,            // Sequencer mpsc channel; default 64
}

impl RuntimeChaintapeConfig {
    /// Build from env vars. Returns `None` if `TURINGOS_CHAINTAPE_PATH` unset.
    pub fn from_env() -> Option<Self> {
        let runtime_repo_path = std::env::var("TURINGOS_CHAINTAPE_PATH").ok()?.into();
        // ... cas_path defaults to <runtime_repo_path>/../cas;
        // ... run_id defaults to env("TURINGOS_RUN_ID").unwrap_or_else(default-timestamp).
        // ... queue_capacity from env or default 64.
        ...
    }
}
```

### §3.2 build_chaintape_sequencer factory (proposed shape)

```rust
// src/runtime/mod.rs (new file)

use std::sync::Arc;
use tokio::sync::RwLock;

/// Bundle returned by the factory. Caller wires `bus = TuringBus::with_sequencer(kernel, config, sequencer)`.
pub struct ChaintapeBundle {
    pub sequencer: Arc<Sequencer>,
    pub ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
    pub rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
    pub queue_rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
}

pub fn build_chaintape_sequencer(
    config: &RuntimeChaintapeConfig,
    keypair: Arc<Ed25519Keypair>,
    pinned_pubkeys: Arc<PinnedSystemPubkeys>,
    initial_q: QState,
    cas: Arc<RwLock<CasStore>>,
    predicate_registry: Arc<PredicateRegistry>,
    tool_registry: Arc<ToolRegistry>,
    epoch: SystemEpoch,
) -> Result<ChaintapeBundle, BootstrapError> {
    let git_writer = Git2LedgerWriter::open(&config.runtime_repo_path)?;
    let ledger_writer: Arc<RwLock<dyn LedgerWriter>> =
        Arc::new(RwLock::new(git_writer));
    let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::new()));
    let (sequencer, queue_rx) = Sequencer::new(
        cas, keypair, epoch, ledger_writer.clone(),
        rejection_writer.clone(), predicate_registry, tool_registry,
        pinned_pubkeys, initial_q, config.queue_capacity,
    );
    Ok(ChaintapeBundle {
        sequencer: Arc::new(sequencer),
        ledger_writer,
        rejection_writer,
        queue_rx,
    })
}
```

### §3.3 Evaluator integration sketch

```rust
// experiments/minif2f_v4/src/bin/evaluator.rs (around the bus-construction site)

let bus = if let Some(chaintape_config) = RuntimeChaintapeConfig::from_env() {
    let bundle = build_chaintape_sequencer(&chaintape_config, /* ... */)?;
    // Spawn the Sequencer driver loop on a background tokio task.
    let seq_clone = bundle.sequencer.clone();
    tokio::spawn(async move { seq_clone.run(bundle.queue_rx).await });
    TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer)
} else {
    // Legacy path — bit-identical to TB-5 evaluator behavior.
    TuringBus::new(kernel, BusConfig::default())
};
```

### §3.4 What Atom 1 deliberately leaves to later atoms

- **Routing PputEvent / Agent proposals → WorkTx**: Atom 2.
- **Producing the actual on-disk smoke from a real LLM run on `mathd_algebra_107`**: Atom 3.
- **`verify_chaintape` CLI**: Atom 4.
- **Agent audit trail (proposal CIDs, read_set / write_set)**: Atom 5.
- **Branch / fork visibility summary**: Atom 6.
- **Synthetic-rejection-labelled L4.E in evidence dir**: Atom 3 (with explicit label per ruling § 3.6).

---

## §4 Phase 1 atom plan (post-audit; conditional on Codex PASS)

```text
Atom 1.1 — Add src/runtime/mod.rs (new module: RuntimeChaintapeConfig + ChaintapeBundle + build_chaintape_sequencer factory + BootstrapError type) + src/lib.rs re-export. Pure additive.
Atom 1.2 — Add tests/tb_6_runtime_chaintape_bootstrap.rs covering:
            T1 build_chaintape_sequencer_returns_non_none_sequencer_with_git_writer
            T2 build_chaintape_sequencer_idempotent_on_existing_repo (re-open does not corrupt)
            T3 build_chaintape_sequencer_initial_q_round_trip (initial_q persists after construction)
            T4 RuntimeChaintapeConfig_from_env_returns_none_when_var_unset
            T5 RuntimeChaintapeConfig_from_env_parses_when_var_set
Atom 1.3 — Wire experiments/minif2f_v4/src/bin/evaluator.rs around the bus-construction site (env-flag-gated branch); legacy path bit-identical to TB-5 ship when var unset.
Atom 1.4 — Add Atom 1 self-test: when env unset, evaluator's prompt_context_hash on mathd_algebra_107 oneshot stays "a1f43584a17d1226" (regression smoke; non-blocking — full chain-backed smoke is Atom 3).
```

Each Atom 1.N is a single commit; combined commit count = 4. cargo test --workspace must remain green at 617 + new TB-6 tests at every Atom 1.N. Disk pressure: `cargo clean` likely required before Atom 1.1 (currently 178M free; target/ is 7.5G).

---

## §5 Charter § 12 Q1-Q6 resolutions (proposed; subject to Codex audit)

| Q | Question | Proposed resolution |
|---|---|---|
| Q1 | TuringBus extension vs new constructor? | **Use existing `with_sequencer` as-is.** No new constructor. bus.rs remains untouched. |
| Q2 | runtime_repo path — production deploy vs ship-evidence? | **Both paths configurable via `RuntimeChaintapeConfig.runtime_repo_path`.** Ship-evidence path = `handover/evidence/tb_6_chaintape_smoke_2026-05-XX/runtime_repo`. Production deploy path = caller-specified (likely `~/turingos/runtime_repo` or similar, NOT in repo). |
| Q3 | How does main.rs / evaluator decide chaintape vs legacy? | **Env var `TURINGOS_CHAINTAPE_PATH`.** When set: chaintape mode. When unset: legacy. Optional secondary `TURINGOS_RUN_ID` (defaults to timestamp). Charter § 4.7 binding: env var is opt-in; legacy stays default. |
| Q4 | Runtime keypair source? | **Same `Ed25519Keypair` shape as cargo-test fixtures**: a fresh per-run keypair is constructed and pinned via `PinnedSystemPubkeys::from_iter([(epoch, kp.public_key())])`. Genesis-pinned production keypair (sourced from `genesis_payload.toml [system_pubkeys]`) is a future refinement (Atom 7 or later) and is NOT required for TB-6's chain-backed smoke proof. The chain produced is verifiable against the per-run pinned pubkeys; replay verifier (Atom 4) confirms signatures using the SAME pinned set written into the evidence directory. |
| Q5 | Synthetic rejection trigger for Atom 3 fallback? | **Stake-insufficient WorkTx submitted via the production binary.** Cleanest "natural" rejection because it doesn't require crafting a malformed envelope; the agent simply submits with `stake = 0` (or any value below required). L4.E row appended; `state_root` unchanged. If this happens naturally during the smoke run (likely — early agent proposals often miss admission requirements), no synthetic case needed. If not, Atom 3 fixture explicitly synthesizes one with `synthetic_rejection_for_l4e_gate = true` label per ruling § 3.6 Atom 3. |
| Q6 | Agent audit trail in CAS only or ChainTape extensions? | **CAS-only with tx_id back-link.** ChainTape `LedgerEntry.extensions` field already exists for forward compat (per `transition_ledger.rs:81+`); we MIGHT use one extension key for `agent_proposal_cid` back-link (read-only; not part of `state_root`), but the audit trail proper lives in CAS payloads. This keeps `LedgerEntry` schema unchanged in TB-6 (charter § 6 #10). |

---

## §6 Test plan (Atom 1.2 + Atom 1.3 + Atom 1.4)

### §6.1 New tests (target: 5-7 tests)

(See § 4 Atom 1.2 list.) Plus optional:
- **T6** (Atom 1.4): `evaluator_legacy_mode_prompt_context_hash_is_a1f43584a17d1226` — soft regression check that the evaluator's pre-runtime emit pipeline is still bit-identical when chaintape mode is OFF.
- **T7** (Atom 1.3 in-evaluator): `evaluator_chaintape_mode_constructs_bus_with_sequencer` — sets `TURINGOS_CHAINTAPE_PATH=<tmpdir>` and asserts the constructed bus has `sequencer.is_some()`.

### §6.2 cargo test --workspace target

Pre-Atom 1: 617/617 (TB-5 baseline).
Post-Atom 1.1: 617 (no new tests yet; just the new module compiles).
Post-Atom 1.2: 622 (T1-T5).
Post-Atom 1.3: 622 (evaluator wiring change; no new tests at this stage).
Post-Atom 1.4: 624 (T6 + T7).

Every commit reports `cargo test --workspace` count delta per ruling D4.

### §6.3 Test isolation

`tests/tb_6_runtime_chaintape_bootstrap.rs` uses `tempfile::TempDir` for runtime repo paths; concurrent tests get distinct tmpdirs; per `feedback_env_var_test_lock`, env-var-mutating tests (T4 + T5 + T7) need a static `Mutex` to survive cargo's parallel runner.

---

## §7 Audit gate (D3 production-wire-up class)

### §7.1 Codex implementation audit (REQUIRED)

Audit brief for Codex:
1. **Necessity**: do you agree TB-6 Atom 1 closes a real gap that no less-invasive alternative covers? See §1 + §3.4.
2. **Minimal-sufficient**: is the §3 sketch over- or under-scoped? Specifically: (a) is the `RuntimeChaintapeConfig` field set right? (b) should the factory return a `ChaintapeBundle` struct or a tuple? (c) is `Arc<RwLock<dyn LedgerWriter>>` the correct trait-object shape for production?
3. **Surface-map correctness**: are §2.1 line refs accurate against current `main` HEAD `7970d2d`? Specifically `bus.rs:117` `with_sequencer`, `sequencer.rs:1138` `Sequencer::new`, `transition_ledger.rs:659` `Git2LedgerWriter::open`.
4. **STEP_B applicability**: do you agree §2.4 — that no restricted file is touched, so STEP_B Phase-1 parallel-branch A/B is NOT triggered? Or do you see a hidden bus.rs / sequencer.rs / kernel.rs touch we missed?
5. **Q1-Q6 resolutions**: which proposed resolutions in §5 do you challenge? (Especially Q4 keypair source — fresh per-run vs genesis-pinned for TB-6.)
6. **Test plan**: is §6 sufficient? Specifically: is regression smoke T6 enough to claim "chaintape mode OFF = TB-5 evaluator behavior preserved"?
7. **Tokio lifecycle**: §3.3 spawns the Sequencer driver loop on a background tokio task. Is this safe given the evaluator's existing tokio runtime? Does the task need explicit shutdown / join at evaluator exit?

### §7.2 Gemini architecture audit (REQUIRED if available; else degraded label)

Audit brief for Gemini at strategic tier:
1. Does Atom 1's factory pattern preserve the WP-canonical "Anti-Oreo agent ≠ direct state writer" property? Specifically: does adding `Sequencer` to the production path expose any new agent-write affordance that wasn't there in cargo-test-only mode?
2. Does the proposed env-var trigger respect "production deploys MUST configure chaintape; legacy is for tests only"? Or should we go further and FAIL-CLOSED in production-mode boot when the env var is unset (analogous to P0.R production placeholder check)?
3. Per architect ruling § 3.6 Atom 5 + § 4.2: is the §5 Q6 resolution (CAS-only audit trail; no LedgerEntry schema mutation) the right boundary?

If Gemini at strategic tier (`gemini-2.5-pro` / `2.5-flash`) is `429 MODEL_CAPACITY_EXHAUSTED`, label the merged verdict `degraded` per `feedback_dual_audit` and proceed.

### §7.3 Verdict shapes

- **PASS / PASS**: proceed to Phase 1 implementation immediately.
- **CHALLENGE (round 1)**: revise this preflight to v2; re-launch narrow Codex audit per round-cap-2.
- **VETO** (constitutional violation; e.g. hidden bus.rs touch surfaces): redesign Atom 1 to a different factory shape, re-issue this preflight as v2.
- **PASS / degraded-PASS**: proceed; document degraded label in Atom 1 ship audit doc.

---

## §8 Risks + open questions

### §8.1 Disk pressure (BLOCKING)

178M free at `/dev/sda1`; `target/` is 7.5G. `cargo test --workspace` from clean WILL fail with ENOSPC. Options for unblock:
- **(a) `cargo clean`**: frees 7.5G but loses incremental compile state — first cargo check after will rebuild from scratch (5-15 min).
- **(b) Selective prune of `target/debug/incremental/`**: smaller savings; partial preservation of incremental state.
- **(c) Move `target/` to another mount**: requires available external mount; not assumed.

User must approve the cleanup approach before Atom 1.1 commits.

### §8.2 Tokio runtime + Sequencer driver loop

The evaluator already runs in `#[tokio::main]` async. The Sequencer driver loop (`Sequencer::run(receiver)`) is a long-running async task that processes the mpsc queue. Spawning it via `tokio::spawn` is the natural pattern — but evaluator exit must drop the queue sender so the Sequencer's `run` loop terminates cleanly.

### §8.3 Initial QState shape

Cargo-test fixtures construct `initial_q: QState` via various helpers in `src/state/q_state.rs::tests` and `tests/`. For Atom 1 production wire-up we need a "first-boot" QState that's analogous but production-suitable. Likely path: a `QState::genesis(epoch, system_pubkeys) -> QState` constructor that mirrors the cargo-test fixture default — or, if such a constructor doesn't exist yet, Atom 1.1 adds one. Codex audit should validate.

### §8.4 No real LLM run in Atom 1

Atom 1 produces NO chain entries from a real LLM run; that's Atom 3's job. Atom 1 only provides the wiring + tests that the wiring is correct. Codex audit should agree this is the right scope split (per ruling § 3.6 separation of Atom 1 vs Atom 3).

### §8.5 sequencing inside a TB-6 worktree

If we move Atom 1 into a worktree branch `experiment/tb6-chaintape-bootstrap` per `feedback_step_b_protocol` recommendation (even though STEP_B isn't strictly triggered), we maintain clean rollback. Worktree creation requires disk; with 178M free, worktree spawn may fail or be tight. Disk cleanup precedes worktree creation.

---

## §9 Cross-references

- **TB-6 charter**: `handover/tracer_bullets/TB-6_charter_2026-05-01.md`
- **Architect ruling**: `handover/directives/2026-05-01_TB6_ARCHITECT_RULING.md`
- **STEP_B protocol**: `handover/ai-direct/STEP_B_PROTOCOL.md`
- **TB-5 self-audit (gap discovery)**: `handover/audits/SELF_AUDIT_TB_5_SMOKE_TAPE_2026-05-01.md`
- **Surface citations**:
  - `src/bus.rs:73` `pub sequencer: Option<Arc<Sequencer>>` field
  - `src/bus.rs:97` `TuringBus::new`
  - `src/bus.rs:117` `TuringBus::with_sequencer`
  - `src/bus.rs:135` `TuringBus::submit_typed_tx`
  - `src/state/sequencer.rs:1138` `Sequencer::new`
  - `src/state/sequencer.rs:1098` `ledger_writer: Arc<RwLock<dyn LedgerWriter>>`
  - `src/bottom_white/ledger/transition_ledger.rs:642` `Git2LedgerWriter` struct
  - `src/bottom_white/ledger/transition_ledger.rs:659` `Git2LedgerWriter::open`
  - `src/bottom_white/ledger/transition_ledger.rs:243` `InMemoryLedgerWriter`
  - `src/main.rs` (19 lines; trust-root only)
  - `experiments/minif2f_v4/src/bin/evaluator.rs:16-26` (TuringBus import without sequencer)
- **Memory rules consulted**:
  - `feedback_step_b_protocol` (parallel-branch A/B; not triggered for Atom 1 per §2.4)
  - `feedback_dual_audit` (hybrid-by-risk; production wire-up class = Codex impl + Gemini arch)
  - `feedback_iteration_cap_24h` (production-wire-up exception: 72h-to-Atom-3)
  - `feedback_smoke_before_batch` (env-flag changes need smoke probe; T6 covers regression smoke)
  - `feedback_env_var_test_lock` (T4 + T5 + T7 need static Mutex)
  - `feedback_workspace_test_canonical` (cargo test --workspace count required at every commit)
  - `feedback_no_fake_menus` (Q1-Q6 are recommendations, not menus)
  - `feedback_chaintape_wire_up_priority` (D1 binding — Path A precedence)
