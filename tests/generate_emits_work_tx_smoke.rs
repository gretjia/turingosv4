//! Polymarket (2026-05-23 REVISED post-Codex/Karpathy audit) —
//! `turingos generate` end-to-end smoke + cold-restart replay.
//!
//! After `cmd_generate.rs` writes the ArtifactBundleManifest and the TDMA
//! judge passes + the internal test pipeline reports overall_pass=true, the
//! post-judge `emit_polymarket_market_for_session` MUST:
//!
//!   1. Read the workspace's `genesis_payload.toml` + parse the
//!      `[treasury]` + `[worker_wallets]` tables.
//!   2. Build a genesis QState via `runtime::adapter::genesis_with_balances`
//!      so treasury has 100_000µ plus worker/verifier/provider wallets.
//!   3. Open the canonical workspace ChainTape via
//!      `build_chaintape_sequencer_with_initial_q` (resume_existing_chain=true).
//!   4. Submit TaskOpen → EscrowLock → WorkTx → MarketSeed through the
//!      canonical sequencer; admissions land on
//!      `<workspace>/runtime_repo/refs/transitions/main` (NOT an ephemeral
//!      in-memory ledger — Constitution agent fix 2026-05-23 closure).
//!
//! Σ Coin conservation (architect §7.10 verbatim gate 2) holds at each
//! step because every preseed agent lives in the genesis QState; all
//! subsequent tx admissions go through the same kernel paths that
//! `constitution_polymarket_smoke.rs` covers.
//!
//! **Cold-restart replay (AC6)**: after the first `turingos generate`
//! completes, this test re-opens the workspace's chain (without re-running
//! the CLI) and verifies the post-state matches what was just committed.
//! This proves the chain is the canonical source of truth — same state
//! re-derives from the persisted bytes alone.

use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::thread;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    RejectionClass as L4ERejectionClass, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::system_keypair::{
    PinnedSystemPubkeys, SystemEpoch, SystemPublicKey,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    replay_full_transition_with_predicate_binding, Git2LedgerWriter, LedgerEntry, LedgerWriter,
    TxKind,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::runtime::artifact_bundle::{
    latest_artifact_bundle_cid_for_session, ARTIFACT_BUNDLE_SCHEMA_ID,
};
use turingosv4::runtime::bootstrap::parse_treasury_and_worker_preseed;
use turingosv4::runtime::proposal_telemetry::read_from_cas as read_proposal_telemetry;
use turingosv4::runtime::rejection_capsule::GENERATE_REJECTION_CAPSULE_SCHEMA_ID;
use turingosv4::runtime::PinnedPubkeyManifest;
use turingosv4::state::q_state::{AgentId, QState, TaskId, TaskMarketState, TxId};
use turingosv4::state::typed_tx::{AgentSignature, EventId, TypedTx};
use turingosv4::top_white::predicates::registry::{BootPredicateManifest, PredicateRegistry};

fn turingos_bin() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let candidates = [
        format!("{manifest_dir}/target/debug/turingos"),
        format!("{manifest_dir}/target/release/turingos"),
    ];
    for candidate in candidates.iter() {
        let path = PathBuf::from(candidate);
        if path.exists() {
            return path;
        }
    }
    panic!("turingos binary not found at any of {:?}", candidates);
}

/// Spin up a one-shot HTTP server on a random local port that returns the
/// supplied `response_body` to the first incoming request. The body must be
/// a valid SiliconFlow / OpenAI-compatible chat-completion JSON.
fn start_mock_llm_server(response_body: String) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    thread::spawn(move || {
        // Accept multiple connections in case the TDMA-bounded runner retries.
        for _ in 0..10 {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 8192];
                let _ = stream.read(&mut buf);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    response_body.len(),
                    response_body
                );
                let _ = stream.write_all(response.as_bytes());
                let _ = stream.flush();
            } else {
                break;
            }
        }
    });
    format!("http://127.0.0.1:{}", port)
}

/// Build a SiliconFlow chat-completion response that emits a single
/// self-contained `index.html` artifact (the happy-path shape).
fn happy_path_llm_response() -> String {
    let body = "### File: index.html\n```html\n<!DOCTYPE html><html><head><title>Polymarket Smoke</title><style>:root{--accent:#4e8b7a}body{font-family:'IBM Plex Sans',system-ui,sans-serif;background:#f8f6f1;color:#1a1a1a;line-height:1.6}h1{font-family:'Fraunces',Georgia,serif;color:var(--accent)}</style></head><body><main><h1>Polymarket smoke</h1><p>ok</p></main></body></html>\n```";
    let payload = serde_json::json!({
        "choices": [
            {
                "message": {
                    "role": "assistant",
                    "content": body,
                },
                "finish_reason": "stop"
            }
        ],
        "usage": {
            "prompt_tokens": 10,
            "completion_tokens": 100,
            "total_tokens": 110,
        }
    });
    payload.to_string()
}

fn no_files_llm_response() -> String {
    let payload = serde_json::json!({
        "choices": [
            {
                "message": {
                    "role": "assistant",
                    "content": "I cannot produce files for this request.",
                },
                "finish_reason": "stop"
            }
        ],
        "usage": {
            "prompt_tokens": 10,
            "completion_tokens": 8,
            "total_tokens": 18,
        }
    });
    payload.to_string()
}

fn assert_nonzero_agent_signature(signature: AgentSignature, label: &str) {
    assert!(
        signature.as_bytes().iter().any(|b| *b != 0),
        "{label} must carry a real non-zero agent signature"
    );
}

/// Walk every L4 entry in `<workspace>/runtime_repo` and decode the TypedTx
/// for each. Returns the (entry, decoded_tx) pairs.
fn walk_chain(runtime_repo_path: &std::path::Path, cas: &CasStore) -> Vec<(LedgerEntry, TypedTx)> {
    use turingosv4::bottom_white::ledger::transition_ledger::canonical_decode;
    let writer = Git2LedgerWriter::open(runtime_repo_path).expect("open Git2LedgerWriter");
    let n = writer.len();
    let mut out = Vec::with_capacity(n as usize);
    for t in 1..=n {
        let entry = writer.read_at(t).expect("read_at");
        let bytes = cas.get(&entry.tx_payload_cid).expect("cas.get");
        let tx: TypedTx = canonical_decode(&bytes).expect("decode TypedTx");
        out.push((entry, tx));
    }
    out
}

/// Cold-restart replay of the workspace's chain (no live sequencer). Mirrors
/// the canonical path that `verify_chaintape` takes and that
/// `runtime::build_chaintape_sequencer_with_initial_q`'s resume branch uses
/// under the hood — both call `replay_full_transition`.
fn replay_from_disk(runtime_repo_path: &std::path::Path, cas_path: &std::path::Path) -> QState {
    let initial_q_path = runtime_repo_path.join("initial_q_state.json");
    let initial_q_json = fs::read_to_string(&initial_q_path).expect("read initial_q_state.json");
    let initial_q: QState =
        serde_json::from_str(&initial_q_json).expect("parse initial_q_state.json");

    let writer = Git2LedgerWriter::open(runtime_repo_path).expect("open Git2LedgerWriter");
    let n = writer.len();
    let entries: Vec<LedgerEntry> = (1..=n)
        .map(|t| writer.read_at(t).expect("read_at"))
        .collect();

    let manifest_path = runtime_repo_path.join("pinned_pubkeys.json");
    let manifest_json = fs::read_to_string(&manifest_path).expect("read pinned_pubkeys.json");
    let manifest: PinnedPubkeyManifest =
        serde_json::from_str(&manifest_json).expect("parse pinned_pubkeys.json");
    let mut pinned = PinnedSystemPubkeys::new();
    for entry in &manifest.pubkeys {
        let bytes: Vec<u8> = (0..entry.pubkey_hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&entry.pubkey_hex[i..i + 2], 16).expect("hex"))
            .collect();
        let arr: [u8; 32] = bytes.as_slice().try_into().expect("32-byte pubkey");
        pinned.insert(
            SystemEpoch::new(entry.epoch),
            SystemPublicKey::from_bytes(arr),
        );
    }

    let cas = CasStore::open(cas_path).expect("open cas");
    let predicates = PredicateRegistry::from_boot_manifest(BootPredicateManifest::v8_production())
        .expect("v8 predicate manifest");
    let tools = ToolRegistry::new();

    replay_full_transition_with_predicate_binding(
        &initial_q,
        &entries,
        &cas,
        &cas,
        &pinned,
        &predicates,
        &tools,
    )
    .expect("replay_full_transition_with_predicate_binding")
}

#[test]
fn generate_emits_work_tx_and_market_seed_on_canonical_chain() {
    // Silence the unused-import warning on `Arc` — only retained so a future
    // edit reaching for `Arc::new(...)` on a sequencer handle won't need to
    // re-import.
    let _arc_keepalive: Arc<()> = Arc::new(());

    let tmp = tempfile::tempdir().expect("create temp parent");
    let ws = tmp.path().join("ws-polymarket-smoke");

    // ──────────────── Step 1: turingos init ────────────────
    let status = Command::new(turingos_bin())
        .arg("init")
        .arg("--project")
        .arg(&ws)
        .status()
        .expect("run init");
    assert!(status.success(), "turingos init must succeed");

    // ──────────────── Step 2: parse workspace genesis preseed ────────────────
    // After init, the workspace's `genesis_payload.toml` MUST carry the
    // `[treasury]` + `[worker_wallets]` tables (cmd_init template).
    let genesis_text =
        fs::read_to_string(ws.join("genesis_payload.toml")).expect("read workspace genesis");
    let preseed =
        parse_treasury_and_worker_preseed(&genesis_text).expect("preseed sections present");
    let treasury_entry = preseed
        .iter()
        .find(|(a, _)| a.0 == "treasury")
        .expect("treasury entry");
    assert_eq!(
        treasury_entry.1.micro_units(),
        100_000,
        "treasury 100_000µ preseed"
    );
    let worker_entry = preseed
        .iter()
        .find(|(a, _)| a.0 == "worker-alpha")
        .expect("worker-alpha entry");
    assert_eq!(
        worker_entry.1.micro_units(),
        10_000,
        "worker-alpha 10_000µ preseed"
    );

    // ──────────────── Step 3: write spec.md ────────────────
    let spec_content = "# Polymarket Smoke Spec\nMinimal HTML page.";
    fs::write(ws.join("spec.md"), spec_content).expect("write spec.md");

    // ──────────────── Step 4: stub LLM via local mock server ────────────────
    let endpoint = start_mock_llm_server(happy_path_llm_response());

    // ──────────────── Step 5: run turingos generate ────────────────
    let output = Command::new(turingos_bin())
        .arg("generate")
        .arg("--workspace")
        .arg(&ws)
        .arg("--entrypoint")
        .arg("index.html")
        .arg("--n-parallel-workers")
        .arg("3")
        .env("TURINGOS_SILICONFLOW_ENDPOINT", &endpoint)
        .env("SILICONFLOW_API_KEY", "mock-key")
        .output()
        .expect("run generate");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        panic!(
            "turingos generate failed.\n--- stdout ---\n{}\n--- stderr ---\n{}",
            stdout, stderr
        );
    }

    // ──────────────── Step 6: assert ArtifactBundleManifest committed ────────────────
    let bundle_cid_hex = latest_artifact_bundle_cid_for_session(&ws, "default")
        .expect("latest_artifact_bundle_cid_for_session ok")
        .expect("ArtifactBundleManifest must be committed for the session");
    assert_eq!(
        bundle_cid_hex.len(),
        64,
        "bundle cid is 64-char hex (sha256)"
    );
    let cas_dir = ws.join("cas");
    let store = CasStore::open(&cas_dir).expect("open cas");
    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
    let bundle_found = cids.iter().any(|c| {
        if c.hex() != bundle_cid_hex {
            return false;
        }
        store
            .metadata(c)
            .and_then(|m| m.schema_id.clone())
            .map(|s| s == ARTIFACT_BUNDLE_SCHEMA_ID)
            .unwrap_or(false)
    });
    assert!(
        bundle_found,
        "ArtifactBundleManifest with cid {bundle_cid_hex} must be in CAS"
    );

    // ──────────────── Step 7: assert canonical ChainTape contains 4 admissions ────────────────
    // Constitution-agent fix (2026-05-23 closure): admissions land on the
    // workspace's runtime_repo, NOT an ephemeral InMemoryLedger. The chain
    // MUST have 4 L4 entries in TaskOpen → EscrowLock → Work → MarketSeed
    // order.
    let runtime_repo = ws.join("runtime_repo");
    assert!(
        runtime_repo.join(".git").exists(),
        "runtime_repo must be a real git repo (canonical Git2LedgerWriter)"
    );
    let chain = walk_chain(&runtime_repo, &store);
    assert!(
        chain.len() >= 4,
        "chain must have ≥ 4 L4 entries (TaskOpen + EscrowLock + Work + MarketSeed); got {}",
        chain.len()
    );

    // Per-tx assertions on the last 4 entries (cold-start; no prior chain).
    // Per-call task_id format: pr1-<session_id>; session_id defaults to "default".
    let expected_task = TaskId("pr1-default".into());
    let expected_event = EventId(expected_task.clone());
    let expected_bundle_cid_bytes: [u8; 32] = {
        let mut out = [0u8; 32];
        for (i, byte_pair) in bundle_cid_hex.as_bytes().chunks(2).enumerate() {
            let hi = u8::from_str_radix(std::str::from_utf8(&[byte_pair[0]]).unwrap(), 16).unwrap();
            let lo = u8::from_str_radix(std::str::from_utf8(&[byte_pair[1]]).unwrap(), 16).unwrap();
            out[i] = (hi << 4) | lo;
        }
        out
    };

    // The 4 chain admissions for our session (chain may have other entries
    // from prior tests in CI; this smoke starts from an empty chain, so
    // we sanity-check the first 4 are ours).
    let kinds: Vec<TxKind> = chain.iter().map(|(e, _)| e.tx_kind).collect();
    assert!(
        kinds.windows(9).any(|w| {
            matches!(
                w,
                [
                    TxKind::TaskOpen,
                    TxKind::EscrowLock,
                    TxKind::Work,
                    TxKind::Work,
                    TxKind::Work,
                    TxKind::MarketSeed,
                    TxKind::Verify,
                    TxKind::FinalizeReward,
                    TxKind::EventResolve
                ]
            )
        }),
        "chain must contain the TaskOpen→EscrowLock→Work*3→MarketSeed→Verify→FinalizeReward→EventResolve sequence; \
         got kinds: {:?}",
        kinds
    );
    assert!(
        runtime_repo.join("agent_pubkeys.json").is_file(),
        "runtime_repo/agent_pubkeys.json must pin the signing agents"
    );

    // Decode the WorkTx candidates + assert all three preseeded workers
    // joined the market with distinct CAS-backed candidate bundles. The
    // delivered alpha bundle remains the user-visible artifact; beta/gamma
    // proposals are separate candidate manifests produced by their worker
    // prompt/evidence loops.
    let work_pairs: Vec<_> = chain
        .iter()
        .filter(|(e, _)| e.tx_kind == TxKind::Work)
        .collect();
    assert_eq!(work_pairs.len(), 3, "N=3 fan-out must emit 3 WorkTxs");
    let worker_agents: Vec<_> = work_pairs
        .iter()
        .filter_map(|(_, tx)| match tx {
            TypedTx::Work(work) => Some(work.agent_id.0.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(
        worker_agents,
        vec!["worker-alpha", "worker-beta", "worker-gamma"],
        "N=3 worker roster"
    );
    let proposal_cids: Vec<String> = work_pairs
        .iter()
        .filter_map(|(_, tx)| match tx {
            TypedTx::Work(work) => Some(work.proposal_cid.hex()),
            _ => None,
        })
        .collect();
    let unique_proposal_cids: std::collections::BTreeSet<_> =
        proposal_cids.iter().cloned().collect();
    assert_eq!(
        unique_proposal_cids.len(),
        3,
        "each N=3 worker must have its own ProposalTelemetry CID; got {proposal_cids:?}"
    );
    let mut artifact_cids = Vec::new();
    for (_, tx) in &work_pairs {
        let TypedTx::Work(work) = tx else {
            panic!("Work ledger entry did not decode as TypedTx::Work");
        };
        assert!(
            work.read_set
                .iter()
                .any(|key| key.0 == format!("cas.proposal_telemetry:{}", work.proposal_cid.hex())),
            "real generate WorkTx.read_set must bind its ProposalTelemetry CAS object, got {:?}",
            work.read_set
        );
        assert!(
            work.write_set.iter().any(|key| {
                key.0.starts_with(&format!(
                    "task_output:{}:{}:",
                    work.task_id.0, work.agent_id.0
                ))
            }),
            "real generate WorkTx.write_set must name the task/agent output target, got {:?}",
            work.write_set
        );
        assert!(
            work.read_set.iter().all(|key| key.0 != "k.read")
                && work.write_set.iter().all(|key| key.0 != "k.write"),
            "real generate WorkTx must not carry synthetic fixture read/write placeholders"
        );
    }
    for cid_hex in &proposal_cids {
        let mut bytes = [0u8; 32];
        for (i, byte_pair) in cid_hex.as_bytes().chunks(2).enumerate() {
            let hi = u8::from_str_radix(std::str::from_utf8(&[byte_pair[0]]).unwrap(), 16).unwrap();
            let lo = u8::from_str_radix(std::str::from_utf8(&[byte_pair[1]]).unwrap(), 16).unwrap();
            bytes[i] = (hi << 4) | lo;
        }
        let cid = Cid(bytes);
        let meta = store
            .metadata(&cid)
            .unwrap_or_else(|| panic!("proposal CID {cid_hex} must resolve in CAS"));
        assert_eq!(
            meta.schema_id.as_deref(),
            Some("turingosv4.proposal_telemetry.v1"),
            "WorkTx.proposal_cid {cid_hex} must be ProposalTelemetry for verify_chaintape Gate 5"
        );
        let telemetry = read_proposal_telemetry(&store, &cid)
            .unwrap_or_else(|e| panic!("proposal telemetry {cid_hex} must decode: {e}"));
        artifact_cids.push(telemetry.proposal_artifact_cid.hex());
        let artifact_meta = store
            .metadata(&telemetry.proposal_artifact_cid)
            .unwrap_or_else(|| {
                panic!(
                    "ProposalTelemetry.proposal_artifact_cid {} must resolve in CAS",
                    telemetry.proposal_artifact_cid.hex()
                )
            });
        assert_eq!(
            artifact_meta.schema_id.as_deref(),
            Some(ARTIFACT_BUNDLE_SCHEMA_ID),
            "proposal artifact {} must be an ArtifactBundleManifest",
            telemetry.proposal_artifact_cid.hex()
        );
    }
    let unique_artifact_cids: std::collections::BTreeSet<_> =
        artifact_cids.iter().cloned().collect();
    assert_eq!(
        unique_artifact_cids.len(),
        3,
        "each N=3 worker must have its own ArtifactBundleManifest; got {artifact_cids:?}"
    );
    assert!(
        artifact_cids.contains(&bundle_cid_hex),
        "the delivered session bundle must be one of the worker proposal artifacts"
    );
    let work_pair = work_pairs[0];
    if let TypedTx::Work(work) = &work_pair.1 {
        assert_eq!(work.task_id, expected_task, "WorkTx.task_id");
        assert_eq!(
            work.agent_id,
            AgentId("worker-alpha".into()),
            "WorkTx.agent_id"
        );
        assert_ne!(
            work.proposal_cid.0, expected_bundle_cid_bytes,
            "worker-alpha WorkTx.proposal_cid MUST be ProposalTelemetry, not the raw ArtifactBundleManifest CID"
        );
        let telemetry = read_proposal_telemetry(&store, &work.proposal_cid)
            .expect("worker-alpha WorkTx.proposal_cid must decode as ProposalTelemetry");
        assert_eq!(
            telemetry.proposal_artifact_cid.0, expected_bundle_cid_bytes,
            "ProposalTelemetry.proposal_artifact_cid MUST equal delivered ArtifactBundleManifest.cid"
        );
        assert_nonzero_agent_signature(work.signature, "WorkTx.signature");
    } else {
        panic!("Work entry did not decode as TypedTx::Work");
    }

    // Decode the MarketSeed + assert it targets our event.
    let seed_pair = chain
        .iter()
        .find(|(e, _)| e.tx_kind == TxKind::MarketSeed)
        .expect("at least one MarketSeed entry");
    if let TypedTx::MarketSeed(seed) = &seed_pair.1 {
        assert_eq!(seed.event_id, expected_event, "MarketSeedTx.event_id");
        assert_eq!(
            seed.provider,
            AgentId("market-provider".into()),
            "MarketSeedTx.provider"
        );
        assert_nonzero_agent_signature(seed.signature, "MarketSeedTx.signature");
    } else {
        panic!("MarketSeed entry did not decode as TypedTx::MarketSeed");
    }

    let verify_pair = chain
        .iter()
        .find(|(e, _)| e.tx_kind == TxKind::Verify)
        .expect("at least one Verify entry");
    if let TypedTx::Verify(verify) = &verify_pair.1 {
        assert_nonzero_agent_signature(verify.signature, "VerifyTx.signature");
    } else {
        panic!("Verify entry did not decode as TypedTx::Verify");
    }

    // ──────────────── Step 8: post-state derived from live admission ────────────────
    // Replay the chain from `initial_q_state.json` + L4 entries (same path
    // as a cold reader / `verify_chaintape`).
    let replayed = replay_from_disk(&runtime_repo, &cas_dir);
    assert!(
        replayed
            .economic_state_t
            .task_markets_t
            .0
            .contains_key(&expected_task),
        "EconomicState.task_markets_t MUST contain pr1-default post-admission"
    );
    assert!(
        replayed
            .economic_state_t
            .stakes_t
            .0
            .iter()
            .any(
                |(_tx, entry)| entry.staker == AgentId("worker-alpha".into())
                    && entry.task_id == expected_task
            ),
        "EconomicState.stakes_t MUST contain worker-alpha's stake on the task"
    );
    assert!(
        replayed
            .economic_state_t
            .conditional_collateral_t
            .0
            .contains_key(&expected_event),
        "EconomicState.conditional_collateral_t MUST be seeded by MarketSeed"
    );
    let task_market = replayed
        .economic_state_t
        .task_markets_t
        .0
        .get(&expected_task)
        .expect("task market must exist after replay");
    assert_eq!(
        task_market.state,
        TaskMarketState::Finalized,
        "EventResolve must finalize the task market; winner must be derived from replayed chain"
    );

    // Σ Coin conservation: assert_total_ctf_conserved-equivalent — sum
    // balances + escrow + conditional_collateral should equal the genesis
    // total (150_000µ from treasury 100k + 3 workers + verifier + provider).
    let bal_total: i64 = replayed
        .economic_state_t
        .balances_t
        .0
        .values()
        .map(|m| m.micro_units())
        .sum();
    let escrow_total: i64 = replayed
        .economic_state_t
        .task_markets_t
        .0
        .values()
        .map(|e| e.total_escrow.micro_units())
        .sum();
    let collateral_total: i64 = replayed
        .economic_state_t
        .conditional_collateral_t
        .0
        .values()
        .map(|m| m.micro_units())
        .sum();
    let stakes_total: i64 = replayed
        .economic_state_t
        .stakes_t
        .0
        .values()
        .map(|s| s.amount.micro_units())
        .sum();
    let conserved = bal_total + escrow_total + collateral_total + stakes_total;
    assert_eq!(
        conserved, 150_000,
        "Σ Coin conservation: balances + escrow + collateral + stakes MUST = 150_000µ \
         (genesis treasury 100k + 3 workers + verifier + provider). bal={bal_total} escrow={escrow_total} \
         collateral={collateral_total} stakes={stakes_total}"
    );

    // ──────────────── Step 9: cold-restart replay invariance (AC6) ────────────────
    // Re-open the chain a SECOND time (simulating a fresh process / process
    // crash + restart). The resulting QState MUST be byte-identical to the
    // first replay — proves the chain is the canonical source of truth.
    let replayed_again = replay_from_disk(&runtime_repo, &cas_dir);
    assert_eq!(
        replayed.economic_state_t, replayed_again.economic_state_t,
        "cold-restart replay MUST yield byte-identical EconomicState"
    );
    assert_eq!(
        replayed.state_root_t, replayed_again.state_root_t,
        "cold-restart replay MUST yield identical state_root_t"
    );

    // Silence unused-import warning for TxId.
    let _ = TxId("_".into());
}

/// Codex P1 regression test (2026-05-23): exercise the exact pattern the web
/// flow uses, where `src/web/generate.rs` shells out with
/// `turingos generate --workspace <root>/sessions/<session_id>`.
///
/// The canonical Polymarket chain MUST land at the ROOT (`<root>/runtime_repo`),
/// NOT in the session subdir (`<root>/sessions/<id>/runtime_repo`), so that
/// `src/web/market_view.rs` — which reads `<root>/runtime_repo` — sees the
/// admissions. Before the `find_root_workspace` fix, the Polymarket code read
/// genesis + wrote runtime_repo under the `--workspace` arg verbatim and the
/// web endpoint returned 404.
///
/// This test fails (a) if `genesis_payload.toml` is read from the wrong
/// path, (b) if `runtime_repo` is created under the session subdir, OR
/// (c) if no L4 entries land at the root chain.
#[test]
fn generate_from_session_subdir_writes_chain_to_root_workspace() {
    let tmp = tempfile::tempdir().expect("create temp parent");
    let root_ws = tmp.path().join("ws-root");

    // turingos init at the ROOT.
    let status = Command::new(turingos_bin())
        .arg("init")
        .arg("--project")
        .arg(&root_ws)
        .status()
        .expect("run init");
    assert!(status.success(), "turingos init must succeed");

    // Simulate the web flow: create `<root>/sessions/<id>/`, copy
    // `turingos.toml` + write `spec.md` into the session subdir. Then call
    // `turingos generate --workspace <session_dir>` exactly like
    // `src/web/generate.rs` does.
    let session_id = "websmoke-12345";
    let session_dir = root_ws.join("sessions").join(session_id);
    fs::create_dir_all(&session_dir).expect("mkdir session_dir");
    fs::copy(
        root_ws.join("turingos.toml"),
        session_dir.join("turingos.toml"),
    )
    .expect("copy turingos.toml into session_dir");
    fs::write(
        session_dir.join("spec.md"),
        "# Web Flow Spec\nMinimal HTML.",
    )
    .expect("write spec.md into session_dir");

    let endpoint = start_mock_llm_server(happy_path_llm_response());

    let output = Command::new(turingos_bin())
        .arg("generate")
        .arg("--workspace")
        .arg(&session_dir) // session subdir, matching web flow
        .arg("--entrypoint")
        .arg("index.html")
        .env("TURINGOS_SILICONFLOW_ENDPOINT", &endpoint)
        .env("SILICONFLOW_API_KEY", "mock-key")
        .output()
        .expect("run generate from session subdir");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !output.status.success() {
        panic!(
            "turingos generate (web-mode) failed.\n--- stdout ---\n{}\n--- stderr ---\n{}",
            stdout, stderr
        );
    }

    // The chain MUST land at the ROOT, not in the session subdir.
    let root_runtime_repo = root_ws.join("runtime_repo");
    let session_runtime_repo = session_dir.join("runtime_repo");
    assert!(
        root_runtime_repo.exists(),
        "runtime_repo MUST be created at ROOT workspace ({}), not session subdir",
        root_runtime_repo.display()
    );
    assert!(
        !session_runtime_repo.exists(),
        "runtime_repo MUST NOT be created in session subdir ({}); \
         find_root_workspace should walk up to the root",
        session_runtime_repo.display()
    );

    // Verify the chain at the ROOT has the Polymarket admissions.
    let root_cas_dir = root_ws.join("cas");
    let root_cas = CasStore::open(&root_cas_dir).expect("open root cas");
    let entries = walk_chain(&root_runtime_repo, &root_cas);
    assert!(
        entries.len() >= 4,
        "expected ≥4 L4 entries at ROOT chain (TaskOpen + EscrowLock + WorkTx + optional settlement), got {}",
        entries.len()
    );
    let root_work = entries
        .iter()
        .find_map(|(entry, tx)| {
            if entry.tx_kind == TxKind::Work {
                match tx {
                    TypedTx::Work(work) => Some(work),
                    _ => None,
                }
            } else {
                None
            }
        })
        .expect("root chain must contain a WorkTx");
    root_cas
        .get(&root_work.proposal_cid)
        .expect("root CAS must resolve the WorkTx proposal CID emitted from a session subdir");

    // Replay at the root must succeed (proves canonical state landed there).
    let replayed = replay_from_disk(&root_runtime_repo, &root_cas_dir);
    let task_id = TaskId(format!("pr1-{session_id}"));
    assert!(
        replayed
            .economic_state_t
            .task_markets_t
            .0
            .contains_key(&task_id),
        "task_markets_t at ROOT MUST contain pr1-{session_id} entry"
    );
}

#[test]
fn generate_no_files_failure_emits_rejected_worktx_on_canonical_chain() {
    let tmp = tempfile::tempdir().expect("create temp parent");
    let ws = tmp.path().join("ws-polymarket-all-reject");

    let status = Command::new(turingos_bin())
        .arg("init")
        .arg("--project")
        .arg(&ws)
        .status()
        .expect("run init");
    assert!(status.success(), "turingos init must succeed");

    fs::write(ws.join("spec.md"), "# Rejection Spec\nPlease make an app.").expect("write spec.md");
    let endpoint = start_mock_llm_server(no_files_llm_response());

    let output = Command::new(turingos_bin())
        .arg("generate")
        .arg("--workspace")
        .arg(&ws)
        .arg("--entrypoint")
        .arg("index.html")
        .env("TURINGOS_SILICONFLOW_ENDPOINT", &endpoint)
        .env("SILICONFLOW_API_KEY", "mock-key")
        .output()
        .expect("run generate");

    assert!(
        !output.status.success(),
        "no-files response must fail generate so the rejected WorkTx path is exercised"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("polymarket_rejected_worktx_task_id=pr1-default"),
        "failure footer must report rejected WorkTx admission; stderr={stderr}"
    );

    let runtime_repo = ws.join("runtime_repo");
    let cas_dir = ws.join("cas");
    let cas = CasStore::open(&cas_dir).expect("open root cas");
    let chain = walk_chain(&runtime_repo, &cas);
    let kinds: Vec<TxKind> = chain.iter().map(|(entry, _)| entry.tx_kind).collect();
    assert_eq!(
        kinds,
        vec![
            TxKind::PredicateBindingActivate,
            TxKind::MapReduceTick,
            TxKind::TaskOpen,
            TxKind::EscrowLock
        ],
        "all-rejected market must advance boot activation + FC2 tick + TaskOpen/EscrowLock on L4; WorkTx rejection belongs to L4.E"
    );

    let rejection_path = runtime_repo.join("rejections.jsonl");
    let rejection_writer =
        RejectionEvidenceWriter::open_jsonl(rejection_path).expect("open L4.E rejection log");
    rejection_writer
        .verify_chain()
        .expect("L4.E rejection hash chain verifies");
    let work_rejections: Vec<_> = rejection_writer
        .records()
        .iter()
        .filter(|record| record.tx_kind == TxKind::Work)
        .collect();
    assert_eq!(
        work_rejections.len(),
        1,
        "failed primary candidate must emit exactly one rejected WorkTx"
    );
    let rejected = work_rejections[0];
    assert_eq!(
        rejected.rejection_class,
        L4ERejectionClass::PredicateFailed,
        "predicate=false WorkTx must route to L4.E PredicateFailed"
    );

    let payload_bytes = cas
        .get(&rejected.tx_payload_cid)
        .expect("rejected WorkTx payload must resolve in root CAS");
    let rejected_tx: TypedTx =
        turingosv4::bottom_white::ledger::transition_ledger::canonical_decode(&payload_bytes)
            .expect("decode rejected WorkTx payload");
    let work = match rejected_tx {
        TypedTx::Work(work) => work,
        other => panic!("expected rejected TypedTx::Work, got {other:?}"),
    };
    assert_eq!(work.agent_id, AgentId("worker-alpha".into()));
    assert_eq!(work.task_id, TaskId("pr1-default".into()));
    assert!(
        work.read_set
            .iter()
            .any(|key| key.0 == format!("cas.proposal_telemetry:{}", work.proposal_cid.hex())),
        "rejected real WorkTx.read_set must bind its ProposalTelemetry CAS object, got {:?}",
        work.read_set
    );
    assert!(
        work.write_set.iter().any(|key| {
            key.0.starts_with(&format!(
                "task_output:{}:{}:",
                work.task_id.0, work.agent_id.0
            ))
        }),
        "rejected real WorkTx.write_set must name the task/agent output target, got {:?}",
        work.write_set
    );
    assert!(
        work.read_set.iter().all(|key| key.0 != "k.read")
            && work.write_set.iter().all(|key| key.0 != "k.write"),
        "rejected real WorkTx must not carry synthetic fixture read/write placeholders"
    );
    assert_nonzero_agent_signature(work.signature, "rejected WorkTx.signature");

    let telemetry = read_proposal_telemetry(&cas, &work.proposal_cid)
        .expect("rejected WorkTx proposal_cid must decode as ProposalTelemetry");
    assert_eq!(telemetry.agent_id, AgentId("worker-alpha".into()));
    assert_eq!(
        telemetry.candidate_tactic, "generate-artifact-reject",
        "rejected WorkTx telemetry must declare rejected candidate tactic"
    );
    let rejection_meta = cas
        .metadata(&telemetry.proposal_artifact_cid)
        .expect("rejection proposal payload must resolve in root CAS");
    assert_eq!(
        rejection_meta.schema_id.as_deref(),
        Some(GENERATE_REJECTION_CAPSULE_SCHEMA_ID),
        "no-files rejected proposal payload must be the GenerateRejectionCapsule"
    );
}

#[test]
fn generate_retry_after_rejected_worktx_finalizes_same_session_market() {
    let tmp = tempfile::tempdir().expect("create temp parent");
    let ws = tmp.path().join("ws-polymarket-retry-after-reject");

    let status = Command::new(turingos_bin())
        .arg("init")
        .arg("--project")
        .arg(&ws)
        .status()
        .expect("run init");
    assert!(status.success(), "turingos init must succeed");

    fs::write(ws.join("spec.md"), "# Retry Spec\nPlease make an app.").expect("write spec.md");

    let reject_endpoint = start_mock_llm_server(no_files_llm_response());
    let first = Command::new(turingos_bin())
        .arg("generate")
        .arg("--workspace")
        .arg(&ws)
        .arg("--entrypoint")
        .arg("index.html")
        .env("TURINGOS_SILICONFLOW_ENDPOINT", &reject_endpoint)
        .env("SILICONFLOW_API_KEY", "mock-key")
        .output()
        .expect("run first rejected generate");
    assert!(
        !first.status.success(),
        "first no-files response must fail so retry starts from an Open market"
    );

    let retry_endpoint = start_mock_llm_server(happy_path_llm_response());
    let second = Command::new(turingos_bin())
        .arg("generate")
        .arg("--workspace")
        .arg(&ws)
        .arg("--entrypoint")
        .arg("index.html")
        .env("TURINGOS_SILICONFLOW_ENDPOINT", &retry_endpoint)
        .env("SILICONFLOW_API_KEY", "mock-key")
        .output()
        .expect("run second successful generate");
    if !second.status.success() {
        panic!(
            "retry generate must succeed and finalize the existing Open market.\n--- stdout ---\n{}\n--- stderr ---\n{}",
            String::from_utf8_lossy(&second.stdout),
            String::from_utf8_lossy(&second.stderr)
        );
    }

    let runtime_repo = ws.join("runtime_repo");
    let cas_dir = ws.join("cas");
    let cas = CasStore::open(&cas_dir).expect("open root cas");
    let chain = walk_chain(&runtime_repo, &cas);
    let kinds: Vec<TxKind> = chain.iter().map(|(entry, _)| entry.tx_kind).collect();
    assert_eq!(
        kinds.iter().filter(|kind| **kind == TxKind::TaskOpen).count(),
        1,
        "retry must reuse the existing TaskOpen instead of reopening the same task; kinds={kinds:?}"
    );
    assert_eq!(
        kinds
            .iter()
            .filter(|kind| **kind == TxKind::EscrowLock)
            .count(),
        1,
        "retry must reuse the existing escrow instead of double-locking bounty; kinds={kinds:?}"
    );
    assert!(
        kinds.windows(7).any(|w| {
            matches!(
                w,
                [
                    TxKind::TaskOpen,
                    TxKind::EscrowLock,
                    TxKind::Work,
                    TxKind::MarketSeed,
                    TxKind::Verify,
                    TxKind::FinalizeReward,
                    TxKind::EventResolve
                ]
            )
        }),
        "retry path must advance Open market to finalized via Work→MarketSeed→Verify→FinalizeReward→EventResolve; got {kinds:?}"
    );

    let replayed = replay_from_disk(&runtime_repo, &cas_dir);
    let task_id = TaskId("pr1-default".into());
    let market = replayed
        .economic_state_t
        .task_markets_t
        .0
        .get(&task_id)
        .expect("task market must exist after retry replay");
    assert_eq!(
        market.state,
        TaskMarketState::Finalized,
        "same-session retry must derive finalized winner from the canonical chain"
    );

    let rejection_writer =
        RejectionEvidenceWriter::open_jsonl(runtime_repo.join("rejections.jsonl"))
            .expect("open L4.E rejection log");
    let rejected_work_count = rejection_writer
        .records()
        .iter()
        .filter(|record| record.tx_kind == TxKind::Work)
        .count();
    assert_eq!(
        rejected_work_count, 1,
        "the original failed candidate must remain as exactly one L4.E WorkTx rejection"
    );
}
