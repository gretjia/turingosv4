//! True-suite evidence packaging gate.
//!
//! Fresh broad AGI evidence must be commit-friendly and reconstructable. Git
//! ignores nested `.git` stores, so current-kernel evidence packages those
//! stores into deterministic tarballs instead of relying on loose directories.

use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::Command;

use serde_json::Value;
use sha2::Digest;
use tempfile::TempDir;

const SCRIPT: &str = "scripts/package_true_suite_evidence.sh";

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).expect("read json")).expect("parse json")
}

fn sha256(path: &Path) -> String {
    let mut file = fs::File::open(path).expect("open file");
    let mut hash = sha2::Sha256::new();
    let mut buf = [0_u8; 8192];
    loop {
        let n = file.read(&mut buf).expect("read file");
        if n == 0 {
            break;
        }
        hash.update(&buf[..n]);
    }
    format!("{:x}", hash.finalize())
}

#[test]
fn true_suite_packager_archives_nested_git_stores_and_removes_loose_dirs() {
    let tmp = TempDir::new().expect("tempdir");
    let run_root = tmp.path().join("true_suite_run");
    let boot = run_root.join("boot_cli");
    let tdma = run_root.join("tdma");

    fs::create_dir_all(boot.join("runtime_repo/.git/refs/heads")).expect("mkdir runtime .git");
    fs::write(
        boot.join("runtime_repo/.git/HEAD"),
        "ref: refs/heads/main\n",
    )
    .expect("head");
    fs::write(
        boot.join("runtime_repo/.git/refs/heads/main"),
        "0123456789abcdef\n",
    )
    .expect("ref");
    fs::write(boot.join("runtime_repo/genesis_report.json"), "{}\n").expect("sidecar");

    fs::create_dir_all(boot.join("cas/.git/objects/aa")).expect("mkdir cas .git");
    fs::write(boot.join("cas/.git/objects/aa/blob"), "payload").expect("cas blob");
    fs::write(boot.join("cas/.turingos_cas_index.jsonl"), "{}\n").expect("cas index");

    fs::create_dir_all(tdma.join("tdma_tape.git/refs/tdma")).expect("mkdir tdma bare");
    fs::write(
        tdma.join("tdma_tape.git/HEAD"),
        "ref: refs/tdma/verified_head\n",
    )
    .expect("tdma head");
    fs::write(tdma.join("tdma_run_manifest.json"), "{}\n").expect("tdma manifest");

    let output = Command::new("bash")
        .arg(SCRIPT)
        .arg("--run-root")
        .arg(&run_root)
        .output()
        .expect("run package script");
    assert!(
        output.status.success(),
        "packager failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(!boot.join("runtime_repo/.git").exists());
    assert!(!boot.join("cas/.git").exists());
    assert!(!tdma.join("tdma_tape.git").exists());
    assert!(boot.join("runtime_repo/genesis_report.json").is_file());
    assert!(boot.join("cas/.turingos_cas_index.jsonl").is_file());

    for archive in [
        boot.join("runtime_repo.dotgit.tar.gz"),
        boot.join("cas.dotgit.tar.gz"),
        tdma.join("tdma_tape.git.tar.gz"),
    ] {
        assert!(archive.is_file(), "missing archive {}", archive.display());
        assert!(archive.metadata().expect("stat").len() > 0);
    }

    let manifest = read_json(&run_root.join("evidence_package_manifest.json"));
    assert_eq!(
        manifest.get("schema_version").and_then(Value::as_str),
        Some("turingosv4.true_suite.evidence_package_manifest.v1")
    );
    assert_eq!(
        manifest.get("package_count").and_then(Value::as_u64),
        Some(3)
    );
    let packages = manifest
        .get("packages")
        .and_then(Value::as_array)
        .expect("packages");
    for package in packages {
        assert!(
            package
                .get("archive_sha256")
                .and_then(Value::as_str)
                .is_some_and(|s| s.len() == 64),
            "package missing sha256: {package:?}"
        );
        assert_eq!(
            package.get("removed_loose_store").and_then(Value::as_bool),
            Some(true)
        );
    }
}

#[test]
fn true_suite_packager_archives_restore_to_expected_paths() {
    let tmp = TempDir::new().expect("tempdir");
    let run_root = tmp.path().join("true_suite_run");
    let domain = run_root.join("generate_artifact");

    fs::create_dir_all(domain.join("runtime_repo/.git/objects")).expect("mkdir runtime git");
    fs::write(
        domain.join("runtime_repo/.git/HEAD"),
        "ref: refs/heads/main\n",
    )
    .expect("head");
    fs::create_dir_all(domain.join("cas/.git/objects")).expect("mkdir cas git");
    fs::write(domain.join("cas/.git/HEAD"), "ref: refs/heads/main\n").expect("cas head");

    assert!(Command::new("bash")
        .arg(SCRIPT)
        .arg("--run-root")
        .arg(&run_root)
        .status()
        .expect("run package script")
        .success());

    let restore = tmp.path().join("restore");
    fs::create_dir_all(restore.join("runtime_repo")).expect("mkdir restore runtime");
    fs::create_dir_all(restore.join("cas")).expect("mkdir restore cas");

    assert!(Command::new("tar")
        .arg("-xzf")
        .arg(domain.join("runtime_repo.dotgit.tar.gz"))
        .arg("-C")
        .arg(restore.join("runtime_repo"))
        .status()
        .expect("extract runtime")
        .success());
    assert!(Command::new("tar")
        .arg("-xzf")
        .arg(domain.join("cas.dotgit.tar.gz"))
        .arg("-C")
        .arg(restore.join("cas"))
        .status()
        .expect("extract cas")
        .success());

    assert!(restore.join("runtime_repo/.git/HEAD").is_file());
    assert!(restore.join("cas/.git/HEAD").is_file());
}

#[test]
fn true_suite_packager_tarballs_are_deterministic_for_identical_input() {
    let tmp = TempDir::new().expect("tempdir");

    for name in ["run_a", "run_b"] {
        let domain = tmp.path().join(name).join("boot_cli");
        fs::create_dir_all(domain.join("runtime_repo/.git/refs/heads")).expect("mkdir git");
        fs::write(
            domain.join("runtime_repo/.git/HEAD"),
            "ref: refs/heads/main\n",
        )
        .expect("write head");
        fs::write(domain.join("runtime_repo/.git/refs/heads/main"), "abc123\n").expect("write ref");
        assert!(Command::new("bash")
            .arg(SCRIPT)
            .arg("--run-root")
            .arg(tmp.path().join(name))
            .status()
            .expect("run package script")
            .success());
    }

    let a = tmp.path().join("run_a/boot_cli/runtime_repo.dotgit.tar.gz");
    let b = tmp.path().join("run_b/boot_cli/runtime_repo.dotgit.tar.gz");
    assert_eq!(sha256(&a), sha256(&b));
}
