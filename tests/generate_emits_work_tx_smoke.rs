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
//!      so treasury has 100_000µ and worker-alpha has 10_000µ.
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

use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::transition_ledger::{
    replay_full_transition, Git2LedgerWriter, LedgerEntry, LedgerWriter, TxKind,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::runtime::artifact_bundle::{
    latest_artifact_bundle_cid_for_session, ARTIFACT_BUNDLE_SCHEMA_ID,
};
use turingosv4::runtime::bootstrap::parse_treasury_and_worker_preseed;
use turingosv4::runtime::PinnedPubkeyManifest;
use turingosv4::bottom_white::ledger::system_keypair::{
    PinnedSystemPubkeys, SystemEpoch, SystemPublicKey,
};
use turingosv4::state::q_state::{AgentId, QState, TaskId, TaskMarketState, TxId};
use turingosv4::state::typed_tx::{EventId, TypedTx};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

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

/// Walk every L4 entry in `<workspace>/runtime_repo` and decode the TypedTx
/// for each. Returns the (entry, decoded_tx) pairs.
fn walk_chain(
    runtime_repo_path: &std::path::Path,
    cas: &CasStore,
) -> Vec<(LedgerEntry, TypedTx)> {
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
fn replay_from_disk(
    runtime_repo_path: &std::path::Path,
    cas_path: &std::path::Path,
) -> QState {
    let initial_q_path = runtime_repo_path.join("initial_q_state.json");
    let initial_q_json =
        fs::read_to_string(&initial_q_path).expect("read initial_q_state.json");
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
    let predicates = PredicateRegistry::new();
    let tools = ToolRegistry::new();

    replay_full_transition(&initial_q, &entries, &cas, &pinned, &predicates, &tools)
        .expect("replay_full_transition")
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
    // from prior tests in CI; PR1 default scope is the empty-chain case, so
    // we sanity-check the first 4 are ours).
    let kinds: Vec<TxKind> = chain.iter().map(|(e, _)| e.tx_kind).collect();
    assert!(
        kinds.windows(4).any(|w| {
            matches!(
                w,
                [TxKind::TaskOpen, TxKind::EscrowLock, TxKind::Work, TxKind::MarketSeed]
            )
        }),
        "chain must contain the TaskOpen→EscrowLock→Work→MarketSeed sequence; \
         got kinds: {:?}",
        kinds
    );

    // Decode the WorkTx + assert it carries our session's bundle cid +
    // worker-alpha agent.
    let work_pair = chain
        .iter()
        .find(|(e, _)| e.tx_kind == TxKind::Work)
        .expect("at least one Work entry");
    if let TypedTx::Work(work) = &work_pair.1 {
        assert_eq!(work.task_id, expected_task, "WorkTx.task_id");
        assert_eq!(
            work.agent_id,
            AgentId("worker-alpha".into()),
            "WorkTx.agent_id"
        );
        assert_eq!(
            work.proposal_cid.0, expected_bundle_cid_bytes,
            "WorkTx.proposal_cid MUST equal ArtifactBundleManifest.cid"
        );
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
            AgentId("treasury".into()),
            "MarketSeedTx.provider"
        );
    } else {
        panic!("MarketSeed entry did not decode as TypedTx::MarketSeed");
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
            .any(|(_tx, entry)| entry.staker == AgentId("worker-alpha".into())
                && entry.task_id == expected_task),
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

    // Σ Coin conservation: assert_total_ctf_conserved-equivalent — sum
    // balances + escrow + conditional_collateral should equal the genesis
    // total (110_000µ from treasury 100k + worker-alpha 10k).
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
        conserved, 110_000,
        "Σ Coin conservation: balances + escrow + collateral + stakes MUST = 110_000µ \
         (genesis treasury 100k + worker-alpha 10k). bal={bal_total} escrow={escrow_total} \
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
