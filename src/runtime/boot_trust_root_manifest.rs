//! TRACE_MATRIX FC2 boot + FC3 readonly guard (TC-002): explicit
//! boot trust-root manifest verifier for TC operationalization.
//!
//! This module is deliberately non-authoritative: it verifies a TC manifest
//! against existing repository facts, predicate boot catalog, and Path-B ref
//! topology. It does not mutate `genesis_payload.toml`, ChainTape, CAS, typed
//! tx schemas, or trust-root authority.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use crate::git_tape_ledger::TcHeadRefs;
use crate::top_white::predicates::registry::{
    BootPredicateManifest, PredicateRegistry, RegisterError,
};

pub const TC_BOOT_MANIFEST_SCHEMA_ID: &str = "turingosv4.tc.boot_trust_root_manifest.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TcBootManifest {
    pub schema_id: String,
    pub constitution_sha256: String,
    pub genesis_payload_sha256: String,
    pub predicate_manifest_root: String,
    pub refs: TcBootRefManifest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TcBootRefManifest {
    pub accepted_l4: String,
    pub rejected_l4e: String,
    pub cas_root: String,
    pub tdma_verified: String,
    pub tdma_tail: String,
}

impl TcBootRefManifest {
    pub fn from_locked_refs(refs: TcHeadRefs) -> Self {
        Self {
            accepted_l4: refs.accepted_l4.to_string(),
            rejected_l4e: refs.rejected_l4e.to_string(),
            cas_root: refs.cas_root.to_string(),
            tdma_verified: refs.tdma_verified.to_string(),
            tdma_tail: refs.tdma_tail.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum TcBootManifestError {
    Read {
        path: PathBuf,
        err: std::io::Error,
    },
    ParseToml {
        path: PathBuf,
        err: toml::de::Error,
    },
    Schema {
        expected: &'static str,
        got: String,
    },
    InvalidHex {
        field: &'static str,
        value: String,
    },
    HashMismatch {
        path: PathBuf,
        expected: String,
        actual: String,
    },
    PredicateRegistry(String),
    RefMismatch {
        field: &'static str,
        expected: String,
        actual: String,
    },
}

impl fmt::Display for TcBootManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, err } => write!(f, "cannot read {}: {err}", path.display()),
            Self::ParseToml { path, err } => {
                write!(f, "cannot parse TC boot manifest {}: {err}", path.display())
            }
            Self::Schema { expected, got } => {
                write!(
                    f,
                    "TC boot manifest schema mismatch: expected {expected}, got {got}"
                )
            }
            Self::InvalidHex { field, value } => {
                write!(
                    f,
                    "TC boot manifest field {field} is not 64-char lowercase hex: {value}"
                )
            }
            Self::HashMismatch {
                path,
                expected,
                actual,
            } => write!(
                f,
                "TC boot manifest hash mismatch for {}: expected {expected}, actual {actual}",
                path.display()
            ),
            Self::PredicateRegistry(msg) => {
                write!(
                    f,
                    "TC boot predicate manifest root cannot be derived: {msg}"
                )
            }
            Self::RefMismatch {
                field,
                expected,
                actual,
            } => write!(
                f,
                "TC boot ref contract mismatch for {field}: expected {expected}, actual {actual}"
            ),
        }
    }
}

impl std::error::Error for TcBootManifestError {}

impl From<RegisterError> for TcBootManifestError {
    fn from(value: RegisterError) -> Self {
        Self::PredicateRegistry(format!("{value:?}"))
    }
}

pub fn read_tc_boot_manifest(path: &Path) -> Result<TcBootManifest, TcBootManifestError> {
    let text = fs::read_to_string(path).map_err(|err| TcBootManifestError::Read {
        path: path.to_path_buf(),
        err,
    })?;
    toml::from_str(&text).map_err(|err| TcBootManifestError::ParseToml {
        path: path.to_path_buf(),
        err,
    })
}

pub fn verify_tc_boot_manifest(
    repo_root: &Path,
    manifest: &TcBootManifest,
) -> Result<(), TcBootManifestError> {
    verify_schema(manifest)?;
    verify_hex_field("constitution_sha256", &manifest.constitution_sha256)?;
    verify_hex_field("genesis_payload_sha256", &manifest.genesis_payload_sha256)?;
    verify_hex_field("predicate_manifest_root", &manifest.predicate_manifest_root)?;
    verify_constitution_hash(repo_root, manifest)?;
    verify_genesis_payload_hash(repo_root, manifest)?;
    verify_predicate_manifest_root(manifest)?;
    verify_ref_contract(&manifest.refs)
}

pub fn verify_tc_boot_constitution_hash(
    repo_root: &Path,
    manifest: &TcBootManifest,
) -> Result<(), TcBootManifestError> {
    verify_schema(manifest)?;
    verify_hex_field("constitution_sha256", &manifest.constitution_sha256)?;
    verify_constitution_hash(repo_root, manifest)
}

pub fn verify_tc_boot_predicates(manifest: &TcBootManifest) -> Result<(), TcBootManifestError> {
    verify_schema(manifest)?;
    verify_hex_field("predicate_manifest_root", &manifest.predicate_manifest_root)?;
    verify_predicate_manifest_root(manifest)
}

fn verify_schema(manifest: &TcBootManifest) -> Result<(), TcBootManifestError> {
    if manifest.schema_id != TC_BOOT_MANIFEST_SCHEMA_ID {
        return Err(TcBootManifestError::Schema {
            expected: TC_BOOT_MANIFEST_SCHEMA_ID,
            got: manifest.schema_id.clone(),
        });
    }
    Ok(())
}

fn verify_constitution_hash(
    repo_root: &Path,
    manifest: &TcBootManifest,
) -> Result<(), TcBootManifestError> {
    verify_file_hash(
        repo_root,
        Path::new("constitution.md"),
        &manifest.constitution_sha256,
    )
}

fn verify_genesis_payload_hash(
    repo_root: &Path,
    manifest: &TcBootManifest,
) -> Result<(), TcBootManifestError> {
    verify_file_hash(
        repo_root,
        Path::new("genesis_payload.toml"),
        &manifest.genesis_payload_sha256,
    )
}

fn verify_file_hash(
    repo_root: &Path,
    rel_path: &Path,
    expected: &str,
) -> Result<(), TcBootManifestError> {
    let full = repo_root.join(rel_path);
    let bytes = fs::read(&full).map_err(|err| TcBootManifestError::Read { path: full, err })?;
    let actual = hex_lower(&Sha256::digest(bytes));
    if actual != expected {
        return Err(TcBootManifestError::HashMismatch {
            path: rel_path.to_path_buf(),
            expected: expected.to_string(),
            actual,
        });
    }
    Ok(())
}

fn verify_predicate_manifest_root(manifest: &TcBootManifest) -> Result<(), TcBootManifestError> {
    let registry = PredicateRegistry::from_boot_manifest(BootPredicateManifest::v8_production())?;
    let actual = hex_lower(&registry.merkle_root());
    if actual != manifest.predicate_manifest_root {
        return Err(TcBootManifestError::HashMismatch {
            path: PathBuf::from("BootPredicateManifest::v8_production"),
            expected: manifest.predicate_manifest_root.clone(),
            actual,
        });
    }
    Ok(())
}

fn verify_ref_contract(refs: &TcBootRefManifest) -> Result<(), TcBootManifestError> {
    let expected = TcBootRefManifest::from_locked_refs(TcHeadRefs::default());
    check_ref("accepted_l4", &expected.accepted_l4, &refs.accepted_l4)?;
    check_ref("rejected_l4e", &expected.rejected_l4e, &refs.rejected_l4e)?;
    check_ref("cas_root", &expected.cas_root, &refs.cas_root)?;
    check_ref(
        "tdma_verified",
        &expected.tdma_verified,
        &refs.tdma_verified,
    )?;
    check_ref("tdma_tail", &expected.tdma_tail, &refs.tdma_tail)
}

fn check_ref(field: &'static str, expected: &str, actual: &str) -> Result<(), TcBootManifestError> {
    if expected != actual {
        return Err(TcBootManifestError::RefMismatch {
            field,
            expected: expected.to_string(),
            actual: actual.to_string(),
        });
    }
    Ok(())
}

fn verify_hex_field(field: &'static str, value: &str) -> Result<(), TcBootManifestError> {
    if value.len() == 64
        && value
            .bytes()
            .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
    {
        return Ok(());
    }
    Err(TcBootManifestError::InvalidHex {
        field,
        value: value.to_string(),
    })
}

fn hex_lower(bytes: impl AsRef<[u8]>) -> String {
    let bytes = bytes.as_ref();
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{b:02x}"));
    }
    out
}
