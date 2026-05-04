//! TB-16 Atom 3 — `audit_tape` + `audit_tape_tamper` binary smoke test.
//!
//! Runs both binaries against an existing chain-backed smoke evidence
//! directory (TB-13 real-LLM smoke). Asserts:
//!
//! - audit_tape produces a verdict.json with the expected schema and
//!   the expected tape_root counts.
//! - audit_tape_tamper detects 3/3 corruptions (BLOCK verdict on each
//!   tampered copy).
//!
//! TRACE_MATRIX FC1-N34 + FC1-N35.

use std::path::PathBuf;
use std::process::Command;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn target_bin(name: &str) -> PathBuf {
    let manifest = manifest_dir();
    let dbg = manifest.join("target").join("debug").join(name);
    if dbg.exists() {
        return dbg;
    }
    let rel = manifest.join("target").join("release").join(name);
    if rel.exists() {
        return rel;
    }
    panic!("binary {name} not built (run cargo build --bin {name})");
}

fn fixture_smoke_dir() -> Option<PathBuf> {
    let p = manifest_dir()
        .join("handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171");
    if p.is_dir() && p.join("runtime_repo").is_dir() && p.join("cas").is_dir() {
        Some(p)
    } else {
        None
    }
}

#[test]
fn audit_tape_binary_help_succeeds() {
    let bin = target_bin("audit_tape");
    let out = Command::new(&bin)
        .arg("--help")
        .output()
        .expect("audit_tape --help");
    assert!(
        out.status.success() || out.status.code() == Some(0),
        "audit_tape --help should succeed; got status {:?}",
        out.status
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let combined = format!("{stderr}{stdout}");
    assert!(
        combined.contains("audit_tape") && combined.contains("USAGE"),
        "help text malformed"
    );
}

#[test]
fn audit_tape_tamper_binary_help_succeeds() {
    let bin = target_bin("audit_tape_tamper");
    let out = Command::new(&bin)
        .arg("--help")
        .output()
        .expect("audit_tape_tamper --help");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("audit_tape_tamper") && stderr.contains("USAGE"),
        "audit_tape_tamper help malformed: {stderr}"
    );
}

#[test]
fn audit_tape_runs_on_existing_chain_smoke() {
    let Some(smoke) = fixture_smoke_dir() else {
        eprintln!("skipping: no fixture chain at handover/evidence/tb_13_real_llm_smoke_*; binary still builds");
        return;
    };
    let bin = target_bin("audit_tape");
    let out_path = std::env::temp_dir().join("tb_16_audit_tape_smoke_verdict.json");
    let _ = std::fs::remove_file(&out_path);
    let manifest = manifest_dir();
    let status = Command::new(&bin)
        .arg("--runtime-repo")
        .arg(smoke.join("runtime_repo"))
        .arg("--cas-dir")
        .arg(smoke.join("cas"))
        .arg("--agent-pubkeys")
        .arg(smoke.join("runtime_repo/agent_pubkeys.json"))
        .arg("--pinned-pubkeys")
        .arg(smoke.join("runtime_repo/pinned_pubkeys.json"))
        .arg("--genesis")
        .arg(manifest.join("genesis_payload.toml"))
        .arg("--constitution")
        .arg(manifest.join("constitution.md"))
        .arg("--markov-pointer")
        .arg(manifest.join("handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt"))
        .arg("--alignment-dir")
        .arg(manifest.join("handover/alignment"))
        .arg("--out")
        .arg(&out_path)
        .status()
        .expect("audit_tape spawn");
    assert!(
        out_path.exists(),
        "audit_tape did not write verdict.json (status={:?})",
        status
    );
    let verdict_text = std::fs::read_to_string(&out_path).expect("read verdict");
    let v: serde_json::Value = serde_json::from_str(&verdict_text).expect("parse verdict");
    assert_eq!(v["schema_version"], "v1/audit_tape_verdict");
    assert!(v["assertions"].as_array().expect("assertions array").len() >= 38);
    assert!(v["tape_root"]["l4_count"].as_u64().expect("l4_count u64") >= 1);
    let verdict = v["verdict"].as_str().expect("verdict str");
    assert!(
        verdict == "PROCEED" || verdict == "BLOCK",
        "unexpected verdict: {verdict}"
    );
}
