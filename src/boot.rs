// PPUT-CCL Phase B B7 — Trust Root + Boot freeze (PREREG § 1.8 + § 7).
//
// Constitutional anchor: FC3-S3 `readonly` subgraph (constitution.md
// line 670, system-level flowchart). The constitutional readonly base
// is {constitution-as-ground-truth, logs-archive-as-ground-truth}; B7
// extends this base per PREREG § 1.8 to also cover the case-law glob,
// pre-registration spec, heldout splits, and the PPUT accounting layer.
// TRACE_MATRIX_v0 row FC3-N34 was 📅 Phase 11+ ("FS-level readonly
// check at init") — B7 implements it via SHA-256 manifest verification.
// See `handover/alignment/TRACE_MATRIX_v1_2026-04-25.md`.
//
// At Boot we hash every tracked file and compare against the
// `[trust_root]` manifest in `genesis_payload.toml`. Any mismatch =>
// `TrustRootError::Tampered { .. }`. `src/main.rs` panics with
// `TRUST_ROOT_TAMPERED`.
//
// Manifest derivation (Phase B7, independently re-derived from PREREG
// § 1.8 + B2-B4 mid-term audit recommendation + B6 prompt_guard add):
// see header comment in `genesis_payload.toml`.
//
// TOML parsing is hand-rolled (~30 LOC). The manifest format is flat:
// section header + `"path" = "hash"` lines. Adding a `toml` crate
// dependency would drag in ~5 transitive crates for what we can do
// in-line; compression principle (CLAUDE.md "反奥利奥架构") wins.

use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

/// TRACE_MATRIX FC3-N34: failure variants of the readonly-guard verification.
/// Constitutional role = the diagnostic surface that distinguishes
/// `TRUST_ROOT_TAMPERED` (real readonly violation) from `GenesisRead` /
/// `GenesisParse` (manifest itself unreadable, also a violation but a
/// different fix path).
#[derive(Debug)]
pub enum TrustRootError {
    GenesisRead(std::io::Error),
    GenesisParse(String),
    SectionMissing(&'static str),
    FileRead { path: PathBuf, err: std::io::Error },
    Tampered { path: PathBuf, expected: String, actual: String },
}

impl std::fmt::Display for TrustRootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GenesisRead(e) => write!(f, "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"),
            Self::GenesisParse(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"),
            Self::SectionMissing(s) => write!(f, "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"),
            Self::FileRead { path, err } => write!(f, "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}", path.display()),
            Self::Tampered { path, expected, actual } => write!(
                f,
                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
                path.display(), expected, actual
            ),
        }
    }
}

impl std::error::Error for TrustRootError {}

/// TRACE_MATRIX FC3-N34: implementation of the constitutional `readonly`
/// subgraph (constitution.md FC3, system-level flowchart). Verifies every
/// tracked file's SHA-256 against the `genesis_payload.toml [trust_root]`
/// manifest at Boot. Mismatch => Boot abort; the readonly guarantee that
/// the constitution requires of {constitution, logs} (extended per PREREG
/// § 1.8 to the full PPUT-accounting base) is enforced here.
///
/// `repo_root` is the directory containing `genesis_payload.toml` (typically
/// the workspace root). Paths in the manifest are interpreted relative to it.
pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> {
    let genesis_path = repo_root.join("genesis_payload.toml");
    let genesis_text = fs::read_to_string(&genesis_path).map_err(TrustRootError::GenesisRead)?;
    let manifest = parse_trust_root_section(&genesis_text)?;
    if !has_section(&genesis_text, "pput_accounting_0") {
        return Err(TrustRootError::SectionMissing("pput_accounting_0"));
    }
    for (rel_path, expected) in &manifest {
        let full = repo_root.join(rel_path);
        let bytes = fs::read(&full).map_err(|err| TrustRootError::FileRead {
            path: full.clone(),
            err,
        })?;
        let actual = hex_lower(&Sha256::digest(&bytes));
        if actual != *expected {
            return Err(TrustRootError::Tampered {
                path: full,
                expected: expected.clone(),
                actual,
            });
        }
    }
    Ok(())
}

/// TRACE_MATRIX FC3-N34: helper for `verify_trust_root` — exposed because
/// the trust_root_immutability conformance battery (Phase B7) reads the
/// manifest directly to assert it includes the audit-recommended PPUT
/// accounting layer.
///
/// Parses the `[trust_root]` section of `genesis_payload.toml` into ordered
/// `(path, sha256)` pairs. Hand-rolled — accepts the narrow subset we emit
/// (quoted-key = quoted-value, comments, blank lines).
pub fn parse_trust_root_section(text: &str) -> Result<Vec<(String, String)>, TrustRootError> {
    let mut in_section = false;
    let mut entries = Vec::new();
    for (lineno, raw) in text.lines().enumerate() {
        let line = strip_comment(raw).trim();
        if line.is_empty() {
            continue;
        }
        if let Some(header) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            in_section = header.trim() == "trust_root";
            continue;
        }
        if !in_section {
            continue;
        }
        let (key, value) = line.split_once('=').ok_or_else(|| {
            TrustRootError::GenesisParse(format!("line {}: missing '=' in [trust_root]", lineno + 1))
        })?;
        let key = unquote(key.trim()).ok_or_else(|| {
            TrustRootError::GenesisParse(format!("line {}: key not quoted", lineno + 1))
        })?;
        let value = unquote(value.trim()).ok_or_else(|| {
            TrustRootError::GenesisParse(format!("line {}: value not quoted", lineno + 1))
        })?;
        entries.push((key.to_string(), value.to_string()));
    }
    if entries.is_empty() {
        return Err(TrustRootError::SectionMissing("trust_root"));
    }
    Ok(entries)
}

fn has_section(text: &str, name: &str) -> bool {
    text.lines().any(|raw| {
        let line = strip_comment(raw).trim();
        line
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .map(|h| h.trim() == name)
            .unwrap_or(false)
    })
}

fn strip_comment(line: &str) -> &str {
    let mut in_string = false;
    for (i, c) in line.char_indices() {
        match c {
            '"' => in_string = !in_string,
            '#' if !in_string => return &line[..i],
            _ => {}
        }
    }
    line
}

fn unquote(s: &str) -> Option<&str> {
    s.strip_prefix('"').and_then(|s| s.strip_suffix('"'))
}

fn hex_lower(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(out, "{b:02x}").unwrap();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        // turingosv4 lib is at repo root; CARGO_MANIFEST_DIR == repo root.
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    #[test]
    fn parse_strips_inline_comment_and_blanks() {
        let toml = r#"
            [pput_accounting_0]
            schema_version = "1.0"

            [trust_root]
            # leading comment
            "a/b.rs" = "deadbeef"   # trailing comment
            "c/d.md" = "cafebabe"
        "#;
        let entries = parse_trust_root_section(toml).unwrap();
        assert_eq!(
            entries,
            vec![
                ("a/b.rs".to_string(), "deadbeef".to_string()),
                ("c/d.md".to_string(), "cafebabe".to_string()),
            ]
        );
    }

    #[test]
    fn parse_errors_on_unquoted_key() {
        let toml = "[trust_root]\nfoo = \"deadbeef\"\n";
        assert!(matches!(
            parse_trust_root_section(toml),
            Err(TrustRootError::GenesisParse(_))
        ));
    }

    #[test]
    fn parse_errors_when_section_missing() {
        let toml = "[pput_accounting_0]\nschema_version = \"1.0\"\n";
        assert!(matches!(
            parse_trust_root_section(toml),
            Err(TrustRootError::SectionMissing("trust_root"))
        ));
    }

    #[test]
    fn verify_trust_root_passes_on_intact_repo() {
        verify_trust_root(&repo_root()).expect("intact repo verifies");
    }

    /// Write a single-entry [trust_root] manifest pointing at `only.txt`
    /// with the given hex hash. Used by both tamper and match tests.
    fn write_single_entry_repo(tmp: &Path, only_txt: &str, manifest_hash: &str) {
        let genesis = format!(
            "[pput_accounting_0]\nschema_version = \"1.0\"\n\n\
             [trust_root]\n\"only.txt\" = \"{manifest_hash}\"\n"
        );
        fs::write(tmp.join("genesis_payload.toml"), genesis).unwrap();
        fs::write(tmp.join("only.txt"), only_txt).unwrap();
    }

    #[test]
    fn verify_trust_root_detects_tamper_in_tempdir() {
        // Manifest claims a zero hash; on-disk content "tampered" hashes to
        // anything else, so verify must surface Tampered.
        let tmp = tempdir();
        write_single_entry_repo(&tmp, "tampered", &"0".repeat(64));
        match verify_trust_root(&tmp).expect_err("tamper must be detected") {
            TrustRootError::Tampered { path, expected, actual } => {
                assert!(path.ends_with("only.txt"));
                assert_eq!(expected, "0".repeat(64));
                assert_ne!(actual, expected);
            }
            other => panic!("expected Tampered, got {other:?}"),
        }
    }

    #[test]
    fn verify_trust_root_passes_when_hash_matches_in_tempdir() {
        let tmp = tempdir();
        let payload = "hello";
        let hash = hex_lower(&Sha256::digest(payload.as_bytes()));
        write_single_entry_repo(&tmp, payload, &hash);
        verify_trust_root(&tmp).expect("matching hash verifies");
    }

    fn tempdir() -> PathBuf {
        // Minimal tempdir without adding a `tempfile` dep.
        let pid = std::process::id();
        let nano = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("turingosv4-boot-test-{pid}-{nano}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }
}
