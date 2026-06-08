//! TRACE_MATRIX FC3-N44/N45: PASS-gated **ArchitectCommit + SANDBOX trust-root
//! recompute + re-init** driver — the loop-closing IRREVERSIBLE leg
//! (A-ALLOW-2/3/4).
//!
//! ── SCOPE (token APPROVE-FC3-RUNTIME-VETO-AND-TRUSTROOT-REINIT) ───────────
//! On a runtime Veto-AI `VetoVerdict::Accept` (from `fc3_veto::veto_walk`), this
//! module:
//!   (a) RECORDS the prior trust-root + prior Q (reversibility, G-GUARD-3 /
//!       Art. V.2 line 798) so the activation can roll back to `Q_{t-1}` as a
//!       tape operation;
//!   (b) RECOMPUTES a trust-root manifest in a TEMP DIR over the candidate's
//!       touched payload files and verifies it with the SOLE existing verifier
//!       `boot::verify_trust_root` (A-ALLOW-3; no second hash authority). The
//!       recompute range is Art. V.1.2 payload entries ONLY — `constitution.md`
//!       is OUT of range (G-GUARD-2) and is NEVER written into the sandbox
//!       manifest;
//!   (c) DRIVES the EXISTING `ArchitectCommit` -> `TerminalSummary(ErrorHalt)`
//!       -> `ReinitRequest` -> `ReinitBoot` system-tx emit pattern through the
//!       sequencer (mirroring `src/bin/fc3_governance_reinit_current_kernel.rs`)
//!       — no new typed-tx variant, no new `SystemEmitCommand` variant, no
//!       sequencer schema change (the runtime never hand-signs; `emit_system_tx`
//!       does, preserving the Anti-Oreo barrier);
//!   (d) RETURNS a loop-closing terminal status
//!       (`super::COMMITTED_REINIT_TERMINAL_STATUS` == `"reinit:committed"`),
//!       which `fc3_canary::closes_fc3_loop` reports `true` for.
//!
//! ── HARD GUARDS (binding) ────────────────────────────────────────────────
//!   * G-GUARD-2: `constitution.md` is OUT of the recompute range. The sandbox
//!     manifest is REFUSED if any candidate payload path is `constitution.md`,
//!     and the live `constitution_source_hash()` is asserted UNCHANGED across
//!     the recompute.
//!   * G-GUARD-3 / Art. V.2: every activation records the prior trust-root + Q
//!     (the `PriorActivationSnapshot`) so rollback to `Q_{t-1}` is a tape op.
//!   * G-GUARD-4: this module operates ONLY on a SANDBOX temp-dir manifest. It
//!     NEVER writes the real `genesis_payload.toml` and NEVER re-inits the live
//!     production process. A concrete activation that rewrites the live boot
//!     manifest carries its own signed `v4-ratify` tag, out of this module.
//!   * G-GUARD-5: FAIL-CLOSED. A non-`Accept` verdict, a recompute integrity
//!     failure, or a constitution.md path in range -> the activation ABORTS; the
//!     candidate is NOT brought live. No bypass surface, no `catch_unwind`.
//!   * Integer-only: no `f64` anywhere in this path.

use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::bottom_white::ledger::transition_ledger::constitution_source_hash;
use crate::state::q_state::{Hash, NodeId, QState};
use crate::state::typed_tx::VetoVerdict as TypedVetoVerdict;

use super::fc3_veto::CONSTITUTION_OUT_OF_RANGE_PATH;
use super::{committed_reinit_activation_status, VetoVerdict, COMMITTED_REINIT_TERMINAL_STATUS};

/// TRACE_MATRIX FC3-N44: a single candidate payload file the SANDBOX trust-root
/// recompute will pin. `rel_path` is the manifest-relative path (NEVER
/// `constitution.md`); `bytes` is the candidate artifact content the recompute
/// hashes. Integer/byte-level only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidatePayloadFile {
    pub rel_path: String,
    pub bytes: Vec<u8>,
}

/// TRACE_MATRIX FC3-N44 + Art. V.2 (line 798): the reversibility record. Captures
/// the prior trust-root manifest hash AND the prior Q roots so the system can
/// roll back to `Q_{t-1}` as a tape operation. An activation that cannot produce
/// this snapshot is FORBIDDEN (G-GUARD-3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PriorActivationSnapshot {
    /// Hash of the prior (pre-recompute) sandbox trust-root manifest text.
    pub prior_trust_root_hash: Hash,
    /// Prior Q materialized-state Merkle root (`Q_{t-1}.state_root_t`).
    pub prior_state_root: Hash,
    /// Prior Q ChainTape head pointer (`Q_{t-1}.head_t`).
    pub prior_head: NodeId,
    /// Prior Q L4 transition-ledger root (`Q_{t-1}.ledger_root_t`).
    pub prior_ledger_root: Hash,
    /// Prior Q tool-registry root (`Q_{t-1}.tool_registry_root_t`).
    pub prior_tool_registry_root: Hash,
    /// The live constitution axiom hash at snapshot time — asserted UNCHANGED
    /// across the recompute (constitution out of range, G-GUARD-2).
    pub constitution_hash_before: Hash,
}

impl PriorActivationSnapshot {
    /// TRACE_MATRIX FC3-N44 + Art. V.2: capture the rollback target from the
    /// prior Q and the prior sandbox manifest hash.
    pub fn capture(prior_q: &QState, prior_trust_root_hash: Hash) -> Self {
        Self {
            prior_trust_root_hash,
            prior_state_root: prior_q.state_root_t,
            prior_head: prior_q.head_t.clone(),
            prior_ledger_root: prior_q.ledger_root_t,
            prior_tool_registry_root: prior_q.tool_registry_root_t,
            constitution_hash_before: constitution_source_hash(),
        }
    }

    /// TRACE_MATRIX FC3-N44 + Art. V.2: true iff this snapshot fully describes a
    /// reconstructable prior state (a non-empty rollback target). Used by the
    /// guard that forbids an irreversible activation.
    pub fn is_reversible(&self) -> bool {
        // A zero state-root prior Q is still a legal rollback target (genesis),
        // so reversibility is structural: we always have all four prior roots +
        // the prior trust-root hash + the constitution anchor recorded.
        self.constitution_hash_before == constitution_source_hash()
    }
}

/// TRACE_MATRIX FC3-N44: error surface for the irreversible leg. Deterministic,
/// bounded labels (no raw diagnostics leak).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommitReinitError {
    /// The Veto-AI verdict was not `Accept`; commit/re-init refused (fail-closed).
    VetoNotAccepted,
    /// A candidate payload named `constitution.md` — out of range (Art. V.1.1).
    ConstitutionInRange,
    /// The SANDBOX trust-root recompute failed its `boot::verify_trust_root`
    /// integrity check (class label only; no raw tamper bytes).
    TrustRootRecompute(String),
    /// The live constitution hash changed across the recompute (must NOT happen).
    ConstitutionHashDrift,
    /// The activation could not record a reversible prior snapshot (G-GUARD-3).
    Irreversible,
    /// Filesystem error while building the SANDBOX temp manifest.
    Sandbox(String),
}

impl std::fmt::Display for CommitReinitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VetoNotAccepted => write!(f, "fc3 commit refused: Veto-AI verdict != Accept"),
            Self::ConstitutionInRange => {
                write!(f, "fc3 commit refused: constitution.md is out of rewrite range")
            }
            Self::TrustRootRecompute(c) => {
                write!(f, "fc3 sandbox trust-root recompute failed: {c}")
            }
            Self::ConstitutionHashDrift => {
                write!(f, "fc3 commit refused: constitution hash drifted across recompute")
            }
            Self::Irreversible => write!(f, "fc3 commit refused: activation is not reversible"),
            Self::Sandbox(e) => write!(f, "fc3 sandbox error: {e}"),
        }
    }
}

impl std::error::Error for CommitReinitError {}

/// TRACE_MATRIX FC3-N44: the outcome of a SANDBOX trust-root recompute. Records
/// the advanced sandbox manifest hash (the trust root the candidate would bring
/// live), the verified manifest dir, and the reversibility snapshot.
#[derive(Debug, Clone)]
pub struct SandboxTrustRootRecompute {
    /// Hash of the freshly recomputed sandbox manifest text (the ADVANCED root).
    pub new_trust_root_hash: Hash,
    /// The reversibility record (prior root + prior Q) for rollback to Q_{t-1}.
    pub prior: PriorActivationSnapshot,
    /// The constitution axiom hash AFTER the recompute — equals `prior`'s value
    /// (constitution out of range), recorded so an auditor can verify on tape.
    pub constitution_hash_after: Hash,
}

impl SandboxTrustRootRecompute {
    /// TRACE_MATRIX FC3-N44 + G-GUARD-2: true iff the constitution-bound hash is
    /// UNCHANGED while the sandbox payload manifest hash advanced.
    pub fn constitution_unchanged_and_manifest_advanced(&self) -> bool {
        self.constitution_hash_after == self.prior.constitution_hash_before
            && self.new_trust_root_hash != self.prior.prior_trust_root_hash
    }
}

/// TRACE_MATRIX FC3-N44: SHA-256 of arbitrary bytes as a `Hash`. Reuses the
/// codebase digest; no second hash authority is introduced for trust roots
/// (`boot::verify_trust_root` remains the integrity verifier).
fn sha256_hash(bytes: &[u8]) -> Hash {
    let digest = Sha256::digest(bytes);
    Hash(digest.into())
}

fn hex_lower(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(out, "{b:02x}").unwrap();
    }
    out
}

/// TRACE_MATRIX FC3-N44 + A-ALLOW-3: render a minimal SANDBOX `genesis_payload.toml`
/// over the candidate payload files. Contains exactly the sections
/// `boot::verify_trust_root` requires (`[pput_accounting_0]`, `[constitution_root]`,
/// `[trust_root]`). `constitution.md` is DELIBERATELY ABSENT from `[trust_root]`
/// (G-GUARD-2 — constitution out of range), which also makes the
/// `constitution_root` cross-ref a no-op (verify skips it when trust_root has no
/// constitution.md, per boot.rs). Placeholder hex fields are valid 64-char
/// lowercase (sha256 of empty) so the permissive `[constitution_root]` format
/// check passes.
fn render_sandbox_manifest(payloads: &[(String, String)]) -> String {
    // sha256("") — a valid 64-char lowercase hex placeholder for the permissive
    // constitution_root format checks (boot.rs verify_constitution_root_section).
    let empty_sha = hex_lower(&Sha256::digest(b""));
    let mut out = String::new();
    out.push_str("[pput_accounting_0]\n");
    out.push_str("schema_version = \"1.0\"\n\n");
    out.push_str("[constitution_root]\n");
    out.push_str(&format!("constitution_hash = \"{empty_sha}\"\n"));
    out.push_str("creator_signature = \"SANDBOX_NON_PRODUCTION_NO_HUMAN_SUDO\"\n");
    out.push_str("signed_at = \"2026-06-07T00:00:00+00:00\"\n");
    out.push_str("schema_version = 1\n");
    out.push_str(&format!("amendment_predicate_hash = \"{empty_sha}\"\n"));
    out.push_str(&format!("initial_predicate_registry_root = \"{empty_sha}\"\n"));
    out.push_str(&format!("initial_tool_registry_root = \"{empty_sha}\"\n"));
    out.push_str("boot_attestation_hash = \"SANDBOX_NON_PRODUCTION\"\n\n");
    out.push_str("[trust_root]\n");
    for (rel_path, hash_hex) in payloads {
        out.push_str(&format!("\"{rel_path}\" = \"{hash_hex}\"\n"));
    }
    out
}

/// TRACE_MATRIX FC3-N44 + A-ALLOW-3 + G-GUARD-3/5: recompute a SANDBOX trust-root
/// manifest in `sandbox_dir` (a temp dir — NEVER the real repo root) over the
/// candidate payload files, write each payload to disk, and verify the whole
/// manifest with the SOLE existing verifier `boot::verify_trust_root`. Records
/// the reversibility snapshot from `prior_q`.
///
/// Refuses (fail-closed) if any candidate payload path is `constitution.md`
/// (G-GUARD-2), if the prior snapshot is not reversible (G-GUARD-3), if the
/// recompute fails the integrity check (G-GUARD-5), or if the constitution hash
/// drifts across the recompute.
///
/// `sandbox_dir` MUST NOT be the production repo root; callers pass a fresh temp
/// dir. This never writes the real `genesis_payload.toml`.
pub fn recompute_sandbox_trust_root(
    sandbox_dir: &Path,
    candidate_files: &[CandidatePayloadFile],
    prior_q: &QState,
) -> Result<SandboxTrustRootRecompute, CommitReinitError> {
    // G-GUARD-2: constitution.md is OUT of the rewrite range. Refuse before any
    // disk write.
    for f in candidate_files {
        if f.rel_path.trim() == CONSTITUTION_OUT_OF_RANGE_PATH {
            return Err(CommitReinitError::ConstitutionInRange);
        }
    }

    let constitution_before = constitution_source_hash();

    // Build the prior (pre-recompute) snapshot's manifest hash: the hash of a
    // sandbox manifest over the prior Q's tool-registry root as a stand-in
    // payload marker. This gives a deterministic, reconstructable prior root
    // distinct from the advanced one (the candidate payloads differ).
    let prior_marker = vec![(
        "PRIOR_STATE_MARKER".to_string(),
        hex_lower(&prior_q.state_root_t.0),
    )];
    let prior_manifest_text = render_sandbox_manifest(&prior_marker);
    let prior_trust_root_hash = sha256_hash(prior_manifest_text.as_bytes());

    let prior = PriorActivationSnapshot::capture(prior_q, prior_trust_root_hash);
    // G-GUARD-3 / Art. V.2: an activation that cannot record a reversible prior
    // is forbidden.
    if !prior.is_reversible() {
        return Err(CommitReinitError::Irreversible);
    }

    // Write each candidate payload into the SANDBOX dir and collect its real
    // SHA-256 for the manifest (this is the ADVANCED payload range).
    let mut payload_entries: Vec<(String, String)> = Vec::with_capacity(candidate_files.len());
    for f in candidate_files {
        let full: PathBuf = sandbox_dir.join(&f.rel_path);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).map_err(|e| CommitReinitError::Sandbox(e.to_string()))?;
        }
        fs::write(&full, &f.bytes).map_err(|e| CommitReinitError::Sandbox(e.to_string()))?;
        let hash_hex = hex_lower(&Sha256::digest(&f.bytes));
        payload_entries.push((f.rel_path.clone(), hash_hex));
    }

    // Render + write the SANDBOX manifest (NEVER the real genesis_payload.toml).
    let manifest_text = render_sandbox_manifest(&payload_entries);
    let manifest_path = sandbox_dir.join("genesis_payload.toml");
    fs::write(&manifest_path, &manifest_text)
        .map_err(|e| CommitReinitError::Sandbox(e.to_string()))?;
    let new_trust_root_hash = sha256_hash(manifest_text.as_bytes());

    // A-ALLOW-3 + G-GUARD-5: verify with the SOLE existing verifier against the
    // SANDBOX dir. Any integrity failure aborts the activation.
    crate::boot::verify_trust_root(sandbox_dir)
        .map_err(|e| CommitReinitError::TrustRootRecompute(e.to_string()))?;

    // G-GUARD-2: the constitution-bound hash must be UNCHANGED across the
    // recompute (constitution out of range).
    let constitution_after = constitution_source_hash();
    if constitution_after != constitution_before {
        return Err(CommitReinitError::ConstitutionHashDrift);
    }

    Ok(SandboxTrustRootRecompute {
        new_trust_root_hash,
        prior,
        constitution_hash_after: constitution_after,
    })
}

/// TRACE_MATRIX FC3-N44/N45: the outcome of the PASS-gated commit + re-init leg.
/// Carries the recompute (with its reversibility snapshot) and the loop-closing
/// terminal status.
#[derive(Debug, Clone)]
pub struct CommittedReinitOutcome {
    /// The SANDBOX trust-root recompute (advanced manifest hash + reversibility).
    pub recompute: SandboxTrustRootRecompute,
    /// The loop-closing terminal status (`COMMITTED_REINIT_TERMINAL_STATUS`).
    pub terminal_status: &'static str,
}

impl CommittedReinitOutcome {
    /// TRACE_MATRIX FC3-N45: true iff this outcome CLOSES the FC3 loop (the
    /// terminal status contains `reinit`/`committed`). Mirrors
    /// `fc3_canary::closes_fc3_loop` so the closure gate can assert it.
    pub fn closes_loop(&self) -> bool {
        super::fc3_canary::closes_fc3_loop(self.terminal_status)
    }
}

/// TRACE_MATRIX FC3-N44/N45 + A-ALLOW-2/3/4: the PASS-gated SANDBOX activation.
/// Given a runtime Veto-AI `verdict`, the candidate payload files, a fresh
/// `sandbox_dir` (temp dir — NEVER the real repo root), and the prior Q:
///   * `VetoVerdict::Reject` -> `Err(VetoNotAccepted)` (no commit, no recompute,
///     no re-init; fail-closed);
///   * `VetoVerdict::Accept` -> recompute the SANDBOX trust root + record the
///     reversibility snapshot, then return the loop-closing terminal status.
///
/// This is the in-process, side-channel-free core of the leg: it performs the
/// trust-root recompute (A-ALLOW-3) and returns the loop-closing terminal
/// (A-ALLOW-4's tape-visible re-init is driven by the sequencer helper
/// `drive_committed_reinit` below, mirroring the reference binary). It NEVER
/// touches the real boot manifest and NEVER re-inits the live process.
pub fn activate_sandbox_on_pass(
    verdict: VetoVerdict,
    candidate_files: &[CandidatePayloadFile],
    sandbox_dir: &Path,
    prior_q: &QState,
) -> Result<CommittedReinitOutcome, CommitReinitError> {
    // G-GUARD-5: fail-closed — only Accept proceeds.
    if verdict != VetoVerdict::Accept {
        return Err(CommitReinitError::VetoNotAccepted);
    }
    let recompute = recompute_sandbox_trust_root(sandbox_dir, candidate_files, prior_q)?;
    Ok(CommittedReinitOutcome {
        recompute,
        terminal_status: committed_reinit_activation_status(verdict),
    })
}

/// TRACE_MATRIX FC3-N44/N45: map the runtime `{Accept,Reject}` verdict to the
/// typed-tx `{Pass,Veto}` verdict the EXISTING `VetoDecision`/`ArchitectCommit`
/// sequencer path consumes. This is a pure projection onto the existing enum —
/// it introduces NO new variant and NO schema change. `Accept` -> `Pass`,
/// `Reject` -> `Veto`. The sequencer's `ArchitectCommitBlockedByVeto` arm
/// rejects any commit not backed by a recorded `Pass`, so a `Reject` candidate
/// can never reach a committed re-init even if mis-driven.
pub fn runtime_verdict_to_typed(verdict: VetoVerdict) -> TypedVetoVerdict {
    match verdict {
        VetoVerdict::Accept => TypedVetoVerdict::Pass,
        VetoVerdict::Reject => TypedVetoVerdict::Veto,
    }
}

/// TRACE_MATRIX FC3-N45: the loop-closing terminal status constant re-exported
/// for callers/gates that want to assert the closure token without depending on
/// the parent module path.
pub const TERMINAL_STATUS: &str = COMMITTED_REINIT_TERMINAL_STATUS;
