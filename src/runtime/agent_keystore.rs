//! TB-9 Atom 1 — Durable agent keystore: encrypted-at-rest persistence of
//! per-agent Ed25519 secrets across evaluator restarts.
//!
//! Mirrors `src/bottom_white/ledger/system_keypair.rs:417-463` exactly:
//! - default path `~/.turingos/keystore/agent_keystore.enc` (env override
//!   `TURINGOS_AGENT_KEYSTORE_PATH`)
//! - Argon2id KDF (m=64MiB, t=3, p=4 default; env-tunable via the same
//!   `TURINGOS_KDF_*` knobs used for the system keystore)
//! - ChaCha20-Poly1305 AEAD encryption-at-rest
//! - atomic write 0600 (mode bits enforced on Unix)
//! - format magic `TOS4AGTKEY1` (distinct from system's `TOS4SYSKEY1`)
//!
//! Plaintext shape: bincode-encoded `BTreeMap<String, [u8; 32]>` mapping
//! `AgentId.0` → 32-byte Ed25519 secret seed. Public keys are NOT stored —
//! they are recomputed from each seed at load time.
//!
//! TRACE_MATRIX FC1-N14 (durable agent identity primitive; satisfies
//! architect TB-9 mandate "agent durable key registry" + "cross-run
//! identity").

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Nonce};
use secrecy::{ExposeSecret, SecretString};
use std::collections::BTreeMap;
use std::env;
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use zeroize::Zeroize;

const DEFAULT_KDF_MEMORY_KIB: u32 = 65_536;
const DEFAULT_KDF_ITER: u32 = 3;
const DEFAULT_KDF_LANES: u32 = 4;
const DERIVED_KEY_LEN: usize = 32;
const SECRET_KEY_LEN: usize = 32;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const FORMAT_MAGIC: &[u8; 11] = b"TOS4AGTKEY1";
const FORMAT_VERSION: u8 = 1;

/// TRACE_MATRIX FC1-N14: agent keystore lifecycle / crypto error taxonomy.
#[derive(Debug)]
pub enum AgentKeystoreError {
    Io(std::io::Error),
    Entropy(getrandom::Error),
    KdfParam(String),
    Kdf(argon2::Error),
    Crypto(&'static str),
    InvalidFormat(&'static str),
    HomeUnavailable,
    Bincode(String),
}

impl fmt::Display for AgentKeystoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "agent keystore I/O failed: {e}"),
            Self::Entropy(e) => write!(f, "agent keystore entropy failed: {e}"),
            Self::KdfParam(msg) => write!(f, "agent keystore KDF parameter invalid: {msg}"),
            Self::Kdf(e) => write!(f, "agent keystore KDF failed: {e}"),
            Self::Crypto(msg) => write!(f, "agent keystore crypto failed: {msg}"),
            Self::InvalidFormat(msg) => write!(f, "agent keystore format invalid: {msg}"),
            Self::HomeUnavailable => write!(f, "agent keystore default path requires HOME"),
            Self::Bincode(msg) => write!(f, "agent keystore serde failed: {msg}"),
        }
    }
}

impl std::error::Error for AgentKeystoreError {}

impl From<std::io::Error> for AgentKeystoreError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// TB-9 Atom 2 helper: read the durable keystore password from env (with a
/// hardcoded local-dev fallback, acceptable for solo-runs per
/// `feedback_kolmogorov_compression`). Wraps the result in `SecretString` so
/// callers (binaries) don't need to depend on `secrecy` directly.
pub fn keystore_password_from_env() -> SecretString {
    let raw = env::var("TURINGOS_AGENT_KEYSTORE_PASSWORD")
        .unwrap_or_else(|_| "tb9-local-dev-password-replace-in-production".to_string());
    SecretString::new(raw.into())
}

/// TRACE_MATRIX FC1-N14: resolve `~/.turingos/keystore/agent_keystore.enc`.
///
/// `TURINGOS_AGENT_KEYSTORE_PATH` overrides. Default never points into the
/// repository, CAS, or runtime_repo directories.
pub fn default_agent_keystore_path() -> Result<PathBuf, AgentKeystoreError> {
    if let Ok(path) = env::var("TURINGOS_AGENT_KEYSTORE_PATH") {
        return Ok(PathBuf::from(path));
    }
    let home = env::var("HOME").map_err(|_| AgentKeystoreError::HomeUnavailable)?;
    Ok(PathBuf::from(home)
        .join(".turingos")
        .join("keystore")
        .join("agent_keystore.enc"))
}

/// TRACE_MATRIX FC1-N14: load durable keystore from disk if present, else
/// return an empty map. Returns `(secrets, fresh)` where `fresh=true` if the
/// path did not exist at call time.
pub fn load_or_empty(
    keystore_path: &Path,
    password: &SecretString,
) -> Result<(BTreeMap<String, [u8; SECRET_KEY_LEN]>, bool), AgentKeystoreError> {
    if !keystore_path.exists() {
        return Ok((BTreeMap::new(), true));
    }
    let bytes = fs::read(keystore_path)?;
    let encoded = EncryptedBundle::decode(&bytes)?;
    let mut key = derive_key(password, &encoded.salt, encoded.kdf)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&key)
        .map_err(|_| AgentKeystoreError::Crypto("bad cipher key"))?;
    let plaintext = cipher
        .decrypt(
            Nonce::from_slice(&encoded.nonce),
            encoded.ciphertext.as_ref(),
        )
        .map_err(|_| AgentKeystoreError::Crypto("keystore authentication failed"))?;
    key.zeroize();
    let cfg = bincode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let (secrets, consumed): (BTreeMap<String, [u8; SECRET_KEY_LEN]>, usize) =
        bincode::serde::decode_from_slice(&plaintext, cfg)
            .map_err(|e| AgentKeystoreError::Bincode(e.to_string()))?;
    if consumed != plaintext.len() {
        return Err(AgentKeystoreError::Bincode(format!(
            "trailing bytes: consumed {consumed} of {}",
            plaintext.len()
        )));
    }
    Ok((secrets, false))
}

/// TRACE_MATRIX FC1-N14: encrypt + atomic-write the durable keystore.
pub fn save(
    keystore_path: &Path,
    password: &SecretString,
    secrets: &BTreeMap<String, [u8; SECRET_KEY_LEN]>,
) -> Result<(), AgentKeystoreError> {
    let kdf = KdfParams::from_env()?;
    let mut salt = [0u8; SALT_LEN];
    let mut nonce = [0u8; NONCE_LEN];
    getrandom::getrandom(&mut salt).map_err(AgentKeystoreError::Entropy)?;
    getrandom::getrandom(&mut nonce).map_err(AgentKeystoreError::Entropy)?;

    let mut key = derive_key(password, &salt, kdf)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&key)
        .map_err(|_| AgentKeystoreError::Crypto("bad cipher key"))?;
    let cfg = bincode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let mut plaintext = bincode::serde::encode_to_vec(secrets, cfg)
        .map_err(|e| AgentKeystoreError::Bincode(e.to_string()))?;
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
        .map_err(|_| AgentKeystoreError::Crypto("keystore encryption failed"))?;
    plaintext.zeroize();
    key.zeroize();

    let bundle = EncryptedBundle {
        kdf,
        salt,
        nonce,
        ciphertext,
    }
    .encode()?;

    write_keystore_0600_atomic(keystore_path, &bundle)?;
    Ok(())
}

fn derive_key(
    password: &SecretString,
    salt: &[u8; SALT_LEN],
    kdf: KdfParams,
) -> Result<[u8; DERIVED_KEY_LEN], AgentKeystoreError> {
    let params = Params::new(
        kdf.memory_kib,
        kdf.iterations,
        kdf.lanes,
        Some(DERIVED_KEY_LEN),
    )
    .map_err(|err| AgentKeystoreError::KdfParam(err.to_string()))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; DERIVED_KEY_LEN];
    argon2
        .hash_password_into(password.expose_secret().as_bytes(), salt, &mut key)
        .map_err(AgentKeystoreError::Kdf)?;
    Ok(key)
}

fn write_keystore_0600_atomic(path: &Path, bytes: &[u8]) -> Result<(), AgentKeystoreError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("enc.tmp");
    {
        let mut options = OpenOptions::new();
        options.write(true).create(true).truncate(true);
        set_open_options_mode_0600(&mut options);
        let mut file = options.open(&tmp)?;
        file.write_all(bytes)?;
        file.sync_all()?;
    }
    fs::rename(&tmp, path)?;
    set_file_permissions_0600(path)?;
    Ok(())
}

#[cfg(unix)]
fn set_open_options_mode_0600(options: &mut OpenOptions) {
    use std::os::unix::fs::OpenOptionsExt;
    options.mode(0o600);
}

#[cfg(not(unix))]
fn set_open_options_mode_0600(_options: &mut OpenOptions) {}

#[cfg(unix)]
fn set_file_permissions_0600(path: &Path) -> Result<(), AgentKeystoreError> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).map_err(AgentKeystoreError::Io)
}

#[cfg(not(unix))]
fn set_file_permissions_0600(_path: &Path) -> Result<(), AgentKeystoreError> {
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct KdfParams {
    memory_kib: u32,
    iterations: u32,
    lanes: u32,
}

impl KdfParams {
    fn from_env() -> Result<Self, AgentKeystoreError> {
        Ok(Self {
            memory_kib: read_env_u32("TURINGOS_KDF_MEMORY_KIB", DEFAULT_KDF_MEMORY_KIB)?,
            iterations: read_env_u32("TURINGOS_KDF_ITER", DEFAULT_KDF_ITER)?,
            lanes: read_env_u32("TURINGOS_KDF_LANES", DEFAULT_KDF_LANES)?,
        })
    }
}

fn read_env_u32(name: &str, default: u32) -> Result<u32, AgentKeystoreError> {
    match env::var(name) {
        Ok(value) => {
            let parsed = value
                .parse::<u32>()
                .map_err(|_| AgentKeystoreError::KdfParam(format!("{name} must be u32")))?;
            if parsed == 0 {
                return Err(AgentKeystoreError::KdfParam(format!(
                    "{name} must be non-zero"
                )));
            }
            Ok(parsed)
        }
        Err(env::VarError::NotPresent) => Ok(default),
        Err(env::VarError::NotUnicode(_)) => {
            Err(AgentKeystoreError::KdfParam(format!("{name} is not unicode")))
        }
    }
}

struct EncryptedBundle {
    kdf: KdfParams,
    salt: [u8; SALT_LEN],
    nonce: [u8; NONCE_LEN],
    ciphertext: Vec<u8>,
}

impl EncryptedBundle {
    fn encode(self) -> Result<Vec<u8>, AgentKeystoreError> {
        let ciphertext_len = u32::try_from(self.ciphertext.len())
            .map_err(|_| AgentKeystoreError::InvalidFormat("ciphertext too large"))?;
        let mut out = Vec::with_capacity(
            FORMAT_MAGIC.len() + 1 + 4 + 4 + 4 + SALT_LEN + NONCE_LEN + 4 + self.ciphertext.len(),
        );
        out.extend_from_slice(FORMAT_MAGIC);
        out.push(FORMAT_VERSION);
        out.extend_from_slice(&self.kdf.memory_kib.to_be_bytes());
        out.extend_from_slice(&self.kdf.iterations.to_be_bytes());
        out.extend_from_slice(&self.kdf.lanes.to_be_bytes());
        out.extend_from_slice(&self.salt);
        out.extend_from_slice(&self.nonce);
        out.extend_from_slice(&ciphertext_len.to_be_bytes());
        out.extend_from_slice(&self.ciphertext);
        Ok(out)
    }

    fn decode(bytes: &[u8]) -> Result<Self, AgentKeystoreError> {
        let mut cursor = Cursor::new(bytes);
        if cursor.read(FORMAT_MAGIC.len())? != FORMAT_MAGIC {
            return Err(AgentKeystoreError::InvalidFormat("bad magic"));
        }
        if cursor.read_u8()? != FORMAT_VERSION {
            return Err(AgentKeystoreError::InvalidFormat("bad version"));
        }
        let kdf = KdfParams {
            memory_kib: cursor.read_u32()?,
            iterations: cursor.read_u32()?,
            lanes: cursor.read_u32()?,
        };
        let mut salt = [0u8; SALT_LEN];
        salt.copy_from_slice(cursor.read(SALT_LEN)?);
        let mut nonce = [0u8; NONCE_LEN];
        nonce.copy_from_slice(cursor.read(NONCE_LEN)?);
        let ciphertext_len = cursor.read_u32()? as usize;
        let ciphertext = cursor.read(ciphertext_len)?.to_vec();
        if !cursor.is_finished() {
            return Err(AgentKeystoreError::InvalidFormat("trailing bytes"));
        }
        Ok(Self {
            kdf,
            salt,
            nonce,
            ciphertext,
        })
    }
}

struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    fn read(&mut self, len: usize) -> Result<&'a [u8], AgentKeystoreError> {
        let end = self
            .pos
            .checked_add(len)
            .ok_or(AgentKeystoreError::InvalidFormat("offset overflow"))?;
        if end > self.bytes.len() {
            return Err(AgentKeystoreError::InvalidFormat("truncated keystore"));
        }
        let slice = &self.bytes[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    fn read_u8(&mut self) -> Result<u8, AgentKeystoreError> {
        Ok(self.read(1)?[0])
    }

    fn read_u32(&mut self) -> Result<u32, AgentKeystoreError> {
        let mut out = [0u8; 4];
        out.copy_from_slice(self.read(4)?);
        Ok(u32::from_be_bytes(out))
    }

    fn is_finished(&self) -> bool {
        self.pos == self.bytes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::SecretString;
    use tempfile::TempDir;

    fn fixed_password() -> SecretString {
        SecretString::new("tb9-test-password".into())
    }

    /// U-A2.a — fresh load returns empty map; subsequent save then load returns identical content.
    #[test]
    fn load_then_save_round_trip() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("agent_keystore.enc");
        let pwd = fixed_password();

        let (loaded, fresh) = load_or_empty(&path, &pwd).expect("fresh load");
        assert!(loaded.is_empty());
        assert!(fresh);

        let mut secrets: BTreeMap<String, [u8; 32]> = BTreeMap::new();
        secrets.insert("n1".into(), [1u8; 32]);
        secrets.insert("swarm_b".into(), [2u8; 32]);
        save(&path, &pwd, &secrets).expect("save");

        assert!(path.exists());
        let bytes = fs::read(&path).unwrap();
        assert!(bytes.starts_with(b"TOS4AGTKEY1"), "format magic mismatch");

        let (reloaded, fresh2) = load_or_empty(&path, &pwd).expect("reload");
        assert!(!fresh2);
        assert_eq!(reloaded.len(), 2);
        assert_eq!(reloaded.get("n1").unwrap(), &[1u8; 32]);
        assert_eq!(reloaded.get("swarm_b").unwrap(), &[2u8; 32]);
    }

    /// U-A2.b — wrong password fails to decrypt (not silent regenerate).
    #[test]
    fn wrong_password_fails() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("agent_keystore.enc");

        let mut secrets: BTreeMap<String, [u8; 32]> = BTreeMap::new();
        secrets.insert("n1".into(), [1u8; 32]);
        save(&path, &fixed_password(), &secrets).expect("save");

        let wrong = SecretString::new("not-the-password".into());
        let err = load_or_empty(&path, &wrong).expect_err("wrong password must fail");
        match err {
            AgentKeystoreError::Crypto(_) => {}
            other => panic!("expected Crypto error, got {other}"),
        }
    }

    /// U-A2.c — corrupted ciphertext fails (not silent regenerate).
    #[test]
    fn corrupted_keystore_fails() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("agent_keystore.enc");

        let mut secrets: BTreeMap<String, [u8; 32]> = BTreeMap::new();
        secrets.insert("n1".into(), [9u8; 32]);
        save(&path, &fixed_password(), &secrets).expect("save");

        let mut bytes = fs::read(&path).unwrap();
        let len = bytes.len();
        bytes[len - 5] ^= 0xFF;
        fs::write(&path, &bytes).unwrap();

        let err = load_or_empty(&path, &fixed_password()).expect_err("corrupt must fail");
        match err {
            AgentKeystoreError::Crypto(_) | AgentKeystoreError::InvalidFormat(_) => {}
            other => panic!("expected Crypto or InvalidFormat, got {other}"),
        }
    }

    /// U-A2.d — env override `TURINGOS_AGENT_KEYSTORE_PATH` honored.
    #[test]
    fn env_override_path() {
        // Set + read in a single thread; restore on drop guard.
        struct EnvGuard {
            key: &'static str,
            prev: Option<String>,
        }
        impl Drop for EnvGuard {
            fn drop(&mut self) {
                match self.prev.take() {
                    Some(v) => std::env::set_var(self.key, v),
                    None => std::env::remove_var(self.key),
                }
            }
        }
        let _guard = EnvGuard {
            key: "TURINGOS_AGENT_KEYSTORE_PATH",
            prev: std::env::var("TURINGOS_AGENT_KEYSTORE_PATH").ok(),
        };
        std::env::set_var("TURINGOS_AGENT_KEYSTORE_PATH", "/tmp/tb9-override.enc");
        let p = default_agent_keystore_path().expect("default");
        assert_eq!(p, PathBuf::from("/tmp/tb9-override.enc"));
    }

    /// U-A2.e — file permissions 0600 (Unix only).
    #[cfg(unix)]
    #[test]
    fn keystore_file_is_0600() {
        use std::os::unix::fs::PermissionsExt;
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("agent_keystore.enc");
        let mut secrets: BTreeMap<String, [u8; 32]> = BTreeMap::new();
        secrets.insert("n1".into(), [1u8; 32]);
        save(&path, &fixed_password(), &secrets).expect("save");
        let mode = fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "expected 0600, got {mode:o}");
    }

    /// U-A2.f — bincode shape stable: 32 secrets round-trip preserve bit-equality.
    #[test]
    fn many_agents_round_trip() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("agent_keystore.enc");
        let pwd = fixed_password();
        let mut secrets: BTreeMap<String, [u8; 32]> = BTreeMap::new();
        for i in 0..32u8 {
            let mut s = [0u8; 32];
            s.fill(i);
            secrets.insert(format!("agent_{i}"), s);
        }
        save(&path, &pwd, &secrets).expect("save");
        let (loaded, _) = load_or_empty(&path, &pwd).expect("load");
        assert_eq!(loaded, secrets);
    }
}
