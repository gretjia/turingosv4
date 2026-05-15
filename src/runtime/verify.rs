//! TB-6 Atom 4 — `verify_chaintape` replay verifier (library).
//!
//! Re-opens a runtime_repo + cas + pinned_pubkeys.json and replays the L4
//! chain entry-by-entry through `replay_full_transition` (the I-DETHASH
//! witness from CO1.7-impl A4). Reconstructs `QState` (including
//! `EconomicState`) from L4 alone — L4.E is **evidence-only**, not state-bearing
//! (Inv 7). Verifies every entry's `system_signature` against the persisted
//! `pinned_pubkeys.json` manifest.
//!
//! Architect ruling 2026-05-01 § 3.5 deliverable: `replay_report.json` with the
//! 7 mandated boolean indicators:
//! - `l4_entries`
//! - `l4e_entries`
//! - `ledger_root_verified`
//! - `system_signatures_verified`
//! - `state_reconstructed`
//! - `economic_state_reconstructed`
//! - `cas_payloads_retrievable`
//!
//! Per architect § 3.6 Atom 4 + ruling D2 (1)-(7): chain-backed smoke from
//! TB-6 onward must be replayable. This module is the structural witness.
//!
//! Driven by:
//! - `src/bin/verify_chaintape.rs` — CLI thin wrapper
//! - `tests/tb_6_verify_chaintape.rs` — I90 integration test
//!
//! Initial QState resolution:
//! - If `<runtime_repo>/initial_q_state.json` exists, deserialize it.
//! - Else default to `QState::genesis()` (matches Atom 3 smoke evidence).
//!
//! Bounded by `RejectionEvidenceWriter::open_jsonl` which validates the
//! L4.E `prev_hash → hash` chain on load — tamper any byte of any line and
//! the open call returns `RejectionEvidenceError::ChainBroken { at }`.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::store::CasStore;
use crate::bottom_white::ledger::rejection_evidence::{
    RejectionEvidenceError, RejectionEvidenceWriter,
};
use crate::bottom_white::ledger::system_keypair::{
    PinnedSystemPubkeys, SystemEpoch, SystemPublicKey,
};
use crate::bottom_white::ledger::transition_ledger::{
    replay_full_transition, Git2LedgerWriter, LedgerEntry, LedgerWriter, LedgerWriterError,
    ReplayError,
};
use crate::bottom_white::tools::registry::ToolRegistry;
use crate::runtime::PinnedPubkeyManifest;
use crate::state::q_state::{Hash, QState};
use crate::top_white::predicates::registry::PredicateRegistry;

const PINNED_PUBKEYS_FILENAME: &str = "pinned_pubkeys.json";
const INITIAL_Q_STATE_FILENAME: &str = "initial_q_state.json";
const REJECTIONS_JSONL_FILENAME: &str = "rejections.jsonl";

// ── Errors ──────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — verify_chaintape error class.
///
/// Distinct from `ReplayError`: this covers I/O / config / manifest issues
/// that prevent replay from even starting (vs. mid-chain divergence which is
/// `ReplayError`-shaped).
#[derive(Debug)]
pub enum VerifyError {
    Io(std::io::Error),
    LedgerWriter(LedgerWriterError),
    Cas(String),
    PinnedPubkeysMissing(PathBuf),
    PinnedPubkeysParse(String),
    InitialQStateParse(String),
    PubkeyDecode(String),
    L4eOpen(RejectionEvidenceError),
}

impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::LedgerWriter(e) => write!(f, "ledger writer error: {e}"),
            Self::Cas(e) => write!(f, "cas error: {e}"),
            Self::PinnedPubkeysMissing(p) => {
                write!(f, "pinned_pubkeys.json not found at {p:?}")
            }
            Self::PinnedPubkeysParse(s) => write!(f, "pinned_pubkeys.json parse failed: {s}"),
            Self::InitialQStateParse(s) => write!(f, "initial_q_state.json parse failed: {s}"),
            Self::PubkeyDecode(s) => write!(f, "pubkey hex decode failed: {s}"),
            Self::L4eOpen(e) => write!(f, "rejections.jsonl open / chain-verify failed: {e}"),
        }
    }
}

impl std::error::Error for VerifyError {}

impl From<std::io::Error> for VerifyError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<LedgerWriterError> for VerifyError {
    fn from(e: LedgerWriterError) -> Self {
        Self::LedgerWriter(e)
    }
}

// ── Report shape (replay_report.json wire format) ───────────────────────────

/// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — replay_report.json wire format.
///
/// Stable JSON shape consumed by the smoke evidence dir + CI gates. The 7
/// architect-mandated indicators are top-level fields; richer detail
/// (final state/ledger root hex, classification of any replay error) is
/// captured under `detail` so downstream tooling can drill in without
/// breaking the headline contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayReport {
    /// Number of L4 entries (length of `refs/transitions/main`).
    pub l4_entries: u64,
    /// Number of L4.E entries (length of `rejections.jsonl`).
    pub l4e_entries: u64,
    /// True iff every entry's `parent_ledger_root` chains to the previous
    /// `resulting_ledger_root` and the `append()` fold is byte-stable.
    pub ledger_root_verified: bool,
    /// True iff every entry's `system_signature` verifies against the
    /// persisted pinned-pubkey manifest at the entry's epoch.
    pub system_signatures_verified: bool,
    /// True iff replay produced a `QState` (no `dispatch_transition` or
    /// state-root divergence). Empty chain (`l4_entries == 0`) → `true`.
    pub state_reconstructed: bool,
    /// True iff the replayed `QState.economic_state_t` is consistent with
    /// the chain (i.e., replay completed without error). Currently coupled
    /// to `state_reconstructed`; future work may split when economic-only
    /// replay paths are added (NodeMarket, RSP-M).
    pub economic_state_reconstructed: bool,
    /// True iff every L4 entry's `tx_payload_cid` was retrievable from CAS.
    pub cas_payloads_retrievable: bool,
    /// **TB-7 Atom 4 NEW**: True iff every L4 WorkTx / VerifyTx entry's
    /// `AgentSignature` verifies against the per-run `agent_pubkeys.json`
    /// manifest. Empty chain or chain with no Work/Verify entries → `true`
    /// (no agent signatures to verify ≠ failure).
    ///
    /// This is the Gate 4 evidence (TB-7 charter §8): all WorkTx
    /// signatures verify against agent_pubkeys.json. False on any
    /// signature mismatch (tampering, key drift, unknown agent_id).
    pub agent_signatures_verified: bool,
    /// **TB-7 Atom 4 NEW**: True iff every L4 WorkTx entry's
    /// `proposal_cid` resolves to a CAS-resident `ProposalTelemetry`
    /// object. Empty chain or chain with zero Work entries → `true`.
    ///
    /// This is the Gate 5 evidence (TB-7 charter §8): every
    /// `WorkTx.proposal_cid` resolves to a CAS `ProposalTelemetry`
    /// object with the §4.5 schema. False if any WorkTx points to a
    /// CID that's missing or decodes to non-ProposalTelemetry bytes.
    pub proposal_telemetry_cas_retrievable: bool,
    /// Run-id from `pinned_pubkeys.json` manifest (echoed for forensics).
    pub run_id: String,
    /// Epoch from `pinned_pubkeys.json` manifest.
    pub epoch: u64,
    /// Detail block — non-blocking forensic data.
    pub detail: ReplayReportDetail,
}

/// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — replay_report.json detail block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayReportDetail {
    pub final_state_root_hex: Option<String>,
    pub final_ledger_root_hex: Option<String>,
    /// Lowercase 40-char git commit OID at HEAD of `refs/transitions/main`,
    /// or None if chain is empty.
    pub head_commit_oid_hex: Option<String>,
    /// L4.E chain hash at the end of `rejections.jsonl`, or `Hash::ZERO`
    /// hex if empty.
    pub l4e_last_hash_hex: String,
    /// One-line classification of the replay error if replay failed.
    pub replay_failure: Option<String>,
    /// True iff `<runtime_repo>/initial_q_state.json` was found and loaded.
    /// False when the verifier defaulted to `QState::genesis()`.
    pub initial_q_state_loaded_from_disk: bool,
}

impl ReplayReport {
    /// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — ship-gate aggregator over the 5
    /// architect-mandated boolean indicators. The CLI uses this to drive its
    /// exit code (0 when all pass, 1 otherwise).
    ///
    /// True iff every architect-mandated boolean indicator is `true`.
    /// **TB-7 Atom 4**: also checks the new `agent_signatures_verified` (Gate 4)
    /// and `proposal_telemetry_cas_retrievable` (Gate 5) indicators.
    pub fn all_indicators_pass(&self) -> bool {
        self.ledger_root_verified
            && self.system_signatures_verified
            && self.state_reconstructed
            && self.economic_state_reconstructed
            && self.cas_payloads_retrievable
            && self.agent_signatures_verified
            && self.proposal_telemetry_cas_retrievable
    }
}

// ── Verifier entry-points ───────────────────────────────────────────────────

/// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — verify_chaintape options.
#[derive(Debug, Clone, Default)]
pub struct VerifyOptions {
    /// Optional run-id filter; if provided, the verifier asserts the
    /// pinned-pubkey manifest's `run_id` matches before replay. None =
    /// no filter (smoke evidence may legitimately not echo a run-id).
    pub expected_run_id: Option<String>,
}

/// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — single library entry-point for replay
/// + signature + CAS + L4.E verification. The CLI binary at
/// `src/bin/verify_chaintape.rs` is a thin wrapper around this.
///
/// Steps (mirrors architect § 3.6 Atom 4):
/// 1. Read `pinned_pubkeys.json` from `runtime_repo_path`. Decode hex
///    pubkey(s) into a `PinnedSystemPubkeys` map keyed by `SystemEpoch`.
/// 2. Resolve initial `QState` from `<runtime_repo>/initial_q_state.json` if
///    present; else `QState::genesis()`.
/// 3. Open `Git2LedgerWriter` at `runtime_repo_path`. Read all entries.
/// 4. Open `CasStore` at `cas_path`.
/// 5. Open `RejectionEvidenceWriter::open_jsonl(<runtime_repo>/rejections.jsonl)`
///    — this internally calls `verify_chain()` and rejects tampering.
/// 6. Replay via `replay_full_transition` → either `Ok(QState)` or
///    `ReplayError`. Map the error variant onto the mandated booleans.
/// 7. Build `ReplayReport`. Return `Ok(report)` (errors only for I/O /
///    manifest issues that block replay from starting).
pub fn verify_chaintape(
    runtime_repo_path: &Path,
    cas_path: &Path,
    options: &VerifyOptions,
) -> Result<ReplayReport, VerifyError> {
    // Step 1: pinned-pubkey manifest.
    let manifest_path = runtime_repo_path.join(PINNED_PUBKEYS_FILENAME);
    if !manifest_path.exists() {
        return Err(VerifyError::PinnedPubkeysMissing(manifest_path));
    }
    let manifest_json = std::fs::read_to_string(&manifest_path)?;
    let manifest: PinnedPubkeyManifest = serde_json::from_str(&manifest_json)
        .map_err(|e| VerifyError::PinnedPubkeysParse(e.to_string()))?;
    if let Some(expected) = options.expected_run_id.as_ref() {
        if manifest.run_id != *expected {
            return Err(VerifyError::PinnedPubkeysParse(format!(
                "run_id mismatch: manifest={} expected={}",
                manifest.run_id, expected
            )));
        }
    }
    let mut pinned = PinnedSystemPubkeys::new();
    for entry in &manifest.pubkeys {
        let bytes = decode_pubkey_hex(&entry.pubkey_hex)?;
        let arr: [u8; 32] = bytes.as_slice().try_into().map_err(|_| {
            VerifyError::PubkeyDecode(format!(
                "expected 32-byte ed25519 pubkey, got {} bytes",
                bytes.len()
            ))
        })?;
        let pubkey = SystemPublicKey::from_bytes(arr);
        pinned.insert(SystemEpoch::new(entry.epoch), pubkey);
    }

    // Step 2: initial QState.
    let initial_q_path = runtime_repo_path.join(INITIAL_Q_STATE_FILENAME);
    let (initial_q, initial_q_loaded_from_disk) = if initial_q_path.exists() {
        let s = std::fs::read_to_string(&initial_q_path)?;
        let q: QState =
            serde_json::from_str(&s).map_err(|e| VerifyError::InitialQStateParse(e.to_string()))?;
        (q, true)
    } else {
        (QState::genesis(), false)
    };

    // Step 3: open ledger writer + read all entries.
    let writer = Git2LedgerWriter::open(runtime_repo_path)?;
    let l4_entries = writer.len();
    let head_commit_oid_hex = writer.head_commit_oid_hex();
    let entries: Vec<LedgerEntry> = (1..=l4_entries)
        .map(|t| writer.read_at(t))
        .collect::<Result<Vec<_>, _>>()?;

    // Step 4: open CAS.
    let cas_store = CasStore::open(cas_path).map_err(|e| VerifyError::Cas(e.to_string()))?;

    // Step 5: open + verify L4.E chain.
    let rejections_path = runtime_repo_path.join(REJECTIONS_JSONL_FILENAME);
    let l4e_writer = if rejections_path.exists() {
        RejectionEvidenceWriter::open_jsonl(rejections_path).map_err(VerifyError::L4eOpen)?
    } else {
        RejectionEvidenceWriter::new()
    };
    let l4e_entries = l4e_writer.len() as u64;
    let l4e_last_hash_hex = hash_to_hex(&l4e_writer.last_hash());

    // Step 6: replay.
    let predicate_registry = PredicateRegistry::new();
    let tool_registry = ToolRegistry::new();
    let replay_outcome = replay_full_transition(
        &initial_q,
        &entries,
        &cas_store,
        &pinned,
        &predicate_registry,
        &tool_registry,
    );

    let (
        ledger_root_verified,
        system_signatures_verified,
        state_reconstructed,
        economic_state_reconstructed,
        cas_payloads_retrievable,
        final_state_root_hex,
        final_ledger_root_hex,
        replay_failure,
    ) = match replay_outcome {
        Ok(final_q) => (
            true,
            true,
            true,
            true,
            true,
            Some(hash_to_hex(&final_q.state_root_t)),
            Some(hash_to_hex(&final_q.ledger_root_t)),
            None,
        ),
        Err(err) => classify_replay_error(&err),
    };

    // ── TB-7 Atom 4: agent signature verification (Gate 4) ──
    //
    // Walk every L4 entry; for WorkTx and VerifyTx variants, verify the
    // AgentSignature against the per-run agent_pubkeys.json manifest.
    // Empty chain or chain with no Work/Verify entries → trivially true
    // (no agent signatures to fail).
    let (agent_signatures_verified, proposal_telemetry_cas_retrievable) =
        verify_agent_artifacts(runtime_repo_path, &cas_store, &entries);

    Ok(ReplayReport {
        l4_entries,
        l4e_entries,
        ledger_root_verified,
        system_signatures_verified,
        state_reconstructed,
        economic_state_reconstructed,
        cas_payloads_retrievable,
        agent_signatures_verified,
        proposal_telemetry_cas_retrievable,
        run_id: manifest.run_id,
        epoch: manifest.epoch,
        detail: ReplayReportDetail {
            final_state_root_hex,
            final_ledger_root_hex,
            head_commit_oid_hex,
            l4e_last_hash_hex,
            replay_failure,
            initial_q_state_loaded_from_disk: initial_q_loaded_from_disk,
        },
    })
}

/// TRACE_MATRIX FC1-N14: TB-7 Atom 4 — verify Gate 4 + Gate 5 indicators by
/// walking every L4 entry and (for WorkTx / VerifyTx variants) re-verifying
/// agent signatures against the on-disk `agent_pubkeys.json` manifest, plus
/// checking that every `WorkTx.proposal_cid` resolves to a CAS-resident
/// ProposalTelemetry object.
///
/// Returns `(agent_signatures_verified, proposal_telemetry_cas_retrievable)`.
/// Both default to `true` when the manifest doesn't exist or when no
/// Work/Verify entries are present (no signatures to verify ≠ failure).
fn verify_agent_artifacts(
    runtime_repo_path: &Path,
    cas_store: &CasStore,
    entries: &[LedgerEntry],
) -> (bool, bool) {
    use crate::bottom_white::ledger::transition_ledger::canonical_decode;
    use crate::runtime::agent_keypairs::{verify_agent_signature, AgentPubkeyManifest};
    use crate::runtime::proposal_telemetry::read_from_cas as read_telemetry;
    use crate::state::typed_tx::TypedTx;

    let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
    if !manifest_path.exists() {
        // No agent_pubkeys.json (legacy / pre-Atom-1 chain). Both indicators
        // trivially true since there are no agent-side artifacts to fail.
        return (true, true);
    }
    let manifest = match AgentPubkeyManifest::load(&manifest_path) {
        Ok(m) => m,
        Err(_) => return (false, false), // manifest unparseable = both fail
    };

    let mut agent_signatures_verified = true;
    let mut proposal_telemetry_cas_retrievable = true;

    for entry in entries {
        // Get the typed payload from CAS.
        let payload_bytes = match cas_store.get(&entry.tx_payload_cid) {
            Ok(b) => b,
            Err(_) => continue, // cas_payloads_retrievable already covers this
        };
        let typed_tx: TypedTx = match canonical_decode(&payload_bytes) {
            Ok(tx) => tx,
            Err(_) => continue, // payload decode error already covered upstream
        };

        match &typed_tx {
            TypedTx::Work(work) => {
                // Gate 4 — verify WorkTx signature.
                let payload = work.to_signing_payload();
                let digest = payload.canonical_digest();
                let pubkey_opt = manifest.get(&work.agent_id);
                match pubkey_opt {
                    None => agent_signatures_verified = false,
                    Some(pubkey) => {
                        if verify_agent_signature(&work.signature, &digest, &pubkey).is_err() {
                            agent_signatures_verified = false;
                        }
                    }
                }
                // Gate 5 — verify proposal_cid resolves to a ProposalTelemetry.
                // Skip if proposal_cid is the zero-CID (legacy synthetic seed).
                if work.proposal_cid.0 != [0u8; 32] {
                    if read_telemetry(cas_store, &work.proposal_cid).is_err() {
                        proposal_telemetry_cas_retrievable = false;
                    }
                }
            }
            TypedTx::Verify(verify) => {
                // Gate 4 — verify VerifyTx signature.
                let payload = verify.to_signing_payload();
                let digest = payload.canonical_digest();
                let pubkey_opt = manifest.get(&verify.verifier_agent);
                match pubkey_opt {
                    None => agent_signatures_verified = false,
                    Some(pubkey) => {
                        if verify_agent_signature(&verify.signature, &digest, &pubkey).is_err() {
                            agent_signatures_verified = false;
                        }
                    }
                }
            }
            // TRACE_MATRIX TB-13 Atom 6 round-2 (Codex VETO TB13-V2
            // remediation 2026-05-03): extend Gate 4 to cover the 3
            // agent-signed TB-13 variants. The submit-time verification
            // gap is codebase-wide (also affects Challenge/TaskOpen/
            // EscrowLock); replay-time coverage is the existing TB-7
            // ARCHITECT_RULING D3 model. TB-13 raises the bar to its
            // own three variants because Class 3 money-mover.
            TypedTx::CompleteSetMint(mint) => {
                let payload = mint.to_signing_payload();
                let digest = payload.canonical_digest();
                let pubkey_opt = manifest.get(&mint.owner);
                match pubkey_opt {
                    None => agent_signatures_verified = false,
                    Some(pubkey) => {
                        if verify_agent_signature(&mint.signature, &digest, &pubkey).is_err() {
                            agent_signatures_verified = false;
                        }
                    }
                }
            }
            TypedTx::CompleteSetRedeem(redeem) => {
                let payload = redeem.to_signing_payload();
                let digest = payload.canonical_digest();
                let pubkey_opt = manifest.get(&redeem.owner);
                match pubkey_opt {
                    None => agent_signatures_verified = false,
                    Some(pubkey) => {
                        if verify_agent_signature(&redeem.signature, &digest, &pubkey).is_err() {
                            agent_signatures_verified = false;
                        }
                    }
                }
            }
            TypedTx::MarketSeed(seed) => {
                let payload = seed.to_signing_payload();
                let digest = payload.canonical_digest();
                let pubkey_opt = manifest.get(&seed.provider);
                match pubkey_opt {
                    None => agent_signatures_verified = false,
                    Some(pubkey) => {
                        if verify_agent_signature(&seed.signature, &digest, &pubkey).is_err() {
                            agent_signatures_verified = false;
                        }
                    }
                }
            }
            // Stage C P-M2 / Phase F.1 (architect §7.3): replay-time Gate 4
            // coverage parallel to CompleteSetMint / CompleteSetRedeem /
            // MarketSeed. Owner is the signer; pubkey lookup mirrors
            // CompleteSetMint replay arm.
            TypedTx::CompleteSetMerge(merge) => {
                let payload = merge.to_signing_payload();
                let digest = payload.canonical_digest();
                let pubkey_opt = manifest.get(&merge.owner);
                match pubkey_opt {
                    None => agent_signatures_verified = false,
                    Some(pubkey) => {
                        if verify_agent_signature(&merge.signature, &digest, &pubkey).is_err() {
                            agent_signatures_verified = false;
                        }
                    }
                }
            }
            // Stage C P-M4 / Phase F.3 (architect §7.5): replay-time Gate 4
            // coverage. Provider is the signer; pubkey lookup mirrors
            // MarketSeed replay arm (which also keys on `provider`).
            TypedTx::CpmmPool(pool) => {
                let payload = pool.to_signing_payload();
                let digest = payload.canonical_digest();
                let pubkey_opt = manifest.get(&pool.provider);
                match pubkey_opt {
                    None => agent_signatures_verified = false,
                    Some(pubkey) => {
                        if verify_agent_signature(&pool.signature, &digest, &pubkey).is_err() {
                            agent_signatures_verified = false;
                        }
                    }
                }
            }
            // Stage C P-M5 / Phase F.4 (architect §7.6): replay-time Gate 4
            // coverage. Trader is the signer; pubkey lookup mirrors
            // CompleteSetMint replay arm (keyed on `owner`).
            TypedTx::CpmmSwap(swap) => {
                let payload = swap.to_signing_payload();
                let digest = payload.canonical_digest();
                let pubkey_opt = manifest.get(&swap.trader);
                match pubkey_opt {
                    None => agent_signatures_verified = false,
                    Some(pubkey) => {
                        if verify_agent_signature(&swap.signature, &digest, &pubkey).is_err() {
                            agent_signatures_verified = false;
                        }
                    }
                }
            }
            // Stage C P-M6 / Phase F.5 (architect §7.7): replay-time Gate 4
            // coverage. Buyer is the signer; pubkey lookup mirrors CpmmSwap
            // replay arm (keyed on `trader`) — same agent-signed pattern.
            TypedTx::BuyWithCoinRouter(router) => {
                let payload = router.to_signing_payload();
                let digest = payload.canonical_digest();
                let pubkey_opt = manifest.get(&router.buyer);
                match pubkey_opt {
                    None => agent_signatures_verified = false,
                    Some(pubkey) => {
                        if verify_agent_signature(&router.signature, &digest, &pubkey).is_err() {
                            agent_signatures_verified = false;
                        }
                    }
                }
            }
            // Remaining tx variants (TaskOpen / EscrowLock / Challenge /
            // ChallengeResolve / ReuseTx / FinalizeReward / TaskExpire /
            // TerminalSummary / TaskBankruptcy) are not covered by Gate 4
            // because:
            // - Some are system-emitted (signature path is system, not agent;
            //   covered by system_signatures_verified above).
            // - Others are agent-emitted but their signing payloads need
            //   per-variant signing helpers and are deferred to a future
            //   codebase-wide CO P2.x AgentRegistry pass per `OBS_AGENT_SIG_REPLAY_GAP_2026-05-03`.
            _ => {}
        }
    }

    (
        agent_signatures_verified,
        proposal_telemetry_cas_retrievable,
    )
}

// ── Helpers ─────────────────────────────────────────────────────────────────

fn decode_pubkey_hex(hex: &str) -> Result<Vec<u8>, VerifyError> {
    if hex.len() % 2 != 0 {
        return Err(VerifyError::PubkeyDecode(format!(
            "odd-length hex string ({})",
            hex.len()
        )));
    }
    let mut out = Vec::with_capacity(hex.len() / 2);
    for chunk in hex.as_bytes().chunks(2) {
        let s = std::str::from_utf8(chunk).map_err(|e| VerifyError::PubkeyDecode(e.to_string()))?;
        let byte =
            u8::from_str_radix(s, 16).map_err(|e| VerifyError::PubkeyDecode(e.to_string()))?;
        out.push(byte);
    }
    Ok(out)
}

fn hash_to_hex(h: &Hash) -> String {
    h.0.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Map a `ReplayError` onto the 5 mandated boolean indicators + final-state
/// fields + a one-line failure classifier. The mapping is conservative:
/// an error sets the indicator that the failing stage is responsible for to
/// `false` and leaves the other indicators **also `false`** (replay did not
/// complete, so we cannot honestly claim downstream stages passed).
type ReplayClassification = (
    bool,
    bool,
    bool,
    bool,
    bool,
    Option<String>,
    Option<String>,
    Option<String>,
);
fn classify_replay_error(err: &ReplayError) -> ReplayClassification {
    use ReplayError::*;
    let summary = format!("{err}");
    let (ledger_ok, sig_ok, state_ok, econ_ok, cas_ok) = match err {
        // Stages 1-3 are the chain-integrity stages.
        LogicalTGap { .. }
        | ParentStateMismatch { .. }
        | ParentLedgerMismatch { .. }
        | LedgerRootMismatch { .. } => (false, false, false, false, false),
        // Stage 4 — signature verification.
        BadSignature { .. } => (true, false, false, false, false),
        // Stage 5 — CAS lookup.
        CasMissing { .. } => (true, true, false, false, false),
        // Stages 6 / 6.5 / 7 / 8 — payload decode / kind / dispatch / state.
        PayloadDecode { .. }
        | TxKindMismatch { .. }
        | Transition { .. }
        | StateRootMismatch { .. } => (true, true, false, false, true),
    };
    (
        ledger_ok,
        sig_ok,
        state_ok,
        econ_ok,
        cas_ok,
        None,
        None,
        Some(summary),
    )
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_to_hex_round_trip_zero() {
        assert_eq!(hash_to_hex(&Hash::ZERO), "0".repeat(64));
    }

    #[test]
    fn decode_pubkey_hex_rejects_odd_length() {
        assert!(matches!(
            decode_pubkey_hex("abc"),
            Err(VerifyError::PubkeyDecode(_))
        ));
    }

    #[test]
    fn decode_pubkey_hex_round_trips_lowercase_hex() {
        let bytes = vec![0xde, 0xad, 0xbe, 0xef];
        let hex: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
        let decoded = decode_pubkey_hex(&hex).expect("decode");
        assert_eq!(decoded, bytes);
    }

    #[test]
    fn replay_report_all_indicators_pass_requires_all_seven_booleans() {
        // **TB-7 Atom 4 (2026-05-01)**: indicator count 5 → 7 with the
        // addition of `agent_signatures_verified` (Gate 4) and
        // `proposal_telemetry_cas_retrievable` (Gate 5).
        let mut r = ReplayReport {
            l4_entries: 0,
            l4e_entries: 0,
            ledger_root_verified: true,
            system_signatures_verified: true,
            state_reconstructed: true,
            economic_state_reconstructed: true,
            cas_payloads_retrievable: true,
            agent_signatures_verified: true,
            proposal_telemetry_cas_retrievable: true,
            run_id: "test".into(),
            epoch: 1,
            detail: ReplayReportDetail {
                final_state_root_hex: None,
                final_ledger_root_hex: None,
                head_commit_oid_hex: None,
                l4e_last_hash_hex: hash_to_hex(&Hash::ZERO),
                replay_failure: None,
                initial_q_state_loaded_from_disk: false,
            },
        };
        assert!(r.all_indicators_pass());
        r.system_signatures_verified = false;
        assert!(!r.all_indicators_pass());
        r.system_signatures_verified = true;
        // Atom 4 NEW indicators must also be checked.
        r.agent_signatures_verified = false;
        assert!(!r.all_indicators_pass());
        r.agent_signatures_verified = true;
        r.proposal_telemetry_cas_retrievable = false;
        assert!(!r.all_indicators_pass());
    }
}
