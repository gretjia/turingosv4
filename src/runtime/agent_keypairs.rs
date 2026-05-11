//! TB-7 Atom 1 — Per-agent Ed25519 keypair manager + on-disk pubkey manifest.
//!
//! **Run-local identity ONLY**, NOT durable reputation identity (per
//! ARCHITECT_RULING 2026-05-01 D2 caveat + TB-7 charter §4.2). Cross-run
//! reputation, NodeMarket identity, or long-term agent economic identity
//! REQUIRE a separate TB (`Persistent AgentRegistry + agent keystore`,
//! charter §13 TB-10.5) — explicitly NOT in scope here.
//!
//! Mirrors the structurally proven `PinnedSystemPubkeys` pattern from
//! `bottom_white::ledger::system_keypair`, but agent-side. Differences:
//!
//! | Concern             | System (TB-5)                | Agent (this module)               |
//! |---------------------|------------------------------|-----------------------------------|
//! | Identity domain     | epoch (rotation history)     | `AgentId` (per-agent string)      |
//! | Lifecycle           | persisted with KDF + nonce   | run-local (process memory only)   |
//! | Key store on disk   | encrypted keystore file      | none (private keys drop on exit)  |
//! | Public manifest     | `pinned_pubkeys.json`        | `agent_pubkeys.json`              |
//! | Signature type      | `SystemSignature`            | `AgentSignature` (typed_tx.rs)    |
//! | Verifier            | `verify_system_signature`    | `verify_agent_signature` (here)   |
//!
//! Atom 1 is purely additive: no existing code calls into this module yet.
//! Atom 2 / Atom 3 (evaluator authoritative routing) wire the per-tx signing.
//! Atom 4 (verify_chaintape extension) wires per-tx signature verification on
//! replay.
//!
//! TRACE_MATRIX FC1-N14 (wtool / authoritative state-mutation path; agent
//! signature primitive for real-LLM proposal routing per TB-7 §4.0 / Gate 4).

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::state::q_state::AgentId;
use crate::state::typed_tx::AgentSignature;

const AGENT_SECRET_LEN: usize = 32;
const AGENT_PUBLIC_LEN: usize = 32;

// ── Public-key newtype ──────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N14: per-agent public key, type-distinct from
/// `SystemPublicKey` to prevent agent-vs-system confusion at API boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct AgentPublicKey([u8; AGENT_PUBLIC_LEN]);

impl AgentPublicKey {
    /// TRACE_MATRIX FC1-N14: construct an agent public key from raw bytes.
    pub const fn from_bytes(bytes: [u8; AGENT_PUBLIC_LEN]) -> Self {
        Self(bytes)
    }

    /// TRACE_MATRIX FC1-N14: expose raw bytes for canonical encoding / verify.
    pub const fn as_bytes(&self) -> &[u8; AGENT_PUBLIC_LEN] {
        &self.0
    }

    /// TRACE_MATRIX FC1-N14: hex encoding for the on-disk manifest.
    pub fn to_hex(&self) -> String {
        let mut out = String::with_capacity(AGENT_PUBLIC_LEN * 2);
        for byte in &self.0 {
            out.push_str(&format!("{:02x}", byte));
        }
        out
    }

    /// TRACE_MATRIX FC1-N14: hex decoding from manifest payload.
    pub fn from_hex(hex: &str) -> Result<Self, AgentKeypairError> {
        if hex.len() != AGENT_PUBLIC_LEN * 2 {
            return Err(AgentKeypairError::InvalidFormat(
                "agent pubkey hex must be 64 chars",
            ));
        }
        let mut out = [0u8; AGENT_PUBLIC_LEN];
        for (i, chunk) in hex.as_bytes().chunks_exact(2).enumerate() {
            let s = std::str::from_utf8(chunk)
                .map_err(|_| AgentKeypairError::InvalidFormat("non-utf8 hex"))?;
            out[i] = u8::from_str_radix(s, 16)
                .map_err(|_| AgentKeypairError::InvalidFormat("non-hex digit"))?;
        }
        Ok(Self(out))
    }
}

// ── Per-agent in-memory keypair (run-local; zeroized on drop) ───────────────

/// TRACE_MATRIX FC1-N14: per-agent Ed25519 keypair held in process memory only.
/// Private key zeroed on drop. Run-local; never persisted.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct AgentKeypair {
    secret_key: Box<[u8]>,
    #[zeroize(skip)]
    public_key: AgentPublicKey,
}

impl AgentKeypair {
    /// TRACE_MATRIX FC1-N14: generate a fresh keypair from `getrandom(2)` entropy.
    pub fn generate() -> Result<Self, AgentKeypairError> {
        let mut seed = [0u8; AGENT_SECRET_LEN];
        getrandom::getrandom(&mut seed).map_err(AgentKeypairError::Entropy)?;
        let signing_key = SigningKey::from_bytes(&seed);
        let public_key = AgentPublicKey::from_bytes(signing_key.verifying_key().to_bytes());
        let keypair = Self {
            secret_key: Vec::from(seed).into_boxed_slice(),
            public_key,
        };
        seed.zeroize();
        Ok(keypair)
    }

    /// TRACE_MATRIX FC1-N14 (TB-9 Atom 1): reconstruct a keypair from a saved
    /// 32-byte secret seed (durable keystore load path). Public key is
    /// recomputed from the seed; the keystore stores secrets only.
    pub fn from_secret_bytes(seed: [u8; AGENT_SECRET_LEN]) -> Self {
        let signing_key = SigningKey::from_bytes(&seed);
        let public_key = AgentPublicKey::from_bytes(signing_key.verifying_key().to_bytes());
        let keypair = Self {
            secret_key: Vec::from(seed).into_boxed_slice(),
            public_key,
        };
        let mut s = seed;
        s.zeroize();
        keypair
    }

    /// TRACE_MATRIX FC1-N14 (TB-9 Atom 1): expose the 32-byte secret seed for
    /// durable keystore persistence. Crate-private to keep secret material from
    /// leaking outside the runtime layer.
    pub(crate) fn secret_bytes(&self) -> [u8; AGENT_SECRET_LEN] {
        let mut out = [0u8; AGENT_SECRET_LEN];
        out.copy_from_slice(&self.secret_key);
        out
    }

    /// TRACE_MATRIX FC1-N14: return the public half of the keypair.
    pub const fn public_key(&self) -> AgentPublicKey {
        self.public_key
    }

    /// TRACE_MATRIX FC1-N14: sign a 32-byte canonical digest (e.g.
    /// `WorkSigningPayload::canonical_digest()`). Returns the typed
    /// `AgentSignature` so call sites cannot accidentally place agent
    /// signatures in system fields.
    pub fn sign_digest(&self, digest: [u8; 32]) -> Result<AgentSignature, AgentKeypairError> {
        if self.secret_key.len() != AGENT_SECRET_LEN {
            return Err(AgentKeypairError::InvalidFormat("bad secret length"));
        }
        let mut secret = [0u8; AGENT_SECRET_LEN];
        secret.copy_from_slice(&self.secret_key);
        let signing_key = SigningKey::from_bytes(&secret);
        let signature = signing_key.sign(&digest);
        secret.zeroize();
        Ok(AgentSignature::from_bytes(signature.to_bytes()))
    }
}

impl fmt::Debug for AgentKeypair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AgentKeypair")
            .field("public_key", &self.public_key)
            .field("secret_key", &"<redacted>")
            .finish()
    }
}

// ── Registry: agent_id → keypair (private) and manifest (public) ─────────────

/// TRACE_MATRIX FC1-N14: per-run agent keypair registry. Holds private keypairs
/// for in-process signing AND the on-disk public manifest path. The manifest is
/// what `verify_chaintape` (Atom 4) reads to verify replayed agent signatures.
///
/// **TB-9 extension (2026-05-02)**: optional `durable` slot. When populated,
/// every keypair generation also persists to the encrypted durable keystore
/// (typically `~/.turingos/keystore/agent_keystore.enc`) so that a subsequent
/// evaluator boot via `generate_or_load_durable` recovers the same
/// AgentId → Ed25519 binding — the cross-run identity property mandated by
/// architect directive 2026-05-02 ruling 13.
pub struct AgentKeypairRegistry {
    keypairs: BTreeMap<AgentId, AgentKeypair>,
    manifest_path: PathBuf,
    durable: Option<DurableConfig>,
}

/// TB-9 Atom 1: durable-mode metadata. Path + password are kept here so
/// `persist_manifest` can re-encrypt the keystore on every new agent.
struct DurableConfig {
    keystore_path: PathBuf,
    password: secrecy::SecretString,
}

impl fmt::Debug for AgentKeypairRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AgentKeypairRegistry")
            .field("manifest_path", &self.manifest_path)
            .field("agent_count", &self.keypairs.len())
            .field(
                "agent_ids",
                &self.keypairs.keys().collect::<Vec<_>>(),
            )
            .field(
                "durable",
                &self.durable.as_ref().map(|d| d.keystore_path.clone()),
            )
            .finish()
    }
}

impl AgentKeypairRegistry {
    /// TRACE_MATRIX FC1-N14: open or initialize an agent keypair registry
    /// rooted at the runtime repo. Manifest written at
    /// `<runtime_repo>/agent_pubkeys.json`. Mirrors TB-6 fail-closed
    /// non-empty-runtime-repo gate (refuses reopen when manifest exists).
    pub fn open(runtime_repo_path: &Path) -> Result<Self, AgentKeypairError> {
        let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
        if manifest_path.exists() {
            return Err(AgentKeypairError::ManifestAlreadyExists {
                path: manifest_path,
            });
        }
        let registry = Self {
            keypairs: BTreeMap::new(),
            manifest_path,
            durable: None,
        };
        registry.persist_manifest()?;
        Ok(registry)
    }

    /// TRACE_MATRIX FC1-N14 (TB-9 Atom 1): open or initialize an agent keypair
    /// registry **with durable cross-run identity**. The encrypted keystore at
    /// `durable_keystore_path` (typically `~/.turingos/keystore/agent_keystore.enc`
    /// per [`crate::runtime::agent_keystore::default_agent_keystore_path`]) is
    /// loaded if present — every saved AgentId → secret binding is reconstructed
    /// in-memory before the first `sign()` call.
    ///
    /// Per-run manifest at `<runtime_repo>/agent_pubkeys.json` is still written
    /// (defense-in-depth replay sidecar; TB-7 semantics retained). The
    /// fail-closed-on-existing manifest gate STILL applies — runtime_repo is
    /// supposed to be fresh per evaluator run.
    ///
    /// On every subsequent `sign()` that triggers a fresh keypair generation,
    /// the durable keystore is re-encrypted and atomically written so the new
    /// AgentId → secret binding survives evaluator exit.
    pub fn generate_or_load_durable(
        runtime_repo_path: &Path,
        durable_keystore_path: &Path,
        password: secrecy::SecretString,
    ) -> Result<Self, AgentKeypairError> {
        let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
        if manifest_path.exists() {
            return Err(AgentKeypairError::ManifestAlreadyExists {
                path: manifest_path,
            });
        }
        let (secrets_map, _fresh) =
            crate::runtime::agent_keystore::load_or_empty(durable_keystore_path, &password)
                .map_err(|e| AgentKeypairError::Serde(format!("durable keystore: {e}")))?;
        let mut keypairs: BTreeMap<AgentId, AgentKeypair> = BTreeMap::new();
        for (agent_id_raw, seed) in secrets_map {
            keypairs.insert(AgentId(agent_id_raw), AgentKeypair::from_secret_bytes(seed));
        }
        let registry = Self {
            keypairs,
            manifest_path,
            durable: Some(DurableConfig {
                keystore_path: durable_keystore_path.to_path_buf(),
                password,
            }),
        };
        registry.persist_manifest()?;
        Ok(registry)
    }

    /// TRACE_MATRIX FC2-INV8 (TB-G G1.1 architect §8 SIGNED 2026-05-11 user
    /// directive "断点续作是本项目的核心"; Turing-machine fundamentalist
    /// reading of FC2 §3.2 "every real evidence run must be replayable from
    /// genesis_report + ChainTape + CAS + **agent registry** + system pubkeys"):
    /// open an agent keypair registry on a non-empty runtime_repo by
    /// loading the existing `agent_pubkeys.json` manifest INSTEAD of
    /// fail-closing with `ManifestAlreadyExists`.
    ///
    /// Required by G1.1 resume mode at the **binary** layer: the
    /// evaluator's swarm bootstrap re-attaches to a runtime_repo that
    /// already contains both `pinned_pubkeys.json` (system pubkeys —
    /// G1.1 kernel-side covers) and `agent_pubkeys.json` (agent registry
    /// — this entry covers). Without this, the binary path panics on
    /// the second problem even though the kernel sequencer happily
    /// resumes.
    ///
    /// Semantics:
    /// - **Manifest absent** → fail-closed `ManifestAbsentInResume`.
    ///   Resume mode is contractually predicated on the manifest
    ///   existing; falling through to fresh init would silently
    ///   discard the prior agent registry (constitution violation).
    /// - **Manifest present** → parse it (fail-closed on parse error),
    ///   load secrets from the durable keystore (same as
    ///   `generate_or_load_durable`), reconstruct the in-memory
    ///   keypair map from the secrets (the manifest is the public
    ///   side of those secrets — replay-verifiable), and DO NOT
    ///   re-persist the manifest on construction (preserves the
    ///   existing on-disk bytes verbatim).
    ///
    /// Subsequent `sign()` calls that discover a new agent_id still
    /// trigger `persist_manifest()` via `get_or_create()` — new agents
    /// added during the resumed run are appended; existing agents
    /// remain untouched.
    pub fn resume_existing_durable(
        runtime_repo_path: &Path,
        durable_keystore_path: &Path,
        password: secrecy::SecretString,
    ) -> Result<Self, AgentKeypairError> {
        let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
        if !manifest_path.exists() {
            return Err(AgentKeypairError::ManifestAbsentInResume {
                path: manifest_path,
            });
        }
        // Parse the existing manifest — the public side of the agent
        // registry. Every agent listed here MUST have a secret in the
        // durable keystore; otherwise the resumed registry would
        // silently lose a signing capability and the tape's
        // agent_registry replay input would diverge from the
        // post-resume in-memory state.
        let manifest_bytes = std::fs::read(&manifest_path).map_err(AgentKeypairError::Io)?;
        let parsed: AgentPubkeyManifest = serde_json::from_slice(&manifest_bytes)
            .map_err(|e| AgentKeypairError::Serde(format!("agent_pubkeys.json: {e}")))?;
        let (secrets_map, _fresh) =
            crate::runtime::agent_keystore::load_or_empty(durable_keystore_path, &password)
                .map_err(|e| AgentKeypairError::Serde(format!("durable keystore: {e}")))?;
        let mut keypairs: BTreeMap<AgentId, AgentKeypair> = BTreeMap::new();
        for (agent_id_raw, seed) in secrets_map {
            keypairs.insert(AgentId(agent_id_raw), AgentKeypair::from_secret_bytes(seed));
        }

        // TB-G G1.1 R2 closure (Codex G2 R1.5 Q1+Q8 CHALLENGE): cross-check
        // every agent in the manifest MUST have a corresponding secret in
        // the durable keystore AND the derived pubkey MUST match the
        // manifest pubkey verbatim. Catches:
        // (a) keystore was wiped while manifest survived (registry/keystore
        //     drift),
        // (b) keystore covers different agents (wrong keystore path / wrong
        //     password),
        // (c) manifest was tampered (manifest pubkey != derived pubkey).
        // Fail-closed in all three cases — silent partial resume would
        // violate FC2 §3.2 "agent_registry is a replay input" because the
        // in-memory registry would no longer reproduce the on-disk
        // manifest's binding.
        for (agent_id_raw, manifest_pubkey_hex) in &parsed.agents {
            let agent_id = AgentId(agent_id_raw.clone());
            let keypair = keypairs.get(&agent_id).ok_or_else(|| {
                AgentKeypairError::ResumeKeystoreInconsistent {
                    agent_id: agent_id_raw.clone(),
                    reason: format!(
                        "agent_pubkeys.json lists agent_id={agent_id_raw:?} but the \
                         durable keystore at {durable_keystore_path:?} has no \
                         corresponding secret — keystore was wiped, password is \
                         wrong, or the runtime_repo / keystore are from different runs"
                    ),
                }
            })?;
            let derived_pubkey_hex = keypair.public_key().to_hex();
            if &derived_pubkey_hex != manifest_pubkey_hex {
                return Err(AgentKeypairError::ResumeKeystoreInconsistent {
                    agent_id: agent_id_raw.clone(),
                    reason: format!(
                        "manifest pubkey {manifest_pubkey_hex:?} does NOT match keystore-\
                         derived pubkey {derived_pubkey_hex:?} — possible manifest \
                         tampering or split-brain keystore"
                    ),
                });
            }
        }

        Ok(Self {
            keypairs,
            manifest_path,
            durable: Some(DurableConfig {
                keystore_path: durable_keystore_path.to_path_buf(),
                password,
            }),
        })
    }

    /// TRACE_MATRIX FC1-N14: get-or-create the keypair for `agent_id`. New
    /// agents auto-generate a fresh keypair (and update the on-disk manifest);
    /// existing agents return the cached keypair.
    pub fn get_or_create(&mut self, agent_id: &AgentId) -> Result<&AgentKeypair, AgentKeypairError> {
        if !self.keypairs.contains_key(agent_id) {
            let kp = AgentKeypair::generate()?;
            self.keypairs.insert(agent_id.clone(), kp);
            self.persist_manifest()?;
        }
        Ok(self.keypairs.get(agent_id).expect("just inserted"))
    }

    /// TRACE_MATRIX FC1-N14: sign a 32-byte canonical digest under `agent_id`.
    /// Generates the keypair on-demand if absent. This is the primary call
    /// site for evaluator append-branch / OMEGA-branch routing in Atom 2/3.
    pub fn sign(
        &mut self,
        agent_id: &AgentId,
        digest: [u8; 32],
    ) -> Result<AgentSignature, AgentKeypairError> {
        let keypair = self.get_or_create(agent_id)?;
        keypair.sign_digest(digest)
    }

    /// TRACE_MATRIX FC1-N14: snapshot the public-key map as a manifest object
    /// (sorted by AgentId for determinism).
    pub fn manifest(&self) -> AgentPubkeyManifest {
        AgentPubkeyManifest {
            agents: self
                .keypairs
                .iter()
                .map(|(id, kp)| (id.0.clone(), kp.public_key().to_hex()))
                .collect(),
        }
    }

    /// TRACE_MATRIX FC1-N14: path to the on-disk manifest.
    pub fn manifest_path(&self) -> &Path {
        &self.manifest_path
    }

    /// Atomic write: tmp file + rename. JSON pretty-printed for inspection.
    /// TB-9 Atom 1: also re-encrypts + atomically writes the durable keystore
    /// when `self.durable` is populated.
    fn persist_manifest(&self) -> Result<(), AgentKeypairError> {
        let manifest = self.manifest();
        let serialized = serde_json::to_string_pretty(&manifest)
            .map_err(|e| AgentKeypairError::Serde(e.to_string()))?;
        let tmp = self.manifest_path.with_extension("json.tmp");
        {
            let mut f = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&tmp)?;
            f.write_all(serialized.as_bytes())?;
            f.sync_all()?;
        }
        std::fs::rename(&tmp, &self.manifest_path)?;

        if let Some(durable) = &self.durable {
            let mut secrets: BTreeMap<String, [u8; AGENT_SECRET_LEN]> = BTreeMap::new();
            for (id, kp) in &self.keypairs {
                secrets.insert(id.0.clone(), kp.secret_bytes());
            }
            crate::runtime::agent_keystore::save(
                &durable.keystore_path,
                &durable.password,
                &secrets,
            )
            .map_err(|e| AgentKeypairError::Serde(format!("durable keystore: {e}")))?;
        }
        Ok(())
    }
}

// ── Public manifest: deserialized read-side ──────────────────────────────────

/// TRACE_MATRIX FC1-N14: on-disk shape of `agent_pubkeys.json`.
/// `verify_chaintape` (Atom 4) reads this and rebuilds an `AgentPublicKeyMap`
/// to verify each WorkTx signature.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AgentPubkeyManifest {
    /// AgentId.0 → AgentPublicKey hex
    pub agents: BTreeMap<String, String>,
}

impl AgentPubkeyManifest {
    /// TRACE_MATRIX FC1-N14: load and parse the manifest from disk.
    pub fn load(path: &Path) -> Result<Self, AgentKeypairError> {
        let mut f = OpenOptions::new().read(true).open(path)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        let manifest: AgentPubkeyManifest = serde_json::from_slice(&buf)
            .map_err(|e| AgentKeypairError::Serde(e.to_string()))?;
        Ok(manifest)
    }

    /// TRACE_MATRIX FC1-N14: resolve an AgentId to its pinned public key
    /// (None if unknown).
    pub fn get(&self, agent_id: &AgentId) -> Option<AgentPublicKey> {
        self.agents
            .get(&agent_id.0)
            .and_then(|hex| AgentPublicKey::from_hex(hex).ok())
    }
}

// ── Verification (replay-side) ───────────────────────────────────────────────

/// TRACE_MATRIX FC1-N14: verify an agent signature against a manifest-pinned
/// public key. Returns `Ok(())` on valid signature; `Err(...)` otherwise.
/// Used by Atom 4 `verify_chaintape` to re-check every WorkTx during replay.
pub fn verify_agent_signature(
    signature: &AgentSignature,
    digest: &[u8; 32],
    pubkey: &AgentPublicKey,
) -> Result<(), AgentKeypairError> {
    let verifying = VerifyingKey::from_bytes(pubkey.as_bytes())
        .map_err(|e| AgentKeypairError::Verify(format!("from_bytes: {e}")))?;
    let sig = Signature::from_bytes(signature.as_bytes());
    verifying
        .verify(digest, &sig)
        .map_err(|e| AgentKeypairError::Verify(format!("verify: {e}")))
}

// ── Errors ───────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N14: agent keypair / manifest / signing error taxonomy.
#[derive(Debug)]
pub enum AgentKeypairError {
    Io(std::io::Error),
    Entropy(getrandom::Error),
    Serde(String),
    InvalidFormat(&'static str),
    ManifestAlreadyExists { path: PathBuf },
    /// TB-G G1.1 resume mode (architect §8 SIGNED 2026-05-11; user directive
    /// "断点续作是本项目的核心"): `resume_existing_durable` was called but
    /// the manifest at `path` does not exist. Fail-closed so callers can
    /// distinguish "fresh-init was intended, manifest absent" from
    /// "resume-was-intended, manifest absent" — the latter is an invariant
    /// violation worth panicking on rather than silently reinitializing.
    ManifestAbsentInResume { path: PathBuf },
    /// TB-G G1.1 R2 (Codex G2 R1.5 Q1+Q8 CHALLENGE closure 2026-05-11):
    /// the on-disk `agent_pubkeys.json` references `agent_id` but the
    /// durable keystore either has no secret for it or has a secret that
    /// produces a different public key. Either way the resumed registry
    /// can't faithfully reproduce the manifest's signing capabilities, so
    /// FC2 §3.2 "agent_registry is a replay input" would silently
    /// degrade. Fail-closed.
    ResumeKeystoreInconsistent { agent_id: String, reason: String },
    Verify(String),
}

impl fmt::Display for AgentKeypairError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io: {e}"),
            Self::Entropy(e) => write!(f, "getrandom entropy: {e}"),
            Self::Serde(e) => write!(f, "serde: {e}"),
            Self::InvalidFormat(s) => write!(f, "invalid format: {s}"),
            Self::ManifestAlreadyExists { path } => {
                write!(f, "agent_pubkeys.json already exists at {path:?}")
            }
            Self::ManifestAbsentInResume { path } => {
                write!(
                    f,
                    "resume mode: agent_pubkeys.json missing at {path:?}; \
                     cannot resume the agent registry without a persisted \
                     manifest (FC2 §3.2 mandates agent_registry as a replay input)"
                )
            }
            Self::ResumeKeystoreInconsistent { agent_id, reason } => {
                write!(
                    f,
                    "resume mode: agent_pubkeys.json / durable keystore inconsistency \
                     for agent_id={agent_id:?}: {reason}"
                )
            }
            Self::Verify(e) => write!(f, "agent signature verify: {e}"),
        }
    }
}

impl std::error::Error for AgentKeypairError {}

impl From<std::io::Error> for AgentKeypairError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn fresh_repo() -> TempDir {
        TempDir::new().expect("tempdir")
    }

    fn fresh_digest(seed: u8) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update([seed]);
        h.finalize().into()
    }

    /// U-A1.a — generate produces a non-zero public key + working signature.
    #[test]
    fn generate_produces_signing_keypair() {
        let kp = AgentKeypair::generate().expect("generate");
        assert_ne!(*kp.public_key().as_bytes(), [0u8; AGENT_PUBLIC_LEN]);
        let digest = fresh_digest(0);
        let sig = kp.sign_digest(digest).expect("sign");
        assert!(verify_agent_signature(&sig, &digest, &kp.public_key()).is_ok());
    }

    /// U-A1.b — registry persists manifest with the agent's pubkey after first sign.
    #[test]
    fn registry_persists_manifest_on_first_use() {
        let repo = fresh_repo();
        let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
        assert!(reg.manifest_path().exists());
        let agent = AgentId("n1".into());
        let _sig = reg.sign(&agent, fresh_digest(1)).expect("sign");
        let loaded = AgentPubkeyManifest::load(reg.manifest_path()).expect("load");
        assert!(loaded.get(&agent).is_some(), "n1 missing from manifest");
    }

    /// U-A1.c — same agent reuses cached keypair across calls; signatures verify
    /// under the same pinned pubkey.
    #[test]
    fn same_agent_reuses_keypair_across_signs() {
        let repo = fresh_repo();
        let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
        let agent = AgentId("swarm_a".into());
        let sig1 = reg.sign(&agent, fresh_digest(2)).expect("sign1");
        let sig2 = reg.sign(&agent, fresh_digest(3)).expect("sign2");
        let pubkey = reg
            .manifest()
            .get(&agent)
            .expect("pubkey");
        assert!(verify_agent_signature(&sig1, &fresh_digest(2), &pubkey).is_ok());
        assert!(verify_agent_signature(&sig2, &fresh_digest(3), &pubkey).is_ok());
    }

    /// U-A1.d — manifest survives reload (load from disk == in-memory snapshot).
    #[test]
    fn manifest_round_trip() {
        let repo = fresh_repo();
        let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
        let a1 = AgentId("n1".into());
        let a2 = AgentId("swarm_b".into());
        let _ = reg.sign(&a1, fresh_digest(4)).expect("sign1");
        let _ = reg.sign(&a2, fresh_digest(5)).expect("sign2");
        let in_mem = reg.manifest();
        let loaded = AgentPubkeyManifest::load(reg.manifest_path()).expect("load");
        assert_eq!(in_mem, loaded);
        // Both agents present, ordering deterministic (BTreeMap).
        assert_eq!(loaded.agents.len(), 2);
        assert!(loaded.get(&a1).is_some());
        assert!(loaded.get(&a2).is_some());
    }

    /// U-A1.e — re-opening a runtime repo whose manifest already exists is
    /// rejected (fail-closed; mirrors TB-6 non-empty-runtime-repo gate).
    #[test]
    fn registry_open_refuses_existing_manifest() {
        let repo = fresh_repo();
        let _reg = AgentKeypairRegistry::open(repo.path()).expect("first open");
        let err = AgentKeypairRegistry::open(repo.path()).expect_err("second open");
        match err {
            AgentKeypairError::ManifestAlreadyExists { .. } => {}
            other => panic!("expected ManifestAlreadyExists, got {other}"),
        }
    }

    /// U-A1.f — wrong pubkey rejects valid signature (negative test).
    #[test]
    fn wrong_pubkey_rejects_signature() {
        let kp1 = AgentKeypair::generate().expect("kp1");
        let kp2 = AgentKeypair::generate().expect("kp2");
        let digest = fresh_digest(6);
        let sig = kp1.sign_digest(digest).expect("sign");
        assert!(verify_agent_signature(&sig, &digest, &kp2.public_key()).is_err());
    }

    // ── TB-9 Atom 1 — durable cross-run identity tests ──────────────────────

    /// U-TB9.a — fresh durable boot generates an empty registry; first sign
    /// triggers keypair generation AND persists encrypted keystore on disk.
    #[test]
    fn durable_first_boot_persists_secret() {
        let repo = fresh_repo();
        let keystore_dir = fresh_repo();
        let keystore_path = keystore_dir.path().join("agent_keystore.enc");
        let pwd = secrecy::SecretString::new("tb9-durable-test".into());

        let mut reg =
            AgentKeypairRegistry::generate_or_load_durable(repo.path(), &keystore_path, pwd.clone())
                .expect("first boot");
        let agent = AgentId("n1".into());
        let _sig = reg.sign(&agent, fresh_digest(11)).expect("sign");

        assert!(keystore_path.exists(), "durable keystore not written");
        let bytes = std::fs::read(&keystore_path).unwrap();
        assert!(bytes.starts_with(b"TOS4AGTKEY1"), "magic mismatch");
    }

    /// U-TB9.b — second boot loads existing keystore; same agent_id produces
    /// the same pubkey across the run boundary (cross-run identity).
    #[test]
    fn durable_second_boot_recovers_same_pubkey() {
        let keystore_dir = fresh_repo();
        let keystore_path = keystore_dir.path().join("agent_keystore.enc");
        let pwd = secrecy::SecretString::new("tb9-durable-test".into());
        let agent = AgentId("n1".into());

        // Run A: generate + sign + record pubkey.
        let pubkey_a = {
            let repo_a = fresh_repo();
            let mut reg_a = AgentKeypairRegistry::generate_or_load_durable(
                repo_a.path(),
                &keystore_path,
                pwd.clone(),
            )
            .expect("run A boot");
            let _ = reg_a.sign(&agent, fresh_digest(20)).expect("run A sign");
            reg_a.manifest().get(&agent).expect("run A pubkey")
        }; // reg_a drops here

        // Run B: re-load + sign + verify pubkey is identical.
        let repo_b = fresh_repo();
        let mut reg_b = AgentKeypairRegistry::generate_or_load_durable(
            repo_b.path(),
            &keystore_path,
            pwd.clone(),
        )
        .expect("run B boot");
        let sig_b = reg_b.sign(&agent, fresh_digest(21)).expect("run B sign");
        let pubkey_b = reg_b.manifest().get(&agent).expect("run B pubkey");

        assert_eq!(
            pubkey_a, pubkey_b,
            "cross-run identity broken: pubkey changed across runs"
        );
        assert!(
            verify_agent_signature(&sig_b, &fresh_digest(21), &pubkey_b).is_ok(),
            "run B signature must verify under the durable pubkey"
        );
    }

    /// U-TB9.c — wrong password on second boot rejects (no silent regenerate).
    #[test]
    fn durable_wrong_password_rejected() {
        let keystore_dir = fresh_repo();
        let keystore_path = keystore_dir.path().join("agent_keystore.enc");
        let pwd_a = secrecy::SecretString::new("tb9-correct".into());
        let pwd_b = secrecy::SecretString::new("tb9-wrong".into());
        let agent = AgentId("n1".into());

        let repo_a = fresh_repo();
        let mut reg_a = AgentKeypairRegistry::generate_or_load_durable(
            repo_a.path(),
            &keystore_path,
            pwd_a,
        )
        .expect("run A");
        let _ = reg_a.sign(&agent, fresh_digest(30)).expect("sign");

        let repo_b = fresh_repo();
        let err = AgentKeypairRegistry::generate_or_load_durable(
            repo_b.path(),
            &keystore_path,
            pwd_b,
        )
        .expect_err("wrong password must fail");
        match err {
            AgentKeypairError::Serde(msg) => assert!(
                msg.contains("crypto") || msg.contains("authentication"),
                "expected crypto authentication failure, got {msg}"
            ),
            other => panic!("expected Serde(crypto), got {other}"),
        }
    }
}
