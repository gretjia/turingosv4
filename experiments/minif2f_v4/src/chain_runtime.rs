//! TB-18 Atom B Phase 1 — `SharedChain` initialization lifted from `evaluator.rs::run_swarm`.
//!
//! ## Why this module exists
//!
//! Per architect TB-18 ratification ruling §2.8 (verbatim):
//!
//! > Atom B 要证明的是: one evaluator process / one runtime_repo / one CAS /
//! > one chain / multiple tasks. 如果它只是一个 process 里启动多个 subprocess,
//! > 每个 subprocess 自己起 chain, 那不合格.
//!
//! Driving N tasks against ONE shared chain in ONE process requires lifting
//! the `Kernel + BusConfig + ChaintapeBundle + AgentKeypairRegistry + TuringBus`
//! initialization OUT of `run_swarm` so it can be constructed ONCE per chain
//! and threaded through `drive_task` calls (Atom B Phase 3+ wiring).
//!
//! ## Phase 1 scope (this file's contribution)
//!
//! Pure mechanical extraction of `evaluator.rs::run_swarm` lines 659-789 +
//! 794-833 into `SharedChain::from_env`. Behavior is **byte-identical** to
//! the inline code; this is the safest possible cut.
//!
//! What Phase 1 does NOT do (deferred to subsequent phases):
//!
//! - One-time chain bootstrap (synthetic TaskOpen + zero-stake WorkTx for
//!   L4.E gate + preseed sponsor TaskOpen+EscrowLock) → Phase 2
//! - Per-task body parameterization (`run_swarm_with_shared_chain(chain,
//!   spec, budget)`) → Phase 3
//! - `SharedChain::shutdown(self)` consume method (currently inlined in
//!   `run_swarm` lines 3604-3999) → Phase 3
//! - Substantive `comprehensive_arena.rs` multi-task driver → Phase 4
//!
//! ## Compatibility contract
//!
//! In Phase 1, callers (only `run_swarm` today) destructure `SharedChain`
//! into the same local-variable names that previously lived inline. This
//! keeps the rest of `run_swarm` (lines 834-3999) byte-identical at the
//! source-text level, so we can verify Phase 1 produces byte-identical
//! chain artifacts on a single-task run.
//!
//! Per `feedback_no_workarounds_strict_constitution`: this is NOT 凑活 —
//! the API contract (Self → destructure into same local names) is the
//! architect-ratified Atom B substrate; the body's per-task semantics
//! and the `shutdown` consume method are explicitly forward-bound.
//!
//! ## TRACE_MATRIX
//!
//! `FC-trace: FC3-N1` — production-mode runtime initialization (mirrors
//! `runtime/mod.rs::ChaintapeBundle` factory contract).

use std::sync::{Arc, Mutex};

use log::{error, info, warn};

use turingosv4::bus::{BusConfig, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::ChaintapeBundle;

/// TB-18 Atom B Phase 1: bundle of chain-initialization handles produced by
/// [`SharedChain::from_env`].
///
/// Owns the `TuringBus`, the optional `ChaintapeBundle` (the production
/// chain handle when `TURINGOS_CHAINTAPE_PATH` is set), the durable agent
/// keypair registry (only when chaintape mode is on), and bookkeeping for
/// the genesis-report initial-balances list (only when preseed is enabled).
///
/// In Phase 1, `run_swarm` destructures this into the same local-variable
/// names that previously lived inline. Phase 3 will switch callers to take
/// `&mut SharedChain` and add a `shutdown(self)` consume method.
pub struct SharedChain {
    /// Primary tx submission interface, pre-wired to either the production
    /// `Sequencer` (chaintape mode), a WAL-on-disk path (legacy/dev), or
    /// in-memory (last-resort fallback).
    pub bus: TuringBus,
    /// Production `ChaintapeBundle` when `TURINGOS_CHAINTAPE_PATH` is set;
    /// `None` in legacy/dev modes. Owns the on-disk runtime_repo + CAS
    /// handles + driver task; `bundle.shutdown().await` must run at chain
    /// end to drain queued submissions.
    pub chaintape_bundle: Option<ChaintapeBundle>,
    /// TB-9 durable agent keypair registry when chaintape mode is on; `None`
    /// otherwise. Wrapped in `Arc<Mutex<>>` so the registry can be shared
    /// across the async run loop.
    pub agent_keypairs: Option<Arc<Mutex<AgentKeypairRegistry>>>,
    /// TB-7R Deliverable C: initial balances seeded into the genesis QState
    /// (one entry per preseeded agent_id → micro-coin balance). Empty when
    /// preseed disabled. Used by `genesis_report.json` writer downstream.
    pub initial_balances_for_genesis_report: Vec<(String, i64)>,
    /// `TURINGOS_CHAINTAPE_PRESEED=1` toggle, parsed once at construction.
    /// Downstream code reads this to decide whether to emit the preseed
    /// TaskOpen + EscrowLock pair after init (currently inline in
    /// `evaluator.rs::run_swarm` lines 834+; lifted into Phase 2).
    pub chaintape_preseed_enabled: bool,
}

impl SharedChain {
    /// TB-18 Atom B Phase 1: initialize the shared chain from environment.
    ///
    /// **Lifted** from `evaluator.rs::run_swarm` lines 659-789 + 794-833.
    /// Behavior is byte-identical to the inline code.
    ///
    /// `problem_file` is consumed only by the WAL_DIR branch (legacy/dev
    /// mode where each problem gets its own WAL file). In chaintape mode
    /// (`TURINGOS_CHAINTAPE_PATH` set), it is unused; multi-task callers in
    /// Phase 4 should pass any non-empty stable string and never set
    /// WAL_DIR (the two modes are mutually exclusive: WAL_DIR is silently
    /// disabled when `TURINGOS_CHAINTAPE_PATH` is set, with an info!() log).
    ///
    /// ## Failure modes (preserved from inline code)
    ///
    /// - `TURINGOS_CHAINTAPE_PATH` set but bootstrap fails →
    ///   `std::process::exit(2)` (TB-7 Atom 1.7 fail-closed; matches
    ///   inline behavior at evaluator.rs:741).
    /// - Durable agent-keystore init fails → `panic!()` via `.expect(...)`
    ///   (matches inline behavior at evaluator.rs:777-782).
    /// - WAL file open fails → fall back to in-memory `TuringBus` (matches
    ///   inline behavior at evaluator.rs:820-829).
    pub fn from_env(problem_file: &str) -> Self {
        let kernel = Kernel::new();
        let config = BusConfig {
            // Phase 2.1 (C-043 candidate): OMEGA-accepted proofs are auto-written
            // as tape nodes (mandatory wtool per Art. IV). Full proofs can be
            // long; raise bus caps so winning nodes don't get size-vetoed. Agent
            // partials still typically <1200; no behavioural regression.
            max_payload_chars: 8000,
            max_payload_lines: 200,
            // C-011: decide/omega/native_decide forbidden (brute-force precedent)
            forbidden_patterns: vec![
                "native_decide".into(),
                "decide".into(),
                "omega".into(),
                "#eval".into(),
                "IO.Process".into(),
                "IO.FS".into(),
                "run_tac".into(),
                "unsafe".into(),
            ],
        };

        // TB-6 Atom 1.3: chaintape mode (TURINGOS_CHAINTAPE_PATH).
        // When set, build a production-mode Sequencer + Git2LedgerWriter (L4) +
        // JSONL-backed RejectionEvidenceWriter (L4.E) + driver wrapper, and route
        // bus construction through TuringBus::with_sequencer instead of the legacy
        // WAL_DIR / TuringBus::new paths. Both env vars set → chain wins; WAL_DIR
        // is silently disabled with an info!() log per preflight v2.1 §3.6.
        // Bundle is held across the run; bundle.shutdown().await is invoked at
        // the implicit final return to drain queued submissions.
        // TB-7 Atom 1.7 (Codex audit cc7b3dd action item #1): fail-closed when
        // TURINGOS_CHAINTAPE_PATH is set but bootstrap fails. Silent fallback
        // to legacy mode is the same anti-pattern as legacy `bus.append` as
        // authoritative state mutation (TB-7 charter §4.0 + §6 #31). When the
        // operator declares ChainTape mode, we either get ChainTape or we
        // exit non-zero — never quietly degrade to legacy.
        // TB-7.7 D3: optional pre-seed for L4 accept. Reading
        // TURINGOS_CHAINTAPE_PRESEED=1 enables a custom genesis QState with
        // pre-seeded balances for: (a) `tb7-7-sponsor` (for TaskOpen +
        // EscrowLock), and (b) every Agent_i (for WorkTx.stake admission).
        // Without preseed, real LLM WorkTx with non-zero stake would fail
        // admission with InsufficientBalance → L4.E. With preseed, the
        // chain shows ≥1 accepted L4 WorkTx for the first time.
        let chaintape_preseed_enabled = std::env::var("TURINGOS_CHAINTAPE_PRESEED")
            .ok()
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        // TB-7R Deliverable C: capture initial balances seeded into the genesis
        // QState so the genesis_report.json can record them as the run's starting
        // economic state. Empty when preseed disabled.
        let mut initial_balances_for_genesis_report: Vec<(String, i64)> = Vec::new();
        let chaintape_bundle: Option<ChaintapeBundle> =
            match turingosv4::runtime::RuntimeChaintapeConfig::from_env() {
                None => None, // env unset = legacy mode is the explicit choice
                Some(cfg) => {
                    let result = if chaintape_preseed_enabled {
                        // TB-10 Atom 1: preseed list extracted to runtime factory at
                        // `src/runtime/bootstrap.rs::default_pput_preseed_pairs()`.
                        // Single source of truth shared between evaluator and
                        // `lean_market` user CLI so both processes bootstrap to the
                        // same genesis QState. Includes:
                        //   - tb7-7-sponsor (10_000_000 micro) — TB-7.7 D3 self-fund
                        //   - Agent_user_0  (10_000_000 micro) — TB-10 user CLI sponsor
                        //   - Agent_0..9    ( 1_000_000 micro each) — solver budgets
                        let pairs = turingosv4::runtime::bootstrap::default_pput_preseed_pairs();
                        initial_balances_for_genesis_report = pairs
                            .iter()
                            .map(|(a, m)| (a.0.clone(), m.micro_units()))
                            .collect();
                        let initial_q = turingosv4::runtime::adapter::genesis_with_balances(&pairs);
                        info!(
                            "[chaintape/d3] pre-seed enabled (TB-10 factory): {} entries",
                            pairs.len()
                        );
                        turingosv4::runtime::build_chaintape_sequencer_with_initial_q(
                            &cfg, initial_q,
                        )
                    } else {
                        turingosv4::runtime::build_chaintape_sequencer(&cfg)
                    };
                    match result {
                        Ok(b) => Some(b),
                        Err(e) => {
                            error!(
                                "[chaintape] bootstrap failed under TURINGOS_CHAINTAPE_PATH (declared \
                                 ChainTape mode); exiting non-zero per TB-7 Atom 1.7 fail-closed \
                                 (Codex audit action #1). Error: {e}"
                            );
                            std::process::exit(2);
                        }
                    }
                }
            };
        if chaintape_bundle.is_some() && std::env::var("WAL_DIR").is_ok() {
            info!("[chaintape] WAL_DIR ignored when TURINGOS_CHAINTAPE_PATH is set");
        }

        // TB-7 Atom 2 + TB-9 Atom 2: per-run AgentKeypairRegistry holds Ed25519
        // keypairs for every distinct agent_id that submits a real-LLM proposal
        // through bus.submit_typed_tx. Public keys are persisted per-run to
        // <runtime_repo>/agent_pubkeys.json (TB-7 replay sidecar; unchanged).
        //
        // **TB-9 (2026-05-02)**: secrets are persisted across runs to an encrypted
        // durable keystore at TURINGOS_AGENT_KEYSTORE_PATH (default
        // ~/.turingos/keystore/agent_keystore.enc). Cross-run identity is the
        // architect TB-9 mandate ("agent durable key registry" + "cross-run
        // identity"; directive 2026-05-02 Part C line 1574). The keystore password
        // is read from TURINGOS_AGENT_KEYSTORE_PASSWORD; if unset, a hardcoded
        // local-dev fallback is used (acceptable for solo-runs per
        // feedback_kolmogorov_compression "MVP env-var; production-grade prompt is
        // post-v1.0 polish"). Tests / CI set the env var explicitly.
        //
        // Wrapped in Arc<Mutex<>> so the registry can be shared across the async
        // run loop (interior mutability needed for AgentKeypairRegistry::sign).
        let agent_keypairs: Option<Arc<Mutex<AgentKeypairRegistry>>> =
            chaintape_bundle.as_ref().map(|b| {
                let durable_path = turingosv4::runtime::agent_keystore::default_agent_keystore_path()
                    .expect("[chaintape/tb9] resolve durable agent keystore path (set HOME or TURINGOS_AGENT_KEYSTORE_PATH)");
                let pwd = turingosv4::runtime::agent_keystore::keystore_password_from_env();
                // TB-G G1.1 (architect §8 SIGNED 2026-05-11; user directive
                // "断点续作是本项目的核心" — Turing-machine fundamentalist
                // reading of FC2 §3.2 "every real evidence run must be
                // replayable from genesis_report + ChainTape + CAS + agent
                // registry + system pubkeys"): on resume, the existing
                // `agent_pubkeys.json` IS the agent registry — load it
                // instead of fail-closing. Mirrors the kernel-side
                // `bootstrap_resume_state` behavior for `pinned_pubkeys.json`.
                //
                // **R2 closure (Codex G2 R1.5 Q2+Q3 CHALLENGE 2026-05-11)**:
                // the binary gate is ONLY on the env flag (NOT on
                // manifest-existence). This way, when the user requests
                // resume (`TURINGOS_CHAINTAPE_RESUME=1`) but the manifest
                // is absent, the request routes to `resume_existing_durable`
                // which fail-closes with `ManifestAbsentInResume` — instead
                // of silently falling through to `generate_or_load_durable`
                // which would CREATE a fresh manifest (violating the
                // user-mandated "断点续作是本项目的核心" invariant).
                //
                // Predicate alignment with kernel: kernel's
                // `bootstrap_resume_state` requires
                // `config.resume_existing_chain && head_commit_oid().is_some()`
                // — but a non-empty chain WITHOUT an agent_pubkeys.json
                // is itself an inconsistency the binary must surface.
                // Both layers now fail-closed on env=1 + missing critical
                // input rather than silently degrading.
                let resume_requested = matches!(
                    std::env::var("TURINGOS_CHAINTAPE_RESUME").as_deref(),
                    Ok("1")
                );
                let reg = if resume_requested {
                    AgentKeypairRegistry::resume_existing_durable(
                        &b.runtime_repo_path,
                        &durable_path,
                        pwd,
                    )
                    .expect(
                        "[chaintape/tb9-resume] agent_keypairs resume must succeed \
                         (TURINGOS_CHAINTAPE_RESUME=1 requested). On ManifestAbsentInResume: \
                         the runtime_repo at this path was never agent-registered, so resume \
                         is meaningless — point TURINGOS_CHAINTAPE_PATH at a runtime_repo \
                         from a prior agent-registered run, or unset TURINGOS_CHAINTAPE_RESUME \
                         to start a fresh registry. On ResumeKeystoreInconsistent: \
                         agent_pubkeys.json and the durable keystore disagree about agent \
                         identities — either the keystore was wiped while the manifest \
                         survived, or TURINGOS_AGENT_KEYSTORE_PASSWORD does not match the \
                         password used for the prior run. On a keystore decrypt error: \
                         check TURINGOS_AGENT_KEYSTORE_PASSWORD.",
                    )
                } else {
                    AgentKeypairRegistry::generate_or_load_durable(
                        &b.runtime_repo_path,
                        &durable_path,
                        pwd,
                    )
                    .expect(
                        "[chaintape/tb9] agent_keypairs durable init must succeed (fresh runtime_repo guarantees \
                         manifest absent; if you see this on a non-fresh dir, see TB-6 NonEmptyRuntimeRepo or \
                         enable TURINGOS_CHAINTAPE_RESUME=1 for G1.1 resume mode. \
                         If you see a keystore decrypt error, check TURINGOS_AGENT_KEYSTORE_PASSWORD matches \
                         the password used for the previous run.)",
                    )
                };
                Arc::new(Mutex::new(reg))
            });

        // Phase 1: opt-in tape persistence via env. WAL_DIR=<dir> enables WAL
        // writes to <dir>/<problem>_<timestamp>.jsonl; resumes if file exists.
        // Default off for backward-compat baseline runs.
        let bus = if let Some(ref bundle) = chaintape_bundle {
            info!(
                "[chaintape] bus wired with Sequencer + on-disk ChainTape at {:?}",
                bundle.runtime_repo_path
            );
            TuringBus::with_sequencer(kernel, config, bundle.sequencer.clone())
        } else if let Ok(wal_dir) = std::env::var("WAL_DIR") {
            let problem_stem = std::path::Path::new(problem_file)
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| "unknown".into());
            let resume_id = std::env::var("WAL_RESUME_ID").ok();
            let id = resume_id.unwrap_or_else(|| {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs().to_string())
                    .unwrap_or_else(|_| "0".into())
            });
            let wal_path = std::path::Path::new(&wal_dir)
                .join(format!("{}_{}.jsonl", problem_stem, id));
            info!("[wal] using {:?}", wal_path);
            match TuringBus::with_wal_path(kernel, config, wal_path) {
                Ok(b) => b,
                Err(e) => {
                    error!("[wal] open failed: {} — falling back to in-memory", e);
                    TuringBus::new(
                        Kernel::new(),
                        BusConfig {
                            max_payload_chars: 1200,
                            max_payload_lines: 18,
                            forbidden_patterns: vec![
                                "native_decide".into(),
                                "decide".into(),
                                "omega".into(),
                                "#eval".into(),
                                "IO.Process".into(),
                                "IO.FS".into(),
                                "run_tac".into(),
                                "unsafe".into(),
                            ],
                        },
                    )
                }
            }
        } else {
            TuringBus::new(kernel, config)
        };

        Self {
            bus,
            chaintape_bundle,
            agent_keypairs,
            initial_balances_for_genesis_report,
            chaintape_preseed_enabled,
        }
    }
}

/// TB-18 Atom B Phase 2: write the synthetic L4 + L4.E pipeline-liveness gate
/// (TB-6 Atom 3 — synthetic TaskOpen + zero-stake WorkTx) plus the chain-level
/// `genesis_report.json` (TB-7R Deliverable C). Lifted verbatim from
/// `evaluator.rs::run_swarm` lines 1439-1562 — behavior is byte-identical to
/// the inline code; this is a pure mechanical extraction.
///
/// **Caller contract**: must hold an `&mut bus` whose sequencer was wired from
/// the same `bundle` reference passed here (i.e. `bus` and `bundle` come from
/// the same `SharedChain::from_env` call). `seed_id` is the chain-level
/// identifier used for the synthetic task_id (`smoke-{seed_id}`) and inline
/// run_id metadata. In single-task mode (`evaluator.rs::run_swarm`),
/// `seed_id == run_id` to preserve the exact tx_id digests the M0 retry
/// chain audit observed. In Phase 4 multi-task mode
/// (`comprehensive_arena.rs`), `seed_id` is a chain-level UUID minted at
/// chain start.
///
/// **Pre-condition**: caller has already:
///   1. Constructed `SharedChain::from_env(...)` (Phase 1)
///   2. Run any per-task preseed + arena-hook env-var processing (TB-7.7 D3
///      preseed + FORCE_* hooks; currently inline at evaluator.rs lines
///      706-1437; will move to per-task body in Phase 3 / drive_task)
///
/// **Post-condition**: chain contains 1 synthetic L4 (TaskOpen) + 1 synthetic
/// L4.E (zero-stake WorkTx; `synthetic_rejection_for_l4e_gate=true` label);
/// `<runtime_repo>/synthetic_rejection_label.json` exists; agent_audit_trail
/// records pair written to CAS + jsonl index; `<runtime_repo>/genesis_report.json`
/// written.
///
/// **Failure modes**: all preserve original inline-code behavior:
///   - synthetic TaskOpen submit fail → `error!()` log, continues
///   - synthetic WorkTx submit fail → `error!()` log, continues
///   - audit_trail write fail → `error!()` log, continues
///   - genesis_report write fail → `warn!()` log, continues (per TB-7R
///     Deliverable C "non-fatal — evidence collection continues, but
///     post-hoc audit must note absence")
///
/// `FC-trace: FC1-N34` — synthetic L4.E gate is an audit_tape input that the
/// post-hoc verifier separates from natural rejections via the
/// `synthetic_rejection_for_l4e_gate=true` label.
pub async fn write_synthetic_l4_l4e_gate_and_genesis_report(
    bus: &mut TuringBus,
    bundle: &ChaintapeBundle,
    initial_balances: &[(String, i64)],
    chaintape_preseed_enabled: bool,
    seed_id: &str,
) {
    let task_id_str = format!("smoke-{}", seed_id);
    let task_open = turingosv4::runtime::adapter::make_synthetic_task_open(
        &task_id_str,
        "tb6-smoke-sponsor",
        turingosv4::state::q_state::Hash::ZERO,
        "atom3-seed",
    );
    let task_open_tx_id =
        turingosv4::state::q_state::TxId(format!("taskopen-{}-atom3-seed", task_id_str));
    if let Err(e) = bus.submit_typed_tx(task_open).await {
        error!("[chaintape] synthetic TaskOpen submit failed: {e}");
    } else {
        info!("[chaintape] seeded synthetic TaskOpen for {}", task_id_str);
    }
    let bad_worktx = turingosv4::runtime::adapter::make_synthetic_worktx(
        &task_id_str,
        "tb6-smoke-agent",
        turingosv4::state::q_state::Hash::ZERO,
        0,
        "atom3-l4e-synthetic-rejection",
        true,
    );
    let bad_worktx_tx_id = turingosv4::state::q_state::TxId(format!(
        "worktx-{}-atom3-l4e-synthetic-rejection",
        task_id_str
    ));
    if let Err(e) = bus.submit_typed_tx(bad_worktx).await {
        error!("[chaintape] synthetic zero-stake WorkTx submit failed: {e}");
    } else {
        info!(
            "[chaintape] seeded synthetic zero-stake WorkTx \
             (synthetic_rejection_for_l4e_gate=true) for {}",
            task_id_str
        );
    }
    // Mark the synthetic-seed in the evidence dir so verify_chaintape (Atom 4)
    // can distinguish synthetic-rejection from natural rejection.
    let label_path = bundle.runtime_repo_path.join("synthetic_rejection_label.json");
    let _ = std::fs::write(
        &label_path,
        format!(
            r#"{{"synthetic_rejection_for_l4e_gate": true, "run_id": "{}", "atom": "TB-6 Atom 3", "rationale": "≥1 L4.E entry seeded via zero-stake WorkTx; per architect ruling 2026-05-01 § 3.6 Atom 3"}}"#,
            seed_id
        ),
    );

    // TB-6 Atom 5: write AgentProposalRecord pairs to CAS + index for both
    // synthetic envelopes. Each record carries the architect's 9 fields
    // + logical_t. The index links L4 / L4.E tx_id → CAS record CID.
    if let Err(e) = turingosv4::runtime::agent_audit_trail::write_synthetic_seed_audit_pair(
        &bundle.cas_path,
        &bundle.runtime_repo_path,
        seed_id,
        &task_open_tx_id,
        &bad_worktx_tx_id,
    ) {
        error!("[chaintape] Atom 5 audit-trail write failed: {e}");
    } else {
        info!(
            "[chaintape] Atom 5 audit-trail records written to CAS + indexed for {}",
            task_id_str
        );
    }

    // TB-7R Deliverable C (verdict 2026-05-01 §6.1): emit
    // `<runtime_repo>/genesis_report.json` so post-hoc audits can
    // verify the run's genesis preconditions (constitution_hash,
    // runtime_repo, cas_path, system_pubkey, agent_pubkeys path,
    // initial_balances) plus — when preseed is enabled — the
    // task_id / task_open_tx / escrow_lock_tx that established the
    // task and escrow on-chain.
    let preseed_task_id = if chaintape_preseed_enabled {
        Some(format!("task-{}", seed_id))
    } else {
        None
    };
    // TB-10 Atom 1+3: tx_id suffix depends on user-mode flag (mirrors the
    // make_real_*_signed_by suffix passed in lines above).
    let user_task_mode = std::env::var("TURINGOS_USER_TASK_MODE")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let preseed_task_open_tx = preseed_task_id.as_ref().map(|t| {
        if user_task_mode {
            format!("taskopen-{}-tb10-user-seed", t)
        } else {
            format!("taskopen-{}-tb7-7-d3-seed", t)
        }
    });
    let preseed_escrow_lock_tx = preseed_task_id.as_ref().map(|t| {
        if user_task_mode {
            format!("escrowlock-{}-tb10-user-escrow", t)
        } else {
            format!("escrowlock-{}-tb7-7-d3-escrow", t)
        }
    });
    let report = turingosv4::runtime::genesis_report::GenesisReport {
        constitution_hash:
            turingosv4::runtime::genesis_report::GenesisReport::hash_constitution_md(
                std::path::Path::new("constitution.md"),
            ),
        runtime_repo: bundle.runtime_repo_path.display().to_string(),
        cas_path: bundle.cas_path.display().to_string(),
        system_pubkey_hash:
            turingosv4::runtime::genesis_report::GenesisReport::hash_system_pubkey_manifest(
                &bundle.runtime_repo_path,
            ),
        agent_pubkeys_path: "agent_pubkeys.json".into(),
        initial_balances: initial_balances.to_vec(),
        task_id: preseed_task_id,
        task_open_tx: preseed_task_open_tx,
        escrow_lock_tx: preseed_escrow_lock_tx,
    };
    if let Err(e) = report.write_to_runtime_repo(&bundle.runtime_repo_path) {
        warn!(
            "[chaintape/d_c] genesis_report.json write failed: {e} (non-fatal — \
             evidence collection continues, but post-hoc audit must note absence)"
        );
    } else {
        info!(
            "[chaintape/d_c] genesis_report.json written to {:?}",
            bundle.runtime_repo_path
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TB-18 Atom B Phase 1: legacy in-memory mode (no env vars set) returns
    /// a SharedChain with `chaintape_bundle = None`, `agent_keypairs = None`,
    /// `initial_balances_for_genesis_report = empty`, `chaintape_preseed_enabled
    /// = false`. The TuringBus is the in-memory variant (constructed via
    /// `TuringBus::new`).
    ///
    /// This is the most basic invariant: from_env compiles + runs without
    /// crashing in the no-env-set path that other Phase 1 unit tests rely on.
    ///
    /// **Env-var hygiene**: this test is in the global `cargo test --workspace`
    /// pool which runs in parallel by default. Other tests in the same crate
    /// may set `TURINGOS_CHAINTAPE_PATH`/`TURINGOS_CHAINTAPE_PRESEED`/`WAL_DIR`.
    /// Per `feedback_env_var_test_lock`, mutually-exclusive env-var-driven
    /// tests need a static Mutex if they mutate process-global env. Here we
    /// only READ (no mutation), but to avoid flakes from concurrent writers
    /// we explicitly skip the assertion if any of the three env vars is set
    /// at the test entry point (the test then becomes a smoke-only that
    /// from_env doesn't panic).
    #[test]
    fn shared_chain_from_env_no_env_vars_set_legacy_mode() {
        // Skip if any of the chaintape/wal env vars is set — concurrent test
        // race; this test only validates the legacy no-env-set branch.
        if std::env::var("TURINGOS_CHAINTAPE_PATH").is_ok()
            || std::env::var("TURINGOS_CHAINTAPE_PRESEED").is_ok()
            || std::env::var("WAL_DIR").is_ok()
        {
            eprintln!(
                "[shared_chain_test] skipped (concurrent env-var writer in test pool); \
                 from_env legacy-mode path verified by smoke probes downstream"
            );
            return;
        }
        let chain = SharedChain::from_env("data/heldout/mathd_algebra_107.lean");
        assert!(chain.chaintape_bundle.is_none());
        assert!(chain.agent_keypairs.is_none());
        assert!(chain.initial_balances_for_genesis_report.is_empty());
        assert!(!chain.chaintape_preseed_enabled);
    }
}
