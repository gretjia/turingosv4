use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

use turingosv4::git_tape_ledger::TcHeadRefs;
use turingosv4::runtime::boot_trust_root_manifest::{
    verify_tc_boot_manifest, TcBootManifest, TcBootManifestError, TC_BOOT_MANIFEST_SCHEMA_ID,
};
use turingosv4::runtime::tc_tape_canonical::{TapeAnchor, TcTapeCanonicalFact};
use turingosv4::top_white::predicates::registry::{BootPredicateManifest, PredicateRegistry};

fn hex_lower(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

fn sha256_file(path: &Path) -> String {
    hex_lower(&Sha256::digest(fs::read(path).expect("read fixture file")))
}

fn predicate_root_hex() -> String {
    let registry = PredicateRegistry::from_boot_manifest(BootPredicateManifest::v8_production())
        .expect("v8 predicate manifest constructs");
    hex_lower(&registry.merkle_root())
}

fn valid_manifest_for(repo_root: &Path) -> String {
    let refs = TcHeadRefs::default();
    format!(
        r#"
schema_id = "{schema}"
constitution_sha256 = "{constitution}"
genesis_payload_sha256 = "{genesis}"
predicate_manifest_root = "{predicate_root}"

[refs]
accepted_l4 = "{accepted_l4}"
rejected_l4e = "{rejected_l4e}"
cas_root = "{cas_root}"
tdma_verified = "{tdma_verified}"
tdma_tail = "{tdma_tail}"
"#,
        schema = TC_BOOT_MANIFEST_SCHEMA_ID,
        constitution = sha256_file(&repo_root.join("constitution.md")),
        genesis = sha256_file(&repo_root.join("genesis_payload.toml")),
        predicate_root = predicate_root_hex(),
        accepted_l4 = refs.accepted_l4,
        rejected_l4e = refs.rejected_l4e,
        cas_root = refs.cas_root,
        tdma_verified = refs.tdma_verified,
        tdma_tail = refs.tdma_tail,
    )
}

fn anchor() -> TapeAnchor {
    TapeAnchor {
        run_id: "run-tc-boot".to_string(),
        logical_t: Some(1),
        submit_id: None,
        head_ref: "refs/chaintape/l4".to_string(),
        head_oid: "0123456789abcdef0123456789abcdef01234567".to_string(),
    }
}

#[test]
fn tc_boot_manifest_verifies_constitution_predicates_payload_and_refs() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest: TcBootManifest =
        toml::from_str(&valid_manifest_for(&repo_root)).expect("manifest parses");

    verify_tc_boot_manifest(&repo_root, &manifest).expect("valid TC boot manifest verifies");
}

#[test]
fn tc_boot_manifest_sha_mismatch_fails_closed() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut manifest: TcBootManifest =
        toml::from_str(&valid_manifest_for(&repo_root)).expect("manifest parses");
    manifest.constitution_sha256 = "0".repeat(64);

    match verify_tc_boot_manifest(&repo_root, &manifest).expect_err("mismatch must fail") {
        TcBootManifestError::HashMismatch {
            path,
            expected,
            actual,
        } => {
            assert_eq!(path, PathBuf::from("constitution.md"));
            assert_eq!(expected, "0".repeat(64));
            assert_ne!(actual, expected);
        }
        other => panic!("expected HashMismatch, got {other:?}"),
    }
}

#[test]
fn turingos_boot_verify_manifest_cli_passes_and_mismatch_fails() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tmp = std::env::temp_dir().join(format!(
        "turingosv4-tc-boot-manifest-{}",
        std::process::id()
    ));
    fs::create_dir_all(&tmp).expect("create tmp");
    let ok_manifest = tmp.join("tc_boot_manifest.toml");
    fs::write(&ok_manifest, valid_manifest_for(&repo_root)).expect("write ok manifest");

    let bin = PathBuf::from(env!("CARGO_BIN_EXE_turingos"));
    let ok = Command::new(&bin)
        .arg("boot")
        .arg("--verify-manifest")
        .arg(&ok_manifest)
        .arg("--repo-root")
        .arg(&repo_root)
        .output()
        .expect("run turingos boot");
    assert!(
        ok.status.success(),
        "valid manifest should pass: stdout={} stderr={}",
        String::from_utf8_lossy(&ok.stdout),
        String::from_utf8_lossy(&ok.stderr)
    );

    let constitution = Command::new(&bin)
        .arg("boot")
        .arg("--verify-constitution-hash")
        .arg(&ok_manifest)
        .arg("--repo-root")
        .arg(&repo_root)
        .output()
        .expect("run turingos boot constitution");
    assert!(
        constitution.status.success(),
        "constitution hash gate should pass: stdout={} stderr={}",
        String::from_utf8_lossy(&constitution.stdout),
        String::from_utf8_lossy(&constitution.stderr)
    );

    let predicates = Command::new(&bin)
        .arg("boot")
        .arg("--verify-predicates")
        .arg(&ok_manifest)
        .output()
        .expect("run turingos boot predicates");
    assert!(
        predicates.status.success(),
        "predicate gate should pass: stdout={} stderr={}",
        String::from_utf8_lossy(&predicates.stdout),
        String::from_utf8_lossy(&predicates.stderr)
    );

    let bad_manifest = tmp.join("tc_boot_manifest_bad.toml");
    let bad_text = valid_manifest_for(&repo_root).replace(
        &sha256_file(&repo_root.join("genesis_payload.toml")),
        &"f".repeat(64),
    );
    fs::write(&bad_manifest, bad_text).expect("write bad manifest");

    let bad = Command::new(&bin)
        .arg("boot")
        .arg("--verify-manifest")
        .arg(&bad_manifest)
        .arg("--repo-root")
        .arg(&repo_root)
        .output()
        .expect("run turingos boot bad");
    assert!(
        !bad.status.success(),
        "mismatch manifest must fail closed: stdout={} stderr={}",
        String::from_utf8_lossy(&bad.stdout),
        String::from_utf8_lossy(&bad.stderr)
    );
}

#[test]
fn boot_provenance_binds_manifest_hashes_refs_and_predicate_root() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest: TcBootManifest =
        toml::from_str(&valid_manifest_for(&repo_root)).expect("manifest parses");
    let constitution_before = sha256_file(&repo_root.join("constitution.md"));
    let genesis_before = sha256_file(&repo_root.join("genesis_payload.toml"));

    let fact =
        TcTapeCanonicalFact::boot_provenance(anchor(), &manifest, "boot provenance verified")
            .expect("boot provenance fact builds");

    assert_eq!(fact.fact.kind, "boot_provenance");
    assert_eq!(fact.constitution_sha256, constitution_before);
    assert_eq!(fact.genesis_payload_sha256, genesis_before);
    assert_eq!(fact.predicate_manifest_root, predicate_root_hex());
    assert_eq!(fact.refs.accepted_l4, "refs/chaintape/l4");
    assert_eq!(fact.refs.rejected_l4e, "refs/chaintape/l4e");
    assert_eq!(fact.refs.cas_root, "refs/chaintape/cas");
    assert_eq!(fact.refs.tdma_verified, "refs/tdma/verified_head");
    assert_eq!(fact.refs.tdma_tail, "refs/tdma/ledger_tail");

    assert_eq!(
        sha256_file(&repo_root.join("constitution.md")),
        constitution_before
    );
    assert_eq!(
        sha256_file(&repo_root.join("genesis_payload.toml")),
        genesis_before
    );
}
