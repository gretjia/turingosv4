//! A03 KEEP-SRC-BOOT gate for boot Trust Root manifest verification.
//!
//! FC-trace: FC2 boot / CLI entry, FC3-N34 readonly Trust Root guard.
//! Risk: Class 3 floor. The ratified implementation shape keeps
//! `src/boot.rs::verify_trust_root` authoritative and adds focused tests only.

use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const EMPTY_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
const A03_DIRECTIVE: &str =
    "handover/directives/2026-06-05_A03_BOOT_TRUST_ROOT_MANIFEST_PREFLIGHT_AND_SECTION8_REQUEST.md";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn turingos_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_turingos"))
}

fn read_repo_text(rel: &str) -> String {
    fs::read_to_string(repo_root().join(rel)).unwrap_or_else(|err| panic!("read {rel}: {err}"))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(64);
    for byte in digest {
        write!(&mut out, "{byte:02x}").expect("write sha256 hex");
    }
    out
}

fn constitution_root_section(constitution_hash: &str) -> String {
    format!(
        "[constitution_root]\n\
         constitution_hash = \"{constitution_hash}\"\n\
         creator_signature = \"A03_TEST_PLACEHOLDER\"\n\
         signed_at = \"2026-06-06T00:00:00+00:00\"\n\
         schema_version = 1\n\
         amendment_predicate_hash = \"{EMPTY_SHA256}\"\n\
         initial_predicate_registry_root = \"{EMPTY_SHA256}\"\n\
         initial_tool_registry_root = \"{EMPTY_SHA256}\"\n\
         boot_attestation_hash = \"A03_TEST_PLACEHOLDER\"\n"
    )
}

fn write_genesis(repo: &Path, constitution_hash: &str, trust_root: &[(&str, String)]) {
    let mut text = String::new();
    text.push_str("[pput_accounting_0]\n");
    text.push_str("schema_version = \"1.0\"\n\n");
    text.push_str(&constitution_root_section(constitution_hash));
    text.push_str("\n[trust_root]\n");
    for (path, hash) in trust_root {
        writeln!(&mut text, "\"{path}\" = \"{hash}\"").expect("write trust root line");
    }
    fs::write(repo.join("genesis_payload.toml"), text).expect("write genesis_payload.toml");
}

fn boot_verify(repo: &Path) -> Output {
    Command::new(turingos_bin())
        .arg("boot")
        .arg("--verify-manifest")
        .arg("--repo")
        .arg(repo)
        .env("TURINGOS_TRUST_ROOT_BYPASS", "1")
        .env("ALLOW_TRUST_ROOT_BYPASS", "1")
        .env("TURINGOS_SKIP_TRUST_ROOT", "1")
        .output()
        .expect("run turingos boot --verify-manifest")
}

fn assert_failed_closed(output: &Output, expected_stderr: &str) {
    assert!(
        !output.status.success(),
        "boot verification unexpectedly succeeded\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("TRUST_ROOT_TAMPERED"),
        "stderr must name TRUST_ROOT_TAMPERED, got:\n{stderr}"
    );
    assert!(
        stderr.contains(expected_stderr),
        "stderr must contain {expected_stderr:?}, got:\n{stderr}"
    );
}

#[test]
fn a03_keep_src_boot_ratification_is_recorded_without_authority_migration() {
    let latest = read_repo_text("handover/ai-direct/LATEST.md");
    let directive = read_repo_text(A03_DIRECTIVE);

    for (name, text) in [("LATEST.md", latest), ("A03 directive", directive)] {
        assert!(
            text.contains("APPROVE-A03-SECTION8-KEEP-SRC-BOOT"),
            "{name} must record the exact A03 ratification phrase"
        );
        assert!(
            text.contains("A03_KEEP_SRC_BOOT_LANDED"),
            "{name} must record the landed A03 KEEP-SRC-BOOT marker"
        );
        assert!(
            text.contains("src/boot.rs::verify_trust_root"),
            "{name} must keep src/boot.rs::verify_trust_root as authority"
        );
    }

    assert!(
        !repo_root()
            .join("src/runtime/boot_trust_root_manifest.rs")
            .exists(),
        "KEEP-SRC-BOOT must not add a second boot Trust Root authority"
    );
}

#[test]
fn a03_keep_src_boot_uses_single_src_boot_authority_without_bypass_surface() {
    let cmd_boot = read_repo_text("src/bin/turingos/cmd_boot.rs");
    let main_rs = read_repo_text("src/main.rs");
    let boot_rs = read_repo_text("src/boot.rs");

    assert!(
        cmd_boot.contains("turingosv4::boot::verify_trust_root(&repo_root)"),
        "turingos boot must delegate to src/boot.rs::verify_trust_root"
    );
    assert!(
        main_rs.contains("turingosv4::boot::verify_trust_root(&repo_root)"),
        "default boot path must still call src/boot.rs::verify_trust_root"
    );
    assert!(
        boot_rs.contains("pub fn verify_trust_root(repo_root: &Path)"),
        "src/boot.rs must remain the public authoritative verifier"
    );

    for forbidden in [
        "std::env::var",
        "catch_unwind",
        "ALLOW_TRUST_ROOT",
        "BYPASS_TRUST_ROOT",
        "SKIP_TRUST_ROOT",
    ] {
        assert!(
            !cmd_boot.contains(forbidden),
            "cmd_boot.rs must not contain trust-root bypass surface {forbidden}"
        );
    }
}

#[test]
fn boot_cli_accepts_intact_repo_via_public_command() {
    let output = boot_verify(&repo_root());
    assert!(
        output.status.success(),
        "intact repo boot verification failed\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("BOOT-MANIFEST-VERIFIED"),
        "success stdout must carry BOOT-MANIFEST-VERIFIED"
    );
}

#[test]
fn boot_cli_fails_closed_on_file_hash_mismatch_even_with_bypass_envs() {
    let tmp = tempfile::tempdir().expect("temp repo");
    fs::write(tmp.path().join("only.txt"), "tampered").expect("write only.txt");
    write_genesis(tmp.path(), EMPTY_SHA256, &[("only.txt", "0".repeat(64))]);

    let output = boot_verify(tmp.path());
    assert_failed_closed(&output, "only.txt hash mismatch");
}

#[test]
fn boot_cli_fails_closed_on_constitution_hash_mismatch() {
    let tmp = tempfile::tempdir().expect("temp repo");
    let constitution = b"test constitution\n";
    let constitution_hash = sha256_hex(constitution);
    fs::write(tmp.path().join("constitution.md"), constitution).expect("write constitution.md");
    write_genesis(
        tmp.path(),
        &"1".repeat(64),
        &[("constitution.md", constitution_hash)],
    );

    let output = boot_verify(tmp.path());
    assert_failed_closed(&output, "constitution.md hash mismatch");
}

#[test]
fn boot_cli_fails_closed_on_child_manifest_payload_mismatch() {
    let tmp = tempfile::tempdir().expect("temp repo");
    fs::create_dir_all(tmp.path().join("cases")).expect("mkdir cases");
    fs::write(tmp.path().join("cases/case.yaml"), "actual: tampered\n").expect("write child");
    let child_manifest = format!("{}  cases/case.yaml\n", "0".repeat(64));
    fs::write(tmp.path().join("cases/MANIFEST.sha256"), &child_manifest)
        .expect("write child manifest");
    let child_manifest_hash = sha256_hex(child_manifest.as_bytes());
    write_genesis(
        tmp.path(),
        EMPTY_SHA256,
        &[("cases/MANIFEST.sha256", child_manifest_hash)],
    );

    let output = boot_verify(tmp.path());
    assert_failed_closed(&output, "cases/case.yaml hash mismatch");
}

#[test]
fn boot_cli_fails_closed_when_trust_root_section_is_missing() {
    let tmp = tempfile::tempdir().expect("temp repo");
    let genesis = format!(
        "[pput_accounting_0]\n\
         schema_version = \"1.0\"\n\n\
         {}\n",
        constitution_root_section(EMPTY_SHA256)
    );
    fs::write(tmp.path().join("genesis_payload.toml"), genesis).expect("write genesis");

    let output = boot_verify(tmp.path());
    assert_failed_closed(&output, "missing section [trust_root]");
}

#[test]
fn boot_cli_fails_closed_when_constitution_root_section_is_missing() {
    let tmp = tempfile::tempdir().expect("temp repo");
    fs::write(tmp.path().join("only.txt"), "hello").expect("write only.txt");
    let hash = sha256_hex(b"hello");
    let genesis = format!(
        "[pput_accounting_0]\n\
         schema_version = \"1.0\"\n\n\
         [trust_root]\n\
         \"only.txt\" = \"{hash}\"\n"
    );
    fs::write(tmp.path().join("genesis_payload.toml"), genesis).expect("write genesis");

    let output = boot_verify(tmp.path());
    assert_failed_closed(&output, "missing section [constitution_root]");
}
