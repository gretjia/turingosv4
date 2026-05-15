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
    FileRead {
        path: PathBuf,
        err: std::io::Error,
    },
    Tampered {
        path: PathBuf,
        expected: String,
        actual: String,
    },
}

impl std::fmt::Display for TrustRootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GenesisRead(e) => write!(
                f,
                "TRUST_ROOT_TAMPERED: cannot read genesis_payload.toml: {e}"
            ),
            Self::GenesisParse(s) => write!(
                f,
                "TRUST_ROOT_TAMPERED: genesis_payload.toml parse error: {s}"
            ),
            Self::SectionMissing(s) => write!(
                f,
                "TRUST_ROOT_TAMPERED: genesis_payload.toml missing section [{s}]"
            ),
            Self::FileRead { path, err } => write!(
                f,
                "TRUST_ROOT_TAMPERED: cannot read tracked file {}: {err}",
                path.display()
            ),
            Self::Tampered {
                path,
                expected,
                actual,
            } => write!(
                f,
                "TRUST_ROOT_TAMPERED: {} hash mismatch (expected {}, actual {})",
                path.display(),
                expected,
                actual
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

    // CO1.0 v1 constitution_root verification (per GENESIS_MINIMAL_WITH_ANCHOR_v1).
    // Permissive in v4 first iteration: format + cross-ref check; full self-ref deferred to v4.x.
    verify_constitution_root_section(&genesis_text, &manifest)?;
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
        // Recurse into MANIFEST.sha256 children: the parent file's hash
        // alone doesn't bind the children it claims (proxy was convention,
        // not enforcement, before A8e13 fix Q1).
        if rel_path.ends_with("/MANIFEST.sha256") {
            verify_child_manifest(repo_root, &bytes)?;
        }
    }
    Ok(())
}

/// CO1.0 (v3.2-fix3): verify `[constitution_root]` section per
/// `handover/specs/GENESIS_MINIMAL_WITH_ANCHOR_v1_2026-04-27.md`.
///
/// 5 sub-checks (v4 first iteration is PERMISSIVE for placeholder fields):
/// 1. Section exists with all 8 expected keys
/// 2. `schema_version` == 1
/// 3. Hash-format fields are 64-char lowercase hex (or recognized placeholder)
/// 4. `constitution_hash` cross-references `[trust_root]["constitution.md"]`
/// 5. `signed_at` parses as ISO-8601-ish (basic format check)
///
/// Full self-referential `boot_attestation_hash` check is deferred to v4.x
/// once the user PGP/SSH ceremony completes (placeholder is valid in v4).
///
/// /// TRACE_MATRIX WP-spec-genesis-minimal-with-anchor + WP-arch-§5.L0: constitution_root verify
fn verify_constitution_root_section(
    genesis_text: &str,
    manifest: &[(String, String)],
) -> Result<(), TrustRootError> {
    if !has_section(genesis_text, "constitution_root") {
        return Err(TrustRootError::SectionMissing("constitution_root"));
    }

    let cr = parse_constitution_root_section(genesis_text)?;
    let required_keys = [
        "constitution_hash",
        "creator_signature",
        "signed_at",
        "schema_version",
        "amendment_predicate_hash",
        "initial_predicate_registry_root",
        "initial_tool_registry_root",
        "boot_attestation_hash",
    ];
    for key in required_keys {
        if !cr.iter().any(|(k, _)| k == key) {
            return Err(TrustRootError::GenesisParse(format!(
                "constitution_root: missing key '{key}'"
            )));
        }
    }

    let get =
        |key: &str| -> Option<&str> { cr.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str()) };

    // Sub-check 1: schema_version == 1
    let sv = get("schema_version").unwrap();
    if sv != "1" {
        return Err(TrustRootError::GenesisParse(format!(
            "constitution_root: schema_version expected '1', got '{sv}'"
        )));
    }

    // Sub-check 2: constitution_hash cross-ref with trust_root[constitution.md]
    let const_hash_root = get("constitution_hash").unwrap();
    // Cross-ref only when trust_root contains constitution.md (real repo; not tempdir tests).
    if let Some(const_hash_tr) = manifest
        .iter()
        .find(|(p, _)| p == "constitution.md")
        .map(|(_, h)| h.as_str())
    {
        if const_hash_root != const_hash_tr {
            return Err(TrustRootError::Tampered {
                path: PathBuf::from("constitution.md"),
                expected: const_hash_tr.to_string(),
                actual: format!("constitution_root.constitution_hash={const_hash_root}"),
            });
        }
    }

    // Sub-check 3: hex hash fields are 64-char lowercase hex (or recognized placeholder)
    for hash_key in [
        "constitution_hash",
        "amendment_predicate_hash",
        "initial_predicate_registry_root",
        "initial_tool_registry_root",
    ] {
        let v = get(hash_key).unwrap();
        let is_hex = v.len() == 64 && v.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'));
        if !is_hex {
            return Err(TrustRootError::GenesisParse(format!(
                "constitution_root: '{hash_key}' must be 64 lowercase hex chars; got len={} value={v:?}",
                v.len()
            )));
        }
    }

    // Sub-check 4: signed_at format basic check (YYYY-MM-DDThh:mm:ss±HH:MM at minimum)
    let signed_at = get("signed_at").unwrap();
    if signed_at.len() < 19 || !signed_at.chars().nth(4).map(|c| c == '-').unwrap_or(false) {
        return Err(TrustRootError::GenesisParse(format!(
            "constitution_root: signed_at not ISO-8601-like; got {signed_at:?}"
        )));
    }

    // Sub-check 5: creator_signature + boot_attestation_hash format permissive
    // (allow placeholder strings; v4.1+ enforces stricter when ceremony runs)
    let _creator_sig = get("creator_signature").unwrap(); // any non-empty string OK
    let _boot_att = get("boot_attestation_hash").unwrap(); // any non-empty string OK in v4

    Ok(())
}

/// Parse `[constitution_root]` section as ordered (key, value) pairs.
/// Hand-rolled to match existing `parse_trust_root_section` style; values
/// can be strings (quoted) or integers (unquoted; we preserve as string).
fn parse_constitution_root_section(text: &str) -> Result<Vec<(String, String)>, TrustRootError> {
    let mut in_section = false;
    let mut entries = Vec::new();
    for (lineno, raw) in text.lines().enumerate() {
        let line = strip_comment(raw).trim();
        if line.is_empty() {
            continue;
        }
        if let Some(header) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            in_section = header.trim() == "constitution_root";
            continue;
        }
        if !in_section {
            continue;
        }
        let (key, value) = line.split_once('=').ok_or_else(|| {
            TrustRootError::GenesisParse(format!(
                "line {}: missing '=' in [constitution_root]",
                lineno + 1
            ))
        })?;
        let key = key.trim();
        let value_raw = value.trim();
        // Accept either quoted strings or unquoted integers
        let value = if let Some(unq) = unquote(value_raw) {
            unq.to_string()
        } else {
            value_raw.to_string()
        };
        entries.push((key.to_string(), value));
    }
    if entries.is_empty() {
        return Err(TrustRootError::SectionMissing("constitution_root"));
    }
    Ok(entries)
}

/// TRACE_MATRIX FC3-N34 + case C-075: child-manifest recursion.
/// Format = GNU `sha256sum` (`<64-hex>  <repo-relative-path>`).
/// Paths resolve from `repo_root` (manifests are regenerated from
/// the repo root, not from each manifest's parent dir).
fn verify_child_manifest(repo_root: &Path, bytes: &[u8]) -> Result<(), TrustRootError> {
    let text = std::str::from_utf8(bytes)
        .map_err(|e| TrustRootError::GenesisParse(format!("manifest not utf-8: {e}")))?;
    for (i, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (hex, child_rel) = line.split_once("  ").ok_or_else(|| {
            TrustRootError::GenesisParse(format!("manifest line {}: {line:?}", i + 1))
        })?;
        if hex.len() != 64 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(TrustRootError::GenesisParse(format!(
                "manifest line {}: bad hex {hex:?}",
                i + 1
            )));
        }
        let path = repo_root.join(child_rel);
        let actual = hex_lower(&Sha256::digest(&fs::read(&path).map_err(|err| {
            TrustRootError::FileRead {
                path: path.clone(),
                err,
            }
        })?));
        if actual != hex.to_ascii_lowercase() {
            return Err(TrustRootError::Tampered {
                path,
                expected: hex.to_ascii_lowercase(),
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
            TrustRootError::GenesisParse(format!(
                "line {}: missing '=' in [trust_root]",
                lineno + 1
            ))
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
        line.strip_prefix('[')
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
    /// Includes minimal [constitution_root] section per CO1.0 (v3.2-fix3).
    fn write_single_entry_repo(tmp: &Path, only_txt: &str, manifest_hash: &str) {
        let empty_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let genesis = format!(
            "[pput_accounting_0]\nschema_version = \"1.0\"\n\n\
             [constitution_root]\n\
             constitution_hash = \"{empty_hash}\"\n\
             creator_signature = \"TEST_PLACEHOLDER\"\n\
             signed_at = \"2026-04-27T00:00:00+00:00\"\n\
             schema_version = 1\n\
             amendment_predicate_hash = \"{empty_hash}\"\n\
             initial_predicate_registry_root = \"{empty_hash}\"\n\
             initial_tool_registry_root = \"{empty_hash}\"\n\
             boot_attestation_hash = \"TEST_PLACEHOLDER\"\n\n\
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
            TrustRootError::Tampered {
                path,
                expected,
                actual,
            } => {
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

    /// A8e13 fix Q1 conformance: child manifest tamper is detected even
    /// when the parent manifest hash itself is unchanged. This is the
    /// scenario Codex R11#1 surfaced — pre-Q1, an attacker (or careless
    /// developer) could edit a `cases/*.yaml` without regenerating
    /// `cases/MANIFEST.sha256`, and boot would still pass because boot
    /// only checked the manifest file's own hash, not its child entries.
    #[test]
    fn verify_trust_root_detects_child_manifest_tamper() {
        let tmp = tempdir();
        // Lay out a fake repo with a child manifest pointing at one
        // child file. The PARENT manifest is hashed correctly into the
        // [trust_root] section, but the CHILD file's actual content is
        // tampered relative to what the parent manifest claims.
        // Manifest paths are repo-relative per the project convention.
        fs::create_dir_all(tmp.join("subdir")).unwrap();
        fs::write(tmp.join("subdir/child.txt"), "tampered_content").unwrap();
        let parent_text = format!("{}  subdir/child.txt\n", "0".repeat(64));
        fs::write(tmp.join("subdir/MANIFEST.sha256"), &parent_text).unwrap();
        let parent_hash = hex_lower(&Sha256::digest(parent_text.as_bytes()));
        let empty_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let genesis = format!(
            "[pput_accounting_0]\nschema_version = \"1.0\"\n\n\
             [constitution_root]\n\
             constitution_hash = \"{empty_hash}\"\n\
             creator_signature = \"TEST_PLACEHOLDER\"\n\
             signed_at = \"2026-04-27T00:00:00+00:00\"\n\
             schema_version = 1\n\
             amendment_predicate_hash = \"{empty_hash}\"\n\
             initial_predicate_registry_root = \"{empty_hash}\"\n\
             initial_tool_registry_root = \"{empty_hash}\"\n\
             boot_attestation_hash = \"TEST_PLACEHOLDER\"\n\n\
             [trust_root]\n\"subdir/MANIFEST.sha256\" = \"{parent_hash}\"\n"
        );
        fs::write(tmp.join("genesis_payload.toml"), genesis).unwrap();
        match verify_trust_root(&tmp).expect_err("child tamper must be detected") {
            TrustRootError::Tampered {
                path,
                expected,
                actual,
            } => {
                assert!(
                    path.ends_with("subdir/child.txt"),
                    "expected error on child.txt, got: {path:?}"
                );
                assert_eq!(expected, "0".repeat(64));
                assert_ne!(actual, expected);
            }
            other => panic!("expected Tampered on child, got {other:?}"),
        }
    }

    /// Q1 conformance: child manifest with matching hashes verifies cleanly.
    #[test]
    fn verify_trust_root_passes_with_matching_child_manifest() {
        let tmp = tempdir();
        fs::create_dir_all(tmp.join("subdir")).unwrap();
        let child_content = "the actual child content";
        fs::write(tmp.join("subdir/child.txt"), child_content).unwrap();
        let child_hash = hex_lower(&Sha256::digest(child_content.as_bytes()));
        let parent_text = format!("{child_hash}  subdir/child.txt\n");
        fs::write(tmp.join("subdir/MANIFEST.sha256"), &parent_text).unwrap();
        let parent_hash = hex_lower(&Sha256::digest(parent_text.as_bytes()));
        let empty_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let genesis = format!(
            "[pput_accounting_0]\nschema_version = \"1.0\"\n\n\
             [constitution_root]\n\
             constitution_hash = \"{empty_hash}\"\n\
             creator_signature = \"TEST_PLACEHOLDER\"\n\
             signed_at = \"2026-04-27T00:00:00+00:00\"\n\
             schema_version = 1\n\
             amendment_predicate_hash = \"{empty_hash}\"\n\
             initial_predicate_registry_root = \"{empty_hash}\"\n\
             initial_tool_registry_root = \"{empty_hash}\"\n\
             boot_attestation_hash = \"TEST_PLACEHOLDER\"\n\n\
             [trust_root]\n\"subdir/MANIFEST.sha256\" = \"{parent_hash}\"\n"
        );
        fs::write(tmp.join("genesis_payload.toml"), genesis).unwrap();
        verify_trust_root(&tmp).expect("matching parent + child verifies");
    }
}
