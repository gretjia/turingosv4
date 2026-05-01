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
pub struct AgentKeypairRegistry {
    keypairs: BTreeMap<AgentId, AgentKeypair>,
    manifest_path: PathBuf,
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
        };
        registry.persist_manifest()?;
        Ok(registry)
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
}
